// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::{Finite, Infinity, NaN, Zero};
use crate::big_exponent_float::arithmetic::add::significand_prec;
use crate::big_exponent_float::arithmetic::mul_rational::shift_finite;
use crate::big_exponent_float::bracket::{
    Bracket, ZIV_GUARD_BITS, floor_bracket, rational_exponent_bound, rounding_possible,
    ziv_loop_certified,
};
use crate::big_exponent_float::exponential::exp::power_of_2_integer;
use crate::float::exponential::exp::primitive_float_exp;
use crate::float::exponential::ln::primitive_float_ln;
use alloc::borrow::Cow;
use core::cmp::{
    Ordering::{self, *},
    max,
};
use core::mem::swap;
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{Abs, Sqrt};
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, NegativeOne, One, Two, Zero as ZeroTrait,
};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_q::Rational;

// The largest absolute value of an argument for which a seed is made in `f64`: 2^1000. From about
// 2^1021 on, the product (w + 2) * f in the `f64` iteration overflows and the iteration stays at
// its start value.
const SEED_MAX: f64 = 1.0715086071862673e301;

// The number of Halley steps taken in `f64` to make a seed.
const SEED_STEPS: usize = 8;

// The number of Newton steps taken to make a seed from the logarithm of the argument.
const LOG_SEED_STEPS: usize = 4;

// The number of mean-value rounds that shrink the enclosure around a candidate.
const MEAN_VALUE_ROUNDS: usize = 3;

// The argument of a Lambert W function: a `BigExponentFloat` or a `Rational`.
pub(crate) enum LambertTarget<'a> {
    BigExponentFloat(&'a BigExponentFloat),
    Rational(&'a Rational),
}

impl LambertTarget<'_> {
    // A float at or below the argument, at precision `prec`: the float itself for a float, and the
    // `Floor` rounding of a rational.
    //
    // This is equivalent to the first components of `floatAt` and `ratAt` from
    // `AzFloat/LambertWBracket.lean`, Azurite.
    #[inline]
    fn lower(&self, prec: u64) -> Cow<'_, BigExponentFloat> {
        match self {
            LambertTarget::BigExponentFloat(x) => Cow::Borrowed(*x),
            LambertTarget::Rational(q) => {
                Cow::Owned(BigExponentFloat::from_rational_prec_round_ref(q, prec, Floor).0)
            }
        }
    }

    // A float at or above the argument, at precision `prec`: the float itself for a float, and the
    // `Ceiling` rounding of a rational.
    //
    // This is equivalent to the second components of `floatAt` and `ratAt` from
    // `AzFloat/LambertWBracket.lean`, Azurite.
    #[inline]
    fn upper(&self, prec: u64) -> Cow<'_, BigExponentFloat> {
        match self {
            LambertTarget::BigExponentFloat(x) => Cow::Borrowed(*x),
            LambertTarget::Rational(q) => {
                Cow::Owned(BigExponentFloat::from_rational_prec_round_ref(q, prec, Ceiling).0)
            }
        }
    }
}

// This is equivalent to `maxOf` from `AzFloat/Compare.lean`, Azurite.
fn max_of(x: BigExponentFloat, y: BigExponentFloat) -> BigExponentFloat {
    if x < y { y } else { x }
}

// This is equivalent to `minOf` from `AzFloat/Compare.lean`, Azurite.
fn min_of(x: BigExponentFloat, y: BigExponentFloat) -> BigExponentFloat {
    if y < x { y } else { x }
}

// An enclosure of e^x at precision `prec`, for a finite x.
//
// This is equivalent to `expBracketAt` from `AzFloat/ExpApprox.lean`, Azurite, with the correctly
// rounded exponential in place of Azurite's bracket.
fn exp_bracket_at(x: &BigExponentFloat, prec: u64) -> Bracket {
    if x.is_zero() {
        // Only a candidate clamped to zero or a bisection midpoint at zero gets here.
        fail_on_untested_path("exp_bracket_at, x == 0");
        return (BigExponentFloat::ONE, BigExponentFloat::ONE);
    }
    floor_bracket(x.exp_prec_round_ref(prec, Floor).0, prec)
}

// `y` times a bracket of any signs, at precision `prec`, rounded outward.
//
// This is equivalent to `mulBracketAny` from `AzFloat/PowBracket.lean`, Azurite.
fn mul_bracket_any(y: &BigExponentFloat, b: &Bracket, prec: u64) -> Bracket {
    let (lo, hi) = if y.is_sign_negative() {
        (&b.1, &b.0)
    } else {
        (&b.0, &b.1)
    };
    (
        y.mul_prec_round_ref_ref(lo, prec, Floor).0,
        y.mul_prec_round_ref_ref(hi, prec, Ceiling).0,
    )
}

// `Some(true)` when m * e^m < x is certain at precision `prec`, `Some(false)` when m * e^m > x is,
// and `None` otherwise.
//
// This is equivalent to `sideAt` (and its `mulExpBracket`) from `AzFloat/LambertWBracket.lean`,
// Azurite.
fn side_at(target: &LambertTarget, m: &BigExponentFloat, prec: u64) -> Option<bool> {
    let f = mul_bracket_any(m, &exp_bracket_at(m, prec), prec);
    if f.1 < *target.lower(prec) {
        Some(true)
    } else if *target.upper(prec) < f.0 {
        Some(false)
    } else {
        None
    }
}

// Whether m * e^m < x, decided at the first of the precisions 64, 128, 256, ... that can.
//
// This is equivalent to `sideOf` (and its `findSome`) from `AzFloat/LambertWBracket.lean`, Azurite.
fn side_of(target: &LambertTarget, m: &BigExponentFloat) -> bool {
    let mut prec = 64;
    loop {
        if let Some(below) = side_at(target, m, prec) {
            return below;
        }
        prec <<= 1;
    }
}

