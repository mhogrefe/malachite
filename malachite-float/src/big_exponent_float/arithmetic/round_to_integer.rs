// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::{Finite, Infinity, NaN, Zero};
use alloc::borrow::Cow;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::traits::{One, Zero as ZeroTrait};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitAccess, SignificantBits};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

// The finite nonzero `x` written as ±m * 2^t with m odd: the pair (m, t).
pub(crate) fn finite_odd_part(x: &BigExponentFloat) -> (Natural, Integer) {
    let BigExponentFloat(Finite {
        exponent,
        significand,
        ..
    }) = x
    else {
        unreachable!()
    };
    let m = significand >> significand.trailing_zeros().unwrap();
    let t = exponent - Integer::from(m.significant_bits());
    (m, t)
}

// The number of significant bits of the finite nonzero `x`, not counting trailing zeros: the number
// of bits of m when x = ±m * 2^t with m odd.
pub(crate) fn finite_odd_part_bits(x: &BigExponentFloat) -> u64 {
    let BigExponentFloat(Finite { significand, .. }) = x else {
        unreachable!()
    };
    significand.significant_bits() - significand.trailing_zeros().unwrap()
}

// Whether the finite nonzero `x` is an integer.
pub(crate) fn finite_is_integer(x: &BigExponentFloat) -> bool {
    let BigExponentFloat(Finite { exponent, .. }) = x else {
        unreachable!()
    };
    *exponent >= finite_odd_part_bits(x)
}

// The integer part of |x|, for finite nonzero `x` that is not an integer, along with the bit just
// below it (the "half" bit) and whether any lower bit is set (the "sticky" bit).
pub(crate) fn split_non_integer(x: &BigExponentFloat) -> (Natural, bool, bool) {
    let BigExponentFloat(Finite {
        exponent,
        significand,
        ..
    }) = x
    else {
        unreachable!()
    };
    let bits = significand.significant_bits();
    let zeros = significand.trailing_zeros().unwrap();
    if *exponent <= 0u32 {
        // |x| < 1: |x| is at least 1/2 iff its exponent is 0, and then it is exactly 1/2 iff the
        // significand is a power of 2.
        let half = *exponent == 0u32;
        return (Natural::ZERO, half, !half || zeros != bits - 1);
    }
    // Since x is not an integer, 0 < exponent < bits - zeros, so the exponent fits in a `u64`. The
    // low `shift` bits of the significand are x's fractional bits.
    let shift = bits - u64::exact_from(exponent);
    let half = significand.get_bit(shift - 1);
    let sticky = zeros < shift - 1;
    (significand >> shift, half, sticky)
}

// The finite nonzero integer ±m, or zero.
fn signed_natural(sign: bool, m: Natural) -> BigExponentFloat {
    if m == 0u32 {
        BigExponentFloat::ZERO
    } else {
        let exponent = Integer::from(m.significant_bits());
        BigExponentFloat::from_parts(sign, exponent, m)
    }
}

// x rounded to an integer exactly, with `rm` (and, if `ties_away` is set, `Nearest` rounding ties
// away from zero rather than to even); an `Ordering` comparing the integer with x; and whether x is
// an integer. `NaN`s and infinities are returned unchanged, and are not integers.
pub(crate) fn round_to_integer_exact(
    x: Cow<'_, BigExponentFloat>,
    rm: RoundingMode,
    ties_away: bool,
) -> (BigExponentFloat, Ordering, bool) {
    assert_ne!(rm, Exact, "Exact rounding to an integer is not allowed");
    let sign = match &*x {
        BigExponentFloat(NaN | Infinity { .. }) => return (x.into_owned(), Equal, false),
        BigExponentFloat(Zero) => return (BigExponentFloat::ZERO, Equal, true),
        BigExponentFloat(Finite { sign, .. }) => *sign,
    };
    if finite_is_integer(&x) {
        return (x.into_owned(), Equal, true);
    }
    let (int_part, half, sticky) = split_non_integer(&x);
    // The rounding decision is made on the magnitude, so the directed modes are inverted for
    // negative x.
    let rm_abs = if sign { rm } else { -rm };
    let up = match rm_abs {
        Floor | Down => false,
        Ceiling | Up => true,
        Nearest => half && (sticky || ties_away || int_part.odd()),
        Exact => unreachable!(),
    };
    let (int_part, o) = if up {
        (int_part + Natural::ONE, Greater)
    } else {
        (int_part, Less)
    };
    (
        signed_natural(sign, int_part),
        if sign { o } else { o.reverse() },
        false,
    )
}

// x rounded to `prec` bits with `rm`, except that if `ties_away` is set, `Nearest` rounds ties away
// from zero. A tie is a value with exactly `prec + 1` significant bits.
fn round_to_prec_ties(
    x: Cow<'_, BigExponentFloat>,
    prec: u64,
    rm: RoundingMode,
    ties_away: bool,
) -> (BigExponentFloat, Ordering) {
    let rm = if ties_away && rm == Nearest && finite_odd_part_bits(&x) == prec + 1 {
        Up
    } else {
        rm
    };
    x.into_owned().round_to_prec_round(prec, rm)
}

