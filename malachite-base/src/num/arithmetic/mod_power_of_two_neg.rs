use num::arithmetic::traits::{
    ModPowerOfTwo, ModPowerOfTwoAssign, ModPowerOfTwoNeg, ModPowerOfTwoNegAssign, WrappingNegAssign,
};
use num::basic::integers::PrimitiveInteger;

macro_rules! impl_mod_power_of_two_neg {
    ($t:ident) => {
        impl ModPowerOfTwoNeg for $t {
            type Output = $t;

            /// Computes `-self` mod 2<sup>`pow`</sup>. Assumes the input is already reduced mod
            /// 2<sup>`pow`</sup>.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoNeg;
            ///
            /// assert_eq!(0u8.mod_power_of_two_neg(5), 0);
            /// assert_eq!(10u32.mod_power_of_two_neg(4), 6);
            /// assert_eq!(100u16.mod_power_of_two_neg(8), 156);
            /// ```
            #[inline]
            fn mod_power_of_two_neg(self, pow: u64) -> $t {
                assert!(pow <= $t::WIDTH);
                self.wrapping_neg().mod_power_of_two(pow)
            }
        }

        impl ModPowerOfTwoNegAssign for $t {
            /// Replaces `self` with `-self` mod 2<sup>`pow`</sup>. Assumes the input is already
            /// reduced mod 2<sup>`pow`</sup>.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoNegAssign;
            ///
            /// let mut n = 0u8;
            /// n.mod_power_of_two_neg_assign(5);
            /// assert_eq!(n, 0);
            ///
            /// let mut n = 10u32;
            /// n.mod_power_of_two_neg_assign(4);
            /// assert_eq!(n, 6);
            ///
            /// let mut n = 100u16;
            /// n.mod_power_of_two_neg_assign(8);
            /// assert_eq!(n, 156);
            /// ```
            #[inline]
            fn mod_power_of_two_neg_assign(&mut self, pow: u64) {
                assert!(pow <= $t::WIDTH);
                self.wrapping_neg_assign();
                self.mod_power_of_two_assign(pow);
            }
        }
    };
}

impl_mod_power_of_two_neg!(u8);
impl_mod_power_of_two_neg!(u16);
impl_mod_power_of_two_neg!(u32);
impl_mod_power_of_two_neg!(u64);
impl_mod_power_of_two_neg!(u128);
impl_mod_power_of_two_neg!(usize);
