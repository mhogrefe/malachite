use malachite_base::round::RoundingMode;
use malachite_base::traits::{ShlRound, ShlRoundAssign, ShrRound, ShrRoundAssign};
use natural::Natural;
use std::ops::{Shl, ShlAssign};

/// Shifts a `Natural` left (multiplies it by a power of 2 or divides it by a power of 2 and takes
/// the floor), taking the `Natural` by value.
///
/// Time: worst case O(`other`)
///
/// Additional memory: worst case O(`other`)
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_native;
///
/// use malachite_base::traits::Zero;
/// use malachite_native::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((Natural::ZERO << 10i32).to_string(), "0");
///     assert_eq!((Natural::from(123u32) << 2i32).to_string(), "492");
///     assert_eq!((Natural::from(123u32) << 100i32).to_string(),
///         "155921023828072216384094494261248");
///     assert_eq!((Natural::ZERO << -10i32).to_string(), "0");
///     assert_eq!((Natural::from(492u32) << -2i32).to_string(), "123");
///     assert_eq!((Natural::from_str("1000000000000").unwrap() << -10i32).to_string(),
///         "976562500");
/// }
/// ```
impl Shl<i32> for Natural {
    type Output = Natural;

    fn shl(mut self, other: i32) -> Natural {
        self <<= other;
        self
    }
}

/// Shifts a `Natural` left (multiplies it by a power of 2 or divides it by a power of 2 and takes
/// the floor), taking the `Natural` by reference.
///
/// Time: worst case O(`other`)
///
/// Additional memory: worst case O(`other`)
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_native;
///
/// use malachite_base::traits::Zero;
/// use malachite_native::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((&Natural::ZERO << 10i32).to_string(), "0");
///     assert_eq!((&Natural::from(123u32) << 2i32).to_string(), "492");
///     assert_eq!((&Natural::from(123u32) << 100i32).to_string(),
///         "155921023828072216384094494261248");
///     assert_eq!((&Natural::ZERO << -10i32).to_string(), "0");
///     assert_eq!((&Natural::from(492u32) << -2i32).to_string(), "123");
///     assert_eq!((&Natural::from_str("1000000000000").unwrap() << -10i32).to_string(),
///         "976562500");
/// }
/// ```
impl<'a> Shl<i32> for &'a Natural {
    type Output = Natural;

    fn shl(self, other: i32) -> Natural {
        if other >= 0 {
            self << (other as u32)
        } else {
            self >> (other.wrapping_abs() as u32)
        }
    }
}

/// Shifts a `Natural` left (multiplies it by a power of 2 or divides it by a power of 2 and takes
/// the floor) in place.
///
/// Time: worst case O(`other`)
///
/// Additional memory: worst case O(`other`)
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_native;
///
/// use malachite_base::traits::One;
/// use malachite_native::natural::Natural;
///
/// fn main() {
///     let mut x = Natural::ONE;
///     x <<= 1i32;
///     x <<= 2i32;
///     x <<= 3i32;
///     x <<= 4i32;
///     assert_eq!(x.to_string(), "1024");
///
///     let mut x = Natural::from(1024u32);
///     x <<= -1;
///     x <<= -2;
///     x <<= -3;
///     x <<= -4;
///     assert_eq!(x.to_string(), "1");
/// }
/// ```
impl ShlAssign<i32> for Natural {
    fn shl_assign(&mut self, other: i32) {
        if other >= 0 {
            *self <<= other as u32;
        } else {
            *self >>= other.wrapping_abs() as u32;
        }
    }
}

