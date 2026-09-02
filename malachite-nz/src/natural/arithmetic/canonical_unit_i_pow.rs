// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::CanonicalUnitIPow;

impl CanonicalUnitIPow for Natural {
    /// Finds the power of $i$ that brings a [`Natural`] into canonical unit form. A [`Natural`] is
    /// already in canonical unit form, so this is always 0.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CanonicalUnitIPow;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(123u32).canonical_unit_i_pow(), 0);
    /// ```
    #[inline]
    fn canonical_unit_i_pow(&self) -> u64 {
        0
    }
}
