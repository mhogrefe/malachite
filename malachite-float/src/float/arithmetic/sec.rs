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

// Port of MPFR's secant. `mpfr_sec` (`sec.c`) instantiates the generic reciprocal template
// (`gen_inverse.h`) with the cosine: the cosine is taken at the working precision, rounded toward
// zero, its reciprocal is rounded to nearest, and the result is certified with two bits of slack,
// inside a Ziv loop. The secant never underflows, since its magnitude is at least 1, but it
// overflows for an input within 2^(-2^30) of an odd multiple of pi/2, which MPFR's wider exponent
// range never sees; a reciprocal at the top of the range is decided from an exact bracket instead.

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::float::arithmetic::cos::{
    cos_rational_helper, cos_turns_helper, phi_minus_1_prec_round, signed_constant,
};
use crate::float::arithmetic::round_near_x::{float_round_near_x, round_from_below};
use crate::float::arithmetic::tan::{MAX_SETTLED_EXPONENT, round_bracket_signed_by};
use crate::{Float, emulate_float_to_float_fn, emulate_rational_to_float_fn};
use core::cmp::Ordering::{self, Equal, Greater, Less};
use core::cmp::{max, min};
use malachite_base::num::arithmetic::traits::{
    Abs, CeilingLogBase2, Mod, PowerOf2, Reciprocal, Sec, SecAssign,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, NegativeInfinity, One,
};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, Ceiling, Down, Exact, Floor, Nearest};
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::float::round::float_can_round;
use malachite_nz::platform::Limb;
use malachite_q::Rational;

// As in mpfr_overflow, with the overflow's sign: the toward-zero modes give the largest finite
// value, and the other modes an infinity.
fn sec_overflow(negative: bool, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    match (negative, rm) {
        (_, Exact) => panic!("Inexact sec"),
        (false, Floor | Down) => (Float::max_finite_value_with_prec(prec), Less),
        (false, _) => (Float::INFINITY, Greater),
        (true, Ceiling | Down) => (-Float::max_finite_value_with_prec(prec), Greater),
        (true, _) => (Float::NEGATIVE_INFINITY, Less),
    }
}

// Decides sec(x) = 1/c from the cosine rounded toward zero at precision m, by a `Rational` bracket,
// for the cases the `Float` reciprocal cannot settle: it overflowed, or lies within two bits of the
// top of the exponent range, where rounding it to `prec` could still cross the end. Returns `None`
// if the bracket does not decide the rounding, so that the working precision must grow.
fn sec_bracket(c: &Float, m: u64, prec: u64, rm: RoundingMode) -> Option<(Float, Ordering)> {
    let negative = c.is_sign_negative();
    // A cosine that underflowed toward zero is below the smallest positive `Float`, so the secant
    // is above 2^(2^30), beyond the largest finite one.
    if *c == 0u32 {
        return Some(sec_overflow(negative, prec, rm));
    }
    // Rounding toward zero puts the cosine's magnitude in [|c|, |c| + ulp), so the secant's lies in
    // (1/(|c| + ulp), 1/|c|].
    let exp_c = i64::from(c.get_exponent().unwrap());
    let lo = Rational::exact_from(c).abs();
    let hi = &lo + Rational::power_of_2(exp_c - i64::exact_from(m));
    round_bracket_signed_by(negative, hi.reciprocal(), lo.reciprocal(), prec, rm)
}

// This is mpfr_sec from sec.c, MPFR 4.2.2, with the bracket path for results near the top of the
// exponent range.
fn sec_prec_round_normal_ref(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact sec");
    let exp_x = i64::from(x.get_exponent().unwrap());
    // sec(x) = 1 + x^2/2 + ..., more precisely |sec(x) - 1| < x^2 for |x| <= 1, so the error is
    // below 2^(2*EXP(x)) and lies above 1.
    //
    // MPFR_FAST_COMPUTE_IF_SMALL_INPUT (y, __gmpfr_one, -2 * MPFR_GET_EXP (x), 0, 1, r, ...)
    let neg_err = -(exp_x << 1);
    if neg_err > 0 {
        let err = u64::exact_from(neg_err);
        if err > prec + 1 {
            // The reference value 1 has precision 1 < err, so float_round_near_x always succeeds.
            // The error bound only has to clear prec + 1; passing an enormous err (a tiny x has one
            // around 2^31) would make float_round_near_x do work proportional to it.
            return float_round_near_x(&Float::ONE, min(err, prec + 2), true, prec, rm).unwrap();
        }
    }
    // Compute initial precision
    let mut m = prec + prec.ceiling_log_base_2() + 3;
    let mut increment = Limb::WIDTH;
    loop {
        // err < 1 ulp, and of a known sign: rounding toward zero puts the cosine below the true one
        // in magnitude
        let c = x.cos_prec_round_ref(m, Down).0;
        // err < 1/2 + 2 < 4 ulps in all, as in algorithms.tex
        let r = (&c).reciprocal();
        // A reciprocal whose exponent is below MAX_SETTLED_EXPONENT can be rounded to any precision
        // without leaving the exponent range, so the `Float` reciprocal settles it; the rest go to
        // the bracket. The secant's magnitude is at least 1, so only the top of the range is in
        // play.
        match r.get_exponent().map(i64::from) {
            Some(e) if e < MAX_SETTLED_EXPONENT => {
                if float_can_round(r.significand_ref().unwrap(), m - 2, prec, rm) {
                    return Float::from_float_prec_round(r, prec, rm);
                }
            }
            _ => {
                if let Some(result) = sec_bracket(&c, m, prec, rm) {
                    return result;
                }
            }
        }
        m += increment;
        increment = m >> 1;
    }
}

// Computes sec(x) for a nonzero `Rational` x, rounded to precision `prec` with rounding mode `rm`.
// (sec(0) = 1 is handled by the caller.) The secant of a nonzero rational is transcendental, so the
// result is never exactly representable and `rm` must not be `Exact`.
//
// This is the `Float` algorithm with the cosine taken from `cos_rational_helper`, which rounds the
// input once and handles both a tiny x and an x too large to be a `Float`, and with a direct
// bracket for a tiny input, where sec x is 1 + x^2/2 + O(x^4).
pub(crate) fn sec_rational_helper(x: &Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact sec");
    let exp_x = x.floor_log_base_2_abs() + 1; // the MPFR-style exponent of x
    // sec(x) = 1 + x^2/2 + 5x^4/24 + ..., with every term positive, and for |x| <= 1/2 the terms
    // past x^2/2 sum to less than x^4, so [1 + x^2/2, 1 + x^2/2 + x^4] brackets the secant. x^2 <
    // 2^(-prec - 1) here, so the secant lies strictly between 1 and 1 + 2^(-prec - 1), short of the
    // next `Float` above 1 and of the midpoint below it: the answer is 1 itself, nudged up by the
    // rounding mode. Forming the bracket [1 + x^2/2, 1 + x^2/2 + x^4] exactly would say the same,
    // at the cost of a dense `Rational` of about 2 |EXP(x)| bits -- 14 seconds for x =
    // 2^-536870908.
    if -(exp_x << 1) > i64::exact_from(prec) + 1 {
        return round_from_below(Float::one_prec(prec), Equal, false, rm);
    }
    let mut m = prec + prec.ceiling_log_base_2() + 3;
    let mut increment = Limb::WIDTH;
    loop {
        // err < 1 ulp, and of a known sign: rounding toward zero puts the cosine below the true one
        // in magnitude
        let c = cos_rational_helper(x, m, Down).0;
        // err < 1/2 + 2 < 4 ulps in all, as in algorithms.tex
        let r = (&c).reciprocal();
        match r.get_exponent().map(i64::from) {
            Some(e) if e < MAX_SETTLED_EXPONENT => {
                if float_can_round(r.significand_ref().unwrap(), m - 2, prec, rm) {
                    return Float::from_float_prec_round(r, prec, rm);
                }
            }
            _ => {
                if let Some(result) = sec_bracket(&c, m, prec, rm) {
                    return result;
                }
            }
        }
        m += increment;
        increment = m >> 1;
    }
}

// The exact and closed-form values of sec(2 pi q) at the eighths and twelfths of a turn, where the
// cosine is 0, ±1, ±1/2, ±sqrt(2)/2, or ±sqrt(3)/2. Returns `None` when q is none of them, or
// when only an inexact value is available and `rm` is `Exact`.
fn sec_turns_special_case(q: &Rational, prec: u64, rm: RoundingMode) -> Option<(Float, Ordering)> {
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
        2 | 4 | 8 => match n * (8 / d) {
            // sec(180°) = -1
            4 => Some((-Float::one_prec(prec), Equal)),
            // The poles at 90° and 270°. The cosine returns +0.0 at both, so its reciprocal is
            // +infinity at both; that keeps the secant the exact reciprocal of the cosine, and
            // keeps it even, which taking the sign of the approach would not.
            2 | 6 => Some((Float::INFINITY, Equal)),
            _ if rm == Exact => None,
            // sec(45°) = sec(315°) = sqrt(2), sec(135°) = sec(225°) = -sqrt(2)
            1 | 7 => Some(signed_constant(Float::sqrt_2_prec_round, false, prec, rm)),
            _ => Some(signed_constant(Float::sqrt_2_prec_round, true, prec, rm)),
        },
        // twelfths of a turn
        3 | 6 | 12 => match n * (12 / d) {
            // sec(60°) = sec(300°) = 2, sec(120°) = sec(240°) = -2
            2 | 10 => Some((Float::one_prec(prec) << 1u32, Equal)),
            4 | 8 => Some((-(Float::one_prec(prec) << 1u32), Equal)),
            _ if rm == Exact => None,
            // sec(30°) = sec(330°) = 2 sqrt(3)/3, sec(150°) = sec(210°) = -2 sqrt(3)/3.
            // Doubling is exact, so the correctly rounded constant stays correctly rounded.
            1 | 11 => Some(doubled(signed_constant(
                Float::sqrt_3_over_3_prec_round,
                false,
                prec,
                rm,
            ))),
            _ => Some(doubled(signed_constant(
                Float::sqrt_3_over_3_prec_round,
                true,
                prec,
                rm,
            ))),
        },
        _ if rm == Exact => None,
        // Fifths and tenths of a turn, where the cosine is ±phi/2 or ±(phi - 1)/2, so the secant
        // is ±2(phi - 1) or ±2 phi. sec(72°) = 2 phi, sec(144°) = -2(phi - 1)
        5 => Some(if n == 1 || n == 4 {
            doubled(signed_constant(Float::phi_prec_round, false, prec, rm))
        } else {
            doubled(signed_constant(phi_minus_1_prec_round, true, prec, rm))
        }),
        // sec(36°) = 2(phi - 1), sec(108°) = -2 phi
        10 => Some(if n == 1 || n == 9 {
            doubled(signed_constant(phi_minus_1_prec_round, false, prec, rm))
        } else {
            doubled(signed_constant(Float::phi_prec_round, true, prec, rm))
        }),
        _ => None,
    }
}

// Multiplies a correctly rounded value by 2, which is exact and so leaves the `Ordering` alone.
pub(crate) fn doubled((x, o): (Float, Ordering)) -> (Float, Ordering) {
    (x << 1u32, o)
}