// x rounded, in one step, to an integer representable with `prec` bits, as described for
// `round_to_integer_prec_round`.
//
// An integer x is just rounded to `prec` bits. Otherwise, let e be x's exponent. If e > prec, the
// values with `prec` bits near x, which lie in [2^(e - 1), 2^e) in magnitude, are multiples of 2^(e
// - prec) >= 2, so they are all integers, and the midpoints between them are integers too, so that
// x is not a tie; rounding x to `prec` bits gives the result. If e <= prec, every integer of
// magnitude at most 2^e has at most `prec` bits, so rounding x to an integer gives the result.
pub(crate) fn round_to_integer_prec_round_helper(
    x: Cow<'_, BigExponentFloat>,
    prec: u64,
    rm: RoundingMode,
    ties_away: bool,
) -> (BigExponentFloat, Ordering, bool) {
    assert_ne!(prec, 0);
    assert_ne!(rm, Exact, "Exact rounding to an integer is not allowed");
    let exponent = match &*x {
        BigExponentFloat(NaN | Infinity { .. }) => return (x.into_owned(), Equal, false),
        BigExponentFloat(Zero) => return (BigExponentFloat::ZERO, Equal, true),
        BigExponentFloat(Finite { exponent, .. }) => exponent,
    };
    if finite_is_integer(&x) {
        let (y, o) = round_to_prec_ties(x, prec, rm, ties_away);
        (y, o, true)
    } else if *exponent > prec {
        let (y, o) = round_to_prec_ties(x, prec, rm, ties_away);
        (y, o, false)
    } else {
        round_to_integer_exact(x, rm, ties_away)
    }
}

// x rounded to an integer exactly with `irm` (or, if `ties_away` is set, to the nearest integer
// with ties away from zero), and then to `prec` bits with `rm`. The `Ordering` compares the result
// with the integer.
pub(crate) fn round_to_integer_then_prec_round_helper(
    x: Cow<'_, BigExponentFloat>,
    irm: RoundingMode,
    ties_away: bool,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    round_to_integer_exact(x, irm, ties_away)
        .0
        .round_to_prec_round(prec, rm)
}

impl BigExponentFloat {
    /// Rounds a [`BigExponentFloat`] to an integer representable with the specified precision, in
    /// the direction given by the specified rounding mode. An [`Ordering`] comparing the result
    /// with the input is also returned, along with a `bool` indicating whether the input was an
    /// integer. The [`BigExponentFloat`] is taken by value.
    ///
    /// The result is produced by a single rounding to an integer representable with the target
    /// precision: if the input's integer part needs more bits than the precision provides, no
    /// intermediate integer is formed. For example, $10.5$ rounded to the nearest integer at a
    /// precision of 2 bits is $12$: not first $10$, and then $10$ rounded again. The rounding mode
    /// gives the integer-rounding direction: `Floor` and `Ceiling` are the floor and ceiling
    /// functions, `Down` is truncation, `Up` rounds away from zero, and `Nearest` rounds to the
    /// nearest integer with ties to even. `Exact` is not allowed.
    ///
    /// The pair of the [`Ordering`] and the `bool` carries the same information as `mpfr_rint`'s
    /// ternary value: `(Equal, true)` means the input was an integer representable at the target
    /// precision, returned unchanged; `(Less, true)` and `(Greater, true)` mean the input was an
    /// integer that required rounding to fit the precision; `(Less, false)` and `(Greater, false)`
    /// mean the input was not an integer. `(Equal, false)` cannot occur.
    ///
    /// `NaN`s, infinities, and zero are returned unchanged with `Equal`; of these, only zero is
    /// considered an integer.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::round_to_integer_prec`] instead.
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
    /// Panics if `prec` is zero or if `rm` is `Exact`.
    ///
    /// This is MPFR's `mpfr_rint`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (z, o, integer) =
    ///     BigExponentFloat::from(3141.592653589793).round_to_integer_prec_round(5, Floor);
    /// assert_eq!(format!("{z:?}"), "0xc.0E+2");
    /// assert_eq!(o, Less);
    /// assert_eq!(integer, false);
    ///
    /// let (z, o, integer) =
    ///     BigExponentFloat::from(3141.592653589793).round_to_integer_prec_round(5, Ceiling);
    /// assert_eq!(format!("{z:?}"), "0xc.8E+2");
    /// assert_eq!(o, Greater);
    /// assert_eq!(integer, false);
    ///
    /// let (z, o, integer) =
    ///     BigExponentFloat::from(3141.592653589793).round_to_integer_prec_round(5, Nearest);
    /// assert_eq!(format!("{z:?}"), "0xc.8E+2");
    /// assert_eq!(o, Greater);
    /// assert_eq!(integer, false);
    ///
    /// let (z, o, integer) =
    ///     BigExponentFloat::from(3141.592653589793).round_to_integer_prec_round(20, Floor);
    /// assert_eq!(format!("{z:?}"), "0xc45.0");
    /// assert_eq!(o, Less);
    /// assert_eq!(integer, false);
    ///
    /// let (z, o, integer) =
    ///     BigExponentFloat::from(3141.592653589793).round_to_integer_prec_round(20, Ceiling);
    /// assert_eq!(format!("{z:?}"), "0xc46.0");
    /// assert_eq!(o, Greater);
    /// assert_eq!(integer, false);
    ///
    /// let (z, o, integer) =
    ///     BigExponentFloat::from(3141.592653589793).round_to_integer_prec_round(20, Nearest);
    /// assert_eq!(format!("{z:?}"), "0xc46.0");
    /// assert_eq!(o, Greater);
    /// assert_eq!(integer, false);
    /// ```
    #[inline]
    pub fn round_to_integer_prec_round(
        self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, bool) {
        round_to_integer_prec_round_helper(Cow::Owned(self), prec, rm, false)
    }

