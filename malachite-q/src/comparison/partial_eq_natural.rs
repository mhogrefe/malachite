// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_nz::natural::Natural;

impl PartialEq<Natural> for Rational {
    /// Determines whether a [`Rational`] is equal to a [`Natural`].
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    ///
    /// assert!(Rational::from(123) == Natural::from(123u32));
    /// assert!(Rational::from_signeds(22, 7) != Natural::from(5u32));
    /// ```
    fn eq(&self, other: &Natural) -> bool {
        self.sign && self.denominator == 1 && self.numerator == *other
    }
}

impl PartialEq<Rational> for Natural {
    /// Determines whether a [`Natural`] is equal to a [`Rational`].
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    ///
    /// assert!(Natural::from(123u32) == Rational::from(123));
    /// assert!(Natural::from(5u32) != Rational::from_signeds(22, 7));
    /// ```
    fn eq(&self, other: &Rational) -> bool {
        other.sign && other.denominator == 1 && *self == other.numerator
    }
}
