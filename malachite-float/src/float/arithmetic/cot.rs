// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright © 2005-2025 Free Software Foundation, Inc.
//
//      Contributed by the Pascaline and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

// Port of MPFR's cotangent. `mpfr_cot` (`cot.c`) instantiates the generic reciprocal template
// (`gen_inverse.h`) with the tangent, and MPFR's tangent is in turn a quotient of a sine and a
// cosine; taking cot x as cos x / sin x directly saves the middle rounding and gives the two ends
// of the exponent range the same treatment, so that is what the Ziv loop below does, with the same
// working precision and the same two bits of slack. Unlike the secant and the cosecant, the
// cotangent is not bounded away from zero, so it both overflows, within 2^(-2^30) of a multiple of
// pi, and underflows, within 2^(-2^30) of an odd multiple of pi/2; each end is decided from an
// exact bracket. MPFR's shortcut for a tiny input, where cot x is 1/x - x/3 + O(x^3), is kept: the
// quotient there is exactly representable, so the Ziv loop could never certify it.

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::float::arithmetic::cos::{
    reduce_huge, signed_constant, sin_bound, trig_near_zero_bracket,
    trig_rational_near_zero_bracket, trig_turns_near_zero_bracket,
};
use crate::float::arithmetic::sin_cos::{
    sin_cos_rational_helper, sin_cos_turns_helper, sin_cos_with_period_prec_round_normal_ref,
};
use crate::float::arithmetic::tan::{
    MAX_CANCEL, MAX_SETTLED_EXPONENT, MIN_SETTLED_EXPONENT, nearest_bracket, round_bracket_signed,
    round_bracket_signed_by,
};
use crate::{Float, emulate_float_to_float_fn, emulate_rational_to_float_fn};
use core::cmp::Ordering::{self, Equal, Greater, Less};
use core::cmp::max;
use malachite_base::num::arithmetic::traits::{
    Abs, AddMul, CeilingLogBase2, Cot, CotAssign, IsPowerOf2, Mod, Parity, Pow, Reciprocal, Square,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, NegativeInfinity,
    NegativeZero as NegativeZeroTrait, One, Zero as ZeroTrait,
};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, Ceiling, Down, Exact, Floor, Nearest};
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::float::round::float_can_round;
use malachite_nz::platform::Limb;
use malachite_q::Rational;

// cot x for a tiny x, where cot x = 1/x - x/3 - ... and |cot x - 1/x| <= 0.36 for |x| <= 1, with
// the correction opposing the sign of 1/x, so that |cot x| < |1/x|. MPFR's condition, EXP(x) + 1 <=
// -2 max(PREC(x), prec), makes rounding 1/x settle the cotangent, except when 1/x is exact (x a
// power of 2), where the true value lies one step short of it, toward zero. The general loop could
// not settle that case at any working precision, since the quotient is then exactly representable.
//
// This is ACTION_TINY from cot.c, MPFR 4.2.2.
fn cot_tiny(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    let (r, o) = x.reciprocal_prec_round_ref(prec, rm);
    if o != Equal {
        return (r, o);
    }
    assert_ne!(rm, Exact, "Inexact cot");
    let negative = x.is_sign_negative();
    // 1/x is exact, so the cotangent is one step short of it, toward zero
    let toward = match rm {
        Floor => !negative,
        Ceiling => negative,
        Down => true,
        _ => false,
    };
    let mut r = r;
    if toward {
        if negative {
            r.increment();
        } else {
            r.decrement();
        }
        (r, if negative { Greater } else { Less })
    } else {
        (r, if negative { Less } else { Greater })
    }
}

// As in mpfr_overflow, with the overflow's sign: the toward-zero modes give the largest finite
// value, and the other modes an infinity.
fn cot_overflow(negative: bool, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    match (negative, rm) {
        (_, Exact) => panic!("Inexact cot"),
        (false, Floor | Down) => (Float::max_finite_value_with_prec(prec), Less),
        (false, _) => (Float::INFINITY, Greater),
        (true, Ceiling | Down) => (-Float::max_finite_value_with_prec(prec), Greater),
        (true, _) => (Float::NEGATIVE_INFINITY, Less),
    }
}

// Decides cot(x) = c/s from the sine and cosine rounded to nearest at precision m, by a `Rational`
// bracket, for the cases the `Float` quotient cannot settle: it overflowed, underflowed, or lies
// within two bits of either end of the exponent range, or the sine or cosine underflowed. Returns
// `None` if the bracket does not decide the rounding, so that the working precision must grow.
fn cot_bracket(
    x: &Float,
    s: &Float,
    c: &Float,
    m: u64,
    prec: u64,
    rm: RoundingMode,
) -> Option<(Float, Ordering)> {
    let negative = s.is_sign_negative() != c.is_sign_negative();
    // A sine that underflowed is at most 1.5 times the smallest positive `Float`, and the cosine is
    // then within 2^-m of 1, so the cotangent is at least 2^(2^30)/1.5 in magnitude, beyond the
    // largest finite `Float`.
    if *s == 0u32
        || (s.get_exponent() == Some(Float::MIN_EXPONENT)
            && s.significand_ref().unwrap().is_power_of_2())
    {
        return Some(cot_overflow(negative, prec, rm));
    }
    let (s_lo, s_hi) = nearest_bracket(s, m);
    let (c_lo, c_hi) = if *c == 0u32 {
        // The cosine underflowed, so its rounding says only that it is below half the smallest
        // positive `Float`, and the cotangent, barely larger than the cosine, cannot be placed
        // against that same bound: take the cosine's exact bracket from the distance to the nearest
        // odd multiple of pi/2, as the near-zero path does.
        let (lo, hi) = trig_near_zero_bracket(x, m + 64, MAX_CANCEL, true);
        if lo < 0u32 { (-hi, -lo) } else { (lo, hi) }
    } else {
        nearest_bracket(c, m)
    };
    round_bracket_signed_by(negative, c_lo / s_hi, c_hi / s_lo, prec, rm)
}

// This is mpfr_cot from cot.c, MPFR 4.2.2, with the tangent's own quotient of a sine and a cosine
// folded in and the bracket path for results near the ends of the exponent range.
fn cot_prec_round_normal_ref(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact cot");
    let exp_x = i64::from(x.get_exponent().unwrap());
    // ACTION_TINY from cot.c: EXP(x) + 1 <= -2 max(PREC(x), PREC(y))
    let n = i64::exact_from(max(x.get_prec().unwrap(), prec));
    if exp_x < -(n << 1) {
        return cot_tiny(x, prec, rm);
    }
    // Compute initial precision
    let mut m = prec + prec.ceiling_log_base_2() + 13;
    let mut increment = Limb::WIDTH;
    loop {
        // err <= 1/2 ulp on s and c, each correctly rounded even within 2^(-2^30) of a zero of its
        // function, where it may underflow
        let (s, c, _, _) = x.sin_cos_prec_ref(m);
        // err <= 4 ulps
        let q = if s == 0u32 || c == 0u32 {
            None
        } else {
            Some(c.div_prec_ref_ref(&s, m).0)
        };
        // A quotient that overflowed, underflowed, or lies within two bits of either end of the
        // exponent range, where rounding it to `prec` could still cross the end, is decided from
        // brackets, as is a sine or cosine that underflowed.
        match q.as_ref().and_then(Float::get_exponent).map(i64::from) {
            Some(e) if e > MIN_SETTLED_EXPONENT && e < MAX_SETTLED_EXPONENT => {
                let q = q.unwrap();
                if float_can_round(q.significand_ref().unwrap(), m - 2, prec, rm) {
                    return Float::from_float_prec_round(q, prec, rm);
                }
            }
            _ => {
                if let Some(result) = cot_bracket(x, &s, &c, m, prec, rm) {
                    return result;
                }
            }
        }
        m += increment;
        increment = m >> 1;
    }
}

// cot x for a tiny nonzero `Rational` x whose two-term bracket straddles a rounding boundary: the
// sine is bracketed by `sin_bound` at a growing working precision, and the cosine by consecutive
// partial sums of its alternating series, until the quotient's bracket rounds unambiguously (the
// cotangent is transcendental, so it eventually does). This is `tan_rational_tiny` with the
// quotient the other way up.
fn cot_rational_tiny(
    x: &Rational,
    ax: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    let x2 = ax.square();
    let mut w = prec + 64;
    let mut terms = 2u64;
    loop {
        let s_lo = sin_bound(ax, w, false);
        let s_hi = sin_bound(ax, w, true);
        // cos x = 1 - x^2/2 + x^4/24 - ..., an alternating series with decreasing terms for |x| <=
        // 1, so the partial sums with an even and an odd number of terms bracket it
        let mut c_lo = Rational::ONE;
        let mut term = Rational::ONE;
        let mut c_hi = Rational::ONE;
        for k in 1..=terms {
            term *= &x2;
            term /= Rational::from((k << 1) * ((k << 1) - 1));
            if k.odd() {
                c_lo = &c_hi - &term;
            } else {
                c_hi = &c_lo + &term;
            }
        }
        let lo = c_lo / s_hi;
        let hi = c_hi / s_lo;
        if let Some(result) = round_bracket_signed(x, lo, hi, prec, rm) {
            return result;
        }
        w <<= 1;
        terms += 1;
    }
}

// `cot_bracket` for a `Rational` input: the cosine's exact bracket, when it underflowed, comes from
// the `Rational` near-zero machinery, on the input reduced modulo 2 pi if it is too large to be a
// `Float`.
fn cot_rational_bracket(
    x: &Rational,
    exp_x: i64,
    s: &Float,
    c: &Float,
    m: u64,
    prec: u64,
    rm: RoundingMode,
) -> Option<(Float, Ordering)> {
    let negative = s.is_sign_negative() != c.is_sign_negative();
    if *s == 0u32
        || (s.get_exponent() == Some(Float::MIN_EXPONENT)
            && s.significand_ref().unwrap().is_power_of_2())
    {
        return Some(cot_overflow(negative, prec, rm));
    }
    let (s_lo, s_hi) = nearest_bracket(s, m);
    let (c_lo, c_hi) = if *c == 0u32 {
        let w = m + 64;
        let reduced;
        let (y, extra) = if exp_x >= Float::MAX_EXPONENT_I64 {
            reduced = reduce_huge(x, exp_x, w);
            (&reduced, Some(2 - i64::exact_from(w)))
        } else {
            (x, None)
        };
        let exp_y = y.floor_log_base_2_abs() + 1;
        let (lo, hi) = trig_rational_near_zero_bracket(y, exp_y, extra, w, m, true);
        if lo < 0u32 { (-hi, -lo) } else { (lo, hi) }
    } else {
        nearest_bracket(c, m)
    };
    round_bracket_signed_by(negative, c_lo / s_hi, c_hi / s_lo, prec, rm)
}