// Bounds of q = 1 + e * x for x < 0 at precision `prec`.
//
// This is equivalent to `qBounds` from `AzFloat/LambertWBracket.lean`, Azurite.
fn q_bounds(target: &LambertTarget, prec: u64) -> Bracket {
    let (e_lo, e_hi) = exp_bracket_at(&BigExponentFloat::ONE, prec);
    let ex_lo = e_hi
        .mul_prec_round_val_ref(&target.lower(prec), prec, Floor)
        .0;
    let ex_hi = e_lo
        .mul_prec_round_val_ref(&target.upper(prec), prec, Ceiling)
        .0;
    (
        BigExponentFloat::ONE.add_prec_round(ex_lo, prec, Floor).0,
        BigExponentFloat::ONE.add_prec_round(ex_hi, prec, Ceiling).0,
    )
}

// A decision about q = 1 + e * x for x < 0 at precision `prec`: `Some(None)` when q < 0 is certain,
// `Some(Some((lo, hi)))` when 0 < lo <= q <= hi is, and `None` otherwise.
//
// This is equivalent to `qAt` from `AzFloat/LambertWBracket.lean`, Azurite.
fn q_at(target: &LambertTarget, prec: u64) -> Option<Option<Bracket>> {
    let q = q_bounds(target, prec);
    if q.1 < 0u32 {
        Some(None)
    } else if q.0 > 0u32 {
        Some(Some(q))
    } else {
        None
    }
}

// The decision about q, made at the first of the precisions 64, 128, 256, ... that can: `None` when
// q < 0, and an enclosure of q with a positive lower end otherwise. This does not return when q is
// zero, which no float or rational argument achieves.
//
// This is equivalent to `qEncl` (and its `findSome`) from `AzFloat/LambertWBracket.lean`, Azurite.
fn q_encl(target: &LambertTarget) -> Option<Bracket> {
    let mut prec = 64;
    loop {
        if let Some(q) = q_at(target, prec) {
            return q;
        }
        prec <<= 1;
    }
}

// The extra working bits near the branch point: about -lg(q) / 2 + 2.
//
// This is equivalent to `compBits` from `AzFloat/LambertWBracket.lean`, Azurite.
fn comp_bits(q: Option<&Bracket>) -> u64 {
    match q.and_then(|(lo, _)| lo.get_exponent()) {
        Some(e) if *e < 0u32 => u64::exact_from(&-e).div_ceil(2) + 2,
        _ => 0,
    }
}

// Near the branch point the enclosure of q is recomputed at 128 + 2 * `comp_bits` bits and kept
// when its lower end is positive.
//
// This is equivalent to `qTight` from `AzFloat/LambertWBracket.lean`, Azurite.
fn q_tight(target: &LambertTarget, q: Bracket) -> Bracket {
    let c = comp_bits(Some(&q));
    if c == 0 {
        return q;
    }
    let tight = q_bounds(target, 128 + (c << 1));
    if tight.0 > 0u32 {
        tight
    } else {
        // q was bounded away from zero with at most about -lg(q) + 64 bits, and the recomputation
        // uses more than that.
        fail_on_untested_path("q_tight, recomputed lower end not positive");
        q
    }
}

// The bit length of the exponent of a float: ln|x| has about that many integer bits.
//
// This is equivalent to `expSize` from `AzFloat/LambertWBracket.lean`, Azurite.
fn exponent_size(x: &BigExponentFloat) -> u64 {
    x.get_exponent()
        .map_or(0, SignificantBits::significant_bits)
}

// An enclosure of W_0(x) for x > 0: [x / (1 + x), x] for x < 3, and [ln(x) - ln(ln(x)), ln(x)]
// otherwise. The logarithms are taken with `prec` bits after the binary point, so that the lower
// end stays close to W_0(x) however large x is.
//
// This is equivalent to `aprioriW0Pos` from `AzFloat/LambertWBracket.lean`, Azurite.
fn apriori_w0_pos(target: &LambertTarget, prec: u64) -> Bracket {
    let x_lo = target.lower(prec);
    let x_hi = target.upper(prec).into_owned();
    if *x_lo < 3u32 {
        let d = BigExponentFloat::ONE
            .add_prec_round_val_ref(&x_lo, prec, Ceiling)
            .0;
        (x_lo.div_prec_round_ref_val(d, prec, Floor).0, x_hi)
    } else {
        let prec = prec + exponent_size(&x_hi);
        let ln_lo = x_lo.ln_prec_round_ref(prec, Floor).0;
        let ln_hi = x_hi.ln_prec_round(prec, Ceiling).0;
        let ln_ln_hi = ln_hi.ln_prec_round_ref(prec, Ceiling).0;
        (ln_lo.sub_prec_round(ln_ln_hi, prec, Floor).0, ln_hi)
    }
}

// An enclosure of W_0(x) for -1/e < x < 0 from an enclosure of q: [max(-1, -1 + sqrt(2 * q_lo /
// e)), min(0, -1 + sqrt(2 * q_hi))].
//
// This is equivalent to `aprioriW0Neg` from `AzFloat/LambertWBracket.lean`, Azurite.
fn apriori_w0_neg(q: &Bracket, prec: u64) -> Bracket {
    let e_hi = exp_bracket_at(&BigExponentFloat::ONE, prec).1;
    let r = (&q.0 << 1u32)
        .div_prec_round(e_hi, prec, Floor)
        .0
        .sqrt_prec_round(prec, Floor)
        .0;
    let s = (&q.1 << 1u32).sqrt_prec_round(prec, Ceiling).0;
    (
        max_of(
            BigExponentFloat::NEGATIVE_ONE,
            BigExponentFloat::NEGATIVE_ONE
                .add_prec_round(r, prec, Floor)
                .0,
        ),
        min_of(
            BigExponentFloat::ZERO,
            BigExponentFloat::NEGATIVE_ONE
                .add_prec_round(s, prec, Ceiling)
                .0,
        ),
    )
}

