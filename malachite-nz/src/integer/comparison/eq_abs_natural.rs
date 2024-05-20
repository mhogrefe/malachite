// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
use malachite_base::num::comparison::traits::EqAbs;

impl EqAbs<Natural> for Integer {
    /// Determines whether the absolute values of an [`Integer`] and a [`Natural`] are equal.
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
    /// use malachite_base::num::comparison::traits::EqAbs;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Integer::from(-123).eq_abs(&Natural::from(122u32)), false);
    /// assert_eq!(Integer::from(-123).eq_abs(&Natural::from(124u32)), false);
    /// assert_eq!(Integer::from(123).eq_abs(&Natural::from(123u32)), true);
    /// assert_eq!(Integer::from(-123).eq_abs(&Natural::from(123u32)), true);
    /// ```
    #[inline]
    fn eq_abs(&self, other: &Natural) -> bool {
        self.abs == *other
    }
}

impl EqAbs<Integer> for Natural {
    /// Determines whether the absolute values of an [`Integer`] and a [`Natural`] are equal.
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
    /// use malachite_base::num::comparison::traits::EqAbs;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(122u32).eq_abs(&Integer::from(-123)), false);
    /// assert_eq!(Natural::from(124u32).eq_abs(&Integer::from(-123)), false);
    /// assert_eq!(Natural::from(123u32).eq_abs(&Integer::from(123)), true);
    /// assert_eq!(Natural::from(123u32).eq_abs(&Integer::from(-123)), true);
    /// ```
    #[inline]
    fn eq_abs(&self, other: &Integer) -> bool {
        *self == other.abs
    }
}
