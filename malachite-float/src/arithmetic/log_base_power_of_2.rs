// Copyright © 2026 Mikhail Hogrefe
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
    CeilingLogBase2, CheckedLogBase2, IsPowerOf2, LogBasePowerOf2, LogBasePowerOf2Assign, Sign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero as ZeroTrait;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::arithmetic::float_extras::float_can_round;
use malachite_nz::platform::Limb;
use malachite_q::Rational;

// The computation of log_base_power_of_2(x, pow) is done by log_{2^pow}(x) = log_2(x) / pow, where
// the input is finite, nonzero, and positive.
fn log_base_power_of_2_prec_round_normal(
    x: &Float,
    pow: i64,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    // If x is 1, the result is 0.
    if *x == 1u32 {
        return (Float::ZERO, Equal);
    }
    // If x is 2^m, then log_2(x) = m and the result is the rational m / pow (exact when
    // representable at the target precision).
    if x.is_power_of_2() {
        let m = i64::from(x.get_exponent().unwrap()) - 1;
        return Float::from(m).div_prec_round(Float::from(pow), prec, rm);
    }
    // The result is never exactly representable otherwise.
    assert_ne!(rm, Exact, "Inexact log_base_power_of_2");
    let mut working_prec = prec + 3 + prec.ceiling_log_base_2();
    let mut increment = Limb::WIDTH;
    loop {
        // log_2(x) / pow, with two correctly-rounded operations: log_base_2 (at most 1/2 ulp) and
        // division by the exact integer pow (at most 1/2 ulp). The relative error is thus below
        // 2^(1 - working_prec), so working_prec - 2 correct bits suffice for rounding.
        let t = x
            .log_base_2_prec_ref(working_prec)
            .0
            .div_prec(Float::from(pow), working_prec)
            .0;
        if float_can_round(t.significand_ref().unwrap(), working_prec - 2, prec, rm) {
            return Float::from_float_prec_round(t, prec, rm);
        }
        // Increase the precision.
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

// The computation of log_base_power_of_2_rational(x, pow) is done by log_{2^pow}(x) =
// log_2(x) / pow, where the input is a positive [`Rational`] that is not a power of 2. The
// base-2 logarithm of a [`Rational`] (computed by `log_base_2_rational_prec_ref`) already handles
// inputs that are extremely close to a power of 2 without needing extra precision, so a simple Ziv
// loop dividing by the exact integer pow suffices here.
fn log_base_power_of_2_rational_prec_round_helper(
    x: &Rational,
    pow: i64,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    let mut working_prec = prec + 3 + prec.ceiling_log_base_2();
    let mut increment = Limb::WIDTH;
    loop {
        // log_2(x) / pow, with two correctly-rounded operations: log_base_2_rational (at most 1/2
        // ulp) and division by the exact integer pow (at most 1/2 ulp). The relative error is thus
        // below 2^(1 - working_prec), so working_prec - 2 correct bits suffice for rounding.
        let t = Float::log_base_2_rational_prec_ref(x, working_prec)
            .0
            .div_prec(Float::from(pow), working_prec)
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
    /// Computes $\log_{2^k} x$, where $x$ is a [`Float`] and the base is $2^k$ for some nonzero
    /// integer $k$, rounding the result to the specified precision and with the specified rounding
    /// mode. The base's exponent $k$ is `pow`, which may be negative. The [`Float`] is taken by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded value is less than,
    /// equal to, or greater than the exact value. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The base-$2^k$ logarithm of any nonzero negative number is `NaN`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,k,p,m) = \log_{2^k} x+\varepsilon.
    /// $$
    /// - If $\log_{2^k} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\log_{2^k} x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_{2^k} x|\rfloor-p+1}$.
    /// - If $\log_{2^k} x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_{2^k} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},k,p,m)=\text{NaN}$
    /// - $f(\infty,k,p,m)=\infty$ if $k>0$, and $-\infty$ if $k<0$
    /// - $f(-\infty,k,p,m)=\text{NaN}$
    /// - $f(\pm0.0,k,p,m)=-\infty$ if $k>0$, and $\infty$ if $k<0$
    /// - $f(1.0,k,p,m)=0.0$, and the result is exact
    /// - $f(2^m,k,p,m')=m/k$, rounded to precision $p$; the result is exact if and only if $m/k$ is
    ///   representable with precision $p$ (for example $\log_4 8=3/2$ is exact, but $\log_8 4=2/3$
    ///   is not)
    /// - $f(x,k,p,m)=\text{NaN}$ for $x<0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::log_base_power_of_2_prec`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::log_base_power_of_2_round`] instead. If both of these things are true,
    /// consider using [`Float::log_base_power_of_2`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, if `pow` is zero (the base $2^0=1$ has no logarithm), or if `rm`
    /// is `Exact` but the result cannot be represented exactly with the given precision. (The
    /// result is exactly representable if and only if the input is `NaN`, infinite, zero, equal to
    /// 1, or a power of 2 whose base-$2^k$ logarithm is representable with the given precision.)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_power_of_2_prec_round(2, 5, Floor);
    /// assert_eq!(log.to_string(), "1.62");
    /// assert_eq!(o, Less);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_power_of_2_prec_round(2, 5, Ceiling);
    /// assert_eq!(log.to_string(), "1.7");
    /// assert_eq!(o, Greater);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_power_of_2_prec_round(2, 5, Nearest);
    /// assert_eq!(log.to_string(), "1.7");
    /// assert_eq!(o, Greater);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_power_of_2_prec_round(3, 20, Floor);
    /// assert_eq!(log.to_string(), "1.107309");
    /// assert_eq!(o, Less);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_power_of_2_prec_round(3, 20, Ceiling);
    /// assert_eq!(log.to_string(), "1.107311");
    /// assert_eq!(o, Greater);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_power_of_2_prec_round(3, 20, Nearest);
    /// assert_eq!(log.to_string(), "1.107309");
    /// assert_eq!(o, Less);
    ///
    /// // log_4(8) = 3/2, exactly representable
    /// let (log, o) = Float::from(8u32).log_base_power_of_2_prec_round(2, 10, Nearest);
    /// assert_eq!(log.to_string(), "1.5");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_power_of_2_prec_round(
        self,
        pow: i64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        assert_ne!(pow, 0, "Cannot take base-1 logarithm");
        match self {
            Self(NaN | Infinity { sign: false } | Finite { sign: false, .. }) => {
                (float_nan!(), Equal)
            }
            float_either_zero!() => (
                if pow > 0 {
                    float_negative_infinity!()
                } else {
                    float_infinity!()
                },
                Equal,
            ),
            float_infinity!() => (
                if pow > 0 {
                    float_infinity!()
                } else {
                    float_negative_infinity!()
                },
                Equal,
            ),
            _ => log_base_power_of_2_prec_round_normal(&self, pow, prec, rm),
        }
    }

    /// Computes $\log_{2^k} x$, where $x$ is a [`Float`] and the base is $2^k$ for some nonzero
    /// integer $k$, rounding the result to the specified precision and with the specified rounding
    /// mode. The base's exponent $k$ is `pow`, which may be negative. The [`Float`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded value is less
    /// than, equal to, or greater than the exact value. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The base-$2^k$ logarithm of any nonzero negative number is `NaN`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,k,p,m) = \log_{2^k} x+\varepsilon.
    /// $$
    /// - If $\log_{2^k} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\log_{2^k} x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_{2^k} x|\rfloor-p+1}$.
    /// - If $\log_{2^k} x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_{2^k} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},k,p,m)=\text{NaN}$
    /// - $f(\infty,k,p,m)=\infty$ if $k>0$, and $-\infty$ if $k<0$
    /// - $f(-\infty,k,p,m)=\text{NaN}$
    /// - $f(\pm0.0,k,p,m)=-\infty$ if $k>0$, and $\infty$ if $k<0$
    /// - $f(1.0,k,p,m)=0.0$, and the result is exact
    /// - $f(2^m,k,p,m')=m/k$, rounded to precision $p$; the result is exact if and only if $m/k$ is
    ///   representable with precision $p$ (for example $\log_4 8=3/2$ is exact, but $\log_8 4=2/3$
    ///   is not)
    /// - $f(x,k,p,m)=\text{NaN}$ for $x<0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::log_base_power_of_2_prec_ref`] instead. If you know that your target precision is
    /// the precision of the input, consider using [`Float::log_base_power_of_2_round_ref`] instead.
    /// If both of these things are true, consider using `(&Float).log_base_power_of_2()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, if `pow` is zero (the base $2^0=1$ has no logarithm), or if `rm`
    /// is `Exact` but the result cannot be represented exactly with the given precision. (The
    /// result is exactly representable if and only if the input is `NaN`, infinite, zero, equal to
    /// 1, or a power of 2 whose base-$2^k$ logarithm is representable with the given precision.)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_power_of_2_prec_round_ref(2, 5, Floor);
    /// assert_eq!(log.to_string(), "1.62");
    /// assert_eq!(o, Less);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_power_of_2_prec_round_ref(2, 5, Ceiling);
    /// assert_eq!(log.to_string(), "1.7");
    /// assert_eq!(o, Greater);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_power_of_2_prec_round_ref(2, 5, Nearest);
    /// assert_eq!(log.to_string(), "1.7");
    /// assert_eq!(o, Greater);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_power_of_2_prec_round_ref(3, 20, Floor);
    /// assert_eq!(log.to_string(), "1.107309");
    /// assert_eq!(o, Less);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_power_of_2_prec_round_ref(3, 20, Ceiling);
    /// assert_eq!(log.to_string(), "1.107311");
    /// assert_eq!(o, Greater);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_power_of_2_prec_round_ref(3, 20, Nearest);
    /// assert_eq!(log.to_string(), "1.107309");
    /// assert_eq!(o, Less);
    ///
    /// // log_4(8) = 3/2, exactly representable
    /// let (log, o) = Float::from(8u32).log_base_power_of_2_prec_round_ref(2, 10, Nearest);
    /// assert_eq!(log.to_string(), "1.5");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_power_of_2_prec_round_ref(
        &self,
        pow: i64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        assert_ne!(pow, 0, "Cannot take base-1 logarithm");
        match self {
            Self(NaN | Infinity { sign: false } | Finite { sign: false, .. }) => {
                (float_nan!(), Equal)
            }
            float_either_zero!() => (
                if pow > 0 {
                    float_negative_infinity!()
                } else {
                    float_infinity!()
                },
                Equal,
            ),
            float_infinity!() => (
                if pow > 0 {
                    float_infinity!()
                } else {
                    float_negative_infinity!()
                },
                Equal,
            ),
            _ => log_base_power_of_2_prec_round_normal(self, pow, prec, rm),
        }
    }

    /// Computes $\log_{2^k} x$, where $x$ is a [`Float`] and the base is $2^k$ for some nonzero
    /// integer $k$, rounding the result to the nearest value of the specified precision. The base's
    /// exponent $k$ is `pow`, which may be negative. The [`Float`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded value is less than, equal to,
    /// or greater than the exact value. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The base-$2^k$ logarithm of any nonzero negative number is `NaN`.
    ///
    /// If the logarithm is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,k,p) = \log_{2^k} x+\varepsilon.
    /// $$
    /// - If $\log_{2^k} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\log_{2^k} x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\log_{2^k} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},k,p)=\text{NaN}$
    /// - $f(\infty,k,p)=\infty$ if $k>0$, and $-\infty$ if $k<0$
    /// - $f(-\infty,k,p)=\text{NaN}$
    /// - $f(\pm0.0,k,p)=-\infty$ if $k>0$, and $\infty$ if $k<0$
    /// - $f(1.0,k,p)=0.0$, and the result is exact
    /// - $f(2^m,k,p)=m/k$, rounded to precision $p$; the result is exact if and only if $m/k$ is
    ///   representable with precision $p$ (for example $\log_4 8=3/2$ is exact, but $\log_8 4=2/3$
    ///   is not)
    /// - $f(x,k,p)=\text{NaN}$ for $x<0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::log_base_power_of_2_prec_round`] instead. If you know that your target precision is
    /// the precision of the input, consider using [`Float::log_base_power_of_2`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `pow` is zero (the base $2^0=1$ has no logarithm).
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100).0.log_base_power_of_2_prec(2, 5);
    /// assert_eq!(log.to_string(), "1.7");
    /// assert_eq!(o, Greater);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100).0.log_base_power_of_2_prec(3, 20);
    /// assert_eq!(log.to_string(), "1.107309");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn log_base_power_of_2_prec(self, pow: i64, prec: u64) -> (Self, Ordering) {
        self.log_base_power_of_2_prec_round(pow, prec, Nearest)
    }

    /// Computes $\log_{2^k} x$, where $x$ is a [`Float`] and the base is $2^k$ for some nonzero
    /// integer $k$, rounding the result to the nearest value of the specified precision. The base's
    /// exponent $k$ is `pow`, which may be negative. The [`Float`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded value is less than, equal to,
    /// or greater than the exact value. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The base-$2^k$ logarithm of any nonzero negative number is `NaN`.
    ///
    /// If the logarithm is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,k,p) = \log_{2^k} x+\varepsilon.
    /// $$
    /// - If $\log_{2^k} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\log_{2^k} x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\log_{2^k} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},k,p)=\text{NaN}$
    /// - $f(\infty,k,p)=\infty$ if $k>0$, and $-\infty$ if $k<0$
    /// - $f(-\infty,k,p)=\text{NaN}$
    /// - $f(\pm0.0,k,p)=-\infty$ if $k>0$, and $\infty$ if $k<0$
    /// - $f(1.0,k,p)=0.0$, and the result is exact
    /// - $f(2^m,k,p)=m/k$, rounded to precision $p$; the result is exact if and only if $m/k$ is
    ///   representable with precision $p$ (for example $\log_4 8=3/2$ is exact, but $\log_8 4=2/3$
    ///   is not)
    /// - $f(x,k,p)=\text{NaN}$ for $x<0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::log_base_power_of_2_prec_round_ref`] instead. If you know that your target
    /// precision is the precision of the input, consider using `(&Float).log_base_power_of_2()`
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
    /// Panics if `prec` is zero or if `pow` is zero (the base $2^0=1$ has no logarithm).
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_power_of_2_prec_ref(2, 5);
    /// assert_eq!(log.to_string(), "1.7");
    /// assert_eq!(o, Greater);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_power_of_2_prec_ref(3, 20);
    /// assert_eq!(log.to_string(), "1.107309");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn log_base_power_of_2_prec_ref(&self, pow: i64, prec: u64) -> (Self, Ordering) {
        self.log_base_power_of_2_prec_round_ref(pow, prec, Nearest)
    }

    /// Computes $\log_{2^k} x$, where $x$ is a [`Float`] and the base is $2^k$ for some nonzero
    /// integer $k$, rounding the result with the specified rounding mode. The base's exponent $k$
    /// is `pow`, which may be negative. The [`Float`] is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The base-$2^k$ logarithm of any nonzero negative number is `NaN`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,k,m) = \log_{2^k} x+\varepsilon.
    /// $$
    /// - If $\log_{2^k} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\log_{2^k} x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_{2^k} x|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $\log_{2^k} x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_{2^k} x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},k,m)=\text{NaN}$
    /// - $f(\infty,k,m)=\infty$ if $k>0$, and $-\infty$ if $k<0$
    /// - $f(-\infty,k,m)=\text{NaN}$
    /// - $f(\pm0.0,k,m)=-\infty$ if $k>0$, and $\infty$ if $k<0$
    /// - $f(1.0,k,m)=0.0$, and the result is exact
    /// - $f(2^m,k,m')=m/k$, rounded to the precision of the input; the result is exact if and only
    ///   if $m/k$ is representable with that precision (for example $\log_4 8=3/2$ is exact, but
    ///   $\log_8 4=2/3$ is not)
    /// - $f(x,k,m)=\text{NaN}$ for $x<0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::log_base_power_of_2_prec_round`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using [`Float::log_base_power_of_2`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
    ///
    /// # Panics
    /// Panics if `pow` is zero (the base $2^0=1$ has no logarithm), or if `rm` is `Exact` but the
    /// result cannot be represented exactly with the input precision. (The result is exactly
    /// representable if and only if the input is `NaN`, infinite, zero, equal to 1, or a power of 2
    /// whose base-$2^k$ logarithm is representable with the input precision.)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_power_of_2_round(2, Floor);
    /// assert_eq!(log.to_string(), "1.660964047443681173935159714743");
    /// assert_eq!(o, Less);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_power_of_2_round(2, Ceiling);
    /// assert_eq!(log.to_string(), "1.660964047443681173935159714745");
    /// assert_eq!(o, Greater);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_power_of_2_round(2, Nearest);
    /// assert_eq!(log.to_string(), "1.660964047443681173935159714745");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_power_of_2_round(self, pow: i64, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.log_base_power_of_2_prec_round(pow, prec, rm)
    }

    /// Computes $\log_{2^k} x$, where $x$ is a [`Float`] and the base is $2^k$ for some nonzero
    /// integer $k$, rounding the result with the specified rounding mode. The base's exponent $k$
    /// is `pow`, which may be negative. The [`Float`] is taken by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded value is less than, equal to, or greater than
    /// the exact value. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The base-$2^k$ logarithm of any nonzero negative number is `NaN`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,k,m) = \log_{2^k} x+\varepsilon.
    /// $$
    /// - If $\log_{2^k} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\log_{2^k} x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_{2^k} x|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $\log_{2^k} x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_{2^k} x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},k,m)=\text{NaN}$
    /// - $f(\infty,k,m)=\infty$ if $k>0$, and $-\infty$ if $k<0$
    /// - $f(-\infty,k,m)=\text{NaN}$
    /// - $f(\pm0.0,k,m)=-\infty$ if $k>0$, and $\infty$ if $k<0$
    /// - $f(1.0,k,m)=0.0$, and the result is exact
    /// - $f(2^m,k,m')=m/k$, rounded to the precision of the input; the result is exact if and only
    ///   if $m/k$ is representable with that precision (for example $\log_4 8=3/2$ is exact, but
    ///   $\log_8 4=2/3$ is not)
    /// - $f(x,k,m)=\text{NaN}$ for $x<0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::log_base_power_of_2_prec_round_ref`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using `(&Float).log_base_power_of_2()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
    ///
    /// # Panics
    /// Panics if `pow` is zero (the base $2^0=1$ has no logarithm), or if `rm` is `Exact` but the
    /// result cannot be represented exactly with the input precision. (The result is exactly
    /// representable if and only if the input is `NaN`, infinite, zero, equal to 1, or a power of 2
    /// whose base-$2^k$ logarithm is representable with the input precision.)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_power_of_2_round_ref(2, Floor);
    /// assert_eq!(log.to_string(), "1.660964047443681173935159714743");
    /// assert_eq!(o, Less);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_power_of_2_round_ref(2, Ceiling);
    /// assert_eq!(log.to_string(), "1.660964047443681173935159714745");
    /// assert_eq!(o, Greater);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_power_of_2_round_ref(2, Nearest);
    /// assert_eq!(log.to_string(), "1.660964047443681173935159714745");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_power_of_2_round_ref(&self, pow: i64, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.log_base_power_of_2_prec_round_ref(pow, prec, rm)
    }

    /// Computes $\log_{2^k} x$, where $x$ is a [`Float`] and the base is $2^k$ for some nonzero
    /// integer $k$, in place, rounding the result to the specified precision and with the specified
    /// rounding mode. The base's exponent $k$ is `pow`, which may be negative. An [`Ordering`] is
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// The base-$2^k$ logarithm of any nonzero negative number is `NaN`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \log_{2^k} x+\varepsilon.
    /// $$
    /// - If $\log_{2^k} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\log_{2^k} x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_{2^k} x|\rfloor-p+1}$.
    /// - If $\log_{2^k} x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_{2^k} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::log_base_power_of_2_prec_round`] documentation for information on special
    /// cases, overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::log_base_power_of_2_prec_assign`] instead. If you know that your target precision
    /// is the precision of the input, consider using [`Float::log_base_power_of_2_round_assign`]
    /// instead. If both of these things are true, consider using
    /// [`Float::log_base_power_of_2_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, if `pow` is zero (the base $2^0=1$ has no logarithm), or if `rm`
    /// is `Exact` but the result cannot be represented exactly with the given precision. (The
    /// result is exactly representable if and only if the input is `NaN`, infinite, zero, equal to
    /// 1, or a power of 2 whose base-$2^k$ logarithm is representable with the given precision.)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.log_base_power_of_2_prec_round_assign(2, 5, Floor), Less);
    /// assert_eq!(x.to_string(), "1.62");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.log_base_power_of_2_prec_round_assign(2, 5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.7");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.log_base_power_of_2_prec_round_assign(2, 5, Nearest), Greater);
    /// assert_eq!(x.to_string(), "1.7");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.log_base_power_of_2_prec_round_assign(3, 20, Floor), Less);
    /// assert_eq!(x.to_string(), "1.107309");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.log_base_power_of_2_prec_round_assign(3, 20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.107311");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.log_base_power_of_2_prec_round_assign(3, 20, Nearest), Less);
    /// assert_eq!(x.to_string(), "1.107309");
    /// ```
    #[inline]
    pub fn log_base_power_of_2_prec_round_assign(
        &mut self,
        pow: i64,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (result, o) = core::mem::take(self).log_base_power_of_2_prec_round(pow, prec, rm);
        *self = result;
        o
    }

    /// Computes $\log_{2^k} x$, where $x$ is a [`Float`] and the base is $2^k$ for some nonzero
    /// integer $k$, in place, rounding the result to the nearest value of the specified precision.
    /// The base's exponent $k$ is `pow`, which may be negative. An [`Ordering`] is returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`Float`], whenever this function sets the
    /// [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// The base-$2^k$ logarithm of any nonzero negative number is `NaN`.
    ///
    /// If the logarithm is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \log_{2^k} x+\varepsilon.
    /// $$
    /// - If $\log_{2^k} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\log_{2^k} x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\log_{2^k} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::log_base_power_of_2_prec`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::log_base_power_of_2_prec_round_assign`] instead. If you know that your target
    /// precision is the precision of the input, consider using
    /// [`Float::log_base_power_of_2_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `pow` is zero (the base $2^0=1$ has no logarithm).
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.log_base_power_of_2_prec_assign(2, 5), Greater);
    /// assert_eq!(x.to_string(), "1.7");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.log_base_power_of_2_prec_assign(3, 20), Less);
    /// assert_eq!(x.to_string(), "1.107309");
    /// ```
    #[inline]
    pub fn log_base_power_of_2_prec_assign(&mut self, pow: i64, prec: u64) -> Ordering {
        self.log_base_power_of_2_prec_round_assign(pow, prec, Nearest)
    }

    /// Computes $\log_{2^k} x$, where $x$ is a [`Float`] and the base is $2^k$ for some nonzero
    /// integer $k$, in place, rounding the result with the specified rounding mode. The base's
    /// exponent $k$ is `pow`, which may be negative. An [`Ordering`] is returned, indicating
    /// whether the rounded value is less than, equal to, or greater than the exact value. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function sets the [`Float`] to
    /// `NaN` it also returns `Equal`.
    ///
    /// The base-$2^k$ logarithm of any nonzero negative number is `NaN`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \log_{2^k} x+\varepsilon.
    /// $$
    /// - If $\log_{2^k} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\log_{2^k} x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_{2^k} x|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $\log_{2^k} x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_{2^k} x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::log_base_power_of_2_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::log_base_power_of_2_prec_round_assign`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using [`Float::log_base_power_of_2_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
    ///
    /// # Panics
    /// Panics if `pow` is zero (the base $2^0=1$ has no logarithm), or if `rm` is `Exact` but the
    /// result cannot be represented exactly with the input precision. (The result is exactly
    /// representable if and only if the input is `NaN`, infinite, zero, equal to 1, or a power of 2
    /// whose base-$2^k$ logarithm is representable with the input precision.)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.log_base_power_of_2_round_assign(2, Floor), Less);
    /// assert_eq!(x.to_string(), "1.660964047443681173935159714743");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.log_base_power_of_2_round_assign(2, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.660964047443681173935159714745");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.log_base_power_of_2_round_assign(2, Nearest), Greater);
    /// assert_eq!(x.to_string(), "1.660964047443681173935159714745");
    /// ```
    #[inline]
    pub fn log_base_power_of_2_round_assign(&mut self, pow: i64, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.log_base_power_of_2_prec_round_assign(pow, prec, rm)
    }

    /// Computes $\log_{2^k} x$, where $x$ is a [`Rational`] and the base is $2^k$ for some nonzero
    /// integer $k$, rounding the result to the specified precision and with the specified rounding
    /// mode and returning the result as a [`Float`]. The base's exponent $k$ is `pow`, which may be
    /// negative. The [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded value is less than, equal to, or greater than the exact value. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The base-$2^k$ logarithm of any negative number is `NaN`.
    ///
    /// Inputs of any magnitude are handled, including [`Rational`]s whose magnitudes are too large
    /// or too small to be representable as [`Float`]s.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,k,p,m) = \log_{2^k} x+\varepsilon.
    /// $$
    /// - If $\log_{2^k} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\log_{2^k} x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_{2^k} x|\rfloor-p+1}$.
    /// - If $\log_{2^k} x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_{2^k} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,k,p,m)=-\infty$ if $k>0$, and $\infty$ if $k<0$
    /// - $f(x,k,p,m)=\text{NaN}$ for $x<0$
    /// - $f(1,k,p,m)=0.0$, and the result is exact
    /// - $f(2^m,k,p,m')=m/k$, rounded to precision $p$; the result is exact if and only if $m/k$ is
    ///   representable with precision $p$ (for example $\log_4 8=3/2$ is exact, but $\log_8 4=2/3$
    ///   is not). This includes negative powers of 2 like $1/4$, and powers of 2 whose exponents
    ///   $m$ lie far outside the exponent range of [`Float`].
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::log_base_power_of_2_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, if `pow` is zero (the base $2^0=1$ has no logarithm), or if `rm`
    /// is `Exact` but the result cannot be represented exactly with the given precision. (The
    /// result is exactly representable if and only if $x\leq 0$ or $x$ is a power of 2 whose
    /// base-$2^k$ logarithm is representable with the given precision.)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::log_base_power_of_2_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     2,
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(log.to_string(), "-0.3684831");
    /// assert_eq!(o, Less);
    ///
    /// let (log, o) = Float::log_base_power_of_2_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     2,
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(log.to_string(), "-0.3684826");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn log_base_power_of_2_rational_prec_round(
        x: Rational,
        pow: i64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::log_base_power_of_2_rational_prec_round_ref(&x, pow, prec, rm)
    }

    /// Computes $\log_{2^k} x$, where $x$ is a [`Rational`] and the base is $2^k$ for some nonzero
    /// integer $k$, rounding the result to the specified precision and with the specified rounding
    /// mode and returning the result as a [`Float`]. The base's exponent $k$ is `pow`, which may be
    /// negative. The [`Rational`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// The base-$2^k$ logarithm of any negative number is `NaN`.
    ///
    /// Inputs of any magnitude are handled, including [`Rational`]s whose magnitudes are too large
    /// or too small to be representable as [`Float`]s.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,k,p,m) = \log_{2^k} x+\varepsilon.
    /// $$
    /// - If $\log_{2^k} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\log_{2^k} x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_{2^k} x|\rfloor-p+1}$.
    /// - If $\log_{2^k} x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_{2^k} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,k,p,m)=-\infty$ if $k>0$, and $\infty$ if $k<0$
    /// - $f(x,k,p,m)=\text{NaN}$ for $x<0$
    /// - $f(1,k,p,m)=0.0$, and the result is exact
    /// - $f(2^m,k,p,m')=m/k$, rounded to precision $p$; the result is exact if and only if $m/k$ is
    ///   representable with precision $p$ (for example $\log_4 8=3/2$ is exact, but $\log_8 4=2/3$
    ///   is not). This includes negative powers of 2 like $1/4$, and powers of 2 whose exponents
    ///   $m$ lie far outside the exponent range of [`Float`].
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::log_base_power_of_2_rational_prec_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, if `pow` is zero (the base $2^0=1$ has no logarithm), or if `rm`
    /// is `Exact` but the result cannot be represented exactly with the given precision. (The
    /// result is exactly representable if and only if $x\leq 0$ or $x$ is a power of 2 whose
    /// base-$2^k$ logarithm is representable with the given precision.)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::log_base_power_of_2_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     2,
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(log.to_string(), "-0.3684831");
    /// assert_eq!(o, Less);
    ///
    /// let (log, o) = Float::log_base_power_of_2_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     2,
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(log.to_string(), "-0.3684826");
    /// assert_eq!(o, Greater);
    ///
    /// // log_4(8) = 3/2, exactly representable
    /// let (log, o) = Float::log_base_power_of_2_rational_prec_round_ref(
    ///     &Rational::from(8u32),
    ///     2,
    ///     10,
    ///     Nearest,
    /// );
    /// assert_eq!(log.to_string(), "1.5");
    /// assert_eq!(o, Equal);
    /// ```
    pub fn log_base_power_of_2_rational_prec_round_ref(
        x: &Rational,
        pow: i64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        assert_ne!(pow, 0, "Cannot take base-1 logarithm");
        match x.sign() {
            Equal => {
                return (
                    if pow > 0 {
                        float_negative_infinity!()
                    } else {
                        float_infinity!()
                    },
                    Equal,
                );
            }
            Less => return (float_nan!(), Equal),
            Greater => {}
        }
        // If x is 2^m, then log_2(x) = m and the result is the rational m / pow (exact when
        // representable at the target precision).
        if let Some(m) = x.checked_log_base_2() {
            return Self::from(m).div_prec_round(Self::from(pow), prec, rm);
        }
        // The result is never exactly representable otherwise.
        assert_ne!(rm, Exact, "Inexact log_base_power_of_2");
        log_base_power_of_2_rational_prec_round_helper(x, pow, prec, rm)
    }

    /// Computes $\log_{2^k} x$, where $x$ is a [`Rational`] and the base is $2^k$ for some nonzero
    /// integer $k$, rounding the result to the nearest value of the specified precision and
    /// returning the result as a [`Float`]. The base's exponent $k$ is `pow`, which may be
    /// negative. The [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded value is less than, equal to, or greater than the exact value. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The base-$2^k$ logarithm of any negative number is `NaN`.
    ///
    /// Inputs of any magnitude are handled, including [`Rational`]s whose magnitudes are too large
    /// or too small to be representable as [`Float`]s.
    ///
    /// If the logarithm is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,k,p) = \log_{2^k} x+\varepsilon.
    /// $$
    /// - If $\log_{2^k} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\log_{2^k} x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\log_{2^k} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,k,p)=-\infty$ if $k>0$, and $\infty$ if $k<0$
    /// - $f(x,k,p)=\text{NaN}$ for $x<0$
    /// - $f(1,k,p)=0.0$, and the result is exact
    /// - $f(2^m,k,p)=m/k$, rounded to precision $p$; the result is exact if and only if $m/k$ is
    ///   representable with precision $p$ (for example $\log_4 8=3/2$ is exact, but $\log_8 4=2/3$
    ///   is not). This includes negative powers of 2 like $1/4$, and powers of 2 whose exponents
    ///   $m$ lie far outside the exponent range of [`Float`].
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::log_base_power_of_2_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `pow` is zero (the base $2^0=1$ has no logarithm).
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) =
    ///     Float::log_base_power_of_2_rational_prec(Rational::from_unsigneds(3u8, 5), 2, 20);
    /// assert_eq!(log.to_string(), "-0.3684826");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_power_of_2_rational_prec(x: Rational, pow: i64, prec: u64) -> (Self, Ordering) {
        Self::log_base_power_of_2_rational_prec_round(x, pow, prec, Nearest)
    }

    /// Computes $\log_{2^k} x$, where $x$ is a [`Rational`] and the base is $2^k$ for some nonzero
    /// integer $k$, rounding the result to the nearest value of the specified precision and
    /// returning the result as a [`Float`]. The base's exponent $k$ is `pow`, which may be
    /// negative. The [`Rational`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// The base-$2^k$ logarithm of any negative number is `NaN`.
    ///
    /// Inputs of any magnitude are handled, including [`Rational`]s whose magnitudes are too large
    /// or too small to be representable as [`Float`]s.
    ///
    /// If the logarithm is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,k,p) = \log_{2^k} x+\varepsilon.
    /// $$
    /// - If $\log_{2^k} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\log_{2^k} x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\log_{2^k} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,k,p)=-\infty$ if $k>0$, and $\infty$ if $k<0$
    /// - $f(x,k,p)=\text{NaN}$ for $x<0$
    /// - $f(1,k,p)=0.0$, and the result is exact
    /// - $f(2^m,k,p)=m/k$, rounded to precision $p$; the result is exact if and only if $m/k$ is
    ///   representable with precision $p$ (for example $\log_4 8=3/2$ is exact, but $\log_8 4=2/3$
    ///   is not). This includes negative powers of 2 like $1/4$, and powers of 2 whose exponents
    ///   $m$ lie far outside the exponent range of [`Float`].
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::log_base_power_of_2_rational_prec_round_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `pow` is zero (the base $2^0=1$ has no logarithm).
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) =
    ///     Float::log_base_power_of_2_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 2, 20);
    /// assert_eq!(log.to_string(), "-0.3684826");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_power_of_2_rational_prec_ref(
        x: &Rational,
        pow: i64,
        prec: u64,
    ) -> (Self, Ordering) {
        Self::log_base_power_of_2_rational_prec_round_ref(x, pow, prec, Nearest)
    }
}