// Computes sec(2 pi x/u) for a finite nonzero `Float` x and a nonzero u. This has no MPFR
// counterpart; it is `sec` with the cosine taken in uths of a turn, which reduces the argument
// exactly rather than modulo an approximation of 2 pi, and so reaches the exact and closed-form
// cases that the radian version cannot see.
fn sec_with_period_prec_round_normal_ref(
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
            // x is a multiple of u, so the cosine is 1 and the secant is 1
            return (Float::one_prec(prec), Equal);
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
            sec_turns_special_case(&(Rational::exact_from(xp) / Rational::from(u)), prec, rm)
    {
        return result;
    }
    // Only the exact cases can be rounded exactly
    assert_ne!(rm, Exact, "Inexact sec_with_period");
    // u >= 2^log2u, so |2 pi x/u| < 2^(exp_x + 3 - log2u)
    let log2u = if u == 1 {
        0
    } else {
        i64::exact_from(u.ceiling_log_base_2()) - 1
    };
    let bound = exp_x + 3 - log2u;
    if bound < 0 {
        // sec(t) = 1 + t^2/2 + ..., and |sec(t) - 1| < t^2 for |t| <= 1, so the error is below 2^(2
        // bound). Without this shortcut the loop below would have to raise the working precision to
        // about twice the angle's exponent, which is unbounded for an angle below the `Float`
        // exponent range.
        let err = u64::exact_from(-(bound << 1));
        if err > prec + 1 {
            // The reference value 1 has precision 1 < err, so float_round_near_x always succeeds.
            return float_round_near_x(&Float::ONE, min(err, prec + 2), true, prec, rm).unwrap();
        }
    }
    let mut m = prec + prec.ceiling_log_base_2() + 3;
    let mut increment = Limb::WIDTH;
    loop {
        // err < 1 ulp, and of a known sign: rounding toward zero puts the cosine below the true one
        // in magnitude
        let c = xp.cos_with_period_prec_round_ref(u, m, Down).0;
        // err < 1/2 + 2 < 4 ulps in all, as in algorithms.tex
        let r = (&c).reciprocal();
        match r.get_exponent().map(i64::from) {
            Some(e) if e < MAX_SETTLED_EXPONENT => {
                if float_can_round(r.significand_ref().unwrap(), m - 2, prec, rm) {
                    return Float::from_float_prec_round(r, prec, rm);
                }
            }
            _ => {
                if let Some(result) = sec_bracket(&c, m, prec, rm) {
                    return result;
                }
            }
        }
        m += increment;
        increment = m >> 1;
    }
}

// Computes sec(2 pi q) for a nonzero fraction of a turn q with |q| < 1, rounded to precision `prec`
// with rounding mode `rm`. This is the `Rational` counterpart of
// `sec_with_period_prec_round_normal_ref`, with the same structure: the small-input shortcut, the
// closed-form cases, and a Ziv loop around the reciprocal of `cos_turns_helper`. `rm` may be
// `Exact` only in the exact cases.
fn sec_turns_helper(q: &Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    let exp_q = q.floor_log_base_2_abs() + 1;
    // sec(t) = 1 + t^2/2 + ... with |sec(t) - 1| < t^2 for |t| <= 1, and |2 pi q| < 2^(exp_q + 3),
    // so |sec(2 pi q) - 1| < 2^(6 + 2 EXP(q)), and the secant lies above 1
    let err = -(exp_q << 1) - 6;
    if err > 0 {
        let err = u64::exact_from(err);
        if err > prec + 1 {
            // As in the `Float` version: the reference value 1 always rounds, the bound need not
            // exceed prec + 2, and such a tiny q is neither a special case nor exact.
            assert_ne!(rm, Exact, "Inexact sec_with_period");
            return float_round_near_x(&Float::ONE, min(err, prec + 2), true, prec, rm).unwrap();
        }
    }
    // The special cases need |q| >= 1/12
    if exp_q >= -4
        && let Some(result) = sec_turns_special_case(q, prec, rm)
    {
        return result;
    }
    // Only the exact cases can be rounded exactly
    assert_ne!(rm, Exact, "Inexact sec_with_period");
    let mut m = prec + prec.ceiling_log_base_2() + 3;
    let mut increment = Limb::WIDTH;
    loop {
        // err < 1 ulp, and of a known sign: rounding toward zero puts the cosine below the true one
        // in magnitude
        let c = cos_turns_helper(q, m, Down).0;
        // err < 1/2 + 2 < 4 ulps in all, as in algorithms.tex
        let r = (&c).reciprocal();
        match r.get_exponent().map(i64::from) {
            Some(e) if e < MAX_SETTLED_EXPONENT => {
                if float_can_round(r.significand_ref().unwrap(), m - 2, prec, rm) {
                    return Float::from_float_prec_round(r, prec, rm);
                }
            }
            _ => {
                if let Some(result) = sec_bracket(&c, m, prec, rm) {
                    return result;
                }
            }
        }
        m += increment;
        increment = m >> 1;
    }
}

