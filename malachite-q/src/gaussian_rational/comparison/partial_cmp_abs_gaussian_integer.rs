// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_rational::GaussianRational;
use core::cmp::Ordering::{self, Equal};
use malachite_base::num::arithmetic::traits::AbsSquared;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_nz::gaussian_integer::GaussianInteger;

// If two component comparisons pull in the same direction (or one is a tie), they decide the
// comparison of the sums of squares; only a strict conflict is indecisive. This mirrors the screens
// in the Gaussian types' `OrdAbs` implementations.
fn combine(x: Ordering, y: Ordering) -> Option<Ordering> {
    if x == y || y == Equal {
        Some(x)
    } else if x == Equal {
        Some(y)
    } else {
        None
    }
}

impl PartialOrdAbs<GaussianInteger> for GaussianRational {
    /// Compares the absolute values of a [`GaussianRational`] and a
    /// [`GaussianInteger`](malachite_nz::gaussian_integer::GaussianInteger).
    ///
    /// The absolute value of a complex number is its distance from the origin, so this is
    /// equivalent to comparing squared absolute values. The squared absolute values are usually not
    /// actually computed: comparing the parts componentwise, either directly or crosswise, often
    /// decides the ordering, and the [`AbsSquared`] fallback only runs when both pairings strictly
    /// conflict.
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
    /// use malachite_base::num::comparison::traits::{EqAbs, PartialOrdAbs};
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianRational::from_str("2-i").unwrap();
    /// assert!(x.eq_abs(&GaussianInteger::from_str("1+2i").unwrap()));
    /// assert!(x.lt_abs(&GaussianInteger::from_str("2+2i").unwrap()));
    /// assert!(x.gt_abs(&GaussianInteger::from_str("2").unwrap()));
    /// ```
    fn partial_cmp_abs(&self, other: &GaussianInteger) -> Option<Ordering> {
        if let Some(o) = combine(
            self.real.partial_cmp_abs(&other.real)?,
            self.imaginary.partial_cmp_abs(&other.imaginary)?,
        ) {
            return Some(o);
        }
        if let Some(o) = combine(
            self.real.partial_cmp_abs(&other.imaginary)?,
            self.imaginary.partial_cmp_abs(&other.real)?,
        ) {
            return Some(o);
        }
        self.abs_squared().partial_cmp(&other.abs_squared())
    }
}

impl PartialOrdAbs<GaussianRational> for GaussianInteger {
    /// Compares the absolute values of a
    /// [`GaussianInteger`](malachite_nz::gaussian_integer::GaussianInteger) and a
    /// [`GaussianRational`].
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
    /// use malachite_base::num::comparison::traits::{EqAbs, PartialOrdAbs};
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianInteger::from_str("1+2i").unwrap();
    /// assert!(x.eq_abs(&GaussianRational::from_str("2-i").unwrap()));
    /// assert!(x.gt_abs(&GaussianRational::from_str("1/2+2i").unwrap()));
    /// assert!(x.lt_abs(&GaussianRational::from_str("5/2").unwrap()));
    /// ```
    #[inline]
    fn partial_cmp_abs(&self, other: &GaussianRational) -> Option<Ordering> {
        other.partial_cmp_abs(self).map(Ordering::reverse)
    }
}
