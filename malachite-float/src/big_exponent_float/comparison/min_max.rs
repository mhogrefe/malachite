// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use core::cmp::Ordering::{self, *};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_q::Rational;

// Whether min or max selects the second operand. One NaN gives the other operand, and two NaNs give
// NaN. Equal values have the same representation, so ties may go to either operand.
fn selects_second(x: &BigExponentFloat, y: &BigExponentFloat, is_max: bool) -> bool {
    match (x.is_nan(), y.is_nan()) {
        (_, true) => false,
        (true, false) => true,
        (false, false) => x.partial_cmp(y).unwrap() == if is_max { Less } else { Greater },
    }
}

// Whether min or max of a `BigExponentFloat` and a `Rational` selects the `Rational`. The
// comparison is exact, so only the winner is rounded; a NaN `BigExponentFloat` yields the
// `Rational`, and on a tie the `BigExponentFloat` is selected.
fn selects_rational(x: &BigExponentFloat, y: &Rational, is_max: bool) -> bool {
    x.partial_cmp(y)
        .is_none_or(|o| o == if is_max { Less } else { Greater })
}

impl BigExponentFloat {
    /// Returns the minimum of two [`BigExponentFloat`]s. The result is exact.
    ///
    /// If one of the operands is NaN, the other operand is returned; if both are NaN, the result is
    /// NaN. Otherwise, the smaller operand is returned. This is the IEEE 754 minNum/maxNum
    /// convention.
    ///
    /// Both [`BigExponentFloat`]s are taken by value.
    ///
    /// If you want to round the result, consider using [`BigExponentFloat::min_prec_round`] or
    /// [`BigExponentFloat::min_prec`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::num::basic::traits::NaN;
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     BigExponentFloat::from(PI).min(BigExponentFloat::from(E)),
    ///     BigExponentFloat::from(E)
    /// );
    /// assert_eq!(
    ///     BigExponentFloat::NAN.min(BigExponentFloat::from(PI)),
    ///     BigExponentFloat::from(PI)
    /// );
    /// assert!(BigExponentFloat::NAN.min(BigExponentFloat::NAN).is_nan());
    /// ```
    pub fn min(self, other: Self) -> Self {
        if selects_second(&self, &other, false) {
            other
        } else {
            self
        }
    }

    /// Returns the minimum of two [`BigExponentFloat`]s. The result is exact.
    ///
    /// If one of the operands is NaN, the other operand is returned; if both are NaN, the result is
    /// NaN. Otherwise, the smaller operand is returned. This is the IEEE 754 minNum/maxNum
    /// convention.
    ///
    /// The first [`BigExponentFloat`] is taken by value and the second by reference.
    ///
    /// If you want to round the result, consider using [`BigExponentFloat::min_prec_round_val_ref`]
    /// or [`BigExponentFloat::min_prec_val_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::num::basic::traits::NaN;
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     BigExponentFloat::from(PI).min_val_ref(&BigExponentFloat::from(E)),
    ///     BigExponentFloat::from(E)
    /// );
    /// assert_eq!(
    ///     BigExponentFloat::NAN.min_val_ref(&BigExponentFloat::from(PI)),
    ///     BigExponentFloat::from(PI)
    /// );
    /// assert!(
    ///     BigExponentFloat::NAN
    ///         .min_val_ref(&BigExponentFloat::NAN)
    ///         .is_nan()
    /// );
    /// ```
    pub fn min_val_ref(self, other: &Self) -> Self {
        if selects_second(&self, other, false) {
            other.clone()
        } else {
            self
        }
    }

    /// Returns the minimum of two [`BigExponentFloat`]s. The result is exact.
    ///
    /// If one of the operands is NaN, the other operand is returned; if both are NaN, the result is
    /// NaN. Otherwise, the smaller operand is returned. This is the IEEE 754 minNum/maxNum
    /// convention.
    ///
    /// The first [`BigExponentFloat`] is taken by reference and the second by value.
    ///
    /// If you want to round the result, consider using [`BigExponentFloat::min_prec_round_ref_val`]
    /// or [`BigExponentFloat::min_prec_ref_val`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::num::basic::traits::NaN;
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     BigExponentFloat::from(PI).min_ref_val(BigExponentFloat::from(E)),
    ///     BigExponentFloat::from(E)
    /// );
    /// assert_eq!(
    ///     BigExponentFloat::NAN.min_ref_val(BigExponentFloat::from(PI)),
    ///     BigExponentFloat::from(PI)
    /// );
    /// assert!(
    ///     BigExponentFloat::NAN
    ///         .min_ref_val(BigExponentFloat::NAN)
    ///         .is_nan()
    /// );
    /// ```
    pub fn min_ref_val(&self, other: Self) -> Self {
        if selects_second(self, &other, false) {
            other
        } else {
            self.clone()
        }
    }

    /// Returns the minimum of two [`BigExponentFloat`]s. The result is exact.
    ///
    /// If one of the operands is NaN, the other operand is returned; if both are NaN, the result is
    /// NaN. Otherwise, the smaller operand is returned. This is the IEEE 754 minNum/maxNum
    /// convention.
    ///
    /// Both [`BigExponentFloat`]s are taken by reference.
    ///
    /// If you want to round the result, consider using [`BigExponentFloat::min_prec_round_ref_ref`]
    /// or [`BigExponentFloat::min_prec_ref_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::num::basic::traits::NaN;
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     BigExponentFloat::from(PI).min_ref_ref(&BigExponentFloat::from(E)),
    ///     BigExponentFloat::from(E)
    /// );
    /// assert_eq!(
    ///     BigExponentFloat::NAN.min_ref_ref(&BigExponentFloat::from(PI)),
    ///     BigExponentFloat::from(PI)
    /// );
    /// assert!(
    ///     BigExponentFloat::NAN
    ///         .min_ref_ref(&BigExponentFloat::NAN)
    ///         .is_nan()
    /// );
    /// ```
    pub fn min_ref_ref(&self, other: &Self) -> Self {
        if selects_second(self, other, false) {
            other.clone()
        } else {
            self.clone()
        }
    }

