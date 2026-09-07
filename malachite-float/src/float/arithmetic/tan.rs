// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright © 2001-2025 Free Software Foundation, Inc.
//
//      Contributed by the Pascaline and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

// Port of MPFR's tangent. `mpfr_tan` (`tan.c`) computes the sine and cosine together at the working
// precision, divides, and certifies the quotient with two bits of slack, inside a Ziv loop. MPFR's
// exponent range is wide enough that the quotient never overflows or underflows there; Malachite's
// is not (the tangent of an input within 2^(-2^30) of an odd multiple of pi/2 overflows, and of one
// within that distance of a multiple of pi underflows), so a quotient near either end of the range
// is decided from exact brackets instead.

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::float::arithmetic::cos::{round_bracket, trig_near_zero_bracket};
use crate::float::arithmetic::round_near_x::float_round_near_x;
use crate::{Float, emulate_float_to_float_fn};
use core::cmp::Ordering::{self, Equal, Greater, Less};
use core::cmp::min;
use malachite_base::num::arithmetic::traits::{
    Abs, CeilingLogBase2, IsPowerOf2, PowerOf2, Tan, TanAssign,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, NegativeInfinity, Zero as ZeroTrait,
};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, Ceiling, Down, Exact, Floor, Nearest};
use malachite_nz::natural::arithmetic::float::round::float_can_round;
use malachite_nz::platform::Limb;
use malachite_q::Rational;

// As in mpfr_overflow, with the overflow's sign: the toward-zero modes give the largest finite
// value, and the other modes an infinity.
fn tan_overflow(negative: bool, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    match (negative, rm) {
        (_, Exact) => panic!("Inexact tan"),
        (false, Floor | Down) => (Float::max_finite_value_with_prec(prec), Less),
        (false, _) => (Float::INFINITY, Greater),
        (true, Ceiling | Down) => (-Float::max_finite_value_with_prec(prec), Greater),
        (true, _) => (Float::NEGATIVE_INFINITY, Less),
    }
}

// A bracket for the magnitude of the true value of a sine or cosine v that `sin_cos` rounded to
// nearest at precision m: within half an ulp of v, or, if v underflowed, within the rounding rule's
// bounds: a zero stands for a magnitude of at most 2^(MIN_EXPONENT - 2), half the smallest positive
// `Float`, and the smallest positive `Float` itself may have been reached from as low as half of
// it.
fn nearest_bracket(v: &Float, m: u64) -> (Rational, Rational) {
    if *v == 0u32 {
        return (
            Rational::ZERO,
            Rational::exact_from(&Float::min_positive_value_prec(1)) >> 1u32,
        );
    }
    let exp = i64::from(v.get_exponent().unwrap());
    let abs = Rational::exact_from(v).abs();
    let half_ulp = Rational::power_of_2(exp - i64::exact_from(m) - 1);
    if exp == Float::MIN_EXPONENT_I64 && v.significand_ref().unwrap().is_power_of_2() {
        (Rational::power_of_2(exp - 2), abs + half_ulp)
    } else {
        (&abs - &half_ulp, abs + half_ulp)
    }
}

// Decides tan(x) = s/c from the sine and cosine rounded to nearest at precision m, by a `Rational`
// bracket, for the cases the `Float` quotient cannot settle: it overflowed, underflowed, or lies
// within two bits of either end of the exponent range, or the sine or cosine underflowed. Returns
// `None` if the bracket does not decide the rounding, so that the working precision must grow.
fn tan_bracket(
    x: &Float,
    s: &Float,
    c: &Float,
    m: u64,
    prec: u64,
    rm: RoundingMode,
) -> Option<(Float, Ordering)> {
    let negative = s.is_sign_negative() != c.is_sign_negative();
    // A cosine that underflowed is at most 1.5 times the smallest positive `Float`, and the sine is
    // then within 2^-m of 1, so the tangent is at least 2^(2^30)/1.5 in magnitude, beyond the
    // largest finite `Float`.
    if *c == 0u32
        || (c.get_exponent() == Some(Float::MIN_EXPONENT)
            && c.significand_ref().unwrap().is_power_of_2())
    {
        return Some(tan_overflow(negative, prec, rm));
    }
    let (c_lo, c_hi) = nearest_bracket(c, m);
    let (s_lo, s_hi) = if *s == 0u32 {
        // The sine underflowed, so its rounding says only that it is below half the smallest
        // positive `Float`, and the tangent, barely larger than the sine, cannot be placed against
        // that same bound: take the sine's exact bracket from the distance to the nearest multiple
        // of pi, as the near-zero path does.
        let (lo, hi) =
            trig_near_zero_bracket(x, m + 64, const { Float::MAX_EXPONENT as u64 + 2 }, false);
        if lo < 0u32 { (-hi, -lo) } else { (lo, hi) }
    } else {
        nearest_bracket(s, m)
    };
    let lo = s_lo / c_hi;
    let hi = s_hi / c_lo;
    if negative {
        round_bracket(&-hi, &-lo, prec, rm)
    } else {
        round_bracket(&lo, &hi, prec, rm)
    }
}

