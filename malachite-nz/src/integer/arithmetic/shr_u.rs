use integer::Integer;
use malachite_base::num::{ShrRound, ShrRoundAssign, Zero};
use malachite_base::round::RoundingMode;
use std::ops::{Shr, ShrAssign};

macro_rules! impl_integer_shr_unsigned {
    ($t:ident) => {
        /// Shifts a `Integer` right (divides it by a power of 2 and takes the floor), taking the
        /// `Integer` by value.
        ///
        /// Time: worst case O(`other`)
        ///
        /// Additional memory: worst case O(1)
        ///
        /// # Examples
        /// ```
        /// extern crate malachite_base;
        /// extern crate malachite_nz;
        ///
        /// use malachite_base::num::Zero;
        /// use malachite_nz::integer::Integer;
        ///
        /// fn main() {
        ///     assert_eq!((Integer::ZERO >> 10u8).to_string(), "0");
        ///     assert_eq!((Integer::from(492) >> 2u16).to_string(), "123");
        ///     assert_eq!((-Integer::trillion() >> 10u32).to_string(),
        ///         "-976562500");
        /// }
        /// ```
        impl Shr<$t> for Integer {
            type Output = Integer;

            #[inline]
            fn shr(mut self, other: $t) -> Integer {
                self >>= other;
                self
            }
        }

        /// Shifts a `Integer` right (divides it by a power of 2 and takes the floor), taking the
        /// `Integer` by reference.
        ///
        /// Time: worst case O(`other`)
        ///
        /// Additional memory: worst case O(`other`)
        ///
        /// # Examples
        /// ```
        /// extern crate malachite_base;
        /// extern crate malachite_nz;
        ///
        /// use malachite_base::num::Zero;
        /// use malachite_nz::integer::Integer;
        ///
        /// fn main() {
        ///     assert_eq!((&Integer::ZERO >> 10u8).to_string(), "0");
        ///     assert_eq!((&Integer::from(492) >> 2u16).to_string(), "123");
        ///     assert_eq!((&(-Integer::trillion()) >> 10u64).to_string(), "-976562500");
        /// }
        /// ```
        impl<'a> Shr<$t> for &'a Integer {
            type Output = Integer;

            fn shr(self, other: $t) -> Integer {
                match *self {
                    Integer {
                        sign: true,
                        ref abs,
                    } => Integer {
                        sign: true,
                        abs: abs >> other,
                    },
                    Integer {
                        sign: false,
                        ref abs,
                    } => {
                        let abs_shifted = abs.shr_round(other, RoundingMode::Ceiling);
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

        /// Shifts a `Integer` right (divides it by a power of 2 and takes the floor) in place.
        ///
        /// Time: worst case O(`other`)
        ///
        /// Additional memory: worst case O(1)
        ///
        /// # Examples
        /// ```
        /// use malachite_nz::integer::Integer;
        ///
        /// let mut x = Integer::from(1024);
        /// x >>= 1u8;
        /// x >>= 2u16;
        /// x >>= 3u32;
        /// x >>= 4u64;
        /// assert_eq!(x.to_string(), "1");
        /// ```
        impl ShrAssign<$t> for Integer {
            fn shr_assign(&mut self, other: $t) {
                match *self {
                    Integer {
                        sign: true,
                        ref mut abs,
                    } => {
                        *abs >>= other;
                    }
                    Integer {
                        sign: false,
                        ref mut abs,
                    } => {
                        abs.shr_round_assign(other, RoundingMode::Ceiling);
                        if *abs == 0 {
                            self.sign = true;
                        }
                    }
                }
            }
        }

        /// Shifts a `Integer` right (divides it by a power of 2) and rounds according to the
        /// specified rounding mode, taking the `Integer` by value. Passing `RoundingMode::Floor` or
        /// `RoundingMode::Down` is equivalent to using `>>`. To test whether `RoundingMode::Exact`
        /// can be passed, use `self.is_divisible_by_power_of_two(other)`.
        ///
        /// Time: worst case O(`other`)
        ///
        /// Additional memory: worst case O(1)
        ///
        /// # Panics
        /// Panics if `rm` is `RoundingMode::Exact`
        /// but `self` is not divisible by 2<pow>`other`</pow>.
        ///
        /// # Examples
        /// ```
        /// extern crate malachite_base;
        /// extern crate malachite_nz;
        ///
        /// use malachite_base::round::RoundingMode;
        /// use malachite_base::num::ShrRound;
        /// use malachite_nz::integer::Integer;
        ///
        /// fn main() {
        ///     assert_eq!(Integer::from(0x101).shr_round(8u8, RoundingMode::Down).to_string(),
        ///         "1");
        ///     assert_eq!(Integer::from(0x101).shr_round(8u16, RoundingMode::Up).to_string(), "2");
        ///
        ///     assert_eq!(Integer::from(-0x101).shr_round(9u32, RoundingMode::Down).to_string(),
        ///         "0");
        ///     assert_eq!(Integer::from(-0x101).shr_round(9u64, RoundingMode::Up).to_string(),
        ///         "-1");
        ///     assert_eq!(Integer::from(-0x101).shr_round(9u8, RoundingMode::Nearest).to_string(),
        ///         "-1");
        ///     assert_eq!(Integer::from(-0xff).shr_round(9u16, RoundingMode::Nearest).to_string(),
        ///         "0");
        ///     assert_eq!(Integer::from(-0x100).shr_round(9u64, RoundingMode::Nearest).to_string(),
        ///         "0");
        ///
        ///     assert_eq!(Integer::from(0x100u32).shr_round(8u32, RoundingMode::Exact).to_string(),
        ///         "1");
        /// }
        /// ```
        impl ShrRound<$t> for Integer {
            type Output = Integer;

            #[inline]
            fn shr_round(mut self, other: $t, rm: RoundingMode) -> Integer {
                self.shr_round_assign(other, rm);
                self
            }
        }

        /// Shifts a `Integer` right (divides it by a power of 2) and rounds according to the
        /// specified rounding mode, taking the `Integer` by reference. Passing
        /// `RoundingMode::Floor` or `RoundingMode::Down` is equivalent to using `>>`. To test
        /// whether `RoundingMode::Exact` can be passed, use
        /// `self.is_divisible_by_power_of_two(other)`.
        ///
        /// Time: worst case O(`other`)
        ///
        /// Additional memory: worst case O(`other`)
        ///
        /// # Panics
        /// Panics if `rm` is `RoundingMode::Exact` but `self` is not divisible by
        /// 2<pow>`other`</pow>.
        ///
        /// # Examples
        /// ```
        /// extern crate malachite_base;
        /// extern crate malachite_nz;
        ///
        /// use malachite_base::round::RoundingMode;
        /// use malachite_base::num::ShrRound;
        /// use malachite_nz::integer::Integer;
        ///
        /// fn main() {
        ///     assert_eq!((&Integer::from(0x101)).shr_round(8u8, RoundingMode::Down).to_string(),
        ///         "1");
        ///     assert_eq!((&Integer::from(0x101)).shr_round(8u16, RoundingMode::Up).to_string(),
        ///         "2");
        ///
        ///     assert_eq!((&Integer::from(-0x101)).shr_round(9u32, RoundingMode::Down).to_string(),
        ///         "0");
        ///     assert_eq!((&Integer::from(-0x101)).shr_round(9u64, RoundingMode::Up).to_string(),
        ///         "-1");
        ///     assert_eq!((&Integer::from(-0x101)).shr_round(9u8, RoundingMode::Nearest)
        ///         .to_string(), "-1");
        ///     assert_eq!((&Integer::from(-0xff)).shr_round(9u16, RoundingMode::Nearest)
        ///         .to_string(), "0");
        ///     assert_eq!((&Integer::from(-0x100)).shr_round(9u32, RoundingMode::Nearest)
        ///         .to_string(), "0");
        ///
        ///     assert_eq!((&Integer::from(0x100)).shr_round(8u64, RoundingMode::Exact).to_string(),
        ///         "1");
        /// }
        /// ```
        impl<'a> ShrRound<$t> for &'a Integer {
            type Output = Integer;

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

        /// Shifts a `Integer` right (divides it by a power of 2) and rounds according to the
        /// specified rounding mode, in place. Passing `RoundingMode::Floor` or `RoundingMode::Down`
        /// is equivalent to using `>>=`. To test whether `RoundingMode::Exact` can be passed, use
        /// `self.is_divisible_by_power_of_two(other)`.
        ///
        /// Time: worst case O(`other`)
        ///
        /// Additional memory: worst case O(1)
        ///
        /// # Panics
        /// Panics if `rm` is `RoundingMode::Exact` but `self` is not divisible by
        /// 2<pow>`other`</pow>.
        ///
        /// # Examples
        /// ```
        /// extern crate malachite_base;
        /// extern crate malachite_nz;
        ///
        /// use malachite_base::round::RoundingMode;
        /// use malachite_base::num::ShrRoundAssign;
        /// use malachite_nz::integer::Integer;
        ///
        /// fn main() {
        ///     let mut n = Integer::from(0x101);
        ///     n.shr_round_assign(8u8, RoundingMode::Down);
        ///     assert_eq!(n.to_string(), "1");
        ///
        ///     let mut n = Integer::from(0x101);
        ///     n.shr_round_assign(8u16, RoundingMode::Up);
        ///     assert_eq!(n.to_string(), "2");
        ///
        ///     let mut n = Integer::from(-0x101);
        ///     n.shr_round_assign(9u32, RoundingMode::Down);
        ///     assert_eq!(n.to_string(), "0");
        ///
        ///     let mut n = Integer::from(-0x101);
        ///     n.shr_round_assign(9u64, RoundingMode::Up);
        ///     assert_eq!(n.to_string(), "-1");
        ///
        ///     let mut n = Integer::from(-0x101);
        ///     n.shr_round_assign(9u8, RoundingMode::Nearest);
        ///     assert_eq!(n.to_string(), "-1");
        ///
        ///     let mut n = Integer::from(-0xff);
        ///     n.shr_round_assign(9u16, RoundingMode::Nearest);
        ///     assert_eq!(n.to_string(), "0");
        ///
        ///     let mut n = Integer::from(-0x100);
        ///     n.shr_round_assign(9u32, RoundingMode::Nearest);
        ///     assert_eq!(n.to_string(), "0");
        ///
        ///     let mut n = Integer::from(0x100);
        ///     n.shr_round_assign(8u64, RoundingMode::Exact);
        ///     assert_eq!(n.to_string(), "1");
        /// }
        /// ```
        impl ShrRoundAssign<$t> for Integer {
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
impl_integer_shr_unsigned!(u8);
impl_integer_shr_unsigned!(u16);
impl_integer_shr_unsigned!(u32);
impl_integer_shr_unsigned!(u64);
impl_integer_shr_unsigned!(u128);
