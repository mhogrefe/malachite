// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::DivRound;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{ConvertibleFrom, RoundingFrom};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::Natural;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NaturalFromRationalError;

impl TryFrom<Rational> for Natural {
    type Error = NaturalFromRationalError;

    /// Converts a [`Rational`] to a [`Natural`], taking the [`Rational`] by value. If the
    /// [`Rational`] is negative or not an integer, an error is returned.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::conversion::natural_from_rational::NaturalFromRationalError;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Natural::try_from(Rational::from(123)).unwrap(), 123);
    /// assert_eq!(
    ///     Natural::try_from(Rational::from(-123)),
    ///     Err(NaturalFromRationalError)
    /// );
    /// assert_eq!(
    ///     Natural::try_from(Rational::from_signeds(22, 7)),
    ///     Err(NaturalFromRationalError)
    /// );
    /// ```
    fn try_from(x: Rational) -> Result<Natural, Self::Error> {
        if x.sign && x.denominator == 1u32 {
            Ok(x.numerator)
        } else {
            Err(NaturalFromRationalError)
        }
    }
}

impl TryFrom<&Rational> for Natural {
    type Error = NaturalFromRationalError;

    /// Converts a [`Rational`] to a [`Natural`], taking the [`Rational`] by reference. If the
    /// [`Rational`] is negative or not an integer, an error is returned.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::conversion::natural_from_rational::NaturalFromRationalError;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Natural::try_from(&Rational::from(123)).unwrap(), 123);
    /// assert_eq!(
    ///     Natural::try_from(&Rational::from(-123)),
    ///     Err(NaturalFromRationalError)
    /// );
    /// assert_eq!(
    ///     Natural::try_from(&Rational::from_signeds(22, 7)),
    ///     Err(NaturalFromRationalError)
    /// );
    /// ```
    fn try_from(x: &Rational) -> Result<Natural, Self::Error> {
        if x.sign && x.denominator == 1u32 {
            Ok(x.numerator.clone())
        } else {
            Err(NaturalFromRationalError)
        }
    }
}

impl ConvertibleFrom<&Rational> for Natural {
    /// Determines whether a [`Rational`] can be converted to a [`Natural`] (when the [`Rational`]
    /// is non-negative and an integer), taking the [`Rational`] by reference.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::traits::ConvertibleFrom;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Natural::convertible_from(&Rational::from(123)), true);
    /// assert_eq!(Natural::convertible_from(&Rational::from(-123)), false);
    /// assert_eq!(
    ///     Natural::convertible_from(&Rational::from_signeds(22, 7)),
    ///     false
    /// );
    /// ```
    #[inline]
    fn convertible_from(x: &Rational) -> bool {
        x.sign && x.denominator == 1u32
    }
}

