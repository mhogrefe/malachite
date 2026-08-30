// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_rational::GaussianRational;
use malachite_nz::gaussian_integer::GaussianInteger;

impl PartialEq<GaussianInteger> for GaussianRational {
    /// Determines whether a [`GaussianRational`] is equal to a
    /// [`GaussianInteger`](malachite_nz::gaussian_integer::GaussianInteger), comparing
    /// componentwise.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of significant bits
    /// of the real and imaginary parts of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// assert!(
    ///     GaussianRational::from_str("123+i").unwrap()
    ///         == GaussianInteger::from_str("123+i").unwrap()
    /// );
    /// assert!(
    ///     GaussianRational::from_str("123+i/2").unwrap()
    ///         != GaussianInteger::from_str("123+i").unwrap()
    /// );
    /// ```
    fn eq(&self, other: &GaussianInteger) -> bool {
        self.real == other.real && self.imaginary == other.imaginary
    }
}

impl PartialEq<GaussianRational> for GaussianInteger {
    /// Determines whether a [`GaussianInteger`](malachite_nz::gaussian_integer::GaussianInteger) is
    /// equal to a [`GaussianRational`], comparing componentwise.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of significant bits
    /// of the real and imaginary parts of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// assert!(
    ///     GaussianInteger::from_str("123+i").unwrap()
    ///         == GaussianRational::from_str("123+i").unwrap()
    /// );
    /// assert!(
    ///     GaussianInteger::from_str("123+i").unwrap()
    ///         != GaussianRational::from_str("123+i/2").unwrap()
    /// );
    /// ```
    fn eq(&self, other: &GaussianRational) -> bool {
        other.real == self.real && other.imaginary == self.imaginary
    }
}