// An enclosure of W_{-1}(x) for -1/e < x < 0 from an enclosure of q. The upper end is -1 - sqrt(2 *
// q_lo). The lower end is -1 - sqrt(2 * e * q_hi) when q_hi <= 1/4, and ln(y) - ln(-ln(y)) - 1 with
// y = -x otherwise, the logarithms being taken with `prec` bits after the binary point.
//
// This is equivalent to `aprioriWm1` from `AzFloat/LambertWBracket.lean`, Azurite.
fn apriori_wm1(target: &LambertTarget, q: &Bracket, prec: u64) -> Bracket {
    let s = (&q.0 << 1u32).sqrt_prec_round(prec, Floor).0;
    let hi = min_of(
        BigExponentFloat::NEGATIVE_ONE,
        BigExponentFloat::NEGATIVE_ONE
            .sub_prec_round(s, prec, Ceiling)
            .0,
    );
    let lo = if q.1 <= 0.25f64 {
        let e_hi = exp_bracket_at(&BigExponentFloat::ONE, prec).1;
        let t = (e_hi << 1u32)
            .mul_prec_round_val_ref(&q.1, prec, Ceiling)
            .0
            .sqrt_prec_round(prec, Ceiling)
            .0;
        BigExponentFloat::NEGATIVE_ONE
            .sub_prec_round(t, prec, Floor)
            .0
    } else {
        let x_hi = target.upper(prec);
        let prec = prec + exponent_size(&x_hi);
        let ln_lo = (-&*x_hi).ln_prec_round(prec, Floor).0;
        let ln_ln_hi = (-&ln_lo).ln_prec_round(prec, Ceiling).0;
        ln_lo
            .sub_prec_round(ln_ln_hi, prec, Floor)
            .0
            .sub_prec_round(BigExponentFloat::ONE, prec, Floor)
            .0
    };
    (lo, hi)
}

// A lower bound of e^a from a lower bound `exp_lo` of e^n: `exp_lo` * (1 + a - n) when that is
// positive, and a fresh exponential otherwise.
//
// This is equivalent to `derivLowerExp` from `AzFloat/LambertWBracket.lean`, Azurite.
fn deriv_lower_exp(
    a: &BigExponentFloat,
    n: &BigExponentFloat,
    exp_lo: &BigExponentFloat,
) -> BigExponentFloat {
    let c = a
        .sub_prec_round_ref_ref(n, 64, Floor)
        .0
        .add_prec_round(BigExponentFloat::ONE, 64, Floor)
        .0;
    if c > 0u32 {
        exp_lo.mul_prec_round_ref_val(c, 64, Floor).0
    } else {
        exp_bracket_at(a, 64).0
    }
}

// One mean-value round: the root is within `delta` / d of the candidate `n`, where `delta` >= |x -
// n * e^n| and d is a lower bound of the derivative (1 + w) * e^w over the interval.
//
// This is equivalent to `mvtRound` from `AzFloat/LambertWBracket.lean`, Azurite.
fn mvt_round(
    branch0: bool,
    n: &BigExponentFloat,
    exp_lo: &BigExponentFloat,
    delta: &BigExponentFloat,
    interval: Bracket,
    prec: u64,
) -> Bracket {
    let dist = if branch0 {
        interval
            .0
            .add_prec_round_ref_val(BigExponentFloat::ONE, 64, Floor)
            .0
    } else {
        BigExponentFloat::NEGATIVE_ONE
            .sub_prec_round_val_ref(&interval.1, 64, Floor)
            .0
    };
    if dist > 0u32 {
        // d is positive: it is the rounded-down product of two positive numbers, and a
        // `BigExponentFloat` cannot underflow.
        let d = dist
            .mul_prec_round(deriv_lower_exp(&interval.0, n, exp_lo), 64, Floor)
            .0;
        let r = delta.div_prec_round_ref_val(d, prec, Ceiling).0;
        let lo = n.sub_prec_round_ref_ref(&r, prec, Floor).0;
        let hi = n.add_prec_round_ref_val(r, prec, Ceiling).0;
        (max_of(interval.0, lo), min_of(interval.1, hi))
    } else {
        // The a priori interval would have to touch -1, which takes a q below 2^(-2 * prec).
        fail_on_untested_path("mvt_round, dist <= 0");
        interval
    }
}

// Three mean-value rounds around the candidate `n`, starting from the a priori interval.
//
// This is equivalent to `mvtRounds` (and its `mvtDelta`) from `AzFloat/LambertWBracket.lean`,
// Azurite.
fn mvt_rounds(
    branch0: bool,
    target: &LambertTarget,
    n: &BigExponentFloat,
    apriori: &Bracket,
    prec: u64,
) -> Bracket {
    let e = exp_bracket_at(n, prec);
    let f = mul_bracket_any(n, &e, prec);
    // delta >= |x - n * e^n|
    let delta = max_of(
        f.1.sub_prec_round_ref_ref(&target.lower(prec), prec, Ceiling)
            .0,
        target
            .upper(prec)
            .sub_prec_round_ref_ref(&f.0, prec, Ceiling)
            .0,
    );
    let mut interval = apriori.clone();
    for _ in 0..MEAN_VALUE_ROUNDS {
        interval = mvt_round(branch0, n, &e.0, &delta, interval, prec);
    }
    interval
}

// m * 2^e.
fn integer_shl(m: &Integer, e: &Integer) -> BigExponentFloat {
    let x = BigExponentFloat::from(m);
    if x.is_zero() { x } else { shift_finite(x, e) }
}

// The integers a and b such that [a, b] encloses an interval, clamped to the monotone region of the
// branch.
//
// This is equivalent to `gridOf` from `AzFloat/LambertWBracket.lean`, Azurite.
fn grid_of(branch0: bool, interval: &Bracket) -> (Integer, Integer) {
    let a = Integer::rounding_from(&interval.0, Down).0 - Integer::ONE;
    if branch0 {
        let b = Integer::rounding_from(&interval.1, Down).0 + Integer::ONE;
        (max(a, Integer::NEGATIVE_ONE), b)
    } else {
        // The upper end of an a priori interval of W_{-1} is at least -1 - sqrt(2), so its integer
        // above is at least -1, and it is clamped to -1.
        (a, Integer::NEGATIVE_ONE)
    }
}