// This is mpfr_tan from tan.c, MPFR 4.2.2, with the bracket path for results near the ends of the
// exponent range.
fn tan_prec_round_normal_ref(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact tan");
    let exp_x = i64::from(x.get_exponent().unwrap());
    // tan(x) = x + x^3/3 + ... so the error is < 2^(3*EXP(x)-1)
    //
    // MPFR_FAST_COMPUTE_IF_SMALL_INPUT (y, x, -2 * MPFR_GET_EXP (x), 1, 1, rnd_mode, {});
    let err1 = -(exp_x << 1);
    if err1 > 0 {
        let err = u64::exact_from(err1) + 1;
        // The error bound only has to clear prec + 1; passing an enormous err (a tiny x has one
        // around 2^31) would make float_round_near_x do work proportional to it.
        if err > prec + 1
            && let Some(result) = float_round_near_x(x, min(err, prec + 2), true, prec, rm)
        {
            return result;
        }
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
            Some(s.div_prec_ref_ref(&c, m).0)
        };
        // "The only way to get an overflow is to get ~ Pi/2. But the result will be ~ 2^Prec(y)",
        // MPFR notes; here the exponent range is narrower. A quotient that overflowed, underflowed,
        // or lies within two bits of either end of the exponent range, where rounding it to `prec`
        // could still cross the end, is decided from brackets, as is a sine or cosine that
        // underflowed.
        let exp_q = q.as_ref().and_then(Float::get_exponent).map(i64::from);
        match exp_q {
            Some(e)
                if e > const { Float::MIN_EXPONENT_I64 + 1 }
                    && e < const { Float::MAX_EXPONENT_I64 - 1 } =>
            {
                let q = q.unwrap();
                if float_can_round(q.significand_ref().unwrap(), m - 2, prec, rm) {
                    return Float::from_float_prec_round(q, prec, rm);
                }
            }
            _ => {
                if let Some(result) = tan_bracket(x, &s, &c, m, prec, rm) {
                    return result;
                }
            }
        }
        m += increment;
        increment = m >> 1;
    }
}

impl Float {
    /// Computes $\tan x$, the tangent of a [`Float`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded tangent is less than, equal
    /// to, or greater than the exact tangent. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \tan x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\tan
    ///   x|\rfloor-p+1}$.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\tan
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=\pm0.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $f(x,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// Overflow requires an input within $2^{-2^{30}}$ of an odd multiple of $\pi/2$, and underflow
    /// an input within $2^{-2^{30}}$ of a nonzero multiple of $\pi$, either of which takes more
    /// than $2^{30}$ bits of precision; underflow also occurs for an input of magnitude
    /// $2^{-2^{30}}$, the smallest positive [`Float`], rounded toward zero.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::tan_prec`] instead. If you
    /// know that your target precision is the precision of the input, consider using
    /// [`Float::tan_round`] instead. If both of these things are true, consider using
    /// [`Float::tan`] instead.
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
    /// Unlike most functions, `tan` therefore gets slower as the magnitude of its input grows, not
    /// just as the precision does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the tangent of a finite nonzero [`Float`] is never exactly
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
    ///     .tan_prec_round(5, Floor);
    /// assert_eq!(c.to_string(), "1.50");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .tan_prec_round(5, Ceiling);
    /// assert_eq!(c.to_string(), "1.56");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .tan_prec_round(5, Nearest);
    /// assert_eq!(c.to_string(), "1.56");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .tan_prec_round(20, Floor);
    /// assert_eq!(c.to_string(), "1.5574074");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .tan_prec_round(20, Ceiling);
    /// assert_eq!(c.to_string(), "1.5574093");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .tan_prec_round(20, Nearest);
    /// assert_eq!(c.to_string(), "1.5574074");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn tan_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.tan_prec_round_ref(prec, rm)
    }