/// Shifts a `Natural` left (multiplies it by a power of 2 or divides it by a power of 2 and takes
/// the floor) and rounds according to the specified rounding mode, taking the `Natural` by value.
/// Passing `RoundingMode::Floor` or `RoundingMode::Down` is equivalent to using `>>`. To test
/// whether `RoundingMode::Exact` can be passed, use
/// `other > 0 || self.is_divisible_by_power_of_two(other)`.
///
/// Time: worst case O(`other`)
///
/// Additional memory: worst case O(`other`)
///
/// # Panics
/// Panics if `other` is positive and `rm` is `RoundingMode::Exact` but `self` is not divisible by
/// 2^(`other`).
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_native;
///
/// use malachite_base::round::RoundingMode;
/// use malachite_base::traits::{ShlRound, Zero};
/// use malachite_native::natural::Natural;
///
/// fn main() {
///     assert_eq!(Natural::from(257u32).shl_round(-8i32, RoundingMode::Down).to_string(), "1");
///     assert_eq!(Natural::from(257u32).shl_round(-8i32, RoundingMode::Up).to_string(), "2");
///
///     assert_eq!(Natural::from(257u32).shl_round(-9i32, RoundingMode::Down).to_string(), "0");
///     assert_eq!(Natural::from(257u32).shl_round(-9i32, RoundingMode::Up).to_string(), "1");
///     assert_eq!(Natural::from(257u32).shl_round(-9i32, RoundingMode::Nearest).to_string(), "1");
///     assert_eq!(Natural::from(255u32).shl_round(-9i32, RoundingMode::Nearest).to_string(), "0");
///     assert_eq!(Natural::from(256u32).shl_round(-9i32, RoundingMode::Nearest).to_string(), "0");
///
///     assert_eq!(Natural::from(256u32).shl_round(-8i32, RoundingMode::Exact).to_string(), "1");
///
///     assert_eq!(Natural::ZERO.shl_round(10i32, RoundingMode::Exact).to_string(), "0");
///     assert_eq!(Natural::from(123u32).shl_round(2i32, RoundingMode::Exact).to_string(), "492");
///     assert_eq!(Natural::from(123u32).shl_round(100i32, RoundingMode::Exact).to_string(),
///         "155921023828072216384094494261248");
/// }
impl ShlRound<i32> for Natural {
    type Output = Natural;

    fn shl_round(mut self, other: i32, rm: RoundingMode) -> Natural {
        self.shl_round_assign(other, rm);
        self
    }
}

/// Shifts a `Natural` left (multiplies it by a power of 2 or divides it by a power of 2 and takes
/// the floor) and rounds according to the specified rounding mode, taking the `Natural` by
/// reference. Passing `RoundingMode::Floor` or `RoundingMode::Down` is equivalent to using `>>`. To
/// test whether `RoundingMode::Exact` can be passed, use
/// `other > 0 || self.is_divisible_by_power_of_two(other as u32)`.
///
/// Time: worst case O(`other`)
///
/// Additional memory: worst case O(`other`)
///
/// # Panics
/// Panics if `other` is positive and `rm` is `RoundingMode::Exact` but `self` is not divisible by
/// 2^(`other`).
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_native;
///
/// use malachite_base::round::RoundingMode;
/// use malachite_base::traits::{ShlRound, Zero};
/// use malachite_native::natural::Natural;
///
/// fn main() {
///     assert_eq!((&Natural::from(257u32)).shl_round(-8i32, RoundingMode::Down).to_string(), "1");
///     assert_eq!((&Natural::from(257u32)).shl_round(-8i32, RoundingMode::Up).to_string(), "2");
///
///     assert_eq!((&Natural::from(257u32)).shl_round(-9i32, RoundingMode::Down).to_string(), "0");
///     assert_eq!((&Natural::from(257u32)).shl_round(-9i32, RoundingMode::Up).to_string(), "1");
///     assert_eq!((&Natural::from(257u32)).shl_round(-9i32, RoundingMode::Nearest).to_string(),
///         "1");
///     assert_eq!((&Natural::from(255u32)).shl_round(-9i32, RoundingMode::Nearest).to_string(),
///         "0");
///     assert_eq!((&Natural::from(256u32)).shl_round(-9i32, RoundingMode::Nearest).to_string(),
///         "0");
///
///     assert_eq!((&Natural::from(256u32)).shl_round(-8i32, RoundingMode::Exact).to_string(), "1");
///
///     assert_eq!((&Natural::ZERO).shl_round(10i32, RoundingMode::Exact).to_string(), "0");
///     assert_eq!((&Natural::from(123u32)).shl_round(2i32, RoundingMode::Exact).to_string(),
///         "492");
///     assert_eq!((&Natural::from(123u32)).shl_round(100i32, RoundingMode::Exact).to_string(),
///         "155921023828072216384094494261248");
/// }
impl<'a> ShlRound<i32> for &'a Natural {
    type Output = Natural;

