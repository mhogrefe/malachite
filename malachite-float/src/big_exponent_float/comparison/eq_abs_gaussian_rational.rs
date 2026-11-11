// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use core::cmp::Ordering::*;
use malachite_base::num::comparison::traits::{EqAbs, PartialOrdAbs};
use malachite_q::gaussian_rational::GaussianRational;

impl EqAbs<GaussianRational> for BigExponentFloat {
    /// Determines whether the absolute value of a [`BigExponentFloat`] is equal to the modulus of a
    /// [`GaussianRational`].
    ///
    /// $\infty$, $-\infty$, and NaN are not equal to any [`GaussianRational`] in absolute value.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::comparison::traits::EqAbs;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// assert!(
    ///     BigExponentFloat::from(2.5).eq_abs(&GaussianRational::from_str("-3/2+2i").unwrap())
    /// );
    /// assert!(
    ///     BigExponentFloat::from(2.5).ne_abs(&GaussianRational::from_str("3/2+3i/2").unwrap())
    /// );
    /// ```
    #[inline]
    fn eq_abs(&self, other: &GaussianRational) -> bool {
        self.partial_cmp_abs(other) == Some(Equal)
    }
}

impl EqAbs<BigExponentFloat> for GaussianRational {
    /// Determines whether the modulus of a [`GaussianRational`] is equal to the absolute value of a
    /// [`BigExponentFloat`].
    ///
    /// No [`GaussianRational`] is equal to $\infty$, $-\infty$, or NaN in absolute value.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::comparison::traits::EqAbs;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// assert!(
    ///     GaussianRational::from_str("-3/2+2i")
    ///         .unwrap()
    ///         .eq_abs(&BigExponentFloat::from(2.5))
    /// );
    /// assert!(
    ///     GaussianRational::from_str("3/2+3i/2")
    ///         .unwrap()
    ///         .ne_abs(&BigExponentFloat::from(2.5))
    /// );
    /// ```
    #[inline]
    fn eq_abs(&self, other: &BigExponentFloat) -> bool {
        other.eq_abs(self)
    }
}
