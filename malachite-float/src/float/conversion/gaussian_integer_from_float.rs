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
use malachite_nz::integer::Integer;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GaussianIntegerFromFloatError;

impl TryFrom<Float> for GaussianInteger {
    type Error = GaussianIntegerFromFloatError;

    /// Converts a [`Float`] to a [`GaussianInteger`], producing a purely real value and taking the
    /// [`Float`] by value. If the [`Float`] is NaN, infinite, or not an integer, an error is
    /// returned.
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
    /// use malachite_float::float::conversion::gaussian_integer_from_float::*;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    ///
    /// let x = Float::from(123);
    /// assert_eq!(GaussianInteger::try_from(x).unwrap().to_string(), "123");
    ///
    /// let x = Float::from(-123);
    /// assert_eq!(GaussianInteger::try_from(x).unwrap().to_string(), "-123");
    ///
    /// assert_eq!(GaussianInteger::try_from(Float::from(1.5)), Err(GaussianIntegerFromFloatError));
    /// assert_eq!(GaussianInteger::try_from(Float::NAN), Err(GaussianIntegerFromFloatError));
    /// assert_eq!(GaussianInteger::try_from(Float::INFINITY), Err(GaussianIntegerFromFloatError));
    /// ```
    #[inline]
    fn try_from(x: Float) -> Result<Self, Self::Error> {
        Integer::try_from(x)
            .map(Self::from)
            .map_err(|_| GaussianIntegerFromFloatError)
    }
}

impl TryFrom<&Float> for GaussianInteger {
    type Error = GaussianIntegerFromFloatError;

    /// Converts a [`Float`] to a [`GaussianInteger`], producing a purely real value and taking the
    /// [`Float`] by reference. If the [`Float`] is NaN, infinite, or not an integer, an error is
    /// returned.
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
    /// use malachite_float::float::conversion::gaussian_integer_from_float::*;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    ///
    /// let x = Float::from(123);
    /// assert_eq!(GaussianInteger::try_from(&x).unwrap().to_string(), "123");
    ///
    /// let x = Float::from(-123);
    /// assert_eq!(GaussianInteger::try_from(&x).unwrap().to_string(), "-123");
    ///
    /// assert_eq!(
    ///     GaussianInteger::try_from(&Float::from(1.5)),
    ///     Err(GaussianIntegerFromFloatError)
    /// );
    /// assert_eq!(GaussianInteger::try_from(&Float::NAN), Err(GaussianIntegerFromFloatError));
    /// assert_eq!(GaussianInteger::try_from(&Float::INFINITY), Err(GaussianIntegerFromFloatError));
    /// ```
    #[inline]
    fn try_from(x: &Float) -> Result<Self, Self::Error> {
        Integer::try_from(x)
            .map(Self::from)
            .map_err(|_| GaussianIntegerFromFloatError)
    }
}

impl ConvertibleFrom<&Float> for GaussianInteger {
    /// Determines whether a [`Float`] can be converted to a [`GaussianInteger`] (that is, whether
    /// it is finite and an integer), taking the [`Float`] by reference.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN};
    /// use malachite_base::num::conversion::traits::ConvertibleFrom;
    /// use malachite_float::Float;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    ///
    /// assert_eq!(GaussianInteger::convertible_from(&Float::from(123)), true);
    /// assert_eq!(GaussianInteger::convertible_from(&Float::from(-123)), true);
    /// assert_eq!(GaussianInteger::convertible_from(&Float::from(1.5)), false);
    /// assert_eq!(GaussianInteger::convertible_from(&Float::NAN), false);
    /// assert_eq!(GaussianInteger::convertible_from(&Float::INFINITY), false);
    /// ```
    #[inline]
    fn convertible_from(x: &Float) -> bool {
        Integer::convertible_from(x)
    }
}
