// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use core::cmp::Ordering::Equal;
use malachite_base::num::comparison::traits::{EqAbs, OrdAbs};

impl EqAbs for GaussianInteger {
    /// Determines whether the absolute values of two [`GaussianInteger`]s are equal.
    ///
    /// The absolute value of a complex number is its distance from the origin, so this is
    /// equivalent to comparing squared absolute values. The comparison delegates to [`OrdAbs`],
    /// whose componentwise and crosswise screens usually decide the answer without computing the
    /// squared absolute values.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::comparison::traits::EqAbs;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianInteger::from_str("1+2i").unwrap();
    /// let y = GaussianInteger::from_str("-2+i").unwrap();
    /// assert!(x.eq_abs(&y));
    ///
    /// let x = GaussianInteger::from_str("2+2i").unwrap();
    /// let y = GaussianInteger::from_str("3i").unwrap();
    /// // |2+2i|^2 = 8 and |3i|^2 = 9
    /// assert_eq!(x.eq_abs(&y), false);
    /// ```
    #[inline]
    fn eq_abs(&self, other: &Self) -> bool {
        self.cmp_abs(other) == Equal
    }
}
