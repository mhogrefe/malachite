// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::big_exponent_float::BigExponentFloat;
use crate::float::conversion::primitive_float_from_float::FloatFromFloatError;
use core::cmp::Ordering;
use malachite_base::num::conversion::traits::{ConvertibleFrom, RoundingFrom};
use malachite_base::rounding_modes::RoundingMode;

// These conversions go through a `Float` whose exponent is clamped to `Float`'s range. A primitive
// float's range lies well inside `Float`'s, so clamping changes neither the rounded value nor the
// direction of rounding.
macro_rules! impl_primitive_float_from {
    ($t: ident) => {
        impl RoundingFrom<BigExponentFloat> for $t {
            /// Converts a [`BigExponentFloat`] to a primitive float, using a specified
            /// [`RoundingMode`] and taking the [`BigExponentFloat`] by value. An [`Ordering`] is
            /// also returned, indicating whether the returned value is less than, equal to, or
            /// greater than the original value. (Although a NaN is not comparable to anything,
            /// converting a NaN to a NaN will also return `Equal`.)
            ///
            /// Overflow, underflow, and rounding behave as in the conversion from a
            /// [`Float`](crate::Float) with the same value; see
            /// [`RoundingFrom<Float>`](crate::float::conversion::primitive_float_from_float).
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
            ///
            /// # Panics
            /// Panics if the [`BigExponentFloat`] is not exactly equal to any float of the target
            /// type, and `rm` is `Exact`.
            ///
            /// # Examples
            /// See [here](super::primitive_float_from_big_exponent_float#rounding_from).
            #[inline]
            fn rounding_from(x: BigExponentFloat, rm: RoundingMode) -> ($t, Ordering) {
                $t::rounding_from(x.into_float_clamped(), rm)
            }
        }

        impl RoundingFrom<&BigExponentFloat> for $t {
            /// Converts a [`BigExponentFloat`] to a primitive float, using a specified
            /// [`RoundingMode`] and taking the [`BigExponentFloat`] by reference. An [`Ordering`]
            /// is also returned, indicating whether the returned value is less than, equal to, or
            /// greater than the original value. (Although a NaN is not comparable to anything,
            /// converting a NaN to a NaN will also return `Equal`.)
            ///
            /// Overflow, underflow, and rounding behave as in the conversion from a
            /// [`Float`](crate::Float) with the same value; see
            /// [`RoundingFrom<Float>`](crate::float::conversion::primitive_float_from_float).
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
            ///
            /// # Panics
            /// Panics if the [`BigExponentFloat`] is not exactly equal to any float of the target
            /// type, and `rm` is `Exact`.
            ///
            /// # Examples
            /// See [here](super::primitive_float_from_big_exponent_float#rounding_from).
            #[inline]
            fn rounding_from(x: &BigExponentFloat, rm: RoundingMode) -> ($t, Ordering) {
                $t::rounding_from(x.to_float_clamped(), rm)
            }
        }

        impl TryFrom<BigExponentFloat> for $t {
            type Error = FloatFromFloatError;

            /// Converts a [`BigExponentFloat`] to a primitive float, taking the
            /// [`BigExponentFloat`] by value. If the [`BigExponentFloat`] is not equal to a
            /// primitive float of the given type, an error is returned.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::primitive_float_from_big_exponent_float#try_from).
            #[inline]
            fn try_from(x: BigExponentFloat) -> Result<$t, Self::Error> {
                $t::try_from(x.into_float_clamped())
            }
        }

        impl TryFrom<&BigExponentFloat> for $t {
            type Error = FloatFromFloatError;

            /// Converts a [`BigExponentFloat`] to a primitive float, taking the
            /// [`BigExponentFloat`] by reference. If the [`BigExponentFloat`] is not equal to a
            /// primitive float of the given type, an error is returned.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::primitive_float_from_big_exponent_float#try_from).
            #[inline]
            fn try_from(x: &BigExponentFloat) -> Result<$t, Self::Error> {
                $t::try_from(x.to_float_clamped())
            }
        }

        impl ConvertibleFrom<&BigExponentFloat> for $t {
            /// Determines whether a [`BigExponentFloat`] can be converted to a primitive float,
            /// taking the [`BigExponentFloat`] by reference.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::primitive_float_from_big_exponent_float#convertible_from).
            #[inline]
            fn convertible_from(x: &BigExponentFloat) -> bool {
                $t::convertible_from(&x.to_float_clamped())
            }
        }
    };
}
apply_to_primitive_floats!(impl_primitive_float_from);
