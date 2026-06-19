// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Infinity, NaN, Zero};
use crate::arithmetic::log_base_2_1_plus_x::log_base_2_1_plus_x_exact;
use crate::{Float, emulate_float_to_float_fn, float_infinity, float_nan, float_negative_infinity};
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{
    CeilingLogBase2, IsPowerOf2, LogBasePowerOf2Of1PlusX, LogBasePowerOf2Of1PlusXAssign,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::arithmetic::float_extras::float_can_round;
use malachite_nz::platform::Limb;

// The computation of log_base_power_of_2_1_plus_x(x, pow) is done by log_{2^pow}(1 + x) = log_2(1 +
// x) / pow, where the input is finite, nonzero, and greater than -1.
fn log_base_power_of_2_1_plus_x_prec_round_normal(
    x: &Float,
    pow: i64,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    // log_{2^pow}(1 + x) is undefined for x < -1.
    match x.partial_cmp(&-1i32).unwrap() {
        // 1 + x = 0, so log_2(1 + x) = -infinity.
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
        _ => {}
    }
    // If 1 + x is exactly 2^m, then log_2(1 + x) = m and the result is the rational m / pow (exact
    // when representable at the target precision).
    if let Some(m) = log_base_2_1_plus_x_exact(x) {
        return Float::from(m).div_prec_round(Float::from(pow), prec, rm);
    }
    // The result is never exactly representable otherwise.
    assert_ne!(rm, Exact, "Inexact log_base_power_of_2_1_plus_x");
    let pow_f = Float::from(pow);
    let e_pow = i64::from(pow_f.get_exponent().unwrap());
    let min_exp = i64::from(Float::MIN_EXPONENT);
    // If x = 2^k for a k large enough that 1 + x is astronomically close to 2^k, then log_2(1 + x)
    // is k plus a positive infinitesimal (delta < 2^(2 - expx)), so log_{2^pow}(1 + x) is k / pow
    // nudged infinitesimally away from k / pow (toward +infinity when pow > 0, toward -infinity
    // when pow < 0). The Ziv loop below could never resolve this: log_2(1 + x) rounds to exactly k
    // at every working precision (this is log_base_2_1_plus_x's own `_special` regime), so when k /
    // pow is exactly representable the rounding test can never certify it and the precision grows
    // without bound. Handle it directly, but only once delta is below a quarter ulp of the *result*
    // k / pow (note the deviation must be measured at the result's scale, e_res, not k's scale --
    // dividing by pow can make the ulp finer relative to delta); otherwise delta is large enough
    // that the ordinary Ziv loop both terminates and is needed for correctness.
    if x.is_power_of_2() {
        let expx = i64::from(x.get_exponent().unwrap());
        let k = expx - 1; // x = 2^k
        if k >= 1 {
            // log_2(1 + x) lies in (k, k + delta). Represent "just above k" at a precision high
            // enough that the nudge, once divided by pow, stays far below one ulp of k / pow (so it
            // shares k / pow's rounding cell) yet is strictly nonzero. This pushes a representable
            // or tied k / pow off the boundary in the correct direction, so div_prec_round returns
            // the correctly-rounded value and ternary for the true (infinitesimally offset) result.
            let high_prec = prec + e_pow.unsigned_abs() + Limb::WIDTH;
            let mut t = Float::from_signed_prec(k, high_prec).0;
            t.increment();
            let (res, o) = t.div_prec_round(pow_f.clone(), prec, rm);
            let e_res = i64::from(res.get_exponent().unwrap());
            if 2 - expx < e_res - i64::exact_from(prec) - 1 {
                return (res, o);
            }
        }
    }
    let mut working_prec = prec + 3 + prec.ceiling_log_base_2();
    let mut increment = Limb::WIDTH;
    loop {
        // log_2(1 + x), correctly rounded to working_prec.
        let num = x.log_base_2_1_plus_x_prec_ref(working_prec).0;
        // log_2(1 + x) is always within the Float exponent range, but dividing by pow (with |pow| >
        // 1) can push the result below MIN_EXPONENT; overflow is impossible, since |pow| >= 1 means
        // |result| <= |log_2(1 + x)|. The quotient's exponent is e_num - e_pow or e_num - e_pow +
        // 1. When the result underflows, the Ziv test below can never resolve it (the quotient
        // clamps to a power of 2 at MIN_EXPONENT), so hand the rounding to div_prec_round, which
        // clamps to zero or the minimum positive value per the rounding mode. The exact quotient
        // exponent is resolved only in the narrow band where the cheap exponent bound is
        // inconclusive (then e_num - e_pow == min_exp - 1, so the result underflows iff |log_2(1 +
        // x) / pow| < 2^(min_exp - 1), i.e. iff |log_2(1 + x)| * 2^(1 - min_exp) < |pow|). The left
        // shift only adjusts the exponent (the shifted value's exponent is e_pow, well within
        // range), so this avoids converting a near-MIN_EXPONENT `num` to a `Rational` with a
        // ~2^30-bit denominator.
        let e_num = i64::from(num.get_exponent().unwrap());
        if e_num - e_pow + 1 < min_exp
            || (e_num - e_pow < min_exp && (&num << u64::exact_from(1 - min_exp)).lt_abs(&pow_f))
        {
            return num.div_prec_round(pow_f, prec, rm);
        }
        // log_2(1 + x) / pow, with two correctly-rounded operations: log_base_2_1_plus_x (at most
        // 1/2 ulp) and division by the exact integer pow (at most 1/2 ulp). The relative error is
        // thus below 2^(1 - working_prec), so working_prec - 2 correct bits suffice for rounding.
        // (log_base_2_1_plus_x itself handles inputs x = 2^k with k so large that 1 + x is
        // astronomically close to a power of 2, so no extra precision is needed here.)
        let t = num.div_prec(pow_f.clone(), working_prec).0;
        if float_can_round(t.significand_ref().unwrap(), working_prec - 2, prec, rm) {
            return Float::from_float_prec_round(t, prec, rm);
        }
        // Increase the precision.
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

impl Float {
    /// Computes $\log_{2^k}(1+x)$, where $x$ is a [`Float`] and the base is $2^k$ for some nonzero
    /// integer $k$, rounding the result to the specified precision and with the specified rounding
    /// mode. The base's exponent $k$ is `pow`, which may be negative. The [`Float`] is taken by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded value is less than,
    /// equal to, or greater than the exact value. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\log_{2^k}(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, `NaN` is returned.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,k,p,m) = \log_{2^k}(1+x)+\varepsilon.
    /// $$
    /// - If $\log_{2^k}(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\log_{2^k}(1+x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 |\log_{2^k}(1+x)|\rfloor-p+1}$.
    /// - If $\log_{2^k}(1+x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_{2^k}(1+x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},k,p,m)=\text{NaN}$
    /// - $f(\infty,k,p,m)=\infty$ if $k>0$, and $-\infty$ if $k<0$
    /// - $f(-\infty,k,p,m)=\text{NaN}$
    /// - $f(0.0,k,p,m)=0.0$ if $k>0$, and $-0.0$ if $k<0$
    /// - $f(-0.0,k,p,m)=-0.0$ if $k>0$, and $0.0$ if $k<0$
    /// - $f(-1.0,k,p,m)=-\infty$ if $k>0$, and $\infty$ if $k<0$
    /// - $f(x,k,p,m)=\text{NaN}$ for $x<-1$
    /// - $f(x,k,p,m)=m/k$ when $1+x=2^m$, rounded to precision $p$; the result is exact if and only
    ///   if $m/k$ is representable with precision $p$ (for example $\log_4 8=3/2$ when $x=7$ is
    ///   exact, but $\log_8 4=2/3$ when $x=3$ is not)
    ///
    /// This function cannot overflow, but it can underflow:
    /// - If $0<f(x,k,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,k,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,k,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,k,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,k,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,k,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,k,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,k,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::log_base_power_of_2_1_plus_x_prec`] instead. If you know that your target precision
    /// is the precision of the input, consider using [`Float::log_base_power_of_2_1_plus_x_round`]
    /// instead. If both of these things are true, consider using
    /// [`Float::log_base_power_of_2_1_plus_x`] instead.
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
    /// is `Exact` but the result cannot be represented exactly with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_power_of_2_1_plus_x_prec_round(2, 20, Floor);
    /// assert_eq!(log.to_string(), "1.729715");
    /// assert_eq!(o, Less);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_power_of_2_1_plus_x_prec_round(2, 20, Ceiling);
    /// assert_eq!(log.to_string(), "1.729717");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_power_of_2_1_plus_x_prec_round(
        self,
        pow: i64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        assert_ne!(pow, 0, "Cannot take base-1 logarithm");
        match self {
            Self(NaN | Infinity { sign: false }) => (float_nan!(), Equal),
            float_infinity!() => (
                if pow > 0 {
                    float_infinity!()
                } else {
                    float_negative_infinity!()
                },
                Equal,
            ),
            // log_{2^pow}(1 ± 0) = ±0, with the sign flipped when pow < 0
            Self(Zero { .. }) => (if pow > 0 { self } else { -self }, Equal),
            _ => log_base_power_of_2_1_plus_x_prec_round_normal(&self, pow, prec, rm),
        }
    }

    /// Computes $\log_{2^k}(1+x)$, where $x$ is a [`Float`] and the base is $2^k$ for some nonzero
    /// integer $k$, rounding the result to the specified precision and with the specified rounding
    /// mode. The base's exponent $k$ is `pow`, which may be negative. The [`Float`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded value is less
    /// than, equal to, or greater than the exact value. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\log_{2^k}(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, `NaN` is returned.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,k,p,m) = \log_{2^k}(1+x)+\varepsilon.
    /// $$
    /// - If $\log_{2^k}(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\log_{2^k}(1+x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 |\log_{2^k}(1+x)|\rfloor-p+1}$.
    /// - If $\log_{2^k}(1+x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_{2^k}(1+x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},k,p,m)=\text{NaN}$
    /// - $f(\infty,k,p,m)=\infty$ if $k>0$, and $-\infty$ if $k<0$
    /// - $f(-\infty,k,p,m)=\text{NaN}$
    /// - $f(0.0,k,p,m)=0.0$ if $k>0$, and $-0.0$ if $k<0$
    /// - $f(-0.0,k,p,m)=-0.0$ if $k>0$, and $0.0$ if $k<0$
    /// - $f(-1.0,k,p,m)=-\infty$ if $k>0$, and $\infty$ if $k<0$
    /// - $f(x,k,p,m)=\text{NaN}$ for $x<-1$
    /// - $f(x,k,p,m)=m/k$ when $1+x=2^m$, rounded to precision $p$; the result is exact if and only
    ///   if $m/k$ is representable with precision $p$ (for example $\log_4 8=3/2$ when $x=7$ is
    ///   exact, but $\log_8 4=2/3$ when $x=3$ is not)
    ///
    /// This function cannot overflow, but it can underflow:
    /// - If $0<f(x,k,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,k,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,k,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,k,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,k,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,k,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,k,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,k,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::log_base_power_of_2_1_plus_x_prec_ref`] instead. If you know that your target
    /// precision is the precision of the input, consider using
    /// [`Float::log_base_power_of_2_1_plus_x_round_ref`] instead. If both of these things are true,
    /// consider using `(&Float).log_base_power_of_2_1_plus_x()` instead.
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
    /// is `Exact` but the result cannot be represented exactly with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_power_of_2_1_plus_x_prec_round_ref(2, 20, Floor);
    /// assert_eq!(log.to_string(), "1.729715");
    /// assert_eq!(o, Less);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_power_of_2_1_plus_x_prec_round_ref(2, 20, Ceiling);
    /// assert_eq!(log.to_string(), "1.729717");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_power_of_2_1_plus_x_prec_round_ref(
        &self,
        pow: i64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        assert_ne!(pow, 0, "Cannot take base-1 logarithm");
        match self {
            Self(NaN | Infinity { sign: false }) => (float_nan!(), Equal),
            float_infinity!() => (
                if pow > 0 {
                    float_infinity!()
                } else {
                    float_negative_infinity!()
                },
                Equal,
            ),
            // log_{2^pow}(1 ± 0) = ±0, with the sign flipped when pow < 0
            Self(Zero { .. }) => (if pow > 0 { self.clone() } else { -self }, Equal),
            _ => log_base_power_of_2_1_plus_x_prec_round_normal(self, pow, prec, rm),
        }
    }

    /// Computes $\log_{2^k}(1+x)$, where $x$ is a [`Float`] and the base is $2^k$ for some nonzero
    /// integer $k$, rounding the result to the nearest value of the specified precision. The base's
    /// exponent $k$ is `pow`, which may be negative. The [`Float`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded value is less than, equal to,
    /// or greater than the exact value. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\log_{2^k}(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, `NaN` is returned.
    ///
    /// If the logarithm is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,k,p) = \log_{2^k}(1+x)+\varepsilon.
    /// $$
    /// - If $\log_{2^k}(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\log_{2^k}(1+x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\log_{2^k}(1+x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},k,p)=\text{NaN}$
    /// - $f(\infty,k,p)=\infty$ if $k>0$, and $-\infty$ if $k<0$
    /// - $f(-\infty,k,p)=\text{NaN}$
    /// - $f(0.0,k,p)=0.0$ if $k>0$, and $-0.0$ if $k<0$
    /// - $f(-0.0,k,p)=-0.0$ if $k>0$, and $0.0$ if $k<0$
    /// - $f(-1.0,k,p)=-\infty$ if $k>0$, and $\infty$ if $k<0$
    /// - $f(x,k,p)=\text{NaN}$ for $x<-1$
    /// - $f(x,k,p)=m/k$ when $1+x=2^m$, rounded to precision $p$; the result is exact if and only
    ///   if $m/k$ is representable with precision $p$
    ///
    /// This function cannot overflow, but it can underflow:
    /// - If $0<f(x,k,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,k,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,k,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,k,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::log_base_power_of_2_1_plus_x_prec_round`] instead. If you know that your target
    /// precision is the precision of the input, consider using
    /// [`Float::log_base_power_of_2_1_plus_x`] instead.
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
    ///     .log_base_power_of_2_1_plus_x_prec(2, 20);
    /// assert_eq!(log.to_string(), "1.729715");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn log_base_power_of_2_1_plus_x_prec(self, pow: i64, prec: u64) -> (Self, Ordering) {
        self.log_base_power_of_2_1_plus_x_prec_round(pow, prec, Nearest)
    }

    /// Computes $\log_{2^k}(1+x)$, where $x$ is a [`Float`] and the base is $2^k$ for some nonzero
    /// integer $k$, rounding the result to the nearest value of the specified precision. The base's
    /// exponent $k$ is `pow`, which may be negative. The [`Float`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded value is less than, equal to,
    /// or greater than the exact value. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\log_{2^k}(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, `NaN` is returned.
    ///
    /// If the logarithm is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,k,p) = \log_{2^k}(1+x)+\varepsilon.
    /// $$
    /// - If $\log_{2^k}(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\log_{2^k}(1+x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\log_{2^k}(1+x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},k,p)=\text{NaN}$
    /// - $f(\infty,k,p)=\infty$ if $k>0$, and $-\infty$ if $k<0$
    /// - $f(-\infty,k,p)=\text{NaN}$
    /// - $f(0.0,k,p)=0.0$ if $k>0$, and $-0.0$ if $k<0$
    /// - $f(-0.0,k,p)=-0.0$ if $k>0$, and $0.0$ if $k<0$
    /// - $f(-1.0,k,p)=-\infty$ if $k>0$, and $\infty$ if $k<0$
    /// - $f(x,k,p)=\text{NaN}$ for $x<-1$
    /// - $f(x,k,p)=m/k$ when $1+x=2^m$, rounded to precision $p$; the result is exact if and only
    ///   if $m/k$ is representable with precision $p$
    ///
    /// This function cannot overflow, but it can underflow:
    /// - If $0<f(x,k,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,k,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,k,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,k,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::log_base_power_of_2_1_plus_x_prec_round_ref`] instead. If you know that your target
    /// precision is the precision of the input, consider using
    /// `(&Float).log_base_power_of_2_1_plus_x()` instead.
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
    ///     .log_base_power_of_2_1_plus_x_prec_ref(2, 20);
    /// assert_eq!(log.to_string(), "1.729715");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn log_base_power_of_2_1_plus_x_prec_ref(&self, pow: i64, prec: u64) -> (Self, Ordering) {
        self.log_base_power_of_2_1_plus_x_prec_round_ref(pow, prec, Nearest)
    }

    /// Computes $\log_{2^k}(1+x)$, where $x$ is a [`Float`] and the base is $2^k$ for some nonzero
    /// integer $k$, rounding the result with the specified rounding mode. The base's exponent $k$
    /// is `pow`, which may be negative. The [`Float`] is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $\log_{2^k}(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, `NaN` is returned.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,k,m) = \log_{2^k}(1+x)+\varepsilon.
    /// $$
    /// - If $\log_{2^k}(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\log_{2^k}(1+x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 |\log_{2^k}(1+x)|\rfloor-p+1}$, where $p$ is the precision of the
    ///   input.
    /// - If $\log_{2^k}(1+x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_{2^k}(1+x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::log_base_power_of_2_1_plus_x_prec_round`] documentation for information on
    /// special cases, overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::log_base_power_of_2_1_plus_x_prec_round`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using [`Float::log_base_power_of_2_1_plus_x`] instead.
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
    /// result cannot be represented exactly with the input precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_power_of_2_1_plus_x_round(2, Floor);
    /// assert_eq!(log.to_string(), "1.729715809318648628099681523362");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn log_base_power_of_2_1_plus_x_round(
        self,
        pow: i64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.log_base_power_of_2_1_plus_x_prec_round(pow, prec, rm)
    }

    /// Computes $\log_{2^k}(1+x)$, where $x$ is a [`Float`] and the base is $2^k$ for some nonzero
    /// integer $k$, rounding the result with the specified rounding mode. The base's exponent $k$
    /// is `pow`, which may be negative. The [`Float`] is taken by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded value is less than, equal to, or greater than
    /// the exact value. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $\log_{2^k}(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, `NaN` is returned.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,k,m) = \log_{2^k}(1+x)+\varepsilon.
    /// $$
    /// - If $\log_{2^k}(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\log_{2^k}(1+x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 |\log_{2^k}(1+x)|\rfloor-p+1}$, where $p$ is the precision of the
    ///   input.
    /// - If $\log_{2^k}(1+x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_{2^k}(1+x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::log_base_power_of_2_1_plus_x_prec_round`] documentation for information on
    /// special cases, overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::log_base_power_of_2_1_plus_x_prec_round_ref`] instead. If you know you'll be using
    /// the `Nearest` rounding mode, consider using `(&Float).log_base_power_of_2_1_plus_x()`
    /// instead.
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
    /// result cannot be represented exactly with the input precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_power_of_2_1_plus_x_round_ref(2, Floor);
    /// assert_eq!(log.to_string(), "1.729715809318648628099681523362");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn log_base_power_of_2_1_plus_x_round_ref(
        &self,
        pow: i64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.log_base_power_of_2_1_plus_x_prec_round_ref(pow, prec, rm)
    }

    /// Computes $\log_{2^k}(1+x)$, where $x$ is a [`Float`] and the base is $2^k$ for some nonzero
    /// integer $k$, in place, rounding the result to the specified precision and with the specified
    /// rounding mode. The base's exponent $k$ is `pow`, which may be negative. An [`Ordering`] is
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// $\log_{2^k}(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, the [`Float`] is set to
    /// `NaN`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \log_{2^k}(1+x)+\varepsilon.
    /// $$
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::log_base_power_of_2_1_plus_x_prec_round`] documentation for information on
    /// special cases, overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::log_base_power_of_2_1_plus_x_prec_assign`] instead. If you know that your target
    /// precision is the precision of the input, consider using
    /// [`Float::log_base_power_of_2_1_plus_x_round_assign`] instead. If both of these things are
    /// true, consider using [`Float::log_base_power_of_2_1_plus_x_assign`] instead.
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
    /// is `Exact` but the result cannot be represented exactly with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(
    ///     x.log_base_power_of_2_1_plus_x_prec_round_assign(2, 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "1.729715");
    /// ```
    #[inline]
    pub fn log_base_power_of_2_1_plus_x_prec_round_assign(
        &mut self,
        pow: i64,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (result, o) =
            core::mem::take(self).log_base_power_of_2_1_plus_x_prec_round(pow, prec, rm);
        *self = result;
        o
    }

    /// Computes $\log_{2^k}(1+x)$, where $x$ is a [`Float`] and the base is $2^k$ for some nonzero
    /// integer $k$, in place, rounding the result to the nearest value of the specified precision.
    /// The base's exponent $k$ is `pow`, which may be negative. An [`Ordering`] is returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`Float`], whenever this function sets the
    /// [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// $\log_{2^k}(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, the [`Float`] is set to
    /// `NaN`.
    ///
    /// If the logarithm is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \log_{2^k}(1+x)+\varepsilon.
    /// $$
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::log_base_power_of_2_1_plus_x_prec`] documentation for information on
    /// special cases, overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::log_base_power_of_2_1_plus_x_prec_round_assign`] instead. If you know that your
    /// target precision is the precision of the input, consider using
    /// [`Float::log_base_power_of_2_1_plus_x_assign`] instead.
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
    /// assert_eq!(x.log_base_power_of_2_1_plus_x_prec_assign(2, 20), Less);
    /// assert_eq!(x.to_string(), "1.729715");
    /// ```
    #[inline]
    pub fn log_base_power_of_2_1_plus_x_prec_assign(&mut self, pow: i64, prec: u64) -> Ordering {
        self.log_base_power_of_2_1_plus_x_prec_round_assign(pow, prec, Nearest)
    }

    /// Computes $\log_{2^k}(1+x)$, where $x$ is a [`Float`] and the base is $2^k$ for some nonzero
    /// integer $k$, in place, rounding the result with the specified rounding mode. The base's
    /// exponent $k$ is `pow`, which may be negative. An [`Ordering`] is returned, indicating
    /// whether the rounded value is less than, equal to, or greater than the exact value. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function sets the [`Float`] to
    /// `NaN` it also returns `Equal`.
    ///
    /// $\log_{2^k}(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, the [`Float`] is set to
    /// `NaN`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \log_{2^k}(1+x)+\varepsilon.
    /// $$
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::log_base_power_of_2_1_plus_x_round`] documentation for information on
    /// special cases, overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::log_base_power_of_2_1_plus_x_prec_round_assign`] instead. If you know you'll be
    /// using the `Nearest` rounding mode, consider using
    /// [`Float::log_base_power_of_2_1_plus_x_assign`] instead.
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
    /// result cannot be represented exactly with the input precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.log_base_power_of_2_1_plus_x_round_assign(2, Floor), Less);
    /// assert_eq!(x.to_string(), "1.729715809318648628099681523362");
    /// ```
    #[inline]
    pub fn log_base_power_of_2_1_plus_x_round_assign(
        &mut self,
        pow: i64,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = self.significant_bits();
        self.log_base_power_of_2_1_plus_x_prec_round_assign(pow, prec, rm)
    }
}