impl LogBasePowerOf2<i64> for Float {
    type Output = Self;

    /// Computes $\log_{2^k} x$, where $x$ is a [`Float`] and the base is $2^k$ for some nonzero
    /// integer $k$, taking it by value. The base's exponent $k$ is `pow`, which may be negative.
    ///
    /// If the output has a precision, it is the precision of the input. If the logarithm is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// The base-$2^k$ logarithm of any nonzero negative number is `NaN`.
    ///
    /// $$
    /// f(x,k) = \log_{2^k} x+\varepsilon.
    /// $$
    /// - If $\log_{2^k} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\log_{2^k} x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\log_{2^k} x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},k)=\text{NaN}$
    /// - $f(\infty,k)=\infty$ if $k>0$, and $-\infty$ if $k<0$
    /// - $f(-\infty,k)=\text{NaN}$
    /// - $f(\pm0.0,k)=-\infty$ if $k>0$, and $\infty$ if $k<0$
    /// - $f(1.0,k)=0.0$, and the result is exact
    /// - $f(2^m,k)=m/k$, rounded to the precision of the input; the result is exact if and only if
    ///   $m/k$ is representable with that precision (for example $\log_4 8=3/2$ is exact, but
    ///   $\log_8 4=2/3$ is not)
    /// - $f(x,k)=\text{NaN}$ for $x<0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::log_base_power_of_2_round`] instead. If you want to specify the output precision,
    /// consider using [`Float::log_base_power_of_2_prec`]. If you want both of these things,
    /// consider using [`Float::log_base_power_of_2_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
    ///
    /// # Panics
    /// Panics if `pow` is zero (the base $2^0=1$ has no logarithm).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::LogBasePowerOf2;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.log_base_power_of_2(2).is_nan());
    /// assert_eq!(Float::INFINITY.log_base_power_of_2(2), Float::INFINITY);
    /// assert_eq!(
    ///     Float::INFINITY.log_base_power_of_2(-2),
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert!(Float::NEGATIVE_INFINITY.log_base_power_of_2(2).is_nan());
    /// assert_eq!(
    ///     Float::from_unsigned_prec(10u32, 100)
    ///         .0
    ///         .log_base_power_of_2(2)
    ///         .to_string(),
    ///     "1.660964047443681173935159714745"
    /// );
    /// assert!(
    ///     Float::from_signed_prec(-10, 100)
    ///         .0
    ///         .log_base_power_of_2(2)
    ///         .is_nan()
    /// );
    /// ```
    #[inline]
    fn log_base_power_of_2(self, pow: i64) -> Self {
        let prec = self.significant_bits();
        self.log_base_power_of_2_prec_round(pow, prec, Nearest).0
    }
}

