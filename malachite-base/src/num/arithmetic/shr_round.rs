use num::arithmetic::traits::{Parity, PowerOfTwo, ShrRound, ShrRoundAssign, UnsignedAbs};
use num::basic::integers::PrimitiveInteger;
use num::conversion::traits::{WrappingFrom, WrappingInto};
use rounding_modes::RoundingMode;

macro_rules! impl_shr_round_unsigned_unsigned {
    ($t:ident, $u:ident) => {
        impl ShrRound<$u> for $t {
            type Output = $t;

            /// Shifts `self` right (divides it by a power of 2) and rounds according to the
            /// specified rounding mode. Passing `RoundingMode::Floor` or `RoundingMode::Down` is
            /// equivalent to using `>>`. To test whether `RoundingMode::Exact` can be passed, use
            /// `self.divisible_by_power_of_two(bits)`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `rm` is `RoundingMode::Exact` but `self` is not divisible by
            /// 2<sup>`bits`</sup>.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::rounding_modes::RoundingMode;
            /// use malachite_base::num::arithmetic::traits::ShrRound;
            ///
            /// assert_eq!(0x101u32.shr_round(8u8, RoundingMode::Down), 1);
            /// assert_eq!(0x101u16.shr_round(8u16, RoundingMode::Up), 2);
            ///
            /// assert_eq!(0x101u64.shr_round(9u32, RoundingMode::Down), 0);
            /// assert_eq!(0x101u32.shr_round(9u64, RoundingMode::Up), 1);
            /// assert_eq!(0x101u16.shr_round(9u8, RoundingMode::Nearest), 1);
            /// assert_eq!(0xffu8.shr_round(9u16, RoundingMode::Nearest), 0);
            /// assert_eq!(0x100u32.shr_round(9u32, RoundingMode::Nearest), 0);
            ///
            /// assert_eq!(0x100u32.shr_round(8u64, RoundingMode::Exact), 1);
            /// ```
            fn shr_round(self, bits: $u, rm: RoundingMode) -> $t {
                if bits == 0 || self == 0 {
                    return self;
                }
                let width = $u::wrapping_from($t::WIDTH);
                match rm {
                    RoundingMode::Down | RoundingMode::Floor if bits >= width => 0,
                    RoundingMode::Down | RoundingMode::Floor => self >> bits,
                    RoundingMode::Up | RoundingMode::Ceiling if bits >= width => 1,
                    RoundingMode::Up | RoundingMode::Ceiling => {
                        let shifted = self >> bits;
                        if shifted << bits == self {
                            shifted
                        } else {
                            shifted + 1
                        }
                    }
                    RoundingMode::Nearest
                        if bits == width && self > $t::power_of_two($t::WIDTH - 1) =>
                    {
                        1
                    }
                    RoundingMode::Nearest if bits >= width => 0,
                    RoundingMode::Nearest => {
                        let mostly_shifted = self >> (bits - 1);
                        if mostly_shifted.even() {
                            // round down
                            mostly_shifted >> 1
                        } else if mostly_shifted << (bits - 1) != self {
                            // round up
                            (mostly_shifted >> 1) + 1
                        } else {
                            // result is half-integer; round to even
                            let shifted = mostly_shifted >> 1;
                            if shifted.even() {
                                shifted
                            } else {
                                shifted + 1
                            }
                        }
                    }
                    RoundingMode::Exact if bits >= width => {
                        panic!("Right shift is not exact: {} >> {}", self, bits);
                    }
                    RoundingMode::Exact => {
                        let shifted = self >> bits;
                        if shifted << bits != self {
                            panic!("Right shift is not exact: {} >> {}", self, bits);
                        }
                        shifted
                    }
                }
            }
        }

        impl ShrRoundAssign<$u> for $t {
            /// Shifts `self` right (divides it by a power of 2) and rounds according to the
            /// specified rounding mode, in place. Passing `RoundingMode::Floor` or
            /// `RoundingMode::Down` is equivalent to using `>>`. To test whether
            /// `RoundingMode::Exact` can be passed, use `self.divisible_by_power_of_two(bits)`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `rm` is `RoundingMode::Exact` but `self` is not divisible by
            /// 2<sup>`bits`</sup>.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::rounding_modes::RoundingMode;
            /// use malachite_base::num::arithmetic::traits::ShrRoundAssign;
            ///
            /// let mut x = 0x101u32;
            /// x.shr_round_assign(8u8, RoundingMode::Down);
            /// assert_eq!(x, 1);
            ///
            /// let mut x = 0x101u16;
            /// x.shr_round_assign(8u16, RoundingMode::Up);
            /// assert_eq!(x, 2);
            ///
            /// let mut x = 0x101u64;
            /// x.shr_round_assign(9u32, RoundingMode::Down);
            /// assert_eq!(x, 0);
            ///
            /// let mut x = 0x101u32;
            /// x.shr_round_assign(9u64, RoundingMode::Up);
            /// assert_eq!(x, 1);
            ///
            /// let mut x = 0x101u16;
            /// x.shr_round_assign(9u8, RoundingMode::Nearest);
            /// assert_eq!(x, 1);
            ///
            /// let mut x = 0xffu8;
            /// x.shr_round_assign(9u16, RoundingMode::Nearest);
            /// assert_eq!(x, 0);
            ///
            /// let mut x = 0x100u32;
            /// x.shr_round_assign(9u32, RoundingMode::Nearest);
            /// assert_eq!(x, 0);
            ///
            /// let mut x = 0x100u32;
            /// x.shr_round_assign(8u64, RoundingMode::Exact);
            /// assert_eq!(x, 1);
            /// ```
            fn shr_round_assign(&mut self, bits: $u, rm: RoundingMode) {
                if bits == 0 || *self == 0 {
                    return;
                }
                let width = $u::wrapping_from($t::WIDTH);
                match rm {
                    RoundingMode::Down | RoundingMode::Floor if bits >= width => *self = 0,
                    RoundingMode::Down | RoundingMode::Floor => *self >>= bits,
                    RoundingMode::Up | RoundingMode::Ceiling if bits >= width => *self = 1,
                    RoundingMode::Up | RoundingMode::Ceiling => {
                        let original = *self;
                        *self >>= bits;
                        if *self << bits != original {
                            *self += 1;
                        }
                    }
                    RoundingMode::Nearest
                        if bits == width && *self > $t::power_of_two($t::WIDTH - 1) =>
                    {
                        *self = 1;
                    }
                    RoundingMode::Nearest if bits >= width => *self = 0,
                    RoundingMode::Nearest => {
                        let original = *self;
                        *self >>= bits - 1;
                        if self.even() {
                            // round down
                            *self >>= 1;
                        } else if *self << (bits - 1) != original {
                            // round up
                            *self >>= 1;
                            *self += 1;
                        } else {
                            // result is half-integer; round to even
                            *self >>= 1;
                            if self.odd() {
                                *self += 1;
                            }
                        }
                    }
                    RoundingMode::Exact if bits >= width => {
                        panic!("Right shift is not exact: {} >>= {}", *self, bits);
                    }
                    RoundingMode::Exact => {
                        let original = *self;
                        *self >>= bits;
                        if *self << bits != original {
                            panic!("Right shift is not exact: {} >>= {}", original, bits);
                        }
                    }
                }
            }
        }
    };
}
impl_shr_round_unsigned_unsigned!(u8, u8);
impl_shr_round_unsigned_unsigned!(u8, u16);
impl_shr_round_unsigned_unsigned!(u8, u32);
impl_shr_round_unsigned_unsigned!(u8, u64);
impl_shr_round_unsigned_unsigned!(u8, u128);
impl_shr_round_unsigned_unsigned!(u8, usize);
impl_shr_round_unsigned_unsigned!(u16, u8);
impl_shr_round_unsigned_unsigned!(u16, u16);
impl_shr_round_unsigned_unsigned!(u16, u32);
impl_shr_round_unsigned_unsigned!(u16, u64);
impl_shr_round_unsigned_unsigned!(u16, u128);
impl_shr_round_unsigned_unsigned!(u16, usize);
impl_shr_round_unsigned_unsigned!(u32, u8);
impl_shr_round_unsigned_unsigned!(u32, u16);
impl_shr_round_unsigned_unsigned!(u32, u32);
impl_shr_round_unsigned_unsigned!(u32, u64);
impl_shr_round_unsigned_unsigned!(u32, u128);
impl_shr_round_unsigned_unsigned!(u32, usize);
impl_shr_round_unsigned_unsigned!(u64, u8);
impl_shr_round_unsigned_unsigned!(u64, u16);
impl_shr_round_unsigned_unsigned!(u64, u32);
impl_shr_round_unsigned_unsigned!(u64, u64);
impl_shr_round_unsigned_unsigned!(u64, u128);
impl_shr_round_unsigned_unsigned!(u64, usize);
impl_shr_round_unsigned_unsigned!(u128, u8);
impl_shr_round_unsigned_unsigned!(u128, u16);
impl_shr_round_unsigned_unsigned!(u128, u32);
impl_shr_round_unsigned_unsigned!(u128, u64);
impl_shr_round_unsigned_unsigned!(u128, u128);
impl_shr_round_unsigned_unsigned!(u128, usize);
impl_shr_round_unsigned_unsigned!(usize, u8);
impl_shr_round_unsigned_unsigned!(usize, u16);
impl_shr_round_unsigned_unsigned!(usize, u32);
impl_shr_round_unsigned_unsigned!(usize, u64);
impl_shr_round_unsigned_unsigned!(usize, u128);
impl_shr_round_unsigned_unsigned!(usize, usize);

