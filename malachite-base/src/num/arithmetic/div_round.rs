use std::fmt::Display;
use std::ops::{Add, Div, Mul, Shr, Sub};

use num::arithmetic::traits::{DivRound, DivRoundAssign, Parity, UnsignedAbs, WrappingNeg};
use num::basic::traits::{One, Zero};
use num::conversion::traits::{ExactFrom, WrappingFrom};
use rounding_modes::RoundingMode;

fn _div_round_unsigned<T: Copy + Display + Eq + One + Ord + Parity + Zero>(
    x: T,
    other: T,
    rm: RoundingMode,
) -> T
where
    T: Add<T, Output = T>
        + Div<T, Output = T>
        + Mul<T, Output = T>
        + Shr<u64, Output = T>
        + Sub<T, Output = T>,
{
    let quotient = x / other;
    if rm == RoundingMode::Down || rm == RoundingMode::Floor {
        quotient
    } else {
        let remainder = x - quotient * other;
        match rm {
            _ if remainder == T::ZERO => quotient,
            RoundingMode::Up | RoundingMode::Ceiling => quotient + T::ONE,
            RoundingMode::Nearest => {
                let shifted_other = other >> 1;
                if remainder > shifted_other
                    || remainder == shifted_other && other.even() && quotient.odd()
                {
                    quotient + T::ONE
                } else {
                    quotient
                }
            }
            RoundingMode::Exact => {
                panic!("Division is not exact: {} / {}", x, other);
            }
            _ => unreachable!(),
        }
    }
}

macro_rules! impl_div_round_unsigned {
    ($t:ident) => {
        impl DivRound<$t> for $t {
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
            /// use malachite_base::rounding_modes::RoundingMode;
            ///
            /// assert_eq!(10u8.div_round(4, RoundingMode::Down), 2);
            /// assert_eq!(10u16.div_round(4, RoundingMode::Up), 3);
            /// assert_eq!(10u32.div_round(5, RoundingMode::Exact), 2);
            /// assert_eq!(10u64.div_round(3, RoundingMode::Nearest), 3);
            /// assert_eq!(20u128.div_round(3, RoundingMode::Nearest), 7);
            /// assert_eq!(10usize.div_round(4, RoundingMode::Nearest), 2);
            /// assert_eq!(14u8.div_round(4, RoundingMode::Nearest), 4);
            /// ```
            #[inline]
            fn div_round(self, other: $t, rm: RoundingMode) -> $t {
                _div_round_unsigned(self, other, rm)
            }
        }

        impl DivRoundAssign<$t> for $t {
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
            /// use malachite_base::rounding_modes::RoundingMode;
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
apply_to_unsigneds!(impl_div_round_unsigned);

fn _div_round_signed<U, T: Copy + Ord + Zero>(x: T, other: T, rm: RoundingMode) -> T
where
    T: ExactFrom<U> + UnsignedAbs<Output = U> + WrappingFrom<U> + WrappingNeg<Output = T>,
    U: DivRound<U, Output = U>,
{
    if (x >= T::ZERO) == (other >= T::ZERO) {
        T::exact_from(x.unsigned_abs().div_round(other.unsigned_abs(), rm))
    } else {
        // Has to be wrapping so that (self, other) == (T::MIN, 1) works
        T::wrapping_from(x.unsigned_abs().div_round(other.unsigned_abs(), -rm)).wrapping_neg()
    }
}

macro_rules! impl_div_round_signed {
    ($t:ident) => {
        impl DivRound<$t> for $t {
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
            /// use malachite_base::rounding_modes::RoundingMode;
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
                _div_round_signed(self, other, rm)
            }
        }

        impl DivRoundAssign<$t> for $t {
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
            /// use malachite_base::rounding_modes::RoundingMode;
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
apply_to_signeds!(impl_div_round_signed);
