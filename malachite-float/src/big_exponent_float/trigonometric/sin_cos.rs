// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::{Finite, Infinity, NaN, Zero};
use crate::big_exponent_float::arithmetic::mul_rational::{
    finite_to_float, finite_to_integer_and_shift, float_rm, from_float_result,
};
use crate::big_exponent_float::exponential::exp::{round_nudged, ziv_loop};
use alloc::borrow::Cow;
use core::cmp::Ordering::{self, *};
use core::cmp::min;
use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::traits::{
    NaN as NaNTrait, NegativeOne, One, OneHalf, Two, Zero as ZeroTrait,
};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::exponential::traits::CeilingLogBase2;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::num::modular::traits::{DivisibleBy, Mod, ModPow, ModPowerOf2};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_q::Rational;

crate_test_enum! {
    // The six forward trigonometric functions, which share one implementation.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    TrigFunction {
        Sin,
        Cos,
        Tan,
        Cot,
        Sec,
        Csc,
    }
}

impl TrigFunction {
    pub(crate) const fn name(self) -> &'static str {
        match self {
            Self::Sin => "sin",
            Self::Cos => "cos",
            Self::Tan => "tan",
            Self::Cot => "cot",
            Self::Sec => "sec",
            Self::Csc => "csc",
        }
    }

    // The function at the angle 0. The cotangent and cosecant have a pole there; as a
    // `BigExponentFloat` has no signed zero, the two one-sided limits (-infinity and infinity)
    // cannot be told apart, and the result is NaN, as 1/0 is in `arf_div`.
    const fn at_zero(self) -> BigExponentFloat {
        match self {
            Self::Sin | Self::Tan => BigExponentFloat::ZERO,
            Self::Cos | Self::Sec => BigExponentFloat::ONE,
            Self::Cot | Self::Csc => BigExponentFloat::NAN,
        }
    }

    // The function at the angle 2πj/24, if it is rational (by Niven's theorem, the rational values
    // of the sine and cosine at rational multiples of π are 0, ±1/2, and ±1, and those of the
    // tangent are 0 and ±1; the secant and cosecant are rational exactly where the cosine and sine
    // are), or `None`. At a pole the result is NaN: every pole is a zero of a sine or cosine, and
    // the two one-sided limits there have opposite signs.
    fn at_24th_of_turn(self, j: u64) -> Option<BigExponentFloat> {
        // the sine and cosine, in halves
        let sin_halves = |j| match j {
            0 | 12 => Some(0),
            2 | 10 => Some(1),
            6 => Some(2),
            14 | 22 => Some(-1),
            18 => Some(-2),
            _ => None,
        };
        let cos_halves = |j| sin_halves((j + 6) % 24);
        let halves_to_value = |h: i32| match h {
            0 => BigExponentFloat::ZERO,
            1 => BigExponentFloat::ONE_HALF,
            -1 => -BigExponentFloat::ONE_HALF,
            2 => BigExponentFloat::ONE,
            _ => BigExponentFloat::NEGATIVE_ONE,
        };
        match self {
            Self::Sin => sin_halves(j).map(halves_to_value),
            Self::Cos => cos_halves(j).map(halves_to_value),
            Self::Sec | Self::Csc => {
                let h = if self == Self::Sec {
                    cos_halves(j)
                } else {
                    sin_halves(j)
                }?;
                // 1/(h/2) = 2/h
                Some(match h {
                    0 => BigExponentFloat::NAN,
                    1 => BigExponentFloat::TWO,
                    -1 => -BigExponentFloat::TWO,
                    2 => BigExponentFloat::ONE,
                    _ => BigExponentFloat::NEGATIVE_ONE,
                })
            }
            Self::Tan | Self::Cot => {
                if !j.divisible_by(3) {
                    return None;
                }
                // tan at 0, 1/8, 1/4, 3/8 of a turn: 0, 1, pole, -1
                let k = (j / 3) % 4;
                let k = if self == Self::Cot { (k + 2) % 4 } else { k };
                Some(match k {
                    0 => BigExponentFloat::ZERO,
                    1 => {
                        if self == Self::Cot {
                            BigExponentFloat::NEGATIVE_ONE
                        } else {
                            BigExponentFloat::ONE
                        }
                    }
                    2 => BigExponentFloat::NAN,
                    _ => {
                        if self == Self::Cot {
                            BigExponentFloat::ONE
                        } else {
                            BigExponentFloat::NEGATIVE_ONE
                        }
                    }
                })
            }
        }
    }

    fn float(self, x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
        match self {
            Self::Sin => x.sin_prec_round_ref(prec, rm),
            Self::Cos => x.cos_prec_round_ref(prec, rm),
            Self::Tan => x.tan_prec_round_ref(prec, rm),
            Self::Cot => x.cot_prec_round_ref(prec, rm),
            Self::Sec => x.sec_prec_round_ref(prec, rm),
            Self::Csc => x.csc_prec_round_ref(prec, rm),
        }
    }

    fn float_rational(self, x: &Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
        match self {
            Self::Sin => Float::sin_rational_prec_round_ref(x, prec, rm),
            Self::Cos => Float::cos_rational_prec_round_ref(x, prec, rm),
            Self::Tan => Float::tan_rational_prec_round_ref(x, prec, rm),
            Self::Cot => Float::cot_rational_prec_round_ref(x, prec, rm),
            Self::Sec => Float::sec_rational_prec_round_ref(x, prec, rm),
            Self::Csc => Float::csc_rational_prec_round_ref(x, prec, rm),
        }
    }

    fn float_with_period(
        self,
        x: &Float,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        match self {
            Self::Sin => x.sin_with_period_prec_round_ref(u, prec, rm),
            Self::Cos => x.cos_with_period_prec_round_ref(u, prec, rm),
            Self::Tan => x.tan_with_period_prec_round_ref(u, prec, rm),
            Self::Cot => x.cot_with_period_prec_round_ref(u, prec, rm),
            Self::Sec => x.sec_with_period_prec_round_ref(u, prec, rm),
            Self::Csc => x.csc_with_period_prec_round_ref(u, prec, rm),
        }
    }

    fn float_with_period_rational(
        self,
        x: &Rational,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        match self {
            Self::Sin => Float::sin_with_period_rational_prec_round_ref(x, u, prec, rm),
            Self::Cos => Float::cos_with_period_rational_prec_round_ref(x, u, prec, rm),
            Self::Tan => Float::tan_with_period_rational_prec_round_ref(x, u, prec, rm),
            Self::Cot => Float::cot_with_period_rational_prec_round_ref(x, u, prec, rm),
            Self::Sec => Float::sec_with_period_rational_prec_round_ref(x, u, prec, rm),
            Self::Csc => Float::csc_with_period_rational_prec_round_ref(x, u, prec, rm),
        }
    }

    // The function at the angle qπ/2 + r, from s ≈ sin r and c ≈ cos r with relative errors
    // below 2^(-ks) and 2^(-kc), as an approximation and an `err` as in `ziv_loop`. A value with a
    // relative error below 2^(-k) (k >= 1) has an error below 2^(EXP - k + 1). A quotient a/b, or a
    // reciprocal 1/b, of values with relative errors α and β <= 1/2, rounded to `wp` bits, has a
    // relative error of at most α + 2β + 2^(-wp) (plus a second-order term), below 2^(-k) for k =
    // min(ka, kb - 1, wp) - 2.
    fn combine(
        self,
        q: u64,
        s: &BigExponentFloat,
        ks: u64,
        c: &BigExponentFloat,
        kc: u64,
        wp: u64,
    ) -> (BigExponentFloat, u64) {
        // sin and cos of the angle, each with its relative error bits
        let sin = match q {
            0 => (s.clone(), ks),
            1 => (c.clone(), kc),
            2 => (-s, ks),
            _ => (-c, kc),
        };
        let cos = match q {
            0 => (c.clone(), kc),
            1 => (-s, ks),
            2 => (-c, kc),
            _ => (s.clone(), ks),
        };
        let quotient_bits =
            |ka: u64, kb: u64| min(min(ka, kb.saturating_sub(1)), wp).saturating_sub(2);
        let (y, k) = match self {
            Self::Sin => sin,
            Self::Cos => cos,
            Self::Tan => (
                sin.0.div_prec_ref_ref(&cos.0, wp).0,
                quotient_bits(sin.1, cos.1),
            ),
            Self::Cot => (
                cos.0.div_prec_ref_ref(&sin.0, wp).0,
                quotient_bits(cos.1, sin.1),
            ),
            Self::Sec => (
                cos.0.reciprocal_prec_ref(wp).0,
                quotient_bits(u64::MAX, cos.1),
            ),
            Self::Csc => (
                sin.0.reciprocal_prec_ref(wp).0,
                quotient_bits(u64::MAX, sin.1),
            ),
        };
        (y, k.saturating_sub(1))
    }
}

// The result of one of `Float`'s periodic functions, with a pole (an exact infinity) replaced by
// NaN; see `TrigFunction::at_24th_of_turn`.
fn pole_to_nan((y, o): (Float, Ordering)) -> (Float, Ordering) {
    if y.is_infinite() && o == Equal {
        (Float::NAN, Equal)
    } else {
        (y, o)
    }
}

// Whether the exponent of the finite nonzero x is within `Float`'s exponent range, and if so, the
// exponent as an `i32`.
fn float_exponent(x: &BigExponentFloat) -> Option<i32> {
    let e = i32::try_from(x.get_exponent().unwrap()).ok()?;
    (Float::MIN_EXPONENT..=Float::MAX_EXPONENT)
        .contains(&e)
        .then_some(e)
}

// The result of one of `Float`'s functions, converted if it neither overflowed nor underflowed;
// with `Exact`, the `Float` function is called with `Floor`, and the result must then be exact.
fn from_float_trig(
    result: (Float, Ordering),
    f: TrigFunction,
    rm: RoundingMode,
) -> Option<(BigExponentFloat, Ordering)> {
    let (y, o) = from_float_result(result, None)?;
    assert!(
        rm != Exact || o == Equal,
        "Inexact BigExponentFloat {}",
        f.name()
    );
    Some((y, o))
}

// Approximations of sin r and cos r for 0 < |r| <= 4/5, from their Taylor series with every
// operation rounded to `wp` bits, and a k such that both relative errors are below 2^(-k).
//
// The k-th terms r^(2k)/(2k)! and r^(2k+1)/(2k+1)! are each formed with at most 3k + 1 roundings,
// so their relative errors are below (3k + 1)2^(1 - wp); the terms decrease so fast (by a factor of
// at most 0.64/12 after the first) that the weighted sum of these errors is below 2^(1 - wp) times
// the sum of the absolute values of the terms. Each of the n additions adds an error of at most
// 2^(-wp) times that sum, and so does the truncation, as the sum stops once a term is below 2^(-wp
// - 2). The sums of absolute values are at most cosh(4/5) < 1.34 and sinh(4/5) / (4/5) < 1.12,
// while cos r > 0.69 and sin r / r > 0.89, so both relative errors are below (n + 4) 2^(2 - wp).
fn sin_cos_series(r: &BigExponentFloat, wp: u64) -> (BigExponentFloat, BigExponentFloat, u64) {
    let r2 = r.square_prec_ref(wp).0;
    let mut cos_sum = BigExponentFloat::ONE;
    let mut sin_sum = BigExponentFloat::ONE;
    let mut term = BigExponentFloat::ONE;
    let limit = -Integer::from(wp + 2);
    let mut k: u64 = 1;
    loop {
        // term = r^(2k) / (2k)!
        term = term
            .mul_prec_ref_ref(&r2, wp)
            .0
            .div_prec_val_ref(&BigExponentFloat::from(((k << 1) - 1) * (k << 1)), wp)
            .0;
        let sin_term = term
            .div_prec_ref_ref(&BigExponentFloat::from((k << 1) + 1), wp)
            .0;
        let subtract = k.odd();
        cos_sum = if subtract {
            cos_sum.sub_prec_ref_ref(&term, wp).0
        } else {
            cos_sum.add_prec_ref_ref(&term, wp).0
        };
        sin_sum = if subtract {
            sin_sum.sub_prec_ref_ref(&sin_term, wp).0
        } else {
            sin_sum.add_prec_ref_ref(&sin_term, wp).0
        };
        if *term.get_exponent().unwrap() < limit {
            break;
        }
        k += 1;
    }
    let s = r.mul_prec_ref_ref(&sin_sum, wp).0;
    (
        s,
        cos_sum,
        wp.saturating_sub((k + 4).ceiling_log_base_2() + 2),
    )
}

// Approximations of sin r and cos r for 0 < |r| <= 4/5, where r is within a relative distance
// 2^(-kr) of the exact reduced argument r' (kr >= 2), and their relative-error bits. sin r differs
// from sin r' by a relative amount below (|r - r'| / |r'|)(|r'| / |sin r'|) < 1.12 * 2^(-kr), and
// cos r from cos r' by |r - r'| |sin r'| < 2^(-kr) |r'|^2 < 0.69 * 2^(-kr), relatively below
// 2^(-kr). sin r and cos r are computed with `sin_cos_series`. (`Float`'s functions are not used
// here: the arguments that reach this function either have exponents beyond `Float`'s range, whose
// reduction, needing billions of bits of π, dominates the cost, or are tiny, or come from the
// reference implementations.)
fn sin_cos_reduced(
    r: &BigExponentFloat,
    kr: u64,
    wp: u64,
) -> (BigExponentFloat, u64, BigExponentFloat, u64) {
    let (s, c, k) = sin_cos_series(r, wp);
    let k = min(k, kr.saturating_sub(1));
    (s, k.saturating_sub(1), c, k.saturating_sub(1))
}

