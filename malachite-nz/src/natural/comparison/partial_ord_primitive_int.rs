use malachite_base::num::arithmetic::traits::ShrRound;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode;
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;
use std::cmp::Ordering;

macro_rules! impl_partial_ord_limb {
    ($u: ident) => {
        impl PartialOrd<$u> for Natural {
            /// Compares a `Natural` to a `Limb`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_nz::natural::Natural;
            ///
            /// assert!(Natural::from(123u32) > 122u32);
            /// assert!(Natural::from(123u32) >= 122u32);
            /// assert!(Natural::from(123u32) < 124u32);
            /// assert!(Natural::from(123u32) <= 124u32);
            /// assert!(Natural::trillion() > 123u32);
            /// assert!(Natural::trillion() >= 123u32);
            /// ```
            fn partial_cmp(&self, other: &$u) -> Option<Ordering> {
                match *self {
                    Natural(Small(small)) => small.partial_cmp(other),
                    Natural(Large(_)) => Some(Ordering::Greater),
                }
            }
        }

        impl PartialOrd<Natural> for $u {
            /// Compares a `Limb` to a `Natural`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_nz::natural::Natural;
            ///
            /// assert!(122u32 < Natural::from(123u32));
            /// assert!(122u32 <= Natural::from(123u32));
            /// assert!(124u32 > Natural::from(123u32));
            /// assert!(124u32 >= Natural::from(123u32));
            /// assert!(123u32 < Natural::trillion());
            /// assert!(123u32 <= Natural::trillion());
            /// ```
            #[inline]
            fn partial_cmp(&self, other: &Natural) -> Option<Ordering> {
                other.partial_cmp(self).map(Ordering::reverse)
            }
        }
    };
}

macro_rules! impl_partial_ord_smaller_than_limb {
    ($u: ident) => {
        impl PartialOrd<$u> for Natural {
            /// Compares a `Natural` to a value of a primitive unsigned integer type that's smaller
            /// than a `Limb`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_nz::natural::Natural;
            ///
            /// assert!(Natural::from(123u32) > 122u8);
            /// assert!(Natural::from(123u32) >= 122u8);
            /// assert!(Natural::from(123u32) < 124u8);
            /// assert!(Natural::from(123u32) <= 124u8);
            /// assert!(Natural::trillion() > 123u8);
            /// assert!(Natural::trillion() >= 123u8);
            /// ```
            #[inline]
            fn partial_cmp(&self, other: &$u) -> Option<Ordering> {
                self.partial_cmp(&Limb::from(*other))
            }
        }

        impl PartialOrd<Natural> for $u {
            /// Compares a value of a primitive unsigned integer type that's smaller than a `Limb`
            /// to a `Natural`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_nz::natural::Natural;
            ///
            /// assert!(122u8 < Natural::from(123u32));
            /// assert!(122u8 <= Natural::from(123u32));
            /// assert!(124u8 > Natural::from(123u32));
            /// assert!(124u8 >= Natural::from(123u32));
            /// assert!(123u8 < Natural::trillion());
            /// assert!(123u8 <= Natural::trillion());
            /// ```
            #[inline]
            fn partial_cmp(&self, other: &Natural) -> Option<Ordering> {
                other.partial_cmp(self).map(Ordering::reverse)
            }
        }
    };
}

macro_rules! impl_partial_ord_larger_than_limb_or_usize {
    ($u: ident) => {
        impl PartialOrd<Natural> for $u {
            /// Compares a value of a primitive unsigned integer type that's larger than a
            /// `Limb` to a `Natural`. This implementation is general enough to also work for
            /// `usize`, regardless of whether it is equal in width to `Limb`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_nz::natural::Natural;
            ///
            /// assert!(122u64 < Natural::from(123u32));
            /// assert!(122u64 <= Natural::from(123u32));
            /// assert!(124u64 > Natural::from(123u32));
            /// assert!(124u64 >= Natural::from(123u32));
            /// assert!(123u64 < Natural::trillion());
            /// assert!(123u64 <= Natural::trillion());
            /// ```
            #[inline]
            fn partial_cmp(&self, other: &Natural) -> Option<Ordering> {
                other.partial_cmp(self).map(Ordering::reverse)
            }
        }
    };
}

