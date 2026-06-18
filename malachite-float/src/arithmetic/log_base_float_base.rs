// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Infinity, NaN};
use crate::arithmetic::log_base_rational_rational_base::rational_log_base_rational_rational_base;
use crate::basic::extended::ExtendedFloat;
use crate::{
    Float, emulate_float_float_to_float_fn, float_infinity, float_nan, float_negative_infinity,
};
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{CeilingLogBase2, LogBase, LogBaseAssign};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{NegativeZero, One, Zero as ZeroTrait};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::arithmetic::float_extras::float_can_round;
use malachite_nz::platform::Limb;
use malachite_q::Rational;

// Returns `Some(log_base(x))` when it is rational, and `None` when it is irrational. The inputs `x`
// and `base` must both be finite, positive, and not equal to 1.
//
// `log_base(x)` is rational exactly when `x` and `base` are commensurable (both powers of a common
// rational). Both are dyadic, so this reuses `rational_log_base_rational_rational_base` on their
// exact `Rational` values; for a base in (0, 1) -- where `Rational::checked_log_base` requires a
// base above 1 -- the identity `log_b(x) = -log_{1/b}(x)` reduces to a base above 1.
//
// Detecting these rational results up front is essential: the Ziv loop could never certify an
// exactly-representable one. The check is balloon-safe: it materializes `x` and `base` as
// `Rational`s only when their exponents and precisions are within `64 * prec`.
pub(crate) fn log_base_float_base_rational(x: &Float, base: &Float, prec: u64) -> Option<Rational> {
    let bound = prec.saturating_mul(64);
    if i64::from(x.get_exponent()?).unsigned_abs() > bound
        || i64::from(base.get_exponent()?).unsigned_abs() > bound
        || x.significant_bits() > bound
        || base.significant_bits() > bound
    {
        return None;
    }
    let xr = Rational::exact_from(x);
    let br = Rational::exact_from(base);
    if br > 1u32 {
        rational_log_base_rational_rational_base(&xr, &br, prec)
    } else {
        rational_log_base_rational_rational_base(&xr, &(Rational::ONE / br), prec).map(|q| -q)
    }
}

