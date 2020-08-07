use comparison::traits::Min;
use num::arithmetic::traits::{
    CheckedAdd, CheckedNeg, OverflowingAdd, Parity, RoundToMultiple, RoundToMultipleAssign,
    UnsignedAbs,
};
use num::basic::traits::Zero;
use num::conversion::traits::ExactFrom;
use num::logic::traits::TrailingZeros;
use rounding_modes::RoundingMode;
use std::cmp::Ordering;
use std::fmt::Display;
use std::ops::{Rem, Shr, Sub};

fn _round_to_multiple_unsigned<T: Copy + Display + Eq + Ord + Parity + TrailingZeros + Zero>(
    x: T,
    other: T,
    rm: RoundingMode,
) -> T
where
    T: CheckedAdd<T, Output = T>
        + OverflowingAdd<T, Output = T>
        + Rem<T, Output = T>
        + Shr<u64, Output = T>
        + Sub<T, Output = T>,
{
    match (x, other) {
        (x, y) if x == y => x,
        (x, y) if y == T::ZERO => match rm {
            RoundingMode::Down | RoundingMode::Floor | RoundingMode::Nearest => T::ZERO,
            _ => panic!("Cannot round {} to zero using RoundingMode {}", x, rm),
        },
        (x, y) => {
            let r = x % y;
            if r == T::ZERO {
                x
            } else {
                let floor = x - r;
                match rm {
                    RoundingMode::Down | RoundingMode::Floor => floor,
                    RoundingMode::Up | RoundingMode::Ceiling => floor.checked_add(y).unwrap(),
                    RoundingMode::Nearest => {
                        match r.cmp(&(y >> 1)) {
                            Ordering::Less => floor,
                            Ordering::Greater => floor.checked_add(y).unwrap(),
                            Ordering::Equal => {
                                if y.odd() {
                                    floor
                                } else {
                                    // The even multiple of y will have more trailing zeros.
                                    let (ceiling, overflow) = floor.overflowing_add(y);
                                    if floor.trailing_zeros() > ceiling.trailing_zeros() {
                                        floor
                                    } else if overflow {
                                        panic!(
                                            "Cannot round {} to {} using RoundingMode {}",
                                            x, y, rm
                                        );
                                    } else {
                                        ceiling
                                    }
                                }
                            }
                        }
                    }
                    RoundingMode::Exact => {
                        panic!("Cannot round {} to {} using RoundingMode {}", x, y, rm)
                    }
                }
            }
        }
    }
}

