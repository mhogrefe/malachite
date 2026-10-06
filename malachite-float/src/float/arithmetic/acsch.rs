// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::float::arithmetic::asinh::{
    asinh_abs_general, ln_of_large_sum, round_with_error, square_may_overflow,
};
use crate::{Float, emulate_float_to_float_fn, emulate_rational_to_float_fn};
use core::cmp::Ordering::{self, *};
use core::cmp::max;
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{
    Abs, Acsch, AcschAssign, CeilingLogBase2, IsPowerOf2, Reciprocal,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, NegativeInfinity, NegativeZero, Zero as ZeroTrait,
};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::platform::Limb;
use malachite_q::Rational;

// A positive `Float` x with an exponent of at least this has 2/x <= 2^(MAX_EXPONENT - 1), which
// cannot overflow even when rounded up; for a smaller exponent, 1/x or 2/x might overflow.
pub(crate) const RECIPROCAL_SAFE_EXPONENT: i64 = 3 - Float::MAX_EXPONENT_I64;

// For a function f(x) = g(1/x), where g has a correctly rounded `Float` implementation: if x =
// ±2^k, then 1/x = ±2^-k exactly, and g rounds f(x) = g(±2^-k) correctly. A Ziv loop for f could
// not: for the inverse hyperbolic cosecant and cotangent, f(x) lies just short of or just beyond
// 2^-k in magnitude, an exactly representable value, so the loop's approximations would land on
// 2^-k at every working precision, and the rounding test would never pass. Returns `None` when x is
// not a power of 2. 1/x must not overflow.
pub(crate) fn via_exact_reciprocal<G: Fn(Float, u64, RoundingMode) -> (Float, Ordering)>(
    x: &Float,
    prec: u64,
    rm: RoundingMode,
    g: G,
) -> Option<(Float, Ordering)> {
    x.significand_ref()
        .unwrap()
        .is_power_of_2()
        .then(|| g(x.reciprocal_prec_ref(1).0, prec, rm))
}