impl Float {
    /// Computes $\sec x$, the secant of a [`Float`], rounding the result to the specified precision
    /// and with the specified rounding mode. The [`Float`] is taken by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded secant is less than, equal to, or greater than
    /// the exact secant. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \sec x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\sec
    ///   x|\rfloor-p+1}$.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\sec
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=1.0$
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
    /// Underflow is not possible, since $|\sec x| \geq 1$. Overflow requires an input within
    /// $2^{-2^{30}}$ of an odd multiple of $\pi/2$, which takes more than $2^{30}$ bits of
    /// precision.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sec_prec`] instead. If you
    /// know that your target precision is the precision of the input, consider using
    /// [`Float::sec_round`] instead. If both of these things are true, consider using
    /// [`Float::sec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the cosine at working precision $n$, summed by binary splitting of the
    /// Taylor series for large $n$, and its reciprocal cost the first term, and for $|x| \geq 4$
    /// the argument is reduced modulo $2\pi$, which requires $\pi$ to about $n + e$ bits and a
    /// remainder of the $m$-bit input. Unlike most functions, `sec` therefore gets slower as the
    /// magnitude of its input grows, not just as the precision does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the secant of a finite nonzero [`Float`] is never exactly
    /// representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sec_prec_round(5, Floor);
    /// assert_eq!(c.to_string(), "1.81");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sec_prec_round(5, Ceiling);
    /// assert_eq!(c.to_string(), "1.88");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sec_prec_round(5, Nearest);
    /// assert_eq!(c.to_string(), "1.88");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sec_prec_round(20, Floor);
    /// assert_eq!(c.to_string(), "1.8508148");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sec_prec_round(20, Ceiling);
    /// assert_eq!(c.to_string(), "1.8508167");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sec_prec_round(20, Nearest);
    /// assert_eq!(c.to_string(), "1.8508148");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sec_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.sec_prec_round_ref(prec, rm)
    }

    /// Computes $\sec x$, the secant of a [`Float`], rounding the result to the specified precision
    /// and with the specified rounding mode. The [`Float`] is taken by reference. An [`Ordering`]
    /// is also returned, indicating whether the rounded secant is less than, equal to, or greater
    /// than the exact secant. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \sec x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\sec
    ///   x|\rfloor-p+1}$.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\sec
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=1.0$
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
    /// Underflow is not possible, since $|\sec x| \geq 1$. Overflow requires an input within
    /// $2^{-2^{30}}$ of an odd multiple of $\pi/2$, which takes more than $2^{30}$ bits of
    /// precision.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sec_prec_ref`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::sec_round_ref`] instead. If both of these things are true, consider using
    /// `(&Float).sec()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the cosine at working precision $n$, summed by binary splitting of the
    /// Taylor series for large $n$, and its reciprocal cost the first term, and for $|x| \geq 4$
    /// the argument is reduced modulo $2\pi$, which requires $\pi$ to about $n + e$ bits and a
    /// remainder of the $m$-bit input. Unlike most functions, `sec` therefore gets slower as the
    /// magnitude of its input grows, not just as the precision does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the secant of a finite nonzero [`Float`] is never exactly
    /// representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).sec_prec_round_ref(5, Floor);
    /// assert_eq!(c.to_string(), "1.81");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).sec_prec_round_ref(5, Ceiling);
    /// assert_eq!(c.to_string(), "1.88");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).sec_prec_round_ref(5, Nearest);
    /// assert_eq!(c.to_string(), "1.88");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).sec_prec_round_ref(20, Floor);
    /// assert_eq!(c.to_string(), "1.8508148");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).sec_prec_round_ref(20, Ceiling);
    /// assert_eq!(c.to_string(), "1.8508167");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).sec_prec_round_ref(20, Nearest);
    /// assert_eq!(c.to_string(), "1.8508148");
    /// assert_eq!(o, Less);
    /// ```
    pub fn sec_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match &self.0 {
            NaN | Infinity { .. } => (Self::NAN, Equal),
            // sec(+0) = sec(-0) = 1
            Zero { .. } => (Self::one_prec(prec), Equal),
            Finite { .. } => sec_prec_round_normal_ref(self, prec, rm),
        }
    }

    /// Computes $\sec x$, the secant of a [`Float`], rounding the result to the nearest value of
    /// the specified precision. The [`Float`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded secant is less than, equal to, or greater than the exact
    /// secant. Although `NaN`s are not comparable to any [`Float`], whenever this function returns
    /// a `NaN` it also returns `Equal`.
    ///
    /// If the secant is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \sec x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\sec x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\text{NaN}$
    /// - $f(\pm0.0,p)=1.0$
    ///
    /// Overflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    ///
    /// Underflow is not possible, since $|\sec x| \geq 1$. Overflow requires an input within
    /// $2^{-2^{30}}$ of an odd multiple of $\pi/2$, which takes more than $2^{30}$ bits of
    /// precision.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sec_prec_round`] instead. If you know that your target precision is the precision
    /// of the input, consider using [`Float::sec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the cosine at working precision $n$, summed by binary splitting of the
    /// Taylor series for large $n$, and its reciprocal cost the first term, and for $|x| \geq 4$
    /// the argument is reduced modulo $2\pi$, which requires $\pi$ to about $n + e$ bits and a
    /// remainder of the $m$-bit input. Unlike most functions, `sec` therefore gets slower as the
    /// magnitude of its input grows, not just as the precision does.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.sec_prec(5);
    /// assert_eq!(c.to_string(), "1.88");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.sec_prec(20);
    /// assert_eq!(c.to_string(), "1.8508148");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sec_prec(self, prec: u64) -> (Self, Ordering) {
        self.sec_prec_round(prec, Nearest)
    }

    /// Computes $\sec x$, the secant of a [`Float`], rounding the result to the nearest value of
    /// the specified precision. The [`Float`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded secant is less than, equal to, or greater than the
    /// exact secant. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// If the secant is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \sec x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\sec x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\text{NaN}$
    /// - $f(\pm0.0,p)=1.0$
    ///
    /// Overflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    ///
    /// Underflow is not possible, since $|\sec x| \geq 1$. Overflow requires an input within
    /// $2^{-2^{30}}$ of an odd multiple of $\pi/2$, which takes more than $2^{30}$ bits of
    /// precision.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sec_prec_round_ref`] instead. If you know that your target precision is the
    /// precision of the input, consider using `(&Float).sec()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the cosine at working precision $n$, summed by binary splitting of the
    /// Taylor series for large $n$, and its reciprocal cost the first term, and for $|x| \geq 4$
    /// the argument is reduced modulo $2\pi$, which requires $\pi$ to about $n + e$ bits and a
    /// remainder of the $m$-bit input. Unlike most functions, `sec` therefore gets slower as the
    /// magnitude of its input grows, not just as the precision does.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).sec_prec_ref(5);
    /// assert_eq!(c.to_string(), "1.88");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).sec_prec_ref(20);
    /// assert_eq!(c.to_string(), "1.8508148");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sec_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.sec_prec_round_ref(prec, Nearest)
    }

    /// Computes $\sec x$, the secant of a [`Float`], rounding the result with the specified
    /// rounding mode. The [`Float`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded secant is less than, equal to, or greater than the exact secant.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \sec x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\sec
    ///   x|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\sec
    ///   x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\pm\infty,m)=\text{NaN}$
    /// - $f(\pm0.0,m)=1.0$
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
    /// Underflow is not possible, since $|\sec x| \geq 1$. Overflow requires an input within
    /// $2^{-2^{30}}$ of an odd multiple of $\pi/2$, which takes more than $2^{30}$ bits of
    /// precision.
    ///
    /// If you want to specify an output precision, consider using [`Float::sec_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::sec`] instead.
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
    /// e$ bits. Unlike most functions, `sec` therefore gets slower as the magnitude of its input
    /// grows, not just as the precision does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the secant of a finite nonzero [`Float`] is never exactly
    /// representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.sec_round(Floor);
    /// assert_eq!(c.to_string(), "1.8508157176809256179117532413979");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.sec_round(Ceiling);
    /// assert_eq!(c.to_string(), "1.8508157176809256179117532413995");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.sec_round(Nearest);
    /// assert_eq!(c.to_string(), "1.8508157176809256179117532413979");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sec_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.sec_prec_round(prec, rm)
    }

    /// Computes $\sec x$, the secant of a [`Float`], rounding the result with the specified
    /// rounding mode. The [`Float`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded secant is less than, equal to, or greater than the exact
    /// secant. Although `NaN`s are not comparable to any [`Float`], whenever this function returns
    /// a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \sec x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\sec
    ///   x|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\sec
    ///   x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\pm\infty,m)=\text{NaN}$
    /// - $f(\pm0.0,m)=1.0$
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
    /// Underflow is not possible, since $|\sec x| \geq 1$. Overflow requires an input within
    /// $2^{-2^{30}}$ of an odd multiple of $\pi/2$, which takes more than $2^{30}$ bits of
    /// precision.
    ///
    /// If you want to specify an output precision, consider using [`Float::sec_prec_round_ref`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// `(&Float).sec()` instead.
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
    /// e$ bits. Unlike most functions, `sec` therefore gets slower as the magnitude of its input
    /// grows, not just as the precision does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the secant of a finite nonzero [`Float`] is never exactly
    /// representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).sec_round_ref(Floor);
    /// assert_eq!(c.to_string(), "1.8508157176809256179117532413979");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).sec_round_ref(Ceiling);
    /// assert_eq!(c.to_string(), "1.8508157176809256179117532413995");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).sec_round_ref(Nearest);
    /// assert_eq!(c.to_string(), "1.8508157176809256179117532413979");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sec_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.sec_prec_round_ref(self.significant_bits(), rm)
    }

    /// Computes $\sec x$, the secant of a [`Float`], rounding the result to the specified precision
    /// and with the specified rounding mode. The [`Float`] is replaced by the result, and an
    /// [`Ordering`] is returned, indicating whether the rounded secant is less than, equal to, or
    /// greater than the exact secant. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function sets a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \sec x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\sec
    ///   x|\rfloor-p+1}$.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\sec
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::sec_prec_round`] documentation for information on special cases and
    /// overflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sec_prec_assign`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::sec_round_assign`] instead. If both of these things are true, consider using
    /// [`Float::sec_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the cosine at working precision $n$, summed by binary splitting of the
    /// Taylor series for large $n$, and its reciprocal cost the first term, and for $|x| \geq 4$
    /// the argument is reduced modulo $2\pi$, which requires $\pi$ to about $n + e$ bits and a
    /// remainder of the $m$-bit input. Unlike most functions, `sec` therefore gets slower as the
    /// magnitude of its input grows, not just as the precision does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the secant of a finite nonzero [`Float`] is never exactly
    /// representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sec_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "1.81");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sec_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.88");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sec_prec_round_assign(5, Nearest), Greater);
    /// assert_eq!(x.to_string(), "1.88");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sec_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "1.8508148");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sec_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.8508167");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sec_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "1.8508148");
    /// ```
    #[inline]
    pub fn sec_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let o;
        (*self, o) = self.sec_prec_round_ref(prec, rm);
        o
    }

    /// Computes $\sec x$, the secant of a [`Float`], rounding the result to the nearest value of
    /// the specified precision. The [`Float`] is replaced by the result, and an [`Ordering`] is
    /// returned, indicating whether the rounded secant is less than, equal to, or greater than the
    /// exact secant. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// sets a `NaN` it also returns `Equal`.
    ///
    /// If the secant is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \sec x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\sec x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::sec_prec`] documentation for information on special cases and overflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sec_prec_round_assign`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::sec_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the cosine at working precision $n$, summed by binary splitting of the
    /// Taylor series for large $n$, and its reciprocal cost the first term, and for $|x| \geq 4$
    /// the argument is reduced modulo $2\pi$, which requires $\pi$ to about $n + e$ bits and a
    /// remainder of the $m$-bit input. Unlike most functions, `sec` therefore gets slower as the
    /// magnitude of its input grows, not just as the precision does.
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
    /// assert_eq!(x.sec_prec_assign(5), Greater);
    /// assert_eq!(x.to_string(), "1.88");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sec_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "1.8508148");
    /// ```
    #[inline]
    pub fn sec_prec_assign(&mut self, prec: u64) -> Ordering {
        self.sec_prec_round_assign(prec, Nearest)
    }

    /// Computes $\sec x$, the secant of a [`Float`], rounding the result with the specified
    /// rounding mode. The [`Float`] is replaced by the result, and an [`Ordering`] is returned,
    /// indicating whether the rounded secant is less than, equal to, or greater than the exact
    /// secant. Although `NaN`s are not comparable to any [`Float`], whenever this function sets a
    /// `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \sec x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\sec
    ///   x|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\sec
    ///   x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::sec_round`] documentation for information on special cases and overflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::sec_prec_round_assign`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::sec_assign`] instead.
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
    /// e$ bits. Unlike most functions, `sec` therefore gets slower as the magnitude of its input
    /// grows, not just as the precision does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the secant of a finite nonzero [`Float`] is never exactly
    /// representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sec_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "1.8508157176809256179117532413979");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sec_round_assign(Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.8508157176809256179117532413995");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sec_round_assign(Nearest), Less);
    /// assert_eq!(x.to_string(), "1.8508157176809256179117532413979");
    /// ```
    #[inline]
    pub fn sec_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.sec_prec_round_assign(prec, rm)
    }

    /// Computes $\sec x$, the secant of a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded secant is less than, equal to, or greater than the exact secant.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \sec x+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\sec x|\rfloor-p+1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\sec x|\rfloor-p}$.
    ///
    /// These bounds do not apply when the result overflows; see below.
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=1$.
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
    /// Underflow is not possible, since $|\sec x| \geq 1$. Overflow requires an input within
    /// $2^{-2^{30}}$ of an odd multiple of $\pi/2$, which takes a denominator of more than $2^{30}$
    /// bits.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sec_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is `x.significant_bits()`,
    /// and $e$ is `x.floor_log_base_2_abs()` (taken as 0 when it is negative or $x = 0$): the input
    /// is rounded to a working precision and its [`Float`] cosine taken there, then reciprocated,
    /// which for $|x| \geq 2$ reduces the argument modulo $2\pi$ and so needs $\pi$ to about $n +
    /// e$ bits.
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
    /// let (c, o) = Float::sec_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(c.to_string(), "1.19");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::sec_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Ceiling);
    /// assert_eq!(c.to_string(), "1.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::sec_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Floor);
    /// assert_eq!(c.to_string(), "1.2116280");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::sec_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Ceiling);
    /// assert_eq!(c.to_string(), "1.2116299");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn sec_rational_prec_round(x: Rational, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        Self::sec_rational_prec_round_ref(&x, prec, rm)
    }

    /// Computes $\sec x$, the secant of a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded secant is less than, equal to, or greater than the exact secant.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \sec x+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\sec x|\rfloor-p+1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\sec x|\rfloor-p}$.
    ///
    /// These bounds do not apply when the result overflows.
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=1$.
    ///
    /// See the [`Float::sec_rational_prec_round`] documentation for information on overflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sec_rational_prec_ref`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is `x.significant_bits()`,
    /// and $e$ is `x.floor_log_base_2_abs()` (taken as 0 when it is negative or $x = 0$): the input
    /// is rounded to a working precision and its [`Float`] cosine taken there, then reciprocated,
    /// which for $|x| \geq 2$ reduces the argument modulo $2\pi$ and so needs $\pi$ to about $n +
    /// e$ bits.
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
    ///     Float::sec_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(c.to_string(), "1.19");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) =
    ///     Float::sec_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 5, Ceiling);
    /// assert_eq!(c.to_string(), "1.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) =
    ///     Float::sec_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 20, Floor);
    /// assert_eq!(c.to_string(), "1.2116280");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) =
    ///     Float::sec_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 20, Ceiling);
    /// assert_eq!(c.to_string(), "1.2116299");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn sec_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        if *x == 0u32 {
            // sec(0) = 1, exactly
            return (Self::one_prec(prec), Equal);
        }
        sec_rational_helper(x, prec, rm)
    }

    /// Computes $\sec x$, the secant of a [`Rational`], rounding the result to the nearest value of
    /// the specified precision and returning the result as a [`Float`]. The [`Rational`] is taken
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded secant is less
    /// than, equal to, or greater than the exact secant.
    ///
    /// If the secant is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \sec x+\varepsilon,
    /// $$
    /// where $|\varepsilon| \leq 2^{\lfloor\log_2 |\sec x|\rfloor-p}$ (unless the result overflows;
    /// see below).
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=1$.
    ///
    /// Overflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    ///
    /// Underflow is not possible, since $|\sec x| \geq 1$. Overflow requires an input within
    /// $2^{-2^{30}}$ of an odd multiple of $\pi/2$, which takes a denominator of more than $2^{30}$
    /// bits.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sec_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is `x.significant_bits()`,
    /// and $e$ is `x.floor_log_base_2_abs()` (taken as 0 when it is negative or $x = 0$): the input
    /// is rounded to a working precision and its [`Float`] cosine taken there, then reciprocated,
    /// which for $|x| \geq 2$ reduces the argument modulo $2\pi$ and so needs $\pi$ to about $n +
    /// e$ bits.
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
    /// let (c, o) = Float::sec_rational_prec(Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(c.to_string(), "1.19");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::sec_rational_prec(Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(c.to_string(), "1.2116280");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn sec_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::sec_rational_prec_round_ref(&x, prec, Nearest)
    }

    /// Computes $\sec x$, the secant of a [`Rational`], rounding the result to the nearest value of
    /// the specified precision and returning the result as a [`Float`]. The [`Rational`] is taken
    /// by reference. An [`Ordering`] is also returned, indicating whether the rounded secant is
    /// less than, equal to, or greater than the exact secant.
    ///
    /// If the secant is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \sec x+\varepsilon,
    /// $$
    /// where $|\varepsilon| \leq 2^{\lfloor\log_2 |\sec x|\rfloor-p}$ (unless the result
    /// overflows).
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=1$.
    ///
    /// See the [`Float::sec_rational_prec`] documentation for information on overflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sec_rational_prec_round_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is `x.significant_bits()`,
    /// and $e$ is `x.floor_log_base_2_abs()` (taken as 0 when it is negative or $x = 0$): the input
    /// is rounded to a working precision and its [`Float`] cosine taken there, then reciprocated,
    /// which for $|x| \geq 2$ reduces the argument modulo $2\pi$ and so needs $\pi$ to about $n +
    /// e$ bits.
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
    /// let (c, o) = Float::sec_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(c.to_string(), "1.19");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::sec_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(c.to_string(), "1.2116280");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sec_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::sec_rational_prec_round_ref(x, prec, Nearest)
    }

    /// Computes $\sec(2\pi x/u)$, the secant of a [`Float`] measured in $u$ths of a turn, rounding
    /// the result to the specified precision and with the specified rounding mode. The [`Float`] is
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded secant is
    /// less than, equal to, or greater than the exact secant. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = \sec(2\pi x/u)+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $u=0$, or $x/u$ is an odd multiple of $1/4$, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $x$ is finite, $u\neq 0$, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\sec(2\pi x/u)|\rfloor-p+1}$.
    /// - If $x$ is finite, $u\neq 0$, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\sec(2\pi x/u)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p,m)=\text{NaN}$
    /// - $f(\pm\infty,u,p,m)=\text{NaN}$
    /// - $f(x,0,p,m)=\text{NaN}$
    /// - $f(\pm0.0,u,p,m)=1.0$
    /// - If $x/u$ is an even multiple of $1/2$, the result is exactly $1$, and at an odd multiple
    ///   exactly $-1$.
    /// - If $x/u$ is an odd multiple of $1/4$, the secant has a pole there, and the result is
    ///   exactly $\infty$: the cosine is $+0.0$ at every such point, and the secant is its
    ///   reciprocal.
    /// - If $x/u$ is an odd multiple of $1/8$, the result is $\pm\sqrt2$.
    ///
    /// When $x/u$ in lowest terms has denominator 3 or 6, the result is exactly $\pm2$; when it has
    /// denominator 5, 8, 10, or 12, the result is $\pm2\varphi$, $\pm\sqrt2$, $\pm2(\varphi-1)$, or
    /// $\pm2\sqrt3/3$, computed from a single correctly rounded constant rather than from $\pi$ and
    /// a cosine, which is far faster.
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
    /// Underflow is not possible, since $|\sec(2\pi x/u)| \geq 1$. Overflow requires $x/u$ within
    /// $2^{-2^{30}}$ of an odd multiple of $1/4$ without being one, which takes more than $2^{30}$
    /// bits of precision.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sec_with_period_prec`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::sec_with_period_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the argument is reduced modulo $u$ exactly, and the cosine of $2\pi x/u$ is
    /// then taken at a working precision of about $n + e$ bits, which needs $\pi$ to that many
    /// bits, and reciprocated.
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
    /// let (t, o) = Float::ONE.sec_with_period_prec_round(7, 10, Floor);
    /// assert_eq!(t.to_string(), "1.6035");
    /// assert_eq!(o, Less);
    ///
    /// let (t, o) = Float::ONE.sec_with_period_prec_round(7, 10, Ceiling);
    /// assert_eq!(t.to_string(), "1.6055");
    /// assert_eq!(o, Greater);
    ///
    /// // a quarter turn is a pole
    /// let (t, o) = Float::from(90u32).sec_with_period_prec_round(360, 10, Exact);
    /// assert_eq!(t.to_string(), "Infinity");
    /// assert_eq!(o, Equal);
    ///
    /// // a half turn is exactly -1
    /// let (t, o) = Float::from(180u32).sec_with_period_prec_round(360, 10, Exact);
    /// assert_eq!(t.to_string(), "-1.0000");
    /// assert_eq!(o, Equal);
    ///
    /// // a twelfth of a turn: 2 sqrt(3)/3
    /// let (t, o) = Float::from(30u32).sec_with_period_prec_round(360, 10, Nearest);
    /// assert_eq!(t.to_string(), "1.1543");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sec_with_period_prec_round(
        self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        self.sec_with_period_prec_round_ref(u, prec, rm)
    }

    /// Computes $\sec(2\pi x/u)$, the secant of a [`Float`] measured in $u$ths of a turn, rounding
    /// the result to the specified precision and with the specified rounding mode. The [`Float`] is
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded secant
    /// is less than, equal to, or greater than the exact secant. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::sec_with_period_prec_round`] for the error bounds, the special and closed-form
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
    /// let (t, o) = Float::ONE.sec_with_period_prec_round_ref(7, 10, Floor);
    /// assert_eq!(t.to_string(), "1.6035");
    /// assert_eq!(o, Less);
    /// ```
    pub fn sec_with_period_prec_round_ref(
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
            // x is zero: sec(±0) = 1
            Zero { .. } => (Self::one_prec(prec), Equal),
            Finite { .. } => sec_with_period_prec_round_normal_ref(self, u, prec, rm),
        }
    }

    /// Computes $\sec(2\pi x/u)$, the secant of a [`Float`] measured in $u$ths of a turn, rounding
    /// the result to the nearest value of the specified precision. The [`Float`] is taken by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded secant is less than, equal
    /// to, or greater than the exact secant. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the secant is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// See [`Float::sec_with_period_prec_round`] for the error bounds, the special and closed-form
    /// cases, overflow, and the complexity; this function behaves the same way with `Nearest`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sec_with_period_prec_round`] instead.
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
    /// let (t, o) = Float::ONE.sec_with_period_prec(7, 10);
    /// assert_eq!(t.to_string(), "1.6035");
    /// assert_eq!(o, Less);
    ///
    /// // an eighth of a turn: sqrt(2)
    /// let (t, o) = Float::ONE.sec_with_period_prec(8, 10);
    /// assert_eq!(t.to_string(), "1.4141");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sec_with_period_prec(self, u: u64, prec: u64) -> (Self, Ordering) {
        self.sec_with_period_prec_round(u, prec, Nearest)
    }

    /// Computes $\sec(2\pi x/u)$, the secant of a [`Float`] measured in $u$ths of a turn, rounding
    /// the result to the nearest value of the specified precision. The [`Float`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded secant is less
    /// than, equal to, or greater than the exact secant. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::sec_with_period_prec`] and [`Float::sec_with_period_prec_round`]; this function
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
    /// let (t, o) = Float::ONE.sec_with_period_prec_ref(7, 10);
    /// assert_eq!(t.to_string(), "1.6035");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sec_with_period_prec_ref(&self, u: u64, prec: u64) -> (Self, Ordering) {
        self.sec_with_period_prec_round_ref(u, prec, Nearest)
    }

    /// Computes $\sec(2\pi x/u)$, the secant of a [`Float`] measured in $u$ths of a turn, rounding
    /// the result to the precision of the input and with the specified rounding mode. The [`Float`]
    /// is taken by value. An [`Ordering`] is also returned, indicating whether the rounded secant
    /// is less than, equal to, or greater than the exact secant. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::sec_with_period_prec_round`] for the error bounds, the special and closed-form
    /// cases, overflow, and the complexity; this function behaves the same way with `prec` equal to
    /// the precision of the input.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::sec_with_period_prec_round`] instead.
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
    ///     .sec_with_period_round(7, Floor);
    /// assert_eq!(t.to_string(), "1.6035");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sec_with_period_round(self, u: u64, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.sec_with_period_prec_round(u, prec, rm)
    }

    /// Computes $\sec(2\pi x/u)$, the secant of a [`Float`] measured in $u$ths of a turn, rounding
    /// the result to the precision of the input and with the specified rounding mode. The [`Float`]
    /// is taken by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// secant is less than, equal to, or greater than the exact secant. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::sec_with_period_round`] and [`Float::sec_with_period_prec_round`]; this
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
    ///     .sec_with_period_round_ref(7, Floor);
    /// assert_eq!(t.to_string(), "1.6035");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sec_with_period_round_ref(&self, u: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.sec_with_period_prec_round_ref(u, self.significant_bits(), rm)
    }

    /// Computes $\sec(2\pi x/u)$, the secant of a [`Float`] measured in $u$ths of a turn (so that
    /// `u = 360` is degrees), rounding the result to the precision of the input and to the nearest
    /// [`Float`]. The [`Float`] is taken by value.
    ///
    /// If the secant is equidistant from two [`Float`]s with the precision of the input, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// See [`Float::sec_with_period_prec_round`] for the error bounds, the special and closed-form
    /// cases, overflow, and the complexity; this function behaves the same way with `prec` equal to
    /// the precision of the input and `rm` equal to `Nearest`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sec_with_period_round`] instead. If you want to specify an output precision,
    /// consider using [`Float::sec_with_period_prec`]. If you want both of these things, consider
    /// using [`Float::sec_with_period_prec_round`].
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// let t = Float::from_unsigned_prec(1u32, 10).0.sec_with_period(7);
    /// assert_eq!(t.to_string(), "1.6035");
    ///
    /// // a quarter turn is a pole
    /// assert_eq!(
    ///     Float::from(90u32).sec_with_period(360).to_string(),
    ///     "Infinity"
    /// );
    /// ```
    #[inline]
    pub fn sec_with_period(self, u: u64) -> Self {
        let prec = self.significant_bits();
        self.sec_with_period_prec(u, prec).0
    }

    /// Computes $\sec(2\pi x/u)$, the secant of a [`Float`] measured in $u$ths of a turn (so that
    /// `u = 360` is degrees), rounding the result to the precision of the input and to the nearest
    /// [`Float`]. The [`Float`] is taken by reference.
    ///
    /// If the secant is equidistant from two [`Float`]s with the precision of the input, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// See [`Float::sec_with_period_prec_round`] for the error bounds, the special and closed-form
    /// cases, overflow, and the complexity; this function behaves the same way with `prec` equal to
    /// the precision of the input and `rm` equal to `Nearest`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sec_with_period_round_ref`] instead. If you want to specify an output precision,
    /// consider using [`Float::sec_with_period_prec_ref`]. If you want both of these things,
    /// consider using [`Float::sec_with_period_prec_round_ref`].
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// let t = (&Float::from_unsigned_prec(1u32, 10).0).sec_with_period_ref(7);
    /// assert_eq!(t.to_string(), "1.6035");
    /// ```
    #[inline]
    pub fn sec_with_period_ref(&self, u: u64) -> Self {
        self.sec_with_period_prec_ref(u, self.significant_bits()).0
    }

    /// Replaces a [`Float`] measured in $u$ths of a turn with its secant, rounding the result to
    /// the specified precision and with the specified rounding mode. An [`Ordering`] is returned,
    /// indicating whether the rounded secant is less than, equal to, or greater than the exact
    /// secant. Although `NaN`s are not comparable to any [`Float`], whenever this function sets a
    /// `NaN` it also returns `Equal`.
    ///
    /// See [`Float::sec_with_period_prec_round`] for the error bounds, the special and closed-form
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
    /// assert_eq!(x.sec_with_period_prec_round_assign(7, 10, Floor), Less);
    /// assert_eq!(x.to_string(), "1.6035");
    /// ```
    #[inline]
    pub fn sec_with_period_prec_round_assign(
        &mut self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (t, o) = self.sec_with_period_prec_round_ref(u, prec, rm);
        *self = t;
        o
    }

    /// Replaces a [`Float`] measured in $u$ths of a turn with its secant, rounding the result to
    /// the nearest value of the specified precision. An [`Ordering`] is returned, indicating
    /// whether the rounded secant is less than, equal to, or greater than the exact secant.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function sets a `NaN` it
    /// also returns `Equal`.
    ///
    /// See [`Float::sec_with_period_prec`] and [`Float::sec_with_period_prec_round`]; this function
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
    /// assert_eq!(x.sec_with_period_prec_assign(7, 10), Less);
    /// assert_eq!(x.to_string(), "1.6035");
    /// ```
    #[inline]
    pub fn sec_with_period_prec_assign(&mut self, u: u64, prec: u64) -> Ordering {
        self.sec_with_period_prec_round_assign(u, prec, Nearest)
    }

    /// Replaces a [`Float`] measured in $u$ths of a turn with its secant, rounding the result to
    /// the precision of the input and with the specified rounding mode. An [`Ordering`] is
    /// returned, indicating whether the rounded secant is less than, equal to, or greater than the
    /// exact secant. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// sets a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::sec_with_period_round`] and [`Float::sec_with_period_prec_round`]; this
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
    /// assert_eq!(x.sec_with_period_round_assign(7, Floor), Less);
    /// assert_eq!(x.to_string(), "1.6035");
    /// ```
    #[inline]
    pub fn sec_with_period_round_assign(&mut self, u: u64, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.sec_with_period_prec_round_assign(u, prec, rm)
    }

    /// Computes $\sec(2\pi x/u)$, the secant of a [`Float`] measured in $u$ths of a turn (so that
    /// `u = 360` is degrees), rounding the result to the precision of the input and to the nearest
    /// [`Float`]. The [`Float`] is replaced by the result.
    ///
    /// If the secant is equidistant from two [`Float`]s with the precision of the input, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// See [`Float::sec_with_period_prec_round`] for the error bounds, the special and closed-form
    /// cases, overflow, and the complexity; this function behaves the same way with `prec` equal to
    /// the precision of the input and `rm` equal to `Nearest`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sec_with_period_round_assign`] instead. If you want to specify an output precision,
    /// consider using [`Float::sec_with_period_prec_assign`]. If you want both of these things,
    /// consider using [`Float::sec_with_period_prec_round_assign`].
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 10).0;
    /// x.sec_with_period_assign(7);
    /// assert_eq!(x.to_string(), "1.6035");
    /// ```
    #[inline]
    pub fn sec_with_period_assign(&mut self, u: u64) {
        let prec = self.significant_bits();
        self.sec_with_period_prec_assign(u, prec);
    }

    /// Computes $\sec(2\pi x/u)$, the secant of a [`Rational`] measured in $u$ths of a turn,
    /// rounding the result to the specified precision and with the specified rounding mode, and
    /// returning the result as a [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded secant is less than, equal to, or greater than
    /// the exact secant. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = \sec(2\pi x/u)+\varepsilon.
    /// $$
    /// - If $u=0$ or $x/u$ is an odd multiple of $1/4$, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $u\neq 0$ and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\sec(2\pi
    ///   x/u)|\rfloor-p+1}$.
    /// - If $u\neq 0$ and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\sec(2\pi
    ///   x/u)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=\text{NaN}$
    /// - $f(0,u,p,m)=1$
    /// - If $x/u$ is an even multiple of $1/2$, the result is exactly $1$, and at an odd multiple
    ///   exactly $-1$.
    /// - If $x/u$ is an odd multiple of $1/4$, the secant has a pole there, and the result is
    ///   exactly $\infty$: the cosine is $+0.0$ at every such point, and the secant is its
    ///   reciprocal.
    /// - If $x/u$ is an odd multiple of $1/8$, the result is $\pm\sqrt2$.
    ///
    /// When $x/u$ in lowest terms has denominator 3 or 6, the result is exactly $\pm2$; when it has
    /// denominator 5, 8, 10, or 12, the result is $\pm2\varphi$, $\pm\sqrt2$, $\pm2(\varphi-1)$, or
    /// $\pm2\sqrt3/3$, computed from a single correctly rounded constant rather than from $\pi$ and
    /// a cosine, which is far faster.
    ///
    /// Underflow is not possible, since $|\sec(2\pi x/u)| \geq 1$. Overflow is as for
    /// [`Float::sec_with_period_prec_round`], and requires $x/u$ within $2^{-2^{30}}$ of an odd
    /// multiple of $1/4$ without being one, which takes a denominator of more than $2^{30}$ bits.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::sec_with_period_rational_prec`] instead.
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
    /// let (t, o) = Float::sec_with_period_rational_prec_round(Rational::ONE, 7, 10, Floor);
    /// assert_eq!(t.to_string(), "1.6035");
    /// assert_eq!(o, Less);
    ///
    /// let (t, o) = Float::sec_with_period_rational_prec_round(Rational::ONE, 7, 10, Ceiling);
    /// assert_eq!(t.to_string(), "1.6055");
    /// assert_eq!(o, Greater);
    ///
    /// // a quarter turn is a pole
    /// let (t, o) = Float::sec_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 4),
    ///     1,
    ///     10,
    ///     Exact,
    /// );
    /// assert_eq!(t.to_string(), "Infinity");
    /// assert_eq!(o, Equal);
    ///
    /// // a twelfth of a turn: 2 sqrt(3)/3
    /// let (t, o) = Float::sec_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 12),
    ///     1,
    ///     10,
    ///     Nearest,
    /// );
    /// assert_eq!(t.to_string(), "1.1543");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn sec_with_period_rational_prec_round(
        x: Rational,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::sec_with_period_rational_prec_round_ref(&x, u, prec, rm)
    }

    /// Computes $\sec(2\pi x/u)$, the secant of a [`Rational`] measured in $u$ths of a turn,
    /// rounding the result to the specified precision and with the specified rounding mode, and
    /// returning the result as a [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`]
    /// is also returned, indicating whether the rounded secant is less than, equal to, or greater
    /// than the exact secant. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::sec_with_period_rational_prec_round`] for the error bounds, the special and
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
    /// let (t, o) = Float::sec_with_period_rational_prec_round_ref(&Rational::ONE, 7, 10, Floor);
    /// assert_eq!(t.to_string(), "1.6035");
    /// assert_eq!(o, Less);
    /// ```
    pub fn sec_with_period_rational_prec_round_ref(
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
        // sec(0) = 1
        if *x == 0u32 {
            return (Self::one_prec(prec), Equal);
        }
        // q = x/u, reduced to (-1, 1): sec(2 pi q) has period 1 in q, and a multiple of u gives a
        // cosine of 1, so a secant of 1
        let q = x / Rational::from(u) % Rational::ONE;
        if q == 0u32 {
            return (Self::one_prec(prec), Equal);
        }
        sec_turns_helper(&q, prec, rm)
    }

    /// Computes $\sec(2\pi x/u)$, the secant of a [`Rational`] measured in $u$ths of a turn,
    /// rounding the result to the nearest value of the specified precision, and returning the
    /// result as a [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded secant is less than, equal to, or greater than the exact
    /// secant. Although `NaN`s are not comparable to any [`Float`], whenever this function returns
    /// a `NaN` it also returns `Equal`.
    ///
    /// If the secant is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// See [`Float::sec_with_period_rational_prec_round`] for the error bounds, the special and
    /// closed-form cases, overflow, and the complexity; this function behaves the same way with
    /// `Nearest`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sec_with_period_rational_prec_round`] instead.
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
    /// let (t, o) = Float::sec_with_period_rational_prec(Rational::ONE, 7, 10);
    /// assert_eq!(t.to_string(), "1.6035");
    /// assert_eq!(o, Less);
    ///
    /// // an eighth of a turn: sqrt(2)
    /// let (t, o) = Float::sec_with_period_rational_prec(Rational::ONE, 8, 10);
    /// assert_eq!(t.to_string(), "1.4141");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn sec_with_period_rational_prec(x: Rational, u: u64, prec: u64) -> (Self, Ordering) {
        Self::sec_with_period_rational_prec_round_ref(&x, u, prec, Nearest)
    }

    /// Computes $\sec(2\pi x/u)$, the secant of a [`Rational`] measured in $u$ths of a turn,
    /// rounding the result to the nearest value of the specified precision, and returning the
    /// result as a [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded secant is less than, equal to, or greater than the
    /// exact secant. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::sec_with_period_rational_prec`] and
    /// [`Float::sec_with_period_rational_prec_round`]; this function behaves the same way.
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
    /// let (t, o) = Float::sec_with_period_rational_prec_ref(&Rational::ONE, 7, 10);
    /// assert_eq!(t.to_string(), "1.6035");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sec_with_period_rational_prec_ref(x: &Rational, u: u64, prec: u64) -> (Self, Ordering) {
        Self::sec_with_period_rational_prec_round_ref(x, u, prec, Nearest)
    }

    /// Computes $\sec(\pi x)$, the secant of a [`Float`] measured in half-turns, rounding the
    /// result to the specified precision and with the specified rounding mode. The [`Float`] is
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded secant is
    /// less than, equal to, or greater than the exact secant. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// This is `sec_with_period` with a period of 2: see [`Float::sec_with_period_prec_round`] for
    /// the error bounds, the special and closed-form cases (even integers give $1$ and odd ones
    /// $-1$; half-integers are poles and give $\infty$; odd multiples of $1/4$ give $\pm\sqrt2$;
    /// multiples of $1/3$ give $\pm2$; and odd multiples of $1/6$, and multiples of $1/5$ and
    /// $1/10$, give $\pm2\sqrt3/3$, $\pm2\varphi$, or $\pm2(\varphi-1)$, where $\varphi$ is the
    /// golden ratio), overflow, and the complexity, with $u = 2$.
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
    /// let (t, o) = Float::from(0.1f64).sec_pi_prec_round(10, Floor);
    /// assert_eq!(t.to_string(), "1.0508");
    /// assert_eq!(o, Less);
    ///
    /// let (t, o) = Float::from(0.1f64).sec_pi_prec_round(10, Ceiling);
    /// assert_eq!(t.to_string(), "1.0527");
    /// assert_eq!(o, Greater);
    ///
    /// // a half-turn is exactly zero, reached from below
    /// let (t, o) = Float::ONE.sec_pi_prec_round(10, Exact);
    /// assert_eq!(t.to_string(), "-1.0000");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn sec_pi_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.sec_with_period_prec_round(2, prec, rm)
    }

    /// Computes $\sec(\pi x)$, the secant of a [`Float`] measured in half-turns, rounding the
    /// result to the specified precision and with the specified rounding mode. The [`Float`] is
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded secant
    /// is less than, equal to, or greater than the exact secant. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// This is `sec_with_period` with a period of 2: see [`Float::sec_with_period_prec_round_ref`]
    /// for the error bounds, the special and closed-form cases (even integers give $1$ and odd ones
    /// $-1$; half-integers are poles and give $\infty$; odd multiples of $1/4$ give $\pm\sqrt2$;
    /// multiples of $1/3$ give $\pm2$; and odd multiples of $1/6$, and multiples of $1/5$ and
    /// $1/10$, give $\pm2\sqrt3/3$, $\pm2\varphi$, or $\pm2(\varphi-1)$, where $\varphi$ is the
    /// golden ratio), overflow, and the complexity, with $u = 2$.
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
    /// let (t, o) = (Float::from(0.1f64)).sec_pi_prec_round_ref(10, Floor);
    /// assert_eq!(t.to_string(), "1.0508");
    /// assert_eq!(o, Less);
    ///
    /// let (t, o) = (Float::from(0.1f64)).sec_pi_prec_round_ref(10, Ceiling);
    /// assert_eq!(t.to_string(), "1.0527");
    /// assert_eq!(o, Greater);
    ///
    /// // a half-turn is exactly zero, reached from below
    /// let (t, o) = (&Float::ONE).sec_pi_prec_round_ref(10, Exact);
    /// assert_eq!(t.to_string(), "-1.0000");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn sec_pi_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.sec_with_period_prec_round_ref(2, prec, rm)
    }

    /// Computes $\sec(\pi x)$, the secant of a [`Float`] measured in half-turns, rounding the
    /// result to the nearest value of the specified precision. The [`Float`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded secant is less than, equal to,
    /// or greater than the exact secant. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// This is `sec_with_period` with a period of 2: see [`Float::sec_with_period_prec`] for the
    /// error bounds, the special and closed-form cases (even integers give $1$ and odd ones $-1$;
    /// half-integers are poles and give $\infty$; odd multiples of $1/4$ give $\pm\sqrt2$;
    /// multiples of $1/3$ give $\pm2$; and odd multiples of $1/6$, and multiples of $1/5$ and
    /// $1/10$, give $\pm2\sqrt3/3$, $\pm2\varphi$, or $\pm2(\varphi-1)$, where $\varphi$ is the
    /// golden ratio), overflow, and the complexity, with $u = 2$.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = Float::from(0.1f64).sec_pi_prec(10);
    /// assert_eq!(t.to_string(), "1.0508");
    /// assert_eq!(o, Less);
    ///
    /// let (t, o) = Float::from(0.1f64).sec_pi_prec(53);
    /// assert_eq!(t.to_string(), "1.0514622242382672");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sec_pi_prec(self, prec: u64) -> (Self, Ordering) {
        self.sec_with_period_prec(2, prec)
    }

    /// Computes $\sec(\pi x)$, the secant of a [`Float`] measured in half-turns, rounding the
    /// result to the nearest value of the specified precision. The [`Float`] is taken by reference.
    /// An [`Ordering`] is also returned, indicating whether the rounded secant is less than, equal
    /// to, or greater than the exact secant. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// This is `sec_with_period` with a period of 2: see [`Float::sec_with_period_prec_ref`] for
    /// the error bounds, the special and closed-form cases (even integers give $1$ and odd ones
    /// $-1$; half-integers are poles and give $\infty$; odd multiples of $1/4$ give $\pm\sqrt2$;
    /// multiples of $1/3$ give $\pm2$; and odd multiples of $1/6$, and multiples of $1/5$ and
    /// $1/10$, give $\pm2\sqrt3/3$, $\pm2\varphi$, or $\pm2(\varphi-1)$, where $\varphi$ is the
    /// golden ratio), overflow, and the complexity, with $u = 2$.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = (Float::from(0.1f64)).sec_pi_prec_ref(10);
    /// assert_eq!(t.to_string(), "1.0508");
    /// assert_eq!(o, Less);
    ///
    /// let (t, o) = (Float::from(0.1f64)).sec_pi_prec_ref(53);
    /// assert_eq!(t.to_string(), "1.0514622242382672");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sec_pi_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.sec_with_period_prec_ref(2, prec)
    }

    /// Computes $\sec(\pi x)$, the secant of a [`Float`] measured in half-turns, rounding the
    /// result with the specified rounding mode. The precision of the output is the precision of the
    /// input. The [`Float`] is taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded secant is less than, equal to, or greater than the exact secant. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// This is `sec_with_period` with a period of 2: see [`Float::sec_with_period_round`] for the
    /// error bounds, the special and closed-form cases (even integers give $1$ and odd ones $-1$;
    /// half-integers are poles and give $\infty$; odd multiples of $1/4$ give $\pm\sqrt2$;
    /// multiples of $1/3$ give $\pm2$; and odd multiples of $1/6$, and multiples of $1/5$ and
    /// $1/10$, give $\pm2\sqrt3/3$, $\pm2\varphi$, or $\pm2(\varphi-1)$, where $\varphi$ is the
    /// golden ratio), overflow, and the complexity, with $u = 2$.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = Float::from(0.1f64).sec_pi_round(Floor);
    /// assert_eq!(t.to_string(), "1.0514622242382670");
    /// assert_eq!(o, Less);
    ///
    /// let (t, o) = Float::from(0.1f64).sec_pi_round(Nearest);
    /// assert_eq!(t.to_string(), "1.0514622242382674");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn sec_pi_round(self, rm: RoundingMode) -> (Self, Ordering) {
        self.sec_with_period_round(2, rm)
    }

    /// Computes $\sec(\pi x)$, the secant of a [`Float`] measured in half-turns, rounding the
    /// result with the specified rounding mode. The precision of the output is the precision of the
    /// input. The [`Float`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded secant is less than, equal to, or greater than the exact secant.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// This is `sec_with_period` with a period of 2: see [`Float::sec_with_period_round_ref`] for
    /// the error bounds, the special and closed-form cases (even integers give $1$ and odd ones
    /// $-1$; half-integers are poles and give $\infty$; odd multiples of $1/4$ give $\pm\sqrt2$;
    /// multiples of $1/3$ give $\pm2$; and odd multiples of $1/6$, and multiples of $1/5$ and
    /// $1/10$, give $\pm2\sqrt3/3$, $\pm2\varphi$, or $\pm2(\varphi-1)$, where $\varphi$ is the
    /// golden ratio), overflow, and the complexity, with $u = 2$.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = (Float::from(0.1f64)).sec_pi_round_ref(Floor);
    /// assert_eq!(t.to_string(), "1.0514622242382670");
    /// assert_eq!(o, Less);
    ///
    /// let (t, o) = (Float::from(0.1f64)).sec_pi_round_ref(Nearest);
    /// assert_eq!(t.to_string(), "1.0514622242382674");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn sec_pi_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.sec_with_period_round_ref(2, rm)
    }

    /// Computes $\sec(\pi x)$, the secant of a [`Float`] measured in half-turns, rounding the
    /// result to the precision of the input and to the nearest [`Float`]. The [`Float`] is taken by
    /// value.
    ///
    /// If the secant is equidistant from two [`Float`]s with the precision of the input, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// This is `sec_with_period` with a period of 2: see [`Float::sec_with_period`] for the error
    /// bounds, the special and closed-form cases (even integers give $1$ and odd ones $-1$;
    /// half-integers are poles and give $\infty$; odd multiples of $1/4$ give $\pm\sqrt2$;
    /// multiples of $1/3$ give $\pm2$; and odd multiples of $1/6$, and multiples of $1/5$ and
    /// $1/10$, give $\pm2\sqrt3/3$, $\pm2\varphi$, or $\pm2(\varphi-1)$, where $\varphi$ is the
    /// golden ratio), overflow, and the complexity, with $u = 2$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sec_pi_round`] instead. If you want to specify an output precision, consider using
    /// [`Float::sec_pi_prec`]. If you want both of these things, consider using
    /// [`Float::sec_pi_prec_round`].
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// let t = Float::from(0.1f64).sec_pi();
    /// assert_eq!(t.to_string(), "1.0514622242382674");
    ///
    /// // a half-integer is a pole
    /// assert_eq!(Float::from(0.5f64).sec_pi().to_string(), "Infinity");
    /// ```
    #[inline]
    pub fn sec_pi(self) -> Self {
        let prec = self.significant_bits();
        self.sec_pi_prec(prec).0
    }

    /// Computes $\sec(\pi x)$, the secant of a [`Float`] measured in half-turns, rounding the
    /// result to the precision of the input and to the nearest [`Float`]. The [`Float`] is taken by
    /// reference.
    ///
    /// If the secant is equidistant from two [`Float`]s with the precision of the input, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// This is `sec_with_period` with a period of 2: see [`Float::sec_with_period`] for the error
    /// bounds, the special and closed-form cases (even integers give $1$ and odd ones $-1$;
    /// half-integers are poles and give $\infty$; odd multiples of $1/4$ give $\pm\sqrt2$;
    /// multiples of $1/3$ give $\pm2$; and odd multiples of $1/6$, and multiples of $1/5$ and
    /// $1/10$, give $\pm2\sqrt3/3$, $\pm2\varphi$, or $\pm2(\varphi-1)$, where $\varphi$ is the
    /// golden ratio), overflow, and the complexity, with $u = 2$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sec_pi_round_ref`] instead. If you want to specify an output precision, consider
    /// using [`Float::sec_pi_prec_ref`]. If you want both of these things, consider using
    /// [`Float::sec_pi_prec_round_ref`].
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// let t = (&Float::from(0.1f64)).sec_pi_ref();
    /// assert_eq!(t.to_string(), "1.0514622242382674");
    /// ```
    #[inline]
    pub fn sec_pi_ref(&self) -> Self {
        self.sec_pi_prec_ref(self.significant_bits()).0
    }

    /// Computes $\sec(\pi x)$, the secant of a [`Float`] measured in half-turns, rounding the
    /// result to the specified precision and with the specified rounding mode. The [`Float`] is
    /// replaced by the result, and an [`Ordering`] is returned, indicating whether the rounded
    /// secant is less than, equal to, or greater than the exact secant. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function sets a `NaN` it also returns `Equal`.
    ///
    /// This is `sec_with_period` with a period of 2: see
    /// [`Float::sec_with_period_prec_round_assign`] for the error bounds, the special and
    /// closed-form cases (even integers give $1$ and odd ones $-1$; half-integers are poles and
    /// give $\infty$; odd multiples of $1/4$ give $\pm\sqrt2$; multiples of $1/3$ give $\pm2$; and
    /// odd multiples of $1/6$, and multiples of $1/5$ and $1/10$, give $\pm2\sqrt3/3$,
    /// $\pm2\varphi$, or $\pm2(\varphi-1)$, where $\varphi$ is the golden ratio), overflow, and the
    /// complexity, with $u = 2$.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(0.1f64);
    /// assert_eq!(x.sec_pi_prec_round_assign(10, Floor), Less);
    /// assert_eq!(x.to_string(), "1.0508");
    ///
    /// let mut x = Float::from(0.1f64);
    /// assert_eq!(x.sec_pi_prec_round_assign(10, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.0527");
    /// ```
    #[inline]
    pub fn sec_pi_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        self.sec_with_period_prec_round_assign(2, prec, rm)
    }

    /// Computes $\sec(\pi x)$, the secant of a [`Float`] measured in half-turns, rounding the
    /// result to the nearest value of the specified precision. The [`Float`] is replaced by the
    /// result, and an [`Ordering`] is returned, indicating whether the rounded secant is less than,
    /// equal to, or greater than the exact secant. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function sets a `NaN` it also returns `Equal`.
    ///
    /// This is `sec_with_period` with a period of 2: see [`Float::sec_with_period_prec_assign`] for
    /// the error bounds, the special and closed-form cases (even integers give $1$ and odd ones
    /// $-1$; half-integers are poles and give $\infty$; odd multiples of $1/4$ give $\pm\sqrt2$;
    /// multiples of $1/3$ give $\pm2$; and odd multiples of $1/6$, and multiples of $1/5$ and
    /// $1/10$, give $\pm2\sqrt3/3$, $\pm2\varphi$, or $\pm2(\varphi-1)$, where $\varphi$ is the
    /// golden ratio), overflow, and the complexity, with $u = 2$.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(0.1f64);
    /// assert_eq!(x.sec_pi_prec_assign(10), Less);
    /// assert_eq!(x.to_string(), "1.0508");
    /// ```
    #[inline]
    pub fn sec_pi_prec_assign(&mut self, prec: u64) -> Ordering {
        self.sec_with_period_prec_assign(2, prec)
    }

    /// Computes $\sec(\pi x)$, the secant of a [`Float`] measured in half-turns, rounding the
    /// result with the specified rounding mode. The precision of the output is the precision of the
    /// input. The [`Float`] is replaced by the result, and an [`Ordering`] is returned, indicating
    /// whether the rounded secant is less than, equal to, or greater than the exact secant.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function sets a `NaN` it
    /// also returns `Equal`.
    ///
    /// This is `sec_with_period` with a period of 2: see [`Float::sec_with_period_round_assign`]
    /// for the error bounds, the special and closed-form cases (even integers give $1$ and odd ones
    /// $-1$; half-integers are poles and give $\infty$; odd multiples of $1/4$ give $\pm\sqrt2$;
    /// multiples of $1/3$ give $\pm2$; and odd multiples of $1/6$, and multiples of $1/5$ and
    /// $1/10$, give $\pm2\sqrt3/3$, $\pm2\varphi$, or $\pm2(\varphi-1)$, where $\varphi$ is the
    /// golden ratio), overflow, and the complexity, with $u = 2$.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(0.1f64);
    /// assert_eq!(x.sec_pi_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "1.0514622242382670");
    /// ```
    #[inline]
    pub fn sec_pi_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        self.sec_with_period_round_assign(2, rm)
    }

    /// Computes $\sec(\pi x)$, the secant of a [`Float`] measured in half-turns, rounding the
    /// result to the precision of the input and to the nearest [`Float`]. The [`Float`] is replaced
    /// by the result.
    ///
    /// If the secant is equidistant from two [`Float`]s with the precision of the input, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// This is `sec_with_period` with a period of 2: see [`Float::sec_with_period`] for the error
    /// bounds, the special and closed-form cases (even integers give $1$ and odd ones $-1$;
    /// half-integers are poles and give $\infty$; odd multiples of $1/4$ give $\pm\sqrt2$;
    /// multiples of $1/3$ give $\pm2$; and odd multiples of $1/6$, and multiples of $1/5$ and
    /// $1/10$, give $\pm2\sqrt3/3$, $\pm2\varphi$, or $\pm2(\varphi-1)$, where $\varphi$ is the
    /// golden ratio), overflow, and the complexity, with $u = 2$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sec_pi_round_assign`] instead. If you want to specify an output precision, consider
    /// using [`Float::sec_pi_prec_assign`]. If you want both of these things, consider using
    /// [`Float::sec_pi_prec_round_assign`].
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(0.1f64);
    /// x.sec_pi_assign();
    /// assert_eq!(x.to_string(), "1.0514622242382674");
    /// ```
    #[inline]
    pub fn sec_pi_assign(&mut self) {
        let prec = self.significant_bits();
        self.sec_pi_prec_assign(prec);
    }

    /// Computes $\sec(\pi x)$, the secant of a [`Rational`] measured in half-turns, rounding the
    /// result to the specified precision and with the specified rounding mode and returning the
    /// result as a [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded secant is less than, equal to, or greater than the exact
    /// secant.
    ///
    /// This is `sec_with_period_rational` with a period of 2: see
    /// [`Float::sec_with_period_rational_prec_round`] for the error bounds, the special and
    /// closed-form cases, overflow, and the complexity, with $u = 2$.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = Float::sec_pi_rational_prec_round(Rational::from_unsigneds(1u8, 7), 10, Floor);
    /// assert_eq!(t.to_string(), "1.1094");
    /// assert_eq!(o, Less);
    ///
    /// // a third of a half-turn is exactly 2
    /// let (t, o) = Float::sec_pi_rational_prec_round(Rational::from_unsigneds(1u8, 3), 10, Exact);
    /// assert_eq!(t.to_string(), "2.0000");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn sec_pi_rational_prec_round(
        x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::sec_with_period_rational_prec_round_ref(&x, 2, prec, rm)
    }

    /// Computes $\sec(\pi x)$, the secant of a [`Rational`] measured in half-turns, rounding the
    /// result to the specified precision and with the specified rounding mode and returning the
    /// result as a [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded secant is less than, equal to, or greater than the
    /// exact secant.
    ///
    /// This is `sec_with_period_rational` with a period of 2: see
    /// [`Float::sec_with_period_rational_prec_round_ref`] for the error bounds, the special and
    /// closed-form cases, overflow, and the complexity, with $u = 2$.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) =
    ///     Float::sec_pi_rational_prec_round_ref(&Rational::from_unsigneds(1u8, 7), 10, Ceiling);
    /// assert_eq!(t.to_string(), "1.1113");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn sec_pi_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::sec_with_period_rational_prec_round_ref(x, 2, prec, rm)
    }

    /// Computes $\sec(\pi x)$, the secant of a [`Rational`] measured in half-turns, rounding the
    /// result to the nearest value of the specified precision and returning the result as a
    /// [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded secant is less than, equal to, or greater than the exact secant.
    ///
    /// This is `sec_with_period_rational` with a period of 2: see
    /// [`Float::sec_with_period_rational_prec`] for the error bounds, the special and closed-form
    /// cases, overflow, and the complexity, with $u = 2$.
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
    /// let (t, o) = Float::sec_pi_rational_prec(Rational::from_unsigneds(1u8, 7), 53);
    /// assert_eq!(t.to_string(), "1.1099162641747424");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn sec_pi_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::sec_with_period_rational_prec_ref(&x, 2, prec)
    }

    /// Computes $\sec(\pi x)$, the secant of a [`Rational`] measured in half-turns, rounding the
    /// result to the nearest value of the specified precision and returning the result as a
    /// [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded secant is less than, equal to, or greater than the exact
    /// secant.
    ///
    /// This is `sec_with_period_rational` with a period of 2: see
    /// [`Float::sec_with_period_rational_prec_ref`] for the error bounds, the special and
    /// closed-form cases, overflow, and the complexity, with $u = 2$.
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
    /// let (t, o) = Float::sec_pi_rational_prec_ref(&Rational::from_unsigneds(1u8, 7), 53);
    /// assert_eq!(t.to_string(), "1.1099162641747424");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn sec_pi_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::sec_with_period_rational_prec_ref(x, 2, prec)
    }
}

