// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Finite, Zero};
use crate::{Float, significand_bits};
use malachite_base::num::basic::traits::Zero as ZeroTrait;
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_nz::integer::Integer;
use malachite_q::Rational;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RationalFromFloatError;

impl TryFrom<Float> for Rational {
    type Error = RationalFromFloatError;

    /// Converts a [`Float`] to a [`Rational`], taking the [`Float`] by value. If the [`Float`] is
    /// not finite, an error is returned.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `x.complexity()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, Zero};
    /// use malachite_float::conversion::rational_from_float::RationalFromFloatError;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::try_from(Float::ZERO).unwrap(), 0);
    /// assert_eq!(
    ///     Rational::try_from(Float::from(1.5)).unwrap().to_string(),
    ///     "3/2"
    /// );
    /// assert_eq!(
    ///     Rational::try_from(Float::from(-1.5)).unwrap().to_string(),
    ///     "-3/2"
    /// );
    ///
    /// assert_eq!(
    ///     Rational::try_from(Float::INFINITY),
    ///     Err(RationalFromFloatError)
    /// );
    /// assert_eq!(Rational::try_from(Float::NAN), Err(RationalFromFloatError));
    /// ```
    fn try_from(x: Float) -> Result<Self, Self::Error> {
        match x {
            float_either_zero!() => Ok(Self::ZERO),
            Float(Finite {
                sign,
                exponent,
                significand,
                ..
            }) => {
                let bits = significand_bits(&significand);
                Ok(Self::from(Integer::from_sign_and_abs(sign, significand))
                    << (i128::from(exponent) - i128::from(bits)))
            }
            _ => Err(RationalFromFloatError),
        }
    }
}

impl TryFrom<&Float> for Rational {
    type Error = RationalFromFloatError;

    /// Converts a [`Float`] to a [`Rational`], taking the [`Float`] by reference. If the [`Float`]
    /// is not finite, an error is returned.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `x.complexity()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, Zero};
    /// use malachite_float::conversion::rational_from_float::RationalFromFloatError;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::try_from(&Float::ZERO).unwrap(), 0);
    /// assert_eq!(
    ///     Rational::try_from(&Float::from(1.5)).unwrap().to_string(),
    ///     "3/2"
    /// );
    /// assert_eq!(
    ///     Rational::try_from(&Float::from(-1.5)).unwrap().to_string(),
    ///     "-3/2"
    /// );
    ///
    /// assert_eq!(
    ///     Rational::try_from(&Float::INFINITY),
    ///     Err(RationalFromFloatError)
    /// );
    /// assert_eq!(Rational::try_from(&Float::NAN), Err(RationalFromFloatError));
    /// ```
    fn try_from(x: &Float) -> Result<Self, Self::Error> {
        match x {
            float_either_zero!() => Ok(Self::ZERO),
            Float(Finite {
                sign,
                exponent,
                significand,
                ..
            }) => {
                let bits = significand_bits(significand);
                Ok(
                    Self::from(Integer::from_sign_and_abs_ref(*sign, significand))
                        << (i128::from(*exponent) - i128::from(bits)),
                )
            }
            _ => Err(RationalFromFloatError),
        }
    }
}

impl ConvertibleFrom<&Float> for Rational {
    /// Determines whether a [`Float`] can be converted to a [`Rational`] (which is when the
    /// [`Float`] is finite), taking the [`Float`] by reference.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, Zero};
    /// use malachite_base::num::conversion::traits::ConvertibleFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::convertible_from(&Float::ZERO), true);
    /// assert_eq!(Rational::convertible_from(&Float::from(123.0)), true);
    /// assert_eq!(Rational::convertible_from(&Float::from(-123.0)), true);
    /// assert_eq!(Rational::convertible_from(&Float::from(1.5)), true);
    ///
    /// assert_eq!(Rational::convertible_from(&Float::INFINITY), false);
    /// assert_eq!(Rational::convertible_from(&Float::NAN), false);
    /// ```
    #[inline]
    fn convertible_from(x: &Float) -> bool {
        x.is_finite()
    }
}
