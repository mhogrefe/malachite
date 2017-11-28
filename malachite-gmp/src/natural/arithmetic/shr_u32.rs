use gmp_mpfr_sys::gmp::{self, mpz_t};
use malachite_base::round::RoundingMode;
use malachite_base::traits::{ShrRound, ShrRoundAssign, Zero};
use natural::Natural::{self, Large, Small};
use std::mem;
use std::ops::{Shr, ShrAssign};

/// Shifts a `Natural` right (divides it by a power of 2 and takes the floor), taking the `Natural`
/// by value.
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
///     assert_eq!((Natural::ZERO >> 10).to_string(), "0");
///     assert_eq!((Natural::from(492u32) >> 2).to_string(), "123");
///     assert_eq!((Natural::from_str("1000000000000").unwrap() >> 10).to_string(), "976562500");
/// }
/// ```
impl Shr<u32> for Natural {
    type Output = Natural;

    fn shr(mut self, other: u32) -> Natural {
        self >>= other;
        self
    }
}

/// Shifts a `Natural` right (divides it by a power of 2 and takes the floor), taking the `Natural`
/// by reference.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
/// use std::str::FromStr;
///
/// use malachite_base::traits::Zero;
/// use malachite_gmp::natural::Natural;
///
/// fn main() {
///     assert_eq!((&Natural::ZERO >> 10).to_string(), "0");
///     assert_eq!((&Natural::from(492u32) >> 2).to_string(), "123");
///     assert_eq!((&Natural::from_str("1000000000000").unwrap() >> 10).to_string(), "976562500");
/// }
/// ```
impl<'a> Shr<u32> for &'a Natural {
    type Output = Natural;

    fn shr(self, other: u32) -> Natural {
        if other == 0 || self == &0 {
            return self.clone();
        }
        match *self {
            Small(_) if other >= 32 => Natural::ZERO,
            Small(small) => Small(small >> other),
            Large(ref large) => unsafe {
                let mut result: mpz_t = mem::uninitialized();
                gmp::mpz_init(&mut result);
                gmp::mpz_tdiv_q_2exp(&mut result, large, other.into());
                let mut result = Large(result);
                result.demote_if_small();
                result
            },
        }
    }
}

/// Shifts a `Natural` right (divides it by a power of 2 and takes the floor) in place.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
///
/// let mut x = Natural::from(1024u32);
/// x >>= 1;
/// x >>= 2;
/// x >>= 3;
/// x >>= 4;
/// assert_eq!(x.to_string(), "1");
/// ```
impl ShrAssign<u32> for Natural {
    fn shr_assign(&mut self, other: u32) {
        if other == 0 || *self == 0 {
            return;
        }
        match self {
            &mut Small(ref mut small) if other >= 32 => *small = 0,
            &mut Small(ref mut small) => {
                *small >>= other;
                return;
            }
            &mut Large(ref mut large) => unsafe {
                gmp::mpz_tdiv_q_2exp(large, large, other.into());
            },
        }
        self.demote_if_small();
    }
}

/// Shifts a `Natural` right (divides it by a power of 2) and rounds according to the specified
/// rounding mode, taking the `Natural` by value. Passing `RoundingMode::Floor` or
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
/// use malachite_gmp::natural::Natural;
///
/// fn main() {
///     assert_eq!(Natural::from(257u32).shr_round(8, RoundingMode::Down).to_string(), "1");
///     assert_eq!(Natural::from(257u32).shr_round(8, RoundingMode::Up).to_string(), "2");
///
///     assert_eq!(Natural::from(257u32).shr_round(9, RoundingMode::Down).to_string(), "0");
///     assert_eq!(Natural::from(257u32).shr_round(9, RoundingMode::Up).to_string(), "1");
///     assert_eq!(Natural::from(257u32).shr_round(9, RoundingMode::Nearest).to_string(), "1");
///     assert_eq!(Natural::from(255u32).shr_round(9, RoundingMode::Nearest).to_string(), "0");
///     assert_eq!(Natural::from(256u32).shr_round(9, RoundingMode::Nearest).to_string(), "0");
///
///     assert_eq!(Natural::from(256u32).shr_round(8, RoundingMode::Exact).to_string(), "1");
/// }
impl ShrRound<u32> for Natural {
    type Output = Natural;

    fn shr_round(mut self, other: u32, rm: RoundingMode) -> Natural {
        self.shr_round_assign(other, rm);
        self
    }
}

