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
use crate::float::arithmetic::cos::{
    reduce_huge, round_bracket, sin_bound, trig_near_zero_bracket, trig_rational_near_zero_bracket,
};
use crate::float::arithmetic::round_near_x::float_round_near_x;
use crate::float::arithmetic::sin_cos::sin_cos_rational_helper;
use crate::{Float, emulate_float_to_float_fn, emulate_rational_to_float_fn};
use core::cmp::Ordering::{self, Equal, Greater, Less};
use core::cmp::min;
use malachite_base::num::arithmetic::traits::{
    Abs, AddMul, CeilingLogBase2, IsPowerOf2, Parity, Pow, PowerOf2, Square, Tan, TanAssign,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, NegativeInfinity, One, Zero as ZeroTrait,
};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, Ceiling, Down, Exact, Floor, Nearest};
use malachite_nz::natural::arithmetic::float::round::float_can_round;
use malachite_nz::platform::Limb;
use malachite_q::Rational;

// A quotient whose exponent lies strictly between these can be rounded to any precision without
// leaving the exponent range, so the `Float` division settles it; the rest go to the brackets.
const MIN_SETTLED_EXPONENT: i64 = Float::MIN_EXPONENT_I64 + 1;
const MAX_SETTLED_EXPONENT: i64 = Float::MAX_EXPONENT_I64 - 1;

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
            Some(e) if e > MIN_SETTLED_EXPONENT && e < MAX_SETTLED_EXPONENT => {
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

// Computes tan(x) for a nonzero `Rational` x, rounded to precision `prec` with rounding mode `rm`.
// (x = 0 is handled by the caller.) The result is never exactly representable, so `rm` must not be
// `Exact`.
//
// This is the `Float` algorithm with the sine and cosine taken from `sin_cos_rational_helper`,
// which rounds the input once and shares the argument reduction, and with a direct bracket for a
// tiny input, where tan x is x + x^3/3 + O(x^5): that also covers inputs below the `Float` exponent
// range, which no other path could even round.
pub(crate) fn tan_rational_helper(x: &Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact tan");
    let exp_x = x.floor_log_base_2_abs() + 1; // the MPFR-style exponent of x
    // For |x| <= 1/2, |x| + |x|^3/3 <= |tan x| <= |x| + |x|^3/3 + |x|^5 (the remaining terms of the
    // series sum to less than |x|^5 there), a bracket of relative width below x^4, which decides
    // the rounding once x^4 is below 2^-(prec + 3), unless the tangent lies within that of a
    // rounding boundary.
    if exp_x < 0 && -(exp_x << 2) > i64::exact_from(prec) + 3 {
        let ax = x.abs();
        let ax3 = (&ax).pow(3u64);
        let lo = &ax + &ax3 / const { Rational::const_from_unsigned(3) };
        let hi = (&lo).add_mul(&ax3, &(&ax).square());
        if let Some(result) = round_bracket_signed(x, lo, hi, prec, rm) {
            return result;
        }
        // The bracket straddles a rounding boundary. Below the exponent range, where the general
        // path could not even round x, tighten it from the series of the sine and cosine, which
        // narrows without bound; otherwise the general path takes over.
        if exp_x <= const { Float::MIN_EXPONENT_I64 + 2 } {
            return tan_rational_tiny(x, &ax, prec, rm);
        }
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
            Some(s.div_prec_ref_ref(&c, m).0)
        };
        let exp_q = q.as_ref().and_then(Float::get_exponent).map(i64::from);
        match exp_q {
            Some(e) if e > MIN_SETTLED_EXPONENT && e < MAX_SETTLED_EXPONENT => {
                let q = q.unwrap();
                if float_can_round(q.significand_ref().unwrap(), m - 2, prec, rm) {
                    return Float::from_float_prec_round(q, prec, rm);
                }
            }
            _ => {
                if let Some(result) = tan_rational_bracket(x, exp_x, &s, &c, m, prec, rm) {
                    return result;
                }
            }
        }
        m += increment;
        increment = m >> 1;
    }
}

// `round_bracket` for a bracket [lo, hi] of the magnitude of the tangent, restoring the sign of x.
fn round_bracket_signed(
    x: &Rational,
    lo: Rational,
    hi: Rational,
    prec: u64,
    rm: RoundingMode,
) -> Option<(Float, Ordering)> {
    if *x < 0u32 {
        round_bracket(&-hi, &-lo, prec, rm)
    } else {
        round_bracket(&lo, &hi, prec, rm)
    }
}

