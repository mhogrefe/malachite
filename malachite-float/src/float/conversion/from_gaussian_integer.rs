// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_nz::gaussian_integer::GaussianInteger;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FloatFromGaussianIntegerError;

impl TryFrom<GaussianInteger> for Float {
    type Error = FloatFromGaussianIntegerError;

    /// Converts a [`GaussianInteger`] to a [`Float`], taking the [`GaussianInteger`] by value. If
    /// the [`GaussianInteger`] is not real, an error is returned.
    ///
    /// If the [`GaussianInteger`] is nonzero, the precision of the [`Float`] is the minimum
    /// possible precision to represent it exactly.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `x.real.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_float::float::conversion::from_gaussian_integer::*;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianInteger::from_str("123").unwrap();
    /// assert_eq!(Float::try_from(x).unwrap().to_string(), "123.0");
    ///
    /// let x = GaussianInteger::from_str("2-3i").unwrap();
    /// assert_eq!(Float::try_from(x), Err(FloatFromGaussianIntegerError));
    /// ```
    fn try_from(x: GaussianInteger) -> Result<Self, Self::Error> {
        if x.imaginary == 0u32 {
            Self::try_from(x.real).map_err(|_| FloatFromGaussianIntegerError)
        } else {
            Err(FloatFromGaussianIntegerError)
        }
    }
}

impl TryFrom<&GaussianInteger> for Float {
    type Error = FloatFromGaussianIntegerError;

    /// Converts a [`GaussianInteger`] to a [`Float`], taking the [`GaussianInteger`] by reference.
    /// If the [`GaussianInteger`] is not real, an error is returned.
    ///
    /// If the [`GaussianInteger`] is nonzero, the precision of the [`Float`] is the minimum
    /// possible precision to represent it exactly.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `x.real.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_float::float::conversion::from_gaussian_integer::*;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianInteger::from_str("123").unwrap();
    /// assert_eq!(Float::try_from(&x).unwrap().to_string(), "123.0");
    ///
    /// let x = GaussianInteger::from_str("2-3i").unwrap();
    /// assert_eq!(Float::try_from(&x), Err(FloatFromGaussianIntegerError));
    /// ```
    fn try_from(x: &GaussianInteger) -> Result<Self, Self::Error> {
        if x.imaginary == 0u32 {
            Self::try_from(&x.real).map_err(|_| FloatFromGaussianIntegerError)
        } else {
            Err(FloatFromGaussianIntegerError)
        }
    }
}

impl ConvertibleFrom<&GaussianInteger> for Float {
    /// Determines whether a [`GaussianInteger`] can be converted to a [`Float`] (that is, whether
    /// it is real), taking the [`GaussianInteger`] by reference.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `x.real.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::traits::ConvertibleFrom;
    /// use malachite_float::Float;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianInteger::from_str("123").unwrap();
    /// assert_eq!(Float::convertible_from(&x), true);
    ///
    /// let x = GaussianInteger::from_str("2-3i").unwrap();
    /// assert_eq!(Float::convertible_from(&x), false);
    /// ```
    #[inline]
    fn convertible_from(x: &GaussianInteger) -> bool {
        x.imaginary == 0u32 && Self::convertible_from(&x.real)
    }
}