impl Sec for Float {
    type Output = Self;

    /// Computes $\sec x$, the secant of a [`Float`], taking it by value.
    ///
    /// If the output has a precision, it is the precision of the input. If the secant is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x) = \sec x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\sec x|\rfloor-p}$, where $p$ is
    ///   the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\pm\infty)=\text{NaN}$
    /// - $f(\pm0.0)=1.0$
    ///
    /// See the [`Float::sec_round`] documentation for information on overflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using [`Float::sec_round`]
    /// instead. If you want to specify the output precision, consider using [`Float::sec_prec`]. If
    /// you want both of these things, consider using [`Float::sec_prec_round`].
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
    /// e$ bits. Unlike most functions, `sec` therefore gets slower as the magnitude of its input
    /// grows, not just as the precision does.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Sec;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.sec().is_nan());
    /// assert!(Float::INFINITY.sec().is_nan());
    /// assert!(Float::NEGATIVE_INFINITY.sec().is_nan());
    /// assert_eq!(Float::ZERO.sec().to_string(), "1.0");
    /// assert_eq!(Float::NEGATIVE_ZERO.sec().to_string(), "1.0");
    /// assert_eq!(
    ///     Float::from_unsigned_prec(1u32, 100).0.sec().to_string(),
    ///     "1.8508157176809256179117532413979"
    /// );
    /// assert_eq!(
    ///     Float::from_unsigned_prec(100u32, 100).0.sec().to_string(),
    ///     "1.1596638229046938325514044465873"
    /// );
    /// ```
    #[inline]
    fn sec(self) -> Self {
        let prec = self.significant_bits();
        self.sec_prec_round(prec, Nearest).0
    }
}

