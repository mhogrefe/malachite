// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Infinity, NaN, Zero};
use crate::arithmetic::log_base::{dyadic_1p_log_of_root, rational_root_parts};
use crate::arithmetic::log_base_2::extended_log_base_2_of_rational;
use crate::basic::extended::ExtendedFloat;
use crate::{Float, emulate_float_to_float_fn, float_infinity, float_nan, float_negative_infinity};
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{
    CeilingLogBase2, LogBaseOf1PlusX, LogBaseOf1PlusXAssign,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero as ZeroTrait;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::factorization::traits::ExpressAsPower;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::arithmetic::float_extras::float_can_round;
use malachite_nz::platform::Limb;
use malachite_q::Rational;

// Returns `Some(m / e_base)` -- the value of `log_base(1 + x)` -- when `1 + x = g^m` for the
// primitive root `g` of `base` (so `base = g^e_base` and `log_base(1 + x)` is rational), and `None`
// when it is irrational. The input `x` must be finite and greater than -1, and `base` must be
// greater than 1.
//
// Unlike the integer-base case, `g` may be a (dyadic) fraction such as 3/2, so `1 + x = g^m` can be
// an exact `Float` value for an `m` of either sign or for an `x` that is not an integer (for
// example `1 + 1/2 = (3/2)^1`). Forming `1 + x` exactly as a `Rational` and calling
// `Rational::checked_log_base` covers all of these uniformly.
//
// Detecting these rational results up front is essential: the Ziv loop in
// `log_base_rational_base_1_plus_x_prec_round_normal` could never certify an exactly-representable
// one. The check is balloon-safe via the same exponent/size bound as the non-`1 + x` sibling: when
// `x`'s exponent (so `1 + x`'s magnitude) or `base`'s bit length exceeds `64 * x.get_prec()`, no
// representable power relationship is possible at this precision, so it is left to the Ziv loop and
// `1 + x` is never materialized. (An `x` near -1 has a near-zero exponent and is materialized, but
// its size is then bounded by `x`'s precision.)
pub(crate) fn rational_log_base_rational_base_1_plus_x(
    x: &Float,
    base: &Rational,
) -> Option<Rational> {
    if *x == 0u32 {
        return Some(Rational::ZERO);
    }
    // `express_as_power` returns `None` when `base` is not a perfect power, in which case `base`
    // itself is `g` (with exponent 1); its cost is polynomial in `base`, which the caller holds
    // materialized. `1 + x` is dyadic, so only the orientation of the root with no odd denominator
    // (for positive powers) or no odd numerator (for negative powers) can match; `1 + x` itself is
    // matched implicitly (see `dyadic_1p_log_of_root`), since its integer form can be enormous even
    // when `x` has few bits. No size cutoff: skipping the check when the result is exactly
    // representable would leave the Ziv loop unable to terminate.
    let (root, e_base) = base.express_as_power().unwrap_or_else(|| (base.clone(), 1));
    let (z, hn, hd) = rational_root_parts(&root);
    let m = if hd == 1u32 {
        dyadic_1p_log_of_root(x, z, &hn)
    } else if hn == 1u32 {
        dyadic_1p_log_of_root(x, -z, &hd).map(|m| -m)
    } else {
        // Both an odd numerator and an odd denominator: no nonzero power is dyadic.
        None
    }?;
    Some(Rational::from_signeds(m, i64::exact_from(e_base)))
}

// The computation of log_base(1 + x) for a `Rational` base is done by log_base(1 + x) = log_2(1 +
// x) / log_2(base). The input is finite and greater than -1, and `base` is greater than 1.
//
// Routing through `log_base_2_1_plus_x` (rather than forming `1 + x` and taking its log) preserves
// accuracy for x near 0. `log_2(base)` is computed in the extended exponent range (see
// `extended_log_base_2_of_rational`) so that a base near 1 -- where `log_2(base)` is tiny and would
// otherwise underflow an ordinary `Float`, losing the operand -- is represented faithfully. The
// quotient is also kept extended, and the single conversion back to a `Float`, via
// `ExtendedFloat::into_float_helper`, performs the one correctly-rounded clamp. Unlike an integer
// base, a `Rational` base allows both overflow (base near 1, so `log_2(base)` is tiny and the
// quotient is huge) and underflow (a large base dividing a tiny `log_2(1 + x)` for x near 0); both
// are handled by that clamp. (`log_2(1 + x)` itself never underflows: x is a `Float`, so `|x|` is
// at least the smallest positive `Float`, keeping `|log_2(1 + x)|` representable.)
fn log_base_rational_base_1_plus_x_prec_round_normal(
    x: &Float,
    base: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    // log_base(1 + x) is undefined for x < -1.
    match x.partial_cmp(&-1i32).unwrap() {
        // 1 + x = 0, so log_base(1 + x) = -infinity (base > 1).
        Equal => return (float_negative_infinity!(), Equal),
        Less => return (float_nan!(), Equal),
        _ => {}
    }
    // If 1 + x = g^m, then log_base(1 + x) = m / e_base is rational and exact.
    if let Some(q) = rational_log_base_rational_base_1_plus_x(x, base) {
        return Float::from_rational_prec_round(q, prec, rm);
    }
    // The result is irrational, so it is never exactly representable.
    assert_ne!(rm, Exact, "Inexact log_base_rational_base_1_plus_x");
    // The initial slack keeps working_prec at least 7, so the working_prec - 6 below stays
    // positive.
    let mut working_prec = prec + 6 + prec.ceiling_log_base_2();
    let mut increment = Limb::WIDTH;
    loop {
        // log_2(1 + x), correctly rounded to working_prec; nonzero (x is not 0) and never
        // underflowing, so the ordinary log wrapped as an ExtendedFloat suffices.
        let num = ExtendedFloat::from(x.log_base_2_1_plus_x_prec_ref(working_prec).0);
        // log_2(base) > 0, extended (may be tiny for a base near 1).
        let den = extended_log_base_2_of_rational(base, working_prec);
        // log_2(1 + x) / log_2(base) in the extended range; cannot overflow or underflow here.
        let (quotient, _) = num.div_prec_val_ref(&den, working_prec);
        // log_2(1 + x) is correctly rounded (<= 1/2 ulp), log_2(base) is within 2 ulps, and the
        // division adds at most 1 more, for at most 4 ulps total; working_prec - 6 correct bits
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

impl Float {
    /// Computes $\log_b(1+x)$, where $x$ is a [`Float`] and $b$ is a [`Rational`] greater than 1,
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`Float`] is taken by value and the base by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $\log_b(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, `NaN` is returned.
    ///
    /// This computes $\log_2(1+x) / \log_2 b$, routing through
    /// [`Float::log_base_2_1_plus_x_prec_ref`] to preserve accuracy for $x$ near 0, and evaluating
    /// $\log_2 b$ in an extended exponent range so that a base near 1 (where $\log_2 b$ is tiny)
    /// does not lose accuracy. The single conversion of the quotient back to a [`Float`] performs
    /// the one correctly-rounded clamp.
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
    /// Special cases:
    /// - $f(\text{NaN},b,p,m)=\text{NaN}$
    /// - $f(\infty,b,p,m)=\infty$
    /// - $f(-\infty,b,p,m)=\text{NaN}$
    /// - $f(\pm0.0,b,p,m)=\pm0.0$
    /// - $f(-1.0,b,p,m)=-\infty$
    /// - $f(x,b,p,m)=\text{NaN}$ for $x<-1$
    /// - $f(x,b,p,m)=m/e$ when $1+x=g^m$, where $g$ is the primitive root of $b$ and $b=g^e$,
    ///   rounded to precision $p$; the result is exact if and only if $m/e$ is representable with
    ///   precision $p$ (for example $\log_4(1+1)=1/2$ is exact)
    ///
    /// Unlike a logarithm with an integer base, this function can both overflow (for a base near 1)
    /// and underflow (for an $x$ near 0).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, if `base` is less than or equal to 1, or if `rm` is `Exact` but
    /// the result cannot be represented exactly with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::from(3).log_base_rational_base_1_plus_x_prec_round(
    ///     &Rational::from(4),
    ///     10,
    ///     Exact,
    /// );
    /// assert_eq!(log.to_string(), "1.0"); // log_4(1 + 3) = log_4(4) = 1
    /// assert_eq!(o, Equal);
    ///
    /// let (log, o) = Float::from(1).log_base_rational_base_1_plus_x_prec_round(
    ///     &Rational::from(4),
    ///     10,
    ///     Exact,
    /// );
    /// assert_eq!(log.to_string(), "0.5"); // log_4(1 + 1) = log_4(2) = 1/2
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_rational_base_1_plus_x_prec_round(
        self,
        base: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        assert!(*base > 1u32, "Logarithm base must be greater than 1");
        match self {
            Self(NaN | Infinity { sign: false }) => (float_nan!(), Equal),
            float_infinity!() => (float_infinity!(), Equal),
            Self(Zero { .. }) => (self, Equal),
            _ => log_base_rational_base_1_plus_x_prec_round_normal(&self, base, prec, rm),
        }
    }

    /// Computes $\log_b(1+x)$, where $x$ is a [`Float`] and $b$ is a [`Rational`] greater than 1,
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`Float`] and the base are both taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value.
    ///
    /// See [`Float::log_base_rational_base_1_plus_x_prec_round`] for details, special cases, and a
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
    /// Panics if `prec` is zero, if `base` is less than or equal to 1, or if `rm` is `Exact` but
    /// the result cannot be represented exactly with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = (&Float::from(8)).log_base_rational_base_1_plus_x_prec_round_ref(
    ///     &Rational::from(3),
    ///     10,
    ///     Exact,
    /// );
    /// assert_eq!(log.to_string(), "2.0"); // log_3(1 + 8) = log_3(9) = 2
    /// assert_eq!(o, Equal);
    ///
    /// let (log, o) = (&Float::from(1)).log_base_rational_base_1_plus_x_prec_round_ref(
    ///     &Rational::from(3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(log.to_string(), "0.630929"); // log_3(2), rounded down
    /// assert_eq!(o, Less);
    /// ```
    pub fn log_base_rational_base_1_plus_x_prec_round_ref(
        &self,
        base: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        assert!(*base > 1u32, "Logarithm base must be greater than 1");
        match self {
            Self(NaN | Infinity { sign: false }) => (float_nan!(), Equal),
            float_infinity!() => (float_infinity!(), Equal),
            Self(Zero { .. }) => (self.clone(), Equal),
            _ => log_base_rational_base_1_plus_x_prec_round_normal(self, base, prec, rm),
        }
    }

    /// Computes $\log_b(1+x)$, where $x$ is a [`Float`] and $b$ is a [`Rational`] greater than 1,
    /// rounding the result to the nearest value of the specified precision. The [`Float`] is taken
    /// by value and the base by reference. An [`Ordering`] is also returned.
    ///
    /// See [`Float::log_base_rational_base_1_plus_x_prec_round`] for details and special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `base` is less than or equal to 1.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::from(8).log_base_rational_base_1_plus_x_prec(&Rational::from(3), 10);
    /// assert_eq!(log.to_string(), "2.0"); // log_3(1 + 8) = log_3(9) = 2
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_rational_base_1_plus_x_prec(
        self,
        base: &Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        self.log_base_rational_base_1_plus_x_prec_round(base, prec, Nearest)
    }

    /// Computes $\log_b(1+x)$, where $x$ is a [`Float`] and $b$ is a [`Rational`] greater than 1,
    /// rounding the result to the nearest value of the specified precision. The [`Float`] and the
    /// base are both taken by reference. An [`Ordering`] is also returned.
    ///
    /// See [`Float::log_base_rational_base_1_plus_x_prec_round`] for details and special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `base` is less than or equal to 1.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) =
    ///     (&Float::from(8)).log_base_rational_base_1_plus_x_prec_ref(&Rational::from(3), 10);
    /// assert_eq!(log.to_string(), "2.0"); // log_3(1 + 8) = log_3(9) = 2
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_rational_base_1_plus_x_prec_ref(
        &self,
        base: &Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        self.log_base_rational_base_1_plus_x_prec_round_ref(base, prec, Nearest)
    }

    /// Computes $\log_b(1+x)$, where $x$ is a [`Float`] and $b$ is a [`Rational`] greater than 1,
    /// rounding the result to the precision of the input and with the specified rounding mode. The
    /// [`Float`] is taken by value and the base by reference. An [`Ordering`] is also returned.
    ///
    /// See [`Float::log_base_rational_base_1_plus_x_prec_round`] for details and special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the precision of the input.
    ///
    /// # Panics
    /// Panics if `base` is less than or equal to 1, or if `rm` is `Exact` but the result cannot be
    /// represented exactly with the input's precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) =
    ///     Float::from(8).log_base_rational_base_1_plus_x_round(&Rational::from(3), Exact);
    /// assert_eq!(log.to_string(), "2.0"); // log_3(1 + 8) = log_3(9) = 2
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_rational_base_1_plus_x_round(
        self,
        base: &Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.log_base_rational_base_1_plus_x_prec_round(base, prec, rm)
    }

    /// Computes $\log_b(1+x)$, where $x$ is a [`Float`] and $b$ is a [`Rational`] greater than 1,
    /// rounding the result to the precision of the input and with the specified rounding mode. The
    /// [`Float`] and the base are both taken by reference. An [`Ordering`] is also returned.
    ///
    /// See [`Float::log_base_rational_base_1_plus_x_prec_round`] for details and special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the precision of the input.
    ///
    /// # Panics
    /// Panics if `base` is less than or equal to 1, or if `rm` is `Exact` but the result cannot be
    /// represented exactly with the input's precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) =
    ///     (&Float::from(8)).log_base_rational_base_1_plus_x_round_ref(&Rational::from(3), Exact);
    /// assert_eq!(log.to_string(), "2.0"); // log_3(1 + 8) = log_3(9) = 2
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_rational_base_1_plus_x_round_ref(
        &self,
        base: &Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        self.log_base_rational_base_1_plus_x_prec_round_ref(base, self.significant_bits(), rm)
    }

    /// Computes $\log_b(1+x)$, where $x$ is a [`Float`] and $b$ is a [`Rational`] greater than 1,
    /// in place, rounding the result to the specified precision and with the specified rounding
    /// mode. The base is taken by reference. An [`Ordering`] is returned.
    ///
    /// See [`Float::log_base_rational_base_1_plus_x_prec_round`] for details and special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, if `base` is less than or equal to 1, or if `rm` is `Exact` but
    /// the result cannot be represented exactly with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(8);
    /// assert_eq!(
    ///     x.log_base_rational_base_1_plus_x_prec_round_assign(&Rational::from(3), 10, Exact),
    ///     Equal
    /// );
    /// assert_eq!(x.to_string(), "2.0"); // log_3(1 + 8) = log_3(9) = 2
    /// ```
    #[inline]
    pub fn log_base_rational_base_1_plus_x_prec_round_assign(
        &mut self,
        base: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (result, o) =
            core::mem::take(self).log_base_rational_base_1_plus_x_prec_round(base, prec, rm);
        *self = result;
        o
    }

    /// Computes $\log_b(1+x)$, where $x$ is a [`Float`] and $b$ is a [`Rational`] greater than 1,
    /// in place, rounding the result to the nearest value of the specified precision. The base is
    /// taken by reference. An [`Ordering`] is returned.
    ///
    /// See [`Float::log_base_rational_base_1_plus_x_prec_round`] for details and special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `base` is less than or equal to 1.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::from(8);
    /// x.log_base_rational_base_1_plus_x_prec_assign(&Rational::from(3), 10);
    /// assert_eq!(x.to_string(), "2.0"); // log_3(1 + 8) = log_3(9) = 2
    /// ```
    #[inline]
    pub fn log_base_rational_base_1_plus_x_prec_assign(
        &mut self,
        base: &Rational,
        prec: u64,
    ) -> Ordering {
        self.log_base_rational_base_1_plus_x_prec_round_assign(base, prec, Nearest)
    }

    /// Computes $\log_b(1+x)$, where $x$ is a [`Float`] and $b$ is a [`Rational`] greater than 1,
    /// in place, rounding the result to the precision of the input and with the specified rounding
    /// mode. The base is taken by reference. An [`Ordering`] is returned.
    ///
    /// See [`Float::log_base_rational_base_1_plus_x_prec_round`] for details and special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the precision of the input.
    ///
    /// # Panics
    /// Panics if `base` is less than or equal to 1, or if `rm` is `Exact` but the result cannot be
    /// represented exactly with the input's precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::from(8);
    /// x.log_base_rational_base_1_plus_x_round_assign(&Rational::from(3), Exact);
    /// assert_eq!(x.to_string(), "2.0"); // log_3(1 + 8) = log_3(9) = 2
    /// ```
    #[inline]
    pub fn log_base_rational_base_1_plus_x_round_assign(
        &mut self,
        base: &Rational,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = self.significant_bits();
        self.log_base_rational_base_1_plus_x_prec_round_assign(base, prec, rm)
    }
}

impl LogBaseOf1PlusX<Rational> for Float {
    type Output = Self;

    /// Computes $\log_b(1+x)$, where $x$ is a [`Float`] and $b$ is a [`Rational`] greater than 1,
    /// rounding the result to the nearest value of the input's precision. Both are taken by value.
    ///
    /// See [`Float::log_base_rational_base_1_plus_x_prec_round`] for special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the precision of the input.
    ///
    /// # Panics
    /// Panics if `base` is less than or equal to 1.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::LogBaseOf1PlusX;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// // log_3(1 + 8) = log_3(9) = 2
    /// assert_eq!(
    ///     Float::from(8)
    ///         .log_base_1_plus_x(Rational::from(3))
    ///         .to_string(),
    ///     "2.0"
    /// );
    /// ```
    #[inline]
    fn log_base_1_plus_x(self, base: Rational) -> Self {
        let prec = self.significant_bits();
        self.log_base_rational_base_1_plus_x_prec_round(&base, prec, Nearest)
            .0
    }
}

impl LogBaseOf1PlusX<&Rational> for &Float {
    type Output = Float;

    /// Computes $\log_b(1+x)$, where $x$ is a [`Float`] and $b$ is a [`Rational`] greater than 1,
    /// rounding the result to the nearest value of the input's precision. Both are taken by
    /// reference.
    ///
    /// See [`Float::log_base_rational_base_1_plus_x_prec_round`] for special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the precision of the input.
    ///
    /// # Panics
    /// Panics if `base` is less than or equal to 1.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::LogBaseOf1PlusX;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// // log_3(1 + 8) = log_3(9) = 2
    /// assert_eq!(
    ///     (&Float::from(8))
    ///         .log_base_1_plus_x(&Rational::from(3))
    ///         .to_string(),
    ///     "2.0"
    /// );
    /// ```
    #[inline]
    fn log_base_1_plus_x(self, base: &Rational) -> Float {
        self.log_base_rational_base_1_plus_x_prec_round_ref(base, self.significant_bits(), Nearest)
            .0
    }
}

impl LogBaseOf1PlusXAssign<&Rational> for Float {
    /// Replaces a [`Float`] $x$ with $\log_b(1+x)$, where $b$ is a [`Rational`] greater than 1,
    /// rounding the result to the nearest value of the input's precision. The base is taken by
    /// reference.
    ///
    /// See [`Float::log_base_rational_base_1_plus_x_prec_round`] for special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the precision of the input.
    ///
    /// # Panics
    /// Panics if `base` is less than or equal to 1.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::LogBaseOf1PlusXAssign;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::from(8);
    /// x.log_base_1_plus_x_assign(&Rational::from(3));
    /// assert_eq!(x.to_string(), "2.0"); // log_3(1 + 8) = log_3(9) = 2
    /// ```
    #[inline]
    fn log_base_1_plus_x_assign(&mut self, base: &Rational) {
        let prec = self.significant_bits();
        self.log_base_rational_base_1_plus_x_prec_round_assign(base, prec, Nearest);
    }
}

/// Computes $\log_b(1+x)$, the base-$b$ logarithm of one plus a primitive float, where the base $b$
/// is a [`Rational`] greater than 1, returning a primitive float result. Using this function is
/// more accurate than computing the logarithm using the standard library, both because $1+x$ may
/// not be representable as a primitive float and because the standard library's `log` is not always
/// correctly rounded.
///
/// $\log_b(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, `NaN` is returned.
///
/// $$
/// f(x,b) = \log_b(1+x)+\varepsilon.
/// $$
/// - If $\log_b(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $\log_b(1+x)$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
///   |\log_b(1+x)|\rfloor-p}$, where $p$ is precision of the output (typically 24 if `T` is a
///   [`f32`] and 53 if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(\text{NaN},b)=\text{NaN}$
/// - $f(\infty,b)=\infty$
/// - $f(-\infty,b)=\text{NaN}$
/// - $f(\pm0.0,b)=\pm0.0$
/// - $f(-1.0,b)=-\infty$
/// - $f(x,b)=\text{NaN}$ for $x<-1$
///
/// Unlike a logarithm with an integer base, this function can both overflow (for a base near 1) and
/// underflow (for an $x$ near 0).
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `base` is less than or equal to 1.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::arithmetic::log_base_rational_base_1_plus_x::*;
/// use malachite_q::Rational;
///
/// assert!(
///     primitive_float_log_base_rational_base_1_plus_x(f32::NAN, &Rational::from(10)).is_nan()
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_rational_base_1_plus_x(
///         f32::INFINITY,
///         &Rational::from(10)
///     )),
///     NiceFloat(f32::INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_rational_base_1_plus_x(
///         -1.0f32,
///         &Rational::from(10)
///     )),
///     NiceFloat(f32::NEGATIVE_INFINITY)
/// );
/// assert!(primitive_float_log_base_rational_base_1_plus_x(-2.0f32, &Rational::from(10)).is_nan());
/// // log_4(1 + 3) = log_4(4) = 1
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_rational_base_1_plus_x(
///         3.0f32,
///         &Rational::from(4)
///     )),
///     NiceFloat(1.0)
/// );
/// // log_4(1 + 1) = log_4(2) = 1/2
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_rational_base_1_plus_x(
///         1.0f32,
///         &Rational::from(4)
///     )),
///     NiceFloat(0.5)
/// );
/// // log_(3/2)(1 + 1/2) = log_(3/2)(3/2) = 1
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_rational_base_1_plus_x(
///         0.5f32,
///         &Rational::from_unsigneds(3u8, 2)
///     )),
///     NiceFloat(1.0)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_log_base_rational_base_1_plus_x<T: PrimitiveFloat>(
    x: T,
    base: &Rational,
) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(
        |x, prec| x.log_base_rational_base_1_plus_x_prec(base, prec),
        x,
    )
}
