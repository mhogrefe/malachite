use num::arithmetic::traits::{DivisibleByPowerOfTwo, EqModPowerOfTwo};

macro_rules! impl_eq_mod_power_of_two {
    ($t:ident) => {
        impl EqModPowerOfTwo<$t> for $t {
            /// Returns whether `self` is equal to `other` mod 2<sup>`pow`</sup>.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::EqModPowerOfTwo;
            ///
            /// assert_eq!(0u16.eq_mod_power_of_two(256, 8), true);
            /// assert_eq!((-0b1101i32).eq_mod_power_of_two(0b11011, 3), true);
            /// assert_eq!((-0b1101i64).eq_mod_power_of_two(0b11011, 4), false);
            /// ```
            #[inline]
            fn eq_mod_power_of_two(self, other: $t, pow: u64) -> bool {
                (self ^ other).divisible_by_power_of_two(pow)
            }
        }
    };
}
apply_to_primitive_ints!(impl_eq_mod_power_of_two);
