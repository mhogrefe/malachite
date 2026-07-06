// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright 2001-2026 Free Software Foundation, Inc.
//
//      Contributed by the Pascaline and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::arithmetic::ln::sliver_of_one;
use crate::arithmetic::log_base_2::extended_log_base_2_of_rational;
use crate::basic::extended::ExtendedFloat;
use crate::{
    Float, emulate_float_to_float_fn, emulate_rational_to_float_fn, float_either_zero,
    float_infinity, float_nan, float_negative_infinity,
};
use alloc::vec::Vec;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{
    CeilingLogBase2, CheckedLogBase, Gcd, IsPowerOf2, LogBase, LogBaseAssign, Mod, ModAdd, ModMul,
    ModPow, Pow, Sign,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Two, Zero as ZeroTrait};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::factorization::traits::ExpressAsPower;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::float_extras::float_can_round;
use malachite_nz::platform::Limb;
use malachite_q::Rational;

// Decomposes a positive finite nonzero `Float` into `(s, t)` with value `s * 2^t` and `s` odd. `s`
// has at most `prec` bits, so everything downstream of this decomposition works on small numbers no
// matter how extreme `x`'s exponent is.
pub(crate) fn odd_significand_and_exponent(x: &Float) -> (Natural, i64) {
    let sig = x.significand_ref().unwrap();
    let strip = sig.trailing_zeros().unwrap();
    let s = sig >> strip;
    let t = i64::from(x.get_exponent().unwrap()) - i64::exact_from(sig.significant_bits())
        + i64::exact_from(strip);
    (s, t)
}

// Given a positive dyadic value `s * 2^t` (`s` odd) and a root `2^z * h` (`h` an odd `Natural`, the
// root positive and different from 1), returns the integer `m` with `s * 2^t = (2^z * h)^m`, or
// `None` if no such integer exists.
//
// All arithmetic is on `s`, `h`, and `i64` exponents: nothing the size of `2^|t|` is ever
// materialized, so the check is cheap for arbitrary exponents. `m` may be negative only when `h =
// 1` (an odd `h >= 3` in the root makes every negative power non-dyadic).
pub(crate) fn dyadic_log_of_root(s: &Natural, t: i64, z: i64, h: &Natural) -> Option<i64> {
    if *h == 1u32 {
        // The root is 2^z, so s * 2^t = 2^(z * m) requires s = 1 and z | t.
        return if *s == 1u32 && z != 0 && t % z == 0 {
            Some(t / z)
        } else {
            None
        };
    }
    // h >= 3: (2^z * h)^m = 2^(z * m) * h^m with h^m odd, so s = h^m and t = z * m. A negative m
    // would put h in the denominator, which the dyadic s * 2^t cannot cancel, so m >= 0; and m = 0
    // means the value is 1, which callers handle separately.
    if *s == 1u32 {
        return None;
    }
    let m = i64::exact_from(s.checked_log_base(h)?);
    if z.checked_mul(m)? == t {
        Some(m)
    } else {
        None
    }
}

// Given a positive dyadic base `s_b * 2^t_b` (`s_b` odd) with base not 1, returns `(z, h, e_base)`
// such that `base = (2^z * h)^e_base` with `h` odd and `e_base` maximal (the primitive root). All
// work is on `s_b` (at most the base's precision in bits) and `i64` exponents, so the base is never
// materialized as an integer or `Rational`, no matter how extreme its exponent.
pub(crate) fn dyadic_primitive_root(s_b: &Natural, t_b: i64) -> (i64, Natural, u64) {
    if *s_b == 1u32 {
        // base = 2^t_b with t_b != 0: the primitive root is 2 (or 1/2 for a base below 1).
        return if t_b > 0 {
            (1, Natural::ONE, u64::exact_from(t_b))
        } else {
            (-1, Natural::ONE, u64::exact_from(-t_b))
        };
    }
    // s_b = h0^k with k maximal (k = 1 when s_b is not a perfect power).
    let (h0, k) = s_b.express_as_power().unwrap_or_else(|| (s_b.clone(), 1));
    // base = h0^k * 2^t_b = (2^(t_b / e) * h0^(k / e))^e for any common divisor e of k and t_b; the
    // primitive root takes e maximal.
    let e = if t_b == 0 {
        k
    } else {
        k.gcd(t_b.unsigned_abs())
    };
    (t_b / i64::exact_from(e), h0.pow(k / e), e)
}

// Given a positive `Rational` `g` other than 1, decomposes it as `g = 2^z * hn / hd` with `hn` and
// `hd` odd (coprime) `Natural`s. At most one of the numerator and denominator is even, since they
// are coprime.
pub(crate) fn rational_root_parts(g: &Rational) -> (i64, Natural, Natural) {
    let num = g.numerator_ref();
    let den = g.denominator_ref();
    let num_z = num.trailing_zeros().unwrap();
    let den_z = den.trailing_zeros().unwrap();
    (
        i64::exact_from(num_z) - i64::exact_from(den_z),
        num >> num_z,
        den >> den_z,
    )
}