    /// Rounds a [`BigExponentFloat`] to an integer representable with the specified precision, in
    /// the direction given by the specified rounding mode. An [`Ordering`] comparing the result
    /// with the input is also returned, along with a `bool` indicating whether the input was an
    /// integer. The [`BigExponentFloat`] is taken by reference.
    ///
    /// The result is produced by a single rounding to an integer representable with the target
    /// precision: if the input's integer part needs more bits than the precision provides, no
    /// intermediate integer is formed. For example, $10.5$ rounded to the nearest integer at a
    /// precision of 2 bits is $12$: not first $10$, and then $10$ rounded again. The rounding mode
    /// gives the integer-rounding direction: `Floor` and `Ceiling` are the floor and ceiling
    /// functions, `Down` is truncation, `Up` rounds away from zero, and `Nearest` rounds to the
    /// nearest integer with ties to even. `Exact` is not allowed.
    ///
    /// The pair of the [`Ordering`] and the `bool` carries the same information as `mpfr_rint`'s
    /// ternary value: `(Equal, true)` means the input was an integer representable at the target
    /// precision, returned unchanged; `(Less, true)` and `(Greater, true)` mean the input was an
    /// integer that required rounding to fit the precision; `(Less, false)` and `(Greater, false)`
    /// mean the input was not an integer. `(Equal, false)` cannot occur.
    ///
    /// `NaN`s, infinities, and zero are returned unchanged with `Equal`; of these, only zero is
    /// considered an integer.
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
    /// Panics if `prec` is zero or if `rm` is `Exact`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (z, o, integer) =
    ///     (&BigExponentFloat::from(3141.592653589793)).round_to_integer_prec_round_ref(5, Floor);
    /// assert_eq!(format!("{z:?}"), "0xc.0E+2");
    /// assert_eq!(o, Less);
    /// assert_eq!(integer, false);
    ///
    /// let (z, o, integer) = (&BigExponentFloat::from(3141.592653589793))
    ///     .round_to_integer_prec_round_ref(5, Ceiling);
    /// assert_eq!(format!("{z:?}"), "0xc.8E+2");
    /// assert_eq!(o, Greater);
    /// assert_eq!(integer, false);
    ///
    /// let (z, o, integer) = (&BigExponentFloat::from(3141.592653589793))
    ///     .round_to_integer_prec_round_ref(5, Nearest);
    /// assert_eq!(format!("{z:?}"), "0xc.8E+2");
    /// assert_eq!(o, Greater);
    /// assert_eq!(integer, false);
    ///
    /// let (z, o, integer) =
    ///     (&BigExponentFloat::from(3141.592653589793)).round_to_integer_prec_round_ref(20, Floor);
    /// assert_eq!(format!("{z:?}"), "0xc45.0");
    /// assert_eq!(o, Less);
    /// assert_eq!(integer, false);
    ///
    /// let (z, o, integer) = (&BigExponentFloat::from(3141.592653589793))
    ///     .round_to_integer_prec_round_ref(20, Ceiling);
    /// assert_eq!(format!("{z:?}"), "0xc46.0");
    /// assert_eq!(o, Greater);
    /// assert_eq!(integer, false);
    ///
    /// let (z, o, integer) = (&BigExponentFloat::from(3141.592653589793))
    ///     .round_to_integer_prec_round_ref(20, Nearest);
    /// assert_eq!(format!("{z:?}"), "0xc46.0");
    /// assert_eq!(o, Greater);
    /// assert_eq!(integer, false);
    /// ```
    #[inline]
    pub fn round_to_integer_prec_round_ref(
        &self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, bool) {
        round_to_integer_prec_round_helper(Cow::Borrowed(self), prec, rm, false)
    }