// `steps` bisection steps on the grids 2^e * Z, starting from the integers around an interval: a
// step halves [a * 2^e, b * 2^e], refining the grid first when a and b are adjacent.
//
// This is equivalent to `bisectGrid` (with `gridOf`, `gridBisectStep`, and `gridFloats`) from
// `AzFloat/LambertWBracket.lean`, Azurite.
fn bisect_grid(branch0: bool, target: &LambertTarget, steps: u64, interval: &Bracket) -> Bracket {
    let (mut a, mut b) = grid_of(branch0, interval);
    let mut e = Integer::ZERO;
    for _ in 0..steps {
        if &b - &a <= 1u32 {
            a <<= 1u32;
            b <<= 1u32;
            e -= Integer::ONE;
        }
        let m = (&a + &b) >> 1u32;
        // When m * e^m < x, W > m on the increasing branch and W < m on the decreasing one.
        if side_of(target, &integer_shl(&m, &e)) == branch0 {
            a = m;
        } else {
            b = m;
        }
    }
    (integer_shl(&a, &e), integer_shl(&b, &e))
}

// The tiny-argument path of W_0, for an argument x with |x| < 2^e, given by the rounding procedure
// `nx` of -x and a float `nx_hi` >= -x: -W_0(x) lies in (-x, -x + 2^(2e + 2)].
//
// This is equivalent to `lambertW0Tiny?` from `AzFloat/LambertWBracket.lean`, Azurite.
fn lambert_w0_tiny<F: Fn(u64, RoundingMode) -> (BigExponentFloat, Ordering)>(
    nx: &F,
    nx_hi: &BigExponentFloat,
    e: &Integer,
    w: u64,
    prec: u64,
    rm: RoundingMode,
) -> Option<(BigExponentFloat, Ordering)> {
    if *e > -Integer::from(w + 8) {
        return None;
    }
    assert_ne!(rm, Exact, "Inexact Lambert W function");
    let hi = nx_hi
        .add_prec_round_ref_val(
            power_of_2_integer((e << 1u32) + Integer::TWO),
            w + 8,
            Ceiling,
        )
        .0;
    rounding_possible(nx, &|p, m| hi.round_to_prec_round_ref(p, m), prec, -rm)
        .map(|(r, o)| (-r, o.reverse()))
}

// One Halley step for w * e^w = x at precision `prec`.
//
// This is equivalent to `lambertHalleyStep` from `AzFloat/LambertWBracket.lean`, Azurite.
fn lambert_halley_step(x: &BigExponentFloat, w: &BigExponentFloat, prec: u64) -> BigExponentFloat {
    let e = w.exp_prec_ref(prec).0;
    let f = w.mul_prec_ref_ref(&e, prec).0.sub_prec_val_ref(x, prec).0;
    let w1 = w.add_prec_ref_val(BigExponentFloat::ONE, prec).0;
    let fp = e.mul_prec_val_ref(&w1, prec).0;
    let w2 = w.add_prec_ref_val(BigExponentFloat::TWO, prec).0;
    let corr = w2.mul_prec_val_ref(&f, prec).0.div_prec(w1 << 1u32, prec).0;
    let den = fp.sub_prec(corr, prec).0;
    w.sub_prec_ref_val(f.div_prec(den, prec).0, prec).0
}

// Halley steps at precisions `prec` + `loss`, 2 * `prec` + `loss`, 4 * `prec` + `loss`, ..., then
// one at `w_prec` + `loss`.
//
// This is equivalent to `halleyRefine` from `AzFloat/LambertWBracket.lean`, Azurite, whose fuel
// never runs out.
fn halley_refine(
    x: &BigExponentFloat,
    mut w: BigExponentFloat,
    mut prec: u64,
    w_prec: u64,
    loss: u64,
) -> BigExponentFloat {
    while (w_prec << 1) + 32 > 3 * prec {
        w = lambert_halley_step(x, &w, prec + loss);
        prec <<= 1;
    }
    lambert_halley_step(x, &w, w_prec + loss)
}

// `SEED_STEPS` steps of Halley's iteration in `f64` from a start value.
//
// This is equivalent to `halleyFloat` from `AzFloat/LambertWBracket.lean`, Azurite.
fn halley_float(x: f64, mut w: f64) -> f64 {
    for _ in 0..SEED_STEPS {
        let e = primitive_float_exp(w);
        let f = w * e - x;
        let w1 = w + 1.0;
        let den = e * w1 - (w + 2.0) * f / (2.0 * w1);
        w -= f / den;
    }
    w
}

// An `f64` seed for the branch. There is none when `f64` cannot represent the problem: when the
// argument is below the normal range, where the conversion keeps only a few bits of the argument
// and the seed would have as few, or when it is above `SEED_MAX`.
//
// This is equivalent to `seedFloat` from `AzFloat/LambertWBracket.lean`, Azurite.
fn seed_float(branch0: bool, x: &BigExponentFloat) -> Option<BigExponentFloat> {
    let x = f64::rounding_from(x, Nearest).0;
    if !(f64::MIN_POSITIVE..=SEED_MAX).contains(&x.abs()) {
        return None;
    }
    let q = 1.0 + primitive_float_exp(1.0) * x;
    let start = if branch0 {
        if x < 0.0 {
            -1.0 + Sqrt::sqrt(2.0 * q) - 2.0 * q / 3.0
        } else if x < 1.0 {
            x
        } else {
            let ln_x = primitive_float_ln(x);
            ln_x - primitive_float_ln(1.0 + ln_x)
        }
    } else if q < 0.3 {
        -1.0 - Sqrt::sqrt(2.0 * q) - 2.0 * q / 3.0
    } else {
        let ln_y = primitive_float_ln(-x);
        ln_y - primitive_float_ln(-ln_y)
    };
    let w = halley_float(x, start);
    if w.is_finite() {
        Some(BigExponentFloat::from(w))
    } else {
        fail_on_untested_path("seed_float, iteration not finite");
        None
    }
}

// One Newton step for w + ln|w| = l at precision `prec`.
//
// This is equivalent to `logSeedStep` from `AzFloat/LambertWBracket.lean`, Azurite.
fn log_seed_step(l: &BigExponentFloat, w: &BigExponentFloat, prec: u64) -> BigExponentFloat {
    let g = w
        .add_prec_ref_val(w.abs().ln_prec(prec).0, prec)
        .0
        .sub_prec_val_ref(l, prec)
        .0;
    let c = g
        .mul_prec_val_ref(w, prec)
        .0
        .div_prec(w.add_prec_ref_val(BigExponentFloat::ONE, prec).0, prec)
        .0;
    w.sub_prec_ref_val(c, prec).0
}

