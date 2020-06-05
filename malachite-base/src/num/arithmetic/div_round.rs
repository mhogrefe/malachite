use num::arithmetic::traits::{DivRound, DivRoundAssign, Parity, UnsignedAbs};
use num::conversion::traits::{ExactFrom, WrappingFrom};
use round::RoundingMode;

macro_rules! impl_div_round_unsigned {
    ($t:ident) => {
        impl DivRound for $t {
            type Output = $t;

            /// Divides a value by another value and rounds according to a specified rounding mode.
            /// See the `RoundingMode` documentation for details.
            ///
            /// Time: Worst case O(1)
            ///
            /// Additional memory: Worst case O(1)
            ///
            /// # Panics
            /// Panics if `other` is zero, or if `rm` is `Exact` but `self` is not divisible by
            /// `other`.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::DivRound;
            /// use malachite_base::round::RoundingMode;
            ///
            /// assert_eq!(10u8.div_round(4, RoundingMode::Down), 2);
            /// assert_eq!(10u16.div_round(4, RoundingMode::Up), 3);
            /// assert_eq!(10u32.div_round(5, RoundingMode::Exact), 2);
            /// assert_eq!(10u64.div_round(3, RoundingMode::Nearest), 3);
            /// assert_eq!(20u128.div_round(3, RoundingMode::Nearest), 7);
            /// assert_eq!(10usize.div_round(4, RoundingMode::Nearest), 2);
            /// assert_eq!(14u8.div_round(4, RoundingMode::Nearest), 4);
            /// ```
            fn div_round(self, other: $t, rm: RoundingMode) -> $t {
                let quotient = self / other;
                if rm == RoundingMode::Down || rm == RoundingMode::Floor {
                    quotient
                } else {
                    let remainder = self - quotient * other;
                    match rm {
                        _ if remainder == 0 => quotient,
                        RoundingMode::Up | RoundingMode::Ceiling => quotient + 1,
                        RoundingMode::Nearest => {
                            let shifted_other = other >> 1;
                            if remainder > shifted_other
                                || remainder == shifted_other && other.even() && quotient.odd()
                            {
                                quotient + 1
                            } else {
                                quotient
                            }
                        }
                        RoundingMode::Exact => {
                            panic!("Division is not exact: {} / {}", self, other);
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }

        impl DivRoundAssign for $t {
            /// Divides a value by another value in place and rounds according to a specified
            /// rounding mode. See the `RoundingMode` documentation for details.
            ///
            /// Time: Worst case O(1)
            ///
            /// Additional memory: Worst case O(1)
            ///
            /// # Panics
            /// Panics if `other` is zero, or if `rm` is `Exact` but `self` is not divisible by
            /// `other`.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::DivRoundAssign;
            /// use malachite_base::round::RoundingMode;
            ///
            /// let mut x = 10u8;
            /// x.div_round_assign(4, RoundingMode::Down);
            /// assert_eq!(x, 2);
            ///
            /// let mut x = 10u16;
            /// x.div_round_assign(4, RoundingMode::Up);
            /// assert_eq!(x, 3);
            ///
            /// let mut x = 10u32;
            /// x.div_round_assign(5, RoundingMode::Exact);
            /// assert_eq!(x, 2);
            ///
            /// let mut x = 10u64;
            /// x.div_round_assign(3, RoundingMode::Nearest);
            /// assert_eq!(x, 3);
            ///
            /// let mut x = 20u128;
            /// x.div_round_assign(3, RoundingMode::Nearest);
            /// assert_eq!(x, 7);
            ///
            /// let mut x = 10usize;
            /// x.div_round_assign(4, RoundingMode::Nearest);
            /// assert_eq!(x, 2);
            ///
            /// let mut x = 14u8;
            /// x.div_round_assign(4, RoundingMode::Nearest);
            /// assert_eq!(x, 4);
            /// ```
            #[inline]
            fn div_round_assign(&mut self, other: $t, rm: RoundingMode) {
                *self = self.div_round(other, rm);
            }
        }
    };
}
impl_div_round_unsigned!(u8);
impl_div_round_unsigned!(u16);
impl_div_round_unsigned!(u32);
impl_div_round_unsigned!(u64);
impl_div_round_unsigned!(u128);
impl_div_round_unsigned!(usize);

macro_rules! impl_div_round_signed {
    ($t:ident) => {
        impl DivRound for $t {
            type Output = $t;

            /// Divides a value by another value and rounds according to a specified rounding mode.
            /// See the `RoundingMode` documentation for details.
            ///
            /// Time: Worst case O(1)
            ///
            /// Additional memory: Worst case O(1)
            ///
            /// # Panics
            /// Panics if `other` is zero, if `self` is `$t::MIN` and `other` is `-1`, or if `rm` is
            /// `Exact` but `self` is not divisible by `other`.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::DivRound;
            /// use malachite_base::round::RoundingMode;
            ///
            /// assert_eq!((-10i8).div_round(4, RoundingMode::Down), -2);
            /// assert_eq!((-10i16).div_round(4, RoundingMode::Up), -3);
            /// assert_eq!((-10i32).div_round(5, RoundingMode::Exact), -2);
            /// assert_eq!((-10i64).div_round(3, RoundingMode::Nearest), -3);
            /// assert_eq!((-20i128).div_round(3, RoundingMode::Nearest), -7);
            /// assert_eq!((-10isize).div_round(4, RoundingMode::Nearest), -2);
            /// assert_eq!((-14i8).div_round(4, RoundingMode::Nearest), -4);
            ///
            /// assert_eq!((-10i16).div_round(-4, RoundingMode::Down), 2);
            /// assert_eq!((-10i32).div_round(-4, RoundingMode::Up), 3);
            /// assert_eq!((-10i64).div_round(-5, RoundingMode::Exact), 2);
            /// assert_eq!((-10i128).div_round(-3, RoundingMode::Nearest), 3);
            /// assert_eq!((-20isize).div_round(-3, RoundingMode::Nearest), 7);
            /// assert_eq!((-10i8).div_round(-4, RoundingMode::Nearest), 2);
            /// assert_eq!((-14i16).div_round(-4, RoundingMode::Nearest), 4);
            /// ```
            fn div_round(self, other: $t, rm: RoundingMode) -> $t {
                if (self >= 0) == (other >= 0) {
                    $t::exact_from(self.unsigned_abs().div_round(other.unsigned_abs(), rm))
                } else {
                    // Has to be wrapping so that (self, other) == ($t::MIN, 1) works
                    $t::wrapping_from(self.unsigned_abs().div_round(other.unsigned_abs(), -rm))
                        .wrapping_neg()
                }
            }
        }

        impl DivRoundAssign for $t {
            /// Divides a value by another value in place and rounds according to a specified
            /// rounding mode.
            /// See the `RoundingMode` documentation for details.
            ///
            /// Time: Worst case O(1)
            ///
            /// Additional memory: Worst case O(1)
            ///
            /// # Panics
            /// Panics if `other` is zero, if `self` is `$t::MIN` and `other` is `-1`, or if `rm` is
            /// `Exact` but `self` is not divisible by `other`.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::DivRoundAssign;
            /// use malachite_base::round::RoundingMode;
            ///
            /// let mut x = -10i8;
            /// x.div_round_assign(4, RoundingMode::Down);
            /// assert_eq!(x, -2);
            ///
            /// let mut x = -10i16;
            /// x.div_round_assign(4, RoundingMode::Up);
            /// assert_eq!(x, -3);
            ///
            /// let mut x = -10i32;
            /// x.div_round_assign(5, RoundingMode::Exact);
            /// assert_eq!(x, -2);
            ///
            /// let mut x = -10i64;
            /// x.div_round_assign(3, RoundingMode::Nearest);
            /// assert_eq!(x, -3);
            ///
            /// let mut x = -20i128;
            /// x.div_round_assign(3, RoundingMode::Nearest);
            /// assert_eq!(x, -7);
            ///
            /// let mut x = -10isize;
            /// x.div_round_assign(4, RoundingMode::Nearest);
            /// assert_eq!(x, -2);
            ///
            /// let mut x = -14i8;
            /// x.div_round_assign(4, RoundingMode::Nearest);
            /// assert_eq!(x, -4);
            ///
            /// let mut x = -10i16;
            /// x.div_round_assign(-4, RoundingMode::Down);
            /// assert_eq!(x, 2);
            ///
            /// let mut x = -10i32;
            /// x.div_round_assign(-4, RoundingMode::Up);
            /// assert_eq!(x, 3);
            ///
            /// let mut x = -10i64;
            /// x.div_round_assign(-5, RoundingMode::Exact);
            /// assert_eq!(x, 2);
            ///
            /// let mut x = -10i128;
            /// x.div_round_assign(-3, RoundingMode::Nearest);
            /// assert_eq!(x, 3);
            ///
            /// let mut x = -20isize;
            /// x.div_round_assign(-3, RoundingMode::Nearest);
            /// assert_eq!(x, 7);
            ///
            /// let mut x = -10i8;
            /// x.div_round_assign(-4, RoundingMode::Nearest);
            /// assert_eq!(x, 2);
            ///
            /// let mut x = -14i16;
            /// x.div_round_assign(-4, RoundingMode::Nearest);
            /// assert_eq!(x, 4);
            /// ```
            #[inline]
            fn div_round_assign(&mut self, other: $t, rm: RoundingMode) {
                *self = self.div_round(other, rm);
            }
        }
    };
}
impl_div_round_signed!(i8);
impl_div_round_signed!(i16);
impl_div_round_signed!(i32);
impl_div_round_signed!(i64);
impl_div_round_signed!(i128);
impl_div_round_signed!(isize);
