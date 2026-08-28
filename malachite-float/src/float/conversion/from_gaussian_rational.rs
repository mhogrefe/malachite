// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_q::gaussian_rational::GaussianRational;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FloatFromGaussianRationalError;

impl TryFrom<GaussianRational> for Float {
    type Error = FloatFromGaussianRationalError;

    /// Converts a [`GaussianRational`] to a [`Float`], taking the [`GaussianRational`] by value. If
    /// the [`GaussianRational`] is not real or is not exactly representable as a dyadic rational,
    /// an error is returned.
    ///
    /// If the [`GaussianRational`] is nonzero, the precision of the [`Float`] is the minimum
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
    /// use malachite_float::float::conversion::from_gaussian_rational::*;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianRational::from_str("5/2").unwrap();
    /// assert_eq!(Float::try_from(x).unwrap().to_string(), "2.5");
    ///
    /// let x = GaussianRational::from_str("1/3").unwrap();
    /// assert_eq!(Float::try_from(x), Err(FloatFromGaussianRationalError));
    ///
    /// let x = GaussianRational::from_str("2-3i").unwrap();
    /// assert_eq!(Float::try_from(x), Err(FloatFromGaussianRationalError));
    /// ```
    fn try_from(x: GaussianRational) -> Result<Self, Self::Error> {
        if x.imaginary == 0u32 {
            Self::try_from(x.real).map_err(|_| FloatFromGaussianRationalError)
        } else {
            Err(FloatFromGaussianRationalError)
        }
    }
}

impl TryFrom<&GaussianRational> for Float {
    type Error = FloatFromGaussianRationalError;

    /// Converts a [`GaussianRational`] to a [`Float`], taking the [`GaussianRational`] by
    /// reference. If the [`GaussianRational`] is not real or is not exactly representable as a
    /// dyadic rational, an error is returned.
    ///
    /// If the [`GaussianRational`] is nonzero, the precision of the [`Float`] is the minimum
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
    /// use malachite_float::float::conversion::from_gaussian_rational::*;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianRational::from_str("5/2").unwrap();
    /// assert_eq!(Float::try_from(&x).unwrap().to_string(), "2.5");
    ///
    /// let x = GaussianRational::from_str("1/3").unwrap();
    /// assert_eq!(Float::try_from(&x), Err(FloatFromGaussianRationalError));
    ///
    /// let x = GaussianRational::from_str("2-3i").unwrap();
    /// assert_eq!(Float::try_from(&x), Err(FloatFromGaussianRationalError));
    /// ```
    fn try_from(x: &GaussianRational) -> Result<Self, Self::Error> {
        if x.imaginary == 0u32 {
            Self::try_from(&x.real).map_err(|_| FloatFromGaussianRationalError)
        } else {
            Err(FloatFromGaussianRationalError)
        }
    }
}

impl ConvertibleFrom<&GaussianRational> for Float {
    /// Determines whether a [`GaussianRational`] can be converted to a [`Float`] (that is, whether
    /// it is real or is not exactly representable as a dyadic rational), taking the
    /// [`GaussianRational`] by reference.
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
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianRational::from_str("5/2").unwrap();
    /// assert_eq!(Float::convertible_from(&x), true);
    ///
    /// let x = GaussianRational::from_str("1/3").unwrap();
    /// assert_eq!(Float::convertible_from(&x), false);
    ///
    /// let x = GaussianRational::from_str("2-3i").unwrap();
    /// assert_eq!(Float::convertible_from(&x), false);
    /// ```
    #[inline]
    fn convertible_from(x: &GaussianRational) -> bool {
        x.imaginary == 0u32 && Self::convertible_from(&x.real)
    }
}
