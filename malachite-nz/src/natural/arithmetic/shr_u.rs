use malachite_base::limbs::{limbs_delete_left, limbs_test_zero};
use malachite_base::misc::{CheckedFrom, WrappingFrom};
use malachite_base::num::{Parity, PrimitiveInteger, ShrRound, ShrRoundAssign, Zero};
use malachite_base::round::RoundingMode;
use natural::arithmetic::add_limb::limbs_vec_add_limb_in_place;
use natural::arithmetic::divisible_by_power_of_two::limbs_divisible_by_power_of_two;
use natural::logic::bit_access::limbs_get_bit;
use natural::Natural::{self, Large, Small};
use platform::Limb;
use std::ops::{Shr, ShrAssign};

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
/// limbs of the `Natural` right-shifted by a `Limb`, rounding down.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = max(1, `limbs.len()` - `bits` / 32)
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::shr_u::limbs_shr;
///
/// assert_eq!(limbs_shr(&[1], 1), &[0]);
/// assert_eq!(limbs_shr(&[3], 1), &[1]);
/// assert_eq!(limbs_shr(&[122, 456], 1), &[61, 228]);
/// assert_eq!(limbs_shr(&[123, 456], 1), &[61, 228]);
/// assert_eq!(limbs_shr(&[123, 455], 1), &[2_147_483_709, 227]);
/// assert_eq!(limbs_shr(&[123, 456], 31), &[912, 0]);
/// assert_eq!(limbs_shr(&[123, 456], 32), &[456]);
/// assert_eq!(limbs_shr(&[123, 456], 100), Vec::<u32>::new());
/// assert_eq!(limbs_shr(&[256, 456], 8), &[3_355_443_201, 1]);
/// assert_eq!(limbs_shr(&[4_294_967_295, 1], 1), &[4_294_967_295, 0]);
/// assert_eq!(limbs_shr(&[4_294_967_295, 4_294_967_295], 32), &[4_294_967_295]);
/// ```
pub fn limbs_shr(limbs: &[Limb], bits: u64) -> Vec<Limb> {
    let limbs_to_delete = (bits >> Limb::LOG_WIDTH) as usize;
    if limbs_to_delete >= limbs.len() {
        Vec::new()
    } else {
        let small_bits = u32::wrapping_from(bits) & Limb::WIDTH_MASK;
        let mut result_limbs = limbs[limbs_to_delete..].to_vec();
        if small_bits != 0 {
            limbs_slice_shr_in_place(&mut result_limbs, small_bits);
        }
        result_limbs
    }
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
/// limbs of the `Natural` right-shifted by a `Limb`, rounding up. The limbs should not all be zero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = max(1, `limbs.len()` - `bits` / 32)
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::shr_u::limbs_shr_round_up;
///
/// assert_eq!(limbs_shr_round_up(&[1], 1), &[1]);
/// assert_eq!(limbs_shr_round_up(&[3], 1), &[2]);
/// assert_eq!(limbs_shr_round_up(&[122, 456], 1), &[61, 228]);
/// assert_eq!(limbs_shr_round_up(&[123, 456], 1), &[62, 228]);
/// assert_eq!(limbs_shr_round_up(&[123, 455], 1), &[2_147_483_710, 227]);
/// assert_eq!(limbs_shr_round_up(&[123, 456], 31), &[913, 0]);
/// assert_eq!(limbs_shr_round_up(&[123, 456], 32), &[457]);
/// assert_eq!(limbs_shr_round_up(&[123, 456], 100), &[1]);
/// assert_eq!(limbs_shr_round_up(&[256, 456], 8), &[3_355_443_201, 1]);
/// assert_eq!(limbs_shr_round_up(&[4_294_967_295, 1], 1), &[0, 1]);
/// assert_eq!(limbs_shr_round_up(&[4_294_967_295, 4_294_967_295], 32), &[0, 1]);
/// ```
pub fn limbs_shr_round_up(limbs: &[Limb], bits: u64) -> Vec<Limb> {
    let limbs_to_delete = (bits >> Limb::LOG_WIDTH) as usize;
    if limbs_to_delete >= limbs.len() {
        vec![1]
    } else {
        let mut exact = limbs_test_zero(&limbs[..limbs_to_delete]);
        let small_bits = u32::wrapping_from(bits) & Limb::WIDTH_MASK;
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

fn limbs_shr_round_half_integer_to_even(limbs: &[Limb], bits: u64) -> Vec<Limb> {
    let limbs_to_delete = (bits >> Limb::LOG_WIDTH) as usize;
    if limbs_to_delete >= limbs.len() {
        Vec::new()
    } else {
        let small_bits = u32::wrapping_from(bits) & Limb::WIDTH_MASK;
        let mut result_limbs = limbs[limbs_to_delete..].to_vec();
        if small_bits != 0 {
            limbs_slice_shr_in_place(&mut result_limbs, small_bits);
        }
        if !result_limbs.is_empty() && result_limbs[0].odd() {
            limbs_vec_add_limb_in_place(&mut result_limbs, 1);
        }
        result_limbs
    }
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
/// limbs of the `Natural` right-shifted by a `Limb`, rounding to the `Natural` nearest to the
/// actual value of `self` divided by 2<sup>`other`</sup>. If the actual value is exactly between
/// two integers, it is rounded to the even one.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(m)
///
/// where n = `limbs.len()`, m = max(1, `limbs.len()` - `bits` / 32)
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::shr_u::limbs_shr_round_to_nearest;
///
/// assert_eq!(limbs_shr_round_to_nearest(&[1], 1), &[0]);
/// assert_eq!(limbs_shr_round_to_nearest(&[3], 1), &[2]);
/// assert_eq!(limbs_shr_round_to_nearest(&[122, 456], 1), &[61, 228]);
/// assert_eq!(limbs_shr_round_to_nearest(&[123, 456], 1), &[62, 228]);
/// assert_eq!(limbs_shr_round_to_nearest(&[123, 455], 1), &[2_147_483_710, 227]);
/// assert_eq!(limbs_shr_round_to_nearest(&[123, 456], 31), &[912, 0]);
/// assert_eq!(limbs_shr_round_to_nearest(&[123, 456], 32), &[456]);
/// assert_eq!(limbs_shr_round_to_nearest(&[123, 456], 100), Vec::<u32>::new());
/// assert_eq!(limbs_shr_round_to_nearest(&[256, 456], 8), &[3_355_443_201, 1]);
/// assert_eq!(limbs_shr_round_to_nearest(&[4_294_967_295, 1], 1), &[0, 1]);
/// assert_eq!(limbs_shr_round_to_nearest(&[4_294_967_295, 4_294_967_295], 32), &[0, 1]);
/// ```
pub fn limbs_shr_round_to_nearest(limbs: &[Limb], bits: u64) -> Vec<Limb> {
    if !limbs_get_bit(limbs, bits - 1) {
        limbs_shr(limbs, bits)
    } else if !limbs_divisible_by_power_of_two(limbs, bits - 1) {
        limbs_shr_round_up(limbs, bits)
    } else {
        limbs_shr_round_half_integer_to_even(limbs, bits)
    }
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
/// limbs of the `Natural` right-shifted by a `Limb`, if the shift is exact (doesn't remove any
/// `true` bits). If the shift is inexact, `None` is returned. The limbs should not all be zero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(m)
///
/// where n = `limbs.len()`, m = max(1, `limbs.len()` - `bits` / 32)
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::shr_u::limbs_shr_exact;
///
/// assert_eq!(limbs_shr_exact(&[1], 1), None);
/// assert_eq!(limbs_shr_exact(&[3], 1), None);
/// assert_eq!(limbs_shr_exact(&[122, 456], 1), Some(vec![61, 228]));
/// assert_eq!(limbs_shr_exact(&[123, 456], 1), None);
/// assert_eq!(limbs_shr_exact(&[123, 455], 1), None);
/// assert_eq!(limbs_shr_exact(&[123, 456], 31), None);
/// assert_eq!(limbs_shr_exact(&[123, 456], 32), None);
/// assert_eq!(limbs_shr_exact(&[123, 456], 100), None);
/// assert_eq!(limbs_shr_exact(&[256, 456], 8), Some(vec![3_355_443_201, 1]));
/// assert_eq!(limbs_shr_exact(&[4_294_967_295, 1], 1), None);
/// assert_eq!(limbs_shr_exact(&[4_294_967_295, 4_294_967_295], 32), None);
/// ```
pub fn limbs_shr_exact(limbs: &[Limb], bits: u64) -> Option<Vec<Limb>> {
    if limbs_divisible_by_power_of_two(limbs, bits) {
        Some(limbs_shr(limbs, bits))
    } else {
        None
    }
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
/// limbs of the `Natural` right-shifted by a `Limb`, rounded using a specified rounding format. The
/// limbs should not all be zero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(m)
///
/// where n = `limbs.len()`, m = max(1, `limbs.len()` - `bits` / 32)
///
/// # Example
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::round::RoundingMode;
/// use malachite_nz::natural::arithmetic::shr_u::limbs_shr_round;
///
/// fn main() {
///     assert_eq!(limbs_shr_round(&[1], 1, RoundingMode::Nearest), Some(vec![0]));
///     assert_eq!(limbs_shr_round(&[3], 1, RoundingMode::Nearest), Some(vec![2]));
///     assert_eq!(limbs_shr_round(&[122, 456], 1, RoundingMode::Floor), Some(vec![61, 228]));
///     assert_eq!(limbs_shr_round(&[123, 456], 1, RoundingMode::Floor), Some(vec![61, 228]));
///     assert_eq!(limbs_shr_round(&[123, 455], 1, RoundingMode::Floor),
///         Some(vec![2_147_483_709, 227]));
///     assert_eq!(limbs_shr_round(&[123, 456], 31, RoundingMode::Ceiling), Some(vec![913, 0]));
///     assert_eq!(limbs_shr_round(&[123, 456], 32, RoundingMode::Up), Some(vec![457]));
///     assert_eq!(limbs_shr_round(&[123, 456], 100, RoundingMode::Down), Some(vec![]));
///     assert_eq!(limbs_shr_round(&[256, 456], 8, RoundingMode::Exact),
///         Some(vec![3_355_443_201, 1]));
///     assert_eq!(limbs_shr_round(&[4_294_967_295, 1], 1, RoundingMode::Exact), None);
///     assert_eq!(limbs_shr_round(&[4_294_967_295, 4_294_967_295], 32, RoundingMode::Down),
///         Some(vec![4_294_967_295]));
/// }
/// ```
pub fn limbs_shr_round(limbs: &[Limb], bits: u64, rm: RoundingMode) -> Option<Vec<Limb>> {
    match rm {
        RoundingMode::Down | RoundingMode::Floor => Some(limbs_shr(limbs, bits)),
        RoundingMode::Up | RoundingMode::Ceiling => Some(limbs_shr_round_up(limbs, bits)),
        RoundingMode::Nearest => Some(limbs_shr_round_to_nearest(limbs, bits)),
        RoundingMode::Exact => limbs_shr_exact(limbs, bits),
    }
}

/// Interpreting a nonempty slice of `Limb`s as the limbs (in ascending order) of a `Natural`,
/// writes the limbs of the `Natural` right-shifted by a `Limb` to an output slice. The output slice
/// must be at least as long as the input slice. The `Limb` must be between 1 and 31, inclusive. The
/// carry, or the bits that are shifted past the width of the input slice, is returned. The input
/// slice should not only contain zeros.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `in_limbs` is empty, `out_limbs` is shorter than `in_limbs`, `bits` is 0, or `bits` is
/// greater than 31.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::shr_u::limbs_shr_to_out;
///
/// let mut out_limbs = vec![0, 0, 0];
/// assert_eq!(limbs_shr_to_out(&mut out_limbs, &[123, 456], 1), 2_147_483_648);
/// assert_eq!(out_limbs, &[61, 228, 0]);
///
/// let mut out_limbs = vec![0, 0, 0];
/// assert_eq!(limbs_shr_to_out(&mut out_limbs, &[122, 455], 1), 0);
/// assert_eq!(out_limbs, &[2_147_483_709, 227, 0]);
/// ```
pub fn limbs_shr_to_out(out_limbs: &mut [Limb], in_limbs: &[Limb], bits: u32) -> Limb {
    let len = in_limbs.len();
    assert!(len > 0);
    assert!(bits > 0);
    assert!(bits < Limb::WIDTH);
    assert!(out_limbs.len() >= len);
    let cobits = Limb::WIDTH - bits;
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

/// Interpreting a nonempty slice of `Limb`s as the limbs (in ascending order) of a `Natural`,
/// writes the limbs of the `Natural` right-shifted by a `Limb` to the input slice. The `Limb` must
/// be between 1 and 31, inclusive. The carry, or the bits that are shifted past the width of the
/// input slice, is returned.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `in_limbs` is empty, `bits` is 0, or `bits` is greater than 31.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::shr_u::limbs_slice_shr_in_place;
///
/// let mut limbs = vec![123, 456];
/// assert_eq!(limbs_slice_shr_in_place(&mut limbs, 1), 2_147_483_648);
/// assert_eq!(limbs, &[61, 228]);
///
/// let mut limbs = vec![122, 455];
/// assert_eq!(limbs_slice_shr_in_place(&mut limbs, 1), 0);
/// assert_eq!(limbs, &[2_147_483_709, 227]);
/// ```
pub fn limbs_slice_shr_in_place(limbs: &mut [Limb], bits: u32) -> Limb {
    assert!(bits > 0);
    assert!(bits < Limb::WIDTH);
    let len = limbs.len();
    assert!(len > 0);
    let cobits = Limb::WIDTH - bits;
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

/// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the `Natural` right-shifted by a `Limb` to the input `Vec`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = max(1, `limbs.len()` - `bits` / 32)
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::shr_u::limbs_vec_shr_in_place;
///
/// let mut limbs = vec![1];
/// limbs_vec_shr_in_place(&mut limbs, 1);
/// assert_eq!(limbs, &[0]);
///
/// let mut limbs = vec![3];
/// limbs_vec_shr_in_place(&mut limbs, 1);
/// assert_eq!(limbs, &[1]);
///
/// let mut limbs = vec![122, 456];
/// limbs_vec_shr_in_place(&mut limbs, 1);
/// assert_eq!(limbs, &[61, 228]);
///
/// let mut limbs = vec![123, 456];
/// limbs_vec_shr_in_place(&mut limbs, 1);
/// assert_eq!(limbs, &[61, 228]);
///
/// let mut limbs = vec![123, 455];
/// limbs_vec_shr_in_place(&mut limbs, 1);
/// assert_eq!(limbs, &[2_147_483_709, 227]);
///
/// let mut limbs = vec![123, 456];
/// limbs_vec_shr_in_place(&mut limbs, 31);
/// assert_eq!(limbs, &[912, 0]);
///
/// let mut limbs = vec![123, 456];
/// limbs_vec_shr_in_place(&mut limbs, 32);
/// assert_eq!(limbs, &[456]);
///
/// let mut limbs = vec![123, 456];
/// limbs_vec_shr_in_place(&mut limbs, 100);
/// assert_eq!(limbs, Vec::<u32>::new());
///
/// let mut limbs = vec![256, 456];
/// limbs_vec_shr_in_place(&mut limbs, 8);
/// assert_eq!(limbs, &[3_355_443_201, 1]);
///
/// let mut limbs = vec![4_294_967_295, 1];
/// limbs_vec_shr_in_place(&mut limbs, 1);
/// assert_eq!(limbs, &[4_294_967_295, 0]);
///
/// let mut limbs = vec![4_294_967_295, 4_294_967_295];
/// limbs_vec_shr_in_place(&mut limbs, 32);
/// assert_eq!(limbs, &[4_294_967_295]);
/// ```
pub fn limbs_vec_shr_in_place(limbs: &mut Vec<Limb>, bits: u64) {
    let limbs_to_delete = (bits >> Limb::LOG_WIDTH) as usize;
    if limbs_to_delete >= limbs.len() {
        limbs.clear();
    } else {
        let small_shift = u32::wrapping_from(bits) & Limb::WIDTH_MASK;
        limbs_delete_left(limbs, limbs_to_delete);
        if small_shift != 0 {
            limbs_slice_shr_in_place(limbs, small_shift);
        }
    }
}

/// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the `Natural` right-shifted by a `Limb`, rounding up, to the input `Vec`. The limbs
/// should not all be zero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = max(1, `limbs.len()` - `bits` / 32)
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::shr_u::limbs_vec_shr_round_up_in_place;
///
/// let mut limbs = vec![1];
/// limbs_vec_shr_round_up_in_place(&mut limbs, 1);
/// assert_eq!(limbs, &[1]);
///
/// let mut limbs = vec![3];
/// limbs_vec_shr_round_up_in_place(&mut limbs, 1);
/// assert_eq!(limbs, &[2]);
///
/// let mut limbs = vec![122, 456];
/// limbs_vec_shr_round_up_in_place(&mut limbs, 1);
/// assert_eq!(limbs, &[61, 228]);
///
/// let mut limbs = vec![123, 456];
/// limbs_vec_shr_round_up_in_place(&mut limbs, 1);
/// assert_eq!(limbs, &[62, 228]);
///
/// let mut limbs = vec![123, 455];
/// limbs_vec_shr_round_up_in_place(&mut limbs, 1);
/// assert_eq!(limbs, &[2_147_483_710, 227]);
///
/// let mut limbs = vec![123, 456];
/// limbs_vec_shr_round_up_in_place(&mut limbs, 31);
/// assert_eq!(limbs, &[913, 0]);
///
/// let mut limbs = vec![123, 456];
/// limbs_vec_shr_round_up_in_place(&mut limbs, 32);
/// assert_eq!(limbs, &[457]);
///
/// let mut limbs = vec![123, 456];
/// limbs_vec_shr_round_up_in_place(&mut limbs, 100);
/// assert_eq!(limbs, &[1]);
///
/// let mut limbs = vec![256, 456];
/// limbs_vec_shr_round_up_in_place(&mut limbs, 8);
/// assert_eq!(limbs, &[3_355_443_201, 1]);
///
/// let mut limbs = vec![4_294_967_295, 1];
/// limbs_vec_shr_round_up_in_place(&mut limbs, 1);
/// assert_eq!(limbs, &[0, 1]);
///
/// let mut limbs = vec![4_294_967_295, 4_294_967_295];
/// limbs_vec_shr_round_up_in_place(&mut limbs, 32);
/// assert_eq!(limbs, &[0, 1]);
/// ```
pub fn limbs_vec_shr_round_up_in_place(limbs: &mut Vec<Limb>, bits: u64) {
    let limbs_to_delete = (bits >> Limb::LOG_WIDTH) as usize;
    if limbs_to_delete >= limbs.len() {
        limbs.truncate(1);
        limbs[0] = 1;
    } else {
        let mut exact = limbs_test_zero(&limbs[..limbs_to_delete]);
        let small_bits = u32::wrapping_from(bits) & Limb::WIDTH_MASK;
        limbs_delete_left(limbs, limbs_to_delete);
        if small_bits != 0 {
            exact &= limbs_slice_shr_in_place(limbs, small_bits) == 0;
        }
        if !exact {
            limbs_vec_add_limb_in_place(limbs, 1);
        }
    }
}

fn limbs_vec_shr_round_half_integer_to_even_in_place(limbs: &mut Vec<Limb>, bits: u64) {
    let limbs_to_delete = (bits >> Limb::LOG_WIDTH) as usize;
    if limbs_to_delete >= limbs.len() {
        limbs.clear();
    } else {
        let small_bits = u32::wrapping_from(bits) & Limb::WIDTH_MASK;
        limbs_delete_left(limbs, limbs_to_delete);
        if small_bits != 0 {
            limbs_slice_shr_in_place(limbs, small_bits);
        }
        if !limbs.is_empty() && limbs[0].odd() {
            limbs_vec_add_limb_in_place(limbs, 1);
        }
    }
}

/// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the `Natural` right-shifted by a `Limb` to the input `Vec`, rounding to the `Natural`
/// nearest to the actual value of `self` divided by 2<sup>`other`</sup>. If the actual value is
/// exactly between two integers, it is rounded to the even one.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::shr_u::limbs_vec_shr_round_to_nearest_in_place;
///
/// let mut limbs = vec![1];
/// limbs_vec_shr_round_to_nearest_in_place(&mut limbs, 1);
/// assert_eq!(limbs, &[0]);
///
/// let mut limbs = vec![3];
/// limbs_vec_shr_round_to_nearest_in_place(&mut limbs, 1);
/// assert_eq!(limbs, &[2]);
///
/// let mut limbs = vec![122, 456];
/// limbs_vec_shr_round_to_nearest_in_place(&mut limbs, 1);
/// assert_eq!(limbs, &[61, 228]);
///
/// let mut limbs = vec![123, 456];
/// limbs_vec_shr_round_to_nearest_in_place(&mut limbs, 1);
/// assert_eq!(limbs, &[62, 228]);
///
/// let mut limbs = vec![123, 455];
/// limbs_vec_shr_round_to_nearest_in_place(&mut limbs, 1);
/// assert_eq!(limbs, &[2_147_483_710, 227]);
///
/// let mut limbs = vec![123, 456];
/// limbs_vec_shr_round_to_nearest_in_place(&mut limbs, 31);
/// assert_eq!(limbs, &[912, 0]);
///
/// let mut limbs = vec![123, 456];
/// limbs_vec_shr_round_to_nearest_in_place(&mut limbs, 32);
/// assert_eq!(limbs, &[456]);
///
/// let mut limbs = vec![123, 456];
/// limbs_vec_shr_round_to_nearest_in_place(&mut limbs, 100);
/// assert_eq!(limbs, Vec::<u32>::new());
///
/// let mut limbs = vec![256, 456];
/// limbs_vec_shr_round_to_nearest_in_place(&mut limbs, 8);
/// assert_eq!(limbs, &[3_355_443_201, 1]);
///
/// let mut limbs = vec![4_294_967_295, 1];
/// limbs_vec_shr_round_to_nearest_in_place(&mut limbs, 1);
/// assert_eq!(limbs, &[0, 1]);
///
/// let mut limbs = vec![4_294_967_295, 4_294_967_295];
/// limbs_vec_shr_round_to_nearest_in_place(&mut limbs, 32);
/// assert_eq!(limbs, &[0, 1]);
/// ```
pub fn limbs_vec_shr_round_to_nearest_in_place(limbs: &mut Vec<Limb>, bits: u64) {
    if !limbs_get_bit(limbs, bits - 1) {
        limbs_vec_shr_in_place(limbs, bits)
    } else if !limbs_divisible_by_power_of_two(limbs, bits - 1) {
        limbs_vec_shr_round_up_in_place(limbs, bits)
    } else {
        limbs_vec_shr_round_half_integer_to_even_in_place(limbs, bits)
    }
}

/// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the `Natural` right-shifted by a `Limb` to the input `Vec`, if the shift is exact
/// (doesn't remove any `true` bits). Returns whether the shift was exact. The limbs should not all
/// be zero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::shr_u::limbs_vec_shr_exact_in_place;
///
/// let mut limbs = vec![1];
/// assert_eq!(limbs_vec_shr_exact_in_place(&mut limbs, 1), false);
///
/// let mut limbs = vec![3];
/// assert_eq!(limbs_vec_shr_exact_in_place(&mut limbs, 1), false);
///
/// let mut limbs = vec![122, 456];
/// assert_eq!(limbs_vec_shr_exact_in_place(&mut limbs, 1), true);
/// assert_eq!(limbs, &[61, 228]);
///
/// let mut limbs = vec![123, 456];
/// assert_eq!(limbs_vec_shr_exact_in_place(&mut limbs, 1), false);
///
/// let mut limbs = vec![123, 455];
/// assert_eq!(limbs_vec_shr_exact_in_place(&mut limbs, 1), false);
///
/// let mut limbs = vec![123, 456];
/// assert_eq!(limbs_vec_shr_exact_in_place(&mut limbs, 31), false);
///
/// let mut limbs = vec![123, 456];
/// assert_eq!(limbs_vec_shr_exact_in_place(&mut limbs, 32), false);
///
/// let mut limbs = vec![123, 456];
/// assert_eq!(limbs_vec_shr_exact_in_place(&mut limbs, 100), false);
///
/// let mut limbs = vec![256, 456];
/// assert_eq!(limbs_vec_shr_exact_in_place(&mut limbs, 8), true);
/// assert_eq!(limbs, &[3_355_443_201, 1]);
///
/// let mut limbs = vec![4_294_967_295, 1];
/// assert_eq!(limbs_vec_shr_exact_in_place(&mut limbs, 1), false);
///
/// let mut limbs = vec![4_294_967_295, 4_294_967_295];
/// assert_eq!(limbs_vec_shr_exact_in_place(&mut limbs, 32), false);
/// ```
pub fn limbs_vec_shr_exact_in_place(limbs: &mut Vec<Limb>, bits: u64) -> bool {
    if limbs_divisible_by_power_of_two(limbs, bits) {
        limbs_vec_shr_in_place(limbs, bits);
        true
    } else {
        false
    }
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the `Natural` right-shifted by a `Limb` to the input `Vec`, rounded using a specified
/// rounding format. If the shift is inexact (removes some `true` bits) and the `RoundingMode` is
/// `Exact`, the value of `limbs` becomes unspecified and `false` is returned. Otherwise, `true` is
/// returned. The limbs should not all be zero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Example
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::round::RoundingMode;
/// use malachite_nz::natural::arithmetic::shr_u::limbs_vec_shr_round_in_place;
///
/// fn main() {
///     let mut limbs = vec![1];
///     assert_eq!(limbs_vec_shr_round_in_place(&mut limbs, 1, RoundingMode::Nearest), true);
///     assert_eq!(limbs, &[0]);
///
///     let mut limbs = vec![3];
///     assert_eq!(limbs_vec_shr_round_in_place(&mut limbs, 1, RoundingMode::Nearest), true);
///     assert_eq!(limbs, &[2]);
///
///     let mut limbs = vec![122, 456];
///     assert_eq!(limbs_vec_shr_round_in_place(&mut limbs, 1, RoundingMode::Floor), true);
///     assert_eq!(limbs, &[61, 228]);
///
///     let mut limbs = vec![123, 456];
///     assert_eq!(limbs_vec_shr_round_in_place(&mut limbs, 1, RoundingMode::Floor), true);
///     assert_eq!(limbs, &[61, 228]);
///
///     let mut limbs = vec![123, 455];
///     assert_eq!(limbs_vec_shr_round_in_place(&mut limbs, 1, RoundingMode::Floor), true);
///     assert_eq!(limbs, &[2_147_483_709, 227]);
///
///     let mut limbs = vec![123, 456];
///     assert_eq!(limbs_vec_shr_round_in_place(&mut limbs, 31, RoundingMode::Ceiling), true);
///     assert_eq!(limbs, &[913, 0]);
///
///     let mut limbs = vec![123, 456];
///     assert_eq!(limbs_vec_shr_round_in_place(&mut limbs, 32, RoundingMode::Up), true);
///     assert_eq!(limbs, &[457]);
///
///     let mut limbs = vec![123, 456];
///     assert_eq!(limbs_vec_shr_round_in_place(&mut limbs, 100, RoundingMode::Down), true);
///     assert_eq!(limbs, Vec::<u32>::new());
///
///     let mut limbs = vec![256, 456];
///     assert_eq!(limbs_vec_shr_round_in_place(&mut limbs, 8, RoundingMode::Exact), true);
///     assert_eq!(limbs, vec![3_355_443_201, 1]);
///
///     let mut limbs = vec![4_294_967_295, 1];
///     assert_eq!(limbs_vec_shr_round_in_place(&mut limbs, 1, RoundingMode::Exact), false);
///
///     let mut limbs = vec![4_294_967_295, 4_294_967_295];
///     assert_eq!(limbs_vec_shr_round_in_place(&mut limbs, 32, RoundingMode::Down), true);
///     assert_eq!(limbs, vec![4_294_967_295]);
/// }
/// ```
pub fn limbs_vec_shr_round_in_place(limbs: &mut Vec<Limb>, bits: u64, rm: RoundingMode) -> bool {
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
            limbs_vec_shr_round_to_nearest_in_place(limbs, bits);
            true
        }
        RoundingMode::Exact => limbs_vec_shr_exact_in_place(limbs, bits),
    }
}

macro_rules! impl_natural_shr_unsigned {
    ($t:ident) => {
        /// Shifts a `Natural` right (divides it by a power of 2 and takes the floor), taking the
        /// `Natural` by value.
        ///
        /// Time: worst case O(n)
        ///
        /// Additional memory: worst case O(1)
        ///
        /// where n = max(1, `self.significant_bits()` - `other`)
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
        ///     assert_eq!((Natural::ZERO >> 10u8).to_string(), "0");
        ///     assert_eq!((Natural::from(492u32) >> 2u32).to_string(), "123");
        ///     assert_eq!((Natural::trillion() >> 10u64).to_string(), "976562500");
        /// }
        /// ```
        impl Shr<$t> for Natural {
            type Output = Natural;

            fn shr(mut self, other: $t) -> Natural {
                self >>= other;
                self
            }
        }

        /// Shifts a `Natural` right (divides it by a power of 2 and takes the floor), taking the
        /// `Natural` by reference.
        ///
        /// Time: worst case O(n)
        ///
        /// Additional memory: worst case O(n)
        ///
        /// where n = max(1, `self.significant_bits()` - `other`)
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
        ///     assert_eq!((&Natural::ZERO >> 10u8).to_string(), "0");
        ///     assert_eq!((&Natural::from(492u32) >> 2u32).to_string(), "123");
        ///     assert_eq!((&Natural::trillion() >> 10u64).to_string(), "976562500");
        /// }
        /// ```
        impl<'a> Shr<$t> for &'a Natural {
            type Output = Natural;

            fn shr(self, other: $t) -> Natural {
                if other == 0 || *self == 0 {
                    return self.clone();
                }
                match *self {
                    Small(_) if other >= $t::wrapping_from(Limb::WIDTH) => Natural::ZERO,
                    Small(small) => Small(small >> other),
                    Large(ref limbs) => {
                        let mut result = Large(limbs_shr(limbs, u64::checked_from(other).unwrap()));
                        result.trim();
                        result
                    }
                }
            }
        }

        /// Shifts a `Natural` right (divides it by a power of 2 and takes the floor) in place.
        ///
        /// Time: worst case O(n)
        ///
        /// Additional memory: worst case O(1)
        ///
        /// where n = max(1, `self.significant_bits()` - `other`)
        ///
        /// # Examples
        /// ```
        /// use malachite_nz::natural::Natural;
        ///
        /// let mut x = Natural::from(1024u32);
        /// x >>= 1u8;
        /// x >>= 2u16;
        /// x >>= 3u32;
        /// x >>= 4u64;
        /// assert_eq!(x.to_string(), "1");
        /// ```
        impl ShrAssign<$t> for Natural {
            fn shr_assign(&mut self, other: $t) {
                if other == 0 || *self == 0 {
                    return;
                }
                match *self {
                    Small(ref mut small) if other >= $t::wrapping_from(Limb::WIDTH) => {
                        *small = 0;
                        return;
                    }
                    Small(ref mut small) => {
                        *small >>= other;
                        return;
                    }
                    Large(ref mut limbs) => {
                        limbs_vec_shr_in_place(limbs, u64::checked_from(other).unwrap());
                    }
                }
                self.trim();
            }
        }

        /// Shifts a `Natural` right (divides it by a power of 2) and rounds according to the
        /// specified rounding mode, taking the `Natural` by value. Passing `RoundingMode::Floor` or
        /// `RoundingMode::Down` is equivalent to using `>>`. To test whether `RoundingMode::Exact`
        /// can be passed, use `self.is_divisible_by_power_of_two(other)`.
        ///
        /// Time: worst case O(n)
        ///
        /// Additional memory: worst case O(1)
        ///
        /// where n = `limbs.len()`
        ///
        /// # Panics
        /// Panics if `rm` is `RoundingMode::Exact` but `self` is not divisible by
        /// 2<sup>`other`</sup>.
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
        ///     assert_eq!(Natural::from(0x101u32).shr_round(8u8, RoundingMode::Down).to_string(),
        ///         "1");
        ///     assert_eq!(Natural::from(0x101u32).shr_round(8u16, RoundingMode::Up).to_string(),
        ///         "2");
        ///
        ///     assert_eq!(Natural::from(0x101u32).shr_round(9u32, RoundingMode::Down).to_string(),
        ///         "0");
        ///     assert_eq!(Natural::from(0x101u32).shr_round(9u64, RoundingMode::Up).to_string(),
        ///         "1");
        ///     assert_eq!(Natural::from(0x101u32).shr_round(9u8, RoundingMode::Nearest)
        ///         .to_string(), "1");
        ///     assert_eq!(Natural::from(0xffu32).shr_round(9u16, RoundingMode::Nearest)
        ///         .to_string(), "0");
        ///     assert_eq!(Natural::from(0x100u32).shr_round(9u32, RoundingMode::Nearest)
        ///         .to_string(), "0");
        ///
        ///     assert_eq!(Natural::from(0x100u32).shr_round(8u64, RoundingMode::Exact).to_string(),
        ///         "1");
        /// }
        impl ShrRound<$t> for Natural {
            type Output = Natural;

            fn shr_round(mut self, other: $t, rm: RoundingMode) -> Natural {
                self.shr_round_assign(other, rm);
                self
            }
        }

        /// Shifts a `Natural` right (divides it by a power of 2) and rounds according to the
        /// specified rounding mode, taking the `Natural` by reference. Passing
        /// `RoundingMode::Floor` or `RoundingMode::Down` is equivalent to using `>>`. To test
        /// whether `RoundingMode::Exact` can be passed, use
        /// `self.divisible_by_power_of_two(other)`.
        ///
        /// Time: worst case O(n)
        ///
        /// Additional memory: worst case O(m)
        ///
        /// where n = `self.significant_bits()`, m = max(1, `self.significant_bits()` - `other`)
        ///
        /// # Panics
        /// Panics if `rm` is `RoundingMode::Exact` but `self` is not divisible by
        /// 2<sup>`other`</sup>.
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
        ///     assert_eq!((&Natural::from(0x101u32)).shr_round(8u8, RoundingMode::Down)
        ///         .to_string(), "1");
        ///     assert_eq!((&Natural::from(0x101u32)).shr_round(8u16, RoundingMode::Up).to_string(),
        ///         "2");
        ///
        ///     assert_eq!((&Natural::from(0x101u32)).shr_round(9u32, RoundingMode::Down)
        ///         .to_string(), "0");
        ///     assert_eq!((&Natural::from(0x101u32)).shr_round(9u64, RoundingMode::Up).to_string(),
        ///         "1");
        ///     assert_eq!((&Natural::from(0x101u32)).shr_round(9u8, RoundingMode::Nearest)
        ///         .to_string(), "1");
        ///     assert_eq!((&Natural::from(0xffu32)).shr_round(9u16, RoundingMode::Nearest)
        ///         .to_string(), "0");
        ///     assert_eq!((&Natural::from(0x100u32)).shr_round(9u32, RoundingMode::Nearest)
        ///         .to_string(), "0");
        ///
        ///     assert_eq!((&Natural::from(0x100u32)).shr_round(8u64, RoundingMode::Exact)
        ///         .to_string(), "1");
        /// }
        impl<'a> ShrRound<$t> for &'a Natural {
            type Output = Natural;

            fn shr_round(self, other: $t, rm: RoundingMode) -> Natural {
                if other == 0 || *self == 0 {
                    return self.clone();
                }
                match *self {
                    Small(ref small) => Small(small.shr_round(other, rm)),
                    Large(ref limbs) => {
                        if let Some(result_limbs) =
                            limbs_shr_round(limbs, u64::checked_from(other).unwrap(), rm)
                        {
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

        /// Shifts a `Natural` right (divides it by a power of 2) and rounds according to the
        /// specified rounding mode, in place. Passing `RoundingMode::Floor` or `RoundingMode::Down`
        /// is equivalent to using `>>=`. To test whether `RoundingMode::Exact` can be passed, use
        /// `self.divisible_by_power_of_two(other)`.
        ///
        /// Time: worst case O(n)
        ///
        /// Additional memory: worst case O(1)
        ///
        /// where n = `limbs.len()`
        ///
        /// # Panics
        /// Panics if `rm` is `RoundingMode::Exact` but `self` is not divisible by
        /// 2<sup>`other`</sup>.
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
        ///     n.shr_round_assign(8u8, RoundingMode::Down);
        ///     assert_eq!(n.to_string(), "1");
        ///
        ///     let mut n = Natural::from(0x101u32);
        ///     n.shr_round_assign(8u16, RoundingMode::Up);
        ///     assert_eq!(n.to_string(), "2");
        ///
        ///     let mut n = Natural::from(0x101u32);
        ///     n.shr_round_assign(9u32, RoundingMode::Down);
        ///     assert_eq!(n.to_string(), "0");
        ///
        ///     let mut n = Natural::from(0x101u32);
        ///     n.shr_round_assign(9u64, RoundingMode::Up);
        ///     assert_eq!(n.to_string(), "1");
        ///
        ///     let mut n = Natural::from(0x101u32);
        ///     n.shr_round_assign(9u8, RoundingMode::Nearest);
        ///     assert_eq!(n.to_string(), "1");
        ///
        ///     let mut n = Natural::from(0xffu32);
        ///     n.shr_round_assign(9u16, RoundingMode::Nearest);
        ///     assert_eq!(n.to_string(), "0");
        ///
        ///     let mut n = Natural::from(0x100u32);
        ///     n.shr_round_assign(9u32, RoundingMode::Nearest);
        ///     assert_eq!(n.to_string(), "0");
        ///
        ///     let mut n = Natural::from(0x100u32);
        ///     n.shr_round_assign(8u64, RoundingMode::Exact);
        ///     assert_eq!(n.to_string(), "1");
        /// }
        impl ShrRoundAssign<$t> for Natural {
            fn shr_round_assign(&mut self, other: $t, rm: RoundingMode) {
                if other == 0 || *self == 0 {
                    return;
                }
                match *self {
                    Small(ref mut small) => {
                        small.shr_round_assign(other, rm);
                        return;
                    }
                    Large(ref mut limbs) => {
                        if !limbs_vec_shr_round_in_place(
                            limbs,
                            u64::checked_from(other).unwrap(),
                            rm,
                        ) {
                            panic!("Right shift is not exact.");
                        }
                    }
                }
                self.trim();
            }
        }
    };
}
impl_natural_shr_unsigned!(u8);
impl_natural_shr_unsigned!(u16);
impl_natural_shr_unsigned!(u32);
impl_natural_shr_unsigned!(u64);
impl_natural_shr_unsigned!(u128);
