use std::cmp::Ordering;

use comparison::Min;
use num::arithmetic::traits::{
    CheckedAdd, CheckedDiv, CheckedMul, CheckedPow, CheckedRem, CheckedSub, DivAssignRem, DivExact,
    DivExactAssign, DivRem, DivRound, DivRoundAssign, DivisibleBy, DivisibleByPowerOfTwo, EqMod,
    EqModPowerOfTwo, Mod, ModAssign, OverflowingAdd, OverflowingAddAssign, OverflowingDiv,
    OverflowingDivAssign, OverflowingMul, OverflowingMulAssign, OverflowingNeg,
    OverflowingNegAssign, OverflowingPow, OverflowingRem, OverflowingRemAssign, OverflowingSub,
    OverflowingSubAssign, Parity, Pow, SaturatingAdd, SaturatingAddAssign, SaturatingMul,
    SaturatingMulAssign, SaturatingPow, SaturatingSub, SaturatingSubAssign, ShlRound,
    ShlRoundAssign, ShrRound, ShrRoundAssign, Sign, UnsignedAbs, WrappingAdd, WrappingAddAssign,
    WrappingDiv, WrappingDivAssign, WrappingMul, WrappingMulAssign, WrappingNeg, WrappingNegAssign,
    WrappingPow, WrappingRem, WrappingRemAssign, WrappingSub, WrappingSubAssign,
};
use num::conversion::traits::{ExactFrom, WrappingFrom};
use round::RoundingMode;

