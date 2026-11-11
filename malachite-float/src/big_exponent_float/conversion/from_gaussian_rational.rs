// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::big_exponent_float::BigExponentFloat;
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_q::gaussian_rational::GaussianRational;

/// The error that results when converting a [`GaussianRational`] with a nonzero imaginary part, or
/// a non-dyadic real part, to a [`BigExponentFloat`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BigExponentFloatFromGaussianRationalError;

impl TryFrom<&GaussianRational> for BigExponentFloat {
    type Error = BigExponentFloatFromGaussianRationalError;

    /// Converts a [`GaussianRational`] to a [`BigExponentFloat`], taking the [`GaussianRational`]
    /// by reference. The conversion succeeds exactly when the imaginary part is zero and the real
    /// part's denominator is a power of 2.
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
    /// use malachite_q::Rational;
    /// use malachite_q::gaussian_rational::GaussianRational;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         BigExponentFloat::try_from(&GaussianRational::from(Rational::from_signeds(-3, 8)))
    ///             .unwrap()
    ///     ),
    ///     "-0x0.6"
    /// );
    /// assert!(
    ///     BigExponentFloat::try_from(&GaussianRational::from(Rational::from_signeds(1, 3)))
    ///         .is_err()
    /// );
    /// ```
    fn try_from(x: &GaussianRational) -> Result<Self, Self::Error> {
        if x.imaginary == 0u32 {
            Self::try_from(&x.real).map_err(|_| BigExponentFloatFromGaussianRationalError)
        } else {
            Err(BigExponentFloatFromGaussianRationalError)
        }
    }
}

impl TryFrom<GaussianRational> for BigExponentFloat {
    type Error = BigExponentFloatFromGaussianRationalError;

    /// Converts a [`GaussianRational`] to a [`BigExponentFloat`], taking the [`GaussianRational`]
    /// by value. The conversion succeeds exactly when the imaginary part is zero and the real
    /// part's denominator is a power of 2.
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
    /// use malachite_q::Rational;
    /// use malachite_q::gaussian_rational::GaussianRational;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         BigExponentFloat::try_from(GaussianRational::from(Rational::from_signeds(-3, 8)))
    ///             .unwrap()
    ///     ),
    ///     "-0x0.6"
    /// );
    /// assert!(
    ///     BigExponentFloat::try_from(GaussianRational::from(Rational::from_signeds(1, 3)))
    ///         .is_err()
    /// );
    /// ```
    #[inline]
    fn try_from(x: GaussianRational) -> Result<Self, Self::Error> {
        Self::try_from(&x)
    }
}

impl ConvertibleFrom<&GaussianRational> for BigExponentFloat {
    /// Determines whether a [`GaussianRational`] can be converted to a [`BigExponentFloat`].
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
    /// use malachite_q::Rational;
    /// use malachite_q::gaussian_rational::GaussianRational;
    ///
    /// assert!(BigExponentFloat::convertible_from(&GaussianRational::from(
    ///     Rational::from_signeds(-3, 8)
    /// )));
    /// assert!(!BigExponentFloat::convertible_from(
    ///     &GaussianRational::from(Rational::from_signeds(1, 3))
    /// ));
    /// ```
    #[inline]
    fn convertible_from(x: &GaussianRational) -> bool {
        x.imaginary == 0u32 && Self::convertible_from(&x.real)
    }
}