// Given a positive dyadic value `x = s * 2^t` (`s` odd) and a `Rational` root `g != 1` (`g > 0`),
// returns the integer `m` with `x = g^m`, or `None` if no such integer exists. Writing `g = 2^z *
// hn / hd` with `hn`, `hd` odd and coprime: a positive `m` requires `hd = 1` (an odd denominator
// could never cancel against the dyadic `x`), and a negative `m` symmetrically requires `hn = 1`,
// in which case `x = (2^(-z) * hd)^(-m)`.
pub(crate) fn dyadic_log_of_rational_root(s: &Natural, t: i64, g: &Rational) -> Option<i64> {
    let (z, hn, hd) = rational_root_parts(g);
    if hd == 1u32 {
        dyadic_log_of_root(s, t, z, &hn)
    } else if hn == 1u32 {
        dyadic_log_of_root(s, t, -z, &hd).map(|m| -m)
    } else {
        // Both an odd numerator and an odd denominator: no nonzero power is dyadic.
        None
    }
}

// Given a positive `Rational` `x` and a root `2^z * h` (`h` an odd `Natural`, the root positive and
// different from 1), returns the integer `m` with `x = (2^z * h)^m`, or `None` if no such integer
// exists. All big-number work is on `x`'s odd numerator and denominator parts and on `h`; the
// power-of-2 parts stay as `i64` exponents, so the root's 2-power is never materialized.
pub(crate) fn rational_value_log_of_dyadic_root(x: &Rational, z: i64, h: &Natural) -> Option<i64> {
    let num = x.numerator_ref();
    let den = x.denominator_ref();
    let num_z = num.trailing_zeros().unwrap();
    let den_z = den.trailing_zeros().unwrap();
    let v2 = i64::exact_from(num_z) - i64::exact_from(den_z);
    let on = num >> num_z;
    let od = den >> den_z;
    if *h == 1u32 {
        // The root is 2^z: x = 2^(z * m) requires odd parts 1 and z | v2.
        return if on == 1u32 && od == 1u32 && z != 0 && v2 % z == 0 {
            Some(v2 / z)
        } else {
            None
        };
    }
    // h >= 3: a positive power puts h^m in the numerator, a negative one in the denominator.
    if on != 1u32 && od == 1u32 {
        let m = i64::exact_from(on.checked_log_base(h)?);
        if z.checked_mul(m)? == v2 {
            Some(m)
        } else {
            None
        }
    } else if on == 1u32 && od != 1u32 {
        let m = i64::exact_from(od.checked_log_base(h)?);
        if z.checked_mul(m)? == -v2 {
            Some(-m)
        } else {
            None
        }
    } else {
        // on = od = 1 means x is a power of 2, requiring m = 0 and hence x = 1 (callers handle);
        // on, od > 1 cannot both come from a single power of h.
        None
    }
}

// A large prime modulus for the congruence filter in `dyadic_1p_log_of_root`: 2^64 - 59, the
// largest 64-bit prime.
const FILTER_PRIME: u64 = 0xFFFFFFFFFFFFFFC5;

// Whether `n = h^m`, where `n` is only available as the implicit sum `high * 2^shift + low` (with
// `shift` possibly enormous). A congruence modulo `FILTER_PRIME` proves inequality cheaply -- the
// power and the shift reduce via modular exponentiation -- and only a match (in practice a genuine
// power) is verified exactly, at cost proportional to `shift`.
fn implicit_sum_is_pow(high: &Natural, shift: u64, low: &Integer, h: &Natural, m: u64) -> bool {
    let p = Natural::from(FILTER_PRIME);
    let lhs = (high % &p)
        .mod_mul(Natural::TWO.mod_pow(Natural::from(shift), &p), &p)
        .mod_add(Natural::exact_from(low.mod_op(Integer::from(&p))), &p);
    if (h % &p).mod_pow(Natural::from(m), &p) != lhs {
        return false;
    }
    // The filter passed; verify exactly.
    Integer::from(high << shift) + low == h.pow(m)
}