impl LogBasePowerOf2Of1PlusX<i64> for Float {
    type Output = Self;

    /// Computes $\log_{2^k}(1+x)$, where $x$ is a [`Float`] and the base is $2^k$ for some nonzero
    /// integer $k$, taking it by value. The base's exponent $k$ is `pow`, which may be negative.
    ///
    /// If the output has a precision, it is the precision of the input. If the logarithm is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $\log_{2^k}(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, `NaN` is returned.
    ///
    /// $$
    /// f(x,k) = \log_{2^k}(1+x)+\varepsilon.
    /// $$
    /// - If $\log_{2^k}(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\log_{2^k}(1+x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\log_{2^k}(1+x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// See the [`Float::log_base_power_of_2_1_plus_x_prec_round`] documentation for information on
    /// special cases, overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::log_base_power_of_2_1_plus_x_round`] instead. If you want to specify the output
    /// precision, consider using [`Float::log_base_power_of_2_1_plus_x_prec`]. If you want both of
    /// these things, consider using [`Float::log_base_power_of_2_1_plus_x_prec_round`].
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
    /// use malachite_base::num::arithmetic::traits::LogBasePowerOf2Of1PlusX;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.log_base_power_of_2_1_plus_x(2).is_nan());
    /// assert_eq!(
    ///     Float::INFINITY.log_base_power_of_2_1_plus_x(2),
    ///     Float::INFINITY
    /// );
    /// assert!(Float::NEGATIVE_INFINITY
    ///     .log_base_power_of_2_1_plus_x(2)
    ///     .is_nan());
    /// assert_eq!(
    ///     Float::from_unsigned_prec(10u32, 100)
    ///         .0
    ///         .log_base_power_of_2_1_plus_x(2)
    ///         .to_string(),
    ///     "1.729715809318648628099681523362"
    /// );
    /// ```
    #[inline]
    fn log_base_power_of_2_1_plus_x(self, pow: i64) -> Self {
        let prec = self.significant_bits();
        self.log_base_power_of_2_1_plus_x_prec_round(pow, prec, Nearest)
            .0
    }
}

