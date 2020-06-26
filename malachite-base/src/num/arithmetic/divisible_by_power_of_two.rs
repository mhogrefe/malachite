use num::arithmetic::traits::{DivisibleByPowerOfTwo, ModPowerOfTwo};
use num::conversion::traits::WrappingFrom;

macro_rules! impl_divisible_by_power_of_two_unsigned {
    ($t:ident) => {
        impl DivisibleByPowerOfTwo for $t {
            /// Returns whether `self` is divisible by 2<sup>`pow`</sup>.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::DivisibleByPowerOfTwo;
            ///
            /// assert_eq!(0u8.divisible_by_power_of_two(100), true);
            /// assert_eq!(96u16.divisible_by_power_of_two(5), true);
            /// assert_eq!(96u32.divisible_by_power_of_two(6), false);
            /// ```
            #[inline]
            fn divisible_by_power_of_two(self, pow: u64) -> bool {
                self.mod_power_of_two(pow) == 0
            }
        }
    };
}
apply_to_unsigneds!(impl_divisible_by_power_of_two_unsigned);

macro_rules! impl_divisible_by_power_of_two_signed {
    ($u:ident, $s:ident) => {
        impl DivisibleByPowerOfTwo for $s {
            /// Returns whether `self` is divisible by 2<sup>`pow`</sup>.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::DivisibleByPowerOfTwo;
            ///
            /// assert_eq!(0i8.divisible_by_power_of_two(100), true);
            /// assert_eq!((-96i16).divisible_by_power_of_two(5), true);
            /// assert_eq!(96i32.divisible_by_power_of_two(6), false);
            /// ```
            #[inline]
            fn divisible_by_power_of_two(self, pow: u64) -> bool {
                $u::wrapping_from(self).divisible_by_power_of_two(pow)
            }
        }
    };
}
apply_to_unsigned_signed_pair!(impl_divisible_by_power_of_two_signed);
