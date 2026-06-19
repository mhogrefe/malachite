// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Infinity, NaN};
use crate::arithmetic::log_base_2::extended_log_base_2_of_rational;
use crate::arithmetic::log_base_rational_rational_base::rational_log_base_rational_rational_base;
use crate::basic::extended::ExtendedFloat;
use crate::{
    Float, emulate_rational_float_to_float_fn, float_infinity, float_nan, float_negative_infinity,
};
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::CeilingLogBase2;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{NegativeZero, One, Zero as ZeroTrait};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::arithmetic::float_extras::float_can_round;
use malachite_nz::platform::Limb;
use malachite_q::Rational;

// Returns `Some(log_base(x))` when it is rational, and `None` when it is irrational. The input `x`
// must be a positive [`Rational`] not equal to 1, and `base` a finite positive [`Float`] not equal
// to 1.
//
// `log_base(x)` is rational exactly when `x` and `base` are commensurable. `base` is dyadic, so
// this reuses `rational_log_base_rational_rational_base` on the two `Rational`s; for a base in (0,
// 1) -- where `Rational::checked_log_base` requires a base above 1 -- the identity `log_b(x) =
// -log_{1/b}(x)` reduces to a base above 1. Balloon-safe via the `64 * prec` size bound.
pub(crate) fn log_base_rational_float_base_rational(
    x: &Rational,
    base: &Float,
    prec: u64,
) -> Option<Rational> {
    let bound = prec.saturating_mul(64);
    if x.significant_bits() > bound
        || i64::from(base.get_exponent()?).unsigned_abs() > bound
        || base.significant_bits() > bound
    {
        return None;
    }
    let br = Rational::exact_from(base);
    if br > 1u32 {
        rational_log_base_rational_rational_base(x, &br, prec)
    } else {
        rational_log_base_rational_rational_base(x, &(Rational::ONE / br), prec).map(|q| -q)
    }
}

