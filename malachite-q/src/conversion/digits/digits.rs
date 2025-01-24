// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::conversion::digits::power_of_2_digits::RationalPowerOf2Digits;
use crate::Rational;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{Abs, CheckedLogBase2, Floor, UnsignedAbs};
use malachite_base::num::conversion::traits::Digits;
use malachite_nz::natural::Natural;

#[doc(hidden)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RationalGeneralDigits {
    base: Rational,
    remainder: Rational,
}

impl Iterator for RationalGeneralDigits {
    type Item = Natural;

    fn next(&mut self) -> Option<Natural> {
        if self.remainder == 0u32 {
            None
        } else {
            self.remainder *= &self.base;
            let digit = (&self.remainder).floor().unsigned_abs();
            self.remainder -= Rational::from(&digit);
            Some(digit)
        }
    }
}

/// Represents the base-$b$ digits of the fractional portion of a [`Rational`] number.
///
/// See [`digits`](Rational::digits) for more information.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RationalDigits {
    General(RationalGeneralDigits),
    PowerOf2(RationalPowerOf2Digits),
}

impl Iterator for RationalDigits {
    type Item = Natural;

    fn next(&mut self) -> Option<Natural> {
        match self {
            RationalDigits::General(xs) => xs.next(),
            RationalDigits::PowerOf2(xs) => xs.next(),
        }
    }
}

impl Rational {
    /// Returns the base-$b$ digits of a [`Rational`].
    ///
    /// The output has two components. The first is a [`Vec`] of the digits of the integer portion
    /// of the [`Rational`], least- to most-significant. The second is an iterator of the digits of
    /// the fractional portion.
    ///
    /// The output is in its simplest form: the integer-portion digits do not end with a zero, and
    /// the fractional-portion digits do not end with infinitely many zeros or $(b-1)$s.
    ///
    /// If the [`Rational`] has a small denominator, it may be more efficient to use
    /// [`to_digits`](Rational::to_digits) or [`into_digits`](Rational::into_digits) instead. These
    /// functions compute the entire repeating portion of the repeating digits.
    ///
    /// For example, consider these two expressions:
    /// - `Rational::from_signeds(1, 7).digits(Natural::from(10u32)).1.nth(1000)`
    /// - `Rational::from_signeds(1, 7).into_digits(Natural::from(10u32)).1[1000]`
    ///
    /// Both get the thousandth digit after the decimal point of `1/7`. The first way explicitly
    /// calculates each digit after the decimal point, whereas the second way determines that `1/7`
    /// is `0.(142857)`, with the `142857` repeating, and takes `1000 % 6 == 4` to determine that
    /// the thousandth digit is 5. But when the [`Rational`] has a large denominator, the second way
    /// is less efficient.
    ///
    /// # Worst-case complexity per iteration
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// base.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `base` is less than 2.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::iterators::prefix_to_string;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    ///
    /// let (before_point, after_point) = Rational::from(3u32).digits(&Natural::from(10u32));
    /// assert_eq!(before_point.to_debug_string(), "[3]");
    /// assert_eq!(prefix_to_string(after_point, 10), "[]");
    ///
    /// let (before_point, after_point) =
    ///     Rational::from_signeds(22, 7).digits(&Natural::from(10u32));
    /// assert_eq!(before_point.to_debug_string(), "[3]");
    /// assert_eq!(
    ///     prefix_to_string(after_point, 10),
    ///     "[1, 4, 2, 8, 5, 7, 1, 4, 2, 8, ...]"
    /// );
    /// ```
    pub fn digits(&self, base: &Natural) -> (Vec<Natural>, RationalDigits) {
        if let Some(log_base) = base.checked_log_base_2() {
            let (before_point, after_point) = self.power_of_2_digits(log_base);
            (before_point, RationalDigits::PowerOf2(after_point))
        } else {
            let mut remainder = self.abs();
            let floor = (&remainder).floor().unsigned_abs();
            remainder -= Rational::from(&floor);
            (
                floor.to_digits_asc(base),
                RationalDigits::General(RationalGeneralDigits {
                    base: Rational::from(base),
                    remainder,
                }),
            )
        }
    }
}