impl Sec for &Float {
    type Output = Float;

    /// Computes $\sec x$, the secant of a [`Float`], taking it by reference.
    ///
    /// If the output has a precision, it is the precision of the input. If the secant is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x) = \sec x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\sec x|\rfloor-p}$, where $p$ is
    ///   the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\pm\infty)=\text{NaN}$
    /// - $f(\pm0.0)=1.0$
    ///
    /// See the [`Float::sec_round`] documentation for information on overflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sec_round_ref`] instead. If you want to specify the output precision, consider
    /// using [`Float::sec_prec_ref`]. If you want both of these things, consider using
    /// [`Float::sec_prec_round_ref`].
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
    /// e$ bits. Unlike most functions, `sec` therefore gets slower as the magnitude of its input
    /// grows, not just as the precision does.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Sec;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.sec().is_nan());
    /// assert!(Float::INFINITY.sec().is_nan());
    /// assert!(Float::NEGATIVE_INFINITY.sec().is_nan());
    /// assert_eq!(Float::ZERO.sec().to_string(), "1.0");
    /// assert_eq!(Float::NEGATIVE_ZERO.sec().to_string(), "1.0");
    /// assert_eq!(
    ///     (&Float::from_unsigned_prec(1u32, 100).0).sec().to_string(),
    ///     "1.8508157176809256179117532413979"
    /// );
    /// assert_eq!(
    ///     (&Float::from_unsigned_prec(100u32, 100).0)
    ///         .sec()
    ///         .to_string(),
    ///     "1.1596638229046938325514044465873"
    /// );
    /// ```
    #[inline]
    fn sec(self) -> Float {
        self.sec_prec_round_ref(self.significant_bits(), Nearest).0
    }
}

