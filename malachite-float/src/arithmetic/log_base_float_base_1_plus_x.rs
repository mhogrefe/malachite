// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Infinity, NaN};
use crate::arithmetic::log_base::{
    dyadic_1p_log_of_root, dyadic_primitive_root, odd_significand_and_exponent,
};
use crate::basic::extended::ExtendedFloat;
use crate::{
    Float, emulate_float_float_to_float_fn, float_infinity, float_nan, float_negative_infinity,
};
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{
    CeilingLogBase2, LogBaseOf1PlusX, LogBaseOf1PlusXAssign,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{NegativeZero, Zero as ZeroTrait};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::arithmetic::float_extras::float_can_round;
use malachite_nz::platform::Limb;
use malachite_q::Rational;

// Returns `Some(log_base(1 + x))` when it is rational, and `None` when it is irrational. `x` must
// be a finite [`Float`] in (-1, 0) or positive (not 0), and `base` a finite positive [`Float`] not
// equal to 1.
//
// `log_base(1 + x)` is rational exactly when `1 + x` and `base` are commensurable. `1 + x` is
// formed exactly as a `Rational`, and `base` is dyadic, so this reuses
// `rational_log_base_rational_rational_base`; for a base in (0, 1) -- where
// `Rational::checked_log_base` requires a base above 1 -- the identity `log_b(y) = -log_{1/b}(y)`
// reduces to a base above 1. Balloon-safe via the `64 * prec` size bound on `x`'s exponent and the
// operands' precisions (an `x` near -1 has a near-zero exponent and is materialized, but `1 + x` is
// then bounded by `x`'s precision).
pub(crate) fn log_base_float_base_1_plus_x_rational(x: &Float, base: &Float) -> Option<Rational> {
    if *x == 0u32 {
        return Some(Rational::ZERO);
    }
    // The base's primitive root comes from its odd significand and exponent without materializing
    // it, and `1 + x` is matched against that root implicitly (see `dyadic_1p_log_of_root`): its
    // integer form can be enormous even when `x` has few bits, so it is only materialized to verify
    // a match that a cheap congruence filter has already endorsed. No size cutoff: skipping the
    // check when the result is exactly representable would leave the Ziv loop unable to terminate.
    let (s_b, t_b) = odd_significand_and_exponent(base);
    let (z, h, e_base) = dyadic_primitive_root(&s_b, t_b);
    let m = dyadic_1p_log_of_root(x, z, &h)?;
    Some(Rational::from_signeds(m, i64::exact_from(e_base)))
}

// The computation of log_base(1 + x) for a `Float` base is done by log_base(1 + x) = log_2(1 + x) /
// log_2(base). The inputs are a finite `Float` `x` in (-1, 0) or positive (not 0), and a finite
// positive `Float` base not equal to 1.
//
// Both logs are ordinary native `Float`s. `log_2(1 + x)` is routed through `log_base_2_1_plus_x` to
// preserve accuracy for x near 0; it cannot underflow (|x| is at least the smallest positive
// `Float`). Their quotient can overflow (base near 1) or underflow (x near 0), so the operands are
// wrapped as `ExtendedFloat`s, divided in the extended range, and converted back with a single
// `into_float_helper` clamp. A base in (0, 1) gives a negative `log_2(base)`, so the division
// yields the (sign-flipped) result for free.
fn log_base_float_base_1_plus_x_normal(
    x: &Float,
    base: &Float,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    // If log_base(1 + x) is rational -- 1 + x and base commensurable -- compute it directly.
    if let Some(q) = log_base_float_base_1_plus_x_rational(x, base) {
        return Float::from_rational_prec_round(q, prec, rm);
    }
    // The result is irrational, so it is never exactly representable.
    assert_ne!(rm, Exact, "Inexact log_base_float_base_1_plus_x");
    // The initial slack keeps working_prec at least 7, so the working_prec - 6 below stays
    // positive.
    let mut working_prec = prec + 6 + prec.ceiling_log_base_2();
    let mut increment = Limb::WIDTH;
    loop {
        // log_2(1 + x) and log_2(base), correctly rounded and wrapped; both finite and nonzero (x
        // is not 0 and base is not 1), neither underflowing.
        let num = ExtendedFloat::from(x.log_base_2_1_plus_x_prec_ref(working_prec).0);
        let den = ExtendedFloat::from(base.log_base_2_prec_ref(working_prec).0);
        // log_2(1 + x) / log_2(base) in the extended range; cannot overflow or underflow here.
        let (quotient, _) = num.div_prec_val_ref(&den, working_prec);
        // Two correctly-rounded logs (<= 1/2 ulp each) and the division (<= 1/2 ulp) give under 2
        // ulps total; working_prec - 6 correct bits comfortably suffice for the rounding test.
        if float_can_round(
            quotient.x.significand_ref().unwrap(),
            working_prec - 6,
            prec,
            rm,
        ) {
            // Round the mantissa to prec, then place the extended exponent, clamping once to the
            // Float range as the rounding mode dictates.
            let (rounded, o) = Float::from_float_prec_round(quotient.x, prec, rm);
            let mut result = ExtendedFloat::from(rounded);
            result.exp = result.exp.checked_add(quotient.exp).unwrap();
            return result.into_float_helper(prec, rm, o);
        }
        // Increase the precision.
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

// Computes log_base(1 + x) = ln(1 + x) / ln(base) for `Float` `x` and `base`, following IEEE
// division of the natural logs for every special case (so the function is total: no input value
// panics). `ln(1 + x)` uses the sign-preserving `ln_1p` convention, so `ln_1p(x)` has the sign of
// `x` for `x` in (-1, infinity].
fn log_base_float_base_1_plus_x_helper(
    x: &Float,
    base: &Float,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    // ln(1 + x) or ln(base) is NaN: x or base is NaN, base is negative, or 1 + x < 0.
    if x.is_nan() || base.is_nan() {
        return (float_nan!(), Equal);
    }
    if *base < 0u32 {
        return (float_nan!(), Equal); // base negative finite or -infinity
    }
    if *x < -1i32 {
        return (float_nan!(), Equal); // 1 + x < 0 (including x = -infinity)
    }
    // x is in [-1, infinity] and not NaN; base is +infinity, zero, or positive finite. ln_1p(x) has
    // the sign of x (negative for x in [-1, 0) including -0.0).
    let x_neg = x.is_sign_negative();
    if base.is_infinite() {
        // ln(base) = +infinity. ln_1p(x) / +infinity = 0 for finite ln_1p(x) (NaN when it is
        // +-infinity, i.e. x = +infinity or x = -1), with the sign of ln_1p(x).
        if x.is_infinite() || *x == -1i32 {
            return (float_nan!(), Equal);
        }
        return if x_neg {
            (Float::NEGATIVE_ZERO, Equal)
        } else {
            (Float::ZERO, Equal)
        };
    }
    if *base == 0u32 {
        // ln(base) = -infinity. Sign-flipped from the +infinity case.
        if x.is_infinite() || *x == -1i32 {
            return (float_nan!(), Equal);
        }
        return if x_neg {
            (Float::ZERO, Equal)
        } else {
            (Float::NEGATIVE_ZERO, Equal)
        };
    }
    if *base == 1u32 {
        // ln(base) = +0. ln_1p(x) / +0 = +-infinity by the sign of ln_1p(x), or NaN for ln_1p(x) =
        // +-0 (x = +-0).
        if *x == 0u32 {
            return (float_nan!(), Equal);
        }
        return if x_neg {
            (float_negative_infinity!(), Equal)
        } else {
            (float_infinity!(), Equal)
        };
    }
    // base is positive finite and not 1.
    if x.is_infinite() {
        // ln_1p(+infinity) = +infinity. +infinity / ln(base): +infinity for base > 1, -infinity for
        // base < 1.
        return if *base < 1u32 {
            (float_negative_infinity!(), Equal)
        } else {
            (float_infinity!(), Equal)
        };
    }
    if *x == -1i32 {
        // ln_1p(-1) = ln(0) = -infinity. -infinity / ln(base): -infinity for base > 1, +infinity
        // for base < 1.
        return if *base < 1u32 {
            (float_infinity!(), Equal)
        } else {
            (float_negative_infinity!(), Equal)
        };
    }
    if *x == 0u32 {
        // ln_1p(+-0) = +-0. +-0 / ln(base) = 0, with the sign of ln_1p(x) times the sign of
        // ln(base) (positive for base > 1, negative for base < 1).
        return if x_neg == (*base < 1u32) {
            (Float::ZERO, Equal)
        } else {
            (Float::NEGATIVE_ZERO, Equal)
        };
    }
    // x is finite in (-1, 0) or positive (not 0), and base is positive finite and not 1.
    log_base_float_base_1_plus_x_normal(x, base, prec, rm)
}

impl Float {
    /// Computes $\log_b(1+x)$, where $x$ and the base $b$ are both [`Float`]s, rounding the result
    /// to the specified precision and with the specified rounding mode. The [`Float`] is taken by
    /// value and the base by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded value is less than, equal to, or greater than the exact value. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// $\log_b(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, `NaN` is returned. Otherwise the
    /// base may be any [`Float`]: the function is defined as $\ln(1+x) / \ln b$ for every pair,
    /// applying IEEE division to the natural logs, and never panics on an input value. In
    /// particular a base in $(0,1)$ gives a (sign-flipped) logarithm, and the non-normal and
    /// degenerate bases follow the limits below.
    ///
    /// This computes $\log_2(1+x) / \log_2 b$, routing through
    /// [`Float::log_base_2_1_plus_x_prec_ref`] to preserve accuracy for $x$ near 0, and wrapping
    /// the quotient so it may overflow (base near 1) or underflow (x near 0) and be clamped exactly
    /// once.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,b,p,m) = \log_b(1+x)+\varepsilon.
    /// $$
    /// - If $\log_b(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\log_b(1+x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_b(1+x)|\rfloor-p+1}$.
    /// - If $\log_b(1+x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_b(1+x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases (with $b$ the base):
    /// - $f(\text{NaN},b,p,m)=\text{NaN}$, and $f(x,\text{NaN},p,m)=\text{NaN}$
    /// - $f(x,b,p,m)=\text{NaN}$ for $x<-1$ or $b<0$
    /// - $f(\infty,b,p,m)=\infty$ for $b>1$, and $-\infty$ for $0\leq b<1$
    /// - $f(-1.0,b,p,m)=-\infty$ for $b>1$, and $\infty$ for $0<b<1$
    /// - $f(\pm0.0,b,p,m)=0$ (the sign of $\pm0.0$ times the sign of $1/\ln b$)
    /// - $f(x,\infty,p,m)=0$ for finite $x>-1$ with $x\neq0$ (and $\text{NaN}$ for
    ///   $x\in\{\infty,-1\}$)
    /// - $f(x,\pm0.0,p,m)=0$ for finite $x>-1$ with $x\neq0$ (and $\text{NaN}$ for
    ///   $x\in\{\infty,-1\}$)
    /// - $f(x,1.0,p,m)=\infty$ for $x>0$ or $x=\infty$, $-\infty$ for $-1\leq x<0$, and
    ///   $\text{NaN}$ for $x=\pm0.0$
    /// - $f(g^a-1,g^e,p,m)=a/e$ for a common rational $g$, rounded to precision $p$; the result is
    ///   exact if and only if $a/e$ is representable with precision $p$ (for example
    ///   $\log_4(1+1)=1/2$)
    ///
    /// This function can both overflow (for a base near 1) and underflow (for an $x$ near 0).
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
    /// let (log, o) =
    ///     Float::from(8).log_base_float_base_1_plus_x_prec_round(&Float::from(3), 10, Exact);
    /// assert_eq!(log.to_string(), "2.0"); // log_3(1 + 8) = log_3(9) = 2
    /// assert_eq!(o, Equal);
    ///
    /// let (log, o) =
    ///     Float::from(3).log_base_float_base_1_plus_x_prec_round(&Float::from(0.5), 10, Exact);
    /// assert_eq!(log.to_string(), "-2.0"); // log_{1/2}(1 + 3) = log_{1/2}(4) = -2
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_float_base_1_plus_x_prec_round(
        self,
        base: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        log_base_float_base_1_plus_x_helper(&self, base, prec, rm)
    }

    /// Computes $\log_b(1+x)$, where $x$ and the base $b$ are both [`Float`]s, rounding the result
    /// to the specified precision and with the specified rounding mode. Both are taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded value is less
    /// than, equal to, or greater than the exact value.
    ///
    /// See [`Float::log_base_float_base_1_plus_x_prec_round`] for details, special cases, and a
    /// description of the rounding behavior.
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
    /// let x = Float::from(7);
    /// let (log, o) = x.log_base_float_base_1_plus_x_prec_round_ref(&Float::from(2), 10, Exact);
    /// assert_eq!(log.to_string(), "3.0"); // log_2(1 + 7) = log_2(8) = 3
    /// assert_eq!(o, Equal);
    ///
    /// let x = Float::from(1);
    /// let (log, o) = x.log_base_float_base_1_plus_x_prec_round_ref(&Float::from(3), 20, Floor);
    /// assert_eq!(log.to_string(), "0.630929"); // log_3(2), rounded down
    /// assert_eq!(o, Less);
    /// ```
    pub fn log_base_float_base_1_plus_x_prec_round_ref(
        &self,
        base: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        log_base_float_base_1_plus_x_helper(self, base, prec, rm)
    }

    /// Computes $\log_b(1+x)$, where $x$ and the base $b$ are both [`Float`]s, rounding the result
    /// to the nearest value of the specified precision. The [`Float`] is taken by value and the
    /// base by reference. An [`Ordering`] is also returned.
    ///
    /// See [`Float::log_base_float_base_1_plus_x_prec_round`] for details and special cases.
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
    /// let (log, o) = Float::from(8).log_base_float_base_1_plus_x_prec(&Float::from(3), 10);
    /// assert_eq!(log.to_string(), "2.0"); // log_3(1 + 8) = log_3(9) = 2
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_float_base_1_plus_x_prec(self, base: &Self, prec: u64) -> (Self, Ordering) {
        self.log_base_float_base_1_plus_x_prec_round(base, prec, Nearest)
    }

    /// Computes $\log_b(1+x)$, where $x$ and the base $b$ are both [`Float`]s, rounding the result
    /// to the nearest value of the specified precision. Both are taken by reference. An
    /// [`Ordering`] is also returned.
    ///
    /// See [`Float::log_base_float_base_1_plus_x_prec_round`] for details and special cases.
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
    /// let (log, o) = (&Float::from(8)).log_base_float_base_1_plus_x_prec_ref(&Float::from(3), 10);
    /// assert_eq!(log.to_string(), "2.0"); // log_3(1 + 8) = log_3(9) = 2
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_float_base_1_plus_x_prec_ref(
        &self,
        base: &Self,
        prec: u64,
    ) -> (Self, Ordering) {
        self.log_base_float_base_1_plus_x_prec_round_ref(base, prec, Nearest)
    }

    /// Computes $\log_b(1+x)$, where $x$ and the base $b$ are both [`Float`]s, rounding the result
    /// to the precision of the input and with the specified rounding mode. The [`Float`] is taken
    /// by value and the base by reference. An [`Ordering`] is also returned.
    ///
    /// See [`Float::log_base_float_base_1_plus_x_prec_round`] for details and special cases.
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
    /// let (log, o) = Float::from(8).log_base_float_base_1_plus_x_round(&Float::from(3), Exact);
    /// assert_eq!(log.to_string(), "2.0"); // log_3(1 + 8) = log_3(9) = 2
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_float_base_1_plus_x_round(
        self,
        base: &Self,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.log_base_float_base_1_plus_x_prec_round(base, prec, rm)
    }

    /// Computes $\log_b(1+x)$, where $x$ and the base $b$ are both [`Float`]s, rounding the result
    /// to the precision of the input and with the specified rounding mode. Both are taken by
    /// reference. An [`Ordering`] is also returned.
    ///
    /// See [`Float::log_base_float_base_1_plus_x_prec_round`] for details and special cases.
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
    /// let (log, o) =
    ///     (&Float::from(8)).log_base_float_base_1_plus_x_round_ref(&Float::from(3), Exact);
    /// assert_eq!(log.to_string(), "2.0"); // log_3(1 + 8) = log_3(9) = 2
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_float_base_1_plus_x_round_ref(
        &self,
        base: &Self,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        self.log_base_float_base_1_plus_x_prec_round_ref(base, self.significant_bits(), rm)
    }

    /// Computes $\log_b(1+x)$, where $x$ and the base $b$ are both [`Float`]s, in place, rounding
    /// the result to the specified precision and with the specified rounding mode. The base is
    /// taken by reference. An [`Ordering`] is returned.
    ///
    /// See [`Float::log_base_float_base_1_plus_x_prec_round`] for details and special cases.
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
    /// let mut x = Float::from(8);
    /// assert_eq!(
    ///     x.log_base_float_base_1_plus_x_prec_round_assign(&Float::from(3), 10, Exact),
    ///     Equal
    /// );
    /// assert_eq!(x.to_string(), "2.0"); // log_3(1 + 8) = log_3(9) = 2
    /// ```
    #[inline]
    pub fn log_base_float_base_1_plus_x_prec_round_assign(
        &mut self,
        base: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (result, o) =
            core::mem::take(self).log_base_float_base_1_plus_x_prec_round(base, prec, rm);
        *self = result;
        o
    }

    /// Computes $\log_b(1+x)$, where $x$ and the base $b$ are both [`Float`]s, in place, rounding
    /// the result to the nearest value of the specified precision. The base is taken by reference.
    /// An [`Ordering`] is returned.
    ///
    /// See [`Float::log_base_float_base_1_plus_x_prec_round`] for details and special cases.
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
    /// let mut x = Float::from(8);
    /// x.log_base_float_base_1_plus_x_prec_assign(&Float::from(3), 10);
    /// assert_eq!(x.to_string(), "2.0"); // log_3(1 + 8) = log_3(9) = 2
    /// ```
    #[inline]
    pub fn log_base_float_base_1_plus_x_prec_assign(&mut self, base: &Self, prec: u64) -> Ordering {
        self.log_base_float_base_1_plus_x_prec_round_assign(base, prec, Nearest)
    }

    /// Computes $\log_b(1+x)$, where $x$ and the base $b$ are both [`Float`]s, in place, rounding
    /// the result to the precision of the input and with the specified rounding mode. The base is
    /// taken by reference. An [`Ordering`] is returned.
    ///
    /// See [`Float::log_base_float_base_1_plus_x_prec_round`] for details and special cases.
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
    /// let mut x = Float::from(8);
    /// x.log_base_float_base_1_plus_x_round_assign(&Float::from(3), Exact);
    /// assert_eq!(x.to_string(), "2.0"); // log_3(1 + 8) = log_3(9) = 2
    /// ```
    #[inline]
    pub fn log_base_float_base_1_plus_x_round_assign(
        &mut self,
        base: &Self,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = self.significant_bits();
        self.log_base_float_base_1_plus_x_prec_round_assign(base, prec, rm)
    }
}

impl LogBaseOf1PlusX<Self> for Float {
    type Output = Self;

    /// Computes $\log_b(1+x)$, where $x$ and the base $b$ are both [`Float`]s, rounding the result
    /// to the nearest value of the input's precision. Both are taken by value.
    ///
    /// See [`Float::log_base_float_base_1_plus_x_prec_round`] for special cases.
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
    /// use malachite_base::num::arithmetic::traits::LogBaseOf1PlusX;
    /// use malachite_float::Float;
    ///
    /// // log_3(1 + 8) = log_3(9) = 2
    /// assert_eq!(
    ///     Float::from(8).log_base_1_plus_x(Float::from(3)).to_string(),
    ///     "2.0"
    /// );
    /// ```
    #[inline]
    fn log_base_1_plus_x(self, base: Self) -> Self {
        let prec = self.significant_bits();
        self.log_base_float_base_1_plus_x_prec_round(&base, prec, Nearest)
            .0
    }
}

impl LogBaseOf1PlusX<&Float> for &Float {
    type Output = Float;

    /// Computes $\log_b(1+x)$, where $x$ and the base $b$ are both [`Float`]s, rounding the result
    /// to the nearest value of the input's precision. Both are taken by reference.
    ///
    /// See [`Float::log_base_float_base_1_plus_x_prec_round`] for special cases.
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
    /// use malachite_base::num::arithmetic::traits::LogBaseOf1PlusX;
    /// use malachite_float::Float;
    ///
    /// // log_3(1 + 8) = log_3(9) = 2
    /// assert_eq!(
    ///     (&Float::from(8))
    ///         .log_base_1_plus_x(&Float::from(3))
    ///         .to_string(),
    ///     "2.0"
    /// );
    /// ```
    #[inline]
    fn log_base_1_plus_x(self, base: &Float) -> Float {
        self.log_base_float_base_1_plus_x_prec_round_ref(base, self.significant_bits(), Nearest)
            .0
    }
}

impl LogBaseOf1PlusXAssign<&Self> for Float {
    /// Replaces a [`Float`] $x$ with $\log_b(1+x)$, where the base $b$ is a [`Float`], rounding the
    /// result to the nearest value of the input's precision. The base is taken by reference.
    ///
    /// See [`Float::log_base_float_base_1_plus_x_prec_round`] for special cases.
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
    /// use malachite_base::num::arithmetic::traits::LogBaseOf1PlusXAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(8);
    /// x.log_base_1_plus_x_assign(&Float::from(3));
    /// assert_eq!(x.to_string(), "2.0"); // log_3(1 + 8) = log_3(9) = 2
    /// ```
    #[inline]
    fn log_base_1_plus_x_assign(&mut self, base: &Self) {
        let prec = self.significant_bits();
        self.log_base_float_base_1_plus_x_prec_round_assign(base, prec, Nearest);
    }
}

/// Computes $\log_b(1+x)$, the base-$b$ logarithm of one plus a primitive float, where the base $b$
/// is also a primitive float, returning a primitive float result. Using this function is more
/// accurate than computing the logarithm using the standard library, both because $1+x$ may not be
/// representable as a primitive float and because the standard library's `log` is not always
/// correctly rounded.
///
/// $\log_b(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, `NaN` is returned. Otherwise the base
/// may be any primitive float: the function is defined as $\ln(1+x) / \ln b$ and never panics on an
/// input value. A base in $(0,1)$ gives a (sign-flipped) logarithm, and the non-normal and
/// degenerate bases follow the limits below.
///
/// $$
/// f(x,b) = \log_b(1+x)+\varepsilon.
/// $$
/// - If $\log_b(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $\log_b(1+x)$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
///   |\log_b(1+x)|\rfloor-p}$, where $p$ is precision of the output (typically 24 if `T` is a
///   [`f32`] and 53 if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases (with $b$ the base):
/// - $f(\text{NaN},b)=\text{NaN}$, and $f(x,\text{NaN})=\text{NaN}$
/// - $f(x,b)=\text{NaN}$ for $x<-1$ or $b<0$
/// - $f(\infty,b)=\infty$ for $b>1$, and $-\infty$ for $0\leq b<1$
/// - $f(-1.0,b)=-\infty$ for $b>1$, and $\infty$ for $0<b<1$
/// - $f(\pm0.0,b)=0$ (the sign of $\pm0.0$ times the sign of $1/\ln b$)
/// - $f(x,\infty)=0$ for finite $x>-1$ with $x\neq0$ (and $\text{NaN}$ for $x\in\{\infty,-1\}$)
/// - $f(x,\pm0.0)=0$ for finite $x>-1$ with $x\neq0$ (and $\text{NaN}$ for $x\in\{\infty,-1\}$)
/// - $f(x,1.0)=\infty$ for $x>0$ or $x=\infty$, $-\infty$ for $-1\leq x<0$, and $\text{NaN}$ for
///   $x=\pm0.0$
///
/// This function can both overflow (for a base near 1) and underflow (for an $x$ near 0).
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::arithmetic::log_base_float_base_1_plus_x::*;
///
/// // log_4(1 + 3) = log_4(4) = 1
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_float_base_1_plus_x(3.0f32, 4.0)),
///     NiceFloat(1.0)
/// );
/// // log_4(1 + 1) = log_4(2) = 1/2
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_float_base_1_plus_x(1.0f32, 4.0)),
///     NiceFloat(0.5)
/// );
/// // log_(1/2)(1 + 3) = log_(1/2)(4) = -2
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_float_base_1_plus_x(3.0f32, 0.5)),
///     NiceFloat(-2.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_float_base_1_plus_x(-1.0f32, 10.0)),
///     NiceFloat(f32::NEGATIVE_INFINITY)
/// );
/// assert!(primitive_float_log_base_float_base_1_plus_x(-2.0f32, 10.0).is_nan());
/// assert!(primitive_float_log_base_float_base_1_plus_x(3.0f32, f32::NAN).is_nan());
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_log_base_float_base_1_plus_x<T: PrimitiveFloat>(x: T, base: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_float_to_float_fn(
        |x, base, prec| x.log_base_float_base_1_plus_x_prec(&base, prec),
        x,
        base,
    )
}