// tan x for a tiny x (|x| <= 1/2, in fact far below the `Float` exponent range) whose two-term
// bracket straddles a rounding boundary: the sine is bracketed by `sin_bound` at a growing working
// precision, and the cosine by consecutive partial sums of its alternating series, until the
// quotient's bracket rounds unambiguously (the tangent is transcendental, so it eventually does).
fn tan_rational_tiny(
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
        let lo = s_lo / &c_hi;
        let hi = s_hi / c_lo;
        if let Some(result) = round_bracket_signed(x, lo, hi, prec, rm) {
            return result;
        }
        w <<= 1;
        terms += 1;
    }
}

// `tan_bracket` for a `Rational` input: the sine's exact bracket, when it underflowed, comes from
// the `Rational` near-zero machinery, on the input reduced modulo 2 pi if it is too large to be a
// `Float`.
fn tan_rational_bracket(
    x: &Rational,
    exp_x: i64,
    s: &Float,
    c: &Float,
    m: u64,
    prec: u64,
    rm: RoundingMode,
) -> Option<(Float, Ordering)> {
    let negative = s.is_sign_negative() != c.is_sign_negative();
    if *c == 0u32
        || (c.get_exponent() == Some(Float::MIN_EXPONENT)
            && c.significand_ref().unwrap().is_power_of_2())
    {
        return Some(tan_overflow(negative, prec, rm));
    }
    let (c_lo, c_hi) = nearest_bracket(c, m);
    let (s_lo, s_hi) = if *s == 0u32 {
        let w = m + 64;
        let reduced;
        let (y, extra) = if exp_x >= Float::MAX_EXPONENT_I64 {
            reduced = reduce_huge(x, exp_x, w);
            (&reduced, Some(2 - i64::exact_from(w)))
        } else {
            (x, None)
        };
        let exp_y = y.floor_log_base_2_abs() + 1;
        let (lo, hi) = trig_rational_near_zero_bracket(y, exp_y, extra, w, m, false);
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

impl Float {
    /// Computes $\tan x$, the tangent of a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded tangent is less than, equal to, or greater than the exact tangent.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \tan x+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\tan x|\rfloor-p+1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\tan x|\rfloor-p}$.
    ///
    /// These bounds do not apply when the result underflows; see below.
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=0$.
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
    /// an input of magnitude about $2^{-2^{30}}$ or less, or one within $2^{-2^{30}}$ of a nonzero
    /// multiple of $\pi$; either near-multiple case takes more than $2^{30}$ bits.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::tan_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is `x.significant_bits()`,
    /// and $e$ is `x.floor_log_base_2_abs()` (taken as 0 when it is negative or $x = 0$): the input
    /// is rounded to a working precision and the [`Float`] sine and cosine taken there together,
    /// and their quotient, which for $|x| \geq 2$ reduces the argument modulo $2\pi$ and so needs
    /// $\pi$ to about $n + e$ bits.
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
    /// let (c, o) = Float::tan_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(c.to_string(), "0.656");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::tan_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Ceiling);
    /// assert_eq!(c.to_string(), "0.688");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::tan_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Floor);
    /// assert_eq!(c.to_string(), "0.68413639");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::tan_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Ceiling);
    /// assert_eq!(c.to_string(), "0.68413734");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn tan_rational_prec_round(x: Rational, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        Self::tan_rational_prec_round_ref(&x, prec, rm)
    }

    /// Computes $\tan x$, the tangent of a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded tangent is less than, equal to, or greater than the exact tangent.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \tan x+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\tan x|\rfloor-p+1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\tan x|\rfloor-p}$.
    ///
    /// These bounds do not apply when the result underflows.
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=0$.
    ///
    /// See the [`Float::tan_rational_prec_round`] documentation for information on overflow and
    /// underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::tan_rational_prec_ref`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is `x.significant_bits()`,
    /// and $e$ is `x.floor_log_base_2_abs()` (taken as 0 when it is negative or $x = 0$): the input
    /// is rounded to a working precision and the [`Float`] sine and cosine taken there together,
    /// and their quotient, which for $|x| \geq 2$ reduces the argument modulo $2\pi$ and so needs
    /// $\pi$ to about $n + e$ bits.
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
    ///     Float::tan_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(c.to_string(), "0.656");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) =
    ///     Float::tan_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 5, Ceiling);
    /// assert_eq!(c.to_string(), "0.688");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) =
    ///     Float::tan_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 20, Floor);
    /// assert_eq!(c.to_string(), "0.68413639");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) =
    ///     Float::tan_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 20, Ceiling);
    /// assert_eq!(c.to_string(), "0.68413734");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn tan_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        if *x == 0u32 {
            // tan(0) = 0, exactly
            return (Self::ZERO, Equal);
        }
        tan_rational_helper(x, prec, rm)
    }

    /// Computes $\tan x$, the tangent of a [`Rational`], rounding the result to the nearest value
    /// of the specified precision and returning the result as a [`Float`]. The [`Rational`] is
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded tangent is
    /// less than, equal to, or greater than the exact tangent.
    ///
    /// If the tangent is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \tan x+\varepsilon,
    /// $$
    /// where $|\varepsilon| \leq 2^{\lfloor\log_2 |\tan x|\rfloor-p}$ (unless the result
    /// underflows; see below).
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=0$.
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
    /// an input of magnitude about $2^{-2^{30}}$ or less, or one within $2^{-2^{30}}$ of a nonzero
    /// multiple of $\pi$; either near-multiple case takes more than $2^{30}$ bits.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::tan_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is `x.significant_bits()`,
    /// and $e$ is `x.floor_log_base_2_abs()` (taken as 0 when it is negative or $x = 0$): the input
    /// is rounded to a working precision and the [`Float`] sine and cosine taken there together,
    /// and their quotient, which for $|x| \geq 2$ reduces the argument modulo $2\pi$ and so needs
    /// $\pi$ to about $n + e$ bits.
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
    /// let (c, o) = Float::tan_rational_prec(Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(c.to_string(), "0.688");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::tan_rational_prec(Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(c.to_string(), "0.68413639");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn tan_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::tan_rational_prec_round_ref(&x, prec, Nearest)
    }

    /// Computes $\tan x$, the tangent of a [`Rational`], rounding the result to the nearest value
    /// of the specified precision and returning the result as a [`Float`]. The [`Rational`] is
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded tangent
    /// is less than, equal to, or greater than the exact tangent.
    ///
    /// If the tangent is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \tan x+\varepsilon,
    /// $$
    /// where $|\varepsilon| \leq 2^{\lfloor\log_2 |\tan x|\rfloor-p}$ (unless the result
    /// underflows).
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=0$.
    ///
    /// See the [`Float::tan_rational_prec`] documentation for information on overflow and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::tan_rational_prec_round_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is `x.significant_bits()`,
    /// and $e$ is `x.floor_log_base_2_abs()` (taken as 0 when it is negative or $x = 0$): the input
    /// is rounded to a working precision and the [`Float`] sine and cosine taken there together,
    /// and their quotient, which for $|x| \geq 2$ reduces the argument modulo $2\pi$ and so needs
    /// $\pi$ to about $n + e$ bits.
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
    /// let (c, o) = Float::tan_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(c.to_string(), "0.688");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::tan_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(c.to_string(), "0.68413639");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn tan_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::tan_rational_prec_round_ref(x, prec, Nearest)
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