impl SecAssign for Float {
    /// Computes $\sec x$, the secant of a [`Float`], in place.
    ///
    /// If the output has a precision, it is the precision of the input. If the secant is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x \gets \sec x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\sec x|\rfloor-p}$, where $p$ is
    ///   the precision of the input.
    ///
    /// See the [`Float::sec`] documentation for information on special cases and overflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sec_round_assign`] instead. If you want to specify the output precision, consider
    /// using [`Float::sec_prec_assign`]. If you want both of these things, consider using
    /// [`Float::sec_prec_round_assign`].
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
    /// e$ bits. Unlike most functions, `sec` therefore gets slower as the magnitude of its input
    /// grows, not just as the precision does.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::SecAssign;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.sec_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x.sec_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x.sec_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::ZERO;
    /// x.sec_assign();
    /// assert_eq!(x.to_string(), "1.0");
    ///
    /// let mut x = Float::NEGATIVE_ZERO;
    /// x.sec_assign();
    /// assert_eq!(x.to_string(), "1.0");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// x.sec_assign();
    /// assert_eq!(x.to_string(), "1.8508157176809256179117532413979");
    ///
    /// let mut x = Float::from_unsigned_prec(100u32, 100).0;
    /// x.sec_assign();
    /// assert_eq!(x.to_string(), "1.1596638229046938325514044465873");
    /// ```
    #[inline]
    fn sec_assign(&mut self) {
        let prec = self.significant_bits();
        self.sec_prec_round_assign(prec, Nearest);
    }
}