impl LogBasePowerOf2Of1PlusX<i64> for &Float {
    type Output = Float;

    /// Computes $\log_{2^k}(1+x)$, where $x$ is a [`Float`] and the base is $2^k$ for some nonzero
    /// integer $k$, taking it by reference. The base's exponent $k$ is `pow`, which may be
    /// negative.
    ///
    /// If the output has a precision, it is the precision of the input. If the logarithm is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $\log_{2^k}(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, `NaN` is returned.
    ///
    /// $$
    /// f(x,k) = \log_{2^k}(1+x)+\varepsilon.
    /// $$
    /// - If $\log_{2^k}(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\log_{2^k}(1+x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\log_{2^k}(1+x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// See the [`Float::log_base_power_of_2_1_plus_x_prec_round`] documentation for information on
    /// special cases, overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::log_base_power_of_2_1_plus_x_round_ref`] instead. If you want to specify the output
    /// precision, consider using [`Float::log_base_power_of_2_1_plus_x_prec_ref`]. If you want both
    /// of these things, consider using [`Float::log_base_power_of_2_1_plus_x_prec_round_ref`].
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
    /// use malachite_base::num::arithmetic::traits::LogBasePowerOf2Of1PlusX;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::NAN).log_base_power_of_2_1_plus_x(2).is_nan());
    /// assert_eq!(
    ///     (&Float::INFINITY).log_base_power_of_2_1_plus_x(2),
    ///     Float::INFINITY
    /// );
    /// assert!((&Float::NEGATIVE_INFINITY)
    ///     .log_base_power_of_2_1_plus_x(2)
    ///     .is_nan());
    /// assert_eq!(
    ///     (&Float::from_unsigned_prec(10u32, 100).0)
    ///         .log_base_power_of_2_1_plus_x(2)
    ///         .to_string(),
    ///     "1.729715809318648628099681523362"
    /// );
    /// ```
    #[inline]
    fn log_base_power_of_2_1_plus_x(self, pow: i64) -> Float {
        let prec = self.significant_bits();
        self.log_base_power_of_2_1_plus_x_prec_round_ref(pow, prec, Nearest)
            .0
    }
}