// A seed for arguments outside the `f64` range, from l = ln|x|: x itself for W_0 of a tiny x, and
// otherwise `LOG_SEED_STEPS` Newton steps for w + ln|w| = l from l - ln|l|.
//
// This is equivalent to `logSeed` from `AzFloat/LambertWBracket.lean`, Azurite.
fn log_seed(branch0: bool, x: &BigExponentFloat) -> Option<BigExponentFloat> {
    let abs_x = x.abs();
    let rough_l = abs_x.ln_prec_ref(64).0;
    if branch0 && rough_l < -1i32 {
        Some(x.clone())
    } else if (branch0 && rough_l > 1u32) || (!branch0 && rough_l < -1i32) {
        let prec = 64
            + match rough_l.get_exponent() {
                Some(e) if *e >= 0u32 => u64::exact_from(e),
                _ => 0,
            };
        let l = abs_x.ln_prec(prec).0;
        let mut w = l.sub_prec_ref_val((&l).abs().ln_prec(prec).0, prec).0;
        for _ in 0..LOG_SEED_STEPS {
            w = log_seed_step(&l, &w, prec);
        }
        Some(w)
    } else {
        // This function is only called for arguments that `f64` cannot represent, whose logarithms
        // are far from [-1, 1], unless the `f64` iteration fails.
        fail_on_untested_path("log_seed, no seed");
        None
    }
}

// The branch-point series seed -1 + p - p^2/3 + 11p^3/72 - 43p^4/540 with p = ±sqrt(2q).
//
// This is equivalent to `seriesSeed` from `AzFloat/LambertWBracket.lean`, Azurite.
fn series_seed(branch0: bool, q_lo: &BigExponentFloat, prec: u64) -> BigExponentFloat {
    let p = (q_lo << 1u32).sqrt_prec(prec).0;
    let p = if branch0 { p } else { -p };
    let p2 = p.square_prec_ref(prec).0;
    let p3 = p2.mul_prec_ref_ref(&p, prec).0;
    let p4 = p2.square_prec_ref(prec).0;
    let t2 = p2
        .div_prec(const { BigExponentFloat::const_from_unsigned(3) }, prec)
        .0;
    let t3 = p3
        .mul_prec(const { BigExponentFloat::const_from_unsigned(11) }, prec)
        .0
        .div_prec(const { BigExponentFloat::const_from_unsigned(72) }, prec)
        .0;
    let t4 = p4
        .mul_prec(const { BigExponentFloat::const_from_unsigned(43) }, prec)
        .0
        .div_prec(const { BigExponentFloat::const_from_unsigned(540) }, prec)
        .0;
    BigExponentFloat::NEGATIVE_ONE
        .add_prec(p, prec)
        .0
        .sub_prec(t2, prec)
        .0
        .add_prec(t3, prec)
        .0
        .sub_prec(t4, prec)
        .0
}

// The candidate clamped into an interval (its lower end when the candidate is not finite).
//
// This is equivalent to `clampInto` from `AzFloat/LambertWBracket.lean`, Azurite.
fn clamp_into(interval: &Bracket, n: BigExponentFloat) -> BigExponentFloat {
    if n.is_finite() {
        max_of(interval.0.clone(), min_of(interval.1.clone(), n))
    } else {
        fail_on_untested_path("clamp_into, candidate not finite");
        interval.0.clone()
    }
}

// The integer bits of the root: the larger exponent of the ends of the a priori interval, when
// positive. w * e^w is evaluated at the candidate, and an error of epsilon in w is a relative error
// of epsilon in e^w, so the working precision has to cover these bits.
//
// This is equivalent to `lambertIntBits` from `AzFloat/LambertWBracket.lean`, Azurite.
fn lambert_int_bits(apriori: &Bracket) -> u64 {
    let int_bits = |x: &BigExponentFloat| match x.get_exponent() {
        Some(e) if *e > 0u32 => u64::exact_from(e),
        _ => 0,
    };
    max(int_bits(&apriori.0), int_bits(&apriori.1))
}

// The unverified candidate for W at working precision `w_prec`: a seed refined by Halley's
// iteration. The seed is the branch-point series when q < 2^(-40), and otherwise the `f64` seed,
// the logarithmic seed, or the midpoint of the a priori interval, whichever exists first. The
// schedule of the iteration counts the bits after the binary point, and every step carries the
// integer bits of the root on top of them.
//
// This is equivalent to `halleyCandidate` from `AzFloat/LambertWBracket.lean`, Azurite.
fn halley_candidate(
    branch0: bool,
    target: &LambertTarget,
    q: Option<&Bracket>,
    apriori: &Bracket,
    w_prec: u64,
) -> BigExponentFloat {
    let loss = (comp_bits(q) << 1) + 16;
    let int_bits = lambert_int_bits(apriori);
    let x = target.lower(w_prec + loss);
    let seed = match q {
        // q_lo < 2^(-40)
        Some((q_lo, _)) if q_lo.get_exponent().is_some_and(|e| *e <= -40i32) => {
            series_seed(branch0, q_lo, w_prec)
        }
        _ => seed_float(branch0, &x)
            .or_else(|| log_seed(branch0, &x))
            .unwrap_or_else(|| apriori.0.add_prec_ref_ref(&apriori.1, 64).0 >> 1u32),
    };
    let fraction_prec = w_prec - int_bits;
    let start_prec = fraction_prec >> ((fraction_prec >> 6).significant_bits() - 1);
    let n = halley_refine(
        &x,
        clamp_into(apriori, seed),
        start_prec,
        fraction_prec,
        loss + int_bits,
    );
    clamp_into(apriori, n)
}

