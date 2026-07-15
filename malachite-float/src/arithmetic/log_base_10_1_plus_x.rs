// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Infinity, NaN, Zero};
use crate::arithmetic::log_base_1_plus_x::log_base_1_plus_x_rational;
use crate::{Float, emulate_float_to_float_fn, float_infinity, float_nan, float_negative_infinity};
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{
    CeilingLogBase2, LogBase10Of1PlusX, LogBase10Of1PlusXAssign,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::arithmetic::float_extras::float_can_round;
use malachite_nz::platform::Limb;

// The computation of log_base_10_1_plus_x(x) is done by log_10(1 + x) = log_2(1 + x) / log_2(10).
// The input is finite and greater than -1.
//
// This specializes `log_base_1_plus_x` to base 10. Like that function (and unlike the plain
// `log_base_10`), it routes through `log_base_2_1_plus_x` rather than computing `log_10(1 + x)`
// from `1 + x` directly, preserving accuracy when x is near 0 where `1 + x` would lose precision.
// Since 10 = 2 * 5 is not a perfect power, `log_10(1 + x)` is rational only when `1 + x = 10^m` (m
// a nonnegative integer, so x = 0 or x = 10^m - 1); those exact results are detected up front (the
// Ziv loop could never certify an exactly-representable one). `log_2(10)` is irrational, so every
// other result is strictly between `Float`s and the loop converges.
fn log_base_10_1_plus_x_prec_round_normal(
    x: &Float,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    // log_10(1 + x) is undefined for x < -1.
    match x.partial_cmp(&-1i32).unwrap() {
        // 1 + x = 0, so log_10(1 + x) = -infinity.
        Equal => return (float_negative_infinity!(), Equal),
        Less => return (float_nan!(), Equal),
        _ => {}
    }
    // If 1 + x = 10^m, then log_10(1 + x) = m is rational and exact. `log_base_1_plus_x_rational`
    // with base 10 returns `Some(m / 1)`.
    if let Some(q) = log_base_1_plus_x_rational(x, 10) {
        return Float::from_rational_prec_round(q, prec, rm);
    }
    // The result is irrational, so it is never exactly representable.
    assert_ne!(rm, Exact, "Inexact log_base_10_1_plus_x");
    const TEN: Float = Float::const_from_unsigned(10);
    let min_exp = i64::from(Float::MIN_EXPONENT);
    let mut working_prec = prec + 4 + prec.ceiling_log_base_2();
    let mut increment = Limb::WIDTH;
    loop {
        // log_2(1 + x), correctly rounded to working_prec; always within the Float exponent range.
        let num = x.log_base_2_1_plus_x_prec_ref(working_prec).0;
        // log_2(10) > 1, correctly rounded to working_prec.
        let den = TEN.log_base_2_prec(working_prec).0;
        // Dividing by log_2(10) > 1 only shrinks the magnitude (overflow is impossible), but can
        // push the result below MIN_EXPONENT. When it underflows, the Ziv test below could never
        // resolve it (the quotient clamps), so hand the rounding to div_prec_round, which clamps to
        // zero or the minimum positive value per the rounding mode. The exact quotient exponent is
        // only resolved in the narrow band where the cheap exponent bound is inconclusive (then
        // e_num - e_den == min_exp - 1, so the result underflows iff |log_2(1 + x)| * 2^(1 -
        // min_exp) < log_2(10)). The left shift only adjusts the exponent, avoiding a huge Rational
        // conversion.
        let e_num = i64::from(num.get_exponent().unwrap());
        let e_den = i64::from(den.get_exponent().unwrap());
        if e_num - e_den + 1 < min_exp
            || (e_num - e_den < min_exp && (&num << u64::exact_from(1 - min_exp)).lt_abs(&den))
        {
            return num.div_prec_round(den, prec, rm);
        }
        // log_2(1 + x) / log_2(10), with three correctly-rounded operations (log_base_2_1_plus_x,
        // log_base_2, and the division, each at most 1/2 ulp), so the relative error is below 2^(2
        // - working_prec) and working_prec - 4 correct bits suffice for rounding.
        let t = num / den;
        if float_can_round(t.significand_ref().unwrap(), working_prec - 4, prec, rm) {
            return Float::from_float_prec_round(t, prec, rm);
        }
        // Increase the precision.
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

impl Float {
    /// Computes $\log_{10}(1+x)$, where $x$ is a [`Float`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded value is less than, equal to,
    /// or greater than the exact value. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\log_{10}(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, `NaN` is returned.
    ///
    /// This computes $\log_2(1+x) / \log_2 10$, preserving accuracy for $x$ near 0.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \log_{10}(1+x)+\varepsilon.
    /// $$
    /// - If $\log_{10}(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\log_{10}(1+x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_{10}(1+x)|\rfloor-p+1}$.
    /// - If $\log_{10}(1+x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_{10}(1+x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=\pm0.0$
    /// - $f(-1.0,p,m)=-\infty$
    /// - $f(x,p,m)=\text{NaN}$ for $x<-1$
    /// - $f(x,p,m)=m$ when $1+x=10^m$, rounded to precision $p$; the result is exact if and only if
    ///   $m$ is representable with precision $p$ (for example $\log_{10}(1+9)=1$ when $x=9$ is
    ///   exact)
    ///
    /// This function cannot overflow, but it can underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::log_base_10_1_plus_x_prec`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::log_base_10_1_plus_x_round`] instead. If both of these things are true,
    /// consider using `(&Float).log_base_10_1_plus_x()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
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
    /// let (log, o) = Float::from(9).log_base_10_1_plus_x_prec_round(10, Exact);
    /// assert_eq!(log.to_string(), "1.0"); // log_10(10) = 1
    /// assert_eq!(o, Equal);
    ///
    /// let (log, o) = Float::from(1).log_base_10_1_plus_x_prec_round(20, Nearest);
    /// assert_eq!(log.to_string(), "0.3010302"); // log_10(2)
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_10_1_plus_x_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match self {
            Self(NaN | Infinity { sign: false }) => (float_nan!(), Equal),
            float_infinity!() => (float_infinity!(), Equal),
            Self(Zero { .. }) => (self, Equal),
            _ => log_base_10_1_plus_x_prec_round_normal(&self, prec, rm),
        }
    }

    /// Computes $\log_{10}(1+x)$, where $x$ is a [`Float`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded value is less than, equal to,
    /// or greater than the exact value. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::log_base_10_1_plus_x_prec_round`] for details, special cases, and a description
    /// of the rounding behavior.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
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
    /// let (log, o) = (&Float::from(99)).log_base_10_1_plus_x_prec_round_ref(10, Exact);
    /// assert_eq!(log.to_string(), "2.0"); // log_10(100) = 2
    /// assert_eq!(o, Equal);
    ///
    /// let (log, o) = (&Float::from(1)).log_base_10_1_plus_x_prec_round_ref(20, Floor);
    /// assert_eq!(log.to_string(), "0.3010297"); // log_10(2), rounded down
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn log_base_10_1_plus_x_prec_round_ref(
        &self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match self {
            Self(NaN | Infinity { sign: false }) => (float_nan!(), Equal),
            float_infinity!() => (float_infinity!(), Equal),
            Self(Zero { .. }) => (self.clone(), Equal),
            _ => log_base_10_1_plus_x_prec_round_normal(self, prec, rm),
        }
    }

    /// Computes $\log_{10}(1+x)$, where $x$ is a [`Float`], rounding the result to the nearest
    /// value of the specified precision. The [`Float`] is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value.
    ///
    /// See [`Float::log_base_10_1_plus_x_prec_round`] for details and special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::from(9).log_base_10_1_plus_x_prec(10);
    /// assert_eq!(log.to_string(), "1.0"); // log_10(10) = 1
    /// assert_eq!(o, Equal);
    ///
    /// let (log, o) = Float::from(1).log_base_10_1_plus_x_prec(20);
    /// assert_eq!(log.to_string(), "0.3010302"); // log_10(2)
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_10_1_plus_x_prec(self, prec: u64) -> (Self, Ordering) {
        self.log_base_10_1_plus_x_prec_round(prec, Nearest)
    }

    /// Computes $\log_{10}(1+x)$, where $x$ is a [`Float`], rounding the result to the nearest
    /// value of the specified precision. The [`Float`] is taken by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded value is less than, equal to, or greater than
    /// the exact value.
    ///
    /// See [`Float::log_base_10_1_plus_x_prec_round`] for details and special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = (&Float::from(99)).log_base_10_1_plus_x_prec_ref(10);
    /// assert_eq!(log.to_string(), "2.0"); // log_10(100) = 2
    /// assert_eq!(o, Equal);
    ///
    /// let (log, o) = (&Float::from(7)).log_base_10_1_plus_x_prec_ref(30);
    /// assert_eq!(log.to_string(), "0.903089987"); // log_10(8)
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_10_1_plus_x_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.log_base_10_1_plus_x_prec_round_ref(prec, Nearest)
    }

    /// Computes $\log_{10}(1+x)$, where $x$ is a [`Float`], rounding the result to the precision of
    /// the input and with the specified rounding mode. The [`Float`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded value is less than, equal to,
    /// or greater than the exact value.
    ///
    /// See [`Float::log_base_10_1_plus_x_prec_round`] for details and special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the precision of the input.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input's
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::from(9).log_base_10_1_plus_x_round(Exact);
    /// assert_eq!(log.to_string(), "1.0"); // log_10(10) = 1
    /// assert_eq!(o, Equal);
    ///
    /// let (log, o) = Float::from(99).log_base_10_1_plus_x_round(Exact);
    /// assert_eq!(log.to_string(), "2.0"); // log_10(100) = 2
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_10_1_plus_x_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.log_base_10_1_plus_x_prec_round(prec, rm)
    }

    /// Computes $\log_{10}(1+x)$, where $x$ is a [`Float`], rounding the result to the precision of
    /// the input and with the specified rounding mode. The [`Float`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded value is less than, equal to,
    /// or greater than the exact value.
    ///
    /// See [`Float::log_base_10_1_plus_x_prec_round`] for details and special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the precision of the input.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input's
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = (&Float::from(99)).log_base_10_1_plus_x_round_ref(Exact);
    /// assert_eq!(log.to_string(), "2.0"); // log_10(100) = 2
    /// assert_eq!(o, Equal);
    ///
    /// let (log, o) = (&Float::from(9)).log_base_10_1_plus_x_round_ref(Exact);
    /// assert_eq!(log.to_string(), "1.0"); // log_10(10) = 1
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_10_1_plus_x_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.log_base_10_1_plus_x_prec_round_ref(self.significant_bits(), rm)
    }

    /// Computes $\log_{10}(1+x)$, where $x$ is a [`Float`], in place, rounding the result to the
    /// specified precision and with the specified rounding mode. An [`Ordering`] is returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value.
    ///
    /// See [`Float::log_base_10_1_plus_x_prec_round`] for details and special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
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
    /// let mut x = Float::from(9);
    /// assert_eq!(x.log_base_10_1_plus_x_prec_round_assign(10, Exact), Equal);
    /// assert_eq!(x.to_string(), "1.0"); // log_10(10) = 1
    ///
    /// let mut x = Float::from(1);
    /// assert_eq!(x.log_base_10_1_plus_x_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "0.3010297"); // log_10(2), rounded down
    /// ```
    #[inline]
    pub fn log_base_10_1_plus_x_prec_round_assign(
        &mut self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (result, o) = core::mem::take(self).log_base_10_1_plus_x_prec_round(prec, rm);
        *self = result;
        o
    }

    /// Computes $\log_{10}(1+x)$, where $x$ is a [`Float`], in place, rounding the result to the
    /// nearest value of the specified precision. An [`Ordering`] is returned, indicating whether
    /// the rounded value is less than, equal to, or greater than the exact value.
    ///
    /// See [`Float::log_base_10_1_plus_x_prec_round`] for details and special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(9);
    /// x.log_base_10_1_plus_x_prec_assign(10);
    /// assert_eq!(x.to_string(), "1.0"); // log_10(10) = 1
    ///
    /// let mut x = Float::from(99);
    /// x.log_base_10_1_plus_x_prec_assign(10);
    /// assert_eq!(x.to_string(), "2.0"); // log_10(100) = 2
    /// ```
    #[inline]
    pub fn log_base_10_1_plus_x_prec_assign(&mut self, prec: u64) -> Ordering {
        self.log_base_10_1_plus_x_prec_round_assign(prec, Nearest)
    }

    /// Computes $\log_{10}(1+x)$, where $x$ is a [`Float`], in place, rounding the result to the
    /// precision of the input and with the specified rounding mode. An [`Ordering`] is returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value.
    ///
    /// See [`Float::log_base_10_1_plus_x_prec_round`] for details and special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the precision of the input.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input's
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(9);
    /// x.log_base_10_1_plus_x_round_assign(Exact);
    /// assert_eq!(x.to_string(), "1.0"); // log_10(10) = 1
    ///
    /// let mut x = Float::from(99);
    /// x.log_base_10_1_plus_x_round_assign(Exact);
    /// assert_eq!(x.to_string(), "2.0"); // log_10(100) = 2
    /// ```
    #[inline]
    pub fn log_base_10_1_plus_x_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.log_base_10_1_plus_x_prec_round_assign(prec, rm)
    }
}

