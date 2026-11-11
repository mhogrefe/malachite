// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::{Finite, Zero};
use crate::big_exponent_float::conversion::natural_from_big_exponent_float::{
    abs_is_integer, abs_rounding_to_natural, abs_to_natural_exact,
};
use core::cmp::Ordering::{self, *};
use malachite_base::num::basic::traits::Zero as _;
use malachite_base::num::conversion::from::SignedFromFloatError;
use malachite_base::num::conversion::traits::{ConvertibleFrom, RoundingFrom};
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::integer::Integer;

impl RoundingFrom<&BigExponentFloat> for Integer {
    /// Converts a [`BigExponentFloat`] to an [`Integer`], using a specified [`RoundingMode`] and
    /// taking the [`BigExponentFloat`] by reference. An [`Ordering`] is also returned, indicating
    /// whether the returned value is less than, equal to, or greater than the original value.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `x.complexity()`.
    ///
    /// # Panics
    /// Panics if the [`BigExponentFloat`] is NaN or infinite, if `rm` is `Exact` and the
    /// [`BigExponentFloat`] is not an integer, or if the result would have more than $2^{64}$ bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::traits::RoundingFrom;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::{BigExponentFloat, Float};
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = BigExponentFloat::from(Float::from(-2.5));
    /// assert_eq!(Integer::rounding_from(&x, Floor), (Integer::from(-3), Less));
    /// assert_eq!(
    ///     Integer::rounding_from(&x, Ceiling),
    ///     (Integer::from(-2), Greater)
    /// );
    /// assert_eq!(
    ///     Integer::rounding_from(&x, Nearest),
    ///     (Integer::from(-2), Greater)
    /// );
    /// ```
    fn rounding_from(x: &BigExponentFloat, rm: RoundingMode) -> (Self, Ordering) {
        match x {
            BigExponentFloat(Zero) => (Self::ZERO, Equal),
            BigExponentFloat(Finite {
                sign,
                exponent,
                significand,
            }) => {
                let abs_rm = if *sign { rm } else { -rm };
                let (abs, o) = abs_rounding_to_natural(significand, exponent, abs_rm);
                if *sign {
                    (Self::from(abs), o)
                } else {
                    (-abs, o.reverse())
                }
            }
            _ => panic!("Cannot convert {x:?} to Integer using {rm}"),
        }
    }
}

impl RoundingFrom<BigExponentFloat> for Integer {
    /// Converts a [`BigExponentFloat`] to an [`Integer`], using a specified [`RoundingMode`] and
    /// taking the [`BigExponentFloat`] by value. An [`Ordering`] is also returned, indicating
    /// whether the returned value is less than, equal to, or greater than the original value.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `x.complexity()`.
    ///
    /// # Panics
    /// Panics if the [`BigExponentFloat`] is NaN or infinite, if `rm` is `Exact` and the
    /// [`BigExponentFloat`] is not an integer, or if the result would have more than $2^{64}$ bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::traits::RoundingFrom;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::{BigExponentFloat, Float};
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = BigExponentFloat::from(Float::from(-2.5));
    /// assert_eq!(Integer::rounding_from(x, Up), (Integer::from(-3), Less));
    /// ```
    #[inline]
    fn rounding_from(x: BigExponentFloat, rm: RoundingMode) -> (Self, Ordering) {
        Self::rounding_from(&x, rm)
    }
}

impl TryFrom<&BigExponentFloat> for Integer {
    type Error = SignedFromFloatError;

    /// Converts a [`BigExponentFloat`] to an [`Integer`], taking the [`BigExponentFloat`] by
    /// reference. If the [`BigExponentFloat`] is not equal to an integer, an error is returned.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `x.complexity()`.
    ///
    /// # Panics
    /// Panics if the result would have more than $2^{64}$ bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::NaN;
    /// use malachite_base::num::conversion::from::SignedFromFloatError::*;
    /// use malachite_float::{BigExponentFloat, Float};
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::try_from(&BigExponentFloat::from(Float::from(-4.0))),
    ///     Ok(Integer::from(-4))
    /// );
    /// assert_eq!(
    ///     Integer::try_from(&BigExponentFloat::from(Float::from(2.5))),
    ///     Err(FloatNonIntegerOrOutOfRange)
    /// );
    /// assert_eq!(
    ///     Integer::try_from(&BigExponentFloat::NAN),
    ///     Err(FloatInfiniteOrNan)
    /// );
    /// ```
    fn try_from(x: &BigExponentFloat) -> Result<Self, Self::Error> {
        match x {
            BigExponentFloat(Zero) => Ok(Self::ZERO),
            BigExponentFloat(Finite {
                sign,
                exponent,
                significand,
            }) => abs_to_natural_exact(significand, exponent)
                .map(|abs| Self::from_sign_and_abs(*sign, abs))
                .ok_or(SignedFromFloatError::FloatNonIntegerOrOutOfRange),
            _ => Err(SignedFromFloatError::FloatInfiniteOrNan),
        }
    }
}

impl TryFrom<BigExponentFloat> for Integer {
    type Error = SignedFromFloatError;

    /// Converts a [`BigExponentFloat`] to an [`Integer`], taking the [`BigExponentFloat`] by value.
    /// If the [`BigExponentFloat`] is not equal to an integer, an error is returned.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `x.complexity()`.
    ///
    /// # Panics
    /// Panics if the result would have more than $2^{64}$ bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::{BigExponentFloat, Float};
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::try_from(BigExponentFloat::from(Float::from(-4.0))),
    ///     Ok(Integer::from(-4))
    /// );
    /// ```
    #[inline]
    fn try_from(x: BigExponentFloat) -> Result<Self, Self::Error> {
        Self::try_from(&x)
    }
}

impl ConvertibleFrom<&BigExponentFloat> for Integer {
    /// Determines whether a [`BigExponentFloat`] can be converted to an [`Integer`]: whether it is
    /// an integer.
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
    /// use malachite_nz::integer::Integer;
    ///
    /// assert!(Integer::convertible_from(&BigExponentFloat::from(
    ///     Float::from(-4.0)
    /// )));
    /// assert!(!Integer::convertible_from(&BigExponentFloat::from(
    ///     Float::from(2.5)
    /// )));
    /// ```
    fn convertible_from(x: &BigExponentFloat) -> bool {
        match x {
            BigExponentFloat(Zero) => true,
            BigExponentFloat(Finite {
                exponent,
                significand,
                ..
            }) => abs_is_integer(significand, exponent),
            _ => false,
        }
    }
}