    /// Rounds a [`BigExponentFloat`] to the nearest integer representable with the specified
    /// precision, with ties to even. An [`Ordering`] comparing the result with the input is also
    /// returned, along with a `bool` indicating whether the input was an integer. The
    /// [`BigExponentFloat`] is taken by value.
    ///
    /// The result is produced by a single rounding, as described for
    /// [`BigExponentFloat::round_to_integer_prec_round`].
    ///
    /// The pair of the [`Ordering`] and the `bool` carries the same information as `mpfr_rint`'s
    /// ternary value: `(Equal, true)` means the input was an integer representable at the target
    /// precision, returned unchanged; `(Less, true)` and `(Greater, true)` mean the input was an
    /// integer that required rounding to fit the precision; `(Less, false)` and `(Greater, false)`
    /// mean the input was not an integer. `(Equal, false)` cannot occur.
    ///
    /// `NaN`s, infinities, and zero are returned unchanged with `Equal`; of these, only zero is
    /// considered an integer.
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
    /// let (z, o, integer) = BigExponentFloat::from(3141.592653589793).round_to_integer_prec(5);
    /// assert_eq!(format!("{z:?}"), "0xc.8E+2");
    /// assert_eq!(o, Greater);
    /// assert_eq!(integer, false);
    ///
    /// let (z, o, integer) = BigExponentFloat::from(3141.592653589793).round_to_integer_prec(20);
    /// assert_eq!(format!("{z:?}"), "0xc46.0");
    /// assert_eq!(o, Greater);
    /// assert_eq!(integer, false);
    /// ```
    #[inline]
    pub fn round_to_integer_prec(self, prec: u64) -> (Self, Ordering, bool) {
        round_to_integer_prec_round_helper(Cow::Owned(self), prec, Nearest, false)
    }

    /// Rounds a [`BigExponentFloat`] to the nearest integer representable with the specified
    /// precision, with ties to even. An [`Ordering`] comparing the result with the input is also
    /// returned, along with a `bool` indicating whether the input was an integer. The
    /// [`BigExponentFloat`] is taken by reference.
    ///
    /// The result is produced by a single rounding, as described for
    /// [`BigExponentFloat::round_to_integer_prec_round`].
    ///
    /// The pair of the [`Ordering`] and the `bool` carries the same information as `mpfr_rint`'s
    /// ternary value: `(Equal, true)` means the input was an integer representable at the target
    /// precision, returned unchanged; `(Less, true)` and `(Greater, true)` mean the input was an
    /// integer that required rounding to fit the precision; `(Less, false)` and `(Greater, false)`
    /// mean the input was not an integer. `(Equal, false)` cannot occur.
    ///
    /// `NaN`s, infinities, and zero are returned unchanged with `Equal`; of these, only zero is
    /// considered an integer.
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
    /// let (z, o, integer) =
    ///     (&BigExponentFloat::from(3141.592653589793)).round_to_integer_prec_ref(5);
    /// assert_eq!(format!("{z:?}"), "0xc.8E+2");
    /// assert_eq!(o, Greater);
    /// assert_eq!(integer, false);
    ///
    /// let (z, o, integer) =
    ///     (&BigExponentFloat::from(3141.592653589793)).round_to_integer_prec_ref(20);
    /// assert_eq!(format!("{z:?}"), "0xc46.0");
    /// assert_eq!(o, Greater);
    /// assert_eq!(integer, false);
    /// ```
    #[inline]
    pub fn round_to_integer_prec_ref(&self, prec: u64) -> (Self, Ordering, bool) {
        round_to_integer_prec_round_helper(Cow::Borrowed(self), prec, Nearest, false)
    }

    /// Rounds a [`BigExponentFloat`] to an integer, in the direction given by the specified
    /// rounding mode. The integer is exact: since a [`BigExponentFloat`] has no fixed precision,
    /// any integer can be returned. An [`Ordering`] comparing the result with the input is also
    /// returned, along with a `bool` indicating whether the input was an integer. The
    /// [`BigExponentFloat`] is taken by value.
    ///
    /// `Floor` and `Ceiling` are the floor and ceiling functions, `Down` is truncation, `Up` rounds
    /// away from zero, and `Nearest` rounds to the nearest integer with ties to even. `Exact` is
    /// not allowed.
    ///
    /// The [`Ordering`] compares the integer with the input, and the `bool` tells whether the input
    /// was an integer; `(Equal, true)` means that it was, and `(Equal, false)` cannot occur.
    ///
    /// `NaN`s, infinities, and zero are returned unchanged with `Equal`; of these, only zero is
    /// considered an integer.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (z, o, integer) = BigExponentFloat::from(-2.5).round_to_integer_round(Floor);
    /// assert_eq!(format!("{z:?}"), "-0x3.0");
    /// assert_eq!(o, Less);
    /// assert_eq!(integer, false);
    ///
    /// let (z, o, integer) = BigExponentFloat::from(-2.5).round_to_integer_round(Ceiling);
    /// assert_eq!(format!("{z:?}"), "-0x2.0");
    /// assert_eq!(o, Greater);
    /// assert_eq!(integer, false);
    ///
    /// let (z, o, integer) = BigExponentFloat::from(-2.5).round_to_integer_round(Down);
    /// assert_eq!(format!("{z:?}"), "-0x2.0");
    /// assert_eq!(o, Greater);
    /// assert_eq!(integer, false);
    ///
    /// let (z, o, integer) = BigExponentFloat::from(-2.5).round_to_integer_round(Up);
    /// assert_eq!(format!("{z:?}"), "-0x3.0");
    /// assert_eq!(o, Less);
    /// assert_eq!(integer, false);
    ///
    /// let (z, o, integer) = BigExponentFloat::from(-2.5).round_to_integer_round(Nearest);
    /// assert_eq!(format!("{z:?}"), "-0x2.0");
    /// assert_eq!(o, Greater);
    /// assert_eq!(integer, false);
    /// ```
    #[inline]
    pub fn round_to_integer_round(self, rm: RoundingMode) -> (Self, Ordering, bool) {
        round_to_integer_exact(Cow::Owned(self), rm, false)
    }