impl LogBasePowerOf2Of1PlusXAssign<i64> for Float {
    /// Computes $\log_{2^k}(1+x)$, where $x$ is a [`Float`] and the base is $2^k$ for some nonzero
    /// integer $k$, in place. The base's exponent $k$ is `pow`, which may be negative.
    ///
    /// If the output has a precision, it is the precision of the input. If the logarithm is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $\log_{2^k}(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, the [`Float`] is set to
    /// `NaN`.
    ///
    /// $$
    /// x \gets \log_{2^k}(1+x)+\varepsilon.
    /// $$
    /// - If $\log_{2^k}(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\log_{2^k}(1+x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\log_{2^k}(1+x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// See the [`Float::log_base_power_of_2_1_plus_x`] documentation for information on special
    /// cases, overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::log_base_power_of_2_1_plus_x_round_assign`] instead. If you want to specify the
    /// output precision, consider using [`Float::log_base_power_of_2_1_plus_x_prec_assign`]. If you
    /// want both of these things, consider using
    /// [`Float::log_base_power_of_2_1_plus_x_prec_round_assign`].
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
    /// use malachite_base::num::arithmetic::traits::LogBasePowerOf2Of1PlusXAssign;
    /// use malachite_base::num::basic::traits::{Infinity, NaN};
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.log_base_power_of_2_1_plus_x_assign(2);
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x.log_base_power_of_2_1_plus_x_assign(2);
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// x.log_base_power_of_2_1_plus_x_assign(2);
    /// assert_eq!(x.to_string(), "1.729715809318648628099681523362");
    /// ```
    #[inline]
    fn log_base_power_of_2_1_plus_x_assign(&mut self, pow: i64) {
        let prec = self.significant_bits();
        self.log_base_power_of_2_1_plus_x_prec_round_assign(pow, prec, Nearest);
    }
}

