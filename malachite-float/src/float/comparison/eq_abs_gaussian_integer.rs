// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use malachite_base::num::arithmetic::traits::AbsSquared;
use malachite_base::num::comparison::traits::{EqAbs, PartialOrdAbs};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_q::Rational;

impl EqAbs<GaussianInteger> for Float {
    /// Determines whether the absolute values of a [`Float`] and a
    /// [`GaussianInteger`](malachite_nz::gaussian_integer::GaussianInteger) are equal.
    ///
    /// The absolute value of a complex number is its distance from the origin, so two values are
    /// equal in absolute value exactly when their squared absolute values are equal. Equality is
    /// impossible unless the float exceeds both components in absolute value, so the squared
    /// absolute values are only computed in that case. $\infty$, $-\infty$, and NaN are not equal
    /// in absolute value to any [`GaussianInteger`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of `self` and of the real and imaginary parts of `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::comparison::traits::EqAbs;
    /// use malachite_float::Float;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// // |3+4i| = 5
    /// let y = GaussianInteger::from_str("3+4i").unwrap();
    /// assert!(Float::from(-5).eq_abs(&y));
    /// assert_eq!(Float::from(4).eq_abs(&y), false);
    /// ```
    fn eq_abs(&self, other: &GaussianInteger) -> bool {
        if other.imaginary == 0u32 {
            self.eq_abs(&other.real)
        } else if other.real == 0u32 {
            self.eq_abs(&other.imaginary)
        } else if self.is_finite() {
            self.gt_abs(&other.real)
                && self.gt_abs(&other.imaginary)
                && Rational::exact_from(self).abs_squared() == other.abs_squared()
        } else {
            false
        }
    }
}

impl EqAbs<Float> for GaussianInteger {
    /// Determines whether the absolute values of a
    /// [`GaussianInteger`](malachite_nz::gaussian_integer::GaussianInteger) and a [`Float`] are
    /// equal.
    ///
    /// No [`GaussianInteger`] is equal in absolute value to $\infty$, $-\infty$, or NaN.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of `other` and of the real and imaginary parts of `self`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::comparison::traits::EqAbs;
    /// use malachite_float::Float;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// // |3+4i| = 5
    /// let x = GaussianInteger::from_str("3+4i").unwrap();
    /// assert!(x.eq_abs(&Float::from(-5)));
    /// assert_eq!(x.eq_abs(&Float::from(4)), false);
    /// ```
    #[inline]
    fn eq_abs(&self, other: &Float) -> bool {
        other.eq_abs(self)
    }
}
