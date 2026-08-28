// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use crate::natural::Natural;
use malachite_base::num::conversion::traits::ConvertibleFrom;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NaturalFromGaussianIntegerError;

impl TryFrom<GaussianInteger> for Natural {
    type Error = NaturalFromGaussianIntegerError;

    /// Converts a [`GaussianInteger`] to a [`Natural`], taking the [`GaussianInteger`] by value. If
    /// the [`GaussianInteger`] is not real or is negative, an error is returned.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::gaussian_integer::conversion::natural_from_gaussian_integer::*;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianInteger::from_str("123").unwrap();
    /// assert_eq!(Natural::try_from(x).unwrap(), 123);
    ///
    /// let x = GaussianInteger::from_str("-123").unwrap();
    /// assert_eq!(Natural::try_from(x), Err(NaturalFromGaussianIntegerError));
    ///
    /// let x = GaussianInteger::from_str("2-3i").unwrap();
    /// assert_eq!(Natural::try_from(x), Err(NaturalFromGaussianIntegerError));
    /// ```
    fn try_from(x: GaussianInteger) -> Result<Self, Self::Error> {
        if x.imaginary == 0u32 {
            Self::try_from(x.real).map_err(|_| NaturalFromGaussianIntegerError)
        } else {
            Err(NaturalFromGaussianIntegerError)
        }
    }
}

impl TryFrom<&GaussianInteger> for Natural {
    type Error = NaturalFromGaussianIntegerError;

    /// Converts a [`GaussianInteger`] to a [`Natural`], taking the [`GaussianInteger`] by
    /// reference. If the [`GaussianInteger`] is not real or is negative, an error is returned.
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
    /// use malachite_nz::gaussian_integer::conversion::natural_from_gaussian_integer::*;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianInteger::from_str("123").unwrap();
    /// assert_eq!(Natural::try_from(&x).unwrap(), 123);
    ///
    /// let x = GaussianInteger::from_str("-123").unwrap();
    /// assert_eq!(Natural::try_from(&x), Err(NaturalFromGaussianIntegerError));
    ///
    /// let x = GaussianInteger::from_str("2-3i").unwrap();
    /// assert_eq!(Natural::try_from(&x), Err(NaturalFromGaussianIntegerError));
    /// ```
    fn try_from(x: &GaussianInteger) -> Result<Self, Self::Error> {
        if x.imaginary == 0u32 {
            Self::try_from(&x.real).map_err(|_| NaturalFromGaussianIntegerError)
        } else {
            Err(NaturalFromGaussianIntegerError)
        }
    }
}

impl ConvertibleFrom<&GaussianInteger> for Natural {
    /// Determines whether a [`GaussianInteger`] can be converted to a [`Natural`] (that is, whether
    /// it is real and non-negative), taking the [`GaussianInteger`] by reference.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::traits::ConvertibleFrom;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianInteger::from_str("123").unwrap();
    /// assert_eq!(Natural::convertible_from(&x), true);
    ///
    /// let x = GaussianInteger::from_str("-123").unwrap();
    /// assert_eq!(Natural::convertible_from(&x), false);
    ///
    /// let x = GaussianInteger::from_str("2-3i").unwrap();
    /// assert_eq!(Natural::convertible_from(&x), false);
    /// ```
    #[inline]
    fn convertible_from(x: &GaussianInteger) -> bool {
        x.imaginary == 0u32 && Self::convertible_from(&x.real)
    }
}
