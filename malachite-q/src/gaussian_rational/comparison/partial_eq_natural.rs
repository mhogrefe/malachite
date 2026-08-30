// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_rational::GaussianRational;
use malachite_nz::natural::Natural;

impl PartialEq<Natural> for GaussianRational {
    /// Determines whether a [`GaussianRational`] is equal to a
    /// [`Natural`](malachite_nz::natural::Natural).
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// assert!(GaussianRational::from(123) == Natural::from(123u32));
    /// assert!(GaussianRational::from_str("123+i").unwrap() != Natural::from(123u32));
    /// ```
    fn eq(&self, other: &Natural) -> bool {
        self.imaginary == 0u32 && self.real == *other
    }
}

impl PartialEq<GaussianRational> for Natural {
    /// Determines whether a [`Natural`](malachite_nz::natural::Natural) is equal to a
    /// [`GaussianRational`].
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// assert!(Natural::from(123u32) == GaussianRational::from(123));
    /// assert!(Natural::from(123u32) != GaussianRational::from_str("123+i").unwrap());
    /// ```
    fn eq(&self, other: &GaussianRational) -> bool {
        other.imaginary == 0u32 && other.real == *self
    }
}
