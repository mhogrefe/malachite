// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::big_exponent_float::BigExponentFloat;
use core::cmp::Ordering;
use malachite_base::num::conversion::from::{SignedFromFloatError, UnsignedFromFloatError};
use malachite_base::num::conversion::traits::{ConvertibleFrom, RoundingFrom};
use malachite_base::rounding_modes::RoundingMode;

// These conversions go through a `Float` whose exponent is clamped to `Float`'s range. A primitive
// integer's range lies well inside `Float`'s, so clamping changes neither the rounded value nor the
// direction of rounding.
macro_rules! impl_unsigned_from {
    ($t: ident) => {
        impl RoundingFrom<BigExponentFloat> for $t {
            /// Converts a [`BigExponentFloat`] to an unsigned primitive integer, using a specified
            /// [`RoundingMode`] and taking the [`BigExponentFloat`] by value. An [`Ordering`] is
            /// also returned, indicating whether the returned value is less than, equal to, or
            /// greater than the original value.
            ///
            /// Negative values, overflow, and rounding behave as in the conversion from a
            /// [`Float`](crate::Float) with the same value; see
            /// [`RoundingFrom<Float>`](crate::float::conversion::primitive_int_from_float).
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
            ///
            /// # Panics
            /// Panics if the [`BigExponentFloat`] is NaN, or under the same conditions as the
            /// conversion from a [`Float`](crate::Float) with the same value.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_big_exponent_float#rounding_from).
            #[inline]
            fn rounding_from(x: BigExponentFloat, rm: RoundingMode) -> ($t, Ordering) {
                $t::rounding_from(x.into_float_clamped(), rm)
            }
        }

        impl RoundingFrom<&BigExponentFloat> for $t {
            /// Converts a [`BigExponentFloat`] to an unsigned primitive integer, using a specified
            /// [`RoundingMode`] and taking the [`BigExponentFloat`] by reference. An [`Ordering`]
            /// is also returned, indicating whether the returned value is less than, equal to, or
            /// greater than the original value.
            ///
            /// Negative values, overflow, and rounding behave as in the conversion from a
            /// [`Float`](crate::Float) with the same value; see
            /// [`RoundingFrom<Float>`](crate::float::conversion::primitive_int_from_float).
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
            ///
            /// # Panics
            /// Panics if the [`BigExponentFloat`] is NaN, or under the same conditions as the
            /// conversion from a [`Float`](crate::Float) with the same value.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_big_exponent_float#rounding_from).
            #[inline]
            fn rounding_from(x: &BigExponentFloat, rm: RoundingMode) -> ($t, Ordering) {
                $t::rounding_from(x.to_float_clamped(), rm)
            }
        }

        impl TryFrom<BigExponentFloat> for $t {
            type Error = UnsignedFromFloatError;

            /// Converts a [`BigExponentFloat`] to an unsigned primitive integer, taking the
            /// [`BigExponentFloat`] by value. If the [`BigExponentFloat`] is not equal to a
            /// primitive integer of the given type, an error is returned.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_big_exponent_float#try_from).
            #[inline]
            fn try_from(x: BigExponentFloat) -> Result<$t, Self::Error> {
                $t::try_from(x.into_float_clamped())
            }
        }

        impl TryFrom<&BigExponentFloat> for $t {
            type Error = UnsignedFromFloatError;

            /// Converts a [`BigExponentFloat`] to an unsigned primitive integer, taking the
            /// [`BigExponentFloat`] by reference. If the [`BigExponentFloat`] is not equal to a
            /// primitive integer of the given type, an error is returned.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_big_exponent_float#try_from).
            #[inline]
            fn try_from(x: &BigExponentFloat) -> Result<$t, Self::Error> {
                $t::try_from(x.to_float_clamped())
            }
        }

        impl ConvertibleFrom<&BigExponentFloat> for $t {
            /// Determines whether a [`BigExponentFloat`] can be converted to an unsigned primitive
            /// integer, taking the [`BigExponentFloat`] by reference.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_big_exponent_float#convertible_from).
            #[inline]
            fn convertible_from(x: &BigExponentFloat) -> bool {
                $t::convertible_from(&x.to_float_clamped())
            }
        }
    };
}
apply_to_unsigneds!(impl_unsigned_from);