// Given a `Float` `x` (finite, nonzero, greater than -1) and a root `2^z * h` (`h` an odd
// `Natural`, the root positive and different from 1), returns the integer `m` with `1 + x = (2^z *
// h)^m`, or `None` if no such integer exists.
//
// `1 + x` is never materialized up front: its bit length can be as large as `|EXP(x)|` (up to
// ~2^30) even when `x` itself has few bits. Instead, the structure of `1 + x` -- an implicit sum
// with an odd significand pinned by `x`'s own odd significand `s` and exponent `t` -- determines at
// most a couple of candidate exponents `m`, each checked by a congruence filter that proves
// non-powers unequal; the expensive exact verification runs only for a match, whose cost is
// proportional to the true size of `1 + x`.
pub(crate) fn dyadic_1p_log_of_root(x: &Float, z: i64, h: &Natural) -> Option<i64> {
    let (s, t) = odd_significand_and_exponent(x);
    let neg = *x < 0u32;
    if t >= 0 {
        // x is an integer; x > -1 and x != 0, so x >= 1 and 1 + x = s * 2^t + 1.
        debug_assert!(!neg);
        if t == 0 {
            // 1 + x = s + 1 is small: decompose it and match directly.
            let n = &s + Natural::ONE;
            let r = n.trailing_zeros().unwrap();
            return dyadic_log_of_root(&(n >> r), i64::exact_from(r), z, h);
        }
        // t > 0: 1 + x = s * 2^t + 1 is odd and greater than 1, so the root's power-of-2 part must
        // vanish and h contributes a positive power.
        if z != 0 || *h == 1u32 {
            return None;
        }
        // 1 + x has bit length exactly t + bits(s). Each candidate m must reproduce it.
        let l = u64::exact_from(t) + s.significant_bits();
        for m in pow_bit_length_candidates(h, l) {
            if implicit_sum_is_pow(&s, u64::exact_from(t), &Integer::ONE, h, m) {
                return Some(i64::exact_from(m));
            }
        }
        return None;
    }
    // t < 0: 1 + x = (2^|t| ± s) * 2^t, with an odd numerator n = 2^|t| - s (x < 0) or 2^|t| + s
    // (x > 0). The 2-adic valuation of 1 + x is exactly t, so z * m = t.
    let t_abs = u64::exact_from(-t);
    if *h == 1u32 {
        // Root 2^z: 1 + x = 2^(z * m) requires n = 1, i.e. x < 0 and s = 2^|t| - 1 (an all-ones odd
        // number, checked without materializing 2^|t|).
        return if neg
            && z != 0
            && t % z == 0
            && s.significant_bits() == t_abs
            && (&s + Natural::ONE).is_power_of_2()
        {
            Some(t / z)
        } else {
            None
        };
    }
    if z == 0 || t % z != 0 {
        // An odd root can't produce the nonzero 2-adic valuation t, and a mixed root needs z | t.
        return None;
    }
    // The candidate exponent is pinned by the 2-adic valuation alone: n = h^m needs m >= 1, since n
    // is a positive integer and n = 1 (forcing m = 0) was the h = 1 case above. The congruence
    // filter then proves or refutes n = h^m without materializing n.
    let m = t / z;
    if m <= 0 {
        return None;
    }
    let low = if neg {
        -Integer::from(&s)
    } else {
        Integer::from(&s)
    };
    if implicit_sum_is_pow(&Natural::ONE, t_abs, &low, h, u64::exact_from(m)) {
        Some(m)
    } else {
        None
    }
}

// The integers `m` for which `h^m` (with `h` an odd `Natural` at least 3) can have bit length
// exactly `l`: at most a couple of values, pinned by 80-bit directed bounds on `log_2(h)`.
fn pow_bit_length_candidates(h: &Natural, l: u64) -> Vec<u64> {
    // log_2(h) is irrational (h is odd and at least 3), so directed rounding gives strict bounds.
    let h_float = Float::exact_from(h.clone());
    let lo = Rational::exact_from(h_float.log_base_2_prec_round_ref(80, Floor).0);
    let hi = Rational::exact_from(h_float.log_base_2_prec_round_ref(80, Ceiling).0);
    // h^m has bit length l iff l - 1 <= m * log_2(h) < l.
    let m_min = Integer::rounding_from(Rational::from(l - 1) / hi, Ceiling).0;
    let m_max = Integer::rounding_from(Rational::from(l) / lo, Floor).0;
    let mut candidates = Vec::new();
    let mut m = m_min;
    while m <= m_max && candidates.len() < 4 {
        if m > 0u32 {
            candidates.push(u64::exact_from(&m));
        }
        m += Integer::ONE;
    }
    candidates
}

// `log_base(x)` is rational exactly when `x` and `base` are both powers of a common root `g`, say
// `x = g ^ m` and `base = g ^ e_base`; then `log_base(x) = m / e_base`. Taking `g` to be the
// smallest integer of which `base` is a power (obtained by stripping `base` of perfect-power
// factors via `express_as_power`) and writing `g = 2^z * h` with `h` odd, this holds iff `x`'s odd
// significand is the corresponding power of `h` and its exponent matches (see
// `dyadic_log_of_root`).
//
// Detecting these rational results up front is essential, not just an optimization: when the result
// is exactly representable (for example `log_9(3) = 1/2`), the Ziv loop in
// `log_base_prec_round_normal` would never terminate, because the rounding test can never certify a
// value that sits exactly on a representable point (or exactly on a tie). This generalizes the
// `10^n` exactness check in mpfr_log10, which only catches integer results.
//
// The check is complete and cheap for any input: representable results can have enormous exponents
// with few significant bits (`log_4` of the smallest positive `Float` is `-2^29`, exact at
// precision 1), so no size cutoff on `x`'s exponent is sound; instead the decomposition keeps all
// big-number work on `x`'s odd significand, which has at most `prec(x)` bits.
pub(crate) fn rational_log_base(x: &Float, base: u64) -> Option<Rational> {
    // `express_as_power` returns `None` when `base` is not a perfect power, in which case `base`
    // itself is `g` (with exponent 1).
    let (g, e_base) = base.express_as_power().unwrap_or((base, 1));
    let z = i64::exact_from(g.trailing_zeros());
    let h = Natural::from(g >> g.trailing_zeros());
    let (s, t) = odd_significand_and_exponent(x);
    let m = dyadic_log_of_root(&s, t, z, &h)?;
    Some(Rational::from_signeds(m, i64::exact_from(e_base)))
}

