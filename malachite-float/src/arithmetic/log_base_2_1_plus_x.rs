// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright 2001-2026 Free Software Foundation, Inc.
//
//      Contributed by the AriC and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Infinity, NaN, Zero};
use crate::{Float, float_infinity, float_nan, float_negative_infinity};
use core::cmp::Ordering::{self, *};
use core::cmp::max;
use malachite_base::num::arithmetic::traits::{
    CeilingLogBase2, IsPowerOf2, LogBase2Of1PlusX, LogBase2Of1PlusXAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::arithmetic::float_extras::{
    float_can_round, float_significand_leading_ones,
};
use malachite_nz::platform::Limb;

// Returns `Some(k)` if `1 + x` is exactly $2^k$ (equivalently $x = 2^k - 1$), and `None` otherwise.
// The input must be finite, nonzero, and greater than $-1$.
//
// `1 + x` is a power of 2 exactly when the mantissa of `x` is a run of ones (the value $2^j - 1$),
// the exponent of `x` equals $j$ (so `x` is the integer $2^j - 1$ and $k = j$) when `x` is
// positive, or the exponent of `x` is 0 (so `x` is $-(1 - 2^{-j})$ and $k = -j$) when `x` is
// negative. This replaces MPFR's `mpfr_log2p1_isexact`, which adds 1 to `x` and tests for a power
// of 2; we test the significand's bits directly.
pub(crate) fn log_base_2_1_plus_x_exact(x: &Float) -> Option<i64> {
    let j = i64::exact_from(float_significand_leading_ones(
        x.significand_ref().unwrap(),
    )?);
    let e = i64::from(x.get_exponent().unwrap());
    if *x > 0u32 {
        (e == j).then_some(j)
    } else {
        (e == 0).then_some(-j)
    }
}

// If `x` is $2^k$ for a `k` large enough that the Ziv loop would never converge, returns the
// correctly-rounded value of $\log_2(1+x)$; otherwise returns `None`. The input must be finite,
// nonzero, and greater than $-1$, and `1 + x` must not be a power of 2.
//
// This is mpfr_log2p1_special from log2p1.c, MPFR 4.3.0. For $x = 2^k$ with $k \geq 1$ we have $k <
// \log_2(1+x) < k + 2/x$. When $2/x$ is below a quarter of an ulp of $k$, the result rounds the
// same way as $k$ stepped up by a single ulp, so the rounding can be decided directly.
fn log_base_2_1_plus_x_special(
    x: &Float,
    prec: u64,
    rm: RoundingMode,
) -> Option<(Float, Ordering)> {
    if !x.is_power_of_2() {
        return None;
    }
    let expx = i64::from(x.get_exponent().unwrap());
    // x = 2^k
    let k = expx - 1;
    if k <= 0 {
        return None;
    }
    // expk is the exponent of k. We have 2 / x < 2^(2 - expx), so if 2 - expx < expk - prec - 1,
    // then 2 / x < (1/4) ulp(k) and the correct rounding can be decided.
    let expk = i64::exact_from(u64::exact_from(k).ceiling_log_base_2());
    if 2 - expx >= expk - i64::exact_from(prec) - 1 {
        return None;
    }
    // log_2(1 + x) lies in (k, k + 1/4 ulp(k)); round k stepped up by one ulp.
    let high_prec = max(prec + 2, Limb::WIDTH);
    let mut t = Float::from_signed_prec(k, high_prec).0;
    t.increment();
    Some(Float::from_float_prec_round(t, prec, rm))
}

// The computation of log2p1 is done by log_base_2_1_plus_x(x) = ln_1_plus_x(x) / ln(2).
//
// This is mpfr_log2p1 from log2p1.c, MPFR 4.3.0, where the input is finite and nonzero.
fn log_base_2_1_plus_x_prec_round_normal(
    x: &Float,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    // log_2(1 + x) is undefined for x < -1.
    match x.partial_cmp(&-1i32).unwrap() {
        Equal => return (float_negative_infinity!(), Equal),
        Less => return (float_nan!(), Equal),
        _ => {}
    }
    // If 1 + x is exactly a power of 2, the result is an integer (subject to rounding at the target
    // precision).
    if let Some(k) = log_base_2_1_plus_x_exact(x) {
        return Float::from_signed_prec_round(k, prec, rm);
    }
    // The result is never exactly representable otherwise.
    assert_ne!(rm, Exact, "Inexact log_base_2_1_plus_x");
    // If x = 2^k with k huge, the Ziv loop would never converge; handle it specially.
    if let Some(result) = log_base_2_1_plus_x_special(x, prec, rm) {
        return result;
    }
    // General case. Compute the precision of the intermediary variable: the optimal number of bits,
    // see algorithms.tex.
    let mut working_prec = prec + prec.ceiling_log_base_2() + 6;
    let mut increment = Limb::WIDTH;
    loop {
        // ln(1 + x) / ln(2). This is log_2(1 + x) * (1 + theta)^3 with |theta| < 2^-working_prec,
        // and |(1 + theta)^3 - 1| < 4 * theta for working_prec >= 2, i.e. 4 ulps of error.
        let t = x
            .ln_1_plus_x_prec_ref(working_prec)
            .0
            .div_prec(Float::ln_2_prec(working_prec).0, working_prec)
            .0;
        if float_can_round(t.significand_ref().unwrap(), working_prec - 2, prec, rm) {
            return Float::from_float_prec_round(t, prec, rm);
        }
        // Increase the precision.
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

impl Float {
    /// Computes $\log_2(1+x)$, where $x$ is a [`Float`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded value is less than, equal to,
    /// or greater than the exact value. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\log_2(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, `NaN` is returned.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \log_2(1+x)+\varepsilon.
    /// $$
    /// - If $\log_2(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\log_2(1+x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_2(1+x)|\rfloor-p+1}$.
    /// - If $\log_2(1+x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_2(1+x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=\pm0.0$
    /// - $f(-1,p,m)=-\infty$
    /// - $f(x,p,m)=\text{NaN}$ for $x<-1$
    /// - $f(x,p,m)=k$ when $1+x=2^k$. The result is the integer $k$ (subject to rounding at
    ///   precision $p$, and exact iff $k$ is representable with precision $p$). This covers $x$ a
    ///   power of 2 minus 1 (e.g. $x=1\to1$, $x=3\to2$) and negative $x$ such as $x=-1/2\to-1$ and
    ///   $x=-3/4\to-2$.
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::log_base_2_1_plus_x_prec`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::log_base_2_1_plus_x_round`] instead. If both of these things are true,
    /// consider using [`Float::log_base_2_1_plus_x`] instead.
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
    /// with the given precision. (The result is exactly representable only when the input is `NaN`,
    /// infinite, zero, $-1$, less than $-1$, or a value for which $1+x$ is a power of 2 whose
    /// base-2 logarithm is representable with the given precision.)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_1_plus_x_prec_round(5, Floor);
    /// assert_eq!(log.to_string(), "3.4");
    /// assert_eq!(o, Less);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_1_plus_x_prec_round(5, Ceiling);
    /// assert_eq!(log.to_string(), "3.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_1_plus_x_prec_round(5, Nearest);
    /// assert_eq!(log.to_string(), "3.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_1_plus_x_prec_round(20, Floor);
    /// assert_eq!(log.to_string(), "3.459431");
    /// assert_eq!(o, Less);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_1_plus_x_prec_round(20, Ceiling);
    /// assert_eq!(log.to_string(), "3.459435");
    /// assert_eq!(o, Greater);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_1_plus_x_prec_round(20, Nearest);
    /// assert_eq!(log.to_string(), "3.459431");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn log_base_2_1_plus_x_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match self {
            Self(NaN | Infinity { sign: false }) => (float_nan!(), Equal),
            float_infinity!() => (float_infinity!(), Equal),
            // log_base_2_1_plus_x(±0) = ±0
            Self(Zero { .. }) => (self, Equal),
            _ => log_base_2_1_plus_x_prec_round_normal(&self, prec, rm),
        }
    }

    /// Computes $\log_2(1+x)$, where $x$ is a [`Float`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded value is less than, equal to,
    /// or greater than the exact value. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\log_2(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, `NaN` is returned.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \log_2(1+x)+\varepsilon.
    /// $$
    /// - If $\log_2(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\log_2(1+x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_2(1+x)|\rfloor-p+1}$.
    /// - If $\log_2(1+x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_2(1+x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=\pm0.0$
    /// - $f(-1,p,m)=-\infty$
    /// - $f(x,p,m)=\text{NaN}$ for $x<-1$
    /// - $f(x,p,m)=k$ when $1+x=2^k$. The result is the integer $k$ (subject to rounding at
    ///   precision $p$, and exact iff $k$ is representable with precision $p$). This covers $x$ a
    ///   power of 2 minus 1 (e.g. $x=1\to1$, $x=3\to2$) and negative $x$ such as $x=-1/2\to-1$ and
    ///   $x=-3/4\to-2$.
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::log_base_2_1_plus_x_prec_ref`] instead. If you know that your target precision is
    /// the precision of the input, consider using [`Float::log_base_2_1_plus_x_round_ref`] instead.
    /// If both of these things are true, consider using `(&Float).log_base_2_1_plus_x()` instead.
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
    /// with the given precision. (The result is exactly representable only when the input is `NaN`,
    /// infinite, zero, $-1$, less than $-1$, or a value for which $1+x$ is a power of 2 whose
    /// base-2 logarithm is representable with the given precision.)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_1_plus_x_prec_round_ref(5, Floor);
    /// assert_eq!(log.to_string(), "3.4");
    /// assert_eq!(o, Less);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_1_plus_x_prec_round_ref(5, Ceiling);
    /// assert_eq!(log.to_string(), "3.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_1_plus_x_prec_round_ref(5, Nearest);
    /// assert_eq!(log.to_string(), "3.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_1_plus_x_prec_round_ref(20, Floor);
    /// assert_eq!(log.to_string(), "3.459431");
    /// assert_eq!(o, Less);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_1_plus_x_prec_round_ref(20, Ceiling);
    /// assert_eq!(log.to_string(), "3.459435");
    /// assert_eq!(o, Greater);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_1_plus_x_prec_round_ref(20, Nearest);
    /// assert_eq!(log.to_string(), "3.459431");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn log_base_2_1_plus_x_prec_round_ref(
        &self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match self {
            Self(NaN | Infinity { sign: false }) => (float_nan!(), Equal),
            float_infinity!() => (float_infinity!(), Equal),
            // log_base_2_1_plus_x(±0) = ±0
            Self(Zero { .. }) => (self.clone(), Equal),
            _ => log_base_2_1_plus_x_prec_round_normal(self, prec, rm),
        }
    }

    /// Computes $\log_2(1+x)$, where $x$ is a [`Float`], rounding the result to the nearest value
    /// of the specified precision. The [`Float`] is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $\log_2(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, `NaN` is returned.
    ///
    /// If the result is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \log_2(1+x)+\varepsilon.
    /// $$
    /// - If $\log_2(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\log_2(1+x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\log_2(1+x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=\text{NaN}$
    /// - $f(\pm0.0,p)=\pm0.0$
    /// - $f(-1,p)=-\infty$
    /// - $f(x,p)=\text{NaN}$ for $x<-1$
    /// - $f(x,p)=k$ when $1+x=2^k$. The result is the integer $k$ (subject to rounding at precision
    ///   $p$, and exact iff $k$ is representable with precision $p$). This covers $x$ a power of 2
    ///   minus 1 (e.g. $x=1\to1$, $x=3\to2$) and negative $x$ such as $x=-1/2\to-1$ and
    ///   $x=-3/4\to-2$.
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::log_base_2_1_plus_x_prec_round`] instead. If you know that your target precision is
    /// the precision of the input, consider using [`Float::log_base_2_1_plus_x`] instead.
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
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_1_plus_x_prec(5);
    /// assert_eq!(log.to_string(), "3.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_1_plus_x_prec(20);
    /// assert_eq!(log.to_string(), "3.459431");
    /// assert_eq!(o, Less);
    ///
    /// let (log, o) = Float::ONE.log_base_2_1_plus_x_prec(20);
    /// assert_eq!(log.to_string(), "1.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_2_1_plus_x_prec(self, prec: u64) -> (Self, Ordering) {
        self.log_base_2_1_plus_x_prec_round(prec, Nearest)
    }

    /// Computes $\log_2(1+x)$, where $x$ is a [`Float`], rounding the result to the nearest value
    /// of the specified precision. The [`Float`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $\log_2(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, `NaN` is returned.
    ///
    /// If the result is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \log_2(1+x)+\varepsilon.
    /// $$
    /// - If $\log_2(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\log_2(1+x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\log_2(1+x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=\text{NaN}$
    /// - $f(\pm0.0,p)=\pm0.0$
    /// - $f(-1,p)=-\infty$
    /// - $f(x,p)=\text{NaN}$ for $x<-1$
    /// - $f(x,p)=k$ when $1+x=2^k$. The result is the integer $k$ (subject to rounding at precision
    ///   $p$, and exact iff $k$ is representable with precision $p$). This covers $x$ a power of 2
    ///   minus 1 (e.g. $x=1\to1$, $x=3\to2$) and negative $x$ such as $x=-1/2\to-1$ and
    ///   $x=-3/4\to-2$.
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::log_base_2_1_plus_x_prec_round_ref`] instead. If you know that your target
    /// precision is the precision of the input, consider using `(&Float).log_base_2_1_plus_x()`
    /// instead.
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
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_1_plus_x_prec_ref(5);
    /// assert_eq!(log.to_string(), "3.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_1_plus_x_prec_ref(20);
    /// assert_eq!(log.to_string(), "3.459431");
    /// assert_eq!(o, Less);
    ///
    /// let (log, o) = Float::ONE.log_base_2_1_plus_x_prec_ref(20);
    /// assert_eq!(log.to_string(), "1.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_2_1_plus_x_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.log_base_2_1_plus_x_prec_round_ref(prec, Nearest)
    }

    /// Computes $\log_2(1+x)$, where $x$ is a [`Float`], rounding the result with the specified
    /// rounding mode. The [`Float`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded value is less than, equal to, or greater than the exact value. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $\log_2(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, `NaN` is returned.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \log_2(1+x)+\varepsilon.
    /// $$
    /// - If $\log_2(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\log_2(1+x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_2(1+x)|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $\log_2(1+x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_2(1+x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=\infty$
    /// - $f(-\infty,m)=\text{NaN}$
    /// - $f(\pm0.0,m)=\pm0.0$
    /// - $f(-1,m)=-\infty$
    /// - $f(x,m)=\text{NaN}$ for $x<-1$
    /// - $f(x,m)=k$ when $1+x=2^k$. The result is the integer $k$ (subject to rounding at the input
    ///   precision $p$, and exact iff $k$ is representable with precision $p$). This covers $x$ a
    ///   power of 2 minus 1 (e.g. $x=1\to1$, $x=3\to2$) and negative $x$ such as $x=-1/2\to-1$ and
    ///   $x=-3/4\to-2$.
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::log_base_2_1_plus_x_prec_round`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using [`Float::log_base_2_1_plus_x`] instead.
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
    /// precision. (The result is exactly representable only when the input is `NaN`, infinite,
    /// zero, $-1$, less than $-1$, or a value for which $1+x$ is a power of 2 whose base-2
    /// logarithm is representable with the given precision.)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_1_plus_x_round(Floor);
    /// assert_eq!(log.to_string(), "3.459431618637297256199363046725");
    /// assert_eq!(o, Less);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_1_plus_x_round(Ceiling);
    /// assert_eq!(log.to_string(), "3.459431618637297256199363046728");
    /// assert_eq!(o, Greater);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_1_plus_x_round(Nearest);
    /// assert_eq!(log.to_string(), "3.459431618637297256199363046725");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn log_base_2_1_plus_x_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.log_base_2_1_plus_x_prec_round(prec, rm)
    }

    /// Computes $\log_2(1+x)$, where $x$ is a [`Float`], rounding the result with the specified
    /// rounding mode. The [`Float`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $\log_2(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, `NaN` is returned.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \log_2(1+x)+\varepsilon.
    /// $$
    /// - If $\log_2(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\log_2(1+x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_2(1+x)|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $\log_2(1+x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_2(1+x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=\infty$
    /// - $f(-\infty,m)=\text{NaN}$
    /// - $f(\pm0.0,m)=\pm0.0$
    /// - $f(-1,m)=-\infty$
    /// - $f(x,m)=\text{NaN}$ for $x<-1$
    /// - $f(x,m)=k$ when $1+x=2^k$. The result is the integer $k$ (subject to rounding at the input
    ///   precision $p$, and exact iff $k$ is representable with precision $p$). This covers $x$ a
    ///   power of 2 minus 1 (e.g. $x=1\to1$, $x=3\to2$) and negative $x$ such as $x=-1/2\to-1$ and
    ///   $x=-3/4\to-2$.
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::log_base_2_1_plus_x_prec_round_ref`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using `(&Float).log_base_2_1_plus_x()` instead.
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
    /// precision. (The result is exactly representable only when the input is `NaN`, infinite,
    /// zero, $-1$, less than $-1$, or a value for which $1+x$ is a power of 2 whose base-2
    /// logarithm is representable with the given precision.)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_1_plus_x_round_ref(Floor);
    /// assert_eq!(log.to_string(), "3.459431618637297256199363046725");
    /// assert_eq!(o, Less);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_1_plus_x_round_ref(Ceiling);
    /// assert_eq!(log.to_string(), "3.459431618637297256199363046728");
    /// assert_eq!(o, Greater);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_1_plus_x_round_ref(Nearest);
    /// assert_eq!(log.to_string(), "3.459431618637297256199363046725");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn log_base_2_1_plus_x_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.log_base_2_1_plus_x_prec_round_ref(prec, rm)
    }

    /// Computes $\log_2(1+x)$, where $x$ is a [`Float`], in place, rounding the result to the
    /// specified precision and with the specified rounding mode. An [`Ordering`] is returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`Float`], whenever this function sets the
    /// [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// $\log_2(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, the [`Float`] is set to `NaN`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \log_2(1+x)+\varepsilon.
    /// $$
    /// - If $\log_2(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\log_2(1+x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_2(1+x)|\rfloor-p+1}$.
    /// - If $\log_2(1+x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_2(1+x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::log_base_2_1_plus_x_prec_round`] documentation for information on special
    /// cases, overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::log_base_2_1_plus_x_prec_assign`] instead. If you know that your target precision
    /// is the precision of the input, consider using [`Float::log_base_2_1_plus_x_round_assign`]
    /// instead. If both of these things are true, consider using
    /// [`Float::log_base_2_1_plus_x_assign`] instead.
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
    /// with the given precision. (The result is exactly representable only when the input is `NaN`,
    /// infinite, zero, $-1$, less than $-1$, or a value for which $1+x$ is a power of 2 whose
    /// base-2 logarithm is representable with the given precision.)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.log_base_2_1_plus_x_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "3.4");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.log_base_2_1_plus_x_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "3.5");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.log_base_2_1_plus_x_prec_round_assign(5, Nearest), Greater);
    /// assert_eq!(x.to_string(), "3.5");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.log_base_2_1_plus_x_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "3.459431");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(
    ///     x.log_base_2_1_plus_x_prec_round_assign(20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "3.459435");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.log_base_2_1_plus_x_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "3.459431");
    /// ```
    #[inline]
    pub fn log_base_2_1_plus_x_prec_round_assign(
        &mut self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (result, o) = core::mem::take(self).log_base_2_1_plus_x_prec_round(prec, rm);
        *self = result;
        o
    }

    /// Computes $\log_2(1+x)$, where $x$ is a [`Float`], in place, rounding the result to the
    /// nearest value of the specified precision. An [`Ordering`] is returned, indicating whether
    /// the rounded value is less than, equal to, or greater than the exact value. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function sets the [`Float`] to `NaN` it
    /// also returns `Equal`.
    ///
    /// $\log_2(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, the [`Float`] is set to `NaN`.
    ///
    /// If the result is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \log_2(1+x)+\varepsilon.
    /// $$
    /// - If $\log_2(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\log_2(1+x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\log_2(1+x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::log_base_2_1_plus_x_prec`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::log_base_2_1_plus_x_prec_round_assign`] instead. If you know that your target
    /// precision is the precision of the input, consider using
    /// [`Float::log_base_2_1_plus_x_assign`] instead.
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
    /// assert_eq!(x.log_base_2_1_plus_x_prec_assign(5), Greater);
    /// assert_eq!(x.to_string(), "3.5");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.log_base_2_1_plus_x_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "3.459431");
    /// ```
    #[inline]
    pub fn log_base_2_1_plus_x_prec_assign(&mut self, prec: u64) -> Ordering {
        self.log_base_2_1_plus_x_prec_round_assign(prec, Nearest)
    }

    /// Computes $\log_2(1+x)$, where $x$ is a [`Float`], in place, rounding the result with the
    /// specified rounding mode. An [`Ordering`] is returned, indicating whether the rounded value
    /// is less than, equal to, or greater than the exact value. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function sets the [`Float`] to `NaN` it also returns
    /// `Equal`.
    ///
    /// $\log_2(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, the [`Float`] is set to `NaN`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \log_2(1+x)+\varepsilon.
    /// $$
    /// - If $\log_2(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\log_2(1+x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_2(1+x)|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $\log_2(1+x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_2(1+x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::log_base_2_1_plus_x_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::log_base_2_1_plus_x_prec_round_assign`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using [`Float::log_base_2_1_plus_x_assign`] instead.
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
    /// precision. (The result is exactly representable only when the input is `NaN`, infinite,
    /// zero, $-1$, less than $-1$, or a value for which $1+x$ is a power of 2 whose base-2
    /// logarithm is representable with the given precision.)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.log_base_2_1_plus_x_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "3.459431618637297256199363046725");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.log_base_2_1_plus_x_round_assign(Ceiling), Greater);
    /// assert_eq!(x.to_string(), "3.459431618637297256199363046728");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.log_base_2_1_plus_x_round_assign(Nearest), Less);
    /// assert_eq!(x.to_string(), "3.459431618637297256199363046725");
    /// ```
    #[inline]
    pub fn log_base_2_1_plus_x_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.log_base_2_1_plus_x_prec_round_assign(prec, rm)
    }
}

