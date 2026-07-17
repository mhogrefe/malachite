// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::arithmetic::root::{primitive_float_root_u, primitive_float_root_u_rational};
use core::cmp::Ordering;
use malachite_base::num::arithmetic::traits::{Cbrt, CbrtAssign};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode;
use malachite_q::Rational;

impl Float {
    /// Takes the cube root of a [`Float`], rounding the result to the specified precision and with
    /// the specified rounding mode. The [`Float`] is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded cube root is less than, equal to, or greater than
    /// the exact cube root. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// Unlike the square root, the cube root of a negative number is a (negative) real number, so a
    /// negative input does not produce a `NaN`.
    ///
    /// $$
    /// f(x,p,m) = \sqrt[3]{x}+\varepsilon.
    /// $$
    /// - If $\sqrt[3]{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\sqrt[3]{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\sqrt[3]{x}|\rfloor-p+1}$.
    /// - If $\sqrt[3]{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\sqrt[3]{x}|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=-\infty$
    /// - $f(0.0,p,m)=0.0$
    /// - $f(-0.0,p,m)=-0.0$
    ///
    /// The result never overflows or underflows: its exponent is close to the exponent of $x$
    /// divided by 3.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::cbrt_prec`] instead. If you
    /// know that your target precision is the precision of the input, consider using
    /// [`Float::cbrt_round`] instead. If both of these things are true, consider using the
    /// [`Cbrt`](malachite_base::num::arithmetic::traits::Cbrt) implementation instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the cube root is not exact.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (cbrt, o) = Float::from(27.0).cbrt_prec_round(10, Floor);
    /// assert_eq!(cbrt.to_string(), "3.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (cbrt, o) = Float::from(2.0).cbrt_prec_round(10, Floor);
    /// assert_eq!(cbrt.to_string(), "1.26");
    /// assert_eq!(o, Less);
    ///
    /// let (cbrt, o) = Float::from(2.0).cbrt_prec_round(10, Ceiling);
    /// assert_eq!(cbrt.to_string(), "1.262");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn cbrt_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.root_u_prec_round(3, prec, rm)
    }

    /// Takes the cube root of a [`Float`], rounding the result to the specified precision and with
    /// the specified rounding mode. The [`Float`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded cube root is less than, equal to, or greater than
    /// the exact cube root. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See the [`Float::cbrt_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::cbrt_prec_ref`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::cbrt_round_ref`] instead. If both of these things are true, consider using the
    /// [`Cbrt`](malachite_base::num::arithmetic::traits::Cbrt) implementation instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the cube root is not exact.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (cbrt, o) = Float::from(2.0).cbrt_prec_round_ref(10, Floor);
    /// assert_eq!(cbrt.to_string(), "1.26");
    /// assert_eq!(o, Less);
    ///
    /// let (cbrt, o) = Float::from(2.0).cbrt_prec_round_ref(10, Ceiling);
    /// assert_eq!(cbrt.to_string(), "1.262");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn cbrt_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.root_u_prec_round_ref(3, prec, rm)
    }

    /// Takes the cube root of a [`Float`], rounding the result to the specified precision and to
    /// the nearest value. The [`Float`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded cube root is less than, equal to, or greater than the exact
    /// cube root. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// If the cube root is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// See the [`Float::cbrt_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::cbrt_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (cbrt, o) = Float::from(27.0).cbrt_prec(10);
    /// assert_eq!(cbrt.to_string(), "3.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (cbrt, o) = Float::from(2.0).cbrt_prec(10);
    /// assert_eq!(cbrt.to_string(), "1.26");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn cbrt_prec(self, prec: u64) -> (Self, Ordering) {
        self.root_u_prec(3, prec)
    }

    /// Takes the cube root of a [`Float`], rounding the result to the specified precision and to
    /// the nearest value. The [`Float`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded cube root is less than, equal to, or greater than the exact
    /// cube root. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See the [`Float::cbrt_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::cbrt_prec_round_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (cbrt, o) = Float::from(2.0).cbrt_prec_ref(10);
    /// assert_eq!(cbrt.to_string(), "1.26");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn cbrt_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.root_u_prec_ref(3, prec)
    }

    /// Takes the cube root of a [`Float`], rounding the result to the precision of the input and
    /// with the specified rounding mode. The [`Float`] is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded cube root is less than, equal to, or greater than
    /// the exact cube root. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See the [`Float::cbrt_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::cbrt_prec_round`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the cube root is not exact.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (cbrt, o) = Float::from(-8.0).cbrt_round(Floor);
    /// assert_eq!(cbrt.to_string(), "-2.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn cbrt_round(self, rm: RoundingMode) -> (Self, Ordering) {
        self.root_u_round(3, rm)
    }

    /// Takes the cube root of a [`Float`], rounding the result to the precision of the input and
    /// with the specified rounding mode. The [`Float`] is taken by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded cube root is less than, equal to, or greater
    /// than the exact cube root. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See the [`Float::cbrt_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::cbrt_prec_round_ref`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the cube root is not exact.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (cbrt, o) = (&Float::from(-8.0)).cbrt_round_ref(Floor);
    /// assert_eq!(cbrt.to_string(), "-2.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn cbrt_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.root_u_round_ref(3, rm)
    }

    /// Takes the cube root of a [`Float`] in place, rounding the result to the specified precision
    /// and with the specified rounding mode. An [`Ordering`] is returned, indicating whether the
    /// rounded cube root is less than, equal to, or greater than the exact cube root. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function sets the [`Float`] to
    /// `NaN` it also returns `Equal`.
    ///
    /// See the [`Float::cbrt_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the cube root is not exact.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(2.0);
    /// assert_eq!(x.cbrt_prec_round_assign(10, Floor), Less);
    /// assert_eq!(x.to_string(), "1.26");
    /// ```
    #[inline]
    pub fn cbrt_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        self.root_u_prec_round_assign(3, prec, rm)
    }

    /// Takes the cube root of a [`Float`] in place, rounding the result to the specified precision
    /// and to the nearest value. An [`Ordering`] is returned, indicating whether the rounded cube
    /// root is less than, equal to, or greater than the exact cube root. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function sets the [`Float`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// See the [`Float::cbrt_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(2.0);
    /// assert_eq!(x.cbrt_prec_assign(10), Less);
    /// assert_eq!(x.to_string(), "1.26");
    /// ```
    #[inline]
    pub fn cbrt_prec_assign(&mut self, prec: u64) -> Ordering {
        self.root_u_prec_assign(3, prec)
    }

    /// Takes the cube root of a [`Float`] in place, rounding the result to the precision of the
    /// input and with the specified rounding mode. An [`Ordering`] is returned, indicating whether
    /// the rounded cube root is less than, equal to, or greater than the exact cube root. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function sets the [`Float`] to
    /// `NaN` it also returns `Equal`.
    ///
    /// See the [`Float::cbrt_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the cube root is not exact.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(-8.0);
    /// assert_eq!(x.cbrt_round_assign(Floor), Equal);
    /// assert_eq!(x.to_string(), "-2.0");
    /// ```
    #[inline]
    pub fn cbrt_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        self.root_u_round_assign(3, rm)
    }

    /// Takes the cube root of a [`Rational`], producing a [`Float`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`Rational`] is taken by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded cube root is less
    /// than, equal to, or greater than the exact cube root.
    ///
    /// See the [`Float::cbrt_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the cube root is not exact.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (cbrt, o) = Float::cbrt_rational_prec_round(Rational::from(27), 10, Floor);
    /// assert_eq!(cbrt.to_string(), "3.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn cbrt_rational_prec_round(x: Rational, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        Self::root_u_rational_prec_round(x, 3, prec, rm)
    }

    /// Takes the cube root of a [`Rational`], producing a [`Float`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`Rational`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded cube root is
    /// less than, equal to, or greater than the exact cube root.
    ///
    /// See the [`Float::cbrt_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the cube root is not exact.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (cbrt, o) = Float::cbrt_rational_prec_round_ref(&Rational::from(27), 10, Floor);
    /// assert_eq!(cbrt.to_string(), "3.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn cbrt_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::root_u_rational_prec_round_ref(x, 3, prec, rm)
    }

    /// Takes the cube root of a [`Rational`], producing a [`Float`], rounding the result to the
    /// specified precision and to the nearest value. The [`Rational`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded cube root is less than, equal
    /// to, or greater than the exact cube root.
    ///
    /// See the [`Float::cbrt_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits())`.
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
    /// let (cbrt, o) = Float::cbrt_rational_prec(Rational::from(27), 10);
    /// assert_eq!(cbrt.to_string(), "3.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn cbrt_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::root_u_rational_prec(x, 3, prec)
    }

    /// Takes the cube root of a [`Rational`], producing a [`Float`], rounding the result to the
    /// specified precision and to the nearest value. The [`Rational`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded cube root is less than, equal
    /// to, or greater than the exact cube root.
    ///
    /// See the [`Float::cbrt_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits())`.
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
    /// let (cbrt, o) = Float::cbrt_rational_prec_ref(&Rational::from(27), 10);
    /// assert_eq!(cbrt.to_string(), "3.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn cbrt_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::root_u_rational_prec_ref(x, 3, prec)
    }
}