impl LogBasePowerOf2<i64> for &Float {
    type Output = Float;

    /// Computes $\log_{2^k} x$, where $x$ is a [`Float`] and the base is $2^k$ for some nonzero
    /// integer $k$, taking it by reference. The base's exponent $k$ is `pow`, which may be
    /// negative.
    ///
    /// If the output has a precision, it is the precision of the input. If the logarithm is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// The base-$2^k$ logarithm of any nonzero negative number is `NaN`.
    ///
    /// $$
    /// f(x,k) = \log_{2^k} x+\varepsilon.
    /// $$
    /// - If $\log_{2^k} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\log_{2^k} x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\log_{2^k} x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},k)=\text{NaN}$
    /// - $f(\infty,k)=\infty$ if $k>0$, and $-\infty$ if $k<0$
    /// - $f(-\infty,k)=\text{NaN}$
    /// - $f(\pm0.0,k)=-\infty$ if $k>0$, and $\infty$ if $k<0$
    /// - $f(1.0,k)=0.0$, and the result is exact
    /// - $f(2^m,k)=m/k$, rounded to the precision of the input; the result is exact if and only if
    ///   $m/k$ is representable with that precision (for example $\log_4 8=3/2$ is exact, but
    ///   $\log_8 4=2/3$ is not)
    /// - $f(x,k)=\text{NaN}$ for $x<0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::log_base_power_of_2_round_ref`] instead. If you want to specify the output
    /// precision, consider using [`Float::log_base_power_of_2_prec_ref`]. If you want both of these
    /// things, consider using [`Float::log_base_power_of_2_prec_round_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
    ///
    /// # Panics
    /// Panics if `pow` is zero (the base $2^0=1$ has no logarithm).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::LogBasePowerOf2;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::NAN).log_base_power_of_2(2).is_nan());
    /// assert_eq!((&Float::INFINITY).log_base_power_of_2(2), Float::INFINITY);
    /// assert_eq!(
    ///     (&Float::INFINITY).log_base_power_of_2(-2),
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert!((&Float::NEGATIVE_INFINITY).log_base_power_of_2(2).is_nan());
    /// assert_eq!(
    ///     (&Float::from_unsigned_prec(10u32, 100).0)
    ///         .log_base_power_of_2(2)
    ///         .to_string(),
    ///     "1.660964047443681173935159714745"
    /// );
    /// assert!(
    ///     (&Float::from_signed_prec(-10, 100).0)
    ///         .log_base_power_of_2(2)
    ///         .is_nan()
    /// );
    /// ```
    #[inline]
    fn log_base_power_of_2(self, pow: i64) -> Float {
        let prec = self.significant_bits();
        self.log_base_power_of_2_prec_round_ref(pow, prec, Nearest)
            .0
    }
}

