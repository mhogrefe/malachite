// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
use core::cmp::Ordering;
use malachite_base::num::conversion::from::{SignedFromFloatError, UnsignedFromFloatError};
use malachite_base::num::conversion::traits::{ConvertibleFrom, RoundingFrom};
use malachite_base::rounding_modes::RoundingMode;

macro_rules! float_impls {
    ($f: ident) => {
        impl RoundingFrom<$f> for Integer {
            /// Converts a primitive float to an [`Integer`], using the specified rounding mode. An
            /// [`Ordering`] is also returned, indicating whether the returned value is less than,
            /// equal to, or greater than the original value.
            ///
            /// The floating-point value cannot be NaN or infinite.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `value.sci_exponent()`.
            ///
            /// # Panics
            /// Panics if `value` is NaN or infinite or if the rounding mode is `Exact` and `value`
            /// is not an integer.
            ///
            /// # Examples
            /// See [here](super::from_primitive_float#rounding_from).
            fn rounding_from(value: $f, rm: RoundingMode) -> (Self, Ordering) {
                if value >= 0.0 {
                    let (abs, o) = Natural::rounding_from(value, rm);
                    (Integer { sign: true, abs }, o)
                } else {
                    let (n, o) = Natural::rounding_from(-value, -rm);
                    (-n, o.reverse())
                }
            }
        }

        impl TryFrom<$f> for Integer {
            type Error = SignedFromFloatError;

            /// Converts a primitive float to an [`Integer`].
            ///
            /// If the input isn't exactly equal to some [`Integer`], an error is returned.
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
            fn try_from(value: $f) -> Result<Integer, Self::Error> {
                Natural::try_from(value.abs())
                    .map(|n| Integer {
                        sign: value >= 0.0,
                        abs: n,
                    })
                    .map_err(|e| match e {
                        UnsignedFromFloatError::FloatInfiniteOrNan => {
                            SignedFromFloatError::FloatInfiniteOrNan
                        }
                        UnsignedFromFloatError::FloatNonIntegerOrOutOfRange => {
                            SignedFromFloatError::FloatNonIntegerOrOutOfRange
                        }
                        _ => unreachable!(),
                    })
            }
        }

        impl ConvertibleFrom<$f> for Integer {
            /// Determines whether a primitive float can be exactly converted to an [`Integer`].
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::from_primitive_float#convertible_from).
            fn convertible_from(value: $f) -> bool {
                Natural::convertible_from(value.abs())
            }
        }
    };
}
apply_to_primitive_floats!(float_impls);
