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
use crate::{
    Float, emulate_float_to_float_fn, emulate_rational_to_float_fn, float_either_zero,
    float_infinity, float_nan, float_negative_infinity,
};
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{
    CeilingLogBase2, CheckedLogBase, LogBase10, LogBase10Assign, Sign,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero as ZeroTrait;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::float_extras::float_can_round;
use malachite_nz::platform::Limb;
use malachite_q::Rational;

// Returns `Some(n)` when `x == 10^n` for some integer `n >= 1`. The input `x` must be finite,
// positive, and not equal to 1.
//
// `log_base_10(10^n) = n` is an exactly-representable integer, but the Ziv loop in
// `log_base_10_prec_round_normal` could never certify it (the computed quotient lands on a
// representable value the rounding test cannot resolve), so the exact case must be detected up
// front. This is the `10^n` exactness check from mpfr_log10. Unlike a general base, `10 = 2 * 5` is
// not a perfect power, so `log_base_10(x)` is rational only when `x` is a power of 10, and then the
// result is the integer `n` -- there are no dyadic results to handle.
//
// The check is balloon-safe. An exact `10^n` has bit length about `n * log2(10)`, but its odd part
// (the only part stored in the significand) is `5^n`, needing `n * log2(5)` bits, so the bit length
// is at most about `64 * prec`. When `x`'s exponent exceeds that bound, `x` is too large to be an
// exact power of 10 and is left to the Ziv loop (which then converges, `x` not being a power of
// 10), so `x` is materialized as an integer only when doing so is cheap.
pub(crate) fn float_is_power_of_10(x: &Float) -> Option<u64> {
    let e = i64::from(x.get_exponent().unwrap());
    // x < 1 cannot equal 10^n for n >= 1, and only positive exponents can.
    if e < 1 || u64::exact_from(e) > x.get_prec().unwrap().saturating_mul(64) {
        return None;
    }
    // `Natural::try_from` fails unless `x` is a nonnegative integer.
    let n = Natural::try_from(x).ok()?;
    (&n).checked_log_base(&const { Natural::const_from(10) })
}

// The computation of log_base_10(x) is done by log_base_10(x) = ln(x) / ln(10).
//
// This is mpfr_log10 from log10.c, MPFR 4.3.0. The input is finite, nonzero, and positive.
fn log_base_10_prec_round_normal(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    // If x is 1, the result is 0.
    if *x == 1u32 {
        return (Float::ZERO, Equal);
    }
    // If x = 10^n for some n >= 1, log_base_10(x) = n is exact (though possibly subject to rounding
    // at the target precision).
    if let Some(n) = float_is_power_of_10(x) {
        return Float::from_unsigned_prec_round(n, prec, rm);
    }
    // The result is irrational, so it is never exactly representable.
    assert_ne!(rm, Exact, "Inexact log_base_10");
    const TEN: Float = Float::const_from_unsigned(10);
    // Compute the precision of the intermediary variable: the optimal number of bits, see
    // algorithms.tex.
    let mut working_prec = prec + 4 + prec.ceiling_log_base_2();
    let mut increment = Limb::WIDTH;
    loop {
        // ln(x) / ln(10). ln(x), ln(10), and the division are each correctly rounded (at most 1/2
        // ulp), so the relative error is below 2^(2 - working_prec) and working_prec - 4 correct
        // bits suffice for rounding (mpfr_log10 uses Nt - 4).
        let t = x
            .ln_prec_ref(working_prec)
            .0
            .div_prec(TEN.ln_prec(working_prec).0, working_prec)
            .0;
        if float_can_round(t.significand_ref().unwrap(), working_prec - 4, prec, rm) {
            return Float::from_float_prec_round(t, prec, rm);
        }
        // Increase the precision.
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

// Computes log_base_10(x) for a positive `Rational` x whose logarithm is irrational, in a Ziv loop.
//
// log_base_10(x) = log_2(x) / log_2(10). As in log_base_rational, routing through
// `log_base_2_rational` (rather than computing `ln(x) / ln(10)` directly) reuses its handling of x
// near a power of 2 -- in particular x near 1, where the result is near 0 and a direct computation
// would need a working precision proportional to how close x is to 1. log_2(x), log_2(10), and the
// division are each correctly rounded (at most 1/2 ulp), so the relative error is below 2^(2 -
// working_prec) and working_prec - 4 correct bits suffice for rounding.
fn log_base_10_rational_prec_round_helper(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    const TEN: Float = Float::const_from_unsigned(10);
    let mut working_prec = prec + 4 + prec.ceiling_log_base_2();
    let mut increment = Limb::WIDTH;
    loop {
        let t = Float::log_base_2_rational_prec_ref(x, working_prec)
            .0
            .div_prec(TEN.log_base_2_prec(working_prec).0, working_prec)
            .0;
        if float_can_round(t.significand_ref().unwrap(), working_prec - 4, prec, rm) {
            return Float::from_float_prec_round(t, prec, rm);
        }
        // Increase the precision.
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

impl Float {
    /// Computes $\log_{10} x$, where $x$ is a [`Float`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded value is less than, equal to,
    /// or greater than the exact value. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The base-10 logarithm of any nonzero negative number is `NaN`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \log_{10} x+\varepsilon.
    /// $$
    /// - If $\log_{10} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\log_{10} x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_{10} x|\rfloor-p+1}$.
    /// - If $\log_{10} x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_{10} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=-\infty$
    /// - $f(1.0,p,m)=0.0$, and the result is exact
    /// - $f(10^n,p,m)=n$, rounded to precision $p$; the result is exact if and only if $n$ is
    ///   representable with precision $p$
    /// - $f(x,p,m)=\text{NaN}$ for $x<0$
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::log_base_10_prec`] instead.
    /// If you know that your target precision is the precision of the input, consider using
    /// [`Float::log_base_10_round`] instead. If both of these things are true, consider using
    /// [`Float::log_base_10`] instead.
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
    /// let (log, o) = Float::from(1000).log_base_10_prec_round(10, Nearest);
    /// assert_eq!(log.to_string(), "3.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (log, o) = Float::from(50).log_base_10_prec_round(10, Floor);
    /// assert_eq!(log.to_string(), "1.697");
    /// assert_eq!(o, Less);
    ///
    /// let (log, o) = Float::from(50).log_base_10_prec_round(10, Ceiling);
    /// assert_eq!(log.to_string(), "1.699");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_10_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match self {
            Self(NaN | Infinity { sign: false } | Finite { sign: false, .. }) => {
                (float_nan!(), Equal)
            }
            float_either_zero!() => (float_negative_infinity!(), Equal),
            float_infinity!() => (float_infinity!(), Equal),
            _ => log_base_10_prec_round_normal(&self, prec, rm),
        }
    }

    /// Computes $\log_{10} x$, where $x$ is a [`Float`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded value is less than, equal to,
    /// or greater than the exact value. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::log_base_10_prec_round`] for details, special cases, and a description of the
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
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::from(1000).log_base_10_prec_round_ref(10, Nearest);
    /// assert_eq!(log.to_string(), "3.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_10_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match self {
            Self(NaN | Infinity { sign: false } | Finite { sign: false, .. }) => {
                (float_nan!(), Equal)
            }
            float_either_zero!() => (float_negative_infinity!(), Equal),
            float_infinity!() => (float_infinity!(), Equal),
            _ => log_base_10_prec_round_normal(self, prec, rm),
        }
    }

    /// Computes $\log_{10} x$, where $x$ is a [`Float`], rounding the result to the nearest value
    /// of the specified precision. The [`Float`] is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value.
    ///
    /// See [`Float::log_base_10_prec_round`] for details and special cases.
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
    /// let (log, o) = Float::from(50).log_base_10_prec(10);
    /// assert_eq!(log.to_string(), "1.699");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_10_prec(self, prec: u64) -> (Self, Ordering) {
        self.log_base_10_prec_round(prec, Nearest)
    }

    /// Computes $\log_{10} x$, where $x$ is a [`Float`], rounding the result to the nearest value
    /// of the specified precision. The [`Float`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value.
    ///
    /// See [`Float::log_base_10_prec_round`] for details and special cases.
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
    /// let (log, o) = Float::from(50).log_base_10_prec_ref(10);
    /// assert_eq!(log.to_string(), "1.699");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_10_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.log_base_10_prec_round_ref(prec, Nearest)
    }

    /// Computes $\log_{10} x$, where $x$ is a [`Float`], rounding the result to the precision of
    /// the input and with the specified rounding mode. The [`Float`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded value is less than, equal to,
    /// or greater than the exact value.
    ///
    /// See [`Float::log_base_10_prec_round`] for details and special cases.
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
    /// let (log, o) = Float::from(1000).log_base_10_round(Floor);
    /// assert_eq!(log.to_string(), "3.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_10_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.log_base_10_prec_round(prec, rm)
    }

    /// Computes $\log_{10} x$, where $x$ is a [`Float`], rounding the result to the precision of
    /// the input and with the specified rounding mode. The [`Float`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded value is less than, equal to,
    /// or greater than the exact value.
    ///
    /// See [`Float::log_base_10_prec_round`] for details and special cases.
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
    /// let (log, o) = Float::from(100).log_base_10_round_ref(Ceiling);
    /// assert_eq!(log.to_string(), "2.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_10_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.log_base_10_prec_round_ref(self.significant_bits(), rm)
    }

    /// Computes $\log_{10} x$, where $x$ is a [`Float`], in place, rounding the result to the
    /// specified precision and with the specified rounding mode. An [`Ordering`] is returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value.
    ///
    /// See [`Float::log_base_10_prec_round`] for details and special cases.
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
    /// let mut x = Float::from(50);
    /// let o = x.log_base_10_prec_round_assign(10, Floor);
    /// assert_eq!(x.to_string(), "1.697");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn log_base_10_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let (result, o) = core::mem::take(self).log_base_10_prec_round(prec, rm);
        *self = result;
        o
    }

    /// Computes $\log_{10} x$, where $x$ is a [`Float`], in place, rounding the result to the
    /// nearest value of the specified precision. An [`Ordering`] is returned, indicating whether
    /// the rounded value is less than, equal to, or greater than the exact value.
    ///
    /// See [`Float::log_base_10_prec_round`] for details and special cases.
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
    /// let mut x = Float::from(1000);
    /// let o = x.log_base_10_prec_assign(10);
    /// assert_eq!(x.to_string(), "3.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_10_prec_assign(&mut self, prec: u64) -> Ordering {
        self.log_base_10_prec_round_assign(prec, Nearest)
    }

    /// Computes $\log_{10} x$, where $x$ is a [`Float`], in place, rounding the result to the
    /// precision of the input and with the specified rounding mode. An [`Ordering`] is returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value.
    ///
    /// See [`Float::log_base_10_prec_round`] for details and special cases.
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
    /// let mut x = Float::from(100);
    /// let o = x.log_base_10_round_assign(Nearest);
    /// assert_eq!(x.to_string(), "2.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_10_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.log_base_10_prec_round_assign(prec, rm)
    }

    /// Computes $\log_{10} x$, where $x$ is a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded value is less than, equal to, or greater than the exact value. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The base-10 logarithm of any negative number is `NaN`.
    ///
    /// Inputs of any magnitude are handled, including [`Rational`]s whose magnitudes are too large
    /// or too small to be representable as [`Float`]s. Neither overflow nor underflow of the output
    /// is possible.
    ///
    /// See [`Float::log_base_10_prec_round`] for details and a description of the rounding
    /// behavior.
    ///
    /// Special cases:
    /// - $f(0,p,m)=-\infty$
    /// - $f(x,p,m)=\text{NaN}$ for $x<0$
    /// - $f(1,p,m)=0.0$, and the result is exact
    /// - $f(10^n,p,m)=n$, rounded to precision $p$; the result is exact if and only if the integer
    ///   $n$ is representable with precision $p$. This includes negative powers of 10 like $1/100$,
    ///   and powers of 10 whose exponents lie far outside the exponent range of [`Float`].
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
    /// with the given precision. (The result is exactly representable if and only if $x \leq 0$ or
    /// $x$ is a power of 10 whose base-10 logarithm is representable with the given precision.)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::log_base_10_rational_prec_round(Rational::from(1000), 10, Exact);
    /// assert_eq!(log.to_string(), "3.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (log, o) =
    ///     Float::log_base_10_rational_prec_round(Rational::from_signeds(1, 100), 10, Exact);
    /// assert_eq!(log.to_string(), "-2.0"); // log_10(1/100) = -2
    /// assert_eq!(o, Equal);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn log_base_10_rational_prec_round(
        x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::log_base_10_rational_prec_round_ref(&x, prec, rm)
    }

    /// Computes $\log_{10} x$, where $x$ is a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded value is less than, equal to, or greater than the exact value. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// See [`Float::log_base_10_rational_prec_round`] for details, special cases, and a description
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
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) =
    ///     Float::log_base_10_rational_prec_round_ref(&Rational::from(1000), 10, Nearest);
    /// assert_eq!(log.to_string(), "3.0");
    /// assert_eq!(o, Equal);
    /// ```
    pub fn log_base_10_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match x.sign() {
            Equal => return (float_negative_infinity!(), Equal),
            Less => return (float_nan!(), Equal),
            Greater => {}
        }
        // If x = 10^m, then log_base_10(x) = m is an exact integer (m may be negative, for x < 1).
        // The Ziv loop could never certify it (see float_is_power_of_10 for the Float analog).
        if let Some(m) = x.checked_log_base(10) {
            return Self::from_signed_prec_round(m, prec, rm);
        }
        // The result is irrational, so it is never exactly representable.
        assert_ne!(rm, Exact, "Inexact log_base_10");
        log_base_10_rational_prec_round_helper(x, prec, rm)
    }

    /// Computes $\log_{10} x$, where $x$ is a [`Rational`], rounding the result to the nearest
    /// value of the specified precision and returning the result as a [`Float`]. The [`Rational`]
    /// is taken by value. An [`Ordering`] is also returned, indicating whether the rounded value is
    /// less than, equal to, or greater than the exact value.
    ///
    /// See [`Float::log_base_10_rational_prec_round`] for details and special cases.
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
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::log_base_10_rational_prec(Rational::from_signeds(1, 100), 10);
    /// assert_eq!(log.to_string(), "-2.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_10_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::log_base_10_rational_prec_round(x, prec, Nearest)
    }

    /// Computes $\log_{10} x$, where $x$ is a [`Rational`], rounding the result to the nearest
    /// value of the specified precision and returning the result as a [`Float`]. The [`Rational`]
    /// is taken by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// value is less than, equal to, or greater than the exact value.
    ///
    /// See [`Float::log_base_10_rational_prec_round`] for details and special cases.
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
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::log_base_10_rational_prec_ref(&Rational::from(50), 10);
    /// assert_eq!(log.to_string(), "1.699");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_10_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::log_base_10_rational_prec_round_ref(x, prec, Nearest)
    }
}