/// Computes $\sec x$, the secant of a primitive float, correctly rounded. Neither the standard
/// library nor `libm` provides a secant.
///
/// $$
/// f(x) = \sec x+\varepsilon.
/// $$
/// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\sec x|\rfloor-p}$, where $p$ is the
///   precision of the output (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]).
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(\pm\infty)=\text{NaN}$
/// - $f(\pm0.0)=1.0$
///
/// Overflow is not possible: no [`f32`] or [`f64`] is close enough to an odd multiple of $\pi/2$
/// for its secant to exceed the largest finite value (the largest secant of an [`f64`], like the
/// largest tangent, is below $2^{55}$). The result is never subnormal, since $|\sec x| \geq 1$.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::sec::primitive_float_sec;
///
/// assert!(primitive_float_sec(f32::NAN).is_nan());
/// assert!(primitive_float_sec(f32::INFINITY).is_nan());
/// assert!(primitive_float_sec(f32::NEGATIVE_INFINITY).is_nan());
/// assert_eq!(NiceFloat(primitive_float_sec(0.0f32)), NiceFloat(1.0));
/// assert_eq!(NiceFloat(primitive_float_sec(-0.0f32)), NiceFloat(1.0));
/// assert_eq!(NiceFloat(primitive_float_sec(1.0f32)), NiceFloat(1.8508158));
/// assert_eq!(
///     NiceFloat(primitive_float_sec(1.0f64)),
///     NiceFloat(1.8508157176809257)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_sec<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::sec_prec, x)
}