// Computes cot(x) for a nonzero `Rational` x, rounded to precision `prec` with rounding mode `rm`.
// (x = 0 is handled by the caller.) The result is never exactly representable, so `rm` must not be
// `Exact`.
//
// This is the `Float` algorithm with the sine and cosine taken from `sin_cos_rational_helper`,
// which rounds the input once and shares the argument reduction, and with a direct bracket for a
// tiny input, inverting the tangent's own series bracket: that also covers inputs below the `Float`
// exponent range, which no other path could even round, and the powers of 2, whose reciprocals are
// exactly representable and which the general loop could therefore never certify.
pub(crate) fn cot_rational_helper(x: &Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact cot");
    let exp_x = x.floor_log_base_2_abs() + 1; // the MPFR-style exponent of x
    // For |x| <= 1/2, |x| + |x|^3/3 <= |tan x| <= |x| + |x|^3/3 + |x|^5, so the cotangent lies
    // between the reciprocals of those, a bracket of relative width below x^4, which decides the
    // rounding once x^4 is below 2^-(prec + 3), unless the cotangent lies within that of a rounding
    // boundary.
    if exp_x < 0 && -(exp_x << 2) > i64::exact_from(prec) + 3 {
        let ax = x.abs();
        let ax3 = (&ax).pow(3u64);
        let t_lo = &ax + &ax3 / const { Rational::const_from_unsigned(3) };
        let t_hi = (&t_lo).add_mul(&ax3, &(&ax).square());
        if let Some(result) =
            round_bracket_signed(x, t_hi.reciprocal(), t_lo.reciprocal(), prec, rm)
        {
            return result;
        }
        // The bracket straddles a rounding boundary; tighten it from the series of the sine and
        // cosine, which narrows without bound.
        return cot_rational_tiny(x, &ax, prec, rm);
    }
    let mut m = prec + prec.ceiling_log_base_2() + 13;
    let mut increment = Limb::WIDTH;
    loop {
        // the sine and cosine correctly rounded at m, even within 2^(-2^30) of a zero of either,
        // where they may underflow
        let (s, c, _, _) = sin_cos_rational_helper(x, m, Nearest);
        // err <= 4 ulps
        let q = if s == 0u32 || c == 0u32 {
            None
        } else {
            Some(c.div_prec_ref_ref(&s, m).0)
        };
        match q.as_ref().and_then(Float::get_exponent).map(i64::from) {
            Some(e) if e > MIN_SETTLED_EXPONENT && e < MAX_SETTLED_EXPONENT => {
                let q = q.unwrap();
                if float_can_round(q.significand_ref().unwrap(), m - 2, prec, rm) {
                    return Float::from_float_prec_round(q, prec, rm);
                }
            }
            _ => {
                if let Some(result) = cot_rational_bracket(x, exp_x, &s, &c, m, prec, rm) {
                    return result;
                }
            }
        }
        m += increment;
        increment = m >> 1;
    }
}

// The closed-form cases of cot(2 pi x / u), keyed by the denominator d of x/u in lowest terms (with
// 0 < |x/u| < 1, so d > 1 and the numerator n is the angle in units of 1/d of a turn). These are
// the tangent's, reciprocated: an odd multiple of 1/4 of a turn, where the tangent has a pole, is a
// zero of the cotangent, and a multiple of 1/2 is the other way about; an odd multiple of 1/8 gives
// 1 or -1, as it does for the tangent; and d = 3 or 6 gives sqrt(3)/3 while d = 12 gives sqrt(3),
// up to sign. Those two constants are never exact, so they return `None` for `Exact`.
fn cot_turns_special_case(q: &Rational, prec: u64, rm: RoundingMode) -> Option<(Float, Ordering)> {
    let d = q.denominator_ref();
    if *d > 12u32 {
        return None;
    }
    let d = u64::exact_from(d);
    let negative = *q < 0u32;
    // the angle in units of 1/d of a turn (the numerator of a `Rational` is unsigned, so the sign
    // is restored before reducing modulo d)
    let n = u64::exact_from(
        &Integer::from_sign_and_abs_ref(!negative, q.numerator_ref()).mod_op(Integer::from(d)),
    );
    match d {
        // eighths of a turn; n cannot be 0, since 0 < |q| < 1
        2 | 4 | 8 => Some(match n * (8 / d) {
            // the pole at 180°, where the sine is a zero with the sign of q and the cosine is -1
            4 => (
                if negative {
                    Float::INFINITY
                } else {
                    Float::NEGATIVE_INFINITY
                },
                Equal,
            ),
            // cot(90°) = +0, cot(270°) = -0
            2 => (Float::ZERO, Equal),
            6 => (Float::NEGATIVE_ZERO, Equal),
            // cot(45°) = cot(225°) = 1, cot(135°) = cot(315°) = -1
            1 | 5 => (Float::one_prec(prec), Equal),
            _ => (-Float::one_prec(prec), Equal),
        }),
        _ if rm == Exact => None,
        // twelfths of a turn
        3 | 6 | 12 => Some(match n * (12 / d) {
            // cot(30°) = cot(210°) = sqrt(3), cot(150°) = cot(330°) = -sqrt(3)
            1 | 7 => signed_constant(Float::sqrt_3_prec_round, false, prec, rm),
            5 | 11 => signed_constant(Float::sqrt_3_prec_round, true, prec, rm),
            // cot(60°) = cot(240°) = sqrt(3)/3, cot(120°) = cot(300°) = -sqrt(3)/3
            2 | 8 => signed_constant(Float::sqrt_3_over_3_prec_round, false, prec, rm),
            _ => signed_constant(Float::sqrt_3_over_3_prec_round, true, prec, rm),
        }),
        _ => None,
    }
}

// `cot_bracket` for an argument in u ths of a turn: the cosine's exact bracket, when it
// underflowed, comes from the distance of the fraction of a turn to the nearest odd multiple of
// 1/4, as the near-zero path uses it. `q` produces that fraction, which the `Float` caller forms
// only here.
fn cot_turns_bracket<F: Fn() -> Rational>(
    q: F,
    s: &Float,
    c: &Float,
    m: u64,
    prec: u64,
    rm: RoundingMode,
) -> Option<(Float, Ordering)> {
    let negative = s.is_sign_negative() != c.is_sign_negative();
    if *s == 0u32
        || (s.get_exponent() == Some(Float::MIN_EXPONENT)
            && s.significand_ref().unwrap().is_power_of_2())
    {
        return Some(cot_overflow(negative, prec, rm));
    }
    let (s_lo, s_hi) = nearest_bracket(s, m);
    let (c_lo, c_hi) = if *c == 0u32 {
        let (lo, hi) = trig_turns_near_zero_bracket(&q(), m, true)?;
        if lo < 0u32 { (-hi, -lo) } else { (lo, hi) }
    } else {
        nearest_bracket(c, m)
    };
    round_bracket_signed_by(negative, c_lo / s_hi, c_hi / s_lo, prec, rm)
}

// Computes cot(2 pi q) for a nonzero `Rational` fraction of a turn q in (-1, 1), rounded to
// precision `prec` with rounding mode `rm`. `rm` may be `Exact` only in the exact cases (see
// `cot_turns_special_case`). This is the `Float` algorithm with the fraction of a turn taken
// directly: since q is exact, only pi and the sine and cosine are rounded, and no argument
// reduction is needed beyond the exact one the caller has already done. The tangent's shortcut for
// a tiny q is not needed here either: `sin_cos_turns_helper` rounds the sine through its own
// near-zero path, and a sine that underflows there, with the cosine at 1, is the overflow the
// bracket reads it as.
fn cot_turns_helper(q: &Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    let exp_q = q.floor_log_base_2_abs() + 1;
    // The special cases need |q| >= 1/12
    if exp_q >= -4
        && let Some(result) = cot_turns_special_case(q, prec, rm)
    {
        return result;
    }
    // Only the exact cases can be rounded exactly
    assert_ne!(rm, Exact, "Inexact cot_with_period");
    let mut m = prec + prec.ceiling_log_base_2() + 13;
    let mut increment = Limb::WIDTH;
    loop {
        // err <= 1/2 ulp on s and c, each correctly rounded even within 2^(-2^30) of a zero of its
        // function, where it may underflow
        let (s, c, _, _) = sin_cos_turns_helper(q, m, Nearest);
        // err <= 4 ulps
        let t = if s == 0u32 || c == 0u32 {
            None
        } else {
            Some(c.div_prec_ref_ref(&s, m).0)
        };
        // as in the `Float` version, a quotient at either end of the exponent range, or a sine or
        // cosine that underflowed, is decided from brackets
        match t.as_ref().and_then(Float::get_exponent).map(i64::from) {
            Some(e) if e > MIN_SETTLED_EXPONENT && e < MAX_SETTLED_EXPONENT => {
                let t = t.unwrap();
                if float_can_round(t.significand_ref().unwrap(), m - 2, prec, rm) {
                    return Float::from_float_prec_round(t, prec, rm);
                }
            }
            _ => {
                if let Some(result) = cot_turns_bracket(|| q.clone(), &s, &c, m, prec, rm) {
                    return result;
                }
            }
        }
        m += increment;
        increment = m >> 1;
    }
}