// Computes acsch(x) = asinh(1/x) for a finite nonzero `Float` x. MPFR has no acsch, so this is
// Malachite's own algorithm. The work is done on |x|, acsch being odd.
//
// The relative condition number of asinh, y / (sqrt(1 + y^2) asinh(y)), is at most 1, so unlike
// asech = acosh(1/x), acsch can go through the reciprocal: 1/|x|, rounded with a relative error of
// at most 2^-wp, moves asinh(1/|x|) by at most 2^-wp asinh(1/|x|), below 1 ulp. asinh(1/|x|) is
// then approximated by `asinh`'s own internals, with their error bound, rather than by a correctly
// rounded `asinh`, whose own Ziv loop would run inside this one.
//
// When x is a power of 2, 1/x is exact, and the result is taken from `asinh` by
// `via_exact_reciprocal`.
//
// When 1/|x| might overflow (EXP(x) < `RECIPROCAL_SAFE_EXPONENT`), acsch(|x|) = ln(1 + sqrt(1 +
// x^2)) - ln|x| = ln 2 - ln|x| + c with 0 < c < x^2/4, and x^2 is then far below 2^(1-wp) unless
// the working precision exceeds 2^31: c is below 1 ulp of the sum, whose ulp is at least 2^(1-wp),
// and with the three roundings the error stays below 2.5 ulps, under 2^2.
fn acsch_prec_round_normal_ref(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact acsch");
    let negative = *x < 0u32;
    let x_abs = x.abs();
    let exp_x = i64::from(x.get_exponent().unwrap());
    let reciprocal_overflows = exp_x < RECIPROCAL_SAFE_EXPONENT;
    if !reciprocal_overflows
        && let Some(result) = via_exact_reciprocal(x, prec, rm, Float::asinh_prec_round)
    {
        return result;
    }
    let mut working_prec = prec + prec.ceiling_log_base_2() + 10;
    let mut increment = Limb::WIDTH;
    loop {
        let (t, err) = if reciprocal_overflows {
            if exp_x << 1 > 1 - i64::exact_from(working_prec) {
                fail_on_untested_path("acsch_prec_round_normal_ref, x^2 not negligible");
            }
            // ln 2 - ln|x|
            let t = Float::ln_2_prec(working_prec).0 - x_abs.ln_prec_ref(working_prec).0;
            (t, 2)
        } else {
            // y = 1/|x|, with a relative error of at most 2^-wp, which moves asinh(y) by at most 1
            // ulp of the result, since asinh's relative condition number is at most 1
            let y = x_abs.reciprocal_prec_ref(working_prec).0;
            let exp_y = i64::from(y.get_exponent().unwrap());
            let (t, err) = if -(exp_y << 1) >= i64::exact_from(working_prec) {
                // y^2 < 2^-wp, so asinh(y) = y (1 - d) with 0 < d < y^2/6, far below an ulp: the
                // error is below 2 ulps
                (y, 1)
            } else if square_may_overflow(&y) {
                // as in `asinh`, the error is below 2 ulps of t
                (ln_of_large_sum(&y, working_prec, true), 2)
            } else {
                // as in `asinh`: see algorithms.tex
                let t = asinh_abs_general(&y, working_prec);
                let err = max(4 - i64::from(t.get_exponent().unwrap()), 0) + 1;
                (t, err)
            };
            // with y's own error, at most 1 ulp more: 2^err + 1 <= 2^(err + 1)
            (t, err + 1)
        };
        if let Some(result) =
            round_with_error(if negative { -t } else { t }, working_prec, err, prec, rm)
        {
            return result;
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

impl Float {
    /// Computes $\operatorname{acsch} x$, the inverse hyperbolic cosecant of a [`Float`], rounding
    /// the result to the specified precision and with the specified rounding mode. The [`Float`] is
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded inverse
    /// hyperbolic cosecant is less than, equal to, or greater than the exact inverse hyperbolic
    /// cosecant. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{acsch} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acsch} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acsch} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{acsch} x|\rfloor-p+1}$.
    /// - If $\operatorname{acsch} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{acsch} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=0.0$
    /// - $f(-\infty,p,m)=-0.0$
    /// - $f(0.0,p,m)=\infty$
    /// - $f(-0.0,p,m)=-\infty$
    ///
    /// The result never overflows or underflows: for a finite nonzero $x$, $|\operatorname{acsch}
    /// x| < \ln(1 + 2/|x|) < 2^{30}$, and $|\operatorname{acsch} x| > 1/(2|x|)$ for $|x| \geq 1$,
    /// while every finite [`Float`] is below $2^{2^{30}-1}$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::acsch_prec`] instead. If you
    /// know that your target precision is the precision of the input, consider using
    /// [`Float::acsch_round`] instead. If both of these things are true, consider using
    /// [`Float::acsch`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite, nonzero, and less than 1 in absolute value,
    /// since the inverse hyperbolic cosecant of such a [`Float`] is never exactly representable, or
    /// if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (Float::one_prec(100) << 1u32).acsch_prec_round(5, Floor);
    /// assert_eq!(c.to_string(), "0.469");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (Float::one_prec(100) << 1u32).acsch_prec_round(5, Ceiling);
    /// assert_eq!(c.to_string(), "0.484");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (Float::one_prec(100) << 1u32).acsch_prec_round(5, Nearest);
    /// assert_eq!(c.to_string(), "0.484");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (Float::one_prec(100) << 1u32).acsch_prec_round(20, Floor);
    /// assert_eq!(c.to_string(), "0.48121166");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (Float::one_prec(100) << 1u32).acsch_prec_round(20, Ceiling);
    /// assert_eq!(c.to_string(), "0.48121214");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (Float::one_prec(100) << 1u32).acsch_prec_round(20, Nearest);
    /// assert_eq!(c.to_string(), "0.48121166");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acsch_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.acsch_prec_round_ref(prec, rm)
    }

    /// Computes $\operatorname{acsch} x$, the inverse hyperbolic cosecant of a [`Float`], rounding
    /// the result to the specified precision and with the specified rounding mode. The [`Float`] is
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded inverse
    /// hyperbolic cosecant is less than, equal to, or greater than the exact inverse hyperbolic
    /// cosecant. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{acsch} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acsch} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acsch} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{acsch} x|\rfloor-p+1}$.
    /// - If $\operatorname{acsch} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{acsch} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=0.0$
    /// - $f(-\infty,p,m)=-0.0$
    /// - $f(0.0,p,m)=\infty$
    /// - $f(-0.0,p,m)=-\infty$
    ///
    /// The result never overflows or underflows: for a finite nonzero $x$, $|\operatorname{acsch}
    /// x| < \ln(1 + 2/|x|) < 2^{30}$, and $|\operatorname{acsch} x| > 1/(2|x|)$ for $|x| \geq 1$,
    /// while every finite [`Float`] is below $2^{2^{30}-1}$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::acsch_prec_ref`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::acsch_round_ref`] instead. If both of these things are true, consider using
    /// `(&Float).acsch()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite, nonzero, and less than 1 in absolute value,
    /// since the inverse hyperbolic cosecant of such a [`Float`] is never exactly representable, or
    /// if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (Float::one_prec(100) << 1u32).acsch_prec_round_ref(5, Floor);
    /// assert_eq!(c.to_string(), "0.469");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (Float::one_prec(100) << 1u32).acsch_prec_round_ref(5, Ceiling);
    /// assert_eq!(c.to_string(), "0.484");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (Float::one_prec(100) << 1u32).acsch_prec_round_ref(5, Nearest);
    /// assert_eq!(c.to_string(), "0.484");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (Float::one_prec(100) << 1u32).acsch_prec_round_ref(20, Floor);
    /// assert_eq!(c.to_string(), "0.48121166");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (Float::one_prec(100) << 1u32).acsch_prec_round_ref(20, Ceiling);
    /// assert_eq!(c.to_string(), "0.48121214");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (Float::one_prec(100) << 1u32).acsch_prec_round_ref(20, Nearest);
    /// assert_eq!(c.to_string(), "0.48121166");
    /// assert_eq!(o, Less);
    /// ```
    pub fn acsch_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match &self.0 {
            NaN => (Self::NAN, Equal),
            // acsch(±inf) = ±0
            Infinity { sign } => (
                if *sign {
                    Self::ZERO
                } else {
                    Self::NEGATIVE_ZERO
                },
                Equal,
            ),
            // acsch(±0) = ±inf
            Zero { sign } => (
                if *sign {
                    Self::INFINITY
                } else {
                    Self::NEGATIVE_INFINITY
                },
                Equal,
            ),
            Finite { .. } => acsch_prec_round_normal_ref(self, prec, rm),
        }
    }

    /// Computes $\operatorname{acsch} x$, the inverse hyperbolic cosecant of a [`Float`], rounding
    /// the result to the nearest value of the specified precision. The [`Float`] is taken by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded inverse hyperbolic cosecant
    /// is less than, equal to, or greater than the exact inverse hyperbolic cosecant. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// If the inverse hyperbolic cosecant is equidistant from two [`Float`]s with the specified
    /// precision, the [`Float`] with fewer 1s in its binary expansion is chosen. See
    /// [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \operatorname{acsch} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acsch} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acsch} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{acsch} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=0.0$
    /// - $f(-\infty,p)=-0.0$
    /// - $f(0.0,p)=\infty$
    /// - $f(-0.0,p)=-\infty$
    ///
    /// The result never overflows or underflows: for a finite nonzero $x$, $|\operatorname{acsch}
    /// x| < \ln(1 + 2/|x|) < 2^{30}$, and $|\operatorname{acsch} x| > 1/(2|x|)$ for $|x| \geq 1$,
    /// while every finite [`Float`] is below $2^{2^{30}-1}$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acsch_prec_round`] instead. If you know that your target precision is the precision
    /// of the input, consider using [`Float::acsch`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (Float::one_prec(100) << 1u32).acsch_prec(5);
    /// assert_eq!(c.to_string(), "0.484");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (Float::one_prec(100) << 1u32).acsch_prec(20);
    /// assert_eq!(c.to_string(), "0.48121166");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acsch_prec(self, prec: u64) -> (Self, Ordering) {
        self.acsch_prec_round(prec, Nearest)
    }

    /// Computes $\operatorname{acsch} x$, the inverse hyperbolic cosecant of a [`Float`], rounding
    /// the result to the nearest value of the specified precision. The [`Float`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded inverse
    /// hyperbolic cosecant is less than, equal to, or greater than the exact inverse hyperbolic
    /// cosecant. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// If the inverse hyperbolic cosecant is equidistant from two [`Float`]s with the specified
    /// precision, the [`Float`] with fewer 1s in its binary expansion is chosen. See
    /// [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \operatorname{acsch} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acsch} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acsch} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{acsch} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=0.0$
    /// - $f(-\infty,p)=-0.0$
    /// - $f(0.0,p)=\infty$
    /// - $f(-0.0,p)=-\infty$
    ///
    /// The result never overflows or underflows: for a finite nonzero $x$, $|\operatorname{acsch}
    /// x| < \ln(1 + 2/|x|) < 2^{30}$, and $|\operatorname{acsch} x| > 1/(2|x|)$ for $|x| \geq 1$,
    /// while every finite [`Float`] is below $2^{2^{30}-1}$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acsch_prec_round_ref`] instead. If you know that your target precision is the
    /// precision of the input, consider using `(&Float).acsch()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (Float::one_prec(100) << 1u32).acsch_prec_ref(5);
    /// assert_eq!(c.to_string(), "0.484");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (Float::one_prec(100) << 1u32).acsch_prec_ref(20);
    /// assert_eq!(c.to_string(), "0.48121166");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acsch_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.acsch_prec_round_ref(prec, Nearest)
    }

    /// Computes $\operatorname{acsch} x$, the inverse hyperbolic cosecant of a [`Float`], rounding
    /// the result with the specified rounding mode. The [`Float`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded inverse hyperbolic cosecant is
    /// less than, equal to, or greater than the exact inverse hyperbolic cosecant. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \operatorname{acsch} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acsch} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acsch} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{acsch} x|\rfloor-p+1}$, where $p$ is the
    ///   precision of the input.
    /// - If $\operatorname{acsch} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{acsch} x|\rfloor-p}$, where $p$ is the
    ///   precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=0.0$
    /// - $f(-\infty,m)=-0.0$
    /// - $f(0.0,m)=\infty$
    /// - $f(-0.0,m)=-\infty$
    ///
    /// The result never overflows or underflows: for a finite nonzero $x$, $|\operatorname{acsch}
    /// x| < \ln(1 + 2/|x|) < 2^{30}$, and $|\operatorname{acsch} x| > 1/(2|x|)$ for $|x| \geq 1$,
    /// while every finite [`Float`] is below $2^{2^{30}-1}$.
    ///
    /// If you want to specify an output precision, consider using [`Float::acsch_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::acsch`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite, nonzero, and less than 1 in absolute value,
    /// since the inverse hyperbolic cosecant of such a [`Float`] is never exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (Float::one_prec(100) << 1u32).acsch_round(Floor);
    /// assert_eq!(c.to_string(), "0.48121182505960344749775891342426");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (Float::one_prec(100) << 1u32).acsch_round(Ceiling);
    /// assert_eq!(c.to_string(), "0.48121182505960344749775891342465");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (Float::one_prec(100) << 1u32).acsch_round(Nearest);
    /// assert_eq!(c.to_string(), "0.48121182505960344749775891342426");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acsch_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.acsch_prec_round(prec, rm)
    }

    /// Computes $\operatorname{acsch} x$, the inverse hyperbolic cosecant of a [`Float`], rounding
    /// the result with the specified rounding mode. The [`Float`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded inverse hyperbolic cosecant is
    /// less than, equal to, or greater than the exact inverse hyperbolic cosecant. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \operatorname{acsch} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acsch} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acsch} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{acsch} x|\rfloor-p+1}$, where $p$ is the
    ///   precision of the input.
    /// - If $\operatorname{acsch} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{acsch} x|\rfloor-p}$, where $p$ is the
    ///   precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=0.0$
    /// - $f(-\infty,m)=-0.0$
    /// - $f(0.0,m)=\infty$
    /// - $f(-0.0,m)=-\infty$
    ///
    /// The result never overflows or underflows: for a finite nonzero $x$, $|\operatorname{acsch}
    /// x| < \ln(1 + 2/|x|) < 2^{30}$, and $|\operatorname{acsch} x| > 1/(2|x|)$ for $|x| \geq 1$,
    /// while every finite [`Float`] is below $2^{2^{30}-1}$.
    ///
    /// If you want to specify an output precision, consider using [`Float::acsch_prec_round_ref`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// `(&Float).acsch()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite, nonzero, and less than 1 in absolute value,
    /// since the inverse hyperbolic cosecant of such a [`Float`] is never exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (Float::one_prec(100) << 1u32).acsch_round_ref(Floor);
    /// assert_eq!(c.to_string(), "0.48121182505960344749775891342426");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (Float::one_prec(100) << 1u32).acsch_round_ref(Ceiling);
    /// assert_eq!(c.to_string(), "0.48121182505960344749775891342465");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (Float::one_prec(100) << 1u32).acsch_round_ref(Nearest);
    /// assert_eq!(c.to_string(), "0.48121182505960344749775891342426");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acsch_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.acsch_prec_round_ref(self.significant_bits(), rm)
    }

    /// Computes $\operatorname{acsch} x$, the inverse hyperbolic cosecant of a [`Float`], rounding
    /// the result to the specified precision and with the specified rounding mode. The [`Float`] is
    /// replaced by the result, and an [`Ordering`] is returned, indicating whether the rounded
    /// inverse hyperbolic cosecant is less than, equal to, or greater than the exact inverse
    /// hyperbolic cosecant. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function sets a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \operatorname{acsch} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acsch} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acsch} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{acsch} x|\rfloor-p+1}$.
    /// - If $\operatorname{acsch} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{acsch} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::acsch_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::acsch_prec_assign`] instead.
    /// If you know that your target precision is the precision of the input, consider using
    /// [`Float::acsch_round_assign`] instead. If both of these things are true, consider using
    /// [`Float::acsch_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite, nonzero, and less than 1 in absolute value,
    /// since the inverse hyperbolic cosecant of such a [`Float`] is never exactly representable, or
    /// if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::one_prec(100) << 1u32;
    /// assert_eq!(x.acsch_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "0.469");
    ///
    /// let mut x = Float::one_prec(100) << 1u32;
    /// assert_eq!(x.acsch_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.484");
    ///
    /// let mut x = Float::one_prec(100) << 1u32;
    /// assert_eq!(x.acsch_prec_round_assign(5, Nearest), Greater);
    /// assert_eq!(x.to_string(), "0.484");
    ///
    /// let mut x = Float::one_prec(100) << 1u32;
    /// assert_eq!(x.acsch_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "0.48121166");
    ///
    /// let mut x = Float::one_prec(100) << 1u32;
    /// assert_eq!(x.acsch_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.48121214");
    ///
    /// let mut x = Float::one_prec(100) << 1u32;
    /// assert_eq!(x.acsch_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.48121166");
    /// ```
    #[inline]
    pub fn acsch_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let o;
        (*self, o) = self.acsch_prec_round_ref(prec, rm);
        o
    }

    /// Computes $\operatorname{acsch} x$, the inverse hyperbolic cosecant of a [`Float`], rounding
    /// the result to the nearest value of the specified precision. The [`Float`] is replaced by the
    /// result, and an [`Ordering`] is returned, indicating whether the rounded inverse hyperbolic
    /// cosecant is less than, equal to, or greater than the exact inverse hyperbolic cosecant.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function sets a `NaN` it
    /// also returns `Equal`.
    ///
    /// If the inverse hyperbolic cosecant is equidistant from two [`Float`]s with the specified
    /// precision, the [`Float`] with fewer 1s in its binary expansion is chosen. See
    /// [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \operatorname{acsch} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acsch} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acsch} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{acsch} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::acsch_prec`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acsch_prec_round_assign`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::acsch_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::one_prec(100) << 1u32;
    /// assert_eq!(x.acsch_prec_assign(5), Greater);
    /// assert_eq!(x.to_string(), "0.484");
    ///
    /// let mut x = Float::one_prec(100) << 1u32;
    /// assert_eq!(x.acsch_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "0.48121166");
    /// ```
    #[inline]
    pub fn acsch_prec_assign(&mut self, prec: u64) -> Ordering {
        self.acsch_prec_round_assign(prec, Nearest)
    }

    /// Computes $\operatorname{acsch} x$, the inverse hyperbolic cosecant of a [`Float`], rounding
    /// the result with the specified rounding mode. The [`Float`] is replaced by the result, and an
    /// [`Ordering`] is returned, indicating whether the rounded inverse hyperbolic cosecant is less
    /// than, equal to, or greater than the exact inverse hyperbolic cosecant. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function sets a `NaN` it also returns
    /// `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \operatorname{acsch} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acsch} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acsch} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{acsch} x|\rfloor-p+1}$, where $p$ is the
    ///   precision of the input.
    /// - If $\operatorname{acsch} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{acsch} x|\rfloor-p}$, where $p$ is the
    ///   precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::acsch_round`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::acsch_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using [`Float::acsch_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite, nonzero, and less than 1 in absolute value,
    /// since the inverse hyperbolic cosecant of such a [`Float`] is never exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::one_prec(100) << 1u32;
    /// assert_eq!(x.acsch_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "0.48121182505960344749775891342426");
    ///
    /// let mut x = Float::one_prec(100) << 1u32;
    /// assert_eq!(x.acsch_round_assign(Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.48121182505960344749775891342465");
    ///
    /// let mut x = Float::one_prec(100) << 1u32;
    /// assert_eq!(x.acsch_round_assign(Nearest), Less);
    /// assert_eq!(x.to_string(), "0.48121182505960344749775891342426");
    /// ```
    #[inline]
    pub fn acsch_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.acsch_prec_round_assign(prec, rm)
    }
}