    fn shl_round(self, other: i32, rm: RoundingMode) -> Natural {
        if other >= 0 {
            self << (other as u32)
        } else {
            self.shr_round(other.wrapping_abs() as u32, rm)
        }
    }
}

/// Shifts a `Natural` left (multiplies it by a power of 2 or divides it by a power of 2 and takes
/// the floor) and rounds according to the specified rounding mode, in place. Passing
/// `RoundingMode::Floor` or `RoundingMode::Down` is equivalent to using `>>=`. To test whether
/// `RoundingMode::Exact` can be passed, use
/// `other > 0 || self.is_divisible_by_power_of_two(other as u32)`.
///
/// Time: worst case O(`other`)
///
/// Additional memory: worst case O(`other`)
///
/// # Panics
/// Panics if `other` is positive and `rm` is `RoundingMode::Exact` but `self` is not divisible by
/// 2^(`other`).
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_native;
///
/// use malachite_base::round::RoundingMode;
/// use malachite_base::traits::{One, ShlRoundAssign};
/// use malachite_native::natural::Natural;
///
/// fn main() {
///     let mut n = Natural::from(257u32);
///     n.shl_round_assign(-8i32, RoundingMode::Down);
///     assert_eq!(n.to_string(), "1");
///
///     let mut n = Natural::from(257u32);
///     n.shl_round_assign(-8i32, RoundingMode::Up);
///     assert_eq!(n.to_string(), "2");
///
///     let mut n = Natural::from(257u32);
///     n.shl_round_assign(-9i32, RoundingMode::Down);
///     assert_eq!(n.to_string(), "0");
///
///     let mut n = Natural::from(257u32);
///     n.shl_round_assign(-9i32, RoundingMode::Up);
///     assert_eq!(n.to_string(), "1");
///
///     let mut n = Natural::from(257u32);
///     n.shl_round_assign(-9i32, RoundingMode::Nearest);
///     assert_eq!(n.to_string(), "1");
///
///     let mut n = Natural::from(255u32);
///     n.shl_round_assign(-9i32, RoundingMode::Nearest);
///     assert_eq!(n.to_string(), "0");
///
///     let mut n = Natural::from(256u32);
///     n.shl_round_assign(-9i32, RoundingMode::Nearest);
///     assert_eq!(n.to_string(), "0");
///
///     let mut n = Natural::from(256u32);
///     n.shl_round_assign(-8i32, RoundingMode::Exact);
///     assert_eq!(n.to_string(), "1");
///
///     let mut x = Natural::ONE;
///     x.shl_round_assign(1i32, RoundingMode::Exact);
///     x.shl_round_assign(2i32, RoundingMode::Exact);
///     x.shl_round_assign(3i32, RoundingMode::Exact);
///     x.shl_round_assign(4i32, RoundingMode::Exact);
///     assert_eq!(x.to_string(), "1024");
/// }
impl ShlRoundAssign<i32> for Natural {
    fn shl_round_assign(&mut self, other: i32, rm: RoundingMode) {
        if other >= 0 {
            *self <<= other as u32;
        } else {
            self.shr_round_assign(other.wrapping_abs() as u32, rm);
        }
    }
}