// Computes cot(2 pi x/u) for a finite nonzero `Float` x and a nonzero u, rounded to precision
// `prec` with rounding mode `rm`. `rm` may be `Exact` only in the exact cases (see
// `cot_turns_special_case`).
//
// This has no MPFR counterpart; it is `cot` with the sine and cosine taken in u ths of a turn,
// which reduces the argument exactly rather than modulo an approximation of 2 pi, and so reaches
// the exact and closed-form cases that the radian version cannot see. The tangent's shortcut for a
// tiny x/u is not needed: there the sine underflows while the cosine is 1, and the bracket reads
// that as the overflow it is.
fn cot_with_period_prec_round_normal_ref(
    x: &Float,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    // Range reduction, as in `tan_with_period`: the argument is already reduced if |x| < u.
    let xr;
    let xp = if x.lt_abs(&u) {
        x
    } else {
        // xr = x mod u, with the sign of x, exactly
        let p = i64::exact_from(x.get_prec().unwrap()) - i64::from(x.get_exponent().unwrap());
        let (r, o) =
            x.rem_unsigned_prec_round_ref(u, u64::WIDTH + u64::exact_from(max(p, 0)), Exact);
        assert_eq!(o, Equal);
        if r == 0u32 {
            // x is a multiple of u, so the sine is a zero with the sign of x, the cosine is 1, and
            // the cotangent is an infinity with that sign
            return (
                if *x < 0u32 {
                    Float::NEGATIVE_INFINITY
                } else {
                    Float::INFINITY
                },
                Equal,
            );
        }
        xr = r;
        &xr
    };
    // now |xp/u| < 1
    let exp_x = i64::from(xp.get_exponent().unwrap());
    // The special cases need |x/u| >= 1/12, so the exponent test skips the `Rational` construction
    // for the small x that would make it expensive (a tiny x has a huge power-of-2 denominator).
    if exp_x >= i64::exact_from(u.significant_bits()) - 4
        && let Some(result) =
            cot_turns_special_case(&(Rational::exact_from(xp) / Rational::from(u)), prec, rm)
    {
        return result;
    }
    // Only the exact cases can be rounded exactly
    assert_ne!(rm, Exact, "Inexact cot_with_period");
    let mut m = prec + prec.ceiling_log_base_2() + 13;
    let mut increment = Limb::WIDTH;
    loop {
        // err <= 1/2 ulp on s and c, each correctly rounded even within 2^(-2^30) of a zero of its
        // function, where it may underflow
        let (s, c, _, _) = sin_cos_with_period_prec_round_normal_ref(xp, u, m, Nearest);
        // err <= 4 ulps
        let q = if s == 0u32 || c == 0u32 {
            None
        } else {
            Some(c.div_prec_ref_ref(&s, m).0)
        };
        // A quotient that overflowed, underflowed, or lies within two bits of either end of the
        // exponent range, where rounding it to `prec` could still cross the end, is decided from
        // brackets, as is a sine or cosine that underflowed.
        match q.as_ref().and_then(Float::get_exponent).map(i64::from) {
            Some(e) if e > MIN_SETTLED_EXPONENT && e < MAX_SETTLED_EXPONENT => {
                let q = q.unwrap();
                if float_can_round(q.significand_ref().unwrap(), m - 2, prec, rm) {
                    return Float::from_float_prec_round(q, prec, rm);
                }
            }
            _ => {
                if let Some(result) = cot_turns_bracket(
                    || Rational::exact_from(xp) / Rational::from(u),
                    &s,
                    &c,
                    m,
                    prec,
                    rm,
                ) {
                    return result;
                }
            }
        }
        m += increment;
        increment = m >> 1;
    }
}

