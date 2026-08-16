// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright © 2001-2025 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::InnerFloat::Zero;
use core::cmp::Ordering::{Equal, Greater, Less};
use core::cmp::{Ordering, max};
use malachite_base::num::basic::traits::Zero as ZeroTrait;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, Nearest};
use malachite_q::Rational;

// Which operand mpfr_min/mpfr_max selects: one NaN gives the other; both NaN gives the first, whose
// rounding produces the NaN result; two zeros are picked by sign (min prefers the negative zero,
// max the positive); otherwise the comparison decides, with ties going to the first operand. This
// is the case analysis of mpfr_min and mpfr_max from minmax.c, MPFR 4.2.2.
enum Choice {
    First,
    Second,
}

fn min_max_choice(x: &Float, y: &Float, is_max: bool) -> Choice {
    match (x.is_nan(), y.is_nan()) {
        (_, true) => Choice::First,
        (true, false) => Choice::Second,
        (false, false) => {
            if x.is_zero() && y.is_zero() {
                // pick by sign: min takes a negative zero, max a positive one
                if x.is_sign_negative() == is_max {
                    Choice::Second
                } else {
                    Choice::First
                }
            } else {
                let le = x.partial_cmp(y) != Some(Ordering::Greater);
                if le == is_max {
                    Choice::Second
                } else {
                    Choice::First
                }
            }
        }
    }
}

