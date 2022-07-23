use crate::natural::InnerNatural::Small;
use crate::natural::Natural;
use crate::platform::Limb;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, SaturatingFrom, VecFromOtherType,
};

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

macro_rules! impl_signed {
    ($t: ident) => {
        impl CheckedFrom<$t> for Natural {
            /// Converts a signed primitive integer to a [`Natural`]. If the integer is negative,
            /// `None` is returned.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::from_primitive_int#checked_from).
            #[inline]
            fn checked_from(i: $t) -> Option<Natural> {
                if i >= 0 {
                    Some(Natural::from(i.unsigned_abs()))
                } else {
                    None
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
