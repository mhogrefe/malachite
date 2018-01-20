use malachite_base::round::RoundingMode;
use malachite_base::traits::{ShrRound, ShrRoundAssign, Zero};
use integer::Integer;
use std::ops::{Shr, ShrAssign};

/// Shifts a `Integer` right (divides it by a power of 2 and takes the floor), taking the `Integer`
/// by value.
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
/// use malachite_base::traits::Zero;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!((Integer::ZERO >> 10u32).to_string(), "0");
///     assert_eq!((Integer::from(492) >> 2u32).to_string(), "123");
///     assert_eq!((-Integer::trillion() >> 10u32).to_string(),
///         "-976562500");
/// }
/// ```
impl Shr<u32> for Integer {
    type Output = Integer;

    fn shr(mut self, other: u32) -> Integer {
        self >>= other;
        self
    }
}

/// Shifts a `Integer` right (divides it by a power of 2 and takes the floor), taking the `Integer`
/// by reference.
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
/// use malachite_base::traits::Zero;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!((&Integer::ZERO >> 10u32).to_string(), "0");
///     assert_eq!((&Integer::from(492) >> 2u32).to_string(), "123");
///     assert_eq!((&(-Integer::trillion()) >> 10u32).to_string(), "-976562500");
/// }
/// ```
impl<'a> Shr<u32> for &'a Integer {
    type Output = Integer;

    fn shr(self, other: u32) -> Integer {
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
/// x >>= 1;
/// x >>= 2;
/// x >>= 3;
/// x >>= 4;
/// assert_eq!(x.to_string(), "1");
/// ```
impl ShrAssign<u32> for Integer {
    fn shr_assign(&mut self, other: u32) {
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

/// Shifts a `Integer` right (divides it by a power of 2) and rounds according to the specified
/// rounding mode, taking the `Integer` by value. Passing `RoundingMode::Floor` or
/// `RoundingMode::Down` is equivalent to using `>>`. To test whether `RoundingMode::Exact` can be
/// passed, use `self.is_divisible_by_power_of_two(other)`.
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
/// use malachite_base::traits::ShrRound;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!(Integer::from(0x101).shr_round(8u32, RoundingMode::Down).to_string(), "1");
///     assert_eq!(Integer::from(0x101).shr_round(8u32, RoundingMode::Up).to_string(), "2");
///
///     assert_eq!(Integer::from(-0x101).shr_round(9u32, RoundingMode::Down).to_string(), "0");
///     assert_eq!(Integer::from(-0x101).shr_round(9u32, RoundingMode::Up).to_string(), "-1");
///     assert_eq!(Integer::from(-0x101).shr_round(9u32, RoundingMode::Nearest).to_string(), "-1");
///     assert_eq!(Integer::from(-0xff).shr_round(9u32, RoundingMode::Nearest).to_string(), "0");
///     assert_eq!(Integer::from(-0x100).shr_round(9u32, RoundingMode::Nearest).to_string(), "0");
///
///     assert_eq!(Integer::from(0x100u32).shr_round(8u32, RoundingMode::Exact).to_string(), "1");
/// }
impl ShrRound<u32> for Integer {
    type Output = Integer;

    fn shr_round(mut self, other: u32, rm: RoundingMode) -> Integer {
        self.shr_round_assign(other, rm);
        self
    }
}

/// Shifts a `Integer` right (divides it by a power of 2) and rounds according to the specified
/// rounding mode, taking the `Integer` by reference. Passing `RoundingMode::Floor` or
/// `RoundingMode::Down` is equivalent to using `>>`. To test whether `RoundingMode::Exact` can be
/// passed, use `self.is_divisible_by_power_of_two(other)`.
///
/// Time: worst case O(`other`)
///
/// Additional memory: worst case O(`other`)
///
/// # Panics
/// Panics if `rm` is `RoundingMode::Exact` but `self` is not divisible by 2<pow>`other`</pow>.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::round::RoundingMode;
/// use malachite_base::traits::ShrRound;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!((&Integer::from(0x101)).shr_round(8u32, RoundingMode::Down).to_string(), "1");
///     assert_eq!((&Integer::from(0x101)).shr_round(8u32, RoundingMode::Up).to_string(), "2");
///
///     assert_eq!((&Integer::from(-0x101)).shr_round(9u32, RoundingMode::Down).to_string(), "0");
///     assert_eq!((&Integer::from(-0x101)).shr_round(9u32, RoundingMode::Up).to_string(), "-1");
///     assert_eq!((&Integer::from(-0x101)).shr_round(9u32, RoundingMode::Nearest).to_string(),
///         "-1");
///     assert_eq!((&Integer::from(-0xff)).shr_round(9u32, RoundingMode::Nearest).to_string(), "0");
///     assert_eq!((&Integer::from(-0x100)).shr_round(9u32, RoundingMode::Nearest).to_string(),
///         "0");
///
///     assert_eq!((&Integer::from(0x100)).shr_round(8u32, RoundingMode::Exact).to_string(), "1");
/// }
impl<'a> ShrRound<u32> for &'a Integer {
    type Output = Integer;

    fn shr_round(self, other: u32, rm: RoundingMode) -> Integer {
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

/// Shifts a `Integer` right (divides it by a power of 2) and rounds according to the specified
/// rounding mode, in place. Passing `RoundingMode::Floor` or `RoundingMode::Down` is equivalent to
/// using `>>=`. To test whether `RoundingMode::Exact` can be passed, use
/// `self.is_divisible_by_power_of_two(other)`.
///
/// Time: worst case O(`other`)
///
/// Additional memory: worst case O(1)
///
/// # Panics
/// Panics if `rm` is `RoundingMode::Exact` but `self` is not divisible by 2<pow>`other`</pow>.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::round::RoundingMode;
/// use malachite_base::traits::ShrRoundAssign;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     let mut n = Integer::from(0x101);
///     n.shr_round_assign(8, RoundingMode::Down);
///     assert_eq!(n.to_string(), "1");
///
///     let mut n = Integer::from(0x101);
///     n.shr_round_assign(8, RoundingMode::Up);
///     assert_eq!(n.to_string(), "2");
///
///     let mut n = Integer::from(-0x101);
///     n.shr_round_assign(9, RoundingMode::Down);
///     assert_eq!(n.to_string(), "0");
///
///     let mut n = Integer::from(-0x101);
///     n.shr_round_assign(9, RoundingMode::Up);
///     assert_eq!(n.to_string(), "-1");
///
///     let mut n = Integer::from(-0x101);
///     n.shr_round_assign(9, RoundingMode::Nearest);
///     assert_eq!(n.to_string(), "-1");
///
///     let mut n = Integer::from(-0xff);
///     n.shr_round_assign(9, RoundingMode::Nearest);
///     assert_eq!(n.to_string(), "0");
///
///     let mut n = Integer::from(-0x100);
///     n.shr_round_assign(9, RoundingMode::Nearest);
///     assert_eq!(n.to_string(), "0");
///
///     let mut n = Integer::from(0x100);
///     n.shr_round_assign(8, RoundingMode::Exact);
///     assert_eq!(n.to_string(), "1");
/// }
impl ShrRoundAssign<u32> for Integer {
    fn shr_round_assign(&mut self, other: u32, rm: RoundingMode) {
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
