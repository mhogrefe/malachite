use num::basic::integers::PrimitiveInt;
use num::basic::traits::NegativeOne;
use num::logic::traits::LowMask;

fn _low_mask_unsigned<T: PrimitiveInt>(bits: u64) -> T {
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
            /// assert_eq!(u64::low_mask(40), 0xffffffffff);
            /// ```
            #[inline]
            fn low_mask(bits: u64) -> $t {
                _low_mask_unsigned(bits)
            }
        }
    };
}
apply_to_unsigneds!(impl_low_mask_unsigned);

fn _low_mask_signed<T: NegativeOne + PrimitiveInt>(bits: u64) -> T {
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
            /// assert_eq!(u64::low_mask(40), 0xffffffffff);
            /// ```
            #[inline]
            fn low_mask(bits: u64) -> $t {
                _low_mask_signed(bits)
            }
        }
    };
}
apply_to_signeds!(impl_low_mask_signed);