impl Cbrt for Float {
    type Output = Self;

    /// Takes the cube root of a [`Float`], taking it by value.
    ///
    /// If the output has a precision, it is the precision of the input. If the cube root is
    /// equidistant from two [`Float`]s with that precision, the [`Float`] with fewer 1s in its
    /// binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding
    /// mode.
    ///
    /// Unlike the square root, the cube root of a negative number is a (negative) real number, so a
    /// negative input does not produce a `NaN`.
    ///
    /// $$
    /// f(x) = \sqrt[3]{x}+\varepsilon.
    /// $$
    /// - If $\sqrt[3]{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\sqrt[3]{x}$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\sqrt[3]{x}|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=\infty$
    /// - $f(-\infty)=-\infty$
    /// - $f(0.0)=0.0$
    /// - $f(-0.0)=-0.0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using [`Float::cbrt_prec`]
    /// instead. If you want to specify the output precision, consider using [`Float::cbrt_round`].
    /// If you want both of these things, consider using [`Float::cbrt_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Cbrt;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.cbrt().is_nan());
    /// assert_eq!(Float::INFINITY.cbrt(), Float::INFINITY);
    /// assert_eq!(Float::NEGATIVE_INFINITY.cbrt(), Float::NEGATIVE_INFINITY);
    /// assert_eq!(Float::from(27.0).cbrt(), 3.0);
    /// assert_eq!(Float::from(-8.0).cbrt(), -2.0);
    /// ```
    #[inline]
    fn cbrt(self) -> Self {
        let prec = self.significant_bits();
        self.cbrt_prec(prec).0
    }
}

