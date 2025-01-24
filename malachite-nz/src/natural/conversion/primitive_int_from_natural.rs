// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::Limb;
use malachite_base::num::basic::integers::{PrimitiveInt, USIZE_IS_U32};
use malachite_base::num::conversion::traits::{
    ConvertibleFrom, FromOtherTypeSlice, OverflowingFrom, SaturatingFrom, WrappingFrom,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UnsignedFromNaturalError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SignedFromNaturalError;

macro_rules! impl_from_limb {
    ($u: ident, $s: ident) => {
        impl<'a> TryFrom<&'a Natural> for $u {
            type Error = UnsignedFromNaturalError;

            /// Converts a [`Natural`] to a [`Limb`](crate#limbs), returning an error if the
            /// [`Natural`] is too large.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_natural#try_from).
            fn try_from(value: &Natural) -> Result<$u, Self::Error> {
                match *value {
                    Natural(Small(small)) => Ok(small),
                    Natural(Large(_)) => Err(UnsignedFromNaturalError),
                }
            }
        }

        impl<'a> WrappingFrom<&'a Natural> for $u {
            /// Converts a [`Natural`] to a [`Limb`](crate#limbs), wrapping modulo $2^W$, where $W$
            /// is the width of a limb.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_natural#wrapping_from).
            fn wrapping_from(value: &Natural) -> $u {
                match *value {
                    Natural(Small(small)) => small,
                    Natural(Large(ref limbs)) => limbs[0],
                }
            }
        }

        impl<'a> SaturatingFrom<&'a Natural> for $u {
            /// Converts a [`Natural`] to a [`Limb`](crate#limbs).
            ///
            /// If the [`Natural`] is too large to fit in a [`Limb`](crate#limbs), the maximum
            /// representable value is returned.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_natural#saturating_from).
            fn saturating_from(value: &Natural) -> $u {
                match *value {
                    Natural(Small(small)) => small,
                    Natural(Large(_)) => $u::MAX,
                }
            }
        }

        impl<'a> OverflowingFrom<&'a Natural> for $u {
            /// Converts a [`Natural`] to a [`Limb`](crate#limbs), wrapping modulo $2^W$, where $W$
            /// is the width of a limb.
            ///
            /// The returned boolean value indicates whether wrapping occurred.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_natural#overflowing_from).
            fn overflowing_from(value: &Natural) -> ($u, bool) {
                match *value {
                    Natural(Small(small)) => (small, false),
                    Natural(Large(ref limbs)) => (limbs[0], true),
                }
            }
        }

        impl<'a> ConvertibleFrom<&'a Natural> for $u {
            /// Determines whether a [`Natural`] can be converted to a [`Limb`](crate#limbs).
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_natural#convertible_from).
            fn convertible_from(value: &Natural) -> bool {
                match *value {
                    Natural(Small(_)) => true,
                    Natural(Large(_)) => false,
                }
            }
        }

        impl<'a> TryFrom<&'a Natural> for $s {
            type Error = SignedFromNaturalError;

            /// Converts a [`Natural`] to a `SignedLimb` (the signed type whose width is the same as
            /// a [limb](crate#limbs)'s), returning an error if the [`Natural`] is too large.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_natural#try_from).
            fn try_from(value: &Natural) -> Result<$s, Self::Error> {
                match *value {
                    Natural(Small(small)) => {
                        $s::try_from(small).map_err(|_| SignedFromNaturalError)
                    }
                    Natural(Large(_)) => Err(SignedFromNaturalError),
                }
            }
        }

        impl<'a> WrappingFrom<&'a Natural> for $s {
            /// Converts a [`Natural`] to a `SignedLimb` (the signed type whose width is the same as
            /// a [limb](crate#limbs)'s), wrapping modulo $2^W$, where $W$ is the width of a limb.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_natural#wrapping_from).
            #[inline]
            fn wrapping_from(value: &Natural) -> $s {
                $s::wrapping_from($u::wrapping_from(value))
            }
        }

        impl<'a> SaturatingFrom<&'a Natural> for $s {
            /// Converts a [`Natural`] to a `SignedLimb` (the signed type whose width is the same as
            /// a [limb](crate#limbs)'s).
            ///
            /// If the [`Natural`] is too large to fit in a `SignedLimb`, the largest representable
            /// value is returned.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_natural#saturating_from).
            #[inline]
            fn saturating_from(value: &Natural) -> $s {
                $s::saturating_from($u::saturating_from(value))
            }
        }

        impl<'a> OverflowingFrom<&'a Natural> for $s {
            /// Converts a [`Natural`] to a `SignedLimb` (the signed type whose width is the same as
            /// a [limb](crate#limbs)'s), wrapping modulo $2^W$, where $W$ is the width of a limb.
            ///
            /// The returned boolean value indicates whether wrapping occurred.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_natural#overflowing_from).
            fn overflowing_from(value: &Natural) -> ($s, bool) {
                let (result, overflow_1) = $u::overflowing_from(value);
                let (result, overflow_2) = $s::overflowing_from(result);
                (result, overflow_1 || overflow_2)
            }
        }

        impl<'a> ConvertibleFrom<&'a Natural> for $s {
            /// Determines whether a [`Natural`] can be converted to a `SignedLimb` (the signed type
            /// whose width is the same as a [limb](crate#limbs)'s).
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_natural#convertible_from).
            fn convertible_from(value: &Natural) -> bool {
                match *value {
                    Natural(Small(small)) => $s::convertible_from(small),
                    Natural(Large(_)) => false,
                }
            }
        }
    };
}

macro_rules! impl_from_smaller_than_limb {
    ($u: ident, $s: ident) => {
        impl<'a> TryFrom<&'a Natural> for $u {
            type Error = UnsignedFromNaturalError;

            /// Converts a [`Natural`] to a value of an unsigned primitive integer type that's
            /// smaller than a [`Limb`](crate#limbs), returning an error if the [`Natural`] is too
            /// large.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_natural#try_from).
            #[inline]
            fn try_from(value: &Natural) -> Result<$u, Self::Error> {
                Limb::try_from(value)
                    .map_err(|_| UnsignedFromNaturalError)
                    .and_then(|x| $u::try_from(x).map_err(|_| UnsignedFromNaturalError))
            }
        }

        impl<'a> WrappingFrom<&'a Natural> for $u {
            /// Converts a [`Natural`] to a value of an unsigned primitive integer type that's
            /// smaller than a [`Limb`](crate#limbs), wrapping modulo $2^W$, where $W$ is the width
            /// of a limb.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_natural#wrapping_from).
            #[inline]
            fn wrapping_from(value: &Natural) -> $u {
                $u::wrapping_from(Limb::wrapping_from(value))
            }
        }

        impl<'a> SaturatingFrom<&'a Natural> for $u {
            /// Converts a [`Natural`] to a value of an unsigned primitive integer type that's
            /// smaller than a [`Limb`](crate#limbs). If the [`Natural`] is too large to fit in the
            /// output type, the largest representable value is returned.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_natural#saturating_from).
            #[inline]
            fn saturating_from(value: &Natural) -> $u {
                $u::saturating_from(Limb::saturating_from(value))
            }
        }

        impl<'a> OverflowingFrom<&'a Natural> for $u {
            /// Converts a [`Natural`] to a value of an unsigned primitive integer type that's
            /// smaller than a [`Limb`](crate#limbs), wrapping modulo $2^W$, where $W$ is the width
            /// of a limb.
            ///
            /// The returned boolean value indicates whether wrapping occurred.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_natural#overflowing_from).
            #[inline]
            fn overflowing_from(value: &Natural) -> ($u, bool) {
                let (result, overflow_1) = Limb::overflowing_from(value);
                let (result, overflow_2) = $u::overflowing_from(result);
                (result, overflow_1 || overflow_2)
            }
        }

        impl<'a> ConvertibleFrom<&'a Natural> for $u {
            /// Determines whether a [`Natural`] can be converted to a value of a primitive unsigned
            /// integer type that's smaller than a [`Limb`](crate#limbs).
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_natural#convertible_from).
            fn convertible_from(value: &Natural) -> bool {
                match *value {
                    Natural(Small(small)) => $u::convertible_from(small),
                    Natural(Large(_)) => false,
                }
            }
        }

        impl<'a> TryFrom<&'a Natural> for $s {
            type Error = SignedFromNaturalError;

            /// Converts a [`Natural`] to a value of a signed primitive integer type that's smaller
            /// than a [`Limb`](crate#limbs), returning an error if the [`Natural`] is too large.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_natural#try_from).
            #[inline]
            fn try_from(value: &Natural) -> Result<$s, Self::Error> {
                Limb::try_from(value)
                    .map_err(|_| SignedFromNaturalError)
                    .and_then(|x| $s::try_from(x).map_err(|_| SignedFromNaturalError))
            }
        }

        impl<'a> WrappingFrom<&'a Natural> for $s {
            /// Converts a [`Natural`] to a value of a signed primitive integer type that's smaller
            /// than a [`Limb`](crate#limbs), wrapping modulo $2^W$, where $W$ is the width of a
            /// limb.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_natural#wrapping_from).
            #[inline]
            fn wrapping_from(value: &Natural) -> $s {
                $s::wrapping_from(Limb::wrapping_from(value))
            }
        }

        impl<'a> SaturatingFrom<&'a Natural> for $s {
            /// Converts a [`Natural`] to a value of an unsigned primitive integer type that's
            /// smaller than a [`Limb`](crate#limbs). If the [`Natural`] is too large to fit in the
            /// output type, the largest representable value is returned.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_natural#saturating_from).
            #[inline]
            fn saturating_from(value: &Natural) -> $s {
                $s::saturating_from(Limb::saturating_from(value))
            }
        }

        impl<'a> OverflowingFrom<&'a Natural> for $s {
            /// Converts a [`Natural`] to a value of a signed primitive integer type that's smaller
            /// than a [`Limb`](crate#limbs), wrapping modulo $2^W$, where $W$ is the width of a
            /// limb.
            ///
            /// The returned boolean value indicates whether wrapping occurred.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_natural#overflowing_from).
            #[inline]
            fn overflowing_from(value: &Natural) -> ($s, bool) {
                let (result, overflow_1) = Limb::overflowing_from(value);
                let (result, overflow_2) = $s::overflowing_from(result);
                (result, overflow_1 || overflow_2)
            }
        }

        impl<'a> ConvertibleFrom<&'a Natural> for $s {
            /// Determines whether a [`Natural`] can be converted to a value of a signed primitive
            /// integer type that's smaller than a [`Limb`](crate#limbs).
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_natural#convertible_from).
            fn convertible_from(value: &Natural) -> bool {
                match *value {
                    Natural(Small(small)) => $s::convertible_from(small),
                    Natural(Large(_)) => false,
                }
            }
        }
    };
}

macro_rules! impl_from_larger_than_limb_or_xsize {
    ($u: ident, $s: ident) => {
        impl<'a> WrappingFrom<&'a Natural> for $u {
            /// Converts a [`Natural`] to a [`usize`] or a value of an unsigned primitive integer
            /// type that's larger than a [`Limb`](crate#limbs), wrapping modulo $2^W$, where $W$ is
            /// the width of a limb.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_natural#wrapping_from).
            fn wrapping_from(value: &Natural) -> $u {
                match *value {
                    Natural(Small(small)) => $u::wrapping_from(small),
                    Natural(Large(ref limbs)) => $u::from_other_type_slice(limbs),
                }
            }
        }

        impl<'a> TryFrom<&'a Natural> for $s {
            type Error = SignedFromNaturalError;

            /// Converts a [`Natural`] to an [`isize`] or value of a signed primitive integer type
            /// that's larger than a [`Limb`](crate#limbs), returning an error if the [`Natural`] is
            /// too large.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_natural#try_from).
            #[inline]
            fn try_from(value: &Natural) -> Result<$s, Self::Error> {
                $u::try_from(value)
                    .map_err(|_| SignedFromNaturalError)
                    .and_then(|x| $s::try_from(x).map_err(|_| SignedFromNaturalError))
            }
        }

        impl<'a> WrappingFrom<&'a Natural> for $s {
            /// Converts a [`Natural`] to an [`isize`] or a value of a signed primitive integer type
            /// that's larger than a [`Limb`](crate#limbs), wrapping modulo $2^W$, where $W$ is the
            /// width of a limb.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_natural#wrapping_from).
            #[inline]
            fn wrapping_from(value: &Natural) -> $s {
                $s::wrapping_from($u::wrapping_from(value))
            }
        }

        impl<'a> SaturatingFrom<&'a Natural> for $s {
            /// Converts a [`Natural`] to an [`isize`] or a value of a signed primitive integer type
            /// that's larger than a [`Limb`](crate#limbs), If the [`Natural`] is too large to fit
            /// in the output type, the largest representable value is returned.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_natural#saturating_from).
            #[inline]
            fn saturating_from(value: &Natural) -> $s {
                $s::saturating_from($u::saturating_from(value))
            }
        }

        impl<'a> OverflowingFrom<&'a Natural> for $s {
            /// Converts a [`Natural`] to an [`isize`] or a value of a signed primitive integer type
            /// that's larger than a [`Limb`](crate#limbs), wrapping modulo $2^W$, where $W$ is the
            /// width of a limb.
            ///
            /// The returned boolean value indicates whether wrapping occurred.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_natural#overflowing_from).
            fn overflowing_from(value: &Natural) -> ($s, bool) {
                let (result, overflow_1) = $u::overflowing_from(value);
                let (result, overflow_2) = $s::overflowing_from(result);
                (result, overflow_1 || overflow_2)
            }
        }
    };
}

macro_rules! impl_from_larger_than_limb {
    ($u: ident, $s: ident) => {
        impl_from_larger_than_limb_or_xsize!($u, $s);

        impl<'a> TryFrom<&'a Natural> for $u {
            type Error = UnsignedFromNaturalError;

            /// Converts a [`Natural`] to a value of an unsigned primitive integer type that's
            /// larger than a [`Limb`](crate#limbs), returning an error if the [`Natural`] is too
            /// large.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_natural#try_from).
            fn try_from(value: &Natural) -> Result<$u, Self::Error> {
                const SIZE_RATIO: usize = 1 << ($u::LOG_WIDTH - Limb::LOG_WIDTH);
                match *value {
                    Natural(Small(small)) => Ok($u::from(small)),
                    Natural(Large(ref limbs)) if limbs.len() <= SIZE_RATIO => {
                        Ok($u::from_other_type_slice(limbs))
                    }
                    Natural(Large(_)) => Err(UnsignedFromNaturalError),
                }
            }
        }

        impl<'a> SaturatingFrom<&'a Natural> for $u {
            /// Converts a [`Natural`] to a value of an unsigned primitive integer type that's
            /// larger than a [`Limb`](crate#limbs). If the [`Natural`] is too large to fit in the
            /// output type, the largest representable value is returned.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_natural#saturating_from).
            fn saturating_from(value: &Natural) -> $u {
                const SIZE_RATIO: usize = 1 << ($u::LOG_WIDTH - Limb::LOG_WIDTH);
                match *value {
                    Natural(Small(small)) => $u::from(small),
                    Natural(Large(ref limbs)) if limbs.len() <= SIZE_RATIO => {
                        $u::from_other_type_slice(limbs)
                    }
                    Natural(Large(_)) => $u::MAX,
                }
            }
        }

        impl<'a> OverflowingFrom<&'a Natural> for $u {
            /// Converts a [`Natural`] to a value of an unsigned primitive integer type that's
            /// larger than a [`Limb`](crate#limbs), wrapping modulo $2^W$, where $W$ is the width
            /// of a limb.
            ///
            /// The returned boolean value indicates whether wrapping occurred.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_natural#overflowing_from).
            fn overflowing_from(value: &Natural) -> ($u, bool) {
                const SIZE_RATIO: usize = 1 << ($u::LOG_WIDTH - Limb::LOG_WIDTH);
                match *value {
                    Natural(Small(small)) => ($u::from(small), false),
                    Natural(Large(ref limbs)) => {
                        ($u::from_other_type_slice(limbs), limbs.len() > SIZE_RATIO)
                    }
                }
            }
        }

        impl<'a> ConvertibleFrom<&'a Natural> for $u {
            /// Determines whether a [`Natural`] can be converted to a value of a primitive unsigned
            /// integer type that's larger than a [`Limb`](crate#limbs).
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_natural#convertible_from).
            fn convertible_from(value: &Natural) -> bool {
                const SIZE_RATIO: usize = 1 << ($u::LOG_WIDTH - Limb::LOG_WIDTH);
                match *value {
                    Natural(Small(_)) => true,
                    Natural(Large(ref limbs)) => limbs.len() <= SIZE_RATIO,
                }
            }
        }

        impl<'a> ConvertibleFrom<&'a Natural> for $s {
            /// Determines whether a [`Natural`] can be converted to a value of a signed primitive
            /// integer type that's larger than a [`Limb`](crate#limbs).
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_natural#convertible_from).
            fn convertible_from(value: &Natural) -> bool {
                const SIZE_RATIO: usize = 1 << ($u::LOG_WIDTH - Limb::LOG_WIDTH);
                match *value {
                    Natural(Small(_)) => true,
                    Natural(Large(ref limbs)) => {
                        limbs.len() < SIZE_RATIO
                            || limbs.len() == SIZE_RATIO && !limbs[SIZE_RATIO - 1].get_highest_bit()
                    }
                }
            }
        }
    };
}

impl TryFrom<&Natural> for usize {
    type Error = UnsignedFromNaturalError;

    /// Converts a [`Natural`] to a [`usize`], returning an error if the [`Natural`] is too large.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See [here](super::primitive_int_from_natural#try_from).
    fn try_from(value: &Natural) -> Result<usize, Self::Error> {
        if USIZE_IS_U32 {
            u32::try_from(value).map(usize::wrapping_from)
        } else {
            assert_eq!(usize::WIDTH, u64::WIDTH);
            u64::try_from(value).map(usize::wrapping_from)
        }
    }
}

impl SaturatingFrom<&Natural> for usize {
    /// Converts a [`Natural`] to a [`usize`]. If the [`Natural`] is too large to fit in a
    /// [`usize`], the largest representable value is returned.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See [here](super::primitive_int_from_natural#saturating_from).
    fn saturating_from(value: &Natural) -> usize {
        if USIZE_IS_U32 {
            usize::wrapping_from(u32::saturating_from(value))
        } else {
            assert_eq!(usize::WIDTH, u64::WIDTH);
            usize::wrapping_from(u64::saturating_from(value))
        }
    }
}

impl OverflowingFrom<&Natural> for usize {
    /// Converts a [`Natural`] to a [`usize`], wrapping modulo $2^W$, where $W$ is the width of a
    /// limb.
    ///
    /// The returned boolean value indicates whether wrapping occurred.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See [here](super::primitive_int_from_natural#overflowing_from).
    fn overflowing_from(value: &Natural) -> (usize, bool) {
        if USIZE_IS_U32 {
            let (result, overflow) = u32::overflowing_from(value);
            (usize::wrapping_from(result), overflow)
        } else {
            assert_eq!(usize::WIDTH, u64::WIDTH);
            let (result, overflow) = u64::overflowing_from(value);
            (usize::wrapping_from(result), overflow)
        }
    }
}

impl ConvertibleFrom<&Natural> for usize {
    /// Determines whether a [`Natural`] can be converted to a [`usize`].
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See [here](super::primitive_int_from_natural#convertible_from).
    fn convertible_from(value: &Natural) -> bool {
        if USIZE_IS_U32 {
            u32::convertible_from(value)
        } else {
            assert_eq!(usize::WIDTH, u64::WIDTH);
            u64::convertible_from(value)
        }
    }
}

impl ConvertibleFrom<&Natural> for isize {
    /// Determines whether a [`Natural`] can be converted to an [`isize`].
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See [here](super::primitive_int_from_natural#convertible_from).
    fn convertible_from(value: &Natural) -> bool {
        if USIZE_IS_U32 {
            i32::convertible_from(value)
        } else {
            assert_eq!(usize::WIDTH, u64::WIDTH);
            i64::convertible_from(value)
        }
    }
}

impl_from_smaller_than_limb!(u8, i8);
impl_from_smaller_than_limb!(u16, i16);
#[cfg(feature = "32_bit_limbs")]
impl_from_limb!(u32, i32);
#[cfg(not(feature = "32_bit_limbs"))]
impl_from_smaller_than_limb!(u32, i32);
#[cfg(feature = "32_bit_limbs")]
impl_from_larger_than_limb!(u64, i64);
#[cfg(not(feature = "32_bit_limbs"))]
impl_from_limb!(u64, i64);
impl_from_larger_than_limb!(u128, i128);
impl_from_larger_than_limb_or_xsize!(usize, isize);
