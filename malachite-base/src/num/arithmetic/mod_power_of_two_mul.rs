use num::arithmetic::traits::{
    ModPowerOfTwo, ModPowerOfTwoAssign, ModPowerOfTwoMul, ModPowerOfTwoMulAssign, WrappingMulAssign,
};
use num::basic::integers::PrimitiveInteger;

macro_rules! impl_arithmetic_traits {
    ($t:ident) => {
        impl ModPowerOfTwoMul for $t {
            type Output = $t;

            /// Computes `self * rhs` mod 2<sup>`pow`</sup>. Assumes the inputs are already reduced
            /// mod 2<sup>`pow`</sup>.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoMul;
            ///
            /// assert_eq!(3u8.mod_power_of_two_mul(2, 5), 6);
            /// assert_eq!(10u32.mod_power_of_two_mul(14, 4), 12);
            /// ```
            #[inline]
            fn mod_power_of_two_mul(self, rhs: $t, pow: u64) -> $t {
                assert!(pow <= $t::WIDTH);
                self.wrapping_mul(rhs).mod_power_of_two(pow)
            }
        }

        impl ModPowerOfTwoMulAssign for $t {
            /// Replaces `self` with `self * rhs` mod 2<sup>`pow`</sup>. Assumes the inputs are
            /// already reduced mod 2<sup>`pow`</sup>.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoMulAssign;
            ///
            /// let mut n = 3u8;
            /// n.mod_power_of_two_mul_assign(2, 5);
            /// assert_eq!(n, 6);
            ///
            /// let mut n = 10u32;
            /// n.mod_power_of_two_mul_assign(14, 4);
            /// assert_eq!(n, 12);
            /// ```
            #[inline]
            fn mod_power_of_two_mul_assign(&mut self, rhs: $t, pow: u64) {
                assert!(pow <= $t::WIDTH);
                self.wrapping_mul_assign(rhs);
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
