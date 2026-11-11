// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::{Finite, Infinity, Zero};
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::ShrRound;
use malachite_base::num::basic::traits::Zero as _;
use malachite_base::num::conversion::from::UnsignedFromFloatError;
use malachite_base::num::conversion::traits::{
    ConvertibleFrom, ExactFrom, RoundingFrom, SaturatingFrom,
};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::num::modular::traits::DivisibleByPowerOf2;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

// The absolute value of the finite nonzero value s * 2^(e - b), where b is the number of bits of
// the significand s, rounded to an integer with `abs_rm`. For a huge e, the exact integer is
// formed, which panics if it would have more than 2^64 bits.
pub(crate) fn abs_rounding_to_natural(
    significand: &Natural,
    exponent: &Integer,
    abs_rm: RoundingMode,
) -> (Natural, Ordering) {
    let bits = Integer::from(significand.significant_bits());
    if *exponent >= bits {
        (significand << u64::exact_from(&(exponent - bits)), Equal)
    } else {
        // A right shift by more than 2^64 bits gives the same result as one by 2^64 - 1 bits.
        significand.shr_round(u64::saturating_from(&(bits - exponent)), abs_rm)
    }
}

// The absolute value of the finite nonzero value s * 2^(e - b), if it is an integer.
pub(crate) fn abs_to_natural_exact(significand: &Natural, exponent: &Integer) -> Option<Natural> {
    if *exponent <= 0u32 {
        return None;
    }
    let bits = Integer::from(significand.significant_bits());
    if *exponent >= bits {
        Some(significand << u64::exact_from(&(exponent - bits)))
    } else {
        let shift = u64::exact_from(&(bits - exponent));
        if significand.divisible_by_power_of_2(shift) {
            Some(significand >> shift)
        } else {
            None
        }
    }
}

// Whether the finite nonzero value s * 2^(e - b) is an integer.
pub(crate) fn abs_is_integer(significand: &Natural, exponent: &Integer) -> bool {
    *exponent > 0u32 && {
        let bits = Integer::from(significand.significant_bits());
        *exponent >= bits
            || significand.divisible_by_power_of_2(u64::exact_from(&(bits - exponent)))
    }
}

impl RoundingFrom<&BigExponentFloat> for Natural {
    /// Converts a [`BigExponentFloat`] to a [`Natural`], using a specified [`RoundingMode`] and
    /// taking the [`BigExponentFloat`] by reference. An [`Ordering`] is also returned, indicating
    /// whether the returned value is less than, equal to, or greater than the original value.
    ///
    /// If the [`BigExponentFloat`] is negative (including $-\infty$), then it will be rounded to
    /// zero when the [`RoundingMode`] is `Ceiling`, `Down`, or `Nearest`. Otherwise, this function
    /// will panic. This is the behavior of [`Float`](crate::Float)'s conversion.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `x.complexity()`.
    ///
    /// # Panics
    /// Panics if the [`BigExponentFloat`] is NaN or $\infty$, if it is negative and `rm` is not
    /// `Ceiling`, `Down`, or `Nearest`, if `rm` is `Exact` and the [`BigExponentFloat`] is not an
    /// integer, or if the result would have more than $2^{64}$ bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::traits::RoundingFrom;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::{BigExponentFloat, Float};
    /// use malachite_nz::natural::Natural;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = BigExponentFloat::from(Float::from(2.5));
    /// assert_eq!(
    ///     Natural::rounding_from(&x, Floor),
    ///     (Natural::from(2u32), Less)
    /// );
    /// assert_eq!(
    ///     Natural::rounding_from(&x, Ceiling),
    ///     (Natural::from(3u32), Greater)
    /// );
    /// assert_eq!(
    ///     Natural::rounding_from(&x, Nearest),
    ///     (Natural::from(2u32), Less)
    /// );
    /// ```
    fn rounding_from(x: &BigExponentFloat, rm: RoundingMode) -> (Self, Ordering) {
        match x {
            BigExponentFloat(Zero) => (Self::ZERO, Equal),
            BigExponentFloat(Infinity { sign: false } | Finite { sign: false, .. }) => match rm {
                Ceiling | Down | Nearest => (Self::ZERO, Greater),
                _ => panic!("Cannot convert {x:?} to Natural using {rm}"),
            },
            BigExponentFloat(Finite {
                exponent,
                significand,
                ..
            }) => abs_rounding_to_natural(significand, exponent, rm),
            _ => panic!("Cannot convert {x:?} to Natural using {rm}"),
        }
    }
}

