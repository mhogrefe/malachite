use num::arithmetic::traits::{CeilingLogBase2, CheckedLogBase2, FloorLogBase2};
use num::basic::integers::PrimitiveInt;
use num::logic::traits::{LeadingZeros, SignificantBits, TrailingZeros};

macro_rules! impl_arithmetic_traits {
    ($t:ident) => {
        impl CheckedLogBase2 for $t {
            #[inline]
            fn checked_log_base_2(self) -> Option<u64> {
                if self == 0 {
                    panic!("Cannot take the base-2 logarithm of 0.");
                }
                let leading_zeros = LeadingZeros::leading_zeros(self);
                let trailing_zeros = TrailingZeros::trailing_zeros(self);
                if leading_zeros + trailing_zeros == $t::WIDTH - 1 {
                    Some(trailing_zeros)
                } else {
                    None
                }
            }
        }

        impl FloorLogBase2 for $t {
            /// Returns the floor of the base-2 logarithm of a positive primitive unsigned integer.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `self` is 0.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::FloorLogBase2;
            ///
            /// assert_eq!(1u8.floor_log_base_2(), 0);
            /// assert_eq!(100u64.floor_log_base_2(), 6);
            /// ```
            #[inline]
            fn floor_log_base_2(self) -> u64 {
                if self == 0 {
                    panic!("Cannot take the base-2 logarithm of 0.");
                }
                self.significant_bits() - 1
            }
        }

        impl CeilingLogBase2 for $t {
            /// Returns the ceiling of the base-2 logarithm of a positive primitive unsigned
            /// integer.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `self` is 0.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::CeilingLogBase2;
            ///
            /// assert_eq!(1u8.ceiling_log_base_2(), 0);
            /// assert_eq!(100u64.ceiling_log_base_2(), 7);
            /// ```
            #[inline]
            fn ceiling_log_base_2(self) -> u64 {
                let floor_log_base_2 = self.floor_log_base_2();
                if self.is_power_of_two() {
                    floor_log_base_2
                } else {
                    floor_log_base_2 + 1
                }
            }
        }
    };
}
apply_to_unsigneds!(impl_arithmetic_traits);
