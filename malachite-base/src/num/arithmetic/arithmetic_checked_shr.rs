use num::arithmetic::traits::{ArithmeticCheckedShl, ArithmeticCheckedShr, UnsignedAbs};
use num::basic::integers::PrimitiveInteger;
use num::conversion::traits::WrappingInto;

macro_rules! impl_arithmetic_checked_shr_unsigned_signed {
    ($t:ident, $u:ident) => {
        impl ArithmeticCheckedShr<$u> for $t {
            type Output = $t;

            /// Shifts `self` right (divides it by a power of 2). If the result is too large to fit
            /// in a `$t`, `None` is returned. Zero may be shifted by any amount, and any number
            /// may be shifted by any non-negative amount; shifting by a large amount returns
            /// `Some(0)`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::ArithmeticCheckedShr;
            ///
            /// assert_eq!(100u8.arithmetic_checked_shr(3), Some(12u8));
            /// assert_eq!(100u8.arithmetic_checked_shr(100), Some(0u8));
            /// assert_eq!(3u8.arithmetic_checked_shr(-6), Some(192u8));
            /// assert_eq!(3u8.arithmetic_checked_shr(-7), None);
            /// assert_eq!(3u8.arithmetic_checked_shr(-100), None);
            /// assert_eq!(0u8.arithmetic_checked_shr(-100), Some(0u8));
            /// ```
            fn arithmetic_checked_shr(self, bits: $u) -> Option<$t> {
                if bits < 0 {
                    self.arithmetic_checked_shl(bits.unsigned_abs())
                } else {
                    let abs_bits = bits.unsigned_abs();
                    Some(if abs_bits >= $t::WIDTH.wrapping_into() {
                        0
                    } else {
                        self >> abs_bits
                    })
                }
            }
        }
    };
}
impl_arithmetic_checked_shr_unsigned_signed!(u8, i8);
impl_arithmetic_checked_shr_unsigned_signed!(u8, i16);
impl_arithmetic_checked_shr_unsigned_signed!(u8, i32);
impl_arithmetic_checked_shr_unsigned_signed!(u8, i64);
impl_arithmetic_checked_shr_unsigned_signed!(u8, i128);
impl_arithmetic_checked_shr_unsigned_signed!(u8, isize);
impl_arithmetic_checked_shr_unsigned_signed!(u16, i8);
impl_arithmetic_checked_shr_unsigned_signed!(u16, i16);
impl_arithmetic_checked_shr_unsigned_signed!(u16, i32);
impl_arithmetic_checked_shr_unsigned_signed!(u16, i64);
impl_arithmetic_checked_shr_unsigned_signed!(u16, i128);
impl_arithmetic_checked_shr_unsigned_signed!(u16, isize);
impl_arithmetic_checked_shr_unsigned_signed!(u32, i8);
impl_arithmetic_checked_shr_unsigned_signed!(u32, i16);
impl_arithmetic_checked_shr_unsigned_signed!(u32, i32);
impl_arithmetic_checked_shr_unsigned_signed!(u32, i64);
impl_arithmetic_checked_shr_unsigned_signed!(u32, i128);
impl_arithmetic_checked_shr_unsigned_signed!(u32, isize);
impl_arithmetic_checked_shr_unsigned_signed!(u64, i8);
impl_arithmetic_checked_shr_unsigned_signed!(u64, i16);
impl_arithmetic_checked_shr_unsigned_signed!(u64, i32);
impl_arithmetic_checked_shr_unsigned_signed!(u64, i64);
impl_arithmetic_checked_shr_unsigned_signed!(u64, i128);
impl_arithmetic_checked_shr_unsigned_signed!(u64, isize);
impl_arithmetic_checked_shr_unsigned_signed!(u128, i8);
impl_arithmetic_checked_shr_unsigned_signed!(u128, i16);
impl_arithmetic_checked_shr_unsigned_signed!(u128, i32);
impl_arithmetic_checked_shr_unsigned_signed!(u128, i64);
impl_arithmetic_checked_shr_unsigned_signed!(u128, i128);
impl_arithmetic_checked_shr_unsigned_signed!(u128, isize);
impl_arithmetic_checked_shr_unsigned_signed!(usize, i8);
impl_arithmetic_checked_shr_unsigned_signed!(usize, i16);
impl_arithmetic_checked_shr_unsigned_signed!(usize, i32);
impl_arithmetic_checked_shr_unsigned_signed!(usize, i64);
impl_arithmetic_checked_shr_unsigned_signed!(usize, i128);
impl_arithmetic_checked_shr_unsigned_signed!(usize, isize);

