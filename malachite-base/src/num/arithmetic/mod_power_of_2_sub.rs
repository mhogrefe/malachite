use num::arithmetic::traits::{ModPowerOf2, ModPowerOf2Sub, ModPowerOf2SubAssign};
use num::basic::integers::PrimitiveInt;

fn _mod_power_of_2_sub<T: ModPowerOf2<Output = T> + PrimitiveInt>(x: T, other: T, pow: u64) -> T {
    assert!(pow <= T::WIDTH);
    x.wrapping_sub(other).mod_power_of_2(pow)
}

fn _mod_power_of_2_sub_assign<T: PrimitiveInt>(x: &mut T, other: T, pow: u64) {
    assert!(pow <= T::WIDTH);
    x.wrapping_sub_assign(other);
    x.mod_power_of_2_assign(pow);
}

macro_rules! impl_mod_power_of_2_sub {
    ($t:ident) => {
        impl ModPowerOf2Sub<$t> for $t {
            type Output = $t;

            /// Computes `self - other` mod 2<sup>`pow`</sup>. Assumes the inputs are already
            /// reduced mod 2<sup>`pow`</sup>.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModPowerOf2Sub;
            ///
            /// assert_eq!(5u8.mod_power_of_2_sub(2, 5), 3);
            /// assert_eq!(10u32.mod_power_of_2_sub(14, 4), 12);
            /// ```
            #[inline]
            fn mod_power_of_2_sub(self, other: $t, pow: u64) -> $t {
                _mod_power_of_2_sub(self, other, pow)
            }
        }

        impl ModPowerOf2SubAssign<$t> for $t {
            /// Replaces `self` with `self - other` mod 2<sup>`pow`</sup>. Assumes the inputs are
            /// already reduced mod 2<sup>`pow`</sup>.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModPowerOf2SubAssign;
            ///
            /// let mut n = 5u8;
            /// n.mod_power_of_2_sub_assign(2, 5);
            /// assert_eq!(n, 3);
            ///
            /// let mut n = 10u32;
            /// n.mod_power_of_2_sub_assign(14, 4);
            /// assert_eq!(n, 12);
            /// ```
            #[inline]
            fn mod_power_of_2_sub_assign(&mut self, other: $t, pow: u64) {
                _mod_power_of_2_sub_assign(self, other, pow);
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_power_of_2_sub);
