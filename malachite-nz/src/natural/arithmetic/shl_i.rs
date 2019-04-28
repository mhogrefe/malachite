use std::ops::{Shl, ShlAssign};

use malachite_base::num::signeds::PrimitiveSigned;
use malachite_base::num::traits::{
    ShlRound, ShlRoundAssign, ShrRound, ShrRoundAssign, UnsignedAbs,
};
use malachite_base::round::RoundingMode;

use natural::Natural;

macro_rules! impl_natural_shl_signed {
    ($t:ident) => {
        /// Shifts a `Natural` left (multiplies it by a power of 2 or divides it by a power of 2 and
        /// takes the floor), taking the `Natural` by value.
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
        /// use malachite_base::num::traits::Zero;
        /// use malachite_nz::natural::Natural;
        ///
        /// fn main() {
        ///     assert_eq!((Natural::ZERO << 10i8).to_string(), "0");
        ///     assert_eq!((Natural::from(123u32) << 2i16).to_string(), "492");
        ///     assert_eq!((Natural::from(123u32) << 100i32).to_string(),
        ///         "155921023828072216384094494261248");
        ///     assert_eq!((Natural::ZERO << -10i64).to_string(), "0");
        ///     assert_eq!((Natural::from(492u32) << -2i8).to_string(), "123");
        ///     assert_eq!((Natural::trillion() << -10i16).to_string(), "976562500");
        /// }
        /// ```
        impl Shl<$t> for Natural {
            type Output = Natural;

            #[inline]
            fn shl(mut self, other: $t) -> Natural {
                self <<= other;
                self
            }
        }

        /// Shifts a `Natural` left (multiplies it by a power of 2 or divides it by a power of 2 and
        /// takes the floor), taking the `Natural` by reference.
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
        /// use malachite_base::num::traits::Zero;
        /// use malachite_nz::natural::Natural;
        ///
        /// fn main() {
        ///     assert_eq!((&Natural::ZERO << 10i8).to_string(), "0");
        ///     assert_eq!((&Natural::from(123u32) << 2i16).to_string(), "492");
        ///     assert_eq!((&Natural::from(123u32) << 100i32).to_string(),
        ///         "155921023828072216384094494261248");
        ///     assert_eq!((&Natural::ZERO << -10i64).to_string(), "0");
        ///     assert_eq!((&Natural::from(492u32) << -2i8).to_string(), "123");
        ///     assert_eq!((&Natural::trillion() << -10i16).to_string(), "976562500");
        /// }
        /// ```
        impl<'a> Shl<$t> for &'a Natural {
            type Output = Natural;

            fn shl(self, other: $t) -> Natural {
                if other >= 0 {
                    self << other.to_unsigned_bitwise()
                } else {
                    self >> other.unsigned_abs()
                }
            }
        }

        /// Shifts a `Natural` left (multiplies it by a power of 2 or divides it by a power of 2 and
        /// takes the floor) in place.
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
        /// use malachite_base::num::traits::One;
        /// use malachite_nz::natural::Natural;
        ///
        /// fn main() {
        ///     let mut x = Natural::ONE;
        ///     x <<= 1i8;
        ///     x <<= 2i16;
        ///     x <<= 3i32;
        ///     x <<= 4i64;
        ///     assert_eq!(x.to_string(), "1024");
        ///
        ///     let mut x = Natural::from(1024u32);
        ///     x <<= -1i8;
        ///     x <<= -2i16;
        ///     x <<= -3i32;
        ///     x <<= -4i64;
        ///     assert_eq!(x.to_string(), "1");
        /// }
        /// ```
        impl ShlAssign<$t> for Natural {
            fn shl_assign(&mut self, other: $t) {
                if other >= 0 {
                    *self <<= other.to_unsigned_bitwise();
                } else {
                    *self >>= other.unsigned_abs();
                }
            }
        }

        /// Shifts a `Natural` left (multiplies it by a power of 2 or divides it by a power of 2 and
        /// takes the floor) and rounds according to the specified rounding mode, taking the
        /// `Natural` by value. Passing `RoundingMode::Floor` or `RoundingMode::Down` is equivalent
        /// to using `>>`. To test whether `RoundingMode::Exact` can be passed, use
        /// `other > 0 || self.is_divisible_by_power_of_two(other)`.
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
        /// use malachite_base::num::traits::{ShlRound, Zero};
        /// use malachite_nz::natural::Natural;
        ///
        /// fn main() {
        ///     assert_eq!(Natural::from(0x101u32).shl_round(-8i8, RoundingMode::Down).to_string(),
        ///         "1");
        ///     assert_eq!(Natural::from(0x101u32).shl_round(-8i16, RoundingMode::Up).to_string(),
        ///         "2");
        ///
        ///     assert_eq!(Natural::from(0x101u32).shl_round(-9i32, RoundingMode::Down).to_string(),
        ///         "0");
        ///     assert_eq!(Natural::from(0x101u32).shl_round(-9i64, RoundingMode::Up).to_string(),
        ///         "1");
        ///     assert_eq!(Natural::from(0x101u32).shl_round(-9i8, RoundingMode::Nearest)
        ///         .to_string(), "1");
        ///     assert_eq!(Natural::from(0xffu32).shl_round(-9i16, RoundingMode::Nearest)
        ///         .to_string(), "0");
        ///     assert_eq!(Natural::from(0x100u32).shl_round(-9i32, RoundingMode::Nearest)
        ///         .to_string(), "0");
        ///
        ///     assert_eq!(Natural::from(0x100u32).shl_round(-8i64, RoundingMode::Exact)
        ///         .to_string(), "1");
        ///
        ///     assert_eq!(Natural::ZERO.shl_round(10i8, RoundingMode::Exact).to_string(), "0");
        ///     assert_eq!(Natural::from(123u32).shl_round(2i16, RoundingMode::Exact).to_string(),
        ///         "492");
        ///     assert_eq!(Natural::from(123u32).shl_round(100i32, RoundingMode::Exact).to_string(),
        ///         "155921023828072216384094494261248");
        /// }
        /// ```
        impl ShlRound<$t> for Natural {
            type Output = Natural;

            #[inline]
            fn shl_round(mut self, other: $t, rm: RoundingMode) -> Natural {
                self.shl_round_assign(other, rm);
                self
            }
        }

        /// Shifts a `Natural` left (multiplies it by a power of 2 or divides it by a power of 2 and
        /// takes the floor) and rounds according to the specified rounding mode, taking the
        /// `Natural` by reference. Passing `RoundingMode::Floor` or `RoundingMode::Down` is
        /// equivalent to using `>>`. To test whether `RoundingMode::Exact` can be passed, use
        /// `other > 0 || self.is_divisible_by_power_of_two(other)`.
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
        /// use malachite_base::num::traits::{ShlRound, Zero};
        /// use malachite_nz::natural::Natural;
        ///
        /// fn main() {
        ///     assert_eq!((&Natural::from(0x101u32)).shl_round(-8i8, RoundingMode::Down)
        ///         .to_string(), "1");
        ///     assert_eq!((&Natural::from(0x101u32)).shl_round(-8i16, RoundingMode::Up)
        ///         .to_string(), "2");
        ///
        ///     assert_eq!((&Natural::from(0x101u32)).shl_round(-9i32, RoundingMode::Down)
        ///         .to_string(), "0");
        ///     assert_eq!((&Natural::from(0x101u32)).shl_round(-9i64, RoundingMode::Up)
        ///         .to_string(), "1");
        ///     assert_eq!((&Natural::from(0x101u32)).shl_round(-9i8, RoundingMode::Nearest)
        ///         .to_string(), "1");
        ///     assert_eq!((&Natural::from(0xffu32)).shl_round(-9i16, RoundingMode::Nearest)
        ///         .to_string(), "0");
        ///     assert_eq!((&Natural::from(0x100u32)).shl_round(-9i32, RoundingMode::Nearest)
        ///         .to_string(), "0");
        ///
        ///     assert_eq!((&Natural::from(0x100u32)).shl_round(-8i64, RoundingMode::Exact)
        ///         .to_string(), "1");
        ///
        ///     assert_eq!((&Natural::ZERO).shl_round(10i8, RoundingMode::Exact).to_string(), "0");
        ///     assert_eq!((&Natural::from(123u32)).shl_round(2i16, RoundingMode::Exact)
        ///         .to_string(), "492");
        ///     assert_eq!((&Natural::from(123u32)).shl_round(100i32, RoundingMode::Exact)
        ///         .to_string(), "155921023828072216384094494261248");
        /// }
        impl<'a> ShlRound<$t> for &'a Natural {
            type Output = Natural;

            fn shl_round(self, other: $t, rm: RoundingMode) -> Natural {
                if other >= 0 {
                    self << other.to_unsigned_bitwise()
                } else {
                    self.shr_round(other.unsigned_abs(), rm)
                }
            }
        }

        /// Shifts a `Natural` left (multiplies it by a power of 2 or divides it by a power of 2 and
        /// takes the floor) and rounds according to the specified rounding mode, in place. Passing
        /// `RoundingMode::Floor` or `RoundingMode::Down` is equivalent to using `>>=`. To test
        /// whether `RoundingMode::Exact` can be passed, use
        /// `other > 0 || self.is_divisible_by_power_of_two(other)`.
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
        /// use malachite_base::num::traits::{One, ShlRoundAssign};
        /// use malachite_nz::natural::Natural;
        ///
        /// fn main() {
        ///     let mut n = Natural::from(0x101u32);
        ///     n.shl_round_assign(-8i8, RoundingMode::Down);
        ///     assert_eq!(n.to_string(), "1");
        ///
        ///     let mut n = Natural::from(0x101u32);
        ///     n.shl_round_assign(-8i16, RoundingMode::Up);
        ///     assert_eq!(n.to_string(), "2");
        ///
        ///     let mut n = Natural::from(0x101u32);
        ///     n.shl_round_assign(-9i32, RoundingMode::Down);
        ///     assert_eq!(n.to_string(), "0");
        ///
        ///     let mut n = Natural::from(0x101u32);
        ///     n.shl_round_assign(-9i64, RoundingMode::Up);
        ///     assert_eq!(n.to_string(), "1");
        ///
        ///     let mut n = Natural::from(0x101u32);
        ///     n.shl_round_assign(-9i8, RoundingMode::Nearest);
        ///     assert_eq!(n.to_string(), "1");
        ///
        ///     let mut n = Natural::from(0xffu32);
        ///     n.shl_round_assign(-9i16, RoundingMode::Nearest);
        ///     assert_eq!(n.to_string(), "0");
        ///
        ///     let mut n = Natural::from(0x100u32);
        ///     n.shl_round_assign(-9i32, RoundingMode::Nearest);
        ///     assert_eq!(n.to_string(), "0");
        ///
        ///     let mut n = Natural::from(0x100u32);
        ///     n.shl_round_assign(-8i64, RoundingMode::Exact);
        ///     assert_eq!(n.to_string(), "1");
        ///
        ///     let mut x = Natural::ONE;
        ///     x.shl_round_assign(1i8, RoundingMode::Exact);
        ///     x.shl_round_assign(2i16, RoundingMode::Exact);
        ///     x.shl_round_assign(3i32, RoundingMode::Exact);
        ///     x.shl_round_assign(4i64, RoundingMode::Exact);
        ///     assert_eq!(x.to_string(), "1024");
        /// }
        impl ShlRoundAssign<$t> for Natural {
            fn shl_round_assign(&mut self, other: $t, rm: RoundingMode) {
                if other >= 0 {
                    *self <<= other.to_unsigned_bitwise();
                } else {
                    self.shr_round_assign(other.unsigned_abs(), rm);
                }
            }
        }
    };
}
impl_natural_shl_signed!(i8);
impl_natural_shl_signed!(i16);
impl_natural_shl_signed!(i32);
impl_natural_shl_signed!(i64);