crate_test_enum! {
    // The argument of a non-periodic trigonometric function.
    TrigArgument<'a> {
        BigExponentFloat(&'a BigExponentFloat),
        Rational(&'a Rational),
    }
}

impl TrigArgument<'_> {
    // The exponent e of the nonzero argument x, with 2^(e - 1) <= |x| < 2^e.
    fn exponent(&self) -> Integer {
        match self {
            TrigArgument::BigExponentFloat(x) => x.get_exponent().unwrap().clone(),
            TrigArgument::Rational(x) => Integer::from(x.floor_log_base_2_abs()) + Integer::ONE,
        }
    }

    // The argument, rounded to `prec` bits for a `Rational` (with a relative error of at most
    // 2^(-prec)).
    fn approx(&self, prec: u64) -> Cow<'_, BigExponentFloat> {
        match self {
            TrigArgument::BigExponentFloat(x) => Cow::Borrowed(x),
            TrigArgument::Rational(x) => {
                Cow::Owned(BigExponentFloat::from_rational_prec_ref(x, prec).0)
            }
        }
    }
}

// An approximation of f(x) for a nonzero x, and an `err` as in `ziv_loop`.
//
// If |x| < 1/2, there is no reduction: r = x, exactly for a `BigExponentFloat` and with a relative
// error of at most 2^(-wp - 10) for a `Rational`. Otherwise x = qπ/2 + r' with q = round(2x/π)
// (computed from approximations, so that |r'| <= (1/2 + 2^(-9))π/2 < 4/5): with e the exponent of
// x and lp = wp + e + 20, π/2 is rounded to lp bits, with an error below 2^(1 - lp); |q| < 2^(e +
// 1), so qπ/2 has an error below 2^(e + 2 - lp), and a `Rational` x rounded to lp + e + 2 bits
// adds less than 2^(-lp - 2). The reduced argument r is computed exactly from these, so |r - r'| <
// 2^(e + 3 - lp), a relative error below 2^(e + 4 - lp - EXP(r)). If r is zero, or so small that
// this bound is useless, the approximation fails the rounding test and the next working precision,
// which uses more bits of π, is tried.
//
// Reducing a huge x takes about e bits of π.
fn trig_approx(f: TrigFunction, x: &TrigArgument<'_>, wp: u64) -> (BigExponentFloat, u64) {
    let e = x.exponent();
    let (q, r, kr) = if e <= 0u32 {
        (0, x.approx(wp + 10).into_owned(), wp + 10)
    } else {
        let e = u64::exact_from(&e);
        let lp = wp + e + 20;
        let half_pi = BigExponentFloat::from(Float::pi_prec(lp).0) >> 1u32;
        let xa = x.approx(lp + e + 2);
        let quotient = xa.div_prec_ref_ref(&half_pi, e + 10).0;
        let n = Integer::rounding_from(&quotient, Nearest).0;
        let r = &*xa - BigExponentFloat::from(n.clone()) * half_pi;
        let BigExponentFloat(Finite { exponent: er, .. }) = &r else {
            return (r, 0);
        };
        // lp - e - 4 = wp + 16
        let kr = Integer::from(wp + 16) + er;
        let kr = if kr < 2u32 {
            return (r, 0);
        } else {
            min(u64::exact_from(&kr), wp + 10)
        };
        (u64::exact_from(&n.mod_power_of_2(2)), r, kr)
    };
    let (s, ks, c, kc) = sin_cos_reduced(&r, kr, wp + 10);
    f.combine(q, &s, ks, &c, kc, wp)
}

// For a nonzero x so small that f(x) is within a tiny fraction of an ulp of a simple exact value v,
// the pair (v, up): f(x) is v plus (if `up`) or minus a positive amount small enough for
// `round_nudged`, so that the rounding can be decided without a Ziv loop (which would need about
// 2|e| or 3|e| bits to separate f(x) from v, for x's exponent e). With e = EXP(x), so that |x| <
// 2^e, and q the precision of v, `round_nudged` needs the amount to be below 2^(EXP(v) - max(p, q)
// - 2); the condition 2e <= -max(p, q) - 4 gives this in each case:
// - sin x = x - (x^3/6)(1 - ...) and tan x = x + (x^3/3)(1 + ...), differing from x by less than
//   2^(3e);
// - cos x = 1 - x^2/2 + ... and sec x = 1 + x^2/2 + ..., differing from 1 by less than 2^(2e - 1);
// - cot x = 1/x - x/3 - ... and csc x = 1/x + x/6 + ..., differing from 1/x by less than |x|/2 <
//   2^(e - 1), when 1/x is exact, that is, when x is a power of 2 (otherwise 1/x is not a dyadic
//   number, and the Ziv loop terminates).
// Each sign follows from the series: sin x lies between 0 and x, tan x is beyond x, cos x < 1 < sec
// x, cot x lies between 0 and 1/x, and csc x is beyond 1/x.
fn trig_tiny_nudge(
    f: TrigFunction,
    x: &BigExponentFloat,
    prec: u64,
) -> Option<(BigExponentFloat, bool)> {
    let e = x.get_exponent().unwrap();
    let negative = *x < 0u32;
    let (v, up) = match f {
        TrigFunction::Sin => (x.clone(), negative),
        TrigFunction::Tan => (x.clone(), !negative),
        TrigFunction::Cos => (BigExponentFloat::ONE, false),
        TrigFunction::Sec => (BigExponentFloat::ONE, true),
        TrigFunction::Cot | TrigFunction::Csc => {
            if x.get_prec() != Some(1) {
                return None;
            }
            let v = x.reciprocal_prec_ref(1).0;
            (v, (f == TrigFunction::Csc) != negative)
        }
    };
    let q = v.get_prec().unwrap();
    if core::cmp::max(prec, q) + 4 <= -(e << 1u32) {
        Some((v, up))
    } else {
        None
    }
}

// f(x), rounded to `prec` bits with `rm`. The special values are those of `Float`'s functions,
// except at 0, where the cotangent and cosecant give NaN (see `TrigFunction::at_zero`). For a
// nonzero x the value is transcendental, so `Exact` is rejected. If x's exponent is within
// `Float`'s range, `Float`'s function is used unless its result overflows or underflows (which is
// possible only for an x with billions of bits, within 2^(-2^30) of a zero or pole); otherwise
// `trig_approx` is used in a Ziv loop.
pub(crate) fn trig_helper(
    f: TrigFunction,
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
    use_float: bool,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    match x {
        BigExponentFloat(NaN | Infinity { .. }) => return (BigExponentFloat::NAN, Equal),
        BigExponentFloat(Zero) => return (f.at_zero(), Equal),
        _ => {}
    }
    assert_ne!(rm, Exact, "Inexact BigExponentFloat {}", f.name());
    if use_float
        && let Some(e) = float_exponent(x)
        && let Some(result) = from_float_trig(
            f.float(&finite_to_float(Cow::Borrowed(x), e), prec, rm),
            f,
            rm,
        )
    {
        return result;
    }
    if let Some((v, up)) = trig_tiny_nudge(f, x, prec) {
        return round_nudged(&v, up, prec, rm);
    }
    ziv_loop(prec, rm, |wp| {
        trig_approx(f, &TrigArgument::BigExponentFloat(x), wp)
    })
}

// f(x) for a `Rational` x, rounded to `prec` bits with `rm`: as in `trig_helper`, with `Float`'s
// `Rational` function in place of its `Float` function.
pub(crate) fn trig_rational_helper(
    f: TrigFunction,
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
    use_float: bool,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    if *x == 0u32 {
        return (f.at_zero(), Equal);
    }
    assert_ne!(rm, Exact, "Inexact BigExponentFloat {}", f.name());
    if use_float && let Some(result) = from_float_trig(f.float_rational(x, prec, rm), f, rm) {
        return result;
    }
    ziv_loop(prec, rm, |wp| {
        trig_approx(f, &TrigArgument::Rational(x), wp)
    })
}

// x mod u, exactly, as a `Rational` in [0, u), for a finite nonzero x = m 2^k and u > 0: for k >=
// 0, (m mod u)(2^k mod u) mod u, with the power computed by modular exponentiation (so a huge
// exponent costs only its bits); otherwise (m mod u 2^(-k)) / 2^(-k).
fn finite_mod_unsigned(x: &BigExponentFloat, u: u64) -> Rational {
    let (m, k) = finite_to_integer_and_shift(x);
    let un = Natural::from(u);
    if k >= 0u32 {
        let base = Natural::from(2 % u);
        let p = base.mod_pow(Natural::exact_from(&k), &un);
        let mm = Natural::exact_from(m.mod_op(Integer::from(u)));
        Rational::from((mm * p).mod_op(&un))
    } else {
        let s = u64::exact_from(&-k);
        Rational::from(m.mod_op(Integer::from(un) << s)) >> s
    }
}

// The approximation of f(2πδ + qπ/2) for a nonzero `Rational` δ with |δ| <= 1/8 (given as δ,
// or as x/u for a `BigExponentFloat` x), and an `err` as in `ziv_loop`. 2πδ is formed from δ (or
// x/u) rounded to wp + 12 bits and 2π rounded to wp + 12 bits, so its relative error is below
// 2^(-wp - 9); |2πδ| <= π/4 < 4/5.
fn trig_turns_approx(
    f: TrigFunction,
    delta: TurnsDelta<'_>,
    q: u64,
    wp: u64,
) -> (BigExponentFloat, u64) {
    let p = wp + 12;
    let two_pi = BigExponentFloat::from(Float::pi_prec(p).0) << 1u32;
    let y = match delta {
        TurnsDelta::Rational(d) => {
            BigExponentFloat::from_rational_prec_ref(d, p)
                .0
                .mul_prec_val_ref(&two_pi, p)
                .0
        }
        TurnsDelta::OverU(x, u) => {
            x.mul_prec_ref_ref(&two_pi, p)
                .0
                .div_prec_val_ref(&BigExponentFloat::from(u), p)
                .0
        }
    };
    let (s, ks, c, kc) = sin_cos_reduced(&y, wp + 9, wp + 10);
    f.combine(q, &s, ks, &c, kc, wp)
}

