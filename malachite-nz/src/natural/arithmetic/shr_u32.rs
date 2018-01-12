use malachite_base::round::RoundingMode;
use malachite_base::traits::{One, ShrRound, ShrRoundAssign, Zero};
use natural::{delete_left, mpn_zero_p, LIMB_BITS, LIMB_BITS_MASK, LOG_LIMB_BITS};
use natural::Natural::{self, Large, Small};
use std::ops::{Shr, ShrAssign};

// Shift u right by bits, and write the result to r. The bits shifted out at the right are returned
// in the most significant bits of the return value (the rest of the return value is zero).
// u.len() > 0, r.len() >= u.len(), 1 <= bits < LIMB_BITS
pub fn mpn_rshift(r: &mut [u32], u: &[u32], bits: u32) -> u32 {
    let u_len = u.len();
    assert!(u_len > 0);
    assert!(bits > 0);
    assert!(bits < LIMB_BITS);
    let cobits = LIMB_BITS - bits;
    let mut high_limb = u[0];
    let remaining_bits = high_limb << cobits;
    let mut low_limb = high_limb >> bits;
    for i in 1..u_len {
        high_limb = u[i];
        r[i - 1] = low_limb | (high_limb << cobits);
        low_limb = high_limb >> bits;
    }
    *r.last_mut().unwrap() = low_limb;
    remaining_bits
}

// Shift u right by bits, and write the result to u. The bits shifted out at the right are returned
// in the most significant bits of the return value (the rest of the return value is zero).
// u.len() > 0, 1 <= bits < LIMB_BITS
pub fn mpn_rshift_in_place(u: &mut [u32], bits: u32) -> u32 {
    assert!(!u.is_empty());
    assert!(bits > 0);
    assert!(bits < LIMB_BITS);
    let cobits = LIMB_BITS - bits;
    let mut high_limb = u[0];
    let remaining_bits = high_limb << cobits;
    let mut low_limb = high_limb >> bits;
    for i in 1..u.len() {
        high_limb = u[i];
        u[i - 1] = low_limb | (high_limb << cobits);
        low_limb = high_limb >> bits;
    }
    *u.last_mut().unwrap() = low_limb;
    remaining_bits
}

/// Shifts a `Natural` right (divides it by a power of 2 and takes the floor), taking the `Natural`
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
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((Natural::ZERO >> 10u32).to_string(), "0");
///     assert_eq!((Natural::from(492u32) >> 2u32).to_string(), "123");
///     assert_eq!((Natural::trillion() >> 10u32).to_string(), "976562500");
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
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((&Natural::ZERO >> 10u32).to_string(), "0");
///     assert_eq!((&Natural::from(492u32) >> 2u32).to_string(), "123");
///     assert_eq!((&Natural::trillion() >> 10u32).to_string(), "976562500");
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
            Large(ref limbs) => {
                let limbs_to_delete = (other >> LOG_LIMB_BITS) as usize;
                if limbs_to_delete >= limbs.len() {
                    Natural::ZERO
                } else {
                    let small_shift = other & LIMB_BITS_MASK;
                    let mut result = limbs[limbs_to_delete..].to_vec();
                    if small_shift != 0 {
                        mpn_rshift_in_place(&mut result, small_shift);
                    }
                    let mut result = Large(result);
                    result.trim();
                    result
                }
            }
        }
    }
}

fn shr_helper(limbs: &mut Vec<u32>, other: u32) {
    let limbs_to_delete = (other >> LOG_LIMB_BITS) as usize;
    if limbs_to_delete >= limbs.len() {
        limbs.clear();
    } else {
        let small_shift = other & LIMB_BITS_MASK;
        delete_left(limbs, limbs_to_delete);
        if small_shift != 0 {
            mpn_rshift_in_place(limbs, small_shift);
        }
    }
}