// The computation of log_base(x) for a `Rational` `x` and a `Float` base is done by log_base(x) =
// log_2(x) / log_2(base). The inputs are a positive `Rational` `x` not equal to 1 and a finite
// positive `Float` base not equal to 1.
//
// `log_2(x)` is computed in the extended exponent range (`extended_log_base_2_of_rational`) so that
// an `x` near 1 -- a `Rational` can be arbitrarily close, making `log_2(x)` underflow an ordinary
// `Float` -- is represented faithfully (this is the underflow source). `log_2(base)` is an ordinary
// native `Float` log (a `Float` base cannot be close enough to 1 to underflow its `log_2` at
// practical precision; a base near 1 is instead the overflow source, where `log_2(base)` is tiny
// and the quotient is huge). Both are wrapped as `ExtendedFloat`s, divided in the extended range,
// and converted back with a single `into_float_helper` clamp. A base in (0, 1) gives a negative
// `log_2(base)`, so the division yields the (sign-flipped) result for free.
fn log_base_rational_float_base_normal(
    x: &Rational,
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
    // If log_base(x) is rational -- x and base commensurable -- compute it directly.
    if let Some(q) = log_base_rational_float_base_rational(x, base, prec) {
        return Float::from_rational_prec_round(q, prec, rm);
    }
    // The result is irrational, so it is never exactly representable.
    assert_ne!(rm, Exact, "Inexact log_base_rational_float_base");
    // The initial slack keeps working_prec at least 7, so the working_prec - 6 below stays
    // positive.
    let mut working_prec = prec + 6 + prec.ceiling_log_base_2();
    let mut increment = Limb::WIDTH;
    loop {
        // log_2(x), extended (handles an x near 1 without underflow); finite and nonzero (x is
        // positive and not 1).
        let num = extended_log_base_2_of_rational(x, working_prec);
        // log_2(base), correctly rounded and wrapped; finite and nonzero (base positive and not 1).
        let den = ExtendedFloat::from(base.log_base_2_prec_ref(working_prec).0);
        // log_2(x) / log_2(base) in the extended range; cannot overflow or underflow here.
        let (quotient, _) = num.div_prec_val_ref(&den, working_prec);
        // log_2(x) is within 2 ulps, log_2(base) is correctly rounded (<= 1/2 ulp), and the
        // division adds at most 1 more, for under 4 ulps total; working_prec - 6 correct bits
        // comfortably suffice for the rounding test.
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

// Computes log_base(x) = ln(x) / ln(base) for a `Rational` `x` and a `Float` base, following IEEE
// division of the natural logs for every special case (so the function is total: no input value
// panics). `x` is always finite.
fn log_base_rational_float_base_helper(
    x: &Rational,
    base: &Float,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    // ln(base) is NaN for a NaN or negative base (negative finite or -infinity).
    if base.is_nan() || *base < 0u32 {
        return (float_nan!(), Equal);
    }
    // ln(x) is NaN for a negative x.
    if *x < 0u32 {
        return (float_nan!(), Equal);
    }
    if base.is_infinite() {
        // ln(base) = +infinity. ln(x) / +infinity = 0 for x > 0 (NaN for x = 0): +0 for x >= 1, -0
        // for 0 < x < 1.
        if *x == 0u32 {
            return (float_nan!(), Equal);
        }
        return if *x < 1u32 {
            (Float::NEGATIVE_ZERO, Equal)
        } else {
            (Float::ZERO, Equal)
        };
    }
    if *base == 0u32 {
        // ln(base) = -infinity. ln(x) / -infinity = 0 for x > 0 (NaN for x = 0), sign-flipped: -0
        // for x >= 1, +0 for 0 < x < 1.
        if *x == 0u32 {
            return (float_nan!(), Equal);
        }
        return if *x < 1u32 {
            (Float::ZERO, Equal)
        } else {
            (Float::NEGATIVE_ZERO, Equal)
        };
    }
    if *base == 1u32 {
        // ln(base) = +0. ln(x) / +0 = +-infinity by the sign of ln(x), or NaN for ln(x) = +0.
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
    if *x == 0u32 {
        // ln(0) = -infinity. -infinity / ln(base): -infinity for base > 1, +infinity for base < 1.
        return if *base < 1u32 {
            (float_infinity!(), Equal)
        } else {
            (float_negative_infinity!(), Equal)
        };
    }
    // x is a positive Rational and base is positive finite and not 1.
    log_base_rational_float_base_normal(x, base, prec, rm)
}

impl Float {
    /// Computes $\log_b x$, where $x$ is a [`Rational`] and the base $b$ is a [`Float`], returning
    /// a [`Float`] rounded to the specified precision and with the specified rounding mode. Both
    /// are taken by value. An [`Ordering`] is also returned, indicating whether the rounded value
    /// is less than, equal to, or greater than the exact value. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The base may be any [`Float`]: the function is defined as $\ln x / \ln b$ for every
    /// [`Rational`] $x$ and [`Float`] $b$, applying IEEE division to the natural logs, and never
    /// panics on an input value. In particular a base in $(0,1)$ gives a (sign-flipped) logarithm,
    /// and the non-normal and degenerate bases follow the limits below.
    ///
    /// This computes $\log_2 x / \log_2 b$, evaluating $\log_2 x$ in an extended exponent range (so
    /// an $x$ near 1 does not lose accuracy) and wrapping the quotient so it may overflow (base
    /// near 1) or underflow (x near 1) and be clamped exactly once.
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
    /// - $f(x,\text{NaN},p,m)=\text{NaN}$
    /// - $f(x,b,p,m)=\text{NaN}$ for $x<0$ or $b<0$ (including $b=-\infty$)
    /// - $f(0,b,p,m)=-\infty$ for $b>1$, and $\infty$ for $0<b<1$ (and $\text{NaN}$ for
    ///   $b\in\{\infty,\pm0.0\}$)
    /// - $f(1,b,p,m)=0$ (with the sign of $1/\ln b$)
    /// - $f(x,\infty,p,m)=0$ for $x>0$ (and $\text{NaN}$ for $x=0$)
    /// - $f(x,\pm0.0,p,m)=0$ for $x>0$ (and $\text{NaN}$ for $x=0$)
    /// - $f(x,1.0,p,m)=\infty$ for $x>1$, $-\infty$ for $0\leq x<1$, and $\text{NaN}$ for $x=1$
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
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::log_base_rational_float_base_prec_round(
    ///     Rational::from(8),
    ///     Float::from(4),
    ///     10,
    ///     Exact,
    /// );
    /// assert_eq!(log.to_string(), "1.5"); // log_4(8) = 3/2
    /// assert_eq!(o, Equal);
    ///
    /// let (log, o) = Float::log_base_rational_float_base_prec_round(
    ///     Rational::from(4),
    ///     Float::from(0.5),
    ///     10,
    ///     Exact,
    /// );
    /// assert_eq!(log.to_string(), "-2.0"); // log_{1/2}(4) = -2
    /// assert_eq!(o, Equal);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn log_base_rational_float_base_prec_round(
        x: Rational,
        base: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::log_base_rational_float_base_prec_round_ref(&x, &base, prec, rm)
    }

    /// Computes $\log_b x$, where $x$ is a [`Rational`] and the base $b$ is a [`Float`], returning
    /// a [`Float`] rounded to the specified precision and with the specified rounding mode. Both
    /// are taken by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// value is less than, equal to, or greater than the exact value.
    ///
    /// See [`Float::log_base_rational_float_base_prec_round`] for details, special cases, and a
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
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::log_base_rational_float_base_prec_round_ref(
    ///     &Rational::from(9),
    ///     &Float::from(3),
    ///     10,
    ///     Exact,
    /// );
    /// assert_eq!(log.to_string(), "2.0"); // log_3(9) = 2
    /// assert_eq!(o, Equal);
    ///
    /// let (log, o) = Float::log_base_rational_float_base_prec_round_ref(
    ///     &Rational::from_signeds(1, 3),
    ///     &Float::from(3),
    ///     10,
    ///     Exact,
    /// );
    /// assert_eq!(log.to_string(), "-1.0"); // log_3(1/3) = -1
    /// assert_eq!(o, Equal);
    /// ```
    pub fn log_base_rational_float_base_prec_round_ref(
        x: &Rational,
        base: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        log_base_rational_float_base_helper(x, base, prec, rm)
    }

    /// Computes $\log_b x$, where $x$ is a [`Rational`] and the base $b$ is a [`Float`], returning
    /// a [`Float`] rounded to the nearest value of the specified precision. Both are taken by
    /// value. An [`Ordering`] is also returned.
    ///
    /// See [`Float::log_base_rational_float_base_prec_round`] for details and special cases.
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
    /// let (log, o) =
    ///     Float::log_base_rational_float_base_prec(Rational::from(8), Float::from(4), 10);
    /// assert_eq!(log.to_string(), "1.5"); // log_4(8) = 3/2
    /// assert_eq!(o, Equal);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn log_base_rational_float_base_prec(
        x: Rational,
        base: Self,
        prec: u64,
    ) -> (Self, Ordering) {
        Self::log_base_rational_float_base_prec_round_ref(&x, &base, prec, Nearest)
    }

    /// Computes $\log_b x$, where $x$ is a [`Rational`] and the base $b$ is a [`Float`], returning
    /// a [`Float`] rounded to the nearest value of the specified precision. Both are taken by
    /// reference. An [`Ordering`] is also returned.
    ///
    /// See [`Float::log_base_rational_float_base_prec_round`] for details and special cases.
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
    /// let (log, o) =
    ///     Float::log_base_rational_float_base_prec_ref(&Rational::from(9), &Float::from(3), 10);
    /// assert_eq!(log.to_string(), "2.0"); // log_3(9) = 2
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_rational_float_base_prec_ref(
        x: &Rational,
        base: &Self,
        prec: u64,
    ) -> (Self, Ordering) {
        Self::log_base_rational_float_base_prec_round_ref(x, base, prec, Nearest)
    }
}

/// Computes $\log_b x$, the base-$b$ logarithm of a [`Rational`], where the base $b$ is a primitive
/// float, returning a primitive float result. Using this function is more accurate than computing
/// the logarithm using the standard library, whose logarithm functions are not always correctly
/// rounded.
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
/// - $f(x,\text{NaN})=\text{NaN}$
/// - $f(x,b)=\text{NaN}$ for $x<0$ or $b<0$ (including $b=-\infty$)
/// - $f(0,b)=-\infty$ for $b>1$, and $\infty$ for $0<b<1$ (and $\text{NaN}$ for
///   $b\in\{\infty,\pm0.0\}$)
/// - $f(1,b)=0.0$ (with the sign of $1/\ln b$)
/// - $f(x,\infty)=0.0$ for $x>0$ (and $\text{NaN}$ for $x=0$)
/// - $f(x,\pm0.0)=0.0$ for $x>0$ (and $\text{NaN}$ for $x=0$)
/// - $f(x,1.0)=\infty$ for $x>1$, $-\infty$ for $0\leq x<1$, and $\text{NaN}$ for $x=1$
///
/// This function can both overflow (for a base near 1) and underflow (for an $x$ near 1).
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::arithmetic::log_base_rational_float_base::*;
/// use malachite_q::Rational;
///
/// // log_4(8) = 3/2
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_rational_float_base::<f32>(
///         &Rational::from(8),
///         4.0
///     )),
///     NiceFloat(1.5)
/// );
/// // log_(1/2)(4) = -2
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_rational_float_base::<f32>(
///         &Rational::from(4),
///         0.5
///     )),
///     NiceFloat(-2.0)
/// );
/// // log_10(1/3)
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_rational_float_base::<f32>(
///         &Rational::from_unsigneds(1u8, 3),
///         10.0
///     )),
///     NiceFloat(-0.47712126)
/// );
/// assert!(
///     primitive_float_log_base_rational_float_base::<f32>(&Rational::from(-1), 10.0).is_nan()
/// );
/// assert!(
///     primitive_float_log_base_rational_float_base::<f32>(&Rational::from(8), f32::NAN).is_nan()
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_log_base_rational_float_base<T: PrimitiveFloat>(x: &Rational, base: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_float_to_float_fn(
        |x, base, prec| Float::log_base_rational_float_base_prec_ref(x, &base, prec),
        x,
        base,
    )
}
