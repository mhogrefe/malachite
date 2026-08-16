// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright © 1999-2025 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::emulate_float_to_float_fn;
use malachite_base::num::basic::floats::PrimitiveFloat;

use crate::Float;
use crate::InnerFloat::{Finite, Infinity, Zero};
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{ModPowerOf2, NegAssign, NegModPowerOf2};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::platform::Limb;

impl Float {
    /// Returns the fractional part of a [`Float`], rounded to the specified precision with the
    /// specified rounding mode, along with an [`Ordering`] comparing the result to the exact
    /// fraction. The [`Float`] is taken by value.
    ///
    /// The fractional part has the same sign as the input, and the rounding mode rounds the exact
    /// fraction rather than shaping it: for a negative input, `Floor` rounds the (negative)
    /// fraction downward. The fractional part of an integer or an infinity is a zero with the
    /// input's sign, and `NaN` propagates. The [`Ordering`]s compare each result to its exact
    /// value; whenever a result equals its exact value, its [`Ordering`] is `Equal`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fraction is not exactly
    /// representable at the target precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(3.25f64);
    /// assert_eq!(
    ///     x.fractional_part_prec_round_ref(10, Floor),
    ///     (Float::from(0.25f64), Equal)
    /// );
    /// // the fraction of a negative value is negative, and Floor rounds it downward
    /// let y = Float::from(-3.375f64);
    /// assert_eq!(
    ///     y.fractional_part_prec_round(1, Floor),
    ///     (Float::from(-0.5f64), Less)
    /// );
    /// ```
    #[inline]
    pub fn fractional_part_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.fractional_part_helper(prec, rm)
    }

    /// Returns the fractional part of a [`Float`], rounded to the specified precision with the
    /// specified rounding mode, along with an [`Ordering`] comparing the result to the exact
    /// fraction. The [`Float`] is taken by reference.
    ///
    /// The fractional part has the same sign as the input, and the rounding mode rounds the exact
    /// fraction rather than shaping it: for a negative input, `Floor` rounds the (negative)
    /// fraction downward. The fractional part of an integer or an infinity is a zero with the
    /// input's sign, and `NaN` propagates. The [`Ordering`]s compare each result to its exact
    /// value; whenever a result equals its exact value, its [`Ordering`] is `Equal`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fraction is not exactly
    /// representable at the target precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(3.25f64);
    /// assert_eq!(
    ///     x.fractional_part_prec_round_ref(10, Floor),
    ///     (Float::from(0.25f64), Equal)
    /// );
    /// // the fraction of a negative value is negative, and Floor rounds it downward
    /// let y = Float::from(-3.375f64);
    /// assert_eq!(
    ///     y.fractional_part_prec_round(1, Floor),
    ///     (Float::from(-0.5f64), Less)
    /// );
    /// ```
    #[inline]
    pub fn fractional_part_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.fractional_part_helper(prec, rm)
    }

    /// Returns the fractional part of a [`Float`], rounded to the specified precision with the
    /// `Nearest` rounding mode, along with an [`Ordering`] comparing the result to the exact
    /// fraction. The [`Float`] is taken by value.
    ///
    /// The fractional part has the same sign as the input, and the rounding mode rounds the exact
    /// fraction rather than shaping it: for a negative input, `Floor` rounds the (negative)
    /// fraction downward. The fractional part of an integer or an infinity is a zero with the
    /// input's sign, and `NaN` propagates. The [`Ordering`]s compare each result to its exact
    /// value; whenever a result equals its exact value, its [`Ordering`] is `Equal`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(3.25f64);
    /// assert_eq!(
    ///     x.fractional_part_prec_ref(10),
    ///     (Float::from(0.25f64), Equal)
    /// );
    /// ```
    #[inline]
    pub fn fractional_part_prec(self, prec: u64) -> (Self, Ordering) {
        self.fractional_part_helper(prec, Nearest)
    }

    /// Returns the fractional part of a [`Float`], rounded to the specified precision with the
    /// `Nearest` rounding mode, along with an [`Ordering`] comparing the result to the exact
    /// fraction. The [`Float`] is taken by reference.
    ///
    /// The fractional part has the same sign as the input, and the rounding mode rounds the exact
    /// fraction rather than shaping it: for a negative input, `Floor` rounds the (negative)
    /// fraction downward. The fractional part of an integer or an infinity is a zero with the
    /// input's sign, and `NaN` propagates. The [`Ordering`]s compare each result to its exact
    /// value; whenever a result equals its exact value, its [`Ordering`] is `Equal`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(3.25f64);
    /// assert_eq!(
    ///     x.fractional_part_prec_ref(10),
    ///     (Float::from(0.25f64), Equal)
    /// );
    /// ```
    #[inline]
    pub fn fractional_part_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.fractional_part_helper(prec, Nearest)
    }

    /// Returns the fractional part of a [`Float`], rounded to the input's precision with the
    /// specified rounding mode, along with an [`Ordering`] comparing the result to the exact
    /// fraction. The [`Float`] is taken by value.
    ///
    /// The fractional part has the same sign as the input, and the rounding mode rounds the exact
    /// fraction rather than shaping it: for a negative input, `Floor` rounds the (negative)
    /// fraction downward. The fractional part of an integer or an infinity is a zero with the
    /// input's sign, and `NaN` propagates. The [`Ordering`]s compare each result to its exact
    /// value; whenever a result equals its exact value, its [`Ordering`] is `Equal`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the fraction is not exactly representable at the input's
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(3.25f64);
    /// assert_eq!(
    ///     x.fractional_part_round_ref(Ceiling),
    ///     (Float::from(0.25f64), Equal)
    /// );
    /// ```
    #[inline]
    pub fn fractional_part_round(self, rm: RoundingMode) -> (Self, Ordering) {
        self.fractional_part_helper(self.significant_bits(), rm)
    }

    /// Returns the fractional part of a [`Float`], rounded to the input's precision with the
    /// specified rounding mode, along with an [`Ordering`] comparing the result to the exact
    /// fraction. The [`Float`] is taken by reference.
    ///
    /// The fractional part has the same sign as the input, and the rounding mode rounds the exact
    /// fraction rather than shaping it: for a negative input, `Floor` rounds the (negative)
    /// fraction downward. The fractional part of an integer or an infinity is a zero with the
    /// input's sign, and `NaN` propagates. The [`Ordering`]s compare each result to its exact
    /// value; whenever a result equals its exact value, its [`Ordering`] is `Equal`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the fraction is not exactly representable at the input's
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(3.25f64);
    /// assert_eq!(
    ///     x.fractional_part_round_ref(Ceiling),
    ///     (Float::from(0.25f64), Equal)
    /// );
    /// ```
    #[inline]
    pub fn fractional_part_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.fractional_part_helper(self.significant_bits(), rm)
    }

    /// Returns the fractional part of a [`Float`], rounded to the input's precision with the
    /// `Nearest` rounding mode, along with an [`Ordering`] comparing the result to the exact
    /// fraction. The [`Float`] is taken by value.
    ///
    /// The fractional part has the same sign as the input, and the rounding mode rounds the exact
    /// fraction rather than shaping it: for a negative input, `Floor` rounds the (negative)
    /// fraction downward. The fractional part of an integer or an infinity is a zero with the
    /// input's sign, and `NaN` propagates. The [`Ordering`]s compare each result to its exact
    /// value; whenever a result equals its exact value, its [`Ordering`] is `Equal`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Never panics.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(3.25f64);
    /// assert_eq!(x.fractional_part_ref(), (Float::from(0.25f64), Equal));
    /// ```
    #[inline]
    pub fn fractional_part(self) -> (Self, Ordering) {
        self.fractional_part_helper(self.significant_bits(), Nearest)
    }

    /// Returns the fractional part of a [`Float`], rounded to the input's precision with the
    /// `Nearest` rounding mode, along with an [`Ordering`] comparing the result to the exact
    /// fraction. The [`Float`] is taken by reference.
    ///
    /// The fractional part has the same sign as the input, and the rounding mode rounds the exact
    /// fraction rather than shaping it: for a negative input, `Floor` rounds the (negative)
    /// fraction downward. The fractional part of an integer or an infinity is a zero with the
    /// input's sign, and `NaN` propagates. The [`Ordering`]s compare each result to its exact
    /// value; whenever a result equals its exact value, its [`Ordering`] is `Equal`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Never panics.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(3.25f64);
    /// assert_eq!(x.fractional_part_ref(), (Float::from(0.25f64), Equal));
    /// ```
    #[inline]
    pub fn fractional_part_ref(&self) -> (Self, Ordering) {
        self.fractional_part_helper(self.significant_bits(), Nearest)
    }

    /// Returns the integral and fractional parts of a [`Float`], rounded to the specified
    /// precisions with the specified rounding mode. The [`Float`] is taken by value.
    ///
    /// The integral part is the input truncated toward zero and then correctly rounded to its
    /// target precision, as by [`Float::round_to_integer_then_prec_round`] with `Down`; the
    /// fractional part is as by [`Float::fractional_part_prec_round`]. Both parts have the input's
    /// sign; for an infinity, the integral part is the infinity and the fractional part a signed
    /// zero, and `NaN` propagates to both parts. The [`Ordering`]s compare each result to its exact
    /// value; whenever a result equals its exact value, its [`Ordering`] is `Equal`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(iprec, fprec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `iprec` or `fprec` is zero, or if `rm` is `Exact` and either part is not exactly
    /// representable at its target precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(-3.25f64);
    /// let ((i, io), (f, fo)) = x.integer_and_fractional_parts_prec_round_ref(10, 10, Nearest);
    /// assert_eq!(i, Float::from(-3i32));
    /// assert_eq!(f, Float::from(-0.25f64));
    /// assert_eq!((io, fo), (Equal, Equal));
    /// ```
    #[inline]
    pub fn integer_and_fractional_parts_prec_round(
        self,
        iprec: u64,
        fprec: u64,
        rm: RoundingMode,
    ) -> ((Self, Ordering), (Self, Ordering)) {
        self.integer_and_fractional_parts_helper(iprec, fprec, rm)
    }

    /// Returns the integral and fractional parts of a [`Float`], rounded to the specified
    /// precisions with the specified rounding mode. The [`Float`] is taken by reference.
    ///
    /// The integral part is the input truncated toward zero and then correctly rounded to its
    /// target precision, as by [`Float::round_to_integer_then_prec_round`] with `Down`; the
    /// fractional part is as by [`Float::fractional_part_prec_round`]. Both parts have the input's
    /// sign; for an infinity, the integral part is the infinity and the fractional part a signed
    /// zero, and `NaN` propagates to both parts. The [`Ordering`]s compare each result to its exact
    /// value; whenever a result equals its exact value, its [`Ordering`] is `Equal`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(iprec, fprec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `iprec` or `fprec` is zero, or if `rm` is `Exact` and either part is not exactly
    /// representable at its target precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(-3.25f64);
    /// let ((i, io), (f, fo)) = x.integer_and_fractional_parts_prec_round_ref(10, 10, Nearest);
    /// assert_eq!(i, Float::from(-3i32));
    /// assert_eq!(f, Float::from(-0.25f64));
    /// assert_eq!((io, fo), (Equal, Equal));
    /// ```
    #[inline]
    pub fn integer_and_fractional_parts_prec_round_ref(
        &self,
        iprec: u64,
        fprec: u64,
        rm: RoundingMode,
    ) -> ((Self, Ordering), (Self, Ordering)) {
        self.integer_and_fractional_parts_helper(iprec, fprec, rm)
    }

    /// Returns the integral and fractional parts of a [`Float`], rounded to the specified
    /// precisions with the `Nearest` rounding mode. The [`Float`] is taken by value.
    ///
    /// The integral part is the input truncated toward zero and then correctly rounded to its
    /// target precision, as by [`Float::round_to_integer_then_prec_round`] with `Down`; the
    /// fractional part is as by [`Float::fractional_part_prec_round`]. Both parts have the input's
    /// sign; for an infinity, the integral part is the infinity and the fractional part a signed
    /// zero, and `NaN` propagates to both parts. The [`Ordering`]s compare each result to its exact
    /// value; whenever a result equals its exact value, its [`Ordering`] is `Equal`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(iprec, fprec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `iprec` or `fprec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(3.25f64);
    /// let ((i, io), (f, fo)) = x.integer_and_fractional_parts_prec_ref(10, 10);
    /// assert_eq!(i, Float::from(3u32));
    /// assert_eq!(f, Float::from(0.25f64));
    /// assert_eq!((io, fo), (Equal, Equal));
    /// ```
    #[inline]
    pub fn integer_and_fractional_parts_prec(
        self,
        iprec: u64,
        fprec: u64,
    ) -> ((Self, Ordering), (Self, Ordering)) {
        self.integer_and_fractional_parts_helper(iprec, fprec, Nearest)
    }

    /// Returns the integral and fractional parts of a [`Float`], rounded to the specified
    /// precisions with the `Nearest` rounding mode. The [`Float`] is taken by reference.
    ///
    /// The integral part is the input truncated toward zero and then correctly rounded to its
    /// target precision, as by [`Float::round_to_integer_then_prec_round`] with `Down`; the
    /// fractional part is as by [`Float::fractional_part_prec_round`]. Both parts have the input's
    /// sign; for an infinity, the integral part is the infinity and the fractional part a signed
    /// zero, and `NaN` propagates to both parts. The [`Ordering`]s compare each result to its exact
    /// value; whenever a result equals its exact value, its [`Ordering`] is `Equal`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(iprec, fprec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `iprec` or `fprec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(3.25f64);
    /// let ((i, io), (f, fo)) = x.integer_and_fractional_parts_prec_ref(10, 10);
    /// assert_eq!(i, Float::from(3u32));
    /// assert_eq!(f, Float::from(0.25f64));
    /// assert_eq!((io, fo), (Equal, Equal));
    /// ```
    #[inline]
    pub fn integer_and_fractional_parts_prec_ref(
        &self,
        iprec: u64,
        fprec: u64,
    ) -> ((Self, Ordering), (Self, Ordering)) {
        self.integer_and_fractional_parts_helper(iprec, fprec, Nearest)
    }

    /// Returns the integral and fractional parts of a [`Float`], rounded to the input's precision
    /// with the specified rounding mode. The [`Float`] is taken by value.
    ///
    /// The integral part is the input truncated toward zero and then correctly rounded to its
    /// target precision, as by [`Float::round_to_integer_then_prec_round`] with `Down`; the
    /// fractional part is as by [`Float::fractional_part_prec_round`]. Both parts have the input's
    /// sign; for an infinity, the integral part is the infinity and the fractional part a signed
    /// zero, and `NaN` propagates to both parts. The [`Ordering`]s compare each result to its exact
    /// value; whenever a result equals its exact value, its [`Ordering`] is `Equal`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the fraction is not exactly representable at the input's
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(3.25f64);
    /// let ((i, io), (f, fo)) = x.integer_and_fractional_parts_round_ref(Floor);
    /// assert_eq!(i, Float::from(3u32));
    /// assert_eq!(f, Float::from(0.25f64));
    /// assert_eq!((io, fo), (Equal, Equal));
    /// ```
    #[inline]
    pub fn integer_and_fractional_parts_round(
        self,
        rm: RoundingMode,
    ) -> ((Self, Ordering), (Self, Ordering)) {
        let prec = self.significant_bits();
        self.integer_and_fractional_parts_helper(prec, prec, rm)
    }

    /// Returns the integral and fractional parts of a [`Float`], rounded to the input's precision
    /// with the specified rounding mode. The [`Float`] is taken by reference.
    ///
    /// The integral part is the input truncated toward zero and then correctly rounded to its
    /// target precision, as by [`Float::round_to_integer_then_prec_round`] with `Down`; the
    /// fractional part is as by [`Float::fractional_part_prec_round`]. Both parts have the input's
    /// sign; for an infinity, the integral part is the infinity and the fractional part a signed
    /// zero, and `NaN` propagates to both parts. The [`Ordering`]s compare each result to its exact
    /// value; whenever a result equals its exact value, its [`Ordering`] is `Equal`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the fraction is not exactly representable at the input's
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(3.25f64);
    /// let ((i, io), (f, fo)) = x.integer_and_fractional_parts_round_ref(Floor);
    /// assert_eq!(i, Float::from(3u32));
    /// assert_eq!(f, Float::from(0.25f64));
    /// assert_eq!((io, fo), (Equal, Equal));
    /// ```
    #[inline]
    pub fn integer_and_fractional_parts_round_ref(
        &self,
        rm: RoundingMode,
    ) -> ((Self, Ordering), (Self, Ordering)) {
        let prec = self.significant_bits();
        self.integer_and_fractional_parts_helper(prec, prec, rm)
    }

    /// Returns the integral and fractional parts of a [`Float`], rounded to the input's precision
    /// with the `Nearest` rounding mode. The [`Float`] is taken by value.
    ///
    /// The integral part is the input truncated toward zero and then correctly rounded to its
    /// target precision, as by [`Float::round_to_integer_then_prec_round`] with `Down`; the
    /// fractional part is as by [`Float::fractional_part_prec_round`]. Both parts have the input's
    /// sign; for an infinity, the integral part is the infinity and the fractional part a signed
    /// zero, and `NaN` propagates to both parts. The [`Ordering`]s compare each result to its exact
    /// value; whenever a result equals its exact value, its [`Ordering`] is `Equal`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Never panics.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(3.25f64);
    /// let ((i, io), (f, fo)) = x.integer_and_fractional_parts_ref();
    /// assert_eq!(i, Float::from(3u32));
    /// assert_eq!(f, Float::from(0.25f64));
    /// assert_eq!((io, fo), (Equal, Equal));
    /// ```
    #[inline]
    pub fn integer_and_fractional_parts(self) -> ((Self, Ordering), (Self, Ordering)) {
        let prec = self.significant_bits();
        self.integer_and_fractional_parts_helper(prec, prec, Nearest)
    }

    /// Returns the integral and fractional parts of a [`Float`], rounded to the input's precision
    /// with the `Nearest` rounding mode. The [`Float`] is taken by reference.
    ///
    /// The integral part is the input truncated toward zero and then correctly rounded to its
    /// target precision, as by [`Float::round_to_integer_then_prec_round`] with `Down`; the
    /// fractional part is as by [`Float::fractional_part_prec_round`]. Both parts have the input's
    /// sign; for an infinity, the integral part is the infinity and the fractional part a signed
    /// zero, and `NaN` propagates to both parts. The [`Ordering`]s compare each result to its exact
    /// value; whenever a result equals its exact value, its [`Ordering`] is `Equal`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Never panics.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(3.25f64);
    /// let ((i, io), (f, fo)) = x.integer_and_fractional_parts_ref();
    /// assert_eq!(i, Float::from(3u32));
    /// assert_eq!(f, Float::from(0.25f64));
    /// assert_eq!((io, fo), (Equal, Equal));
    /// ```
    #[inline]
    pub fn integer_and_fractional_parts_ref(&self) -> ((Self, Ordering), (Self, Ordering)) {
        let prec = self.significant_bits();
        self.integer_and_fractional_parts_helper(prec, prec, Nearest)
    }

    // This is mpfr_frac from frac.c, MPFR 4.2.2, with the result's precision passed explicitly.
    // Rather than MPFR's in-place limb manipulation, the fractional bits are extracted with a mask
    // and rounded in a single step, which also handles the case of a fraction too small for the
    // exponent range (MPFR relies on an extended exponent range there).
    fn fractional_part_helper(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        let Self(Finite {
            sign,
            exponent,
            precision,
            significand,
        }) = self
        else {
            return match self {
                Self(Infinity { sign }) => (Self(Zero { sign: *sign }), Equal),
                _ => (self.clone(), Equal),
            };
        };
        let sign = *sign;
        let exp = i64::from(*exponent);
        if exp <= 0 {
            // 0 < |u| < 1: the value is its own fractional part
            return Self::from_float_prec_round_ref(self, prec, rm);
        }
        let total = i64::exact_from(precision.neg_mod_power_of_2(Limb::LOG_WIDTH) + *precision);
        if exp >= total {
            // all significand bits belong to the integer part
            return (Self(Zero { sign }), Equal);
        }
        let frac = significand.mod_power_of_2(u64::exact_from(total - exp));
        if frac == 0u32 {
            // u is an integer
            return (Self(Zero { sign }), Equal);
        }
        // The exact fractional part is frac * 2^(exp - total). Negate before rounding, since the
        // directed rounding modes do not commute with negation.
        let mut exact = Self::exact_from(frac);
        if !sign {
            exact.neg_assign();
        }
        exact.shr_prec_round(total - exp, prec, rm)
    }

    // This is mpfr_modf from modf.c, MPFR 4.2.2, with the two results' precisions passed
    // explicitly. The integral part is rounded as by [`Float::round_to_integer_then_prec_round`]
    // with `Down` (truncation, then a rounding to the target precision), and the fractional part as
    // by [`Float::fractional_part_prec_round`].
    fn integer_and_fractional_parts_helper(
        &self,
        iprec: u64,
        fprec: u64,
        rm: RoundingMode,
    ) -> ((Self, Ordering), (Self, Ordering)) {
        assert_ne!(iprec, 0);
        assert_ne!(fprec, 0);
        let Self(Finite {
            sign,
            exponent,
            precision,
            ..
        }) = self
        else {
            return match self {
                Self(Infinity { sign }) => {
                    ((self.clone(), Equal), (Self(Zero { sign: *sign }), Equal))
                }
                _ => ((self.clone(), Equal), (self.clone(), Equal)),
            };
        };
        let sign = *sign;
        let exp = i64::from(*exponent);
        if exp <= 0 {
            // 0 < |u| < 1: the integral part is zero and the fractional part is the value
            (
                (Self(Zero { sign }), Equal),
                Self::from_float_prec_round_ref(self, fprec, rm),
            )
        } else if exp >= i64::exact_from(*precision) {
            // u has no fractional part
            (
                Self::from_float_prec_round_ref(self, iprec, rm),
                (Self(Zero { sign }), Equal),
            )
        } else {
            (
                self.round_to_integer_then_prec_round_ref(Down, iprec, rm),
                self.fractional_part_helper(fprec, rm),
            )
        }
    }
}

