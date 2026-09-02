// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_rational::GaussianRational;
use core::cmp::Ordering::{self, Greater};
use malachite_base::num::arithmetic::traits::AbsSquared;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_nz::integer::Integer;

impl PartialOrdAbs<Integer> for GaussianRational {
    /// Compares the absolute values of a [`GaussianRational`] and an
    /// [`Integer`](malachite_nz::integer::Integer).
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::comparison::traits::{EqAbs, PartialOrdAbs};
    /// use malachite_nz::integer::Integer;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// // |3/5+4i/5| = 1
    /// let x = GaussianRational::from_str("3/5+4i/5").unwrap();
    /// assert!(x.eq_abs(&Integer::from(-1)));
    /// assert!(x.gt_abs(&Integer::ZERO));
    /// assert!(x.lt_abs(&Integer::from(-2)));
    /// ```
    fn partial_cmp_abs(&self, other: &Integer) -> Option<Ordering> {
        if self.imaginary == 0u32 {
            self.real.partial_cmp_abs(other)
        } else if self.real == 0u32 {
            self.imaginary.partial_cmp_abs(other)
        } else if !self.real.lt_abs(other) || !self.imaginary.lt_abs(other) {
            Some(Greater)
        } else {
            self.abs_squared().partial_cmp(&other.abs_squared())
        }
    }
}

impl PartialOrdAbs<GaussianRational> for Integer {
    /// Compares the absolute values of an [`Integer`](malachite_nz::integer::Integer) and a
    /// [`GaussianRational`].
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::comparison::traits::{EqAbs, PartialOrdAbs};
    /// use malachite_nz::integer::Integer;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// // |3/5+4i/5| = 1
    /// let y = GaussianRational::from_str("3/5+4i/5").unwrap();
    /// assert!(Integer::from(-1).eq_abs(&y));
    /// assert!(Integer::ZERO.lt_abs(&y));
    /// assert!(Integer::from(-2).gt_abs(&y));
    /// ```
    #[inline]
    fn partial_cmp_abs(&self, other: &GaussianRational) -> Option<Ordering> {
        other.partial_cmp_abs(self).map(Ordering::reverse)
    }
}
