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
use crate::{Float, float_either_zero, float_infinity, float_nan, float_negative_infinity};
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{
    CeilingLogBase2, CheckedLogBase, IsPowerOf2, LogBase, LogBaseAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero as ZeroTrait;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::factorization::traits::ExpressAsPower;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::float_extras::float_can_round;
use malachite_nz::platform::Limb;
use malachite_q::Rational;

// Returns `Some(e_x / e_base)` when `log_base(x)` is rational, and `None` when it is irrational.
// The input `x` must be finite, positive, and not equal to 1, and `base > 1` must not be a power of
// 2.
//
// `log_base(x)` is rational exactly when `x` and `base` are both powers of a common integer `g`,
// say `x = g ^ e_x` and `base = g ^ e_base`; then `log_base(x) = e_x / e_base`. Taking `g` to be
// the smallest integer of which `base` is a power (obtained by stripping `base` of perfect-power
// factors via `express_as_power`), this holds iff `x` is a positive integer that is a power of `g`.
//
// Detecting these rational results up front is essential, not just an optimization: when the result
// is exactly representable (for example `log_9(3) = 1/2`), the Ziv loop in
// `log_base_prec_round_normal` would never terminate, because the rounding test can never certify a
// value that sits exactly on a representable point (or exactly on a tie). This generalizes the
// `10^n` exactness check in mpfr_log10, which only catches integer results.
//
// The check is balloon-safe. If `x = g ^ e_x` then `x`'s bit length is `e_x * log2(g) >= e_x`, and
// `e_x <= e_base * prec` is needed for `e_x / e_base` to be representable in `prec` bits, so an `x`
// worth materializing has bit length at most about `64 * prec`. When `x`'s exponent exceeds that
// bound, `x` is left to the Ziv loop (which then converges, `x` not being a power of `g`), so `x`
// is materialized as an integer only when doing so is cheap.
pub(crate) fn rational_log_base(x: &Float, base: u64) -> Option<Rational> {
    let e = i64::from(x.get_exponent().unwrap());
    // x < 1 cannot be a power of g >= 2, and only positive exponents can.
    if e < 1 || u64::exact_from(e) > x.get_prec().unwrap().saturating_mul(64) {
        return None;
    }
    // `Natural::try_from` fails unless `x` is a nonnegative integer.
    let n = Natural::try_from(x).ok()?;
    // `express_as_power` returns `None` when `base` is not a perfect power, in which case `base`
    // itself is `g` (with exponent 1).
    let (root, e_base) = base.express_as_power().unwrap_or((base, 1));
    let e_x = (&n).checked_log_base(&Natural::from(root))?;
    Some(Rational::from_unsigneds(e_x, e_base))
}

// The computation of log_base(x, base) is done by log_base(x) = ln(x) / ln(base). When `base` is a
// power of 2 the caller delegates to `log_base_power_of_2`, so here `base` is not a power of 2.
//
// This is mpfr_log10 from log10.c, MPFR 4.3.0, generalized from base 10 to an arbitrary non-power-
// of-2 `base`. The input is finite, nonzero, and positive.
fn log_base_prec_round_normal(
    x: &Float,
    base: u64,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    // If x is 1, the result is 0.
    if *x == 1u32 {
        return (Float::ZERO, Equal);
    }
    // If log_base(x) is rational -- x and base are both powers of a common integer -- compute it
    // directly. This includes the exactly-representable results (integers like log_8(64) = 2 and
    // dyadics like log_9(3) = 1/2), which the Ziv loop below could never certify, as well as
    // non-representable rationals like log_27(9) = 2/3, which it could but for which the direct
    // computation is cheaper and exact.
    if let Some(q) = rational_log_base(x, base) {
        return Float::from_rational_prec_round(q, prec, rm);
    }
    // The result is irrational, so it is never exactly representable.
    assert_ne!(rm, Exact, "Inexact log_base");
    let base_float = Float::from(base);
    // Compute the precision of the intermediary variable: the optimal number of bits, see
    // algorithms.tex.
    let mut working_prec = prec + 4 + prec.ceiling_log_base_2();
    let mut increment = Limb::WIDTH;
    loop {
        // ln(x) / ln(base). ln(x), ln(base), and the division are each correctly rounded (at most
        // 1/2 ulp), so the relative error is below 2^(2 - working_prec) and working_prec - 4
        // correct bits suffice for rounding (mpfr_log10 uses Nt - 4).
        let t = x
            .ln_prec_ref(working_prec)
            .0
            .div_prec(base_float.ln_prec_ref(working_prec).0, working_prec)
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
    /// Computes $\log_b x$, where $x$ is a [`Float`] and $b$ is a `u64` greater than 1, rounding
    /// the result to the specified precision and with the specified rounding mode. The [`Float`] is
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded value is
    /// less than, equal to, or greater than the exact value. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The base-$b$ logarithm of any nonzero negative number is `NaN`.
    ///
    /// When `base` is a power of 2, this function delegates to
    /// [`Float::log_base_power_of_2_prec_round`]; otherwise it computes $\ln x / \ln b$.
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
    /// Special cases:
    /// - $f(\text{NaN},b,p,m)=\text{NaN}$
    /// - $f(\infty,b,p,m)=\infty$
    /// - $f(-\infty,b,p,m)=\text{NaN}$
    /// - $f(\pm0.0,b,p,m)=-\infty$
    /// - $f(1.0,b,p,m)=0.0$, and the result is exact
    /// - $f(b^n,b,p,m)=n$, rounded to precision $p$; the result is exact if and only if $n$ is
    ///   representable with precision $p$
    /// - $f(x,b,p,m)=\text{NaN}$ for $x<0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::log_base_prec`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::log_base_round`] instead. If both of these things are true, consider using
    /// [`Float::log_base`] instead.
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
    /// let (log, o) = Float::from(1000).log_base_prec_round(10, 10, Nearest);
    /// assert_eq!(log.to_string(), "3.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (log, o) = Float::from(50).log_base_prec_round(10, 10, Floor);
    /// assert_eq!(log.to_string(), "1.697");
    /// assert_eq!(o, Less);
    ///
    /// let (log, o) = Float::from(50).log_base_prec_round(10, 10, Ceiling);
    /// assert_eq!(log.to_string(), "1.699");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_prec_round(self, base: u64, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        assert!(base > 1, "Logarithm base must be greater than 1");
        if base.is_power_of_2() {
            return self.log_base_power_of_2_prec_round(i64::from(base.trailing_zeros()), prec, rm);
        }
        match self {
            Self(NaN | Infinity { sign: false } | Finite { sign: false, .. }) => {
                (float_nan!(), Equal)
            }
            float_either_zero!() => (float_negative_infinity!(), Equal),
            float_infinity!() => (float_infinity!(), Equal),
            _ => log_base_prec_round_normal(&self, base, prec, rm),
        }
    }

    /// Computes $\log_b x$, where $x$ is a [`Float`] and $b$ is a `u64` greater than 1, rounding
    /// the result to the specified precision and with the specified rounding mode. The [`Float`] is
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded value
    /// is less than, equal to, or greater than the exact value. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::log_base_prec_round`] for details, special cases, and a description of the
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
    /// Panics if `prec` is zero, if `base` is less than 2, or if `rm` is `Exact` but the result
    /// cannot be represented exactly with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::from(1000).log_base_prec_round_ref(10, 10, Nearest);
    /// assert_eq!(log.to_string(), "3.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_prec_round_ref(
        &self,
        base: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        assert!(base > 1, "Logarithm base must be greater than 1");
        if base.is_power_of_2() {
            return self.log_base_power_of_2_prec_round_ref(
                i64::from(base.trailing_zeros()),
                prec,
                rm,
            );
        }
        match self {
            Self(NaN | Infinity { sign: false } | Finite { sign: false, .. }) => {
                (float_nan!(), Equal)
            }
            float_either_zero!() => (float_negative_infinity!(), Equal),
            float_infinity!() => (float_infinity!(), Equal),
            _ => log_base_prec_round_normal(self, base, prec, rm),
        }
    }

    /// Computes $\log_b x$, where $x$ is a [`Float`] and $b$ is a `u64` greater than 1, rounding
    /// the result to the nearest value of the specified precision. The [`Float`] is taken by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded value is less than, equal
    /// to, or greater than the exact value.
    ///
    /// See [`Float::log_base_prec_round`] for details and special cases.
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
    /// let (log, o) = Float::from(50).log_base_prec(10, 10);
    /// assert_eq!(log.to_string(), "1.699");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_prec(self, base: u64, prec: u64) -> (Self, Ordering) {
        self.log_base_prec_round(base, prec, Nearest)
    }

    /// Computes $\log_b x$, where $x$ is a [`Float`] and $b$ is a `u64` greater than 1, rounding
    /// the result to the nearest value of the specified precision. The [`Float`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded value is less
    /// than, equal to, or greater than the exact value.
    ///
    /// See [`Float::log_base_prec_round`] for details and special cases.
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
    /// let (log, o) = Float::from(50).log_base_prec_ref(10, 10);
    /// assert_eq!(log.to_string(), "1.699");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_prec_ref(&self, base: u64, prec: u64) -> (Self, Ordering) {
        self.log_base_prec_round_ref(base, prec, Nearest)
    }

    /// Computes $\log_b x$, where $x$ is a [`Float`] and $b$ is a `u64` greater than 1, rounding
    /// the result to the precision of the input and with the specified rounding mode. The [`Float`]
    /// is taken by value. An [`Ordering`] is also returned, indicating whether the rounded value is
    /// less than, equal to, or greater than the exact value.
    ///
    /// See [`Float::log_base_prec_round`] for details and special cases.
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
    /// let (log, o) = Float::from(1000).log_base_round(10, Floor);
    /// assert_eq!(log.to_string(), "3.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_round(self, base: u64, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.log_base_prec_round(base, prec, rm)
    }

    /// Computes $\log_b x$, where $x$ is a [`Float`] and $b$ is a `u64` greater than 1, rounding
    /// the result to the precision of the input and with the specified rounding mode. The [`Float`]
    /// is taken by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// value is less than, equal to, or greater than the exact value.
    ///
    /// See [`Float::log_base_prec_round`] for details and special cases.
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
    /// let (log, o) = Float::from(81).log_base_round_ref(3, Ceiling);
    /// assert_eq!(log.to_string(), "4.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_round_ref(&self, base: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.log_base_prec_round_ref(base, self.significant_bits(), rm)
    }

    /// Computes $\log_b x$, where $x$ is a [`Float`] and $b$ is a `u64` greater than 1, in place,
    /// rounding the result to the specified precision and with the specified rounding mode. An
    /// [`Ordering`] is returned, indicating whether the rounded value is less than, equal to, or
    /// greater than the exact value.
    ///
    /// See [`Float::log_base_prec_round`] for details and special cases.
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
    /// let mut x = Float::from(50);
    /// let o = x.log_base_prec_round_assign(10, 10, Floor);
    /// assert_eq!(x.to_string(), "1.697");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn log_base_prec_round_assign(
        &mut self,
        base: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (result, o) = core::mem::take(self).log_base_prec_round(base, prec, rm);
        *self = result;
        o
    }

    /// Computes $\log_b x$, where $x$ is a [`Float`] and $b$ is a `u64` greater than 1, in place,
    /// rounding the result to the nearest value of the specified precision. An [`Ordering`] is
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value.
    ///
    /// See [`Float::log_base_prec_round`] for details and special cases.
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
    /// let mut x = Float::from(1000);
    /// let o = x.log_base_prec_assign(10, 10);
    /// assert_eq!(x.to_string(), "3.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_prec_assign(&mut self, base: u64, prec: u64) -> Ordering {
        self.log_base_prec_round_assign(base, prec, Nearest)
    }

    /// Computes $\log_b x$, where $x$ is a [`Float`] and $b$ is a `u64` greater than 1, in place,
    /// rounding the result to the precision of the input and with the specified rounding mode. An
    /// [`Ordering`] is returned, indicating whether the rounded value is less than, equal to, or
    /// greater than the exact value.
    ///
    /// See [`Float::log_base_prec_round`] for details and special cases.
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
    /// let mut x = Float::from(81);
    /// let o = x.log_base_round_assign(3, Nearest);
    /// assert_eq!(x.to_string(), "4.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_round_assign(&mut self, base: u64, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.log_base_prec_round_assign(base, prec, rm)
    }
}

