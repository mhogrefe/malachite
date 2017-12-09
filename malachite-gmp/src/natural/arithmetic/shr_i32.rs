use malachite_base::round::RoundingMode;
use malachite_base::traits::{ShrRound, ShrRoundAssign};
use natural::Natural;
use std::ops::{Shr, ShrAssign};

/// Shifts a `Natural` right (divides it by a power of 2 and takes the floor or multiplies it by a
/// power of 2), taking the `Natural` by value.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::Zero;
/// use malachite_gmp::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((Natural::ZERO >> 10i32).to_string(), "0");
///     assert_eq!((Natural::from(492u32) >> 2i32).to_string(), "123");
///     assert_eq!((Natural::from_str("1000000000000").unwrap() >> 10i32).to_string(),
///         "976562500");
///     assert_eq!((Natural::ZERO >> -10i32).to_string(), "0");
///     assert_eq!((Natural::from(123u32) >> -2i32).to_string(), "492");
///     assert_eq!((Natural::from(123u32) >> -100i32).to_string(),
///         "155921023828072216384094494261248");
/// }
/// ```
impl Shr<i32> for Natural {
    type Output = Natural;

    fn shr(mut self, other: i32) -> Natural {
        self >>= other;
        self
    }
}

/// Shifts a `Natural` right (divides it by a power of 2 and takes the floor or multiplies it by a
/// power of 2), taking the `Natural` by reference.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::Zero;
/// use malachite_gmp::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((&Natural::ZERO >> -10i32).to_string(), "0");
///     assert_eq!((&Natural::from(123u32) >> -2i32).to_string(), "492");
///     assert_eq!((&Natural::from(123u32) >> -100i32).to_string(),
///         "155921023828072216384094494261248");
///     assert_eq!((&Natural::ZERO >> 10i32).to_string(), "0");
///     assert_eq!((&Natural::from(492u32) >> 2i32).to_string(), "123");
///     assert_eq!((&Natural::from_str("1000000000000").unwrap() >> 10i32).to_string(),
///         "976562500");
/// }
/// ```
impl<'a> Shr<i32> for &'a Natural {
    type Output = Natural;

    fn shr(self, other: i32) -> Natural {
        if other >= 0 {
            self >> (other as u32)
        } else {
            self << (other.wrapping_abs() as u32)
        }
    }
}

/// Shifts a `Natural` right (divides it by a power of 2 and takes the floor or multiplies it by a
/// power of 2) in place.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::One;
/// use malachite_gmp::natural::Natural;
///
/// fn main() {
///     let mut x = Natural::ONE;
///     x >>= -1;
///     x >>= -2;
///     x >>= -3;
///     x >>= -4;
///     assert_eq!(x.to_string(), "1024");
///
///     let mut x = Natural::from(1024u32);
///     x >>= 1;
///     x >>= 2;
///     x >>= 3;
///     x >>= 4;
///     assert_eq!(x.to_string(), "1");
/// }
/// ```
impl ShrAssign<i32> for Natural {
    fn shr_assign(&mut self, other: i32) {
        if other >= 0 {
            *self >>= other as u32;
        } else {
            *self <<= other.wrapping_abs() as u32;
        }
    }
}

/// Shifts a `Natural` right (divides it by a power of 2 and takes the floor or multiplies it by a
/// power of 2) and rounds according to the specified rounding mode, taking the `Natural` by value.
/// Passing `RoundingMode::Floor` or `RoundingMode::Down` is equivalent to using `>>`. To test
/// whether `RoundingMode::Exact` can be passed, use
/// `other < 0 || self.is_divisible_by_power_of_two(other)`.
///
/// # Panics
/// Panics if `other` is positive and `rm` is `RoundingMode::Exact` but `self` is not divisible by
/// 2^(`other`).
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::round::RoundingMode;
/// use malachite_base::traits::{ShrRound, Zero};
/// use malachite_gmp::natural::Natural;
///
/// fn main() {
///     assert_eq!(Natural::from(257u32).shr_round(8i32, RoundingMode::Down).to_string(), "1");
///     assert_eq!(Natural::from(257u32).shr_round(8i32, RoundingMode::Up).to_string(), "2");
///
///     assert_eq!(Natural::from(257u32).shr_round(9i32, RoundingMode::Down).to_string(), "0");
///     assert_eq!(Natural::from(257u32).shr_round(9i32, RoundingMode::Up).to_string(), "1");
///     assert_eq!(Natural::from(257u32).shr_round(9i32, RoundingMode::Nearest).to_string(), "1");
///     assert_eq!(Natural::from(255u32).shr_round(9i32, RoundingMode::Nearest).to_string(), "0");
///     assert_eq!(Natural::from(256u32).shr_round(9i32, RoundingMode::Nearest).to_string(), "0");
///
///     assert_eq!(Natural::from(256u32).shr_round(8i32, RoundingMode::Exact).to_string(), "1");
///
///     assert_eq!(Natural::ZERO.shr_round(-10i32, RoundingMode::Exact).to_string(), "0");
///     assert_eq!(Natural::from(123u32).shr_round(-2i32, RoundingMode::Exact).to_string(), "492");
///     assert_eq!(Natural::from(123u32).shr_round(-100i32, RoundingMode::Exact).to_string(),
///         "155921023828072216384094494261248");
/// }
impl ShrRound<i32> for Natural {
    type Output = Natural;

