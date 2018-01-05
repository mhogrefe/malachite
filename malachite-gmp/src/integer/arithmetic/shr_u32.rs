use gmp_mpfr_sys::gmp::{self, mpz_t};
use malachite_base::round::RoundingMode;
use malachite_base::traits::{NegativeOne, ShrRound, ShrRoundAssign, Zero};
use integer::Integer::{self, Large, Small};
use std::mem;
use std::ops::{Shr, ShrAssign};

/// Shifts an `Integer` right (divides it by a power of 2 and takes the floor), taking the `Integer`
/// by value.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::Zero;
/// use malachite_gmp::integer::Integer;
///
/// fn main() {
///     assert_eq!((Integer::ZERO >> 10u32).to_string(), "0");
///     assert_eq!((Integer::from(492) >> 2u32).to_string(), "123");
///     assert_eq!((-Integer::trillion() >> 10u32).to_string(), "-976562500");
/// }
/// ```
impl Shr<u32> for Integer {
    type Output = Integer;

    fn shr(mut self, other: u32) -> Integer {
        self >>= other;
        self
    }
}

/// Shifts an `Integer` right (divides it by a power of 2 and takes the floor), taking the `Integer`
/// by reference.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::Zero;
/// use malachite_gmp::integer::Integer;
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
        if other == 0 || self == &0 {
            return self.clone();
        }
        match *self {
            Small(small) if small > 0 && other >= 31 => Integer::ZERO,
            Small(small) if small < 0 && other >= 31 => Integer::NEGATIVE_ONE,
            Small(small) => Small(small >> other),
            Large(ref large) => unsafe {
                let mut result: mpz_t = mem::uninitialized();
                gmp::mpz_init(&mut result);
                gmp::mpz_fdiv_q_2exp(&mut result, large, other.into());
                let mut result = Large(result);
                result.demote_if_small();
                result
            },
        }
    }
}

/// Shifts an `Integer` right (divides it by a power of 2 and takes the floor) in place.
///
/// # Examples
/// ```
/// use malachite_gmp::integer::Integer;
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
        if other == 0 || *self == 0 {
            return;
        }
        match *self {
            Small(ref mut small) if *small > 0 && other >= 31 => *small = 0,
            Small(ref mut small) if *small < 0 && other >= 31 => *small = -1,
            Small(ref mut small) => {
                *small >>= other;
                return;
            }
            Large(ref mut large) => unsafe {
                gmp::mpz_fdiv_q_2exp(large, large, other.into());
            },
        }
        self.demote_if_small();
    }
}

/// Shifts an `Integer` right (divides it by a power of 2) and rounds according to the specified
/// rounding mode, taking the `Integer` by value. Passing `RoundingMode::Floor` or
/// `RoundingMode::Down` is equivalent to using `>>`. To test whether `RoundingMode::Exact` can be
/// passed, use `self.is_divisible_by_power_of_two(other)`.
///
/// # Panics
/// Panics if `rm` is `RoundingMode::Exact`
/// but `self` is not divisible by 2^(`other`).
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::round::RoundingMode;
/// use malachite_base::traits::ShrRound;
/// use malachite_gmp::integer::Integer;
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
///     assert_eq!(Integer::from(0x100).shr_round(8u32, RoundingMode::Exact).to_string(), "1");
/// }
impl ShrRound<u32> for Integer {
    type Output = Integer;

    fn shr_round(mut self, other: u32, rm: RoundingMode) -> Integer {
        self.shr_round_assign(other, rm);
        self
    }
}