// The bracket of the root at Ziv level `w`: three mean-value rounds around a candidate when they
// bring the width down to max(|a|, |b|) * 2^(-w - 1), where [a, b] is the a priori interval, and a
// bisection of the a priori interval otherwise. The working precision is `w` + 10, plus the extra
// bits near the branch point and the integer bits of the root, and at least 64 before the latter
// are added.
//
// This is equivalent to `lambertBracket` (with `lambertWork` and `lambertTarget`) from
// `AzFloat/LambertWBracket.lean`, Azurite.
fn lambert_bracket(
    branch0: bool,
    target: &LambertTarget,
    q: Option<&Bracket>,
    apriori: &Bracket,
    w: u64,
) -> Bracket {
    let w_prec = max(w + 10 + comp_bits(q), 64) + lambert_int_bits(apriori);
    let n = halley_candidate(branch0, target, q, apriori, w_prec);
    let interval = mvt_rounds(branch0, target, &n, apriori, w_prec);
    let width = interval
        .1
        .sub_prec_round_ref_ref(&interval.0, 64, Ceiling)
        .0;
    if width <= max_of((&apriori.0).abs(), (&apriori.1).abs()) >> (w + 1) {
        interval
    } else {
        // This is the fallback that makes the bracket certain to be narrow enough whatever the
        // candidate is. No argument is known for which the candidate is not good enough.
        fail_on_untested_path("lambert_bracket, bisection fallback");
        bisect_grid(branch0, target, 3 * w_prec + 8, apriori)
    }
}

// Ziv's loop for W_0 of a positive target.
//
// This is equivalent to `lambertW0Pos` from `AzFloat/LambertW.lean`, Azurite.
fn lambert_w0_pos(
    target: &LambertTarget,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(rm, Exact, "Inexact Lambert W function");
    let apriori = apriori_w0_pos(target, 64);
    ziv_loop_certified(
        &|w| lambert_bracket(true, target, None, &apriori, w),
        prec,
        rm,
    )
}

// Ziv's loop for W_0 (when `branch0` is set) or W_{-1} of a negative target (NaN below -1/e).
//
// This is equivalent to `lambertW0Neg` and `lambertWm1Neg` from `AzFloat/LambertW.lean`, Azurite.
pub(crate) fn lambert_w_neg(
    branch0: bool,
    target: &LambertTarget,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let Some(q) = q_encl(target) else {
        return (BigExponentFloat::NAN, Equal);
    };
    assert_ne!(rm, Exact, "Inexact Lambert W function");
    let q = q_tight(target, q);
    let apriori_prec = 64 + (comp_bits(Some(&q)) << 1);
    let apriori = if branch0 {
        apriori_w0_neg(&q, apriori_prec)
    } else {
        apriori_wm1(target, &q, apriori_prec)
    };
    ziv_loop_certified(
        &|w| lambert_bracket(branch0, target, Some(&q), &apriori, w),
        prec,
        rm,
    )
}

// W_0(x), rounded to `prec` bits with `rm`.
//
// This is equivalent to `lambertW0PrecRound` from `AzFloat/LambertW.lean`, Azurite.
pub(crate) fn lambert_w0_helper(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    let (sign, exponent, significand) = match x {
        BigExponentFloat(NaN | Infinity { sign: false }) => return (BigExponentFloat::NAN, Equal),
        BigExponentFloat(Infinity { sign: true }) => return (BigExponentFloat::INFINITY, Equal),
        BigExponentFloat(Zero) => return (BigExponentFloat::ZERO, Equal),
        BigExponentFloat(Finite {
            sign,
            exponent,
            significand,
        }) => (*sign, exponent, significand),
    };
    let nx = -x;
    if let Some(r) = lambert_w0_tiny(
        &|p, m| nx.round_to_prec_round_ref(p, m),
        &nx,
        exponent,
        max(prec, significand_prec(significand)) + ZIV_GUARD_BITS,
        prec,
        rm,
    ) {
        return r;
    }
    let target = LambertTarget::BigExponentFloat(x);
    if sign {
        lambert_w0_pos(&target, prec, rm)
    } else {
        lambert_w_neg(true, &target, prec, rm)
    }
}

// W_0(q) for a `Rational` q, rounded to `prec` bits with `rm`.
//
// This is equivalent to `lambertW0RatPrecRound` from `AzFloat/LambertW.lean`, Azurite.
pub(crate) fn lambert_w0_rational_helper(
    q: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    if *q == 0u32 {
        return (BigExponentFloat::ZERO, Equal);
    }
    let bound = rational_exponent_bound(q);
    let w = prec + ZIV_GUARD_BITS;
    // The tiny path applies under this condition, which is tested here as well to avoid converting
    // the argument when it does not hold.
    if bound <= -Integer::from(w + 8) {
        let nq = -q;
        if let Some(r) = lambert_w0_tiny(
            &|p, m| BigExponentFloat::from_rational_prec_round_ref(&nq, p, m),
            &BigExponentFloat::from_rational_prec_round_ref(&nq, w + 8, Ceiling).0,
            &bound,
            w,
            prec,
            rm,
        ) {
            return r;
        }
    }
    let target = LambertTarget::Rational(q);
    if *q > 0u32 {
        lambert_w0_pos(&target, prec, rm)
    } else {
        lambert_w_neg(true, &target, prec, rm)
    }
}

