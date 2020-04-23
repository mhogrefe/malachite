use num::arithmetic::traits::UnsignedAbs;
use num::basic::integers::PrimitiveInteger;
use num::logic::traits::{LeadingZeros, SignificantBits};

macro_rules! impl_significant_bits_unsigned {
    ($t:ident) => {
        impl SignificantBits for $t {
            /// Returns the number of significant bits of a primitive unsigned integer; this is the
            /// integer's width minus the number of leading zeros.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::logic::traits::SignificantBits;
            ///
            /// assert_eq!(0u8.significant_bits(), 0);
            /// assert_eq!(100u64.significant_bits(), 7);
            /// ```
            #[inline]
            fn significant_bits(self) -> u64 {
                Self::WIDTH - LeadingZeros::leading_zeros(self)
            }
        }
    };
}

impl_significant_bits_unsigned!(u8);
impl_significant_bits_unsigned!(u16);
impl_significant_bits_unsigned!(u32);
impl_significant_bits_unsigned!(u64);
impl_significant_bits_unsigned!(u128);
impl_significant_bits_unsigned!(usize);

macro_rules! impl_significant_bits_signed {
    ($t:ident) => {
        /// Returns the number of significant bits of a primitive signed integer; this is the
        /// integer's width minus the number of leading zeros of its absolute value.
        ///
        /// Time: worst case O(1)
        ///
        /// Additional memory: worst case O(1)
        ///
        /// # Example
        /// ```
        /// use malachite_base::num::logic::traits::SignificantBits;
        ///
        /// assert_eq!(0i8.significant_bits(), 0);
        /// assert_eq!((-100i64).significant_bits(), 7);
        /// ```
        impl SignificantBits for $t {
            #[inline]
            fn significant_bits(self) -> u64 {
                self.unsigned_abs().significant_bits()
            }
        }
    };
}

impl_significant_bits_signed!(i8);
impl_significant_bits_signed!(i16);
impl_significant_bits_signed!(i32);
impl_significant_bits_signed!(i64);
impl_significant_bits_signed!(i128);
impl_significant_bits_signed!(isize);
