// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use malachite_base::num::arithmetic::traits::CanonicalUnitIPow;

impl CanonicalUnitIPow for Float {
    /// Finds the power of $i$ that brings a [`Float`] into canonical unit form. The canonical unit
    /// form of a [`Float`] is its absolute value, so this is 2 for values with the sign bit set,
    /// negative zero and negative infinity included, since $x i^2 = -x$, and 0 otherwise, NaN
    /// included.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CanonicalUnitIPow;
    /// use malachite_base::num::basic::traits::{NaN, NegativeZero};
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::from(1.5).canonical_unit_i_pow(), 0);
    /// assert_eq!(Float::from(-1.5).canonical_unit_i_pow(), 2);
    /// assert_eq!(Float::NEGATIVE_ZERO.canonical_unit_i_pow(), 2);
    /// assert_eq!(Float::NAN.canonical_unit_i_pow(), 0);
    /// ```
    #[inline]
    fn canonical_unit_i_pow(&self) -> u64 {
        if self.is_sign_negative() && !self.is_nan() {
            2
        } else {
            0
        }
    }
}