    /// Computes $\tan x$, the tangent of a [`Float`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded tangent is less than, equal
    /// to, or greater than the exact tangent. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \tan x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\tan
    ///   x|\rfloor-p+1}$.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\tan
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=\pm0.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $f(x,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// Overflow requires an input within $2^{-2^{30}}$ of an odd multiple of $\pi/2$, and underflow
    /// an input within $2^{-2^{30}}$ of a nonzero multiple of $\pi$, either of which takes more
    /// than $2^{30}$ bits of precision; underflow also occurs for an input of magnitude
    /// $2^{-2^{30}}$, the smallest positive [`Float`], rounded toward zero.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::tan_prec_ref`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::tan_round_ref`] instead. If both of these things are true, consider using
    /// `(&Float).tan()` instead.
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
    /// Unlike most functions, `tan` therefore gets slower as the magnitude of its input grows, not
    /// just as the precision does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the tangent of a finite nonzero [`Float`] is never exactly
    /// representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).tan_prec_round_ref(5, Floor);
    /// assert_eq!(c.to_string(), "1.50");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).tan_prec_round_ref(5, Ceiling);
    /// assert_eq!(c.to_string(), "1.56");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).tan_prec_round_ref(5, Nearest);
    /// assert_eq!(c.to_string(), "1.56");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).tan_prec_round_ref(20, Floor);
    /// assert_eq!(c.to_string(), "1.5574074");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).tan_prec_round_ref(20, Ceiling);
    /// assert_eq!(c.to_string(), "1.5574093");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).tan_prec_round_ref(20, Nearest);
    /// assert_eq!(c.to_string(), "1.5574074");
    /// assert_eq!(o, Less);
    /// ```
    pub fn tan_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match &self.0 {
            NaN | Infinity { .. } => (Self::NAN, Equal),
            // tan(+0) = +0, tan(-0) = -0
            Zero { .. } => (self.clone(), Equal),
            Finite { .. } => tan_prec_round_normal_ref(self, prec, rm),
        }
    }

