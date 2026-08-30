// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::gaussian_rational::GaussianRational;

impl PartialEq<Rational> for GaussianRational {
    /// Determines whether a [`GaussianRational`] is equal to a [`Rational`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `min(self.real.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_q::Rational;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// assert!(GaussianRational::from_str("22/7").unwrap() == Rational::from_signeds(22, 7));
    /// assert!(GaussianRational::from_str("22/7+i").unwrap() != Rational::from_signeds(22, 7));
    /// ```
    fn eq(&self, other: &Rational) -> bool {
        self.imaginary == 0u32 && self.real == *other
    }
}

impl PartialEq<GaussianRational> for Rational {
    /// Determines whether a [`Rational`] is equal to a [`GaussianRational`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `min(self.significant_bits(),
    /// other.real.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_q::Rational;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// assert!(Rational::from_signeds(22, 7) == GaussianRational::from_str("22/7").unwrap());
    /// assert!(Rational::from_signeds(22, 7) != GaussianRational::from_str("22/7+i").unwrap());
    /// ```
    fn eq(&self, other: &GaussianRational) -> bool {
        other.imaginary == 0u32 && other.real == *self
    }
}
