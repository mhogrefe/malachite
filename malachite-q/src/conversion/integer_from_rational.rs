// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use core::cmp::Ordering;
use malachite_base::num::arithmetic::traits::DivRound;
use malachite_base::num::conversion::traits::{ConvertibleFrom, RoundingFrom};
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::integer::Integer;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IntegerFromRationalError;

impl TryFrom<Rational> for Integer {
    type Error = IntegerFromRationalError;

    /// Converts a [`Rational`] to an [`Integer`], taking the [`Rational`] by value. If the
    /// [`Rational`] is not an integer, an error is returned.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    /// use malachite_q::conversion::integer_from_rational::IntegerFromRationalError;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Integer::try_from(Rational::from(123)).unwrap(), 123);
    /// assert_eq!(Integer::try_from(Rational::from(-123)).unwrap(), -123);
    /// assert_eq!(
    ///     Integer::try_from(Rational::from_signeds(22, 7)),
    ///     Err(IntegerFromRationalError)
    /// );
    /// ```
    fn try_from(x: Rational) -> Result<Integer, Self::Error> {
        if x.denominator == 1u32 {
            Ok(Integer::from_sign_and_abs(x.sign, x.numerator))
        } else {
            Err(IntegerFromRationalError)
        }
    }
}

impl TryFrom<&Rational> for Integer {
    type Error = IntegerFromRationalError;

    /// Converts a [`Rational`] to an [`Integer`], taking the [`Rational`] by reference. If the
    /// [`Rational`] is not an integer, an error is returned.
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
    /// use malachite_nz::integer::Integer;
    /// use malachite_q::conversion::integer_from_rational::IntegerFromRationalError;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Integer::try_from(&Rational::from(123)).unwrap(), 123);
    /// assert_eq!(Integer::try_from(&Rational::from(-123)).unwrap(), -123);
    /// assert_eq!(
    ///     Integer::try_from(&Rational::from_signeds(22, 7)),
    ///     Err(IntegerFromRationalError)
    /// );
    /// ```
    fn try_from(x: &Rational) -> Result<Integer, Self::Error> {
        if x.denominator == 1u32 {
            Ok(Integer::from_sign_and_abs_ref(x.sign, &x.numerator))
        } else {
            Err(IntegerFromRationalError)
        }
    }
}

impl ConvertibleFrom<&Rational> for Integer {
    /// Determines whether a [`Rational`] can be converted to an [`Integer`], taking the
    /// [`Rational`] by reference.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::traits::ConvertibleFrom;
    /// use malachite_nz::integer::Integer;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Integer::convertible_from(&Rational::from(123)), true);
    /// assert_eq!(Integer::convertible_from(&Rational::from(-123)), true);
    /// assert_eq!(
    ///     Integer::convertible_from(&Rational::from_signeds(22, 7)),
    ///     false
    /// );
    /// ```
    #[inline]
    fn convertible_from(x: &Rational) -> bool {
        x.denominator == 1u32
    }
}

