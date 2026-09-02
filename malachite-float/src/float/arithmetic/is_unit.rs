// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use malachite_base::num::arithmetic::traits::IsUnit;

impl IsUnit for Float {
    /// Determines whether a [`Float`] is a unit: whether it is finite and nonzero, so that it has a
    /// multiplicative inverse; NaN and the infinities are not units.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::IsUnit;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, One, Zero};
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::ONE.is_unit(), true);
    /// assert_eq!(Float::from(-1.5).is_unit(), true);
    /// assert_eq!(Float::ZERO.is_unit(), false);
    /// assert_eq!(Float::NAN.is_unit(), false);
    /// assert_eq!(Float::INFINITY.is_unit(), false);
    /// ```
    #[inline]
    fn is_unit(&self) -> bool {
        self.is_finite() && *self != 0u32
    }
}
