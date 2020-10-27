use std::ops::{Shl, ShlAssign};

use malachite_base::num::arithmetic::traits::{ShrRound, ShrRoundAssign, UnsignedAbs};
use malachite_base::num::basic::traits::Zero;
use malachite_base::rounding_modes::RoundingMode;

use integer::Integer;
use natural::Natural;

fn _shr_round_unsigned_ref<'a, T>(x: &'a Integer, bits: T, rm: RoundingMode) -> Integer
where
    &'a Natural: ShrRound<T, Output = Natural>,
{
    match *x {
        Integer {
            sign: true,
            ref abs,
        } => Integer {
            sign: true,
            abs: abs.shr_round(bits, rm),
        },
        Integer {
            sign: false,
            ref abs,
        } => {
            let abs_shifted = abs.shr_round(bits, -rm);
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

fn _shr_round_assign_unsigned<T>(x: &mut Integer, bits: T, rm: RoundingMode)
where
    Natural: ShrRoundAssign<T>,
{
    match *x {
        Integer {
            sign: true,
            ref mut abs,
        } => {
            abs.shr_round_assign(bits, rm);
        }
        Integer {
            sign: false,
            ref mut abs,
        } => {
            abs.shr_round_assign(bits, -rm);
            if *abs == 0 {
                x.sign = true;
            }
        }
    }
}

macro_rules! impl_shr_round_unsigned {
    ($t:ident) => {
        impl ShrRound<$t> for Integer {
            type Output = Integer;

            /// Shifts an `Integer` right (divides it by a power of 2) and rounds according to the
            /// specified rounding mode, taking the `Integer` by value. Passing
            /// `RoundingMode::Floor` or `RoundingMode::Down` is equivalent to using `>>`. To test
            /// whether `RoundingMode::Exact` can be passed, use
            /// `self.divisible_by_power_of_two(bits)`.
            ///
            /// Time: worst case O(`bits`)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `rm` is `RoundingMode::Exact` but `self` is not divisible by
            /// 2<sup>`bits`</sup>.
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use malachite_base::rounding_modes::RoundingMode;
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
            fn shr_round(mut self, bits: $t, rm: RoundingMode) -> Integer {
                self.shr_round_assign(bits, rm);
                self
            }
        }

        impl<'a> ShrRound<$t> for &'a Integer {
            type Output = Integer;

            /// Shifts an `Integer` right (divides it by a power of 2) and rounds according to the
            /// specified rounding mode, taking the `Integer` by reference. Passing
            /// `RoundingMode::Floor` or `RoundingMode::Down` is equivalent to using `>>`. To test
            /// whether `RoundingMode::Exact` can be passed, use
            /// `self.divisible_by_power_of_two(bits)`.
            ///
            /// Time: worst case O(`bits`)
            ///
            /// Additional memory: worst case O(`bits`)
            ///
            /// # Panics
            /// Panics if `rm` is `RoundingMode::Exact` but `self` is not divisible by
            /// 2<sup>`bits`</sup>.
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use malachite_base::rounding_modes::RoundingMode;
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
            #[inline]
            fn shr_round(self, bits: $t, rm: RoundingMode) -> Integer {
                _shr_round_unsigned_ref(self, bits, rm)
            }
        }

        impl ShrRoundAssign<$t> for Integer {
            /// Shifts an `Integer` right (divides it by a power of 2) and rounds according to the
            /// specified rounding mode, in place. Passing `RoundingMode::Floor` or
            /// `RoundingMode::Down` is equivalent to using `>>=`. To test whether
            /// `RoundingMode::Exact` can be passed, use `self.divisible_by_power_of_two(bits)`.
            ///
            /// Time: worst case O(`bits`)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `rm` is `RoundingMode::Exact` but `self` is not divisible by
            /// 2<sup>`bits`</sup>.
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use malachite_base::rounding_modes::RoundingMode;
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
            fn shr_round_assign(&mut self, bits: $t, rm: RoundingMode) {
                _shr_round_assign_unsigned(self, bits, rm);
            }
        }
    };
}
apply_to_unsigneds!(impl_shr_round_unsigned);

fn _shr_round_signed_ref<'a, U, S: Copy + Ord + UnsignedAbs<Output = U> + Zero>(
    x: &'a Integer,
    bits: S,
    rm: RoundingMode,
) -> Integer
where
    &'a Integer: Shl<U, Output = Integer> + ShrRound<U, Output = Integer>,
{
    if bits >= S::ZERO {
        x.shr_round(bits.unsigned_abs(), rm)
    } else {
        x << bits.unsigned_abs()
    }
}

fn _shr_round_assign_signed<U, S: Copy + Ord + UnsignedAbs<Output = U> + Zero>(
    x: &mut Integer,
    bits: S,
    rm: RoundingMode,
) where
    Integer: ShlAssign<U> + ShrRoundAssign<U>,
{
    if bits >= S::ZERO {
        x.shr_round_assign(bits.unsigned_abs(), rm);
    } else {
        *x <<= bits.unsigned_abs();
    }
}

macro_rules! impl_shr_round_signed {
    ($t:ident) => {
        impl ShrRound<$t> for Integer {
            type Output = Integer;

            /// Shifts an `Integer` right (divides it by a power of 2 and takes the floor or
            /// multiplies it by a power of 2) and rounds according to the specified rounding mode,
            /// taking the `Integer` by value. Passing `RoundingMode::Floor` or `RoundingMode::Down`
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
            /// use malachite_base::rounding_modes::RoundingMode;
            /// use malachite_base::num::arithmetic::traits::ShrRound;
            /// use malachite_base::num::basic::traits::Zero;
            /// use malachite_nz::integer::Integer;
            ///
            /// assert_eq!(
            ///     Integer::from(0x101u32).shr_round(8i8, RoundingMode::Down).to_string(),
            ///     "1"
            /// );
            /// assert_eq!(
            ///     Integer::from(0x101u32).shr_round(8i16, RoundingMode::Up).to_string(),
            ///     "2"
            /// );
            ///
            /// assert_eq!(
            ///     Integer::from(-0x101).shr_round(9i32, RoundingMode::Down).to_string(),
            ///     "0"
            /// );
            /// assert_eq!(
            ///     Integer::from(-0x101).shr_round(9i64, RoundingMode::Up).to_string(),
            ///     "-1"
            /// );
            /// assert_eq!(
            ///     Integer::from(-0x101).shr_round(9i8, RoundingMode::Nearest).to_string(),
            ///     "-1"
            /// );
            /// assert_eq!(
            ///     Integer::from(-0xff).shr_round(9i16, RoundingMode::Nearest).to_string(),
            ///     "0"
            /// );
            /// assert_eq!(
            ///     Integer::from(-0x100).shr_round(9i32, RoundingMode::Nearest).to_string(),
            ///     "0"
            /// );
            ///
            /// assert_eq!(
            ///     Integer::from(0x100u32).shr_round(8i64, RoundingMode::Exact).to_string(),
            ///     "1"
            /// );
            ///
            /// assert_eq!(Integer::ZERO.shr_round(-10i8, RoundingMode::Exact).to_string(), "0");
            /// assert_eq!(
            ///     Integer::from(123u32).shr_round(-2i16, RoundingMode::Exact).to_string(),
            ///     "492"
            /// );
            /// assert_eq!(
            ///     Integer::from(123u32).shr_round(-100i32, RoundingMode::Exact).to_string(),
            ///     "155921023828072216384094494261248"
            /// );
            /// ```
            #[inline]
            fn shr_round(mut self, bits: $t, rm: RoundingMode) -> Integer {
                self.shr_round_assign(bits, rm);
                self
            }
        }

        impl<'a> ShrRound<$t> for &'a Integer {
            type Output = Integer;

            /// Shifts an `Integer` right (divides it by a power of 2 and takes the floor or
            /// multiplies it by a power of 2) and rounds according to the specified rounding mode,
            /// taking the `Integer` by reference. Passing `RoundingMode::Floor` or
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
            /// use malachite_base::rounding_modes::RoundingMode;
            /// use malachite_base::num::arithmetic::traits::ShrRound;
            /// use malachite_base::num::basic::traits::Zero;
            /// use malachite_nz::integer::Integer;
            ///
            /// assert_eq!(
            ///     (&Integer::from(0x101u32)).shr_round(8i8, RoundingMode::Down).to_string(),
            ///     "1"
            /// );
            /// assert_eq!(
            ///     (&Integer::from(0x101u32)).shr_round(8i16, RoundingMode::Up).to_string(),
            ///     "2"
            /// );
            ///
            /// assert_eq!(
            ///     (&Integer::from(-0x101)).shr_round(9i32, RoundingMode::Down).to_string(),
            ///     "0"
            /// );
            /// assert_eq!(
            ///     (&Integer::from(-0x101)).shr_round(9i64, RoundingMode::Up).to_string(),
            ///     "-1"
            /// );
            /// assert_eq!(
            ///     (&Integer::from(-0x101)).shr_round(9i8, RoundingMode::Nearest).to_string(),
            ///     "-1"
            /// );
            /// assert_eq!(
            ///     (&Integer::from(-0xff)).shr_round(9i16, RoundingMode::Nearest).to_string(),
            ///     "0"
            /// );
            /// assert_eq!(
            ///     (&Integer::from(-0x100)).shr_round(9i32, RoundingMode::Nearest).to_string(),
            ///     "0"
            /// );
            ///
            /// assert_eq!(
            ///     (&Integer::from(0x100u32)).shr_round(8i64, RoundingMode::Exact).to_string(),
            ///     "1"
            /// );
            ///
            /// assert_eq!(
            ///     (&Integer::ZERO).shr_round(-10i8, RoundingMode::Exact).to_string(),
            ///     "0"
            /// );
            /// assert_eq!(
            ///     (&Integer::from(123u32)).shr_round(-2i16, RoundingMode::Exact).to_string(),
            ///     "492"
            /// );
            /// assert_eq!(
            ///     (&Integer::from(123u32)).shr_round(-100i32, RoundingMode::Exact).to_string(),
            ///     "155921023828072216384094494261248"
            /// );
            /// ```
            #[inline]
            fn shr_round(self, bits: $t, rm: RoundingMode) -> Integer {
                _shr_round_signed_ref(self, bits, rm)
            }
        }

        impl ShrRoundAssign<$t> for Integer {
            /// Shifts an `Integer` right (divides it by a power of 2 and takes the floor or
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
            /// use malachite_base::rounding_modes::RoundingMode;
            /// use malachite_base::num::arithmetic::traits::ShrRoundAssign;
            /// use malachite_base::num::basic::traits::One;
            /// use malachite_nz::integer::Integer;
            ///
            /// let mut n = Integer::from(0x101u32);
            /// n.shr_round_assign(8i8, RoundingMode::Down);
            /// assert_eq!(n.to_string(), "1");
            ///
            /// let mut n = Integer::from(0x101u32);
            /// n.shr_round_assign(8i16, RoundingMode::Up);
            /// assert_eq!(n.to_string(), "2");
            ///
            /// let mut n = Integer::from(-0x101);
            /// n.shr_round_assign(9i32, RoundingMode::Down);
            /// assert_eq!(n.to_string(), "0");
            ///
            /// let mut n = Integer::from(-0x101);
            /// n.shr_round_assign(9i64, RoundingMode::Up);
            /// assert_eq!(n.to_string(), "-1");
            ///
            /// let mut n = Integer::from(-0x101);
            /// n.shr_round_assign(9i8, RoundingMode::Nearest);
            /// assert_eq!(n.to_string(), "-1");
            ///
            /// let mut n = Integer::from(-0xff);
            /// n.shr_round_assign(9i16, RoundingMode::Nearest);
            /// assert_eq!(n.to_string(), "0");
            ///
            /// let mut n = Integer::from(-0x100);
            /// n.shr_round_assign(9i32, RoundingMode::Nearest);
            /// assert_eq!(n.to_string(), "0");
            ///
            /// let mut n = Integer::from(0x100u32);
            /// n.shr_round_assign(8i64, RoundingMode::Exact);
            /// assert_eq!(n.to_string(), "1");
            ///
            /// let mut x = Integer::ONE;
            /// x.shr_round_assign(-1i8, RoundingMode::Exact);
            /// x.shr_round_assign(-2i16, RoundingMode::Exact);
            /// x.shr_round_assign(-3i32, RoundingMode::Exact);
            /// x.shr_round_assign(-4i64, RoundingMode::Exact);
            /// assert_eq!(x.to_string(), "1024");
            /// ```
            #[inline]
            fn shr_round_assign(&mut self, bits: $t, rm: RoundingMode) {
                _shr_round_assign_signed(self, bits, rm);
            }
        }
    };
}
apply_to_signeds!(impl_shr_round_signed);
