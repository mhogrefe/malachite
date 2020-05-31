use num::arithmetic::traits::{CeilingMod, CeilingModAssign, DivRound, Mod, NegMod, UnsignedAbs};
use num::conversion::traits::ExactFrom;
use round::RoundingMode;

macro_rules! impl_arithmetic_traits {
    ($t:ident) => {
        impl Mod for $t {
            type Output = $t;

            #[inline]
            fn mod_op(self, other: $t) -> $t {
                let remainder = if (self >= 0) == (other >= 0) {
                    self.unsigned_abs().mod_op(other.unsigned_abs())
                } else {
                    self.unsigned_abs().neg_mod(other.unsigned_abs())
                };
                if other >= 0 {
                    $t::exact_from(remainder)
                } else {
                    -$t::exact_from(remainder)
                }
            }
        }

        impl CeilingMod for $t {
            type Output = $t;

            #[inline]
            fn ceiling_mod(self, other: $t) -> $t {
                let remainder = if (self >= 0) == (other >= 0) {
                    self.unsigned_abs().neg_mod(other.unsigned_abs())
                } else {
                    self.unsigned_abs().mod_op(other.unsigned_abs())
                };
                if other >= 0 {
                    -$t::exact_from(remainder)
                } else {
                    $t::exact_from(remainder)
                }
            }
        }

        impl CeilingModAssign for $t {
            #[inline]
            fn ceiling_mod_assign(&mut self, other: $t) {
                *self = self.ceiling_mod(other);
            }
        }

        impl DivRound for $t {
            type Output = $t;

            fn div_round(self, other: $t, rm: RoundingMode) -> $t {
                let result_sign = (self >= 0) == (other >= 0);
                let abs = if result_sign {
                    self.unsigned_abs().div_round(other.unsigned_abs(), rm)
                } else {
                    self.unsigned_abs().div_round(other.unsigned_abs(), -rm)
                };
                if result_sign {
                    $t::exact_from(abs)
                } else {
                    -$t::exact_from(abs)
                }
            }
        }
    };
}

impl_arithmetic_traits!(i8);
impl_arithmetic_traits!(i16);
impl_arithmetic_traits!(i32);
impl_arithmetic_traits!(i64);
impl_arithmetic_traits!(i128);
impl_arithmetic_traits!(isize);