/// Computes $\tan x$, the tangent of a [`Rational`], returning the result as a primitive float.
///
/// $$
/// f(x) = \tan x+\varepsilon,
/// $$
/// where $|\varepsilon| < 2^{\lfloor\log_2 |\tan x|\rfloor-p}$, and $p$ is the precision of the
/// output (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]).
///
/// Special cases:
/// - $f(0)=0$
///
/// Overflow is possible: a [`Rational`] within about $2^{-128}$ of an odd multiple of $\pi/2$ has a
/// tangent beyond the largest [`f32`], and one within about $2^{-1024}$ of it beyond the largest
/// [`f64`], and the result is then $\pm\infty$. The result underflows, to a subnormal or to zero,
/// when $x$ is tiny, since $\tan x$ is then very close to $x$; a [`Rational`] close enough to a
/// nonzero multiple of $\pi$ for its tangent to be subnormal would need a denominator of more than
/// 100 bits, in which case the result is still correctly rounded.
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
/// use malachite_float::float::arithmetic::tan::primitive_float_tan_rational;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_tan_rational::<f64>(&Rational::ZERO)),
///     NiceFloat(0.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_tan_rational::<f64>(
///         &Rational::from_unsigneds(1u8, 3)
///     )),
///     NiceFloat(0.34625354951057546)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_tan_rational::<f32>(
///         &Rational::from_unsigneds(1u8, 3)
///     )),
///     NiceFloat(0.34625354)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_tan_rational::<f64>(&Rational::from(10000))),
///     NiceFloat(0.3209711346238147)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_tan_rational<T: PrimitiveFloat>(x: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_to_float_fn(Float::tan_rational_prec_ref, x)
}