impl LogBasePowerOf2Assign<i64> for Float {
    /// Computes $\log_{2^k} x$, where $x$ is a [`Float`] and the base is $2^k$ for some nonzero
    /// integer $k$, in place. The base's exponent $k$ is `pow`, which may be negative.
    ///
    /// If the output has a precision, it is the precision of the input. If the logarithm is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// The base-$2^k$ logarithm of any nonzero negative number is `NaN`.
    ///
    /// $$
    /// x \gets \log_{2^k} x+\varepsilon.
    /// $$
    /// - If $\log_{2^k} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\log_{2^k} x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\log_{2^k} x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// See the [`Float::log_base_power_of_2`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::log_base_power_of_2_round_assign`] instead. If you want to specify the output
    /// precision, consider using [`Float::log_base_power_of_2_prec_assign`]. If you want both of
    /// these things, consider using [`Float::log_base_power_of_2_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
    ///
    /// # Panics
    /// Panics if `pow` is zero (the base $2^0=1$ has no logarithm).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::LogBasePowerOf2Assign;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.log_base_power_of_2_assign(2);
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x.log_base_power_of_2_assign(2);
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::INFINITY;
    /// x.log_base_power_of_2_assign(-2);
    /// assert_eq!(x, Float::NEGATIVE_INFINITY);
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x.log_base_power_of_2_assign(2);
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// x.log_base_power_of_2_assign(2);
    /// assert_eq!(x.to_string(), "1.660964047443681173935159714745");
    ///
    /// let mut x = Float::from_signed_prec(-10, 100).0;
    /// x.log_base_power_of_2_assign(2);
    /// assert!(x.is_nan());
    /// ```
    #[inline]
    fn log_base_power_of_2_assign(&mut self, pow: i64) {
        let prec = self.significant_bits();
        self.log_base_power_of_2_prec_round_assign(pow, prec, Nearest);
    }
}
