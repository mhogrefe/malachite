use malachite_base::num::arithmetic::traits::{
    ShlRound, ShlRoundAssign, ShrRound, ShrRoundAssign, UnsignedAbs,
};
use malachite_base::rounding_modes::RoundingMode;

use natural::Natural;

macro_rules! impl_natural_shl_round_signed {
    ($t:ident) => {
        impl ShlRound<$t> for Natural {
            type Output = Natural;

            /// Shifts a `Natural` left (multiplies it by a power of 2 or divides it by a power of 2
            /// and takes the floor) and rounds according to the specified rounding mode, taking the
            /// `Natural` by value. Passing `RoundingMode::Floor` or `RoundingMode::Down` is
            /// equivalent to using `>>`. To test whether `RoundingMode::Exact` can be passed, use
            /// `bits > 0 || self.divisible_by_power_of_two(bits)`.
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
            /// use malachite_base::rounding_modes::RoundingMode;
            /// use malachite_base::num::arithmetic::traits::ShlRound;
            /// use malachite_base::num::basic::traits::Zero;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(Natural::from(0x101u32).shl_round(-8i8, RoundingMode::Down).to_string(),
            ///     "1");
            /// assert_eq!(
            ///     Natural::from(0x101u32).shl_round(-8i16, RoundingMode::Up).to_string(),
            ///     "2"
            /// );
            ///
            /// assert_eq!(
            ///     Natural::from(0x101u32).shl_round(-9i32, RoundingMode::Down).to_string(),
            ///     "0"
            /// );
            /// assert_eq!(
            ///     Natural::from(0x101u32).shl_round(-9i64, RoundingMode::Up).to_string(),
            ///     "1"
            /// );
            /// assert_eq!(
            ///     Natural::from(0x101u32).shl_round(-9i8, RoundingMode::Nearest).to_string(),
            ///     "1"
            /// );
            /// assert_eq!(
            ///     Natural::from(0xffu32).shl_round(-9i16, RoundingMode::Nearest).to_string(),
            ///     "0"
            /// );
            /// assert_eq!(
            ///     Natural::from(0x100u32).shl_round(-9i32, RoundingMode::Nearest).to_string(),
            ///     "0"
            /// );
            ///
            /// assert_eq!(
            ///     Natural::from(0x100u32).shl_round(-8i64, RoundingMode::Exact).to_string(),
            ///     "1"
            /// );
            ///
            /// assert_eq!(Natural::ZERO.shl_round(10i8, RoundingMode::Exact).to_string(), "0");
            /// assert_eq!(Natural::from(123u32).shl_round(2i16, RoundingMode::Exact).to_string(),
            ///     "492");
            /// assert_eq!(Natural::from(123u32).shl_round(100i32, RoundingMode::Exact).to_string(),
            ///     "155921023828072216384094494261248");
            /// ```
            #[inline]
            fn shl_round(mut self, bits: $t, rm: RoundingMode) -> Natural {
                self.shl_round_assign(bits, rm);
                self
            }
        }

        impl<'a> ShlRound<$t> for &'a Natural {
            type Output = Natural;

            /// Shifts a `Natural` left (multiplies it by a power of 2 or divides it by a power of 2
            /// and takes the floor) and rounds according to the specified rounding mode, taking the
            /// `Natural` by reference. Passing `RoundingMode::Floor` or `RoundingMode::Down` is
            /// equivalent to using `>>`. To test whether `RoundingMode::Exact` can be passed, use
            /// `bits > 0 || self.divisible_by_power_of_two(bits)`.
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
            /// use malachite_base::rounding_modes::RoundingMode;
            /// use malachite_base::num::arithmetic::traits::ShlRound;
            /// use malachite_base::num::basic::traits::Zero;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(
            ///     (&Natural::from(0x101u32)).shl_round(-8i8, RoundingMode::Down).to_string(),
            ///     "1"
            /// );
            /// assert_eq!(
            ///     (&Natural::from(0x101u32)).shl_round(-8i16, RoundingMode::Up).to_string(),
            ///     "2"
            /// );
            ///
            /// assert_eq!(
            ///     (&Natural::from(0x101u32)).shl_round(-9i32, RoundingMode::Down).to_string(),
            ///     "0"
            /// );
            /// assert_eq!(
            ///     (&Natural::from(0x101u32)).shl_round(-9i64, RoundingMode::Up).to_string(),
            ///     "1"
            /// );
            /// assert_eq!(
            ///     (&Natural::from(0x101u32)).shl_round(-9i8, RoundingMode::Nearest).to_string(),
            ///     "1"
            /// );
            /// assert_eq!(
            ///     (&Natural::from(0xffu32)).shl_round(-9i16, RoundingMode::Nearest).to_string(),
            ///     "0"
            /// );
            /// assert_eq!(
            ///     (&Natural::from(0x100u32)).shl_round(-9i32, RoundingMode::Nearest).to_string(),
            ///     "0"
            /// );
            ///
            /// assert_eq!(
            ///     (&Natural::from(0x100u32)).shl_round(-8i64, RoundingMode::Exact).to_string(),
            ///     "1"
            /// );
            ///
            /// assert_eq!((&Natural::ZERO).shl_round(10i8, RoundingMode::Exact).to_string(), "0");
            /// assert_eq!(
            ///     (&Natural::from(123u32)).shl_round(2i16, RoundingMode::Exact).to_string(),
            ///     "492"
            /// );
            /// assert_eq!(
            ///     (&Natural::from(123u32)).shl_round(100i32, RoundingMode::Exact).to_string(),
            ///     "155921023828072216384094494261248"
            /// );
            /// ```
            fn shl_round(self, bits: $t, rm: RoundingMode) -> Natural {
                if bits >= 0 {
                    self << bits.unsigned_abs()
                } else {
                    self.shr_round(bits.unsigned_abs(), rm)
                }
            }
        }

        impl ShlRoundAssign<$t> for Natural {
            /// Shifts a `Natural` left (multiplies it by a power of 2 or divides it by a power of 2
            /// and takes the floor) and rounds according to the specified rounding mode, in place.
            /// Passing `RoundingMode::Floor` or `RoundingMode::Down` is equivalent to using `>>=`.
            /// To test whether `RoundingMode::Exact` can be passed, use
            /// `bits > 0 || self.divisible_by_power_of_two(bits)`.
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
            /// use malachite_base::rounding_modes::RoundingMode;
            /// use malachite_base::num::arithmetic::traits::ShlRoundAssign;
            /// use malachite_base::num::basic::traits::One;
            /// use malachite_nz::natural::Natural;
            ///
            /// let mut n = Natural::from(0x101u32);
            /// n.shl_round_assign(-8i8, RoundingMode::Down);
            /// assert_eq!(n.to_string(), "1");
            ///
            /// let mut n = Natural::from(0x101u32);
            /// n.shl_round_assign(-8i16, RoundingMode::Up);
            /// assert_eq!(n.to_string(), "2");
            ///
            /// let mut n = Natural::from(0x101u32);
            /// n.shl_round_assign(-9i32, RoundingMode::Down);
            /// assert_eq!(n.to_string(), "0");
            ///
            /// let mut n = Natural::from(0x101u32);
            /// n.shl_round_assign(-9i64, RoundingMode::Up);
            /// assert_eq!(n.to_string(), "1");
            ///
            /// let mut n = Natural::from(0x101u32);
            /// n.shl_round_assign(-9i8, RoundingMode::Nearest);
            /// assert_eq!(n.to_string(), "1");
            ///
            /// let mut n = Natural::from(0xffu32);
            /// n.shl_round_assign(-9i16, RoundingMode::Nearest);
            /// assert_eq!(n.to_string(), "0");
            ///
            /// let mut n = Natural::from(0x100u32);
            /// n.shl_round_assign(-9i32, RoundingMode::Nearest);
            /// assert_eq!(n.to_string(), "0");
            ///
            /// let mut n = Natural::from(0x100u32);
            /// n.shl_round_assign(-8i64, RoundingMode::Exact);
            /// assert_eq!(n.to_string(), "1");
            ///
            /// let mut x = Natural::ONE;
            /// x.shl_round_assign(1i8, RoundingMode::Exact);
            /// x.shl_round_assign(2i16, RoundingMode::Exact);
            /// x.shl_round_assign(3i32, RoundingMode::Exact);
            /// x.shl_round_assign(4i64, RoundingMode::Exact);
            /// assert_eq!(x.to_string(), "1024");
            /// ```
            fn shl_round_assign(&mut self, bits: $t, rm: RoundingMode) {
                if bits >= 0 {
                    *self <<= bits.unsigned_abs();
                } else {
                    self.shr_round_assign(bits.unsigned_abs(), rm);
                }
            }
        }
    };
}
apply_to_signeds!(impl_natural_shl_round_signed);
