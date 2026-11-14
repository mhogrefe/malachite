// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::exponential::ln::{LogBase, log_helper, log_rational_helper};
use alloc::borrow::Cow;
use core::cmp::Ordering;
use malachite_base::num::basic::traits::Zero;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_q::Rational;

impl BigExponentFloat {
    /// Computes the base-10 logarithm of a [`BigExponentFloat`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`BigExponentFloat`] is taken
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded logarithm is less
    /// than, equal to, or greater than the exact logarithm. Although `NaN`s are not comparable to
    /// any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \log_{10} x+\varepsilon.
    /// $$
    /// - If $\log_{10} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\log_{10} x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_{10} x|\rfloor-p+1}$.
    /// - If $\log_{10} x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_{10} x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=\text{NaN}$
    /// - $f(0,p,m)=-\infty$
    /// - $f(1,p,m)=0$
    /// - $f(x,p,m)=\text{NaN}$ if $x<0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::log_base_10_prec_round), this function
    /// accepts arguments with any exponent.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::log_base_10_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::from(PI).log_base_10_prec_round(5, Floor);
    /// assert_eq!(format!("{y:?}"), "0x0.7c");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).log_base_10_prec_round(5, Ceiling);
    /// assert_eq!(format!("{y:?}"), "0x0.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).log_base_10_prec_round(5, Nearest);
    /// assert_eq!(format!("{y:?}"), "0x0.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).log_base_10_prec_round(20, Floor);
    /// assert_eq!(format!("{y:?}"), "0x0.7f453");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).log_base_10_prec_round(20, Ceiling);
    /// assert_eq!(format!("{y:?}"), "0x0.7f4538");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).log_base_10_prec_round(20, Nearest);
    /// assert_eq!(format!("{y:?}"), "0x0.7f4538");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_10_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        log_helper(Cow::Owned(self), LogBase::Ten, prec, rm)
    }

    /// Computes the base-10 logarithm of a [`BigExponentFloat`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`BigExponentFloat`] is taken
    /// by reference. An [`Ordering`] is also returned, indicating whether the rounded logarithm is
    /// less than, equal to, or greater than the exact logarithm. Although `NaN`s are not comparable
    /// to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \log_{10} x+\varepsilon.
    /// $$
    /// - If $\log_{10} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\log_{10} x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_{10} x|\rfloor-p+1}$.
    /// - If $\log_{10} x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_{10} x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=\text{NaN}$
    /// - $f(0,p,m)=-\infty$
    /// - $f(1,p,m)=0$
    /// - $f(x,p,m)=\text{NaN}$ if $x<0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::log_base_10_prec_round), this function
    /// accepts arguments with any exponent.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).log_base_10_prec_round_ref(5, Floor);
    /// assert_eq!(format!("{y:?}"), "0x0.7c");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).log_base_10_prec_round_ref(5, Ceiling);
    /// assert_eq!(format!("{y:?}"), "0x0.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).log_base_10_prec_round_ref(5, Nearest);
    /// assert_eq!(format!("{y:?}"), "0x0.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).log_base_10_prec_round_ref(20, Floor);
    /// assert_eq!(format!("{y:?}"), "0x0.7f453");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).log_base_10_prec_round_ref(20, Ceiling);
    /// assert_eq!(format!("{y:?}"), "0x0.7f4538");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).log_base_10_prec_round_ref(20, Nearest);
    /// assert_eq!(format!("{y:?}"), "0x0.7f4538");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_10_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        log_helper(Cow::Borrowed(self), LogBase::Ten, prec, rm)
    }

    /// Computes the base-10 logarithm of a [`BigExponentFloat`], in place, rounding the result to
    /// the specified precision and with the specified rounding mode. An [`Ordering`] is returned,
    /// indicating whether the rounded logarithm is less than, equal to, or greater than the exact
    /// logarithm. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \log_{10} x+\varepsilon.
    /// $$
    /// - If $\log_{10} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\log_{10} x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_{10} x|\rfloor-p+1}$.
    /// - If $\log_{10} x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_{10} x|\rfloor-p}$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::log_base_10_prec_round), this function
    /// accepts arguments with any exponent.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.log_base_10_prec_round_assign(5, Floor), Less);
    /// assert_eq!(format!("{x:?}"), "0x0.7c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.log_base_10_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(format!("{x:?}"), "0x0.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.log_base_10_prec_round_assign(5, Nearest), Greater);
    /// assert_eq!(format!("{x:?}"), "0x0.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.log_base_10_prec_round_assign(20, Floor), Less);
    /// assert_eq!(format!("{x:?}"), "0x0.7f453");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.log_base_10_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(format!("{x:?}"), "0x0.7f4538");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.log_base_10_prec_round_assign(20, Nearest), Greater);
    /// assert_eq!(format!("{x:?}"), "0x0.7f4538");
    /// ```
    #[inline]
    pub fn log_base_10_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = log_helper(Cow::Owned(x), LogBase::Ten, prec, rm);
        *self = result;
        o
    }

    /// Computes the base-10 logarithm of a [`BigExponentFloat`], rounding the result to the nearest
    /// value of the specified precision. The [`BigExponentFloat`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded logarithm is less than, equal
    /// to, or greater than the exact logarithm. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \log_{10} x+\varepsilon.
    /// $$
    /// - If $\log_{10} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\log_{10} x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\log_{10} x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=\text{NaN}$
    /// - $f(0,p)=-\infty$
    /// - $f(1,p)=0$
    /// - $f(x,p)=\text{NaN}$ if $x<0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::log_base_10_prec_round), this function
    /// accepts arguments with any exponent.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::log_base_10_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::from(PI).log_base_10_prec(5);
    /// assert_eq!(format!("{y:?}"), "0x0.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).log_base_10_prec(20);
    /// assert_eq!(format!("{y:?}"), "0x0.7f4538");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::power_of_2(1000000000000i64).log_base_10_prec(10);
    /// assert_eq!(format!("{y:?}"), "0x4.62E+9");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_10_prec(self, prec: u64) -> (Self, Ordering) {
        log_helper(Cow::Owned(self), LogBase::Ten, prec, Nearest)
    }

    /// Computes the base-10 logarithm of a [`BigExponentFloat`], rounding the result to the nearest
    /// value of the specified precision. The [`BigExponentFloat`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded logarithm is less than, equal
    /// to, or greater than the exact logarithm. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \log_{10} x+\varepsilon.
    /// $$
    /// - If $\log_{10} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\log_{10} x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\log_{10} x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=\text{NaN}$
    /// - $f(0,p)=-\infty$
    /// - $f(1,p)=0$
    /// - $f(x,p)=\text{NaN}$ if $x<0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::log_base_10_prec_round), this function
    /// accepts arguments with any exponent.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).log_base_10_prec_ref(5);
    /// assert_eq!(format!("{y:?}"), "0x0.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).log_base_10_prec_ref(20);
    /// assert_eq!(format!("{y:?}"), "0x0.7f4538");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_10_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        log_helper(Cow::Borrowed(self), LogBase::Ten, prec, Nearest)
    }

    /// Computes the base-10 logarithm of a [`BigExponentFloat`], in place, rounding the result to
    /// the nearest value of the specified precision. An [`Ordering`] is returned, indicating
    /// whether the rounded logarithm is less than, equal to, or greater than the exact logarithm.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function sets
    /// the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// x \gets \log_{10} x+\varepsilon.
    /// $$
    /// - If $\log_{10} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\log_{10} x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\log_{10} x|\rfloor-p}$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::log_base_10_prec_round), this function
    /// accepts arguments with any exponent.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.log_base_10_prec_assign(5), Greater);
    /// assert_eq!(format!("{x:?}"), "0x0.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.log_base_10_prec_assign(20), Greater);
    /// assert_eq!(format!("{x:?}"), "0x0.7f4538");
    /// ```
    #[inline]
    pub fn log_base_10_prec_assign(&mut self, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = log_helper(Cow::Owned(x), LogBase::Ten, prec, Nearest);
        *self = result;
        o
    }

    #[allow(clippy::needless_pass_by_value)]
    /// Computes the base-10 logarithm of a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode and returning the result as a
    /// [`BigExponentFloat`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded logarithm is less than, equal to, or greater than the exact
    /// logarithm. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \log_{10} x+\varepsilon.
    /// $$
    /// - If $\log_{10} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\log_{10} x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_{10} x|\rfloor-p+1}$.
    /// - If $\log_{10} x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_{10} x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p,m)=-\infty$
    /// - $f(1,p,m)=0$
    /// - $f(x,p,m)=\text{NaN}$ if $x<0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::log_base_10_rational_prec_round), this
    /// function's result never underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::log_base_10_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::log_base_10_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{y:?}"), "-0x0.3a");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::log_base_10_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{y:?}"), "-0x0.38");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::log_base_10_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{y:?}"), "-0x0.38");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::log_base_10_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{y:?}"), "-0x0.38cb18");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::log_base_10_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{y:?}"), "-0x0.38cb14");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::log_base_10_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{y:?}"), "-0x0.38cb14");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_10_rational_prec_round(
        x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        log_rational_helper(&x, LogBase::Ten, prec, rm)
    }

    /// Computes the base-10 logarithm of a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode and returning the result as a
    /// [`BigExponentFloat`]. The [`Rational`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded logarithm is less than, equal to, or greater than
    /// the exact logarithm. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \log_{10} x+\varepsilon.
    /// $$
    /// - If $\log_{10} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\log_{10} x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_{10} x|\rfloor-p+1}$.
    /// - If $\log_{10} x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_{10} x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p,m)=-\infty$
    /// - $f(1,p,m)=0$
    /// - $f(x,p,m)=\text{NaN}$ if $x<0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::log_base_10_rational_prec_round), this
    /// function's result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::log_base_10_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{y:?}"), "-0x0.3a");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::log_base_10_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{y:?}"), "-0x0.38");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::log_base_10_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{y:?}"), "-0x0.38");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::log_base_10_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{y:?}"), "-0x0.38cb18");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::log_base_10_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{y:?}"), "-0x0.38cb14");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::log_base_10_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{y:?}"), "-0x0.38cb14");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_10_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        log_rational_helper(x, LogBase::Ten, prec, rm)
    }

    #[allow(clippy::needless_pass_by_value)]
    /// Computes the base-10 logarithm of a [`Rational`], rounding the result to the nearest value
    /// of the specified precision and returning the result as a [`BigExponentFloat`]. The
    /// [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded logarithm is less than, equal to, or greater than the exact logarithm. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \log_{10} x+\varepsilon.
    /// $$
    /// - If $\log_{10} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\log_{10} x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\log_{10} x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p)=-\infty$
    /// - $f(1,p)=0$
    /// - $f(x,p)=\text{NaN}$ if $x<0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::log_base_10_rational_prec_round), this
    /// function's result never underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::log_base_10_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) =
    ///     BigExponentFloat::log_base_10_rational_prec(Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(format!("{y:?}"), "-0x0.38");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::log_base_10_rational_prec(Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(format!("{y:?}"), "-0x0.38cb14");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_10_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        log_rational_helper(&x, LogBase::Ten, prec, Nearest)
    }

    /// Computes the base-10 logarithm of a [`Rational`], rounding the result to the nearest value
    /// of the specified precision and returning the result as a [`BigExponentFloat`]. The
    /// [`Rational`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded logarithm is less than, equal to, or greater than the exact logarithm. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \log_{10} x+\varepsilon.
    /// $$
    /// - If $\log_{10} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\log_{10} x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\log_{10} x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p)=-\infty$
    /// - $f(1,p)=0$
    /// - $f(x,p)=\text{NaN}$ if $x<0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::log_base_10_rational_prec_round), this
    /// function's result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) =
    ///     BigExponentFloat::log_base_10_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(format!("{y:?}"), "-0x0.38");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::log_base_10_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(format!("{y:?}"), "-0x0.38cb14");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_10_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        log_rational_helper(x, LogBase::Ten, prec, Nearest)
    }
}