impl LogBase10Of1PlusX for Float {
    type Output = Self;

    /// Computes $\log_{10}(1+x)$, where $x$ is a [`Float`], rounding the result to the nearest
    /// value of the input's precision. The [`Float`] is taken by value.
    ///
    /// $\log_{10}(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, `NaN` is returned. See
    /// [`Float::log_base_10_1_plus_x_prec_round`] for the other special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the precision of the input.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::LogBase10Of1PlusX;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::from(9).log_base_10_1_plus_x().to_string(), "1.0"); // log_10(10) = 1
    /// assert_eq!(Float::from(99).log_base_10_1_plus_x().to_string(), "2.0"); // log_10(100) = 2
    /// ```
    #[inline]
    fn log_base_10_1_plus_x(self) -> Self {
        let prec = self.significant_bits();
        self.log_base_10_1_plus_x_prec_round(prec, Nearest).0
    }
}

impl LogBase10Of1PlusX for &Float {
    type Output = Float;

    /// Computes $\log_{10}(1+x)$, where $x$ is a [`Float`], rounding the result to the nearest
    /// value of the input's precision. The [`Float`] is taken by reference.
    ///
    /// $\log_{10}(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, `NaN` is returned. See
    /// [`Float::log_base_10_1_plus_x_prec_round`] for the other special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the precision of the input.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::LogBase10Of1PlusX;
    /// use malachite_float::Float;
    ///
    /// assert_eq!((&Float::from(9)).log_base_10_1_plus_x().to_string(), "1.0"); // log_10(10) = 1
    /// assert_eq!((&Float::from(99)).log_base_10_1_plus_x().to_string(), "2.0"); // log_10(100) = 2
    /// ```
    #[inline]
    fn log_base_10_1_plus_x(self) -> Float {
        self.log_base_10_1_plus_x_prec_round_ref(self.significant_bits(), Nearest)
            .0
    }
}

