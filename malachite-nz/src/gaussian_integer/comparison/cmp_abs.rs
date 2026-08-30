// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use core::cmp::Ordering::{self, Equal};
use malachite_base::num::arithmetic::traits::AbsSquared;
use malachite_base::num::comparison::traits::{OrdAbs, PartialOrdAbs};

// If two component comparisons pull in the same direction (or one is a tie), they decide the
// comparison of the sums of squares; only a strict conflict is indecisive.
fn combine(x: Ordering, y: Ordering) -> Option<Ordering> {
    if x == y || y == Equal {
        Some(x)
    } else if x == Equal {
        Some(y)
    } else {
        None
    }
}

impl PartialOrdAbs for GaussianInteger {
    /// Compares the absolute values of two [`GaussianInteger`]s.
    ///
    /// See the documentation for the [`OrdAbs`] implementation.
    #[inline]
    fn partial_cmp_abs(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp_abs(other))
    }
}

impl OrdAbs for GaussianInteger {
    /// Compares the absolute values of two [`GaussianInteger`]s.
    ///
    /// The absolute value of a complex number is its distance from the origin, so this is
    /// equivalent to comparing squared absolute values:
    ///
    /// $$
    /// f(x, y) = \operatorname{cmp}(|x|, |y|) = \operatorname{cmp}(|x|^2, |y|^2).
    /// $$
    ///
    /// The squared absolute values are usually not actually computed: comparing the
    /// [`Integer`](crate::integer::Integer) parts componentwise, either directly or crosswise,
    /// often decides the ordering, and the [`AbsSquared`] fallback only runs when both pairings
    /// strictly conflict.
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
    /// use malachite_base::num::comparison::traits::{OrdAbs, PartialOrdAbs};
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::cmp::Ordering::*;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianInteger::from_str("2+2i").unwrap();
    /// let y = GaussianInteger::from_str("3i").unwrap();
    /// // |2+2i|^2 = 8 and |3i|^2 = 9
    /// assert_eq!(x.cmp_abs(&y), Less);
    /// assert!(x.lt_abs(&y));
    ///
    /// let x = GaussianInteger::from_str("1+2i").unwrap();
    /// let y = GaussianInteger::from_str("-2+i").unwrap();
    /// assert_eq!(x.cmp_abs(&y), Equal);
    ///
    /// let x = GaussianInteger::from_str("3").unwrap();
    /// let y = GaussianInteger::from_str("2+2i").unwrap();
    /// // |3|^2 = 9 and |2+2i|^2 = 8
    /// assert_eq!(x.cmp_abs(&y), Greater);
    /// ```
    fn cmp_abs(&self, other: &Self) -> Ordering {
        if let Some(o) = combine(
            self.real.cmp_abs(&other.real),
            self.imaginary.cmp_abs(&other.imaginary),
        ) {
            return o;
        }
        if let Some(o) = combine(
            self.real.cmp_abs(&other.imaginary),
            self.imaginary.cmp_abs(&other.real),
        ) {
            return o;
        }
        self.abs_squared().cmp(&other.abs_squared())
    }
}
