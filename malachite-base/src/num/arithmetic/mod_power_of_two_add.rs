use num::arithmetic::traits::{
    ModPowerOfTwo, ModPowerOfTwoAdd, ModPowerOfTwoAddAssign, ModPowerOfTwoAssign, WrappingAddAssign,
};
use num::basic::integers::PrimitiveInteger;

macro_rules! impl_arithmetic_traits {
    ($t:ident) => {
        impl ModPowerOfTwoAdd for $t {
            type Output = $t;

            /// Computes `self + rhs` mod 2<sup>`pow`</sup>. Assumes the inputs are already reduced
            /// mod 2<sup>`pow`</sup>.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoAdd;
            ///
            /// assert_eq!(0u8.mod_power_of_two_add(2, 5), 2);
            /// assert_eq!(10u32.mod_power_of_two_add(14, 4), 8);
            /// ```
            #[inline]
            fn mod_power_of_two_add(self, rhs: $t, pow: u64) -> $t {
                assert!(pow <= $t::WIDTH);
                self.wrapping_add(rhs).mod_power_of_two(pow)
            }
        }

        impl ModPowerOfTwoAddAssign for $t {
            /// Replaces `self` with `self + rhs` mod 2<sup>`pow`</sup>. Assumes the inputs are
            /// already reduced mod 2<sup>`pow`</sup>.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoAddAssign;
            ///
            /// let mut n = 0u8;
            /// n.mod_power_of_two_add_assign(2, 5);
            /// assert_eq!(n, 2);
            ///
            /// let mut n = 10u32;
            /// n.mod_power_of_two_add_assign(14, 4);
            /// assert_eq!(n, 8);
            /// ```
            #[inline]
            fn mod_power_of_two_add_assign(&mut self, rhs: $t, pow: u64) {
                assert!(pow <= $t::WIDTH);
                self.wrapping_add_assign(rhs);
                self.mod_power_of_two_assign(pow);
            }
        }
    };
}

impl_arithmetic_traits!(u8);
impl_arithmetic_traits!(u16);
impl_arithmetic_traits!(u32);
impl_arithmetic_traits!(u64);
impl_arithmetic_traits!(u128);
impl_arithmetic_traits!(usize);