    /// Returns the minimum of two [`BigExponentFloat`]s, rounding the result to the specified
    /// precision and with the specified rounding mode. An [`Ordering`] is also returned, indicating
    /// whether the rounded minimum is less than, equal to, or greater than the exact minimum.
    /// Whenever this function returns a NaN it also returns `Equal`.
    ///
    /// The operand is selected as by [`BigExponentFloat::min`], and is then rounded to `prec` bits
    /// using `rm`, as by [`BigExponentFloat::round_to_prec_round`].
    ///
    /// Both [`BigExponentFloat`]s are taken by value.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::min_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the selected operand cannot be
    /// represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = BigExponentFloat::from(PI).min_prec_round(BigExponentFloat::from(E), 5, Floor);
    /// assert_eq!(format!("{x:?}"), "0x2.a");
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) =
    ///     BigExponentFloat::from(PI).min_prec_round(BigExponentFloat::from(E), 5, Ceiling);
    /// assert_eq!(format!("{x:?}"), "0x2.c");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn min_prec_round(self, other: Self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        if selects_second(&self, &other, false) {
            other.round_to_prec_round(prec, rm)
        } else {
            self.round_to_prec_round(prec, rm)
        }
    }

    /// Returns the minimum of two [`BigExponentFloat`]s, rounding the result to the specified
    /// precision and with the specified rounding mode. An [`Ordering`] is also returned, indicating
    /// whether the rounded minimum is less than, equal to, or greater than the exact minimum.
    /// Whenever this function returns a NaN it also returns `Equal`.
    ///
    /// The operand is selected as by [`BigExponentFloat::min_val_ref`], and is then rounded to
    /// `prec` bits using `rm`, as by [`BigExponentFloat::round_to_prec_round`].
    ///
    /// The first [`BigExponentFloat`] is taken by value and the second by reference.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::min_prec_val_ref`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the selected operand cannot be
    /// represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) =
    ///     BigExponentFloat::from(PI).min_prec_round_val_ref(&BigExponentFloat::from(E), 5, Floor);
    /// assert_eq!(format!("{x:?}"), "0x2.a");
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) = BigExponentFloat::from(PI).min_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.c");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn min_prec_round_val_ref(
        self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        if selects_second(&self, other, false) {
            other.round_to_prec_round_ref(prec, rm)
        } else {
            self.round_to_prec_round(prec, rm)
        }
    }

    /// Returns the minimum of two [`BigExponentFloat`]s, rounding the result to the specified
    /// precision and with the specified rounding mode. An [`Ordering`] is also returned, indicating
    /// whether the rounded minimum is less than, equal to, or greater than the exact minimum.
    /// Whenever this function returns a NaN it also returns `Equal`.
    ///
    /// The operand is selected as by [`BigExponentFloat::min_ref_val`], and is then rounded to
    /// `prec` bits using `rm`, as by [`BigExponentFloat::round_to_prec_round`].
    ///
    /// The first [`BigExponentFloat`] is taken by reference and the second by value.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::min_prec_ref_val`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the selected operand cannot be
    /// represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) =
    ///     BigExponentFloat::from(PI).min_prec_round_ref_val(BigExponentFloat::from(E), 5, Floor);
    /// assert_eq!(format!("{x:?}"), "0x2.a");
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) = BigExponentFloat::from(PI).min_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.c");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn min_prec_round_ref_val(
        &self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        if selects_second(self, &other, false) {
            other.round_to_prec_round(prec, rm)
        } else {
            self.round_to_prec_round_ref(prec, rm)
        }
    }

    /// Returns the minimum of two [`BigExponentFloat`]s, rounding the result to the specified
    /// precision and with the specified rounding mode. An [`Ordering`] is also returned, indicating
    /// whether the rounded minimum is less than, equal to, or greater than the exact minimum.
    /// Whenever this function returns a NaN it also returns `Equal`.
    ///
    /// The operand is selected as by [`BigExponentFloat::min_ref_ref`], and is then rounded to
    /// `prec` bits using `rm`, as by [`BigExponentFloat::round_to_prec_round`].
    ///
    /// Both [`BigExponentFloat`]s are taken by reference.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::min_prec_ref_ref`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the selected operand cannot be
    /// represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) =
    ///     BigExponentFloat::from(PI).min_prec_round_ref_ref(&BigExponentFloat::from(E), 5, Floor);
    /// assert_eq!(format!("{x:?}"), "0x2.a");
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) = BigExponentFloat::from(PI).min_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.c");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn min_prec_round_ref_ref(
        &self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        if selects_second(self, other, false) {
            other.round_to_prec_round_ref(prec, rm)
        } else {
            self.round_to_prec_round_ref(prec, rm)
        }
    }

    /// Returns the minimum of two [`BigExponentFloat`]s, rounding the result to the nearest value
    /// of the specified precision. An [`Ordering`] is also returned, indicating whether the rounded
    /// minimum is less than, equal to, or greater than the exact minimum. Whenever this function
    /// returns a NaN it also returns `Equal`.
    ///
    /// The operand is selected as by [`BigExponentFloat::min`], and is then rounded to `prec` bits
    /// using `Nearest`.
    ///
    /// Both [`BigExponentFloat`]s are taken by value.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::min_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = BigExponentFloat::from(PI).min_prec(BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{x:?}"), "0x2.c");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn min_prec(self, other: Self, prec: u64) -> (Self, Ordering) {
        self.min_prec_round(other, prec, Nearest)
    }

    /// Returns the minimum of two [`BigExponentFloat`]s, rounding the result to the nearest value
    /// of the specified precision. An [`Ordering`] is also returned, indicating whether the rounded
    /// minimum is less than, equal to, or greater than the exact minimum. Whenever this function
    /// returns a NaN it also returns `Equal`.
    ///
    /// The operand is selected as by [`BigExponentFloat::min_val_ref`], and is then rounded to
    /// `prec` bits using `Nearest`.
    ///
    /// The first [`BigExponentFloat`] is taken by value and the second by reference.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::min_prec_round_val_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = BigExponentFloat::from(PI).min_prec_val_ref(&BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{x:?}"), "0x2.c");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn min_prec_val_ref(self, other: &Self, prec: u64) -> (Self, Ordering) {
        self.min_prec_round_val_ref(other, prec, Nearest)
    }

    /// Returns the minimum of two [`BigExponentFloat`]s, rounding the result to the nearest value
    /// of the specified precision. An [`Ordering`] is also returned, indicating whether the rounded
    /// minimum is less than, equal to, or greater than the exact minimum. Whenever this function
    /// returns a NaN it also returns `Equal`.
    ///
    /// The operand is selected as by [`BigExponentFloat::min_ref_val`], and is then rounded to
    /// `prec` bits using `Nearest`.
    ///
    /// The first [`BigExponentFloat`] is taken by reference and the second by value.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::min_prec_round_ref_val`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = BigExponentFloat::from(PI).min_prec_ref_val(BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{x:?}"), "0x2.c");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn min_prec_ref_val(&self, other: Self, prec: u64) -> (Self, Ordering) {
        self.min_prec_round_ref_val(other, prec, Nearest)
    }

    /// Returns the minimum of two [`BigExponentFloat`]s, rounding the result to the nearest value
    /// of the specified precision. An [`Ordering`] is also returned, indicating whether the rounded
    /// minimum is less than, equal to, or greater than the exact minimum. Whenever this function
    /// returns a NaN it also returns `Equal`.
    ///
    /// The operand is selected as by [`BigExponentFloat::min_ref_ref`], and is then rounded to
    /// `prec` bits using `Nearest`.
    ///
    /// Both [`BigExponentFloat`]s are taken by reference.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::min_prec_round_ref_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = BigExponentFloat::from(PI).min_prec_ref_ref(&BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{x:?}"), "0x2.c");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn min_prec_ref_ref(&self, other: &Self, prec: u64) -> (Self, Ordering) {
        self.min_prec_round_ref_ref(other, prec, Nearest)
    }

    /// Returns the minimum of a [`BigExponentFloat`] and a [`Rational`], rounding the result to the
    /// specified precision and with the specified rounding mode. An [`Ordering`] is also returned,
    /// indicating whether the rounded minimum is less than, equal to, or greater than the exact
    /// minimum. Whenever this function returns a NaN it also returns `Equal`.
    ///
    /// The comparison is exact, and only the winning operand is rounded, as by
    /// [`BigExponentFloat::round_to_prec_round`] or [`BigExponentFloat::from_rational_prec_round`].
    /// Converting the [`Rational`] to a [`BigExponentFloat`] first could select the wrong operand,
    /// when the conversion crosses the other operand's value.
    ///
    /// If the [`BigExponentFloat`] is NaN, the [`Rational`] is selected. If the operands are equal,
    /// the [`BigExponentFloat`] is selected.
    ///
    /// The [`BigExponentFloat`] and the [`Rational`] are both taken by value.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::min_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the selected operand cannot be
    /// represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = BigExponentFloat::from(3).min_rational_prec_round(
    ///     Rational::from_signeds(22, 7),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{x:?}"), "0x3.0");
    /// assert_eq!(o, Equal);
    /// ```
    pub fn min_rational_prec_round(
        self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        if selects_rational(&self, &other, false) {
            Self::from_rational_prec_round(other, prec, rm)
        } else {
            self.round_to_prec_round(prec, rm)
        }
    }

    /// Returns the minimum of a [`BigExponentFloat`] and a [`Rational`], rounding the result to the
    /// specified precision and with the specified rounding mode. An [`Ordering`] is also returned,
    /// indicating whether the rounded minimum is less than, equal to, or greater than the exact
    /// minimum. Whenever this function returns a NaN it also returns `Equal`.
    ///
    /// The comparison is exact, and only the winning operand is rounded, as by
    /// [`BigExponentFloat::round_to_prec_round`] or [`BigExponentFloat::from_rational_prec_round`].
    /// Converting the [`Rational`] to a [`BigExponentFloat`] first could select the wrong operand,
    /// when the conversion crosses the other operand's value.
    ///
    /// If the [`BigExponentFloat`] is NaN, the [`Rational`] is selected. If the operands are equal,
    /// the [`BigExponentFloat`] is selected.
    ///
    /// The [`BigExponentFloat`] is taken by value and the [`Rational`] by reference.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::min_rational_prec_val_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the selected operand cannot be
    /// represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = BigExponentFloat::from(3).min_rational_prec_round_val_ref(
    ///     &Rational::from_signeds(22, 7),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{x:?}"), "0x3.0");
    /// assert_eq!(o, Equal);
    /// ```
    pub fn min_rational_prec_round_val_ref(
        self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        if selects_rational(&self, other, false) {
            Self::from_rational_prec_round_ref(other, prec, rm)
        } else {
            self.round_to_prec_round(prec, rm)
        }
    }

    /// Returns the minimum of a [`BigExponentFloat`] and a [`Rational`], rounding the result to the
    /// specified precision and with the specified rounding mode. An [`Ordering`] is also returned,
    /// indicating whether the rounded minimum is less than, equal to, or greater than the exact
    /// minimum. Whenever this function returns a NaN it also returns `Equal`.
    ///
    /// The comparison is exact, and only the winning operand is rounded, as by
    /// [`BigExponentFloat::round_to_prec_round`] or [`BigExponentFloat::from_rational_prec_round`].
    /// Converting the [`Rational`] to a [`BigExponentFloat`] first could select the wrong operand,
    /// when the conversion crosses the other operand's value.
    ///
    /// If the [`BigExponentFloat`] is NaN, the [`Rational`] is selected. If the operands are equal,
    /// the [`BigExponentFloat`] is selected.
    ///
    /// The [`BigExponentFloat`] is taken by reference and the [`Rational`] by value.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::min_rational_prec_ref_val`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the selected operand cannot be
    /// represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = BigExponentFloat::from(3).min_rational_prec_round_ref_val(
    ///     Rational::from_signeds(22, 7),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{x:?}"), "0x3.0");
    /// assert_eq!(o, Equal);
    /// ```
    pub fn min_rational_prec_round_ref_val(
        &self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        if selects_rational(self, &other, false) {
            Self::from_rational_prec_round(other, prec, rm)
        } else {
            self.round_to_prec_round_ref(prec, rm)
        }
    }

    /// Returns the minimum of a [`BigExponentFloat`] and a [`Rational`], rounding the result to the
    /// specified precision and with the specified rounding mode. An [`Ordering`] is also returned,
    /// indicating whether the rounded minimum is less than, equal to, or greater than the exact
    /// minimum. Whenever this function returns a NaN it also returns `Equal`.
    ///
    /// The comparison is exact, and only the winning operand is rounded, as by
    /// [`BigExponentFloat::round_to_prec_round`] or [`BigExponentFloat::from_rational_prec_round`].
    /// Converting the [`Rational`] to a [`BigExponentFloat`] first could select the wrong operand,
    /// when the conversion crosses the other operand's value.
    ///
    /// If the [`BigExponentFloat`] is NaN, the [`Rational`] is selected. If the operands are equal,
    /// the [`BigExponentFloat`] is selected.
    ///
    /// The [`BigExponentFloat`] and the [`Rational`] are both taken by reference.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::min_rational_prec_ref_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the selected operand cannot be
    /// represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = BigExponentFloat::from(3).min_rational_prec_round_ref_ref(
    ///     &Rational::from_signeds(22, 7),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{x:?}"), "0x3.0");
    /// assert_eq!(o, Equal);
    /// ```
    pub fn min_rational_prec_round_ref_ref(
        &self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        if selects_rational(self, other, false) {
            Self::from_rational_prec_round_ref(other, prec, rm)
        } else {
            self.round_to_prec_round_ref(prec, rm)
        }
    }

    /// Returns the minimum of a [`BigExponentFloat`] and a [`Rational`], rounding the result to the
    /// nearest value of the specified precision. An [`Ordering`] is also returned, indicating
    /// whether the rounded minimum is less than, equal to, or greater than the exact minimum.
    /// Whenever this function returns a NaN it also returns `Equal`.
    ///
    /// The comparison is exact, and only the winning operand is rounded, as described in
    /// [`BigExponentFloat::min_rational_prec_round`].
    ///
    /// The [`BigExponentFloat`] and the [`Rational`] are both taken by value.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::min_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
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
    /// let (x, o) = BigExponentFloat::from(3).min_rational_prec(Rational::from_signeds(22, 7), 5);
    /// assert_eq!(format!("{x:?}"), "0x3.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn min_rational_prec(self, other: Rational, prec: u64) -> (Self, Ordering) {
        self.min_rational_prec_round(other, prec, Nearest)
    }

    /// Returns the minimum of a [`BigExponentFloat`] and a [`Rational`], rounding the result to the
    /// nearest value of the specified precision. An [`Ordering`] is also returned, indicating
    /// whether the rounded minimum is less than, equal to, or greater than the exact minimum.
    /// Whenever this function returns a NaN it also returns `Equal`.
    ///
    /// The comparison is exact, and only the winning operand is rounded, as described in
    /// [`BigExponentFloat::min_rational_prec_round_val_ref`].
    ///
    /// The [`BigExponentFloat`] is taken by value and the [`Rational`] by reference.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::min_rational_prec_round_val_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
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
    /// let (x, o) =
    ///     BigExponentFloat::from(3).min_rational_prec_val_ref(&Rational::from_signeds(22, 7), 5);
    /// assert_eq!(format!("{x:?}"), "0x3.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn min_rational_prec_val_ref(self, other: &Rational, prec: u64) -> (Self, Ordering) {
        self.min_rational_prec_round_val_ref(other, prec, Nearest)
    }

    /// Returns the minimum of a [`BigExponentFloat`] and a [`Rational`], rounding the result to the
    /// nearest value of the specified precision. An [`Ordering`] is also returned, indicating
    /// whether the rounded minimum is less than, equal to, or greater than the exact minimum.
    /// Whenever this function returns a NaN it also returns `Equal`.
    ///
    /// The comparison is exact, and only the winning operand is rounded, as described in
    /// [`BigExponentFloat::min_rational_prec_round_ref_val`].
    ///
    /// The [`BigExponentFloat`] is taken by reference and the [`Rational`] by value.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::min_rational_prec_round_ref_val`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
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
    /// let (x, o) =
    ///     BigExponentFloat::from(3).min_rational_prec_ref_val(Rational::from_signeds(22, 7), 5);
    /// assert_eq!(format!("{x:?}"), "0x3.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn min_rational_prec_ref_val(&self, other: Rational, prec: u64) -> (Self, Ordering) {
        self.min_rational_prec_round_ref_val(other, prec, Nearest)
    }

    /// Returns the minimum of a [`BigExponentFloat`] and a [`Rational`], rounding the result to the
    /// nearest value of the specified precision. An [`Ordering`] is also returned, indicating
    /// whether the rounded minimum is less than, equal to, or greater than the exact minimum.
    /// Whenever this function returns a NaN it also returns `Equal`.
    ///
    /// The comparison is exact, and only the winning operand is rounded, as described in
    /// [`BigExponentFloat::min_rational_prec_round_ref_ref`].
    ///
    /// The [`BigExponentFloat`] and the [`Rational`] are both taken by reference.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::min_rational_prec_round_ref_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
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
    /// let (x, o) =
    ///     BigExponentFloat::from(3).min_rational_prec_ref_ref(&Rational::from_signeds(22, 7), 5);
    /// assert_eq!(format!("{x:?}"), "0x3.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn min_rational_prec_ref_ref(&self, other: &Rational, prec: u64) -> (Self, Ordering) {
        self.min_rational_prec_round_ref_ref(other, prec, Nearest)
    }

    /// Returns the maximum of two [`BigExponentFloat`]s. The result is exact.
    ///
    /// If one of the operands is NaN, the other operand is returned; if both are NaN, the result is
    /// NaN. Otherwise, the larger operand is returned. This is the IEEE 754 minNum/maxNum
    /// convention.
    ///
    /// Both [`BigExponentFloat`]s are taken by value.
    ///
    /// If you want to round the result, consider using [`BigExponentFloat::max_prec_round`] or
    /// [`BigExponentFloat::max_prec`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::num::basic::traits::NaN;
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     BigExponentFloat::from(PI).max(BigExponentFloat::from(E)),
    ///     BigExponentFloat::from(PI)
    /// );
    /// assert_eq!(
    ///     BigExponentFloat::NAN.max(BigExponentFloat::from(PI)),
    ///     BigExponentFloat::from(PI)
    /// );
    /// assert!(BigExponentFloat::NAN.max(BigExponentFloat::NAN).is_nan());
    /// ```
    pub fn max(self, other: Self) -> Self {
        if selects_second(&self, &other, true) {
            other
        } else {
            self
        }
    }

    /// Returns the maximum of two [`BigExponentFloat`]s. The result is exact.
    ///
    /// If one of the operands is NaN, the other operand is returned; if both are NaN, the result is
    /// NaN. Otherwise, the larger operand is returned. This is the IEEE 754 minNum/maxNum
    /// convention.
    ///
    /// The first [`BigExponentFloat`] is taken by value and the second by reference.
    ///
    /// If you want to round the result, consider using [`BigExponentFloat::max_prec_round_val_ref`]
    /// or [`BigExponentFloat::max_prec_val_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::num::basic::traits::NaN;
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     BigExponentFloat::from(PI).max_val_ref(&BigExponentFloat::from(E)),
    ///     BigExponentFloat::from(PI)
    /// );
    /// assert_eq!(
    ///     BigExponentFloat::NAN.max_val_ref(&BigExponentFloat::from(PI)),
    ///     BigExponentFloat::from(PI)
    /// );
    /// assert!(
    ///     BigExponentFloat::NAN
    ///         .max_val_ref(&BigExponentFloat::NAN)
    ///         .is_nan()
    /// );
    /// ```
    pub fn max_val_ref(self, other: &Self) -> Self {
        if selects_second(&self, other, true) {
            other.clone()
        } else {
            self
        }
    }

    /// Returns the maximum of two [`BigExponentFloat`]s. The result is exact.
    ///
    /// If one of the operands is NaN, the other operand is returned; if both are NaN, the result is
    /// NaN. Otherwise, the larger operand is returned. This is the IEEE 754 minNum/maxNum
    /// convention.
    ///
    /// The first [`BigExponentFloat`] is taken by reference and the second by value.
    ///
    /// If you want to round the result, consider using [`BigExponentFloat::max_prec_round_ref_val`]
    /// or [`BigExponentFloat::max_prec_ref_val`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::num::basic::traits::NaN;
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     BigExponentFloat::from(PI).max_ref_val(BigExponentFloat::from(E)),
    ///     BigExponentFloat::from(PI)
    /// );
    /// assert_eq!(
    ///     BigExponentFloat::NAN.max_ref_val(BigExponentFloat::from(PI)),
    ///     BigExponentFloat::from(PI)
    /// );
    /// assert!(
    ///     BigExponentFloat::NAN
    ///         .max_ref_val(BigExponentFloat::NAN)
    ///         .is_nan()
    /// );
    /// ```
    pub fn max_ref_val(&self, other: Self) -> Self {
        if selects_second(self, &other, true) {
            other
        } else {
            self.clone()
        }
    }

    /// Returns the maximum of two [`BigExponentFloat`]s. The result is exact.
    ///
    /// If one of the operands is NaN, the other operand is returned; if both are NaN, the result is
    /// NaN. Otherwise, the larger operand is returned. This is the IEEE 754 minNum/maxNum
    /// convention.
    ///
    /// Both [`BigExponentFloat`]s are taken by reference.
    ///
    /// If you want to round the result, consider using [`BigExponentFloat::max_prec_round_ref_ref`]
    /// or [`BigExponentFloat::max_prec_ref_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::num::basic::traits::NaN;
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     BigExponentFloat::from(PI).max_ref_ref(&BigExponentFloat::from(E)),
    ///     BigExponentFloat::from(PI)
    /// );
    /// assert_eq!(
    ///     BigExponentFloat::NAN.max_ref_ref(&BigExponentFloat::from(PI)),
    ///     BigExponentFloat::from(PI)
    /// );
    /// assert!(
    ///     BigExponentFloat::NAN
    ///         .max_ref_ref(&BigExponentFloat::NAN)
    ///         .is_nan()
    /// );
    /// ```
    pub fn max_ref_ref(&self, other: &Self) -> Self {
        if selects_second(self, other, true) {
            other.clone()
        } else {
            self.clone()
        }
    }

    /// Returns the maximum of two [`BigExponentFloat`]s, rounding the result to the specified
    /// precision and with the specified rounding mode. An [`Ordering`] is also returned, indicating
    /// whether the rounded maximum is less than, equal to, or greater than the exact maximum.
    /// Whenever this function returns a NaN it also returns `Equal`.
    ///
    /// The operand is selected as by [`BigExponentFloat::max`], and is then rounded to `prec` bits
    /// using `rm`, as by [`BigExponentFloat::round_to_prec_round`].
    ///
    /// Both [`BigExponentFloat`]s are taken by value.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::max_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the selected operand cannot be
    /// represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = BigExponentFloat::from(PI).max_prec_round(BigExponentFloat::from(E), 5, Floor);
    /// assert_eq!(format!("{x:?}"), "0x3.2");
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) =
    ///     BigExponentFloat::from(PI).max_prec_round(BigExponentFloat::from(E), 5, Ceiling);
    /// assert_eq!(format!("{x:?}"), "0x3.4");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn max_prec_round(self, other: Self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        if selects_second(&self, &other, true) {
            other.round_to_prec_round(prec, rm)
        } else {
            self.round_to_prec_round(prec, rm)
        }
    }

    /// Returns the maximum of two [`BigExponentFloat`]s, rounding the result to the specified
    /// precision and with the specified rounding mode. An [`Ordering`] is also returned, indicating
    /// whether the rounded maximum is less than, equal to, or greater than the exact maximum.
    /// Whenever this function returns a NaN it also returns `Equal`.
    ///
    /// The operand is selected as by [`BigExponentFloat::max_val_ref`], and is then rounded to
    /// `prec` bits using `rm`, as by [`BigExponentFloat::round_to_prec_round`].
    ///
    /// The first [`BigExponentFloat`] is taken by value and the second by reference.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::max_prec_val_ref`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the selected operand cannot be
    /// represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) =
    ///     BigExponentFloat::from(PI).max_prec_round_val_ref(&BigExponentFloat::from(E), 5, Floor);
    /// assert_eq!(format!("{x:?}"), "0x3.2");
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) = BigExponentFloat::from(PI).max_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{x:?}"), "0x3.4");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn max_prec_round_val_ref(
        self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        if selects_second(&self, other, true) {
            other.round_to_prec_round_ref(prec, rm)
        } else {
            self.round_to_prec_round(prec, rm)
        }
    }

    /// Returns the maximum of two [`BigExponentFloat`]s, rounding the result to the specified
    /// precision and with the specified rounding mode. An [`Ordering`] is also returned, indicating
    /// whether the rounded maximum is less than, equal to, or greater than the exact maximum.
    /// Whenever this function returns a NaN it also returns `Equal`.
    ///
    /// The operand is selected as by [`BigExponentFloat::max_ref_val`], and is then rounded to
    /// `prec` bits using `rm`, as by [`BigExponentFloat::round_to_prec_round`].
    ///
    /// The first [`BigExponentFloat`] is taken by reference and the second by value.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::max_prec_ref_val`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the selected operand cannot be
    /// represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) =
    ///     BigExponentFloat::from(PI).max_prec_round_ref_val(BigExponentFloat::from(E), 5, Floor);
    /// assert_eq!(format!("{x:?}"), "0x3.2");
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) = BigExponentFloat::from(PI).max_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{x:?}"), "0x3.4");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn max_prec_round_ref_val(
        &self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        if selects_second(self, &other, true) {
            other.round_to_prec_round(prec, rm)
        } else {
            self.round_to_prec_round_ref(prec, rm)
        }
    }

    /// Returns the maximum of two [`BigExponentFloat`]s, rounding the result to the specified
    /// precision and with the specified rounding mode. An [`Ordering`] is also returned, indicating
    /// whether the rounded maximum is less than, equal to, or greater than the exact maximum.
    /// Whenever this function returns a NaN it also returns `Equal`.
    ///
    /// The operand is selected as by [`BigExponentFloat::max_ref_ref`], and is then rounded to
    /// `prec` bits using `rm`, as by [`BigExponentFloat::round_to_prec_round`].
    ///
    /// Both [`BigExponentFloat`]s are taken by reference.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::max_prec_ref_ref`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the selected operand cannot be
    /// represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) =
    ///     BigExponentFloat::from(PI).max_prec_round_ref_ref(&BigExponentFloat::from(E), 5, Floor);
    /// assert_eq!(format!("{x:?}"), "0x3.2");
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) = BigExponentFloat::from(PI).max_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{x:?}"), "0x3.4");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn max_prec_round_ref_ref(
        &self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        if selects_second(self, other, true) {
            other.round_to_prec_round_ref(prec, rm)
        } else {
            self.round_to_prec_round_ref(prec, rm)
        }
    }

    /// Returns the maximum of two [`BigExponentFloat`]s, rounding the result to the nearest value
    /// of the specified precision. An [`Ordering`] is also returned, indicating whether the rounded
    /// maximum is less than, equal to, or greater than the exact maximum. Whenever this function
    /// returns a NaN it also returns `Equal`.
    ///
    /// The operand is selected as by [`BigExponentFloat::max`], and is then rounded to `prec` bits
    /// using `Nearest`.
    ///
    /// Both [`BigExponentFloat`]s are taken by value.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::max_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = BigExponentFloat::from(PI).max_prec(BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{x:?}"), "0x3.2");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn max_prec(self, other: Self, prec: u64) -> (Self, Ordering) {
        self.max_prec_round(other, prec, Nearest)
    }

    /// Returns the maximum of two [`BigExponentFloat`]s, rounding the result to the nearest value
    /// of the specified precision. An [`Ordering`] is also returned, indicating whether the rounded
    /// maximum is less than, equal to, or greater than the exact maximum. Whenever this function
    /// returns a NaN it also returns `Equal`.
    ///
    /// The operand is selected as by [`BigExponentFloat::max_val_ref`], and is then rounded to
    /// `prec` bits using `Nearest`.
    ///
    /// The first [`BigExponentFloat`] is taken by value and the second by reference.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::max_prec_round_val_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = BigExponentFloat::from(PI).max_prec_val_ref(&BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{x:?}"), "0x3.2");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn max_prec_val_ref(self, other: &Self, prec: u64) -> (Self, Ordering) {
        self.max_prec_round_val_ref(other, prec, Nearest)
    }

    /// Returns the maximum of two [`BigExponentFloat`]s, rounding the result to the nearest value
    /// of the specified precision. An [`Ordering`] is also returned, indicating whether the rounded
    /// maximum is less than, equal to, or greater than the exact maximum. Whenever this function
    /// returns a NaN it also returns `Equal`.
    ///
    /// The operand is selected as by [`BigExponentFloat::max_ref_val`], and is then rounded to
    /// `prec` bits using `Nearest`.
    ///
    /// The first [`BigExponentFloat`] is taken by reference and the second by value.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::max_prec_round_ref_val`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = BigExponentFloat::from(PI).max_prec_ref_val(BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{x:?}"), "0x3.2");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn max_prec_ref_val(&self, other: Self, prec: u64) -> (Self, Ordering) {
        self.max_prec_round_ref_val(other, prec, Nearest)
    }

    /// Returns the maximum of two [`BigExponentFloat`]s, rounding the result to the nearest value
    /// of the specified precision. An [`Ordering`] is also returned, indicating whether the rounded
    /// maximum is less than, equal to, or greater than the exact maximum. Whenever this function
    /// returns a NaN it also returns `Equal`.
    ///
    /// The operand is selected as by [`BigExponentFloat::max_ref_ref`], and is then rounded to
    /// `prec` bits using `Nearest`.
    ///
    /// Both [`BigExponentFloat`]s are taken by reference.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::max_prec_round_ref_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = BigExponentFloat::from(PI).max_prec_ref_ref(&BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{x:?}"), "0x3.2");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn max_prec_ref_ref(&self, other: &Self, prec: u64) -> (Self, Ordering) {
        self.max_prec_round_ref_ref(other, prec, Nearest)
    }

    /// Returns the maximum of a [`BigExponentFloat`] and a [`Rational`], rounding the result to the
    /// specified precision and with the specified rounding mode. An [`Ordering`] is also returned,
    /// indicating whether the rounded maximum is less than, equal to, or greater than the exact
    /// maximum. Whenever this function returns a NaN it also returns `Equal`.
    ///
    /// The comparison is exact, and only the winning operand is rounded, as by
    /// [`BigExponentFloat::round_to_prec_round`] or [`BigExponentFloat::from_rational_prec_round`].
    /// Converting the [`Rational`] to a [`BigExponentFloat`] first could select the wrong operand,
    /// when the conversion crosses the other operand's value.
    ///
    /// If the [`BigExponentFloat`] is NaN, the [`Rational`] is selected. If the operands are equal,
    /// the [`BigExponentFloat`] is selected.
    ///
    /// The [`BigExponentFloat`] and the [`Rational`] are both taken by value.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::max_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the selected operand cannot be
    /// represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = BigExponentFloat::from(3).max_rational_prec_round(
    ///     Rational::from_signeds(22, 7),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{x:?}"), "0x3.2");
    /// assert_eq!(o, Less);
    /// ```
    pub fn max_rational_prec_round(
        self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        if selects_rational(&self, &other, true) {
            Self::from_rational_prec_round(other, prec, rm)
        } else {
            self.round_to_prec_round(prec, rm)
        }
    }

    /// Returns the maximum of a [`BigExponentFloat`] and a [`Rational`], rounding the result to the
    /// specified precision and with the specified rounding mode. An [`Ordering`] is also returned,
    /// indicating whether the rounded maximum is less than, equal to, or greater than the exact
    /// maximum. Whenever this function returns a NaN it also returns `Equal`.
    ///
    /// The comparison is exact, and only the winning operand is rounded, as by
    /// [`BigExponentFloat::round_to_prec_round`] or [`BigExponentFloat::from_rational_prec_round`].
    /// Converting the [`Rational`] to a [`BigExponentFloat`] first could select the wrong operand,
    /// when the conversion crosses the other operand's value.
    ///
    /// If the [`BigExponentFloat`] is NaN, the [`Rational`] is selected. If the operands are equal,
    /// the [`BigExponentFloat`] is selected.
    ///
    /// The [`BigExponentFloat`] is taken by value and the [`Rational`] by reference.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::max_rational_prec_val_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the selected operand cannot be
    /// represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = BigExponentFloat::from(3).max_rational_prec_round_val_ref(
    ///     &Rational::from_signeds(22, 7),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{x:?}"), "0x3.2");
    /// assert_eq!(o, Less);
    /// ```
    pub fn max_rational_prec_round_val_ref(
        self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        if selects_rational(&self, other, true) {
            Self::from_rational_prec_round_ref(other, prec, rm)
        } else {
            self.round_to_prec_round(prec, rm)
        }
    }

    /// Returns the maximum of a [`BigExponentFloat`] and a [`Rational`], rounding the result to the
    /// specified precision and with the specified rounding mode. An [`Ordering`] is also returned,
    /// indicating whether the rounded maximum is less than, equal to, or greater than the exact
    /// maximum. Whenever this function returns a NaN it also returns `Equal`.
    ///
    /// The comparison is exact, and only the winning operand is rounded, as by
    /// [`BigExponentFloat::round_to_prec_round`] or [`BigExponentFloat::from_rational_prec_round`].
    /// Converting the [`Rational`] to a [`BigExponentFloat`] first could select the wrong operand,
    /// when the conversion crosses the other operand's value.
    ///
    /// If the [`BigExponentFloat`] is NaN, the [`Rational`] is selected. If the operands are equal,
    /// the [`BigExponentFloat`] is selected.
    ///
    /// The [`BigExponentFloat`] is taken by reference and the [`Rational`] by value.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::max_rational_prec_ref_val`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the selected operand cannot be
    /// represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = BigExponentFloat::from(3).max_rational_prec_round_ref_val(
    ///     Rational::from_signeds(22, 7),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{x:?}"), "0x3.2");
    /// assert_eq!(o, Less);
    /// ```
    pub fn max_rational_prec_round_ref_val(
        &self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        if selects_rational(self, &other, true) {
            Self::from_rational_prec_round(other, prec, rm)
        } else {
            self.round_to_prec_round_ref(prec, rm)
        }
    }

    /// Returns the maximum of a [`BigExponentFloat`] and a [`Rational`], rounding the result to the
    /// specified precision and with the specified rounding mode. An [`Ordering`] is also returned,
    /// indicating whether the rounded maximum is less than, equal to, or greater than the exact
    /// maximum. Whenever this function returns a NaN it also returns `Equal`.
    ///
    /// The comparison is exact, and only the winning operand is rounded, as by
    /// [`BigExponentFloat::round_to_prec_round`] or [`BigExponentFloat::from_rational_prec_round`].
    /// Converting the [`Rational`] to a [`BigExponentFloat`] first could select the wrong operand,
    /// when the conversion crosses the other operand's value.
    ///
    /// If the [`BigExponentFloat`] is NaN, the [`Rational`] is selected. If the operands are equal,
    /// the [`BigExponentFloat`] is selected.
    ///
    /// The [`BigExponentFloat`] and the [`Rational`] are both taken by reference.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::max_rational_prec_ref_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the selected operand cannot be
    /// represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = BigExponentFloat::from(3).max_rational_prec_round_ref_ref(
    ///     &Rational::from_signeds(22, 7),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{x:?}"), "0x3.2");
    /// assert_eq!(o, Less);
    /// ```
    pub fn max_rational_prec_round_ref_ref(
        &self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        if selects_rational(self, other, true) {
            Self::from_rational_prec_round_ref(other, prec, rm)
        } else {
            self.round_to_prec_round_ref(prec, rm)
        }
    }

    /// Returns the maximum of a [`BigExponentFloat`] and a [`Rational`], rounding the result to the
    /// nearest value of the specified precision. An [`Ordering`] is also returned, indicating
    /// whether the rounded maximum is less than, equal to, or greater than the exact maximum.
    /// Whenever this function returns a NaN it also returns `Equal`.
    ///
    /// The comparison is exact, and only the winning operand is rounded, as described in
    /// [`BigExponentFloat::max_rational_prec_round`].
    ///
    /// The [`BigExponentFloat`] and the [`Rational`] are both taken by value.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::max_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
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
    /// let (x, o) = BigExponentFloat::from(3).max_rational_prec(Rational::from_signeds(22, 7), 5);
    /// assert_eq!(format!("{x:?}"), "0x3.2");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn max_rational_prec(self, other: Rational, prec: u64) -> (Self, Ordering) {
        self.max_rational_prec_round(other, prec, Nearest)
    }

    /// Returns the maximum of a [`BigExponentFloat`] and a [`Rational`], rounding the result to the
    /// nearest value of the specified precision. An [`Ordering`] is also returned, indicating
    /// whether the rounded maximum is less than, equal to, or greater than the exact maximum.
    /// Whenever this function returns a NaN it also returns `Equal`.
    ///
    /// The comparison is exact, and only the winning operand is rounded, as described in
    /// [`BigExponentFloat::max_rational_prec_round_val_ref`].
    ///
    /// The [`BigExponentFloat`] is taken by value and the [`Rational`] by reference.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::max_rational_prec_round_val_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
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
    /// let (x, o) =
    ///     BigExponentFloat::from(3).max_rational_prec_val_ref(&Rational::from_signeds(22, 7), 5);
    /// assert_eq!(format!("{x:?}"), "0x3.2");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn max_rational_prec_val_ref(self, other: &Rational, prec: u64) -> (Self, Ordering) {
        self.max_rational_prec_round_val_ref(other, prec, Nearest)
    }

    /// Returns the maximum of a [`BigExponentFloat`] and a [`Rational`], rounding the result to the
    /// nearest value of the specified precision. An [`Ordering`] is also returned, indicating
    /// whether the rounded maximum is less than, equal to, or greater than the exact maximum.
    /// Whenever this function returns a NaN it also returns `Equal`.
    ///
    /// The comparison is exact, and only the winning operand is rounded, as described in
    /// [`BigExponentFloat::max_rational_prec_round_ref_val`].
    ///
    /// The [`BigExponentFloat`] is taken by reference and the [`Rational`] by value.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::max_rational_prec_round_ref_val`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
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
    /// let (x, o) =
    ///     BigExponentFloat::from(3).max_rational_prec_ref_val(Rational::from_signeds(22, 7), 5);
    /// assert_eq!(format!("{x:?}"), "0x3.2");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn max_rational_prec_ref_val(&self, other: Rational, prec: u64) -> (Self, Ordering) {
        self.max_rational_prec_round_ref_val(other, prec, Nearest)
    }

    /// Returns the maximum of a [`BigExponentFloat`] and a [`Rational`], rounding the result to the
    /// nearest value of the specified precision. An [`Ordering`] is also returned, indicating
    /// whether the rounded maximum is less than, equal to, or greater than the exact maximum.
    /// Whenever this function returns a NaN it also returns `Equal`.
    ///
    /// The comparison is exact, and only the winning operand is rounded, as described in
    /// [`BigExponentFloat::max_rational_prec_round_ref_ref`].
    ///
    /// The [`BigExponentFloat`] and the [`Rational`] are both taken by reference.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::max_rational_prec_round_ref_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
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
    /// let (x, o) =
    ///     BigExponentFloat::from(3).max_rational_prec_ref_ref(&Rational::from_signeds(22, 7), 5);
    /// assert_eq!(format!("{x:?}"), "0x3.2");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn max_rational_prec_ref_ref(&self, other: &Rational, prec: u64) -> (Self, Ordering) {
        self.max_rational_prec_round_ref_ref(other, prec, Nearest)
    }
}
