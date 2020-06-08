use num::arithmetic::traits::{DivRound, RoundToMultiple, RoundToMultipleAssign, UnsignedAbs};
use num::conversion::traits::ExactFrom;
use rounding_mode::RoundingMode;

macro_rules! impl_round_to_multiple_unsigned {
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
            /// use malachite_base::rounding_mode::RoundingMode;
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
            fn round_to_multiple(self, other: $t, rm: RoundingMode) -> $t {
                match (self, other, rm) {
                    (x, y, _) if x == y => x,
                    (_, 0, RoundingMode::Down)
                    | (_, 0, RoundingMode::Floor)
                    | (_, 0, RoundingMode::Nearest) => 0,
                    (x, 0, rm) => panic!("Cannot round {} to zero using RoundingMode {}", x, rm),
                    (x, y, rm) => x.div_round(y, rm).checked_mul(other).unwrap(),
                }
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
            /// use malachite_base::rounding_mode::RoundingMode;
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
impl_round_to_multiple_unsigned!(u8);
impl_round_to_multiple_unsigned!(u16);
impl_round_to_multiple_unsigned!(u32);
impl_round_to_multiple_unsigned!(u64);
impl_round_to_multiple_unsigned!(u128);
impl_round_to_multiple_unsigned!(usize);

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
            /// use malachite_base::rounding_mode::RoundingMode;
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
            fn round_to_multiple(self, other: $t, rm: RoundingMode) -> $t {
                if self >= 0 {
                    $t::exact_from(
                        self.unsigned_abs()
                            .round_to_multiple(other.unsigned_abs(), rm),
                    )
                } else {
                    let abs_result = self
                        .unsigned_abs()
                        .round_to_multiple(other.unsigned_abs(), -rm);
                    if abs_result == $t::MIN.unsigned_abs() {
                        $t::MIN
                    } else {
                        $t::exact_from(abs_result).checked_neg().unwrap()
                    }
                }
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
            /// use malachite_base::rounding_mode::RoundingMode;
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
impl_round_to_multiple_signed!(i8);
impl_round_to_multiple_signed!(i16);
impl_round_to_multiple_signed!(i32);
impl_round_to_multiple_signed!(i64);
impl_round_to_multiple_signed!(i128);
impl_round_to_multiple_signed!(isize);