macro_rules! impl_signed_from {
    ($t: ident) => {
        impl RoundingFrom<BigExponentFloat> for $t {
            /// Converts a [`BigExponentFloat`] to a signed primitive integer, using a specified
            /// [`RoundingMode`] and taking the [`BigExponentFloat`] by value. An [`Ordering`] is
            /// also returned, indicating whether the returned value is less than, equal to, or
            /// greater than the original value.
            ///
            /// Negative values, overflow, and rounding behave as in the conversion from a
            /// [`Float`](crate::Float) with the same value; see
            /// [`RoundingFrom<Float>`](crate::float::conversion::primitive_int_from_float).
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
            ///
            /// # Panics
            /// Panics if the [`BigExponentFloat`] is NaN, or under the same conditions as the
            /// conversion from a [`Float`](crate::Float) with the same value.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_big_exponent_float#rounding_from).
            #[inline]
            fn rounding_from(x: BigExponentFloat, rm: RoundingMode) -> ($t, Ordering) {
                $t::rounding_from(x.into_float_clamped(), rm)
            }
        }

        impl RoundingFrom<&BigExponentFloat> for $t {
            /// Converts a [`BigExponentFloat`] to a signed primitive integer, using a specified
            /// [`RoundingMode`] and taking the [`BigExponentFloat`] by reference. An [`Ordering`]
            /// is also returned, indicating whether the returned value is less than, equal to, or
            /// greater than the original value.
            ///
            /// Negative values, overflow, and rounding behave as in the conversion from a
            /// [`Float`](crate::Float) with the same value; see
            /// [`RoundingFrom<Float>`](crate::float::conversion::primitive_int_from_float).
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
            ///
            /// # Panics
            /// Panics if the [`BigExponentFloat`] is NaN, or under the same conditions as the
            /// conversion from a [`Float`](crate::Float) with the same value.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_big_exponent_float#rounding_from).
            #[inline]
            fn rounding_from(x: &BigExponentFloat, rm: RoundingMode) -> ($t, Ordering) {
                $t::rounding_from(x.to_float_clamped(), rm)
            }
        }

        impl TryFrom<BigExponentFloat> for $t {
            type Error = SignedFromFloatError;

            /// Converts a [`BigExponentFloat`] to a signed primitive integer, taking the
            /// [`BigExponentFloat`] by value. If the [`BigExponentFloat`] is not equal to a
            /// primitive integer of the given type, an error is returned.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_big_exponent_float#try_from).
            #[inline]
            fn try_from(x: BigExponentFloat) -> Result<$t, Self::Error> {
                $t::try_from(x.into_float_clamped())
            }
        }

        impl TryFrom<&BigExponentFloat> for $t {
            type Error = SignedFromFloatError;

            /// Converts a [`BigExponentFloat`] to a signed primitive integer, taking the
            /// [`BigExponentFloat`] by reference. If the [`BigExponentFloat`] is not equal to a
            /// primitive integer of the given type, an error is returned.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_big_exponent_float#try_from).
            #[inline]
            fn try_from(x: &BigExponentFloat) -> Result<$t, Self::Error> {
                $t::try_from(x.to_float_clamped())
            }
        }

        impl ConvertibleFrom<&BigExponentFloat> for $t {
            /// Determines whether a [`BigExponentFloat`] can be converted to a signed primitive
            /// integer, taking the [`BigExponentFloat`] by reference.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_big_exponent_float#convertible_from).
            #[inline]
            fn convertible_from(x: &BigExponentFloat) -> bool {
                $t::convertible_from(&x.to_float_clamped())
            }
        }
    };
}
apply_to_signeds!(impl_signed_from);
