// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_nz::integer::Integer;

impl PartialEq<Integer> for Rational {
    /// Determines whether a [`Rational`] is equal to an [`Integer`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `min(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    /// use malachite_q::Rational;
    ///
    /// assert!(Rational::from(-123) == Integer::from(-123));
    /// assert!(Rational::from_signeds(22, 7) != Integer::from(5));
    /// ```
    fn eq(&self, other: &Integer) -> bool {
        self.sign == (*other >= 0)
            && self.denominator == 1
            && self.numerator == *other.unsigned_abs_ref()
    }
}

impl PartialEq<Rational> for Integer {
    /// Determines whether an [`Integer`] is equal to a [`Rational`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `min(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    /// use malachite_q::Rational;
    ///
    /// assert!(Integer::from(-123) == Rational::from(-123));
    /// assert!(Integer::from(5) != Rational::from_signeds(22, 7));
    /// ```
    fn eq(&self, other: &Rational) -> bool {
        other.sign == (*self >= 0)
            && other.denominator == 1
            && *self.unsigned_abs_ref() == other.numerator
    }
}