    /// Rounds a [`BigExponentFloat`] to an integer, in the direction given by the specified
    /// rounding mode. The integer is exact: since a [`BigExponentFloat`] has no fixed precision,
    /// any integer can be returned. An [`Ordering`] comparing the result with the input is also
    /// returned, along with a `bool` indicating whether the input was an integer. The
    /// [`BigExponentFloat`] is taken by reference.
    ///
    /// `Floor` and `Ceiling` are the floor and ceiling functions, `Down` is truncation, `Up` rounds
    /// away from zero, and `Nearest` rounds to the nearest integer with ties to even. `Exact` is
    /// not allowed.
    ///
    /// The [`Ordering`] compares the integer with the input, and the `bool` tells whether the input
    /// was an integer; `(Equal, true)` means that it was, and `(Equal, false)` cannot occur.
    ///
    /// `NaN`s, infinities, and zero are returned unchanged with `Equal`; of these, only zero is
    /// considered an integer.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (z, o, integer) = (&BigExponentFloat::from(-2.5)).round_to_integer_round_ref(Floor);
    /// assert_eq!(format!("{z:?}"), "-0x3.0");
    /// assert_eq!(o, Less);
    /// assert_eq!(integer, false);
    ///
    /// let (z, o, integer) = (&BigExponentFloat::from(-2.5)).round_to_integer_round_ref(Ceiling);
    /// assert_eq!(format!("{z:?}"), "-0x2.0");
    /// assert_eq!(o, Greater);
    /// assert_eq!(integer, false);
    ///
    /// let (z, o, integer) = (&BigExponentFloat::from(-2.5)).round_to_integer_round_ref(Down);
    /// assert_eq!(format!("{z:?}"), "-0x2.0");
    /// assert_eq!(o, Greater);
    /// assert_eq!(integer, false);
    ///
    /// let (z, o, integer) = (&BigExponentFloat::from(-2.5)).round_to_integer_round_ref(Up);
    /// assert_eq!(format!("{z:?}"), "-0x3.0");
    /// assert_eq!(o, Less);
    /// assert_eq!(integer, false);
    ///
    /// let (z, o, integer) = (&BigExponentFloat::from(-2.5)).round_to_integer_round_ref(Nearest);
    /// assert_eq!(format!("{z:?}"), "-0x2.0");
    /// assert_eq!(o, Greater);
    /// assert_eq!(integer, false);
    /// ```
    #[inline]
    pub fn round_to_integer_round_ref(&self, rm: RoundingMode) -> (Self, Ordering, bool) {
        round_to_integer_exact(Cow::Borrowed(self), rm, false)
    }

    /// Rounds a [`BigExponentFloat`] to the nearest integer, with ties to even. The integer is
    /// exact. An [`Ordering`] comparing the result with the input is also returned, along with a
    /// `bool` indicating whether the input was an integer. The [`BigExponentFloat`] is taken by
    /// value.
    ///
    /// The [`Ordering`] compares the integer with the input, and the `bool` tells whether the input
    /// was an integer; `(Equal, true)` means that it was, and `(Equal, false)` cannot occur.
    ///
    /// `NaN`s, infinities, and zero are returned unchanged with `Equal`; of these, only zero is
    /// considered an integer.
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
    /// use std::cmp::Ordering::*;
    ///
    /// let (z, o, integer) = BigExponentFloat::from(2.5).round_to_integer();
    /// assert_eq!(format!("{z:?}"), "0x2.0");
    /// assert_eq!(o, Less);
    /// assert_eq!(integer, false);
    /// ```
    #[inline]
    pub fn round_to_integer(self) -> (Self, Ordering, bool) {
        round_to_integer_exact(Cow::Owned(self), Nearest, false)
    }

    /// Rounds a [`BigExponentFloat`] to the nearest integer, with ties to even. The integer is
    /// exact. An [`Ordering`] comparing the result with the input is also returned, along with a
    /// `bool` indicating whether the input was an integer. The [`BigExponentFloat`] is taken by
    /// reference.
    ///
    /// The [`Ordering`] compares the integer with the input, and the `bool` tells whether the input
    /// was an integer; `(Equal, true)` means that it was, and `(Equal, false)` cannot occur.
    ///
    /// `NaN`s, infinities, and zero are returned unchanged with `Equal`; of these, only zero is
    /// considered an integer.
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
    /// use std::cmp::Ordering::*;
    ///
    /// let (z, o, integer) = (&BigExponentFloat::from(2.5)).round_to_integer_ref();
    /// assert_eq!(format!("{z:?}"), "0x2.0");
    /// assert_eq!(o, Less);
    /// assert_eq!(integer, false);
    /// ```
    #[inline]
    pub fn round_to_integer_ref(&self) -> (Self, Ordering, bool) {
        round_to_integer_exact(Cow::Borrowed(self), Nearest, false)
    }

