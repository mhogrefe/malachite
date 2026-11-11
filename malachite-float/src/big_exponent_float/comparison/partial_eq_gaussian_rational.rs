// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use malachite_q::gaussian_rational::GaussianRational;

impl PartialEq<GaussianRational> for BigExponentFloat {
    /// Determines whether a [`BigExponentFloat`] is equal to a [`GaussianRational`].
    ///
    /// They are equal when the [`GaussianRational`] is real and its real part is equal to the
    /// [`BigExponentFloat`]. $\infty$, $-\infty$, and NaN are not equal to any
    /// [`GaussianRational`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// assert!(BigExponentFloat::from(-1.5) == GaussianRational::from_str("-3/2").unwrap());
    /// assert!(BigExponentFloat::from(-1.5) != GaussianRational::from_str("-3/2+i/2").unwrap());
    /// ```
    #[inline]
    fn eq(&self, other: &GaussianRational) -> bool {
        other.imaginary == 0u32 && *self == other.real
    }
}

impl PartialEq<BigExponentFloat> for GaussianRational {
    /// Determines whether a [`GaussianRational`] is equal to a [`BigExponentFloat`].
    ///
    /// They are equal when the [`GaussianRational`] is real and its real part is equal to the
    /// [`BigExponentFloat`]. No [`GaussianRational`] is equal to $\infty$, $-\infty$, or NaN.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// assert!(GaussianRational::from_str("-3/2").unwrap() == BigExponentFloat::from(-1.5));
    /// assert!(GaussianRational::from_str("-3/2+i/2").unwrap() != BigExponentFloat::from(-1.5));
    /// ```
    #[inline]
    fn eq(&self, other: &BigExponentFloat) -> bool {
        *other == *self
    }
}
