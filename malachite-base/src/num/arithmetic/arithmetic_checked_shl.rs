use num::arithmetic::traits::{ArithmeticCheckedShl, UnsignedAbs};
use num::basic::integers::PrimitiveInteger;
use num::basic::traits::Iverson;
use num::conversion::traits::{CheckedFrom, WrappingFrom, WrappingInto};

macro_rules! impl_arithmetic_checked_shl_unsigned_unsigned {
    ($t:ident, $u:ident) => {
        impl ArithmeticCheckedShl<$u> for $t {
            type Output = $t;

            /// Shifts `self` left (multiplies it by a power of 2). If the result is too large to
            /// fit in a `$t`, `None` is returned. Zero may be shifted by any amount.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::ArithmeticCheckedShl;
            ///
            /// assert_eq!(3u8.arithmetic_checked_shl(6), Some(192u8));
            /// assert_eq!(3u8.arithmetic_checked_shl(7), None);
            /// assert_eq!(3u8.arithmetic_checked_shl(100), None);
            /// assert_eq!(0u8.arithmetic_checked_shl(100), Some(0u8));
            /// ```
            fn arithmetic_checked_shl(self, bits: $u) -> Option<$t> {
                if self == 0 {
                    Some(self)
                } else if bits >= $u::wrapping_from($t::WIDTH) {
                    None
                } else {
                    let result = self << bits;
                    if result >> bits == self {
                        Some(result)
                    } else {
                        None
                    }
                }
            }
        }
    };
}
impl_arithmetic_checked_shl_unsigned_unsigned!(u8, u8);
impl_arithmetic_checked_shl_unsigned_unsigned!(u8, u16);
impl_arithmetic_checked_shl_unsigned_unsigned!(u8, u32);
impl_arithmetic_checked_shl_unsigned_unsigned!(u8, u64);
impl_arithmetic_checked_shl_unsigned_unsigned!(u8, u128);
impl_arithmetic_checked_shl_unsigned_unsigned!(u8, usize);
impl_arithmetic_checked_shl_unsigned_unsigned!(u16, u8);
impl_arithmetic_checked_shl_unsigned_unsigned!(u16, u16);
impl_arithmetic_checked_shl_unsigned_unsigned!(u16, u32);
impl_arithmetic_checked_shl_unsigned_unsigned!(u16, u64);
impl_arithmetic_checked_shl_unsigned_unsigned!(u16, u128);
impl_arithmetic_checked_shl_unsigned_unsigned!(u16, usize);
impl_arithmetic_checked_shl_unsigned_unsigned!(u32, u8);
impl_arithmetic_checked_shl_unsigned_unsigned!(u32, u16);
impl_arithmetic_checked_shl_unsigned_unsigned!(u32, u32);
impl_arithmetic_checked_shl_unsigned_unsigned!(u32, u64);
impl_arithmetic_checked_shl_unsigned_unsigned!(u32, u128);
impl_arithmetic_checked_shl_unsigned_unsigned!(u32, usize);
impl_arithmetic_checked_shl_unsigned_unsigned!(u64, u8);
impl_arithmetic_checked_shl_unsigned_unsigned!(u64, u16);
impl_arithmetic_checked_shl_unsigned_unsigned!(u64, u32);
impl_arithmetic_checked_shl_unsigned_unsigned!(u64, u64);
impl_arithmetic_checked_shl_unsigned_unsigned!(u64, u128);
impl_arithmetic_checked_shl_unsigned_unsigned!(u64, usize);
impl_arithmetic_checked_shl_unsigned_unsigned!(u128, u8);
impl_arithmetic_checked_shl_unsigned_unsigned!(u128, u16);
impl_arithmetic_checked_shl_unsigned_unsigned!(u128, u32);
impl_arithmetic_checked_shl_unsigned_unsigned!(u128, u64);
impl_arithmetic_checked_shl_unsigned_unsigned!(u128, u128);
impl_arithmetic_checked_shl_unsigned_unsigned!(u128, usize);
impl_arithmetic_checked_shl_unsigned_unsigned!(usize, u8);
impl_arithmetic_checked_shl_unsigned_unsigned!(usize, u16);
impl_arithmetic_checked_shl_unsigned_unsigned!(usize, u32);
impl_arithmetic_checked_shl_unsigned_unsigned!(usize, u64);
impl_arithmetic_checked_shl_unsigned_unsigned!(usize, u128);
impl_arithmetic_checked_shl_unsigned_unsigned!(usize, usize);