impl Cbrt for &Float {
    type Output = Float;

    /// Takes the cube root of a [`Float`], taking it by reference.
    ///
    /// If the output has a precision, it is the precision of the input. If the cube root is
    /// equidistant from two [`Float`]s with that precision, the [`Float`] with fewer 1s in its
    /// binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding
    /// mode.
    ///
    /// Unlike the square root, the cube root of a negative number is a (negative) real number, so a
    /// negative input does not produce a `NaN`.
    ///
    /// $$
    /// f(x) = \sqrt[3]{x}+\varepsilon.
    /// $$
    /// - If $\sqrt[3]{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\sqrt[3]{x}$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\sqrt[3]{x}|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=\infty$
    /// - $f(-\infty)=-\infty$
    /// - $f(0.0)=0.0$
    /// - $f(-0.0)=-0.0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::cbrt_prec_ref`] instead. If you want to specify the output precision, consider
    /// using [`Float::cbrt_round_ref`]. If you want both of these things, consider using
    /// [`Float::cbrt_prec_round_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Cbrt;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::NAN).cbrt().is_nan());
    /// assert_eq!((&Float::INFINITY).cbrt(), Float::INFINITY);
    /// assert_eq!((&Float::NEGATIVE_INFINITY).cbrt(), Float::NEGATIVE_INFINITY);
    /// assert_eq!((&Float::from(27.0)).cbrt(), 3.0);
    /// assert_eq!((&Float::from(-8.0)).cbrt(), -2.0);
    /// ```
    #[inline]
    fn cbrt(self) -> Float {
        let prec = self.significant_bits();
        self.cbrt_prec_ref(prec).0
    }
}