    /// Computes $\tan x$, the tangent of a [`Float`], rounding the result to the nearest value of
    /// the specified precision. The [`Float`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded tangent is less than, equal to, or greater than the exact
    /// tangent. Although `NaN`s are not comparable to any [`Float`], whenever this function returns
    /// a `NaN` it also returns `Equal`.
    ///
    /// If the tangent is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \tan x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\tan x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\text{NaN}$
    /// - $f(\pm0.0,p)=1.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// Overflow requires an input within $2^{-2^{30}}$ of an odd multiple of $\pi/2$, and underflow
    /// an input within $2^{-2^{30}}$ of a nonzero multiple of $\pi$, either of which takes more
    /// than $2^{30}$ bits of precision; underflow also occurs for an input of magnitude
    /// $2^{-2^{30}}$, the smallest positive [`Float`], rounded toward zero.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::tan_prec_round`] instead. If you know that your target precision is the precision
    /// of the input, consider using [`Float::tan`] instead.
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
    /// Unlike most functions, `tan` therefore gets slower as the magnitude of its input grows, not
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
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.tan_prec(5);
    /// assert_eq!(c.to_string(), "1.56");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.tan_prec(20);
    /// assert_eq!(c.to_string(), "1.5574074");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn tan_prec(self, prec: u64) -> (Self, Ordering) {
        self.tan_prec_round(prec, Nearest)
    }

    /// Computes $\tan x$, the tangent of a [`Float`], rounding the result to the nearest value of
    /// the specified precision. The [`Float`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded tangent is less than, equal to, or greater than the
    /// exact tangent. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// If the tangent is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \tan x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\tan x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\text{NaN}$
    /// - $f(\pm0.0,p)=1.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// Overflow requires an input within $2^{-2^{30}}$ of an odd multiple of $\pi/2$, and underflow
    /// an input within $2^{-2^{30}}$ of a nonzero multiple of $\pi$, either of which takes more
    /// than $2^{30}$ bits of precision; underflow also occurs for an input of magnitude
    /// $2^{-2^{30}}$, the smallest positive [`Float`], rounded toward zero.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::tan_prec_round_ref`] instead. If you know that your target precision is the
    /// precision of the input, consider using `(&Float).tan()` instead.
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
    /// Unlike most functions, `tan` therefore gets slower as the magnitude of its input grows, not
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
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).tan_prec_ref(5);
    /// assert_eq!(c.to_string(), "1.56");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).tan_prec_ref(20);
    /// assert_eq!(c.to_string(), "1.5574074");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn tan_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.tan_prec_round_ref(prec, Nearest)
    }

    /// Computes $\tan x$, the tangent of a [`Float`], rounding the result with the specified
    /// rounding mode. The [`Float`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded tangent is less than, equal to, or greater than the exact tangent.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \tan x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\tan
    ///   x|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\tan
    ///   x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\pm\infty,m)=\text{NaN}$
    /// - $f(\pm0.0,m)=1.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $f(x,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead.
    /// - If $0<f(x,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// Overflow requires an input within $2^{-2^{30}}$ of an odd multiple of $\pi/2$, and underflow
    /// an input within $2^{-2^{30}}$ of a nonzero multiple of $\pi$, either of which takes more
    /// than $2^{30}$ bits of precision; underflow also occurs for an input of magnitude
    /// $2^{-2^{30}}$, the smallest positive [`Float`], rounded toward zero.
    ///
    /// If you want to specify an output precision, consider using [`Float::tan_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::tan`] instead.
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
    /// e$ bits. Unlike most functions, `tan` therefore gets slower as the magnitude of its input
    /// grows, not just as the precision does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the tangent of a finite nonzero [`Float`] is never exactly
    /// representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.tan_round(Floor);
    /// assert_eq!(c.to_string(), "1.5574077246549022305069748074575");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.tan_round(Ceiling);
    /// assert_eq!(c.to_string(), "1.5574077246549022305069748074591");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.tan_round(Nearest);
    /// assert_eq!(c.to_string(), "1.5574077246549022305069748074591");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn tan_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.tan_prec_round(prec, rm)
    }

    /// Computes $\tan x$, the tangent of a [`Float`], rounding the result with the specified
    /// rounding mode. The [`Float`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded tangent is less than, equal to, or greater than the exact
    /// tangent. Although `NaN`s are not comparable to any [`Float`], whenever this function returns
    /// a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \tan x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\tan
    ///   x|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\tan
    ///   x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\pm\infty,m)=\text{NaN}$
    /// - $f(\pm0.0,m)=1.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $f(x,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead.
    /// - If $0<f(x,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// Overflow requires an input within $2^{-2^{30}}$ of an odd multiple of $\pi/2$, and underflow
    /// an input within $2^{-2^{30}}$ of a nonzero multiple of $\pi$, either of which takes more
    /// than $2^{30}$ bits of precision; underflow also occurs for an input of magnitude
    /// $2^{-2^{30}}$, the smallest positive [`Float`], rounded toward zero.
    ///
    /// If you want to specify an output precision, consider using [`Float::tan_prec_round_ref`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// `(&Float).tan()` instead.
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
    /// e$ bits. Unlike most functions, `tan` therefore gets slower as the magnitude of its input
    /// grows, not just as the precision does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the tangent of a finite nonzero [`Float`] is never exactly
    /// representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).tan_round_ref(Floor);
    /// assert_eq!(c.to_string(), "1.5574077246549022305069748074575");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).tan_round_ref(Ceiling);
    /// assert_eq!(c.to_string(), "1.5574077246549022305069748074591");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).tan_round_ref(Nearest);
    /// assert_eq!(c.to_string(), "1.5574077246549022305069748074591");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn tan_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.tan_prec_round_ref(self.significant_bits(), rm)
    }

    /// Computes $\tan x$, the tangent of a [`Float`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is replaced by the result, and
    /// an [`Ordering`] is returned, indicating whether the rounded tangent is less than, equal to,
    /// or greater than the exact tangent. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function sets a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \tan x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\tan
    ///   x|\rfloor-p+1}$.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\tan
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::tan_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::tan_prec_assign`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::tan_round_assign`] instead. If both of these things are true, consider using
    /// [`Float::tan_assign`] instead.
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
    /// Unlike most functions, `tan` therefore gets slower as the magnitude of its input grows, not
    /// just as the precision does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the tangent of a finite nonzero [`Float`] is never exactly
    /// representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.tan_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "1.50");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.tan_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.56");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.tan_prec_round_assign(5, Nearest), Greater);
    /// assert_eq!(x.to_string(), "1.56");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.tan_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "1.5574074");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.tan_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.5574093");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.tan_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "1.5574074");
    /// ```
    #[inline]
    pub fn tan_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let o;
        (*self, o) = self.tan_prec_round_ref(prec, rm);
        o
    }

    /// Computes $\tan x$, the tangent of a [`Float`], rounding the result to the nearest value of
    /// the specified precision. The [`Float`] is replaced by the result, and an [`Ordering`] is
    /// returned, indicating whether the rounded tangent is less than, equal to, or greater than the
    /// exact tangent. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// sets a `NaN` it also returns `Equal`.
    ///
    /// If the tangent is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \tan x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\tan x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::tan_prec`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::tan_prec_round_assign`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::tan_assign`] instead.
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
    /// Unlike most functions, `tan` therefore gets slower as the magnitude of its input grows, not
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
    /// assert_eq!(x.tan_prec_assign(5), Greater);
    /// assert_eq!(x.to_string(), "1.56");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.tan_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "1.5574074");
    /// ```
    #[inline]
    pub fn tan_prec_assign(&mut self, prec: u64) -> Ordering {
        self.tan_prec_round_assign(prec, Nearest)
    }

    /// Computes $\tan x$, the tangent of a [`Float`], rounding the result with the specified
    /// rounding mode. The [`Float`] is replaced by the result, and an [`Ordering`] is returned,
    /// indicating whether the rounded tangent is less than, equal to, or greater than the exact
    /// tangent. Although `NaN`s are not comparable to any [`Float`], whenever this function sets a
    /// `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \tan x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\tan
    ///   x|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\tan
    ///   x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::tan_round`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::tan_prec_round_assign`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::tan_assign`] instead.
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
    /// e$ bits. Unlike most functions, `tan` therefore gets slower as the magnitude of its input
    /// grows, not just as the precision does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the tangent of a finite nonzero [`Float`] is never exactly
    /// representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.tan_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "1.5574077246549022305069748074575");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.tan_round_assign(Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.5574077246549022305069748074591");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.tan_round_assign(Nearest), Greater);
    /// assert_eq!(x.to_string(), "1.5574077246549022305069748074591");
    /// ```
    #[inline]
    pub fn tan_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.tan_prec_round_assign(prec, rm)
    }
}