// Returns `Some(m / e_base)` -- the value of `log_base(x)` -- when the positive `Rational` `x`
// equals `g ^ m` for the root `g` of `base` (so `base = g ^ e_base` and `log_base(x)` is rational),
// and `None` when `log_base(x)` is irrational. `x` must be positive and `base > 1`.
//
// `m` (signed) is found by `Rational::checked_log_base`, which also covers `x < 1` (negative `m`).
// Detecting these rational results up front is essential: the Ziv loop could never certify an
// exactly-representable one (see `rational_log_base` for the `Float` analog).
pub(crate) fn rational_log_base_of_rational(x: &Rational, base: u64) -> Option<Rational> {
    let (g, e_base) = base.express_as_power().unwrap_or((base, 1));
    x.checked_log_base(g)
        .map(|m| Rational::from_signeds(m, i64::exact_from(e_base)))
}

// The computation of log_base(x, base) is done by log_base(x) = ln(x) / ln(base). When `base` is a
// power of 2 the caller delegates to `log_base_power_of_2`, so here `base` is not a power of 2.
//
// This is mpfr_log10 from log10.c, MPFR 4.3.0, generalized from base 10 to an arbitrary non-power-
// of-2 `base`. The input is finite, nonzero, and positive.
fn log_base_prec_round_normal(
    x: &Float,
    base: u64,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    // If x is 1, the result is 0.
    if *x == 1u32 {
        return (Float::ZERO, Equal);
    }
    // If log_base(x) is rational -- x and base are both powers of a common integer -- compute it
    // directly. This includes the exactly-representable results (integers like log_8(64) = 2 and
    // dyadics like log_9(3) = 1/2), which the Ziv loop below could never certify, as well as
    // non-representable rationals like log_27(9) = 2/3, which it could but for which the direct
    // computation is cheaper and exact.
    if let Some(q) = rational_log_base(x, base) {
        return Float::from_rational_prec_round(q, prec, rm);
    }
    // log_base(x) for x in a sliver of 1 can fall below the smallest positive Float; the 1-plus-x
    // form handles that underflow region.
    if let Some(d) = sliver_of_one(x) {
        return d.log_base_1_plus_x_prec_round(base, prec, rm);
    }
    // The result is irrational, so it is never exactly representable.
    assert_ne!(rm, Exact, "Inexact log_base");
    let base_float = Float::from(base);
    // Compute the precision of the intermediary variable: the optimal number of bits, see
    // algorithms.tex.
    let mut working_prec = prec + 4 + prec.ceiling_log_base_2();
    let mut increment = Limb::WIDTH;
    loop {
        // ln(x) / ln(base). ln(x), ln(base), and the division are each correctly rounded (at most
        // 1/2 ulp), so the relative error is below 2^(2 - working_prec) and working_prec - 4
        // correct bits suffice for rounding (mpfr_log10 uses Nt - 4).
        let t = x
            .ln_prec_ref(working_prec)
            .0
            .div_prec(base_float.ln_prec_ref(working_prec).0, working_prec)
            .0;
        if float_can_round(t.significand_ref().unwrap(), working_prec - 4, prec, rm) {
            return Float::from_float_prec_round(t, prec, rm);
        }
        // Increase the precision.
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

// Computes log_base(x) for a positive `Rational` x whose logarithm is irrational, in a Ziv loop.
// `base > 1` is not a power of 2.
//
// log_base(x) = log_2(x) / log_2(base). Routing through `log_base_2_rational` (rather than
// computing `ln(x) / ln(base)` directly) reuses its handling of x near a power of 2 -- in
// particular x near 1, where the result is near 0 and a direct computation would need a working
// precision proportional to how close x is to 1. log_2(x), log_2(base), and the division are each
// correctly rounded (at most 1/2 ulp), so the relative error is below 2^(2 - working_prec) and
// working_prec - 4 correct bits suffice for rounding.
fn log_base_rational_prec_round_helper(
    x: &Rational,
    base: u64,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    let base_float = Float::from(base);
    // The initial slack keeps working_prec at least 7, so the working_prec - 6 below stays
    // positive.
    let mut working_prec = prec + 6 + prec.ceiling_log_base_2();
    let mut increment = Limb::WIDTH;
    loop {
        // log_2(x) in the extended exponent range: for x within a sliver of 1 the ordinary Float
        // form would flush to zero or clamp, and the rounding test below could never resolve it.
        let num = extended_log_base_2_of_rational(x, working_prec);
        let den = ExtendedFloat::from(base_float.log_base_2_prec_ref(working_prec).0);
        let (quotient, _) = num.div_prec_val_ref(&den, working_prec);
        // log_2(x) is within 2 ulps, log_2(base) within 1/2, and the division adds 1/2 more, so
        // working_prec - 6 correct bits comfortably suffice.
        if float_can_round(
            quotient.x.significand_ref().unwrap(),
            working_prec - 6,
            prec,
            rm,
        ) {
            let (rounded, o) = Float::from_float_prec_round(quotient.x, prec, rm);
            let mut result = ExtendedFloat::from(rounded);
            result.exp = result.exp.checked_add(quotient.exp).unwrap();
            return result.into_float_helper(prec, rm, o);
        }
        // Increase the precision.
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

impl Float {
    /// Computes $\log_b x$, where $x$ is a [`Float`] and $b$ is a `u64` greater than 1, rounding
    /// the result to the specified precision and with the specified rounding mode. The [`Float`] is
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded value is
    /// less than, equal to, or greater than the exact value. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The base-$b$ logarithm of any nonzero negative number is `NaN`.
    ///
    /// When `base` is a power of 2, this function delegates to
    /// [`Float::log_base_power_of_2_prec_round`]; otherwise it computes $\ln x / \ln b$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,b,p,m) = \log_b x+\varepsilon.
    /// $$
    /// - If $\log_b x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\log_b x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_b x|\rfloor-p+1}$.
    /// - If $\log_b x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_b x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},b,p,m)=\text{NaN}$
    /// - $f(\infty,b,p,m)=\infty$
    /// - $f(-\infty,b,p,m)=\text{NaN}$
    /// - $f(\pm0.0,b,p,m)=-\infty$
    /// - $f(1.0,b,p,m)=0.0$, and the result is exact
    /// - $f(b^n,b,p,m)=n$, rounded to precision $p$; the result is exact if and only if $n$ is
    ///   representable with precision $p$
    /// - $f(x,b,p,m)=\text{NaN}$ for $x<0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::log_base_prec`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::log_base_round`] instead. If both of these things are true, consider using
    /// [`Float::log_base`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, if `base` is less than 2, or if `rm` is `Exact` but the result
    /// cannot be represented exactly with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::from(1000).log_base_prec_round(10, 10, Nearest);
    /// assert_eq!(log.to_string(), "3.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (log, o) = Float::from(50).log_base_prec_round(10, 10, Floor);
    /// assert_eq!(log.to_string(), "1.697");
    /// assert_eq!(o, Less);
    ///
    /// let (log, o) = Float::from(50).log_base_prec_round(10, 10, Ceiling);
    /// assert_eq!(log.to_string(), "1.699");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_prec_round(self, base: u64, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        assert!(base > 1, "Logarithm base must be greater than 1");
        if base.is_power_of_2() {
            return self.log_base_power_of_2_prec_round(i64::from(base.trailing_zeros()), prec, rm);
        }
        match self {
            Self(NaN | Infinity { sign: false } | Finite { sign: false, .. }) => {
                (float_nan!(), Equal)
            }
            float_either_zero!() => (float_negative_infinity!(), Equal),
            float_infinity!() => (float_infinity!(), Equal),
            _ => log_base_prec_round_normal(&self, base, prec, rm),
        }
    }

    /// Computes $\log_b x$, where $x$ is a [`Float`] and $b$ is a `u64` greater than 1, rounding
    /// the result to the specified precision and with the specified rounding mode. The [`Float`] is
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded value
    /// is less than, equal to, or greater than the exact value. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::log_base_prec_round`] for details, special cases, and a description of the
    /// rounding behavior.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, if `base` is less than 2, or if `rm` is `Exact` but the result
    /// cannot be represented exactly with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::from(1000).log_base_prec_round_ref(10, 10, Nearest);
    /// assert_eq!(log.to_string(), "3.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_prec_round_ref(
        &self,
        base: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        assert!(base > 1, "Logarithm base must be greater than 1");
        if base.is_power_of_2() {
            return self.log_base_power_of_2_prec_round_ref(
                i64::from(base.trailing_zeros()),
                prec,
                rm,
            );
        }
        match self {
            Self(NaN | Infinity { sign: false } | Finite { sign: false, .. }) => {
                (float_nan!(), Equal)
            }
            float_either_zero!() => (float_negative_infinity!(), Equal),
            float_infinity!() => (float_infinity!(), Equal),
            _ => log_base_prec_round_normal(self, base, prec, rm),
        }
    }

    /// Computes $\log_b x$, where $x$ is a [`Float`] and $b$ is a `u64` greater than 1, rounding
    /// the result to the nearest value of the specified precision. The [`Float`] is taken by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded value is less than, equal
    /// to, or greater than the exact value.
    ///
    /// See [`Float::log_base_prec_round`] for details and special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `base` is less than 2.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::from(50).log_base_prec(10, 10);
    /// assert_eq!(log.to_string(), "1.699");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_prec(self, base: u64, prec: u64) -> (Self, Ordering) {
        self.log_base_prec_round(base, prec, Nearest)
    }

    /// Computes $\log_b x$, where $x$ is a [`Float`] and $b$ is a `u64` greater than 1, rounding
    /// the result to the nearest value of the specified precision. The [`Float`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded value is less
    /// than, equal to, or greater than the exact value.
    ///
    /// See [`Float::log_base_prec_round`] for details and special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `base` is less than 2.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::from(50).log_base_prec_ref(10, 10);
    /// assert_eq!(log.to_string(), "1.699");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_prec_ref(&self, base: u64, prec: u64) -> (Self, Ordering) {
        self.log_base_prec_round_ref(base, prec, Nearest)
    }

    /// Computes $\log_b x$, where $x$ is a [`Float`] and $b$ is a `u64` greater than 1, rounding
    /// the result to the precision of the input and with the specified rounding mode. The [`Float`]
    /// is taken by value. An [`Ordering`] is also returned, indicating whether the rounded value is
    /// less than, equal to, or greater than the exact value.
    ///
    /// See [`Float::log_base_prec_round`] for details and special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the precision of the input.
    ///
    /// # Panics
    /// Panics if `base` is less than 2, or if `rm` is `Exact` but the result cannot be represented
    /// exactly with the input's precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::from(1000).log_base_round(10, Floor);
    /// assert_eq!(log.to_string(), "3.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_round(self, base: u64, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.log_base_prec_round(base, prec, rm)
    }

    /// Computes $\log_b x$, where $x$ is a [`Float`] and $b$ is a `u64` greater than 1, rounding
    /// the result to the precision of the input and with the specified rounding mode. The [`Float`]
    /// is taken by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// value is less than, equal to, or greater than the exact value.
    ///
    /// See [`Float::log_base_prec_round`] for details and special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the precision of the input.
    ///
    /// # Panics
    /// Panics if `base` is less than 2, or if `rm` is `Exact` but the result cannot be represented
    /// exactly with the input's precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::from(81).log_base_round_ref(3, Ceiling);
    /// assert_eq!(log.to_string(), "4.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_round_ref(&self, base: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.log_base_prec_round_ref(base, self.significant_bits(), rm)
    }

    /// Computes $\log_b x$, where $x$ is a [`Float`] and $b$ is a `u64` greater than 1, in place,
    /// rounding the result to the specified precision and with the specified rounding mode. An
    /// [`Ordering`] is returned, indicating whether the rounded value is less than, equal to, or
    /// greater than the exact value.
    ///
    /// See [`Float::log_base_prec_round`] for details and special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, if `base` is less than 2, or if `rm` is `Exact` but the result
    /// cannot be represented exactly with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(50);
    /// let o = x.log_base_prec_round_assign(10, 10, Floor);
    /// assert_eq!(x.to_string(), "1.697");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn log_base_prec_round_assign(
        &mut self,
        base: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (result, o) = core::mem::take(self).log_base_prec_round(base, prec, rm);
        *self = result;
        o
    }

    /// Computes $\log_b x$, where $x$ is a [`Float`] and $b$ is a `u64` greater than 1, in place,
    /// rounding the result to the nearest value of the specified precision. An [`Ordering`] is
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value.
    ///
    /// See [`Float::log_base_prec_round`] for details and special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `base` is less than 2.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(1000);
    /// let o = x.log_base_prec_assign(10, 10);
    /// assert_eq!(x.to_string(), "3.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_prec_assign(&mut self, base: u64, prec: u64) -> Ordering {
        self.log_base_prec_round_assign(base, prec, Nearest)
    }

    /// Computes $\log_b x$, where $x$ is a [`Float`] and $b$ is a `u64` greater than 1, in place,
    /// rounding the result to the precision of the input and with the specified rounding mode. An
    /// [`Ordering`] is returned, indicating whether the rounded value is less than, equal to, or
    /// greater than the exact value.
    ///
    /// See [`Float::log_base_prec_round`] for details and special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the precision of the input.
    ///
    /// # Panics
    /// Panics if `base` is less than 2, or if `rm` is `Exact` but the result cannot be represented
    /// exactly with the input's precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(81);
    /// let o = x.log_base_round_assign(3, Nearest);
    /// assert_eq!(x.to_string(), "4.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_round_assign(&mut self, base: u64, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.log_base_prec_round_assign(base, prec, rm)
    }

    /// Computes $\log_b x$, where $x$ is a [`Rational`] and $b$ is a `u64` greater than 1, rounding
    /// the result to the specified precision and with the specified rounding mode and returning the
    /// result as a [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// The base-$b$ logarithm of any negative number is `NaN`.
    ///
    /// Inputs of any magnitude are handled, including [`Rational`]s whose magnitudes are too large
    /// or too small to be representable as [`Float`]s. Neither overflow nor underflow of the output
    /// is possible.
    ///
    /// When `base` is a power of 2, this function delegates to
    /// [`Float::log_base_power_of_2_rational_prec_round`].
    ///
    /// See [`Float::log_base_prec_round`] for details and a description of the rounding behavior.
    ///
    /// Special cases:
    /// - $f(0,b,p,m)=-\infty$
    /// - $f(x,b,p,m)=\text{NaN}$ for $x<0$
    /// - $f(1,b,p,m)=0.0$, and the result is exact
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, if `base` is less than 2, or if `rm` is `Exact` but the result
    /// cannot be represented exactly with the given precision. (The result is exactly representable
    /// if and only if $x \leq 0$ or $\log_b x$ is rational and representable with the given
    /// precision.)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::log_base_rational_prec_round(Rational::from(3), 9, 10, Exact);
    /// assert_eq!(log.to_string(), "0.5"); // log_9(3) = 1/2
    /// assert_eq!(o, Equal);
    ///
    /// let (log, o) = Float::log_base_rational_prec_round(Rational::from(2), 3, 20, Nearest);
    /// assert_eq!(log.to_string(), "0.63093");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn log_base_rational_prec_round(
        x: Rational,
        base: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::log_base_rational_prec_round_ref(&x, base, prec, rm)
    }

    /// Computes $\log_b x$, where $x$ is a [`Rational`] and $b$ is a `u64` greater than 1, rounding
    /// the result to the specified precision and with the specified rounding mode and returning the
    /// result as a [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::log_base_rational_prec_round`] for details, special cases, and a description of
    /// the rounding behavior.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, if `base` is less than 2, or if `rm` is `Exact` but the result
    /// cannot be represented exactly with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) =
    ///     Float::log_base_rational_prec_round_ref(&Rational::from_signeds(1, 9), 3, 10, Exact);
    /// assert_eq!(log.to_string(), "-2.0"); // log_3(1/9) = -2
    /// assert_eq!(o, Equal);
    /// ```
    pub fn log_base_rational_prec_round_ref(
        x: &Rational,
        base: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        assert!(base > 1, "Logarithm base must be greater than 1");
        if base.is_power_of_2() {
            return Self::log_base_power_of_2_rational_prec_round_ref(
                x,
                i64::from(base.trailing_zeros()),
                prec,
                rm,
            );
        }
        match x.sign() {
            Equal => return (float_negative_infinity!(), Equal),
            Less => return (float_nan!(), Equal),
            Greater => {}
        }
        // If x = g^m for the base's root g (so base = g^e_base), then log_base(x) = m / e_base is
        // rational, and exact -- the Ziv loop could never certify it (see rational_log_base).
        if let Some(q) = rational_log_base_of_rational(x, base) {
            return Self::from_rational_prec_round(q, prec, rm);
        }
        // The result is irrational, so it is never exactly representable.
        assert_ne!(rm, Exact, "Inexact log_base");
        log_base_rational_prec_round_helper(x, base, prec, rm)
    }

    /// Computes $\log_b x$, where $x$ is a [`Rational`] and $b$ is a `u64` greater than 1, rounding
    /// the result to the nearest value of the specified precision and returning the result as a
    /// [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded value is less than, equal to, or greater than the exact value.
    ///
    /// See [`Float::log_base_rational_prec_round`] for details and special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `base` is less than 2.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::log_base_rational_prec(Rational::from_signeds(1, 9), 3, 10);
    /// assert_eq!(log.to_string(), "-2.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_rational_prec(x: Rational, base: u64, prec: u64) -> (Self, Ordering) {
        Self::log_base_rational_prec_round(x, base, prec, Nearest)
    }

    /// Computes $\log_b x$, where $x$ is a [`Rational`] and $b$ is a `u64` greater than 1, rounding
    /// the result to the nearest value of the specified precision and returning the result as a
    /// [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value.
    ///
    /// See [`Float::log_base_rational_prec_round`] for details and special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `base` is less than 2.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::log_base_rational_prec_ref(&Rational::from(2), 3, 20);
    /// assert_eq!(log.to_string(), "0.63093");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_rational_prec_ref(x: &Rational, base: u64, prec: u64) -> (Self, Ordering) {
        Self::log_base_rational_prec_round_ref(x, base, prec, Nearest)
    }
}

