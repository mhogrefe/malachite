// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use malachite_base::num::comparison::traits::EqAbs;

impl EqAbs for Integer {
    /// Determines whether the absolute values of two [`Integer`]s are equal.
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
    ///
    /// assert_eq!(Integer::from(-123).eq_abs(&Integer::from(-122)), false);
    /// assert_eq!(Integer::from(-123).eq_abs(&Integer::from(-124)), false);
    /// assert_eq!(Integer::from(123).eq_abs(&Integer::from(123)), true);
    /// assert_eq!(Integer::from(123).eq_abs(&Integer::from(-123)), true);
    /// assert_eq!(Integer::from(-123).eq_abs(&Integer::from(123)), true);
    /// assert_eq!(Integer::from(-123).eq_abs(&Integer::from(-123)), true);
    /// ```
    #[inline]
    fn eq_abs(&self, other: &Integer) -> bool {
        self.abs == other.abs
    }
}