/// Shifts a `Natural` right (divides it by a power of 2 and takes the floor) in place.
///
/// Time: worst case O(`other`)
///
/// Additional memory: worst case O(1)
///
/// # Examples
/// ```
/// use malachite_nz::natural::Natural;
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
        match *self {
            Small(ref mut small) if other >= 32 => *small = 0,
            Small(ref mut small) => {
                *small >>= other;
                return;
            }
            Large(ref mut limbs) => {
                shr_helper(limbs, other);
            }
        }
        self.trim();
    }
}

/// Shifts a `Natural` right (divides it by a power of 2) and rounds according to the specified
/// rounding mode, taking the `Natural` by value. Passing `RoundingMode::Floor` or
/// `RoundingMode::Down` is equivalent to using `>>`. To test whether `RoundingMode::Exact` can be
/// passed, use `self.is_divisible_by_power_of_two(other)`.
///
/// Time: worst case O(`other`)
///
/// Additional memory: worst case O(1)
///
/// # Panics
/// Panics if `rm` is `RoundingMode::Exact` but `self` is not divisible by 2^(`other`).
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::round::RoundingMode;
/// use malachite_base::traits::ShrRound;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!(Natural::from(0x101u32).shr_round(8u32, RoundingMode::Down).to_string(), "1");
///     assert_eq!(Natural::from(0x101u32).shr_round(8u32, RoundingMode::Up).to_string(), "2");
///
///     assert_eq!(Natural::from(0x101u32).shr_round(9u32, RoundingMode::Down).to_string(), "0");
///     assert_eq!(Natural::from(0x101u32).shr_round(9u32, RoundingMode::Up).to_string(), "1");
///     assert_eq!(Natural::from(0x101u32).shr_round(9u32, RoundingMode::Nearest).to_string(), "1");
///     assert_eq!(Natural::from(0xffu32).shr_round(9u32, RoundingMode::Nearest).to_string(), "0");
///     assert_eq!(Natural::from(0x100u32).shr_round(9u32, RoundingMode::Nearest).to_string(), "0");
///
///     assert_eq!(Natural::from(0x100u32).shr_round(8u32, RoundingMode::Exact).to_string(), "1");
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
/// Time: worst case O(`other`)
///
/// Additional memory: worst case O(`other`)
///
/// # Panics
/// Panics if `rm` is `RoundingMode::Exact` but `self` is not divisible by 2^(`other`).
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::round::RoundingMode;
/// use malachite_base::traits::ShrRound;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((&Natural::from(0x101u32)).shr_round(8u32, RoundingMode::Down).to_string(), "1");
///     assert_eq!((&Natural::from(0x101u32)).shr_round(8u32, RoundingMode::Up).to_string(), "2");
///
///     assert_eq!((&Natural::from(0x101u32)).shr_round(9u32, RoundingMode::Down).to_string(), "0");
///     assert_eq!((&Natural::from(0x101u32)).shr_round(9u32, RoundingMode::Up).to_string(), "1");
///     assert_eq!((&Natural::from(0x101u32)).shr_round(9u32, RoundingMode::Nearest).to_string(),
///         "1");
///     assert_eq!((&Natural::from(0xffu32)).shr_round(9u32, RoundingMode::Nearest).to_string(),
///         "0");
///     assert_eq!((&Natural::from(0x100u32)).shr_round(9u32, RoundingMode::Nearest).to_string(),
///         "0");
///
///     assert_eq!((&Natural::from(0x100u32)).shr_round(8u32, RoundingMode::Exact).to_string(),
///         "1");
/// }
// TODO fix complexity
#[allow(unknown_lints, cyclomatic_complexity, unit_expr)]
impl<'a> ShrRound<u32> for &'a Natural {
    type Output = Natural;

