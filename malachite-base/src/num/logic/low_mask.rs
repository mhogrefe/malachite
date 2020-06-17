use num::basic::integers::PrimitiveInteger;
use num::basic::traits::NegativeOne;
use num::logic::traits::LowMask;

#[inline]
pub fn _low_mask_unsigned<T: PrimitiveInteger>(bits: u64) -> T {
    assert!(bits <= T::WIDTH);
    if bits == T::WIDTH {
        T::MAX
    } else {
        T::power_of_two(bits) - T::ONE
    }
}

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
                _low_mask_unsigned(bits)
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

pub fn _low_mask_signed<T: NegativeOne + PrimitiveInteger>(bits: u64) -> T {
    assert!(bits <= T::WIDTH);
    if bits == T::WIDTH {
        T::NEGATIVE_ONE
    } else if bits == T::WIDTH - 1 {
        T::MAX
    } else {
        T::power_of_two(bits) - T::ONE
    }
}

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
                _low_mask_signed(bits)
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
