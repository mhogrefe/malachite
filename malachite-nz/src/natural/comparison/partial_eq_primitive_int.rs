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
use malachite_base::num::conversion::traits::WrappingFrom;

macro_rules! impl_partial_eq_limb {
    ($u: ident) => {
        impl PartialEq<$u> for Natural {
            /// Determines whether a [`Natural`] is equal to a [`Limb`](crate#limbs).
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::partial_eq_primitive_int#partial_eq).
            fn eq(&self, other: &$u) -> bool {
                match *self {
                    Natural(Small(x)) => x == *other,
                    Natural(Large(_)) => false,
                }
            }
        }

        impl PartialEq<Natural> for $u {
            /// Determines whether a [`Limb`](crate#limbs) is equal to a [`Natural`].
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::partial_eq_primitive_int#partial_eq).
            #[inline]
            fn eq(&self, other: &Natural) -> bool {
                other == self
            }
        }
    };
}

macro_rules! impl_partial_eq_smaller_than_limb {
    ($u: ident) => {
        impl PartialEq<$u> for Natural {
            /// Determines whether a [`Natural`] is equal to a value of an unsigned primitive
            /// integer type that's smaller than a [`Limb`](crate#limbs).
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::partial_eq_primitive_int#partial_eq).
            #[allow(clippy::cmp_owned)]
            #[inline]
            fn eq(&self, other: &$u) -> bool {
                *self == Limb::from(*other)
            }
        }

        impl PartialEq<Natural> for $u {
            /// Determines whether a value of an unsigned primitive integer type that's smaller than
            /// a [`Limb`](crate#limbs) is equal to a [`Natural`].
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::partial_eq_primitive_int#partial_eq).
            #[allow(clippy::cmp_owned)]
            #[inline]
            fn eq(&self, other: &Natural) -> bool {
                Limb::from(*self) == *other
            }
        }
    };
}

macro_rules! impl_partial_eq_larger_than_limb_or_usize {
    ($u: ident) => {
        impl PartialEq<Natural> for $u {
            /// Determines whether a value of an unsigned primitive integer type that's larger than
            /// a [`Limb`](crate#limbs) is equal to a [`Natural`].
            ///
            /// This implementation is general enough to also work for [`usize`], regardless of
            /// whether it is equal in width to [`Limb`](crate#limbs).
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::partial_eq_primitive_int#partial_eq).
            #[inline]
            fn eq(&self, other: &Natural) -> bool {
                other == self
            }
        }
    };
}

macro_rules! impl_partial_eq_larger_than_limb {
    ($u: ident) => {
        impl_partial_eq_larger_than_limb_or_usize!($u);

        impl PartialEq<$u> for Natural {
            /// Determines whether a [`Natural`] is equal to a value of an unsigned primitive
            /// integer type that's larger than a [`Limb`](crate#limbs).
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::partial_eq_primitive_int#partial_eq).
            #[inline]
            fn eq(&self, other: &$u) -> bool {
                let mut other = *other;
                for limb in self.limbs() {
                    if other == 0 || limb != Limb::wrapping_from(other) {
                        return false;
                    }
                    other >>= Limb::WIDTH;
                }
                other == 0
            }
        }
    };
}

macro_rules! impl_signed {
    ($t: ident) => {
        impl PartialEq<$t> for Natural {
            /// Determines whether a [`Natural`] is equal to a signed primitive integer.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::partial_eq_primitive_int#partial_eq).
            fn eq(&self, other: &$t) -> bool {
                *other >= 0 && *self == other.unsigned_abs()
            }
        }

        impl PartialEq<Natural> for $t {
            /// Determines whether a signed primitive integer is equal to a [`Natural`].
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::partial_eq_primitive_int#partial_eq).
            #[inline]
            fn eq(&self, other: &Natural) -> bool {
                other == self
            }
        }
    };
}

impl_partial_eq_smaller_than_limb!(u8);
impl_partial_eq_smaller_than_limb!(u16);
#[cfg(feature = "32_bit_limbs")]
impl_partial_eq_limb!(u32);
#[cfg(not(feature = "32_bit_limbs"))]
impl_partial_eq_smaller_than_limb!(u32);
#[cfg(feature = "32_bit_limbs")]
impl_partial_eq_larger_than_limb!(u64);
#[cfg(not(feature = "32_bit_limbs"))]
impl_partial_eq_limb!(u64);
impl_partial_eq_larger_than_limb!(u128);
impl_partial_eq_larger_than_limb_or_usize!(usize);

apply_to_signeds!(impl_signed);

impl PartialEq<usize> for Natural {
    /// Determines whether a [`Natural`] is equal to a [`usize`].
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// See [here](super::partial_eq_primitive_int#partial_eq).
    #[inline]
    fn eq(&self, other: &usize) -> bool {
        if USIZE_IS_U32 {
            *self == u32::wrapping_from(*other)
        } else {
            assert_eq!(usize::WIDTH, u64::WIDTH);
            *self == u64::wrapping_from(*other)
        }
    }
}