    /// Rounds a [`BigExponentFloat`] to the nearest integer representable with the specified
    /// precision, with ties away from zero. An [`Ordering`] comparing the result with the input is
    /// also returned, along with a `bool` indicating whether the input was an integer. The
    /// [`BigExponentFloat`] is taken by value.
    ///
    /// The result is produced by a single rounding, as described for
    /// [`BigExponentFloat::round_to_integer_prec_round`].
    ///
    /// The pair of the [`Ordering`] and the `bool` carries the same information as `mpfr_rint`'s
    /// ternary value: `(Equal, true)` means the input was an integer representable at the target
    /// precision, returned unchanged; `(Less, true)` and `(Greater, true)` mean the input was an
    /// integer that required rounding to fit the precision; `(Less, false)` and `(Greater, false)`
    /// mean the input was not an integer. `(Equal, false)` cannot occur.
    ///
    /// `NaN`s, infinities, and zero are returned unchanged with `Equal`; of these, only zero is
    /// considered an integer.
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
    /// This is MPFR's `mpfr_round`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (z, o, integer) = BigExponentFloat::from(3141.5).round_to_integer_ties_away_prec(5);
    /// assert_eq!(format!("{z:?}"), "0xc.8E+2");
    /// assert_eq!(o, Greater);
    /// assert_eq!(integer, false);
    ///
    /// let (z, o, integer) = BigExponentFloat::from(3141.5).round_to_integer_ties_away_prec(20);
    /// assert_eq!(format!("{z:?}"), "0xc46.0");
    /// assert_eq!(o, Greater);
    /// assert_eq!(integer, false);
    /// ```
    #[inline]
    pub fn round_to_integer_ties_away_prec(self, prec: u64) -> (Self, Ordering, bool) {
        round_to_integer_prec_round_helper(Cow::Owned(self), prec, Nearest, true)
    }

    /// Rounds a [`BigExponentFloat`] to the nearest integer representable with the specified
    /// precision, with ties away from zero. An [`Ordering`] comparing the result with the input is
    /// also returned, along with a `bool` indicating whether the input was an integer. The
    /// [`BigExponentFloat`] is taken by reference.
    ///
    /// The result is produced by a single rounding, as described for
    /// [`BigExponentFloat::round_to_integer_prec_round`].
    ///
    /// The pair of the [`Ordering`] and the `bool` carries the same information as `mpfr_rint`'s
    /// ternary value: `(Equal, true)` means the input was an integer representable at the target
    /// precision, returned unchanged; `(Less, true)` and `(Greater, true)` mean the input was an
    /// integer that required rounding to fit the precision; `(Less, false)` and `(Greater, false)`
    /// mean the input was not an integer. `(Equal, false)` cannot occur.
    ///
    /// `NaN`s, infinities, and zero are returned unchanged with `Equal`; of these, only zero is
    /// considered an integer.
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
    /// let (z, o, integer) =
    ///     (&BigExponentFloat::from(3141.5)).round_to_integer_ties_away_prec_ref(5);
    /// assert_eq!(format!("{z:?}"), "0xc.8E+2");
    /// assert_eq!(o, Greater);
    /// assert_eq!(integer, false);
    ///
    /// let (z, o, integer) =
    ///     (&BigExponentFloat::from(3141.5)).round_to_integer_ties_away_prec_ref(20);
    /// assert_eq!(format!("{z:?}"), "0xc46.0");
    /// assert_eq!(o, Greater);
    /// assert_eq!(integer, false);
    /// ```
    #[inline]
    pub fn round_to_integer_ties_away_prec_ref(&self, prec: u64) -> (Self, Ordering, bool) {
        round_to_integer_prec_round_helper(Cow::Borrowed(self), prec, Nearest, true)
    }

    /// Rounds a [`BigExponentFloat`] to the nearest integer, with ties away from zero. The integer
    /// is exact. An [`Ordering`] comparing the result with the input is also returned, along with a
    /// `bool` indicating whether the input was an integer. The [`BigExponentFloat`] is taken by
    /// value.
    ///
    /// The [`Ordering`] compares the integer with the input, and the `bool` tells whether the input
    /// was an integer; `(Equal, true)` means that it was, and `(Equal, false)` cannot occur.
    ///
    /// `NaN`s, infinities, and zero are returned unchanged with `Equal`; of these, only zero is
    /// considered an integer.
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
    /// use std::cmp::Ordering::*;
    ///
    /// let (z, o, integer) = BigExponentFloat::from(2.5).round_to_integer_ties_away();
    /// assert_eq!(format!("{z:?}"), "0x3.0");
    /// assert_eq!(o, Greater);
    /// assert_eq!(integer, false);
    /// ```
    #[inline]
    pub fn round_to_integer_ties_away(self) -> (Self, Ordering, bool) {
        round_to_integer_exact(Cow::Owned(self), Nearest, true)
    }