/// Shifts a `Natural` right (divides it by a power of 2) and rounds according to the specified
/// rounding mode, taking the `Natural` by reference. Passing `RoundingMode::Floor` or
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
/// use malachite_gmp::natural::Natural;
///
/// fn main() {
///     assert_eq!((&Natural::from(257u32)).shr_round(8, RoundingMode::Down).to_string(), "1");
///     assert_eq!((&Natural::from(257u32)).shr_round(8, RoundingMode::Up).to_string(), "2");
///
///     assert_eq!((&Natural::from(257u32)).shr_round(9, RoundingMode::Down).to_string(), "0");
///     assert_eq!((&Natural::from(257u32)).shr_round(9, RoundingMode::Up).to_string(), "1");
///     assert_eq!((&Natural::from(257u32)).shr_round(9, RoundingMode::Nearest).to_string(), "1");
///     assert_eq!((&Natural::from(255u32)).shr_round(9, RoundingMode::Nearest).to_string(), "0");
///     assert_eq!((&Natural::from(256u32)).shr_round(9, RoundingMode::Nearest).to_string(), "0");
///
///     assert_eq!((&Natural::from(256u32)).shr_round(8, RoundingMode::Exact).to_string(), "1");
/// }
impl<'a> ShrRound<u32> for &'a Natural {
    type Output = Natural;

    fn shr_round(self, other: u32, rm: RoundingMode) -> Natural {
        if other == 0 || *self == 0 {
            return self.clone();
        }
        let opt_result = match self {
            &Small(ref small) => {
                return Small(match rm {
                    RoundingMode::Down | RoundingMode::Floor if other >= 32 => 0,
                    RoundingMode::Down | RoundingMode::Floor => *small >> other,
                    RoundingMode::Up | RoundingMode::Ceiling if other >= 32 => 1,
                    RoundingMode::Up | RoundingMode::Ceiling => {
                        let shifted = *small >> other;
                        if shifted << other == *small {
                            shifted
                        } else {
                            shifted + 1
                        }
                    }
                    RoundingMode::Nearest if other == 32 && *small > (1u32 << 31) => 1,
                    RoundingMode::Nearest if other >= 32 => 0,
                    RoundingMode::Nearest => {
                        let mostly_shifted = small >> (other - 1);
                        if (mostly_shifted & 1) == 0 {
                            // round down
                            mostly_shifted >> 1
                        } else if mostly_shifted << (other - 1) != *small {
                            // round up
                            (mostly_shifted >> 1) + 1
                        } else {
                            // result is half-integer; round to even
                            let shifted = mostly_shifted >> 1;
                            if (shifted & 1) == 0 {
                                shifted
                            } else {
                                shifted + 1
                            }
                        }
                    }
                    RoundingMode::Exact if other >= 32 => {
                        panic!("Right shift is not exact: {} >> {}", *small, other);
                    }
                    RoundingMode::Exact => {
                        let shifted = *small >> other;
                        if shifted << other != *small {
                            panic!("Right shift is not exact: {} >> {}", *small, other);
                        }
                        shifted
                    }
                });
            }
            &Large(ref large) => unsafe {
                match rm {
                    RoundingMode::Down | RoundingMode::Floor => {
                        let mut result: mpz_t = mem::uninitialized();
                        gmp::mpz_init(&mut result);
                        gmp::mpz_tdiv_q_2exp(&mut result, large, other.into());
                        Some(result)
                    }
                    RoundingMode::Up | RoundingMode::Ceiling => {
                        let mut result: mpz_t = mem::uninitialized();
                        gmp::mpz_init(&mut result);
                        gmp::mpz_cdiv_q_2exp(&mut result, large, other.into());
                        Some(result)
                    }
                    _ => None,
                }
            },
        };
        let result = match opt_result {
            Some(result) => result,
            None => unsafe {
                let mut result: mpz_t = mem::uninitialized();
                gmp::mpz_init(&mut result);
                match rm {
                    RoundingMode::Nearest => {
                        if !self.get_bit((other - 1).into()) {
                            // round down
                            if let &Large(ref large) = self {
                                gmp::mpz_tdiv_q_2exp(&mut result, large, other.into());
                            }
                        } else if !self.divisible_by_power_of_2((other - 1).into()) {
                            // round up
                            if let &Large(ref large) = self {
                                gmp::mpz_cdiv_q_2exp(&mut result, large, other.into());
                            }
                        } else {
                            // result is half-integer; round to even
                            if let &Large(ref large) = self {
                                gmp::mpz_tdiv_q_2exp(&mut result, large, other.into());
                            }
                            let mut result = Large(result);
                            result.demote_if_small();
                            if result.is_odd() {
                                result += 1;
                            }
                            return result;
                        }
                    }
                    RoundingMode::Exact => {
                        if !self.divisible_by_power_of_2(other.into()) {
                            panic!("Right shift is not exact: {} >> {}", self, other);
                        }
                        if let &Large(ref large) = self {
                            gmp::mpz_tdiv_q_2exp(&mut result, large, other.into());
                        }
                    }
                    _ => unreachable!(),
                }
                result
            },
        };
        let mut result = Large(result);
        result.demote_if_small();
        result
    }
}

