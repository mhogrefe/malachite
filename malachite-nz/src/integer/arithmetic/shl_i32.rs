use integer::Integer;
use malachite_base::num::{ShlRound, ShlRoundAssign, ShrRound, ShrRoundAssign, UnsignedAbs};
use malachite_base::round::RoundingMode;
use std::ops::{Shl, ShlAssign};

/// Shifts an `Integer` left (multiplies it by a power of 2 or divides it by a power of 2 and takes
/// the floor), taking the `Integer` by value.
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
///     assert_eq!((Integer::ZERO << 10i32).to_string(), "0");
///     assert_eq!((Integer::from(123) << 2i32).to_string(), "492");
///     assert_eq!((Integer::from(123) << 100i32).to_string(),
///         "155921023828072216384094494261248");
///     assert_eq!((Integer::from(-123) << 2i32).to_string(), "-492");
///     assert_eq!((Integer::from(-123) << 100i32).to_string(),
///         "-155921023828072216384094494261248");
///     assert_eq!((Integer::ZERO << -10i32).to_string(), "0");
///     assert_eq!((Integer::from(492) << -2i32).to_string(), "123");
///     assert_eq!((-Integer::trillion() << -10i32).to_string(), "-976562500");
/// }
/// ```
impl Shl<i32> for Integer {
    type Output = Integer;

    fn shl(mut self, other: i32) -> Integer {
        self <<= other;
        self
    }
}

/// Shifts an `Integer` left (multiplies it by a power of 2 or divides it by a power of 2 and takes
/// the floor), taking the `Integer` by reference.
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
///     assert_eq!((&Integer::ZERO << 10i32).to_string(), "0");
///     assert_eq!((&Integer::from(123) << 2i32).to_string(), "492");
///     assert_eq!((&Integer::from(123) << 100i32).to_string(),
///         "155921023828072216384094494261248");
///     assert_eq!((&Integer::from(-123) << 2i32).to_string(), "-492");
///     assert_eq!((&Integer::from(-123) << 100i32).to_string(),
///         "-155921023828072216384094494261248");
///     assert_eq!((&Integer::ZERO << -10i32).to_string(), "0");
///     assert_eq!((&Integer::from(492) << -2i32).to_string(), "123");
///     assert_eq!((&(-Integer::trillion()) << -10i32).to_string(), "-976562500");
/// }
/// ```
impl<'a> Shl<i32> for &'a Integer {
    type Output = Integer;

    fn shl(self, other: i32) -> Integer {
        if other >= 0 {
            self << (other as u32)
        } else {
            self >> other.unsigned_abs()
        }
    }
}

/// Shifts an `Integer` left (multiplies it by a power of 2 or divides it by a power of 2 and takes
/// the floor) in place.
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
/// use malachite_base::num::{NegativeOne, One};
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::ONE;
///     x <<= 1i32;
///     x <<= 2i32;
///     x <<= 3i32;
///     x <<= 4i32;
///     assert_eq!(x.to_string(), "1024");
///     let mut x = Integer::NEGATIVE_ONE;
///     x <<= 1i32;
///     x <<= 2i32;
///     x <<= 3i32;
///     x <<= 4i32;
///     assert_eq!(x.to_string(), "-1024");
///
///     let mut x = Integer::from(1024);
///     x <<= -1;
///     x <<= -2;
///     x <<= -3;
///     x <<= -4;
///     assert_eq!(x.to_string(), "1");
/// }
/// ```
impl ShlAssign<i32> for Integer {
    fn shl_assign(&mut self, other: i32) {
        if other >= 0 {
            *self <<= other as u32;
        } else {
            *self >>= other.unsigned_abs();
        }
    }
}

/// Shifts an `Integer` left (multiplies it by a power of 2 or divides it by a power of 2 and takes
/// the floor) and rounds according to the specified rounding mode, taking the `Integer` by value.
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
/// 2<pow>`other`</pow>.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::round::RoundingMode;
/// use malachite_base::num::{ShlRound, Zero};
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!(Integer::from(0x101).shl_round(-8i32, RoundingMode::Down).to_string(), "1");
///     assert_eq!(Integer::from(0x101).shl_round(-8i32, RoundingMode::Up).to_string(), "2");
///
///     assert_eq!(Integer::from(-0x101).shl_round(-9i32, RoundingMode::Down).to_string(), "0");
///     assert_eq!(Integer::from(-0x101).shl_round(-9i32, RoundingMode::Up).to_string(), "-1");
///     assert_eq!(Integer::from(-0x101).shl_round(-9i32, RoundingMode::Nearest).to_string(), "-1");
///     assert_eq!(Integer::from(-0xff).shl_round(-9i32, RoundingMode::Nearest).to_string(), "0");
///     assert_eq!(Integer::from(-0x100).shl_round(-9i32, RoundingMode::Nearest).to_string(), "0");
///
///     assert_eq!(Integer::from(0x100).shl_round(-8i32, RoundingMode::Exact).to_string(), "1");
///
///     assert_eq!(Integer::ZERO.shl_round(10i32, RoundingMode::Exact).to_string(), "0");
///     assert_eq!(Integer::from(123u32).shl_round(2i32, RoundingMode::Exact).to_string(), "492");
///     assert_eq!(Integer::from(123u32).shl_round(100i32, RoundingMode::Exact).to_string(),
///         "155921023828072216384094494261248");
/// }
impl ShlRound<i32> for Integer {
    type Output = Integer;

