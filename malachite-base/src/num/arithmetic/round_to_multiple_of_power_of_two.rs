use num::arithmetic::traits::{
    ArithmeticCheckedShl, RoundToMultipleOfPowerOfTwo, RoundToMultipleOfPowerOfTwoAssign, ShrRound,
};
use rounding_modes::RoundingMode;

macro_rules! impl_round_to_multiple_of_power_of_two {
    ($t:ident) => {
        impl RoundToMultipleOfPowerOfTwo<u64> for $t {
            type Output = $t;

            /// Rounds `self` to a multiple of a power of 2, according to a specified rounding mode.
            /// The only rounding mode that is guaranteed to return without a panic is `Down`.
            ///
            /// The following two expressions are equivalent:
            ///
            /// `x.round_to_multiple_of_power_of_two(pow, RoundingMode::Exact)`
            /// `{ assert!(x.divisible_by_power_of_two(pow)); x }`
            ///
            /// but the latter should be used as it is clearer and more efficient.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// - If `rm` is `Exact`, but `self` is not a multiple of the power of two.
            /// - If `rm` is `Floor`, but `self` is negative with a too-large absolute value to
            ///   round to the next lowest multiple.
            /// - If `rm` is `Ceiling`, but `self` is too large to round to the next highest
            ///   multiple.
            /// - If `rm` is `Up`, but `self` has too large an absolute value to round to the next
            ///   multiple with a greater absolute value.
            /// - If `rm` is `Nearest`, but the nearest multiple is outside the representable range.
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::RoundToMultipleOfPowerOfTwo;
            /// use malachite_base::rounding_modes::RoundingMode;
            ///
            /// assert_eq!(10u8.round_to_multiple_of_power_of_two(2, RoundingMode::Floor), 8);
            /// assert_eq!(10u8.round_to_multiple_of_power_of_two(2, RoundingMode::Ceiling), 12);
            /// assert_eq!(10u8.round_to_multiple_of_power_of_two(2, RoundingMode::Down), 8);
            /// assert_eq!(10u8.round_to_multiple_of_power_of_two(2, RoundingMode::Up), 12);
            /// assert_eq!(10u8.round_to_multiple_of_power_of_two(2, RoundingMode::Nearest), 8);
            /// assert_eq!(12u8.round_to_multiple_of_power_of_two(2, RoundingMode::Exact), 12);
            /// assert_eq!((-10i8).round_to_multiple_of_power_of_two(2, RoundingMode::Floor), -12);
            /// assert_eq!((-10i8).round_to_multiple_of_power_of_two(2, RoundingMode::Ceiling), -8);
            /// assert_eq!((-10i8).round_to_multiple_of_power_of_two(2, RoundingMode::Down), -8);
            /// assert_eq!((-10i8).round_to_multiple_of_power_of_two(2, RoundingMode::Up), -12);
            /// assert_eq!((-10i8).round_to_multiple_of_power_of_two(2, RoundingMode::Nearest), -8);
            /// assert_eq!((-12i8).round_to_multiple_of_power_of_two(2, RoundingMode::Exact), -12);
            /// ```
            #[inline]
            fn round_to_multiple_of_power_of_two(self, pow: u64, rm: RoundingMode) -> $t {
                self.shr_round(pow, rm).arithmetic_checked_shl(pow).unwrap()
            }
        }

        impl RoundToMultipleOfPowerOfTwoAssign<u64> for $t {
            /// Rounds `self` to a multiple of a power of 2 in place, according to a specified
            /// rounding mode. The only rounding mode that is guaranteed to return without a panic
            /// is `Down`.
            ///
            /// The following two expressions are equivalent:
            ///
            /// `x.round_to_multiple_of_power_of_two_assign(pow, RoundingMode::Exact);`
            /// `assert!(x.divisible_by_power_of_two(pow));`
            ///
            /// but the latter should be used as it is clearer and more efficient.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// - If `rm` is `Exact`, but `self` is not a multiple of the power of two.
            /// - If `rm` is `Floor`, but `self` is negative with a too-large absolute value to
            ///   round to the next lowest multiple.
            /// - If `rm` is `Ceiling`, but `self` is too large to round to the next highest
            ///   multiple.
            /// - If `rm` is `Up`, but `self` has too large an absolute value to round to the next
            ///   multiple with a greater absolute value.
            /// - If `rm` is `Nearest`, but the nearest multiple is outside the representable range.
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::RoundToMultipleOfPowerOfTwoAssign;
            /// use malachite_base::rounding_modes::RoundingMode;
            ///
            /// let mut x = 10u8;
            /// x.round_to_multiple_of_power_of_two_assign(2, RoundingMode::Floor);
            /// assert_eq!(x, 8);
            ///
            /// let mut x = 10u8;
            /// x.round_to_multiple_of_power_of_two_assign(2, RoundingMode::Ceiling);
            /// assert_eq!(x, 12);
            ///
            /// let mut x = 10u8;
            /// x.round_to_multiple_of_power_of_two_assign(2, RoundingMode::Down);
            /// assert_eq!(x, 8);
            ///
            /// let mut x = 10u8;
            /// x.round_to_multiple_of_power_of_two_assign(2, RoundingMode::Up);
            /// assert_eq!(x, 12);
            ///
            /// let mut x = 10u8;
            /// x.round_to_multiple_of_power_of_two_assign(2, RoundingMode::Nearest);
            /// assert_eq!(x, 8);
            ///
            /// let mut x = 12u8;
            /// x.round_to_multiple_of_power_of_two_assign(2, RoundingMode::Exact);
            /// assert_eq!(x, 12);
            ///
            /// let mut x = -10i8;
            /// x.round_to_multiple_of_power_of_two_assign(2, RoundingMode::Floor);
            /// assert_eq!(x, -12);
            ///
            /// let mut x = -10i8;
            /// x.round_to_multiple_of_power_of_two_assign(2, RoundingMode::Ceiling);
            /// assert_eq!(x, -8);
            ///
            /// let mut x = -10i8;
            /// x.round_to_multiple_of_power_of_two_assign(2, RoundingMode::Down);
            /// assert_eq!(x, -8);
            ///
            /// let mut x = -10i8;
            /// x.round_to_multiple_of_power_of_two_assign(2, RoundingMode::Up);
            /// assert_eq!(x, -12);
            ///
            /// let mut x = -10i8;
            /// x.round_to_multiple_of_power_of_two_assign(2, RoundingMode::Nearest);
            /// assert_eq!(x, -8);
            ///
            /// let mut x = -12i8;
            /// x.round_to_multiple_of_power_of_two_assign(2, RoundingMode::Exact);
            /// assert_eq!(x, -12);
            /// ```
            #[inline]
            fn round_to_multiple_of_power_of_two_assign(&mut self, pow: u64, rm: RoundingMode) {
                *self = self.round_to_multiple_of_power_of_two(pow, rm);
            }
        }
    };
}

impl_round_to_multiple_of_power_of_two!(u8);
impl_round_to_multiple_of_power_of_two!(u16);
impl_round_to_multiple_of_power_of_two!(u32);
impl_round_to_multiple_of_power_of_two!(u64);
impl_round_to_multiple_of_power_of_two!(u128);
impl_round_to_multiple_of_power_of_two!(usize);
impl_round_to_multiple_of_power_of_two!(i8);
impl_round_to_multiple_of_power_of_two!(i16);
impl_round_to_multiple_of_power_of_two!(i32);
impl_round_to_multiple_of_power_of_two!(i64);
impl_round_to_multiple_of_power_of_two!(i128);
impl_round_to_multiple_of_power_of_two!(isize);
