// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::big_exponent_float::BigExponentFloat;
use crate::float::conversion::gaussian_integer_from_float::GaussianIntegerFromFloatError;
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::integer::Integer;

impl TryFrom<&BigExponentFloat> for GaussianInteger {
    type Error = GaussianIntegerFromFloatError;

    /// Converts a [`BigExponentFloat`] to a [`GaussianInteger`] with a zero imaginary part, taking
    /// the [`BigExponentFloat`] by reference. An error is returned if the [`BigExponentFloat`]
    /// cannot be converted to an [`Integer`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `x.complexity()`.
    ///
    /// # Panics
    /// Panics under the conditions of the conversion to an [`Integer`].
    ///
    /// # Examples
    /// ```
    /// use malachite_float::{BigExponentFloat, Float};
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     GaussianInteger::try_from(&BigExponentFloat::from(Float::from(-4.0))).unwrap(),
    ///     GaussianInteger::from(Integer::from(-4))
    /// );
    /// assert!(GaussianInteger::try_from(&BigExponentFloat::from(Float::from(2.5))).is_err());
    /// ```
    #[inline]
    fn try_from(x: &BigExponentFloat) -> Result<Self, Self::Error> {
        Integer::try_from(x)
            .map(Self::from)
            .map_err(|_| GaussianIntegerFromFloatError)
    }
}

impl TryFrom<BigExponentFloat> for GaussianInteger {
    type Error = GaussianIntegerFromFloatError;

    /// Converts a [`BigExponentFloat`] to a [`GaussianInteger`] with a zero imaginary part, taking
    /// the [`BigExponentFloat`] by value. An error is returned if the [`BigExponentFloat`] cannot
    /// be converted to an [`Integer`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `x.complexity()`.
    ///
    /// # Panics
    /// Panics under the conditions of the conversion to an [`Integer`].
    ///
    /// # Examples
    /// ```
    /// use malachite_float::{BigExponentFloat, Float};
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     GaussianInteger::try_from(BigExponentFloat::from(Float::from(-4.0))).unwrap(),
    ///     GaussianInteger::from(Integer::from(-4))
    /// );
    /// ```
    #[inline]
    fn try_from(x: BigExponentFloat) -> Result<Self, Self::Error> {
        Self::try_from(&x)
    }
}

impl ConvertibleFrom<&BigExponentFloat> for GaussianInteger {
    /// Determines whether a [`BigExponentFloat`] can be converted to a [`GaussianInteger`].
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
    /// use malachite_base::num::conversion::traits::ConvertibleFrom;
    /// use malachite_float::{BigExponentFloat, Float};
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    ///
    /// assert!(GaussianInteger::convertible_from(&BigExponentFloat::from(
    ///     Float::from(-4.0)
    /// )));
    /// assert!(!GaussianInteger::convertible_from(&BigExponentFloat::from(
    ///     Float::from(2.5)
    /// )));
    /// ```
    #[inline]
    fn convertible_from(x: &BigExponentFloat) -> bool {
        Integer::convertible_from(x)
    }
}