impl RoundingFrom<Rational> for Integer {
    /// Converts a [`Rational`] to an [`Integer`], using a specified [`RoundingMode`] and taking the
    /// [`Rational`] by value. An [`Ordering`] is also returned, indicating whether the returned
    /// value is less than, equal to, or greater than the original value.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if the [`Rational`] is not an integer and `rm` is `Exact`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::traits::RoundingFrom;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer::Integer;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Integer::rounding_from(Rational::from(123), Exact).to_debug_string(),
    ///     "(123, Equal)"
    /// );
    /// assert_eq!(
    ///     Integer::rounding_from(Rational::from(-123), Exact).to_debug_string(),
    ///     "(-123, Equal)"
    /// );
    ///
    /// assert_eq!(
    ///     Integer::rounding_from(Rational::from_signeds(22, 7), Floor).to_debug_string(),
    ///     "(3, Less)"
    /// );
    /// assert_eq!(
    ///     Integer::rounding_from(Rational::from_signeds(22, 7), Down).to_debug_string(),
    ///     "(3, Less)"
    /// );
    /// assert_eq!(
    ///     Integer::rounding_from(Rational::from_signeds(22, 7), Ceiling).to_debug_string(),
    ///     "(4, Greater)"
    /// );
    /// assert_eq!(
    ///     Integer::rounding_from(Rational::from_signeds(22, 7), Up).to_debug_string(),
    ///     "(4, Greater)"
    /// );
    /// assert_eq!(
    ///     Integer::rounding_from(Rational::from_signeds(22, 7), Nearest).to_debug_string(),
    ///     "(3, Less)"
    /// );
    ///
    /// assert_eq!(
    ///     Integer::rounding_from(Rational::from_signeds(-22, 7), Floor).to_debug_string(),
    ///     "(-4, Less)"
    /// );
    /// assert_eq!(
    ///     Integer::rounding_from(Rational::from_signeds(-22, 7), Down).to_debug_string(),
    ///     "(-3, Greater)"
    /// );
    /// assert_eq!(
    ///     Integer::rounding_from(Rational::from_signeds(-22, 7), Ceiling).to_debug_string(),
    ///     "(-3, Greater)"
    /// );
    /// assert_eq!(
    ///     Integer::rounding_from(Rational::from_signeds(-22, 7), Up).to_debug_string(),
    ///     "(-4, Less)"
    /// );
    /// assert_eq!(
    ///     Integer::rounding_from(Rational::from_signeds(-22, 7), Nearest).to_debug_string(),
    ///     "(-3, Greater)"
    /// );
    /// ```
    fn rounding_from(x: Rational, rm: RoundingMode) -> (Integer, Ordering) {
        let s = x.sign;
        let (n, o) = x
            .numerator
            .div_round(x.denominator, if s { rm } else { -rm });
        (
            Integer::from_sign_and_abs(x.sign, n),
            if s { o } else { o.reverse() },
        )
    }
}

impl RoundingFrom<&Rational> for Integer {
    /// Converts a [`Rational`] to an [`Integer`], using a specified [`RoundingMode`] and taking the
    /// [`Rational`] by reference. An [`Ordering`] is also returned, indicating whether the returned
    /// value is less than, equal to, or greater than the original value.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if the [`Rational`] is not an integer and `rm` is `Exact`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::traits::RoundingFrom;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer::Integer;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Integer::rounding_from(&Rational::from(123), Exact).to_debug_string(),
    ///     "(123, Equal)"
    /// );
    /// assert_eq!(
    ///     Integer::rounding_from(&Rational::from(-123), Exact).to_debug_string(),
    ///     "(-123, Equal)"
    /// );
    ///
    /// assert_eq!(
    ///     Integer::rounding_from(&Rational::from_signeds(22, 7), Floor).to_debug_string(),
    ///     "(3, Less)"
    /// );
    /// assert_eq!(
    ///     Integer::rounding_from(&Rational::from_signeds(22, 7), Down).to_debug_string(),
    ///     "(3, Less)"
    /// );
    /// assert_eq!(
    ///     Integer::rounding_from(&Rational::from_signeds(22, 7), Ceiling).to_debug_string(),
    ///     "(4, Greater)"
    /// );
    /// assert_eq!(
    ///     Integer::rounding_from(&Rational::from_signeds(22, 7), Up).to_debug_string(),
    ///     "(4, Greater)"
    /// );
    /// assert_eq!(
    ///     Integer::rounding_from(&Rational::from_signeds(22, 7), Nearest).to_debug_string(),
    ///     "(3, Less)"
    /// );
    ///
    /// assert_eq!(
    ///     Integer::rounding_from(Rational::from_signeds(-22, 7), Floor).to_debug_string(),
    ///     "(-4, Less)"
    /// );
    /// assert_eq!(
    ///     Integer::rounding_from(Rational::from_signeds(-22, 7), Down).to_debug_string(),
    ///     "(-3, Greater)"
    /// );
    /// assert_eq!(
    ///     Integer::rounding_from(Rational::from_signeds(-22, 7), Ceiling).to_debug_string(),
    ///     "(-3, Greater)"
    /// );
    /// assert_eq!(
    ///     Integer::rounding_from(Rational::from_signeds(-22, 7), Up).to_debug_string(),
    ///     "(-4, Less)"
    /// );
    /// assert_eq!(
    ///     Integer::rounding_from(Rational::from_signeds(-22, 7), Nearest).to_debug_string(),
    ///     "(-3, Greater)"
    /// );
    /// ```
    fn rounding_from(x: &Rational, rm: RoundingMode) -> (Integer, Ordering) {
        let (n, o) = (&x.numerator).div_round(&x.denominator, if x.sign { rm } else { -rm });
        (
            Integer::from_sign_and_abs(x.sign, n),
            if x.sign { o } else { o.reverse() },
        )
    }
}
