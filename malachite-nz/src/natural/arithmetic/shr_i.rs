use std::ops::{Shr, ShrAssign};

use malachite_base::num::arithmetic::traits::{ShrRound, ShrRoundAssign, UnsignedAbs};
use malachite_base::rounding_mode::RoundingMode;

use natural::Natural;

macro_rules! impl_natural_shr_signed {
    ($t:ident) => {
        impl Shr<$t> for Natural {
            type Output = Natural;

            /// Shifts a `Natural` right (divides it by a power of 2 and takes the floor or
            /// multiplies it by a power of 2), taking the `Natural` by value.
            ///
            /// Time: worst case O(`bits`)
            ///
            /// Additional memory: worst case O(`bits`)
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use malachite_base::num::basic::traits::Zero;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!((Natural::ZERO >> 10i8).to_string(), "0");
            /// assert_eq!((Natural::from(492u32) >> 2i16).to_string(), "123");
            /// assert_eq!((Natural::trillion() >> 10i32).to_string(), "976562500");
            /// assert_eq!((Natural::ZERO >> -10i64).to_string(), "0");
            /// assert_eq!((Natural::from(123u32) >> -2i8).to_string(), "492");
            /// assert_eq!((Natural::from(123u32) >> -100i16).to_string(),
            ///     "155921023828072216384094494261248");
            /// ```
            #[inline]
            fn shr(mut self, bits: $t) -> Natural {
                self >>= bits;
                self
            }
        }

        impl<'a> Shr<$t> for &'a Natural {
            type Output = Natural;

            /// Shifts a `Natural` right (divides it by a power of 2 and takes the floor or
            /// multiplies it by a power of 2), taking the `Natural` by reference.
            ///
            /// Time: worst case O(`bits`)
            ///
            /// Additional memory: worst case O(`bits`)
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use malachite_base::num::basic::traits::Zero;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!((&Natural::ZERO >> -10i8).to_string(), "0");
            /// assert_eq!((&Natural::from(123u32) >> -2i16).to_string(), "492");
            /// assert_eq!((&Natural::from(123u32) >> -100i32).to_string(),
            ///     "155921023828072216384094494261248");
            /// assert_eq!((&Natural::ZERO >> 10i64).to_string(), "0");
            /// assert_eq!((&Natural::from(492u32) >> 2i8).to_string(), "123");
            /// assert_eq!((&Natural::trillion() >> 10i16).to_string(), "976562500");
            /// ```
            fn shr(self, bits: $t) -> Natural {
                if bits >= 0 {
                    self >> bits.unsigned_abs()
                } else {
                    self << bits.unsigned_abs()
                }
            }
        }

        impl ShrAssign<$t> for Natural {
            /// Shifts a `Natural` right (divides it by a power of 2 and takes the floor or
            /// multiplies it by a power of 2) in place.
            ///
            /// Time: worst case O(`bits`)
            ///
            /// Additional memory: worst case O(`bits`)
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use malachite_base::num::basic::traits::One;
            /// use malachite_nz::natural::Natural;
            ///
            /// let mut x = Natural::ONE;
            /// x >>= -1i8;
            /// x >>= -2i16;
            /// x >>= -3i32;
            /// x >>= -4i64;
            /// assert_eq!(x.to_string(), "1024");
            ///
            /// let mut x = Natural::from(1024u32);
            /// x >>= 1i8;
            /// x >>= 2i16;
            /// x >>= 3i32;
            /// x >>= 4i64;
            /// assert_eq!(x.to_string(), "1");
            /// ```
            fn shr_assign(&mut self, bits: $t) {
                if bits >= 0 {
                    *self >>= bits.unsigned_abs();
                } else {
                    *self <<= bits.unsigned_abs();
                }
            }
        }

        impl ShrRound<$t> for Natural {
            type Output = Natural;

            /// Shifts a `Natural` right (divides it by a power of 2 and takes the floor or
            /// multiplies it by a power of 2) and rounds according to the specified rounding mode,
            /// taking the `Natural` by value. Passing `RoundingMode::Floor` or `RoundingMode::Down`
            /// is equivalent to using `>>`. To test whether `RoundingMode::Exact` can be passed,
            /// use `bits < 0 || self.divisible_by_power_of_two(bits)`.
            ///
            /// Time: worst case O(`bits`)
            ///
            /// Additional memory: worst case O(`bits`)
            ///
            /// # Panics
            /// Panics if `bits` is positive and `rm` is `RoundingMode::Exact` but `self` is not
            /// divisible by 2<sup>`bits`</sup>.
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use malachite_base::rounding_mode::RoundingMode;
            /// use malachite_base::num::arithmetic::traits::ShrRound;
            /// use malachite_base::num::basic::traits::Zero;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(
            ///     Natural::from(0x101u32).shr_round(8i8, RoundingMode::Down).to_string(),
            ///     "1"
            /// );
            /// assert_eq!(
            ///     Natural::from(0x101u32).shr_round(8i16, RoundingMode::Up).to_string(),
            ///     "2"
            /// );
            ///
            /// assert_eq!(Natural::from(0x101u32).shr_round(9i32, RoundingMode::Down).to_string(),
            ///     "0");
            /// assert_eq!(
            ///     Natural::from(0x101u32).shr_round(9i64, RoundingMode::Up).to_string(),
            ///     "1"
            /// );
            /// assert_eq!(
            ///     Natural::from(0x101u32).shr_round(9i8, RoundingMode::Nearest).to_string(),
            ///     "1"
            /// );
            /// assert_eq!(
            ///     Natural::from(0xffu32).shr_round(9i16, RoundingMode::Nearest).to_string(),
            ///     "0"
            /// );
            /// assert_eq!(
            ///     Natural::from(0x100u32).shr_round(9i32, RoundingMode::Nearest).to_string(),
            ///     "0"
            /// );
            ///
            /// assert_eq!(Natural::from(0x100u32).shr_round(8i64, RoundingMode::Exact).to_string(),
            ///     "1");
            ///
            /// assert_eq!(Natural::ZERO.shr_round(-10i8, RoundingMode::Exact).to_string(), "0");
            /// assert_eq!(Natural::from(123u32).shr_round(-2i16, RoundingMode::Exact).to_string(),
            ///     "492");
            /// assert_eq!(
            ///     Natural::from(123u32).shr_round(-100i32, RoundingMode::Exact).to_string(),
            ///     "155921023828072216384094494261248"
            /// );
            /// ```
            #[inline]
            fn shr_round(mut self, bits: $t, rm: RoundingMode) -> Natural {
                self.shr_round_assign(bits, rm);
                self
            }
        }

        impl<'a> ShrRound<$t> for &'a Natural {
            type Output = Natural;

            /// Shifts a `Natural` right (divides it by a power of 2 and takes the floor or
            /// multiplies it by a power of 2) and rounds according to the specified rounding mode,
            /// taking the `Natural` by reference. Passing `RoundingMode::Floor` or
            /// `RoundingMode::Down` is equivalent to using `>>`. To test whether
            /// `RoundingMode::Exact` can be passed, use
            /// `bits < 0 || self.divisible_by_power_of_two(bits)`.
            ///
            /// Time: worst case O(`bits`)
            ///
            /// Additional memory: worst case O(`bits`)
            ///
            /// # Panics
            /// Panics if `bits` is positive and `rm` is `RoundingMode::Exact` but `self` is not
            /// divisible by 2<sup>`bits`</sup>.
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use malachite_base::rounding_mode::RoundingMode;
            /// use malachite_base::num::arithmetic::traits::ShrRound;
            /// use malachite_base::num::basic::traits::Zero;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(
            ///     (&Natural::from(0x101u32)).shr_round(8i8, RoundingMode::Down).to_string(),
            ///     "1"
            /// );
            /// assert_eq!(
            ///     (&Natural::from(0x101u32)).shr_round(8i16, RoundingMode::Up).to_string(),
            ///     "2"
            /// );
            ///
            /// assert_eq!(
            ///     (&Natural::from(0x101u32)).shr_round(9i32, RoundingMode::Down).to_string(),
            ///     "0"
            /// );
            /// assert_eq!((&Natural::from(0x101u32)).shr_round(9i64, RoundingMode::Up).to_string(),
            ///     "1");
            /// assert_eq!(
            ///     (&Natural::from(0x101u32)).shr_round(9i8, RoundingMode::Nearest).to_string(),
            ///     "1"
            /// );
            /// assert_eq!(
            ///     (&Natural::from(0xffu32)).shr_round(9i16, RoundingMode::Nearest).to_string(),
            ///     "0"
            /// );
            /// assert_eq!((&Natural::from(0x100u32)).shr_round(9i32, RoundingMode::Nearest)
            ///     .to_string(), "0");
            ///
            /// assert_eq!(
            ///     (&Natural::from(0x100u32)).shr_round(8i64, RoundingMode::Exact).to_string(),
            ///     "1"
            /// );
            ///
            /// assert_eq!((&Natural::ZERO).shr_round(-10i8, RoundingMode::Exact).to_string(), "0");
            /// assert_eq!(
            ///     (&Natural::from(123u32)).shr_round(-2i16, RoundingMode::Exact).to_string(),
            ///     "492"
            /// );
            /// assert_eq!(
            ///     (&Natural::from(123u32)).shr_round(-100i32, RoundingMode::Exact).to_string(),
            ///     "155921023828072216384094494261248"
            /// );
            /// ```
            fn shr_round(self, bits: $t, rm: RoundingMode) -> Natural {
                if bits >= 0 {
                    self.shr_round(bits.unsigned_abs(), rm)
                } else {
                    self << bits.unsigned_abs()
                }
            }
        }

        impl ShrRoundAssign<$t> for Natural {
            /// Shifts a `Natural` right (divides it by a power of 2 and takes the floor or
            /// multiplies it by a power of 2) and rounds according to the specified rounding mode,
            /// in place. Passing `RoundingMode::Floor` or `RoundingMode::Down` is equivalent to
            /// using `>>=`. To test whether `RoundingMode::Exact` can be passed, use
            /// `bits < 0 || self.divisible_by_power_of_two(bits)`.
            ///
            /// Time: worst case O(`bits`)
            ///
            /// Additional memory: worst case O(`bits`)
            ///
            /// # Panics
            /// Panics if `bits` is positive and `rm` is `RoundingMode::Exact` but `self` is not
            /// divisible by 2<sup>`bits`</sup>.
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use malachite_base::rounding_mode::RoundingMode;
            /// use malachite_base::num::arithmetic::traits::ShrRoundAssign;
            /// use malachite_base::num::basic::traits::One;
            /// use malachite_nz::natural::Natural;
            ///
            /// let mut n = Natural::from(0x101u32);
            /// n.shr_round_assign(8i8, RoundingMode::Down);
            /// assert_eq!(n.to_string(), "1");
            ///
            /// let mut n = Natural::from(0x101u32);
            /// n.shr_round_assign(8i16, RoundingMode::Up);
            /// assert_eq!(n.to_string(), "2");
            ///
            /// let mut n = Natural::from(0x101u32);
            /// n.shr_round_assign(9i32, RoundingMode::Down);
            /// assert_eq!(n.to_string(), "0");
            ///
            /// let mut n = Natural::from(0x101u32);
            /// n.shr_round_assign(9i64, RoundingMode::Up);
            /// assert_eq!(n.to_string(), "1");
            ///
            /// let mut n = Natural::from(0x101u32);
            /// n.shr_round_assign(9i8, RoundingMode::Nearest);
            /// assert_eq!(n.to_string(), "1");
            ///
            /// let mut n = Natural::from(0xffu32);
            /// n.shr_round_assign(9i16, RoundingMode::Nearest);
            /// assert_eq!(n.to_string(), "0");
            ///
            /// let mut n = Natural::from(0x100u32);
            /// n.shr_round_assign(9i32, RoundingMode::Nearest);
            /// assert_eq!(n.to_string(), "0");
            ///
            /// let mut n = Natural::from(0x100u32);
            /// n.shr_round_assign(8i64, RoundingMode::Exact);
            /// assert_eq!(n.to_string(), "1");
            ///
            /// let mut x = Natural::ONE;
            /// x.shr_round_assign(-1i8, RoundingMode::Exact);
            /// x.shr_round_assign(-2i16, RoundingMode::Exact);
            /// x.shr_round_assign(-3i32, RoundingMode::Exact);
            /// x.shr_round_assign(-4i64, RoundingMode::Exact);
            /// assert_eq!(x.to_string(), "1024");
            /// ```
            fn shr_round_assign(&mut self, bits: $t, rm: RoundingMode) {
                if bits >= 0 {
                    self.shr_round_assign(bits.unsigned_abs(), rm);
                } else {
                    *self <<= bits.unsigned_abs()
                }
            }
        }
    };
}
impl_natural_shr_signed!(i8);
impl_natural_shr_signed!(i16);
impl_natural_shr_signed!(i32);
impl_natural_shr_signed!(i64);
impl_natural_shr_signed!(i128);
impl_natural_shr_signed!(isize);