    fn shr_round(mut self, other: i32, rm: RoundingMode) -> Natural {
        self.shr_round_assign(other, rm);
        self
    }
}

/// Shifts a `Natural` right (divides it by a power of 2 and takes the floor or multiplies it by a
/// power of 2) and rounds according to the specified rounding mode, taking the `Natural` by
/// reference. Passing `RoundingMode::Floor` or `RoundingMode::Down` is equivalent to using `>>`. To
/// test whether `RoundingMode::Exact` can be passed, use
/// `other < 0 || self.is_divisible_by_power_of_two(other as u32)`.
///
/// # Panics
/// Panics if `other` is positive and `rm` is `RoundingMode::Exact` but `self` is not divisible by
/// 2^(`other`).
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::round::RoundingMode;
/// use malachite_base::traits::{ShrRound, Zero};
/// use malachite_gmp::natural::Natural;
///
/// fn main() {
///     assert_eq!((&Natural::from(257u32)).shr_round(8i32, RoundingMode::Down).to_string(), "1");
///     assert_eq!((&Natural::from(257u32)).shr_round(8i32, RoundingMode::Up).to_string(), "2");
///
///     assert_eq!((&Natural::from(257u32)).shr_round(9i32, RoundingMode::Down).to_string(), "0");
///     assert_eq!((&Natural::from(257u32)).shr_round(9i32, RoundingMode::Up).to_string(), "1");
///     assert_eq!((&Natural::from(257u32)).shr_round(9i32, RoundingMode::Nearest).to_string(),
///         "1");
///     assert_eq!((&Natural::from(255u32)).shr_round(9i32, RoundingMode::Nearest).to_string(),
///         "0");
///     assert_eq!((&Natural::from(256u32)).shr_round(9i32, RoundingMode::Nearest).to_string(),
///         "0");
///
///     assert_eq!((&Natural::from(256u32)).shr_round(8i32, RoundingMode::Exact).to_string(), "1");
///
///     assert_eq!((&Natural::ZERO).shr_round(-10i32, RoundingMode::Exact).to_string(), "0");
///     assert_eq!((&Natural::from(123u32)).shr_round(-2i32, RoundingMode::Exact).to_string(),
///         "492");
///     assert_eq!((&Natural::from(123u32)).shr_round(-100i32, RoundingMode::Exact).to_string(),
///         "155921023828072216384094494261248");
/// }
impl<'a> ShrRound<i32> for &'a Natural {
    type Output = Natural;

    fn shr_round(self, other: i32, rm: RoundingMode) -> Natural {
        if other >= 0 {
            self.shr_round(other as u32, rm)
        } else {
            self << (other.wrapping_abs() as u32)
        }
    }
}

/// Shifts a `Natural` right (divides it by a power of 2 and takes the floor or multiplies it by a
/// power of 2) and rounds according to the specified rounding mode, in place. Passing
/// `RoundingMode::Floor` or `RoundingMode::Down` is equivalent to using `>>=`. To test whether
/// `RoundingMode::Exact` can be passed, use
/// `other < 0 || self.is_divisible_by_power_of_two(other as u32)`.
///
/// # Panics
/// Panics if `other` is positive and `rm` is `RoundingMode::Exact` but `self` is not divisible by
/// 2^(`other`).
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::round::RoundingMode;
/// use malachite_base::traits::{One, ShrRoundAssign};
/// use malachite_gmp::natural::Natural;
///
/// fn main() {
///     let mut n = Natural::from(257u32);
///     n.shr_round_assign(8i32, RoundingMode::Down);
///     assert_eq!(n.to_string(), "1");
///
///     let mut n = Natural::from(257u32);
///     n.shr_round_assign(8i32, RoundingMode::Up);
///     assert_eq!(n.to_string(), "2");
///
///     let mut n = Natural::from(257u32);
///     n.shr_round_assign(9i32, RoundingMode::Down);
///     assert_eq!(n.to_string(), "0");
///
///     let mut n = Natural::from(257u32);
///     n.shr_round_assign(9i32, RoundingMode::Up);
///     assert_eq!(n.to_string(), "1");
///
///     let mut n = Natural::from(257u32);
///     n.shr_round_assign(9i32, RoundingMode::Nearest);
///     assert_eq!(n.to_string(), "1");
///
///     let mut n = Natural::from(255u32);
///     n.shr_round_assign(9i32, RoundingMode::Nearest);
///     assert_eq!(n.to_string(), "0");
///
///     let mut n = Natural::from(256u32);
///     n.shr_round_assign(9i32, RoundingMode::Nearest);
///     assert_eq!(n.to_string(), "0");
///
///     let mut n = Natural::from(256u32);
///     n.shr_round_assign(8i32, RoundingMode::Exact);
///     assert_eq!(n.to_string(), "1");
///
///     let mut x = Natural::ONE;
///     x.shr_round_assign(-1i32, RoundingMode::Exact);
///     x.shr_round_assign(-2i32, RoundingMode::Exact);
///     x.shr_round_assign(-3i32, RoundingMode::Exact);
///     x.shr_round_assign(-4i32, RoundingMode::Exact);
///     assert_eq!(x.to_string(), "1024");
/// }
impl ShrRoundAssign<i32> for Natural {
    fn shr_round_assign(&mut self, other: i32, rm: RoundingMode) {
        if other >= 0 {
            self.shr_round_assign(other as u32, rm);
        } else {
            *self <<= other.wrapping_abs() as u32;
        }
    }
}