impl LogBase<u64> for Float {
    type Output = Self;

    /// Computes $\log_b x$, where $x$ is a [`Float`] and $b$ is a `u64` greater than 1, rounding
    /// the result to the nearest value of the input's precision. The [`Float`] is taken by value.
    ///
    /// The base-$b$ logarithm of any nonzero negative number is `NaN`. See
    /// [`Float::log_base_prec_round`] for the special cases.
    ///
    /// $$
    /// f(x,b) = \log_b x+\varepsilon,
    /// $$
    /// where $|\varepsilon| \leq 2^{\lfloor\log_2 |\log_b x|\rfloor-p}$ and $p$ is the precision of
    /// the input.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the precision of the input.
    ///
    /// # Panics
    /// Panics if `base` is less than 2.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::LogBase;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::from(1000).log_base(10).to_string(), "3.0");
    /// assert_eq!(Float::from(81).log_base(3).to_string(), "4.0");
    /// ```
    #[inline]
    fn log_base(self, base: u64) -> Self {
        let prec = self.significant_bits();
        self.log_base_prec_round(base, prec, Nearest).0
    }
}

impl LogBase<u64> for &Float {
    type Output = Float;

    /// Computes $\log_b x$, where $x$ is a [`Float`] and $b$ is a `u64` greater than 1, rounding
    /// the result to the nearest value of the input's precision. The [`Float`] is taken by
    /// reference.
    ///
    /// The base-$b$ logarithm of any nonzero negative number is `NaN`. See
    /// [`Float::log_base_prec_round`] for the special cases.
    ///
    /// $$
    /// f(x,b) = \log_b x+\varepsilon,
    /// $$
    /// where $|\varepsilon| \leq 2^{\lfloor\log_2 |\log_b x|\rfloor-p}$ and $p$ is the precision of
    /// the input.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the precision of the input.
    ///
    /// # Panics
    /// Panics if `base` is less than 2.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::LogBase;
    /// use malachite_float::Float;
    ///
    /// assert_eq!((&Float::from(1000)).log_base(10).to_string(), "3.0");
    /// ```
    #[inline]
    fn log_base(self, base: u64) -> Float {
        self.log_base_prec_round_ref(base, self.significant_bits(), Nearest)
            .0
    }
}

