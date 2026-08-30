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
use malachite_base::num::comparison::traits::{EqAbs, PartialOrdAbs};
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

impl EqAbs<GaussianInteger> for GaussianRational {
    /// Determines whether the absolute values of a [`GaussianRational`] and a
    /// [`GaussianInteger`](malachite_nz::gaussian_integer::GaussianInteger) are equal.
    ///
    /// The absolute value of a complex number is its distance from the origin, so two values are
    /// equal in absolute value exactly when their squared absolute values are equal. The squared
    /// absolute values are usually not actually computed: comparing the parts componentwise, either
    /// directly or crosswise, often decides the answer, and the [`AbsSquared`] fallback only runs
    /// when both pairings strictly conflict.
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
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianRational::from_str("2-i").unwrap();
    /// let y = GaussianInteger::from_str("1+2i").unwrap();
    /// assert!(x.eq_abs(&y));
    ///
    /// let x = GaussianRational::from_str("1/2+i/2").unwrap();
    /// let y = GaussianInteger::from_str("1").unwrap();
    /// // |1/2+i/2|^2 = 1/2 and |1|^2 = 1
    /// assert_eq!(x.eq_abs(&y), false);
    /// ```
    fn eq_abs(&self, other: &GaussianInteger) -> bool {
        if let Some(o) = combine(
            self.real.partial_cmp_abs(&other.real).unwrap(),
            self.imaginary.partial_cmp_abs(&other.imaginary).unwrap(),
        ) {
            return o == Equal;
        }
        if let Some(o) = combine(
            self.real.partial_cmp_abs(&other.imaginary).unwrap(),
            self.imaginary.partial_cmp_abs(&other.real).unwrap(),
        ) {
            return o == Equal;
        }
        self.abs_squared() == other.abs_squared()
    }
}

impl EqAbs<GaussianRational> for GaussianInteger {
    /// Determines whether the absolute values of a
    /// [`GaussianInteger`](malachite_nz::gaussian_integer::GaussianInteger) and a
    /// [`GaussianRational`] are equal.
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
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianInteger::from_str("1+2i").unwrap();
    /// let y = GaussianRational::from_str("2-i").unwrap();
    /// assert!(x.eq_abs(&y));
    /// ```
    #[inline]
    fn eq_abs(&self, other: &GaussianRational) -> bool {
        other.eq_abs(self)
    }
}