macro_rules! impl_partial_ord_larger_than_limb {
    ($u: ident) => {
        impl_partial_ord_larger_than_limb_or_usize!($u);

        impl PartialOrd<$u> for Natural {
            /// Compares a `Natural` to a value of a primitive unsigned integer type that's larger
            /// than a `Limb`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_nz::natural::Natural;
            ///
            /// assert!(Natural::from(123u32) > 122u64);
            /// assert!(Natural::from(123u32) >= 122u64);
            /// assert!(Natural::from(123u32) < 124u64);
            /// assert!(Natural::from(123u32) <= 124u64);
            /// assert!(Natural::trillion() > 123u64);
            /// assert!(Natural::trillion() >= 123u64);
            /// ```
            #[inline]
            fn partial_cmp(&self, other: &$u) -> Option<Ordering> {
                let limb_count = other
                    .significant_bits()
                    .shr_round(Limb::LOG_WIDTH, RoundingMode::Ceiling);
                let limb_count_cmp = self.limb_count().cmp(&limb_count);
                if limb_count_cmp != Ordering::Equal || limb_count == 0 {
                    return Some(limb_count_cmp);
                }
                let width = Limb::WIDTH;
                let mut i = limb_count << Limb::LOG_WIDTH;
                let mut mask = $u::from(Limb::MAX) << (i - width);
                for limb in self.limbs().rev() {
                    i -= width;
                    let limb_cmp = limb.cmp(&Limb::wrapping_from((other & mask) >> i));
                    if limb_cmp != Ordering::Equal {
                        return Some(limb_cmp);
                    }
                    mask >>= width;
                }
                Some(Ordering::Equal)
            }
        }
    };
}

macro_rules! impl_signed {
    ($t: ident) => {
        impl PartialOrd<$t> for Natural {
            /// Compares a `Natural` to a a value of signed primitive integer type.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_nz::natural::Natural;
            ///
            /// assert!(Natural::from(123u32) > 122i64);
            /// assert!(Natural::from(123u32) >= 122i64);
            /// assert!(Natural::from(123u32) < 124i64);
            /// assert!(Natural::from(123u32) <= 124i64);
            /// assert!(Natural::from(123u32) > -124i64);
            /// assert!(Natural::from(123u32) >= -124i64);
            /// assert!(Natural::trillion() > 123i64);
            /// assert!(Natural::trillion() >= 123i64);
            /// ```
            fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
                if *other < 0 {
                    Some(Ordering::Greater)
                } else {
                    self.partial_cmp(&other.unsigned_abs())
                }
            }
        }

        impl PartialOrd<Natural> for $t {
            /// Compares a value of signed primitive integer type to a `Natural`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_nz::natural::Natural;
            ///
            /// assert!(122i64 < Natural::from(123u32));
            /// assert!(122i64 <= Natural::from(123u32));
            /// assert!(124i64 > Natural::from(123u32));
            /// assert!(124i64 >= Natural::from(123u32));
            /// assert!(-124i64 < Natural::from(123u32));
            /// assert!(-124i64 <= Natural::from(123u32));
            /// assert!(123i64 < Natural::trillion());
            /// assert!(123i64 <= Natural::trillion());
            /// ```
            #[inline]
            fn partial_cmp(&self, other: &Natural) -> Option<Ordering> {
                other.partial_cmp(self).map(Ordering::reverse)
            }
        }
    };
}

impl_partial_ord_smaller_than_limb!(u8);
impl_partial_ord_smaller_than_limb!(u16);
#[cfg(feature = "32_bit_limbs")]
impl_partial_ord_limb!(u32);
#[cfg(not(feature = "32_bit_limbs"))]
impl_partial_ord_smaller_than_limb!(u32);
#[cfg(feature = "32_bit_limbs")]
impl_partial_ord_larger_than_limb!(u64);
#[cfg(not(feature = "32_bit_limbs"))]
impl_partial_ord_limb!(u64);
impl_partial_ord_larger_than_limb!(u128);
impl_partial_ord_larger_than_limb_or_usize!(usize);

apply_to_signeds!(impl_signed);

impl PartialOrd<usize> for Natural {
    /// Compares a `Natural` to a `usize`.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    #[inline]
    fn partial_cmp(&self, other: &usize) -> Option<Ordering> {
        if usize::WIDTH == u32::WIDTH {
            self.partial_cmp(&u32::wrapping_from(*other))
        } else {
            assert_eq!(usize::WIDTH, u64::WIDTH);
            self.partial_cmp(&u64::wrapping_from(*other))
        }
    }
}