impl RoundingFrom<BigExponentFloat> for Natural {
    /// Converts a [`BigExponentFloat`] to a [`Natural`], using a specified [`RoundingMode`] and
    /// taking the [`BigExponentFloat`] by value. An [`Ordering`] is also returned, indicating
    /// whether the returned value is less than, equal to, or greater than the original value.
    ///
    /// If the [`BigExponentFloat`] is negative (including $-\infty$), then it will be rounded to
    /// zero when the [`RoundingMode`] is `Ceiling`, `Down`, or `Nearest`. Otherwise, this function
    /// will panic.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `x.complexity()`.
    ///
    /// # Panics
    /// Panics if the [`BigExponentFloat`] is NaN or $\infty$, if it is negative and `rm` is not
    /// `Ceiling`, `Down`, or `Nearest`, if `rm` is `Exact` and the [`BigExponentFloat`] is not an
    /// integer, or if the result would have more than $2^{64}$ bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::traits::RoundingFrom;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::{BigExponentFloat, Float};
    /// use malachite_nz::natural::Natural;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = BigExponentFloat::from(Float::from(2.5));
    /// assert_eq!(
    ///     Natural::rounding_from(x, Up),
    ///     (Natural::from(3u32), Greater)
    /// );
    /// ```
    #[inline]
    fn rounding_from(x: BigExponentFloat, rm: RoundingMode) -> (Self, Ordering) {
        Self::rounding_from(&x, rm)
    }
}

impl TryFrom<&BigExponentFloat> for Natural {
    type Error = UnsignedFromFloatError;

    /// Converts a [`BigExponentFloat`] to a [`Natural`], taking the [`BigExponentFloat`] by
    /// reference. If the [`BigExponentFloat`] is not equal to a non-negative integer, an error is
    /// returned.
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
    /// use malachite_base::num::conversion::from::UnsignedFromFloatError::*;
    /// use malachite_float::{BigExponentFloat, Float};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::try_from(&BigExponentFloat::from(Float::from(4.0))),
    ///     Ok(Natural::from(4u32))
    /// );
    /// assert_eq!(
    ///     Natural::try_from(&BigExponentFloat::from(Float::from(2.5))),
    ///     Err(FloatNonIntegerOrOutOfRange)
    /// );
    /// assert_eq!(
    ///     Natural::try_from(&BigExponentFloat::from(Float::from(-4.0))),
    ///     Err(FloatNegative)
    /// );
    /// assert_eq!(
    ///     Natural::try_from(&BigExponentFloat::NAN),
    ///     Err(FloatInfiniteOrNan)
    /// );
    /// ```
    fn try_from(x: &BigExponentFloat) -> Result<Self, Self::Error> {
        match x {
            BigExponentFloat(Zero) => Ok(Self::ZERO),
            BigExponentFloat(Finite { sign: false, .. }) => {
                Err(UnsignedFromFloatError::FloatNegative)
            }
            BigExponentFloat(Finite {
                exponent,
                significand,
                ..
            }) => abs_to_natural_exact(significand, exponent)
                .ok_or(UnsignedFromFloatError::FloatNonIntegerOrOutOfRange),
            _ => Err(UnsignedFromFloatError::FloatInfiniteOrNan),
        }
    }
}

impl TryFrom<BigExponentFloat> for Natural {
    type Error = UnsignedFromFloatError;

    /// Converts a [`BigExponentFloat`] to a [`Natural`], taking the [`BigExponentFloat`] by value.
    /// If the [`BigExponentFloat`] is not equal to a non-negative integer, an error is returned.
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
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::try_from(BigExponentFloat::from(Float::from(4.0))),
    ///     Ok(Natural::from(4u32))
    /// );
    /// ```
    #[inline]
    fn try_from(x: BigExponentFloat) -> Result<Self, Self::Error> {
        Self::try_from(&x)
    }
}

impl ConvertibleFrom<&BigExponentFloat> for Natural {
    /// Determines whether a [`BigExponentFloat`] can be converted to a [`Natural`]: whether it is a
    /// non-negative integer.
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
    /// use malachite_nz::natural::Natural;
    ///
    /// assert!(Natural::convertible_from(&BigExponentFloat::from(
    ///     Float::from(4.0)
    /// )));
    /// assert!(!Natural::convertible_from(&BigExponentFloat::from(
    ///     Float::from(2.5)
    /// )));
    /// assert!(!Natural::convertible_from(&BigExponentFloat::from(
    ///     Float::from(-4.0)
    /// )));
    /// ```
    fn convertible_from(x: &BigExponentFloat) -> bool {
        match x {
            BigExponentFloat(Zero) => true,
            BigExponentFloat(Finite {
                sign,
                exponent,
                significand,
            }) => *sign && abs_is_integer(significand, exponent),
            _ => false,
        }
    }
}
