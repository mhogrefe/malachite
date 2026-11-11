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
use malachite_nz::gaussian_integer::GaussianInteger;

impl EqAbs<GaussianInteger> for BigExponentFloat {
    /// Determines whether the absolute value of a [`BigExponentFloat`] is equal to the modulus of a
    /// [`GaussianInteger`].
    ///
    /// $\infty$, $-\infty$, and NaN are not equal to any [`GaussianInteger`] in absolute value.
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
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// assert!(BigExponentFloat::from(5).eq_abs(&GaussianInteger::from_str("-3+4i").unwrap()));
    /// assert!(BigExponentFloat::from(5).ne_abs(&GaussianInteger::from_str("3+3i").unwrap()));
    /// ```
    #[inline]
    fn eq_abs(&self, other: &GaussianInteger) -> bool {
        self.partial_cmp_abs(other) == Some(Equal)
    }
}

impl EqAbs<BigExponentFloat> for GaussianInteger {
    /// Determines whether the modulus of a [`GaussianInteger`] is equal to the absolute value of a
    /// [`BigExponentFloat`].
    ///
    /// No [`GaussianInteger`] is equal to $\infty$, $-\infty$, or NaN in absolute value.
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
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// assert!(
    ///     GaussianInteger::from_str("-3+4i")
    ///         .unwrap()
    ///         .eq_abs(&BigExponentFloat::from(5))
    /// );
    /// assert!(
    ///     GaussianInteger::from_str("3+3i")
    ///         .unwrap()
    ///         .ne_abs(&BigExponentFloat::from(5))
    /// );
    /// ```
    #[inline]
    fn eq_abs(&self, other: &BigExponentFloat) -> bool {
        other.eq_abs(self)
    }
}
