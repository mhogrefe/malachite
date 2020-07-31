use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::conversion::traits::WrappingFrom;

use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;

macro_rules! impl_partial_eq_limb {
    ($u: ident) => {
        impl PartialEq<$u> for Natural {
            /// Determines whether a `Natural` is equal to a `Limb`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_nz::natural::Natural;
            ///
            /// assert!(Natural::from(123u32) == 123u32);
            /// assert!(Natural::from(123u32) != 5u32);
            /// ```
            fn eq(&self, other: &$u) -> bool {
                match *self {
                    Natural(Small(x)) => x == *other,
                    Natural(Large(_)) => false,
                }
            }
        }

        impl PartialEq<Natural> for $u {
            /// Determines whether a `Limb` is equal to a `Natural`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_nz::natural::Natural;
            ///
            /// assert!(123u32 == Natural::from(123u32));
            /// assert!(5u32 != Natural::from(123u32));
            /// ```
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
            /// Determines whether a `Natural` is equal to a value of a primitive unsigned integer
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
            /// assert!(Natural::from(123u32) == 123u8);
            /// assert!(Natural::from(123u32) != 5u8);
            /// ```
            #[inline]
            fn eq(&self, other: &$u) -> bool {
                *self == Limb::from(*other)
            }
        }

        impl PartialEq<Natural> for $u {
            /// Determines whether a value of a primitive unsigned integer type that's smaller than
            /// a `Limb` is equal to a `Natural`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_nz::natural::Natural;
            ///
            /// assert!(123u8 == Natural::from(123u32));
            /// assert!(5u8 != Natural::from(123u32));
            /// ```
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
            /// Determines whether a value of a primitive unsigned integer type that's larger than a
            /// `Limb` is equal to a `Natural`. This implementation is general enough to also work
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
            /// assert!(123u64 == Natural::from(123u32));
            /// assert!(5u64 != Natural::from(123u32));
            /// ```
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
            /// Determines whether a `Natural` is equal to a value of a primitive unsigned integer
            /// type that's larger than a `Limb`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_nz::natural::Natural;
            ///
            /// assert!(Natural::from(123u32) == 123u64);
            /// assert!(Natural::from(123u32) != 5u64);
            /// ```
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
            /// Determines whether a `Natural` is equal to a a value of signed primitive integer
            /// type.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_nz::natural::Natural;
            ///
            /// assert!(Natural::from(123u32) == 123i64);
            /// assert!(Natural::from(123u32) != -123i64);
            /// ```
            fn eq(&self, other: &$t) -> bool {
                *other >= 0 && *self == other.unsigned_abs()
            }
        }

        impl PartialEq<Natural> for $t {
            /// Determines whether a value of signed primitive integer type is equal to a `Natural`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_nz::natural::Natural;
            ///
            /// assert!(123i64 == Natural::from(123u32));
            /// assert!(-123i64 != Natural::from(123u32));
            /// ```
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
    /// Determines whether a `Natural` is equal to a `usize`.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    #[inline]
    fn eq(&self, other: &usize) -> bool {
        if usize::WIDTH == u32::WIDTH {
            *self == u32::wrapping_from(*other)
        } else {
            assert_eq!(usize::WIDTH, u64::WIDTH);
            *self == u64::wrapping_from(*other)
        }
    }
}
