// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::Finite;
use crate::{significand_bits, Float};
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{
    CheckedLogBase2, IsPowerOf2, NegModPowerOf2, RoundToMultipleOfPowerOf2, UnsignedAbs,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::platform::Limb;
use malachite_q::conversion::primitive_float_from_rational::FloatFromRationalError;
use malachite_q::Rational;

impl Float {
    /// Converts a [`Rational`] to a [`Float`], taking the [`Rational`] by value. If the [`Float`]
    /// is nonzero, it has the specified precision. If rounding is needed, the specified rounding
    /// mode is used. An [`Ordering`] is also returned, indicating whether the returned value is
    /// less than, equal to, or greater than the original value.
    ///
    /// If you're only using [`Nearest`], try using [`Float::from_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(n.significant_bits(), prec)`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = Float::from_rational_prec_round(Rational::from_signeds(1, 3), 10, Floor);
    /// assert_eq!(x.to_string(), "0.333");
    /// assert_eq!(x.get_prec(), Some(10));
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) = Float::from_rational_prec_round(Rational::from_signeds(1, 3), 10, Ceiling);
    /// assert_eq!(x.to_string(), "0.3335");
    /// assert_eq!(x.get_prec(), Some(10));
    /// assert_eq!(o, Greater);
    ///
    /// let (x, o) = Float::from_rational_prec_round(Rational::from_signeds(1, 3), 10, Nearest);
    /// assert_eq!(x.to_string(), "0.3335");
    /// assert_eq!(x.get_prec(), Some(10));
    /// assert_eq!(o, Greater);
    ///
    /// let (x, o) = Float::from_rational_prec_round(Rational::from_signeds(-1, 3), 10, Floor);
    /// assert_eq!(x.to_string(), "-0.3335");
    /// assert_eq!(x.get_prec(), Some(10));
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) = Float::from_rational_prec_round(Rational::from_signeds(-1, 3), 10, Ceiling);
    /// assert_eq!(x.to_string(), "-0.333");
    /// assert_eq!(x.get_prec(), Some(10));
    /// assert_eq!(o, Greater);
    ///
    /// let (x, o) = Float::from_rational_prec_round(Rational::from_signeds(-1, 3), 10, Nearest);
    /// assert_eq!(x.to_string(), "-0.3335");
    /// assert_eq!(x.get_prec(), Some(10));
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn from_rational_prec_round(x: Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
        assert_ne!(prec, 0);
        if x == 0 {
            (Float::ZERO, Equal)
        } else {
            let mut exponent = i32::exact_from(x.floor_log_base_2_abs());
            let (significand, o) =
                Integer::rounding_from(x << (i32::exact_from(prec) - exponent - 1), rm);
            let sign = significand >= 0;
            let mut significand = significand.unsigned_abs();
            let away_from_0 = if sign { Greater } else { Less };
            if o == away_from_0 && significand.is_power_of_2() {
                exponent += 1;
            }
            significand <<= significand
                .significant_bits()
                .neg_mod_power_of_2(Limb::LOG_WIDTH);
            let target_bits = prec
                .round_to_multiple_of_power_of_2(Limb::LOG_WIDTH, Ceiling)
                .0;
            let current_bits = significand_bits(&significand);
            if current_bits > target_bits {
                significand >>= current_bits - target_bits;
            }
            (
                Float(Finite {
                    sign,
                    exponent: exponent + 1,
                    precision: prec,
                    significand,
                }),
                o,
            )
        }
    }

    /// Converts a [`Rational`] to a [`Float`], taking the [`Rational`] by value. If the [`Float`]
    /// is nonzero, it has the specified precision. An [`Ordering`] is also returned, indicating
    /// whether the returned value is less than, equal to, or greater than the original value.
    ///
    /// If the [`Rational`] is dyadic (its denominator is a power of 2), then you can convert it to
    /// a [`Float`] using `try_from` instead. The precision of the resulting [`Float`] will be the
    /// number of significant bits of the [`Rational`]'s numerator.
    ///
    /// Rounding may occur, in which case [`Nearest`] is used by default. To specify a rounding mode
    /// as well as a precision, try [`Float::from_rational_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(), prec)`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = Float::from_rational_prec(Rational::ZERO, 10);
    /// assert_eq!(x.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (x, o) = Float::from_rational_prec(Rational::from_signeds(1, 3), 10);
    /// assert_eq!(x.to_string(), "0.3335");
    /// assert_eq!(x.get_prec(), Some(10));
    /// assert_eq!(o, Greater);
    ///
    /// let (x, o) = Float::from_rational_prec(Rational::from_signeds(1, 3), 100);
    /// assert_eq!(x.to_string(), "0.3333333333333333333333333333335");
    /// assert_eq!(x.get_prec(), Some(100));
    /// assert_eq!(o, Greater);
    ///
    /// let (x, o) = Float::from_rational_prec(Rational::from_signeds(-1, 3), 10);
    /// assert_eq!(x.to_string(), "-0.3335");
    /// assert_eq!(x.get_prec(), Some(10));
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) = Float::from_rational_prec(Rational::from_signeds(-1, 3), 100);
    /// assert_eq!(x.to_string(), "-0.3333333333333333333333333333335");
    /// assert_eq!(x.get_prec(), Some(100));
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn from_rational_prec(x: Rational, prec: u64) -> (Float, Ordering) {
        Float::from_rational_prec_round(x, prec, Nearest)
    }

    /// Converts a [`Rational`] to a [`Float`], taking the [`Rational`] by reference. If the
    /// [`Float`] is nonzero, it has the specified precision. If rounding is needed, the specified
    /// rounding mode is used. An [`Ordering`] is also returned, indicating whether the returned
    /// value is less than, equal to, or greater than the original value.
    ///
    /// If you're only using [`Nearest`], try using [`Float::from_rational_prec_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(), prec)`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = Float::from_rational_prec_round_ref(&Rational::from_signeds(1, 3), 10, Floor);
    /// assert_eq!(x.to_string(), "0.333");
    /// assert_eq!(x.get_prec(), Some(10));
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) =
    ///     Float::from_rational_prec_round_ref(&Rational::from_signeds(1, 3), 10, Ceiling);
    /// assert_eq!(x.to_string(), "0.3335");
    /// assert_eq!(x.get_prec(), Some(10));
    /// assert_eq!(o, Greater);
    ///
    /// let (x, o) =
    ///     Float::from_rational_prec_round_ref(&Rational::from_signeds(1, 3), 10, Nearest);
    /// assert_eq!(x.to_string(), "0.3335");
    /// assert_eq!(x.get_prec(), Some(10));
    /// assert_eq!(o, Greater);
    ///
    /// let (x, o) = Float::from_rational_prec_round_ref(&Rational::from_signeds(-1, 3), 10, Floor);
    /// assert_eq!(x.to_string(), "-0.3335");
    /// assert_eq!(x.get_prec(), Some(10));
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) =
    ///     Float::from_rational_prec_round_ref(&Rational::from_signeds(-1, 3), 10, Ceiling);
    /// assert_eq!(x.to_string(), "-0.333");
    /// assert_eq!(x.get_prec(), Some(10));
    /// assert_eq!(o, Greater);
    ///
    /// let (x, o) =
    ///     Float::from_rational_prec_round_ref(&Rational::from_signeds(-1, 3), 10, Nearest);
    /// assert_eq!(x.to_string(), "-0.3335");
    /// assert_eq!(x.get_prec(), Some(10));
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn from_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        assert_ne!(prec, 0);
        if *x == 0 {
            (Float::ZERO, Equal)
        } else {
            let mut exponent = i32::exact_from(x.floor_log_base_2_abs());
            let (significand, o) =
                Integer::rounding_from(x << (i32::exact_from(prec) - exponent - 1), rm);
            let sign = significand >= 0;
            let mut significand = significand.unsigned_abs();
            let away_from_0 = if sign { Greater } else { Less };
            if o == away_from_0 && significand.is_power_of_2() {
                exponent += 1;
            }
            significand <<= significand
                .significant_bits()
                .neg_mod_power_of_2(Limb::LOG_WIDTH);
            let target_bits = prec
                .round_to_multiple_of_power_of_2(Limb::LOG_WIDTH, Ceiling)
                .0;
            let current_bits = significand_bits(&significand);
            if current_bits > target_bits {
                significand >>= current_bits - target_bits;
            }
            (
                Float(Finite {
                    sign,
                    exponent: exponent + 1,
                    precision: prec,
                    significand,
                }),
                o,
            )
        }
    }

    /// Converts a [`Rational`] to a [`Float`], taking the [`Rational`] by reference. If the
    /// [`Float`] is nonzero, it has the specified precision. An [`Ordering`] is also returned,
    /// indicating whether the returned value is less than, equal to, or greater than the original
    /// value.
    ///
    /// If the [`Rational`] is dyadic (its denominator is a power of 2), then you can convert it to
    /// a [`Float`] using `try_from` instead. The precision of the resulting [`Float`] will be the
    /// number of significant bits of the [`Rational`]'s numerator.
    ///
    /// Rounding may occur, in which case [`Nearest`] is used by default. To specify a rounding mode
    /// as well as a precision, try [`Float::from_rational_prec_round_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(), prec)`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = Float::from_rational_prec_ref(&Rational::ZERO, 10);
    /// assert_eq!(x.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (x, o) = Float::from_rational_prec_ref(&Rational::from_signeds(1, 3), 10);
    /// assert_eq!(x.to_string(), "0.3335");
    /// assert_eq!(x.get_prec(), Some(10));
    /// assert_eq!(o, Greater);
    ///
    /// let (x, o) = Float::from_rational_prec_ref(&Rational::from_signeds(1, 3), 100);
    /// assert_eq!(x.to_string(), "0.3333333333333333333333333333335");
    /// assert_eq!(x.get_prec(), Some(100));
    /// assert_eq!(o, Greater);
    ///
    /// let (x, o) = Float::from_rational_prec_ref(&Rational::from_signeds(-1, 3), 10);
    /// assert_eq!(x.to_string(), "-0.3335");
    /// assert_eq!(x.get_prec(), Some(10));
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) = Float::from_rational_prec_ref(&Rational::from_signeds(-1, 3), 100);
    /// assert_eq!(x.to_string(), "-0.3333333333333333333333333333335");
    /// assert_eq!(x.get_prec(), Some(100));
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn from_rational_prec_ref(x: &Rational, prec: u64) -> (Float, Ordering) {
        Float::from_rational_prec_round_ref(x, prec, Nearest)
    }
}