macro_rules! impl_round_to_multiple_unsigned {
    ($t:ident) => {
        impl RoundToMultiple<$t> for $t {
            type Output = $t;

            /// Rounds `self` to a multiple of `other`, according to a specified rounding mode. The
            /// only rounding modes that are guaranteed to return without a panic are `Down` and
            /// `Floor`.
            ///
            /// The following two expressions are equivalent:
            ///
            /// `x.round_to_multiple(other, RoundingMode::Exact)`
            /// `{ assert!(x.divisible_by(other)); x }`
            ///
            /// but the latter should be used as it is clearer and more efficient.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// - If `rm` is `Exact`, but `self` is not a multiple of `other`.
            /// - If the multiple is outside the representable range.
            /// - If `self` is nonzero, `other` is zero, and `rm` is trying to round away from zero.
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::RoundToMultiple;
            /// use malachite_base::rounding_modes::RoundingMode;
            ///
            /// assert_eq!(5u32.round_to_multiple(0, RoundingMode::Down), 0);
            ///
            /// assert_eq!(10u8.round_to_multiple(4, RoundingMode::Down), 8);
            /// assert_eq!(10u16.round_to_multiple(4, RoundingMode::Up), 12);
            /// assert_eq!(10u32.round_to_multiple(5, RoundingMode::Exact), 10);
            /// assert_eq!(10u64.round_to_multiple(3, RoundingMode::Nearest), 9);
            /// assert_eq!(20u128.round_to_multiple(3, RoundingMode::Nearest), 21);
            /// assert_eq!(10usize.round_to_multiple(4, RoundingMode::Nearest), 8);
            /// assert_eq!(14u8.round_to_multiple(4, RoundingMode::Nearest), 16);
            /// ```
            #[inline]
            fn round_to_multiple(self, other: $t, rm: RoundingMode) -> $t {
                _round_to_multiple_unsigned(self, other, rm)
            }
        }

        impl RoundToMultipleAssign<$t> for $t {
            /// Rounds `self` to a multiple of `other` in place, according to a specified rounding
            /// mode. The only rounding modes that are guaranteed to return without a panic are
            /// `Down` and `Floor`.
            ///
            /// The following two expressions are equivalent:
            ///
            /// `x.round_to_multiple_assign(other, RoundingMode::Exact);`
            /// `assert!(x.divisible_by(other));`
            ///
            /// but the latter should be used as it is clearer and more efficient.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// - If `rm` is `Exact`, but `self` is not a multiple of `other`.
            /// - If the multiple is outside the representable range.
            /// - If `self` is nonzero, `other` is zero, and `rm` is trying to round away from zero.
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::RoundToMultipleAssign;
            /// use malachite_base::rounding_modes::RoundingMode;
            ///
            /// let mut x = 5u32;
            /// x.round_to_multiple_assign(0, RoundingMode::Down);
            /// assert_eq!(x, 0);
            ///
            /// let mut x = 10u8;
            /// x.round_to_multiple_assign(4, RoundingMode::Down);
            /// assert_eq!(x, 8);
            ///
            /// let mut x = 10u16;
            /// x.round_to_multiple_assign(4, RoundingMode::Up);
            /// assert_eq!(x, 12);
            ///
            /// let mut x = 10u32;
            /// x.round_to_multiple_assign(5, RoundingMode::Exact);
            /// assert_eq!(x, 10);
            ///
            /// let mut x = 10u64;
            /// x.round_to_multiple_assign(3, RoundingMode::Nearest);
            /// assert_eq!(x, 9);
            ///
            /// let mut x = 20u128;
            /// x.round_to_multiple_assign(3, RoundingMode::Nearest);
            /// assert_eq!(x, 21);
            ///
            /// let mut x = 10usize;
            /// x.round_to_multiple_assign(4, RoundingMode::Nearest);
            /// assert_eq!(x, 8);
            ///
            /// let mut x = 14u8;
            /// x.round_to_multiple_assign(4, RoundingMode::Nearest);
            /// assert_eq!(x, 16);
            /// ```
            #[inline]
            fn round_to_multiple_assign(&mut self, other: $t, rm: RoundingMode) {
                *self = self.round_to_multiple(other, rm);
            }
        }
    };
}
apply_to_unsigneds!(impl_round_to_multiple_unsigned);

fn _round_to_multiple_signed<U: Eq, S: Copy + Min + Ord + Zero>(
    x: S,
    other: S,
    rm: RoundingMode,
) -> S
where
    U: RoundToMultiple<U, Output = U>,
    S: CheckedNeg<Output = S> + ExactFrom<U> + UnsignedAbs<Output = U>,
{
    if x >= S::ZERO {
        S::exact_from(x.unsigned_abs().round_to_multiple(other.unsigned_abs(), rm))
    } else {
        let abs_result = x
            .unsigned_abs()
            .round_to_multiple(other.unsigned_abs(), -rm);
        if abs_result == S::MIN.unsigned_abs() {
            S::MIN
        } else {
            S::exact_from(abs_result).checked_neg().unwrap()
        }
    }
}