/// Shifts a `Natural` right (divides it by a power of 2) and rounds according to the specified
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
/// use malachite_gmp::natural::Natural;
///
/// fn main() {
///     let mut n = Natural::from(257u32);
///     n.shr_round_assign(8, RoundingMode::Down);
///     assert_eq!(n.to_string(), "1");
///
///     let mut n = Natural::from(257u32);
///     n.shr_round_assign(8, RoundingMode::Up);
///     assert_eq!(n.to_string(), "2");
///
///     let mut n = Natural::from(257u32);
///     n.shr_round_assign(9, RoundingMode::Down);
///     assert_eq!(n.to_string(), "0");
///
///     let mut n = Natural::from(257u32);
///     n.shr_round_assign(9, RoundingMode::Up);
///     assert_eq!(n.to_string(), "1");
///
///     let mut n = Natural::from(257u32);
///     n.shr_round_assign(9, RoundingMode::Nearest);
///     assert_eq!(n.to_string(), "1");
///
///     let mut n = Natural::from(255u32);
///     n.shr_round_assign(9, RoundingMode::Nearest);
///     assert_eq!(n.to_string(), "0");
///
///     let mut n = Natural::from(256u32);
///     n.shr_round_assign(9, RoundingMode::Nearest);
///     assert_eq!(n.to_string(), "0");
///
///     let mut n = Natural::from(256u32);
///     n.shr_round_assign(8, RoundingMode::Exact);
///     assert_eq!(n.to_string(), "1");
/// }
impl ShrRoundAssign<u32> for Natural {
    fn shr_round_assign(&mut self, other: u32, rm: RoundingMode) {
        if other == 0 || *self == 0 {
            return;
        }
        let needs_more_work = match self {
            &mut Small(ref mut small) => {
                match rm {
                    RoundingMode::Down | RoundingMode::Floor if other >= 32 => *small = 0,
                    RoundingMode::Down | RoundingMode::Floor => *small >>= other,
                    RoundingMode::Up | RoundingMode::Ceiling if other >= 32 => *small = 1,
                    RoundingMode::Up | RoundingMode::Ceiling => {
                        let original = *small;
                        *small >>= other;
                        if *small << other != original {
                            *small += 1
                        }
                    }
                    RoundingMode::Nearest if other == 32 && *small > (1u32 << 31) => *small = 1,
                    RoundingMode::Nearest if other >= 32 => *small = 0,
                    RoundingMode::Nearest => {
                        let original = *small;
                        *small >>= other - 1;
                        if (*small & 1) == 0 {
                            // round down
                            *small >>= 1;
                        } else if *small << (other - 1) != original {
                            // round up
                            *small >>= 1;
                            *small += 1;
                        } else {
                            // result is half-integer; round to even
                            *small >>= 1;
                            if (*small & 1) != 0 {
                                *small += 1;
                            }
                        }
                    }
                    RoundingMode::Exact if other >= 32 => {
                        panic!("Right shift is not exact: {} >>= {}", *small, other);
                    }
                    RoundingMode::Exact => {
                        let original = *small;
                        *small >>= other;
                        if *small << other != original {
                            panic!("Right shift is not exact: {} >>= {}", original, other);
                        }
                    }
                }
                return;
            }
            &mut Large(ref mut large) => unsafe {
                match rm {
                    RoundingMode::Down | RoundingMode::Floor => {
                        gmp::mpz_tdiv_q_2exp(large, large, other.into());
                        false
                    }
                    RoundingMode::Up | RoundingMode::Ceiling => {
                        gmp::mpz_cdiv_q_2exp(large, large, other.into());
                        false
                    }
                    _ => true,
                }
            },
        };
        if needs_more_work {
            match rm {
                RoundingMode::Nearest => {
                    if !self.get_bit((other - 1).into()) {
                        // round down
                        if let &mut Large(ref mut large) = self {
                            unsafe {
                                gmp::mpz_tdiv_q_2exp(large, large, other.into());
                            }
                        }
                    } else if !self.divisible_by_power_of_2((other - 1).into()) {
                        // round up
                        if let &mut Large(ref mut large) = self {
                            unsafe {
                                gmp::mpz_cdiv_q_2exp(large, large, other.into());
                            }
                        }
                    } else {
                        // result is half-integer; round to even
                        if let &mut Large(ref mut large) = self {
                            unsafe {
                                gmp::mpz_tdiv_q_2exp(large, large, other.into());
                            }
                        }
                        self.demote_if_small();
                        if self.is_odd() {
                            *self += 1;
                        }
                        return;
                    }
                }
                RoundingMode::Exact => {
                    if !self.divisible_by_power_of_2(other.into()) {
                        panic!("Right shift is not exact: {} >>= {}", self, other);
                    }
                    if let &mut Large(ref mut large) = self {
                        unsafe {
                            gmp::mpz_tdiv_q_2exp(large, large, other.into());
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
        self.demote_if_small();
    }
}
