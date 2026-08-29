// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_q::Rational;
use malachite_q::gaussian_rational::GaussianRational;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GaussianRationalFromFloatError;

impl TryFrom<Float> for GaussianRational {
    type Error = GaussianRationalFromFloatError;

    /// Converts a [`Float`] to a [`GaussianRational`], producing a purely real value and taking the
    /// [`Float`] by value. If the [`Float`] is NaN or infinite, an error is returned.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN};
    /// use malachite_float::Float;
    /// use malachite_float::float::conversion::gaussian_rational_from_float::*;
    /// use malachite_q::gaussian_rational::GaussianRational;
    ///
    /// let x = Float::from(123);
    /// assert_eq!(GaussianRational::try_from(x).unwrap().to_string(), "123");
    ///
    /// let x = Float::from(1.5);
    /// assert_eq!(GaussianRational::try_from(x).unwrap().to_string(), "3/2");
    ///
    /// assert_eq!(
    ///     GaussianRational::try_from(Float::NAN),
    ///     Err(GaussianRationalFromFloatError)
    /// );
    /// assert_eq!(
    ///     GaussianRational::try_from(Float::INFINITY),
    ///     Err(GaussianRationalFromFloatError)
    /// );
    /// ```
    #[inline]
    fn try_from(x: Float) -> Result<Self, Self::Error> {
        Rational::try_from(x)
            .map(Self::from)
            .map_err(|_| GaussianRationalFromFloatError)
    }
}

impl TryFrom<&Float> for GaussianRational {
    type Error = GaussianRationalFromFloatError;

    /// Converts a [`Float`] to a [`GaussianRational`], producing a purely real value and taking the
    /// [`Float`] by reference. If the [`Float`] is NaN or infinite, an error is returned.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN};
    /// use malachite_float::Float;
    /// use malachite_float::float::conversion::gaussian_rational_from_float::*;
    /// use malachite_q::gaussian_rational::GaussianRational;
    ///
    /// let x = Float::from(123);
    /// assert_eq!(GaussianRational::try_from(&x).unwrap().to_string(), "123");
    ///
    /// let x = Float::from(1.5);
    /// assert_eq!(GaussianRational::try_from(&x).unwrap().to_string(), "3/2");
    ///
    /// assert_eq!(
    ///     GaussianRational::try_from(&Float::NAN),
    ///     Err(GaussianRationalFromFloatError)
    /// );
    /// assert_eq!(
    ///     GaussianRational::try_from(&Float::INFINITY),
    ///     Err(GaussianRationalFromFloatError)
    /// );
    /// ```
    #[inline]
    fn try_from(x: &Float) -> Result<Self, Self::Error> {
        Rational::try_from(x)
            .map(Self::from)
            .map_err(|_| GaussianRationalFromFloatError)
    }
}

impl ConvertibleFrom<&Float> for GaussianRational {
    /// Determines whether a [`Float`] can be converted to a [`GaussianRational`] (that is, whether
    /// it is finite), taking the [`Float`] by reference.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN};
    /// use malachite_base::num::conversion::traits::ConvertibleFrom;
    /// use malachite_float::Float;
    /// use malachite_q::gaussian_rational::GaussianRational;
    ///
    /// assert_eq!(GaussianRational::convertible_from(&Float::from(123)), true);
    /// assert_eq!(GaussianRational::convertible_from(&Float::from(1.5)), true);
    /// assert_eq!(GaussianRational::convertible_from(&Float::NAN), false);
    /// assert_eq!(GaussianRational::convertible_from(&Float::INFINITY), false);
    /// ```
    #[inline]
    fn convertible_from(x: &Float) -> bool {
        Rational::convertible_from(x)
    }
}
