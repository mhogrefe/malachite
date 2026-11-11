// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::big_exponent_float::BigExponentFloat;
use crate::float::conversion::gaussian_rational_from_float::GaussianRationalFromFloatError;
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_q::Rational;
use malachite_q::gaussian_rational::GaussianRational;

impl TryFrom<&BigExponentFloat> for GaussianRational {
    type Error = GaussianRationalFromFloatError;

    /// Converts a [`BigExponentFloat`] to a [`GaussianRational`] with a zero imaginary part, taking
    /// the [`BigExponentFloat`] by reference. An error is returned if the [`BigExponentFloat`]
    /// cannot be converted to a [`Rational`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `x.complexity()`.
    ///
    /// # Panics
    /// Panics under the conditions of the conversion to a [`Rational`].
    ///
    /// # Examples
    /// ```
    /// use malachite_float::{BigExponentFloat, Float};
    /// use malachite_q::Rational;
    /// use malachite_q::gaussian_rational::GaussianRational;
    ///
    /// assert_eq!(
    ///     GaussianRational::try_from(&BigExponentFloat::from(Float::from(-0.375))).unwrap(),
    ///     GaussianRational::from(Rational::from_signeds(-3, 8))
    /// );
    /// assert!(
    ///     GaussianRational::try_from(&BigExponentFloat::from(Float::from(f64::NAN))).is_err()
    /// );
    /// ```
    #[inline]
    fn try_from(x: &BigExponentFloat) -> Result<Self, Self::Error> {
        Rational::try_from(x)
            .map(Self::from)
            .map_err(|_| GaussianRationalFromFloatError)
    }
}

impl TryFrom<BigExponentFloat> for GaussianRational {
    type Error = GaussianRationalFromFloatError;

    /// Converts a [`BigExponentFloat`] to a [`GaussianRational`] with a zero imaginary part, taking
    /// the [`BigExponentFloat`] by value. An error is returned if the [`BigExponentFloat`] cannot
    /// be converted to a [`Rational`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `x.complexity()`.
    ///
    /// # Panics
    /// Panics under the conditions of the conversion to a [`Rational`].
    ///
    /// # Examples
    /// ```
    /// use malachite_float::{BigExponentFloat, Float};
    /// use malachite_q::Rational;
    /// use malachite_q::gaussian_rational::GaussianRational;
    ///
    /// assert_eq!(
    ///     GaussianRational::try_from(BigExponentFloat::from(Float::from(-0.375))).unwrap(),
    ///     GaussianRational::from(Rational::from_signeds(-3, 8))
    /// );
    /// ```
    #[inline]
    fn try_from(x: BigExponentFloat) -> Result<Self, Self::Error> {
        Self::try_from(&x)
    }
}

impl ConvertibleFrom<&BigExponentFloat> for GaussianRational {
    /// Determines whether a [`BigExponentFloat`] can be converted to a [`GaussianRational`].
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
    /// use malachite_q::gaussian_rational::GaussianRational;
    ///
    /// assert!(GaussianRational::convertible_from(&BigExponentFloat::from(
    ///     Float::from(-0.375)
    /// )));
    /// assert!(!GaussianRational::convertible_from(
    ///     &BigExponentFloat::from(Float::from(f64::NAN))
    /// ));
    /// ```
    #[inline]
    fn convertible_from(x: &BigExponentFloat) -> bool {
        Rational::convertible_from(x)
    }
}
