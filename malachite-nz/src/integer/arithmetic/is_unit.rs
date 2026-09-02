// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use malachite_base::num::arithmetic::traits::IsUnit;

impl IsUnit for Integer {
    /// Determines whether a [`Integer`] is a unit: whether it is 1 or $-1$, the only integers with
    /// a multiplicative inverse.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::IsUnit;
    /// use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::ONE.is_unit(), true);
    /// assert_eq!(Integer::NEGATIVE_ONE.is_unit(), true);
    /// assert_eq!(Integer::ZERO.is_unit(), false);
    /// assert_eq!(Integer::from(-123).is_unit(), false);
    /// ```
    #[inline]
    fn is_unit(&self) -> bool {
        self.abs == 1u32
    }
}
