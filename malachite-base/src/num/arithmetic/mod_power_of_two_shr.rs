use num::arithmetic::traits::{
    ModPowerOfTwoShl, ModPowerOfTwoShlAssign, ModPowerOfTwoShr, ModPowerOfTwoShrAssign, UnsignedAbs,
};
use num::basic::integers::PrimitiveInteger;
use num::conversion::traits::WrappingInto;

macro_rules! impl_mod_power_of_two_shr_signed {
    ($t:ident, $u:ident) => {
        impl ModPowerOfTwoShr<$u> for $t {
            type Output = $t;

            /// Computes `self >> other` mod 2<sup>`pow`</sup>. Assumes the input is already reduced
            /// mod 2<sup>`pow`</sup>.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoShr;
            ///
            /// assert_eq!(10u8.mod_power_of_two_shr(2i64, 4), 2);
            /// assert_eq!(12u32.mod_power_of_two_shr(-2i8, 5), 16);
            /// ```
            #[inline]
            fn mod_power_of_two_shr(self, other: $u, pow: u64) -> $t {
                assert!(pow <= $t::WIDTH);
                let other_abs = other.unsigned_abs();
                if other >= 0 {
                    let width = $t::WIDTH.wrapping_into();
                    if width != 0 && other_abs >= width {
                        0
                    } else {
                        self >> other_abs
                    }
                } else {
                    self.mod_power_of_two_shl(other_abs, pow)
                }
            }
        }

        impl ModPowerOfTwoShrAssign<$u> for $t {
            /// Replaces `self` with `self >> other` mod 2<sup>`pow`</sup>. Assumes the input is
            /// already reduced mod 2<sup>`pow`</sup>.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoShrAssign;
            ///
            /// let mut n = 10u8;
            /// n.mod_power_of_two_shr_assign(2i64, 4);
            /// assert_eq!(n, 2);
            ///
            /// let mut n = 12u32;
            /// n.mod_power_of_two_shr_assign(-2i8, 5);
            /// assert_eq!(n, 16);
            /// ```
            #[inline]
            fn mod_power_of_two_shr_assign(&mut self, other: $u, pow: u64) {
                assert!(pow <= $t::WIDTH);
                let other_abs = other.unsigned_abs();
                if other >= 0 {
                    let width = $t::WIDTH.wrapping_into();
                    if width != 0 && other_abs >= width {
                        *self = 0;
                    } else {
                        *self >>= other_abs;
                    }
                } else {
                    self.mod_power_of_two_shl_assign(other_abs, pow);
                }
            }
        }
    };
}
impl_mod_power_of_two_shr_signed!(u8, i8);
impl_mod_power_of_two_shr_signed!(u8, i16);
impl_mod_power_of_two_shr_signed!(u8, i32);
impl_mod_power_of_two_shr_signed!(u8, i64);
impl_mod_power_of_two_shr_signed!(u8, i128);
impl_mod_power_of_two_shr_signed!(u8, isize);
impl_mod_power_of_two_shr_signed!(u16, i8);
impl_mod_power_of_two_shr_signed!(u16, i16);
impl_mod_power_of_two_shr_signed!(u16, i32);
impl_mod_power_of_two_shr_signed!(u16, i64);
impl_mod_power_of_two_shr_signed!(u16, i128);
impl_mod_power_of_two_shr_signed!(u16, isize);
impl_mod_power_of_two_shr_signed!(u32, i8);
impl_mod_power_of_two_shr_signed!(u32, i16);
impl_mod_power_of_two_shr_signed!(u32, i32);
impl_mod_power_of_two_shr_signed!(u32, i64);
impl_mod_power_of_two_shr_signed!(u32, i128);
impl_mod_power_of_two_shr_signed!(u32, isize);
impl_mod_power_of_two_shr_signed!(u64, i8);
impl_mod_power_of_two_shr_signed!(u64, i16);
impl_mod_power_of_two_shr_signed!(u64, i32);
impl_mod_power_of_two_shr_signed!(u64, i64);
impl_mod_power_of_two_shr_signed!(u64, i128);
impl_mod_power_of_two_shr_signed!(u64, isize);
impl_mod_power_of_two_shr_signed!(u128, i8);
impl_mod_power_of_two_shr_signed!(u128, i16);
impl_mod_power_of_two_shr_signed!(u128, i32);
impl_mod_power_of_two_shr_signed!(u128, i64);
impl_mod_power_of_two_shr_signed!(u128, i128);
impl_mod_power_of_two_shr_signed!(u128, isize);
impl_mod_power_of_two_shr_signed!(usize, i8);
impl_mod_power_of_two_shr_signed!(usize, i16);
impl_mod_power_of_two_shr_signed!(usize, i32);
impl_mod_power_of_two_shr_signed!(usize, i64);
impl_mod_power_of_two_shr_signed!(usize, i128);
impl_mod_power_of_two_shr_signed!(usize, isize);