macro_rules! impl_arithmetic_checked_shr_signed_signed {
    ($t:ident, $u:ident) => {
        impl ArithmeticCheckedShr<$u> for $t {
            type Output = $t;

            /// Shifts `self` right (divides it by a power of 2). If the result is too large to fit
            /// in a `$t`, `None` is returned. Zero may be shifted by any amount, and any number may
            /// be shifted by any non-negative amount; shifting by a large amount returns `Some(0)`
            /// if `self` is positive, and `Some(-1)` if `self` is negative.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::ArithmeticCheckedShr;
            ///
            /// assert_eq!(100i8.arithmetic_checked_shr(3), Some(12i8));
            /// assert_eq!((-100i8).arithmetic_checked_shr(3), Some(-13i8));
            /// assert_eq!(100i8.arithmetic_checked_shr(100), Some(0i8));
            /// assert_eq!((-100i8).arithmetic_checked_shr(100), Some(-1i8));
            /// assert_eq!(3i8.arithmetic_checked_shr(-5), Some(96i8));
            /// assert_eq!(3i8.arithmetic_checked_shr(-6), None);
            /// assert_eq!((-3i8).arithmetic_checked_shr(-5), Some(-96i8));
            /// assert_eq!((-3i8).arithmetic_checked_shr(-6), None);
            /// assert_eq!(3i8.arithmetic_checked_shr(-100), None);
            /// assert_eq!((-3i8).arithmetic_checked_shr(-100), None);
            /// assert_eq!(0i8.arithmetic_checked_shr(-100), Some(0i8));
            /// ```
            fn arithmetic_checked_shr(self, bits: $u) -> Option<$t> {
                if bits < 0 {
                    self.arithmetic_checked_shl(bits.unsigned_abs())
                } else {
                    let width = $t::WIDTH.wrapping_into();
                    let abs_bits = bits.unsigned_abs();
                    Some(if width != 0 && abs_bits >= width {
                        if self >= 0 {
                            0
                        } else {
                            -1
                        }
                    } else {
                        self >> abs_bits
                    })
                }
            }
        }
    };
}
impl_arithmetic_checked_shr_signed_signed!(i8, i8);
impl_arithmetic_checked_shr_signed_signed!(i8, i16);
impl_arithmetic_checked_shr_signed_signed!(i8, i32);
impl_arithmetic_checked_shr_signed_signed!(i8, i64);
impl_arithmetic_checked_shr_signed_signed!(i8, i128);
impl_arithmetic_checked_shr_signed_signed!(i8, isize);
impl_arithmetic_checked_shr_signed_signed!(i16, i8);
impl_arithmetic_checked_shr_signed_signed!(i16, i16);
impl_arithmetic_checked_shr_signed_signed!(i16, i32);
impl_arithmetic_checked_shr_signed_signed!(i16, i64);
impl_arithmetic_checked_shr_signed_signed!(i16, i128);
impl_arithmetic_checked_shr_signed_signed!(i16, isize);
impl_arithmetic_checked_shr_signed_signed!(i32, i8);
impl_arithmetic_checked_shr_signed_signed!(i32, i16);
impl_arithmetic_checked_shr_signed_signed!(i32, i32);
impl_arithmetic_checked_shr_signed_signed!(i32, i64);
impl_arithmetic_checked_shr_signed_signed!(i32, i128);
impl_arithmetic_checked_shr_signed_signed!(i32, isize);
impl_arithmetic_checked_shr_signed_signed!(i64, i8);
impl_arithmetic_checked_shr_signed_signed!(i64, i16);
impl_arithmetic_checked_shr_signed_signed!(i64, i32);
impl_arithmetic_checked_shr_signed_signed!(i64, i64);
impl_arithmetic_checked_shr_signed_signed!(i64, i128);
impl_arithmetic_checked_shr_signed_signed!(i64, isize);
impl_arithmetic_checked_shr_signed_signed!(i128, i8);
impl_arithmetic_checked_shr_signed_signed!(i128, i16);
impl_arithmetic_checked_shr_signed_signed!(i128, i32);
impl_arithmetic_checked_shr_signed_signed!(i128, i64);
impl_arithmetic_checked_shr_signed_signed!(i128, i128);
impl_arithmetic_checked_shr_signed_signed!(i128, isize);
impl_arithmetic_checked_shr_signed_signed!(isize, i8);
impl_arithmetic_checked_shr_signed_signed!(isize, i16);
impl_arithmetic_checked_shr_signed_signed!(isize, i32);
impl_arithmetic_checked_shr_signed_signed!(isize, i64);
impl_arithmetic_checked_shr_signed_signed!(isize, i128);
impl_arithmetic_checked_shr_signed_signed!(isize, isize);