impl Float {
    /// Computes $\operatorname{acsch} x$, the inverse hyperbolic cosecant of a [`Rational`],
    /// rounding the result to the specified precision and with the specified rounding mode and
    /// returning the result as a [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded inverse hyperbolic cosecant is less than,
    /// equal to, or greater than the exact inverse hyperbolic cosecant. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{acsch} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acsch} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acsch} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{acsch} x|\rfloor-p+1}$.
    /// - If $\operatorname{acsch} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{acsch} x|\rfloor-p}$.
    ///
    /// These bounds do not apply when the result underflows; see below.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=\infty$
    ///
    /// Overflow and underflow:
    /// - The result never overflows: for $x$ with denominator $d$, $|\operatorname{acsch} x| <
    ///   \ln(1 + 2/|x|) \leq \ln(1 + 2d)$.
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
    /// Underflow requires $|x| > 2^{2^{30}}$, since $|\operatorname{acsch} x| > 1/(2|x|)$ for $|x|
    /// \geq 1$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::acsch_rational_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`: the logarithm is computed at a working precision of about $n$, and
    /// the input is handled with `Rational` arithmetic.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case for every nonzero $x$).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::acsch_rational_prec_round(Rational::from(3u32), 5, Floor);
    /// assert_eq!(c.to_string(), "0.312");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::acsch_rational_prec_round(Rational::from(3u32), 5, Ceiling);
    /// assert_eq!(c.to_string(), "0.328");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::acsch_rational_prec_round(Rational::from(3u32), 20, Floor);
    /// assert_eq!(c.to_string(), "0.32744980");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::acsch_rational_prec_round(Rational::from(3u32), 20, Ceiling);
    /// assert_eq!(c.to_string(), "0.32745028");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn acsch_rational_prec_round(x: Rational, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        Self::acsch_rational_prec_round_ref(&x, prec, rm)
    }

    /// Computes $\operatorname{acsch} x$, the inverse hyperbolic cosecant of a [`Rational`],
    /// rounding the result to the specified precision and with the specified rounding mode and
    /// returning the result as a [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`]
    /// is also returned, indicating whether the rounded inverse hyperbolic cosecant is less than,
    /// equal to, or greater than the exact inverse hyperbolic cosecant. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{acsch} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acsch} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acsch} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{acsch} x|\rfloor-p+1}$.
    /// - If $\operatorname{acsch} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{acsch} x|\rfloor-p}$.
    ///
    /// These bounds do not apply when the result underflows; see below.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=\infty$
    ///
    /// Overflow and underflow:
    /// - The result never overflows: for $x$ with denominator $d$, $|\operatorname{acsch} x| <
    ///   \ln(1 + 2/|x|) \leq \ln(1 + 2d)$.
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
    /// Underflow requires $|x| > 2^{2^{30}}$, since $|\operatorname{acsch} x| > 1/(2|x|)$ for $|x|
    /// \geq 1$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::acsch_rational_prec_ref`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`: the logarithm is computed at a working precision of about $n$, and
    /// the input is handled with `Rational` arithmetic.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case for every nonzero $x$).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::acsch_rational_prec_round_ref(&Rational::from(3u32), 5, Floor);
    /// assert_eq!(c.to_string(), "0.312");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::acsch_rational_prec_round_ref(&Rational::from(3u32), 5, Ceiling);
    /// assert_eq!(c.to_string(), "0.328");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::acsch_rational_prec_round_ref(&Rational::from(3u32), 20, Floor);
    /// assert_eq!(c.to_string(), "0.32744980");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::acsch_rational_prec_round_ref(&Rational::from(3u32), 20, Ceiling);
    /// assert_eq!(c.to_string(), "0.32745028");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn acsch_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        if *x == 0u32 {
            // acsch(0) = inf, the limit from above (a `Rational` zero is unsigned)
            return (Self::INFINITY, Equal);
        }
        // acsch(x) = asinh(1/x), and the reciprocal of a `Rational` is exact
        Self::asinh_rational_prec_round(x.reciprocal(), prec, rm)
    }

    /// Computes $\operatorname{acsch} x$, the inverse hyperbolic cosecant of a [`Rational`],
    /// rounding the result to the nearest value of the specified precision and returning the result
    /// as a [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded inverse hyperbolic cosecant is less than, equal to, or
    /// greater than the exact inverse hyperbolic cosecant. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the inverse hyperbolic cosecant is equidistant from two [`Float`]s with the specified
    /// precision, the [`Float`] with fewer 1s in its binary expansion is chosen. See
    /// [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \operatorname{acsch} x+\varepsilon,
    /// $$
    /// where, if $\operatorname{acsch} x$ is finite and nonzero, $|\varepsilon| \leq
    /// 2^{\lfloor\log_2 |\operatorname{acsch} x|\rfloor-p}$ (unless the result underflows; see
    /// below).
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=\infty$
    ///
    /// Overflow and underflow:
    /// - The result never overflows: for $x$ with denominator $d$, $|\operatorname{acsch} x| <
    ///   \ln(1 + 2/|x|) \leq \ln(1 + 2d)$.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// Underflow requires $|x| > 2^{2^{30}}$, since $|\operatorname{acsch} x| > 1/(2|x|)$ for $|x|
    /// \geq 1$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acsch_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`: the logarithm is computed at a working precision of about $n$, and
    /// the input is handled with `Rational` arithmetic.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::acsch_rational_prec(Rational::from(3u32), 5);
    /// assert_eq!(c.to_string(), "0.328");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::acsch_rational_prec(Rational::from(3u32), 20);
    /// assert_eq!(c.to_string(), "0.32745028");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::acsch_rational_prec(Rational::ZERO, 10);
    /// assert_eq!(c.to_string(), "Infinity");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn acsch_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::acsch_rational_prec_round_ref(&x, prec, Nearest)
    }

    /// Computes $\operatorname{acsch} x$, the inverse hyperbolic cosecant of a [`Rational`],
    /// rounding the result to the nearest value of the specified precision and returning the result
    /// as a [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded inverse hyperbolic cosecant is less than, equal to, or
    /// greater than the exact inverse hyperbolic cosecant. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the inverse hyperbolic cosecant is equidistant from two [`Float`]s with the specified
    /// precision, the [`Float`] with fewer 1s in its binary expansion is chosen. See
    /// [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \operatorname{acsch} x+\varepsilon,
    /// $$
    /// where, if $\operatorname{acsch} x$ is finite and nonzero, $|\varepsilon| \leq
    /// 2^{\lfloor\log_2 |\operatorname{acsch} x|\rfloor-p}$ (unless the result underflows; see
    /// below).
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=\infty$
    ///
    /// Overflow and underflow:
    /// - The result never overflows: for $x$ with denominator $d$, $|\operatorname{acsch} x| <
    ///   \ln(1 + 2/|x|) \leq \ln(1 + 2d)$.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// Underflow requires $|x| > 2^{2^{30}}$, since $|\operatorname{acsch} x| > 1/(2|x|)$ for $|x|
    /// \geq 1$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acsch_rational_prec_round_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`: the logarithm is computed at a working precision of about $n$, and
    /// the input is handled with `Rational` arithmetic.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::acsch_rational_prec_ref(&Rational::from(3u32), 5);
    /// assert_eq!(c.to_string(), "0.328");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::acsch_rational_prec_ref(&Rational::from(3u32), 20);
    /// assert_eq!(c.to_string(), "0.32745028");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::acsch_rational_prec_ref(&Rational::ZERO, 10);
    /// assert_eq!(c.to_string(), "Infinity");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn acsch_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::acsch_rational_prec_round_ref(x, prec, Nearest)
    }
}