impl LogBase10 for Float {
    type Output = Self;

    /// Computes $\log_{10} x$, where $x$ is a [`Float`], rounding the result to the nearest value
    /// of the input's precision. The [`Float`] is taken by value.
    ///
    /// The base-10 logarithm of any nonzero negative number is `NaN`. See
    /// [`Float::log_base_10_prec_round`] for the special cases.
    ///
    /// $$
    /// f(x) = \log_{10} x+\varepsilon,
    /// $$
    /// where $|\varepsilon| \leq 2^{\lfloor\log_2 |\log_{10} x|\rfloor-p}$ and $p$ is the precision
    /// of the input.
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
    /// use malachite_base::num::arithmetic::traits::LogBase10;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::from(1000).log_base_10().to_string(), "3.0");
    /// assert_eq!(Float::from(100).log_base_10().to_string(), "2.0");
    /// ```
    #[inline]
    fn log_base_10(self) -> Self {
        let prec = self.significant_bits();
        self.log_base_10_prec_round(prec, Nearest).0
    }
}

impl LogBase10 for &Float {
    type Output = Float;

    /// Computes $\log_{10} x$, where $x$ is a [`Float`], rounding the result to the nearest value
    /// of the input's precision. The [`Float`] is taken by reference.
    ///
    /// The base-10 logarithm of any nonzero negative number is `NaN`. See
    /// [`Float::log_base_10_prec_round`] for the special cases.
    ///
    /// $$
    /// f(x) = \log_{10} x+\varepsilon,
    /// $$
    /// where $|\varepsilon| \leq 2^{\lfloor\log_2 |\log_{10} x|\rfloor-p}$ and $p$ is the precision
    /// of the input.
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
    /// use malachite_base::num::arithmetic::traits::LogBase10;
    /// use malachite_float::Float;
    ///
    /// assert_eq!((&Float::from(1000)).log_base_10().to_string(), "3.0");
    /// ```
    #[inline]
    fn log_base_10(self) -> Float {
        self.log_base_10_prec_round_ref(self.significant_bits(), Nearest)
            .0
    }
}

