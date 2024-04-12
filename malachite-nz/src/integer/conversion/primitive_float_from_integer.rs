// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use core::cmp::Ordering;
use malachite_base::num::conversion::traits::{ConvertibleFrom, RoundingFrom};
use malachite_base::rounding_modes::RoundingMode;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PrimitiveFloatFromIntegerError;

macro_rules! float_impls {
    ($f: ident) => {
        impl<'a> RoundingFrom<&'a Integer> for $f {
            /// Converts an [`Integer`] to a primitive float according to a specified
            /// [`RoundingMode`]. An [`Ordering`] is also returned, indicating whether the returned
            /// value is less than, equal to, or greater than the original value.
            ///
            /// - If the rounding mode is `Floor` the largest float less than or equal to the
            ///   [`Integer`] is returned. If the [`Integer`] is greater than the maximum finite
            ///   float, then the maximum finite float is returned. If it is smaller than the
            ///   minimum finite float, then negative infinity is returned.
            /// - If the rounding mode is `Ceiling`, the smallest float greater than or equal to the
            ///   [`Integer`] is returned. If the [`Integer`] is greater than the maximum finite
            ///   float, then positive infinity is returned. If it is smaller than the minimum
            ///   finite float, then the minimum finite float is returned.
            /// - If the rounding mode is `Down`, then the rounding proceeds as with `Floor` if the
            ///   [`Integer`] is non-negative and as with `Ceiling` if the [`Integer`] is negative.
            /// - If the rounding mode is `Up`, then the rounding proceeds as with `Ceiling` if the
            ///   [`Integer`] is non-negative and as with `Floor` if the [`Integer`] is negative.
            /// - If the rounding mode is `Nearest`, then the nearest float is returned. If the
            ///   [`Integer`] is exactly between two floats, the float with the zero
            ///   least-significant bit in its representation is selected. If the [`Integer`] is
            ///   greater than the maximum finite float, then the maximum finite float is returned.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `value.significant_bits()`.
            ///
            /// # Panics
            /// Panics if the rounding mode is `Exact` and `value` cannot be represented exactly.
            ///
            /// # Examples
            /// See [here](super::primitive_float_from_integer#rounding_from).
            fn rounding_from(value: &'a Integer, rm: RoundingMode) -> ($f, Ordering) {
                if value.sign {
                    $f::rounding_from(&value.abs, rm)
                } else {
                    let (f, o) = $f::rounding_from(&value.abs, -rm);
                    (-f, o.reverse())
                }
            }
        }

        impl<'a> TryFrom<&'a Integer> for $f {
            type Error = PrimitiveFloatFromIntegerError;

            /// Converts an [`Integer`] to a primitive float.
            ///
            /// If the input isn't exactly equal to some float, an error is returned.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `value.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::primitive_float_from_integer#try_from).
            fn try_from(value: &'a Integer) -> Result<$f, Self::Error> {
                $f::try_from(&value.abs)
                    .map(|f| if value.sign { f } else { -f })
                    .map_err(|_| PrimitiveFloatFromIntegerError)
            }
        }

        impl<'a> ConvertibleFrom<&'a Integer> for $f {
            /// Determines whether an [`Integer`] can be exactly converted to a primitive float.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `value.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::primitive_float_from_integer#convertible_from).
            fn convertible_from(value: &'a Integer) -> bool {
                $f::convertible_from(&value.abs)
            }
        }
    };
}
apply_to_primitive_floats!(float_impls);
