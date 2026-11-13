// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::{Finite, Infinity, NaN, Zero};
use crate::big_exponent_float::arithmetic::add::round_optional;
use crate::big_exponent_float::arithmetic::round_to_integer::{
    finite_is_integer, round_to_integer_exact,
};
use alloc::borrow::Cow;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::ModPowerOf2;
use malachite_base::num::basic::traits::{NaN as NaNTrait, Zero as ZeroTrait};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;

// x - trunc(x), exactly: it has x's sign, and is zero for integers and infinities.
pub(crate) fn fractional_part_exact(x: Cow<'_, BigExponentFloat>) -> BigExponentFloat {
    let (sign, exponent) = match &*x {
        BigExponentFloat(NaN) => return BigExponentFloat::NAN,
        BigExponentFloat(Infinity { .. } | Zero) => return BigExponentFloat::ZERO,
        BigExponentFloat(Finite { sign, exponent, .. }) => (*sign, exponent),
    };
    if *exponent <= 0u32 {
        // |x| < 1
        return x.into_owned();
    }
    if finite_is_integer(&x) {
        return BigExponentFloat::ZERO;
    }
    let BigExponentFloat(Finite { significand, .. }) = &*x else {
        unreachable!()
    };
    // Since x is not an integer, 0 < exponent < bits(significand), so the exponent fits in a `u64`.
    // The low `shift` bits of the significand are x's fractional bits, so x - trunc(x) = f *
    // 2^(-shift), with f those bits.
    let shift = significand.significant_bits() - u64::exact_from(exponent);
    let f = significand.mod_power_of_2(shift);
    let exponent = -Integer::from(shift - f.significant_bits());
    BigExponentFloat::from_parts(sign, exponent, f)
}

// The fractional part of x, rounded with `prec_rm` if it is `Some`.
pub(crate) fn fractional_part_helper(
    x: Cow<'_, BigExponentFloat>,
    prec_rm: Option<(u64, RoundingMode)>,
) -> (BigExponentFloat, Ordering) {
    if let Some((prec, _)) = prec_rm {
        assert_ne!(prec, 0);
    }
    round_optional(fractional_part_exact(x), prec_rm)
}

// trunc(x) and x - trunc(x), rounded to `iprec` and `fprec` bits with `rm`, or exact if
// `iprec_fprec_rm` is `None`. For an infinity, the integer part is the infinity and the fractional
// part zero; `NaN` gives two `NaN`s.
pub(crate) fn integer_and_fractional_parts_helper(
    x: Cow<'_, BigExponentFloat>,
    iprec_fprec_rm: Option<(u64, u64, RoundingMode)>,
) -> ((BigExponentFloat, Ordering), (BigExponentFloat, Ordering)) {
    if let Some((iprec, fprec, _)) = iprec_fprec_rm {
        assert_ne!(iprec, 0);
        assert_ne!(fprec, 0);
    }
    let f = fractional_part_exact(Cow::Borrowed(&*x));
    let i = round_to_integer_exact(x, Down, false).0;
    match iprec_fprec_rm {
        Some((iprec, fprec, rm)) => (
            round_optional(i, Some((iprec, rm))),
            round_optional(f, Some((fprec, rm))),
        ),
        None => ((i, Equal), (f, Equal)),
    }
}