    fn shr_round(self, other: u32, rm: RoundingMode) -> Natural {
        if other == 0 || *self == 0 {
            return self.clone();
        }
        let opt_result = match *self {
            Small(ref small) => {
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
            Large(ref limbs) => match rm {
                RoundingMode::Down | RoundingMode::Floor => {
                    let limbs_to_delete = (other >> LOG_LIMB_BITS) as usize;
                    let result = if limbs_to_delete >= limbs.len() {
                        return Natural::ZERO;
                    } else {
                        let small_shift = other & LIMB_BITS_MASK;
                        let mut result = limbs[limbs_to_delete..].to_vec();
                        if small_shift != 0 {
                            mpn_rshift_in_place(&mut result, small_shift);
                        }
                        result
                    };
                    Some(result)
                }
                RoundingMode::Up | RoundingMode::Ceiling => {
                    let limbs_to_delete = (other >> LOG_LIMB_BITS) as usize;
                    if limbs_to_delete >= limbs.len() {
                        return Natural::ONE;
                    } else {
                        let mut exact = mpn_zero_p(&limbs[0..limbs_to_delete]);
                        let small_shift = other & LIMB_BITS_MASK;
                        let mut result = limbs[limbs_to_delete..].to_vec();
                        if small_shift != 0 {
                            let remaining_bits = mpn_rshift_in_place(&mut result, small_shift);
                            exact &= remaining_bits == 0;
                        }
                        let mut result = Large(result);
                        result.trim();
                        if !exact {
                            result += 1;
                        }
                        return result;
                    }
                }
                RoundingMode::Exact => {
                    let limbs_to_delete = (other >> LOG_LIMB_BITS) as usize;
                    if limbs_to_delete >= limbs.len() {
                        panic!("Right shift is not exact: {} >> {}", self, other);
                    } else {
                        let small_shift = other & LIMB_BITS_MASK;
                        let mut result = limbs[limbs_to_delete..].to_vec();
                        if small_shift != 0 && mpn_rshift_in_place(&mut result, small_shift) != 0 {
                            panic!("Right shift is not exact: {} >> {}", self, other);
                        }
                        Some(result)
                    }
                }
                _ => None,
            },
        };
        let result: Vec<u32> = match opt_result {
            Some(result) => result,
            None => {
                match rm {
                    RoundingMode::Nearest => {
                        let limbs_to_delete = (other >> LOG_LIMB_BITS) as usize;
                        if !self.get_bit((other - 1).into()) {
                            // round down
                            if let Large(ref limbs) = *self {
                                if limbs_to_delete >= limbs.len() {
                                    return Natural::ZERO;
                                } else {
                                    let small_shift = other & LIMB_BITS_MASK;
                                    let mut result = limbs[limbs_to_delete..].to_vec();
                                    if small_shift != 0 {
                                        mpn_rshift_in_place(&mut result, small_shift);
                                    }
                                    result
                                }
                            } else {
                                unreachable!()
                            }
                        } else if !self.divisible_by_power_of_2(other - 1) {
                            // round up
                            if let Large(ref limbs) = *self {
                                return if limbs_to_delete >= limbs.len() {
                                    Natural::ONE
                                } else {
                                    let mut exact = mpn_zero_p(&limbs[0..limbs_to_delete]);
                                    let small_shift = other & LIMB_BITS_MASK;
                                    let mut result = limbs[limbs_to_delete..].to_vec();
                                    if small_shift != 0 {
                                        let remaining_bits =
                                            mpn_rshift_in_place(&mut result, small_shift);
                                        exact &= remaining_bits == 0;
                                    }
                                    let mut result = Large(result);
                                    result.trim();
                                    if !exact {
                                        result += 1;
                                    }
                                    result
                                };
                            } else {
                                unreachable!()
                            }
                        } else {
                            // result is half-integer; round to even
                            let result = if let Large(ref limbs) = *self {
                                if limbs_to_delete >= limbs.len() {
                                    return Natural::ZERO;
                                } else {
                                    let small_shift = other & LIMB_BITS_MASK;
                                    let mut result = limbs[limbs_to_delete..].to_vec();
                                    if small_shift != 0 {
                                        mpn_rshift_in_place(&mut result, small_shift);
                                    }
                                    result
                                }
                            } else {
                                unreachable!()
                            };
                            let mut result = Large(result);
                            result.trim();
                            if result.is_odd() {
                                result += 1;
                            }
                            return result;
                        }
                    }
                    _ => unreachable!(),
                }
            }
        };
        let mut result = Large(result);
        result.trim();
        result
    }
}

/// Shifts a `Natural` right (divides it by a power of 2) and rounds according to the specified
/// rounding mode, in place. Passing `RoundingMode::Floor` or `RoundingMode::Down` is equivalent to
/// using `>>=`. To test whether `RoundingMode::Exact` can be passed, use
/// `self.is_divisible_by_power_of_two(other)`.
///
/// Time: worst case O(`other`)
///
/// Additional memory: worst case O(1)
///
/// # Panics
/// Panics if `rm` is `RoundingMode::Exact` but `self` is not divisible by 2^(`other`).
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::round::RoundingMode;
/// use malachite_base::traits::ShrRoundAssign;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut n = Natural::from(0x101u32);
///     n.shr_round_assign(8, RoundingMode::Down);
///     assert_eq!(n.to_string(), "1");
///
///     let mut n = Natural::from(0x101u32);
///     n.shr_round_assign(8, RoundingMode::Up);
///     assert_eq!(n.to_string(), "2");
///
///     let mut n = Natural::from(0x101u32);
///     n.shr_round_assign(9, RoundingMode::Down);
///     assert_eq!(n.to_string(), "0");
///
///     let mut n = Natural::from(0x101u32);
///     n.shr_round_assign(9, RoundingMode::Up);
///     assert_eq!(n.to_string(), "1");
///
///     let mut n = Natural::from(0x101u32);
///     n.shr_round_assign(9, RoundingMode::Nearest);
///     assert_eq!(n.to_string(), "1");
///
///     let mut n = Natural::from(0xffu32);
///     n.shr_round_assign(9, RoundingMode::Nearest);
///     assert_eq!(n.to_string(), "0");
///
///     let mut n = Natural::from(0x100u32);
///     n.shr_round_assign(9, RoundingMode::Nearest);
///     assert_eq!(n.to_string(), "0");
///
///     let mut n = Natural::from(0x100u32);
///     n.shr_round_assign(8, RoundingMode::Exact);
///     assert_eq!(n.to_string(), "1");
/// }
impl ShrRoundAssign<u32> for Natural {
    fn shr_round_assign(&mut self, other: u32, rm: RoundingMode) {
        if other == 0 || *self == 0 {
            return;
        }
        let needs_more_work = match *self {
            Small(ref mut small) => {
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
            Large(ref mut limbs) => match rm {
                RoundingMode::Down | RoundingMode::Floor => {
                    shr_helper(limbs, other);
                    false
                }
                _ => true,
            },
        };
        if needs_more_work {
            match rm {
                RoundingMode::Up | RoundingMode::Ceiling => {
                    let exact = self.divisible_by_power_of_2(other);
                    if let Large(ref mut limbs) = *self {
                        shr_helper(limbs, other);
                    }
                    self.trim();
                    if !exact {
                        *self += 1;
                    }
                }
                RoundingMode::Nearest => {
                    if !self.get_bit((other - 1).into()) {
                        // round down
                        if let Large(ref mut limbs) = *self {
                            shr_helper(limbs, other);
                        }
                    } else if !self.divisible_by_power_of_2(other - 1) {
                        // round up
                        if let Large(ref mut limbs) = *self {
                            shr_helper(limbs, other);
                        }
                        self.trim();
                        *self += 1;
                        return;
                    } else {
                        // result is half-integer; round to even
                        if let Large(ref mut limbs) = *self {
                            shr_helper(limbs, other);
                        }
                        self.trim();
                        if self.is_odd() {
                            *self += 1;
                        }
                        return;
                    }
                }
                RoundingMode::Exact => {
                    if !self.divisible_by_power_of_2(other) {
                        panic!("Right shift is not exact: {} >>= {}", self, other);
                    }
                    if let Large(ref mut limbs) = *self {
                        shr_helper(limbs, other);
                    }
                }
                _ => unreachable!(),
            }
        }
        self.trim();
    }
}
