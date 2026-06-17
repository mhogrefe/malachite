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
use malachite_base::num::arithmetic::traits::{CeilingLogBase2, CheckedLogBase, LogBase, LogBaseAssign};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero as ZeroTrait;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::factorization::traits::ExpressAsPower;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::arithmetic::float_extras::float_can_round;
use malachite_nz::platform::Limb;
use malachite_q::Rational;

// Returns `Some(log_base(x))` when it is rational, and `None` when it is irrational. The input `x`
// must be finite, positive, and not equal to 1, and `base` must be greater than 1.
//
// `log_base(x)` is rational exactly when `x` and `base` are both powers of a common rational `g`,
// say `x = g^a` and `base = g^e_base`; then `log_base(x) = a / e_base`. Taking `g` to be the
// primitive root of `base` (`base.express_as_power()`), this holds iff `x` is an integer power of
// `g`, found by `Rational::checked_log_base` (which also covers `x < 1`, giving a negative `a`).
//
// Detecting these rational results up front is essential, not just an optimization: when the result
// is exactly representable (for example `log_9(3) = 1/2`), the Ziv loop in
// `log_base_rational_base_prec_round_normal` would never terminate, because the rounding test can
// never certify a value sitting exactly on a representable point or tie.
//
// The check is balloon-safe. If `x = g^a` then `x`'s bit length is about `|a| * log2(g)`, and a
// representable result needs `|a|` within about `e_base * prec` of `x`'s precision; an `x` worth
// materializing as a `Rational` therefore has bit length at most about `64 * x.get_prec()`. When
// `x`'s exponent (or `base`'s size) exceeds that bound, `x` cannot be an exact power of `g` at this
// precision, so it is left to the Ziv loop (which then converges) and is never materialized.
pub(crate) fn rational_log_base_rational_base(x: &Float, base: &Rational) -> Option<Rational> {
    let x_prec = x.get_prec().unwrap();
    let bound = x_prec.saturating_mul(64);
    let e = i64::from(x.get_exponent().unwrap());
    if e.unsigned_abs() > bound || base.significant_bits() > bound {
        return None;
    }
    // `express_as_power` returns `None` when `base` is not a perfect power, in which case `base`
    // itself is `g` (with exponent 1).
    let (root, e_base) = base.express_as_power().unwrap_or_else(|| (base.clone(), 1));
    let a = (&Rational::exact_from(x)).checked_log_base(&root)?;
    Some(Rational::from_signeds(a, i64::exact_from(e_base)))
}