macro_rules! impl_shr_round_primitive_signed {
    ($t:ident, $u:ident) => {
        impl ShrRound<$u> for $t {
            type Output = $t;

            /// Shifts `self` right (divides it by a power of 2) and rounds according to the
            /// specified rounding mode. Passing `RoundingMode::Floor` or `RoundingMode::Down` is
            /// equivalent to using `>>`. To test whether `RoundingMode::Exact` can be passed, use
            /// `self.divisible_by_power_of_two(bits)`. Rounding might only be necessary if `bits`
            /// is non-negative.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `bits` is positive and `rm` is `RoundingMode::Exact` but `self` is not
            /// divisible by 2<sup>`bits`</sup>.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::rounding_modes::RoundingMode;
            /// use malachite_base::num::arithmetic::traits::ShrRound;
            ///
            /// assert_eq!(0x101u32.shr_round(8i8, RoundingMode::Down), 1);
            /// assert_eq!(0x101u16.shr_round(8i16, RoundingMode::Up), 2);
            ///
            /// assert_eq!((-0x101i32).shr_round(9i32, RoundingMode::Down), 0);
            /// assert_eq!((-0x101i64).shr_round(9i64, RoundingMode::Up), -1);
            /// assert_eq!((-0x101i16).shr_round(9i8, RoundingMode::Nearest), -1);
            /// assert_eq!((-0xffi32).shr_round(9i16, RoundingMode::Nearest), 0);
            /// assert_eq!((-0x100i64).shr_round(9i32, RoundingMode::Nearest), 0);
            ///
            /// assert_eq!(0x100u32.shr_round(8i64, RoundingMode::Exact), 1);
            /// ```
            fn shr_round(self, bits: $u, rm: RoundingMode) -> $t {
                if bits >= 0 {
                    self.shr_round(bits.unsigned_abs(), rm)
                } else {
                    let abs = bits.unsigned_abs();
                    if abs >= $t::WIDTH.wrapping_into() {
                        0
                    } else {
                        self << bits.unsigned_abs()
                    }
                }
            }
        }

        impl ShrRoundAssign<$u> for $t {
            /// Shifts `self` right (divides it by a power of 2) and rounds according to the
            /// specified rounding mode, in place. Passing `RoundingMode::Floor` or
            /// `RoundingMode::Down` is equivalent to using `>>`. To test whether
            /// `RoundingMode::Exact` can be passed, use `self.divisible_by_power_of_two(bits)`.
            /// Rounding might only be necessary if `bits` is non-negative.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `bits` is positive and `rm` is `RoundingMode::Exact` but `self` is not
            /// divisible by 2<sup>`bits`</sup>.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::rounding_modes::RoundingMode;
            /// use malachite_base::num::arithmetic::traits::ShrRoundAssign;
            ///
            /// let mut x = 0x101u32;
            /// x.shr_round_assign(8i8, RoundingMode::Down);
            /// assert_eq!(x, 1);
            ///
            /// let mut x = 0x101u16;
            /// x.shr_round_assign(8i16, RoundingMode::Up);
            /// assert_eq!(x, 2);
            ///
            /// let mut x = -0x101i32;
            /// x.shr_round_assign(9i32, RoundingMode::Down);
            /// assert_eq!(x, 0);
            ///
            /// let mut x = -0x101i64;
            /// x.shr_round_assign(9i64, RoundingMode::Up);
            /// assert_eq!(x, -1);
            ///
            /// let mut x = -0x101i16;
            /// x.shr_round_assign(9i8, RoundingMode::Nearest);
            /// assert_eq!(x, -1);
            ///
            /// let mut x = -0xffi32;
            /// x.shr_round_assign(9i16, RoundingMode::Nearest);
            /// assert_eq!(x, 0);
            ///
            /// let mut x = -0x100i64;
            /// x.shr_round_assign(9i32, RoundingMode::Nearest);
            /// assert_eq!(x, 0);
            ///
            /// let mut x = 0x100u32;
            /// x.shr_round_assign(8i64, RoundingMode::Exact);
            /// assert_eq!(x, 1);
            /// ```
            fn shr_round_assign(&mut self, bits: $u, rm: RoundingMode) {
                if bits >= 0 {
                    self.shr_round_assign(bits.unsigned_abs(), rm);
                } else {
                    let abs = bits.unsigned_abs();
                    if abs >= $t::WIDTH.wrapping_into() {
                        *self = 0;
                    } else {
                        *self <<= bits.unsigned_abs();
                    }
                }
            }
        }
    };
}
impl_shr_round_primitive_signed!(u8, i8);
impl_shr_round_primitive_signed!(u8, i16);
impl_shr_round_primitive_signed!(u8, i32);
impl_shr_round_primitive_signed!(u8, i64);
impl_shr_round_primitive_signed!(u8, i128);
impl_shr_round_primitive_signed!(u8, isize);
impl_shr_round_primitive_signed!(u16, i8);
impl_shr_round_primitive_signed!(u16, i16);
impl_shr_round_primitive_signed!(u16, i32);
impl_shr_round_primitive_signed!(u16, i64);
impl_shr_round_primitive_signed!(u16, i128);
impl_shr_round_primitive_signed!(u16, isize);
impl_shr_round_primitive_signed!(u32, i8);
impl_shr_round_primitive_signed!(u32, i16);
impl_shr_round_primitive_signed!(u32, i32);
impl_shr_round_primitive_signed!(u32, i64);
impl_shr_round_primitive_signed!(u32, i128);
impl_shr_round_primitive_signed!(u32, isize);
impl_shr_round_primitive_signed!(u64, i8);
impl_shr_round_primitive_signed!(u64, i16);
impl_shr_round_primitive_signed!(u64, i32);
impl_shr_round_primitive_signed!(u64, i64);
impl_shr_round_primitive_signed!(u64, i128);
impl_shr_round_primitive_signed!(u64, isize);
impl_shr_round_primitive_signed!(u128, i8);
impl_shr_round_primitive_signed!(u128, i16);
impl_shr_round_primitive_signed!(u128, i32);
impl_shr_round_primitive_signed!(u128, i64);
impl_shr_round_primitive_signed!(u128, i128);
impl_shr_round_primitive_signed!(u128, isize);
impl_shr_round_primitive_signed!(usize, i8);
impl_shr_round_primitive_signed!(usize, i16);
impl_shr_round_primitive_signed!(usize, i32);
impl_shr_round_primitive_signed!(usize, i64);
impl_shr_round_primitive_signed!(usize, i128);
impl_shr_round_primitive_signed!(usize, isize);
impl_shr_round_primitive_signed!(i8, i8);
impl_shr_round_primitive_signed!(i8, i16);
impl_shr_round_primitive_signed!(i8, i32);
impl_shr_round_primitive_signed!(i8, i64);
impl_shr_round_primitive_signed!(i8, i128);
impl_shr_round_primitive_signed!(i8, isize);
impl_shr_round_primitive_signed!(i16, i8);
impl_shr_round_primitive_signed!(i16, i16);
impl_shr_round_primitive_signed!(i16, i32);
impl_shr_round_primitive_signed!(i16, i64);
impl_shr_round_primitive_signed!(i16, i128);
impl_shr_round_primitive_signed!(i16, isize);
impl_shr_round_primitive_signed!(i32, i8);
impl_shr_round_primitive_signed!(i32, i16);
impl_shr_round_primitive_signed!(i32, i32);
impl_shr_round_primitive_signed!(i32, i64);
impl_shr_round_primitive_signed!(i32, i128);
impl_shr_round_primitive_signed!(i32, isize);
impl_shr_round_primitive_signed!(i64, i8);
impl_shr_round_primitive_signed!(i64, i16);
impl_shr_round_primitive_signed!(i64, i32);
impl_shr_round_primitive_signed!(i64, i64);
impl_shr_round_primitive_signed!(i64, i128);
impl_shr_round_primitive_signed!(i64, isize);
impl_shr_round_primitive_signed!(i128, i8);
impl_shr_round_primitive_signed!(i128, i16);
impl_shr_round_primitive_signed!(i128, i32);
impl_shr_round_primitive_signed!(i128, i64);
impl_shr_round_primitive_signed!(i128, i128);
impl_shr_round_primitive_signed!(i128, isize);
impl_shr_round_primitive_signed!(isize, i8);
impl_shr_round_primitive_signed!(isize, i16);
impl_shr_round_primitive_signed!(isize, i32);
impl_shr_round_primitive_signed!(isize, i64);
impl_shr_round_primitive_signed!(isize, i128);
impl_shr_round_primitive_signed!(isize, isize);