/// Shifts an `Integer` right (divides it by a power of 2) and rounds according to the specified
/// rounding mode, taking the `Integer` by reference. Passing `RoundingMode::Floor` or
/// `RoundingMode::Down` is equivalent to using `>>`. To test whether `RoundingMode::Exact` can be
/// passed, use `self.is_divisible_by_power_of_two(other)`.
///
/// # Panics
/// Panics if `rm` is `RoundingMode::Exact` but `self` is not divisible by 2^(`other`).
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::round::RoundingMode;
/// use malachite_base::traits::ShrRound;
/// use malachite_gmp::integer::Integer;
///
/// fn main() {
///     assert_eq!((&Integer::from(0x101)).shr_round(8u32, RoundingMode::Down).to_string(), "1");
///     assert_eq!((&Integer::from(0x101)).shr_round(8u32, RoundingMode::Up).to_string(), "2");
///
///     assert_eq!((&Integer::from(-0x101)).shr_round(9u32, RoundingMode::Down).to_string(), "0");
///     assert_eq!((&Integer::from(-0x101)).shr_round(9u32, RoundingMode::Up).to_string(), "-1");
///     assert_eq!((&Integer::from(-0x101)).shr_round(9u32,
///         RoundingMode::Nearest).to_string(), "-1");
///     assert_eq!((&Integer::from(-0xff)).shr_round(9u32, RoundingMode::Nearest).to_string(), "0");
///     assert_eq!((&Integer::from(-0x100)).shr_round(9u32,
///         RoundingMode::Nearest).to_string(), "0");
///
///     assert_eq!((&Integer::from(0x100)).shr_round(8u32, RoundingMode::Exact).to_string(), "1");
/// }
impl<'a> ShrRound<u32> for &'a Integer {
    type Output = Integer;

    fn shr_round(self, other: u32, rm: RoundingMode) -> Integer {
        if other == 0 || *self == 0 {
            self.clone()
        } else if *self > 0 {
            self.to_natural()
                .unwrap()
                .shr_round(other, rm)
                .into_integer()
        } else {
            -self.natural_abs_ref().shr_round(other, -rm)
        }
    }
}

/// Shifts an `Integer` right (divides it by a power of 2) and rounds according to the specified
/// rounding mode, in place. Passing `RoundingMode::Floor` or `RoundingMode::Down` is equivalent to
/// using `>>=`. To test whether `RoundingMode::Exact` can be passed, use
/// `self.is_divisible_by_power_of_two(other)`.
///
/// # Panics
/// Panics if `rm` is `RoundingMode::Exact` but `self` is not divisible by 2^(`other`).
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::round::RoundingMode;
/// use malachite_base::traits::ShrRoundAssign;
/// use malachite_gmp::integer::Integer;
///
/// fn main() {
///     let mut n = Integer::from(0x101);
///     n.shr_round_assign(8u32, RoundingMode::Down);
///     assert_eq!(n.to_string(), "1");
///
///     let mut n = Integer::from(0x101);
///     n.shr_round_assign(8u32, RoundingMode::Up);
///     assert_eq!(n.to_string(), "2");
///
///     let mut n = Integer::from(-0x101);
///     n.shr_round_assign(9u32, RoundingMode::Down);
///     assert_eq!(n.to_string(), "0");
///
///     let mut n = Integer::from(-0x101);
///     n.shr_round_assign(9u32, RoundingMode::Up);
///     assert_eq!(n.to_string(), "-1");
///
///     let mut n = Integer::from(-0x101);
///     n.shr_round_assign(9u32, RoundingMode::Nearest);
///     assert_eq!(n.to_string(), "-1");
///
///     let mut n = Integer::from(-0xff);
///     n.shr_round_assign(9u32, RoundingMode::Nearest);
///     assert_eq!(n.to_string(), "0");
///
///     let mut n = Integer::from(-0x100);
///     n.shr_round_assign(9u32, RoundingMode::Nearest);
///     assert_eq!(n.to_string(), "0");
///
///     let mut n = Integer::from(0x100);
///     n.shr_round_assign(8u32, RoundingMode::Exact);
///     assert_eq!(n.to_string(), "1");
/// }
impl ShrRoundAssign<u32> for Integer {
    fn shr_round_assign(&mut self, other: u32, rm: RoundingMode) {
        //TODO don't waste memory
        if other == 0 || *self == 0 {
        } else if *self > 0 {
            let mut n = self.to_natural().unwrap();
            n.shr_round_assign(other, rm);
            *self = n.into_integer()
        } else {
            let mut n = self.natural_abs_ref();
            n.shr_round_assign(other, -rm);
            *self = -n;
        }
    }
}