/// This macro defines trait implementations that are the same for unsigned and signed types.
macro_rules! impl_arithmetic_traits {
    ($t:ident) => {
        impl Sign for $t {
            /// Returns `Greater`, `Equal`, or `Less`, depending on whether `self` is positive,
            /// zero, or negative, respectively.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::Sign;
            /// use std::cmp::Ordering;
            ///
            /// assert_eq!(0u8.sign(), Ordering::Equal);
            /// assert_eq!(100u64.sign(), Ordering::Greater);
            /// assert_eq!((-100i16).sign(), Ordering::Less);
            /// ```
            #[inline]
            fn sign(&self) -> Ordering {
                self.cmp(&0)
            }
        }

        impl WrappingNeg for $t {
            type Output = $t;

            #[inline]
            fn wrapping_neg(self) -> $t {
                $t::wrapping_neg(self)
            }
        }

        impl WrappingNegAssign for $t {
            /// Replaces `self` with its negative, wrapping around at the boundary of the type.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::WrappingNegAssign;
            ///
            /// let mut x = 0i8;
            /// x.wrapping_neg_assign();
            /// assert_eq!(x, 0);
            ///
            /// let mut x = 100u64;
            /// x.wrapping_neg_assign();
            /// assert_eq!(x, 18446744073709551516);
            ///
            /// let mut x = -100i64;
            /// x.wrapping_neg_assign();
            /// assert_eq!(x, 100);
            ///
            /// let mut x = -128i8;
            /// x.wrapping_neg_assign();
            /// assert_eq!(x, -128);
            /// ```
            #[inline]
            fn wrapping_neg_assign(&mut self) {
                *self = self.wrapping_neg();
            }
        }

        impl OverflowingNeg for $t {
            type Output = $t;

            #[inline]
            fn overflowing_neg(self) -> ($t, bool) {
                $t::overflowing_neg(self)
            }
        }

        impl OverflowingNegAssign for $t {
            /// Replaces `self` with its negative.
            ///
            /// Returns a boolean indicating whether an arithmetic overflow would occur. If an
            /// overflow would have occurred then the wrapped value is assigned.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::OverflowingNegAssign;
            ///
            /// let mut x = 0i8;
            /// assert_eq!(x.overflowing_neg_assign(), false);
            /// assert_eq!(x, 0);
            ///
            /// let mut x = 100u64;
            /// assert_eq!(x.overflowing_neg_assign(), true);
            /// assert_eq!(x, 18446744073709551516);
            ///
            /// let mut x = -100i64;
            /// assert_eq!(x.overflowing_neg_assign(), false);
            /// assert_eq!(x, 100);
            ///
            /// let mut x = -128i8;
            /// assert_eq!(x.overflowing_neg_assign(), true);
            /// assert_eq!(x, -128);
            /// ```
            #[inline]
            fn overflowing_neg_assign(&mut self) -> bool {
                let (result, overflow) = self.overflowing_neg();
                *self = result;
                overflow
            }
        }

        impl CheckedAdd<$t> for $t {
            type Output = $t;

            #[inline]
            fn checked_add(self, rhs: $t) -> Option<$t> {
                $t::checked_add(self, rhs)
            }
        }

        impl SaturatingAdd<$t> for $t {
            type Output = $t;

            #[inline]
            fn saturating_add(self, rhs: $t) -> $t {
                $t::saturating_add(self, rhs)
            }
        }

        impl SaturatingAddAssign for $t {
            /// Replaces `self` with `self + rhs`, saturating at the numeric bounds instead of
            /// overflowing.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::SaturatingAddAssign;
            ///
            /// let mut x = 123u16;
            /// x.saturating_add_assign(456);
            /// assert_eq!(x, 579);
            ///
            /// let mut x = 123u8;
            /// x.saturating_add_assign(200);
            /// assert_eq!(x, 255);
            /// ```
            #[inline]
            fn saturating_add_assign(&mut self, rhs: $t) {
                *self = self.saturating_add(rhs);
            }
        }

        impl WrappingAdd<$t> for $t {
            type Output = $t;

            #[inline]
            fn wrapping_add(self, rhs: $t) -> $t {
                $t::wrapping_add(self, rhs)
            }
        }

        impl WrappingAddAssign for $t {
            /// Replaces `self` with `self + rhs`, wrapping around at the boundary of the type.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::WrappingAddAssign;
            ///
            /// let mut x = 123u16;
            /// x.wrapping_add_assign(456);
            /// assert_eq!(x, 579);
            ///
            /// let mut x = 123u8;
            /// x.wrapping_add_assign(200);
            /// assert_eq!(x, 67);
            /// ```
            #[inline]
            fn wrapping_add_assign(&mut self, rhs: $t) {
                *self = self.wrapping_add(rhs);
            }
        }

        impl OverflowingAdd<$t> for $t {
            type Output = $t;

            #[inline]
            fn overflowing_add(self, rhs: $t) -> ($t, bool) {
                $t::overflowing_add(self, rhs)
            }
        }

        impl OverflowingAddAssign for $t {
            /// Replaces `self` with `self + rhs`.
            ///
            /// Returns a boolean indicating whether an arithmetic overflow would occur. If an
            /// overflow would have occurred then the wrapped value is assigned.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::OverflowingAddAssign;
            ///
            /// let mut x = 123u16;
            /// assert_eq!(x.overflowing_add_assign(456), false);
            /// assert_eq!(x, 579);
            ///
            /// let mut x = 123u8;
            /// assert_eq!(x.overflowing_add_assign(200), true);
            /// assert_eq!(x, 67);
            /// ```
            #[inline]
            fn overflowing_add_assign(&mut self, rhs: $t) -> bool {
                let (result, overflow) = self.overflowing_add(rhs);
                *self = result;
                overflow
            }
        }

        impl CheckedSub<$t> for $t {
            type Output = $t;

            #[inline]
            fn checked_sub(self, rhs: $t) -> Option<$t> {
                $t::checked_sub(self, rhs)
            }
        }

        impl SaturatingSub<$t> for $t {
            type Output = $t;

            #[inline]
            fn saturating_sub(self, rhs: $t) -> $t {
                $t::saturating_sub(self, rhs)
            }
        }

        impl SaturatingSubAssign for $t {
            /// Replaces `self` with `self - rhs`, saturating at the numeric bounds instead of
            /// overflowing.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::SaturatingSubAssign;
            ///
            /// let mut x = 456u16;
            /// x.saturating_sub_assign(123);
            /// assert_eq!(x, 333);
            ///
            /// let mut x = 123u16;
            /// x.saturating_sub_assign(456);
            /// assert_eq!(x, 0);
            /// ```
            #[inline]
            fn saturating_sub_assign(&mut self, rhs: $t) {
                *self = self.saturating_sub(rhs);
            }
        }

        impl WrappingSub<$t> for $t {
            type Output = $t;

            #[inline]
            fn wrapping_sub(self, rhs: $t) -> $t {
                $t::wrapping_sub(self, rhs)
            }
        }

        impl WrappingSubAssign for $t {
            /// Replaces `self` with `self - rhs`, wrapping around at the boundary of the type.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::WrappingSubAssign;
            ///
            /// let mut x = 456u16;
            /// x.wrapping_sub_assign(123);
            /// assert_eq!(x, 333);
            ///
            /// let mut x = 123u16;
            /// x.wrapping_sub_assign(456);
            /// assert_eq!(x, 65_203);
            /// ```
            #[inline]
            fn wrapping_sub_assign(&mut self, rhs: $t) {
                *self = self.wrapping_sub(rhs);
            }
        }

        impl OverflowingSub<$t> for $t {
            type Output = $t;

            #[inline]
            fn overflowing_sub(self, rhs: $t) -> ($t, bool) {
                $t::overflowing_sub(self, rhs)
            }
        }

        impl OverflowingSubAssign for $t {
            /// Replaces `self` with `self - rhs`.
            ///
            /// Returns a boolean indicating whether an arithmetic overflow would occur. If an
            /// overflow would have occurred then the wrapped value is assigned.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::OverflowingSubAssign;
            ///
            /// let mut x = 456u16;
            /// assert_eq!(x.overflowing_sub_assign(123), false);
            /// assert_eq!(x, 333);
            ///
            /// let mut x = 123u16;
            /// assert_eq!(x.overflowing_sub_assign(456), true);
            /// assert_eq!(x, 65_203);
            /// ```
            #[inline]
            fn overflowing_sub_assign(&mut self, rhs: $t) -> bool {
                let (result, overflow) = self.overflowing_sub(rhs);
                *self = result;
                overflow
            }
        }

        impl CheckedMul<$t> for $t {
            type Output = $t;

            #[inline]
            fn checked_mul(self, rhs: $t) -> Option<$t> {
                $t::checked_mul(self, rhs)
            }
        }

        impl SaturatingMul<$t> for $t {
            type Output = $t;

            #[inline]
            fn saturating_mul(self, rhs: $t) -> $t {
                $t::saturating_mul(self, rhs)
            }
        }

        impl SaturatingMulAssign for $t {
            /// Replaces `self` with `self * rhs`, saturating at the numeric bounds instead of
            /// overflowing.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::SaturatingMulAssign;
            ///
            /// let mut x = 123u16;
            /// x.saturating_mul_assign(456);
            /// assert_eq!(x, 56_088);
            ///
            /// let mut x = 123u8;
            /// x.saturating_mul_assign(200);
            /// assert_eq!(x, 255);
            /// ```
            #[inline]
            fn saturating_mul_assign(&mut self, rhs: $t) {
                *self = self.saturating_mul(rhs);
            }
        }

        impl WrappingMul<$t> for $t {
            type Output = $t;

            #[inline]
            fn wrapping_mul(self, rhs: $t) -> $t {
                $t::wrapping_mul(self, rhs)
            }
        }

        impl WrappingMulAssign for $t {
            /// Replaces `self` with `self * rhs`, wrapping around at the boundary of the type.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::WrappingMulAssign;
            ///
            /// let mut x = 123u16;
            /// x.wrapping_mul_assign(456);
            /// assert_eq!(x, 56_088);
            ///
            /// let mut x = 123u8;
            /// x.wrapping_mul_assign(200);
            /// assert_eq!(x, 24);
            /// ```
            #[inline]
            fn wrapping_mul_assign(&mut self, rhs: $t) {
                *self = self.wrapping_mul(rhs);
            }
        }

        impl OverflowingMul<$t> for $t {
            type Output = $t;

            #[inline]
            fn overflowing_mul(self, rhs: $t) -> ($t, bool) {
                $t::overflowing_mul(self, rhs)
            }
        }

        impl OverflowingMulAssign for $t {
            /// Replaces `self` with `self * rhs`.
            ///
            /// Returns a boolean indicating whether an arithmetic overflow would occur. If an
            /// overflow would have occurred then the wrapped value is assigned.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::OverflowingMulAssign;
            ///
            /// let mut x = 123u16;
            /// assert_eq!(x.overflowing_mul_assign(456), false);
            /// assert_eq!(x, 56_088);
            ///
            /// let mut x = 123u8;
            /// assert_eq!(x.overflowing_mul_assign(200), true);
            /// assert_eq!(x, 24);
            /// ```
            #[inline]
            fn overflowing_mul_assign(&mut self, rhs: $t) -> bool {
                let (result, overflow) = self.overflowing_mul(rhs);
                *self = result;
                overflow
            }
        }

        impl CheckedDiv<$t> for $t {
            type Output = $t;

            #[inline]
            fn checked_div(self, rhs: $t) -> Option<$t> {
                $t::checked_div(self, rhs)
            }
        }

        impl CheckedRem<$t> for $t {
            type Output = $t;

            #[inline]
            fn checked_rem(self, rhs: $t) -> Option<$t> {
                $t::checked_rem(self, rhs)
            }
        }

        impl CheckedPow<u64> for $t {
            type Output = $t;

            #[inline]
            fn checked_pow(self, exp: u64) -> Option<$t> {
                $t::checked_pow(self, u32::exact_from(exp))
            }
        }

        impl SaturatingPow<u64> for $t {
            type Output = $t;

            #[inline]
            fn saturating_pow(self, rhs: u64) -> $t {
                $t::saturating_pow(self, u32::exact_from(rhs))
            }
        }

        impl WrappingDiv<$t> for $t {
            type Output = $t;

            #[inline]
            fn wrapping_div(self, rhs: $t) -> $t {
                $t::wrapping_div(self, rhs)
            }
        }

        impl WrappingRem<$t> for $t {
            type Output = $t;

            #[inline]
            fn wrapping_rem(self, rhs: $t) -> $t {
                $t::wrapping_rem(self, rhs)
            }
        }

        impl WrappingPow<u64> for $t {
            type Output = $t;

            #[inline]
            fn wrapping_pow(self, rhs: u64) -> $t {
                $t::wrapping_pow(self, u32::exact_from(rhs))
            }
        }

        impl OverflowingDiv<$t> for $t {
            type Output = $t;

            #[inline]
            fn overflowing_div(self, rhs: $t) -> ($t, bool) {
                $t::overflowing_div(self, rhs)
            }
        }

        impl OverflowingRem<$t> for $t {
            type Output = $t;

            #[inline]
            fn overflowing_rem(self, rhs: $t) -> ($t, bool) {
                $t::overflowing_rem(self, rhs)
            }
        }

        impl OverflowingPow<u64> for $t {
            type Output = $t;

            #[inline]
            fn overflowing_pow(self, rhs: u64) -> ($t, bool) {
                $t::overflowing_pow(self, u32::exact_from(rhs))
            }
        }

        impl Pow<u64> for $t {
            type Output = $t;

            #[inline]
            fn pow(self, exp: u64) -> $t {
                $t::pow(self, u32::exact_from(exp))
            }
        }

        impl WrappingDivAssign for $t {
            #[inline]
            fn wrapping_div_assign(&mut self, rhs: $t) {
                *self = self.wrapping_div(rhs);
            }
        }

        impl WrappingRemAssign for $t {
            #[inline]
            fn wrapping_rem_assign(&mut self, rhs: $t) {
                *self = self.wrapping_rem(rhs);
            }
        }

        impl OverflowingDivAssign for $t {
            #[inline]
            fn overflowing_div_assign(&mut self, rhs: $t) -> bool {
                let (result, overflow) = self.overflowing_div(rhs);
                *self = result;
                overflow
            }
        }

        impl OverflowingRemAssign for $t {
            #[inline]
            fn overflowing_rem_assign(&mut self, rhs: $t) -> bool {
                let (result, overflow) = self.overflowing_rem(rhs);
                *self = result;
                overflow
            }
        }

        impl Parity for $t {
            #[inline]
            fn even(self) -> bool {
                (self & 1) == 0
            }

            #[inline]
            fn odd(self) -> bool {
                (self & 1) != 0
            }
        }

        impl EqModPowerOfTwo<Self> for $t {
            #[inline]
            fn eq_mod_power_of_two(self, other: $t, pow: u64) -> bool {
                (self ^ other).divisible_by_power_of_two(pow)
            }
        }

        impl DivRem for $t {
            type DivOutput = $t;
            type RemOutput = $t;

            #[inline]
            fn div_rem(self, rhs: $t) -> ($t, $t) {
                (self / rhs, self % rhs)
            }
        }

        impl DivAssignRem for $t {
            type RemOutput = $t;

            #[inline]
            fn div_assign_rem(&mut self, rhs: $t) -> $t {
                let rem = *self % rhs;
                *self /= rhs;
                rem
            }
        }

        impl DivExact for $t {
            type Output = $t;

            #[inline]
            fn div_exact(self, rhs: $t) -> $t {
                self / rhs
            }
        }

        impl DivExactAssign for $t {
            #[inline]
            fn div_exact_assign(&mut self, rhs: $t) {
                *self /= rhs;
            }
        }

        impl DivisibleBy for $t {
            #[inline]
            fn divisible_by(self, rhs: $t) -> bool {
                self == 0 || rhs != 0 && self % rhs == 0
            }
        }

        impl DivRoundAssign for $t {
            fn div_round_assign(&mut self, rhs: $t, rm: RoundingMode) {
                *self = self.div_round(rhs, rm);
            }
        }

        impl ModAssign for $t {
            #[inline]
            fn mod_assign(&mut self, rhs: $t) {
                *self %= rhs;
            }
        }

        impl EqMod for $t {
            #[inline]
            fn eq_mod(self, other: $t, m: $t) -> bool {
                self == other || m != 0 && self.mod_op(m) == other.mod_op(m)
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
impl_arithmetic_traits!(i8);
impl_arithmetic_traits!(i16);
impl_arithmetic_traits!(i32);
impl_arithmetic_traits!(i64);
impl_arithmetic_traits!(i128);
impl_arithmetic_traits!(isize);

macro_rules! round_shift_primitive_signed {
    ($t:ident, $u:ident) => {
        impl ShlRound<$u> for $t {
            type Output = $t;

            #[inline]
            fn shl_round(self, other: $u, rm: RoundingMode) -> $t {
                if other >= 0 {
                    self << other.unsigned_abs()
                } else {
                    self.shr_round(other.unsigned_abs(), rm)
                }
            }
        }

        impl ShlRoundAssign<$u> for $t {
            #[inline]
            fn shl_round_assign(&mut self, other: $u, rm: RoundingMode) {
                if other >= 0 {
                    *self <<= other.unsigned_abs();
                } else {
                    self.shr_round_assign(other.unsigned_abs(), rm);
                }
            }
        }

        impl ShrRound<$u> for $t {
            type Output = $t;

            #[inline]
            fn shr_round(self, other: $u, rm: RoundingMode) -> $t {
                if other >= 0 {
                    self.shr_round(other.unsigned_abs(), rm)
                } else {
                    self << other.unsigned_abs()
                }
            }
        }

        impl ShrRoundAssign<$u> for $t {
            #[inline]
            fn shr_round_assign(&mut self, other: $u, rm: RoundingMode) {
                if other >= 0 {
                    self.shr_round_assign(other.unsigned_abs(), rm);
                } else {
                    *self <<= other.unsigned_abs()
                }
            }
        }
    };
}
round_shift_primitive_signed!(u8, i8);
round_shift_primitive_signed!(u8, i16);
round_shift_primitive_signed!(u8, i32);
round_shift_primitive_signed!(u8, i64);
round_shift_primitive_signed!(u8, i128);
round_shift_primitive_signed!(u8, isize);
round_shift_primitive_signed!(u16, i8);
round_shift_primitive_signed!(u16, i16);
round_shift_primitive_signed!(u16, i32);
round_shift_primitive_signed!(u16, i64);
round_shift_primitive_signed!(u16, i128);
round_shift_primitive_signed!(u16, isize);
round_shift_primitive_signed!(u32, i8);
round_shift_primitive_signed!(u32, i16);
round_shift_primitive_signed!(u32, i32);
round_shift_primitive_signed!(u32, i64);
round_shift_primitive_signed!(u32, i128);
round_shift_primitive_signed!(u32, isize);
round_shift_primitive_signed!(u64, i8);
round_shift_primitive_signed!(u64, i16);
round_shift_primitive_signed!(u64, i32);
round_shift_primitive_signed!(u64, i64);
round_shift_primitive_signed!(u64, i128);
round_shift_primitive_signed!(u64, isize);
round_shift_primitive_signed!(u128, i8);
round_shift_primitive_signed!(u128, i16);
round_shift_primitive_signed!(u128, i32);
round_shift_primitive_signed!(u128, i64);
round_shift_primitive_signed!(u128, i128);
round_shift_primitive_signed!(u128, isize);
round_shift_primitive_signed!(usize, i8);
round_shift_primitive_signed!(usize, i16);
round_shift_primitive_signed!(usize, i32);
round_shift_primitive_signed!(usize, i64);
round_shift_primitive_signed!(usize, i128);
round_shift_primitive_signed!(usize, isize);
round_shift_primitive_signed!(i8, i8);
round_shift_primitive_signed!(i8, i16);
round_shift_primitive_signed!(i8, i32);
round_shift_primitive_signed!(i8, i64);
round_shift_primitive_signed!(i8, i128);
round_shift_primitive_signed!(i8, isize);
round_shift_primitive_signed!(i16, i8);
round_shift_primitive_signed!(i16, i16);
round_shift_primitive_signed!(i16, i32);
round_shift_primitive_signed!(i16, i64);
round_shift_primitive_signed!(i16, i128);
round_shift_primitive_signed!(i16, isize);
round_shift_primitive_signed!(i32, i8);
round_shift_primitive_signed!(i32, i16);
round_shift_primitive_signed!(i32, i32);
round_shift_primitive_signed!(i32, i64);
round_shift_primitive_signed!(i32, i128);
round_shift_primitive_signed!(i32, isize);
round_shift_primitive_signed!(i64, i8);
round_shift_primitive_signed!(i64, i16);
round_shift_primitive_signed!(i64, i32);
round_shift_primitive_signed!(i64, i64);
round_shift_primitive_signed!(i64, i128);
round_shift_primitive_signed!(i64, isize);
round_shift_primitive_signed!(i128, i8);
round_shift_primitive_signed!(i128, i16);
round_shift_primitive_signed!(i128, i32);
round_shift_primitive_signed!(i128, i64);
round_shift_primitive_signed!(i128, i128);
round_shift_primitive_signed!(i128, isize);
round_shift_primitive_signed!(isize, i8);
round_shift_primitive_signed!(isize, i16);
round_shift_primitive_signed!(isize, i32);
round_shift_primitive_signed!(isize, i64);
round_shift_primitive_signed!(isize, i128);
round_shift_primitive_signed!(isize, isize);

macro_rules! round_shift_signed_unsigned {
    ($t:ident, $u:ident) => {
        impl ShrRound<$u> for $t {
            type Output = $t;

            fn shr_round(self, other: $u, rm: RoundingMode) -> $t {
                let abs = self.unsigned_abs();
                if self >= 0 {
                    $t::wrapping_from(abs.shr_round(other, rm))
                } else {
                    let abs_shifted = abs.shr_round(other, -rm);
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
            #[inline]
            fn shr_round_assign(&mut self, other: $u, rm: RoundingMode) {
                *self = self.shr_round(other, rm);
            }
        }
    };
}
round_shift_signed_unsigned!(i8, u8);
round_shift_signed_unsigned!(i8, u16);
round_shift_signed_unsigned!(i8, u32);
round_shift_signed_unsigned!(i8, u64);
round_shift_signed_unsigned!(i8, u128);
round_shift_signed_unsigned!(i8, usize);
round_shift_signed_unsigned!(i16, u8);
round_shift_signed_unsigned!(i16, u16);
round_shift_signed_unsigned!(i16, u32);
round_shift_signed_unsigned!(i16, u64);
round_shift_signed_unsigned!(i16, u128);
round_shift_signed_unsigned!(i16, usize);
round_shift_signed_unsigned!(i32, u8);
round_shift_signed_unsigned!(i32, u16);
round_shift_signed_unsigned!(i32, u32);
round_shift_signed_unsigned!(i32, u64);
round_shift_signed_unsigned!(i32, u128);
round_shift_signed_unsigned!(i32, usize);
round_shift_signed_unsigned!(i64, u8);
round_shift_signed_unsigned!(i64, u16);
round_shift_signed_unsigned!(i64, u32);
round_shift_signed_unsigned!(i64, u64);
round_shift_signed_unsigned!(i64, u128);
round_shift_signed_unsigned!(i64, usize);
round_shift_signed_unsigned!(i128, u8);
round_shift_signed_unsigned!(i128, u16);
round_shift_signed_unsigned!(i128, u32);
round_shift_signed_unsigned!(i128, u64);
round_shift_signed_unsigned!(i128, u128);
round_shift_signed_unsigned!(i128, usize);
round_shift_signed_unsigned!(isize, u8);
round_shift_signed_unsigned!(isize, u16);
round_shift_signed_unsigned!(isize, u32);
round_shift_signed_unsigned!(isize, u64);
round_shift_signed_unsigned!(isize, u128);
round_shift_signed_unsigned!(isize, usize);
