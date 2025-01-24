// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{ConvertibleFrom, SaturatingFrom};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NaturalFromIntegerError;

impl TryFrom<Integer> for Natural {
    type Error = NaturalFromIntegerError;

    /// Converts an [`Integer`] to a [`Natural`], taking the [`Natural`] by value. If the
    /// [`Integer`] is negative, an error is returned.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::try_from(Integer::from(123)).to_debug_string(),
    ///     "Ok(123)"
    /// );
    /// assert_eq!(
    ///     Natural::try_from(Integer::from(-123)).to_debug_string(),
    ///     "Err(NaturalFromIntegerError)"
    /// );
    /// assert_eq!(
    ///     Natural::try_from(Integer::from(10u32).pow(12)).to_debug_string(),
    ///     "Ok(1000000000000)"
    /// );
    /// assert_eq!(
    ///     Natural::try_from(-Integer::from(10u32).pow(12)).to_debug_string(),
    ///     "Err(NaturalFromIntegerError)"
    /// );
    /// ```
    fn try_from(value: Integer) -> Result<Natural, Self::Error> {
        match value {
            Integer { sign: false, .. } => Err(NaturalFromIntegerError),
            Integer { sign: true, abs } => Ok(abs),
        }
    }
}

impl<'a> TryFrom<&'a Integer> for Natural {
    type Error = NaturalFromIntegerError;

    /// Converts an [`Integer`] to a [`Natural`], taking the [`Natural`] by reference. If the
    /// [`Integer`] is negative, an error is returned.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `value.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::try_from(&Integer::from(123)).to_debug_string(),
    ///     "Ok(123)"
    /// );
    /// assert_eq!(
    ///     Natural::try_from(&Integer::from(-123)).to_debug_string(),
    ///     "Err(NaturalFromIntegerError)"
    /// );
    /// assert_eq!(
    ///     Natural::try_from(&Integer::from(10u32).pow(12)).to_debug_string(),
    ///     "Ok(1000000000000)"
    /// );
    /// assert_eq!(
    ///     Natural::try_from(&(-Integer::from(10u32).pow(12))).to_debug_string(),
    ///     "Err(NaturalFromIntegerError)"
    /// );
    /// ```
    fn try_from(value: &'a Integer) -> Result<Natural, Self::Error> {
        match *value {
            Integer { sign: false, .. } => Err(NaturalFromIntegerError),
            Integer {
                sign: true,
                ref abs,
            } => Ok(abs.clone()),
        }
    }
}

impl SaturatingFrom<Integer> for Natural {
    /// Converts an [`Integer`] to a [`Natural`], taking the [`Natural`] by value. If the
    /// [`Integer`] is negative, 0 is returned.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::conversion::traits::SaturatingFrom;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::saturating_from(Integer::from(123)), 123);
    /// assert_eq!(Natural::saturating_from(Integer::from(-123)), 0);
    /// assert_eq!(
    ///     Natural::saturating_from(Integer::from(10u32).pow(12)),
    ///     1000000000000u64
    /// );
    /// assert_eq!(Natural::saturating_from(-Integer::from(10u32).pow(12)), 0);
    /// ```
    fn saturating_from(value: Integer) -> Natural {
        match value {
            Integer { sign: false, .. } => Natural::ZERO,
            Integer { sign: true, abs } => abs,
        }
    }
}

impl<'a> SaturatingFrom<&'a Integer> for Natural {
    /// Converts an [`Integer`] to a [`Natural`], taking the [`Natural`] by reference. If the
    /// [`Integer`] is negative, 0 is returned.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::conversion::traits::SaturatingFrom;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::saturating_from(&Integer::from(123)), 123);
    /// assert_eq!(Natural::saturating_from(&Integer::from(-123)), 0);
    /// assert_eq!(
    ///     Natural::saturating_from(&Integer::from(10u32).pow(12)),
    ///     1000000000000u64
    /// );
    /// assert_eq!(Natural::saturating_from(&-Integer::from(10u32).pow(12)), 0);
    /// ```
    fn saturating_from(value: &'a Integer) -> Natural {
        match *value {
            Integer { sign: false, .. } => Natural::ZERO,
            Integer {
                sign: true,
                ref abs,
            } => abs.clone(),
        }
    }
}

impl ConvertibleFrom<Integer> for Natural {
    /// Determines whether an [`Integer`] can be converted to a [`Natural`] (when the [`Integer`] is
    /// non-negative). Takes the [`Integer`] by value.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::conversion::traits::ConvertibleFrom;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::convertible_from(Integer::from(123)), true);
    /// assert_eq!(Natural::convertible_from(Integer::from(-123)), false);
    /// assert_eq!(
    ///     Natural::convertible_from(Integer::from(10u32).pow(12)),
    ///     true
    /// );
    /// assert_eq!(
    ///     Natural::convertible_from(-Integer::from(10u32).pow(12)),
    ///     false
    /// );
    /// ```
    #[inline]
    fn convertible_from(value: Integer) -> bool {
        value.sign
    }
}

impl<'a> ConvertibleFrom<&'a Integer> for Natural {
    /// Determines whether an [`Integer`] can be converted to a [`Natural`] (when the [`Integer`] is
    /// non-negative). Takes the [`Integer`] by reference.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::conversion::traits::ConvertibleFrom;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::convertible_from(&Integer::from(123)), true);
    /// assert_eq!(Natural::convertible_from(&Integer::from(-123)), false);
    /// assert_eq!(
    ///     Natural::convertible_from(&Integer::from(10u32).pow(12)),
    ///     true
    /// );
    /// assert_eq!(
    ///     Natural::convertible_from(&-Integer::from(10u32).pow(12)),
    ///     false
    /// );
    /// ```
    #[inline]
    fn convertible_from(value: &'a Integer) -> bool {
        value.sign
    }
}