impl Acsch for Float {
    type Output = Self;

    /// Computes $\operatorname{acsch} x$, the inverse hyperbolic cosecant of a [`Float`], taking it
    /// by value.
    ///
    /// If the output has a precision, it is the precision of the input. If the inverse hyperbolic
    /// cosecant is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// f(x) = \operatorname{acsch} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acsch} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acsch} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{acsch} x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=0.0$
    /// - $f(-\infty)=-0.0$
    /// - $f(0.0)=\infty$
    /// - $f(-0.0)=-\infty$
    ///
    /// See the [`Float::acsch_round`] documentation for information on overflow and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acsch_round`] instead. If you want to specify the output precision, consider using
    /// [`Float::acsch_prec`]. If you want both of these things, consider using
    /// [`Float::acsch_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Acsch;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.acsch().is_nan());
    /// assert_eq!(Float::INFINITY.acsch().to_string(), "0.0");
    /// assert_eq!(Float::NEGATIVE_INFINITY.acsch().to_string(), "-0.0");
    /// assert_eq!(Float::ZERO.acsch().to_string(), "Infinity");
    /// assert_eq!(Float::NEGATIVE_ZERO.acsch().to_string(), "-Infinity");
    /// assert_eq!(
    ///     (Float::one_prec(100) << 1u32).acsch().to_string(),
    ///     "0.48121182505960344749775891342426"
    /// );
    /// assert_eq!(
    ///     (Float::one_prec(100) >> 1u32).acsch().to_string(),
    ///     "1.4436354751788103424932767402724"
    /// );
    /// assert_eq!(
    ///     (-(Float::one_prec(100) << 1u32)).acsch().to_string(),
    ///     "-0.48121182505960344749775891342426"
    /// );
    /// ```
    #[inline]
    fn acsch(self) -> Self {
        let prec = self.significant_bits();
        self.acsch_prec_round(prec, Nearest).0
    }
}