impl BigExponentFloat {
    /// Computes $W_0(x)$, the principal branch of the Lambert $W$ function, of a
    /// [`BigExponentFloat`], rounding the result to the specified precision and with the specified
    /// rounding mode. The [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $W_0(x)$ is the solution $w \geq -1$ of $we^w = x$, which exists when $x \geq -1/e$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = W_0(x)+\varepsilon.
    /// $$
    /// - If $W_0(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $W_0(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |W_0(x)|\rfloor-p+1}$.
    /// - If $W_0(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |W_0(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=\text{NaN}$
    /// - $f(0,p,m)=0$
    /// - $f(x,p,m)=\text{NaN}$ if $x<-1/e$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::lambert_w0_prec_round), this function never
    /// underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::lambert_w0_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m)^{3/2} \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O((n + m) \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the exponent of $x$.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case whenever the result is finite and nonzero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = BigExponentFloat::ONE.lambert_w0_prec_round(5, Floor);
    /// assert_eq!(w.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::ONE.lambert_w0_prec_round(5, Ceiling);
    /// assert_eq!(w.to_string(), "0.594");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::ONE.lambert_w0_prec_round(5, Nearest);
    /// assert_eq!(w.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::ONE.lambert_w0_prec_round(20, Floor);
    /// assert_eq!(w.to_string(), "0.5671425");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::ONE.lambert_w0_prec_round(20, Ceiling);
    /// assert_eq!(w.to_string(), "0.56714344");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::ONE.lambert_w0_prec_round(20, Nearest);
    /// assert_eq!(w.to_string(), "0.56714344");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn lambert_w0_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        lambert_w0_helper(&self, prec, rm)
    }

    /// Computes $W_0(x)$, the principal branch of the Lambert $W$ function, of a
    /// [`BigExponentFloat`], rounding the result to the specified precision and with the specified
    /// rounding mode. The [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $W_0(x)$ is the solution $w \geq -1$ of $we^w = x$, which exists when $x \geq -1/e$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = W_0(x)+\varepsilon.
    /// $$
    /// - If $W_0(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $W_0(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |W_0(x)|\rfloor-p+1}$.
    /// - If $W_0(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |W_0(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=\text{NaN}$
    /// - $f(0,p,m)=0$
    /// - $f(x,p,m)=\text{NaN}$ if $x<-1/e$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::lambert_w0_prec_round), this function never
    /// underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::lambert_w0_prec_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m)^{3/2} \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O((n + m) \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the exponent of $x$.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case whenever the result is finite and nonzero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = BigExponentFloat::ONE.lambert_w0_prec_round_ref(5, Floor);
    /// assert_eq!(w.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::ONE.lambert_w0_prec_round_ref(5, Ceiling);
    /// assert_eq!(w.to_string(), "0.594");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::ONE.lambert_w0_prec_round_ref(5, Nearest);
    /// assert_eq!(w.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::ONE.lambert_w0_prec_round_ref(20, Floor);
    /// assert_eq!(w.to_string(), "0.5671425");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::ONE.lambert_w0_prec_round_ref(20, Ceiling);
    /// assert_eq!(w.to_string(), "0.56714344");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::ONE.lambert_w0_prec_round_ref(20, Nearest);
    /// assert_eq!(w.to_string(), "0.56714344");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn lambert_w0_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        lambert_w0_helper(self, prec, rm)
    }

    /// Computes $W_0(x)$, the principal branch of the Lambert $W$ function, of a
    /// [`BigExponentFloat`] in place, rounding the result to the specified precision and with the
    /// specified rounding mode. An [`Ordering`] is returned, indicating whether the rounded value
    /// is less than, equal to, or greater than the exact value. Although `NaN`s are not comparable
    /// to any [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN`
    /// it also returns `Equal`.
    ///
    /// $W_0(x)$ is the solution $w \geq -1$ of $we^w = x$, which exists when $x \geq -1/e$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets W_0(x)+\varepsilon.
    /// $$
    /// - If $W_0(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $W_0(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |W_0(x)|\rfloor-p+1}$.
    /// - If $W_0(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |W_0(x)|\rfloor-p}$.
    ///
    /// See the [`BigExponentFloat::lambert_w0_prec_round`] documentation for information on special
    /// cases.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::lambert_w0_prec_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m)^{3/2} \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O((n + m) \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the exponent of $x$.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case whenever the result is finite and nonzero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.lambert_w0_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "0.562");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.lambert_w0_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.594");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.lambert_w0_prec_round_assign(5, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.562");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.lambert_w0_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "0.5671425");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.lambert_w0_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.56714344");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.lambert_w0_prec_round_assign(20, Nearest), Greater);
    /// assert_eq!(x.to_string(), "0.56714344");
    /// ```
    #[inline]
    pub fn lambert_w0_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let (mut w, o) = lambert_w0_helper(self, prec, rm);
        swap(self, &mut w);
        o
    }

    /// Computes $W_0(x)$, the principal branch of the Lambert $W$ function, of a
    /// [`BigExponentFloat`], rounding the result to the nearest value of the specified precision.
    /// The [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded value is less than, equal to, or greater than the exact value. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $W_0(x)$ is the solution $w \geq -1$ of $we^w = x$, which exists when $x \geq -1/e$.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = W_0(x)+\varepsilon.
    /// $$
    /// - If $W_0(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $W_0(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |W_0(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=\text{NaN}$
    /// - $f(0,p)=0$
    /// - $f(x,p)=\text{NaN}$ if $x<-1/e$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::lambert_w0_prec_round), this function never
    /// underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::lambert_w0_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m)^{3/2} \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O((n + m) \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the exponent of $x$.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = BigExponentFloat::ONE.lambert_w0_prec(5);
    /// assert_eq!(w.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::ONE.lambert_w0_prec(20);
    /// assert_eq!(w.to_string(), "0.56714344");
    /// assert_eq!(o, Greater);
    ///
    /// let x = BigExponentFloat::from(1.0e10).exp_prec(10).0;
    /// let (w, o) = x.lambert_w0_prec(20);
    /// assert_eq!(w.to_string(), "1.00000e10");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn lambert_w0_prec(self, prec: u64) -> (Self, Ordering) {
        self.lambert_w0_prec_round(prec, Nearest)
    }

    /// Computes $W_0(x)$, the principal branch of the Lambert $W$ function, of a
    /// [`BigExponentFloat`], rounding the result to the nearest value of the specified precision.
    /// The [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded value is less than, equal to, or greater than the exact value. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $W_0(x)$ is the solution $w \geq -1$ of $we^w = x$, which exists when $x \geq -1/e$.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = W_0(x)+\varepsilon.
    /// $$
    /// - If $W_0(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $W_0(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |W_0(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=\text{NaN}$
    /// - $f(0,p)=0$
    /// - $f(x,p)=\text{NaN}$ if $x<-1/e$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::lambert_w0_prec_round), this function never
    /// underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::lambert_w0_prec_round_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m)^{3/2} \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O((n + m) \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the exponent of $x$.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = BigExponentFloat::ONE.lambert_w0_prec_ref(5);
    /// assert_eq!(w.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::ONE.lambert_w0_prec_ref(20);
    /// assert_eq!(w.to_string(), "0.56714344");
    /// assert_eq!(o, Greater);
    ///
    /// let x = BigExponentFloat::from(1.0e10).exp_prec(10).0;
    /// let (w, o) = x.lambert_w0_prec_ref(20);
    /// assert_eq!(w.to_string(), "1.00000e10");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn lambert_w0_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.lambert_w0_prec_round_ref(prec, Nearest)
    }

    /// Computes $W_0(x)$, the principal branch of the Lambert $W$ function, of a
    /// [`BigExponentFloat`] in place, rounding the result to the nearest value of the specified
    /// precision. An [`Ordering`] is returned, indicating whether the rounded value is less than,
    /// equal to, or greater than the exact value. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// $W_0(x)$ is the solution $w \geq -1$ of $we^w = x$, which exists when $x \geq -1/e$.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets W_0(x)+\varepsilon.
    /// $$
    /// - If $W_0(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $W_0(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |W_0(x)|\rfloor-p}$.
    ///
    /// See the [`BigExponentFloat::lambert_w0_prec_round`] documentation for information on special
    /// cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::lambert_w0_prec_round_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m)^{3/2} \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O((n + m) \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the exponent of $x$.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.lambert_w0_prec_assign(5), Less);
    /// assert_eq!(x.to_string(), "0.562");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.lambert_w0_prec_assign(20), Greater);
    /// assert_eq!(x.to_string(), "0.56714344");
    ///
    /// let mut x = BigExponentFloat::from(1.0e10).exp_prec(10).0;
    /// assert_eq!(x.lambert_w0_prec_assign(20), Greater);
    /// assert_eq!(x.to_string(), "1.00000e10");
    /// ```
    #[inline]
    pub fn lambert_w0_prec_assign(&mut self, prec: u64) -> Ordering {
        self.lambert_w0_prec_round_assign(prec, Nearest)
    }

    /// Computes $W_0(x)$, the principal branch of the Lambert $W$ function, of a [`Rational`],
    /// rounding the result to the specified precision and with the specified rounding mode and
    /// returning the result as a [`BigExponentFloat`]. The [`Rational`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded value is less than, equal to,
    /// or greater than the exact value. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $W_0(x)$ is the solution $w \geq -1$ of $we^w = x$, which exists when $x \geq -1/e$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = W_0(x)+\varepsilon.
    /// $$
    /// - If $W_0(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $W_0(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |W_0(x)|\rfloor-p+1}$.
    /// - If $W_0(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |W_0(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p,m)=0$
    /// - $f(x,p,m)=\text{NaN}$ if $x<-1/e$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::lambert_w0_rational_prec_round), this
    /// function never underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::lambert_w0_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case whenever the result is finite and nonzero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = BigExponentFloat::lambert_w0_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(w.to_string(), "0.391");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::lambert_w0_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(w.to_string(), "0.406");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::lambert_w0_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(w.to_string(), "0.406");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::lambert_w0_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(w.to_string(), "0.40156317");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::lambert_w0_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(w.to_string(), "0.4015636");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::lambert_w0_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(w.to_string(), "0.4015636");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn lambert_w0_rational_prec_round(
        x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        lambert_w0_rational_helper(&x, prec, rm)
    }

    /// Computes $W_0(x)$, the principal branch of the Lambert $W$ function, of a [`Rational`],
    /// rounding the result to the specified precision and with the specified rounding mode and
    /// returning the result as a [`BigExponentFloat`]. The [`Rational`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded value is less than, equal to,
    /// or greater than the exact value. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $W_0(x)$ is the solution $w \geq -1$ of $we^w = x$, which exists when $x \geq -1/e$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = W_0(x)+\varepsilon.
    /// $$
    /// - If $W_0(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $W_0(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |W_0(x)|\rfloor-p+1}$.
    /// - If $W_0(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |W_0(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p,m)=0$
    /// - $f(x,p,m)=\text{NaN}$ if $x<-1/e$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::lambert_w0_rational_prec_round), this
    /// function never underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::lambert_w0_rational_prec_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case whenever the result is finite and nonzero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = BigExponentFloat::lambert_w0_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(w.to_string(), "0.391");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::lambert_w0_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(w.to_string(), "0.406");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::lambert_w0_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(w.to_string(), "0.406");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::lambert_w0_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(w.to_string(), "0.40156317");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::lambert_w0_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(w.to_string(), "0.4015636");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::lambert_w0_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(w.to_string(), "0.4015636");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn lambert_w0_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        lambert_w0_rational_helper(x, prec, rm)
    }

    /// Computes $W_0(x)$, the principal branch of the Lambert $W$ function, of a [`Rational`],
    /// rounding the result to the nearest value of the specified precision and returning the result
    /// as a [`BigExponentFloat`]. The [`Rational`] is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $W_0(x)$ is the solution $w \geq -1$ of $we^w = x$, which exists when $x \geq -1/e$.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = W_0(x)+\varepsilon.
    /// $$
    /// - If $W_0(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $W_0(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |W_0(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p)=0$
    /// - $f(x,p)=\text{NaN}$ if $x<-1/e$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::lambert_w0_rational_prec_round), this
    /// function never underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::lambert_w0_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) =
    ///     BigExponentFloat::lambert_w0_rational_prec(Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(w.to_string(), "0.406");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) =
    ///     BigExponentFloat::lambert_w0_rational_prec(Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(w.to_string(), "0.4015636");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn lambert_w0_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::lambert_w0_rational_prec_round(x, prec, Nearest)
    }

    /// Computes $W_0(x)$, the principal branch of the Lambert $W$ function, of a [`Rational`],
    /// rounding the result to the nearest value of the specified precision and returning the result
    /// as a [`BigExponentFloat`]. The [`Rational`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $W_0(x)$ is the solution $w \geq -1$ of $we^w = x$, which exists when $x \geq -1/e$.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = W_0(x)+\varepsilon.
    /// $$
    /// - If $W_0(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $W_0(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |W_0(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p)=0$
    /// - $f(x,p)=\text{NaN}$ if $x<-1/e$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::lambert_w0_rational_prec_round), this
    /// function never underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::lambert_w0_rational_prec_round_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) =
    ///     BigExponentFloat::lambert_w0_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(w.to_string(), "0.406");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) =
    ///     BigExponentFloat::lambert_w0_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(w.to_string(), "0.4015636");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn lambert_w0_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::lambert_w0_rational_prec_round_ref(x, prec, Nearest)
    }
}
