// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_rational::GaussianRational;
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::integer::Integer;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GaussianIntegerFromGaussianRationalError;

impl TryFrom<GaussianRational> for GaussianInteger {
    type Error = GaussianIntegerFromGaussianRationalError;

    /// Converts a [`GaussianRational`] to a [`GaussianInteger`], taking the [`GaussianRational`] by
    /// value. If the real and imaginary parts of the [`GaussianRational`] are not both integers, an
    /// error is returned.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use malachite_q::gaussian_rational::conversion::gaussian_integer_from_gaussian_rational::*;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianRational::from_str("2-3i").unwrap();
    /// assert_eq!(GaussianInteger::try_from(x).unwrap().to_string(), "2-3i");
    ///
    /// let x = GaussianRational::from_str("-22/7").unwrap();
    /// assert_eq!(
    ///     GaussianInteger::try_from(x),
    ///     Err(GaussianIntegerFromGaussianRationalError)
    /// );
    ///
    /// let x = GaussianRational::from_str("2/3-5i/6").unwrap();
    /// assert_eq!(
    ///     GaussianInteger::try_from(x),
    ///     Err(GaussianIntegerFromGaussianRationalError)
    /// );
    /// ```
    fn try_from(x: GaussianRational) -> Result<Self, Self::Error> {
        Ok(Self {
            real: Integer::try_from(x.real)
                .map_err(|_| GaussianIntegerFromGaussianRationalError)?,
            imaginary: Integer::try_from(x.imaginary)
                .map_err(|_| GaussianIntegerFromGaussianRationalError)?,
        })
    }
}

impl TryFrom<&GaussianRational> for GaussianInteger {
    type Error = GaussianIntegerFromGaussianRationalError;

    /// Converts a [`GaussianRational`] to a [`GaussianInteger`], taking the [`GaussianRational`] by
    /// reference. If the real and imaginary parts of the [`GaussianRational`] are not both
    /// integers, an error is returned.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.real.significant_bits(),
    /// x.imaginary.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use malachite_q::gaussian_rational::conversion::gaussian_integer_from_gaussian_rational::*;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianRational::from_str("2-3i").unwrap();
    /// assert_eq!(GaussianInteger::try_from(&x).unwrap().to_string(), "2-3i");
    ///
    /// let x = GaussianRational::from_str("-22/7").unwrap();
    /// assert_eq!(
    ///     GaussianInteger::try_from(&x),
    ///     Err(GaussianIntegerFromGaussianRationalError)
    /// );
    ///
    /// let x = GaussianRational::from_str("2/3-5i/6").unwrap();
    /// assert_eq!(
    ///     GaussianInteger::try_from(&x),
    ///     Err(GaussianIntegerFromGaussianRationalError)
    /// );
    /// ```
    fn try_from(x: &GaussianRational) -> Result<Self, Self::Error> {
        Ok(Self {
            real: Integer::try_from(&x.real)
                .map_err(|_| GaussianIntegerFromGaussianRationalError)?,
            imaginary: Integer::try_from(&x.imaginary)
                .map_err(|_| GaussianIntegerFromGaussianRationalError)?,
        })
    }
}

impl ConvertibleFrom<&GaussianRational> for GaussianInteger {
    /// Determines whether a [`GaussianRational`] can be converted to a [`GaussianInteger`] (that
    /// is, whether its real and imaginary parts are both integers), taking the [`GaussianRational`]
    /// by reference.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::traits::ConvertibleFrom;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianRational::from_str("2-3i").unwrap();
    /// assert_eq!(GaussianInteger::convertible_from(&x), true);
    ///
    /// let x = GaussianRational::from_str("-22/7").unwrap();
    /// assert_eq!(GaussianInteger::convertible_from(&x), false);
    ///
    /// let x = GaussianRational::from_str("2/3-5i/6").unwrap();
    /// assert_eq!(GaussianInteger::convertible_from(&x), false);
    /// ```
    #[inline]
    fn convertible_from(x: &GaussianRational) -> bool {
        Integer::convertible_from(&x.real) && Integer::convertible_from(&x.imaginary)
    }
}