impl Float {
    /// Returns the minimum of two [`Float`]s, rounding the result to the specified precision and
    /// with the specified rounding mode. An [`Ordering`] is also returned, indicating whether the
    /// rounded minimum is less than, equal to, or greater than the exact minimum. Whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// If one of the operands is a `NaN`, the other operand is selected; if both are `NaN`s, the
    /// result is `NaN`. If both operands are zeros, a negative zero is selected if either zero is
    /// negative, and a positive zero otherwise. Otherwise, the smaller operand is selected.
    ///
    /// The selected operand is then rounded to `prec` bits using `rm`, as by
    /// [`Float::from_float_prec_round`]; like that function, this function may overflow if the
    /// selected operand has the maximum exponent, and it never underflows.
    ///
    /// Both [`Float`]s are taken by value.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::min_prec`] instead. If you
    /// know that your target precision is the maximum of the precisions of the two inputs, consider
    /// using [`Float::min_round`] instead. If both of these things are true, consider using
    /// [`Float::min`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is [`Exact`] but the selected operand cannot be
    /// represented exactly at a precision of `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (min, o) = Float::from(PI).min_prec_round(Float::from(E), 5, Floor);
    /// assert_eq!(min.to_string(), "2.62");
    /// assert_eq!(o, Less);
    ///
    /// let (min, o) = Float::from(PI).min_prec_round(Float::from(E), 5, Ceiling);
    /// assert_eq!(min.to_string(), "2.75");
    /// assert_eq!(o, Greater);
    ///
    /// let (min, o) = Float::from(PI).min_prec_round(Float::from(E), 20, Nearest);
    /// assert_eq!(min.to_string(), "2.7182808");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn min_prec_round(self, other: Self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        match min_max_choice(&self, &other, false) {
            Choice::First => Self::from_float_prec_round(self, prec, rm),
            Choice::Second => Self::from_float_prec_round(other, prec, rm),
        }
    }

    /// Returns the minimum of two [`Float`]s, rounding the result to the specified precision and
    /// with the specified rounding mode. An [`Ordering`] is also returned, indicating whether the
    /// rounded minimum is less than, equal to, or greater than the exact minimum. Whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// If one of the operands is a `NaN`, the other operand is selected; if both are `NaN`s, the
    /// result is `NaN`. If both operands are zeros, a negative zero is selected if either zero is
    /// negative, and a positive zero otherwise. Otherwise, the smaller operand is selected.
    ///
    /// The selected operand is then rounded to `prec` bits using `rm`, as by
    /// [`Float::from_float_prec_round`]; like that function, this function may overflow if the
    /// selected operand has the maximum exponent, and it never underflows.
    ///
    /// The first [`Float`] is taken by value and the second by reference.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::min_prec`] instead. If you
    /// know that your target precision is the maximum of the precisions of the two inputs, consider
    /// using [`Float::min_round`] instead. If both of these things are true, consider using
    /// [`Float::min`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is [`Exact`] but the selected operand cannot be
    /// represented exactly at a precision of `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (min, o) = Float::from(PI).min_prec_round_val_ref(&Float::from(E), 5, Floor);
    /// assert_eq!(min.to_string(), "2.62");
    /// assert_eq!(o, Less);
    ///
    /// let (min, o) = Float::from(PI).min_prec_round_val_ref(&Float::from(E), 5, Ceiling);
    /// assert_eq!(min.to_string(), "2.75");
    /// assert_eq!(o, Greater);
    ///
    /// let (min, o) = Float::from(PI).min_prec_round_val_ref(&Float::from(E), 20, Nearest);
    /// assert_eq!(min.to_string(), "2.7182808");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn min_prec_round_val_ref(
        self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        match min_max_choice(&self, other, false) {
            Choice::First => Self::from_float_prec_round(self, prec, rm),
            Choice::Second => Self::from_float_prec_round_ref(other, prec, rm),
        }
    }

    /// Returns the minimum of two [`Float`]s, rounding the result to the specified precision and
    /// with the specified rounding mode. An [`Ordering`] is also returned, indicating whether the
    /// rounded minimum is less than, equal to, or greater than the exact minimum. Whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// If one of the operands is a `NaN`, the other operand is selected; if both are `NaN`s, the
    /// result is `NaN`. If both operands are zeros, a negative zero is selected if either zero is
    /// negative, and a positive zero otherwise. Otherwise, the smaller operand is selected.
    ///
    /// The selected operand is then rounded to `prec` bits using `rm`, as by
    /// [`Float::from_float_prec_round`]; like that function, this function may overflow if the
    /// selected operand has the maximum exponent, and it never underflows.
    ///
    /// The first [`Float`] is taken by reference and the second by value.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::min_prec`] instead. If you
    /// know that your target precision is the maximum of the precisions of the two inputs, consider
    /// using [`Float::min_round`] instead. If both of these things are true, consider using
    /// [`Float::min`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is [`Exact`] but the selected operand cannot be
    /// represented exactly at a precision of `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (min, o) = Float::from(PI).min_prec_round_ref_val(Float::from(E), 5, Floor);
    /// assert_eq!(min.to_string(), "2.62");
    /// assert_eq!(o, Less);
    ///
    /// let (min, o) = Float::from(PI).min_prec_round_ref_val(Float::from(E), 5, Ceiling);
    /// assert_eq!(min.to_string(), "2.75");
    /// assert_eq!(o, Greater);
    ///
    /// let (min, o) = Float::from(PI).min_prec_round_ref_val(Float::from(E), 20, Nearest);
    /// assert_eq!(min.to_string(), "2.7182808");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn min_prec_round_ref_val(
        &self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        match min_max_choice(self, &other, false) {
            Choice::First => Self::from_float_prec_round_ref(self, prec, rm),
            Choice::Second => Self::from_float_prec_round(other, prec, rm),
        }
    }

    /// Returns the minimum of two [`Float`]s, rounding the result to the specified precision and
    /// with the specified rounding mode. An [`Ordering`] is also returned, indicating whether the
    /// rounded minimum is less than, equal to, or greater than the exact minimum. Whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// If one of the operands is a `NaN`, the other operand is selected; if both are `NaN`s, the
    /// result is `NaN`. If both operands are zeros, a negative zero is selected if either zero is
    /// negative, and a positive zero otherwise. Otherwise, the smaller operand is selected.
    ///
    /// The selected operand is then rounded to `prec` bits using `rm`, as by
    /// [`Float::from_float_prec_round`]; like that function, this function may overflow if the
    /// selected operand has the maximum exponent, and it never underflows.
    ///
    /// Both [`Float`]s are taken by reference.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::min_prec`] instead. If you
    /// know that your target precision is the maximum of the precisions of the two inputs, consider
    /// using [`Float::min_round`] instead. If both of these things are true, consider using
    /// [`Float::min`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is [`Exact`] but the selected operand cannot be
    /// represented exactly at a precision of `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (min, o) = Float::from(PI).min_prec_round_ref_ref(&Float::from(E), 5, Floor);
    /// assert_eq!(min.to_string(), "2.62");
    /// assert_eq!(o, Less);
    ///
    /// let (min, o) = Float::from(PI).min_prec_round_ref_ref(&Float::from(E), 5, Ceiling);
    /// assert_eq!(min.to_string(), "2.75");
    /// assert_eq!(o, Greater);
    ///
    /// let (min, o) = Float::from(PI).min_prec_round_ref_ref(&Float::from(E), 20, Nearest);
    /// assert_eq!(min.to_string(), "2.7182808");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn min_prec_round_ref_ref(
        &self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        match min_max_choice(self, other, false) {
            Choice::First => Self::from_float_prec_round_ref(self, prec, rm),
            Choice::Second => Self::from_float_prec_round_ref(other, prec, rm),
        }
    }

    /// Returns the minimum of two [`Float`]s, rounding the result to the specified precision and
    /// with the `Nearest` rounding mode. An [`Ordering`] is also returned, indicating whether the
    /// rounded minimum is less than, equal to, or greater than the exact minimum. Whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// If one of the operands is a `NaN`, the other operand is selected; if both are `NaN`s, the
    /// result is `NaN`. If both operands are zeros, a negative zero is selected if either zero is
    /// negative, and a positive zero otherwise. Otherwise, the smaller operand is selected.
    ///
    /// The selected operand is then rounded to `prec` bits using the `Nearest` rounding mode, as by
    /// [`Float::from_float_prec`]; like that function, this function may overflow if the selected
    /// operand has the maximum exponent, and it never underflows.
    ///
    /// Both [`Float`]s are taken by value.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::min_prec_round`] instead. If you know that your target precision is the maximum of
    /// the precisions of the two inputs, consider using [`Float::min`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (min, o) = Float::from(PI).min_prec(Float::from(E), 5);
    /// assert_eq!(min.to_string(), "2.75");
    /// assert_eq!(o, Greater);
    ///
    /// let (min, o) = Float::from(PI).min_prec(Float::from(E), 20);
    /// assert_eq!(min.to_string(), "2.7182808");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn min_prec(self, other: Self, prec: u64) -> (Self, Ordering) {
        match min_max_choice(&self, &other, false) {
            Choice::First => Self::from_float_prec(self, prec),
            Choice::Second => Self::from_float_prec(other, prec),
        }
    }

    /// Returns the minimum of two [`Float`]s, rounding the result to the specified precision and
    /// with the `Nearest` rounding mode. An [`Ordering`] is also returned, indicating whether the
    /// rounded minimum is less than, equal to, or greater than the exact minimum. Whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// If one of the operands is a `NaN`, the other operand is selected; if both are `NaN`s, the
    /// result is `NaN`. If both operands are zeros, a negative zero is selected if either zero is
    /// negative, and a positive zero otherwise. Otherwise, the smaller operand is selected.
    ///
    /// The selected operand is then rounded to `prec` bits using the `Nearest` rounding mode, as by
    /// [`Float::from_float_prec`]; like that function, this function may overflow if the selected
    /// operand has the maximum exponent, and it never underflows.
    ///
    /// The first [`Float`] is taken by value and the second by reference.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::min_prec_round`] instead. If you know that your target precision is the maximum of
    /// the precisions of the two inputs, consider using [`Float::min`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (min, o) = Float::from(PI).min_prec_val_ref(&Float::from(E), 5);
    /// assert_eq!(min.to_string(), "2.75");
    /// assert_eq!(o, Greater);
    ///
    /// let (min, o) = Float::from(PI).min_prec_val_ref(&Float::from(E), 20);
    /// assert_eq!(min.to_string(), "2.7182808");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn min_prec_val_ref(self, other: &Self, prec: u64) -> (Self, Ordering) {
        match min_max_choice(&self, other, false) {
            Choice::First => Self::from_float_prec(self, prec),
            Choice::Second => Self::from_float_prec_ref(other, prec),
        }
    }

    /// Returns the minimum of two [`Float`]s, rounding the result to the specified precision and
    /// with the `Nearest` rounding mode. An [`Ordering`] is also returned, indicating whether the
    /// rounded minimum is less than, equal to, or greater than the exact minimum. Whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// If one of the operands is a `NaN`, the other operand is selected; if both are `NaN`s, the
    /// result is `NaN`. If both operands are zeros, a negative zero is selected if either zero is
    /// negative, and a positive zero otherwise. Otherwise, the smaller operand is selected.
    ///
    /// The selected operand is then rounded to `prec` bits using the `Nearest` rounding mode, as by
    /// [`Float::from_float_prec`]; like that function, this function may overflow if the selected
    /// operand has the maximum exponent, and it never underflows.
    ///
    /// The first [`Float`] is taken by reference and the second by value.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::min_prec_round`] instead. If you know that your target precision is the maximum of
    /// the precisions of the two inputs, consider using [`Float::min`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (min, o) = Float::from(PI).min_prec_ref_val(Float::from(E), 5);
    /// assert_eq!(min.to_string(), "2.75");
    /// assert_eq!(o, Greater);
    ///
    /// let (min, o) = Float::from(PI).min_prec_ref_val(Float::from(E), 20);
    /// assert_eq!(min.to_string(), "2.7182808");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn min_prec_ref_val(&self, other: Self, prec: u64) -> (Self, Ordering) {
        match min_max_choice(self, &other, false) {
            Choice::First => Self::from_float_prec_ref(self, prec),
            Choice::Second => Self::from_float_prec(other, prec),
        }
    }

    /// Returns the minimum of two [`Float`]s, rounding the result to the specified precision and
    /// with the `Nearest` rounding mode. An [`Ordering`] is also returned, indicating whether the
    /// rounded minimum is less than, equal to, or greater than the exact minimum. Whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// If one of the operands is a `NaN`, the other operand is selected; if both are `NaN`s, the
    /// result is `NaN`. If both operands are zeros, a negative zero is selected if either zero is
    /// negative, and a positive zero otherwise. Otherwise, the smaller operand is selected.
    ///
    /// The selected operand is then rounded to `prec` bits using the `Nearest` rounding mode, as by
    /// [`Float::from_float_prec`]; like that function, this function may overflow if the selected
    /// operand has the maximum exponent, and it never underflows.
    ///
    /// Both [`Float`]s are taken by reference.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::min_prec_round`] instead. If you know that your target precision is the maximum of
    /// the precisions of the two inputs, consider using [`Float::min`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (min, o) = Float::from(PI).min_prec_ref_ref(&Float::from(E), 5);
    /// assert_eq!(min.to_string(), "2.75");
    /// assert_eq!(o, Greater);
    ///
    /// let (min, o) = Float::from(PI).min_prec_ref_ref(&Float::from(E), 20);
    /// assert_eq!(min.to_string(), "2.7182808");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn min_prec_ref_ref(&self, other: &Self, prec: u64) -> (Self, Ordering) {
        match min_max_choice(self, other, false) {
            Choice::First => Self::from_float_prec_ref(self, prec),
            Choice::Second => Self::from_float_prec_ref(other, prec),
        }
    }

    /// Returns the minimum of two [`Float`]s, rounding the result to the maximum of the operands'
    /// precisions and with the specified rounding mode. An [`Ordering`] is also returned; since the
    /// target precision is at least as high as the precision of the selected operand, the rounding
    /// is always exact, and the result does not depend on `rm`, and the [`Ordering`] is always
    /// `Equal`.
    ///
    /// If one of the operands is a `NaN`, the other operand is selected; if both are `NaN`s, the
    /// result is `NaN`. If both operands are zeros, a negative zero is selected if either zero is
    /// negative, and a positive zero otherwise. Otherwise, the smaller operand is selected.
    ///
    /// The selected operand is then padded to the target precision. This never rounds, overflows,
    /// or underflows.
    ///
    /// Both [`Float`]s are taken by value.
    ///
    /// If the output has a precision, it is the maximum of the operands' precisions.
    ///
    /// If you want to specify an output precision, consider using [`Float::min_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::min`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::num::basic::traits::{NaN, NegativeZero, Zero};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (min, o) = Float::from(PI).min_round(Float::from(E), Floor);
    /// assert_eq!(min.to_string(), "2.7182818284590451");
    /// assert_eq!(o, Equal);
    ///
    /// let (min, o) = Float::NAN.min_round(Float::from(PI), Floor);
    /// assert_eq!(min.to_string(), "3.1415926535897931");
    /// assert_eq!(o, Equal);
    ///
    /// let (min, o) = Float::ZERO.min_round(Float::NEGATIVE_ZERO, Floor);
    /// assert_eq!(min.to_string(), "-0.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn min_round(self, other: Self, rm: RoundingMode) -> (Self, Ordering) {
        let target_prec = max(self.significant_bits(), other.significant_bits());
        match min_max_choice(&self, &other, false) {
            Choice::First => Self::from_float_prec_round(self, target_prec, rm),
            Choice::Second => Self::from_float_prec_round(other, target_prec, rm),
        }
    }

    /// Returns the minimum of two [`Float`]s, rounding the result to the maximum of the operands'
    /// precisions and with the specified rounding mode. An [`Ordering`] is also returned; since the
    /// target precision is at least as high as the precision of the selected operand, the rounding
    /// is always exact, and the result does not depend on `rm`, and the [`Ordering`] is always
    /// `Equal`.
    ///
    /// If one of the operands is a `NaN`, the other operand is selected; if both are `NaN`s, the
    /// result is `NaN`. If both operands are zeros, a negative zero is selected if either zero is
    /// negative, and a positive zero otherwise. Otherwise, the smaller operand is selected.
    ///
    /// The selected operand is then padded to the target precision. This never rounds, overflows,
    /// or underflows.
    ///
    /// The first [`Float`] is taken by value and the second by reference.
    ///
    /// If the output has a precision, it is the maximum of the operands' precisions.
    ///
    /// If you want to specify an output precision, consider using [`Float::min_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::min`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::num::basic::traits::{NaN, NegativeZero, Zero};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (min, o) = Float::from(PI).min_round_val_ref(&Float::from(E), Floor);
    /// assert_eq!(min.to_string(), "2.7182818284590451");
    /// assert_eq!(o, Equal);
    ///
    /// let (min, o) = Float::NAN.min_round_val_ref(&Float::from(PI), Floor);
    /// assert_eq!(min.to_string(), "3.1415926535897931");
    /// assert_eq!(o, Equal);
    ///
    /// let (min, o) = Float::ZERO.min_round_val_ref(&Float::NEGATIVE_ZERO, Floor);
    /// assert_eq!(min.to_string(), "-0.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn min_round_val_ref(self, other: &Self, rm: RoundingMode) -> (Self, Ordering) {
        let target_prec = max(self.significant_bits(), other.significant_bits());
        match min_max_choice(&self, other, false) {
            Choice::First => Self::from_float_prec_round(self, target_prec, rm),
            Choice::Second => Self::from_float_prec_round_ref(other, target_prec, rm),
        }
    }

    /// Returns the minimum of two [`Float`]s, rounding the result to the maximum of the operands'
    /// precisions and with the specified rounding mode. An [`Ordering`] is also returned; since the
    /// target precision is at least as high as the precision of the selected operand, the rounding
    /// is always exact, and the result does not depend on `rm`, and the [`Ordering`] is always
    /// `Equal`.
    ///
    /// If one of the operands is a `NaN`, the other operand is selected; if both are `NaN`s, the
    /// result is `NaN`. If both operands are zeros, a negative zero is selected if either zero is
    /// negative, and a positive zero otherwise. Otherwise, the smaller operand is selected.
    ///
    /// The selected operand is then padded to the target precision. This never rounds, overflows,
    /// or underflows.
    ///
    /// The first [`Float`] is taken by reference and the second by value.
    ///
    /// If the output has a precision, it is the maximum of the operands' precisions.
    ///
    /// If you want to specify an output precision, consider using [`Float::min_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::min`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::num::basic::traits::{NaN, NegativeZero, Zero};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (min, o) = Float::from(PI).min_round_ref_val(Float::from(E), Floor);
    /// assert_eq!(min.to_string(), "2.7182818284590451");
    /// assert_eq!(o, Equal);
    ///
    /// let (min, o) = Float::NAN.min_round_ref_val(Float::from(PI), Floor);
    /// assert_eq!(min.to_string(), "3.1415926535897931");
    /// assert_eq!(o, Equal);
    ///
    /// let (min, o) = Float::ZERO.min_round_ref_val(Float::NEGATIVE_ZERO, Floor);
    /// assert_eq!(min.to_string(), "-0.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn min_round_ref_val(&self, other: Self, rm: RoundingMode) -> (Self, Ordering) {
        let target_prec = max(self.significant_bits(), other.significant_bits());
        match min_max_choice(self, &other, false) {
            Choice::First => Self::from_float_prec_round_ref(self, target_prec, rm),
            Choice::Second => Self::from_float_prec_round(other, target_prec, rm),
        }
    }

    /// Returns the minimum of two [`Float`]s, rounding the result to the maximum of the operands'
    /// precisions and with the specified rounding mode. An [`Ordering`] is also returned; since the
    /// target precision is at least as high as the precision of the selected operand, the rounding
    /// is always exact, and the result does not depend on `rm`, and the [`Ordering`] is always
    /// `Equal`.
    ///
    /// If one of the operands is a `NaN`, the other operand is selected; if both are `NaN`s, the
    /// result is `NaN`. If both operands are zeros, a negative zero is selected if either zero is
    /// negative, and a positive zero otherwise. Otherwise, the smaller operand is selected.
    ///
    /// The selected operand is then padded to the target precision. This never rounds, overflows,
    /// or underflows.
    ///
    /// Both [`Float`]s are taken by reference.
    ///
    /// If the output has a precision, it is the maximum of the operands' precisions.
    ///
    /// If you want to specify an output precision, consider using [`Float::min_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::min`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::num::basic::traits::{NaN, NegativeZero, Zero};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (min, o) = Float::from(PI).min_round_ref_ref(&Float::from(E), Floor);
    /// assert_eq!(min.to_string(), "2.7182818284590451");
    /// assert_eq!(o, Equal);
    ///
    /// let (min, o) = Float::NAN.min_round_ref_ref(&Float::from(PI), Floor);
    /// assert_eq!(min.to_string(), "3.1415926535897931");
    /// assert_eq!(o, Equal);
    ///
    /// let (min, o) = Float::ZERO.min_round_ref_ref(&Float::NEGATIVE_ZERO, Floor);
    /// assert_eq!(min.to_string(), "-0.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn min_round_ref_ref(&self, other: &Self, rm: RoundingMode) -> (Self, Ordering) {
        let target_prec = max(self.significant_bits(), other.significant_bits());
        match min_max_choice(self, other, false) {
            Choice::First => Self::from_float_prec_round_ref(self, target_prec, rm),
            Choice::Second => Self::from_float_prec_round_ref(other, target_prec, rm),
        }
    }

    /// Returns the minimum of two [`Float`]s, rounding the result to the maximum of the operands'
    /// precisions. An [`Ordering`] is also returned; since the target precision is at least as high
    /// as the precision of the selected operand, the rounding is always exact, and the [`Ordering`]
    /// is always `Equal`.
    ///
    /// If one of the operands is a `NaN`, the other operand is selected; if both are `NaN`s, the
    /// result is `NaN`. If both operands are zeros, a negative zero is selected if either zero is
    /// negative, and a positive zero otherwise. Otherwise, the smaller operand is selected.
    ///
    /// The selected operand is then padded to the target precision. This never rounds, overflows,
    /// or underflows.
    ///
    /// Both [`Float`]s are taken by value.
    ///
    /// If the output has a precision, it is the maximum of the operands' precisions.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using [`Float::min_round`]
    /// instead. If you want to specify an output precision, consider using [`Float::min_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::num::basic::traits::{NaN, NegativeZero, Zero};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (min, o) = Float::from(PI).min(Float::from(E));
    /// assert_eq!(min.to_string(), "2.7182818284590451");
    /// assert_eq!(o, Equal);
    ///
    /// let (min, o) = Float::NAN.min(Float::from(PI));
    /// assert_eq!(min.to_string(), "3.1415926535897931");
    /// assert_eq!(o, Equal);
    ///
    /// let (min, o) = Float::ZERO.min(Float::NEGATIVE_ZERO);
    /// assert_eq!(min.to_string(), "-0.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn min(self, other: Self) -> (Self, Ordering) {
        let target_prec = max(self.significant_bits(), other.significant_bits());
        match min_max_choice(&self, &other, false) {
            Choice::First => Self::from_float_prec(self, target_prec),
            Choice::Second => Self::from_float_prec(other, target_prec),
        }
    }

    /// Returns the minimum of two [`Float`]s, rounding the result to the maximum of the operands'
    /// precisions. An [`Ordering`] is also returned; since the target precision is at least as high
    /// as the precision of the selected operand, the rounding is always exact, and the [`Ordering`]
    /// is always `Equal`.
    ///
    /// If one of the operands is a `NaN`, the other operand is selected; if both are `NaN`s, the
    /// result is `NaN`. If both operands are zeros, a negative zero is selected if either zero is
    /// negative, and a positive zero otherwise. Otherwise, the smaller operand is selected.
    ///
    /// The selected operand is then padded to the target precision. This never rounds, overflows,
    /// or underflows.
    ///
    /// The first [`Float`] is taken by value and the second by reference.
    ///
    /// If the output has a precision, it is the maximum of the operands' precisions.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using [`Float::min_round`]
    /// instead. If you want to specify an output precision, consider using [`Float::min_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::num::basic::traits::{NaN, NegativeZero, Zero};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (min, o) = Float::from(PI).min_val_ref(&Float::from(E));
    /// assert_eq!(min.to_string(), "2.7182818284590451");
    /// assert_eq!(o, Equal);
    ///
    /// let (min, o) = Float::NAN.min_val_ref(&Float::from(PI));
    /// assert_eq!(min.to_string(), "3.1415926535897931");
    /// assert_eq!(o, Equal);
    ///
    /// let (min, o) = Float::ZERO.min_val_ref(&Float::NEGATIVE_ZERO);
    /// assert_eq!(min.to_string(), "-0.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn min_val_ref(self, other: &Self) -> (Self, Ordering) {
        let target_prec = max(self.significant_bits(), other.significant_bits());
        match min_max_choice(&self, other, false) {
            Choice::First => Self::from_float_prec(self, target_prec),
            Choice::Second => Self::from_float_prec_ref(other, target_prec),
        }
    }

    /// Returns the minimum of two [`Float`]s, rounding the result to the maximum of the operands'
    /// precisions. An [`Ordering`] is also returned; since the target precision is at least as high
    /// as the precision of the selected operand, the rounding is always exact, and the [`Ordering`]
    /// is always `Equal`.
    ///
    /// If one of the operands is a `NaN`, the other operand is selected; if both are `NaN`s, the
    /// result is `NaN`. If both operands are zeros, a negative zero is selected if either zero is
    /// negative, and a positive zero otherwise. Otherwise, the smaller operand is selected.
    ///
    /// The selected operand is then padded to the target precision. This never rounds, overflows,
    /// or underflows.
    ///
    /// The first [`Float`] is taken by reference and the second by value.
    ///
    /// If the output has a precision, it is the maximum of the operands' precisions.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using [`Float::min_round`]
    /// instead. If you want to specify an output precision, consider using [`Float::min_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::num::basic::traits::{NaN, NegativeZero, Zero};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (min, o) = Float::from(PI).min_ref_val(Float::from(E));
    /// assert_eq!(min.to_string(), "2.7182818284590451");
    /// assert_eq!(o, Equal);
    ///
    /// let (min, o) = Float::NAN.min_ref_val(Float::from(PI));
    /// assert_eq!(min.to_string(), "3.1415926535897931");
    /// assert_eq!(o, Equal);
    ///
    /// let (min, o) = Float::ZERO.min_ref_val(Float::NEGATIVE_ZERO);
    /// assert_eq!(min.to_string(), "-0.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn min_ref_val(&self, other: Self) -> (Self, Ordering) {
        let target_prec = max(self.significant_bits(), other.significant_bits());
        match min_max_choice(self, &other, false) {
            Choice::First => Self::from_float_prec_ref(self, target_prec),
            Choice::Second => Self::from_float_prec(other, target_prec),
        }
    }

    /// Returns the minimum of two [`Float`]s, rounding the result to the maximum of the operands'
    /// precisions. An [`Ordering`] is also returned; since the target precision is at least as high
    /// as the precision of the selected operand, the rounding is always exact, and the [`Ordering`]
    /// is always `Equal`.
    ///
    /// If one of the operands is a `NaN`, the other operand is selected; if both are `NaN`s, the
    /// result is `NaN`. If both operands are zeros, a negative zero is selected if either zero is
    /// negative, and a positive zero otherwise. Otherwise, the smaller operand is selected.
    ///
    /// The selected operand is then padded to the target precision. This never rounds, overflows,
    /// or underflows.
    ///
    /// Both [`Float`]s are taken by reference.
    ///
    /// If the output has a precision, it is the maximum of the operands' precisions.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using [`Float::min_round`]
    /// instead. If you want to specify an output precision, consider using [`Float::min_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::num::basic::traits::{NaN, NegativeZero, Zero};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (min, o) = Float::from(PI).min_ref_ref(&Float::from(E));
    /// assert_eq!(min.to_string(), "2.7182818284590451");
    /// assert_eq!(o, Equal);
    ///
    /// let (min, o) = Float::NAN.min_ref_ref(&Float::from(PI));
    /// assert_eq!(min.to_string(), "3.1415926535897931");
    /// assert_eq!(o, Equal);
    ///
    /// let (min, o) = Float::ZERO.min_ref_ref(&Float::NEGATIVE_ZERO);
    /// assert_eq!(min.to_string(), "-0.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn min_ref_ref(&self, other: &Self) -> (Self, Ordering) {
        let target_prec = max(self.significant_bits(), other.significant_bits());
        match min_max_choice(self, other, false) {
            Choice::First => Self::from_float_prec_ref(self, target_prec),
            Choice::Second => Self::from_float_prec_ref(other, target_prec),
        }
    }

    /// Returns the maximum of two [`Float`]s, rounding the result to the specified precision and
    /// with the specified rounding mode. An [`Ordering`] is also returned, indicating whether the
    /// rounded maximum is less than, equal to, or greater than the exact maximum. Whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// If one of the operands is a `NaN`, the other operand is selected; if both are `NaN`s, the
    /// result is `NaN`. If both operands are zeros, a positive zero is selected if either zero is
    /// positive, and a negative zero otherwise. Otherwise, the larger operand is selected.
    ///
    /// The selected operand is then rounded to `prec` bits using `rm`, as by
    /// [`Float::from_float_prec_round`]; like that function, this function may overflow if the
    /// selected operand has the maximum exponent, and it never underflows.
    ///
    /// Both [`Float`]s are taken by value.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::max_prec`] instead. If you
    /// know that your target precision is the maximum of the precisions of the two inputs, consider
    /// using [`Float::max_round`] instead. If both of these things are true, consider using
    /// [`Float::max`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is [`Exact`] but the selected operand cannot be
    /// represented exactly at a precision of `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (max, o) = Float::from(PI).max_prec_round(Float::from(E), 5, Floor);
    /// assert_eq!(max.to_string(), "3.12");
    /// assert_eq!(o, Less);
    ///
    /// let (max, o) = Float::from(PI).max_prec_round(Float::from(E), 5, Ceiling);
    /// assert_eq!(max.to_string(), "3.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (max, o) = Float::from(PI).max_prec_round(Float::from(E), 20, Nearest);
    /// assert_eq!(max.to_string(), "3.1415939");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn max_prec_round(self, other: Self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        match min_max_choice(&self, &other, true) {
            Choice::First => Self::from_float_prec_round(self, prec, rm),
            Choice::Second => Self::from_float_prec_round(other, prec, rm),
        }
    }

    /// Returns the maximum of two [`Float`]s, rounding the result to the specified precision and
    /// with the specified rounding mode. An [`Ordering`] is also returned, indicating whether the
    /// rounded maximum is less than, equal to, or greater than the exact maximum. Whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// If one of the operands is a `NaN`, the other operand is selected; if both are `NaN`s, the
    /// result is `NaN`. If both operands are zeros, a positive zero is selected if either zero is
    /// positive, and a negative zero otherwise. Otherwise, the larger operand is selected.
    ///
    /// The selected operand is then rounded to `prec` bits using `rm`, as by
    /// [`Float::from_float_prec_round`]; like that function, this function may overflow if the
    /// selected operand has the maximum exponent, and it never underflows.
    ///
    /// The first [`Float`] is taken by value and the second by reference.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::max_prec`] instead. If you
    /// know that your target precision is the maximum of the precisions of the two inputs, consider
    /// using [`Float::max_round`] instead. If both of these things are true, consider using
    /// [`Float::max`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is [`Exact`] but the selected operand cannot be
    /// represented exactly at a precision of `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (max, o) = Float::from(PI).max_prec_round_val_ref(&Float::from(E), 5, Floor);
    /// assert_eq!(max.to_string(), "3.12");
    /// assert_eq!(o, Less);
    ///
    /// let (max, o) = Float::from(PI).max_prec_round_val_ref(&Float::from(E), 5, Ceiling);
    /// assert_eq!(max.to_string(), "3.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (max, o) = Float::from(PI).max_prec_round_val_ref(&Float::from(E), 20, Nearest);
    /// assert_eq!(max.to_string(), "3.1415939");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn max_prec_round_val_ref(
        self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        match min_max_choice(&self, other, true) {
            Choice::First => Self::from_float_prec_round(self, prec, rm),
            Choice::Second => Self::from_float_prec_round_ref(other, prec, rm),
        }
    }

    /// Returns the maximum of two [`Float`]s, rounding the result to the specified precision and
    /// with the specified rounding mode. An [`Ordering`] is also returned, indicating whether the
    /// rounded maximum is less than, equal to, or greater than the exact maximum. Whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// If one of the operands is a `NaN`, the other operand is selected; if both are `NaN`s, the
    /// result is `NaN`. If both operands are zeros, a positive zero is selected if either zero is
    /// positive, and a negative zero otherwise. Otherwise, the larger operand is selected.
    ///
    /// The selected operand is then rounded to `prec` bits using `rm`, as by
    /// [`Float::from_float_prec_round`]; like that function, this function may overflow if the
    /// selected operand has the maximum exponent, and it never underflows.
    ///
    /// The first [`Float`] is taken by reference and the second by value.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::max_prec`] instead. If you
    /// know that your target precision is the maximum of the precisions of the two inputs, consider
    /// using [`Float::max_round`] instead. If both of these things are true, consider using
    /// [`Float::max`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is [`Exact`] but the selected operand cannot be
    /// represented exactly at a precision of `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (max, o) = Float::from(PI).max_prec_round_ref_val(Float::from(E), 5, Floor);
    /// assert_eq!(max.to_string(), "3.12");
    /// assert_eq!(o, Less);
    ///
    /// let (max, o) = Float::from(PI).max_prec_round_ref_val(Float::from(E), 5, Ceiling);
    /// assert_eq!(max.to_string(), "3.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (max, o) = Float::from(PI).max_prec_round_ref_val(Float::from(E), 20, Nearest);
    /// assert_eq!(max.to_string(), "3.1415939");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn max_prec_round_ref_val(
        &self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        match min_max_choice(self, &other, true) {
            Choice::First => Self::from_float_prec_round_ref(self, prec, rm),
            Choice::Second => Self::from_float_prec_round(other, prec, rm),
        }
    }

    /// Returns the maximum of two [`Float`]s, rounding the result to the specified precision and
    /// with the specified rounding mode. An [`Ordering`] is also returned, indicating whether the
    /// rounded maximum is less than, equal to, or greater than the exact maximum. Whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// If one of the operands is a `NaN`, the other operand is selected; if both are `NaN`s, the
    /// result is `NaN`. If both operands are zeros, a positive zero is selected if either zero is
    /// positive, and a negative zero otherwise. Otherwise, the larger operand is selected.
    ///
    /// The selected operand is then rounded to `prec` bits using `rm`, as by
    /// [`Float::from_float_prec_round`]; like that function, this function may overflow if the
    /// selected operand has the maximum exponent, and it never underflows.
    ///
    /// Both [`Float`]s are taken by reference.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::max_prec`] instead. If you
    /// know that your target precision is the maximum of the precisions of the two inputs, consider
    /// using [`Float::max_round`] instead. If both of these things are true, consider using
    /// [`Float::max`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is [`Exact`] but the selected operand cannot be
    /// represented exactly at a precision of `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (max, o) = Float::from(PI).max_prec_round_ref_ref(&Float::from(E), 5, Floor);
    /// assert_eq!(max.to_string(), "3.12");
    /// assert_eq!(o, Less);
    ///
    /// let (max, o) = Float::from(PI).max_prec_round_ref_ref(&Float::from(E), 5, Ceiling);
    /// assert_eq!(max.to_string(), "3.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (max, o) = Float::from(PI).max_prec_round_ref_ref(&Float::from(E), 20, Nearest);
    /// assert_eq!(max.to_string(), "3.1415939");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn max_prec_round_ref_ref(
        &self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        match min_max_choice(self, other, true) {
            Choice::First => Self::from_float_prec_round_ref(self, prec, rm),
            Choice::Second => Self::from_float_prec_round_ref(other, prec, rm),
        }
    }

    /// Returns the maximum of two [`Float`]s, rounding the result to the specified precision and
    /// with the `Nearest` rounding mode. An [`Ordering`] is also returned, indicating whether the
    /// rounded maximum is less than, equal to, or greater than the exact maximum. Whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// If one of the operands is a `NaN`, the other operand is selected; if both are `NaN`s, the
    /// result is `NaN`. If both operands are zeros, a positive zero is selected if either zero is
    /// positive, and a negative zero otherwise. Otherwise, the larger operand is selected.
    ///
    /// The selected operand is then rounded to `prec` bits using the `Nearest` rounding mode, as by
    /// [`Float::from_float_prec`]; like that function, this function may overflow if the selected
    /// operand has the maximum exponent, and it never underflows.
    ///
    /// Both [`Float`]s are taken by value.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::max_prec_round`] instead. If you know that your target precision is the maximum of
    /// the precisions of the two inputs, consider using [`Float::max`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (max, o) = Float::from(PI).max_prec(Float::from(E), 5);
    /// assert_eq!(max.to_string(), "3.12");
    /// assert_eq!(o, Less);
    ///
    /// let (max, o) = Float::from(PI).max_prec(Float::from(E), 20);
    /// assert_eq!(max.to_string(), "3.1415939");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn max_prec(self, other: Self, prec: u64) -> (Self, Ordering) {
        match min_max_choice(&self, &other, true) {
            Choice::First => Self::from_float_prec(self, prec),
            Choice::Second => Self::from_float_prec(other, prec),
        }
    }

    /// Returns the maximum of two [`Float`]s, rounding the result to the specified precision and
    /// with the `Nearest` rounding mode. An [`Ordering`] is also returned, indicating whether the
    /// rounded maximum is less than, equal to, or greater than the exact maximum. Whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// If one of the operands is a `NaN`, the other operand is selected; if both are `NaN`s, the
    /// result is `NaN`. If both operands are zeros, a positive zero is selected if either zero is
    /// positive, and a negative zero otherwise. Otherwise, the larger operand is selected.
    ///
    /// The selected operand is then rounded to `prec` bits using the `Nearest` rounding mode, as by
    /// [`Float::from_float_prec`]; like that function, this function may overflow if the selected
    /// operand has the maximum exponent, and it never underflows.
    ///
    /// The first [`Float`] is taken by value and the second by reference.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::max_prec_round`] instead. If you know that your target precision is the maximum of
    /// the precisions of the two inputs, consider using [`Float::max`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (max, o) = Float::from(PI).max_prec_val_ref(&Float::from(E), 5);
    /// assert_eq!(max.to_string(), "3.12");
    /// assert_eq!(o, Less);
    ///
    /// let (max, o) = Float::from(PI).max_prec_val_ref(&Float::from(E), 20);
    /// assert_eq!(max.to_string(), "3.1415939");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn max_prec_val_ref(self, other: &Self, prec: u64) -> (Self, Ordering) {
        match min_max_choice(&self, other, true) {
            Choice::First => Self::from_float_prec(self, prec),
            Choice::Second => Self::from_float_prec_ref(other, prec),
        }
    }

    /// Returns the maximum of two [`Float`]s, rounding the result to the specified precision and
    /// with the `Nearest` rounding mode. An [`Ordering`] is also returned, indicating whether the
    /// rounded maximum is less than, equal to, or greater than the exact maximum. Whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// If one of the operands is a `NaN`, the other operand is selected; if both are `NaN`s, the
    /// result is `NaN`. If both operands are zeros, a positive zero is selected if either zero is
    /// positive, and a negative zero otherwise. Otherwise, the larger operand is selected.
    ///
    /// The selected operand is then rounded to `prec` bits using the `Nearest` rounding mode, as by
    /// [`Float::from_float_prec`]; like that function, this function may overflow if the selected
    /// operand has the maximum exponent, and it never underflows.
    ///
    /// The first [`Float`] is taken by reference and the second by value.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::max_prec_round`] instead. If you know that your target precision is the maximum of
    /// the precisions of the two inputs, consider using [`Float::max`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (max, o) = Float::from(PI).max_prec_ref_val(Float::from(E), 5);
    /// assert_eq!(max.to_string(), "3.12");
    /// assert_eq!(o, Less);
    ///
    /// let (max, o) = Float::from(PI).max_prec_ref_val(Float::from(E), 20);
    /// assert_eq!(max.to_string(), "3.1415939");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn max_prec_ref_val(&self, other: Self, prec: u64) -> (Self, Ordering) {
        match min_max_choice(self, &other, true) {
            Choice::First => Self::from_float_prec_ref(self, prec),
            Choice::Second => Self::from_float_prec(other, prec),
        }
    }

    /// Returns the maximum of two [`Float`]s, rounding the result to the specified precision and
    /// with the `Nearest` rounding mode. An [`Ordering`] is also returned, indicating whether the
    /// rounded maximum is less than, equal to, or greater than the exact maximum. Whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// If one of the operands is a `NaN`, the other operand is selected; if both are `NaN`s, the
    /// result is `NaN`. If both operands are zeros, a positive zero is selected if either zero is
    /// positive, and a negative zero otherwise. Otherwise, the larger operand is selected.
    ///
    /// The selected operand is then rounded to `prec` bits using the `Nearest` rounding mode, as by
    /// [`Float::from_float_prec`]; like that function, this function may overflow if the selected
    /// operand has the maximum exponent, and it never underflows.
    ///
    /// Both [`Float`]s are taken by reference.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::max_prec_round`] instead. If you know that your target precision is the maximum of
    /// the precisions of the two inputs, consider using [`Float::max`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (max, o) = Float::from(PI).max_prec_ref_ref(&Float::from(E), 5);
    /// assert_eq!(max.to_string(), "3.12");
    /// assert_eq!(o, Less);
    ///
    /// let (max, o) = Float::from(PI).max_prec_ref_ref(&Float::from(E), 20);
    /// assert_eq!(max.to_string(), "3.1415939");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn max_prec_ref_ref(&self, other: &Self, prec: u64) -> (Self, Ordering) {
        match min_max_choice(self, other, true) {
            Choice::First => Self::from_float_prec_ref(self, prec),
            Choice::Second => Self::from_float_prec_ref(other, prec),
        }
    }

    /// Returns the maximum of two [`Float`]s, rounding the result to the maximum of the operands'
    /// precisions and with the specified rounding mode. An [`Ordering`] is also returned; since the
    /// target precision is at least as high as the precision of the selected operand, the rounding
    /// is always exact, and the result does not depend on `rm`, and the [`Ordering`] is always
    /// `Equal`.
    ///
    /// If one of the operands is a `NaN`, the other operand is selected; if both are `NaN`s, the
    /// result is `NaN`. If both operands are zeros, a positive zero is selected if either zero is
    /// positive, and a negative zero otherwise. Otherwise, the larger operand is selected.
    ///
    /// The selected operand is then padded to the target precision. This never rounds, overflows,
    /// or underflows.
    ///
    /// Both [`Float`]s are taken by value.
    ///
    /// If the output has a precision, it is the maximum of the operands' precisions.
    ///
    /// If you want to specify an output precision, consider using [`Float::max_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::max`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::num::basic::traits::{NaN, NegativeZero, Zero};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (max, o) = Float::from(PI).max_round(Float::from(E), Floor);
    /// assert_eq!(max.to_string(), "3.1415926535897931");
    /// assert_eq!(o, Equal);
    ///
    /// let (max, o) = Float::NAN.max_round(Float::from(PI), Floor);
    /// assert_eq!(max.to_string(), "3.1415926535897931");
    /// assert_eq!(o, Equal);
    ///
    /// let (max, o) = Float::ZERO.max_round(Float::NEGATIVE_ZERO, Floor);
    /// assert_eq!(max.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn max_round(self, other: Self, rm: RoundingMode) -> (Self, Ordering) {
        let target_prec = max(self.significant_bits(), other.significant_bits());
        match min_max_choice(&self, &other, true) {
            Choice::First => Self::from_float_prec_round(self, target_prec, rm),
            Choice::Second => Self::from_float_prec_round(other, target_prec, rm),
        }
    }

    /// Returns the maximum of two [`Float`]s, rounding the result to the maximum of the operands'
    /// precisions and with the specified rounding mode. An [`Ordering`] is also returned; since the
    /// target precision is at least as high as the precision of the selected operand, the rounding
    /// is always exact, and the result does not depend on `rm`, and the [`Ordering`] is always
    /// `Equal`.
    ///
    /// If one of the operands is a `NaN`, the other operand is selected; if both are `NaN`s, the
    /// result is `NaN`. If both operands are zeros, a positive zero is selected if either zero is
    /// positive, and a negative zero otherwise. Otherwise, the larger operand is selected.
    ///
    /// The selected operand is then padded to the target precision. This never rounds, overflows,
    /// or underflows.
    ///
    /// The first [`Float`] is taken by value and the second by reference.
    ///
    /// If the output has a precision, it is the maximum of the operands' precisions.
    ///
    /// If you want to specify an output precision, consider using [`Float::max_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::max`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::num::basic::traits::{NaN, NegativeZero, Zero};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (max, o) = Float::from(PI).max_round_val_ref(&Float::from(E), Floor);
    /// assert_eq!(max.to_string(), "3.1415926535897931");
    /// assert_eq!(o, Equal);
    ///
    /// let (max, o) = Float::NAN.max_round_val_ref(&Float::from(PI), Floor);
    /// assert_eq!(max.to_string(), "3.1415926535897931");
    /// assert_eq!(o, Equal);
    ///
    /// let (max, o) = Float::ZERO.max_round_val_ref(&Float::NEGATIVE_ZERO, Floor);
    /// assert_eq!(max.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn max_round_val_ref(self, other: &Self, rm: RoundingMode) -> (Self, Ordering) {
        let target_prec = max(self.significant_bits(), other.significant_bits());
        match min_max_choice(&self, other, true) {
            Choice::First => Self::from_float_prec_round(self, target_prec, rm),
            Choice::Second => Self::from_float_prec_round_ref(other, target_prec, rm),
        }
    }

    /// Returns the maximum of two [`Float`]s, rounding the result to the maximum of the operands'
    /// precisions and with the specified rounding mode. An [`Ordering`] is also returned; since the
    /// target precision is at least as high as the precision of the selected operand, the rounding
    /// is always exact, and the result does not depend on `rm`, and the [`Ordering`] is always
    /// `Equal`.
    ///
    /// If one of the operands is a `NaN`, the other operand is selected; if both are `NaN`s, the
    /// result is `NaN`. If both operands are zeros, a positive zero is selected if either zero is
    /// positive, and a negative zero otherwise. Otherwise, the larger operand is selected.
    ///
    /// The selected operand is then padded to the target precision. This never rounds, overflows,
    /// or underflows.
    ///
    /// The first [`Float`] is taken by reference and the second by value.
    ///
    /// If the output has a precision, it is the maximum of the operands' precisions.
    ///
    /// If you want to specify an output precision, consider using [`Float::max_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::max`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::num::basic::traits::{NaN, NegativeZero, Zero};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (max, o) = Float::from(PI).max_round_ref_val(Float::from(E), Floor);
    /// assert_eq!(max.to_string(), "3.1415926535897931");
    /// assert_eq!(o, Equal);
    ///
    /// let (max, o) = Float::NAN.max_round_ref_val(Float::from(PI), Floor);
    /// assert_eq!(max.to_string(), "3.1415926535897931");
    /// assert_eq!(o, Equal);
    ///
    /// let (max, o) = Float::ZERO.max_round_ref_val(Float::NEGATIVE_ZERO, Floor);
    /// assert_eq!(max.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn max_round_ref_val(&self, other: Self, rm: RoundingMode) -> (Self, Ordering) {
        let target_prec = max(self.significant_bits(), other.significant_bits());
        match min_max_choice(self, &other, true) {
            Choice::First => Self::from_float_prec_round_ref(self, target_prec, rm),
            Choice::Second => Self::from_float_prec_round(other, target_prec, rm),
        }
    }

    /// Returns the maximum of two [`Float`]s, rounding the result to the maximum of the operands'
    /// precisions and with the specified rounding mode. An [`Ordering`] is also returned; since the
    /// target precision is at least as high as the precision of the selected operand, the rounding
    /// is always exact, and the result does not depend on `rm`, and the [`Ordering`] is always
    /// `Equal`.
    ///
    /// If one of the operands is a `NaN`, the other operand is selected; if both are `NaN`s, the
    /// result is `NaN`. If both operands are zeros, a positive zero is selected if either zero is
    /// positive, and a negative zero otherwise. Otherwise, the larger operand is selected.
    ///
    /// The selected operand is then padded to the target precision. This never rounds, overflows,
    /// or underflows.
    ///
    /// Both [`Float`]s are taken by reference.
    ///
    /// If the output has a precision, it is the maximum of the operands' precisions.
    ///
    /// If you want to specify an output precision, consider using [`Float::max_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::max`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::num::basic::traits::{NaN, NegativeZero, Zero};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (max, o) = Float::from(PI).max_round_ref_ref(&Float::from(E), Floor);
    /// assert_eq!(max.to_string(), "3.1415926535897931");
    /// assert_eq!(o, Equal);
    ///
    /// let (max, o) = Float::NAN.max_round_ref_ref(&Float::from(PI), Floor);
    /// assert_eq!(max.to_string(), "3.1415926535897931");
    /// assert_eq!(o, Equal);
    ///
    /// let (max, o) = Float::ZERO.max_round_ref_ref(&Float::NEGATIVE_ZERO, Floor);
    /// assert_eq!(max.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn max_round_ref_ref(&self, other: &Self, rm: RoundingMode) -> (Self, Ordering) {
        let target_prec = max(self.significant_bits(), other.significant_bits());
        match min_max_choice(self, other, true) {
            Choice::First => Self::from_float_prec_round_ref(self, target_prec, rm),
            Choice::Second => Self::from_float_prec_round_ref(other, target_prec, rm),
        }
    }

    /// Returns the maximum of two [`Float`]s, rounding the result to the maximum of the operands'
    /// precisions. An [`Ordering`] is also returned; since the target precision is at least as high
    /// as the precision of the selected operand, the rounding is always exact, and the [`Ordering`]
    /// is always `Equal`.
    ///
    /// If one of the operands is a `NaN`, the other operand is selected; if both are `NaN`s, the
    /// result is `NaN`. If both operands are zeros, a positive zero is selected if either zero is
    /// positive, and a negative zero otherwise. Otherwise, the larger operand is selected.
    ///
    /// The selected operand is then padded to the target precision. This never rounds, overflows,
    /// or underflows.
    ///
    /// Both [`Float`]s are taken by value.
    ///
    /// If the output has a precision, it is the maximum of the operands' precisions.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using [`Float::max_round`]
    /// instead. If you want to specify an output precision, consider using [`Float::max_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::num::basic::traits::{NaN, NegativeZero, Zero};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (max, o) = Float::from(PI).max(Float::from(E));
    /// assert_eq!(max.to_string(), "3.1415926535897931");
    /// assert_eq!(o, Equal);
    ///
    /// let (max, o) = Float::NAN.max(Float::from(PI));
    /// assert_eq!(max.to_string(), "3.1415926535897931");
    /// assert_eq!(o, Equal);
    ///
    /// let (max, o) = Float::ZERO.max(Float::NEGATIVE_ZERO);
    /// assert_eq!(max.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn max(self, other: Self) -> (Self, Ordering) {
        let target_prec = max(self.significant_bits(), other.significant_bits());
        match min_max_choice(&self, &other, true) {
            Choice::First => Self::from_float_prec(self, target_prec),
            Choice::Second => Self::from_float_prec(other, target_prec),
        }
    }

    /// Returns the maximum of two [`Float`]s, rounding the result to the maximum of the operands'
    /// precisions. An [`Ordering`] is also returned; since the target precision is at least as high
    /// as the precision of the selected operand, the rounding is always exact, and the [`Ordering`]
    /// is always `Equal`.
    ///
    /// If one of the operands is a `NaN`, the other operand is selected; if both are `NaN`s, the
    /// result is `NaN`. If both operands are zeros, a positive zero is selected if either zero is
    /// positive, and a negative zero otherwise. Otherwise, the larger operand is selected.
    ///
    /// The selected operand is then padded to the target precision. This never rounds, overflows,
    /// or underflows.
    ///
    /// The first [`Float`] is taken by value and the second by reference.
    ///
    /// If the output has a precision, it is the maximum of the operands' precisions.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using [`Float::max_round`]
    /// instead. If you want to specify an output precision, consider using [`Float::max_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::num::basic::traits::{NaN, NegativeZero, Zero};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (max, o) = Float::from(PI).max_val_ref(&Float::from(E));
    /// assert_eq!(max.to_string(), "3.1415926535897931");
    /// assert_eq!(o, Equal);
    ///
    /// let (max, o) = Float::NAN.max_val_ref(&Float::from(PI));
    /// assert_eq!(max.to_string(), "3.1415926535897931");
    /// assert_eq!(o, Equal);
    ///
    /// let (max, o) = Float::ZERO.max_val_ref(&Float::NEGATIVE_ZERO);
    /// assert_eq!(max.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn max_val_ref(self, other: &Self) -> (Self, Ordering) {
        let target_prec = max(self.significant_bits(), other.significant_bits());
        match min_max_choice(&self, other, true) {
            Choice::First => Self::from_float_prec(self, target_prec),
            Choice::Second => Self::from_float_prec_ref(other, target_prec),
        }
    }

    /// Returns the maximum of two [`Float`]s, rounding the result to the maximum of the operands'
    /// precisions. An [`Ordering`] is also returned; since the target precision is at least as high
    /// as the precision of the selected operand, the rounding is always exact, and the [`Ordering`]
    /// is always `Equal`.
    ///
    /// If one of the operands is a `NaN`, the other operand is selected; if both are `NaN`s, the
    /// result is `NaN`. If both operands are zeros, a positive zero is selected if either zero is
    /// positive, and a negative zero otherwise. Otherwise, the larger operand is selected.
    ///
    /// The selected operand is then padded to the target precision. This never rounds, overflows,
    /// or underflows.
    ///
    /// The first [`Float`] is taken by reference and the second by value.
    ///
    /// If the output has a precision, it is the maximum of the operands' precisions.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using [`Float::max_round`]
    /// instead. If you want to specify an output precision, consider using [`Float::max_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::num::basic::traits::{NaN, NegativeZero, Zero};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (max, o) = Float::from(PI).max_ref_val(Float::from(E));
    /// assert_eq!(max.to_string(), "3.1415926535897931");
    /// assert_eq!(o, Equal);
    ///
    /// let (max, o) = Float::NAN.max_ref_val(Float::from(PI));
    /// assert_eq!(max.to_string(), "3.1415926535897931");
    /// assert_eq!(o, Equal);
    ///
    /// let (max, o) = Float::ZERO.max_ref_val(Float::NEGATIVE_ZERO);
    /// assert_eq!(max.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn max_ref_val(&self, other: Self) -> (Self, Ordering) {
        let target_prec = max(self.significant_bits(), other.significant_bits());
        match min_max_choice(self, &other, true) {
            Choice::First => Self::from_float_prec_ref(self, target_prec),
            Choice::Second => Self::from_float_prec(other, target_prec),
        }
    }

    /// Returns the maximum of two [`Float`]s, rounding the result to the maximum of the operands'
    /// precisions. An [`Ordering`] is also returned; since the target precision is at least as high
    /// as the precision of the selected operand, the rounding is always exact, and the [`Ordering`]
    /// is always `Equal`.
    ///
    /// If one of the operands is a `NaN`, the other operand is selected; if both are `NaN`s, the
    /// result is `NaN`. If both operands are zeros, a positive zero is selected if either zero is
    /// positive, and a negative zero otherwise. Otherwise, the larger operand is selected.
    ///
    /// The selected operand is then padded to the target precision. This never rounds, overflows,
    /// or underflows.
    ///
    /// Both [`Float`]s are taken by reference.
    ///
    /// If the output has a precision, it is the maximum of the operands' precisions.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using [`Float::max_round`]
    /// instead. If you want to specify an output precision, consider using [`Float::max_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::num::basic::traits::{NaN, NegativeZero, Zero};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (max, o) = Float::from(PI).max_ref_ref(&Float::from(E));
    /// assert_eq!(max.to_string(), "3.1415926535897931");
    /// assert_eq!(o, Equal);
    ///
    /// let (max, o) = Float::NAN.max_ref_ref(&Float::from(PI));
    /// assert_eq!(max.to_string(), "3.1415926535897931");
    /// assert_eq!(o, Equal);
    ///
    /// let (max, o) = Float::ZERO.max_ref_ref(&Float::NEGATIVE_ZERO);
    /// assert_eq!(max.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn max_ref_ref(&self, other: &Self) -> (Self, Ordering) {
        let target_prec = max(self.significant_bits(), other.significant_bits());
        match min_max_choice(self, other, true) {
            Choice::First => Self::from_float_prec_ref(self, target_prec),
            Choice::Second => Self::from_float_prec_ref(other, target_prec),
        }
    }
}

