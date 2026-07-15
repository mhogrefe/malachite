// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Infinity, NaN, Zero};
use crate::{Float, emulate_float_to_float_fn, float_infinity, float_nan, float_negative_infinity};
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{
    CeilingLogBase2, CheckedLogBase, IsPowerOf2, LogBaseOf1PlusX, LogBaseOf1PlusXAssign,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One as OneTrait, Zero as ZeroTrait};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::factorization::traits::ExpressAsPower;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::float_extras::float_can_round;
use malachite_nz::platform::Limb;
use malachite_q::Rational;

// Returns `Some(m / e_base)` -- the value of `log_base(1 + x)` -- when `1 + x = g^m` for the root
// `g` of `base` (so `base = g^e_base` and `log_base(1 + x)` is rational), and `None` when it is
// irrational. The input `x` must be finite and greater than -1, and `base > 1` must not be a power
// of 2.
//
// For a non-power-of-2 base, `g` is not a perfect power, so `1 + x = g^m` is an exact (dyadic)
// `Float` value only when `m >= 0`: `m = 0` gives `x = 0`, and `m >= 1` gives `1 + x` a positive
// integer power of `g` (so `x` is a positive integer). A negative `m` would make `g^m` a non-dyadic
// fraction, impossible for the dyadic `1 + x`.
//
// Detecting these rational results up front is essential: the Ziv loop in
// `log_base_1_plus_x_prec_round_normal` could never certify an exactly-representable one. The check
// is balloon-safe, materializing `x` as an integer only when its exponent is within `64 * prec` of
// being a representable `g^m - 1`.
pub(crate) fn log_base_1_plus_x_rational(x: &Float, base: u64) -> Option<Rational> {
    if *x == 0u32 {
        return Some(Rational::ZERO);
    }
    let e = i64::from(x.get_exponent()?);
    if e < 1 || u64::exact_from(e) > x.get_prec()?.saturating_mul(64) {
        return None;
    }
    // `Natural::try_from` fails unless `x` is a nonnegative integer.
    let n = Natural::try_from(x).ok()?;
    let (g, e_base) = base.express_as_power().unwrap_or((base, 1));
    let m = (n + Natural::ONE).checked_log_base(&Natural::from(g))?;
    Some(Rational::from_unsigneds(m, e_base))
}

