// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::ShlRound;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::from::UnsignedFromFloatError;
use malachite_base::num::conversion::traits::{
    ConvertibleFrom, IntegerMantissaAndExponent, IsInteger, RoundingFrom,
};
use malachite_base::rounding_modes::RoundingMode::{self, *};

macro_rules! float_impls {
    ($f: ident) => {
        impl RoundingFrom<$f> for Natural {
            /// Converts a floating-point value to a [`Natural`], using the specified rounding mode.
            /// An [`Ordering`] is also returned, indicating whether the returned value is less
            /// than, equal to, or greater than the original value.
            ///
            /// The floating-point value cannot be NaN or infinite, and it cannot round to a
            /// negative integer.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `value.sci_exponent()`.
            ///
            /// # Panics
            /// Panics if `value` is NaN or infinite, if it would round to a negative integer, or if
            /// the rounding mode is `Exact` and `value` is not an integer.
            ///
            /// # Examples
            /// See [here](super::from_primitive_float#rounding_from).
            fn rounding_from(value: $f, rm: RoundingMode) -> (Self, Ordering) {
                if value.is_nan() || value == $f::INFINITY {
                    panic!("Cannot convert {} to Natural", value);
                } else if value == 0.0 {
                    (Natural::ZERO, Equal)
                } else if value < 0.0 {
                    if rm == Down || rm == Ceiling || rm == Nearest {
                        (Natural::ZERO, Greater)
                    } else {
                        panic!("Result is negative and cannot be converted to a Natural");
                    }
                } else {
                    let (mantissa, exponent) = value.integer_mantissa_and_exponent();
                    Natural::from(mantissa).shl_round(exponent, rm)
                }
            }
        }

        impl TryFrom<$f> for Natural {
            type Error = UnsignedFromFloatError;

            /// Converts a floating-point value to a [`Natural`].
            ///
            /// If the input isn't exactly equal to some [`Natural`], an error is returned.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `value.sci_exponent()`.
            ///
            /// # Examples
            /// See [here](super::from_primitive_float#try_from).
            fn try_from(value: $f) -> Result<Natural, Self::Error> {
                if value.is_nan() || value.is_infinite() {
                    Err(UnsignedFromFloatError::FloatInfiniteOrNan)
                } else if value < 0.0 {
                    Err(UnsignedFromFloatError::FloatNegative)
                } else if value == 0.0 {
                    Ok(Natural::ZERO)
                } else {
                    let (mantissa, exponent) = value.integer_mantissa_and_exponent();
                    if exponent >= 0 {
                        Ok(Natural::from(mantissa) << exponent)
                    } else {
                        Err(UnsignedFromFloatError::FloatNonIntegerOrOutOfRange)
                    }
                }
            }
        }

        impl ConvertibleFrom<$f> for Natural {
            /// Determines whether a floating-point value can be exactly converted to a [`Natural`].
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::from_primitive_float#convertible_from).
            #[inline]
            fn convertible_from(value: $f) -> bool {
                value >= 0.0 && value.is_integer()
            }
        }
    };
}
apply_to_primitive_floats!(float_impls);
