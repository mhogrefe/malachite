use num::arithmetic::traits::{ShlRound, ShlRoundAssign, ShrRound, ShrRoundAssign, UnsignedAbs};
use num::basic::integers::PrimitiveInteger;
use num::conversion::traits::WrappingFrom;
use rounding_mode::RoundingMode;

macro_rules! impl_shl_round {
    ($t:ident, $u:ident) => {
        impl ShlRound<$u> for $t {
            type Output = $t;

            /// Shifts `self` left (multiplies it by a power of 2 or divides it by a power of 2 and
            /// takes the floor) and rounds according to the specified rounding mode. Passing
            /// `RoundingMode::Floor` or `RoundingMode::Down` is equivalent to using `>>`. To test
            /// whether `RoundingMode::Exact` can be passed, use
            /// `bits > 0 || self.divisible_by_power_of_two(bits)`. Rounding might only be necessary
            /// if `bits` is non-negative.
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
            /// use malachite_base::rounding_mode::RoundingMode;
            /// use malachite_base::num::arithmetic::traits::ShlRound;
            ///
            /// assert_eq!(0x101u16.shl_round(-8i8, RoundingMode::Down), 1);
            /// assert_eq!(0x101u32.shl_round(-8i16, RoundingMode::Up), 2);
            ///
            /// assert_eq!((-0x101i16).shl_round(-9i32, RoundingMode::Down), 0);
            /// assert_eq!((-0x101i32).shl_round(-9i64, RoundingMode::Up), -1);
            /// assert_eq!((-0x101i64).shl_round(-9i8, RoundingMode::Nearest), -1);
            /// assert_eq!((-0xffi32).shl_round(-9i16, RoundingMode::Nearest), 0);
            /// assert_eq!((-0x100i16).shl_round(-9i32, RoundingMode::Nearest), 0);
            ///
            /// assert_eq!(0x100u64.shl_round(-8i64, RoundingMode::Exact), 1);
            /// ```
            #[inline]
            fn shl_round(self, bits: $u, rm: RoundingMode) -> $t {
                if bits >= 0 {
                    let width = $u::wrapping_from($t::WIDTH);
                    if width >= 0 && bits >= width {
                        0
                    } else {
                        self << bits.unsigned_abs()
                    }
                } else {
                    self.shr_round(bits.unsigned_abs(), rm)
                }
            }
        }

        impl ShlRoundAssign<$u> for $t {
            /// Shifts `self` left (multiplies it by a power of 2 or divides it by a power of 2 and
            /// takes the floor) and rounds according to the specified rounding mode, in place.
            /// Passing `RoundingMode::Floor` or `RoundingMode::Down` is equivalent to using `>>`.
            /// To test whether `RoundingMode::Exact` can be passed, use
            /// `bits > 0 || self.divisible_by_power_of_two(bits)`. Rounding might only be necessary
            /// if `bits` is non-negative.
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
            /// use malachite_base::rounding_mode::RoundingMode;
            /// use malachite_base::num::arithmetic::traits::ShlRoundAssign;
            ///
            /// let mut x = 0x101u16;
            /// x.shl_round_assign(-8i8, RoundingMode::Down);
            /// assert_eq!(x, 1);
            ///
            /// let mut x = 0x101u32;
            /// x.shl_round_assign(-8i16, RoundingMode::Up);
            /// assert_eq!(x, 2);
            ///
            /// let mut x = -0x101i16;
            /// x.shl_round_assign(-9i32, RoundingMode::Down);
            /// assert_eq!(x, 0);
            ///
            /// let mut x = -0x101i32;
            /// x.shl_round_assign(-9i64, RoundingMode::Up);
            /// assert_eq!(x, -1);
            ///
            /// let mut x = -0x101i64;
            /// x.shl_round_assign(-9i8, RoundingMode::Nearest);
            /// assert_eq!(x, -1);
            ///
            /// let mut x = -0xffi32;
            /// x.shl_round_assign(-9i16, RoundingMode::Nearest);
            /// assert_eq!(x, 0);
            ///
            /// let mut x = -0x100i16;
            /// x.shl_round_assign(-9i32, RoundingMode::Nearest);
            /// assert_eq!(x, 0);
            ///
            /// let mut x = 0x100u64;
            /// x.shl_round_assign(-8i64, RoundingMode::Exact);
            /// assert_eq!(x, 1);
            /// ```
            #[inline]
            fn shl_round_assign(&mut self, bits: $u, rm: RoundingMode) {
                if bits >= 0 {
                    let width = $u::wrapping_from($t::WIDTH);
                    if width >= 0 && bits >= width {
                        *self = 0;
                    } else {
                        *self <<= bits.unsigned_abs();
                    }
                } else {
                    self.shr_round_assign(bits.unsigned_abs(), rm);
                }
            }
        }
    };
}
impl_shl_round!(u8, i8);
impl_shl_round!(u8, i16);
impl_shl_round!(u8, i32);
impl_shl_round!(u8, i64);
impl_shl_round!(u8, i128);
impl_shl_round!(u8, isize);
impl_shl_round!(u16, i8);
impl_shl_round!(u16, i16);
impl_shl_round!(u16, i32);
impl_shl_round!(u16, i64);
impl_shl_round!(u16, i128);
impl_shl_round!(u16, isize);
impl_shl_round!(u32, i8);
impl_shl_round!(u32, i16);
impl_shl_round!(u32, i32);
impl_shl_round!(u32, i64);
impl_shl_round!(u32, i128);
impl_shl_round!(u32, isize);
impl_shl_round!(u64, i8);
impl_shl_round!(u64, i16);
impl_shl_round!(u64, i32);
impl_shl_round!(u64, i64);
impl_shl_round!(u64, i128);
impl_shl_round!(u64, isize);
impl_shl_round!(u128, i8);
impl_shl_round!(u128, i16);
impl_shl_round!(u128, i32);
impl_shl_round!(u128, i64);
impl_shl_round!(u128, i128);
impl_shl_round!(u128, isize);
impl_shl_round!(usize, i8);
impl_shl_round!(usize, i16);
impl_shl_round!(usize, i32);
impl_shl_round!(usize, i64);
impl_shl_round!(usize, i128);
impl_shl_round!(usize, isize);
impl_shl_round!(i8, i8);
impl_shl_round!(i8, i16);
impl_shl_round!(i8, i32);
impl_shl_round!(i8, i64);
impl_shl_round!(i8, i128);
impl_shl_round!(i8, isize);
impl_shl_round!(i16, i8);
impl_shl_round!(i16, i16);
impl_shl_round!(i16, i32);
impl_shl_round!(i16, i64);
impl_shl_round!(i16, i128);
impl_shl_round!(i16, isize);
impl_shl_round!(i32, i8);
impl_shl_round!(i32, i16);
impl_shl_round!(i32, i32);
impl_shl_round!(i32, i64);
impl_shl_round!(i32, i128);
impl_shl_round!(i32, isize);
impl_shl_round!(i64, i8);
impl_shl_round!(i64, i16);
impl_shl_round!(i64, i32);
impl_shl_round!(i64, i64);
impl_shl_round!(i64, i128);
impl_shl_round!(i64, isize);
impl_shl_round!(i128, i8);
impl_shl_round!(i128, i16);
impl_shl_round!(i128, i32);
impl_shl_round!(i128, i64);
impl_shl_round!(i128, i128);
impl_shl_round!(i128, isize);
impl_shl_round!(isize, i8);
impl_shl_round!(isize, i16);
impl_shl_round!(isize, i32);
impl_shl_round!(isize, i64);
impl_shl_round!(isize, i128);
impl_shl_round!(isize, isize);