macro_rules! impl_arithmetic_checked_shl_unsigned_signed {
    ($t:ident, $u:ident) => {
        impl ArithmeticCheckedShl<$u> for $t {
            type Output = $t;

            /// Shifts `self` left (multiplies it by a power of 2). If the result is too large to
            /// fit in a `$t`, `None` is returned. Zero may be shifted by any amount, and any number
            /// may be shifted by any negative amount; shifting by a negative amount with a high
            /// absolute value returns `Some(0)`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::ArithmeticCheckedShl;
            ///
            /// assert_eq!(3u8.arithmetic_checked_shl(6), Some(192u8));
            /// assert_eq!(3u8.arithmetic_checked_shl(7), None);
            /// assert_eq!(3u8.arithmetic_checked_shl(100), None);
            /// assert_eq!(0u8.arithmetic_checked_shl(100), Some(0u8));
            /// assert_eq!(100u8.arithmetic_checked_shl(-3), Some(12u8));
            /// assert_eq!(100u8.arithmetic_checked_shl(-100), Some(0u8));
            /// ```
            fn arithmetic_checked_shl(self, bits: $u) -> Option<$t> {
                if bits >= 0 {
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
impl_arithmetic_checked_shl_unsigned_signed!(u8, i8);
impl_arithmetic_checked_shl_unsigned_signed!(u8, i16);
impl_arithmetic_checked_shl_unsigned_signed!(u8, i32);
impl_arithmetic_checked_shl_unsigned_signed!(u8, i64);
impl_arithmetic_checked_shl_unsigned_signed!(u8, i128);
impl_arithmetic_checked_shl_unsigned_signed!(u8, isize);
impl_arithmetic_checked_shl_unsigned_signed!(u16, i8);
impl_arithmetic_checked_shl_unsigned_signed!(u16, i16);
impl_arithmetic_checked_shl_unsigned_signed!(u16, i32);
impl_arithmetic_checked_shl_unsigned_signed!(u16, i64);
impl_arithmetic_checked_shl_unsigned_signed!(u16, i128);
impl_arithmetic_checked_shl_unsigned_signed!(u16, isize);
impl_arithmetic_checked_shl_unsigned_signed!(u32, i8);
impl_arithmetic_checked_shl_unsigned_signed!(u32, i16);
impl_arithmetic_checked_shl_unsigned_signed!(u32, i32);
impl_arithmetic_checked_shl_unsigned_signed!(u32, i64);
impl_arithmetic_checked_shl_unsigned_signed!(u32, i128);
impl_arithmetic_checked_shl_unsigned_signed!(u32, isize);
impl_arithmetic_checked_shl_unsigned_signed!(u64, i8);
impl_arithmetic_checked_shl_unsigned_signed!(u64, i16);
impl_arithmetic_checked_shl_unsigned_signed!(u64, i32);
impl_arithmetic_checked_shl_unsigned_signed!(u64, i64);
impl_arithmetic_checked_shl_unsigned_signed!(u64, i128);
impl_arithmetic_checked_shl_unsigned_signed!(u64, isize);
impl_arithmetic_checked_shl_unsigned_signed!(u128, i8);
impl_arithmetic_checked_shl_unsigned_signed!(u128, i16);
impl_arithmetic_checked_shl_unsigned_signed!(u128, i32);
impl_arithmetic_checked_shl_unsigned_signed!(u128, i64);
impl_arithmetic_checked_shl_unsigned_signed!(u128, i128);
impl_arithmetic_checked_shl_unsigned_signed!(u128, isize);
impl_arithmetic_checked_shl_unsigned_signed!(usize, i8);
impl_arithmetic_checked_shl_unsigned_signed!(usize, i16);
impl_arithmetic_checked_shl_unsigned_signed!(usize, i32);
impl_arithmetic_checked_shl_unsigned_signed!(usize, i64);
impl_arithmetic_checked_shl_unsigned_signed!(usize, i128);
impl_arithmetic_checked_shl_unsigned_signed!(usize, isize);

macro_rules! impl_arithmetic_checked_shl_signed_unsigned {
    ($t:ident, $u:ident) => {
        impl ArithmeticCheckedShl<$u> for $t {
            type Output = $t;

            /// Shifts `self` left (multiplies it by a power of 2). If the result is too large to
            /// fit in a `$t`, `None` is returned. Zero may be shifted by any amount.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::ArithmeticCheckedShl;
            ///
            /// assert_eq!(3i8.arithmetic_checked_shl(5), Some(96i8));
            /// assert_eq!(3i8.arithmetic_checked_shl(6), None);
            /// assert_eq!((-3i8).arithmetic_checked_shl(5), Some(-96i8));
            /// assert_eq!((-3i8).arithmetic_checked_shl(6), None);
            /// assert_eq!(3i8.arithmetic_checked_shl(100), None);
            /// assert_eq!((-3i8).arithmetic_checked_shl(100), None);
            /// assert_eq!(0i8.arithmetic_checked_shl(100), Some(0i8));
            /// ```
            fn arithmetic_checked_shl(self, bits: $u) -> Option<$t> {
                let abs = self.unsigned_abs();
                if self >= 0 {
                    abs.arithmetic_checked_shl(bits).and_then($t::checked_from)
                } else {
                    abs.arithmetic_checked_shl(bits).and_then(|x| {
                        if x == $t::MIN.unsigned_abs() {
                            Some($t::MIN)
                        } else {
                            $t::checked_from(x).map(|y| -y)
                        }
                    })
                }
            }
        }
    };
}
impl_arithmetic_checked_shl_signed_unsigned!(i8, u8);
impl_arithmetic_checked_shl_signed_unsigned!(i8, u16);
impl_arithmetic_checked_shl_signed_unsigned!(i8, u32);
impl_arithmetic_checked_shl_signed_unsigned!(i8, u64);
impl_arithmetic_checked_shl_signed_unsigned!(i8, u128);
impl_arithmetic_checked_shl_signed_unsigned!(i8, usize);
impl_arithmetic_checked_shl_signed_unsigned!(i16, u8);
impl_arithmetic_checked_shl_signed_unsigned!(i16, u16);
impl_arithmetic_checked_shl_signed_unsigned!(i16, u32);
impl_arithmetic_checked_shl_signed_unsigned!(i16, u64);
impl_arithmetic_checked_shl_signed_unsigned!(i16, u128);
impl_arithmetic_checked_shl_signed_unsigned!(i16, usize);
impl_arithmetic_checked_shl_signed_unsigned!(i32, u8);
impl_arithmetic_checked_shl_signed_unsigned!(i32, u16);
impl_arithmetic_checked_shl_signed_unsigned!(i32, u32);
impl_arithmetic_checked_shl_signed_unsigned!(i32, u64);
impl_arithmetic_checked_shl_signed_unsigned!(i32, u128);
impl_arithmetic_checked_shl_signed_unsigned!(i32, usize);
impl_arithmetic_checked_shl_signed_unsigned!(i64, u8);
impl_arithmetic_checked_shl_signed_unsigned!(i64, u16);
impl_arithmetic_checked_shl_signed_unsigned!(i64, u32);
impl_arithmetic_checked_shl_signed_unsigned!(i64, u64);
impl_arithmetic_checked_shl_signed_unsigned!(i64, u128);
impl_arithmetic_checked_shl_signed_unsigned!(i64, usize);
impl_arithmetic_checked_shl_signed_unsigned!(i128, u8);
impl_arithmetic_checked_shl_signed_unsigned!(i128, u16);
impl_arithmetic_checked_shl_signed_unsigned!(i128, u32);
impl_arithmetic_checked_shl_signed_unsigned!(i128, u64);
impl_arithmetic_checked_shl_signed_unsigned!(i128, u128);
impl_arithmetic_checked_shl_signed_unsigned!(i128, usize);
impl_arithmetic_checked_shl_signed_unsigned!(isize, u8);
impl_arithmetic_checked_shl_signed_unsigned!(isize, u16);
impl_arithmetic_checked_shl_signed_unsigned!(isize, u32);
impl_arithmetic_checked_shl_signed_unsigned!(isize, u64);
impl_arithmetic_checked_shl_signed_unsigned!(isize, u128);
impl_arithmetic_checked_shl_signed_unsigned!(isize, usize);

macro_rules! impl_arithmetic_checked_shl_signed_signed {
    ($t:ident, $u:ident) => {
        impl ArithmeticCheckedShl<$u> for $t {
            type Output = $t;

            /// Shifts `self` left (multiplies it by a power of 2). If the result is too large to
            /// fit in a `$t`, `None` is returned. Zero may be shifted by any amount, and any number
            /// may be shifted by any negative amount; shifting by a negative amount with a high
            /// absolute value returns `Some(0)` if `self` is positive, and `Some(-1)` if `self` is
            /// negative.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::ArithmeticCheckedShl;
            ///
            /// assert_eq!(3i8.arithmetic_checked_shl(5), Some(96i8));
            /// assert_eq!(3i8.arithmetic_checked_shl(6), None);
            /// assert_eq!((-3i8).arithmetic_checked_shl(5), Some(-96i8));
            /// assert_eq!((-3i8).arithmetic_checked_shl(6), None);
            /// assert_eq!(3i8.arithmetic_checked_shl(100), None);
            /// assert_eq!((-3i8).arithmetic_checked_shl(100), None);
            /// assert_eq!(0i8.arithmetic_checked_shl(100), Some(0i8));
            /// assert_eq!(100i8.arithmetic_checked_shl(-3), Some(12i8));
            /// assert_eq!((-100i8).arithmetic_checked_shl(-3), Some(-13i8));
            /// assert_eq!(100i8.arithmetic_checked_shl(-100), Some(0i8));
            /// assert_eq!((-100i8).arithmetic_checked_shl(-100), Some(-1i8));
            /// ```
            fn arithmetic_checked_shl(self, bits: $u) -> Option<$t> {
                if bits >= 0 {
                    self.arithmetic_checked_shl(bits.unsigned_abs())
                } else {
                    let width = $t::WIDTH.wrapping_into();
                    let abs_bits = bits.unsigned_abs();
                    Some(if width != 0 && abs_bits >= width {
                        -$t::iverson(self < 0)
                    } else {
                        self >> abs_bits
                    })
                }
            }
        }
    };
}
impl_arithmetic_checked_shl_signed_signed!(i8, i8);
impl_arithmetic_checked_shl_signed_signed!(i8, i16);
impl_arithmetic_checked_shl_signed_signed!(i8, i32);
impl_arithmetic_checked_shl_signed_signed!(i8, i64);
impl_arithmetic_checked_shl_signed_signed!(i8, i128);
impl_arithmetic_checked_shl_signed_signed!(i8, isize);
impl_arithmetic_checked_shl_signed_signed!(i16, i8);
impl_arithmetic_checked_shl_signed_signed!(i16, i16);
impl_arithmetic_checked_shl_signed_signed!(i16, i32);
impl_arithmetic_checked_shl_signed_signed!(i16, i64);
impl_arithmetic_checked_shl_signed_signed!(i16, i128);
impl_arithmetic_checked_shl_signed_signed!(i16, isize);
impl_arithmetic_checked_shl_signed_signed!(i32, i8);
impl_arithmetic_checked_shl_signed_signed!(i32, i16);
impl_arithmetic_checked_shl_signed_signed!(i32, i32);
impl_arithmetic_checked_shl_signed_signed!(i32, i64);
impl_arithmetic_checked_shl_signed_signed!(i32, i128);
impl_arithmetic_checked_shl_signed_signed!(i32, isize);
impl_arithmetic_checked_shl_signed_signed!(i64, i8);
impl_arithmetic_checked_shl_signed_signed!(i64, i16);
impl_arithmetic_checked_shl_signed_signed!(i64, i32);
impl_arithmetic_checked_shl_signed_signed!(i64, i64);
impl_arithmetic_checked_shl_signed_signed!(i64, i128);
impl_arithmetic_checked_shl_signed_signed!(i64, isize);
impl_arithmetic_checked_shl_signed_signed!(i128, i8);
impl_arithmetic_checked_shl_signed_signed!(i128, i16);
impl_arithmetic_checked_shl_signed_signed!(i128, i32);
impl_arithmetic_checked_shl_signed_signed!(i128, i64);
impl_arithmetic_checked_shl_signed_signed!(i128, i128);
impl_arithmetic_checked_shl_signed_signed!(i128, isize);
impl_arithmetic_checked_shl_signed_signed!(isize, i8);
impl_arithmetic_checked_shl_signed_signed!(isize, i16);
impl_arithmetic_checked_shl_signed_signed!(isize, i32);
impl_arithmetic_checked_shl_signed_signed!(isize, i64);
impl_arithmetic_checked_shl_signed_signed!(isize, i128);
impl_arithmetic_checked_shl_signed_signed!(isize, isize);