impl TryFrom<Rational> for Float {
    type Error = FloatFromRationalError;

    /// Converts a [`Rational`] to an [`Float`], taking the [`Rational`] by value. If the
    /// [`Rational`]'s denominator is not a power of 2, an error is returned.
    ///
    /// The [`Float`]'s precision is the number of significant bits of the numerator of the
    /// [`Rational`].
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_float::Float;
    /// use malachite_q::conversion::primitive_float_from_rational::FloatFromRationalError;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Float::try_from(Rational::ZERO).unwrap(), 0);
    /// assert_eq!(
    ///     Float::try_from(Rational::from_signeds(1, 8)).unwrap(),
    ///     0.125
    /// );
    /// assert_eq!(
    ///     Float::try_from(Rational::from_signeds(-1, 8)).unwrap(),
    ///     -0.125
    /// );
    ///
    /// assert_eq!(
    ///     Float::try_from(Rational::from_signeds(1, 3)),
    ///     Err(FloatFromRationalError)
    /// );
    /// assert_eq!(
    ///     Float::try_from(Rational::from_signeds(-1, 3)),
    ///     Err(FloatFromRationalError)
    /// );
    /// ```
    fn try_from(x: Rational) -> Result<Float, Self::Error> {
        if let Some(log_denominator) = x.denominator_ref().checked_log_base_2() {
            Ok(
                Float::from(Integer::from_sign_and_abs(x >= 0u32, x.into_numerator()))
                    >> i64::try_from(log_denominator).map_err(|_| FloatFromRationalError)?,
            )
        } else {
            Err(FloatFromRationalError)
        }
    }
}

