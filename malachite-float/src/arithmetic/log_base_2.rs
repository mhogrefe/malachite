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
use crate::{Float, float_either_zero, float_infinity, float_nan, float_negative_infinity};
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{
    CeilingLogBase2, IsPowerOf2, LogBase2, LogBase2Assign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero as ZeroTrait;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::arithmetic::float_extras::float_can_round;
use malachite_nz::platform::Limb;

// The computation of log_base_2(x) is done by log_base_2(x) = ln(x) / ln(2).
//
// This is mpfr_log2 from log2.c, MPFR 4.3.0, where the input is finite, nonzero, and positive.
fn log_base_2_prec_round_normal(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    // If x is 1, the result is 0.
    if *x == 1u32 {
        return (Float::ZERO, Equal);
    }
    // If x is 2^k, log_base_2(x) is exact (though possibly subject to rounding at the target
    // precision).
    if x.is_power_of_2() {
        return Float::from_signed_prec_round(i64::from(x.get_exponent().unwrap()) - 1, prec, rm);
    }
    // The result is never exactly representable for other inputs.
    assert_ne!(rm, Exact, "Inexact log_base_2");
    // Compute the precision of the intermediary variable: the optimal number of bits, see
    // algorithms.tex.
    let mut working_prec = prec + 3 + prec.ceiling_log_base_2();
    let mut increment = Limb::WIDTH;
    loop {
        // ln(x) / ln(2)
        let t = x
            .ln_prec_ref(working_prec)
            .0
            .div_prec(Float::ln_2_prec(working_prec).0, working_prec)
            .0;
        // Estimation of the error.
        if float_can_round(t.significand_ref().unwrap(), working_prec - 3, prec, rm) {
            return Float::from_float_prec_round(t, prec, rm);
        }
        // Increase the precision.
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

impl Float {
    /// Computes $\log_2 x$, where $x$ is a [`Float`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded value is less than, equal to,
    /// or greater than the exact value. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The base-2 logarithm of any nonzero negative number is `NaN`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \log_2 x+\varepsilon.
    /// $$
    /// - If $\log_2 x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\log_2 x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_2 x|\rfloor-p+1}$.
    /// - If $\log_2 x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_2 x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=-\infty$
    /// - $f(1.0,p,m)=0.0$, and the result is exact
    /// - $f(2^k,p,m)=k$, rounded to precision $p$; the result is exact if and only if $k$ is
    ///   representable with precision $p$
    /// - $f(x,p,m)=\text{NaN}$ for $x<0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::log_base_2_prec`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::log_base_2_round`] instead. If both of these things are true, consider using
    /// [`Float::log_base_2`] instead.
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
    /// with the given precision. (The result is exactly representable if and only if the input is
    /// `NaN`, infinite, zero, equal to 1, or a power of 2 whose base-2 logarithm is representable
    /// with the given precision.)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_prec_round(5, Floor);
    /// assert_eq!(log.to_string(), "3.2");
    /// assert_eq!(o, Less);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_prec_round(5, Ceiling);
    /// assert_eq!(log.to_string(), "3.4");
    /// assert_eq!(o, Greater);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_prec_round(5, Nearest);
    /// assert_eq!(log.to_string(), "3.4");
    /// assert_eq!(o, Greater);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_prec_round(20, Floor);
    /// assert_eq!(log.to_string(), "3.321926");
    /// assert_eq!(o, Less);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_prec_round(20, Ceiling);
    /// assert_eq!(log.to_string(), "3.32193");
    /// assert_eq!(o, Greater);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_prec_round(20, Nearest);
    /// assert_eq!(log.to_string(), "3.32193");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_2_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match self {
            Self(NaN | Infinity { sign: false } | Finite { sign: false, .. }) => {
                (float_nan!(), Equal)
            }
            float_either_zero!() => (float_negative_infinity!(), Equal),
            float_infinity!() => (float_infinity!(), Equal),
            _ => log_base_2_prec_round_normal(&self, prec, rm),
        }
    }

    /// Computes $\log_2 x$, where $x$ is a [`Float`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded value is less than, equal to,
    /// or greater than the exact value. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The base-2 logarithm of any nonzero negative number is `NaN`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \log_2 x+\varepsilon.
    /// $$
    /// - If $\log_2 x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\log_2 x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_2 x|\rfloor-p+1}$.
    /// - If $\log_2 x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_2 x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=-\infty$
    /// - $f(1.0,p,m)=0.0$, and the result is exact
    /// - $f(2^k,p,m)=k$, rounded to precision $p$; the result is exact if and only if $k$ is
    ///   representable with precision $p$
    /// - $f(x,p,m)=\text{NaN}$ for $x<0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::log_base_2_prec_ref`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::log_base_2_round_ref`] instead. If both of these things are true, consider
    /// using `(&Float).log_base_2()` instead.
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
    /// with the given precision. (The result is exactly representable if and only if the input is
    /// `NaN`, infinite, zero, equal to 1, or a power of 2 whose base-2 logarithm is representable
    /// with the given precision.)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_prec_round_ref(5, Floor);
    /// assert_eq!(log.to_string(), "3.2");
    /// assert_eq!(o, Less);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_prec_round_ref(5, Ceiling);
    /// assert_eq!(log.to_string(), "3.4");
    /// assert_eq!(o, Greater);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_prec_round_ref(5, Nearest);
    /// assert_eq!(log.to_string(), "3.4");
    /// assert_eq!(o, Greater);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_prec_round_ref(20, Floor);
    /// assert_eq!(log.to_string(), "3.321926");
    /// assert_eq!(o, Less);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_prec_round_ref(20, Ceiling);
    /// assert_eq!(log.to_string(), "3.32193");
    /// assert_eq!(o, Greater);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_prec_round_ref(20, Nearest);
    /// assert_eq!(log.to_string(), "3.32193");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_2_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match self {
            Self(NaN | Infinity { sign: false } | Finite { sign: false, .. }) => {
                (float_nan!(), Equal)
            }
            float_either_zero!() => (float_negative_infinity!(), Equal),
            float_infinity!() => (float_infinity!(), Equal),
            _ => log_base_2_prec_round_normal(self, prec, rm),
        }
    }

    /// Computes $\log_2 x$, where $x$ is a [`Float`], rounding the result to the nearest value of
    /// the specified precision. The [`Float`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// The base-2 logarithm of any nonzero negative number is `NaN`.
    ///
    /// If the logarithm is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \log_2 x+\varepsilon.
    /// $$
    /// - If $\log_2 x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\log_2 x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\log_2
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=\text{NaN}$
    /// - $f(\pm0.0,p)=-\infty$
    /// - $f(1.0,p)=0.0$, and the result is exact
    /// - $f(2^k,p)=k$, rounded to precision $p$; the result is exact if and only if $k$ is
    ///   representable with precision $p$
    /// - $f(x,p)=\text{NaN}$ for $x<0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::log_base_2_prec_round`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::log_base_2`] instead.
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
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100).0.log_base_2_prec(5);
    /// assert_eq!(log.to_string(), "3.4");
    /// assert_eq!(o, Greater);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100).0.log_base_2_prec(20);
    /// assert_eq!(log.to_string(), "3.32193");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_2_prec(self, prec: u64) -> (Self, Ordering) {
        self.log_base_2_prec_round(prec, Nearest)
    }

    /// Computes $\log_2 x$, where $x$ is a [`Float`], rounding the result to the nearest value of
    /// the specified precision. The [`Float`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The base-2 logarithm of any nonzero negative number is `NaN`.
    ///
    /// If the logarithm is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \log_2 x+\varepsilon.
    /// $$
    /// - If $\log_2 x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\log_2 x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\log_2
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=\text{NaN}$
    /// - $f(\pm0.0,p)=-\infty$
    /// - $f(1.0,p)=0.0$, and the result is exact
    /// - $f(2^k,p)=k$, rounded to precision $p$; the result is exact if and only if $k$ is
    ///   representable with precision $p$
    /// - $f(x,p)=\text{NaN}$ for $x<0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::log_base_2_prec_round_ref`] instead. If you know that your target precision is the
    /// precision of the input, consider using `(&Float).log_base_2()` instead.
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
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_prec_ref(5);
    /// assert_eq!(log.to_string(), "3.4");
    /// assert_eq!(o, Greater);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_prec_ref(20);
    /// assert_eq!(log.to_string(), "3.32193");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_2_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.log_base_2_prec_round_ref(prec, Nearest)
    }

    /// Computes $\log_2 x$, where $x$ is a [`Float`], rounding the result with the specified
    /// rounding mode. The [`Float`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded value is less than, equal to, or greater than the exact value. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The base-2 logarithm of any nonzero negative number is `NaN`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \log_2 x+\varepsilon.
    /// $$
    /// - If $\log_2 x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\log_2 x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_2 x|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $\log_2 x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_2 x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=\infty$
    /// - $f(-\infty,m)=\text{NaN}$
    /// - $f(\pm0.0,m)=-\infty$
    /// - $f(1.0,m)=0.0$, and the result is exact
    /// - $f(2^k,m)=k$, rounded to the precision of the input; the result is exact if and only if
    ///   $k$ is representable with that precision
    /// - $f(x,m)=\text{NaN}$ for $x<0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to specify an output precision, consider using [`Float::log_base_2_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::log_base_2`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision. (The result is exactly representable if and only if the input is `NaN`, infinite,
    /// zero, equal to 1, or a power of 2 whose base-2 logarithm is representable with the input
    /// precision.)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100).0.log_base_2_round(Floor);
    /// assert_eq!(log.to_string(), "3.321928094887362347870319429487");
    /// assert_eq!(o, Less);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_round(Ceiling);
    /// assert_eq!(log.to_string(), "3.32192809488736234787031942949");
    /// assert_eq!(o, Greater);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_round(Nearest);
    /// assert_eq!(log.to_string(), "3.32192809488736234787031942949");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_2_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.log_base_2_prec_round(prec, rm)
    }

    /// Computes $\log_2 x$, where $x$ is a [`Float`], rounding the result with the specified
    /// rounding mode. The [`Float`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// The base-2 logarithm of any nonzero negative number is `NaN`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \log_2 x+\varepsilon.
    /// $$
    /// - If $\log_2 x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\log_2 x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_2 x|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $\log_2 x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_2 x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=\infty$
    /// - $f(-\infty,m)=\text{NaN}$
    /// - $f(\pm0.0,m)=-\infty$
    /// - $f(1.0,m)=0.0$, and the result is exact
    /// - $f(2^k,m)=k$, rounded to the precision of the input; the result is exact if and only if
    ///   $k$ is representable with that precision
    /// - $f(x,m)=\text{NaN}$ for $x<0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::log_base_2_prec_round_ref`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using `(&Float).log_base_2()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision. (The result is exactly representable if and only if the input is `NaN`, infinite,
    /// zero, equal to 1, or a power of 2 whose base-2 logarithm is representable with the input
    /// precision.)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_round_ref(Floor);
    /// assert_eq!(log.to_string(), "3.321928094887362347870319429487");
    /// assert_eq!(o, Less);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_round_ref(Ceiling);
    /// assert_eq!(log.to_string(), "3.32192809488736234787031942949");
    /// assert_eq!(o, Greater);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_round_ref(Nearest);
    /// assert_eq!(log.to_string(), "3.32192809488736234787031942949");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_2_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.log_base_2_prec_round_ref(prec, rm)
    }

    /// Computes $\log_2 x$, where $x$ is a [`Float`], in place, rounding the result to the
    /// specified precision and with the specified rounding mode. An [`Ordering`] is returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`Float`], whenever this function sets the
    /// [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// The base-2 logarithm of any nonzero negative number is `NaN`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \log_2 x+\varepsilon.
    /// $$
    /// - If $\log_2 x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\log_2 x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_2 x|\rfloor-p+1}$.
    /// - If $\log_2 x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_2 x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::log_base_2_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::log_base_2_prec_assign`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::log_base_2_round_assign`] instead. If both of these things are true, consider
    /// using [`Float::log_base_2_assign`] instead.
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
    /// with the given precision. (The result is exactly representable if and only if the input is
    /// `NaN`, infinite, zero, equal to 1, or a power of 2 whose base-2 logarithm is representable
    /// with the given precision.)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.log_base_2_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "3.2");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.log_base_2_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "3.4");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.log_base_2_prec_round_assign(5, Nearest), Greater);
    /// assert_eq!(x.to_string(), "3.4");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.log_base_2_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "3.321926");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.log_base_2_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "3.32193");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.log_base_2_prec_round_assign(20, Nearest), Greater);
    /// assert_eq!(x.to_string(), "3.32193");
    /// ```
    #[inline]
    pub fn log_base_2_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let (result, o) = core::mem::take(self).log_base_2_prec_round(prec, rm);
        *self = result;
        o
    }

    /// Computes $\log_2 x$, where $x$ is a [`Float`], in place, rounding the result to the nearest
    /// value of the specified precision. An [`Ordering`] is returned, indicating whether the
    /// rounded value is less than, equal to, or greater than the exact value. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function sets the [`Float`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// The base-2 logarithm of any nonzero negative number is `NaN`.
    ///
    /// If the logarithm is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \log_2 x+\varepsilon.
    /// $$
    /// - If $\log_2 x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\log_2 x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\log_2
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::log_base_2_prec`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::log_base_2_prec_round_assign`] instead. If you know that your target precision is
    /// the precision of the input, consider using [`Float::log_base_2_assign`] instead.
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
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.log_base_2_prec_assign(5), Greater);
    /// assert_eq!(x.to_string(), "3.4");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.log_base_2_prec_assign(20), Greater);
    /// assert_eq!(x.to_string(), "3.32193");
    /// ```
    #[inline]
    pub fn log_base_2_prec_assign(&mut self, prec: u64) -> Ordering {
        self.log_base_2_prec_round_assign(prec, Nearest)
    }

    /// Computes $\log_2 x$, where $x$ is a [`Float`], in place, rounding the result with the
    /// specified rounding mode. An [`Ordering`] is returned, indicating whether the rounded value
    /// is less than, equal to, or greater than the exact value. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function sets the [`Float`] to `NaN` it also returns
    /// `Equal`.
    ///
    /// The base-2 logarithm of any nonzero negative number is `NaN`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \log_2 x+\varepsilon.
    /// $$
    /// - If $\log_2 x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\log_2 x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_2 x|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $\log_2 x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_2 x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::log_base_2_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::log_base_2_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using [`Float::log_base_2_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision. (The result is exactly representable if and only if the input is `NaN`, infinite,
    /// zero, equal to 1, or a power of 2 whose base-2 logarithm is representable with the input
    /// precision.)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.log_base_2_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "3.321928094887362347870319429487");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.log_base_2_round_assign(Ceiling), Greater);
    /// assert_eq!(x.to_string(), "3.32192809488736234787031942949");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.log_base_2_round_assign(Nearest), Greater);
    /// assert_eq!(x.to_string(), "3.32192809488736234787031942949");
    /// ```
    #[inline]
    pub fn log_base_2_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.log_base_2_prec_round_assign(prec, rm)
    }
}