// The computation of log_base(x) for a `Rational` base is done by log_base(x) = log_2(x) /
// log_2(base). The input is finite, nonzero, and positive, and `base` is greater than 1.
//
// Routing the base through `log_base_2_rational` (rather than computing `ln(x) / ln(base)` directly)
// reuses its handling of a base near 1, where `log_2(base)` is tiny and a direct computation would
// suffer catastrophic cancellation. Unlike an integer base, a `Rational` base allows both overflow
// (base near 1, so `log_2(base)` is tiny and the quotient is huge) and underflow (x near 1, so
// `log_2(x)` is tiny); both are detected by an exponent bound and handed to `div_prec_round`, which
// clamps to the appropriate infinity/maximum or zero/minimum per the rounding mode.
fn log_base_rational_base_prec_round_normal(
    x: &Float,
    base: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    // If x is 1, the result is 0.
    if *x == 1u32 {
        return (Float::ZERO, Equal);
    }
    // If log_base(x) is rational -- x and base are both powers of a common rational -- compute it
    // directly. This includes exactly-representable results (which the Ziv loop could never
    // certify) as well as non-representable rationals (cheaper and exact this way).
    if let Some(q) = rational_log_base_rational_base(x, base) {
        return Float::from_rational_prec_round(q, prec, rm);
    }
    // The result is irrational, so it is never exactly representable.
    assert_ne!(rm, Exact, "Inexact log_base_rational_base");
    let max_exp = i64::from(Float::MAX_EXPONENT);
    let min_exp = i64::from(Float::MIN_EXPONENT);
    let mut working_prec = prec + 4 + prec.ceiling_log_base_2();
    let mut increment = Limb::WIDTH;
    loop {
        // log_2(x), correctly rounded to working_prec; finite and nonzero (x is positive and not 1).
        let num = x.log_base_2_prec_ref(working_prec).0;
        // log_2(base) > 0, correctly rounded to working_prec.
        let den = Float::log_base_2_rational_prec_ref(base, working_prec).0;
        // The quotient's exponent is e_num - e_den or one more. Detect overflow (too large to
        // represent) and underflow (too small), where the Ziv test below could never resolve the
        // clamped result, and hand the rounding to div_prec_round. Overflow needs e_num - e_den >
        // MAX_EXPONENT; the boundary e_num - e_den == MAX_EXPONENT never overflows, because
        // |log_2(x)| <= 2^30 forces the quotient's mantissa below 1 there. Underflow mirrors the
        // div.rs handling: the narrow band where the cheap bound is inconclusive is resolved by the
        // shift-and-compare (which only adjusts exponents, avoiding a huge Rational conversion).
        //
        // This clamp is only reached by inputs within ~2^(-2^30) of 1 (a base near 1 for overflow,
        // an x near 1 for underflow), which require multi-hundred-megabyte operands and so are
        // beyond the property tests' range; the underflow logic is the same band exercised and
        // verified in log_base_1_plus_x.
        let e_num = i64::from(num.get_exponent().unwrap());
        let e_den = i64::from(den.get_exponent().unwrap());
        let d = e_num - e_den;
        if d > max_exp
            || d + 1 < min_exp
            || (d < min_exp && (&num << u64::exact_from(1 - min_exp)).lt_abs(&den))
        {
            return num.div_prec_round(den, prec, rm);
        }
        // log_2(x) / log_2(base), with three correctly-rounded operations (log_base_2,
        // log_base_2_rational, and the division, each at most 1/2 ulp), so the relative error is
        // below 2^(2 - working_prec) and working_prec - 4 correct bits suffice for rounding.
        let t = num.div_prec(den, working_prec).0;
        if float_can_round(t.significand_ref().unwrap(), working_prec - 4, prec, rm) {
            return Float::from_float_prec_round(t, prec, rm);
        }
        // Increase the precision.
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

impl Float {
    /// Computes $\log_b x$, where $x$ is a [`Float`] and $b$ is a [`Rational`] greater than 1,
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`Float`] is taken by value and the base by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact value.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it
    /// also returns `Equal`.
    ///
    /// This computes $\log_2 x / \log_2 b$, routing the base through
    /// [`Float::log_base_2_rational_prec_ref`] so that a base near 1 (where $\log_2 b$ is tiny) does
    /// not lose accuracy to cancellation.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,b,p,m) = \log_b x+\varepsilon.
    /// $$
    /// - If $\log_b x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\log_b x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_b x|\rfloor-p+1}$.
    /// - If $\log_b x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_b x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},b,p,m)=\text{NaN}$
    /// - $f(\infty,b,p,m)=\infty$
    /// - $f(-\infty,b,p,m)=\text{NaN}$
    /// - $f(\pm0.0,b,p,m)=-\infty$
    /// - $f(x,b,p,m)=\text{NaN}$ for $x<0$
    /// - $f(1.0,b,p,m)=0$
    /// - $f(x,b,p,m)=a/e$ when $x=g^a$, where $g$ is the primitive root of $b$ and $b=g^e$, rounded
    ///   to precision $p$; the result is exact if and only if $a/e$ is representable with precision
    ///   $p$ (for example $\log_4 8=3/2$ is exact)
    ///
    /// Unlike a logarithm with an integer base, this function can both overflow (for a base near 1)
    /// and underflow (for an $x$ near 1).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, if `base` is less than or equal to 1, or if `rm` is `Exact` but the
    /// result cannot be represented exactly with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::from(8).log_base_rational_base_prec_round(&Rational::from(4), 10, Exact);
    /// assert_eq!(log.to_string(), "1.5"); // log_4(8) = 3/2
    /// assert_eq!(o, Equal);
    ///
    /// let (log, o) = Float::from(9).log_base_rational_base_prec_round(&Rational::from(3), 10, Exact);
    /// assert_eq!(log.to_string(), "2.0"); // log_3(9) = 2
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_rational_base_prec_round(
        self,
        base: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        assert!(*base > 1u32, "Logarithm base must be greater than 1");
        match self {
            Self(NaN | Infinity { sign: false } | Finite { sign: false, .. }) => {
                (float_nan!(), Equal)
            }
            float_either_zero!() => (float_negative_infinity!(), Equal),
            float_infinity!() => (float_infinity!(), Equal),
            _ => log_base_rational_base_prec_round_normal(&self, base, prec, rm),
        }
    }

    /// Computes $\log_b x$, where $x$ is a [`Float`] and $b$ is a [`Rational`] greater than 1,
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`Float`] and the base are both taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact value.
    ///
    /// See [`Float::log_base_rational_base_prec_round`] for details, special cases, and a
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
    /// Panics if `prec` is zero, if `base` is less than or equal to 1, or if `rm` is `Exact` but the
    /// result cannot be represented exactly with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) =
    ///     (&Float::from(8)).log_base_rational_base_prec_round_ref(&Rational::from(2), 10, Exact);
    /// assert_eq!(log.to_string(), "3.0"); // log_2(8) = 3
    /// assert_eq!(o, Equal);
    ///
    /// let (log, o) =
    ///     (&Float::from(2)).log_base_rational_base_prec_round_ref(&Rational::from(4), 10, Exact);
    /// assert_eq!(log.to_string(), "0.5"); // log_4(2) = 1/2
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_rational_base_prec_round_ref(
        &self,
        base: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        assert!(*base > 1u32, "Logarithm base must be greater than 1");
        match self {
            Self(NaN | Infinity { sign: false } | Finite { sign: false, .. }) => {
                (float_nan!(), Equal)
            }
            float_either_zero!() => (float_negative_infinity!(), Equal),
            float_infinity!() => (float_infinity!(), Equal),
            _ => log_base_rational_base_prec_round_normal(self, base, prec, rm),
        }
    }

    /// Computes $\log_b x$, where $x$ is a [`Float`] and $b$ is a [`Rational`] greater than 1,
    /// rounding the result to the nearest value of the specified precision. The [`Float`] is taken
    /// by value and the base by reference. An [`Ordering`] is also returned.
    ///
    /// See [`Float::log_base_rational_base_prec_round`] for details and special cases.
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
    /// let (log, o) = Float::from(8).log_base_rational_base_prec(&Rational::from(4), 10);
    /// assert_eq!(log.to_string(), "1.5"); // log_4(8) = 3/2
    /// assert_eq!(o, Equal);
    ///
    /// let (log, o) = Float::from(9).log_base_rational_base_prec(&Rational::from(3), 10);
    /// assert_eq!(log.to_string(), "2.0"); // log_3(9) = 2
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_rational_base_prec(self, base: &Rational, prec: u64) -> (Self, Ordering) {
        self.log_base_rational_base_prec_round(base, prec, Nearest)
    }

    /// Computes $\log_b x$, where $x$ is a [`Float`] and $b$ is a [`Rational`] greater than 1,
    /// rounding the result to the nearest value of the specified precision. The [`Float`] and the
    /// base are both taken by reference. An [`Ordering`] is also returned.
    ///
    /// See [`Float::log_base_rational_base_prec_round`] for details and special cases.
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
    /// let (log, o) = (&Float::from(8)).log_base_rational_base_prec_ref(&Rational::from(2), 10);
    /// assert_eq!(log.to_string(), "3.0"); // log_2(8) = 3
    /// assert_eq!(o, Equal);
    ///
    /// let (log, o) = (&Float::from(2)).log_base_rational_base_prec_ref(&Rational::from(4), 10);
    /// assert_eq!(log.to_string(), "0.5"); // log_4(2) = 1/2
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_rational_base_prec_ref(&self, base: &Rational, prec: u64) -> (Self, Ordering) {
        self.log_base_rational_base_prec_round_ref(base, prec, Nearest)
    }

    /// Computes $\log_b x$, where $x$ is a [`Float`] and $b$ is a [`Rational`] greater than 1,
    /// rounding the result to the precision of the input and with the specified rounding mode. The
    /// [`Float`] is taken by value and the base by reference. An [`Ordering`] is also returned.
    ///
    /// See [`Float::log_base_rational_base_prec_round`] for details and special cases.
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
    /// let (log, o) = Float::from(9).log_base_rational_base_round(&Rational::from(3), Exact);
    /// assert_eq!(log.to_string(), "2.0"); // log_3(9) = 2
    /// assert_eq!(o, Equal);
    ///
    /// let (log, o) = Float::from(2).log_base_rational_base_round(&Rational::from(4), Exact);
    /// assert_eq!(log.to_string(), "0.5"); // log_4(2) = 1/2
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_rational_base_round(self, base: &Rational, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.log_base_rational_base_prec_round(base, prec, rm)
    }

    /// Computes $\log_b x$, where $x$ is a [`Float`] and $b$ is a [`Rational`] greater than 1,
    /// rounding the result to the precision of the input and with the specified rounding mode. The
    /// [`Float`] and the base are both taken by reference. An [`Ordering`] is also returned.
    ///
    /// See [`Float::log_base_rational_base_prec_round`] for details and special cases.
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
    /// let (log, o) = (&Float::from(81)).log_base_rational_base_round_ref(&Rational::from(3), Exact);
    /// assert_eq!(log.to_string(), "4.0"); // log_3(81) = 4
    /// assert_eq!(o, Equal);
    ///
    /// let (log, o) = (&Float::from(9)).log_base_rational_base_round_ref(&Rational::from(3), Exact);
    /// assert_eq!(log.to_string(), "2.0"); // log_3(9) = 2
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_rational_base_round_ref(
        &self,
        base: &Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        self.log_base_rational_base_prec_round_ref(base, self.significant_bits(), rm)
    }

    /// Computes $\log_b x$, where $x$ is a [`Float`] and $b$ is a [`Rational`] greater than 1, in
    /// place, rounding the result to the specified precision and with the specified rounding mode.
    /// The base is taken by reference. An [`Ordering`] is returned.
    ///
    /// See [`Float::log_base_rational_base_prec_round`] for details and special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, if `base` is less than or equal to 1, or if `rm` is `Exact` but the
    /// result cannot be represented exactly with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(8);
    /// assert_eq!(x.log_base_rational_base_prec_round_assign(&Rational::from(4), 10, Exact), Equal);
    /// assert_eq!(x.to_string(), "1.5"); // log_4(8) = 3/2
    ///
    /// let mut x = Float::from(9);
    /// assert_eq!(x.log_base_rational_base_prec_round_assign(&Rational::from(3), 10, Exact), Equal);
    /// assert_eq!(x.to_string(), "2.0"); // log_3(9) = 2
    /// ```
    #[inline]
    pub fn log_base_rational_base_prec_round_assign(
        &mut self,
        base: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (result, o) = core::mem::take(self).log_base_rational_base_prec_round(base, prec, rm);
        *self = result;
        o
    }

    /// Computes $\log_b x$, where $x$ is a [`Float`] and $b$ is a [`Rational`] greater than 1, in
    /// place, rounding the result to the nearest value of the specified precision. The base is taken
    /// by reference. An [`Ordering`] is returned.
    ///
    /// See [`Float::log_base_rational_base_prec_round`] for details and special cases.
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
    /// x.log_base_rational_base_prec_assign(&Rational::from(4), 10);
    /// assert_eq!(x.to_string(), "1.5"); // log_4(8) = 3/2
    ///
    /// let mut x = Float::from(9);
    /// x.log_base_rational_base_prec_assign(&Rational::from(3), 10);
    /// assert_eq!(x.to_string(), "2.0"); // log_3(9) = 2
    /// ```
    #[inline]
    pub fn log_base_rational_base_prec_assign(&mut self, base: &Rational, prec: u64) -> Ordering {
        self.log_base_rational_base_prec_round_assign(base, prec, Nearest)
    }

    /// Computes $\log_b x$, where $x$ is a [`Float`] and $b$ is a [`Rational`] greater than 1, in
    /// place, rounding the result to the precision of the input and with the specified rounding
    /// mode. The base is taken by reference. An [`Ordering`] is returned.
    ///
    /// See [`Float::log_base_rational_base_prec_round`] for details and special cases.
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
    /// let mut x = Float::from(9);
    /// x.log_base_rational_base_round_assign(&Rational::from(3), Exact);
    /// assert_eq!(x.to_string(), "2.0"); // log_3(9) = 2
    ///
    /// let mut x = Float::from(2);
    /// x.log_base_rational_base_round_assign(&Rational::from(4), Exact);
    /// assert_eq!(x.to_string(), "0.5"); // log_4(2) = 1/2
    /// ```
    #[inline]
    pub fn log_base_rational_base_round_assign(
        &mut self,
        base: &Rational,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = self.significant_bits();
        self.log_base_rational_base_prec_round_assign(base, prec, rm)
    }
}

