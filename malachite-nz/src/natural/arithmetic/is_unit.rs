// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::IsUnit;

impl IsUnit for Natural {
    /// Determines whether a [`Natural`] is a unit: whether it is 1, the only natural number with a
    /// multiplicative inverse.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::IsUnit;
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ONE.is_unit(), true);
    /// assert_eq!(Natural::ZERO.is_unit(), false);
    /// assert_eq!(Natural::from(123u32).is_unit(), false);
    /// ```
    #[inline]
    fn is_unit(&self) -> bool {
        *self == 1u32
    }
}
