// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::conversion::digits::to_power_of_2_digits::to_power_of_2_digits_helper;
use crate::Rational;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{
    Abs, AbsAssign, CheckedLogBase2, Floor, UnsignedAbs,
};
use malachite_base::num::conversion::traits::Digits;
use malachite_base::rational_sequences::RationalSequence;
use malachite_nz::natural::Natural;

fn to_digits_helper(x: Rational, base: &Natural) -> (Vec<Natural>, RationalSequence<Natural>) {
    if let Some(log_base) = base.checked_log_base_2() {
        return to_power_of_2_digits_helper(x, log_base);
    }
    let floor = (&x).floor();
    let mut remainder = x - Rational::from(&floor);
    let before_point = floor.unsigned_abs().to_digits_asc(base);
    let mut state_map = BTreeMap::new();
    let mut digits = Vec::new();
    let base = Rational::from(base);
    for i in 0.. {
        if remainder == 0u32 {
            return (before_point, RationalSequence::from_vec(digits));
        }
        if let Some(previous_i) = state_map.insert(remainder.clone(), i) {
            let repeating = digits.drain(previous_i..).collect();
            return (before_point, RationalSequence::from_vecs(digits, repeating));
        }
        remainder *= &base;
        let floor = (&remainder).floor().unsigned_abs();
        digits.push(floor.clone());
        remainder -= Rational::from(floor);
    }
    unreachable!()
}

impl Rational {
    /// Returns the base-$b$ digits of a [`Rational`], taking the [`Rational`] by value.
    ///
    /// The output has two components. The first is a [`Vec`] of the digits of the integer portion
    /// of the [`Rational`], least- to most-significant. The second is a [`RationalSequence`] of the
    /// digits of the fractional portion.
    ///
    /// The output is in its simplest form: the integer-portion digits do not end with a zero, and
    /// the fractional-portion digits do not end with infinitely many zeros or $(b-1)$s.
    ///
    /// The fractional portion may be very large; the length of the repeating part may be almost as
    /// large as the denominator. If the [`Rational`] has a large denominator, consider using
    /// [`digits`](Rational::digits) instead, which returns an iterator. That function computes the
    /// fractional digits lazily and doesn't need to compute the entire repeating part.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m2^n)$
    ///
    /// $M(n, m) = O(m2^n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $m$ is
    /// `base.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `base` is less than 2.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    ///
    /// let (before_point, after_point) = Rational::from(3u32).into_digits(&Natural::from(10u32));
    /// assert_eq!(before_point.to_debug_string(), "[3]");
    /// assert_eq!(after_point.to_string(), "[]");
    ///
    /// let (before_point, after_point) =
    ///     Rational::from_signeds(22, 7).into_digits(&Natural::from(10u32));
    /// assert_eq!(before_point.to_debug_string(), "[3]");
    /// assert_eq!(after_point.to_string(), "[[1, 4, 2, 8, 5, 7]]");
    /// ```
    #[inline]
    pub fn into_digits(mut self, base: &Natural) -> (Vec<Natural>, RationalSequence<Natural>) {
        self.abs_assign();
        to_digits_helper(self, base)
    }

    /// Returns the base-$b$ digits of a [`Rational`], taking the [`Rational`] by reference.
    ///
    /// The output has two components. The first is a [`Vec`] of the digits of the integer portion
    /// of the [`Rational`], least- to most-significant. The second is a [`RationalSequence`] of the
    /// digits of the fractional portion.
    ///
    /// The output is in its simplest form: the integer-portion digits do not end with a zero, and
    /// the fractional-portion digits do not end with infinitely many zeros or $(b-1)$s.
    ///
    /// The fractional portion may be very large; the length of the repeating part may be almost as
    /// large as the denominator. If the [`Rational`] has a large denominator, consider using
    /// [`digits`](Rational::digits) instead, which returns an iterator. That function computes the
    /// fractional digits lazily and doesn't need to compute the entire repeating part.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m2^n)$
    ///
    /// $M(n, m) = O(m2^n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $m$ is
    /// `base.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `base` is less than 2.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    ///
    /// let (before_point, after_point) = Rational::from(3u32).to_digits(&Natural::from(10u32));
    /// assert_eq!(before_point.to_debug_string(), "[3]");
    /// assert_eq!(after_point.to_string(), "[]");
    ///
    /// let (before_point, after_point) =
    ///     Rational::from_signeds(22, 7).to_digits(&Natural::from(10u32));
    /// assert_eq!(before_point.to_debug_string(), "[3]");
    /// assert_eq!(after_point.to_string(), "[[1, 4, 2, 8, 5, 7]]");
    /// ```
    pub fn to_digits(&self, base: &Natural) -> (Vec<Natural>, RationalSequence<Natural>) {
        to_digits_helper(self.abs(), base)
    }
}