impl LogBase10Assign for Float {
    /// Replaces a [`Float`] $x$ with $\log_{10} x$, rounding the result to the nearest value of the
    /// input's precision.
    ///
    /// The base-10 logarithm of any nonzero negative number is `NaN`. See
    /// [`Float::log_base_10_prec_round`] for the special cases.
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
    /// use malachite_base::num::arithmetic::traits::LogBase10Assign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(1000);
    /// x.log_base_10_assign();
    /// assert_eq!(x.to_string(), "3.0");
    /// ```
    #[inline]
    fn log_base_10_assign(&mut self) {
        let prec = self.significant_bits();
        self.log_base_10_prec_round_assign(prec, Nearest);
    }
}

/// Computes $\log_{10} x$, the base-10 logarithm of a primitive float. Using this function is more
/// accurate than using the primitive float `log10` function (the standard library's `log10` is not
/// always correctly rounded).
///
/// The base-10 logarithm of any negative number is `NaN`.
///
/// $$
/// f(x) = \log_{10} x+\varepsilon.
/// $$
/// - If $\log_{10} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $\log_{10} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |\log_{10}
///   x|\rfloor-p}$, where $p$ is precision of the output (typically 24 if `T` is a [`f32`] and 53
///   if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(\infty)=\infty$
/// - $f(-\infty)=\text{NaN}$
/// - $f(\pm0.0)=-\infty$
/// - $f(1.0)=0.0$
/// - $f(x)=\text{NaN}$ for $x<0$
///
/// Neither overflow nor underflow is possible.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::arithmetic::log_base_10::primitive_float_log_base_10;
///
/// assert!(primitive_float_log_base_10(f32::NAN).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_10(f32::INFINITY)),
///     NiceFloat(f32::INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_10(0.0f32)),
///     NiceFloat(f32::NEGATIVE_INFINITY)
/// );
/// // log_10(1000) = 3
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_10(1000.0f32)),
///     NiceFloat(3.0)
/// );
/// // log_10(50)
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_10(50.0f32)),
///     NiceFloat(1.69897)
/// );
/// assert!(primitive_float_log_base_10(-1.0f32).is_nan());
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_log_base_10<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::log_base_10_prec, x)
}

