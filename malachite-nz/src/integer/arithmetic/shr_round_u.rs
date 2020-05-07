use malachite_base::num::arithmetic::traits::{ShrRound, ShrRoundAssign};
use malachite_base::num::basic::traits::Zero;
use malachite_base::round::RoundingMode;

use integer::Integer;

macro_rules! impl_integer_shr_round_unsigned {
    ($t:ident) => {
        impl ShrRound<$t> for Integer {
            type Output = Integer;

            /// Shifts a `Integer` right (divides it by a power of 2) and rounds according to the
            /// specified rounding mode, taking the `Integer` by value. Passing
            /// `RoundingMode::Floor` or `RoundingMode::Down` is equivalent to using `>>`. To test
            /// whether `RoundingMode::Exact` can be passed, use
            /// `self.divisible_by_power_of_two(other)`.
            ///
            /// Time: worst case O(`other`)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `rm` is `RoundingMode::Exact` but `self` is not divisible by
            /// 2<sup>`other`</sup>.
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use malachite_base::round::RoundingMode;
            /// use malachite_base::num::arithmetic::traits::ShrRound;
            /// use malachite_nz::integer::Integer;
            ///
            /// assert_eq!(
            ///     Integer::from(0x101).shr_round(8u8, RoundingMode::Down).to_string(),
            ///     "1"
            /// );
            /// assert_eq!(Integer::from(0x101).shr_round(8u16, RoundingMode::Up).to_string(), "2");
            ///
            /// assert_eq!(
            ///     Integer::from(-0x101).shr_round(9u32, RoundingMode::Down).to_string(),
            ///     "0"
            /// );
            /// assert_eq!(
            ///     Integer::from(-0x101).shr_round(9u64, RoundingMode::Up).to_string(),
            ///     "-1"
            /// );
            /// assert_eq!(
            ///     Integer::from(-0x101).shr_round(9u8, RoundingMode::Nearest).to_string(),
            ///     "-1"
            /// );
            /// assert_eq!(
            ///     Integer::from(-0xff).shr_round(9u16, RoundingMode::Nearest).to_string(),
            ///     "0"
            /// );
            /// assert_eq!(
            ///     Integer::from(-0x100).shr_round(9u64, RoundingMode::Nearest).to_string(),
            ///     "0"
            /// );
            ///
            /// assert_eq!(
            ///     Integer::from(0x100u32).shr_round(8u32, RoundingMode::Exact).to_string(),
            ///     "1"
            /// );
            /// ```
            #[inline]
            fn shr_round(mut self, other: $t, rm: RoundingMode) -> Integer {
                self.shr_round_assign(other, rm);
                self
            }
        }

        impl<'a> ShrRound<$t> for &'a Integer {
            type Output = Integer;

            /// Shifts a `Integer` right (divides it by a power of 2) and rounds according to the
            /// specified rounding mode, taking the `Integer` by reference. Passing
            /// `RoundingMode::Floor` or `RoundingMode::Down` is equivalent to using `>>`. To test
            /// whether `RoundingMode::Exact` can be passed, use
            /// `self.divisible_by_power_of_two(other)`.
            ///
            /// Time: worst case O(`other`)
            ///
            /// Additional memory: worst case O(`other`)
            ///
            /// # Panics
            /// Panics if `rm` is `RoundingMode::Exact` but `self` is not divisible by
            /// 2<sup>`other`</sup>.
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use malachite_base::round::RoundingMode;
            /// use malachite_base::num::arithmetic::traits::ShrRound;
            /// use malachite_nz::integer::Integer;
            ///
            /// assert_eq!(
            ///     (&Integer::from(0x101)).shr_round(8u8, RoundingMode::Down).to_string(),
            ///     "1"
            /// );
            /// assert_eq!(
            ///     (&Integer::from(0x101)).shr_round(8u16, RoundingMode::Up).to_string(),
            ///     "2"
            /// );
            ///
            /// assert_eq!(
            ///     (&Integer::from(-0x101)).shr_round(9u32, RoundingMode::Down).to_string(),
            ///     "0"
            /// );
            /// assert_eq!(
            ///     (&Integer::from(-0x101)).shr_round(9u64, RoundingMode::Up).to_string(),
            ///     "-1"
            /// );
            /// assert_eq!(
            ///     (&Integer::from(-0x101)).shr_round(9u8, RoundingMode::Nearest).to_string(),
            ///     "-1"
            /// );
            /// assert_eq!(
            ///     (&Integer::from(-0xff)).shr_round(9u16, RoundingMode::Nearest).to_string(),
            ///     "0"
            /// );
            /// assert_eq!(
            ///     (&Integer::from(-0x100)).shr_round(9u32, RoundingMode::Nearest).to_string(),
            ///     "0"
            /// );
            ///
            /// assert_eq!(
            ///     (&Integer::from(0x100)).shr_round(8u64, RoundingMode::Exact).to_string(),
            ///     "1"
            /// );
            /// ```
            fn shr_round(self, other: $t, rm: RoundingMode) -> Integer {
                match *self {
                    Integer {
                        sign: true,
                        ref abs,
                    } => Integer {
                        sign: true,
                        abs: abs.shr_round(other, rm),
                    },
                    Integer {
                        sign: false,
                        ref abs,
                    } => {
                        let abs_shifted = abs.shr_round(other, -rm);
                        if abs_shifted == 0 {
                            Integer::ZERO
                        } else {
                            Integer {
                                sign: false,
                                abs: abs_shifted,
                            }
                        }
                    }
                }
            }
        }

        impl ShrRoundAssign<$t> for Integer {
            /// Shifts a `Integer` right (divides it by a power of 2) and rounds according to the
            /// specified rounding mode, in place. Passing `RoundingMode::Floor` or
            /// `RoundingMode::Down` is equivalent to using `>>=`. To test whether
            /// `RoundingMode::Exact` can be passed, use `self.divisible_by_power_of_two(other)`.
            ///
            /// Time: worst case O(`other`)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `rm` is `RoundingMode::Exact` but `self` is not divisible by
            /// 2<sup>`other`</sup>.
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use malachite_base::round::RoundingMode;
            /// use malachite_base::num::arithmetic::traits::ShrRoundAssign;
            /// use malachite_nz::integer::Integer;
            ///
            /// let mut n = Integer::from(0x101);
            /// n.shr_round_assign(8u8, RoundingMode::Down);
            /// assert_eq!(n.to_string(), "1");
            ///
            /// let mut n = Integer::from(0x101);
            /// n.shr_round_assign(8u16, RoundingMode::Up);
            /// assert_eq!(n.to_string(), "2");
            ///
            /// let mut n = Integer::from(-0x101);
            /// n.shr_round_assign(9u32, RoundingMode::Down);
            /// assert_eq!(n.to_string(), "0");
            ///
            /// let mut n = Integer::from(-0x101);
            /// n.shr_round_assign(9u64, RoundingMode::Up);
            /// assert_eq!(n.to_string(), "-1");
            ///
            /// let mut n = Integer::from(-0x101);
            /// n.shr_round_assign(9u8, RoundingMode::Nearest);
            /// assert_eq!(n.to_string(), "-1");
            ///
            /// let mut n = Integer::from(-0xff);
            /// n.shr_round_assign(9u16, RoundingMode::Nearest);
            /// assert_eq!(n.to_string(), "0");
            ///
            /// let mut n = Integer::from(-0x100);
            /// n.shr_round_assign(9u32, RoundingMode::Nearest);
            /// assert_eq!(n.to_string(), "0");
            ///
            /// let mut n = Integer::from(0x100);
            /// n.shr_round_assign(8u64, RoundingMode::Exact);
            /// assert_eq!(n.to_string(), "1");
            /// ```
            fn shr_round_assign(&mut self, other: $t, rm: RoundingMode) {
                match *self {
                    Integer {
                        sign: true,
                        ref mut abs,
                    } => {
                        abs.shr_round_assign(other, rm);
                    }
                    Integer {
                        sign: false,
                        ref mut abs,
                    } => {
                        abs.shr_round_assign(other, -rm);
                        if *abs == 0 {
                            self.sign = true;
                        }
                    }
                }
            }
        }
    };
}
impl_integer_shr_round_unsigned!(u8);
impl_integer_shr_round_unsigned!(u16);
impl_integer_shr_round_unsigned!(u32);
impl_integer_shr_round_unsigned!(u64);
impl_integer_shr_round_unsigned!(u128);
impl_integer_shr_round_unsigned!(usize);
