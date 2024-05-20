// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
use core::cmp::Ordering::{self, *};

impl PartialOrd<Natural> for Integer {
    /// Compares an [`Integer`] to a [`Natural`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where n = `min(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert!(Integer::from(123) > Natural::from(122u32));
    /// assert!(Integer::from(123) >= Natural::from(122u32));
    /// assert!(Integer::from(123) < Natural::from(124u32));
    /// assert!(Integer::from(123) <= Natural::from(124u32));
    /// assert!(Integer::from(-123) < Natural::from(123u32));
    /// assert!(Integer::from(-123) <= Natural::from(123u32));
    /// ```
    fn partial_cmp(&self, other: &Natural) -> Option<Ordering> {
        if self.sign {
            self.abs.partial_cmp(other)
        } else {
            Some(Less)
        }
    }
}

impl PartialOrd<Integer> for Natural {
    /// Compares a [`Natural`] to an [`Integer`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where n = `min(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert!(Natural::from(123u32) > Integer::from(122));
    /// assert!(Natural::from(123u32) >= Integer::from(122));
    /// assert!(Natural::from(123u32) < Integer::from(124));
    /// assert!(Natural::from(123u32) <= Integer::from(124));
    /// assert!(Natural::from(123u32) > Integer::from(-123));
    /// assert!(Natural::from(123u32) >= Integer::from(-123));
    /// ```
    fn partial_cmp(&self, other: &Integer) -> Option<Ordering> {
        if other.sign {
            self.partial_cmp(&other.abs)
        } else {
            Some(Greater)
        }
    }
}