macro_rules! impl_round_to_multiple_signed {
    ($t:ident) => {
        impl RoundToMultiple<$t> for $t {
            type Output = $t;

            /// Rounds `self` to a multiple of `other`, according to a specified rounding mode. The
            /// only rounding mode that is guaranteed to return without a panic is `Down`.
            ///
            /// The following two expressions are equivalent:
            ///
            /// `x.round_to_multiple(other, RoundingMode::Exact)`
            /// `{ assert!(x.divisible_by(other)); x }`
            ///
            /// but the latter should be used as it is clearer and more efficient.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// - If `rm` is `Exact`, but `self` is not a multiple of `other`.
            /// - If the multiple is outside the representable range.
            /// - If `self` is nonzero, `other` is zero, and `rm` is trying to round away from zero.
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::RoundToMultiple;
            /// use malachite_base::rounding_modes::RoundingMode;
            ///
            /// assert_eq!((-5i32).round_to_multiple(0, RoundingMode::Down), 0);
            ///
            /// assert_eq!((-10i8).round_to_multiple(4, RoundingMode::Down), -8);
            /// assert_eq!((-10i16).round_to_multiple(4, RoundingMode::Up), -12);
            /// assert_eq!((-10i32).round_to_multiple(5, RoundingMode::Exact), -10);
            /// assert_eq!((-10i64).round_to_multiple(3, RoundingMode::Nearest), -9);
            /// assert_eq!((-20i128).round_to_multiple(3, RoundingMode::Nearest), -21);
            /// assert_eq!((-10isize).round_to_multiple(4, RoundingMode::Nearest), -8);
            /// assert_eq!((-14i8).round_to_multiple(4, RoundingMode::Nearest), -16);
            ///
            /// assert_eq!((-10i16).round_to_multiple(-4, RoundingMode::Down), -8);
            /// assert_eq!((-10i32).round_to_multiple(-4, RoundingMode::Up), -12);
            /// assert_eq!((-10i64).round_to_multiple(-5, RoundingMode::Exact), -10);
            /// assert_eq!((-10i128).round_to_multiple(-3, RoundingMode::Nearest), -9);
            /// assert_eq!((-20isize).round_to_multiple(-3, RoundingMode::Nearest), -21);
            /// assert_eq!((-10i8).round_to_multiple(-4, RoundingMode::Nearest), -8);
            /// assert_eq!((-14i16).round_to_multiple(-4, RoundingMode::Nearest), -16);
            /// ```
            #[inline]
            fn round_to_multiple(self, other: $t, rm: RoundingMode) -> $t {
                _round_to_multiple_signed(self, other, rm)
            }
        }

        impl RoundToMultipleAssign<$t> for $t {
            /// Rounds `self` to a multiple of `other` in place, according to a specified rounding
            /// mode. The only rounding mode that is guaranteed to return without a panic is `Down`.
            ///
            /// The following two expressions are equivalent:
            ///
            /// `x.round_to_multiple_assign(other, RoundingMode::Exact);`
            /// `assert!(x.divisible_by(other));`
            ///
            /// but the latter should be used as it is clearer and more efficient.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// - If `rm` is `Exact`, but `self` is not a multiple of `other`.
            /// - If the multiple is outside the representable range.
            /// - If `self` is nonzero, `other` is zero, and `rm` is trying to round away from zero.
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::RoundToMultipleAssign;
            /// use malachite_base::rounding_modes::RoundingMode;
            ///
            /// let mut x = -5i32;
            /// x.round_to_multiple_assign(0, RoundingMode::Down);
            /// assert_eq!(x, 0);
            ///
            /// let mut x = -10i8;
            /// x.round_to_multiple_assign(4, RoundingMode::Down);
            /// assert_eq!(x, -8);
            ///
            /// let mut x = -10i16;
            /// x.round_to_multiple_assign(4, RoundingMode::Up);
            /// assert_eq!(x, -12);
            ///
            /// let mut x = -10i32;
            /// x.round_to_multiple_assign(5, RoundingMode::Exact);
            /// assert_eq!(x, -10);
            ///
            /// let mut x = -10i64;
            /// x.round_to_multiple_assign(3, RoundingMode::Nearest);
            /// assert_eq!(x, -9);
            ///
            /// let mut x = -20i128;
            /// x.round_to_multiple_assign(3, RoundingMode::Nearest);
            /// assert_eq!(x, -21);
            ///
            /// let mut x = -10isize;
            /// x.round_to_multiple_assign(4, RoundingMode::Nearest);
            /// assert_eq!(x, -8);
            ///
            /// let mut x = -14i8;
            /// x.round_to_multiple_assign(4, RoundingMode::Nearest);
            /// assert_eq!(x, -16);
            ///
            /// let mut x = -10i16;
            /// x.round_to_multiple_assign(-4, RoundingMode::Down);
            /// assert_eq!(x, -8);
            ///
            /// let mut x = -10i32;
            /// x.round_to_multiple_assign(-4, RoundingMode::Up);
            /// assert_eq!(x, -12);
            ///
            /// let mut x = -10i64;
            /// x.round_to_multiple_assign(-5, RoundingMode::Exact);
            /// assert_eq!(x, -10);
            ///
            /// let mut x = -10i128;
            /// x.round_to_multiple_assign(-3, RoundingMode::Nearest);
            /// assert_eq!(x, -9);
            ///
            /// let mut x = -20isize;
            /// x.round_to_multiple_assign(-3, RoundingMode::Nearest);
            /// assert_eq!(x, -21);
            ///
            /// let mut x = -10i8;
            /// x.round_to_multiple_assign(-4, RoundingMode::Nearest);
            /// assert_eq!(x, -8);
            ///
            /// let mut x = -14i16;
            /// x.round_to_multiple_assign(-4, RoundingMode::Nearest);
            /// assert_eq!(x, -16);
            /// ```
            #[inline]
            fn round_to_multiple_assign(&mut self, other: $t, rm: RoundingMode) {
                *self = self.round_to_multiple(other, rm);
            }
        }
    };
}
apply_to_signeds!(impl_round_to_multiple_signed);
