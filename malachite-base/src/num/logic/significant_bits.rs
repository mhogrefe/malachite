use num::arithmetic::traits::UnsignedAbs;
use num::basic::integers::PrimitiveInt;
use num::logic::traits::{LeadingZeros, SignificantBits};

fn _significant_bits_unsigned<T: PrimitiveInt>(x: T) -> u64 {
    T::WIDTH - LeadingZeros::leading_zeros(x)
}

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
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::SignificantBits;
            ///
            /// assert_eq!(0u8.significant_bits(), 0);
            /// assert_eq!(100u64.significant_bits(), 7);
            /// ```
            #[inline]
            fn significant_bits(self) -> u64 {
                _significant_bits_unsigned(self)
            }
        }
    };
}
apply_to_unsigneds!(impl_significant_bits_unsigned);

fn _significant_bits_signed<U: SignificantBits, S: UnsignedAbs<Output = U>>(x: S) -> u64 {
    x.unsigned_abs().significant_bits()
}

macro_rules! impl_significant_bits_signed {
    ($u:ident, $s:ident) => {
        /// Returns the number of significant bits of a primitive signed integer; this is the
        /// integer's width minus the number of leading zeros of its absolute value.
        ///
        /// Time: worst case O(1)
        ///
        /// Additional memory: worst case O(1)
        ///
        /// # Examples
        /// ```
        /// use malachite_base::num::logic::traits::SignificantBits;
        ///
        /// assert_eq!(0i8.significant_bits(), 0);
        /// assert_eq!((-100i64).significant_bits(), 7);
        /// ```
        impl SignificantBits for $s {
            #[inline]
            fn significant_bits(self) -> u64 {
                _significant_bits_signed(self)
            }
        }
    };
}
apply_to_unsigned_signed_pair!(impl_significant_bits_signed);