/// Computes the fractional part of a primitive float, using emulated [`Float`] arithmetic.
///
/// The result is always exactly representable, matching the standard library's `fract` for finite
/// values with a nonzero fractional part; it serves as a reference implementation. As in
/// `mpfr_frac`, a zero result takes the input's sign (where `fract` of a negative integer is a
/// positive zero), and the fractional part of an infinity is a zero of the same sign (where `fract`
/// returns NaN).
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::fractional_part::primitive_float_fractional_part;
///
/// assert_eq!(
///     NiceFloat(primitive_float_fractional_part(10.5)),
///     NiceFloat(0.5)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_fractional_part(-10.5)),
///     NiceFloat(-0.5)
/// );
/// ```
#[allow(clippy::type_repetition_in_bounds)]
#[inline]
pub fn primitive_float_fractional_part<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::fractional_part_prec, x)
}

/// Computes the integer and fractional parts of a primitive float, using emulated [`Float`]
/// arithmetic.
///
/// Both parts are always exactly representable, and their sum is the input; this matches
/// `x.trunc()` and `x.fract()` for finite values (up to the sign of a zero fraction, which follows
/// the input as in `mpfr_modf`) and serves as a reference implementation. An infinity keeps its
/// integer part and has a zero fraction.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::fractional_part::*;
///
/// let (i, f) = primitive_float_integer_and_fractional_parts(10.5);
/// assert_eq!(NiceFloat(i), NiceFloat(10.0));
/// assert_eq!(NiceFloat(f), NiceFloat(0.5));
/// ```
#[allow(clippy::type_repetition_in_bounds)]
#[inline]
pub fn primitive_float_integer_and_fractional_parts<T: PrimitiveFloat>(x: T) -> (T, T)
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    (
        emulate_float_to_float_fn(
            |x, prec| x.integer_and_fractional_parts_prec(prec, prec).0,
            x,
        ),
        emulate_float_to_float_fn(
            |x, prec| x.integer_and_fractional_parts_prec(prec, prec).1,
            x,
        ),
    )
}
