use std::ops::{Shr, ShrAssign};

use malachite_base::num::arithmetic::traits::{ShrRound, ShrRoundAssign, UnsignedAbs};
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::round::RoundingMode;

use integer::Integer;
use platform::Limb;

macro_rules! impl_integer_shr_signed {
    ($t:ident) => {
        /// Shifts an `Integer` right (divides it by a power of 2 and takes the floor or multiplies
        /// it by a power of 2), taking the `Integer` by value.
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
        /// use malachite_base::num::basic::traits::Zero;
        /// use malachite_nz::integer::Integer;
        ///
        /// fn main() {
        ///     assert_eq!((Integer::ZERO >> 10i8).to_string(), "0");
        ///     assert_eq!((Integer::from(492) >> 2i16).to_string(), "123");
        ///     assert_eq!((-Integer::trillion() >> 10i64).to_string(), "-976562500");
        ///     assert_eq!((Integer::ZERO >> -10i8).to_string(), "0");
        ///     assert_eq!((Integer::from(123) >> -2i16).to_string(), "492");
        ///     assert_eq!((Integer::from(123) >> -100i32).to_string(),
        ///         "155921023828072216384094494261248");
        ///     assert_eq!((Integer::from(-123) >> -2i64).to_string(), "-492");
        ///     assert_eq!((Integer::from(-123) >> -100i8).to_string(),
        ///         "-155921023828072216384094494261248");
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

        /// Shifts an `Integer` right (divides it by a power of 2 and takes the floor or multiplies
        /// it by a power of 2), taking the `Integer` by reference.
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
        /// use malachite_base::num::basic::traits::Zero;
        /// use malachite_nz::integer::Integer;
        ///
        /// fn main() {
        ///     assert_eq!((&Integer::ZERO >> 10i8).to_string(), "0");
        ///     assert_eq!((&Integer::from(492) >> 2i16).to_string(), "123");
        ///     assert_eq!((&(-Integer::trillion()) >> 10i32).to_string(), "-976562500");
        ///     assert_eq!((&Integer::ZERO >> -10i64).to_string(), "0");
        ///     assert_eq!((&Integer::from(123) >> -2i8).to_string(), "492");
        ///     assert_eq!((&Integer::from(123) >> -100i16).to_string(),
        ///         "155921023828072216384094494261248");
        ///     assert_eq!((&Integer::from(-123) >> -2i32).to_string(), "-492");
        ///     assert_eq!((&Integer::from(-123) >> -100i64).to_string(),
        ///         "-155921023828072216384094494261248");
        /// }
        /// ```
        impl<'a> Shr<$t> for &'a Integer {
            type Output = Integer;

            fn shr(self, other: $t) -> Integer {
                if other >= 0 {
                    self >> Limb::wrapping_from(other)
                } else {
                    self << other.unsigned_abs()
                }
            }
        }

        /// Shifts an `Integer` right (divides it by a power of 2 and takes the floor or multiplies
        /// it by a power of 2) in place.
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
        /// use malachite_base::num::basic::traits::{NegativeOne, One};
        /// use malachite_nz::integer::Integer;
        ///
        /// fn main() {
        ///     let mut x = Integer::from(1024);
        ///     x >>= 1i8;
        ///     x >>= 2i16;
        ///     x >>= 3i32;
        ///     x >>= 4i64;
        ///     assert_eq!(x.to_string(), "1");
        ///
        ///     let mut x = Integer::ONE;
        ///     x >>= -1i8;
        ///     x >>= -2i16;
        ///     x >>= -3i32;
        ///     x >>= -4i64;
        ///     assert_eq!(x.to_string(), "1024");
        ///     let mut x = Integer::NEGATIVE_ONE;
        ///     x >>= -1i8;
        ///     x >>= -2i16;
        ///     x >>= -3i32;
        ///     x >>= -4i64;
        ///     assert_eq!(x.to_string(), "-1024");
        /// }
        /// ```
        impl ShrAssign<$t> for Integer {
            fn shr_assign(&mut self, other: $t) {
                if other >= 0 {
                    *self >>= Limb::wrapping_from(other);
                } else {
                    *self <<= other.unsigned_abs();
                }
            }
        }

        /// Shifts an `Integer` right (divides it by a power of 2 and takes the floor or multiplies
        /// it by a power of 2) and rounds according to the specified rounding mode, taking the
        /// `Integer` by value. Passing `RoundingMode::Floor` or `RoundingMode::Down` is equivalent
        /// to using `>>`. To test whether `RoundingMode::Exact` can be passed, use
        /// `other < 0 || self.is_divisible_by_power_of_two(other)`.
        ///
        /// Time: worst case O(`other`)
        ///
        /// Additional memory: worst case O(`other`)
        ///
        /// # Panics
        /// Panics if `other` is positive and `rm` is `RoundingMode::Exact` but `self` is not
        /// divisible by 2<sup>`other`</sup>.
        ///
        /// # Examples
        /// ```
        /// extern crate malachite_base;
        /// extern crate malachite_nz;
        ///
        /// use malachite_base::round::RoundingMode;
        /// use malachite_base::num::arithmetic::traits::ShrRound;
        /// use malachite_base::num::basic::traits::Zero;
        /// use malachite_nz::integer::Integer;
        ///
        /// fn main() {
        ///     assert_eq!(Integer::from(0x101u32).shr_round(8i8, RoundingMode::Down).to_string(),
        ///         "1");
        ///     assert_eq!(Integer::from(0x101u32).shr_round(8i16, RoundingMode::Up).to_string(),
        ///         "2");
        ///
        ///     assert_eq!(Integer::from(-0x101).shr_round(9i32, RoundingMode::Down).to_string(),
        ///         "0");
        ///     assert_eq!(Integer::from(-0x101).shr_round(9i64, RoundingMode::Up).to_string(),
        ///         "-1");
        ///     assert_eq!(Integer::from(-0x101).shr_round(9i8, RoundingMode::Nearest).to_string(),
        ///         "-1");
        ///     assert_eq!(Integer::from(-0xff).shr_round(9i16, RoundingMode::Nearest).to_string(),
        ///         "0");
        ///     assert_eq!(Integer::from(-0x100).shr_round(9i32, RoundingMode::Nearest).to_string(),
        ///         "0");
        ///
        ///     assert_eq!(Integer::from(0x100u32).shr_round(8i64, RoundingMode::Exact).to_string(),
        ///         "1");
        ///
        ///     assert_eq!(Integer::ZERO.shr_round(-10i8, RoundingMode::Exact).to_string(), "0");
        ///     assert_eq!(Integer::from(123u32).shr_round(-2i16, RoundingMode::Exact).to_string(),
        ///         "492");
        ///     assert_eq!(Integer::from(123u32).shr_round(-100i32, RoundingMode::Exact)
        ///         .to_string(), "155921023828072216384094494261248");
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

        /// Shifts an `Integer` right (divides it by a power of 2 and takes the floor or multiplies
        /// it by a power of 2) and rounds according to the specified rounding mode, taking the
        /// `Integer` by reference. Passing `RoundingMode::Floor` or `RoundingMode::Down` is
        /// equivalent to using `>>`. To test whether `RoundingMode::Exact` can be passed, use
        /// `other < 0 || self.is_divisible_by_power_of_two(other)`.
        ///
        /// Time: worst case O(`other`)
        ///
        /// Additional memory: worst case O(`other`)
        ///
        /// # Panics
        /// Panics if `other` is positive and `rm` is `RoundingMode::Exact` but `self` is not
        /// divisible by 2<sup>`other`</sup>.
        ///
        /// # Examples
        /// ```
        /// extern crate malachite_base;
        /// extern crate malachite_nz;
        ///
        /// use malachite_base::round::RoundingMode;
        /// use malachite_base::num::arithmetic::traits::ShrRound;
        /// use malachite_base::num::basic::traits::Zero;
        /// use malachite_nz::integer::Integer;
        ///
        /// fn main() {
        ///     assert_eq!((&Integer::from(0x101u32)).shr_round(8i8, RoundingMode::Down)
        ///         .to_string(), "1");
        ///     assert_eq!((&Integer::from(0x101u32)).shr_round(8i16, RoundingMode::Up).to_string(),
        ///         "2");
        ///
        ///     assert_eq!((&Integer::from(-0x101)).shr_round(9i32, RoundingMode::Down).to_string(),
        ///         "0");
        ///     assert_eq!((&Integer::from(-0x101)).shr_round(9i64, RoundingMode::Up).to_string(),
        ///         "-1");
        ///     assert_eq!((&Integer::from(-0x101)).shr_round(9i8, RoundingMode::Nearest)
        ///         .to_string(), "-1");
        ///     assert_eq!((&Integer::from(-0xff)).shr_round(9i16, RoundingMode::Nearest)
        ///         .to_string(), "0");
        ///     assert_eq!((&Integer::from(-0x100)).shr_round(9i32, RoundingMode::Nearest)
        ///         .to_string(), "0");
        ///
        ///     assert_eq!((&Integer::from(0x100u32)).shr_round(8i64, RoundingMode::Exact)
        ///         .to_string(), "1");
        ///
        ///     assert_eq!((&Integer::ZERO).shr_round(-10i8, RoundingMode::Exact).to_string(), "0");
        ///     assert_eq!((&Integer::from(123u32)).shr_round(-2i16, RoundingMode::Exact)
        ///         .to_string(), "492");
        ///     assert_eq!((&Integer::from(123u32)).shr_round(-100i32, RoundingMode::Exact)
        ///         .to_string(), "155921023828072216384094494261248");
        /// }
        /// ```
        impl<'a> ShrRound<$t> for &'a Integer {
            type Output = Integer;

            fn shr_round(self, other: $t, rm: RoundingMode) -> Integer {
                if other >= 0 {
                    self.shr_round(Limb::wrapping_from(other), rm)
                } else {
                    self << other.unsigned_abs()
                }
            }
        }

        /// Shifts an `Integer` right (divides it by a power of 2 and takes the floor or multiplies
        /// it by a power of 2) and rounds according to the specified rounding mode, in place.
        /// Passing `RoundingMode::Floor` or `RoundingMode::Down` is equivalent to using `>>=`. To
        /// test whether `RoundingMode::Exact` can be passed, use
        /// `other < 0 || self.is_divisible_by_power_of_two(other)`.
        ///
        /// Time: worst case O(`other`)
        ///
        /// Additional memory: worst case O(`other`)
        ///
        /// # Panics
        /// Panics if `other` is positive and `rm` is `RoundingMode::Exact` but `self` is not
        /// divisible by 2<sup>`other`</sup>.
        ///
        /// # Examples
        /// ```
        /// extern crate malachite_base;
        /// extern crate malachite_nz;
        ///
        /// use malachite_base::round::RoundingMode;
        /// use malachite_base::num::arithmetic::traits::ShrRoundAssign;
        /// use malachite_base::num::basic::traits::One;
        /// use malachite_nz::integer::Integer;
        ///
        /// fn main() {
        ///     let mut n = Integer::from(0x101u32);
        ///     n.shr_round_assign(8i8, RoundingMode::Down);
        ///     assert_eq!(n.to_string(), "1");
        ///
        ///     let mut n = Integer::from(0x101u32);
        ///     n.shr_round_assign(8i16, RoundingMode::Up);
        ///     assert_eq!(n.to_string(), "2");
        ///
        ///     let mut n = Integer::from(-0x101);
        ///     n.shr_round_assign(9i32, RoundingMode::Down);
        ///     assert_eq!(n.to_string(), "0");
        ///
        ///     let mut n = Integer::from(-0x101);
        ///     n.shr_round_assign(9i64, RoundingMode::Up);
        ///     assert_eq!(n.to_string(), "-1");
        ///
        ///     let mut n = Integer::from(-0x101);
        ///     n.shr_round_assign(9i8, RoundingMode::Nearest);
        ///     assert_eq!(n.to_string(), "-1");
        ///
        ///     let mut n = Integer::from(-0xff);
        ///     n.shr_round_assign(9i16, RoundingMode::Nearest);
        ///     assert_eq!(n.to_string(), "0");
        ///
        ///     let mut n = Integer::from(-0x100);
        ///     n.shr_round_assign(9i32, RoundingMode::Nearest);
        ///     assert_eq!(n.to_string(), "0");
        ///
        ///     let mut n = Integer::from(0x100u32);
        ///     n.shr_round_assign(8i64, RoundingMode::Exact);
        ///     assert_eq!(n.to_string(), "1");
        ///
        ///     let mut x = Integer::ONE;
        ///     x.shr_round_assign(-1i8, RoundingMode::Exact);
        ///     x.shr_round_assign(-2i16, RoundingMode::Exact);
        ///     x.shr_round_assign(-3i32, RoundingMode::Exact);
        ///     x.shr_round_assign(-4i64, RoundingMode::Exact);
        ///     assert_eq!(x.to_string(), "1024");
        /// }
        /// ```
        impl ShrRoundAssign<$t> for Integer {
            fn shr_round_assign(&mut self, other: $t, rm: RoundingMode) {
                if other >= 0 {
                    self.shr_round_assign(Limb::wrapping_from(other), rm);
                } else {
                    *self <<= other.unsigned_abs();
                }
            }
        }
    };
}
impl_integer_shr_signed!(i8);
impl_integer_shr_signed!(i16);
impl_integer_shr_signed!(i32);
impl_integer_shr_signed!(i64);
impl_integer_shr_signed!(i128);
impl_integer_shr_signed!(isize);
