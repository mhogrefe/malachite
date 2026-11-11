// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::big_exponent_float::BigExponentFloat;
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_nz::gaussian_integer::GaussianInteger;

/// The error that results when converting a [`GaussianInteger`] with a nonzero imaginary part to a
/// [`BigExponentFloat`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BigExponentFloatFromGaussianIntegerError;

impl TryFrom<&GaussianInteger> for BigExponentFloat {
    type Error = BigExponentFloatFromGaussianIntegerError;

    /// Converts a [`GaussianInteger`] to a [`BigExponentFloat`], taking the [`GaussianInteger`] by
    /// reference. The conversion succeeds exactly when the imaginary part is zero.
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
    /// use malachite_float::BigExponentFloat;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         BigExponentFloat::try_from(&GaussianInteger::from(Integer::from(-3))).unwrap()
    ///     ),
    ///     "-0x3.0"
    /// );
    /// assert!(BigExponentFloat::try_from(&GaussianInteger::from_str("1+i").unwrap()).is_err());
    /// ```
    fn try_from(x: &GaussianInteger) -> Result<Self, Self::Error> {
        if x.imaginary == 0u32 {
            Ok(Self::from(&x.real))
        } else {
            Err(BigExponentFloatFromGaussianIntegerError)
        }
    }
}

impl TryFrom<GaussianInteger> for BigExponentFloat {
    type Error = BigExponentFloatFromGaussianIntegerError;

    /// Converts a [`GaussianInteger`] to a [`BigExponentFloat`], taking the [`GaussianInteger`] by
    /// value. The conversion succeeds exactly when the imaginary part is zero.
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
    /// use malachite_float::BigExponentFloat;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         BigExponentFloat::try_from(GaussianInteger::from(Integer::from(-3))).unwrap()
    ///     ),
    ///     "-0x3.0"
    /// );
    /// assert!(BigExponentFloat::try_from(GaussianInteger::from_str("1+i").unwrap()).is_err());
    /// ```
    #[inline]
    fn try_from(x: GaussianInteger) -> Result<Self, Self::Error> {
        Self::try_from(&x)
    }
}

impl ConvertibleFrom<&GaussianInteger> for BigExponentFloat {
    /// Determines whether a [`GaussianInteger`] can be converted to a [`BigExponentFloat`].
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
    /// use malachite_float::BigExponentFloat;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// assert!(BigExponentFloat::convertible_from(&GaussianInteger::from(
    ///     Integer::from(-3)
    /// )));
    /// assert!(!BigExponentFloat::convertible_from(
    ///     &GaussianInteger::from_str("1+i").unwrap()
    /// ));
    /// ```
    #[inline]
    fn convertible_from(x: &GaussianInteger) -> bool {
        x.imaginary == 0u32
    }
}