impl LogBase2Of1PlusX for Float {
    type Output = Self;

    /// Computes $\log_2(1+x)$, where $x$ is a [`Float`], taking the [`Float`] by value.
    ///
    /// If the output has a precision, it is the precision of the input. If the result is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $\log_2(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, `NaN` is returned.
    ///
    /// $$
    /// f(x) = \log_2(1+x)+\varepsilon.
    /// $$
    /// - If $\log_2(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\log_2(1+x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\log_2(1+x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=\infty$
    /// - $f(-\infty)=\text{NaN}$
    /// - $f(\pm0.0)=\pm0.0$
    /// - $f(-1)=-\infty$
    /// - $f(x)=\text{NaN}$ for $x<-1$
    /// - $f(x)=k$ when $1+x=2^k$. The result is the integer $k$ (subject to rounding at the input
    ///   precision $p$, and exact iff $k$ is representable with precision $p$). This covers $x$ a
    ///   power of 2 minus 1 (e.g. $x=1\to1$, $x=3\to2$) and negative $x$ such as $x=-1/2\to-1$ and
    ///   $x=-3/4\to-2$.
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::log_base_2_1_plus_x_round`] instead. If you want to specify the output precision,
    /// consider using [`Float::log_base_2_1_plus_x_prec`]. If you want both of these things,
    /// consider using [`Float::log_base_2_1_plus_x_prec_round`].
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
    /// use malachite_base::num::arithmetic::traits::LogBase2Of1PlusX;
    /// use malachite_base::num::basic::traits::{
    ///     Infinity, NaN, NegativeInfinity, NegativeOne, One,
    /// };
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.log_base_2_1_plus_x().is_nan());
    /// assert_eq!(Float::INFINITY.log_base_2_1_plus_x(), Float::INFINITY);
    /// assert!(Float::NEGATIVE_INFINITY.log_base_2_1_plus_x().is_nan());
    /// assert_eq!(Float::ONE.log_base_2_1_plus_x().to_string(), "1.0");
    /// assert_eq!(
    ///     Float::from_unsigned_prec(10u32, 100)
    ///         .0
    ///         .log_base_2_1_plus_x()
    ///         .to_string(),
    ///     "3.459431618637297256199363046725"
    /// );
    /// assert_eq!(
    ///     Float::NEGATIVE_ONE.log_base_2_1_plus_x(),
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert!(Float::from_signed_prec(-10, 100)
    ///     .0
    ///     .log_base_2_1_plus_x()
    ///     .is_nan());
    /// ```
    #[inline]
    fn log_base_2_1_plus_x(self) -> Self {
        let prec = self.significant_bits();
        self.log_base_2_1_plus_x_prec_round(prec, Nearest).0
    }
}