impl Float {
    /// Computes $\cot x$, the cotangent of a [`Float`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded cotangent is less than, equal
    /// to, or greater than the exact cotangent. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \cot x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\cot
    ///   x|\rfloor-p+1}$.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\cot
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=\pm\infty$
    ///
    /// Overflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $f(x,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead.
    /// - If $0<f(x,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    ///
    /// Overflow requires an input within $2^{-2^{30}}$ of a nonzero multiple of $\pi$, and
    /// underflow an input within $2^{-2^{30}}$ of an odd multiple of $\pi/2$, either of which takes
    /// more than $2^{30}$ bits of precision; overflow also occurs for an input of magnitude about
    /// $2^{-2^{30}}$, whose reciprocal alone is beyond the largest finite [`Float`].
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::cot_prec`] instead. If you
    /// know that your target precision is the precision of the input, consider using
    /// [`Float::cot_round`] instead. If both of these things are true, consider using
    /// [`Float::cot`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the sine and cosine at working precision $n$ (for large $n$ by binary
    /// splitting of the Taylor series, otherwise the cosine, from which the sine is derived), and
    /// their quotient, cost the first term, and for $|x| \geq 4$ the argument is reduced modulo
    /// $2\pi$, which requires $\pi$ to about $n + e$ bits and a remainder of the $m$-bit input.
    /// Unlike most functions, `cot` therefore gets slower as the magnitude of its input grows, not
    /// just as the precision does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the cotangent of a finite nonzero [`Float`] is never
    /// exactly representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .cot_prec_round(5, Floor);
    /// assert_eq!(c.to_string(), "0.625");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .cot_prec_round(5, Ceiling);
    /// assert_eq!(c.to_string(), "0.656");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .cot_prec_round(5, Nearest);
    /// assert_eq!(c.to_string(), "0.656");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .cot_prec_round(20, Floor);
    /// assert_eq!(c.to_string(), "0.64209175");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .cot_prec_round(20, Ceiling);
    /// assert_eq!(c.to_string(), "0.64209270");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .cot_prec_round(20, Nearest);
    /// assert_eq!(c.to_string(), "0.64209270");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn cot_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.cot_prec_round_ref(prec, rm)
    }

    /// Computes $\cot x$, the cotangent of a [`Float`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded cotangent is less than, equal
    /// to, or greater than the exact cotangent. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \cot x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\cot
    ///   x|\rfloor-p+1}$.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\cot
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=\pm\infty$
    ///
    /// Overflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $f(x,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead.
    /// - If $0<f(x,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    ///
    /// Overflow requires an input within $2^{-2^{30}}$ of a nonzero multiple of $\pi$, and
    /// underflow an input within $2^{-2^{30}}$ of an odd multiple of $\pi/2$, either of which takes
    /// more than $2^{30}$ bits of precision; overflow also occurs for an input of magnitude about
    /// $2^{-2^{30}}$, whose reciprocal alone is beyond the largest finite [`Float`].
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::cot_prec_ref`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::cot_round_ref`] instead. If both of these things are true, consider using
    /// `(&Float).cot()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the sine and cosine at working precision $n$ (for large $n$ by binary
    /// splitting of the Taylor series, otherwise the cosine, from which the sine is derived), and
    /// their quotient, cost the first term, and for $|x| \geq 4$ the argument is reduced modulo
    /// $2\pi$, which requires $\pi$ to about $n + e$ bits and a remainder of the $m$-bit input.
    /// Unlike most functions, `cot` therefore gets slower as the magnitude of its input grows, not
    /// just as the precision does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the cotangent of a finite nonzero [`Float`] is never
    /// exactly representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).cot_prec_round_ref(5, Floor);
    /// assert_eq!(c.to_string(), "0.625");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).cot_prec_round_ref(5, Ceiling);
    /// assert_eq!(c.to_string(), "0.656");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).cot_prec_round_ref(5, Nearest);
    /// assert_eq!(c.to_string(), "0.656");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).cot_prec_round_ref(20, Floor);
    /// assert_eq!(c.to_string(), "0.64209175");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).cot_prec_round_ref(20, Ceiling);
    /// assert_eq!(c.to_string(), "0.64209270");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).cot_prec_round_ref(20, Nearest);
    /// assert_eq!(c.to_string(), "0.64209270");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn cot_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match &self.0 {
            NaN | Infinity { .. } => (Self::NAN, Equal),
            // cot(+0) = +infinity, cot(-0) = -infinity
            Zero { .. } => (
                if self.is_sign_negative() {
                    Self::NEGATIVE_INFINITY
                } else {
                    Self::INFINITY
                },
                Equal,
            ),
            Finite { .. } => cot_prec_round_normal_ref(self, prec, rm),
        }
    }

    /// Computes $\cot x$, the cotangent of a [`Float`], rounding the result to the nearest value of
    /// the specified precision. The [`Float`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded cotangent is less than, equal to, or greater than the exact
    /// cotangent. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// If the cotangent is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \cot x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\cot x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\text{NaN}$
    /// - $f(\pm0.0,p)=\pm\infty$
    ///
    /// Overflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    ///
    /// Overflow requires an input within $2^{-2^{30}}$ of a nonzero multiple of $\pi$, and
    /// underflow an input within $2^{-2^{30}}$ of an odd multiple of $\pi/2$, either of which takes
    /// more than $2^{30}$ bits of precision; overflow also occurs for an input of magnitude about
    /// $2^{-2^{30}}$, whose reciprocal alone is beyond the largest finite [`Float`].
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::cot_prec_round`] instead. If you know that your target precision is the precision
    /// of the input, consider using [`Float::cot`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the sine and cosine at working precision $n$ (for large $n$ by binary
    /// splitting of the Taylor series, otherwise the cosine, from which the sine is derived), and
    /// their quotient, cost the first term, and for $|x| \geq 4$ the argument is reduced modulo
    /// $2\pi$, which requires $\pi$ to about $n + e$ bits and a remainder of the $m$-bit input.
    /// Unlike most functions, `cot` therefore gets slower as the magnitude of its input grows, not
    /// just as the precision does.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.cot_prec(5);
    /// assert_eq!(c.to_string(), "0.656");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.cot_prec(20);
    /// assert_eq!(c.to_string(), "0.64209270");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn cot_prec(self, prec: u64) -> (Self, Ordering) {
        self.cot_prec_round(prec, Nearest)
    }

    /// Computes $\cot x$, the cotangent of a [`Float`], rounding the result to the nearest value of
    /// the specified precision. The [`Float`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded cotangent is less than, equal to, or greater than
    /// the exact cotangent. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// If the cotangent is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \cot x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\cot x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\text{NaN}$
    /// - $f(\pm0.0,p)=\pm\infty$
    ///
    /// Overflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    ///
    /// Overflow requires an input within $2^{-2^{30}}$ of a nonzero multiple of $\pi$, and
    /// underflow an input within $2^{-2^{30}}$ of an odd multiple of $\pi/2$, either of which takes
    /// more than $2^{30}$ bits of precision; overflow also occurs for an input of magnitude about
    /// $2^{-2^{30}}$, whose reciprocal alone is beyond the largest finite [`Float`].
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::cot_prec_round_ref`] instead. If you know that your target precision is the
    /// precision of the input, consider using `(&Float).cot()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the sine and cosine at working precision $n$ (for large $n$ by binary
    /// splitting of the Taylor series, otherwise the cosine, from which the sine is derived), and
    /// their quotient, cost the first term, and for $|x| \geq 4$ the argument is reduced modulo
    /// $2\pi$, which requires $\pi$ to about $n + e$ bits and a remainder of the $m$-bit input.
    /// Unlike most functions, `cot` therefore gets slower as the magnitude of its input grows, not
    /// just as the precision does.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).cot_prec_ref(5);
    /// assert_eq!(c.to_string(), "0.656");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).cot_prec_ref(20);
    /// assert_eq!(c.to_string(), "0.64209270");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn cot_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.cot_prec_round_ref(prec, Nearest)
    }

    /// Computes $\cot x$, the cotangent of a [`Float`], rounding the result with the specified
    /// rounding mode. The [`Float`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded cotangent is less than, equal to, or greater than the exact cotangent.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \cot x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\cot
    ///   x|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\cot
    ///   x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\pm\infty,m)=\text{NaN}$
    /// - $f(\pm0.0,m)=\pm\infty$
    ///
    /// Overflow:
    /// - If $f(x,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $f(x,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead.
    /// - If $0<f(x,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    ///
    /// Overflow requires an input within $2^{-2^{30}}$ of a nonzero multiple of $\pi$, and
    /// underflow an input within $2^{-2^{30}}$ of an odd multiple of $\pi/2$, either of which takes
    /// more than $2^{30}$ bits of precision; overflow also occurs for an input of magnitude about
    /// $2^{-2^{30}}$, whose reciprocal alone is beyond the largest finite [`Float`].
    ///
    /// If you want to specify an output precision, consider using [`Float::cot_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::cot`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, e) = O(n (\log n)^3 \log\log n + (n+e) (\log (n+e))^2 \log\log (n+e))$
    ///
    /// $M(n, e) = O((n+e) \log (n+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $e$ is
    /// the exponent of `self` (0 if `self` has no exponent or a negative one): the Taylor series at
    /// working precision $n$, summed by binary splitting for large $n$, costs the first term, and
    /// for $|x| \geq 4$ the argument is reduced modulo $2\pi$, which requires $\pi$ to about $n +
    /// e$ bits. Unlike most functions, `cot` therefore gets slower as the magnitude of its input
    /// grows, not just as the precision does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the cotangent of a finite nonzero [`Float`] is never
    /// exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.cot_round(Floor);
    /// assert_eq!(c.to_string(), "0.64209261593433070300641998659417");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.cot_round(Ceiling);
    /// assert_eq!(c.to_string(), "0.64209261593433070300641998659496");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.cot_round(Nearest);
    /// assert_eq!(c.to_string(), "0.64209261593433070300641998659417");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn cot_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.cot_prec_round(prec, rm)
    }

    /// Computes $\cot x$, the cotangent of a [`Float`], rounding the result with the specified
    /// rounding mode. The [`Float`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded cotangent is less than, equal to, or greater than the exact
    /// cotangent. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \cot x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\cot
    ///   x|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\cot
    ///   x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\pm\infty,m)=\text{NaN}$
    /// - $f(\pm0.0,m)=\pm\infty$
    ///
    /// Overflow:
    /// - If $f(x,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $f(x,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead.
    /// - If $0<f(x,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    ///
    /// Overflow requires an input within $2^{-2^{30}}$ of a nonzero multiple of $\pi$, and
    /// underflow an input within $2^{-2^{30}}$ of an odd multiple of $\pi/2$, either of which takes
    /// more than $2^{30}$ bits of precision; overflow also occurs for an input of magnitude about
    /// $2^{-2^{30}}$, whose reciprocal alone is beyond the largest finite [`Float`].
    ///
    /// If you want to specify an output precision, consider using [`Float::cot_prec_round_ref`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// `(&Float).cot()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, e) = O(n (\log n)^3 \log\log n + (n+e) (\log (n+e))^2 \log\log (n+e))$
    ///
    /// $M(n, e) = O((n+e) \log (n+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $e$ is
    /// the exponent of `self` (0 if `self` has no exponent or a negative one): the Taylor series at
    /// working precision $n$, summed by binary splitting for large $n$, costs the first term, and
    /// for $|x| \geq 4$ the argument is reduced modulo $2\pi$, which requires $\pi$ to about $n +
    /// e$ bits. Unlike most functions, `cot` therefore gets slower as the magnitude of its input
    /// grows, not just as the precision does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the cotangent of a finite nonzero [`Float`] is never
    /// exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).cot_round_ref(Floor);
    /// assert_eq!(c.to_string(), "0.64209261593433070300641998659417");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).cot_round_ref(Ceiling);
    /// assert_eq!(c.to_string(), "0.64209261593433070300641998659496");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).cot_round_ref(Nearest);
    /// assert_eq!(c.to_string(), "0.64209261593433070300641998659417");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn cot_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.cot_prec_round_ref(self.significant_bits(), rm)
    }

    /// Computes $\cot x$, the cotangent of a [`Float`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is replaced by the result, and
    /// an [`Ordering`] is returned, indicating whether the rounded cotangent is less than, equal
    /// to, or greater than the exact cotangent. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function sets a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \cot x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\cot
    ///   x|\rfloor-p+1}$.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\cot
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::cot_prec_round`] documentation for information on special cases and
    /// overflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::cot_prec_assign`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::cot_round_assign`] instead. If both of these things are true, consider using
    /// [`Float::cot_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the sine and cosine at working precision $n$ (for large $n$ by binary
    /// splitting of the Taylor series, otherwise the cosine, from which the sine is derived), and
    /// their quotient, cost the first term, and for $|x| \geq 4$ the argument is reduced modulo
    /// $2\pi$, which requires $\pi$ to about $n + e$ bits and a remainder of the $m$-bit input.
    /// Unlike most functions, `cot` therefore gets slower as the magnitude of its input grows, not
    /// just as the precision does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the cotangent of a finite nonzero [`Float`] is never
    /// exactly representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.cot_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "0.625");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.cot_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.656");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.cot_prec_round_assign(5, Nearest), Greater);
    /// assert_eq!(x.to_string(), "0.656");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.cot_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "0.64209175");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.cot_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.64209270");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.cot_prec_round_assign(20, Nearest), Greater);
    /// assert_eq!(x.to_string(), "0.64209270");
    /// ```
    #[inline]
    pub fn cot_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let o;
        (*self, o) = self.cot_prec_round_ref(prec, rm);
        o
    }

    /// Computes $\cot x$, the cotangent of a [`Float`], rounding the result to the nearest value of
    /// the specified precision. The [`Float`] is replaced by the result, and an [`Ordering`] is
    /// returned, indicating whether the rounded cotangent is less than, equal to, or greater than
    /// the exact cotangent. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function sets a `NaN` it also returns `Equal`.
    ///
    /// If the cotangent is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \cot x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\cot x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::cot_prec`] documentation for information on special cases and overflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::cot_prec_round_assign`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::cot_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the sine and cosine at working precision $n$ (for large $n$ by binary
    /// splitting of the Taylor series, otherwise the cosine, from which the sine is derived), and
    /// their quotient, cost the first term, and for $|x| \geq 4$ the argument is reduced modulo
    /// $2\pi$, which requires $\pi$ to about $n + e$ bits and a remainder of the $m$-bit input.
    /// Unlike most functions, `cot` therefore gets slower as the magnitude of its input grows, not
    /// just as the precision does.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.cot_prec_assign(5), Greater);
    /// assert_eq!(x.to_string(), "0.656");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.cot_prec_assign(20), Greater);
    /// assert_eq!(x.to_string(), "0.64209270");
    /// ```
    #[inline]
    pub fn cot_prec_assign(&mut self, prec: u64) -> Ordering {
        self.cot_prec_round_assign(prec, Nearest)
    }

    /// Computes $\cot x$, the cotangent of a [`Float`], rounding the result with the specified
    /// rounding mode. The [`Float`] is replaced by the result, and an [`Ordering`] is returned,
    /// indicating whether the rounded cotangent is less than, equal to, or greater than the exact
    /// cotangent. Although `NaN`s are not comparable to any [`Float`], whenever this function sets
    /// a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \cot x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\cot
    ///   x|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\cot
    ///   x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::cot_round`] documentation for information on special cases and overflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::cot_prec_round_assign`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::cot_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, e) = O(n (\log n)^3 \log\log n + (n+e) (\log (n+e))^2 \log\log (n+e))$
    ///
    /// $M(n, e) = O((n+e) \log (n+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $e$ is
    /// the exponent of `self` (0 if `self` has no exponent or a negative one): the Taylor series at
    /// working precision $n$, summed by binary splitting for large $n$, costs the first term, and
    /// for $|x| \geq 4$ the argument is reduced modulo $2\pi$, which requires $\pi$ to about $n +
    /// e$ bits. Unlike most functions, `cot` therefore gets slower as the magnitude of its input
    /// grows, not just as the precision does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the cotangent of a finite nonzero [`Float`] is never
    /// exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.cot_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "0.64209261593433070300641998659417");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.cot_round_assign(Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.64209261593433070300641998659496");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.cot_round_assign(Nearest), Less);
    /// assert_eq!(x.to_string(), "0.64209261593433070300641998659417");
    /// ```
    #[inline]
    pub fn cot_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.cot_prec_round_assign(prec, rm)
    }

    /// Computes $\cot x$, the cotangent of a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded cotangent is less than, equal to, or greater than the exact cotangent.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \cot x+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\cot x|\rfloor-p+1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\cot x|\rfloor-p}$.
    ///
    /// These bounds do not apply when the result overflows; see below.
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=\infty$.
    ///
    /// Overflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $f(x,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead.
    /// - If $0<f(x,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    ///
    /// Overflow requires an input within $2^{-2^{30}}$ of a nonzero multiple of $\pi$, and
    /// underflow an input within $2^{-2^{30}}$ of an odd multiple of $\pi/2$, either of which takes
    /// a denominator of more than $2^{30}$ bits; overflow also occurs for an input of magnitude
    /// about $2^{-2^{30}}$ or below, whose reciprocal alone is beyond the largest finite [`Float`].
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::cot_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is `x.significant_bits()`,
    /// and $e$ is `x.floor_log_base_2_abs()` (taken as 0 when it is negative or $x = 0$): the input
    /// is rounded to a working precision and its [`Float`] sine and cosine taken there, then
    /// divided, which for $|x| \geq 2$ reduces the argument modulo $2\pi$ and so needs $\pi$ to
    /// about $n + e$ bits.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case for every nonzero input).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::cot_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(c.to_string(), "1.44");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::cot_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Ceiling);
    /// assert_eq!(c.to_string(), "1.50");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::cot_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Floor);
    /// assert_eq!(c.to_string(), "1.4616947");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::cot_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Ceiling);
    /// assert_eq!(c.to_string(), "1.4616966");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn cot_rational_prec_round(x: Rational, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        Self::cot_rational_prec_round_ref(&x, prec, rm)
    }

    /// Computes $\cot x$, the cotangent of a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded cotangent is less than, equal to, or greater than the exact cotangent.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \cot x+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\cot x|\rfloor-p+1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\cot x|\rfloor-p}$.
    ///
    /// These bounds do not apply when the result overflows.
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=\infty$.
    ///
    /// See the [`Float::cot_rational_prec_round`] documentation for information on overflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::cot_rational_prec_ref`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is `x.significant_bits()`,
    /// and $e$ is `x.floor_log_base_2_abs()` (taken as 0 when it is negative or $x = 0$): the input
    /// is rounded to a working precision and its [`Float`] sine and cosine taken there, then
    /// divided, which for $|x| \geq 2$ reduces the argument modulo $2\pi$ and so needs $\pi$ to
    /// about $n + e$ bits.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case for every nonzero input).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) =
    ///     Float::cot_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(c.to_string(), "1.44");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) =
    ///     Float::cot_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 5, Ceiling);
    /// assert_eq!(c.to_string(), "1.50");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) =
    ///     Float::cot_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 20, Floor);
    /// assert_eq!(c.to_string(), "1.4616947");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) =
    ///     Float::cot_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 20, Ceiling);
    /// assert_eq!(c.to_string(), "1.4616966");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn cot_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        if *x == 0u32 {
            // cot(0) = infinity; a `Rational` zero has no sign, so the result is positive
            return (Self::INFINITY, Equal);
        }
        cot_rational_helper(x, prec, rm)
    }

    /// Computes $\cot x$, the cotangent of a [`Rational`], rounding the result to the nearest value
    /// of the specified precision and returning the result as a [`Float`]. The [`Rational`] is
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded cotangent
    /// is less than, equal to, or greater than the exact cotangent.
    ///
    /// If the cotangent is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \cot x+\varepsilon,
    /// $$
    /// where $|\varepsilon| \leq 2^{\lfloor\log_2 |\cot x|\rfloor-p}$ (unless the result overflows;
    /// see below).
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=\infty$.
    ///
    /// Overflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    ///
    /// Overflow requires an input within $2^{-2^{30}}$ of a nonzero multiple of $\pi$, and
    /// underflow an input within $2^{-2^{30}}$ of an odd multiple of $\pi/2$, either of which takes
    /// a denominator of more than $2^{30}$ bits; overflow also occurs for an input of magnitude
    /// about $2^{-2^{30}}$ or below, whose reciprocal alone is beyond the largest finite [`Float`].
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::cot_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is `x.significant_bits()`,
    /// and $e$ is `x.floor_log_base_2_abs()` (taken as 0 when it is negative or $x = 0$): the input
    /// is rounded to a working precision and its [`Float`] sine and cosine taken there, then
    /// divided, which for $|x| \geq 2$ reduces the argument modulo $2\pi$ and so needs $\pi$ to
    /// about $n + e$ bits.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::cot_rational_prec(Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(c.to_string(), "1.44");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::cot_rational_prec(Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(c.to_string(), "1.4616966");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn cot_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::cot_rational_prec_round_ref(&x, prec, Nearest)
    }

    /// Computes $\cot x$, the cotangent of a [`Rational`], rounding the result to the nearest value
    /// of the specified precision and returning the result as a [`Float`]. The [`Rational`] is
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// cotangent is less than, equal to, or greater than the exact cotangent.
    ///
    /// If the cotangent is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \cot x+\varepsilon,
    /// $$
    /// where $|\varepsilon| \leq 2^{\lfloor\log_2 |\cot x|\rfloor-p}$ (unless the result
    /// overflows).
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=\infty$.
    ///
    /// See the [`Float::cot_rational_prec`] documentation for information on overflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::cot_rational_prec_round_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is `x.significant_bits()`,
    /// and $e$ is `x.floor_log_base_2_abs()` (taken as 0 when it is negative or $x = 0$): the input
    /// is rounded to a working precision and its [`Float`] sine and cosine taken there, then
    /// divided, which for $|x| \geq 2$ reduces the argument modulo $2\pi$ and so needs $\pi$ to
    /// about $n + e$ bits.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::cot_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(c.to_string(), "1.44");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::cot_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(c.to_string(), "1.4616966");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn cot_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::cot_rational_prec_round_ref(x, prec, Nearest)
    }

    /// Computes $\cot(2\pi x/u)$, the cotangent of a [`Float`] measured in $u$ths of a turn,
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`Float`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded cotangent is less than, equal to, or greater than the exact cotangent. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = \cot(2\pi x/u)+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $u=0$, or $x/u$ is a multiple of $1/8$, $\varepsilon$ may be ignored
    ///   or assumed to be 0.
    /// - If $x$ is finite, $u\neq 0$, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\cot(2\pi x/u)|\rfloor-p+1}$.
    /// - If $x$ is finite, $u\neq 0$, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\cot(2\pi x/u)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p,m)=\text{NaN}$
    /// - $f(\pm\infty,u,p,m)=\text{NaN}$
    /// - $f(x,0,p,m)=\text{NaN}$
    /// - $f(\\pm0.0,u,p,m)=\\pm\\infty$
    /// - If $x/u$ is a multiple of $1/2$, the cotangent has a pole there, and the result is exactly
    ///   $\\pm\\infty$: the sine is a zero carrying the sign of $x$ and the cosine is $\\pm1$, so
    ///   the sign is that of $x$ at an even multiple and the opposite at an odd one. Keeping that
    ///   identity is what makes the function odd.
    /// - If $x/u$ is an odd multiple of $1/4$, the result is exactly $\\pm0.0$, with the sign of
    ///   the sine there.
    /// - If $x/u$ is an odd multiple of $1/8$, the result is exactly $\\pm1$.
    ///
    /// When $x/u$ in lowest terms has denominator 3 or 6, the result is $\pm\sqrt3/3$, and when it
    /// has denominator 12, $\pm\sqrt3$; each is computed from a single correctly rounded constant
    /// rather than from $\pi$, a sine, and a cosine, which is far faster.
    ///
    /// Overflow:
    /// - If $f(x,u,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,u,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead.
    /// - If $f(x,u,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,u,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead.
    /// - If $0<f(x,u,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,u,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    ///
    /// Overflow requires $x/u$ within $2^{-2^{30}}$ of a multiple of $1/2$ without being one, and
    /// underflow $x/u$ within $2^{-2^{30}}$ of an odd multiple of $1/4$ without being one, either
    /// of which takes more than $2^{30}$ bits of precision; overflow also occurs for an $x/u$ so
    /// small that $2\pi x/u$ is below $2^{-2^{30}}$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::cot_with_period_prec`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::cot_with_period_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the argument is reduced modulo $u$ exactly, and the sine and cosine of
    /// $2\pi x/u$ are then taken at a working precision of about $n + e$ bits, which needs $\pi$ to
    /// that many bits, and divided.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case unless $x/u$ is a multiple of $1/8$, or $x$ is
    /// zero or not finite, or $u$ is zero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = Float::ONE.cot_with_period_prec_round(7, 10, Floor);
    /// assert_eq!(t.to_string(), "0.79688");
    /// assert_eq!(o, Less);
    ///
    /// let (t, o) = Float::ONE.cot_with_period_prec_round(7, 10, Ceiling);
    /// assert_eq!(t.to_string(), "0.79785");
    /// assert_eq!(o, Greater);
    ///
    /// // a quarter turn is exactly 0
    /// let (t, o) = Float::from(90u32).cot_with_period_prec_round(360, 10, Exact);
    /// assert_eq!(t.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    ///
    /// // a half turn is a pole
    /// let (t, o) = Float::from(180u32).cot_with_period_prec_round(360, 10, Exact);
    /// assert_eq!(t.to_string(), "-Infinity");
    /// assert_eq!(o, Equal);
    ///
    /// // a twelfth of a turn: sqrt(3)
    /// let (t, o) = Float::from(30u32).cot_with_period_prec_round(360, 10, Nearest);
    /// assert_eq!(t.to_string(), "1.7324");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn cot_with_period_prec_round(
        self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        self.cot_with_period_prec_round_ref(u, prec, rm)
    }

    /// Computes $\cot(2\pi x/u)$, the cotangent of a [`Float`] measured in $u$ths of a turn,
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`Float`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded cotangent is less than, equal to, or greater than the exact cotangent. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`Float::cot_with_period_prec_round`] for the error bounds, the special and closed-form
    /// cases, overflow, and the complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = Float::ONE.cot_with_period_prec_round_ref(7, 10, Floor);
    /// assert_eq!(t.to_string(), "0.79688");
    /// assert_eq!(o, Less);
    /// ```
    pub fn cot_with_period_prec_round_ref(
        &self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match &self.0 {
            // for u=0, return NaN
            _ if u == 0 => (Self::NAN, Equal),
            NaN | Infinity { .. } => (Self::NAN, Equal),
            // x is zero: cot(±0) = ±infinity
            Zero { .. } => (
                if self.is_sign_negative() {
                    Self::NEGATIVE_INFINITY
                } else {
                    Self::INFINITY
                },
                Equal,
            ),
            Finite { .. } => cot_with_period_prec_round_normal_ref(self, u, prec, rm),
        }
    }

    /// Computes $\cot(2\pi x/u)$, the cotangent of a [`Float`] measured in $u$ths of a turn,
    /// rounding the result to the nearest value of the specified precision. The [`Float`] is taken
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded cotangent is less
    /// than, equal to, or greater than the exact cotangent. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the cotangent is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// See [`Float::cot_with_period_prec_round`] for the error bounds, the special and closed-form
    /// cases, overflow, and the complexity; this function behaves the same way with `Nearest`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::cot_with_period_prec_round`] instead.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = Float::ONE.cot_with_period_prec(7, 10);
    /// assert_eq!(t.to_string(), "0.79785");
    /// assert_eq!(o, Greater);
    ///
    /// // an eighth of a turn is exactly 1
    /// let (t, o) = Float::ONE.cot_with_period_prec(8, 10);
    /// assert_eq!(t.to_string(), "1.0000");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn cot_with_period_prec(self, u: u64, prec: u64) -> (Self, Ordering) {
        self.cot_with_period_prec_round(u, prec, Nearest)
    }

    /// Computes $\cot(2\pi x/u)$, the cotangent of a [`Float`] measured in $u$ths of a turn,
    /// rounding the result to the nearest value of the specified precision. The [`Float`] is taken
    /// by reference. An [`Ordering`] is also returned, indicating whether the rounded cotangent is
    /// less than, equal to, or greater than the exact cotangent. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::cot_with_period_prec`] and [`Float::cot_with_period_prec_round`]; this function
    /// behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = Float::ONE.cot_with_period_prec_ref(7, 10);
    /// assert_eq!(t.to_string(), "0.79785");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn cot_with_period_prec_ref(&self, u: u64, prec: u64) -> (Self, Ordering) {
        self.cot_with_period_prec_round_ref(u, prec, Nearest)
    }

    /// Computes $\cot(2\pi x/u)$, the cotangent of a [`Float`] measured in $u$ths of a turn,
    /// rounding the result to the precision of the input and with the specified rounding mode. The
    /// [`Float`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded cotangent is less than, equal to, or greater than the exact cotangent. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`Float::cot_with_period_prec_round`] for the error bounds, the special and closed-form
    /// cases, overflow, and the complexity; this function behaves the same way with `prec` equal to
    /// the precision of the input.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::cot_with_period_prec_round`] instead.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the precision of
    /// the input.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = Float::from_unsigned_prec(1u32, 10)
    ///     .0
    ///     .cot_with_period_round(7, Floor);
    /// assert_eq!(t.to_string(), "0.79688");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn cot_with_period_round(self, u: u64, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.cot_with_period_prec_round(u, prec, rm)
    }

    /// Computes $\cot(2\pi x/u)$, the cotangent of a [`Float`] measured in $u$ths of a turn,
    /// rounding the result to the precision of the input and with the specified rounding mode. The
    /// [`Float`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded cotangent is less than, equal to, or greater than the exact cotangent. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`Float::cot_with_period_round`] and [`Float::cot_with_period_prec_round`]; this
    /// function behaves the same way.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the precision of
    /// the input.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = Float::from_unsigned_prec(1u32, 10)
    ///     .0
    ///     .cot_with_period_round_ref(7, Floor);
    /// assert_eq!(t.to_string(), "0.79688");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn cot_with_period_round_ref(&self, u: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.cot_with_period_prec_round_ref(u, self.significant_bits(), rm)
    }

    /// Computes $\cot(2\pi x/u)$, the cotangent of a [`Float`] measured in $u$ths of a turn (so
    /// that `u = 360` is degrees), rounding the result to the precision of the input and to the
    /// nearest [`Float`]. The [`Float`] is taken by value.
    ///
    /// If the cotangent is equidistant from two [`Float`]s with the precision of the input, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// See [`Float::cot_with_period_prec_round`] for the error bounds, the special and closed-form
    /// cases, overflow, and the complexity; this function behaves the same way with `prec` equal to
    /// the precision of the input and `rm` equal to `Nearest`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::cot_with_period_round`] instead. If you want to specify an output precision,
    /// consider using [`Float::cot_with_period_prec`]. If you want both of these things, consider
    /// using [`Float::cot_with_period_prec_round`].
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// let t = Float::from_unsigned_prec(1u32, 10).0.cot_with_period(7);
    /// assert_eq!(t.to_string(), "0.79785");
    ///
    /// // a quarter turn is exactly 0
    /// assert_eq!(Float::from(90u32).cot_with_period(360).to_string(), "0.0");
    /// ```
    #[inline]
    pub fn cot_with_period(self, u: u64) -> Self {
        let prec = self.significant_bits();
        self.cot_with_period_prec(u, prec).0
    }

    /// Computes $\cot(2\pi x/u)$, the cotangent of a [`Float`] measured in $u$ths of a turn (so
    /// that `u = 360` is degrees), rounding the result to the precision of the input and to the
    /// nearest [`Float`]. The [`Float`] is taken by reference.
    ///
    /// If the cotangent is equidistant from two [`Float`]s with the precision of the input, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// See [`Float::cot_with_period_prec_round`] for the error bounds, the special and closed-form
    /// cases, overflow, and the complexity; this function behaves the same way with `prec` equal to
    /// the precision of the input and `rm` equal to `Nearest`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::cot_with_period_round_ref`] instead. If you want to specify an output precision,
    /// consider using [`Float::cot_with_period_prec_ref`]. If you want both of these things,
    /// consider using [`Float::cot_with_period_prec_round_ref`].
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// let t = (&Float::from_unsigned_prec(1u32, 10).0).cot_with_period_ref(7);
    /// assert_eq!(t.to_string(), "0.79785");
    /// ```
    #[inline]
    pub fn cot_with_period_ref(&self, u: u64) -> Self {
        self.cot_with_period_prec_ref(u, self.significant_bits()).0
    }

    /// Replaces a [`Float`] measured in $u$ths of a turn with its cotangent, rounding the result to
    /// the specified precision and with the specified rounding mode. An [`Ordering`] is returned,
    /// indicating whether the rounded cotangent is less than, equal to, or greater than the exact
    /// cotangent. Although `NaN`s are not comparable to any [`Float`], whenever this function sets
    /// a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::cot_with_period_prec_round`] for the error bounds, the special and closed-form
    /// cases, overflow, and the complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::ONE;
    /// assert_eq!(x.cot_with_period_prec_round_assign(7, 10, Floor), Less);
    /// assert_eq!(x.to_string(), "0.79688");
    /// ```
    #[inline]
    pub fn cot_with_period_prec_round_assign(
        &mut self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (t, o) = self.cot_with_period_prec_round_ref(u, prec, rm);
        *self = t;
        o
    }

    /// Replaces a [`Float`] measured in $u$ths of a turn with its cotangent, rounding the result to
    /// the nearest value of the specified precision. An [`Ordering`] is returned, indicating
    /// whether the rounded cotangent is less than, equal to, or greater than the exact cotangent.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function sets a `NaN` it
    /// also returns `Equal`.
    ///
    /// See [`Float::cot_with_period_prec`] and [`Float::cot_with_period_prec_round`]; this function
    /// behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::ONE;
    /// assert_eq!(x.cot_with_period_prec_assign(7, 10), Greater);
    /// assert_eq!(x.to_string(), "0.79785");
    /// ```
    #[inline]
    pub fn cot_with_period_prec_assign(&mut self, u: u64, prec: u64) -> Ordering {
        self.cot_with_period_prec_round_assign(u, prec, Nearest)
    }

    /// Replaces a [`Float`] measured in $u$ths of a turn with its cotangent, rounding the result to
    /// the precision of the input and with the specified rounding mode. An [`Ordering`] is
    /// returned, indicating whether the rounded cotangent is less than, equal to, or greater than
    /// the exact cotangent. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function sets a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::cot_with_period_round`] and [`Float::cot_with_period_prec_round`]; this
    /// function behaves the same way.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the precision of
    /// the input.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 10).0;
    /// assert_eq!(x.cot_with_period_round_assign(7, Floor), Less);
    /// assert_eq!(x.to_string(), "0.79688");
    /// ```
    #[inline]
    pub fn cot_with_period_round_assign(&mut self, u: u64, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.cot_with_period_prec_round_assign(u, prec, rm)
    }

    /// Computes $\cot(2\pi x/u)$, the cotangent of a [`Float`] measured in $u$ths of a turn (so
    /// that `u = 360` is degrees), rounding the result to the precision of the input and to the
    /// nearest [`Float`]. The [`Float`] is replaced by the result.
    ///
    /// If the cotangent is equidistant from two [`Float`]s with the precision of the input, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// See [`Float::cot_with_period_prec_round`] for the error bounds, the special and closed-form
    /// cases, overflow, and the complexity; this function behaves the same way with `prec` equal to
    /// the precision of the input and `rm` equal to `Nearest`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::cot_with_period_round_assign`] instead. If you want to specify an output precision,
    /// consider using [`Float::cot_with_period_prec_assign`]. If you want both of these things,
    /// consider using [`Float::cot_with_period_prec_round_assign`].
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 10).0;
    /// x.cot_with_period_assign(7);
    /// assert_eq!(x.to_string(), "0.79785");
    /// ```
    #[inline]
    pub fn cot_with_period_assign(&mut self, u: u64) {
        let prec = self.significant_bits();
        self.cot_with_period_prec_assign(u, prec);
    }

    /// Computes $\cot(2\pi x/u)$, the cotangent of a [`Rational`] measured in $u$ths of a turn,
    /// rounding the result to the specified precision and with the specified rounding mode, and
    /// returning the result as a [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded cotangent is less than, equal to, or greater
    /// than the exact cotangent. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = \cot(2\pi x/u)+\varepsilon.
    /// $$
    /// - If $u=0$ or $x/u$ is a multiple of $1/8$, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $u\neq 0$ and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\cot(2\pi
    ///   x/u)|\rfloor-p+1}$.
    /// - If $u\neq 0$ and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\cot(2\pi
    ///   x/u)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=\text{NaN}$
    /// - $f(0,u,p,m)=\infty$
    /// - If $x/u$ is a multiple of $1/2$, the cotangent has a pole there, and the result is exactly
    ///   $\\pm\\infty$: the sine is a zero carrying the sign of $x$ and the cosine is $\\pm1$, so
    ///   the sign is that of $x$ at an even multiple and the opposite at an odd one. Keeping that
    ///   identity is what makes the function odd.
    /// - If $x/u$ is an odd multiple of $1/4$, the result is exactly $\\pm0.0$, with the sign of
    ///   the sine there.
    /// - If $x/u$ is an odd multiple of $1/8$, the result is exactly $\\pm1$.
    ///
    /// When $x/u$ in lowest terms has denominator 3 or 6, the result is $\pm\sqrt3/3$, and when it
    /// has denominator 12, $\pm\sqrt3$; each is computed from a single correctly rounded constant
    /// rather than from $\pi$, a sine, and a cosine, which is far faster.
    ///
    /// Overflow and underflow are as for [`Float::cot_with_period_prec_round`]. Overflow requires
    /// $x/u$ within $2^{-2^{30}}$ of a multiple of $1/2$ without being one, and underflow $x/u$
    /// within $2^{-2^{30}}$ of an odd multiple of $1/4$ without being one, either of which takes a
    /// denominator of more than $2^{30}$ bits; overflow also occurs for an $x/u$ so small that
    /// $2\pi x/u$ is below $2^{-2^{30}}$, which a [`Rational`] can be however large its denominator
    /// is not.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::cot_with_period_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + (n+m) (\log (n+m))^2 \log\log (n+m))$
    ///
    /// $M(n, m) = O((n+m) \log (n+m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`: the fraction of a turn is reduced modulo 1 exactly, so only its size
    /// and the precision drive the cost, not the magnitude of $x$.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case unless $x/u$ is a multiple of $1/8$, or $x$ or
    /// $u$ is zero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = Float::cot_with_period_rational_prec_round(Rational::ONE, 7, 10, Floor);
    /// assert_eq!(t.to_string(), "0.79688");
    /// assert_eq!(o, Less);
    ///
    /// let (t, o) = Float::cot_with_period_rational_prec_round(Rational::ONE, 7, 10, Ceiling);
    /// assert_eq!(t.to_string(), "0.79785");
    /// assert_eq!(o, Greater);
    ///
    /// // a quarter turn is exactly 0
    /// let (t, o) = Float::cot_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 4),
    ///     1,
    ///     10,
    ///     Exact,
    /// );
    /// assert_eq!(t.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    ///
    /// // a twelfth of a turn: sqrt(3)
    /// let (t, o) = Float::cot_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 12),
    ///     1,
    ///     10,
    ///     Nearest,
    /// );
    /// assert_eq!(t.to_string(), "1.7324");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn cot_with_period_rational_prec_round(
        x: Rational,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::cot_with_period_rational_prec_round_ref(&x, u, prec, rm)
    }

    /// Computes $\cot(2\pi x/u)$, the cotangent of a [`Rational`] measured in $u$ths of a turn,
    /// rounding the result to the specified precision and with the specified rounding mode, and
    /// returning the result as a [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`]
    /// is also returned, indicating whether the rounded cotangent is less than, equal to, or
    /// greater than the exact cotangent. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::cot_with_period_rational_prec_round`] for the error bounds, the special and
    /// closed-form cases, overflow, and the complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = Float::cot_with_period_rational_prec_round_ref(&Rational::ONE, 7, 10, Floor);
    /// assert_eq!(t.to_string(), "0.79688");
    /// assert_eq!(o, Less);
    /// ```
    pub fn cot_with_period_rational_prec_round_ref(
        x: &Rational,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        // for u = 0, return NaN
        if u == 0 {
            return (Self::NAN, Equal);
        }
        // cot(0) = infinity (a `Rational` zero has no sign)
        if *x == 0u32 {
            return (Self::INFINITY, Equal);
        }
        // q = x/u, reduced to (-1, 1) with the sign of x: cot(2 pi q) has period 1 in q, and a
        // multiple of u is a pole, where the sine is a zero with the sign of x and the cosine is 1,
        // so the cotangent is an infinity with that sign
        let q = x / Rational::from(u) % Rational::ONE;
        if q == 0u32 {
            return (
                if *x < 0u32 {
                    Self::NEGATIVE_INFINITY
                } else {
                    Self::INFINITY
                },
                Equal,
            );
        }
        cot_turns_helper(&q, prec, rm)
    }

    /// Computes $\cot(2\pi x/u)$, the cotangent of a [`Rational`] measured in $u$ths of a turn,
    /// rounding the result to the nearest value of the specified precision, and returning the
    /// result as a [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded cotangent is less than, equal to, or greater than the exact
    /// cotangent. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// If the cotangent is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// See [`Float::cot_with_period_rational_prec_round`] for the error bounds, the special and
    /// closed-form cases, overflow, and the complexity; this function behaves the same way with
    /// `Nearest`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::cot_with_period_rational_prec_round`] instead.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = Float::cot_with_period_rational_prec(Rational::ONE, 7, 10);
    /// assert_eq!(t.to_string(), "0.79785");
    /// assert_eq!(o, Greater);
    ///
    /// // an eighth of a turn is exactly 1
    /// let (t, o) = Float::cot_with_period_rational_prec(Rational::ONE, 8, 10);
    /// assert_eq!(t.to_string(), "1.0000");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn cot_with_period_rational_prec(x: Rational, u: u64, prec: u64) -> (Self, Ordering) {
        Self::cot_with_period_rational_prec_round_ref(&x, u, prec, Nearest)
    }

    /// Computes $\cot(2\pi x/u)$, the cotangent of a [`Rational`] measured in $u$ths of a turn,
    /// rounding the result to the nearest value of the specified precision, and returning the
    /// result as a [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded cotangent is less than, equal to, or greater than
    /// the exact cotangent. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::cot_with_period_rational_prec`] and
    /// [`Float::cot_with_period_rational_prec_round`]; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = Float::cot_with_period_rational_prec_ref(&Rational::ONE, 7, 10);
    /// assert_eq!(t.to_string(), "0.79785");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn cot_with_period_rational_prec_ref(x: &Rational, u: u64, prec: u64) -> (Self, Ordering) {
        Self::cot_with_period_rational_prec_round_ref(x, u, prec, Nearest)
    }
}

