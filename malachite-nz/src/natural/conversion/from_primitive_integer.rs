use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{CheckedFrom, SaturatingFrom, VecFromOtherType};

use natural::InnerNatural::Small;
use natural::Natural;
use platform::Limb;

macro_rules! impl_from_limb {
    ($t: ident) => {
        impl From<$t> for Natural {
            /// Converts a `Limb` to a `Natural`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(Natural::from(123u32).to_string(), "123");
            /// ```
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
            /// Converts a value to a `Natural`, where the value is of a primitive unsigned integer
            /// type that's smaller than a `Limb`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(Natural::from(123u8).to_string(), "123");
            /// ```
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
            /// Converts a value to a `Natural`, where the value is of a primitive unsigned integer
            /// type that's larger than a `Limb`. This implementation is general enough to also work
            /// for `usize`, regardless of whether it is equal in width to `Limb`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(Natural::from(123u64).to_string(), "123");
            /// ```
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
            /// Converts a value of signed primitive integer type to a `Natural`. If the value is
            /// negative, `None` is returned.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// extern crate malachite_base;
            ///
            /// use malachite_base::num::conversion::traits::CheckedFrom;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(format!("{:?}", Natural::checked_from(123i32)), "Some(123)");
            /// assert_eq!(format!("{:?}", Natural::checked_from(-123i32)), "None");
            /// ```
            #[inline]
            fn checked_from(i: $t) -> Option<Natural> {
                if i >= 0 {
                    Some(Natural::from(i.unsigned_abs()))
                } else {
                    None
                }
            }
        }

        impl SaturatingFrom<$t> for Natural {
            /// Converts a value of signed primitive integer type to a `Natural`. If the value is
            /// negative, 0 is returned.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// extern crate malachite_base;
            ///
            /// use malachite_base::num::conversion::traits::SaturatingFrom;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(Natural::saturating_from(123i32).to_string(), "123");
            /// assert_eq!(Natural::saturating_from(-123i32).to_string(), "0");
            /// ```
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

impl_signed!(i8);
impl_signed!(i16);
impl_signed!(i32);
impl_signed!(i64);
impl_signed!(i128);
impl_signed!(isize);
