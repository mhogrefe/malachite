// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::InnerNatural::Small;
use crate::natural::Natural;
use crate::platform::Limb;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{ConvertibleFrom, SaturatingFrom, VecFromOtherType};

impl Natural {
    /// Converts a [`Limb`](crate#limbs) to a [`Natural`].
    ///
    /// This function is const, so it may be used to define constants.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::natural::Natural;
    ///
    /// const TEN: Natural = Natural::const_from(10);
    /// assert_eq!(TEN, 10);
    /// ```
    pub const fn const_from(x: Limb) -> Natural {
        Natural(Small(x))
    }
}

macro_rules! impl_from_limb {
    ($t: ident) => {
        impl From<$t> for Natural {
            /// Converts a [`Limb`](crate#limbs) to a [`Natural`].
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::from_primitive_int#from).
            #[inline]
            fn from(u: $t) -> Natural {
                Natural(Small(u))
            }
        }
    };
}

macro_rules! impl_from_smaller_than_limb {
    ($t: ident) => {
        impl From<$t> for Natural {
            /// Converts an unsigned primitive integer to a [`Natural`], where the integer's width
            /// is smaller than a [`Limb`](crate#limbs)'s.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::from_primitive_int#from).
            #[inline]
            fn from(u: $t) -> Natural {
                Natural(Small(Limb::from(u)))
            }
        }
    };
}

macro_rules! impl_from_larger_than_limb_or_usize {
    ($t: ident) => {
        impl From<$t> for Natural {
            /// Converts an unsigned primitive integer to a [`Natural`], where the integer's width
            /// is larger than a [`Limb`](crate#limbs)'s.
            ///
            /// This implementation is general enough to also work for [`usize`], regardless of
            /// whether it is equal in width to [`Limb`](crate#limbs).
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::from_primitive_int#from).
            #[inline]
            fn from(u: $t) -> Natural {
                Natural::from_owned_limbs_asc(Limb::vec_from_other_type(u))
            }
        }
    };
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NaturalFromSignedError;

macro_rules! impl_signed {
    ($t: ident) => {
        impl TryFrom<$t> for Natural {
            type Error = NaturalFromSignedError;

            /// Converts a signed primitive integer to a [`Natural`]. If the integer is negative, an
            /// error is returned.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::from_primitive_int#try_from).
            #[inline]
            fn try_from(i: $t) -> Result<Natural, Self::Error> {
                if i >= 0 {
                    Ok(Natural::from(i.unsigned_abs()))
                } else {
                    Err(NaturalFromSignedError)
                }
            }
        }

        impl ConvertibleFrom<$t> for Natural {
            /// Determines whether a signed primitive integer can be converted to a [`Natural`].
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::from_primitive_int#convertible_from).
            #[inline]
            fn convertible_from(i: $t) -> bool {
                i >= 0
            }
        }

        impl SaturatingFrom<$t> for Natural {
            /// Converts a signed primitive primitive integer to a [`Natural`]. If the integer is
            /// negative, 0 is returned.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::from_primitive_int#saturating_from).
            #[inline]
            fn saturating_from(i: $t) -> Natural {
                if i >= 0 {
                    Natural::from(i.unsigned_abs())
                } else {
                    Natural::ZERO
                }
            }
        }
    };
}

impl_from_smaller_than_limb!(u8);
impl_from_smaller_than_limb!(u16);
#[cfg(feature = "32_bit_limbs")]
impl_from_limb!(u32);
#[cfg(not(feature = "32_bit_limbs"))]
impl_from_smaller_than_limb!(u32);
#[cfg(feature = "32_bit_limbs")]
impl_from_larger_than_limb_or_usize!(u64);
#[cfg(not(feature = "32_bit_limbs"))]
impl_from_limb!(u64);
impl_from_larger_than_limb_or_usize!(u128);
impl_from_larger_than_limb_or_usize!(usize);

apply_to_signeds!(impl_signed);
