// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::{Finite, Zero};
use crate::float::conversion::rational_from_float::RationalFromFloatError;
use malachite_base::num::basic::traits::Zero as _;
use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::integer::Integer;
use malachite_q::Rational;

impl TryFrom<&BigExponentFloat> for Rational {
    type Error = RationalFromFloatError;

    /// Converts a [`BigExponentFloat`] to a [`Rational`], taking the [`BigExponentFloat`] by
    /// reference. The conversion is exact for every finite [`BigExponentFloat`]; NaN and
    /// $\pm\infty$ give an error.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `x.complexity()`.
    ///
    /// # Panics
    /// Panics if the exponent of the [`BigExponentFloat`], less its precision, does not fit in an
    /// [`i64`]: such a [`Rational`] would have a numerator or denominator of more than $2^{63}$
    /// bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Infinity;
    /// use malachite_float::{BigExponentFloat, Float};
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::try_from(&BigExponentFloat::from(Float::from(-0.375))).unwrap(),
    ///     Rational::from_signeds(-3, 8)
    /// );
    /// assert!(Rational::try_from(&BigExponentFloat::INFINITY).is_err());
    /// ```
    fn try_from(x: &BigExponentFloat) -> Result<Self, Self::Error> {
        match x {
            BigExponentFloat(Zero) => Ok(Self::ZERO),
            BigExponentFloat(Finite {
                sign,
                exponent,
                significand,
            }) => {
                let bits = Integer::from(significand.significant_bits());
                Ok(
                    Self::from(Integer::from_sign_and_abs_ref(*sign, significand))
                        << i64::exact_from(&(exponent - bits)),
                )
            }
            _ => Err(RationalFromFloatError),
        }
    }
}

impl TryFrom<BigExponentFloat> for Rational {
    type Error = RationalFromFloatError;

    /// Converts a [`BigExponentFloat`] to a [`Rational`], taking the [`BigExponentFloat`] by value.
    /// The conversion is exact for every finite [`BigExponentFloat`]; NaN and $\pm\infty$ give an
    /// error.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `x.complexity()`.
    ///
    /// # Panics
    /// Panics if the exponent of the [`BigExponentFloat`], less its precision, does not fit in an
    /// [`i64`].
    ///
    /// # Examples
    /// ```
    /// use malachite_float::{BigExponentFloat, Float};
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::try_from(BigExponentFloat::from(Float::from(-0.375))).unwrap(),
    ///     Rational::from_signeds(-3, 8)
    /// );
    /// ```
    #[inline]
    fn try_from(x: BigExponentFloat) -> Result<Self, Self::Error> {
        Self::try_from(&x)
    }
}

impl ConvertibleFrom<&BigExponentFloat> for Rational {
    /// Determines whether a [`BigExponentFloat`] can be converted to a [`Rational`]: whether it is
    /// finite.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{NaN, One};
    /// use malachite_base::num::conversion::traits::ConvertibleFrom;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    ///
    /// assert!(Rational::convertible_from(&BigExponentFloat::ONE));
    /// assert!(!Rational::convertible_from(&BigExponentFloat::NAN));
    /// ```
    #[inline]
    fn convertible_from(x: &BigExponentFloat) -> bool {
        x.is_finite()
    }
}
