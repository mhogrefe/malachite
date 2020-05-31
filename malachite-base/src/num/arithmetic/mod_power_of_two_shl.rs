use num::arithmetic::traits::{
    ModPowerOfTwo, ModPowerOfTwoAssign, ModPowerOfTwoShl, ModPowerOfTwoShlAssign, UnsignedAbs,
};
use num::basic::integers::PrimitiveInteger;
use num::conversion::traits::{ExactFrom, WrappingInto};

macro_rules! impl_mod_power_of_two_shl_unsigned {
    ($t:ident, $u:ident) => {
        impl ModPowerOfTwoShl<$u> for $t {
            type Output = $t;

            /// Computes `self << other` mod 2<sup>`pow`</sup>. Assumes the input is already reduced
            /// mod 2<sup>`pow`</sup>.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoShl;
            ///
            /// assert_eq!(12u32.mod_power_of_two_shl(2u8, 5), 16);
            /// assert_eq!(10u8.mod_power_of_two_shl(100u64, 4), 0);
            /// ```
            #[inline]
            fn mod_power_of_two_shl(self, other: $u, pow: u64) -> $t {
                assert!(pow <= $t::WIDTH);
                if other >= $u::exact_from($t::WIDTH) {
                    0
                } else {
                    (self << other).mod_power_of_two(pow)
                }
            }
        }

        impl ModPowerOfTwoShlAssign<$u> for $t {
            /// Replaces `self` with `self << other` mod 2<sup>`pow`</sup>. Assumes the input is
            /// already reduced mod 2<sup>`pow`</sup>.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoShlAssign;
            ///
            /// let mut n = 12u32;
            /// n.mod_power_of_two_shl_assign(2u8, 5);
            /// assert_eq!(n, 16);
            ///
            /// let mut n = 10u8;
            /// n.mod_power_of_two_shl_assign(100u64, 4);
            /// assert_eq!(n, 0);
            /// ```
            #[inline]
            fn mod_power_of_two_shl_assign(&mut self, other: $u, pow: u64) {
                assert!(pow <= $t::WIDTH);
                if other >= $u::exact_from($t::WIDTH) {
                    *self = 0;
                } else {
                    *self <<= other;
                    self.mod_power_of_two_assign(pow);
                }
            }
        }
    };
}
impl_mod_power_of_two_shl_unsigned!(u8, u8);
impl_mod_power_of_two_shl_unsigned!(u8, u16);
impl_mod_power_of_two_shl_unsigned!(u8, u32);
impl_mod_power_of_two_shl_unsigned!(u8, u64);
impl_mod_power_of_two_shl_unsigned!(u8, u128);
impl_mod_power_of_two_shl_unsigned!(u8, usize);
impl_mod_power_of_two_shl_unsigned!(u16, u8);
impl_mod_power_of_two_shl_unsigned!(u16, u16);
impl_mod_power_of_two_shl_unsigned!(u16, u32);
impl_mod_power_of_two_shl_unsigned!(u16, u64);
impl_mod_power_of_two_shl_unsigned!(u16, u128);
impl_mod_power_of_two_shl_unsigned!(u16, usize);
impl_mod_power_of_two_shl_unsigned!(u32, u8);
impl_mod_power_of_two_shl_unsigned!(u32, u16);
impl_mod_power_of_two_shl_unsigned!(u32, u32);
impl_mod_power_of_two_shl_unsigned!(u32, u64);
impl_mod_power_of_two_shl_unsigned!(u32, u128);
impl_mod_power_of_two_shl_unsigned!(u32, usize);
impl_mod_power_of_two_shl_unsigned!(u64, u8);
impl_mod_power_of_two_shl_unsigned!(u64, u16);
impl_mod_power_of_two_shl_unsigned!(u64, u32);
impl_mod_power_of_two_shl_unsigned!(u64, u64);
impl_mod_power_of_two_shl_unsigned!(u64, u128);
impl_mod_power_of_two_shl_unsigned!(u64, usize);
impl_mod_power_of_two_shl_unsigned!(u128, u8);
impl_mod_power_of_two_shl_unsigned!(u128, u16);
impl_mod_power_of_two_shl_unsigned!(u128, u32);
impl_mod_power_of_two_shl_unsigned!(u128, u64);
impl_mod_power_of_two_shl_unsigned!(u128, u128);
impl_mod_power_of_two_shl_unsigned!(u128, usize);
impl_mod_power_of_two_shl_unsigned!(usize, u8);
impl_mod_power_of_two_shl_unsigned!(usize, u16);
impl_mod_power_of_two_shl_unsigned!(usize, u32);
impl_mod_power_of_two_shl_unsigned!(usize, u64);
impl_mod_power_of_two_shl_unsigned!(usize, u128);
impl_mod_power_of_two_shl_unsigned!(usize, usize);

