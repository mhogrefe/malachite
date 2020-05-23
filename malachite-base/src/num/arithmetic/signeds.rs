use num::arithmetic::traits::{
    CeilingDivAssignMod, CeilingDivMod, CeilingDivNegMod, CeilingMod, CeilingModAssign,
    DivAssignMod, DivMod, DivRound, DivisibleByPowerOfTwo, Mod, NegMod, UnsignedAbs,
};
use num::basic::signeds::PrimitiveSigned;
use num::conversion::traits::{ExactFrom, WrappingFrom};
use round::RoundingMode;

macro_rules! impl_arithmetic_traits {
    ($t:ident) => {
        impl DivisibleByPowerOfTwo for $t {
            #[inline]
            fn divisible_by_power_of_two(self, pow: u64) -> bool {
                <$t as PrimitiveSigned>::UnsignedOfEqualWidth::wrapping_from(self)
                    .divisible_by_power_of_two(pow)
            }
        }

        impl DivMod for $t {
            type DivOutput = $t;
            type ModOutput = $t;

            #[inline]
            fn div_mod(self, other: $t) -> ($t, $t) {
                let (quotient, remainder) = if (self >= 0) == (other >= 0) {
                    let (quotient, remainder) = self.unsigned_abs().div_mod(other.unsigned_abs());
                    ($t::exact_from(quotient), remainder)
                } else {
                    let (quotient, remainder) = self
                        .unsigned_abs()
                        .ceiling_div_neg_mod(other.unsigned_abs());
                    (-$t::exact_from(quotient), remainder)
                };
                (
                    quotient,
                    if other >= 0 {
                        $t::exact_from(remainder)
                    } else {
                        -$t::exact_from(remainder)
                    },
                )
            }
        }

        impl DivAssignMod for $t {
            type ModOutput = $t;

            #[inline]
            fn div_assign_mod(&mut self, rhs: $t) -> $t {
                let (q, r) = self.div_mod(rhs);
                *self = q;
                r
            }
        }

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

        impl CeilingDivMod for $t {
            type DivOutput = $t;
            type ModOutput = $t;

            #[inline]
            fn ceiling_div_mod(self, other: $t) -> ($t, $t) {
                let (quotient, remainder) = if (self >= 0) == (other >= 0) {
                    let (quotient, remainder) = self
                        .unsigned_abs()
                        .ceiling_div_neg_mod(other.unsigned_abs());
                    ($t::exact_from(quotient), remainder)
                } else {
                    let (quotient, remainder) = self.unsigned_abs().div_mod(other.unsigned_abs());
                    (-$t::exact_from(quotient), remainder)
                };
                (
                    quotient,
                    if other >= 0 {
                        -$t::exact_from(remainder)
                    } else {
                        $t::exact_from(remainder)
                    },
                )
            }
        }

        impl CeilingDivAssignMod for $t {
            type ModOutput = $t;

            #[inline]
            fn ceiling_div_assign_mod(&mut self, rhs: $t) -> $t {
                let (q, r) = self.ceiling_div_mod(rhs);
                *self = q;
                r
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
            fn ceiling_mod_assign(&mut self, rhs: $t) {
                *self = self.ceiling_mod(rhs);
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