impl CbrtAssign for Float {
    /// Takes the cube root of a [`Float`] in place.
    ///
    /// If the output has a precision, it is the precision of the input. If the cube root is
    /// equidistant from two [`Float`]s with that precision, the [`Float`] with fewer 1s in its
    /// binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding
    /// mode.
    ///
    /// Unlike the square root, the cube root of a negative number is a (negative) real number, so a
    /// negative input does not produce a `NaN`.
    ///
    /// $$
    /// x\gets = \sqrt[3]{x}+\varepsilon.
    /// $$
    /// - If $\sqrt[3]{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\sqrt[3]{x}$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\sqrt[3]{x}|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// See the [`Float::cbrt`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CbrtAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(27.0);
    /// x.cbrt_assign();
    /// assert_eq!(x, 3.0);
    ///
    /// let mut x = Float::from(-8.0);
    /// x.cbrt_assign();
    /// assert_eq!(x, -2.0);
    /// ```
    #[inline]
    fn cbrt_assign(&mut self) {
        let prec = self.significant_bits();
        self.cbrt_prec_assign(prec);
    }
}

/// Takes the cube root of a primitive float, returning a correctly-rounded result.
///
/// Unlike the `cbrt` methods of `f32` and `f64` (which are not guaranteed to be correctly rounded),
/// this function returns the primitive float closest to the real cube root of the input, with ties
/// rounded to even. It is computed by taking the cube root of the exact value of the input at the
/// appropriate precision.
///
/// $$
/// f(x) = \sqrt[3]{x}.
/// $$
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
/// use malachite_float::arithmetic::cbrt::primitive_float_cbrt;
///
/// assert!(primitive_float_cbrt::<f64>(f64::NAN).is_nan());
/// assert_eq!(primitive_float_cbrt::<f64>(f64::INFINITY), f64::INFINITY);
/// assert_eq!(primitive_float_cbrt::<f64>(f64::NEGATIVE_INFINITY), f64::NEGATIVE_INFINITY);
/// assert_eq!(primitive_float_cbrt::<f64>(27.0), 3.0);
/// assert_eq!(primitive_float_cbrt::<f64>(-8.0), -2.0);
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_cbrt<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_root_u(x, 3)
}

/// Takes the cube root of a [`Rational`], returning a correctly-rounded primitive float.
///
/// The returned primitive float is the one closest to the real cube root of the input, with ties
/// rounded to even.
///
/// $$
/// f(x) = \sqrt[3]{x}.
/// $$
///
/// # Worst-case complexity
/// $T(n) = O(n \log n \log\log n)$
///
/// $M(n) = O(n \log n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
///
/// # Examples
/// ```
/// use malachite_float::arithmetic::cbrt::primitive_float_cbrt_rational;
/// use malachite_q::Rational;
///
/// assert_eq!(primitive_float_cbrt_rational::<f64>(&Rational::from(27)), 3.0);
/// assert_eq!(primitive_float_cbrt_rational::<f64>(&Rational::from(-8)), -2.0);
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_cbrt_rational<T: PrimitiveFloat>(x: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_root_u_rational(x, 3)
}