    fn shl_round(mut self, other: i32, rm: RoundingMode) -> Integer {
        self.shl_round_assign(other, rm);
        self
    }
}

/// Shifts an `Integer` left (multiplies it by a power of 2 or divides it by a power of 2 and takes
/// the floor) and rounds according to the specified rounding mode, taking the `Integer` by
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
/// 2<pow>`other`</pow>.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::round::RoundingMode;
/// use malachite_base::num::{ShlRound, Zero};
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!((&Integer::from(0x101)).shl_round(-8i32, RoundingMode::Down).to_string(), "1");
///     assert_eq!((&Integer::from(0x101)).shl_round(-8i32, RoundingMode::Up).to_string(), "2");
///
///     assert_eq!((&Integer::from(-0x101)).shl_round(-9i32, RoundingMode::Down).to_string(), "0");
///     assert_eq!((&Integer::from(-0x101)).shl_round(-9i32, RoundingMode::Up).to_string(), "-1");
///     assert_eq!((&Integer::from(-0x101)).shl_round(-9i32, RoundingMode::Nearest).to_string(),
///         "-1");
///     assert_eq!((&Integer::from(-0xff)).shl_round(-9i32, RoundingMode::Nearest).to_string(),
///         "0");
///     assert_eq!((&Integer::from(-0x100)).shl_round(-9i32, RoundingMode::Nearest).to_string(),
///         "0");
///
///     assert_eq!((&Integer::from(0x100)).shl_round(-8i32, RoundingMode::Exact).to_string(), "1");
///
///     assert_eq!((&Integer::ZERO).shl_round(10i32, RoundingMode::Exact).to_string(), "0");
///     assert_eq!((&Integer::from(123u32)).shl_round(2i32, RoundingMode::Exact).to_string(),
///         "492");
///     assert_eq!((&Integer::from(123u32)).shl_round(100i32, RoundingMode::Exact).to_string(),
///         "155921023828072216384094494261248");
/// }
impl<'a> ShlRound<i32> for &'a Integer {
    type Output = Integer;

    fn shl_round(self, other: i32, rm: RoundingMode) -> Integer {
        if other >= 0 {
            self << (other as u32)
        } else {
            self.shr_round(other.unsigned_abs(), rm)
        }
    }
}

/// Shifts an `Integer` left (multiplies it by a power of 2 or divides it by a power of 2 and takes
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
/// 2<pow>`other`</pow>.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::round::RoundingMode;
/// use malachite_base::num::{One, ShlRoundAssign};
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     let mut n = Integer::from(0x101);
///     n.shl_round_assign(-8i32, RoundingMode::Down);
///     assert_eq!(n.to_string(), "1");
///
///     let mut n = Integer::from(0x101);
///     n.shl_round_assign(-8i32, RoundingMode::Up);
///     assert_eq!(n.to_string(), "2");
///
///     let mut n = Integer::from(-0x101);
///     n.shl_round_assign(-9i32, RoundingMode::Down);
///     assert_eq!(n.to_string(), "0");
///
///     let mut n = Integer::from(-0x101);
///     n.shl_round_assign(-9i32, RoundingMode::Up);
///     assert_eq!(n.to_string(), "-1");
///
///     let mut n = Integer::from(-0x101);
///     n.shl_round_assign(-9i32, RoundingMode::Nearest);
///     assert_eq!(n.to_string(), "-1");
///
///     let mut n = Integer::from(-0xff);
///     n.shl_round_assign(-9i32, RoundingMode::Nearest);
///     assert_eq!(n.to_string(), "0");
///
///     let mut n = Integer::from(-0x100);
///     n.shl_round_assign(-9i32, RoundingMode::Nearest);
///     assert_eq!(n.to_string(), "0");
///
///     let mut n = Integer::from(0x100);
///     n.shl_round_assign(-8i32, RoundingMode::Exact);
///     assert_eq!(n.to_string(), "1");
///
///     let mut x = Integer::ONE;
///     x.shl_round_assign(1i32, RoundingMode::Exact);
///     x.shl_round_assign(2i32, RoundingMode::Exact);
///     x.shl_round_assign(3i32, RoundingMode::Exact);
///     x.shl_round_assign(4i32, RoundingMode::Exact);
///     assert_eq!(x.to_string(), "1024");
/// }
impl ShlRoundAssign<i32> for Integer {
    fn shl_round_assign(&mut self, other: i32, rm: RoundingMode) {
        if other >= 0 {
            *self <<= other as u32;
        } else {
            self.shr_round_assign(other.unsigned_abs() as u32, rm);
        }
    }
}
