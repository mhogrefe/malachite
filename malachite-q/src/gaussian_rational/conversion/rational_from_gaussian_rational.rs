// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::gaussian_rational::GaussianRational;
use malachite_base::num::conversion::traits::ConvertibleFrom;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RationalFromGaussianRationalError;

impl TryFrom<GaussianRational> for Rational {
    type Error = RationalFromGaussianRationalError;

    /// Converts a [`GaussianRational`] to a [`Rational`], taking the [`GaussianRational`] by value.
    /// If the [`GaussianRational`] is not real, an error is returned.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_q::gaussian_rational::conversion::rational_from_gaussian_rational::*;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianRational::from_str("123").unwrap();
    /// assert_eq!(Rational::try_from(x).unwrap(), 123);
    ///
    /// let x = GaussianRational::from_str("-22/7").unwrap();
    /// assert_eq!(Rational::try_from(x).unwrap().to_string(), "-22/7");
    ///
    /// let x = GaussianRational::from_str("2/3-5i/6").unwrap();
    /// assert_eq!(Rational::try_from(x), Err(RationalFromGaussianRationalError));
    /// ```
    fn try_from(x: GaussianRational) -> Result<Self, Self::Error> {
        if x.imaginary == 0u32 {
            Ok(x.real)
        } else {
            Err(RationalFromGaussianRationalError)
        }
    }
}

impl TryFrom<&GaussianRational> for Rational {
    type Error = RationalFromGaussianRationalError;

    /// Converts a [`GaussianRational`] to a [`Rational`], taking the [`GaussianRational`] by
    /// reference. If the [`GaussianRational`] is not real, an error is returned.
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
    /// use malachite_q::gaussian_rational::conversion::rational_from_gaussian_rational::*;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianRational::from_str("123").unwrap();
    /// assert_eq!(Rational::try_from(&x).unwrap(), 123);
    ///
    /// let x = GaussianRational::from_str("-22/7").unwrap();
    /// assert_eq!(Rational::try_from(&x).unwrap().to_string(), "-22/7");
    ///
    /// let x = GaussianRational::from_str("2/3-5i/6").unwrap();
    /// assert_eq!(Rational::try_from(&x), Err(RationalFromGaussianRationalError));
    /// ```
    fn try_from(x: &GaussianRational) -> Result<Self, Self::Error> {
        if x.imaginary == 0u32 {
            Ok(x.real.clone())
        } else {
            Err(RationalFromGaussianRationalError)
        }
    }
}

impl ConvertibleFrom<&GaussianRational> for Rational {
    /// Determines whether a [`GaussianRational`] can be converted to a [`Rational`] (that is,
    /// whether it is real), taking the [`GaussianRational`] by reference.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::traits::ConvertibleFrom;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianRational::from_str("123").unwrap();
    /// assert_eq!(Rational::convertible_from(&x), true);
    ///
    /// let x = GaussianRational::from_str("-22/7").unwrap();
    /// assert_eq!(Rational::convertible_from(&x), true);
    ///
    /// let x = GaussianRational::from_str("2/3-5i/6").unwrap();
    /// assert_eq!(Rational::convertible_from(&x), false);
    /// ```
    #[inline]
    fn convertible_from(x: &GaussianRational) -> bool {
        x.imaginary == 0u32
    }
}
