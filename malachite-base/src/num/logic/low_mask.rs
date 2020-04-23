use comparison::Max;
use num::arithmetic::traits::PowerOfTwo;
use num::basic::integers::PrimitiveInteger;
use num::logic::traits::LowMask;

macro_rules! impl_low_mask_unsigned {
    ($t:ident) => {
        impl LowMask for $t {
            /// Returns a value with the least significant `bits` bits on and the remaining bits
            /// off.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `pow` is greater than the width of `$t`.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::LowMask;
            ///
            /// assert_eq!(u16::low_mask(0), 0);
            /// assert_eq!(u8::low_mask(3), 0x7);
            /// assert_eq!(u8::low_mask(8), 0xff);
            /// assert_eq!(u64::low_mask(40), 0xff_ffff_ffff);
            /// ```
            #[inline]
            fn low_mask(bits: u64) -> $t {
                assert!(bits <= $t::WIDTH);
                if bits == $t::WIDTH {
                    $t::MAX
                } else {
                    $t::power_of_two(bits) - 1
                }
            }
        }
    };
}

impl_low_mask_unsigned!(u8);
impl_low_mask_unsigned!(u16);
impl_low_mask_unsigned!(u32);
impl_low_mask_unsigned!(u64);
impl_low_mask_unsigned!(u128);
impl_low_mask_unsigned!(usize);

macro_rules! impl_low_mask_signed {
    ($t:ident) => {
        impl LowMask for $t {
            /// Returns a value with the least significant `bits` bits on and the remaining bits
            /// off.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `pow` is greater than the width of `$t`.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::LowMask;
            ///
            /// assert_eq!(u16::low_mask(0), 0);
            /// assert_eq!(u8::low_mask(3), 0x7);
            /// assert_eq!(u8::low_mask(8), 0xff);
            /// assert_eq!(u64::low_mask(40), 0xff_ffff_ffff);
            /// ```
            #[inline]
            fn low_mask(bits: u64) -> $t {
                assert!(bits <= $t::WIDTH);
                if bits == $t::WIDTH {
                    -1
                } else if bits == $t::WIDTH - 1 {
                    $t::MAX
                } else {
                    $t::power_of_two(bits) - 1
                }
            }
        }
    };
}

impl_low_mask_signed!(i8);
impl_low_mask_signed!(i16);
impl_low_mask_signed!(i32);
impl_low_mask_signed!(i64);
impl_low_mask_signed!(i128);
impl_low_mask_signed!(isize);