impl Cot for Float {
    type Output = Self;

    /// Computes $\cot x$, the cotangent of a [`Float`], taking it by value.
    ///
    /// If the output has a precision, it is the precision of the input. If the cotangent is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x) = \cot x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\cot x|\rfloor-p}$, where $p$ is
    ///   the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\pm\infty)=\text{NaN}$
    /// - $f(\pm0.0)=\pm\infty$
    ///
    /// See the [`Float::cot_round`] documentation for information on overflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using [`Float::cot_round`]
    /// instead. If you want to specify the output precision, consider using [`Float::cot_prec`]. If
    /// you want both of these things, consider using [`Float::cot_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, e) = O(n (\log n)^3 \log\log n + (n+e) (\log (n+e))^2 \log\log (n+e))$
    ///
    /// $M(n, e) = O((n+e) \log (n+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $e$ is
    /// the exponent of `self` (0 if `self` has no exponent or a negative one): the Taylor series at
    /// working precision $n$, summed by binary splitting for large $n$, costs the first term, and
    /// for $|x| \geq 4$ the argument is reduced modulo $2\pi$, which requires $\pi$ to about $n +
    /// e$ bits. Unlike most functions, `cot` therefore gets slower as the magnitude of its input
    /// grows, not just as the precision does.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Cot;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.cot().is_nan());
    /// assert!(Float::INFINITY.cot().is_nan());
    /// assert!(Float::NEGATIVE_INFINITY.cot().is_nan());
    /// assert_eq!(Float::ZERO.cot().to_string(), "Infinity");
    /// assert_eq!(Float::NEGATIVE_ZERO.cot().to_string(), "-Infinity");
    /// assert_eq!(
    ///     Float::from_unsigned_prec(1u32, 100).0.cot().to_string(),
    ///     "0.64209261593433070300641998659417"
    /// );
    /// assert_eq!(
    ///     Float::from_unsigned_prec(100u32, 100).0.cot().to_string(),
    ///     "-1.7029569194264692160987314595571"
    /// );
    /// ```
    #[inline]
    fn cot(self) -> Self {
        let prec = self.significant_bits();
        self.cot_prec_round(prec, Nearest).0
    }
}