impl LogBase2Of1PlusX for &Float {
    type Output = Float;

    /// Computes $\log_2(1+x)$, where $x$ is a [`Float`], taking the [`Float`] by reference.
    ///
    /// If the output has a precision, it is the precision of the input. If the result is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $\log_2(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, `NaN` is returned.
    ///
    /// $$
    /// f(x) = \log_2(1+x)+\varepsilon.
    /// $$
    /// - If $\log_2(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\log_2(1+x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\log_2(1+x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=\infty$
    /// - $f(-\infty)=\text{NaN}$
    /// - $f(\pm0.0)=\pm0.0$
    /// - $f(-1)=-\infty$
    /// - $f(x)=\text{NaN}$ for $x<-1$
    /// - $f(x)=k$ when $1+x=2^k$. The result is the integer $k$ (subject to rounding at the input
    ///   precision $p$, and exact iff $k$ is representable with precision $p$). This covers $x$ a
    ///   power of 2 minus 1 (e.g. $x=1\to1$, $x=3\to2$) and negative $x$ such as $x=-1/2\to-1$ and
    ///   $x=-3/4\to-2$.
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::log_base_2_1_plus_x_round_ref`] instead. If you want to specify the output
    /// precision, consider using [`Float::log_base_2_1_plus_x_prec_ref`]. If you want both of these
    /// things, consider using [`Float::log_base_2_1_plus_x_prec_round_ref`].
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
    /// use malachite_base::num::arithmetic::traits::LogBase2Of1PlusX;
    /// use malachite_base::num::basic::traits::{
    ///     Infinity, NaN, NegativeInfinity, NegativeOne, One,
    /// };
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::NAN).log_base_2_1_plus_x().is_nan());
    /// assert_eq!((&Float::INFINITY).log_base_2_1_plus_x(), Float::INFINITY);
    /// assert!((&Float::NEGATIVE_INFINITY).log_base_2_1_plus_x().is_nan());
    /// assert_eq!((&Float::ONE).log_base_2_1_plus_x().to_string(), "1.0");
    /// assert_eq!(
    ///     (&Float::from_unsigned_prec(10u32, 100).0)
    ///         .log_base_2_1_plus_x()
    ///         .to_string(),
    ///     "3.459431618637297256199363046725"
    /// );
    /// assert_eq!(
    ///     (&Float::NEGATIVE_ONE).log_base_2_1_plus_x(),
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert!((&Float::from_signed_prec(-10, 100).0)
    ///     .log_base_2_1_plus_x()
    ///     .is_nan());
    /// ```
    #[inline]
    fn log_base_2_1_plus_x(self) -> Float {
        let prec = self.significant_bits();
        self.log_base_2_1_plus_x_prec_round_ref(prec, Nearest).0
    }
}

