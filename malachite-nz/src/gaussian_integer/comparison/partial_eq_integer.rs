// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use crate::integer::Integer;

impl PartialEq<Integer> for GaussianInteger {
    /// Determines whether a [`GaussianInteger`] is equal to an [`Integer`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `min(self.real.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// assert!(GaussianInteger::from(123) == Integer::from(123));
    /// assert!(GaussianInteger::from_str("123+i").unwrap() != Integer::from(123));
    /// ```
    fn eq(&self, other: &Integer) -> bool {
        self.imaginary == 0u32 && self.real == *other
    }
}

impl PartialEq<GaussianInteger> for Integer {
    /// Determines whether an [`Integer`] is equal to a [`GaussianInteger`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `min(self.significant_bits(),
    /// other.real.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// assert!(Integer::from(123) == GaussianInteger::from(123));
    /// assert!(Integer::from(123) != GaussianInteger::from_str("123+i").unwrap());
    /// ```
    fn eq(&self, other: &GaussianInteger) -> bool {
        other.imaginary == 0u32 && *self == other.real
    }
}
