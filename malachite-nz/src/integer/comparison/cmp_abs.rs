// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use core::cmp::Ordering;
use malachite_base::num::comparison::traits::{OrdAbs, PartialOrdAbs};

impl PartialOrdAbs for Integer {
    /// Compares the absolute values of two [`Integer`]s.
    ///
    /// See the documentation for the [`OrdAbs`] implementation.
    #[inline]
    fn partial_cmp_abs(&self, other: &Integer) -> Option<Ordering> {
        Some(self.cmp_abs(other))
    }
}

impl OrdAbs for Integer {
    /// Compares the absolute values of two [`Integer`]s.
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
    /// use malachite_base::num::comparison::traits::PartialOrdAbs;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert!(Integer::from(-123).lt_abs(&Integer::from(-124)));
    /// assert!(Integer::from(-123).le_abs(&Integer::from(-124)));
    /// assert!(Integer::from(-124).gt_abs(&Integer::from(-123)));
    /// assert!(Integer::from(-124).ge_abs(&Integer::from(-123)));
    /// ```
    fn cmp_abs(&self, other: &Integer) -> Ordering {
        self.abs.cmp(&other.abs)
    }
}