impl Tan for Float {
    type Output = Self;

    /// Computes $\tan x$, the tangent of a [`Float`], taking it by value.
    ///
    /// If the output has a precision, it is the precision of the input. If the tangent is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x) = \tan x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\tan x|\rfloor-p}$, where $p$ is
    ///   the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\pm\infty)=\text{NaN}$
    /// - $f(\pm0.0)=\pm0.0$
    ///
    /// See the [`Float::tan_round`] documentation for information on overflow and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using [`Float::tan_round`]
    /// instead. If you want to specify the output precision, consider using [`Float::tan_prec`]. If
    /// you want both of these things, consider using [`Float::tan_prec_round`].
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
    /// e$ bits. Unlike most functions, `tan` therefore gets slower as the magnitude of its input
    /// grows, not just as the precision does.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Tan;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.tan().is_nan());
    /// assert!(Float::INFINITY.tan().is_nan());
    /// assert!(Float::NEGATIVE_INFINITY.tan().is_nan());
    /// assert_eq!(Float::ZERO.tan().to_string(), "0.0");
    /// assert_eq!(Float::NEGATIVE_ZERO.tan().to_string(), "-0.0");
    /// assert_eq!(
    ///     Float::from_unsigned_prec(1u32, 100).0.tan().to_string(),
    ///     "1.5574077246549022305069748074591"
    /// );
    /// assert_eq!(
    ///     Float::from_unsigned_prec(100u32, 100).0.tan().to_string(),
    ///     "-0.58721391515692907667780963564448"
    /// );
    /// ```
    #[inline]
    fn tan(self) -> Self {
        let prec = self.significant_bits();
        self.tan_prec_round(prec, Nearest).0
    }
}

impl Tan for &Float {
    type Output = Float;