impl<'a> TryFrom<&'a Rational> for Float {
    type Error = FloatFromRationalError;

    /// Converts a [`Rational`] to an [`Float`], taking the [`Rational`] by reference. If the
    /// [`Rational`]'s denominator is not a power of 2, an error is returned.
    ///
    /// The [`Float`]'s precision is the number of significant bits of the numerator of the
    /// [`Rational`].
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_float::Float;
    /// use malachite_q::conversion::primitive_float_from_rational::FloatFromRationalError;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Float::try_from(&Rational::ZERO).unwrap(), 0);
    /// assert_eq!(
    ///     Float::try_from(&Rational::from_signeds(1, 8)).unwrap(),
    ///     0.125
    /// );
    /// assert_eq!(
    ///     Float::try_from(&Rational::from_signeds(-1, 8)).unwrap(),
    ///     -0.125
    /// );
    ///
    /// assert_eq!(
    ///     Float::try_from(&Rational::from_signeds(1, 3)),
    ///     Err(FloatFromRationalError)
    /// );
    /// assert_eq!(
    ///     Float::try_from(&Rational::from_signeds(-1, 3)),
    ///     Err(FloatFromRationalError)
    /// );
    /// ```
    fn try_from(x: &'a Rational) -> Result<Float, Self::Error> {
        if let Some(log_denominator) = x.denominator_ref().checked_log_base_2() {
            Ok(Float::from(Integer::from_sign_and_abs_ref(
                *x >= 0u32,
                x.numerator_ref(),
            )) >> i64::try_from(log_denominator).map_err(|_| FloatFromRationalError)?)
        } else {
            Err(FloatFromRationalError)
        }
    }
}

impl<'a> ConvertibleFrom<&'a Rational> for Float {
    /// Determines whether a [`Rational`] can be converted to an [`Float`], taking the [`Rational`]
    /// by reference.
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::conversion::traits::ConvertibleFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Float::convertible_from(&Rational::ZERO), true);
    /// assert_eq!(Float::convertible_from(&Rational::from_signeds(3, 8)), true);
    /// assert_eq!(
    ///     Float::convertible_from(&Rational::from_signeds(-3, 8)),
    ///     true
    /// );
    ///
    /// assert_eq!(
    ///     Float::convertible_from(&Rational::from_signeds(1, 3)),
    ///     false
    /// );
    /// assert_eq!(
    ///     Float::convertible_from(&Rational::from_signeds(-1, 3)),
    ///     false
    /// );
    /// ```
    #[inline]
    fn convertible_from(x: &'a Rational) -> bool {
        x.denominator_ref().is_power_of_2()
    }
}