#[derive(Clone, Copy)]
enum TurnsDelta<'a> {
    Rational(&'a Rational),
    OverU(&'a BigExponentFloat, u64),
}

// f(2πq/u) for a `Rational` q, rounded to `prec` bits with `rm`.
//
// The special values are those of `Float`'s functions, except that every pole gives NaN (see
// `TrigFunction::at_24th_of_turn`). If `use_float`, `Float`'s function is used unless its result
// overflows or underflows (possible only for a q with billions of bits, within 2^(-2^30) of a zero
// or pole). Otherwise, with t = (q mod u)/u in [0, 1) the angle in turns: the rational values (at
// multiples of 1/24 of a turn) are exact; and the others are irrational, and computed in a Ziv loop
// from t = k/4 + δ, with k = round(4t) and |δ| <= 1/8, as f(kπ/2 + 2πδ).
pub(crate) fn trig_with_period_rational_helper(
    f: TrigFunction,
    q: &Rational,
    u: u64,
    prec: u64,
    rm: RoundingMode,
    use_float: bool,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    if u == 0 {
        return (BigExponentFloat::NAN, Equal);
    }
    if *q == 0u32 {
        return (f.at_zero(), Equal);
    }
    if use_float
        && let Some(result) = from_float_trig(
            pole_to_nan(f.float_with_period_rational(q, u, prec, float_rm(rm))),
            f,
            rm,
        )
    {
        return result;
    }
    let ur = Rational::from(u);
    let t = q.mod_op(&ur) / ur;
    let t24 = &t * const { Rational::const_from_unsigned(24) };
    if t24.denominator_ref() == &1u32
        && let Some(y) = f.at_24th_of_turn(u64::exact_from(t24.numerator_ref()))
    {
        return y.round_to_prec_round(prec, rm);
    }
    assert_ne!(rm, Exact, "Inexact BigExponentFloat {}", f.name());
    let k = Integer::rounding_from(&(&t << 2u32), Nearest).0;
    let delta = t - (Rational::from(k.clone()) >> 2u32);
    let q4 = u64::exact_from(&k.mod_power_of_2(2));
    ziv_loop(prec, rm, |wp| {
        trig_turns_approx(f, TurnsDelta::Rational(&delta), q4, wp)
    })
}

// f(2πx/u) for a `BigExponentFloat` x, rounded to `prec` bits with `rm`: as in
// `trig_with_period_rational_helper`. If x's exponent is within `Float`'s range and `use_float`,
// `Float`'s function is tried first. Otherwise, if |x| < u/16 (when no angle in turns, x/u, is a
// nonzero multiple of 1/24, so the value is irrational), the angle is used directly, with δ = x/u
// and k = 0, which also covers a tiny x; and otherwise x is reduced modulo u exactly (see
// `finite_mod_unsigned`), which for a huge x costs only the bits of its exponent.
pub(crate) fn trig_with_period_helper(
    f: TrigFunction,
    x: &BigExponentFloat,
    u: u64,
    prec: u64,
    rm: RoundingMode,
    use_float: bool,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    match x {
        BigExponentFloat(NaN | Infinity { .. }) => return (BigExponentFloat::NAN, Equal),
        _ if u == 0 => return (BigExponentFloat::NAN, Equal),
        BigExponentFloat(Zero) => return (f.at_zero(), Equal),
        _ => {}
    }
    if use_float
        && let Some(e) = float_exponent(x)
        && let Some(result) = from_float_trig(
            pole_to_nan(f.float_with_period(
                &finite_to_float(Cow::Borrowed(x), e),
                u,
                prec,
                float_rm(rm),
            )),
            f,
            rm,
        )
    {
        return result;
    }
    if *x.get_exponent().unwrap() <= i64::exact_from(u.significant_bits()) - 5 {
        assert_ne!(rm, Exact, "Inexact BigExponentFloat {}", f.name());
        // y = 2πx/u satisfies |y| < 2^(EXP(x) + 3)/u <= 2^(EXP(x) + 4 - bits(u)), so cos y and sec
        // y differ from 1 by less than 2^(2 EXP(y) - 1) (see `trig_tiny_nudge`).
        if matches!(f, TrigFunction::Cos | TrigFunction::Sec) {
            let ey = x.get_exponent().unwrap()
                + Integer::from(4 - i64::exact_from(u.significant_bits()));
            if prec + 4 <= -(ey << 1u32) {
                return round_nudged(&BigExponentFloat::ONE, f == TrigFunction::Sec, prec, rm);
            }
        }
        return ziv_loop(prec, rm, |wp| {
            trig_turns_approx(f, TurnsDelta::OverU(x, u), 0, wp)
        });
    }
    trig_with_period_rational_helper(f, &finite_mod_unsigned(x, u), u, prec, rm, use_float)
}

crate_test_fn! {
    // f(x) rounded to `prec` bits with `rm`, without `Float`'s trigonometric functions: the
    // argument is reduced with `trig_approx`, and the sine and cosine of the reduced argument are
    // computed with `sin_cos_series`. Used to check and benchmark `trig_helper`.
    trig_prec_round_generic(
        f: TrigFunction,
        x: &BigExponentFloat,
        prec: u64,
        rm: RoundingMode,
    ) -> (BigExponentFloat, Ordering) {
        trig_helper(f, x, prec, rm, false)
    }
}

crate_test_fn! {
    // f(x) for a `Rational` x, as in `trig_prec_round_generic`.
    trig_rational_prec_round_generic(
        f: TrigFunction,
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (BigExponentFloat, Ordering) {
        trig_rational_helper(f, x, prec, rm, false)
    }
}

crate_test_fn! {
    // f(2πx/u) rounded to `prec` bits with `rm`, without `Float`'s trigonometric functions: exact
    // values come from `TrigFunction::at_24th_of_turn`, and the others from `sin_cos_series`.
    trig_with_period_prec_round_generic(
        f: TrigFunction,
        x: &BigExponentFloat,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (BigExponentFloat, Ordering) {
        trig_with_period_helper(f, x, u, prec, rm, false)
    }
}

crate_test_fn! {
    // f(2πq/u) for a `Rational` q, as in `trig_with_period_prec_round_generic`.
    trig_with_period_rational_prec_round_generic(
        f: TrigFunction,
        q: &Rational,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (BigExponentFloat, Ordering) {
        trig_with_period_rational_helper(f, q, u, prec, rm, false)
    }
}

impl BigExponentFloat {
    /// Computes the sine and cosine of a [`BigExponentFloat`], rounding the results to the
    /// specified precision and with the specified rounding mode. The [`BigExponentFloat`] is taken
    /// by value. Two [`Ordering`]s are also returned, indicating whether the rounded sine and
    /// cosine are less than, equal to, or greater than the exact values. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal` for it.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = (\sin x+\varepsilon_1, \cos x+\varepsilon_2).
    /// $$
    /// The errors $\varepsilon_1$ and $\varepsilon_2$ are bounded as for
    /// [`BigExponentFloat::sin_prec_round`] and [`BigExponentFloat::cos_prec_round`].
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=f(\pm\infty,p,m)=(\text{NaN},\text{NaN})$
    /// - $f(0,p,m)=(0,1)$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::sin_cos_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// Warning: an argument with exponent $e > 0$ is reduced modulo $\pi/2$, which takes about $e$
    /// bits of $\pi$. The time and memory needed therefore grow with the exponent, without any
    /// limit: an argument near $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\pi$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::sin_cos_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of the argument (0 if it is negative).
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the argument is finite and nonzero (the
    /// sine and cosine of a nonzero rational number are transcendental).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::ONE.sin_cos_prec_round(5, Floor);
    /// assert_eq!(s.to_string(), "0.812");
    /// assert_eq!(c.to_string(), "0.531");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::ONE.sin_cos_prec_round(5, Ceiling);
    /// assert_eq!(s.to_string(), "0.844");
    /// assert_eq!(c.to_string(), "0.562");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::ONE.sin_cos_prec_round(5, Nearest);
    /// assert_eq!(s.to_string(), "0.844");
    /// assert_eq!(c.to_string(), "0.531");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::ONE.sin_cos_prec_round(20, Floor);
    /// assert_eq!(s.to_string(), "0.8414707");
    /// assert_eq!(c.to_string(), "0.5403023");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::ONE.sin_cos_prec_round(20, Ceiling);
    /// assert_eq!(s.to_string(), "0.84147167");
    /// assert_eq!(c.to_string(), "0.54030323");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::ONE.sin_cos_prec_round(20, Nearest);
    /// assert_eq!(s.to_string(), "0.8414707");
    /// assert_eq!(c.to_string(), "0.5403023");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    /// ```
    #[inline]
    pub fn sin_cos_prec_round(
        self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Self, Ordering, Ordering) {
        let (s, o_s) = trig_helper(TrigFunction::Sin, &self, prec, rm, true);
        let (c, o_c) = trig_helper(TrigFunction::Cos, &self, prec, rm, true);
        (s, c, o_s, o_c)
    }

    /// Computes the sine and cosine of a [`BigExponentFloat`], rounding the results to the
    /// specified precision and with the specified rounding mode. The [`BigExponentFloat`] is taken
    /// by reference. Two [`Ordering`]s are also returned, indicating whether the rounded sine and
    /// cosine are less than, equal to, or greater than the exact values. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal` for it.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = (\sin x+\varepsilon_1, \cos x+\varepsilon_2).
    /// $$
    /// The errors $\varepsilon_1$ and $\varepsilon_2$ are bounded as for
    /// [`BigExponentFloat::sin_prec_round`] and [`BigExponentFloat::cos_prec_round`].
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=f(\pm\infty,p,m)=(\text{NaN},\text{NaN})$
    /// - $f(0,p,m)=(0,1)$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::sin_cos_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// Warning: an argument with exponent $e > 0$ is reduced modulo $\pi/2$, which takes about $e$
    /// bits of $\pi$. The time and memory needed therefore grow with the exponent, without any
    /// limit: an argument near $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\pi$.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of the argument (0 if it is negative).
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the argument is finite and nonzero (the
    /// sine and cosine of a nonzero rational number are transcendental).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = (&BigExponentFloat::ONE).sin_cos_prec_round_ref(5, Floor);
    /// assert_eq!(s.to_string(), "0.812");
    /// assert_eq!(c.to_string(), "0.531");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = (&BigExponentFloat::ONE).sin_cos_prec_round_ref(5, Ceiling);
    /// assert_eq!(s.to_string(), "0.844");
    /// assert_eq!(c.to_string(), "0.562");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = (&BigExponentFloat::ONE).sin_cos_prec_round_ref(5, Nearest);
    /// assert_eq!(s.to_string(), "0.844");
    /// assert_eq!(c.to_string(), "0.531");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = (&BigExponentFloat::ONE).sin_cos_prec_round_ref(20, Floor);
    /// assert_eq!(s.to_string(), "0.8414707");
    /// assert_eq!(c.to_string(), "0.5403023");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = (&BigExponentFloat::ONE).sin_cos_prec_round_ref(20, Ceiling);
    /// assert_eq!(s.to_string(), "0.84147167");
    /// assert_eq!(c.to_string(), "0.54030323");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = (&BigExponentFloat::ONE).sin_cos_prec_round_ref(20, Nearest);
    /// assert_eq!(s.to_string(), "0.8414707");
    /// assert_eq!(c.to_string(), "0.5403023");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    /// ```
    #[inline]
    pub fn sin_cos_prec_round_ref(
        &self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Self, Ordering, Ordering) {
        let (s, o_s) = trig_helper(TrigFunction::Sin, self, prec, rm, true);
        let (c, o_c) = trig_helper(TrigFunction::Cos, self, prec, rm, true);
        (s, c, o_s, o_c)
    }

    /// Replaces a [`BigExponentFloat`] with its sine and writes its cosine to `cos`, rounding both
    /// results to the specified precision and with the specified rounding mode. The previous value
    /// of `cos` is discarded. Two [`Ordering`]s are returned, indicating whether the rounded sine
    /// and cosine are less than, equal to, or greater than the exact values.
    ///
    /// See [`BigExponentFloat::sin_cos_prec_round`] for the error bounds, the special cases, and
    /// the complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the argument is finite and nonzero (the
    /// sine and cosine of a nonzero rational number are transcendental).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{NaN, One};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// let mut c = BigExponentFloat::NAN;
    /// assert_eq!(x.sin_cos_prec_round_assign(&mut c, 5, Floor), (Less, Less));
    /// assert_eq!(x.to_string(), "0.812");
    /// assert_eq!(c.to_string(), "0.531");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// let mut c = BigExponentFloat::NAN;
    /// assert_eq!(
    ///     x.sin_cos_prec_round_assign(&mut c, 5, Ceiling),
    ///     (Greater, Greater)
    /// );
    /// assert_eq!(x.to_string(), "0.844");
    /// assert_eq!(c.to_string(), "0.562");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// let mut c = BigExponentFloat::NAN;
    /// assert_eq!(
    ///     x.sin_cos_prec_round_assign(&mut c, 5, Nearest),
    ///     (Greater, Less)
    /// );
    /// assert_eq!(x.to_string(), "0.844");
    /// assert_eq!(c.to_string(), "0.531");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// let mut c = BigExponentFloat::NAN;
    /// assert_eq!(x.sin_cos_prec_round_assign(&mut c, 20, Floor), (Less, Less));
    /// assert_eq!(x.to_string(), "0.8414707");
    /// assert_eq!(c.to_string(), "0.5403023");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// let mut c = BigExponentFloat::NAN;
    /// assert_eq!(
    ///     x.sin_cos_prec_round_assign(&mut c, 20, Ceiling),
    ///     (Greater, Greater)
    /// );
    /// assert_eq!(x.to_string(), "0.84147167");
    /// assert_eq!(c.to_string(), "0.54030323");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// let mut c = BigExponentFloat::NAN;
    /// assert_eq!(
    ///     x.sin_cos_prec_round_assign(&mut c, 20, Nearest),
    ///     (Less, Less)
    /// );
    /// assert_eq!(x.to_string(), "0.8414707");
    /// assert_eq!(c.to_string(), "0.5403023");
    /// ```
    #[inline]
    pub fn sin_cos_prec_round_assign(
        &mut self,
        cos: &mut Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Ordering, Ordering) {
        let (s, o_s) = trig_helper(TrigFunction::Sin, self, prec, rm, true);
        let (c, o_c) = trig_helper(TrigFunction::Cos, self, prec, rm, true);
        *self = s;
        *cos = c;
        (o_s, o_c)
    }

    /// Computes the sine and cosine of a [`BigExponentFloat`], rounding the results to the nearest
    /// value of the specified precision. The [`BigExponentFloat`] is taken by value. Two
    /// [`Ordering`]s are also returned, indicating whether the rounded sine and cosine are less
    /// than, equal to, or greater than the exact values. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal` for it.
    ///
    /// $$
    /// f(x,p) = (\sin x+\varepsilon_1, \cos x+\varepsilon_2).
    /// $$
    /// The errors $\varepsilon_1$ and $\varepsilon_2$ are bounded as for
    /// [`BigExponentFloat::sin_prec`] and [`BigExponentFloat::cos_prec`].
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=f(\pm\infty,p)=(\text{NaN},\text{NaN})$
    /// - $f(0,p)=(0,1)$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::sin_cos_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// Warning: an argument with exponent $e > 0$ is reduced modulo $\pi/2$, which takes about $e$
    /// bits of $\pi$. The time and memory needed therefore grow with the exponent, without any
    /// limit: an argument near $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\pi$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::sin_cos_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of the argument (0 if it is negative).
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
    /// let (s, c, o_s, o_c) = BigExponentFloat::ONE.sin_cos_prec(5);
    /// assert_eq!(s.to_string(), "0.844");
    /// assert_eq!(c.to_string(), "0.531");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::ONE.sin_cos_prec(20);
    /// assert_eq!(s.to_string(), "0.8414707");
    /// assert_eq!(c.to_string(), "0.5403023");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    /// ```
    #[inline]
    pub fn sin_cos_prec(self, prec: u64) -> (Self, Self, Ordering, Ordering) {
        let (s, o_s) = trig_helper(TrigFunction::Sin, &self, prec, Nearest, true);
        let (c, o_c) = trig_helper(TrigFunction::Cos, &self, prec, Nearest, true);
        (s, c, o_s, o_c)
    }

    /// Computes the sine and cosine of a [`BigExponentFloat`], rounding the results to the nearest
    /// value of the specified precision. The [`BigExponentFloat`] is taken by reference. Two
    /// [`Ordering`]s are also returned, indicating whether the rounded sine and cosine are less
    /// than, equal to, or greater than the exact values. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal` for it.
    ///
    /// $$
    /// f(x,p) = (\sin x+\varepsilon_1, \cos x+\varepsilon_2).
    /// $$
    /// The errors $\varepsilon_1$ and $\varepsilon_2$ are bounded as for
    /// [`BigExponentFloat::sin_prec`] and [`BigExponentFloat::cos_prec`].
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=f(\pm\infty,p)=(\text{NaN},\text{NaN})$
    /// - $f(0,p)=(0,1)$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::sin_cos_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// Warning: an argument with exponent $e > 0$ is reduced modulo $\pi/2$, which takes about $e$
    /// bits of $\pi$. The time and memory needed therefore grow with the exponent, without any
    /// limit: an argument near $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\pi$.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of the argument (0 if it is negative).
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
    /// let (s, c, o_s, o_c) = (&BigExponentFloat::ONE).sin_cos_prec_ref(5);
    /// assert_eq!(s.to_string(), "0.844");
    /// assert_eq!(c.to_string(), "0.531");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = (&BigExponentFloat::ONE).sin_cos_prec_ref(20);
    /// assert_eq!(s.to_string(), "0.8414707");
    /// assert_eq!(c.to_string(), "0.5403023");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    /// ```
    #[inline]
    pub fn sin_cos_prec_ref(&self, prec: u64) -> (Self, Self, Ordering, Ordering) {
        let (s, o_s) = trig_helper(TrigFunction::Sin, self, prec, Nearest, true);
        let (c, o_c) = trig_helper(TrigFunction::Cos, self, prec, Nearest, true);
        (s, c, o_s, o_c)
    }

    /// Replaces a [`BigExponentFloat`] with its sine and writes its cosine to `cos`, rounding both
    /// results to the nearest value of the specified precision. The previous value of `cos` is
    /// discarded. Two [`Ordering`]s are returned, indicating whether the rounded sine and cosine
    /// are less than, equal to, or greater than the exact values.
    ///
    /// See [`BigExponentFloat::sin_cos_prec`] for the error bounds, the special cases, and the
    /// complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{NaN, One};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// let mut c = BigExponentFloat::NAN;
    /// assert_eq!(x.sin_cos_prec_assign(&mut c, 5), (Greater, Less));
    /// assert_eq!(x.to_string(), "0.844");
    /// assert_eq!(c.to_string(), "0.531");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// let mut c = BigExponentFloat::NAN;
    /// assert_eq!(x.sin_cos_prec_assign(&mut c, 20), (Less, Less));
    /// assert_eq!(x.to_string(), "0.8414707");
    /// assert_eq!(c.to_string(), "0.5403023");
    /// ```
    #[inline]
    pub fn sin_cos_prec_assign(&mut self, cos: &mut Self, prec: u64) -> (Ordering, Ordering) {
        let (s, o_s) = trig_helper(TrigFunction::Sin, self, prec, Nearest, true);
        let (c, o_c) = trig_helper(TrigFunction::Cos, self, prec, Nearest, true);
        *self = s;
        *cos = c;
        (o_s, o_c)
    }

    /// Computes the sine and cosine of a [`Rational`], returning [`BigExponentFloat`]s, rounding
    /// the results to the specified precision and with the specified rounding mode. The
    /// [`Rational`] is taken by value. Two [`Ordering`]s are also returned, indicating whether the
    /// rounded sine and cosine are less than, equal to, or greater than the exact values. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal` for it.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = (\sin x+\varepsilon_1, \cos x+\varepsilon_2).
    /// $$
    /// The errors $\varepsilon_1$ and $\varepsilon_2$ are bounded as for
    /// [`BigExponentFloat::sin_rational_prec_round`] and
    /// [`BigExponentFloat::cos_rational_prec_round`].
    ///
    /// Special cases:
    /// - $f(0,p,m)=(0,1)$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::sin_cos_rational_prec_round), this function
    /// never overflows or underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::sin_cos_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is `x.significant_bits()`,
    /// and $e$ is the exponent of the argument (0 if it is negative).
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the argument is finite and nonzero (the
    /// sine and cosine of a nonzero rational number are transcendental).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sin_cos_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(s.to_string(), "0.31");
    /// assert_eq!(c.to_string(), "0.938");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sin_cos_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(s.to_string(), "0.328");
    /// assert_eq!(c.to_string(), "0.969");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sin_cos_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(s.to_string(), "0.328");
    /// assert_eq!(c.to_string(), "0.938");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sin_cos_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(s.to_string(), "0.32719469");
    /// assert_eq!(c.to_string(), "0.94495678");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sin_cos_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(s.to_string(), "0.3271952");
    /// assert_eq!(c.to_string(), "0.9449577");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sin_cos_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(s.to_string(), "0.32719469");
    /// assert_eq!(c.to_string(), "0.94495678");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sin_cos_rational_prec_round(
        x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Self, Ordering, Ordering) {
        let (s, o_s) = trig_rational_helper(TrigFunction::Sin, &x, prec, rm, true);
        let (c, o_c) = trig_rational_helper(TrigFunction::Cos, &x, prec, rm, true);
        (s, c, o_s, o_c)
    }

    /// Computes the sine and cosine of a [`Rational`], returning [`BigExponentFloat`]s, rounding
    /// the results to the specified precision and with the specified rounding mode. The
    /// [`Rational`] is taken by reference. Two [`Ordering`]s are also returned, indicating whether
    /// the rounded sine and cosine are less than, equal to, or greater than the exact values.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal` for it.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = (\sin x+\varepsilon_1, \cos x+\varepsilon_2).
    /// $$
    /// The errors $\varepsilon_1$ and $\varepsilon_2$ are bounded as for
    /// [`BigExponentFloat::sin_rational_prec_round`] and
    /// [`BigExponentFloat::cos_rational_prec_round`].
    ///
    /// Special cases:
    /// - $f(0,p,m)=(0,1)$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::sin_cos_rational_prec_round), this function
    /// never overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is `x.significant_bits()`,
    /// and $e$ is the exponent of the argument (0 if it is negative).
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the argument is finite and nonzero (the
    /// sine and cosine of a nonzero rational number are transcendental).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sin_cos_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(s.to_string(), "0.31");
    /// assert_eq!(c.to_string(), "0.938");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sin_cos_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(s.to_string(), "0.328");
    /// assert_eq!(c.to_string(), "0.969");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sin_cos_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(s.to_string(), "0.328");
    /// assert_eq!(c.to_string(), "0.938");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sin_cos_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(s.to_string(), "0.32719469");
    /// assert_eq!(c.to_string(), "0.94495678");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sin_cos_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(s.to_string(), "0.3271952");
    /// assert_eq!(c.to_string(), "0.9449577");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sin_cos_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(s.to_string(), "0.32719469");
    /// assert_eq!(c.to_string(), "0.94495678");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    /// ```
    #[inline]
    pub fn sin_cos_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Self, Ordering, Ordering) {
        let (s, o_s) = trig_rational_helper(TrigFunction::Sin, x, prec, rm, true);
        let (c, o_c) = trig_rational_helper(TrigFunction::Cos, x, prec, rm, true);
        (s, c, o_s, o_c)
    }

    /// Computes the sine and cosine of a [`Rational`], returning [`BigExponentFloat`]s, rounding
    /// the results to the nearest value of the specified precision. The [`Rational`] is taken by
    /// value. Two [`Ordering`]s are also returned, indicating whether the rounded sine and cosine
    /// are less than, equal to, or greater than the exact values. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal` for it.
    ///
    /// $$
    /// f(x,p) = (\sin x+\varepsilon_1, \cos x+\varepsilon_2).
    /// $$
    /// The errors $\varepsilon_1$ and $\varepsilon_2$ are bounded as for
    /// [`BigExponentFloat::sin_rational_prec`] and [`BigExponentFloat::cos_rational_prec`].
    ///
    /// Special cases:
    /// - $f(0,p)=(0,1)$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::sin_cos_rational_prec_round), this function
    /// never overflows or underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::sin_cos_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is `x.significant_bits()`,
    /// and $e$ is the exponent of the argument (0 if it is negative).
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
    /// let (s, c, o_s, o_c) =
    ///     BigExponentFloat::sin_cos_rational_prec(Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(s.to_string(), "0.328");
    /// assert_eq!(c.to_string(), "0.938");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) =
    ///     BigExponentFloat::sin_cos_rational_prec(Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(s.to_string(), "0.32719469");
    /// assert_eq!(c.to_string(), "0.94495678");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sin_cos_rational_prec(x: Rational, prec: u64) -> (Self, Self, Ordering, Ordering) {
        let (s, o_s) = trig_rational_helper(TrigFunction::Sin, &x, prec, Nearest, true);
        let (c, o_c) = trig_rational_helper(TrigFunction::Cos, &x, prec, Nearest, true);
        (s, c, o_s, o_c)
    }

    /// Computes the sine and cosine of a [`Rational`], returning [`BigExponentFloat`]s, rounding
    /// the results to the nearest value of the specified precision. The [`Rational`] is taken by
    /// reference. Two [`Ordering`]s are also returned, indicating whether the rounded sine and
    /// cosine are less than, equal to, or greater than the exact values. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal` for it.
    ///
    /// $$
    /// f(x,p) = (\sin x+\varepsilon_1, \cos x+\varepsilon_2).
    /// $$
    /// The errors $\varepsilon_1$ and $\varepsilon_2$ are bounded as for
    /// [`BigExponentFloat::sin_rational_prec`] and [`BigExponentFloat::cos_rational_prec`].
    ///
    /// Special cases:
    /// - $f(0,p)=(0,1)$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::sin_cos_rational_prec_round), this function
    /// never overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is `x.significant_bits()`,
    /// and $e$ is the exponent of the argument (0 if it is negative).
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
    /// let (s, c, o_s, o_c) =
    ///     BigExponentFloat::sin_cos_rational_prec_ref(&Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(s.to_string(), "0.328");
    /// assert_eq!(c.to_string(), "0.938");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) =
    ///     BigExponentFloat::sin_cos_rational_prec_ref(&Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(s.to_string(), "0.32719469");
    /// assert_eq!(c.to_string(), "0.94495678");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    /// ```
    #[inline]
    pub fn sin_cos_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Self, Ordering, Ordering) {
        let (s, o_s) = trig_rational_helper(TrigFunction::Sin, x, prec, Nearest, true);
        let (c, o_c) = trig_rational_helper(TrigFunction::Cos, x, prec, Nearest, true);
        (s, c, o_s, o_c)
    }

    /// Computes $\sin(2\pi x/u)$ and $\cos(2\pi x/u)$, the sine and cosine of a
    /// [`BigExponentFloat`] measured in $u$ths of a turn, rounding the results to the specified
    /// precision and with the specified rounding mode. The [`BigExponentFloat`] is taken by value.
    /// Two [`Ordering`]s are also returned, indicating whether the rounded sine and cosine are less
    /// than, equal to, or greater than the exact values. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal` for it.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = (\sin(2\pi x/u)+\varepsilon_1, \cos(2\pi x/u)+\varepsilon_2).
    /// $$
    /// The errors $\varepsilon_1$ and $\varepsilon_2$ are bounded as for
    /// [`BigExponentFloat::sin_with_period_prec_round`] and
    /// [`BigExponentFloat::cos_with_period_prec_round`].
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p,m)=f(\pm\infty,u,p,m)=(\text{NaN},\text{NaN})$
    /// - $f(x,0,p,m)=(\text{NaN},\text{NaN})$
    /// - $f(0,u,p,m)=(0,1)$
    /// - If $x/u$ is a multiple of $1/2$, the sine is exactly $0$; if it is an odd multiple of
    ///   $1/4$, exactly $1$ or $-1$; and if it is $\pm1/12$ or $\pm5/12$ modulo $1$, exactly $1/2$
    ///   or $-1/2$.
    /// - If $x/u$ is a multiple of $1/2$, the cosine is exactly $1$ or $-1$; if it is an odd
    ///   multiple of $1/4$, exactly $0$; and if it is $\pm1/6$ or $\pm1/3$ modulo $1$, exactly
    ///   $1/2$ or $-1/2$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::sin_cos_with_period_prec_round), this
    /// function accepts arguments with any exponent, and its result never overflows or underflows.
    /// The argument is reduced exactly, modulo $u$, which for a large exponent takes time
    /// proportional to the number of bits of the exponent.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::sin_cos_with_period_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()` plus the number of bits of the exponent of the argument.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::ONE.sin_cos_with_period_prec_round(7, 5, Floor);
    /// assert_eq!(s.to_string(), "0.781");
    /// assert_eq!(c.to_string(), "0.594");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::ONE.sin_cos_with_period_prec_round(7, 5, Ceiling);
    /// assert_eq!(s.to_string(), "0.812");
    /// assert_eq!(c.to_string(), "0.62");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::ONE.sin_cos_with_period_prec_round(7, 5, Nearest);
    /// assert_eq!(s.to_string(), "0.781");
    /// assert_eq!(c.to_string(), "0.62");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::ONE.sin_cos_with_period_prec_round(7, 20, Floor);
    /// assert_eq!(s.to_string(), "0.78183079");
    /// assert_eq!(c.to_string(), "0.623489");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::ONE.sin_cos_with_period_prec_round(7, 20, Ceiling);
    /// assert_eq!(s.to_string(), "0.7818317");
    /// assert_eq!(c.to_string(), "0.62349033");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::ONE.sin_cos_with_period_prec_round(7, 20, Nearest);
    /// assert_eq!(s.to_string(), "0.7818317");
    /// assert_eq!(c.to_string(), "0.623489");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Less);
    /// ```
    #[inline]
    pub fn sin_cos_with_period_prec_round(
        self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Self, Ordering, Ordering) {
        let (s, o_s) = trig_with_period_helper(TrigFunction::Sin, &self, u, prec, rm, true);
        let (c, o_c) = trig_with_period_helper(TrigFunction::Cos, &self, u, prec, rm, true);
        (s, c, o_s, o_c)
    }

    /// Computes $\sin(2\pi x/u)$ and $\cos(2\pi x/u)$, the sine and cosine of a
    /// [`BigExponentFloat`] measured in $u$ths of a turn, rounding the results to the specified
    /// precision and with the specified rounding mode. The [`BigExponentFloat`] is taken by
    /// reference. Two [`Ordering`]s are also returned, indicating whether the rounded sine and
    /// cosine are less than, equal to, or greater than the exact values. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal` for it.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = (\sin(2\pi x/u)+\varepsilon_1, \cos(2\pi x/u)+\varepsilon_2).
    /// $$
    /// The errors $\varepsilon_1$ and $\varepsilon_2$ are bounded as for
    /// [`BigExponentFloat::sin_with_period_prec_round`] and
    /// [`BigExponentFloat::cos_with_period_prec_round`].
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p,m)=f(\pm\infty,u,p,m)=(\text{NaN},\text{NaN})$
    /// - $f(x,0,p,m)=(\text{NaN},\text{NaN})$
    /// - $f(0,u,p,m)=(0,1)$
    /// - If $x/u$ is a multiple of $1/2$, the sine is exactly $0$; if it is an odd multiple of
    ///   $1/4$, exactly $1$ or $-1$; and if it is $\pm1/12$ or $\pm5/12$ modulo $1$, exactly $1/2$
    ///   or $-1/2$.
    /// - If $x/u$ is a multiple of $1/2$, the cosine is exactly $1$ or $-1$; if it is an odd
    ///   multiple of $1/4$, exactly $0$; and if it is $\pm1/6$ or $\pm1/3$ modulo $1$, exactly
    ///   $1/2$ or $-1/2$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::sin_cos_with_period_prec_round), this
    /// function accepts arguments with any exponent, and its result never overflows or underflows.
    /// The argument is reduced exactly, modulo $u$, which for a large exponent takes time
    /// proportional to the number of bits of the exponent.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()` plus the number of bits of the exponent of the argument.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) =
    ///     (&BigExponentFloat::ONE).sin_cos_with_period_prec_round_ref(7, 5, Floor);
    /// assert_eq!(s.to_string(), "0.781");
    /// assert_eq!(c.to_string(), "0.594");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) =
    ///     (&BigExponentFloat::ONE).sin_cos_with_period_prec_round_ref(7, 5, Ceiling);
    /// assert_eq!(s.to_string(), "0.812");
    /// assert_eq!(c.to_string(), "0.62");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) =
    ///     (&BigExponentFloat::ONE).sin_cos_with_period_prec_round_ref(7, 5, Nearest);
    /// assert_eq!(s.to_string(), "0.781");
    /// assert_eq!(c.to_string(), "0.62");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) =
    ///     (&BigExponentFloat::ONE).sin_cos_with_period_prec_round_ref(7, 20, Floor);
    /// assert_eq!(s.to_string(), "0.78183079");
    /// assert_eq!(c.to_string(), "0.623489");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) =
    ///     (&BigExponentFloat::ONE).sin_cos_with_period_prec_round_ref(7, 20, Ceiling);
    /// assert_eq!(s.to_string(), "0.7818317");
    /// assert_eq!(c.to_string(), "0.62349033");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) =
    ///     (&BigExponentFloat::ONE).sin_cos_with_period_prec_round_ref(7, 20, Nearest);
    /// assert_eq!(s.to_string(), "0.7818317");
    /// assert_eq!(c.to_string(), "0.623489");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Less);
    /// ```
    #[inline]
    pub fn sin_cos_with_period_prec_round_ref(
        &self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Self, Ordering, Ordering) {
        let (s, o_s) = trig_with_period_helper(TrigFunction::Sin, self, u, prec, rm, true);
        let (c, o_c) = trig_with_period_helper(TrigFunction::Cos, self, u, prec, rm, true);
        (s, c, o_s, o_c)
    }

    /// Replaces a [`BigExponentFloat`] with its sine measured in $u$ths of a turn and writes its
    /// cosine to `cos`, rounding both results to the specified precision and with the specified
    /// rounding mode. The previous value of `cos` is discarded. Two [`Ordering`]s are returned,
    /// indicating whether the rounded sine and cosine are less than, equal to, or greater than the
    /// exact values.
    ///
    /// See [`BigExponentFloat::sin_cos_with_period_prec_round`] for the error bounds, the special
    /// cases, and the complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{NaN, One};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// let mut c = BigExponentFloat::NAN;
    /// assert_eq!(
    ///     x.sin_cos_with_period_prec_round_assign(&mut c, 7, 5, Floor),
    ///     (Less, Less)
    /// );
    /// assert_eq!(x.to_string(), "0.781");
    /// assert_eq!(c.to_string(), "0.594");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// let mut c = BigExponentFloat::NAN;
    /// assert_eq!(
    ///     x.sin_cos_with_period_prec_round_assign(&mut c, 7, 5, Ceiling),
    ///     (Greater, Greater)
    /// );
    /// assert_eq!(x.to_string(), "0.812");
    /// assert_eq!(c.to_string(), "0.62");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// let mut c = BigExponentFloat::NAN;
    /// assert_eq!(
    ///     x.sin_cos_with_period_prec_round_assign(&mut c, 7, 5, Nearest),
    ///     (Less, Greater)
    /// );
    /// assert_eq!(x.to_string(), "0.781");
    /// assert_eq!(c.to_string(), "0.62");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// let mut c = BigExponentFloat::NAN;
    /// assert_eq!(
    ///     x.sin_cos_with_period_prec_round_assign(&mut c, 7, 20, Floor),
    ///     (Less, Less)
    /// );
    /// assert_eq!(x.to_string(), "0.78183079");
    /// assert_eq!(c.to_string(), "0.623489");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// let mut c = BigExponentFloat::NAN;
    /// assert_eq!(
    ///     x.sin_cos_with_period_prec_round_assign(&mut c, 7, 20, Ceiling),
    ///     (Greater, Greater)
    /// );
    /// assert_eq!(x.to_string(), "0.7818317");
    /// assert_eq!(c.to_string(), "0.62349033");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// let mut c = BigExponentFloat::NAN;
    /// assert_eq!(
    ///     x.sin_cos_with_period_prec_round_assign(&mut c, 7, 20, Nearest),
    ///     (Greater, Less)
    /// );
    /// assert_eq!(x.to_string(), "0.7818317");
    /// assert_eq!(c.to_string(), "0.623489");
    /// ```
    #[inline]
    pub fn sin_cos_with_period_prec_round_assign(
        &mut self,
        cos: &mut Self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Ordering, Ordering) {
        let (s, o_s) = trig_with_period_helper(TrigFunction::Sin, self, u, prec, rm, true);
        let (c, o_c) = trig_with_period_helper(TrigFunction::Cos, self, u, prec, rm, true);
        *self = s;
        *cos = c;
        (o_s, o_c)
    }

    /// Computes $\sin(2\pi x/u)$ and $\cos(2\pi x/u)$, the sine and cosine of a
    /// [`BigExponentFloat`] measured in $u$ths of a turn, rounding the results to the nearest value
    /// of the specified precision. The [`BigExponentFloat`] is taken by value. Two [`Ordering`]s
    /// are also returned, indicating whether the rounded sine and cosine are less than, equal to,
    /// or greater than the exact values. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal` for it.
    ///
    /// $$
    /// f(x,u,p) = (\sin(2\pi x/u)+\varepsilon_1, \cos(2\pi x/u)+\varepsilon_2).
    /// $$
    /// The errors $\varepsilon_1$ and $\varepsilon_2$ are bounded as for
    /// [`BigExponentFloat::sin_with_period_prec`] and [`BigExponentFloat::cos_with_period_prec`].
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p)=f(\pm\infty,u,p)=(\text{NaN},\text{NaN})$
    /// - $f(x,0,p)=(\text{NaN},\text{NaN})$
    /// - $f(0,u,p)=(0,1)$
    /// - If $x/u$ is a multiple of $1/2$, the sine is exactly $0$; if it is an odd multiple of
    ///   $1/4$, exactly $1$ or $-1$; and if it is $\pm1/12$ or $\pm5/12$ modulo $1$, exactly $1/2$
    ///   or $-1/2$.
    /// - If $x/u$ is a multiple of $1/2$, the cosine is exactly $1$ or $-1$; if it is an odd
    ///   multiple of $1/4$, exactly $0$; and if it is $\pm1/6$ or $\pm1/3$ modulo $1$, exactly
    ///   $1/2$ or $-1/2$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::sin_cos_with_period_prec_round), this
    /// function accepts arguments with any exponent, and its result never overflows or underflows.
    /// The argument is reduced exactly, modulo $u$, which for a large exponent takes time
    /// proportional to the number of bits of the exponent.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::sin_cos_with_period_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()` plus the number of bits of the exponent of the argument.
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
    /// let (s, c, o_s, o_c) = BigExponentFloat::ONE.sin_cos_with_period_prec(7, 5);
    /// assert_eq!(s.to_string(), "0.781");
    /// assert_eq!(c.to_string(), "0.62");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::ONE.sin_cos_with_period_prec(7, 20);
    /// assert_eq!(s.to_string(), "0.7818317");
    /// assert_eq!(c.to_string(), "0.623489");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Less);
    /// ```
    #[inline]
    pub fn sin_cos_with_period_prec(self, u: u64, prec: u64) -> (Self, Self, Ordering, Ordering) {
        let (s, o_s) = trig_with_period_helper(TrigFunction::Sin, &self, u, prec, Nearest, true);
        let (c, o_c) = trig_with_period_helper(TrigFunction::Cos, &self, u, prec, Nearest, true);
        (s, c, o_s, o_c)
    }

    /// Computes $\sin(2\pi x/u)$ and $\cos(2\pi x/u)$, the sine and cosine of a
    /// [`BigExponentFloat`] measured in $u$ths of a turn, rounding the results to the nearest value
    /// of the specified precision. The [`BigExponentFloat`] is taken by reference. Two
    /// [`Ordering`]s are also returned, indicating whether the rounded sine and cosine are less
    /// than, equal to, or greater than the exact values. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal` for it.
    ///
    /// $$
    /// f(x,u,p) = (\sin(2\pi x/u)+\varepsilon_1, \cos(2\pi x/u)+\varepsilon_2).
    /// $$
    /// The errors $\varepsilon_1$ and $\varepsilon_2$ are bounded as for
    /// [`BigExponentFloat::sin_with_period_prec`] and [`BigExponentFloat::cos_with_period_prec`].
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p)=f(\pm\infty,u,p)=(\text{NaN},\text{NaN})$
    /// - $f(x,0,p)=(\text{NaN},\text{NaN})$
    /// - $f(0,u,p)=(0,1)$
    /// - If $x/u$ is a multiple of $1/2$, the sine is exactly $0$; if it is an odd multiple of
    ///   $1/4$, exactly $1$ or $-1$; and if it is $\pm1/12$ or $\pm5/12$ modulo $1$, exactly $1/2$
    ///   or $-1/2$.
    /// - If $x/u$ is a multiple of $1/2$, the cosine is exactly $1$ or $-1$; if it is an odd
    ///   multiple of $1/4$, exactly $0$; and if it is $\pm1/6$ or $\pm1/3$ modulo $1$, exactly
    ///   $1/2$ or $-1/2$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::sin_cos_with_period_prec_round), this
    /// function accepts arguments with any exponent, and its result never overflows or underflows.
    /// The argument is reduced exactly, modulo $u$, which for a large exponent takes time
    /// proportional to the number of bits of the exponent.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()` plus the number of bits of the exponent of the argument.
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
    /// let (s, c, o_s, o_c) = (&BigExponentFloat::ONE).sin_cos_with_period_prec_ref(7, 5);
    /// assert_eq!(s.to_string(), "0.781");
    /// assert_eq!(c.to_string(), "0.62");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = (&BigExponentFloat::ONE).sin_cos_with_period_prec_ref(7, 20);
    /// assert_eq!(s.to_string(), "0.7818317");
    /// assert_eq!(c.to_string(), "0.623489");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Less);
    /// ```
    #[inline]
    pub fn sin_cos_with_period_prec_ref(
        &self,
        u: u64,
        prec: u64,
    ) -> (Self, Self, Ordering, Ordering) {
        let (s, o_s) = trig_with_period_helper(TrigFunction::Sin, self, u, prec, Nearest, true);
        let (c, o_c) = trig_with_period_helper(TrigFunction::Cos, self, u, prec, Nearest, true);
        (s, c, o_s, o_c)
    }

    /// Replaces a [`BigExponentFloat`] with its sine measured in $u$ths of a turn and writes its
    /// cosine to `cos`, rounding both results to the nearest value of the specified precision. The
    /// previous value of `cos` is discarded. Two [`Ordering`]s are returned, indicating whether the
    /// rounded sine and cosine are less than, equal to, or greater than the exact values.
    ///
    /// See [`BigExponentFloat::sin_cos_with_period_prec`] for the error bounds, the special cases,
    /// and the complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{NaN, One};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// let mut c = BigExponentFloat::NAN;
    /// assert_eq!(
    ///     x.sin_cos_with_period_prec_assign(&mut c, 7, 5),
    ///     (Less, Greater)
    /// );
    /// assert_eq!(x.to_string(), "0.781");
    /// assert_eq!(c.to_string(), "0.62");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// let mut c = BigExponentFloat::NAN;
    /// assert_eq!(
    ///     x.sin_cos_with_period_prec_assign(&mut c, 7, 20),
    ///     (Greater, Less)
    /// );
    /// assert_eq!(x.to_string(), "0.7818317");
    /// assert_eq!(c.to_string(), "0.623489");
    /// ```
    #[inline]
    pub fn sin_cos_with_period_prec_assign(
        &mut self,
        cos: &mut Self,
        u: u64,
        prec: u64,
    ) -> (Ordering, Ordering) {
        let (s, o_s) = trig_with_period_helper(TrigFunction::Sin, self, u, prec, Nearest, true);
        let (c, o_c) = trig_with_period_helper(TrigFunction::Cos, self, u, prec, Nearest, true);
        *self = s;
        *cos = c;
        (o_s, o_c)
    }

    /// Computes $\sin(2\pi x/u)$ and $\cos(2\pi x/u)$, the sine and cosine of a [`Rational`]
    /// measured in $u$ths of a turn, returning [`BigExponentFloat`]s, rounding the results to the
    /// specified precision and with the specified rounding mode. The [`Rational`] is taken by
    /// value. Two [`Ordering`]s are also returned, indicating whether the rounded sine and cosine
    /// are less than, equal to, or greater than the exact values. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal` for it.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = (\sin(2\pi x/u)+\varepsilon_1, \cos(2\pi x/u)+\varepsilon_2).
    /// $$
    /// The errors $\varepsilon_1$ and $\varepsilon_2$ are bounded as for
    /// [`BigExponentFloat::sin_with_period_rational_prec_round`] and
    /// [`BigExponentFloat::cos_with_period_rational_prec_round`].
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=(\text{NaN},\text{NaN})$
    /// - $f(0,u,p,m)=(0,1)$
    /// - If $x/u$ is a multiple of $1/2$, the sine is exactly $0$; if it is an odd multiple of
    ///   $1/4$, exactly $1$ or $-1$; and if it is $\pm1/12$ or $\pm5/12$ modulo $1$, exactly $1/2$
    ///   or $-1/2$.
    /// - If $x/u$ is a multiple of $1/2$, the cosine is exactly $1$ or $-1$; if it is an odd
    ///   multiple of $1/4$, exactly $0$; and if it is $\pm1/6$ or $\pm1/3$ modulo $1$, exactly
    ///   $1/2$ or $-1/2$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::sin_cos_with_period_rational_prec_round),
    /// this function never overflows or underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::sin_cos_with_period_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()` plus the number of bits of the exponent of the argument.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sin_cos_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(s.to_string(), "0.281");
    /// assert_eq!(c.to_string(), "0.938");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sin_cos_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(s.to_string(), "0.297");
    /// assert_eq!(c.to_string(), "0.969");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sin_cos_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(s.to_string(), "0.297");
    /// assert_eq!(c.to_string(), "0.969");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sin_cos_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(s.to_string(), "0.2947550");
    /// assert_eq!(c.to_string(), "0.9555721");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sin_cos_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(s.to_string(), "0.29475546");
    /// assert_eq!(c.to_string(), "0.95557308");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sin_cos_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(s.to_string(), "0.2947550");
    /// assert_eq!(c.to_string(), "0.95557308");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sin_cos_with_period_rational_prec_round(
        x: Rational,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Self, Ordering, Ordering) {
        let (s, o_s) = trig_with_period_rational_helper(TrigFunction::Sin, &x, u, prec, rm, true);
        let (c, o_c) = trig_with_period_rational_helper(TrigFunction::Cos, &x, u, prec, rm, true);
        (s, c, o_s, o_c)
    }

    /// Computes $\sin(2\pi x/u)$ and $\cos(2\pi x/u)$, the sine and cosine of a [`Rational`]
    /// measured in $u$ths of a turn, returning [`BigExponentFloat`]s, rounding the results to the
    /// specified precision and with the specified rounding mode. The [`Rational`] is taken by
    /// reference. Two [`Ordering`]s are also returned, indicating whether the rounded sine and
    /// cosine are less than, equal to, or greater than the exact values. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal` for it.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = (\sin(2\pi x/u)+\varepsilon_1, \cos(2\pi x/u)+\varepsilon_2).
    /// $$
    /// The errors $\varepsilon_1$ and $\varepsilon_2$ are bounded as for
    /// [`BigExponentFloat::sin_with_period_rational_prec_round`] and
    /// [`BigExponentFloat::cos_with_period_rational_prec_round`].
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=(\text{NaN},\text{NaN})$
    /// - $f(0,u,p,m)=(0,1)$
    /// - If $x/u$ is a multiple of $1/2$, the sine is exactly $0$; if it is an odd multiple of
    ///   $1/4$, exactly $1$ or $-1$; and if it is $\pm1/12$ or $\pm5/12$ modulo $1$, exactly $1/2$
    ///   or $-1/2$.
    /// - If $x/u$ is a multiple of $1/2$, the cosine is exactly $1$ or $-1$; if it is an odd
    ///   multiple of $1/4$, exactly $0$; and if it is $\pm1/6$ or $\pm1/3$ modulo $1$, exactly
    ///   $1/2$ or $-1/2$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::sin_cos_with_period_rational_prec_round),
    /// this function never overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()` plus the number of bits of the exponent of the argument.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sin_cos_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(s.to_string(), "0.281");
    /// assert_eq!(c.to_string(), "0.938");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sin_cos_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(s.to_string(), "0.297");
    /// assert_eq!(c.to_string(), "0.969");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sin_cos_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(s.to_string(), "0.297");
    /// assert_eq!(c.to_string(), "0.969");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sin_cos_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(s.to_string(), "0.2947550");
    /// assert_eq!(c.to_string(), "0.9555721");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sin_cos_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(s.to_string(), "0.29475546");
    /// assert_eq!(c.to_string(), "0.95557308");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sin_cos_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(s.to_string(), "0.2947550");
    /// assert_eq!(c.to_string(), "0.95557308");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Greater);
    /// ```
    #[inline]
    pub fn sin_cos_with_period_rational_prec_round_ref(
        x: &Rational,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Self, Ordering, Ordering) {
        let (s, o_s) = trig_with_period_rational_helper(TrigFunction::Sin, x, u, prec, rm, true);
        let (c, o_c) = trig_with_period_rational_helper(TrigFunction::Cos, x, u, prec, rm, true);
        (s, c, o_s, o_c)
    }

    /// Computes $\sin(2\pi x/u)$ and $\cos(2\pi x/u)$, the sine and cosine of a [`Rational`]
    /// measured in $u$ths of a turn, returning [`BigExponentFloat`]s, rounding the results to the
    /// nearest value of the specified precision. The [`Rational`] is taken by value. Two
    /// [`Ordering`]s are also returned, indicating whether the rounded sine and cosine are less
    /// than, equal to, or greater than the exact values. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal` for it.
    ///
    /// $$
    /// f(x,u,p) = (\sin(2\pi x/u)+\varepsilon_1, \cos(2\pi x/u)+\varepsilon_2).
    /// $$
    /// The errors $\varepsilon_1$ and $\varepsilon_2$ are bounded as for
    /// [`BigExponentFloat::sin_with_period_rational_prec`] and
    /// [`BigExponentFloat::cos_with_period_rational_prec`].
    ///
    /// Special cases:
    /// - $f(x,0,p)=(\text{NaN},\text{NaN})$
    /// - $f(0,u,p)=(0,1)$
    /// - If $x/u$ is a multiple of $1/2$, the sine is exactly $0$; if it is an odd multiple of
    ///   $1/4$, exactly $1$ or $-1$; and if it is $\pm1/12$ or $\pm5/12$ modulo $1$, exactly $1/2$
    ///   or $-1/2$.
    /// - If $x/u$ is a multiple of $1/2$, the cosine is exactly $1$ or $-1$; if it is an odd
    ///   multiple of $1/4$, exactly $0$; and if it is $\pm1/6$ or $\pm1/3$ modulo $1$, exactly
    ///   $1/2$ or $-1/2$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::sin_cos_with_period_rational_prec_round),
    /// this function never overflows or underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::sin_cos_with_period_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()` plus the number of bits of the exponent of the argument.
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
    /// let (s, c, o_s, o_c) = BigExponentFloat::sin_cos_with_period_rational_prec(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    /// );
    /// assert_eq!(s.to_string(), "0.297");
    /// assert_eq!(c.to_string(), "0.969");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sin_cos_with_period_rational_prec(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    /// );
    /// assert_eq!(s.to_string(), "0.2947550");
    /// assert_eq!(c.to_string(), "0.95557308");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sin_cos_with_period_rational_prec(
        x: Rational,
        u: u64,
        prec: u64,
    ) -> (Self, Self, Ordering, Ordering) {
        let (s, o_s) =
            trig_with_period_rational_helper(TrigFunction::Sin, &x, u, prec, Nearest, true);
        let (c, o_c) =
            trig_with_period_rational_helper(TrigFunction::Cos, &x, u, prec, Nearest, true);
        (s, c, o_s, o_c)
    }

    /// Computes $\sin(2\pi x/u)$ and $\cos(2\pi x/u)$, the sine and cosine of a [`Rational`]
    /// measured in $u$ths of a turn, returning [`BigExponentFloat`]s, rounding the results to the
    /// nearest value of the specified precision. The [`Rational`] is taken by reference. Two
    /// [`Ordering`]s are also returned, indicating whether the rounded sine and cosine are less
    /// than, equal to, or greater than the exact values. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal` for it.
    ///
    /// $$
    /// f(x,u,p) = (\sin(2\pi x/u)+\varepsilon_1, \cos(2\pi x/u)+\varepsilon_2).
    /// $$
    /// The errors $\varepsilon_1$ and $\varepsilon_2$ are bounded as for
    /// [`BigExponentFloat::sin_with_period_rational_prec`] and
    /// [`BigExponentFloat::cos_with_period_rational_prec`].
    ///
    /// Special cases:
    /// - $f(x,0,p)=(\text{NaN},\text{NaN})$
    /// - $f(0,u,p)=(0,1)$
    /// - If $x/u$ is a multiple of $1/2$, the sine is exactly $0$; if it is an odd multiple of
    ///   $1/4$, exactly $1$ or $-1$; and if it is $\pm1/12$ or $\pm5/12$ modulo $1$, exactly $1/2$
    ///   or $-1/2$.
    /// - If $x/u$ is a multiple of $1/2$, the cosine is exactly $1$ or $-1$; if it is an odd
    ///   multiple of $1/4$, exactly $0$; and if it is $\pm1/6$ or $\pm1/3$ modulo $1$, exactly
    ///   $1/2$ or $-1/2$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::sin_cos_with_period_rational_prec_round),
    /// this function never overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()` plus the number of bits of the exponent of the argument.
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
    /// let (s, c, o_s, o_c) = BigExponentFloat::sin_cos_with_period_rational_prec_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    /// );
    /// assert_eq!(s.to_string(), "0.297");
    /// assert_eq!(c.to_string(), "0.969");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sin_cos_with_period_rational_prec_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    /// );
    /// assert_eq!(s.to_string(), "0.2947550");
    /// assert_eq!(c.to_string(), "0.95557308");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Greater);
    /// ```
    #[inline]
    pub fn sin_cos_with_period_rational_prec_ref(
        x: &Rational,
        u: u64,
        prec: u64,
    ) -> (Self, Self, Ordering, Ordering) {
        let (s, o_s) =
            trig_with_period_rational_helper(TrigFunction::Sin, x, u, prec, Nearest, true);
        let (c, o_c) =
            trig_with_period_rational_helper(TrigFunction::Cos, x, u, prec, Nearest, true);
        (s, c, o_s, o_c)
    }

    /// Computes $\sin(\pi x)$ and $\cos(\pi x)$, the sine and cosine of a [`BigExponentFloat`]
    /// measured in half-turns, rounding the results to the specified precision and with the
    /// specified rounding mode. The [`BigExponentFloat`] is taken by value. Two [`Ordering`]s are
    /// also returned, indicating whether the rounded sine and cosine are less than, equal to, or
    /// greater than the exact values. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal` for it.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = (\sin(\pi x)+\varepsilon_1, \cos(\pi x)+\varepsilon_2).
    /// $$
    /// The errors $\varepsilon_1$ and $\varepsilon_2$ are bounded as for
    /// [`BigExponentFloat::sin_pi_prec_round`] and [`BigExponentFloat::cos_pi_prec_round`].
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=f(\pm\infty,p,m)=(\text{NaN},\text{NaN})$
    /// - $f(0,p,m)=(0,1)$
    /// - If $x$ is an integer, the sine is exactly $0$; if it is an odd multiple of $1/2$, exactly
    ///   $1$ or $-1$; and if it is $\pm1/6$ or $\pm5/6$ modulo $2$, exactly $1/2$ or $-1/2$.
    /// - If $x$ is an integer, the cosine is exactly $1$ or $-1$; if it is an odd multiple of
    ///   $1/2$, exactly $0$; and if it is $\pm1/3$ or $\pm2/3$ modulo $2$, exactly $1/2$ or $-1/2$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::sin_cos_pi_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows. The argument is
    /// reduced exactly, modulo 2, which for a large exponent takes time proportional to the number
    /// of bits of the exponent.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::sin_cos_pi_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()` plus the number of bits of the exponent of the argument.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::from(0.375).sin_cos_pi_prec_round(5, Floor);
    /// assert_eq!(s.to_string(), "0.906");
    /// assert_eq!(c.to_string(), "0.38");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::from(0.375).sin_cos_pi_prec_round(5, Ceiling);
    /// assert_eq!(s.to_string(), "0.938");
    /// assert_eq!(c.to_string(), "0.391");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::from(0.375).sin_cos_pi_prec_round(5, Nearest);
    /// assert_eq!(s.to_string(), "0.938");
    /// assert_eq!(c.to_string(), "0.38");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::from(0.375).sin_cos_pi_prec_round(20, Floor);
    /// assert_eq!(s.to_string(), "0.92387867");
    /// assert_eq!(c.to_string(), "0.38268328");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::from(0.375).sin_cos_pi_prec_round(20, Ceiling);
    /// assert_eq!(s.to_string(), "0.9238796");
    /// assert_eq!(c.to_string(), "0.3826838");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::from(0.375).sin_cos_pi_prec_round(20, Nearest);
    /// assert_eq!(s.to_string(), "0.9238796");
    /// assert_eq!(c.to_string(), "0.38268328");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Less);
    /// ```
    #[inline]
    pub fn sin_cos_pi_prec_round(
        self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Self, Ordering, Ordering) {
        let (s, o_s) = trig_with_period_helper(TrigFunction::Sin, &self, 2, prec, rm, true);
        let (c, o_c) = trig_with_period_helper(TrigFunction::Cos, &self, 2, prec, rm, true);
        (s, c, o_s, o_c)
    }

    /// Computes $\sin(\pi x)$ and $\cos(\pi x)$, the sine and cosine of a [`BigExponentFloat`]
    /// measured in half-turns, rounding the results to the specified precision and with the
    /// specified rounding mode. The [`BigExponentFloat`] is taken by reference. Two [`Ordering`]s
    /// are also returned, indicating whether the rounded sine and cosine are less than, equal to,
    /// or greater than the exact values. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal` for it.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = (\sin(\pi x)+\varepsilon_1, \cos(\pi x)+\varepsilon_2).
    /// $$
    /// The errors $\varepsilon_1$ and $\varepsilon_2$ are bounded as for
    /// [`BigExponentFloat::sin_pi_prec_round`] and [`BigExponentFloat::cos_pi_prec_round`].
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=f(\pm\infty,p,m)=(\text{NaN},\text{NaN})$
    /// - $f(0,p,m)=(0,1)$
    /// - If $x$ is an integer, the sine is exactly $0$; if it is an odd multiple of $1/2$, exactly
    ///   $1$ or $-1$; and if it is $\pm1/6$ or $\pm5/6$ modulo $2$, exactly $1/2$ or $-1/2$.
    /// - If $x$ is an integer, the cosine is exactly $1$ or $-1$; if it is an odd multiple of
    ///   $1/2$, exactly $0$; and if it is $\pm1/3$ or $\pm2/3$ modulo $2$, exactly $1/2$ or $-1/2$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::sin_cos_pi_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows. The argument is
    /// reduced exactly, modulo 2, which for a large exponent takes time proportional to the number
    /// of bits of the exponent.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()` plus the number of bits of the exponent of the argument.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = (&BigExponentFloat::from(0.375)).sin_cos_pi_prec_round_ref(5, Floor);
    /// assert_eq!(s.to_string(), "0.906");
    /// assert_eq!(c.to_string(), "0.38");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) =
    ///     (&BigExponentFloat::from(0.375)).sin_cos_pi_prec_round_ref(5, Ceiling);
    /// assert_eq!(s.to_string(), "0.938");
    /// assert_eq!(c.to_string(), "0.391");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) =
    ///     (&BigExponentFloat::from(0.375)).sin_cos_pi_prec_round_ref(5, Nearest);
    /// assert_eq!(s.to_string(), "0.938");
    /// assert_eq!(c.to_string(), "0.38");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) =
    ///     (&BigExponentFloat::from(0.375)).sin_cos_pi_prec_round_ref(20, Floor);
    /// assert_eq!(s.to_string(), "0.92387867");
    /// assert_eq!(c.to_string(), "0.38268328");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) =
    ///     (&BigExponentFloat::from(0.375)).sin_cos_pi_prec_round_ref(20, Ceiling);
    /// assert_eq!(s.to_string(), "0.9238796");
    /// assert_eq!(c.to_string(), "0.3826838");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) =
    ///     (&BigExponentFloat::from(0.375)).sin_cos_pi_prec_round_ref(20, Nearest);
    /// assert_eq!(s.to_string(), "0.9238796");
    /// assert_eq!(c.to_string(), "0.38268328");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Less);
    /// ```
    #[inline]
    pub fn sin_cos_pi_prec_round_ref(
        &self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Self, Ordering, Ordering) {
        let (s, o_s) = trig_with_period_helper(TrigFunction::Sin, self, 2, prec, rm, true);
        let (c, o_c) = trig_with_period_helper(TrigFunction::Cos, self, 2, prec, rm, true);
        (s, c, o_s, o_c)
    }

    /// Replaces a [`BigExponentFloat`] with its sine measured in half-turns and writes its cosine
    /// to `cos`, rounding both results to the specified precision and with the specified rounding
    /// mode. The previous value of `cos` is discarded. Two [`Ordering`]s are returned, indicating
    /// whether the rounded sine and cosine are less than, equal to, or greater than the exact
    /// values.
    ///
    /// See [`BigExponentFloat::sin_cos_pi_prec_round`] for the error bounds, the special cases, and
    /// the complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::NaN;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// let mut c = BigExponentFloat::NAN;
    /// assert_eq!(
    ///     x.sin_cos_pi_prec_round_assign(&mut c, 5, Floor),
    ///     (Less, Less)
    /// );
    /// assert_eq!(x.to_string(), "0.906");
    /// assert_eq!(c.to_string(), "0.38");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// let mut c = BigExponentFloat::NAN;
    /// assert_eq!(
    ///     x.sin_cos_pi_prec_round_assign(&mut c, 5, Ceiling),
    ///     (Greater, Greater)
    /// );
    /// assert_eq!(x.to_string(), "0.938");
    /// assert_eq!(c.to_string(), "0.391");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// let mut c = BigExponentFloat::NAN;
    /// assert_eq!(
    ///     x.sin_cos_pi_prec_round_assign(&mut c, 5, Nearest),
    ///     (Greater, Less)
    /// );
    /// assert_eq!(x.to_string(), "0.938");
    /// assert_eq!(c.to_string(), "0.38");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// let mut c = BigExponentFloat::NAN;
    /// assert_eq!(
    ///     x.sin_cos_pi_prec_round_assign(&mut c, 20, Floor),
    ///     (Less, Less)
    /// );
    /// assert_eq!(x.to_string(), "0.92387867");
    /// assert_eq!(c.to_string(), "0.38268328");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// let mut c = BigExponentFloat::NAN;
    /// assert_eq!(
    ///     x.sin_cos_pi_prec_round_assign(&mut c, 20, Ceiling),
    ///     (Greater, Greater)
    /// );
    /// assert_eq!(x.to_string(), "0.9238796");
    /// assert_eq!(c.to_string(), "0.3826838");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// let mut c = BigExponentFloat::NAN;
    /// assert_eq!(
    ///     x.sin_cos_pi_prec_round_assign(&mut c, 20, Nearest),
    ///     (Greater, Less)
    /// );
    /// assert_eq!(x.to_string(), "0.9238796");
    /// assert_eq!(c.to_string(), "0.38268328");
    /// ```
    #[inline]
    pub fn sin_cos_pi_prec_round_assign(
        &mut self,
        cos: &mut Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Ordering, Ordering) {
        let (s, o_s) = trig_with_period_helper(TrigFunction::Sin, self, 2, prec, rm, true);
        let (c, o_c) = trig_with_period_helper(TrigFunction::Cos, self, 2, prec, rm, true);
        *self = s;
        *cos = c;
        (o_s, o_c)
    }

    /// Computes $\sin(\pi x)$ and $\cos(\pi x)$, the sine and cosine of a [`BigExponentFloat`]
    /// measured in half-turns, rounding the results to the nearest value of the specified
    /// precision. The [`BigExponentFloat`] is taken by value. Two [`Ordering`]s are also returned,
    /// indicating whether the rounded sine and cosine are less than, equal to, or greater than the
    /// exact values. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal` for it.
    ///
    /// $$
    /// f(x,p) = (\sin(\pi x)+\varepsilon_1, \cos(\pi x)+\varepsilon_2).
    /// $$
    /// The errors $\varepsilon_1$ and $\varepsilon_2$ are bounded as for
    /// [`BigExponentFloat::sin_pi_prec`] and [`BigExponentFloat::cos_pi_prec`].
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=f(\pm\infty,p)=(\text{NaN},\text{NaN})$
    /// - $f(0,p)=(0,1)$
    /// - If $x$ is an integer, the sine is exactly $0$; if it is an odd multiple of $1/2$, exactly
    ///   $1$ or $-1$; and if it is $\pm1/6$ or $\pm5/6$ modulo $2$, exactly $1/2$ or $-1/2$.
    /// - If $x$ is an integer, the cosine is exactly $1$ or $-1$; if it is an odd multiple of
    ///   $1/2$, exactly $0$; and if it is $\pm1/3$ or $\pm2/3$ modulo $2$, exactly $1/2$ or $-1/2$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::sin_cos_pi_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows. The argument is
    /// reduced exactly, modulo 2, which for a large exponent takes time proportional to the number
    /// of bits of the exponent.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::sin_cos_pi_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()` plus the number of bits of the exponent of the argument.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::from(0.375).sin_cos_pi_prec(5);
    /// assert_eq!(s.to_string(), "0.938");
    /// assert_eq!(c.to_string(), "0.38");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::from(0.375).sin_cos_pi_prec(20);
    /// assert_eq!(s.to_string(), "0.9238796");
    /// assert_eq!(c.to_string(), "0.38268328");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Less);
    /// ```
    #[inline]
    pub fn sin_cos_pi_prec(self, prec: u64) -> (Self, Self, Ordering, Ordering) {
        let (s, o_s) = trig_with_period_helper(TrigFunction::Sin, &self, 2, prec, Nearest, true);
        let (c, o_c) = trig_with_period_helper(TrigFunction::Cos, &self, 2, prec, Nearest, true);
        (s, c, o_s, o_c)
    }

    /// Computes $\sin(\pi x)$ and $\cos(\pi x)$, the sine and cosine of a [`BigExponentFloat`]
    /// measured in half-turns, rounding the results to the nearest value of the specified
    /// precision. The [`BigExponentFloat`] is taken by reference. Two [`Ordering`]s are also
    /// returned, indicating whether the rounded sine and cosine are less than, equal to, or greater
    /// than the exact values. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal` for it.
    ///
    /// $$
    /// f(x,p) = (\sin(\pi x)+\varepsilon_1, \cos(\pi x)+\varepsilon_2).
    /// $$
    /// The errors $\varepsilon_1$ and $\varepsilon_2$ are bounded as for
    /// [`BigExponentFloat::sin_pi_prec`] and [`BigExponentFloat::cos_pi_prec`].
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=f(\pm\infty,p)=(\text{NaN},\text{NaN})$
    /// - $f(0,p)=(0,1)$
    /// - If $x$ is an integer, the sine is exactly $0$; if it is an odd multiple of $1/2$, exactly
    ///   $1$ or $-1$; and if it is $\pm1/6$ or $\pm5/6$ modulo $2$, exactly $1/2$ or $-1/2$.
    /// - If $x$ is an integer, the cosine is exactly $1$ or $-1$; if it is an odd multiple of
    ///   $1/2$, exactly $0$; and if it is $\pm1/3$ or $\pm2/3$ modulo $2$, exactly $1/2$ or $-1/2$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::sin_cos_pi_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows. The argument is
    /// reduced exactly, modulo 2, which for a large exponent takes time proportional to the number
    /// of bits of the exponent.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()` plus the number of bits of the exponent of the argument.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = (&BigExponentFloat::from(0.375)).sin_cos_pi_prec_ref(5);
    /// assert_eq!(s.to_string(), "0.938");
    /// assert_eq!(c.to_string(), "0.38");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = (&BigExponentFloat::from(0.375)).sin_cos_pi_prec_ref(20);
    /// assert_eq!(s.to_string(), "0.9238796");
    /// assert_eq!(c.to_string(), "0.38268328");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Less);
    /// ```
    #[inline]
    pub fn sin_cos_pi_prec_ref(&self, prec: u64) -> (Self, Self, Ordering, Ordering) {
        let (s, o_s) = trig_with_period_helper(TrigFunction::Sin, self, 2, prec, Nearest, true);
        let (c, o_c) = trig_with_period_helper(TrigFunction::Cos, self, 2, prec, Nearest, true);
        (s, c, o_s, o_c)
    }

    /// Replaces a [`BigExponentFloat`] with its sine measured in half-turns and writes its cosine
    /// to `cos`, rounding both results to the nearest value of the specified precision. The
    /// previous value of `cos` is discarded. Two [`Ordering`]s are returned, indicating whether the
    /// rounded sine and cosine are less than, equal to, or greater than the exact values.
    ///
    /// See [`BigExponentFloat::sin_cos_pi_prec`] for the error bounds, the special cases, and the
    /// complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::NaN;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// let mut c = BigExponentFloat::NAN;
    /// assert_eq!(x.sin_cos_pi_prec_assign(&mut c, 5), (Greater, Less));
    /// assert_eq!(x.to_string(), "0.938");
    /// assert_eq!(c.to_string(), "0.38");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// let mut c = BigExponentFloat::NAN;
    /// assert_eq!(x.sin_cos_pi_prec_assign(&mut c, 20), (Greater, Less));
    /// assert_eq!(x.to_string(), "0.9238796");
    /// assert_eq!(c.to_string(), "0.38268328");
    /// ```
    #[inline]
    pub fn sin_cos_pi_prec_assign(&mut self, cos: &mut Self, prec: u64) -> (Ordering, Ordering) {
        let (s, o_s) = trig_with_period_helper(TrigFunction::Sin, self, 2, prec, Nearest, true);
        let (c, o_c) = trig_with_period_helper(TrigFunction::Cos, self, 2, prec, Nearest, true);
        *self = s;
        *cos = c;
        (o_s, o_c)
    }

    /// Computes $\sin(\pi x)$ and $\cos(\pi x)$, the sine and cosine of a [`Rational`] measured in
    /// half-turns, returning [`BigExponentFloat`]s, rounding the results to the specified precision
    /// and with the specified rounding mode. The [`Rational`] is taken by value. Two [`Ordering`]s
    /// are also returned, indicating whether the rounded sine and cosine are less than, equal to,
    /// or greater than the exact values. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal` for it.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = (\sin(\pi x)+\varepsilon_1, \cos(\pi x)+\varepsilon_2).
    /// $$
    /// The errors $\varepsilon_1$ and $\varepsilon_2$ are bounded as for
    /// [`BigExponentFloat::sin_pi_rational_prec_round`] and
    /// [`BigExponentFloat::cos_pi_rational_prec_round`].
    ///
    /// Special cases:
    /// - $f(0,p,m)=(0,1)$
    /// - If $x$ is an integer, the sine is exactly $0$; if it is an odd multiple of $1/2$, exactly
    ///   $1$ or $-1$; and if it is $\pm1/6$ or $\pm5/6$ modulo $2$, exactly $1/2$ or $-1/2$.
    /// - If $x$ is an integer, the cosine is exactly $1$ or $-1$; if it is an odd multiple of
    ///   $1/2$, exactly $0$; and if it is $\pm1/3$ or $\pm2/3$ modulo $2$, exactly $1/2$ or $-1/2$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::sin_cos_pi_rational_prec_round), this
    /// function never overflows or underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::sin_cos_pi_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()` plus the number of bits of the exponent of the argument.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sin_cos_pi_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(s.to_string(), "0.562");
    /// assert_eq!(c.to_string(), "0.781");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sin_cos_pi_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(s.to_string(), "0.594");
    /// assert_eq!(c.to_string(), "0.812");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sin_cos_pi_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(s.to_string(), "0.594");
    /// assert_eq!(c.to_string(), "0.812");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sin_cos_pi_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(s.to_string(), "0.58778477");
    /// assert_eq!(c.to_string(), "0.80901623");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sin_cos_pi_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(s.to_string(), "0.5877857");
    /// assert_eq!(c.to_string(), "0.8090172");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sin_cos_pi_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(s.to_string(), "0.5877857");
    /// assert_eq!(c.to_string(), "0.8090172");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sin_cos_pi_rational_prec_round(
        x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Self, Ordering, Ordering) {
        let (s, o_s) = trig_with_period_rational_helper(TrigFunction::Sin, &x, 2, prec, rm, true);
        let (c, o_c) = trig_with_period_rational_helper(TrigFunction::Cos, &x, 2, prec, rm, true);
        (s, c, o_s, o_c)
    }

    /// Computes $\sin(\pi x)$ and $\cos(\pi x)$, the sine and cosine of a [`Rational`] measured in
    /// half-turns, returning [`BigExponentFloat`]s, rounding the results to the specified precision
    /// and with the specified rounding mode. The [`Rational`] is taken by reference. Two
    /// [`Ordering`]s are also returned, indicating whether the rounded sine and cosine are less
    /// than, equal to, or greater than the exact values. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal` for it.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = (\sin(\pi x)+\varepsilon_1, \cos(\pi x)+\varepsilon_2).
    /// $$
    /// The errors $\varepsilon_1$ and $\varepsilon_2$ are bounded as for
    /// [`BigExponentFloat::sin_pi_rational_prec_round`] and
    /// [`BigExponentFloat::cos_pi_rational_prec_round`].
    ///
    /// Special cases:
    /// - $f(0,p,m)=(0,1)$
    /// - If $x$ is an integer, the sine is exactly $0$; if it is an odd multiple of $1/2$, exactly
    ///   $1$ or $-1$; and if it is $\pm1/6$ or $\pm5/6$ modulo $2$, exactly $1/2$ or $-1/2$.
    /// - If $x$ is an integer, the cosine is exactly $1$ or $-1$; if it is an odd multiple of
    ///   $1/2$, exactly $0$; and if it is $\pm1/3$ or $\pm2/3$ modulo $2$, exactly $1/2$ or $-1/2$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::sin_cos_pi_rational_prec_round), this
    /// function never overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()` plus the number of bits of the exponent of the argument.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sin_cos_pi_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(s.to_string(), "0.562");
    /// assert_eq!(c.to_string(), "0.781");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sin_cos_pi_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(s.to_string(), "0.594");
    /// assert_eq!(c.to_string(), "0.812");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sin_cos_pi_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(s.to_string(), "0.594");
    /// assert_eq!(c.to_string(), "0.812");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sin_cos_pi_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(s.to_string(), "0.58778477");
    /// assert_eq!(c.to_string(), "0.80901623");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sin_cos_pi_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(s.to_string(), "0.5877857");
    /// assert_eq!(c.to_string(), "0.8090172");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sin_cos_pi_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(s.to_string(), "0.5877857");
    /// assert_eq!(c.to_string(), "0.8090172");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    /// ```
    #[inline]
    pub fn sin_cos_pi_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Self, Ordering, Ordering) {
        let (s, o_s) = trig_with_period_rational_helper(TrigFunction::Sin, x, 2, prec, rm, true);
        let (c, o_c) = trig_with_period_rational_helper(TrigFunction::Cos, x, 2, prec, rm, true);
        (s, c, o_s, o_c)
    }

    /// Computes $\sin(\pi x)$ and $\cos(\pi x)$, the sine and cosine of a [`Rational`] measured in
    /// half-turns, returning [`BigExponentFloat`]s, rounding the results to the nearest value of
    /// the specified precision. The [`Rational`] is taken by value. Two [`Ordering`]s are also
    /// returned, indicating whether the rounded sine and cosine are less than, equal to, or greater
    /// than the exact values. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal` for it.
    ///
    /// $$
    /// f(x,p) = (\sin(\pi x)+\varepsilon_1, \cos(\pi x)+\varepsilon_2).
    /// $$
    /// The errors $\varepsilon_1$ and $\varepsilon_2$ are bounded as for
    /// [`BigExponentFloat::sin_pi_rational_prec`] and [`BigExponentFloat::cos_pi_rational_prec`].
    ///
    /// Special cases:
    /// - $f(0,p)=(0,1)$
    /// - If $x$ is an integer, the sine is exactly $0$; if it is an odd multiple of $1/2$, exactly
    ///   $1$ or $-1$; and if it is $\pm1/6$ or $\pm5/6$ modulo $2$, exactly $1/2$ or $-1/2$.
    /// - If $x$ is an integer, the cosine is exactly $1$ or $-1$; if it is an odd multiple of
    ///   $1/2$, exactly $0$; and if it is $\pm1/3$ or $\pm2/3$ modulo $2$, exactly $1/2$ or $-1/2$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::sin_cos_pi_rational_prec_round), this
    /// function never overflows or underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::sin_cos_pi_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()` plus the number of bits of the exponent of the argument.
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
    /// let (s, c, o_s, o_c) =
    ///     BigExponentFloat::sin_cos_pi_rational_prec(Rational::from_unsigneds(1u8, 5), 5);
    /// assert_eq!(s.to_string(), "0.594");
    /// assert_eq!(c.to_string(), "0.812");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) =
    ///     BigExponentFloat::sin_cos_pi_rational_prec(Rational::from_unsigneds(1u8, 5), 20);
    /// assert_eq!(s.to_string(), "0.5877857");
    /// assert_eq!(c.to_string(), "0.8090172");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sin_cos_pi_rational_prec(x: Rational, prec: u64) -> (Self, Self, Ordering, Ordering) {
        let (s, o_s) =
            trig_with_period_rational_helper(TrigFunction::Sin, &x, 2, prec, Nearest, true);
        let (c, o_c) =
            trig_with_period_rational_helper(TrigFunction::Cos, &x, 2, prec, Nearest, true);
        (s, c, o_s, o_c)
    }

    /// Computes $\sin(\pi x)$ and $\cos(\pi x)$, the sine and cosine of a [`Rational`] measured in
    /// half-turns, returning [`BigExponentFloat`]s, rounding the results to the nearest value of
    /// the specified precision. The [`Rational`] is taken by reference. Two [`Ordering`]s are also
    /// returned, indicating whether the rounded sine and cosine are less than, equal to, or greater
    /// than the exact values. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal` for it.
    ///
    /// $$
    /// f(x,p) = (\sin(\pi x)+\varepsilon_1, \cos(\pi x)+\varepsilon_2).
    /// $$
    /// The errors $\varepsilon_1$ and $\varepsilon_2$ are bounded as for
    /// [`BigExponentFloat::sin_pi_rational_prec`] and [`BigExponentFloat::cos_pi_rational_prec`].
    ///
    /// Special cases:
    /// - $f(0,p)=(0,1)$
    /// - If $x$ is an integer, the sine is exactly $0$; if it is an odd multiple of $1/2$, exactly
    ///   $1$ or $-1$; and if it is $\pm1/6$ or $\pm5/6$ modulo $2$, exactly $1/2$ or $-1/2$.
    /// - If $x$ is an integer, the cosine is exactly $1$ or $-1$; if it is an odd multiple of
    ///   $1/2$, exactly $0$; and if it is $\pm1/3$ or $\pm2/3$ modulo $2$, exactly $1/2$ or $-1/2$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::sin_cos_pi_rational_prec_round), this
    /// function never overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()` plus the number of bits of the exponent of the argument.
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
    /// let (s, c, o_s, o_c) =
    ///     BigExponentFloat::sin_cos_pi_rational_prec_ref(&Rational::from_unsigneds(1u8, 5), 5);
    /// assert_eq!(s.to_string(), "0.594");
    /// assert_eq!(c.to_string(), "0.812");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) =
    ///     BigExponentFloat::sin_cos_pi_rational_prec_ref(&Rational::from_unsigneds(1u8, 5), 20);
    /// assert_eq!(s.to_string(), "0.5877857");
    /// assert_eq!(c.to_string(), "0.8090172");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    /// ```
    #[inline]
    pub fn sin_cos_pi_rational_prec_ref(
        x: &Rational,
        prec: u64,
    ) -> (Self, Self, Ordering, Ordering) {
        let (s, o_s) =
            trig_with_period_rational_helper(TrigFunction::Sin, x, 2, prec, Nearest, true);
        let (c, o_c) =
            trig_with_period_rational_helper(TrigFunction::Cos, x, 2, prec, Nearest, true);
        (s, c, o_s, o_c)
    }
}
