// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use core::cmp::Ordering;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_nz::integer::Integer;

impl PartialOrdAbs<Integer> for BigExponentFloat {
    /// Compares the absolute values of a [`BigExponentFloat`] and an [`Integer`].
    ///
    /// NaN is not comparable to any [`Integer`]. $\infty$ and $-\infty$ are greater than any
    /// [`Integer`] in absolute value. The [`BigExponentFloat`] zero is equal to the [`Integer`]
    /// zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::NegativeInfinity;
    /// use malachite_base::num::comparison::traits::PartialOrdAbs;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert!(BigExponentFloat::from(80).lt_abs(&Integer::from(-100)));
    /// assert!(BigExponentFloat::from(-120).gt_abs(&Integer::from(100)));
    /// assert!(BigExponentFloat::NEGATIVE_INFINITY.gt_abs(&Integer::from(-100)));
    /// ```
    #[inline]
    fn partial_cmp_abs(&self, other: &Integer) -> Option<Ordering> {
        self.partial_cmp_abs(other.unsigned_abs_ref())
    }
}

impl PartialOrdAbs<BigExponentFloat> for Integer {
    /// Compares the absolute values of an [`Integer`] and a [`BigExponentFloat`].
    ///
    /// No [`Integer`] is comparable to NaN. Every [`Integer`] is smaller than $\infty$ and
    /// $-\infty$ in absolute value. The [`Integer`] zero is equal to the [`BigExponentFloat`] zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::NegativeInfinity;
    /// use malachite_base::num::comparison::traits::PartialOrdAbs;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert!(Integer::from(-100).gt_abs(&BigExponentFloat::from(80)));
    /// assert!(Integer::from(100).lt_abs(&BigExponentFloat::from(-120)));
    /// assert!(Integer::from(-100).lt_abs(&BigExponentFloat::NEGATIVE_INFINITY));
    /// ```
    #[inline]
    fn partial_cmp_abs(&self, other: &BigExponentFloat) -> Option<Ordering> {
        other.partial_cmp_abs(self).map(Ordering::reverse)
    }
}
