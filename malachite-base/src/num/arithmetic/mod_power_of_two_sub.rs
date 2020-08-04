use num::arithmetic::traits::{ModPowerOfTwo, ModPowerOfTwoSub, ModPowerOfTwoSubAssign};
use num::basic::integers::PrimitiveInteger;

fn _mod_power_of_two_sub<T: PrimitiveInteger>(x: T, other: T, pow: u64) -> T
where
    T: ModPowerOfTwo<Output = T>,
{
    assert!(pow <= T::WIDTH);
    x.wrapping_sub(other).mod_power_of_two(pow)
}

fn _mod_power_of_two_sub_assign<T: PrimitiveInteger>(x: &mut T, other: T, pow: u64) {
    assert!(pow <= T::WIDTH);
    x.wrapping_sub_assign(other);
    x.mod_power_of_two_assign(pow);
}

macro_rules! impl_mod_power_of_two_sub {
    ($t:ident) => {
        impl ModPowerOfTwoSub<$t> for $t {
            type Output = $t;

            /// Computes `self - other` mod 2<sup>`pow`</sup>. Assumes the inputs are already
            /// reduced mod 2<sup>`pow`</sup>.
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
            fn mod_power_of_two_sub(self, other: $t, pow: u64) -> $t {
                _mod_power_of_two_sub(self, other, pow)
            }
        }

        impl ModPowerOfTwoSubAssign<$t> for $t {
            /// Replaces `self` with `self - other` mod 2<sup>`pow`</sup>. Assumes the inputs are
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
            fn mod_power_of_two_sub_assign(&mut self, other: $t, pow: u64) {
                _mod_power_of_two_sub_assign(self, other, pow);
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_power_of_two_sub);
