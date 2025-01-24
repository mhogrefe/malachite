// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{Abs, AbsAssign, Floor, UnsignedAbs};
use malachite_base::num::conversion::traits::PowerOf2Digits;
use malachite_base::rational_sequences::RationalSequence;
use malachite_nz::natural::Natural;

pub(crate) fn to_power_of_2_digits_helper(
    x: Rational,
    log_base: u64,
) -> (Vec<Natural>, RationalSequence<Natural>) {
    let floor = (&x).floor();
    let mut remainder = x - Rational::from(&floor);
    let before_point = floor.unsigned_abs().to_power_of_2_digits_asc(log_base);
    let mut state_map = BTreeMap::new();
    let mut digits = Vec::new();
    for i in 0.. {
        if remainder == 0u32 {
            return (before_point, RationalSequence::from_vec(digits));
        }
        if let Some(previous_i) = state_map.insert(remainder.clone(), i) {
            let repeating = digits.drain(previous_i..).collect();
            return (before_point, RationalSequence::from_vecs(digits, repeating));
        }
        remainder <<= log_base;
        let floor = (&remainder).floor().unsigned_abs();
        digits.push(floor.clone());
        remainder -= Rational::from(floor);
    }
    unreachable!()
}

impl Rational {
    /// Returns the base-$2^k$ digits of a [`Rational`], taking the [`Rational`] by value.
    ///
    /// The output has two components. The first is a [`Vec`] of the digits of the integer portion
    /// of the [`Rational`], least- to most-significant. The second is a [`RationalSequence`] of the
    /// digits of the fractional portion.
    ///
    /// The output is in its simplest form: the integer-portion digits do not end with a zero, and
    /// the fractional-portion digits do not end with infinitely many zeros or $(2^k-1)$s.
    ///
    /// The fractional portion may be very large; the length of the repeating part may be almost as
    /// large as the denominator. If the [`Rational`] has a large denominator, consider using
    /// [`power_of_2_digits`](Rational::power_of_2_digits) instead, which returns an iterator. That
    /// function computes the fractional digits lazily and doesn't need to compute the entire
    /// repeating part.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m2^n)$
    ///
    /// $M(n, m) = O(m2^n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $m$ is
    /// `log_base`.
    ///
    /// # Panics
    /// Panics if `log_base` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_q::Rational;
    ///
    /// let (before_point, after_point) = Rational::from(3u32).into_power_of_2_digits(1);
    /// assert_eq!(before_point.to_debug_string(), "[1, 1]");
    /// assert_eq!(after_point.to_string(), "[]");
    ///
    /// let (before_point, after_point) = Rational::from_signeds(22, 7).into_power_of_2_digits(1);
    /// assert_eq!(before_point.to_debug_string(), "[1, 1]");
    /// assert_eq!(after_point.to_string(), "[[0, 0, 1]]");
    ///
    /// let (before_point, after_point) = Rational::from_signeds(22, 7).into_power_of_2_digits(10);
    /// assert_eq!(before_point.to_debug_string(), "[3]");
    /// assert_eq!(after_point.to_string(), "[[146, 292, 585]]");
    /// ```
    #[inline]
    pub fn into_power_of_2_digits(
        mut self,
        log_base: u64,
    ) -> (Vec<Natural>, RationalSequence<Natural>) {
        self.abs_assign();
        to_power_of_2_digits_helper(self, log_base)
    }

    /// Returns the base-$2^k$ digits of a [`Rational`], taking the [`Rational`] by reference.
    ///
    /// The output has two components. The first is a [`Vec`] of the digits of the integer portion
    /// of the [`Rational`], least- to most-significant. The second is a [`RationalSequence`] of the
    /// digits of the fractional portion.
    ///
    /// The output is in its simplest form: the integer-portion digits do not end with a zero, and
    /// the fractional-portion digits do not end with infinitely many zeros or $(2^k-1)$s.
    ///
    /// The fractional portion may be very large; the length of the repeating part may be almost as
    /// large as the denominator. If the [`Rational`] has a large denominator, consider using
    /// [`power_of_2_digits`](Rational::power_of_2_digits) instead, which returns an iterator. That
    /// function computes the fractional digits lazily and doesn't need to compute the entire
    /// repeating part.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m2^n)$
    ///
    /// $M(n, m) = O(m2^n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $m$ is
    /// `log_base`.
    ///
    /// # Panics
    /// Panics if `log_base` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_q::Rational;
    ///
    /// let (before_point, after_point) = Rational::from(3u32).to_power_of_2_digits(1);
    /// assert_eq!(before_point.to_debug_string(), "[1, 1]");
    /// assert_eq!(after_point.to_string(), "[]");
    ///
    /// let (before_point, after_point) = Rational::from_signeds(22, 7).to_power_of_2_digits(1);
    /// assert_eq!(before_point.to_debug_string(), "[1, 1]");
    /// assert_eq!(after_point.to_string(), "[[0, 0, 1]]");
    ///
    /// let (before_point, after_point) = Rational::from_signeds(22, 7).to_power_of_2_digits(10);
    /// assert_eq!(before_point.to_debug_string(), "[3]");
    /// assert_eq!(after_point.to_string(), "[[146, 292, 585]]");
    /// ```
    pub fn to_power_of_2_digits(&self, log_base: u64) -> (Vec<Natural>, RationalSequence<Natural>) {
        to_power_of_2_digits_helper(self.abs(), log_base)
    }
}
