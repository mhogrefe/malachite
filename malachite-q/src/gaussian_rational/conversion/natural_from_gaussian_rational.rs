// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_rational::GaussianRational;
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_nz::natural::Natural;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NaturalFromGaussianRationalError;

impl TryFrom<GaussianRational> for Natural {
    type Error = NaturalFromGaussianRationalError;

    /// Converts a [`GaussianRational`] to a [`Natural`], taking the [`GaussianRational`] by value.
    /// If the [`GaussianRational`] is not a real non-negative integer, an error is returned.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use malachite_q::gaussian_rational::conversion::natural_from_gaussian_rational::*;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianRational::from_str("123").unwrap();
    /// assert_eq!(Natural::try_from(x).unwrap(), 123);
    ///
    /// let x = GaussianRational::from_str("-123").unwrap();
    /// assert_eq!(Natural::try_from(x), Err(NaturalFromGaussianRationalError));
    ///
    /// let x = GaussianRational::from_str("22/7").unwrap();
    /// assert_eq!(Natural::try_from(x), Err(NaturalFromGaussianRationalError));
    ///
    /// let x = GaussianRational::from_str("2-3i").unwrap();
    /// assert_eq!(Natural::try_from(x), Err(NaturalFromGaussianRationalError));
    /// ```
    fn try_from(x: GaussianRational) -> Result<Self, Self::Error> {
        if x.imaginary == 0u32 {
            Self::try_from(x.real).map_err(|_| NaturalFromGaussianRationalError)
        } else {
            Err(NaturalFromGaussianRationalError)
        }
    }
}

impl TryFrom<&GaussianRational> for Natural {
    type Error = NaturalFromGaussianRationalError;

    /// Converts a [`GaussianRational`] to a [`Natural`], taking the [`GaussianRational`] by
    /// reference. If the [`GaussianRational`] is not a real non-negative integer, an error is
    /// returned.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use malachite_q::gaussian_rational::conversion::natural_from_gaussian_rational::*;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianRational::from_str("123").unwrap();
    /// assert_eq!(Natural::try_from(&x).unwrap(), 123);
    ///
    /// let x = GaussianRational::from_str("-123").unwrap();
    /// assert_eq!(Natural::try_from(&x), Err(NaturalFromGaussianRationalError));
    ///
    /// let x = GaussianRational::from_str("22/7").unwrap();
    /// assert_eq!(Natural::try_from(&x), Err(NaturalFromGaussianRationalError));
    ///
    /// let x = GaussianRational::from_str("2-3i").unwrap();
    /// assert_eq!(Natural::try_from(&x), Err(NaturalFromGaussianRationalError));
    /// ```
    fn try_from(x: &GaussianRational) -> Result<Self, Self::Error> {
        if x.imaginary == 0u32 {
            Self::try_from(&x.real).map_err(|_| NaturalFromGaussianRationalError)
        } else {
            Err(NaturalFromGaussianRationalError)
        }
    }
}

impl ConvertibleFrom<&GaussianRational> for Natural {
    /// Determines whether a [`GaussianRational`] can be converted to a [`Natural`] (that is,
    /// whether it is a real non-negative integer), taking the [`GaussianRational`] by reference.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::traits::ConvertibleFrom;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianRational::from_str("123").unwrap();
    /// assert_eq!(Natural::convertible_from(&x), true);
    ///
    /// let x = GaussianRational::from_str("-123").unwrap();
    /// assert_eq!(Natural::convertible_from(&x), false);
    ///
    /// let x = GaussianRational::from_str("22/7").unwrap();
    /// assert_eq!(Natural::convertible_from(&x), false);
    ///
    /// let x = GaussianRational::from_str("2-3i").unwrap();
    /// assert_eq!(Natural::convertible_from(&x), false);
    /// ```
    #[inline]
    fn convertible_from(x: &GaussianRational) -> bool {
        x.imaginary == 0u32 && Self::convertible_from(&x.real)
    }
}