    /// Computes $\tan x$, the tangent of a [`Float`], taking it by reference.
    ///
    /// If the output has a precision, it is the precision of the input. If the tangent is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x) = \tan x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\tan x|\rfloor-p}$, where $p$ is
    ///   the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\pm\infty)=\text{NaN}$
    /// - $f(\pm0.0)=\pm0.0$
    ///
    /// See the [`Float::tan_round`] documentation for information on overflow and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::tan_round_ref`] instead. If you want to specify the output precision, consider
    /// using [`Float::tan_prec_ref`]. If you want both of these things, consider using
    /// [`Float::tan_prec_round_ref`].
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
    /// e$ bits. Unlike most functions, `tan` therefore gets slower as the magnitude of its input
    /// grows, not just as the precision does.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Tan;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.tan().is_nan());
    /// assert!(Float::INFINITY.tan().is_nan());
    /// assert!(Float::NEGATIVE_INFINITY.tan().is_nan());
    /// assert_eq!(Float::ZERO.tan().to_string(), "0.0");
    /// assert_eq!(Float::NEGATIVE_ZERO.tan().to_string(), "-0.0");
    /// assert_eq!(
    ///     (&Float::from_unsigned_prec(1u32, 100).0).tan().to_string(),
    ///     "1.5574077246549022305069748074591"
    /// );
    /// assert_eq!(
    ///     (&Float::from_unsigned_prec(100u32, 100).0)
    ///         .tan()
    ///         .to_string(),
    ///     "-0.58721391515692907667780963564448"
    /// );
    /// ```
    #[inline]
    fn tan(self) -> Float {
        self.tan_prec_round_ref(self.significant_bits(), Nearest).0
    }
}

impl TanAssign for Float {
    /// Computes $\tan x$, the tangent of a [`Float`], in place.
    ///
    /// If the output has a precision, it is the precision of the input. If the tangent is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x \gets \tan x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\tan x|\rfloor-p}$, where $p$ is
    ///   the precision of the input.
    ///
    /// See the [`Float::tan`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::tan_round_assign`] instead. If you want to specify the output precision, consider
    /// using [`Float::tan_prec_assign`]. If you want both of these things, consider using
    /// [`Float::tan_prec_round_assign`].
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
    /// e$ bits. Unlike most functions, `tan` therefore gets slower as the magnitude of its input
    /// grows, not just as the precision does.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::TanAssign;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.tan_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x.tan_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x.tan_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::ZERO;
    /// x.tan_assign();
    /// assert_eq!(x.to_string(), "0.0");
    ///
    /// let mut x = Float::NEGATIVE_ZERO;
    /// x.tan_assign();
    /// assert_eq!(x.to_string(), "-0.0");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// x.tan_assign();
    /// assert_eq!(x.to_string(), "1.5574077246549022305069748074591");
    ///
    /// let mut x = Float::from_unsigned_prec(100u32, 100).0;
    /// x.tan_assign();
    /// assert_eq!(x.to_string(), "-0.58721391515692907667780963564448");
    /// ```
    #[inline]
    fn tan_assign(&mut self) {
        let prec = self.significant_bits();
        self.tan_prec_round_assign(prec, Nearest);
    }
}

/// Computes $\tan x$, the tangent of a primitive float. Using this function is more accurate than
/// using the default `tan` function or the one provided by `libm`.
///
/// $$
/// f(x) = \tan x+\varepsilon.
/// $$
/// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\tan x|\rfloor-p}$, where $p$ is the
///   precision of the output (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]).
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(\pm\infty)=\text{NaN}$
/// - $f(\pm0.0)=\pm0.0$
///
/// Overflow is not possible: no [`f32`] or [`f64`] is close enough to an odd multiple of $\pi/2$
/// for its tangent to exceed the largest finite value (the largest tangent of an [`f64`] is below
/// $2^{54}$). The result is subnormal only when $x$ is, and then it is $x$ itself: no [`f32`] or
/// [`f64`] is close enough to a nonzero multiple of $\pi$ for its tangent to be subnormal.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::tan::primitive_float_tan;
///
/// assert!(primitive_float_tan(f32::NAN).is_nan());
/// assert!(primitive_float_tan(f32::INFINITY).is_nan());
/// assert!(primitive_float_tan(f32::NEGATIVE_INFINITY).is_nan());
/// assert_eq!(NiceFloat(primitive_float_tan(0.0f32)), NiceFloat(0.0));
/// assert_eq!(NiceFloat(primitive_float_tan(-0.0f32)), NiceFloat(-0.0));
/// assert_eq!(NiceFloat(primitive_float_tan(1.0f32)), NiceFloat(1.5574077));
/// assert_eq!(
///     NiceFloat(primitive_float_tan(1.0f64)),
///     NiceFloat(1.5574077246549023)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_tan<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::tan_prec, x)
}