impl RoundingFrom<Rational> for Natural {
    /// Converts a [`Rational`] to a [`Natural`], using a specified [`RoundingMode`] and taking the
    /// [`Rational`] by value. An [`Ordering`] is also returned, indicating whether the returned
    /// value is less than, equal to, or greater than the original value.
    ///
    /// If the [`Rational`] is negative, then it will be rounded to zero when the [`RoundingMode`]
    /// is `Ceiling`, `Down`, or `Nearest`. Otherwise, this function will panic.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if the [`Rational`] is not an integer and `rm` is `Exact`, or if the [`Rational`] is
    /// less than zero and `rm` is not `Down`, `Ceiling`, or `Nearest`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::traits::RoundingFrom;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Natural::rounding_from(Rational::from(123), Exact).to_debug_string(),
    ///     "(123, Equal)"
    /// );
    ///
    /// assert_eq!(
    ///     Natural::rounding_from(Rational::from_signeds(22, 7), Floor).to_debug_string(),
    ///     "(3, Less)"
    /// );
    /// assert_eq!(
    ///     Natural::rounding_from(Rational::from_signeds(22, 7), Down).to_debug_string(),
    ///     "(3, Less)"
    /// );
    /// assert_eq!(
    ///     Natural::rounding_from(Rational::from_signeds(22, 7), Ceiling).to_debug_string(),
    ///     "(4, Greater)"
    /// );
    /// assert_eq!(
    ///     Natural::rounding_from(Rational::from_signeds(22, 7), Up).to_debug_string(),
    ///     "(4, Greater)"
    /// );
    /// assert_eq!(
    ///     Natural::rounding_from(Rational::from_signeds(22, 7), Nearest).to_debug_string(),
    ///     "(3, Less)"
    /// );
    ///
    /// assert_eq!(
    ///     Natural::rounding_from(Rational::from(-123), Down).to_debug_string(),
    ///     "(0, Greater)"
    /// );
    /// assert_eq!(
    ///     Natural::rounding_from(Rational::from(-123), Ceiling).to_debug_string(),
    ///     "(0, Greater)"
    /// );
    /// assert_eq!(
    ///     Natural::rounding_from(Rational::from(-123), Nearest).to_debug_string(),
    ///     "(0, Greater)"
    /// );
    /// ```
    fn rounding_from(x: Rational, rm: RoundingMode) -> (Natural, Ordering) {
        if x.sign {
            x.numerator.div_round(x.denominator, rm)
        } else if rm == Down || rm == Ceiling || rm == Nearest {
            (Natural::ZERO, Greater)
        } else {
            panic!("Cannot round negative Rational to Natural using RoundingMode {rm}");
        }
    }
}

impl RoundingFrom<&Rational> for Natural {
    /// Converts a [`Rational`] to a [`Natural`], using a specified [`RoundingMode`] and taking the
    /// [`Rational`] by reference. An [`Ordering`] is also returned, indicating whether the returned
    /// value is less than, equal to, or greater than the original value.
    ///
    /// If the [`Rational`] is negative, then it will be rounded to zero when the [`RoundingMode`]
    /// is `Ceiling`, `Down`, or `Nearest`. Otherwise, this function will panic.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if the [`Rational`] is not an integer and `rm` is `Exact`, or if the [`Rational`] is
    /// less than zero and `rm` is not `Down`, `Ceiling`, or `Nearest`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::traits::RoundingFrom;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Natural::rounding_from(&Rational::from(123), Exact).to_debug_string(),
    ///     "(123, Equal)"
    /// );
    ///
    /// assert_eq!(
    ///     Natural::rounding_from(&Rational::from_signeds(22, 7), Floor).to_debug_string(),
    ///     "(3, Less)"
    /// );
    /// assert_eq!(
    ///     Natural::rounding_from(&Rational::from_signeds(22, 7), Down).to_debug_string(),
    ///     "(3, Less)"
    /// );
    /// assert_eq!(
    ///     Natural::rounding_from(&Rational::from_signeds(22, 7), Ceiling).to_debug_string(),
    ///     "(4, Greater)"
    /// );
    /// assert_eq!(
    ///     Natural::rounding_from(&Rational::from_signeds(22, 7), Up).to_debug_string(),
    ///     "(4, Greater)"
    /// );
    /// assert_eq!(
    ///     Natural::rounding_from(&Rational::from_signeds(22, 7), Nearest).to_debug_string(),
    ///     "(3, Less)"
    /// );
    ///
    /// assert_eq!(
    ///     Natural::rounding_from(&Rational::from(-123), Down).to_debug_string(),
    ///     "(0, Greater)"
    /// );
    /// assert_eq!(
    ///     Natural::rounding_from(&Rational::from(-123), Ceiling).to_debug_string(),
    ///     "(0, Greater)"
    /// );
    /// assert_eq!(
    ///     Natural::rounding_from(&Rational::from(-123), Nearest).to_debug_string(),
    ///     "(0, Greater)"
    /// );
    /// ```
    fn rounding_from(x: &Rational, rm: RoundingMode) -> (Natural, Ordering) {
        if x.sign {
            (&x.numerator).div_round(&x.denominator, rm)
        } else if rm == Down || rm == Ceiling || rm == Nearest {
            (Natural::ZERO, Greater)
        } else {
            panic!("Cannot round negative Rational to Natural using RoundingMode {rm}");
        }
    }
}
