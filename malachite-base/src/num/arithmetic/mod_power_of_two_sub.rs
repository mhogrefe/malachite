use num::arithmetic::traits::{
    ModPowerOfTwo, ModPowerOfTwoAssign, ModPowerOfTwoSub, ModPowerOfTwoSubAssign, WrappingSubAssign,
};
use num::basic::integers::PrimitiveInteger;

macro_rules! impl_mod_power_of_two_sub {
    ($t:ident) => {
        impl ModPowerOfTwoSub for $t {
            type Output = $t;

            /// Computes `self - rhs` mod 2<sup>`pow`</sup>. Assumes the inputs are already reduced
            /// mod 2<sup>`pow`</sup>.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoSub;
            ///
            /// assert_eq!(5u8.mod_power_of_two_sub(2, 5), 3);
            /// assert_eq!(10u32.mod_power_of_two_sub(14, 4), 12);
            /// ```
            #[inline]
            fn mod_power_of_two_sub(self, rhs: $t, pow: u64) -> $t {
                assert!(pow <= $t::WIDTH);
                self.wrapping_sub(rhs).mod_power_of_two(pow)
            }
        }

        impl ModPowerOfTwoSubAssign for $t {
            /// Replaces `self` with `self - rhs` mod 2<sup>`pow`</sup>. Assumes the inputs are
            /// already reduced mod 2<sup>`pow`</sup>.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoSubAssign;
            ///
            /// let mut n = 5u8;
            /// n.mod_power_of_two_sub_assign(2, 5);
            /// assert_eq!(n, 3);
            ///
            /// let mut n = 10u32;
            /// n.mod_power_of_two_sub_assign(14, 4);
            /// assert_eq!(n, 12);
            /// ```
            #[inline]
            fn mod_power_of_two_sub_assign(&mut self, rhs: $t, pow: u64) {
                assert!(pow <= $t::WIDTH);
                self.wrapping_sub_assign(rhs);
                self.mod_power_of_two_assign(pow);
            }
        }
    };
}

impl_mod_power_of_two_sub!(u8);
impl_mod_power_of_two_sub!(u16);
impl_mod_power_of_two_sub!(u32);
impl_mod_power_of_two_sub!(u64);
impl_mod_power_of_two_sub!(u128);
impl_mod_power_of_two_sub!(usize);