macro_rules! impl_shr_round_signed_unsigned {
    ($t:ident, $u:ident) => {
        impl ShrRound<$u> for $t {
            type Output = $t;

            /// Shifts `self` right (divides it by a power of 2) and rounds according to the
            /// specified rounding mode. Passing `RoundingMode::Floor` or `RoundingMode::Down` is
            /// equivalent to using `>>`. To test whether `RoundingMode::Exact` can be passed, use
            /// `self.divisible_by_power_of_two(bits)`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `rm` is `RoundingMode::Exact` but `self` is not divisible by
            /// 2<sup>`bits`</sup>.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::rounding_modes::RoundingMode;
            /// use malachite_base::num::arithmetic::traits::ShrRound;
            ///
            /// assert_eq!(0x101i32.shr_round(8u8, RoundingMode::Down), 1);
            /// assert_eq!(0x101i16.shr_round(8u16, RoundingMode::Up), 2);
            ///
            /// assert_eq!((-0x101i32).shr_round(9u32, RoundingMode::Down), 0);
            /// assert_eq!((-0x101i64).shr_round(9u64, RoundingMode::Up), -1);
            /// assert_eq!((-0x101i16).shr_round(9u8, RoundingMode::Nearest), -1);
            /// assert_eq!((-0xffi32).shr_round(9u16, RoundingMode::Nearest), 0);
            /// assert_eq!((-0x100i64).shr_round(9u32, RoundingMode::Nearest), 0);
            ///
            /// assert_eq!(0x100i32.shr_round(8u64, RoundingMode::Exact), 1);
            /// ```
            fn shr_round(self, bits: $u, rm: RoundingMode) -> $t {
                let abs = self.unsigned_abs();
                if self >= 0 {
                    $t::wrapping_from(abs.shr_round(bits, rm))
                } else {
                    let abs_shifted = abs.shr_round(bits, -rm);
                    if abs_shifted == 0 {
                        0
                    } else if abs_shifted == $t::MIN.unsigned_abs() {
                        $t::MIN
                    } else {
                        -$t::wrapping_from(abs_shifted)
                    }
                }
            }
        }

        impl ShrRoundAssign<$u> for $t {
            /// Shifts `self` right (divides it by a power of 2) and rounds according to the
            /// specified rounding mode, in place. Passing `RoundingMode::Floor` or
            /// `RoundingMode::Down` is equivalent to using `>>`. To test whether
            /// `RoundingMode::Exact` can be passed, use `self.divisible_by_power_of_two(bits)`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `rm` is `RoundingMode::Exact` but `self` is not divisible by
            /// 2<sup>`bits`</sup>.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::rounding_modes::RoundingMode;
            /// use malachite_base::num::arithmetic::traits::ShrRoundAssign;
            ///
            /// let mut x = 0x101i32;
            /// x.shr_round_assign(8u8, RoundingMode::Down);
            /// assert_eq!(x, 1);
            ///
            /// let mut x = 0x101i16;
            /// x.shr_round_assign(8u16, RoundingMode::Up);
            /// assert_eq!(x, 2);
            ///
            /// let mut x = -0x101i32;
            /// x.shr_round_assign(9u32, RoundingMode::Down);
            /// assert_eq!(x, 0);
            ///
            /// let mut x = -0x101i64;
            /// x.shr_round_assign(9u64, RoundingMode::Up);
            /// assert_eq!(x, -1);
            ///
            /// let mut x = -0x101i16;
            /// x.shr_round_assign(9u8, RoundingMode::Nearest);
            /// assert_eq!(x, -1);
            ///
            /// let mut x = -0xffi32;
            /// x.shr_round_assign(9u16, RoundingMode::Nearest);
            /// assert_eq!(x, 0);
            ///
            /// let mut x = -0x100i64;
            /// x.shr_round_assign(9u32, RoundingMode::Nearest);
            /// assert_eq!(x, 0);
            ///
            /// let mut x = 0x100u32;
            /// x.shr_round_assign(8i64, RoundingMode::Exact);
            /// assert_eq!(x, 1);
            /// ```
            #[inline]
            fn shr_round_assign(&mut self, bits: $u, rm: RoundingMode) {
                *self = self.shr_round(bits, rm);
            }
        }
    };
}
impl_shr_round_signed_unsigned!(i8, u8);
impl_shr_round_signed_unsigned!(i8, u16);
impl_shr_round_signed_unsigned!(i8, u32);
impl_shr_round_signed_unsigned!(i8, u64);
impl_shr_round_signed_unsigned!(i8, u128);
impl_shr_round_signed_unsigned!(i8, usize);
impl_shr_round_signed_unsigned!(i16, u8);
impl_shr_round_signed_unsigned!(i16, u16);
impl_shr_round_signed_unsigned!(i16, u32);
impl_shr_round_signed_unsigned!(i16, u64);
impl_shr_round_signed_unsigned!(i16, u128);
impl_shr_round_signed_unsigned!(i16, usize);
impl_shr_round_signed_unsigned!(i32, u8);
impl_shr_round_signed_unsigned!(i32, u16);
impl_shr_round_signed_unsigned!(i32, u32);
impl_shr_round_signed_unsigned!(i32, u64);
impl_shr_round_signed_unsigned!(i32, u128);
impl_shr_round_signed_unsigned!(i32, usize);
impl_shr_round_signed_unsigned!(i64, u8);
impl_shr_round_signed_unsigned!(i64, u16);
impl_shr_round_signed_unsigned!(i64, u32);
impl_shr_round_signed_unsigned!(i64, u64);
impl_shr_round_signed_unsigned!(i64, u128);
impl_shr_round_signed_unsigned!(i64, usize);
impl_shr_round_signed_unsigned!(i128, u8);
impl_shr_round_signed_unsigned!(i128, u16);
impl_shr_round_signed_unsigned!(i128, u32);
impl_shr_round_signed_unsigned!(i128, u64);
impl_shr_round_signed_unsigned!(i128, u128);
impl_shr_round_signed_unsigned!(i128, usize);
impl_shr_round_signed_unsigned!(isize, u8);
impl_shr_round_signed_unsigned!(isize, u16);
impl_shr_round_signed_unsigned!(isize, u32);
impl_shr_round_signed_unsigned!(isize, u64);
impl_shr_round_signed_unsigned!(isize, u128);
impl_shr_round_signed_unsigned!(isize, usize);