// The comparison of a Float with a Rational is exact, so these mixed functions choose the true
// winner and round only it; converting the Rational to a Float first could select the wrong operand
// when the conversion crosses the other operand's value. A NaN Float yields the other operand, as
// in the Float-Float functions. On a tie, min returns the Float (preserving a negative zero) and
// max prefers the positive zero, matching the zero-sign preferences of mpfr_min and mpfr_max.
fn min_rational_helper(x: &Float, y: &Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    match x.partial_cmp(y) {
        None | Some(Greater) => Float::from_rational_prec_round_ref(y, prec, rm),
        _ => Float::from_float_prec_round_ref(x, prec, rm),
    }
}

fn max_rational_helper(x: &Float, y: &Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    match x.partial_cmp(y) {
        None | Some(Less) => Float::from_rational_prec_round_ref(y, prec, rm),
        Some(Equal) if matches!(x, float_negative_zero!()) => (Float::ZERO, Equal),
        _ => Float::from_float_prec_round_ref(x, prec, rm),
    }
}

impl Float {
    /// Computes the smaller of a [`Float`] and a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] and the [`Rational`] are both
    /// taken by value. An [`Ordering`] is also returned, indicating whether the result is less
    /// than, equal to, or greater than the exact smaller value. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The comparison is exact, and only the winning operand is rounded. Converting the
    /// [`Rational`] to a [`Float`] first could select the wrong operand, when the conversion
    /// crosses the other operand's value.
    ///
    /// Special cases:
    /// - If the [`Float`] is `NaN`, the [`Rational`] operand is returned (rounded), as with the
    ///   [`Float`]-[`Float`] functions.
    /// - If the operands are equal, the [`Float`] operand is returned (rounded), so a negative zero
    ///   is preserved.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the winning operand is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let (r, o) =
    ///     Float::from(3u32).min_rational_prec_round(Rational::from_signeds(22, 7), 5, Floor);
    /// assert_eq!(r.to_string(), "3.00");
    /// assert_eq!(o, Equal);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn min_rational_prec_round(
        self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        min_rational_helper(&self, &other, prec, rm)
    }

    /// Computes the smaller of a [`Float`] and a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is taken by value and the
    /// [`Rational`] by reference. An [`Ordering`] is also returned, indicating whether the result
    /// is less than, equal to, or greater than the exact smaller value. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The comparison is exact, and only the winning operand is rounded. Converting the
    /// [`Rational`] to a [`Float`] first could select the wrong operand, when the conversion
    /// crosses the other operand's value.
    ///
    /// Special cases:
    /// - If the [`Float`] is `NaN`, the [`Rational`] operand is returned (rounded), as with the
    ///   [`Float`]-[`Float`] functions.
    /// - If the operands are equal, the [`Float`] operand is returned (rounded), so a negative zero
    ///   is preserved.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the winning operand is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(3u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o) = x.min_rational_prec_round_val_ref(&y, 5, Floor);
    /// assert_eq!(r.to_string(), "3.00");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn min_rational_prec_round_val_ref(
        self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        min_rational_helper(&self, other, prec, rm)
    }

    /// Computes the smaller of a [`Float`] and a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is taken by reference and the
    /// [`Rational`] by value. An [`Ordering`] is also returned, indicating whether the result is
    /// less than, equal to, or greater than the exact smaller value. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The comparison is exact, and only the winning operand is rounded. Converting the
    /// [`Rational`] to a [`Float`] first could select the wrong operand, when the conversion
    /// crosses the other operand's value.
    ///
    /// Special cases:
    /// - If the [`Float`] is `NaN`, the [`Rational`] operand is returned (rounded), as with the
    ///   [`Float`]-[`Float`] functions.
    /// - If the operands are equal, the [`Float`] operand is returned (rounded), so a negative zero
    ///   is preserved.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the winning operand is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(3u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o) = x.min_rational_prec_round_ref_val(y, 5, Floor);
    /// assert_eq!(r.to_string(), "3.00");
    /// assert_eq!(o, Equal);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn min_rational_prec_round_ref_val(
        &self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        min_rational_helper(self, &other, prec, rm)
    }

    /// Computes the smaller of a [`Float`] and a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] and the [`Rational`] are both
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the result is less
    /// than, equal to, or greater than the exact smaller value. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The comparison is exact, and only the winning operand is rounded. Converting the
    /// [`Rational`] to a [`Float`] first could select the wrong operand, when the conversion
    /// crosses the other operand's value.
    ///
    /// Special cases:
    /// - If the [`Float`] is `NaN`, the [`Rational`] operand is returned (rounded), as with the
    ///   [`Float`]-[`Float`] functions.
    /// - If the operands are equal, the [`Float`] operand is returned (rounded), so a negative zero
    ///   is preserved.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the winning operand is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(3u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o) = x.min_rational_prec_round_ref_ref(&y, 5, Floor);
    /// assert_eq!(r.to_string(), "3.00");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn min_rational_prec_round_ref_ref(
        &self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        min_rational_helper(self, other, prec, rm)
    }

    /// Computes the smaller of a [`Float`] and a [`Rational`], rounding the result to the nearest
    /// value of the specified precision. The [`Float`] and the [`Rational`] are both taken by
    /// value. An [`Ordering`] is also returned, indicating whether the result is less than, equal
    /// to, or greater than the exact smaller value. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The comparison is exact, and only the winning operand is rounded. Converting the
    /// [`Rational`] to a [`Float`] first could select the wrong operand, when the conversion
    /// crosses the other operand's value.
    ///
    /// Special cases:
    /// - If the [`Float`] is `NaN`, the [`Rational`] operand is returned (rounded), as with the
    ///   [`Float`]-[`Float`] functions.
    /// - If the operands are equal, the [`Float`] operand is returned (rounded), so a negative zero
    ///   is preserved.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let (r, o) = Float::from(3u32).min_rational_prec(Rational::from_signeds(22, 7), 5);
    /// assert_eq!(r.to_string(), "3.00");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn min_rational_prec(self, other: Rational, prec: u64) -> (Self, Ordering) {
        self.min_rational_prec_round(other, prec, Nearest)
    }

    /// Computes the smaller of a [`Float`] and a [`Rational`], rounding the result to the nearest
    /// value of the specified precision. The [`Float`] is taken by value and the [`Rational`] by
    /// reference. An [`Ordering`] is also returned, indicating whether the result is less than,
    /// equal to, or greater than the exact smaller value. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The comparison is exact, and only the winning operand is rounded. Converting the
    /// [`Rational`] to a [`Float`] first could select the wrong operand, when the conversion
    /// crosses the other operand's value.
    ///
    /// Special cases:
    /// - If the [`Float`] is `NaN`, the [`Rational`] operand is returned (rounded), as with the
    ///   [`Float`]-[`Float`] functions.
    /// - If the operands are equal, the [`Float`] operand is returned (rounded), so a negative zero
    ///   is preserved.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let (r, o) = Float::from(3u32).min_rational_prec_val_ref(&Rational::from_signeds(22, 7), 5);
    /// assert_eq!(r.to_string(), "3.00");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn min_rational_prec_val_ref(self, other: &Rational, prec: u64) -> (Self, Ordering) {
        self.min_rational_prec_round_val_ref(other, prec, Nearest)
    }

    /// Computes the smaller of a [`Float`] and a [`Rational`], rounding the result to the nearest
    /// value of the specified precision. The [`Float`] is taken by reference and the [`Rational`]
    /// by value. An [`Ordering`] is also returned, indicating whether the result is less than,
    /// equal to, or greater than the exact smaller value. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The comparison is exact, and only the winning operand is rounded. Converting the
    /// [`Rational`] to a [`Float`] first could select the wrong operand, when the conversion
    /// crosses the other operand's value.
    ///
    /// Special cases:
    /// - If the [`Float`] is `NaN`, the [`Rational`] operand is returned (rounded), as with the
    ///   [`Float`]-[`Float`] functions.
    /// - If the operands are equal, the [`Float`] operand is returned (rounded), so a negative zero
    ///   is preserved.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let (r, o) = Float::from(3u32).min_rational_prec_ref_val(Rational::from_signeds(22, 7), 5);
    /// assert_eq!(r.to_string(), "3.00");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn min_rational_prec_ref_val(&self, other: Rational, prec: u64) -> (Self, Ordering) {
        self.min_rational_prec_round_ref_val(other, prec, Nearest)
    }

    /// Computes the smaller of a [`Float`] and a [`Rational`], rounding the result to the nearest
    /// value of the specified precision. The [`Float`] and the [`Rational`] are both taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the result is less than,
    /// equal to, or greater than the exact smaller value. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The comparison is exact, and only the winning operand is rounded. Converting the
    /// [`Rational`] to a [`Float`] first could select the wrong operand, when the conversion
    /// crosses the other operand's value.
    ///
    /// Special cases:
    /// - If the [`Float`] is `NaN`, the [`Rational`] operand is returned (rounded), as with the
    ///   [`Float`]-[`Float`] functions.
    /// - If the operands are equal, the [`Float`] operand is returned (rounded), so a negative zero
    ///   is preserved.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let (r, o) = Float::from(3u32).min_rational_prec_ref_ref(&Rational::from_signeds(22, 7), 5);
    /// assert_eq!(r.to_string(), "3.00");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn min_rational_prec_ref_ref(&self, other: &Rational, prec: u64) -> (Self, Ordering) {
        self.min_rational_prec_round_ref_ref(other, prec, Nearest)
    }

    /// Computes the smaller of a [`Float`] and a [`Rational`], rounding the result to the
    /// [`Float`]'s precision, with the specified rounding mode. The [`Float`] and the [`Rational`]
    /// are both taken by value. An [`Ordering`] is also returned, indicating whether the result is
    /// less than, equal to, or greater than the exact smaller value. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The comparison is exact, and only the winning operand is rounded. Converting the
    /// [`Rational`] to a [`Float`] first could select the wrong operand, when the conversion
    /// crosses the other operand's value.
    ///
    /// Special cases:
    /// - If the [`Float`] is `NaN`, the [`Rational`] operand is returned (rounded), as with the
    ///   [`Float`]-[`Float`] functions.
    /// - If the operands are equal, the [`Float`] operand is returned (rounded), so a negative zero
    ///   is preserved.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the winning operand is not exactly representable with the
    /// output precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let (r, o) = Float::from(3u32).min_rational_round(Rational::from_signeds(22, 7), Floor);
    /// assert_eq!(r.to_string(), "3.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn min_rational_round(self, other: Rational, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.min_rational_prec_round(other, prec, rm)
    }

    /// Computes the smaller of a [`Float`] and a [`Rational`], rounding the result to the
    /// [`Float`]'s precision, with the specified rounding mode. The [`Float`] is taken by value and
    /// the [`Rational`] by reference. An [`Ordering`] is also returned, indicating whether the
    /// result is less than, equal to, or greater than the exact smaller value. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The comparison is exact, and only the winning operand is rounded. Converting the
    /// [`Rational`] to a [`Float`] first could select the wrong operand, when the conversion
    /// crosses the other operand's value.
    ///
    /// Special cases:
    /// - If the [`Float`] is `NaN`, the [`Rational`] operand is returned (rounded), as with the
    ///   [`Float`]-[`Float`] functions.
    /// - If the operands are equal, the [`Float`] operand is returned (rounded), so a negative zero
    ///   is preserved.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the winning operand is not exactly representable with the
    /// output precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let (r, o) =
    ///     Float::from(3u32).min_rational_round_val_ref(&Rational::from_signeds(22, 7), Floor);
    /// assert_eq!(r.to_string(), "3.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn min_rational_round_val_ref(
        self,
        other: &Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.min_rational_prec_round_val_ref(other, prec, rm)
    }

    /// Computes the smaller of a [`Float`] and a [`Rational`], rounding the result to the
    /// [`Float`]'s precision, with the specified rounding mode. The [`Float`] is taken by reference
    /// and the [`Rational`] by value. An [`Ordering`] is also returned, indicating whether the
    /// result is less than, equal to, or greater than the exact smaller value. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The comparison is exact, and only the winning operand is rounded. Converting the
    /// [`Rational`] to a [`Float`] first could select the wrong operand, when the conversion
    /// crosses the other operand's value.
    ///
    /// Special cases:
    /// - If the [`Float`] is `NaN`, the [`Rational`] operand is returned (rounded), as with the
    ///   [`Float`]-[`Float`] functions.
    /// - If the operands are equal, the [`Float`] operand is returned (rounded), so a negative zero
    ///   is preserved.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the winning operand is not exactly representable with the
    /// output precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let (r, o) =
    ///     Float::from(3u32).min_rational_round_ref_val(Rational::from_signeds(22, 7), Floor);
    /// assert_eq!(r.to_string(), "3.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn min_rational_round_ref_val(
        &self,
        other: Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.min_rational_prec_round_ref_val(other, prec, rm)
    }

    /// Computes the smaller of a [`Float`] and a [`Rational`], rounding the result to the
    /// [`Float`]'s precision, with the specified rounding mode. The [`Float`] and the [`Rational`]
    /// are both taken by reference. An [`Ordering`] is also returned, indicating whether the result
    /// is less than, equal to, or greater than the exact smaller value. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The comparison is exact, and only the winning operand is rounded. Converting the
    /// [`Rational`] to a [`Float`] first could select the wrong operand, when the conversion
    /// crosses the other operand's value.
    ///
    /// Special cases:
    /// - If the [`Float`] is `NaN`, the [`Rational`] operand is returned (rounded), as with the
    ///   [`Float`]-[`Float`] functions.
    /// - If the operands are equal, the [`Float`] operand is returned (rounded), so a negative zero
    ///   is preserved.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the winning operand is not exactly representable with the
    /// output precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let (r, o) =
    ///     Float::from(3u32).min_rational_round_ref_ref(&Rational::from_signeds(22, 7), Floor);
    /// assert_eq!(r.to_string(), "3.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn min_rational_round_ref_ref(
        &self,
        other: &Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.min_rational_prec_round_ref_ref(other, prec, rm)
    }

    /// Computes the smaller of a [`Float`] and a [`Rational`], rounding the result to the nearest
    /// value of the [`Float`]'s precision. The [`Float`] and the [`Rational`] are both taken by
    /// value. An [`Ordering`] is also returned, indicating whether the result is less than, equal
    /// to, or greater than the exact smaller value. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The comparison is exact, and only the winning operand is rounded. Converting the
    /// [`Rational`] to a [`Float`] first could select the wrong operand, when the conversion
    /// crosses the other operand's value.
    ///
    /// Special cases:
    /// - If the [`Float`] is `NaN`, the [`Rational`] operand is returned (rounded), as with the
    ///   [`Float`]-[`Float`] functions.
    /// - If the operands are equal, the [`Float`] operand is returned (rounded), so a negative zero
    ///   is preserved.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let (r, o) = Float::from(3u32).min_rational(Rational::from_signeds(22, 7));
    /// assert_eq!(r.to_string(), "3.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn min_rational(self, other: Rational) -> (Self, Ordering) {
        self.min_rational_round(other, Nearest)
    }

    /// Computes the smaller of a [`Float`] and a [`Rational`], rounding the result to the nearest
    /// value of the [`Float`]'s precision. The [`Float`] is taken by value and the [`Rational`] by
    /// reference. An [`Ordering`] is also returned, indicating whether the result is less than,
    /// equal to, or greater than the exact smaller value. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The comparison is exact, and only the winning operand is rounded. Converting the
    /// [`Rational`] to a [`Float`] first could select the wrong operand, when the conversion
    /// crosses the other operand's value.
    ///
    /// Special cases:
    /// - If the [`Float`] is `NaN`, the [`Rational`] operand is returned (rounded), as with the
    ///   [`Float`]-[`Float`] functions.
    /// - If the operands are equal, the [`Float`] operand is returned (rounded), so a negative zero
    ///   is preserved.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let (r, o) = Float::from(3u32).min_rational_val_ref(&Rational::from_signeds(22, 7));
    /// assert_eq!(r.to_string(), "3.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn min_rational_val_ref(self, other: &Rational) -> (Self, Ordering) {
        self.min_rational_round_val_ref(other, Nearest)
    }

    /// Computes the smaller of a [`Float`] and a [`Rational`], rounding the result to the nearest
    /// value of the [`Float`]'s precision. The [`Float`] is taken by reference and the [`Rational`]
    /// by value. An [`Ordering`] is also returned, indicating whether the result is less than,
    /// equal to, or greater than the exact smaller value. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The comparison is exact, and only the winning operand is rounded. Converting the
    /// [`Rational`] to a [`Float`] first could select the wrong operand, when the conversion
    /// crosses the other operand's value.
    ///
    /// Special cases:
    /// - If the [`Float`] is `NaN`, the [`Rational`] operand is returned (rounded), as with the
    ///   [`Float`]-[`Float`] functions.
    /// - If the operands are equal, the [`Float`] operand is returned (rounded), so a negative zero
    ///   is preserved.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let (r, o) = Float::from(3u32).min_rational_ref_val(Rational::from_signeds(22, 7));
    /// assert_eq!(r.to_string(), "3.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn min_rational_ref_val(&self, other: Rational) -> (Self, Ordering) {
        self.min_rational_round_ref_val(other, Nearest)
    }

    /// Computes the smaller of a [`Float`] and a [`Rational`], rounding the result to the nearest
    /// value of the [`Float`]'s precision. The [`Float`] and the [`Rational`] are both taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the result is less than,
    /// equal to, or greater than the exact smaller value. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The comparison is exact, and only the winning operand is rounded. Converting the
    /// [`Rational`] to a [`Float`] first could select the wrong operand, when the conversion
    /// crosses the other operand's value.
    ///
    /// Special cases:
    /// - If the [`Float`] is `NaN`, the [`Rational`] operand is returned (rounded), as with the
    ///   [`Float`]-[`Float`] functions.
    /// - If the operands are equal, the [`Float`] operand is returned (rounded), so a negative zero
    ///   is preserved.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let (r, o) = Float::from(3u32).min_rational_ref_ref(&Rational::from_signeds(22, 7));
    /// assert_eq!(r.to_string(), "3.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn min_rational_ref_ref(&self, other: &Rational) -> (Self, Ordering) {
        self.min_rational_round_ref_ref(other, Nearest)
    }

    /// Computes the larger of a [`Float`] and a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] and the [`Rational`] are both
    /// taken by value. An [`Ordering`] is also returned, indicating whether the result is less
    /// than, equal to, or greater than the exact larger value. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The comparison is exact, and only the winning operand is rounded. Converting the
    /// [`Rational`] to a [`Float`] first could select the wrong operand, when the conversion
    /// crosses the other operand's value.
    ///
    /// Special cases:
    /// - If the [`Float`] is `NaN`, the [`Rational`] operand is returned (rounded), as with the
    ///   [`Float`]-[`Float`] functions.
    /// - If the operands are equal, the [`Float`] operand is returned (rounded), except that a
    ///   negative zero loses the tie against the (unsigned, treated as positive) zero [`Rational`]:
    ///   the result is then a positive zero, matching the positive-zero preference of `mpfr_max`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the winning operand is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let (r, o) =
    ///     Float::from(3u32).max_rational_prec_round(Rational::from_signeds(22, 7), 5, Floor);
    /// assert_eq!(r.to_string(), "3.12");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) =
    ///     Float::from(3u32).max_rational_prec_round(Rational::from_signeds(22, 7), 5, Ceiling);
    /// assert_eq!(r.to_string(), "3.25");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn max_rational_prec_round(
        self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        max_rational_helper(&self, &other, prec, rm)
    }

    /// Computes the larger of a [`Float`] and a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is taken by value and the
    /// [`Rational`] by reference. An [`Ordering`] is also returned, indicating whether the result
    /// is less than, equal to, or greater than the exact larger value. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The comparison is exact, and only the winning operand is rounded. Converting the
    /// [`Rational`] to a [`Float`] first could select the wrong operand, when the conversion
    /// crosses the other operand's value.
    ///
    /// Special cases:
    /// - If the [`Float`] is `NaN`, the [`Rational`] operand is returned (rounded), as with the
    ///   [`Float`]-[`Float`] functions.
    /// - If the operands are equal, the [`Float`] operand is returned (rounded), except that a
    ///   negative zero loses the tie against the (unsigned, treated as positive) zero [`Rational`]:
    ///   the result is then a positive zero, matching the positive-zero preference of `mpfr_max`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the winning operand is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(3u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o) = x.max_rational_prec_round_val_ref(&y, 5, Floor);
    /// assert_eq!(r.to_string(), "3.12");
    /// assert_eq!(o, Less);
    ///
    /// let x = Float::from(3u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o) = x.max_rational_prec_round_val_ref(&y, 5, Ceiling);
    /// assert_eq!(r.to_string(), "3.25");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn max_rational_prec_round_val_ref(
        self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        max_rational_helper(&self, other, prec, rm)
    }

    /// Computes the larger of a [`Float`] and a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is taken by reference and the
    /// [`Rational`] by value. An [`Ordering`] is also returned, indicating whether the result is
    /// less than, equal to, or greater than the exact larger value. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The comparison is exact, and only the winning operand is rounded. Converting the
    /// [`Rational`] to a [`Float`] first could select the wrong operand, when the conversion
    /// crosses the other operand's value.
    ///
    /// Special cases:
    /// - If the [`Float`] is `NaN`, the [`Rational`] operand is returned (rounded), as with the
    ///   [`Float`]-[`Float`] functions.
    /// - If the operands are equal, the [`Float`] operand is returned (rounded), except that a
    ///   negative zero loses the tie against the (unsigned, treated as positive) zero [`Rational`]:
    ///   the result is then a positive zero, matching the positive-zero preference of `mpfr_max`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the winning operand is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(3u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o) = x.max_rational_prec_round_ref_val(y, 5, Floor);
    /// assert_eq!(r.to_string(), "3.12");
    /// assert_eq!(o, Less);
    ///
    /// let x = Float::from(3u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o) = x.max_rational_prec_round_ref_val(y, 5, Ceiling);
    /// assert_eq!(r.to_string(), "3.25");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn max_rational_prec_round_ref_val(
        &self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        max_rational_helper(self, &other, prec, rm)
    }

    /// Computes the larger of a [`Float`] and a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] and the [`Rational`] are both
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the result is less
    /// than, equal to, or greater than the exact larger value. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The comparison is exact, and only the winning operand is rounded. Converting the
    /// [`Rational`] to a [`Float`] first could select the wrong operand, when the conversion
    /// crosses the other operand's value.
    ///
    /// Special cases:
    /// - If the [`Float`] is `NaN`, the [`Rational`] operand is returned (rounded), as with the
    ///   [`Float`]-[`Float`] functions.
    /// - If the operands are equal, the [`Float`] operand is returned (rounded), except that a
    ///   negative zero loses the tie against the (unsigned, treated as positive) zero [`Rational`]:
    ///   the result is then a positive zero, matching the positive-zero preference of `mpfr_max`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the winning operand is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(3u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o) = x.max_rational_prec_round_ref_ref(&y, 5, Floor);
    /// assert_eq!(r.to_string(), "3.12");
    /// assert_eq!(o, Less);
    ///
    /// let x = Float::from(3u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o) = x.max_rational_prec_round_ref_ref(&y, 5, Ceiling);
    /// assert_eq!(r.to_string(), "3.25");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn max_rational_prec_round_ref_ref(
        &self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        max_rational_helper(self, other, prec, rm)
    }

    /// Computes the larger of a [`Float`] and a [`Rational`], rounding the result to the nearest
    /// value of the specified precision. The [`Float`] and the [`Rational`] are both taken by
    /// value. An [`Ordering`] is also returned, indicating whether the result is less than, equal
    /// to, or greater than the exact larger value. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The comparison is exact, and only the winning operand is rounded. Converting the
    /// [`Rational`] to a [`Float`] first could select the wrong operand, when the conversion
    /// crosses the other operand's value.
    ///
    /// Special cases:
    /// - If the [`Float`] is `NaN`, the [`Rational`] operand is returned (rounded), as with the
    ///   [`Float`]-[`Float`] functions.
    /// - If the operands are equal, the [`Float`] operand is returned (rounded), except that a
    ///   negative zero loses the tie against the (unsigned, treated as positive) zero [`Rational`]:
    ///   the result is then a positive zero, matching the positive-zero preference of `mpfr_max`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let (r, o) = Float::from(3u32).max_rational_prec(Rational::from_signeds(22, 7), 5);
    /// assert_eq!(r.to_string(), "3.12");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn max_rational_prec(self, other: Rational, prec: u64) -> (Self, Ordering) {
        self.max_rational_prec_round(other, prec, Nearest)
    }

    /// Computes the larger of a [`Float`] and a [`Rational`], rounding the result to the nearest
    /// value of the specified precision. The [`Float`] is taken by value and the [`Rational`] by
    /// reference. An [`Ordering`] is also returned, indicating whether the result is less than,
    /// equal to, or greater than the exact larger value. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The comparison is exact, and only the winning operand is rounded. Converting the
    /// [`Rational`] to a [`Float`] first could select the wrong operand, when the conversion
    /// crosses the other operand's value.
    ///
    /// Special cases:
    /// - If the [`Float`] is `NaN`, the [`Rational`] operand is returned (rounded), as with the
    ///   [`Float`]-[`Float`] functions.
    /// - If the operands are equal, the [`Float`] operand is returned (rounded), except that a
    ///   negative zero loses the tie against the (unsigned, treated as positive) zero [`Rational`]:
    ///   the result is then a positive zero, matching the positive-zero preference of `mpfr_max`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let (r, o) = Float::from(3u32).max_rational_prec_val_ref(&Rational::from_signeds(22, 7), 5);
    /// assert_eq!(r.to_string(), "3.12");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn max_rational_prec_val_ref(self, other: &Rational, prec: u64) -> (Self, Ordering) {
        self.max_rational_prec_round_val_ref(other, prec, Nearest)
    }

    /// Computes the larger of a [`Float`] and a [`Rational`], rounding the result to the nearest
    /// value of the specified precision. The [`Float`] is taken by reference and the [`Rational`]
    /// by value. An [`Ordering`] is also returned, indicating whether the result is less than,
    /// equal to, or greater than the exact larger value. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The comparison is exact, and only the winning operand is rounded. Converting the
    /// [`Rational`] to a [`Float`] first could select the wrong operand, when the conversion
    /// crosses the other operand's value.
    ///
    /// Special cases:
    /// - If the [`Float`] is `NaN`, the [`Rational`] operand is returned (rounded), as with the
    ///   [`Float`]-[`Float`] functions.
    /// - If the operands are equal, the [`Float`] operand is returned (rounded), except that a
    ///   negative zero loses the tie against the (unsigned, treated as positive) zero [`Rational`]:
    ///   the result is then a positive zero, matching the positive-zero preference of `mpfr_max`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let (r, o) = Float::from(3u32).max_rational_prec_ref_val(Rational::from_signeds(22, 7), 5);
    /// assert_eq!(r.to_string(), "3.12");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn max_rational_prec_ref_val(&self, other: Rational, prec: u64) -> (Self, Ordering) {
        self.max_rational_prec_round_ref_val(other, prec, Nearest)
    }

    /// Computes the larger of a [`Float`] and a [`Rational`], rounding the result to the nearest
    /// value of the specified precision. The [`Float`] and the [`Rational`] are both taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the result is less than,
    /// equal to, or greater than the exact larger value. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The comparison is exact, and only the winning operand is rounded. Converting the
    /// [`Rational`] to a [`Float`] first could select the wrong operand, when the conversion
    /// crosses the other operand's value.
    ///
    /// Special cases:
    /// - If the [`Float`] is `NaN`, the [`Rational`] operand is returned (rounded), as with the
    ///   [`Float`]-[`Float`] functions.
    /// - If the operands are equal, the [`Float`] operand is returned (rounded), except that a
    ///   negative zero loses the tie against the (unsigned, treated as positive) zero [`Rational`]:
    ///   the result is then a positive zero, matching the positive-zero preference of `mpfr_max`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let (r, o) = Float::from(3u32).max_rational_prec_ref_ref(&Rational::from_signeds(22, 7), 5);
    /// assert_eq!(r.to_string(), "3.12");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn max_rational_prec_ref_ref(&self, other: &Rational, prec: u64) -> (Self, Ordering) {
        self.max_rational_prec_round_ref_ref(other, prec, Nearest)
    }

    /// Computes the larger of a [`Float`] and a [`Rational`], rounding the result to the
    /// [`Float`]'s precision, with the specified rounding mode. The [`Float`] and the [`Rational`]
    /// are both taken by value. An [`Ordering`] is also returned, indicating whether the result is
    /// less than, equal to, or greater than the exact larger value. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The comparison is exact, and only the winning operand is rounded. Converting the
    /// [`Rational`] to a [`Float`] first could select the wrong operand, when the conversion
    /// crosses the other operand's value.
    ///
    /// Special cases:
    /// - If the [`Float`] is `NaN`, the [`Rational`] operand is returned (rounded), as with the
    ///   [`Float`]-[`Float`] functions.
    /// - If the operands are equal, the [`Float`] operand is returned (rounded), except that a
    ///   negative zero loses the tie against the (unsigned, treated as positive) zero [`Rational`]:
    ///   the result is then a positive zero, matching the positive-zero preference of `mpfr_max`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the winning operand is not exactly representable with the
    /// output precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let (r, o) = Float::from(3u32).max_rational_round(Rational::from_signeds(22, 7), Floor);
    /// assert_eq!(r.to_string(), "3.0");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn max_rational_round(self, other: Rational, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.max_rational_prec_round(other, prec, rm)
    }

    /// Computes the larger of a [`Float`] and a [`Rational`], rounding the result to the
    /// [`Float`]'s precision, with the specified rounding mode. The [`Float`] is taken by value and
    /// the [`Rational`] by reference. An [`Ordering`] is also returned, indicating whether the
    /// result is less than, equal to, or greater than the exact larger value. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The comparison is exact, and only the winning operand is rounded. Converting the
    /// [`Rational`] to a [`Float`] first could select the wrong operand, when the conversion
    /// crosses the other operand's value.
    ///
    /// Special cases:
    /// - If the [`Float`] is `NaN`, the [`Rational`] operand is returned (rounded), as with the
    ///   [`Float`]-[`Float`] functions.
    /// - If the operands are equal, the [`Float`] operand is returned (rounded), except that a
    ///   negative zero loses the tie against the (unsigned, treated as positive) zero [`Rational`]:
    ///   the result is then a positive zero, matching the positive-zero preference of `mpfr_max`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the winning operand is not exactly representable with the
    /// output precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let (r, o) =
    ///     Float::from(3u32).max_rational_round_val_ref(&Rational::from_signeds(22, 7), Floor);
    /// assert_eq!(r.to_string(), "3.0");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn max_rational_round_val_ref(
        self,
        other: &Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.max_rational_prec_round_val_ref(other, prec, rm)
    }

    /// Computes the larger of a [`Float`] and a [`Rational`], rounding the result to the
    /// [`Float`]'s precision, with the specified rounding mode. The [`Float`] is taken by reference
    /// and the [`Rational`] by value. An [`Ordering`] is also returned, indicating whether the
    /// result is less than, equal to, or greater than the exact larger value. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The comparison is exact, and only the winning operand is rounded. Converting the
    /// [`Rational`] to a [`Float`] first could select the wrong operand, when the conversion
    /// crosses the other operand's value.
    ///
    /// Special cases:
    /// - If the [`Float`] is `NaN`, the [`Rational`] operand is returned (rounded), as with the
    ///   [`Float`]-[`Float`] functions.
    /// - If the operands are equal, the [`Float`] operand is returned (rounded), except that a
    ///   negative zero loses the tie against the (unsigned, treated as positive) zero [`Rational`]:
    ///   the result is then a positive zero, matching the positive-zero preference of `mpfr_max`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the winning operand is not exactly representable with the
    /// output precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let (r, o) =
    ///     Float::from(3u32).max_rational_round_ref_val(Rational::from_signeds(22, 7), Floor);
    /// assert_eq!(r.to_string(), "3.0");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn max_rational_round_ref_val(
        &self,
        other: Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.max_rational_prec_round_ref_val(other, prec, rm)
    }

    /// Computes the larger of a [`Float`] and a [`Rational`], rounding the result to the
    /// [`Float`]'s precision, with the specified rounding mode. The [`Float`] and the [`Rational`]
    /// are both taken by reference. An [`Ordering`] is also returned, indicating whether the result
    /// is less than, equal to, or greater than the exact larger value. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The comparison is exact, and only the winning operand is rounded. Converting the
    /// [`Rational`] to a [`Float`] first could select the wrong operand, when the conversion
    /// crosses the other operand's value.
    ///
    /// Special cases:
    /// - If the [`Float`] is `NaN`, the [`Rational`] operand is returned (rounded), as with the
    ///   [`Float`]-[`Float`] functions.
    /// - If the operands are equal, the [`Float`] operand is returned (rounded), except that a
    ///   negative zero loses the tie against the (unsigned, treated as positive) zero [`Rational`]:
    ///   the result is then a positive zero, matching the positive-zero preference of `mpfr_max`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the winning operand is not exactly representable with the
    /// output precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let (r, o) =
    ///     Float::from(3u32).max_rational_round_ref_ref(&Rational::from_signeds(22, 7), Floor);
    /// assert_eq!(r.to_string(), "3.0");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn max_rational_round_ref_ref(
        &self,
        other: &Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.max_rational_prec_round_ref_ref(other, prec, rm)
    }

    /// Computes the larger of a [`Float`] and a [`Rational`], rounding the result to the nearest
    /// value of the [`Float`]'s precision. The [`Float`] and the [`Rational`] are both taken by
    /// value. An [`Ordering`] is also returned, indicating whether the result is less than, equal
    /// to, or greater than the exact larger value. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The comparison is exact, and only the winning operand is rounded. Converting the
    /// [`Rational`] to a [`Float`] first could select the wrong operand, when the conversion
    /// crosses the other operand's value.
    ///
    /// Special cases:
    /// - If the [`Float`] is `NaN`, the [`Rational`] operand is returned (rounded), as with the
    ///   [`Float`]-[`Float`] functions.
    /// - If the operands are equal, the [`Float`] operand is returned (rounded), except that a
    ///   negative zero loses the tie against the (unsigned, treated as positive) zero [`Rational`]:
    ///   the result is then a positive zero, matching the positive-zero preference of `mpfr_max`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let (r, o) = Float::from(3u32).max_rational(Rational::from_signeds(22, 7));
    /// assert_eq!(r.to_string(), "3.0");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn max_rational(self, other: Rational) -> (Self, Ordering) {
        self.max_rational_round(other, Nearest)
    }

    /// Computes the larger of a [`Float`] and a [`Rational`], rounding the result to the nearest
    /// value of the [`Float`]'s precision. The [`Float`] is taken by value and the [`Rational`] by
    /// reference. An [`Ordering`] is also returned, indicating whether the result is less than,
    /// equal to, or greater than the exact larger value. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The comparison is exact, and only the winning operand is rounded. Converting the
    /// [`Rational`] to a [`Float`] first could select the wrong operand, when the conversion
    /// crosses the other operand's value.
    ///
    /// Special cases:
    /// - If the [`Float`] is `NaN`, the [`Rational`] operand is returned (rounded), as with the
    ///   [`Float`]-[`Float`] functions.
    /// - If the operands are equal, the [`Float`] operand is returned (rounded), except that a
    ///   negative zero loses the tie against the (unsigned, treated as positive) zero [`Rational`]:
    ///   the result is then a positive zero, matching the positive-zero preference of `mpfr_max`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let (r, o) = Float::from(3u32).max_rational_val_ref(&Rational::from_signeds(22, 7));
    /// assert_eq!(r.to_string(), "3.0");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn max_rational_val_ref(self, other: &Rational) -> (Self, Ordering) {
        self.max_rational_round_val_ref(other, Nearest)
    }

    /// Computes the larger of a [`Float`] and a [`Rational`], rounding the result to the nearest
    /// value of the [`Float`]'s precision. The [`Float`] is taken by reference and the [`Rational`]
    /// by value. An [`Ordering`] is also returned, indicating whether the result is less than,
    /// equal to, or greater than the exact larger value. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The comparison is exact, and only the winning operand is rounded. Converting the
    /// [`Rational`] to a [`Float`] first could select the wrong operand, when the conversion
    /// crosses the other operand's value.
    ///
    /// Special cases:
    /// - If the [`Float`] is `NaN`, the [`Rational`] operand is returned (rounded), as with the
    ///   [`Float`]-[`Float`] functions.
    /// - If the operands are equal, the [`Float`] operand is returned (rounded), except that a
    ///   negative zero loses the tie against the (unsigned, treated as positive) zero [`Rational`]:
    ///   the result is then a positive zero, matching the positive-zero preference of `mpfr_max`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let (r, o) = Float::from(3u32).max_rational_ref_val(Rational::from_signeds(22, 7));
    /// assert_eq!(r.to_string(), "3.0");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn max_rational_ref_val(&self, other: Rational) -> (Self, Ordering) {
        self.max_rational_round_ref_val(other, Nearest)
    }

    /// Computes the larger of a [`Float`] and a [`Rational`], rounding the result to the nearest
    /// value of the [`Float`]'s precision. The [`Float`] and the [`Rational`] are both taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the result is less than,
    /// equal to, or greater than the exact larger value. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The comparison is exact, and only the winning operand is rounded. Converting the
    /// [`Rational`] to a [`Float`] first could select the wrong operand, when the conversion
    /// crosses the other operand's value.
    ///
    /// Special cases:
    /// - If the [`Float`] is `NaN`, the [`Rational`] operand is returned (rounded), as with the
    ///   [`Float`]-[`Float`] functions.
    /// - If the operands are equal, the [`Float`] operand is returned (rounded), except that a
    ///   negative zero loses the tie against the (unsigned, treated as positive) zero [`Rational`]:
    ///   the result is then a positive zero, matching the positive-zero preference of `mpfr_max`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let (r, o) = Float::from(3u32).max_rational_ref_ref(&Rational::from_signeds(22, 7));
    /// assert_eq!(r.to_string(), "3.0");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn max_rational_ref_ref(&self, other: &Rational) -> (Self, Ordering) {
        self.max_rational_round_ref_ref(other, Nearest)
    }
}