impl LogBase2Of1PlusXAssign for Float {
    /// Computes $\log_2(1+x)$, where $x$ is a [`Float`], in place.
    ///
    /// If the output has a precision, it is the precision of the input. If the result is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $\log_2(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, the [`Float`] is set to `NaN`.
    ///
    /// $$
    /// x \gets \log_2(1+x)+\varepsilon.
    /// $$
    /// - If $\log_2(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\log_2(1+x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\log_2(1+x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// See the [`Float::log_base_2_1_plus_x`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::log_base_2_1_plus_x_round_assign`] instead. If you want to specify the output
    /// precision, consider using [`Float::log_base_2_1_plus_x_prec_assign`]. If you want both of
    /// these things, consider using [`Float::log_base_2_1_plus_x_prec_round_assign`].
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
    /// use malachite_base::num::arithmetic::traits::LogBase2Of1PlusXAssign;
    /// use malachite_base::num::basic::traits::{
    ///     Infinity, NaN, NegativeInfinity, NegativeOne, One,
    /// };
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.log_base_2_1_plus_x_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x.log_base_2_1_plus_x_assign();
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x.log_base_2_1_plus_x_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::ONE;
    /// x.log_base_2_1_plus_x_assign();
    /// assert_eq!(x.to_string(), "1.0");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// x.log_base_2_1_plus_x_assign();
    /// assert_eq!(x.to_string(), "3.459431618637297256199363046725");
    ///
    /// let mut x = Float::NEGATIVE_ONE;
    /// x.log_base_2_1_plus_x_assign();
    /// assert_eq!(x, Float::NEGATIVE_INFINITY);
    ///
    /// let mut x = Float::from_signed_prec(-10, 100).0;
    /// x.log_base_2_1_plus_x_assign();
    /// assert!(x.is_nan());
    /// ```
    #[inline]
    fn log_base_2_1_plus_x_assign(&mut self) {
        let prec = self.significant_bits();
        self.log_base_2_1_plus_x_prec_round_assign(prec, Nearest);
    }
}