// The computation of log_base(x) for a `Float` base is done by log_base(x) = log_2(x) /
// log_2(base). The inputs are finite, positive, and not equal to 1.
//
// Both logarithms are ordinary, correctly-rounded `Float`s (a `Float` operand cannot be close
// enough to 1 to make its `log_2` underflow at any practical precision), but their quotient can
// overflow (base near 1, so `log_2(base)` is tiny) or underflow (x near 1). So the operands are
// wrapped as `ExtendedFloat`s, divided in the extended exponent range, and converted back with a
// single `into_float_helper` clamp. A base in (0, 1) gives a negative `log_2(base)`, so the
// division yields the (sign-flipped) result for free.
fn log_base_float_base_normal(
    x: &Float,
    base: &Float,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    // log_base(1) = 0, with the sign of 1 / log_2(base): positive for base > 1, negative for a base
    // in (0, 1).
    if *x == 1u32 {
        return if *base < 1u32 {
            (Float::NEGATIVE_ZERO, Equal)
        } else {
            (Float::ZERO, Equal)
        };
    }
    // If log_base(x) is rational -- x and base commensurable -- compute it directly. This includes
    // exactly-representable results (which the Ziv loop could never certify) as well as
    // non-representable rationals (cheaper and exact this way).
    if let Some(q) = log_base_float_base_rational(x, base, prec) {
        return Float::from_rational_prec_round(q, prec, rm);
    }
    // The result is irrational, so it is never exactly representable.
    assert_ne!(rm, Exact, "Inexact log_base_float_base");
    // The initial slack keeps working_prec at least 7, so the working_prec - 6 below stays
    // positive.
    let mut working_prec = prec + 6 + prec.ceiling_log_base_2();
    let mut increment = Limb::WIDTH;
    loop {
        // log_2(x) and log_2(base), correctly rounded; both finite and nonzero (x, base positive
        // and not 1), neither underflowing, so the ordinary logs wrapped as ExtendedFloats suffice.
        let num = ExtendedFloat::from(x.log_base_2_prec_ref(working_prec).0);
        let den = ExtendedFloat::from(base.log_base_2_prec_ref(working_prec).0);
        // log_2(x) / log_2(base) in the extended range; cannot overflow or underflow here.
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

// Computes log_base(x) = ln(x) / ln(base) for `Float` `x` and `base`, following IEEE division of
// the natural logs for every special case (so the function is total: no input value panics).
fn log_base_float_base_helper(
    x: &Float,
    base: &Float,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    // ln of either operand is NaN: x or base is NaN, or (below) negative.
    if x.is_nan() || base.is_nan() {
        return (float_nan!(), Equal);
    }
    // ln(base) is NaN for a negative base (negative finite or -infinity).
    if *base < 0u32 {
        return (float_nan!(), Equal);
    }
    // ln(x) is NaN for a negative x.
    if *x < 0u32 {
        return (float_nan!(), Equal);
    }
    // x and base are each now +infinity, zero, or positive finite.
    if base.is_infinite() {
        // ln(base) = +infinity. ln(x) / +infinity = 0 for finite x (NaN for an infinite or zero x).
        if x.is_infinite() || *x == 0u32 {
            return (float_nan!(), Equal);
        }
        // 0, signed like ln(x): +0 for x >= 1, -0 for 0 < x < 1.
        return if *x < 1u32 {
            (Float::NEGATIVE_ZERO, Equal)
        } else {
            (Float::ZERO, Equal)
        };
    }
    if *base == 0u32 {
        // ln(base) = -infinity. ln(x) / -infinity = 0 for finite x (NaN for an infinite or zero x),
        // sign-flipped: -0 for x >= 1, +0 for 0 < x < 1.
        if x.is_infinite() || *x == 0u32 {
            return (float_nan!(), Equal);
        }
        return if *x < 1u32 {
            (Float::ZERO, Equal)
        } else {
            (Float::NEGATIVE_ZERO, Equal)
        };
    }
    // base is positive finite.
    if *base == 1u32 {
        // ln(base) = +0. ln(x) / +0 = +-infinity by the sign of ln(x), or NaN for ln(x) = +-0.
        if x.is_infinite() {
            return (float_infinity!(), Equal); // ln(+inf) = +inf
        }
        if *x == 0u32 {
            return (float_negative_infinity!(), Equal); // ln(0) = -inf
        }
        return if *x == 1u32 {
            (float_nan!(), Equal) // +0 / +0
        } else if *x > 1u32 {
            (float_infinity!(), Equal)
        } else {
            (float_negative_infinity!(), Equal)
        };
    }
    // base is positive finite and not 1.
    if x.is_infinite() {
        // ln(x) = +infinity. +infinity / ln(base): +infinity for base > 1, -infinity for base < 1.
        return if *base < 1u32 {
            (float_negative_infinity!(), Equal)
        } else {
            (float_infinity!(), Equal)
        };
    }
    if *x == 0u32 {
        // ln(x) = -infinity. -infinity / ln(base): -infinity for base > 1, +infinity for base < 1.
        return if *base < 1u32 {
            (float_infinity!(), Equal)
        } else {
            (float_negative_infinity!(), Equal)
        };
    }
    // x and base are both positive finite, with base not 1.
    log_base_float_base_normal(x, base, prec, rm)
}

impl Float {
    /// Computes $\log_b x$, where $x$ and the base $b$ are both [`Float`]s, rounding the result to
    /// the specified precision and with the specified rounding mode. The [`Float`] is taken by
    /// value and the base by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded value is less than, equal to, or greater than the exact value. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// Unlike the integer- and rational-base logarithms, the base may be any [`Float`]: the
    /// function is defined as $\ln x / \ln b$ for every pair of [`Float`]s, applying IEEE division
    /// to the natural logs, and never panics on an input value. In particular a base in $(0,1)$
    /// gives a (sign-flipped) logarithm, and the non-normal and degenerate bases follow the limits
    /// below.
    ///
    /// This computes $\log_2 x / \log_2 b$, wrapping both logs in an extended exponent range so
    /// that the quotient may overflow (base near 1) or underflow (x near 1) and be clamped exactly
    /// once.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,b,p,m) = \log_b x+\varepsilon.
    /// $$
    /// - If $\log_b x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\log_b x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_b x|\rfloor-p+1}$.
    /// - If $\log_b x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_b x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases (with $b$ the base):
    /// - $f(\text{NaN},b,p,m)=\text{NaN}$, and $f(x,\text{NaN},p,m)=\text{NaN}$
    /// - $f(x,b,p,m)=\text{NaN}$ for $x<0$ or $b<0$ (including $\pm\infty$ where indicated below)
    /// - $f(\infty,b,p,m)=\infty$ for $b>1$, and $-\infty$ for $0\leq b<1$
    /// - $f(\pm0.0,b,p,m)=-\infty$ for $b>1$, and $\infty$ for $0<b<1$
    /// - $f(1.0,b,p,m)=0$ (with the sign of $1/\ln b$)
    /// - $f(x,\infty,p,m)=0$ for finite $x>0$ (and $\text{NaN}$ for $x\in\{\pm\infty,\pm0.0\}$)
    /// - $f(x,\pm0.0,p,m)=0$ for finite $x>0$ (and $\text{NaN}$ for $x\in\{\pm\infty,\pm0.0\}$)
    /// - $f(x,1.0,p,m)=\infty$ for $x>1$ or $x=\infty$, $-\infty$ for $0\leq x<1$, and $\text{NaN}$
    ///   for $x=1$
    /// - $f(g^a,g^e,p,m)=a/e$ for a common rational $g$, rounded to precision $p$; the result is
    ///   exact if and only if $a/e$ is representable with precision $p$ (for example $\log_4
    ///   8=3/2$)
    ///
    /// This function can both overflow (for a base near 1) and underflow (for an $x$ near 1).
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
    /// let (log, o) = Float::from(8).log_base_float_base_prec_round(&Float::from(4), 10, Exact);
    /// assert_eq!(log.to_string(), "1.5"); // log_4(8) = 3/2
    /// assert_eq!(o, Equal);
    ///
    /// let (log, o) =
    ///     Float::from(4).log_base_float_base_prec_round(&Float::from(0.5), 10, Exact);
    /// assert_eq!(log.to_string(), "-2.0"); // log_{1/2}(4) = -2
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_float_base_prec_round(
        self,
        base: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        log_base_float_base_helper(&self, base, prec, rm)
    }

    /// Computes $\log_b x$, where $x$ and the base $b$ are both [`Float`]s, rounding the result to
    /// the specified precision and with the specified rounding mode. Both are taken by reference.
    /// An [`Ordering`] is also returned, indicating whether the rounded value is less than, equal
    /// to, or greater than the exact value.
    ///
    /// See [`Float::log_base_float_base_prec_round`] for details, special cases, and a description
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
    /// let (log, o) =
    ///     (&Float::from(8)).log_base_float_base_prec_round_ref(&Float::from(2), 10, Exact);
    /// assert_eq!(log.to_string(), "3.0"); // log_2(8) = 3
    /// assert_eq!(o, Equal);
    ///
    /// let (log, o) =
    ///     (&Float::from(2)).log_base_float_base_prec_round_ref(&Float::from(4), 10, Exact);
    /// assert_eq!(log.to_string(), "0.5"); // log_4(2) = 1/2
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_float_base_prec_round_ref(
        &self,
        base: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        log_base_float_base_helper(self, base, prec, rm)
    }

    /// Computes $\log_b x$, where $x$ and the base $b$ are both [`Float`]s, rounding the result to
    /// the nearest value of the specified precision. The [`Float`] is taken by value and the base
    /// by reference. An [`Ordering`] is also returned.
    ///
    /// See [`Float::log_base_float_base_prec_round`] for details and special cases.
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
    /// let (log, o) = Float::from(8).log_base_float_base_prec(&Float::from(4), 10);
    /// assert_eq!(log.to_string(), "1.5"); // log_4(8) = 3/2
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_float_base_prec(self, base: &Self, prec: u64) -> (Self, Ordering) {
        self.log_base_float_base_prec_round(base, prec, Nearest)
    }

    /// Computes $\log_b x$, where $x$ and the base $b$ are both [`Float`]s, rounding the result to
    /// the nearest value of the specified precision. Both are taken by reference. An [`Ordering`]
    /// is also returned.
    ///
    /// See [`Float::log_base_float_base_prec_round`] for details and special cases.
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
    /// let (log, o) = (&Float::from(8)).log_base_float_base_prec_ref(&Float::from(4), 10);
    /// assert_eq!(log.to_string(), "1.5"); // log_4(8) = 3/2
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_float_base_prec_ref(&self, base: &Self, prec: u64) -> (Self, Ordering) {
        self.log_base_float_base_prec_round_ref(base, prec, Nearest)
    }

    /// Computes $\log_b x$, where $x$ and the base $b$ are both [`Float`]s, rounding the result to
    /// the precision of the input and with the specified rounding mode. The [`Float`] is taken by
    /// value and the base by reference. An [`Ordering`] is also returned.
    ///
    /// See [`Float::log_base_float_base_prec_round`] for details and special cases.
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
    /// let (log, o) = Float::from(81).log_base_float_base_round(&Float::from(3), Exact);
    /// assert_eq!(log.to_string(), "4.0"); // log_3(81) = 4
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_float_base_round(self, base: &Self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.log_base_float_base_prec_round(base, prec, rm)
    }

    /// Computes $\log_b x$, where $x$ and the base $b$ are both [`Float`]s, rounding the result to
    /// the precision of the input and with the specified rounding mode. Both are taken by
    /// reference. An [`Ordering`] is also returned.
    ///
    /// See [`Float::log_base_float_base_prec_round`] for details and special cases.
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
    /// let (log, o) = (&Float::from(81)).log_base_float_base_round_ref(&Float::from(3), Exact);
    /// assert_eq!(log.to_string(), "4.0"); // log_3(81) = 4
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_float_base_round_ref(&self, base: &Self, rm: RoundingMode) -> (Self, Ordering) {
        self.log_base_float_base_prec_round_ref(base, self.significant_bits(), rm)
    }

    /// Computes $\log_b x$, where $x$ and the base $b$ are both [`Float`]s, in place, rounding the
    /// result to the specified precision and with the specified rounding mode. The base is taken by
    /// reference. An [`Ordering`] is returned.
    ///
    /// See [`Float::log_base_float_base_prec_round`] for details and special cases.
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
    /// assert_eq!(x.log_base_float_base_prec_round_assign(&Float::from(4), 10, Exact), Equal);
    /// assert_eq!(x.to_string(), "1.5"); // log_4(8) = 3/2
    /// ```
    #[inline]
    pub fn log_base_float_base_prec_round_assign(
        &mut self,
        base: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (result, o) = core::mem::take(self).log_base_float_base_prec_round(base, prec, rm);
        *self = result;
        o
    }

    /// Computes $\log_b x$, where $x$ and the base $b$ are both [`Float`]s, in place, rounding the
    /// result to the nearest value of the specified precision. The base is taken by reference. An
    /// [`Ordering`] is returned.
    ///
    /// See [`Float::log_base_float_base_prec_round`] for details and special cases.
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
    /// x.log_base_float_base_prec_assign(&Float::from(4), 10);
    /// assert_eq!(x.to_string(), "1.5"); // log_4(8) = 3/2
    /// ```
    #[inline]
    pub fn log_base_float_base_prec_assign(&mut self, base: &Self, prec: u64) -> Ordering {
        self.log_base_float_base_prec_round_assign(base, prec, Nearest)
    }

    /// Computes $\log_b x$, where $x$ and the base $b$ are both [`Float`]s, in place, rounding the
    /// result to the precision of the input and with the specified rounding mode. The base is taken
    /// by reference. An [`Ordering`] is returned.
    ///
    /// See [`Float::log_base_float_base_prec_round`] for details and special cases.
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
    /// let mut x = Float::from(81);
    /// x.log_base_float_base_round_assign(&Float::from(3), Exact);
    /// assert_eq!(x.to_string(), "4.0"); // log_3(81) = 4
    /// ```
    #[inline]
    pub fn log_base_float_base_round_assign(&mut self, base: &Self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.log_base_float_base_prec_round_assign(base, prec, rm)
    }
}

impl LogBase<Self> for Float {
    type Output = Self;

    /// Computes $\log_b x$, where $x$ and the base $b$ are both [`Float`]s, rounding the result to
    /// the nearest value of the input's precision. Both are taken by value.
    ///
    /// See [`Float::log_base_float_base_prec_round`] for special cases.
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
    /// use malachite_base::num::arithmetic::traits::LogBase;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::from(81).log_base(Float::from(3)).to_string(), "4.0"); // log_3(81) = 4
    /// assert_eq!(Float::from(9).log_base(Float::from(3)).to_string(), "2.0"); // log_3(9) = 2
    /// ```
    #[inline]
    fn log_base(self, base: Self) -> Self {
        let prec = self.significant_bits();
        self.log_base_float_base_prec_round(&base, prec, Nearest).0
    }
}

impl LogBase<&Float> for &Float {
    type Output = Float;

    /// Computes $\log_b x$, where $x$ and the base $b$ are both [`Float`]s, rounding the result to
    /// the nearest value of the input's precision. Both are taken by reference.
    ///
    /// See [`Float::log_base_float_base_prec_round`] for special cases.
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
    /// use malachite_base::num::arithmetic::traits::LogBase;
    /// use malachite_float::Float;
    ///
    /// assert_eq!((&Float::from(81)).log_base(&Float::from(3)).to_string(), "4.0"); // log_3(81) = 4
    /// assert_eq!((&Float::from(9)).log_base(&Float::from(3)).to_string(), "2.0"); // log_3(9) = 2
    /// ```
    #[inline]
    fn log_base(self, base: &Float) -> Float {
        self.log_base_float_base_prec_round_ref(base, self.significant_bits(), Nearest)
            .0
    }
}

impl LogBaseAssign<&Self> for Float {
    /// Replaces a [`Float`] $x$ with $\log_b x$, where the base $b$ is a [`Float`], rounding the
    /// result to the nearest value of the input's precision. The base is taken by reference.
    ///
    /// See [`Float::log_base_float_base_prec_round`] for special cases.
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
    /// use malachite_base::num::arithmetic::traits::LogBaseAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(81);
    /// x.log_base_assign(&Float::from(3));
    /// assert_eq!(x.to_string(), "4.0"); // log_3(81) = 4
    /// ```
    #[inline]
    fn log_base_assign(&mut self, base: &Self) {
        let prec = self.significant_bits();
        self.log_base_float_base_prec_round_assign(base, prec, Nearest);
    }
}

/// Computes $\log_b x$, the base-$b$ logarithm of a primitive float, where the base $b$ is also a
/// primitive float, returning a primitive float result. Using this function is more accurate than
/// computing the logarithm using the standard library, whose logarithm functions are not always
/// correctly rounded.
///
/// Unlike the integer- and rational-base logarithms, the base may be any primitive float: the
/// function is defined as $\ln x / \ln b$ and never panics on an input value. A base in $(0,1)$
/// gives a (sign-flipped) logarithm, and the non-normal and degenerate bases follow the limits
/// below.
///
/// $$
/// f(x,b) = \log_b x+\varepsilon.
/// $$
/// - If $\log_b x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $\log_b x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |\log_b
///   x|\rfloor-p}$, where $p$ is precision of the output (typically 24 if `T` is a [`f32`] and 53
///   if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases (with $b$ the base):
/// - $f(\text{NaN},b)=\text{NaN}$, and $f(x,\text{NaN})=\text{NaN}$
/// - $f(x,b)=\text{NaN}$ for $x<0$ or $b<0$
/// - $f(\infty,b)=\infty$ for $b>1$, and $-\infty$ for $0\leq b<1$
/// - $f(\pm0.0,b)=-\infty$ for $b>1$, and $\infty$ for $0<b<1$
/// - $f(1.0,b)=0.0$ (with the sign of $1/\ln b$)
/// - $f(x,\infty)=0.0$ for finite $x>0$ (and $\text{NaN}$ for $x\in\{\pm\infty,\pm0.0\}$)
/// - $f(x,\pm0.0)=0.0$ for finite $x>0$ (and $\text{NaN}$ for $x\in\{\pm\infty,\pm0.0\}$)
/// - $f(x,1.0)=\infty$ for $x>1$ or $x=\infty$, $-\infty$ for $0\leq x<1$, and $\text{NaN}$ for
///   $x=1$
///
/// This function can both overflow (for a base near 1) and underflow (for an $x$ near 1).
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::arithmetic::log_base_float_base::primitive_float_log_base_float_base;
///
/// // log_4(8) = 3/2
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_float_base(8.0f32, 4.0)),
///     NiceFloat(1.5)
/// );
/// // log_(1/2)(4) = -2
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_float_base(4.0f32, 0.5)),
///     NiceFloat(-2.0)
/// );
/// // log_10(50)
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_float_base(50.0f32, 10.0)),
///     NiceFloat(1.69897)
/// );
/// // log_inf(8) = 0
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_float_base(8.0f32, f32::INFINITY)),
///     NiceFloat(0.0)
/// );
/// assert!(primitive_float_log_base_float_base(-1.0f32, 10.0).is_nan());
/// assert!(primitive_float_log_base_float_base(8.0f32, f32::NAN).is_nan());
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_log_base_float_base<T: PrimitiveFloat>(x: T, base: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_float_to_float_fn(
        |x, base, prec| x.log_base_float_base_prec(&base, prec),
        x,
        base,
    )
}
