use num::arithmetic::traits::ModPowerOfTwoIsReduced;
use num::logic::traits::SignificantBits;

macro_rules! impl_mod_power_of_two_is_reduced {
    ($t:ident) => {
        impl ModPowerOfTwoIsReduced for $t {
            /// Returns whether `self` is reduced mod 2<sup>`pow`</sup>; in other words, whether it
            /// has no more than `pow` significant bits.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoIsReduced;
            ///
            /// assert_eq!(0u8.mod_power_of_two_is_reduced(5), true);
            /// assert_eq!(100u64.mod_power_of_two_is_reduced(5), false);
            /// assert_eq!(100u16.mod_power_of_two_is_reduced(8), true);
            /// ```
            #[inline]
            fn mod_power_of_two_is_reduced(&self, pow: u64) -> bool {
                self.significant_bits() <= pow
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_power_of_two_is_reduced);
