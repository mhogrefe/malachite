use num::arithmetic::traits::{ModPowerOfTwo, ModPowerOfTwoMul, ModPowerOfTwoMulAssign};
use num::basic::integers::PrimitiveInt;

fn _mod_power_of_two_mul<T: ModPowerOfTwo<Output = T> + PrimitiveInt>(
    x: T,
    other: T,
    pow: u64,
) -> T {
    assert!(pow <= T::WIDTH);
    x.wrapping_mul(other).mod_power_of_two(pow)
}

#[inline]
fn _mod_power_of_two_mul_assign<T: PrimitiveInt>(x: &mut T, other: T, pow: u64) {
    assert!(pow <= T::WIDTH);
    x.wrapping_mul_assign(other);
    x.mod_power_of_two_assign(pow);
}

macro_rules! impl_mod_power_of_two_mul {
    ($t:ident) => {
        impl ModPowerOfTwoMul<$t> for $t {
            type Output = $t;

            /// Computes `self * other` mod 2<sup>`pow`</sup>. Assumes the inputs are already
            /// reduced mod 2<sup>`pow`</sup>.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoMul;
            ///
            /// assert_eq!(3u8.mod_power_of_two_mul(2, 5), 6);
            /// assert_eq!(10u32.mod_power_of_two_mul(14, 4), 12);
            /// ```
            #[inline]
            fn mod_power_of_two_mul(self, other: $t, pow: u64) -> $t {
                _mod_power_of_two_mul(self, other, pow)
            }
        }

        impl ModPowerOfTwoMulAssign<$t> for $t {
            /// Replaces `self` with `self * other` mod 2<sup>`pow`</sup>. Assumes the inputs are
            /// already reduced mod 2<sup>`pow`</sup>.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
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
            fn mod_power_of_two_mul_assign(&mut self, other: $t, pow: u64) {
                _mod_power_of_two_mul_assign(self, other, pow)
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_power_of_two_mul);
