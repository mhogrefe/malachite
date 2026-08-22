// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use core::cmp::Ordering;
use malachite_base::num::comparison::traits::{
    OrdAbs, OrdAbsDouble, OrdDouble, PartialOrdAbs, PartialOrdAbsDouble,
};

impl PartialOrdAbs for Integer {
    /// Compares the absolute values of two [`Integer`]s.
    ///
    /// See the documentation for the [`OrdAbs`] implementation.
    #[inline]
    fn partial_cmp_abs(&self, other: &Self) -> Option<Ordering> {
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
    #[inline]
    fn cmp_abs(&self, other: &Self) -> Ordering {
        self.abs.cmp(&other.abs)
    }
}

impl OrdAbsDouble for Integer {
    /// Compares the absolute value of an [`Integer`] with twice the absolute value of another
    /// [`Integer`].
    ///
    /// The doubling is not actually performed, so no memory is allocated. This is the shape of a
    /// round-to-nearest decision, where a remainder is weighed against half a divisor.
    ///
    /// $$
    /// f(x, y) = \operatorname{cmp}(|x|, 2|y|).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_base::num::comparison::traits::OrdAbsDouble;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering::*;
    ///
    /// assert_eq!(Integer::from(4).cmp_abs_double(&Integer::TWO), Equal);
    /// assert_eq!(Integer::from(-4).cmp_abs_double(&Integer::from(-2)), Equal);
    /// assert_eq!(Integer::from(3).cmp_abs_double(&Integer::from(-2)), Less);
    /// assert_eq!(Integer::from(-5).cmp_abs_double(&Integer::TWO), Greater);
    /// ```
    #[inline]
    fn cmp_abs_double(&self, other: &Self) -> Ordering {
        self.unsigned_abs_ref().cmp_double(other.unsigned_abs_ref())
    }
}

impl PartialOrdAbsDouble for Integer {
    /// Compares the absolute value of an [`Integer`] with twice the absolute value of another
    /// [`Integer`].
    ///
    /// See the documentation for the [`OrdAbsDouble`] implementation.
    #[inline]
    fn partial_cmp_abs_double(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp_abs_double(other))
    }
}
