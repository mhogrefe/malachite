// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_rational::GaussianRational;
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_nz::integer::Integer;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IntegerFromGaussianRationalError;

impl TryFrom<GaussianRational> for Integer {
    type Error = IntegerFromGaussianRationalError;

    /// Converts a [`GaussianRational`] to an [`Integer`], taking the [`GaussianRational`] by value.
    /// If the [`GaussianRational`] is not a real integer, an error is returned.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    /// use malachite_q::gaussian_rational::conversion::integer_from_gaussian_rational::*;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianRational::from_str("-123").unwrap();
    /// assert_eq!(Integer::try_from(x).unwrap(), -123);
    ///
    /// let x = GaussianRational::from_str("-22/7").unwrap();
    /// assert_eq!(Integer::try_from(x), Err(IntegerFromGaussianRationalError));
    ///
    /// let x = GaussianRational::from_str("2-3i").unwrap();
    /// assert_eq!(Integer::try_from(x), Err(IntegerFromGaussianRationalError));
    /// ```
    fn try_from(x: GaussianRational) -> Result<Self, Self::Error> {
        if x.imaginary == 0u32 {
            Self::try_from(x.real).map_err(|_| IntegerFromGaussianRationalError)
        } else {
            Err(IntegerFromGaussianRationalError)
        }
    }
}

impl TryFrom<&GaussianRational> for Integer {
    type Error = IntegerFromGaussianRationalError;

    /// Converts a [`GaussianRational`] to an [`Integer`], taking the [`GaussianRational`] by
    /// reference. If the [`GaussianRational`] is not a real integer, an error is returned.
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
    /// use malachite_nz::integer::Integer;
    /// use malachite_q::gaussian_rational::conversion::integer_from_gaussian_rational::*;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianRational::from_str("-123").unwrap();
    /// assert_eq!(Integer::try_from(&x).unwrap(), -123);
    ///
    /// let x = GaussianRational::from_str("-22/7").unwrap();
    /// assert_eq!(Integer::try_from(&x), Err(IntegerFromGaussianRationalError));
    ///
    /// let x = GaussianRational::from_str("2-3i").unwrap();
    /// assert_eq!(Integer::try_from(&x), Err(IntegerFromGaussianRationalError));
    /// ```
    fn try_from(x: &GaussianRational) -> Result<Self, Self::Error> {
        if x.imaginary == 0u32 {
            Self::try_from(&x.real).map_err(|_| IntegerFromGaussianRationalError)
        } else {
            Err(IntegerFromGaussianRationalError)
        }
    }
}

impl ConvertibleFrom<&GaussianRational> for Integer {
    /// Determines whether a [`GaussianRational`] can be converted to an [`Integer`] (that is,
    /// whether it is a real integer), taking the [`GaussianRational`] by reference.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::traits::ConvertibleFrom;
    /// use malachite_nz::integer::Integer;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianRational::from_str("-123").unwrap();
    /// assert_eq!(Integer::convertible_from(&x), true);
    ///
    /// let x = GaussianRational::from_str("-22/7").unwrap();
    /// assert_eq!(Integer::convertible_from(&x), false);
    ///
    /// let x = GaussianRational::from_str("2-3i").unwrap();
    /// assert_eq!(Integer::convertible_from(&x), false);
    /// ```
    #[inline]
    fn convertible_from(x: &GaussianRational) -> bool {
        x.imaginary == 0u32 && Self::convertible_from(&x.real)
    }
}