macro_rules! impl_mod_power_of_two_shl_signed {
    ($t:ident, $u:ident) => {
        impl ModPowerOfTwoShl<$u> for $t {
            type Output = $t;

            /// Computes `self << other` mod 2<sup>`pow`</sup>. Assumes the input is already reduced
            /// mod 2<sup>`pow`</sup>.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoShl;
            ///
            /// assert_eq!(12u32.mod_power_of_two_shl(2i8, 5), 16);
            /// assert_eq!(10u8.mod_power_of_two_shl(-2i64, 4), 2);
            /// ```
            #[inline]
            fn mod_power_of_two_shl(self, other: $u, pow: u64) -> $t {
                assert!(pow <= $t::WIDTH);
                let other_abs = other.unsigned_abs();
                if other >= 0 {
                    self.mod_power_of_two_shl(other_abs, pow)
                } else {
                    let width = $t::WIDTH.wrapping_into();
                    if width != 0 && other_abs >= width {
                        0
                    } else {
                        self >> other_abs
                    }
                }
            }
        }

        impl ModPowerOfTwoShlAssign<$u> for $t {
            /// Replaces `self` with `self << other` mod 2<sup>`pow`</sup>. Assumes the input is
            /// already reduced mod 2<sup>`pow`</sup>.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoShlAssign;
            ///
            /// let mut n = 12u32;
            /// n.mod_power_of_two_shl_assign(2i8, 5);
            /// assert_eq!(n, 16);
            ///
            /// let mut n = 10u8;
            /// n.mod_power_of_two_shl_assign(-2i64, 4);
            /// assert_eq!(n, 2);
            /// ```
            #[inline]
            fn mod_power_of_two_shl_assign(&mut self, other: $u, pow: u64) {
                assert!(pow <= $t::WIDTH);
                let other_abs = other.unsigned_abs();
                if other >= 0 {
                    self.mod_power_of_two_shl_assign(other_abs, pow);
                } else {
                    let width = $t::WIDTH.wrapping_into();
                    if width != 0 && other_abs >= width {
                        *self = 0;
                    } else {
                        *self >>= other_abs;
                    }
                }
            }
        }
    };
}
impl_mod_power_of_two_shl_signed!(u8, i8);
impl_mod_power_of_two_shl_signed!(u8, i16);
impl_mod_power_of_two_shl_signed!(u8, i32);
impl_mod_power_of_two_shl_signed!(u8, i64);
impl_mod_power_of_two_shl_signed!(u8, i128);
impl_mod_power_of_two_shl_signed!(u8, isize);
impl_mod_power_of_two_shl_signed!(u16, i8);
impl_mod_power_of_two_shl_signed!(u16, i16);
impl_mod_power_of_two_shl_signed!(u16, i32);
impl_mod_power_of_two_shl_signed!(u16, i64);
impl_mod_power_of_two_shl_signed!(u16, i128);
impl_mod_power_of_two_shl_signed!(u16, isize);
impl_mod_power_of_two_shl_signed!(u32, i8);
impl_mod_power_of_two_shl_signed!(u32, i16);
impl_mod_power_of_two_shl_signed!(u32, i32);
impl_mod_power_of_two_shl_signed!(u32, i64);
impl_mod_power_of_two_shl_signed!(u32, i128);
impl_mod_power_of_two_shl_signed!(u32, isize);
impl_mod_power_of_two_shl_signed!(u64, i8);
impl_mod_power_of_two_shl_signed!(u64, i16);
impl_mod_power_of_two_shl_signed!(u64, i32);
impl_mod_power_of_two_shl_signed!(u64, i64);
impl_mod_power_of_two_shl_signed!(u64, i128);
impl_mod_power_of_two_shl_signed!(u64, isize);
impl_mod_power_of_two_shl_signed!(u128, i8);
impl_mod_power_of_two_shl_signed!(u128, i16);
impl_mod_power_of_two_shl_signed!(u128, i32);
impl_mod_power_of_two_shl_signed!(u128, i64);
impl_mod_power_of_two_shl_signed!(u128, i128);
impl_mod_power_of_two_shl_signed!(u128, isize);
impl_mod_power_of_two_shl_signed!(usize, i8);
impl_mod_power_of_two_shl_signed!(usize, i16);
impl_mod_power_of_two_shl_signed!(usize, i32);
impl_mod_power_of_two_shl_signed!(usize, i64);
impl_mod_power_of_two_shl_signed!(usize, i128);
impl_mod_power_of_two_shl_signed!(usize, isize);