/// Computes $\log_{10} x$, the base-10 logarithm of a [`Rational`], returning a primitive float
/// result.
///
/// If the logarithm is equidistant from two primitive floats, the primitive float with fewer 1s in
/// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding
/// mode.
///
/// The base-10 logarithm of any negative number is `NaN`.
///
/// $$
/// f(x) = \log_{10} x+\varepsilon.
/// $$
/// - If $\log_{10} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $\log_{10} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |\log_{10}
///   x|\rfloor-p}$, where $p$ is precision of the output (typically 24 if `T` is a [`f32`] and 53
///   if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(0)=-\infty$
/// - $f(x)=\text{NaN}$ for $x<0$
/// - $f(1)=0.0$
///
/// Neither overflow nor underflow is possible.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::{NegativeInfinity, Zero};
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::arithmetic::log_base_10::primitive_float_log_base_10_rational;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_10_rational::<f64>(&Rational::ZERO)),
///     NiceFloat(f64::NEGATIVE_INFINITY)
/// );
/// // log_10(1000) = 3
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_10_rational::<f64>(
///         &Rational::from(1000)
///     )),
///     NiceFloat(3.0)
/// );
/// // log_10(1/3)
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_10_rational::<f64>(
///         &Rational::from_unsigneds(1u8, 3)
///     )),
///     NiceFloat(-0.47712125471966244)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_10_rational::<f64>(
///         &Rational::from(-1000)
///     )),
///     NiceFloat(f64::NAN)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_log_base_10_rational<T: PrimitiveFloat>(x: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_to_float_fn(Float::log_base_10_rational_prec_ref, x)
}
