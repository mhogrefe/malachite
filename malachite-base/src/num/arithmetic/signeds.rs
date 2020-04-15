use comparison::{Max, Min};
use num::arithmetic::traits::{
    CeilingDivAssignMod, CeilingDivMod, CeilingDivNegMod, CeilingMod, CeilingModAssign,
    DivAssignMod, DivMod, DivRound, DivisibleByPowerOfTwo, Mod, ModPowerOfTwo, NegMod, PowerOfTwo,
    SaturatingAbs, SaturatingAbsAssign, SaturatingNeg, SaturatingNegAssign, TrueCheckedShl,
    TrueCheckedShr, UnsignedAbs, WrappingAbs, WrappingAbsAssign,
};
use num::basic::integers::PrimitiveInteger;
use num::basic::signeds::PrimitiveSigned;
use num::conversion::traits::{ExactFrom, WrappingFrom};
use num::logic::traits::LowMask;
use round::RoundingMode;

macro_rules! impl_arithmetic_traits {
    ($t:ident) => {
        impl SaturatingNeg for $t {
            type Output = $t;

            /// Computes `-self`, saturating at the numeric bounds instead of overflowing. For
            /// signed types, that means that this is ordinary negation, except that the negative
            /// of the smallest representable value is the largest representable value.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::SaturatingNeg;
            ///
            /// assert_eq!(0i8.saturating_neg(), 0);
            /// assert_eq!(100i64.saturating_neg(), -100);
            /// assert_eq!((-100i64).saturating_neg(), 100);
            /// assert_eq!((-128i8).saturating_neg(), 127);
            /// ```
            #[inline]
            fn saturating_neg(self) -> $t {
                if self == $t::MIN {
                    $t::MAX
                } else {
                    -self
                }
            }
        }

        #[allow(unstable_name_collisions)]
        impl SaturatingNegAssign for $t {
            /// Replaces `self` with its negative, saturating at the numeric bounds instead of
            /// overflowing.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::SaturatingNegAssign;
            ///
            /// let mut x = 0i8;
            /// x.saturating_neg_assign();
            /// assert_eq!(x, 0);
            ///
            /// let mut x = 100i64;
            /// x.saturating_neg_assign();
            /// assert_eq!(x, -100);
            ///
            /// let mut x = -100i64;
            /// x.saturating_neg_assign();
            /// assert_eq!(x, 100);
            ///
            /// let mut x = -128i8;
            /// x.saturating_neg_assign();
            /// assert_eq!(x, 127);
            /// ```
            #[inline]
            fn saturating_neg_assign(&mut self) {
                *self = self.saturating_neg();
            }
        }

        impl SaturatingAbs for $t {
            type Output = $t;

            /// Computes the absolute value of `self`, saturating at the numeric bounds instead of
            /// overflowing. For signed types, that means that this is ordinary `abs`, except that
            /// the absolute value of the smallest representable value is the largest representable
            /// value.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::SaturatingAbs;
            ///
            /// assert_eq!(0i8.saturating_abs(), 0);
            /// assert_eq!(100i64.saturating_abs(), 100);
            /// assert_eq!((-100i64).saturating_abs(), 100);
            /// assert_eq!((-128i8).saturating_abs(), 127);
            /// ```
            #[inline]
            fn saturating_abs(self) -> $t {
                if self >= 0 {
                    self
                } else if self == $t::MIN {
                    $t::MAX
                } else {
                    -self
                }
            }
        }

        #[allow(unstable_name_collisions)]
        impl SaturatingAbsAssign for $t {
            /// Replace `self` with its absolute value, saturating at the numeric bounds instead of
            /// overflowing.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::SaturatingAbsAssign;
            ///
            /// let mut x = 0i8;
            /// x.saturating_abs_assign();
            /// assert_eq!(x, 0);
            ///
            /// let mut x = 100i64;
            /// x.saturating_abs_assign();
            /// assert_eq!(x, 100);
            ///
            /// let mut x = -100i64;
            /// x.saturating_abs_assign();
            /// assert_eq!(x, 100);
            ///
            /// let mut x = -128i8;
            /// x.saturating_abs_assign();
            /// assert_eq!(x, 127);
            /// ```
            #[inline]
            fn saturating_abs_assign(&mut self) {
                *self = self.saturating_abs();
            }
        }

        impl WrappingAbs for $t {
            type Output = $t;

            #[inline]
            fn wrapping_abs(self) -> $t {
                $t::wrapping_abs(self)
            }
        }

        impl WrappingAbsAssign for $t {
            /// Replaces `self` with its absolute value, wrapping around at the boundary of the
            /// type.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::WrappingAbsAssign;
            ///
            /// let mut x = 0i8;
            /// x.wrapping_abs_assign();
            /// assert_eq!(x, 0);
            ///
            /// let mut x = 100i64;
            /// x.wrapping_abs_assign();
            /// assert_eq!(x, 100);
            ///
            /// let mut x = -100i64;
            /// x.wrapping_abs_assign();
            /// assert_eq!(x, 100);
            ///
            /// let mut x = -128i8;
            /// x.wrapping_abs_assign();
            /// assert_eq!(x, -128);
            /// ```
            #[inline]
            fn wrapping_abs_assign(&mut self) {
                *self = self.wrapping_abs();
            }
        }

        impl DivisibleByPowerOfTwo for $t {
            #[inline]
            fn divisible_by_power_of_two(self, pow: u64) -> bool {
                <$t as PrimitiveSigned>::UnsignedOfEqualWidth::wrapping_from(self)
                    .divisible_by_power_of_two(pow)
            }
        }

        impl ModPowerOfTwo for $t {
            type Output = <$t as PrimitiveSigned>::UnsignedOfEqualWidth;

            #[inline]
            fn mod_power_of_two(self, pow: u64) -> Self::Output {
                if pow > $t::WIDTH && self < 0 {
                    panic!("Result exceeds width of output type");
                }
                let x = Self::Output::wrapping_from(self);
                if x == 0 || pow >= $t::WIDTH {
                    x
                } else {
                    x & Self::Output::low_mask(pow)
                }
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

        impl TrueCheckedShl for $t {
            type Output = $t;

            fn true_checked_shl(self, _rhs: u64) -> Option<$t> {
                unimplemented!();
            }
        }

        impl TrueCheckedShr for $t {
            type Output = $t;

            fn true_checked_shr(self, _rhs: u64) -> Option<$t> {
                unimplemented!();
            }
        }

        impl PowerOfTwo for $t {
            /// Computes 2<sup>`pow`</sup>.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `pow` is greater than or equal to the width of `$t` minus 1.
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::PowerOfTwo;
            ///
            /// assert_eq!(i16::power_of_two(0), 1);
            /// assert_eq!(i8::power_of_two(3), 8);
            /// assert_eq!(i64::power_of_two(40), 1 << 40);
            /// ```
            #[inline]
            fn power_of_two(pow: u64) -> $t {
                assert!(pow < $t::WIDTH - 1);
                1 << pow
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