impl LogBase10Of1PlusXAssign for Float {
    /// Replaces a [`Float`] $x$ with $\log_{10}(1+x)$, rounding the result to the nearest value of
    /// the input's precision.
    ///
    /// $\log_{10}(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, `NaN` is returned. See
    /// [`Float::log_base_10_1_plus_x_prec_round`] for the other special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the precision of the input.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::LogBase10Of1PlusXAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(9);
    /// x.log_base_10_1_plus_x_assign();
    /// assert_eq!(x.to_string(), "1.0"); // log_10(10) = 1
    ///
    /// let mut x = Float::from(99);
    /// x.log_base_10_1_plus_x_assign();
    /// assert_eq!(x.to_string(), "2.0"); // log_10(100) = 2
    /// ```
    #[inline]
    fn log_base_10_1_plus_x_assign(&mut self) {
        let prec = self.significant_bits();
        self.log_base_10_1_plus_x_prec_round_assign(prec, Nearest);
    }
}

/// Computes $\log_{10}(1+x)$, the base-10 logarithm of one plus a primitive float. Using this
/// function is more accurate than computing `(1 + x).log10()`, both because $1+x$ may not be
/// representable as a primitive float and because the standard library's `log10` is not always
/// correctly rounded.
///
/// $\log_{10}(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, `NaN` is returned.
///
/// $$
/// f(x) = \log_{10}(1+x)+\varepsilon.
/// $$
/// - If $\log_{10}(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
///   0.
/// - If $\log_{10}(1+x)$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
///   |\log_{10}(1+x)|\rfloor-p}$, where $p$ is precision of the output (typically 24 if `T` is a
///   [`f32`] and 53 if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(\infty)=\infty$
/// - $f(-\infty)=\text{NaN}$
/// - $f(\pm0.0)=\pm0.0$
/// - $f(-1.0)=-\infty$
/// - $f(x)=\text{NaN}$ for $x<-1$
///
/// This function can underflow (to a subnormal or zero) when $x$ is close to zero, but it cannot
/// overflow.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::arithmetic::log_base_10_1_plus_x::primitive_float_log_base_10_1_plus_x;
///
/// assert!(primitive_float_log_base_10_1_plus_x(f32::NAN).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_10_1_plus_x(f32::INFINITY)),
///     NiceFloat(f32::INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_10_1_plus_x(-1.0f32)),
///     NiceFloat(f32::NEGATIVE_INFINITY)
/// );
/// assert!(primitive_float_log_base_10_1_plus_x(-2.0f32).is_nan());
/// // log_10(1 + 999) = log_10(1000) = 3
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_10_1_plus_x(999.0f32)),
///     NiceFloat(3.0)
/// );
/// // log_10(1 + 9) = log_10(10) = 1
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_10_1_plus_x(9.0f32)),
///     NiceFloat(1.0)
/// );
/// // log_10(1 + 1) = log_10(2)
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_10_1_plus_x(1.0f32)),
///     NiceFloat(std::f32::consts::LOG10_2)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_log_base_10_1_plus_x<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::log_base_10_1_plus_x_prec, x)
}
