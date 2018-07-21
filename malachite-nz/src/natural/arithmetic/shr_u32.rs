use malachite_base::limbs::{limbs_delete_left, limbs_test_zero};
use malachite_base::misc::WrappingFrom;
use malachite_base::num::{Parity, PrimitiveInteger, ShrRound, ShrRoundAssign, Zero};
use malachite_base::round::RoundingMode;
use natural::arithmetic::add_u32::limbs_vec_add_limb_in_place;
use natural::logic::bit_access::limbs_get_bit;
use natural::Natural::{self, Large, Small};
use std::ops::{Shr, ShrAssign};

pub fn limbs_shr(limbs: &[u32], bits: u64) -> Vec<u32> {
    let limbs_to_delete = (bits >> u32::LOG_WIDTH) as usize;
    if limbs_to_delete >= limbs.len() {
        Vec::new()
    } else {
        let small_bits = u32::wrapping_from(bits) & u32::WIDTH_MASK;
        let mut result_limbs = limbs[limbs_to_delete..].to_vec();
        if small_bits != 0 {
            limbs_slice_shr_in_place(&mut result_limbs, small_bits);
        }
        result_limbs
    }
}

pub fn limbs_shr_round_up(limbs: &[u32], bits: u64) -> Vec<u32> {
    let limbs_to_delete = (bits >> u32::LOG_WIDTH) as usize;
    if limbs_to_delete >= limbs.len() {
        vec![1]
    } else {
        let mut exact = limbs_test_zero(&limbs[..limbs_to_delete]);
        let small_bits = u32::wrapping_from(bits) & u32::WIDTH_MASK;
        let mut result_limbs = limbs[limbs_to_delete..].to_vec();
        if small_bits != 0 {
            exact &= limbs_slice_shr_in_place(&mut result_limbs, small_bits) == 0;
        }
        if !exact {
            limbs_vec_add_limb_in_place(&mut result_limbs, 1);
        }
        result_limbs
    }
}

fn limbs_shr_round_half_integer_to_even(limbs: &[u32], bits: u64) -> Vec<u32> {
    let limbs_to_delete = (bits >> u32::LOG_WIDTH) as usize;
    if limbs_to_delete >= limbs.len() {
        Vec::new()
    } else {
        let small_bits = u32::wrapping_from(bits) & u32::WIDTH_MASK;
        let mut result_limbs = limbs[limbs_to_delete..].to_vec();
        if small_bits != 0 {
            limbs_slice_shr_in_place(&mut result_limbs, small_bits);
        }
        if !result_limbs.is_empty() && result_limbs[0].is_odd() {
            limbs_vec_add_limb_in_place(&mut result_limbs, 1);
        }
        result_limbs
    }
}

//TODO only use limb fns
pub fn limbs_shr_round_nearest(limbs: &[u32], bits: u64) -> Vec<u32> {
    if !limbs_get_bit(limbs, bits - 1) {
        limbs_shr(limbs, bits)
    } else if !Natural::from_limbs_asc(limbs).divisible_by_power_of_two((bits - 1) as u32) {
        limbs_shr_round_up(limbs, bits)
    } else {
        limbs_shr_round_half_integer_to_even(limbs, bits)
    }
}

// limbs not all zero
pub fn limbs_shr_exact(limbs: &[u32], bits: u64) -> Option<Vec<u32>> {
    let limbs_to_delete = (bits >> u32::LOG_WIDTH) as usize;
    if limbs_to_delete >= limbs.len() {
        None
    } else {
        let mut exact = limbs_test_zero(&limbs[..limbs_to_delete]);
        let small_bits = u32::wrapping_from(bits) & u32::WIDTH_MASK;
        let mut result_limbs = limbs[limbs_to_delete..].to_vec();
        if small_bits != 0 {
            exact &= limbs_slice_shr_in_place(&mut result_limbs, small_bits) == 0;
        }
        if exact {
            Some(result_limbs)
        } else {
            None
        }
    }
}

