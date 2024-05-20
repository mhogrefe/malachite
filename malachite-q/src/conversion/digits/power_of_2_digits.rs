// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{Abs, Floor, UnsignedAbs};
use malachite_base::num::conversion::traits::PowerOf2Digits;
use malachite_nz::natural::Natural;

/// Represents the base-$2^k$ digits of the fractional portion of a [`Rational`] number.
///
/// See [`power_of_2_digits`](Rational::power_of_2_digits) for more information.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RationalPowerOf2Digits {
    log_base: u64,
    remainder: Rational,
}

impl Iterator for RationalPowerOf2Digits {
    type Item = Natural;

    fn next(&mut self) -> Option<Natural> {
        if self.remainder == 0u32 {
            None
        } else {
            self.remainder <<= self.log_base;
            let digit = (&self.remainder).floor().unsigned_abs();
            self.remainder -= Rational::from(&digit);
            Some(digit)
        }
    }
}

impl Rational {
    /// Returns the base-$2^k$ digits of a [`Rational`].
    ///
    /// The output has two components. The first is a [`Vec`] of the digits of the integer portion
    /// of the [`Rational`], least- to most-significant. The second is an iterator of the digits of
    /// the fractional portion.
    ///
    /// The output is in its simplest form: the integer-portion digits do not end with a zero, and
    /// the fractional-portion digits do not end with infinitely many zeros or $(2^k-1)$s.
    ///
    /// If the [`Rational`] has a small denominator, it may be more efficient to use
    /// [`to_power_of_2_digits`](Rational::to_power_of_2_digits) or
    /// [`into_power_of_2_digits`](Rational::into_power_of_2_digits) instead. These functions
    /// compute the entire repeating portion of the repeating digits.
    ///
    /// For example, consider these two expressions:
    /// - `Rational::from_signeds(1, 7).power_of_2_digits(1).1.nth(1000)`
    /// - `Rational::from_signeds(1, 7).into_power_of_2_digits(1).1[1000]`
    ///
    /// Both get the thousandth digit after the binary point of `1/7`. The first way explicitly
    /// calculates each bit after the binary point, whereas the second way determines that `1/7` is
    /// `0.(001)`, with the `001` repeating, and takes `1000 % 3 == 1` to determine that the
    /// thousandth bit is 0. But when the [`Rational`] has a large denominator, the second way is
    /// less efficient.
    ///
    /// # Worst-case complexity per iteration
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// base)`.
    ///
    /// # Panics
    /// Panics if `log_base` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::iterators::prefix_to_string;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_q::Rational;
    ///
    /// let (before_point, after_point) = Rational::from(3u32).power_of_2_digits(1);
    /// assert_eq!(before_point.to_debug_string(), "[1, 1]");
    /// assert_eq!(prefix_to_string(after_point, 10), "[]");
    ///
    /// let (before_point, after_point) = Rational::from_signeds(22, 7).power_of_2_digits(1);
    /// assert_eq!(before_point.to_debug_string(), "[1, 1]");
    /// assert_eq!(
    ///     prefix_to_string(after_point, 10),
    ///     "[0, 0, 1, 0, 0, 1, 0, 0, 1, 0, ...]"
    /// );
    ///
    /// let (before_point, after_point) = Rational::from_signeds(22, 7).power_of_2_digits(10);
    /// assert_eq!(before_point.to_debug_string(), "[3]");
    /// assert_eq!(
    ///     prefix_to_string(after_point, 10),
    ///     "[146, 292, 585, 146, 292, 585, 146, 292, 585, 146, ...]"
    /// );
    /// ```
    pub fn power_of_2_digits(&self, log_base: u64) -> (Vec<Natural>, RationalPowerOf2Digits) {
        let mut remainder = self.abs();
        let floor = (&remainder).floor().unsigned_abs();
        remainder -= Rational::from(&floor);
        (
            floor.to_power_of_2_digits_asc(log_base),
            RationalPowerOf2Digits {
                log_base,
                remainder,
            },
        )
    }
}