impl Cot for &Float {
    type Output = Float;

    /// Computes $\cot x$, the cotangent of a [`Float`], taking it by reference.
    ///
    /// If the output has a precision, it is the precision of the input. If the cotangent is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x) = \cot x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\cot x|\rfloor-p}$, where $p$ is
    ///   the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\pm\infty)=\text{NaN}$
    /// - $f(\pm0.0)=\pm\infty$
    ///
    /// See the [`Float::cot_round`] documentation for information on overflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::cot_round_ref`] instead. If you want to specify the output precision, consider
    /// using [`Float::cot_prec_ref`]. If you want both of these things, consider using
    /// [`Float::cot_prec_round_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n, e) = O(n (\log n)^3 \log\log n + (n+e) (\log (n+e))^2 \log\log (n+e))$
    ///
    /// $M(n, e) = O((n+e) \log (n+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $e$ is
    /// the exponent of `self` (0 if `self` has no exponent or a negative one): the Taylor series at
    /// working precision $n$, summed by binary splitting for large $n$, costs the first term, and
    /// for $|x| \geq 4$ the argument is reduced modulo $2\pi$, which requires $\pi$ to about $n +
    /// e$ bits. Unlike most functions, `cot` therefore gets slower as the magnitude of its input
    /// grows, not just as the precision does.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Cot;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.cot().is_nan());
    /// assert!(Float::INFINITY.cot().is_nan());
    /// assert!(Float::NEGATIVE_INFINITY.cot().is_nan());
    /// assert_eq!(Float::ZERO.cot().to_string(), "Infinity");
    /// assert_eq!(Float::NEGATIVE_ZERO.cot().to_string(), "-Infinity");
    /// assert_eq!(
    ///     (&Float::from_unsigned_prec(1u32, 100).0).cot().to_string(),
    ///     "0.64209261593433070300641998659417"
    /// );
    /// assert_eq!(
    ///     (&Float::from_unsigned_prec(100u32, 100).0)
    ///         .cot()
    ///         .to_string(),
    ///     "-1.7029569194264692160987314595571"
    /// );
    /// ```
    #[inline]
    fn cot(self) -> Float {
        self.cot_prec_round_ref(self.significant_bits(), Nearest).0
    }
}