impl Acsch for &Float {
    type Output = Float;

    /// Computes $\operatorname{acsch} x$, the inverse hyperbolic cosecant of a [`Float`], taking it
    /// by reference.
    ///
    /// If the output has a precision, it is the precision of the input. If the inverse hyperbolic
    /// cosecant is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// f(x) = \operatorname{acsch} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acsch} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acsch} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{acsch} x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=0.0$
    /// - $f(-\infty)=-0.0$
    /// - $f(0.0)=\infty$
    /// - $f(-0.0)=-\infty$
    ///
    /// See the [`Float::acsch_round`] documentation for information on overflow and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acsch_round_ref`] instead. If you want to specify the output precision, consider
    /// using [`Float::acsch_prec_ref`]. If you want both of these things, consider using
    /// [`Float::acsch_prec_round_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Acsch;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::NAN).acsch().is_nan());
    /// assert_eq!((&Float::INFINITY).acsch().to_string(), "0.0");
    /// assert_eq!((&Float::NEGATIVE_INFINITY).acsch().to_string(), "-0.0");
    /// assert_eq!((&Float::ZERO).acsch().to_string(), "Infinity");
    /// assert_eq!((&Float::NEGATIVE_ZERO).acsch().to_string(), "-Infinity");
    /// assert_eq!(
    ///     (&(Float::one_prec(100) << 1u32)).acsch().to_string(),
    ///     "0.48121182505960344749775891342426"
    /// );
    /// assert_eq!(
    ///     (&(Float::one_prec(100) >> 1u32)).acsch().to_string(),
    ///     "1.4436354751788103424932767402724"
    /// );
    /// assert_eq!(
    ///     (&(-(Float::one_prec(100) << 1u32))).acsch().to_string(),
    ///     "-0.48121182505960344749775891342426"
    /// );
    /// ```
    #[inline]
    fn acsch(self) -> Float {
        self.acsch_prec_round_ref(self.significant_bits(), Nearest)
            .0
    }
}