impl LogBase2 for Float {
    type Output = Self;

    /// Computes $\log_2 x$, where $x$ is a [`Float`], taking it by value.
    ///
    /// If the output has a precision, it is the precision of the input. If the logarithm is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// The base-2 logarithm of any nonzero negative number is `NaN`.
    ///
    /// $$
    /// f(x) = \log_2 x+\varepsilon.
    /// $$
    /// - If $\log_2 x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\log_2 x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\log_2
    ///   x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=\infty$
    /// - $f(-\infty)=\text{NaN}$
    /// - $f(\pm0.0)=-\infty$
    /// - $f(1.0)=0.0$, and the result is exact
    /// - $f(2^k)=k$, rounded to the precision of the input; the result is exact if and only if $k$
    ///   is representable with that precision
    /// - $f(x)=\text{NaN}$ for $x<0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::log_base_2_round`] instead. If you want to specify the output precision, consider
    /// using [`Float::log_base_2_prec`]. If you want both of these things, consider using
    /// [`Float::log_base_2_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::LogBase2;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.log_base_2().is_nan());
    /// assert_eq!(Float::INFINITY.log_base_2(), Float::INFINITY);
    /// assert!(Float::NEGATIVE_INFINITY.log_base_2().is_nan());
    /// assert_eq!(
    ///     Float::from_unsigned_prec(10u32, 100)
    ///         .0
    ///         .log_base_2()
    ///         .to_string(),
    ///     "3.32192809488736234787031942949"
    /// );
    /// assert!(Float::from_signed_prec(-10, 100).0.log_base_2().is_nan());
    /// ```
    #[inline]
    fn log_base_2(self) -> Self {
        let prec = self.significant_bits();
        self.log_base_2_prec_round(prec, Nearest).0
    }
}