impl CotAssign for Float {
    /// Computes $\cot x$, the cotangent of a [`Float`], in place.
    ///
    /// If the output has a precision, it is the precision of the input. If the cotangent is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x \gets \cot x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\cot x|\rfloor-p}$, where $p$ is
    ///   the precision of the input.
    ///
    /// See the [`Float::cot`] documentation for information on special cases and overflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::cot_round_assign`] instead. If you want to specify the output precision, consider
    /// using [`Float::cot_prec_assign`]. If you want both of these things, consider using
    /// [`Float::cot_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, e) = O(n (\log n)^3 \log\log n + (n+e) (\log (n+e))^2 \log\log (n+e))$
    ///
    /// $M(n, e) = O((n+e) \log (n+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $e$ is
    /// the exponent of `self` (0 if `self` has no exponent or a negative one): the Taylor series at
    /// working precision $n$, summed by binary splitting for large $n$, costs the first term, and
    /// for $|x| \geq 4$ the argument is reduced modulo $2\pi$, which requires $\pi$ to about $n +
    /// e$ bits. Unlike most functions, `cot` therefore gets slower as the magnitude of its input
    /// grows, not just as the precision does.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CotAssign;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.cot_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x.cot_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x.cot_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::ZERO;
    /// x.cot_assign();
    /// assert_eq!(x.to_string(), "Infinity");
    ///
    /// let mut x = Float::NEGATIVE_ZERO;
    /// x.cot_assign();
    /// assert_eq!(x.to_string(), "-Infinity");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// x.cot_assign();
    /// assert_eq!(x.to_string(), "0.64209261593433070300641998659417");
    ///
    /// let mut x = Float::from_unsigned_prec(100u32, 100).0;
    /// x.cot_assign();
    /// assert_eq!(x.to_string(), "-1.7029569194264692160987314595571");
    /// ```
    #[inline]
    fn cot_assign(&mut self) {
        let prec = self.significant_bits();
        self.cot_prec_round_assign(prec, Nearest);
    }
}