impl AcschAssign for Float {
    /// Computes $\operatorname{acsch} x$, the inverse hyperbolic cosecant of a [`Float`], in place.
    ///
    /// If the output has a precision, it is the precision of the input. If the inverse hyperbolic
    /// cosecant is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \operatorname{acsch} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acsch} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acsch} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{acsch} x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// See the [`Float::acsch`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acsch_round_assign`] instead. If you want to specify the output precision, consider
    /// using [`Float::acsch_prec_assign`]. If you want both of these things, consider using
    /// [`Float::acsch_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::AcschAssign;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.acsch_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x.acsch_assign();
    /// assert_eq!(x.to_string(), "0.0");
    ///
    /// let mut x = Float::ZERO;
    /// x.acsch_assign();
    /// assert_eq!(x.to_string(), "Infinity");
    ///
    /// let mut x = Float::NEGATIVE_ZERO;
    /// x.acsch_assign();
    /// assert_eq!(x.to_string(), "-Infinity");
    ///
    /// let mut x = Float::one_prec(100) << 1u32;
    /// x.acsch_assign();
    /// assert_eq!(x.to_string(), "0.48121182505960344749775891342426");
    ///
    /// let mut x = Float::one_prec(100) >> 1u32;
    /// x.acsch_assign();
    /// assert_eq!(x.to_string(), "1.4436354751788103424932767402724");
    /// ```
    #[inline]
    fn acsch_assign(&mut self) {
        let prec = self.significant_bits();
        self.acsch_prec_round_assign(prec, Nearest);
    }
}