/// Computes $\log_{2^k}(1+x)$, the base-$2^k$ logarithm of one plus a primitive float, where the
/// base is $2^k$ for some nonzero integer $k$. The exponent $k$ is `pow`, which may be negative.
/// Using this function is more accurate than computing the logarithm using the standard library,
/// both because $1+x$ may not be representable as a primitive float and because the standard
/// library's `log2` is not always correctly rounded.
///
/// $\log_{2^k}(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, `NaN` is returned.
///
/// $$
/// f(x,k) = \log_{2^k}(1+x)+\varepsilon.
/// $$
/// - If $\log_{2^k}(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
///   be 0.
/// - If $\log_{2^k}(1+x)$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
///   |\log_{2^k}(1+x)|\rfloor-p}$, where $p$ is precision of the output (typically 24 if `T` is a
///   [`f32`] and 53 if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(\text{NaN},k)=\text{NaN}$
/// - $f(\infty,k)=\infty$ if $k>0$, and $-\infty$ if $k<0$
/// - $f(-\infty,k)=\text{NaN}$
/// - $f(0.0,k)=0.0$ if $k>0$, and $-0.0$ if $k<0$
/// - $f(-0.0,k)=-0.0$ if $k>0$, and $0.0$ if $k<0$
/// - $f(-1.0,k)=-\infty$ if $k>0$, and $\infty$ if $k<0$
/// - $f(x,k)=\text{NaN}$ for $x<-1$
///
/// This function can underflow (to a subnormal or zero) when $x$ is close to zero and $|k|$ is
/// large, but it cannot overflow.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `pow` is zero (the base $2^0=1$ has no logarithm).
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::arithmetic::log_base_power_of_2_1_plus_x::*;
///
/// assert!(primitive_float_log_base_power_of_2_1_plus_x(f32::NAN, 2).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_power_of_2_1_plus_x(
///         f32::INFINITY,
///         2
///     )),
///     NiceFloat(f32::INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_power_of_2_1_plus_x(-1.0f32, 2)),
///     NiceFloat(f32::NEGATIVE_INFINITY)
/// );
/// assert!(primitive_float_log_base_power_of_2_1_plus_x(-2.0f32, 2).is_nan());
/// // log_4(1 + 15) = log_4(16) = 2
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_power_of_2_1_plus_x(15.0f32, 2)),
///     NiceFloat(2.0)
/// );
/// // log_4(1 + 7) = log_4(8) = 3/2
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_power_of_2_1_plus_x(7.0f32, 2)),
///     NiceFloat(1.5)
/// );
/// // log_8(1 + 63) = log_8(64) = 2
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_power_of_2_1_plus_x(63.0f32, 3)),
///     NiceFloat(2.0)
/// );
/// // log_4(1 + 9) = log_4(10)
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_power_of_2_1_plus_x(9.0f32, 2)),
///     NiceFloat(1.660964)
/// );
/// // log_(1/2)(1 + 7) = log_(1/2)(8) = -3
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_power_of_2_1_plus_x(7.0f32, -1)),
///     NiceFloat(-3.0)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_log_base_power_of_2_1_plus_x<T: PrimitiveFloat>(x: T, pow: i64) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(
        |x, prec| Float::log_base_power_of_2_1_plus_x_prec(x, pow, prec),
        x,
    )
}