impl LogBaseAssign<u64> for Float {
    /// Replaces a [`Float`] $x$ with $\log_b x$, where $b$ is a `u64` greater than 1, rounding the
    /// result to the nearest value of the input's precision.
    ///
    /// The base-$b$ logarithm of any nonzero negative number is `NaN`. See
    /// [`Float::log_base_prec_round`] for the special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the precision of the input.
    ///
    /// # Panics
    /// Panics if `base` is less than 2.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::LogBaseAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(1000);
    /// x.log_base_assign(10);
    /// assert_eq!(x.to_string(), "3.0");
    /// ```
    #[inline]
    fn log_base_assign(&mut self, base: u64) {
        let prec = self.significant_bits();
        self.log_base_prec_round_assign(base, prec, Nearest);
    }
}

/// Computes $\log_b x$, the base-$b$ logarithm of a primitive float, where $b$ is a `u64` greater
/// than 1. Using this function is more accurate than computing the logarithm using the standard
/// library, whose `log` is not always correctly rounded.
///
/// The base-$b$ logarithm of any negative number is `NaN`.
///
/// $$
/// f(x,b) = \log_b x+\varepsilon.
/// $$
/// - If $\log_b x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $\log_b x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |\log_b
///   x|\rfloor-p}$, where $p$ is precision of the output (typically 24 if `T` is a [`f32`] and 53
///   if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(\text{NaN},b)=\text{NaN}$
/// - $f(\infty,b)=\infty$
/// - $f(-\infty,b)=\text{NaN}$
/// - $f(\pm0.0,b)=-\infty$
/// - $f(1.0,b)=0.0$
/// - $f(x,b)=\text{NaN}$ for $x<0$
///
/// Neither overflow nor underflow is possible.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `base` is less than 2.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::arithmetic::log_base::primitive_float_log_base;
///
/// assert!(primitive_float_log_base(f32::NAN, 10).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_log_base(f32::INFINITY, 10)),
///     NiceFloat(f32::INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_log_base(0.0f32, 10)),
///     NiceFloat(f32::NEGATIVE_INFINITY)
/// );
/// // log_10(1000) = 3
/// assert_eq!(
///     NiceFloat(primitive_float_log_base(1000.0f32, 10)),
///     NiceFloat(3.0)
/// );
/// // log_3(9) = 2
/// assert_eq!(
///     NiceFloat(primitive_float_log_base(9.0f32, 3)),
///     NiceFloat(2.0)
/// );
/// // log_10(50)
/// assert_eq!(
///     NiceFloat(primitive_float_log_base(50.0f32, 10)),
///     NiceFloat(1.69897)
/// );
/// assert!(primitive_float_log_base(-1.0f32, 10).is_nan());
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_log_base<T: PrimitiveFloat>(x: T, base: u64) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(|x, prec| Float::log_base_prec(x, base, prec), x)
}