/// Computes $\operatorname{acsch} x$, the inverse hyperbolic cosecant of a primitive float. Using
/// this function is more accurate than using the default `acsch` function or the one provided by
/// `libm`.
///
/// $$
/// f(x) = \operatorname{acsch} x+\varepsilon.
/// $$
/// - If $\operatorname{acsch} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or assumed
///   to be 0.
/// - If $\operatorname{acsch} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
///   |\operatorname{acsch} x|\rfloor-p}$, where $p$ is the precision of the output (24 if `T` is a
///   [`f32`] and 53 if `T` is a [`f64`]).
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(\infty)=0.0$
/// - $f(-\infty)=-0.0$
/// - $f(0.0)=\infty$
/// - $f(-0.0)=-\infty$
///
/// Overflow is not possible. The result is subnormal only when $x$ is, and then it is $x$ itself,
/// since $|\operatorname{acsch} x - x| < |x|^3/2$.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::acsch::primitive_float_acsch;
///
/// assert!(primitive_float_acsch(f32::NAN).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_acsch(f32::INFINITY)),
///     NiceFloat(0.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acsch(f32::NEGATIVE_INFINITY)),
///     NiceFloat(-0.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acsch(0.0f32)),
///     NiceFloat(f32::INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acsch(-0.0f32)),
///     NiceFloat(f32::NEGATIVE_INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acsch(2.0f32)),
///     NiceFloat(0.4812118)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acsch(2.0f64)),
///     NiceFloat(0.48121182505960347)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acsch(-0.5f64)),
///     NiceFloat(-1.4436354751788103)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_acsch<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::acsch_prec, x)
}

/// Computes $\operatorname{acsch} x$, the inverse hyperbolic cosecant of a [`Rational`], returning
/// the result as a primitive float. The result is correctly rounded.
///
/// $$
/// f(x) = \operatorname{acsch} x+\varepsilon.
/// $$
/// - If $\operatorname{acsch} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or assumed
///   to be 0.
/// - If $\operatorname{acsch} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
///   |\operatorname{acsch} x|\rfloor-p}$, where $p$ is the precision of the output (typically 24 if
///   `T` is a [`f32`] and 53 if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(0)=\infty$
///
/// Overflow is not possible. Underflow is: an `x` of large enough magnitude gives `0.0` or `-0.0`.
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
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::acsch::primitive_float_acsch_rational;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_acsch_rational::<f64>(&Rational::ZERO)),
///     NiceFloat(f64::INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acsch_rational::<f64>(&Rational::from(3u32))),
///     NiceFloat(0.32745015023725843)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_acsch_rational<T: PrimitiveFloat>(x: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_to_float_fn(Float::acsch_rational_prec_ref, x)
}
