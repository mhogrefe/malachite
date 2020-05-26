use num::arithmetic::traits::{
    CheckedPow, DivAssignRem, DivExact, DivExactAssign, DivRem, DivRound, DivRoundAssign,
    DivisibleBy, EqMod, Mod, ModAssign, OverflowingPow, Pow, SaturatingPow, WrappingPow,
};
use num::conversion::traits::ExactFrom;
use round::RoundingMode;

macro_rules! impl_arithmetic_traits {
    ($t:ident) => {
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

        impl WrappingPow<u64> for $t {
            type Output = $t;

            #[inline]
            fn wrapping_pow(self, rhs: u64) -> $t {
                $t::wrapping_pow(self, u32::exact_from(rhs))
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