pub fn limbs_shr_round(limbs: &[u32], bits: u64, rm: RoundingMode) -> Option<Vec<u32>> {
    match rm {
        RoundingMode::Down | RoundingMode::Floor => Some(limbs_shr(limbs, bits)),
        RoundingMode::Up | RoundingMode::Ceiling => Some(limbs_shr_round_up(limbs, bits)),
        RoundingMode::Nearest => Some(limbs_shr_round_nearest(limbs, bits)),
        RoundingMode::Exact => limbs_shr_exact(limbs, bits),
    }
}

// Shift u right by bits, and write the result to r. The bits shifted out at the right are returned
// in the most significant bits of the return value (the rest of the return value is zero).
// u.len() > 0, r.len() >= u.len(), 1 <= bits < u32::WIDTH
pub fn limbs_shr_to_out(out_limbs: &mut [u32], in_limbs: &[u32], bits: u32) -> u32 {
    let len = in_limbs.len();
    assert!(len > 0);
    assert!(bits > 0);
    assert!(bits < u32::WIDTH);
    let cobits = u32::WIDTH - bits;
    let mut high_limb = in_limbs[0];
    let remaining_bits = high_limb << cobits;
    let mut low_limb = high_limb >> bits;
    for i in 1..len {
        high_limb = in_limbs[i];
        out_limbs[i - 1] = low_limb | (high_limb << cobits);
        low_limb = high_limb >> bits;
    }
    out_limbs[len - 1] = low_limb;
    remaining_bits
}

// Shift u right by bits, and write the result to u. The bits shifted out at the right are returned
// in the most significant bits of the return value (the rest of the return value is zero).
// u.len() > 0, 1 <= bits < u32::WIDTH
pub fn limbs_slice_shr_in_place(limbs: &mut [u32], bits: u32) -> u32 {
    assert!(!limbs.is_empty());
    assert!(bits > 0);
    assert!(bits < u32::WIDTH);
    let cobits = u32::WIDTH - bits;
    let mut high_limb = limbs[0];
    let remaining_bits = high_limb << cobits;
    let mut low_limb = high_limb >> bits;
    for i in 1..limbs.len() {
        high_limb = limbs[i];
        limbs[i - 1] = low_limb | (high_limb << cobits);
        low_limb = high_limb >> bits;
    }
    *limbs.last_mut().unwrap() = low_limb;
    remaining_bits
}

pub fn limbs_vec_shr_in_place(limbs: &mut Vec<u32>, bits: u64) {
    let limbs_to_delete = (bits >> u32::LOG_WIDTH) as usize;
    if limbs_to_delete >= limbs.len() {
        limbs.clear();
    } else {
        let small_shift = u32::wrapping_from(bits) & u32::WIDTH_MASK;
        limbs_delete_left(limbs, limbs_to_delete);
        if small_shift != 0 {
            limbs_slice_shr_in_place(limbs, small_shift);
        }
    }
}

// limbs nonempty
pub fn limbs_vec_shr_round_up_in_place(limbs: &mut Vec<u32>, bits: u64) {
    let limbs_to_delete = (bits >> u32::LOG_WIDTH) as usize;
    if limbs_to_delete >= limbs.len() {
        limbs.truncate(1);
        limbs[0] = 1;
    } else {
        let mut exact = limbs_test_zero(&limbs[..limbs_to_delete]);
        let small_bits = u32::wrapping_from(bits) & u32::WIDTH_MASK;
        limbs_delete_left(limbs, limbs_to_delete);
        if small_bits != 0 {
            exact &= limbs_slice_shr_in_place(limbs, small_bits) == 0;
        }
        if !exact {
            limbs_vec_add_limb_in_place(limbs, 1);
        }
    }
}

fn limbs_vec_shr_round_half_integer_to_even_in_place(limbs: &mut Vec<u32>, bits: u64) {
    let limbs_to_delete = (bits >> u32::LOG_WIDTH) as usize;
    if limbs_to_delete >= limbs.len() {
        limbs.clear();
    } else {
        let small_bits = u32::wrapping_from(bits) & u32::WIDTH_MASK;
        limbs_delete_left(limbs, limbs_to_delete);
        if small_bits != 0 {
            limbs_slice_shr_in_place(limbs, small_bits);
        }
        if !limbs.is_empty() && limbs[0].is_odd() {
            limbs_vec_add_limb_in_place(limbs, 1);
        }
    }
}