    /// Rounds a [`BigExponentFloat`] to the nearest integer, with ties away from zero. The integer
    /// is exact. An [`Ordering`] comparing the result with the input is also returned, along with a
    /// `bool` indicating whether the input was an integer. The [`BigExponentFloat`] is taken by
    /// reference.
    ///
    /// The [`Ordering`] compares the integer with the input, and the `bool` tells whether the input
    /// was an integer; `(Equal, true)` means that it was, and `(Equal, false)` cannot occur.
    ///
    /// `NaN`s, infinities, and zero are returned unchanged with `Equal`; of these, only zero is
    /// considered an integer.
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
    /// use std::cmp::Ordering::*;
    ///
    /// let (z, o, integer) = (&BigExponentFloat::from(2.5)).round_to_integer_ties_away_ref();
    /// assert_eq!(format!("{z:?}"), "0x3.0");
    /// assert_eq!(o, Greater);
    /// assert_eq!(integer, false);
    /// ```
    #[inline]
    pub fn round_to_integer_ties_away_ref(&self) -> (Self, Ordering, bool) {
        round_to_integer_exact(Cow::Borrowed(self), Nearest, true)
    }

    /// Rounds a [`BigExponentFloat`] to an integer in the direction given by `irm`, and then rounds
    /// that integer to the specified precision with the rounding mode `rm`. An [`Ordering`]
    /// comparing the final result with the integer is also returned. The [`BigExponentFloat`] is
    /// taken by value.
    ///
    /// Unlike [`BigExponentFloat::round_to_integer_prec_round`], which rounds once, this function
    /// is the composition of two roundings, matching MPFR's `mpfr_rint_`-prefixed functions: the
    /// exact integer is formed first, then rounded to the target precision. The two can differ:
    /// under this function with both modes `Nearest`, $10.5$ becomes $10$, which then rounds to $8$
    /// at a precision of 2 bits, while the single-rounding form gives $12$.
    ///
    /// `NaN`s and infinities are returned unchanged with `Equal`.
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
    /// Panics if `prec` is zero, if `irm` is `Exact`, or if `rm` is `Exact` and the integer is not
    /// exactly representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (z, o) = BigExponentFloat::from(3141.592653589793)
    ///     .round_to_integer_then_prec_round(Nearest, 5, Floor);
    /// assert_eq!(format!("{z:?}"), "0xc.0E+2");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = BigExponentFloat::from(3141.592653589793)
    ///     .round_to_integer_then_prec_round(Nearest, 5, Ceiling);
    /// assert_eq!(format!("{z:?}"), "0xc.8E+2");
    /// assert_eq!(o, Greater);
    ///
    /// let (z, o) = BigExponentFloat::from(3141.592653589793)
    ///     .round_to_integer_then_prec_round(Nearest, 5, Nearest);
    /// assert_eq!(format!("{z:?}"), "0xc.8E+2");
    /// assert_eq!(o, Greater);
    ///
    /// let (z, o) = BigExponentFloat::from(3141.592653589793)
    ///     .round_to_integer_then_prec_round(Nearest, 20, Floor);
    /// assert_eq!(format!("{z:?}"), "0xc46.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (z, o) = BigExponentFloat::from(3141.592653589793)
    ///     .round_to_integer_then_prec_round(Nearest, 20, Ceiling);
    /// assert_eq!(format!("{z:?}"), "0xc46.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (z, o) = BigExponentFloat::from(3141.592653589793)
    ///     .round_to_integer_then_prec_round(Nearest, 20, Nearest);
    /// assert_eq!(format!("{z:?}"), "0xc46.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn round_to_integer_then_prec_round(
        self,
        irm: RoundingMode,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        round_to_integer_then_prec_round_helper(Cow::Owned(self), irm, false, prec, rm)
    }

    /// Rounds a [`BigExponentFloat`] to an integer in the direction given by `irm`, and then rounds
    /// that integer to the specified precision with the rounding mode `rm`. An [`Ordering`]
    /// comparing the final result with the integer is also returned. The [`BigExponentFloat`] is
    /// taken by reference.
    ///
    /// Unlike [`BigExponentFloat::round_to_integer_prec_round`], which rounds once, this function
    /// is the composition of two roundings, matching MPFR's `mpfr_rint_`-prefixed functions: the
    /// exact integer is formed first, then rounded to the target precision. The two can differ:
    /// under this function with both modes `Nearest`, $10.5$ becomes $10$, which then rounds to $8$
    /// at a precision of 2 bits, while the single-rounding form gives $12$.
    ///
    /// `NaN`s and infinities are returned unchanged with `Equal`.
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
    /// Panics if `prec` is zero, if `irm` is `Exact`, or if `rm` is `Exact` and the integer is not
    /// exactly representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (z, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .round_to_integer_then_prec_round_ref(Nearest, 5, Floor);
    /// assert_eq!(format!("{z:?}"), "0xc.0E+2");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .round_to_integer_then_prec_round_ref(Nearest, 5, Ceiling);
    /// assert_eq!(format!("{z:?}"), "0xc.8E+2");
    /// assert_eq!(o, Greater);
    ///
    /// let (z, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .round_to_integer_then_prec_round_ref(Nearest, 5, Nearest);
    /// assert_eq!(format!("{z:?}"), "0xc.8E+2");
    /// assert_eq!(o, Greater);
    ///
    /// let (z, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .round_to_integer_then_prec_round_ref(Nearest, 20, Floor);
    /// assert_eq!(format!("{z:?}"), "0xc46.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (z, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .round_to_integer_then_prec_round_ref(Nearest, 20, Ceiling);
    /// assert_eq!(format!("{z:?}"), "0xc46.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (z, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .round_to_integer_then_prec_round_ref(Nearest, 20, Nearest);
    /// assert_eq!(format!("{z:?}"), "0xc46.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn round_to_integer_then_prec_round_ref(
        &self,
        irm: RoundingMode,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        round_to_integer_then_prec_round_helper(Cow::Borrowed(self), irm, false, prec, rm)
    }

    /// Rounds a [`BigExponentFloat`] to the nearest integer, with ties away from zero, and then
    /// rounds that integer to the specified precision with the specified rounding mode. An
    /// [`Ordering`] comparing the final result with the integer is also returned. The
    /// [`BigExponentFloat`] is taken by value.
    ///
    /// This is the composition of two roundings, as described for
    /// [`BigExponentFloat::round_to_integer_then_prec_round`]; it is MPFR's `mpfr_rint_round`.
    ///
    /// `NaN`s and infinities are returned unchanged with `Equal`.
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
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the integer is not exactly representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (z, o) =
    ///     BigExponentFloat::from(3141.5).round_to_integer_ties_away_then_prec_round(5, Floor);
    /// assert_eq!(format!("{z:?}"), "0xc.0E+2");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) =
    ///     BigExponentFloat::from(3141.5).round_to_integer_ties_away_then_prec_round(5, Ceiling);
    /// assert_eq!(format!("{z:?}"), "0xc.8E+2");
    /// assert_eq!(o, Greater);
    ///
    /// let (z, o) =
    ///     BigExponentFloat::from(3141.5).round_to_integer_ties_away_then_prec_round(5, Nearest);
    /// assert_eq!(format!("{z:?}"), "0xc.8E+2");
    /// assert_eq!(o, Greater);
    ///
    /// let (z, o) =
    ///     BigExponentFloat::from(3141.5).round_to_integer_ties_away_then_prec_round(20, Floor);
    /// assert_eq!(format!("{z:?}"), "0xc46.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (z, o) =
    ///     BigExponentFloat::from(3141.5).round_to_integer_ties_away_then_prec_round(20, Ceiling);
    /// assert_eq!(format!("{z:?}"), "0xc46.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (z, o) =
    ///     BigExponentFloat::from(3141.5).round_to_integer_ties_away_then_prec_round(20, Nearest);
    /// assert_eq!(format!("{z:?}"), "0xc46.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn round_to_integer_ties_away_then_prec_round(
        self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        round_to_integer_then_prec_round_helper(Cow::Owned(self), Nearest, true, prec, rm)
    }

    /// Rounds a [`BigExponentFloat`] to the nearest integer, with ties away from zero, and then
    /// rounds that integer to the specified precision with the specified rounding mode. An
    /// [`Ordering`] comparing the final result with the integer is also returned. The
    /// [`BigExponentFloat`] is taken by reference.
    ///
    /// This is the composition of two roundings, as described for
    /// [`BigExponentFloat::round_to_integer_then_prec_round`]; it is MPFR's `mpfr_rint_round`.
    ///
    /// `NaN`s and infinities are returned unchanged with `Equal`.
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
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the integer is not exactly representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (z, o) = (&BigExponentFloat::from(3141.5))
    ///     .round_to_integer_ties_away_then_prec_round_ref(5, Floor);
    /// assert_eq!(format!("{z:?}"), "0xc.0E+2");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = (&BigExponentFloat::from(3141.5))
    ///     .round_to_integer_ties_away_then_prec_round_ref(5, Ceiling);
    /// assert_eq!(format!("{z:?}"), "0xc.8E+2");
    /// assert_eq!(o, Greater);
    ///
    /// let (z, o) = (&BigExponentFloat::from(3141.5))
    ///     .round_to_integer_ties_away_then_prec_round_ref(5, Nearest);
    /// assert_eq!(format!("{z:?}"), "0xc.8E+2");
    /// assert_eq!(o, Greater);
    ///
    /// let (z, o) = (&BigExponentFloat::from(3141.5))
    ///     .round_to_integer_ties_away_then_prec_round_ref(20, Floor);
    /// assert_eq!(format!("{z:?}"), "0xc46.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (z, o) = (&BigExponentFloat::from(3141.5))
    ///     .round_to_integer_ties_away_then_prec_round_ref(20, Ceiling);
    /// assert_eq!(format!("{z:?}"), "0xc46.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (z, o) = (&BigExponentFloat::from(3141.5))
    ///     .round_to_integer_ties_away_then_prec_round_ref(20, Nearest);
    /// assert_eq!(format!("{z:?}"), "0xc46.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn round_to_integer_ties_away_then_prec_round_ref(
        &self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        round_to_integer_then_prec_round_helper(Cow::Borrowed(self), Nearest, true, prec, rm)
    }
}