impl LogBase2 for &Float {
    type Output = Float;

    /// Computes $\log_2 x$, where $x$ is a [`Float`], taking it by reference.
    ///
    /// If the output has a precision, it is the precision of the input. If the logarithm is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// The base-2 logarithm of any nonzero negative number is `NaN`.
    ///
    /// $$
    /// f(x) = \log_2 x+\varepsilon.
    /// $$
    /// - If $\log_2 x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\log_2 x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\log_2
    ///   x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=\infty$
    /// - $f(-\infty)=\text{NaN}$
    /// - $f(\pm0.0)=-\infty$
    /// - $f(1.0)=0.0$, and the result is exact
    /// - $f(2^k)=k$, rounded to the precision of the input; the result is exact if and only if $k$
    ///   is representable with that precision
    /// - $f(x)=\text{NaN}$ for $x<0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::log_base_2_round_ref`] instead. If you want to specify the output precision,
    /// consider using [`Float::log_base_2_prec_ref`]. If you want both of these things, consider
    /// using [`Float::log_base_2_prec_round_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::LogBase2;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::NAN).log_base_2().is_nan());
    /// assert_eq!((&Float::INFINITY).log_base_2(), Float::INFINITY);
    /// assert!((&Float::NEGATIVE_INFINITY).log_base_2().is_nan());
    /// assert_eq!(
    ///     (&Float::from_unsigned_prec(10u32, 100).0)
    ///         .log_base_2()
    ///         .to_string(),
    ///     "3.32192809488736234787031942949"
    /// );
    /// assert!((&Float::from_signed_prec(-10, 100).0).log_base_2().is_nan());
    /// ```
    #[inline]
    fn log_base_2(self) -> Float {
        let prec = self.significant_bits();
        self.log_base_2_prec_round_ref(prec, Nearest).0
    }
}

impl LogBase2Assign for Float {
    /// Computes $\log_2 x$, where $x$ is a [`Float`], in place.
    ///
    /// If the output has a precision, it is the precision of the input. If the logarithm is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// The base-2 logarithm of any nonzero negative number is `NaN`.
    ///
    /// $$
    /// x \gets \log_2 x+\varepsilon.
    /// $$
    /// - If $\log_2 x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\log_2 x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\log_2
    ///   x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// See the [`Float::log_base_2`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::log_base_2_round_assign`] instead. If you want to specify the output precision,
    /// consider using [`Float::log_base_2_prec_assign`]. If you want both of these things, consider
    /// using [`Float::log_base_2_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::LogBase2Assign;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.log_base_2_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x.log_base_2_assign();
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x.log_base_2_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// x.log_base_2_assign();
    /// assert_eq!(x.to_string(), "3.32192809488736234787031942949");
    ///
    /// let mut x = Float::from_signed_prec(-10, 100).0;
    /// x.log_base_2_assign();
    /// assert!(x.is_nan());
    /// ```
    #[inline]
    fn log_base_2_assign(&mut self) {
        let prec = self.significant_bits();
        self.log_base_2_prec_round_assign(prec, Nearest);
    }
}
