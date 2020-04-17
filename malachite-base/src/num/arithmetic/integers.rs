use comparison::Min;
use num::arithmetic::traits::{
    CheckedDiv, CheckedPow, CheckedRem, DivAssignRem, DivExact, DivExactAssign, DivRem, DivRound,
    DivRoundAssign, DivisibleBy, DivisibleByPowerOfTwo, EqMod, EqModPowerOfTwo, Mod, ModAssign,
    OverflowingDiv, OverflowingDivAssign, OverflowingPow, OverflowingRem, OverflowingRemAssign,
    Parity, Pow, SaturatingPow, ShlRound, ShlRoundAssign, ShrRound, ShrRoundAssign, UnsignedAbs,
    WrappingDiv, WrappingDivAssign, WrappingPow, WrappingRem, WrappingRemAssign,
};
use num::conversion::traits::{ExactFrom, WrappingFrom};
use round::RoundingMode;

macro_rules! impl_arithmetic_traits {
    ($t:ident) => {
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
