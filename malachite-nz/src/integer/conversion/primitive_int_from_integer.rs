// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
use core::ops::Neg;
use malachite_base::comparison::traits::Min;
use malachite_base::num::arithmetic::traits::{DivisibleByPowerOf2, WrappingNeg};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{
    ConvertibleFrom, OverflowingFrom, SaturatingFrom, WrappingFrom,
};
use malachite_base::num::logic::traits::SignificantBits;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UnsignedFromIntegerError;

fn try_from_unsigned<'a, T: TryFrom<&'a Natural>>(
    value: &'a Integer,
) -> Result<T, UnsignedFromIntegerError> {
    match *value {
        Integer { sign: false, .. } => Err(UnsignedFromIntegerError),
        Integer {
            sign: true,
            ref abs,
        } => T::try_from(abs).map_err(|_| UnsignedFromIntegerError),
    }
}

fn wrapping_from_unsigned<'a, T: WrappingFrom<&'a Natural> + WrappingNeg<Output = T>>(
    value: &'a Integer,
) -> T {
    match *value {
        Integer {
            sign: true,
            ref abs,
        } => T::wrapping_from(abs),
        Integer {
            sign: false,
            ref abs,
        } => T::wrapping_from(abs).wrapping_neg(),
    }
}

fn saturating_from_unsigned<'a, T: Copy + SaturatingFrom<&'a Natural> + Zero>(
    value: &'a Integer,
) -> T {
    match *value {
        Integer {
            sign: true,
            ref abs,
        } => T::saturating_from(abs),
        _ => T::ZERO,
    }
}

fn overflowing_from_unsigned<
    'a,
    T: OverflowingFrom<&'a Natural> + WrappingFrom<&'a Natural> + WrappingNeg<Output = T>,
>(
    value: &'a Integer,
) -> (T, bool) {
    match *value {
        Integer {
            sign: true,
            ref abs,
        } => T::overflowing_from(abs),
        Integer {
            sign: false,
            ref abs,
        } => (T::wrapping_from(abs).wrapping_neg(), true),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SignedFromIntegerError;

fn try_from_signed<'a, T: ConvertibleFrom<&'a Integer> + WrappingFrom<&'a Integer>>(
    value: &'a Integer,
) -> Result<T, SignedFromIntegerError> {
    if T::convertible_from(value) {
        Ok(T::wrapping_from(value))
    } else {
        Err(SignedFromIntegerError)
    }
}

fn saturating_from_signed<
    'a,
    U: PrimitiveInt + SaturatingFrom<&'a Natural>,
    S: Min + Neg<Output = S> + SaturatingFrom<U> + WrappingFrom<U>,
>(
    value: &'a Integer,
) -> S {
    match *value {
        Integer {
            sign: true,
            ref abs,
        } => S::saturating_from(U::saturating_from(abs)),
        Integer {
            sign: false,
            ref abs,
        } => {
            let abs = U::saturating_from(abs);
            if abs.get_highest_bit() {
                S::MIN
            } else {
                -S::wrapping_from(abs)
            }
        }
    }
}

fn convertible_from_signed<T: PrimitiveInt>(value: &Integer) -> bool {
    match *value {
        Integer {
            sign: true,
            ref abs,
        } => abs.significant_bits() < T::WIDTH,
        Integer {
            sign: false,
            ref abs,
        } => {
            let significant_bits = abs.significant_bits();
            significant_bits < T::WIDTH
                || significant_bits == T::WIDTH && abs.divisible_by_power_of_2(T::WIDTH - 1)
        }
    }
}

macro_rules! impl_from {
    ($u: ident, $s: ident) => {
        impl<'a> TryFrom<&'a Integer> for $u {
            type Error = UnsignedFromIntegerError;

            /// Converts an [`Integer`] to an unsigned primitive integer, returning an error if the
            /// [`Integer`] cannot be represented.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_integer#try_from).
            #[inline]
            fn try_from(value: &Integer) -> Result<$u, Self::Error> {
                try_from_unsigned(value)
            }
        }

        impl<'a> WrappingFrom<&'a Integer> for $u {
            /// Converts an [`Integer`] to an unsigned primitive integer, wrapping modulo $2^W$,
            /// where $W$ is the width of the primitive integer.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_integer#wrapping_from).
            #[inline]
            fn wrapping_from(value: &Integer) -> $u {
                wrapping_from_unsigned(value)
            }
        }

        impl<'a> SaturatingFrom<&'a Integer> for $u {
            /// Converts an [`Integer`] to an unsigned primitive integer.
            ///
            /// If the [`Integer`] cannot be represented by the output type, then either zero or the
            /// maximum representable value is returned, whichever is closer.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_integer#saturating_from).
            #[inline]
            fn saturating_from(value: &Integer) -> $u {
                saturating_from_unsigned(value)
            }
        }

        impl<'a> OverflowingFrom<&'a Integer> for $u {
            /// Converts an [`Integer`] to an unsigned primitive integer, wrapping modulo $2^W$,
            /// where $W$ is the width of the primitive integer.
            ///
            /// The returned boolean value indicates whether wrapping occurred.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_integer#overflowing_from).
            #[inline]
            fn overflowing_from(value: &Integer) -> ($u, bool) {
                overflowing_from_unsigned(value)
            }
        }

        impl<'a> ConvertibleFrom<&'a Integer> for $u {
            /// Determines whether an [`Integer`] can be converted to an unsigned primitive integer.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_integer#convertible_from).
            #[inline]
            fn convertible_from(value: &Integer) -> bool {
                value.sign && $u::convertible_from(&value.abs)
            }
        }

        impl<'a> TryFrom<&'a Integer> for $s {
            type Error = SignedFromIntegerError;

            /// Converts an [`Integer`] to a signed primitive integer, returning an error if the
            /// [`Integer`] cannot be represented.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_integer#try_from).
            #[inline]
            fn try_from(value: &Integer) -> Result<$s, Self::Error> {
                try_from_signed(value)
            }
        }

        impl<'a> WrappingFrom<&'a Integer> for $s {
            /// Converts an [`Integer`] to a signed primitive integer, wrapping modulo $2^W$, where
            /// $W$ is the width of the primitive integer.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_integer#wrapping_from).
            #[inline]
            fn wrapping_from(value: &Integer) -> $s {
                $s::wrapping_from($u::wrapping_from(value))
            }
        }

        impl<'a> SaturatingFrom<&'a Integer> for $s {
            /// Converts an [`Integer`] to a signed primitive integer.
            ///
            /// If the [`Integer`] cannot be represented by the output type, then either the maximum
            /// or the minimum representable value is returned, whichever is closer.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_integer#saturating_from).
            #[inline]
            fn saturating_from(value: &Integer) -> $s {
                saturating_from_signed::<$u, $s>(value)
            }
        }

        impl<'a> OverflowingFrom<&'a Integer> for $s {
            /// Converts an [`Integer`] to a signed primitive integer, wrapping modulo $2^W$, where
            /// $W$ is the width of the primitive integer.
            ///
            /// The returned boolean value indicates whether wrapping occurred.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_integer#overflowing_from).
            #[inline]
            fn overflowing_from(value: &Integer) -> ($s, bool) {
                ($s::wrapping_from(value), !$s::convertible_from(value))
            }
        }

        impl<'a> ConvertibleFrom<&'a Integer> for $s {
            /// Determines whether an [`Integer`] can be converted to a signed primitive integer.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_integer#convertible_from).
            #[inline]
            fn convertible_from(value: &Integer) -> bool {
                convertible_from_signed::<$u>(value)
            }
        }
    };
}
apply_to_unsigned_signed_pairs!(impl_from);
