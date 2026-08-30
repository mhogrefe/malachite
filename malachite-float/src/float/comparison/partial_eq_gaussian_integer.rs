// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use malachite_nz::gaussian_integer::GaussianInteger;

impl PartialEq<GaussianInteger> for Float {
    /// Determines whether a [`Float`] is equal to a
    /// [`GaussianInteger`](malachite_nz::gaussian_integer::GaussianInteger).
    ///
    /// $\infty$, $-\infty$, and NaN are not equal to any [`GaussianInteger`]. Both the [`Float`]
    /// zero and the [`Float`] negative zero are equal to the [`GaussianInteger`] zero.
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
    /// use malachite_float::Float;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// assert!(Float::from(123) == GaussianInteger::from(123));
    /// assert!(Float::from(-123) == GaussianInteger::from(-123));
    /// assert!(Float::from(123) != GaussianInteger::from_str("123+i").unwrap());
    /// ```
    fn eq(&self, other: &GaussianInteger) -> bool {
        other.imaginary == 0u32 && *self == other.real
    }
}

impl PartialEq<Float> for GaussianInteger {
    /// Determines whether a [`GaussianInteger`](malachite_nz::gaussian_integer::GaussianInteger) is
    /// equal to a [`Float`].
    ///
    /// No [`GaussianInteger`] is equal to $\infty$, $-\infty$, or NaN. The [`GaussianInteger`] zero
    /// is equal to both the [`Float`] zero and the [`Float`] negative zero.
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
    /// use malachite_float::Float;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// assert!(GaussianInteger::from(123) == Float::from(123));
    /// assert!(GaussianInteger::from(-123) == Float::from(-123));
    /// assert!(GaussianInteger::from_str("123+i").unwrap() != Float::from(123));
    /// ```
    fn eq(&self, other: &Float) -> bool {
        self.imaginary == 0u32 && self.real == *other
    }
}
