// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use core::cmp::Ordering::{self, Greater, Less};
use malachite_base::num::arithmetic::traits::AbsSquared;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_q::Rational;

impl PartialOrdAbs<GaussianInteger> for Float {
    /// Compares the absolute values of a [`Float`] and a
    /// [`GaussianInteger`](malachite_nz::gaussian_integer::GaussianInteger).
    ///
    /// The absolute value of a complex number is its distance from the origin, so this is
    /// equivalent to comparing squared absolute values. The [`Float`] is smaller in absolute value
    /// unless it exceeds both components in absolute value, so the squared absolute values are only
    /// computed in that case. NaN is not comparable to any [`GaussianInteger`]; $\infty$ and
    /// $-\infty$ are greater in absolute value than any [`GaussianInteger`].
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
    /// use malachite_base::num::basic::traits::Infinity;
    /// use malachite_base::num::comparison::traits::{EqAbs, PartialOrdAbs};
    /// use malachite_float::Float;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// let y = GaussianInteger::from_str("3+4i").unwrap();
    /// assert!(Float::from(-5).eq_abs(&y));
    /// assert!(Float::from(4).lt_abs(&y));
    /// assert!(Float::from(-6).gt_abs(&y));
    /// assert!(Float::INFINITY.gt_abs(&y));
    /// ```
    fn partial_cmp_abs(&self, other: &GaussianInteger) -> Option<Ordering> {
        if self.is_nan() {
            None
        } else if !self.is_finite() {
            Some(Greater)
        } else if other.imaginary == 0u32 {
            self.partial_cmp_abs(&other.real)
        } else if other.real == 0u32 {
            self.partial_cmp_abs(&other.imaginary)
        } else if !self.gt_abs(&other.real) || !self.gt_abs(&other.imaginary) {
            Some(Less)
        } else {
            Rational::exact_from(self)
                .abs_squared()
                .partial_cmp(&other.abs_squared())
        }
    }
}

impl PartialOrdAbs<Float> for GaussianInteger {
    /// Compares the absolute values of a
    /// [`GaussianInteger`](malachite_nz::gaussian_integer::GaussianInteger) and a [`Float`].
    ///
    /// No [`GaussianInteger`] is comparable to NaN, and every [`GaussianInteger`] is smaller in
    /// absolute value than $\infty$ and $-\infty$.
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
    /// use malachite_base::num::basic::traits::Infinity;
    /// use malachite_base::num::comparison::traits::{EqAbs, PartialOrdAbs};
    /// use malachite_float::Float;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianInteger::from_str("3+4i").unwrap();
    /// assert!(x.eq_abs(&Float::from(-5)));
    /// assert!(x.gt_abs(&Float::from(4)));
    /// assert!(x.lt_abs(&Float::from(-6)));
    /// assert!(x.lt_abs(&Float::INFINITY));
    /// ```
    #[inline]
    fn partial_cmp_abs(&self, other: &Float) -> Option<Ordering> {
        other.partial_cmp_abs(self).map(Ordering::reverse)
    }
}