// The computation of log_base_1_plus_x(x, base) is done by log_base(1 + x) = log_2(1 + x) /
// log_2(base). The input is finite and greater than -1, and `base > 1` is not a power of 2.
//
// Routing through `log_base_2_1_plus_x` (rather than computing `log_base(1 + x)` from `1 + x`
// directly) preserves accuracy when x is near 0, where `1 + x` would lose precision. Unlike the
// power-of-2 case, no near-power-of-2 special handling is needed: `log_2(base)` is irrational, so
// every non-rational result is strictly between `Float`s and the Ziv loop converges. (The rational
// results, where `1 + x = g^m`, are detected up front.)
fn log_base_1_plus_x_prec_round_normal(
    x: &Float,
    base: u64,
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
    if let Some(q) = log_base_1_plus_x_rational(x, base) {
        return Float::from_rational_prec_round(q, prec, rm);
    }
    // The result is irrational, so it is never exactly representable.
    assert_ne!(rm, Exact, "Inexact log_base_1_plus_x");
    let base_float = Float::from(base);
    let min_exp = i64::from(Float::MIN_EXPONENT);
    let mut working_prec = prec + 4 + prec.ceiling_log_base_2();
    let mut increment = Limb::WIDTH;
    loop {
        // log_2(1 + x), correctly rounded to working_prec; always within the Float exponent range.
        let num = x.log_base_2_1_plus_x_prec_ref(working_prec).0;
        // log_2(base) > 1, correctly rounded to working_prec.
        let den = base_float.log_base_2_prec_ref(working_prec).0;
        // Dividing by log_2(base) > 1 only shrinks the magnitude (overflow is impossible), but can
        // push the result below MIN_EXPONENT. When it underflows, the Ziv test below could never
        // resolve it (the quotient clamps), so hand the rounding to div_prec_round, which clamps to
        // zero or the minimum positive value per the rounding mode. The exact quotient exponent is
        // only resolved in the narrow band where the cheap exponent bound is inconclusive (then
        // e_num - e_den == min_exp - 1, so the result underflows iff |log_2(1 + x)| * 2^(1 -
        // min_exp) < log_2(base)). The left shift only adjusts the exponent, avoiding a huge
        // Rational conversion.
        let e_num = i64::from(num.get_exponent().unwrap());
        let e_den = i64::from(den.get_exponent().unwrap());
        if e_num - e_den + 1 < min_exp
            || (e_num - e_den < min_exp && (&num << u64::exact_from(1 - min_exp)).lt_abs(&den))
        {
            return num.div_prec_round(den, prec, rm);
        }
        // log_2(1 + x) / log_2(base), with three correctly-rounded operations (log_base_2_1_plus_x,
        // log_base_2, and the division, each at most 1/2 ulp), so the relative error is below 2^(2
        // - working_prec) and working_prec - 4 correct bits suffice for rounding.
        let t = num / den;
        if float_can_round(t.significand_ref().unwrap(), working_prec - 4, prec, rm) {
            return Float::from_float_prec_round(t, prec, rm);
        }
        // Increase the precision.
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

impl Float {
    /// Computes $\log_b(1+x)$, where $x$ is a [`Float`] and $b$ is a `u64` greater than 1, rounding
    /// the result to the specified precision and with the specified rounding mode. The [`Float`] is
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded value is
    /// less than, equal to, or greater than the exact value. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\log_b(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, `NaN` is returned.
    ///
    /// When `base` is a power of 2, this function delegates to
    /// [`Float::log_base_power_of_2_1_plus_x_prec_round`]; otherwise it computes $\log_2(1+x) /
    /// \log_2 b$, preserving accuracy for $x$ near 0.
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
    /// - $f(x,b,p,m)=m/e$ when $1+x=g^m$, where $g$ is the smallest integer of which $b$ is a power
    ///   and $b=g^e$, rounded to precision $p$; the result is exact if and only if $m/e$ is
    ///   representable with precision $p$ (for example $\log_9(1+8)=1$ when $x=8$ is exact)
    ///
    /// This function cannot overflow, but it can underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::log_base_1_plus_x_prec`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::log_base_1_plus_x_round`] instead. If both of these things are true, consider
    /// using `(&Float).log_base_1_plus_x()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, if `base` is less than 2, or if `rm` is `Exact` but the result
    /// cannot be represented exactly with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::from(8).log_base_1_plus_x_prec_round(9, 10, Exact);
    /// assert_eq!(log.to_string(), "1.0"); // log_9(9) = 1
    /// assert_eq!(o, Equal);
    ///
    /// let (log, o) = Float::from(1).log_base_1_plus_x_prec_round(3, 20, Nearest);
    /// assert_eq!(log.to_string(), "0.63093"); // log_3(2)
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_1_plus_x_prec_round(
        self,
        base: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        assert!(base > 1, "Logarithm base must be greater than 1");
        if base.is_power_of_2() {
            return self.log_base_power_of_2_1_plus_x_prec_round(
                i64::from(base.trailing_zeros()),
                prec,
                rm,
            );
        }
        match self {
            Self(NaN | Infinity { sign: false }) => (float_nan!(), Equal),
            float_infinity!() => (float_infinity!(), Equal),
            Self(Zero { .. }) => (self, Equal),
            _ => log_base_1_plus_x_prec_round_normal(&self, base, prec, rm),
        }
    }

    /// Computes $\log_b(1+x)$, where $x$ is a [`Float`] and $b$ is a `u64` greater than 1, rounding
    /// the result to the specified precision and with the specified rounding mode. The [`Float`] is
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded value
    /// is less than, equal to, or greater than the exact value. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::log_base_1_plus_x_prec_round`] for details, special cases, and a description of
    /// the rounding behavior.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, if `base` is less than 2, or if `rm` is `Exact` but the result
    /// cannot be represented exactly with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::from(8).log_base_1_plus_x_prec_round_ref(3, 10, Exact);
    /// assert_eq!(log.to_string(), "2.0"); // log_3(9) = 2
    /// assert_eq!(o, Equal);
    ///
    /// let (log, o) = Float::from(1).log_base_1_plus_x_prec_round_ref(3, 20, Floor);
    /// assert_eq!(log.to_string(), "0.630929"); // log_3(2), rounded down
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn log_base_1_plus_x_prec_round_ref(
        &self,
        base: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        assert!(base > 1, "Logarithm base must be greater than 1");
        if base.is_power_of_2() {
            return self.log_base_power_of_2_1_plus_x_prec_round_ref(
                i64::from(base.trailing_zeros()),
                prec,
                rm,
            );
        }
        match self {
            Self(NaN | Infinity { sign: false }) => (float_nan!(), Equal),
            float_infinity!() => (float_infinity!(), Equal),
            Self(Zero { .. }) => (self.clone(), Equal),
            _ => log_base_1_plus_x_prec_round_normal(self, base, prec, rm),
        }
    }

    /// Computes $\log_b(1+x)$, where $x$ is a [`Float`] and $b$ is a `u64` greater than 1, rounding
    /// the result to the nearest value of the specified precision. The [`Float`] is taken by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded value is less than, equal
    /// to, or greater than the exact value.
    ///
    /// See [`Float::log_base_1_plus_x_prec_round`] for details and special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `base` is less than 2.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::from(8).log_base_1_plus_x_prec(9, 10);
    /// assert_eq!(log.to_string(), "1.0"); // log_9(9) = 1
    /// assert_eq!(o, Equal);
    ///
    /// let (log, o) = Float::from(1).log_base_1_plus_x_prec(3, 20);
    /// assert_eq!(log.to_string(), "0.63093"); // log_3(2)
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_1_plus_x_prec(self, base: u64, prec: u64) -> (Self, Ordering) {
        self.log_base_1_plus_x_prec_round(base, prec, Nearest)
    }

    /// Computes $\log_b(1+x)$, where $x$ is a [`Float`] and $b$ is a `u64` greater than 1, rounding
    /// the result to the nearest value of the specified precision. The [`Float`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded value is less
    /// than, equal to, or greater than the exact value.
    ///
    /// See [`Float::log_base_1_plus_x_prec_round`] for details and special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `base` is less than 2.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = (&Float::from(2)).log_base_1_plus_x_prec_ref(9, 10);
    /// assert_eq!(log.to_string(), "0.5"); // log_9(3) = 1/2
    /// assert_eq!(o, Equal);
    ///
    /// let (log, o) = (&Float::from(7)).log_base_1_plus_x_prec_ref(5, 30);
    /// assert_eq!(log.to_string(), "1.292029675"); // log_5(8)
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_1_plus_x_prec_ref(&self, base: u64, prec: u64) -> (Self, Ordering) {
        self.log_base_1_plus_x_prec_round_ref(base, prec, Nearest)
    }

    /// Computes $\log_b(1+x)$, where $x$ is a [`Float`] and $b$ is a `u64` greater than 1, rounding
    /// the result to the precision of the input and with the specified rounding mode. The [`Float`]
    /// is taken by value. An [`Ordering`] is also returned, indicating whether the rounded value is
    /// less than, equal to, or greater than the exact value.
    ///
    /// See [`Float::log_base_1_plus_x_prec_round`] for details and special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the precision of the input.
    ///
    /// # Panics
    /// Panics if `base` is less than 2, or if `rm` is `Exact` but the result cannot be represented
    /// exactly with the input's precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::from(8).log_base_1_plus_x_round(9, Exact);
    /// assert_eq!(log.to_string(), "1.0"); // log_9(9) = 1
    /// assert_eq!(o, Equal);
    ///
    /// let (log, o) = Float::from(8).log_base_1_plus_x_round(3, Exact);
    /// assert_eq!(log.to_string(), "2.0"); // log_3(9) = 2
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_1_plus_x_round(self, base: u64, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.log_base_1_plus_x_prec_round(base, prec, rm)
    }

    /// Computes $\log_b(1+x)$, where $x$ is a [`Float`] and $b$ is a `u64` greater than 1, rounding
    /// the result to the precision of the input and with the specified rounding mode. The [`Float`]
    /// is taken by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// value is less than, equal to, or greater than the exact value.
    ///
    /// See [`Float::log_base_1_plus_x_prec_round`] for details and special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the precision of the input.
    ///
    /// # Panics
    /// Panics if `base` is less than 2, or if `rm` is `Exact` but the result cannot be represented
    /// exactly with the input's precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = (&Float::from(8)).log_base_1_plus_x_round_ref(3, Exact);
    /// assert_eq!(log.to_string(), "2.0"); // log_3(9) = 2
    /// assert_eq!(o, Equal);
    ///
    /// let (log, o) = (&Float::from(2)).log_base_1_plus_x_round_ref(9, Exact);
    /// assert_eq!(log.to_string(), "0.5"); // log_9(3) = 1/2
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_1_plus_x_round_ref(&self, base: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.log_base_1_plus_x_prec_round_ref(base, self.significant_bits(), rm)
    }

    /// Computes $\log_b(1+x)$, where $x$ is a [`Float`] and $b$ is a `u64` greater than 1, in
    /// place, rounding the result to the specified precision and with the specified rounding mode.
    /// An [`Ordering`] is returned, indicating whether the rounded value is less than, equal to, or
    /// greater than the exact value.
    ///
    /// See [`Float::log_base_1_plus_x_prec_round`] for details and special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, if `base` is less than 2, or if `rm` is `Exact` but the result
    /// cannot be represented exactly with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(8);
    /// assert_eq!(x.log_base_1_plus_x_prec_round_assign(9, 10, Exact), Equal);
    /// assert_eq!(x.to_string(), "1.0"); // log_9(9) = 1
    ///
    /// let mut x = Float::from(1);
    /// assert_eq!(x.log_base_1_plus_x_prec_round_assign(3, 20, Floor), Less);
    /// assert_eq!(x.to_string(), "0.630929"); // log_3(2), rounded down
    /// ```
    #[inline]
    pub fn log_base_1_plus_x_prec_round_assign(
        &mut self,
        base: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (result, o) = core::mem::take(self).log_base_1_plus_x_prec_round(base, prec, rm);
        *self = result;
        o
    }

    /// Computes $\log_b(1+x)$, where $x$ is a [`Float`] and $b$ is a `u64` greater than 1, in
    /// place, rounding the result to the nearest value of the specified precision. An [`Ordering`]
    /// is returned, indicating whether the rounded value is less than, equal to, or greater than
    /// the exact value.
    ///
    /// See [`Float::log_base_1_plus_x_prec_round`] for details and special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `base` is less than 2.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(8);
    /// x.log_base_1_plus_x_prec_assign(3, 10);
    /// assert_eq!(x.to_string(), "2.0"); // log_3(9) = 2
    ///
    /// let mut x = Float::from(2);
    /// x.log_base_1_plus_x_prec_assign(9, 10);
    /// assert_eq!(x.to_string(), "0.5"); // log_9(3) = 1/2
    /// ```
    #[inline]
    pub fn log_base_1_plus_x_prec_assign(&mut self, base: u64, prec: u64) -> Ordering {
        self.log_base_1_plus_x_prec_round_assign(base, prec, Nearest)
    }

    /// Computes $\log_b(1+x)$, where $x$ is a [`Float`] and $b$ is a `u64` greater than 1, in
    /// place, rounding the result to the precision of the input and with the specified rounding
    /// mode. An [`Ordering`] is returned, indicating whether the rounded value is less than, equal
    /// to, or greater than the exact value.
    ///
    /// See [`Float::log_base_1_plus_x_prec_round`] for details and special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the precision of the input.
    ///
    /// # Panics
    /// Panics if `base` is less than 2, or if `rm` is `Exact` but the result cannot be represented
    /// exactly with the input's precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(8);
    /// x.log_base_1_plus_x_round_assign(9, Exact);
    /// assert_eq!(x.to_string(), "1.0"); // log_9(9) = 1
    ///
    /// let mut x = Float::from(8);
    /// x.log_base_1_plus_x_round_assign(3, Exact);
    /// assert_eq!(x.to_string(), "2.0"); // log_3(9) = 2
    /// ```
    #[inline]
    pub fn log_base_1_plus_x_round_assign(&mut self, base: u64, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.log_base_1_plus_x_prec_round_assign(base, prec, rm)
    }
}

impl LogBaseOf1PlusX<u64> for Float {
    type Output = Self;

    /// Computes $\log_b(1+x)$, where $x$ is a [`Float`] and $b$ is a `u64` greater than 1, rounding
    /// the result to the nearest value of the input's precision. The [`Float`] is taken by value.
    ///
    /// $\log_b(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, `NaN` is returned. See
    /// [`Float::log_base_1_plus_x_prec_round`] for the other special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the precision of the input.
    ///
    /// # Panics
    /// Panics if `base` is less than 2.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::LogBaseOf1PlusX;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::from(8).log_base_1_plus_x(9).to_string(), "1.0"); // log_9(9) = 1
    /// assert_eq!(Float::from(8).log_base_1_plus_x(3).to_string(), "2.0"); // log_3(9) = 2
    /// ```
    #[inline]
    fn log_base_1_plus_x(self, base: u64) -> Self {
        let prec = self.significant_bits();
        self.log_base_1_plus_x_prec_round(base, prec, Nearest).0
    }
}

impl LogBaseOf1PlusX<u64> for &Float {
    type Output = Float;

    /// Computes $\log_b(1+x)$, where $x$ is a [`Float`] and $b$ is a `u64` greater than 1, rounding
    /// the result to the nearest value of the input's precision. The [`Float`] is taken by
    /// reference.
    ///
    /// $\log_b(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, `NaN` is returned. See
    /// [`Float::log_base_1_plus_x_prec_round`] for the other special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the precision of the input.
    ///
    /// # Panics
    /// Panics if `base` is less than 2.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::LogBaseOf1PlusX;
    /// use malachite_float::Float;
    ///
    /// assert_eq!((&Float::from(8)).log_base_1_plus_x(9).to_string(), "1.0"); // log_9(9) = 1
    /// assert_eq!((&Float::from(2)).log_base_1_plus_x(9).to_string(), "0.5"); // log_9(3) = 1/2
    /// ```
    #[inline]
    fn log_base_1_plus_x(self, base: u64) -> Float {
        self.log_base_1_plus_x_prec_round_ref(base, self.significant_bits(), Nearest)
            .0
    }
}

impl LogBaseOf1PlusXAssign<u64> for Float {
    /// Replaces a [`Float`] $x$ with $\log_b(1+x)$, where $b$ is a `u64` greater than 1, rounding
    /// the result to the nearest value of the input's precision.
    ///
    /// $\log_b(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, `NaN` is returned. See
    /// [`Float::log_base_1_plus_x_prec_round`] for the other special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the precision of the input.
    ///
    /// # Panics
    /// Panics if `base` is less than 2.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::LogBaseOf1PlusXAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(8);
    /// x.log_base_1_plus_x_assign(9);
    /// assert_eq!(x.to_string(), "1.0"); // log_9(9) = 1
    ///
    /// let mut x = Float::from(8);
    /// x.log_base_1_plus_x_assign(3);
    /// assert_eq!(x.to_string(), "2.0"); // log_3(9) = 2
    /// ```
    #[inline]
    fn log_base_1_plus_x_assign(&mut self, base: u64) {
        let prec = self.significant_bits();
        self.log_base_1_plus_x_prec_round_assign(base, prec, Nearest);
    }
}

/// Computes $\log_b(1+x)$, the base-$b$ logarithm of one plus a primitive float, where $b$ is a
/// `u64` greater than 1. Using this function is more accurate than computing the logarithm using
/// the standard library, both because $1+x$ may not be representable as a primitive float and
/// because the standard library's `log` is not always correctly rounded.
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
/// This function can underflow (to a subnormal or zero) when $x$ is close to zero and $b$ is large,
/// but it cannot overflow.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `base` is less than 2.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::arithmetic::log_base_1_plus_x::primitive_float_log_base_1_plus_x;
///
/// assert!(primitive_float_log_base_1_plus_x(f32::NAN, 10).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_1_plus_x(f32::INFINITY, 10)),
///     NiceFloat(f32::INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_1_plus_x(-1.0f32, 10)),
///     NiceFloat(f32::NEGATIVE_INFINITY)
/// );
/// assert!(primitive_float_log_base_1_plus_x(-2.0f32, 10).is_nan());
/// // log_10(1 + 999) = log_10(1000) = 3
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_1_plus_x(999.0f32, 10)),
///     NiceFloat(3.0)
/// );
/// // log_9(1 + 8) = log_9(9) = 1
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_1_plus_x(8.0f32, 9)),
///     NiceFloat(1.0)
/// );
/// // log_3(1 + 1) = log_3(2)
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_1_plus_x(1.0f32, 3)),
///     NiceFloat(0.63092977)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_log_base_1_plus_x<T: PrimitiveFloat>(x: T, base: u64) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(|x, prec| Float::log_base_1_plus_x_prec(x, base, prec), x)
}