/// Computes $\cot x$, the cotangent of a primitive float, correctly rounded. Neither the standard
/// library nor `libm` provides a cotangent.
///
/// $$
/// f(x) = \cot x+\varepsilon.
/// $$
/// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\cot x|\rfloor-p}$, where $p$ is the
///   precision of the output (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]).
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(\pm\infty)=\text{NaN}$
/// - $f(\pm0.0)=\pm\infty$
///
/// Overflow is possible: the cotangent of a tiny $x$ is close to $1/x$, so an $x$ with $|x|$ below
/// about $2^{-128}$ has a cotangent beyond the largest [`f32`], and one below about $2^{-1024}$
/// beyond the largest [`f64`]; the result is then $\pm\infty$. No [`f32`] or [`f64`] is close
/// enough to a nonzero multiple of $\pi$ for its cotangent to overflow that way, nor close enough
/// to an odd multiple of $\pi/2$ for it to underflow: the floats are spaced far more widely there
/// than either would take.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::cot::primitive_float_cot;
///
/// assert!(primitive_float_cot(f32::NAN).is_nan());
/// assert!(primitive_float_cot(f32::INFINITY).is_nan());
/// assert!(primitive_float_cot(f32::NEGATIVE_INFINITY).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_cot(0.0f32)),
///     NiceFloat(f32::INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_cot(-0.0f32)),
///     NiceFloat(f32::NEGATIVE_INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_cot(1.0f32)),
///     NiceFloat(0.64209265)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_cot(1.0f64)),
///     NiceFloat(0.6420926159343308)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_cot<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::cot_prec, x)
}

/// Computes $\cot x$, the cotangent of a [`Rational`], returning the result as a primitive float.
///
/// $$
/// f(x) = \cot x+\varepsilon,
/// $$
/// where $|\varepsilon| < 2^{\lfloor\log_2 |\cot x|\rfloor-p}$, and $p$ is the precision of the
/// output (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]).
///
/// Special cases:
/// - $f(0)=\infty$
///
/// Both ends are possible: a [`Rational`] within about $2^{-129}$ of a nonzero multiple of $\pi$
/// has a cotangent beyond the largest [`f32`], and one within about $2^{-1025}$ of one beyond the
/// largest [`f64`]; so does any [`Rational`] small enough that its reciprocal alone leaves the
/// range, and $0$ itself, whose cotangent is $\infty$. A [`Rational`] as close to an odd multiple
/// of $\pi/2$ underflows instead, to $\pm0.0$.
///
/// # Worst-case complexity
/// $T(m, e) = O((m+e) (\log (m+e))^2 \log\log (m+e))$
///
/// $M(m, e) = O((m+e) \log (m+e))$
///
/// where $T$ is time, $M$ is additional memory, $m$ is `x.significant_bits()`, and $e$ is
/// `x.floor_log_base_2_abs()` (taken as 0 when it is negative or $x = 0$): for $|x| \geq 3$ the
/// argument is reduced modulo $2\pi$, which needs $\pi$ to about $e$ bits.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::cot::primitive_float_cot_rational;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_cot_rational::<f64>(&Rational::ZERO)),
///     NiceFloat(f64::INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_cot_rational::<f64>(
///         &Rational::from_unsigneds(1u8, 3)
///     )),
///     NiceFloat(2.888057036277277)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_cot_rational::<f32>(
///         &Rational::from_unsigneds(1u8, 3)
///     )),
///     NiceFloat(2.888057)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_cot_rational::<f64>(&Rational::from(10000))),
///     NiceFloat(3.11554495756144)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_cot_rational<T: PrimitiveFloat>(x: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_to_float_fn(Float::cot_rational_prec_ref, x)
}

/// Computes $\cot(2\pi x/u)$, the cotangent of a primitive float measured in $u$ths of a turn (so
/// that `u = 360` is degrees).
///
/// $$
/// f(x,u) = \cot(2\pi x/u)+\varepsilon.
/// $$
/// - If $x$ is not finite, $u=0$, or $x/u$ is a multiple of $1/8$, $\varepsilon$ may be ignored or
///   assumed to be 0.
/// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |\cot(2\pi x/u)|\rfloor-p}$, where $p$ is the
///   precision of the output (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]).
///
/// Special cases:
/// - $f(\text{NaN},u)=\text{NaN}$
/// - $f(\pm\infty,u)=\text{NaN}$
/// - $f(x,0)=\text{NaN}$
/// - $f(\pm0.0,u)=\pm\infty$
/// - If $x/u$ is a multiple of $1/2$, the cotangent has a pole there, and the result is exactly
///   $\\pm\\infty$: the sine is a zero carrying the sign of $x$ and the cosine is $\\pm1$, so the
///   sign is that of $x$ at an even multiple and the opposite at an odd one.
/// - If $x/u$ is an odd multiple of $1/4$, the result is exactly $\\pm0.0$, and if it is an odd
///   multiple of $1/8$, exactly $\\pm1$.
/// - If $x/u$ in lowest terms has denominator 3 or 6, the result is $\\pm\\sqrt3/3$, and if it has
///   denominator 12, $\\pm\\sqrt3$.
///
/// Overflow happens at a pole, where the result is exactly $\pm\infty$, and for a tiny $x/u$, whose
/// cotangent is close to $u/(2\pi x)$: an [`f32`] or [`f64`] whose fraction of a turn is not a
/// multiple of $1/2$ is more than $2^{-66}$ of a turn away from one, so a cotangent that is not a
/// pole stays below $2^{64}$ unless the angle itself is tiny. Underflow happens only at an odd
/// quarter turn, where the result is exactly $\pm0.0$: a fraction of a turn that is not one is more
/// than $2^{-66}$ away from it, so the cotangent stays above $2^{-67}$.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::cot::primitive_float_cot_with_period;
///
/// assert!(primitive_float_cot_with_period(f32::NAN, 360).is_nan());
/// assert!(primitive_float_cot_with_period(f32::INFINITY, 360).is_nan());
/// assert!(primitive_float_cot_with_period(f32::NEGATIVE_INFINITY, 360).is_nan());
/// assert!(primitive_float_cot_with_period(1.0f32, 0).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_cot_with_period(-0.0f32, 360)),
///     NiceFloat(f32::NEGATIVE_INFINITY)
/// );
/// // a quarter turn is exactly 0
/// assert_eq!(
///     NiceFloat(primitive_float_cot_with_period(90.0f32, 360)),
///     NiceFloat(0.0)
/// );
/// // a half turn is a pole
/// assert_eq!(
///     NiceFloat(primitive_float_cot_with_period(180.0f32, 360)),
///     NiceFloat(f32::NEGATIVE_INFINITY)
/// );
/// // a sixth of a turn: sqrt(3)/3
/// assert_eq!(
///     NiceFloat(primitive_float_cot_with_period(60.0f32, 360)),
///     NiceFloat(0.57735026)
/// );
/// // a twelfth of a turn: sqrt(3)
/// assert_eq!(
///     NiceFloat(primitive_float_cot_with_period(30.0f64, 360)),
///     NiceFloat(1.7320508075688772)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_cot_with_period(1.0f32, 7)),
///     NiceFloat(0.7974734)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_cot_with_period(1.0f64, 7)),
///     NiceFloat(0.7974733888824039)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_cot_with_period<T: PrimitiveFloat>(x: T, u: u64) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(|x, prec| Float::cot_with_period_prec(x, u, prec), x)
}

/// Computes $\cot(2\pi x/u)$, the cotangent of a [`Rational`] measured in $u$ths of a turn (so that
/// `u = 360` is degrees), returning the result as a primitive float.
///
/// $$
/// f(x,u) = \cot(2\pi x/u)+\varepsilon.
/// $$
/// - If $u=0$ or $x/u$ is a multiple of $1/8$, $\varepsilon$ may be ignored or assumed to be 0.
/// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |\cot(2\pi x/u)|\rfloor-p}$, where $p$ is the
///   precision of the output (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]).
///
/// Special cases:
/// - $f(x,0)=\text{NaN}$
/// - $f(0,u)=\infty$
/// - If $x/u$ is a multiple of $1/2$, the cotangent has a pole there, and the result is exactly
///   $\\pm\\infty$: the sine is a zero carrying the sign of $x$ and the cosine is $\\pm1$, so the
///   sign is that of $x$ at an even multiple and the opposite at an odd one.
/// - If $x/u$ is an odd multiple of $1/4$, the result is exactly $\\pm0.0$, and if it is an odd
///   multiple of $1/8$, exactly $\\pm1$.
/// - If $x/u$ in lowest terms has denominator 3 or 6, the result is $\\pm\\sqrt3/3$, and if it has
///   denominator 12, $\\pm\\sqrt3$.
///
/// Overflow is possible away from a pole too: a fraction of a turn within about $2^{-130}$ of a
/// multiple of $1/2$ has a cotangent beyond the largest [`f32`], and one within about $2^{-1026}$
/// of one beyond the largest [`f64`]; so does a fraction of a turn small enough on its own, which a
/// [`Rational`] can be however large its denominator is not. The result is then $\pm\infty$. A
/// fraction of a turn as close to an odd multiple of $1/4$ underflows instead, to $\pm0.0$.
///
/// # Worst-case complexity
/// $T(m) = O(m (\log m)^2 \log\log m)$
///
/// $M(m) = O(m \log m)$
///
/// where $T$ is time, $M$ is additional memory, and $m$ is `x.significant_bits()`: the fraction of
/// a turn is reduced modulo 1 exactly, so the magnitude of $x$ does not drive the cost.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::cot::primitive_float_cot_with_period_rational;
/// use malachite_q::Rational;
///
/// assert!(primitive_float_cot_with_period_rational::<f64>(&Rational::ZERO, 0).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_cot_with_period_rational::<f64>(
///         &Rational::ZERO,
///         360
///     )),
///     NiceFloat(f64::INFINITY)
/// );
/// // a quarter turn is exactly 0
/// assert_eq!(
///     NiceFloat(primitive_float_cot_with_period_rational::<f64>(
///         &Rational::from_unsigneds(1u8, 4),
///         1
///     )),
///     NiceFloat(0.0)
/// );
/// // an eighth of a turn is exactly 1
/// assert_eq!(
///     NiceFloat(primitive_float_cot_with_period_rational::<f64>(
///         &Rational::from_unsigneds(1u8, 8),
///         1
///     )),
///     NiceFloat(1.0)
/// );
/// // a twelfth of a turn: sqrt(3)
/// assert_eq!(
///     NiceFloat(primitive_float_cot_with_period_rational::<f64>(
///         &Rational::from_unsigneds(1u8, 12),
///         1
///     )),
///     NiceFloat(1.7320508075688772)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_cot_with_period_rational::<f32>(
///         &Rational::from_unsigneds(1u8, 7),
///         1
///     )),
///     NiceFloat(0.7974734)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_cot_with_period_rational::<f64>(
///         &Rational::from_unsigneds(1u8, 7),
///         1
///     )),
///     NiceFloat(0.7974733888824039)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_cot_with_period_rational<T: PrimitiveFloat>(x: &Rational, u: u64) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_to_float_fn(
        |x, prec| Float::cot_with_period_rational_prec_ref(x, u, prec),
        x,
    )
}