impl LogBase<u64> for Float {
    type Output = Self;

    /// Computes $\log_b x$, where $x$ is a [`Float`] and $b$ is a `u64` greater than 1, rounding
    /// the result to the nearest value of the input's precision. The [`Float`] is taken by value.
    ///
    /// The base-$b$ logarithm of any nonzero negative number is `NaN`. See
    /// [`Float::log_base_prec_round`] for the special cases.
    ///
    /// $$
    /// f(x,b) = \log_b x+\varepsilon,
    /// $$
    /// where $|\varepsilon| \leq 2^{\lfloor\log_2 |\log_b x|\rfloor-p}$ and $p$ is the precision of
    /// the input.
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
    /// use malachite_base::num::arithmetic::traits::LogBase;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::from(1000).log_base(10).to_string(), "3.0");
    /// assert_eq!(Float::from(81).log_base(3).to_string(), "4.0");
    /// ```
    #[inline]
    fn log_base(self, base: u64) -> Self {
        let prec = self.significant_bits();
        self.log_base_prec_round(base, prec, Nearest).0
    }
}

impl LogBase<u64> for &Float {
    type Output = Float;

    /// Computes $\log_b x$, where $x$ is a [`Float`] and $b$ is a `u64` greater than 1, rounding
    /// the result to the nearest value of the input's precision. The [`Float`] is taken by
    /// reference.
    ///
    /// The base-$b$ logarithm of any nonzero negative number is `NaN`. See
    /// [`Float::log_base_prec_round`] for the special cases.
    ///
    /// $$
    /// f(x,b) = \log_b x+\varepsilon,
    /// $$
    /// where $|\varepsilon| \leq 2^{\lfloor\log_2 |\log_b x|\rfloor-p}$ and $p$ is the precision of
    /// the input.
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
    /// use malachite_base::num::arithmetic::traits::LogBase;
    /// use malachite_float::Float;
    ///
    /// assert_eq!((&Float::from(1000)).log_base(10).to_string(), "3.0");
    /// ```
    #[inline]
    fn log_base(self, base: u64) -> Float {
        self.log_base_prec_round_ref(base, self.significant_bits(), Nearest)
            .0
    }
}

impl LogBaseAssign<u64> for Float {
    /// Replaces a [`Float`] $x$ with $\log_b x$, where $b$ is a `u64` greater than 1, rounding the
    /// result to the nearest value of the input's precision.
    ///
    /// The base-$b$ logarithm of any nonzero negative number is `NaN`. See
    /// [`Float::log_base_prec_round`] for the special cases.
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
    /// use malachite_base::num::arithmetic::traits::LogBaseAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(1000);
    /// x.log_base_assign(10);
    /// assert_eq!(x.to_string(), "3.0");
    /// ```
    #[inline]
    fn log_base_assign(&mut self, base: u64) {
        let prec = self.significant_bits();
        self.log_base_prec_round_assign(base, prec, Nearest);
    }
}