impl LogBase<Rational> for Float {
    type Output = Self;

    /// Computes $\log_b x$, where $x$ is a [`Float`] and $b$ is a [`Rational`] greater than 1,
    /// rounding the result to the nearest value of the input's precision. Both are taken by value.
    ///
    /// See [`Float::log_base_rational_base_prec_round`] for special cases.
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
    /// use malachite_base::num::arithmetic::traits::LogBase;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Float::from(2).log_base(Rational::from(4)).to_string(), "0.5"); // log_4(2) = 1/2
    /// assert_eq!(Float::from(9).log_base(Rational::from(3)).to_string(), "2.0"); // log_3(9) = 2
    /// ```
    #[inline]
    fn log_base(self, base: Rational) -> Self {
        let prec = self.significant_bits();
        self.log_base_rational_base_prec_round(&base, prec, Nearest).0
    }
}

impl LogBase<&Rational> for &Float {
    type Output = Float;

    /// Computes $\log_b x$, where $x$ is a [`Float`] and $b$ is a [`Rational`] greater than 1,
    /// rounding the result to the nearest value of the input's precision. Both are taken by
    /// reference.
    ///
    /// See [`Float::log_base_rational_base_prec_round`] for special cases.
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
    /// use malachite_base::num::arithmetic::traits::LogBase;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!((&Float::from(81)).log_base(&Rational::from(3)).to_string(), "4.0"); // log_3(81) = 4
    /// assert_eq!((&Float::from(9)).log_base(&Rational::from(3)).to_string(), "2.0"); // log_3(9) = 2
    /// ```
    #[inline]
    fn log_base(self, base: &Rational) -> Float {
        self.log_base_rational_base_prec_round_ref(base, self.significant_bits(), Nearest)
            .0
    }
}

impl LogBaseAssign<&Rational> for Float {
    /// Replaces a [`Float`] $x$ with $\log_b x$, where $b$ is a [`Rational`] greater than 1, rounding
    /// the result to the nearest value of the input's precision. The base is taken by reference.
    ///
    /// See [`Float::log_base_rational_base_prec_round`] for special cases.
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
    /// use malachite_base::num::arithmetic::traits::LogBaseAssign;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::from(81);
    /// x.log_base_assign(&Rational::from(3));
    /// assert_eq!(x.to_string(), "4.0"); // log_3(81) = 4
    ///
    /// let mut x = Float::from(9);
    /// x.log_base_assign(&Rational::from(3));
    /// assert_eq!(x.to_string(), "2.0"); // log_3(9) = 2
    /// ```
    #[inline]
    fn log_base_assign(&mut self, base: &Rational) {
        let prec = self.significant_bits();
        self.log_base_rational_base_prec_round_assign(base, prec, Nearest);
    }
}