impl BigExponentFloat {
    /// Returns the fractional part of a [`BigExponentFloat`], $x-\operatorname{trunc}(x)$, rounded
    /// to the specified precision with the specified rounding mode, along with an [`Ordering`]
    /// comparing the result with the exact fractional part. The [`BigExponentFloat`] is taken by
    /// value.
    ///
    /// The fractional part has the same sign as the input, and is zero for integers and infinities;
    /// `NaN` gives `NaN`. The rounding mode rounds the exact fraction: for a negative input,
    /// `Floor` rounds the (negative) fraction downward. Whenever the result equals the exact
    /// fraction, the [`Ordering`] is `Equal`.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::fractional_part_prec`] instead.
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
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fraction cannot be represented
    /// exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (f, o) = BigExponentFloat::from(3141.592653589793).fractional_part_prec_round(5, Floor);
    /// assert_eq!(format!("{f:?}"), "0x0.9");
    /// assert_eq!(o, Less);
    ///
    /// let (f, o) =
    ///     BigExponentFloat::from(3141.592653589793).fractional_part_prec_round(5, Ceiling);
    /// assert_eq!(format!("{f:?}"), "0x0.98");
    /// assert_eq!(o, Greater);
    ///
    /// let (f, o) =
    ///     BigExponentFloat::from(3141.592653589793).fractional_part_prec_round(5, Nearest);
    /// assert_eq!(format!("{f:?}"), "0x0.98");
    /// assert_eq!(o, Greater);
    ///
    /// let (f, o) =
    ///     BigExponentFloat::from(3141.592653589793).fractional_part_prec_round(20, Floor);
    /// assert_eq!(format!("{f:?}"), "0x0.97b82");
    /// assert_eq!(o, Less);
    ///
    /// let (f, o) =
    ///     BigExponentFloat::from(3141.592653589793).fractional_part_prec_round(20, Ceiling);
    /// assert_eq!(format!("{f:?}"), "0x0.97b83");
    /// assert_eq!(o, Greater);
    ///
    /// let (f, o) =
    ///     BigExponentFloat::from(3141.592653589793).fractional_part_prec_round(20, Nearest);
    /// assert_eq!(format!("{f:?}"), "0x0.97b82");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn fractional_part_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        fractional_part_helper(Cow::Owned(self), Some((prec, rm)))
    }

    /// Returns the fractional part of a [`BigExponentFloat`], $x-\operatorname{trunc}(x)$, rounded
    /// to the nearest value of the specified precision, along with an [`Ordering`] comparing the
    /// result with the exact fractional part. The [`BigExponentFloat`] is taken by value.
    ///
    /// The fractional part has the same sign as the input, and is zero for integers and infinities;
    /// `NaN` gives `NaN`.
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
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (f, o) = BigExponentFloat::from(3141.592653589793).fractional_part_prec(5);
    /// assert_eq!(format!("{f:?}"), "0x0.98");
    /// assert_eq!(o, Greater);
    ///
    /// let (f, o) = BigExponentFloat::from(3141.592653589793).fractional_part_prec(20);
    /// assert_eq!(format!("{f:?}"), "0x0.97b82");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn fractional_part_prec(self, prec: u64) -> (Self, Ordering) {
        fractional_part_helper(Cow::Owned(self), Some((prec, Nearest)))
    }

    /// Returns the fractional part of a [`BigExponentFloat`], $x-\operatorname{trunc}(x)$. The
    /// fractional part is exact. The [`BigExponentFloat`] is taken by value.
    ///
    /// The fractional part has the same sign as the input, and is zero for integers and infinities;
    /// `NaN` gives `NaN`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!("{:?}", BigExponentFloat::from(-3.25).fractional_part()),
    ///     "-0x0.4"
    /// );
    /// ```
    #[inline]
    pub fn fractional_part(self) -> Self {
        fractional_part_helper(Cow::Owned(self), None).0
    }

    /// Returns the fractional part of a [`BigExponentFloat`], $x-\operatorname{trunc}(x)$, rounded
    /// to the specified precision with the specified rounding mode, along with an [`Ordering`]
    /// comparing the result with the exact fractional part. The [`BigExponentFloat`] is taken by
    /// reference.
    ///
    /// The fractional part has the same sign as the input, and is zero for integers and infinities;
    /// `NaN` gives `NaN`. The rounding mode rounds the exact fraction: for a negative input,
    /// `Floor` rounds the (negative) fraction downward. Whenever the result equals the exact
    /// fraction, the [`Ordering`] is `Equal`.
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
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fraction cannot be represented
    /// exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (f, o) =
    ///     (&BigExponentFloat::from(3141.592653589793)).fractional_part_prec_round_ref(5, Floor);
    /// assert_eq!(format!("{f:?}"), "0x0.9");
    /// assert_eq!(o, Less);
    ///
    /// let (f, o) =
    ///     (&BigExponentFloat::from(3141.592653589793)).fractional_part_prec_round_ref(5, Ceiling);
    /// assert_eq!(format!("{f:?}"), "0x0.98");
    /// assert_eq!(o, Greater);
    ///
    /// let (f, o) =
    ///     (&BigExponentFloat::from(3141.592653589793)).fractional_part_prec_round_ref(5, Nearest);
    /// assert_eq!(format!("{f:?}"), "0x0.98");
    /// assert_eq!(o, Greater);
    ///
    /// let (f, o) =
    ///     (&BigExponentFloat::from(3141.592653589793)).fractional_part_prec_round_ref(20, Floor);
    /// assert_eq!(format!("{f:?}"), "0x0.97b82");
    /// assert_eq!(o, Less);
    ///
    /// let (f, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .fractional_part_prec_round_ref(20, Ceiling);
    /// assert_eq!(format!("{f:?}"), "0x0.97b83");
    /// assert_eq!(o, Greater);
    ///
    /// let (f, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .fractional_part_prec_round_ref(20, Nearest);
    /// assert_eq!(format!("{f:?}"), "0x0.97b82");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn fractional_part_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        fractional_part_helper(Cow::Borrowed(self), Some((prec, rm)))
    }

    /// Returns the fractional part of a [`BigExponentFloat`], $x-\operatorname{trunc}(x)$, rounded
    /// to the nearest value of the specified precision, along with an [`Ordering`] comparing the
    /// result with the exact fractional part. The [`BigExponentFloat`] is taken by reference.
    ///
    /// The fractional part has the same sign as the input, and is zero for integers and infinities;
    /// `NaN` gives `NaN`.
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
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (f, o) = (&BigExponentFloat::from(3141.592653589793)).fractional_part_prec_ref(5);
    /// assert_eq!(format!("{f:?}"), "0x0.98");
    /// assert_eq!(o, Greater);
    ///
    /// let (f, o) = (&BigExponentFloat::from(3141.592653589793)).fractional_part_prec_ref(20);
    /// assert_eq!(format!("{f:?}"), "0x0.97b82");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn fractional_part_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        fractional_part_helper(Cow::Borrowed(self), Some((prec, Nearest)))
    }

    /// Returns the fractional part of a [`BigExponentFloat`], $x-\operatorname{trunc}(x)$. The
    /// fractional part is exact. The [`BigExponentFloat`] is taken by reference.
    ///
    /// The fractional part has the same sign as the input, and is zero for integers and infinities;
    /// `NaN` gives `NaN`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         (&BigExponentFloat::from(-3.25)).fractional_part_ref()
    ///     ),
    ///     "-0x0.4"
    /// );
    /// ```
    #[inline]
    pub fn fractional_part_ref(&self) -> Self {
        fractional_part_helper(Cow::Borrowed(self), None).0
    }

    /// Returns the integer and fractional parts of a [`BigExponentFloat`],
    /// $\operatorname{trunc}(x)$ and $x-\operatorname{trunc}(x)$, rounded to the specified
    /// precisions with the specified rounding mode, each with an [`Ordering`] comparing it with its
    /// exact value. The [`BigExponentFloat`] is taken by value.
    ///
    /// The integer part is the input truncated toward zero and then rounded to its target
    /// precision, as by [`BigExponentFloat::round_to_integer_then_prec_round`] with `Down`; the
    /// fractional part is as by [`BigExponentFloat::fractional_part_prec_round`]. Both parts have
    /// the input's sign. For an infinity, the integer part is the infinity and the fractional part
    /// zero; `NaN` gives two `NaN`s.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::integer_and_fractional_parts_prec`] instead.
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
    /// Panics if `iprec` or `fprec` is zero, or if `rm` is `Exact` and either part cannot be
    /// represented exactly with its precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let ((i, io), (f, fo)) = BigExponentFloat::from(-3141.592653589793)
    ///     .integer_and_fractional_parts_prec_round(5, 5, Floor);
    /// assert_eq!(format!("{i:?}"), "-0xc.8E+2");
    /// assert_eq!(io, Less);
    /// assert_eq!(format!("{f:?}"), "-0x0.98");
    /// assert_eq!(fo, Less);
    ///
    /// let ((i, io), (f, fo)) = BigExponentFloat::from(-3141.592653589793)
    ///     .integer_and_fractional_parts_prec_round(5, 5, Ceiling);
    /// assert_eq!(format!("{i:?}"), "-0xc.0E+2");
    /// assert_eq!(io, Greater);
    /// assert_eq!(format!("{f:?}"), "-0x0.9");
    /// assert_eq!(fo, Greater);
    ///
    /// let ((i, io), (f, fo)) = BigExponentFloat::from(-3141.592653589793)
    ///     .integer_and_fractional_parts_prec_round(5, 5, Nearest);
    /// assert_eq!(format!("{i:?}"), "-0xc.8E+2");
    /// assert_eq!(io, Less);
    /// assert_eq!(format!("{f:?}"), "-0x0.98");
    /// assert_eq!(fo, Less);
    ///
    /// let ((i, io), (f, fo)) = BigExponentFloat::from(-3141.592653589793)
    ///     .integer_and_fractional_parts_prec_round(20, 20, Floor);
    /// assert_eq!(format!("{i:?}"), "-0xc45.0");
    /// assert_eq!(io, Equal);
    /// assert_eq!(format!("{f:?}"), "-0x0.97b83");
    /// assert_eq!(fo, Less);
    ///
    /// let ((i, io), (f, fo)) = BigExponentFloat::from(-3141.592653589793)
    ///     .integer_and_fractional_parts_prec_round(20, 20, Ceiling);
    /// assert_eq!(format!("{i:?}"), "-0xc45.0");
    /// assert_eq!(io, Equal);
    /// assert_eq!(format!("{f:?}"), "-0x0.97b82");
    /// assert_eq!(fo, Greater);
    ///
    /// let ((i, io), (f, fo)) = BigExponentFloat::from(-3141.592653589793)
    ///     .integer_and_fractional_parts_prec_round(20, 20, Nearest);
    /// assert_eq!(format!("{i:?}"), "-0xc45.0");
    /// assert_eq!(io, Equal);
    /// assert_eq!(format!("{f:?}"), "-0x0.97b82");
    /// assert_eq!(fo, Greater);
    /// ```
    #[inline]
    pub fn integer_and_fractional_parts_prec_round(
        self,
        iprec: u64,
        fprec: u64,
        rm: RoundingMode,
    ) -> ((Self, Ordering), (Self, Ordering)) {
        integer_and_fractional_parts_helper(Cow::Owned(self), Some((iprec, fprec, rm)))
    }

    /// Returns the integer and fractional parts of a [`BigExponentFloat`],
    /// $\operatorname{trunc}(x)$ and $x-\operatorname{trunc}(x)$, rounded to the nearest values of
    /// the specified precisions, each with an [`Ordering`] comparing it with its exact value. The
    /// [`BigExponentFloat`] is taken by value.
    ///
    /// Both parts have the input's sign. For an infinity, the integer part is the infinity and the
    /// fractional part zero; `NaN` gives two `NaN`s.
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
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let ((i, io), (f, fo)) =
    ///     BigExponentFloat::from(-3141.592653589793).integer_and_fractional_parts_prec(5, 5);
    /// assert_eq!(format!("{i:?}"), "-0xc.8E+2");
    /// assert_eq!(io, Less);
    /// assert_eq!(format!("{f:?}"), "-0x0.98");
    /// assert_eq!(fo, Less);
    ///
    /// let ((i, io), (f, fo)) =
    ///     BigExponentFloat::from(-3141.592653589793).integer_and_fractional_parts_prec(20, 20);
    /// assert_eq!(format!("{i:?}"), "-0xc45.0");
    /// assert_eq!(io, Equal);
    /// assert_eq!(format!("{f:?}"), "-0x0.97b82");
    /// assert_eq!(fo, Greater);
    /// ```
    #[inline]
    pub fn integer_and_fractional_parts_prec(
        self,
        iprec: u64,
        fprec: u64,
    ) -> ((Self, Ordering), (Self, Ordering)) {
        integer_and_fractional_parts_helper(Cow::Owned(self), Some((iprec, fprec, Nearest)))
    }

    /// Returns the integer and fractional parts of a [`BigExponentFloat`],
    /// $\operatorname{trunc}(x)$ and $x-\operatorname{trunc}(x)$. Both parts are exact. The
    /// [`BigExponentFloat`] is taken by value.
    ///
    /// Both parts have the input's sign. For an infinity, the integer part is the infinity and the
    /// fractional part zero; `NaN` gives two `NaN`s.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// let (i, f) = BigExponentFloat::from(-3.25).integer_and_fractional_parts();
    /// assert_eq!(format!("{i:?}"), "-0x3.0");
    /// assert_eq!(format!("{f:?}"), "-0x0.4");
    /// ```
    #[inline]
    pub fn integer_and_fractional_parts(self) -> (Self, Self) {
        let ((i, _), (f, _)) = integer_and_fractional_parts_helper(Cow::Owned(self), None);
        (i, f)
    }

    /// Returns the integer and fractional parts of a [`BigExponentFloat`],
    /// $\operatorname{trunc}(x)$ and $x-\operatorname{trunc}(x)$, rounded to the specified
    /// precisions with the specified rounding mode, each with an [`Ordering`] comparing it with its
    /// exact value. The [`BigExponentFloat`] is taken by reference.
    ///
    /// The integer part is the input truncated toward zero and then rounded to its target
    /// precision, as by [`BigExponentFloat::round_to_integer_then_prec_round`] with `Down`; the
    /// fractional part is as by [`BigExponentFloat::fractional_part_prec_round`]. Both parts have
    /// the input's sign. For an infinity, the integer part is the infinity and the fractional part
    /// zero; `NaN` gives two `NaN`s.
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
    /// Panics if `iprec` or `fprec` is zero, or if `rm` is `Exact` and either part cannot be
    /// represented exactly with its precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let ((i, io), (f, fo)) = (&BigExponentFloat::from(-3141.592653589793))
    ///     .integer_and_fractional_parts_prec_round_ref(5, 5, Floor);
    /// assert_eq!(format!("{i:?}"), "-0xc.8E+2");
    /// assert_eq!(io, Less);
    /// assert_eq!(format!("{f:?}"), "-0x0.98");
    /// assert_eq!(fo, Less);
    ///
    /// let ((i, io), (f, fo)) = (&BigExponentFloat::from(-3141.592653589793))
    ///     .integer_and_fractional_parts_prec_round_ref(5, 5, Ceiling);
    /// assert_eq!(format!("{i:?}"), "-0xc.0E+2");
    /// assert_eq!(io, Greater);
    /// assert_eq!(format!("{f:?}"), "-0x0.9");
    /// assert_eq!(fo, Greater);
    ///
    /// let ((i, io), (f, fo)) = (&BigExponentFloat::from(-3141.592653589793))
    ///     .integer_and_fractional_parts_prec_round_ref(5, 5, Nearest);
    /// assert_eq!(format!("{i:?}"), "-0xc.8E+2");
    /// assert_eq!(io, Less);
    /// assert_eq!(format!("{f:?}"), "-0x0.98");
    /// assert_eq!(fo, Less);
    ///
    /// let ((i, io), (f, fo)) = (&BigExponentFloat::from(-3141.592653589793))
    ///     .integer_and_fractional_parts_prec_round_ref(20, 20, Floor);
    /// assert_eq!(format!("{i:?}"), "-0xc45.0");
    /// assert_eq!(io, Equal);
    /// assert_eq!(format!("{f:?}"), "-0x0.97b83");
    /// assert_eq!(fo, Less);
    ///
    /// let ((i, io), (f, fo)) = (&BigExponentFloat::from(-3141.592653589793))
    ///     .integer_and_fractional_parts_prec_round_ref(20, 20, Ceiling);
    /// assert_eq!(format!("{i:?}"), "-0xc45.0");
    /// assert_eq!(io, Equal);
    /// assert_eq!(format!("{f:?}"), "-0x0.97b82");
    /// assert_eq!(fo, Greater);
    ///
    /// let ((i, io), (f, fo)) = (&BigExponentFloat::from(-3141.592653589793))
    ///     .integer_and_fractional_parts_prec_round_ref(20, 20, Nearest);
    /// assert_eq!(format!("{i:?}"), "-0xc45.0");
    /// assert_eq!(io, Equal);
    /// assert_eq!(format!("{f:?}"), "-0x0.97b82");
    /// assert_eq!(fo, Greater);
    /// ```
    #[inline]
    pub fn integer_and_fractional_parts_prec_round_ref(
        &self,
        iprec: u64,
        fprec: u64,
        rm: RoundingMode,
    ) -> ((Self, Ordering), (Self, Ordering)) {
        integer_and_fractional_parts_helper(Cow::Borrowed(self), Some((iprec, fprec, rm)))
    }

    /// Returns the integer and fractional parts of a [`BigExponentFloat`],
    /// $\operatorname{trunc}(x)$ and $x-\operatorname{trunc}(x)$, rounded to the nearest values of
    /// the specified precisions, each with an [`Ordering`] comparing it with its exact value. The
    /// [`BigExponentFloat`] is taken by reference.
    ///
    /// Both parts have the input's sign. For an infinity, the integer part is the infinity and the
    /// fractional part zero; `NaN` gives two `NaN`s.
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
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let ((i, io), (f, fo)) = (&BigExponentFloat::from(-3141.592653589793))
    ///     .integer_and_fractional_parts_prec_ref(5, 5);
    /// assert_eq!(format!("{i:?}"), "-0xc.8E+2");
    /// assert_eq!(io, Less);
    /// assert_eq!(format!("{f:?}"), "-0x0.98");
    /// assert_eq!(fo, Less);
    ///
    /// let ((i, io), (f, fo)) = (&BigExponentFloat::from(-3141.592653589793))
    ///     .integer_and_fractional_parts_prec_ref(20, 20);
    /// assert_eq!(format!("{i:?}"), "-0xc45.0");
    /// assert_eq!(io, Equal);
    /// assert_eq!(format!("{f:?}"), "-0x0.97b82");
    /// assert_eq!(fo, Greater);
    /// ```
    #[inline]
    pub fn integer_and_fractional_parts_prec_ref(
        &self,
        iprec: u64,
        fprec: u64,
    ) -> ((Self, Ordering), (Self, Ordering)) {
        integer_and_fractional_parts_helper(Cow::Borrowed(self), Some((iprec, fprec, Nearest)))
    }

    /// Returns the integer and fractional parts of a [`BigExponentFloat`],
    /// $\operatorname{trunc}(x)$ and $x-\operatorname{trunc}(x)$. Both parts are exact. The
    /// [`BigExponentFloat`] is taken by reference.
    ///
    /// Both parts have the input's sign. For an infinity, the integer part is the infinity and the
    /// fractional part zero; `NaN` gives two `NaN`s.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// let (i, f) = (&BigExponentFloat::from(-3.25)).integer_and_fractional_parts_ref();
    /// assert_eq!(format!("{i:?}"), "-0x3.0");
    /// assert_eq!(format!("{f:?}"), "-0x0.4");
    /// ```
    #[inline]
    pub fn integer_and_fractional_parts_ref(&self) -> (Self, Self) {
        let ((i, _), (f, _)) = integer_and_fractional_parts_helper(Cow::Borrowed(self), None);
        (i, f)
    }
}
