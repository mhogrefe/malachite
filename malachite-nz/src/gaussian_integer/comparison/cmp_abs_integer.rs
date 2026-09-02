// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use crate::integer::Integer;
use core::cmp::Ordering::{self, Greater};
use malachite_base::num::arithmetic::traits::AbsSquared;
use malachite_base::num::comparison::traits::PartialOrdAbs;

impl PartialOrdAbs<Integer> for GaussianInteger {
    /// Compares the absolute values of a [`GaussianInteger`] and an [`Integer`].
    ///
    /// The absolute value of a complex number is its distance from the origin, so this is
    /// equivalent to comparing squared absolute values. Purely real and purely imaginary values are
    /// handled by comparing single components. Otherwise, the complex value is greater in absolute
    /// value unless both of its components are smaller in absolute value than the real operand, so
    /// the squared absolute values are only computed in that case.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and of `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::comparison::traits::{EqAbs, PartialOrdAbs};
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// // |3+4i| = 5
    /// let x = GaussianInteger::from_str("3+4i").unwrap();
    /// assert!(x.eq_abs(&Integer::from(-5)));
    /// assert!(x.gt_abs(&Integer::from(4)));
    /// assert!(x.lt_abs(&Integer::from(-6)));
    /// ```
    fn partial_cmp_abs(&self, other: &Integer) -> Option<Ordering> {
        if self.imaginary == 0u32 {
            self.real.partial_cmp_abs(other)
        } else if self.real == 0u32 {
            self.imaginary.partial_cmp_abs(other)
        } else if !self.real.lt_abs(other) || !self.imaginary.lt_abs(other) {
            Some(Greater)
        } else {
            Some(self.abs_squared().cmp(&other.abs_squared()))
        }
    }
}

impl PartialOrdAbs<GaussianInteger> for Integer {
    /// Compares the absolute values of an [`Integer`] and a [`GaussianInteger`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `other` and of `self`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::comparison::traits::{EqAbs, PartialOrdAbs};
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// // |3+4i| = 5
    /// let y = GaussianInteger::from_str("3+4i").unwrap();
    /// assert!(Integer::from(-5).eq_abs(&y));
    /// assert!(Integer::from(4).lt_abs(&y));
    /// assert!(Integer::from(-6).gt_abs(&y));
    /// ```
    #[inline]
    fn partial_cmp_abs(&self, other: &GaussianInteger) -> Option<Ordering> {
        other.partial_cmp_abs(self).map(Ordering::reverse)
    }
}