/// Computes $\log_b x$, the base-$b$ logarithm of a [`Rational`], where $b$ is a `u64` greater than
/// 1, returning a primitive float result.
///
/// If the logarithm is equidistant from two primitive floats, the primitive float with fewer 1s in
/// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding
/// mode.
///
/// The base-$b$ logarithm of any negative number is `NaN`.
///
/// $$
/// f(x,b) = \log_b x+\varepsilon.
/// $$
/// - If $\log_b x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $\log_b x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |\log_b
///   x|\rfloor-p}$, where $p$ is precision of the output (typically 24 if `T` is a [`f32`] and 53
///   if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(0,b)=-\infty$
/// - $f(x,b)=\text{NaN}$ for $x<0$
/// - $f(1,b)=0.0$
///
/// Neither overflow nor underflow is possible.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `base` is less than 2.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::{NegativeInfinity, Zero};
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::arithmetic::log_base::primitive_float_log_base_rational;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_rational::<f64>(
///         &Rational::ZERO,
///         10
///     )),
///     NiceFloat(f64::NEGATIVE_INFINITY)
/// );
/// // log_10(1000) = 3
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_rational::<f64>(
///         &Rational::from(1000),
///         10
///     )),
///     NiceFloat(3.0)
/// );
/// // log_3(1/9) = -2
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_rational::<f64>(
///         &Rational::from_unsigneds(1u8, 9),
///         3
///     )),
///     NiceFloat(-2.0)
/// );
/// // log_10(1/3)
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_rational::<f64>(
///         &Rational::from_unsigneds(1u8, 3),
///         10
///     )),
///     NiceFloat(-0.47712125471966244)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_rational::<f64>(
///         &Rational::from(-1000),
///         10
///     )),
///     NiceFloat(f64::NAN)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_log_base_rational<T: PrimitiveFloat>(x: &Rational, base: u64) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_to_float_fn(
        |x, prec| Float::log_base_rational_prec_ref(x, base, prec),
        x,
    )
}