//TODO only use limb fns
pub fn limbs_vec_shr_round_nearest_in_place(limbs: &mut Vec<u32>, bits: u64) {
    if !limbs_get_bit(limbs, bits - 1) {
        limbs_vec_shr_in_place(limbs, bits)
    } else if !Natural::from_limbs_asc(limbs).divisible_by_power_of_two((bits - 1) as u32) {
        limbs_vec_shr_round_up_in_place(limbs, bits)
    } else {
        limbs_vec_shr_round_half_integer_to_even_in_place(limbs, bits)
    }
}

// limbs not all zero
pub fn limbs_vec_shr_exact_in_place(limbs: &mut Vec<u32>, bits: u64) -> bool {
    let limbs_to_delete = (bits >> u32::LOG_WIDTH) as usize;
    if limbs_to_delete >= limbs.len() || !limbs_test_zero(&limbs[..limbs_to_delete]) {
        false
    } else {
        let small_bits = u32::wrapping_from(bits) & u32::WIDTH_MASK;
        limbs_delete_left(limbs, limbs_to_delete);
        small_bits == 0 || limbs_slice_shr_in_place(limbs, small_bits) == 0
    }
}

pub fn limbs_shr_round_in_place(limbs: &mut Vec<u32>, bits: u64, rm: RoundingMode) -> bool {
    match rm {
        RoundingMode::Down | RoundingMode::Floor => {
            limbs_vec_shr_in_place(limbs, bits);
            true
        }
        RoundingMode::Up | RoundingMode::Ceiling => {
            limbs_vec_shr_round_up_in_place(limbs, bits);
            true
        }
        RoundingMode::Nearest => {
            limbs_vec_shr_round_nearest_in_place(limbs, bits);
            true
        }
        RoundingMode::Exact => limbs_vec_shr_exact_in_place(limbs, bits),
    }
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
/// use malachite_base::num::Zero;
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
/// use malachite_base::num::Zero;
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
            Small(_) if other >= u32::WIDTH => Natural::ZERO,
            Small(small) => Small(small >> other),
            Large(ref limbs) => {
                let mut result = Large(limbs_shr(limbs, u64::from(other)));
                result.trim();
                result
            }
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
            Small(ref mut small) if other >= u32::WIDTH => {
                *small = 0;
                return;
            }
            Small(ref mut small) => {
                *small >>= other;
                return;
            }
            Large(ref mut limbs) => {
                limbs_vec_shr_in_place(limbs, u64::from(other));
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
/// Panics if `rm` is `RoundingMode::Exact` but `self` is not divisible by 2<sup>`other`</sup>.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::round::RoundingMode;
/// use malachite_base::num::ShrRound;
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
/// Panics if `rm` is `RoundingMode::Exact` but `self` is not divisible by 2<sup>`other`</sup>.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::round::RoundingMode;
/// use malachite_base::num::ShrRound;
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
impl<'a> ShrRound<u32> for &'a Natural {
    type Output = Natural;

    fn shr_round(self, other: u32, rm: RoundingMode) -> Natural {
        if other == 0 || *self == 0 {
            return self.clone();
        }
        match *self {
            Small(ref small) => Small(small.shr_round(other, rm)),
            Large(ref limbs) => {
                if let Some(result_limbs) = limbs_shr_round(limbs, u64::from(other), rm) {
                    let mut result = Large(result_limbs);
                    result.trim();
                    result
                } else {
                    panic!("Right shift is not exact: {} >> {}", self, other);
                }
            }
        }
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
/// Panics if `rm` is `RoundingMode::Exact` but `self` is not divisible by 2<sup>`other`</sup>.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::round::RoundingMode;
/// use malachite_base::num::ShrRoundAssign;
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
        match *self {
            Small(ref mut small) => {
                small.shr_round_assign(other, rm);
                return;
            }
            Large(ref mut limbs) => if !limbs_shr_round_in_place(limbs, u64::from(other), rm) {
                panic!("Right shift is not exact.");
            },
        }
        self.trim();
    }
}