/// Computes $\sec x$, the secant of a [`Rational`], returning the result as a primitive float.
///
/// $$
/// f(x) = \sec x+\varepsilon,
/// $$
/// where $|\varepsilon| < 2^{\lfloor\log_2 |\sec x|\rfloor-p}$, and $p$ is the precision of the
/// output (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]).
///
/// Special cases:
/// - $f(0)=1$
///
/// Overflow is possible: a [`Rational`] within about $2^{-129}$ of an odd multiple of $\pi/2$ has a
/// secant beyond the largest [`f32`], and one within about $2^{-1025}$ of one beyond the largest
/// [`f64`], and the result is then $\pm\infty$. Underflow is not possible, since $|\sec x| \geq 1$.
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
/// use malachite_float::float::arithmetic::sec::primitive_float_sec_rational;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_sec_rational::<f64>(&Rational::ZERO)),
///     NiceFloat(1.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_sec_rational::<f64>(
///         &Rational::from_unsigneds(1u8, 3)
///     )),
///     NiceFloat(1.058249271461442)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_sec_rational::<f32>(
///         &Rational::from_unsigneds(1u8, 3)
///     )),
///     NiceFloat(1.0582492)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_sec_rational::<f64>(&Rational::from(10000))),
///     NiceFloat(-1.050248765417841)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_sec_rational<T: PrimitiveFloat>(x: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_to_float_fn(Float::sec_rational_prec_ref, x)
}

/// Computes $\sec(2\pi x/u)$, the secant of a primitive float measured in $u$ths of a turn (so that
/// `u = 360` is degrees).
///
/// $$
/// f(x,u) = \sec(2\pi x/u)+\varepsilon.
/// $$
/// - If $x$ is not finite, $u=0$, or $x/u$ is an odd multiple of $1/4$, $\varepsilon$ may be
///   ignored or assumed to be 0.
/// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |\sec(2\pi x/u)|\rfloor-p}$, where $p$ is the
///   precision of the output (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]).
///
/// Special cases:
/// - $f(\text{NaN},u)=\text{NaN}$
/// - $f(\pm\infty,u)=\text{NaN}$
/// - $f(x,0)=\text{NaN}$
/// - $f(\pm0.0,u)=1.0$
/// - If $x/u$ is an even multiple of $1/2$, the result is exactly $1$, and at an odd multiple
///   exactly $-1$.
/// - If $x/u$ is an odd multiple of $1/4$, the secant has a pole there, and the result is exactly
///   $\infty$: the cosine is $+0.0$ at every such point, and the secant is its reciprocal.
/// - If $x/u$ is an odd multiple of $1/8$, the result is $\pm\sqrt2$; if it is a multiple of $1/3$
///   or $1/6$ but not of $1/2$, the result is exactly $\pm2$; and if it is an odd multiple of
///   $1/12$, the result is $\pm2\sqrt3/3$.
///
/// Overflow happens only at a pole, where the result is exactly $\infty$: an [`f32`] or [`f64`]
/// whose fraction of a turn is not an odd multiple of $1/4$ is more than $2^{-66}$ of a turn away
/// from one, so its secant stays below $2^{64}$. Underflow is not possible, since $|\sec(2\pi x/u)|
/// \geq 1$.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::sec::primitive_float_sec_with_period;
///
/// assert!(primitive_float_sec_with_period(f32::NAN, 360).is_nan());
/// assert!(primitive_float_sec_with_period(f32::INFINITY, 360).is_nan());
/// assert!(primitive_float_sec_with_period(f32::NEGATIVE_INFINITY, 360).is_nan());
/// assert!(primitive_float_sec_with_period(1.0f32, 0).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_sec_with_period(-0.0f32, 360)),
///     NiceFloat(1.0)
/// );
/// // a quarter turn is a pole
/// assert_eq!(
///     NiceFloat(primitive_float_sec_with_period(90.0f32, 360)),
///     NiceFloat(f32::INFINITY)
/// );
/// // a half turn is exactly -1
/// assert_eq!(
///     NiceFloat(primitive_float_sec_with_period(180.0f32, 360)),
///     NiceFloat(-1.0)
/// );
/// // an eighth of a turn: sqrt(2)
/// assert_eq!(
///     NiceFloat(primitive_float_sec_with_period(45.0f32, 360)),
///     NiceFloat(core::f32::consts::SQRT_2)
/// );
/// // a twelfth of a turn: 2 sqrt(3)/3
/// assert_eq!(
///     NiceFloat(primitive_float_sec_with_period(30.0f64, 360)),
///     NiceFloat(1.1547005383792515)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_sec_with_period(1.0f32, 7)),
///     NiceFloat(1.6038755)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_sec_with_period(1.0f64, 7)),
///     NiceFloat(1.6038754716096766)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_sec_with_period<T: PrimitiveFloat>(x: T, u: u64) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(|x, prec| Float::sec_with_period_prec(x, u, prec), x)
}

/// Computes $\sec(2\pi x/u)$, the secant of a [`Rational`] measured in $u$ths of a turn (so that `u
/// = 360` is degrees), returning the result as a primitive float.
///
/// $$
/// f(x,u) = \sec(2\pi x/u)+\varepsilon.
/// $$
/// - If $u=0$ or $x/u$ is an odd multiple of $1/4$, $\varepsilon$ may be ignored or assumed to be
///   0.
/// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |\sec(2\pi x/u)|\rfloor-p}$, where $p$ is the
///   precision of the output (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]).
///
/// Special cases:
/// - $f(x,0)=\text{NaN}$
/// - $f(0,u)=1$
/// - If $x/u$ is an even multiple of $1/2$, the result is exactly $1$, and at an odd multiple
///   exactly $-1$.
/// - If $x/u$ is an odd multiple of $1/4$, the secant has a pole there, and the result is exactly
///   $\infty$: the cosine is $+0.0$ at every such point, and the secant is its reciprocal.
/// - If $x/u$ is an odd multiple of $1/8$, the result is $\pm\sqrt2$; if it is a multiple of $1/3$
///   or $1/6$ but not of $1/2$, the result is exactly $\pm2$; if it is an odd multiple of $1/12$,
///   the result is $\pm2\sqrt3/3$; and fifths and tenths give $\pm2\varphi$ or $\pm2(\varphi-1)$,
///   where $\varphi$ is the golden ratio.
///
/// Overflow is possible away from a pole too: a fraction of a turn within about $2^{-130}$ of an
/// odd multiple of $1/4$ has a secant beyond the largest [`f32`], and one within about $2^{-1026}$
/// of one beyond the largest [`f64`], and the result is then $\pm\infty$. Underflow is not
/// possible, since $|\sec(2\pi x/u)| \geq 1$.
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
/// use malachite_float::float::arithmetic::sec::primitive_float_sec_with_period_rational;
/// use malachite_q::Rational;
///
/// assert!(primitive_float_sec_with_period_rational::<f64>(&Rational::ZERO, 0).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_sec_with_period_rational::<f64>(
///         &Rational::ZERO,
///         360
///     )),
///     NiceFloat(1.0)
/// );
/// // a quarter turn is a pole
/// assert_eq!(
///     NiceFloat(primitive_float_sec_with_period_rational::<f64>(
///         &Rational::from_unsigneds(1u8, 4),
///         1
///     )),
///     NiceFloat(f64::INFINITY)
/// );
/// // an eighth of a turn: sqrt(2)
/// assert_eq!(
///     NiceFloat(primitive_float_sec_with_period_rational::<f64>(
///         &Rational::from_unsigneds(1u8, 8),
///         1
///     )),
///     NiceFloat(core::f64::consts::SQRT_2)
/// );
/// // a twelfth of a turn: 2 sqrt(3)/3
/// assert_eq!(
///     NiceFloat(primitive_float_sec_with_period_rational::<f64>(
///         &Rational::from_unsigneds(1u8, 12),
///         1
///     )),
///     NiceFloat(1.1547005383792515)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_sec_with_period_rational::<f32>(
///         &Rational::from_unsigneds(1u8, 7),
///         1
///     )),
///     NiceFloat(1.6038755)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_sec_with_period_rational::<f64>(
///         &Rational::from_unsigneds(1u8, 7),
///         1
///     )),
///     NiceFloat(1.6038754716096766)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_sec_with_period_rational<T: PrimitiveFloat>(x: &Rational, u: u64) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_to_float_fn(
        |x, prec| Float::sec_with_period_rational_prec_ref(x, u, prec),
        x,
    )
}

/// Computes $\sec(\pi x)$, the secant of a primitive float measured in half-turns.
///
/// This is `primitive_float_sec_with_period` with a period of 2: see
/// [`primitive_float_sec_with_period`] for the error bound and the special cases, with $u = 2$.
/// Half-integers are poles and give exactly $\infty$; even integers give exactly $1$ and odd ones
/// $-1$; odd multiples of $1/4$ give $\pm\sqrt2$; and multiples of $1/3$ give exactly $\pm2$.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::sec::primitive_float_sec_pi;
///
/// assert!(primitive_float_sec_pi(f32::NAN).is_nan());
/// // a half-integer is a pole
/// assert_eq!(
///     NiceFloat(primitive_float_sec_pi(0.5f32)),
///     NiceFloat(f32::INFINITY)
/// );
/// // an odd integer is exactly -1
/// assert_eq!(NiceFloat(primitive_float_sec_pi(1.0f64)), NiceFloat(-1.0));
/// // an odd multiple of a quarter: sqrt(2)
/// assert_eq!(
///     NiceFloat(primitive_float_sec_pi(0.25f32)),
///     NiceFloat(core::f32::consts::SQRT_2)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_sec_pi(0.1f32)),
///     NiceFloat(1.0514622)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_sec_pi(0.1f64)),
///     NiceFloat(1.0514622242382672)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_sec_pi<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_sec_with_period(x, 2)
}

/// Computes $\sec(\pi x)$, the secant of a [`Rational`] measured in half-turns, returning the
/// result as a primitive float.
///
/// This is `primitive_float_sec_with_period_rational` with a period of 2: see
/// [`primitive_float_sec_with_period_rational`] for the error bound, the special cases, and the
/// complexity, with $u = 2$.
///
/// # Worst-case complexity
/// $T(m) = O(m (\log m)^2 \log\log m)$
///
/// $M(m) = O(m \log m)$
///
/// where $T$ is time, $M$ is additional memory, and $m$ is `x.significant_bits()`.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::OneHalf;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::sec::primitive_float_sec_pi_rational;
/// use malachite_q::Rational;
///
/// // a half of a half-turn is a pole
/// assert_eq!(
///     NiceFloat(primitive_float_sec_pi_rational::<f64>(&Rational::ONE_HALF)),
///     NiceFloat(f64::INFINITY)
/// );
/// // a sixth of a half-turn: 2 sqrt(3)/3
/// assert_eq!(
///     NiceFloat(primitive_float_sec_pi_rational::<f64>(
///         &Rational::from_unsigneds(1u8, 6)
///     )),
///     NiceFloat(1.1547005383792515)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_sec_pi_rational::<f64>(
///         &Rational::from_unsigneds(1u8, 7)
///     )),
///     NiceFloat(1.1099162641747424)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_sec_pi_rational<T: PrimitiveFloat>(x: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_sec_with_period_rational(x, 2)
}
