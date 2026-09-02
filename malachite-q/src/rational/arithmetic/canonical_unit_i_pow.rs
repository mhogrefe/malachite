// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_base::num::arithmetic::traits::CanonicalUnitIPow;

impl CanonicalUnitIPow for Rational {
    /// Finds the power of $i$ that brings a [`Rational`] into canonical unit form. The canonical
    /// unit form of a [`Rational`] is its absolute value, so this is 2 for negative values, since
    /// $x i^2 = -x$, and 0 otherwise.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CanonicalUnitIPow;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::from_signeds(22, 7).canonical_unit_i_pow(), 0);
    /// assert_eq!(Rational::from_signeds(-22, 7).canonical_unit_i_pow(), 2);
    /// ```
    #[inline]
    fn canonical_unit_i_pow(&self) -> u64 {
        if *self < 0u32 { 2 } else { 0 }
    }
}
