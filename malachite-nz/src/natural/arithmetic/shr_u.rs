use std::ops::{Shr, ShrAssign};

use malachite_base::num::arithmetic::traits::{Parity, ShrRound, ShrRoundAssign};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::round::RoundingMode;
use malachite_base::slices::slice_test_zero;
use malachite_base::vecs::vec_delete_left;

use natural::arithmetic::add::limbs_vec_add_limb_in_place;
use natural::arithmetic::divisible_by_power_of_two::limbs_divisible_by_power_of_two;
use natural::logic::bit_access::limbs_get_bit;
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
/// limbs of the `Natural` right-shifted by a `Limb`, rounding down.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = max(1, `xs.len()` - `bits` / Limb::WIDTH)
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
///
/// This is mpn_rshift from mpn/generic/rshift.c, GMP 6.1.2, where the result is returned.
pub fn limbs_shr(xs: &[Limb], bits: u64) -> Vec<Limb> {
    let delete_count = usize::exact_from(bits >> Limb::LOG_WIDTH);
    if delete_count >= xs.len() {
        Vec::new()
    } else {
        let mut out = xs[delete_count..].to_vec();
        let small_bits = bits & Limb::WIDTH_MASK;
        if small_bits != 0 {
            limbs_slice_shr_in_place(&mut out, small_bits);
        }
        out
    }
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
/// limbs of the `Natural` right-shifted by a `Limb`, rounding up. The limbs should not all be zero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = max(1, `xs.len()` - `bits` / Limb::WIDTH)
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
///
/// This is cfdiv_q_2exp from mpz/cfdiv_q_2exp.c, GMP 6.1.2, where u is non-negative, dir == 1, and
/// the result is returned.
pub fn limbs_shr_round_up(xs: &[Limb], bits: u64) -> Vec<Limb> {
    let delete_count = usize::exact_from(bits >> Limb::LOG_WIDTH);
    if delete_count >= xs.len() {
        vec![1]
    } else {
        let (xs_lo, xs_hi) = xs.split_at(delete_count);
        let mut exact = slice_test_zero(xs_lo);
        let mut out = xs_hi.to_vec();
        let small_bits = bits & Limb::WIDTH_MASK;
        if small_bits != 0 {
            exact &= limbs_slice_shr_in_place(&mut out, small_bits) == 0;
        }
        if !exact {
            limbs_vec_add_limb_in_place(&mut out, 1);
        }
        out
    }
}

fn limbs_shr_round_half_integer_to_even(xs: &[Limb], bits: u64) -> Vec<Limb> {
    let delete_count = usize::exact_from(bits >> Limb::LOG_WIDTH);
    if delete_count >= xs.len() {
        Vec::new()
    } else {
        let small_bits = bits & Limb::WIDTH_MASK;
        let mut out = xs[delete_count..].to_vec();
        if small_bits != 0 {
            limbs_slice_shr_in_place(&mut out, small_bits);
        }
        if !out.is_empty() && out[0].odd() {
            limbs_vec_add_limb_in_place(&mut out, 1);
        }
        out
    }
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
/// limbs of the `Natural` right-shifted by a `Limb`, rounding to the `Natural` nearest to the
/// actual value of `self` divided by 2<sup>`bits`</sup>. If the actual value is exactly between
/// two integers, it is rounded to the even one.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(m)
///
/// where n = `xs.len()`, m = max(1, `xs.len()` - `bits` / Limb::WIDTH)
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::shr_u::limbs_shr_round_nearest;
///
/// assert_eq!(limbs_shr_round_nearest(&[1], 1), &[0]);
/// assert_eq!(limbs_shr_round_nearest(&[3], 1), &[2]);
/// assert_eq!(limbs_shr_round_nearest(&[122, 456], 1), &[61, 228]);
/// assert_eq!(limbs_shr_round_nearest(&[123, 456], 1), &[62, 228]);
/// assert_eq!(limbs_shr_round_nearest(&[123, 455], 1), &[2_147_483_710, 227]);
/// assert_eq!(limbs_shr_round_nearest(&[123, 456], 31), &[912, 0]);
/// assert_eq!(limbs_shr_round_nearest(&[123, 456], 32), &[456]);
/// assert_eq!(limbs_shr_round_nearest(&[123, 456], 100), Vec::<u32>::new());
/// assert_eq!(limbs_shr_round_nearest(&[256, 456], 8), &[3_355_443_201, 1]);
/// assert_eq!(limbs_shr_round_nearest(&[4_294_967_295, 1], 1), &[0, 1]);
/// assert_eq!(limbs_shr_round_nearest(&[4_294_967_295, 4_294_967_295], 32), &[0, 1]);
/// ```
pub fn limbs_shr_round_nearest(xs: &[Limb], bits: u64) -> Vec<Limb> {
    if bits == 0 {
        xs.to_vec()
    } else if !limbs_get_bit(xs, bits - 1) {
        limbs_shr(xs, bits)
    } else if !limbs_divisible_by_power_of_two(xs, bits - 1) {
        limbs_shr_round_up(xs, bits)
    } else {
        limbs_shr_round_half_integer_to_even(xs, bits)
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
/// where n = `xs.len()`, m = max(1, `xs.len()` - `bits` / Limb::WIDTH)
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
pub fn limbs_shr_exact(xs: &[Limb], bits: u64) -> Option<Vec<Limb>> {
    if limbs_divisible_by_power_of_two(xs, bits) {
        Some(limbs_shr(xs, bits))
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
/// where n = `xs.len()`, m = max(1, `xs.len()` - `bits` / Limb::WIDTH)
///
/// # Example
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::round::RoundingMode;
/// use malachite_nz::natural::arithmetic::shr_u::limbs_shr_round;
///
/// assert_eq!(limbs_shr_round(&[1], 1, RoundingMode::Nearest), Some(vec![0]));
/// assert_eq!(limbs_shr_round(&[3], 1, RoundingMode::Nearest), Some(vec![2]));
/// assert_eq!(limbs_shr_round(&[122, 456], 1, RoundingMode::Floor), Some(vec![61, 228]));
/// assert_eq!(limbs_shr_round(&[123, 456], 1, RoundingMode::Floor), Some(vec![61, 228]));
/// assert_eq!(limbs_shr_round(&[123, 455], 1, RoundingMode::Floor),
///     Some(vec![2_147_483_709, 227]));
/// assert_eq!(limbs_shr_round(&[123, 456], 31, RoundingMode::Ceiling), Some(vec![913, 0]));
/// assert_eq!(limbs_shr_round(&[123, 456], 32, RoundingMode::Up), Some(vec![457]));
/// assert_eq!(limbs_shr_round(&[123, 456], 100, RoundingMode::Down), Some(vec![]));
/// assert_eq!(limbs_shr_round(&[256, 456], 8, RoundingMode::Exact), Some(vec![3_355_443_201, 1]));
/// assert_eq!(limbs_shr_round(&[4_294_967_295, 1], 1, RoundingMode::Exact), None);
/// assert_eq!(limbs_shr_round(&[4_294_967_295, 4_294_967_295], 32, RoundingMode::Down),
///     Some(vec![4_294_967_295]));
/// ```
pub fn limbs_shr_round(xs: &[Limb], bits: u64, rm: RoundingMode) -> Option<Vec<Limb>> {
    match rm {
        RoundingMode::Down | RoundingMode::Floor => Some(limbs_shr(xs, bits)),
        RoundingMode::Up | RoundingMode::Ceiling => Some(limbs_shr_round_up(xs, bits)),
        RoundingMode::Nearest => Some(limbs_shr_round_nearest(xs, bits)),
        RoundingMode::Exact => limbs_shr_exact(xs, bits),
    }
}

/// Interpreting a nonempty slice of `Limb`s as the limbs (in ascending order) of a `Natural`,
/// writes the limbs of the `Natural` right-shifted by a `Limb` to an output slice. The output slice
/// must be at least as long as the input slice. The `Limb` must be between 1 and `Limb::WIDTH` - 1,
/// inclusive. The carry, or the bits that are shifted past the width of the input slice, is
/// returned. The input slice should not only contain zeros.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `xs` is empty, `out` is shorter than `xs`, `bits` is 0, or `bits` is greater than or
/// equal to `Limb::WIDTH`.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::shr_u::limbs_shr_to_out;
///
/// let mut out = vec![0, 0, 0];
/// assert_eq!(limbs_shr_to_out(&mut out, &[123, 456], 1), 2_147_483_648);
/// assert_eq!(out, &[61, 228, 0]);
///
/// let mut out = vec![0, 0, 0];
/// assert_eq!(limbs_shr_to_out(&mut out, &[122, 455], 1), 0);
/// assert_eq!(out, &[2_147_483_709, 227, 0]);
/// ```
///
/// This is mpn_rshift from mpn/generic/rshift.c, GMP 6.1.2.
pub fn limbs_shr_to_out(out: &mut [Limb], xs: &[Limb], bits: u64) -> Limb {
    let len = xs.len();
    assert_ne!(len, 0);
    assert_ne!(bits, 0);
    assert!(bits < Limb::WIDTH);
    assert!(out.len() >= len);
    let cobits = Limb::WIDTH - bits;
    let (xs_head, xs_tail) = xs.split_first().unwrap();
    let remaining_bits = xs_head << cobits;
    let mut previous_x = xs_head >> bits;
    let (out_last, out_init) = out[..len].split_last_mut().unwrap();
    for (out, x) in out_init.iter_mut().zip(xs_tail.iter()) {
        *out = previous_x | (x << cobits);
        previous_x = x >> bits;
    }
    *out_last = previous_x;
    remaining_bits
}

/// Interpreting a nonempty slice of `Limb`s as the limbs (in ascending order) of a `Natural`,
/// writes the limbs of the `Natural` right-shifted by a `Limb` to the input slice. The `Limb` must
/// be between 1 and `Limb::WIDTH` - 1, inclusive. The carry, or the bits that are shifted past the
/// width of the input slice, is returned.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `xs` is empty, `bits` is 0, or `bits` is greater than or equal to `Limb::WIDTH`.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::shr_u::limbs_slice_shr_in_place;
///
/// let mut xs = vec![123, 456];
/// assert_eq!(limbs_slice_shr_in_place(&mut xs, 1), 2_147_483_648);
/// assert_eq!(xs, &[61, 228]);
///
/// let mut xs = vec![122, 455];
/// assert_eq!(limbs_slice_shr_in_place(&mut xs, 1), 0);
/// assert_eq!(xs, &[2_147_483_709, 227]);
/// ```
///
/// This is mpn_rshift from mpn/generic/rshift.c, GMP 6.1.2, where the rp == up.
pub fn limbs_slice_shr_in_place(xs: &mut [Limb], bits: u64) -> Limb {
    assert_ne!(bits, 0);
    assert!(bits < Limb::WIDTH);
    let len = xs.len();
    assert_ne!(len, 0);
    let cobits = Limb::WIDTH - bits;
    let mut x = xs[0];
    let remaining_bits = x << cobits;
    let mut previous_x = x >> bits;
    for i in 1..len {
        x = xs[i];
        xs[i - 1] = previous_x | (x << cobits);
        previous_x = x >> bits;
    }
    *xs.last_mut().unwrap() = previous_x;
    remaining_bits
}

/// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the `Natural` right-shifted by a `Limb` to the input `Vec`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = max(1, `xs.len()` - `bits` / Limb::WIDTH)
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::shr_u::limbs_vec_shr_in_place;
///
/// let mut xs = vec![1];
/// limbs_vec_shr_in_place(&mut xs, 1);
/// assert_eq!(xs, &[0]);
///
/// let mut xs = vec![3];
/// limbs_vec_shr_in_place(&mut xs, 1);
/// assert_eq!(xs, &[1]);
///
/// let mut xs = vec![122, 456];
/// limbs_vec_shr_in_place(&mut xs, 1);
/// assert_eq!(xs, &[61, 228]);
///
/// let mut xs = vec![123, 456];
/// limbs_vec_shr_in_place(&mut xs, 1);
/// assert_eq!(xs, &[61, 228]);
///
/// let mut xs = vec![123, 455];
/// limbs_vec_shr_in_place(&mut xs, 1);
/// assert_eq!(xs, &[2_147_483_709, 227]);
///
/// let mut xs = vec![123, 456];
/// limbs_vec_shr_in_place(&mut xs, 31);
/// assert_eq!(xs, &[912, 0]);
///
/// let mut xs = vec![123, 456];
/// limbs_vec_shr_in_place(&mut xs, 32);
/// assert_eq!(xs, &[456]);
///
/// let mut xs = vec![123, 456];
/// limbs_vec_shr_in_place(&mut xs, 100);
/// assert_eq!(xs, Vec::<u32>::new());
///
/// let mut xs = vec![256, 456];
/// limbs_vec_shr_in_place(&mut xs, 8);
/// assert_eq!(xs, &[3_355_443_201, 1]);
///
/// let mut xs = vec![4_294_967_295, 1];
/// limbs_vec_shr_in_place(&mut xs, 1);
/// assert_eq!(xs, &[4_294_967_295, 0]);
///
/// let mut xs = vec![4_294_967_295, 4_294_967_295];
/// limbs_vec_shr_in_place(&mut xs, 32);
/// assert_eq!(xs, &[4_294_967_295]);
/// ```
///
/// This is mpn_rshift from mpn/generic/rshift.c, GMP 6.1.2, where rp == up and if cnt is
/// sufficiently large, limbs are removed from rp.
pub fn limbs_vec_shr_in_place(xs: &mut Vec<Limb>, bits: u64) {
    let delete_count = usize::exact_from(bits >> Limb::LOG_WIDTH);
    if delete_count >= xs.len() {
        xs.clear();
    } else {
        let small_shift = bits & Limb::WIDTH_MASK;
        vec_delete_left(xs, delete_count);
        if small_shift != 0 {
            limbs_slice_shr_in_place(xs, small_shift);
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
/// where n = max(1, `xs.len()` - `bits` / Limb::WIDTH)
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::shr_u::limbs_vec_shr_round_up_in_place;
///
/// let mut xs = vec![1];
/// limbs_vec_shr_round_up_in_place(&mut xs, 1);
/// assert_eq!(xs, &[1]);
///
/// let mut xs = vec![3];
/// limbs_vec_shr_round_up_in_place(&mut xs, 1);
/// assert_eq!(xs, &[2]);
///
/// let mut xs = vec![122, 456];
/// limbs_vec_shr_round_up_in_place(&mut xs, 1);
/// assert_eq!(xs, &[61, 228]);
///
/// let mut xs = vec![123, 456];
/// limbs_vec_shr_round_up_in_place(&mut xs, 1);
/// assert_eq!(xs, &[62, 228]);
///
/// let mut xs = vec![123, 455];
/// limbs_vec_shr_round_up_in_place(&mut xs, 1);
/// assert_eq!(xs, &[2_147_483_710, 227]);
///
/// let mut xs = vec![123, 456];
/// limbs_vec_shr_round_up_in_place(&mut xs, 31);
/// assert_eq!(xs, &[913, 0]);
///
/// let mut xs = vec![123, 456];
/// limbs_vec_shr_round_up_in_place(&mut xs, 32);
/// assert_eq!(xs, &[457]);
///
/// let mut xs = vec![123, 456];
/// limbs_vec_shr_round_up_in_place(&mut xs, 100);
/// assert_eq!(xs, &[1]);
///
/// let mut xs = vec![256, 456];
/// limbs_vec_shr_round_up_in_place(&mut xs, 8);
/// assert_eq!(xs, &[3_355_443_201, 1]);
///
/// let mut xs = vec![4_294_967_295, 1];
/// limbs_vec_shr_round_up_in_place(&mut xs, 1);
/// assert_eq!(xs, &[0, 1]);
///
/// let mut xs = vec![4_294_967_295, 4_294_967_295];
/// limbs_vec_shr_round_up_in_place(&mut xs, 32);
/// assert_eq!(xs, &[0, 1]);
/// ```
///
/// This is cfdiv_q_2exp from mpz/cfdiv_q_2exp.c, GMP 6.1.2, where u is non-negative, dir == 1, and
/// w == u.
pub fn limbs_vec_shr_round_up_in_place(xs: &mut Vec<Limb>, bits: u64) {
    let delete_count = usize::exact_from(bits >> Limb::LOG_WIDTH);
    if delete_count >= xs.len() {
        xs.truncate(1);
        xs[0] = 1;
    } else {
        let mut exact = slice_test_zero(&xs[..delete_count]);
        let small_bits = bits & Limb::WIDTH_MASK;
        vec_delete_left(xs, delete_count);
        if small_bits != 0 {
            exact &= limbs_slice_shr_in_place(xs, small_bits) == 0;
        }
        if !exact {
            limbs_vec_add_limb_in_place(xs, 1);
        }
    }
}

fn limbs_vec_shr_round_half_integer_to_even_in_place(xs: &mut Vec<Limb>, bits: u64) {
    let delete_count = usize::exact_from(bits >> Limb::LOG_WIDTH);
    if delete_count >= xs.len() {
        xs.clear();
    } else {
        let small_bits = bits & Limb::WIDTH_MASK;
        vec_delete_left(xs, delete_count);
        if small_bits != 0 {
            limbs_slice_shr_in_place(xs, small_bits);
        }
        if !xs.is_empty() && xs[0].odd() {
            limbs_vec_add_limb_in_place(xs, 1);
        }
    }
}

/// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the `Natural` right-shifted by a `Limb` to the input `Vec`, rounding to the `Natural`
/// nearest to the actual value of `self` divided by 2<sup>`bits`</sup>. If the actual value is
/// exactly between two integers, it is rounded to the even one.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::shr_u::limbs_vec_shr_round_nearest_in_place;
///
/// let mut xs = vec![1];
/// limbs_vec_shr_round_nearest_in_place(&mut xs, 1);
/// assert_eq!(xs, &[0]);
///
/// let mut xs = vec![3];
/// limbs_vec_shr_round_nearest_in_place(&mut xs, 1);
/// assert_eq!(xs, &[2]);
///
/// let mut xs = vec![122, 456];
/// limbs_vec_shr_round_nearest_in_place(&mut xs, 1);
/// assert_eq!(xs, &[61, 228]);
///
/// let mut xs = vec![123, 456];
/// limbs_vec_shr_round_nearest_in_place(&mut xs, 1);
/// assert_eq!(xs, &[62, 228]);
///
/// let mut xs = vec![123, 455];
/// limbs_vec_shr_round_nearest_in_place(&mut xs, 1);
/// assert_eq!(xs, &[2_147_483_710, 227]);
///
/// let mut xs = vec![123, 456];
/// limbs_vec_shr_round_nearest_in_place(&mut xs, 31);
/// assert_eq!(xs, &[912, 0]);
///
/// let mut xs = vec![123, 456];
/// limbs_vec_shr_round_nearest_in_place(&mut xs, 32);
/// assert_eq!(xs, &[456]);
///
/// let mut xs = vec![123, 456];
/// limbs_vec_shr_round_nearest_in_place(&mut xs, 100);
/// assert_eq!(xs, Vec::<u32>::new());
///
/// let mut xs = vec![256, 456];
/// limbs_vec_shr_round_nearest_in_place(&mut xs, 8);
/// assert_eq!(xs, &[3_355_443_201, 1]);
///
/// let mut xs = vec![4_294_967_295, 1];
/// limbs_vec_shr_round_nearest_in_place(&mut xs, 1);
/// assert_eq!(xs, &[0, 1]);
///
/// let mut xs = vec![4_294_967_295, 4_294_967_295];
/// limbs_vec_shr_round_nearest_in_place(&mut xs, 32);
/// assert_eq!(xs, &[0, 1]);
/// ```
pub fn limbs_vec_shr_round_nearest_in_place(xs: &mut Vec<Limb>, bits: u64) {
    if bits == 0 {
    } else if !limbs_get_bit(xs, bits - 1) {
        limbs_vec_shr_in_place(xs, bits)
    } else if !limbs_divisible_by_power_of_two(xs, bits - 1) {
        limbs_vec_shr_round_up_in_place(xs, bits)
    } else {
        limbs_vec_shr_round_half_integer_to_even_in_place(xs, bits)
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
/// where n = `xs.len()`
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::shr_u::limbs_vec_shr_exact_in_place;
///
/// let mut xs = vec![1];
/// assert_eq!(limbs_vec_shr_exact_in_place(&mut xs, 1), false);
///
/// let mut xs = vec![3];
/// assert_eq!(limbs_vec_shr_exact_in_place(&mut xs, 1), false);
///
/// let mut xs = vec![122, 456];
/// assert_eq!(limbs_vec_shr_exact_in_place(&mut xs, 1), true);
/// assert_eq!(xs, &[61, 228]);
///
/// let mut xs = vec![123, 456];
/// assert_eq!(limbs_vec_shr_exact_in_place(&mut xs, 1), false);
///
/// let mut xs = vec![123, 455];
/// assert_eq!(limbs_vec_shr_exact_in_place(&mut xs, 1), false);
///
/// let mut xs = vec![123, 456];
/// assert_eq!(limbs_vec_shr_exact_in_place(&mut xs, 31), false);
///
/// let mut xs = vec![123, 456];
/// assert_eq!(limbs_vec_shr_exact_in_place(&mut xs, 32), false);
///
/// let mut xs = vec![123, 456];
/// assert_eq!(limbs_vec_shr_exact_in_place(&mut xs, 100), false);
///
/// let mut xs = vec![256, 456];
/// assert_eq!(limbs_vec_shr_exact_in_place(&mut xs, 8), true);
/// assert_eq!(xs, &[3_355_443_201, 1]);
///
/// let mut xs = vec![4_294_967_295, 1];
/// assert_eq!(limbs_vec_shr_exact_in_place(&mut xs, 1), false);
///
/// let mut xs = vec![4_294_967_295, 4_294_967_295];
/// assert_eq!(limbs_vec_shr_exact_in_place(&mut xs, 32), false);
/// ```
pub fn limbs_vec_shr_exact_in_place(xs: &mut Vec<Limb>, bits: u64) -> bool {
    if limbs_divisible_by_power_of_two(xs, bits) {
        limbs_vec_shr_in_place(xs, bits);
        true
    } else {
        false
    }
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the `Natural` right-shifted by a `Limb` to the input `Vec`, rounded using a specified
/// rounding format. If the shift is inexact (removes some `true` bits) and the `RoundingMode` is
/// `Exact`, the value of `xs` becomes unspecified and `false` is returned. Otherwise, `true` is
/// returned. The limbs should not all be zero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Example
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::round::RoundingMode;
/// use malachite_nz::natural::arithmetic::shr_u::limbs_vec_shr_round_in_place;
///
/// let mut xs = vec![1];
/// assert_eq!(limbs_vec_shr_round_in_place(&mut xs, 1, RoundingMode::Nearest), true);
/// assert_eq!(xs, &[0]);
///
/// let mut xs = vec![3];
/// assert_eq!(limbs_vec_shr_round_in_place(&mut xs, 1, RoundingMode::Nearest), true);
/// assert_eq!(xs, &[2]);
///
/// let mut xs = vec![122, 456];
/// assert_eq!(limbs_vec_shr_round_in_place(&mut xs, 1, RoundingMode::Floor), true);
/// assert_eq!(xs, &[61, 228]);
///
/// let mut xs = vec![123, 456];
/// assert_eq!(limbs_vec_shr_round_in_place(&mut xs, 1, RoundingMode::Floor), true);
/// assert_eq!(xs, &[61, 228]);
///
/// let mut xs = vec![123, 455];
/// assert_eq!(limbs_vec_shr_round_in_place(&mut xs, 1, RoundingMode::Floor), true);
/// assert_eq!(xs, &[2_147_483_709, 227]);
///
/// let mut xs = vec![123, 456];
/// assert_eq!(limbs_vec_shr_round_in_place(&mut xs, 31, RoundingMode::Ceiling), true);
/// assert_eq!(xs, &[913, 0]);
///
/// let mut xs = vec![123, 456];
/// assert_eq!(limbs_vec_shr_round_in_place(&mut xs, 32, RoundingMode::Up), true);
/// assert_eq!(xs, &[457]);
///
/// let mut limbs = vec![123, 456];
/// assert_eq!(limbs_vec_shr_round_in_place(&mut limbs, 100, RoundingMode::Down), true);
/// assert_eq!(limbs, Vec::<u32>::new());
///
/// let mut xs = vec![256, 456];
/// assert_eq!(limbs_vec_shr_round_in_place(&mut xs, 8, RoundingMode::Exact), true);
/// assert_eq!(xs, vec![3_355_443_201, 1]);
///
/// let mut xs = vec![4_294_967_295, 1];
/// assert_eq!(limbs_vec_shr_round_in_place(&mut xs, 1, RoundingMode::Exact), false);
///
/// let mut xs = vec![4_294_967_295, 4_294_967_295];
/// assert_eq!(limbs_vec_shr_round_in_place(&mut xs, 32, RoundingMode::Down), true);
/// assert_eq!(xs, vec![4_294_967_295]);
/// ```
pub fn limbs_vec_shr_round_in_place(xs: &mut Vec<Limb>, bits: u64, rm: RoundingMode) -> bool {
    match rm {
        RoundingMode::Down | RoundingMode::Floor => {
            limbs_vec_shr_in_place(xs, bits);
            true
        }
        RoundingMode::Up | RoundingMode::Ceiling => {
            limbs_vec_shr_round_up_in_place(xs, bits);
            true
        }
        RoundingMode::Nearest => {
            limbs_vec_shr_round_nearest_in_place(xs, bits);
            true
        }
        RoundingMode::Exact => limbs_vec_shr_exact_in_place(xs, bits),
    }
}

macro_rules! impl_natural_shr_unsigned {
    ($t:ident) => {
        impl Shr<$t> for Natural {
            type Output = Natural;

            /// Shifts a `Natural` right (divides it by a power of 2 and takes the floor), taking
            /// the `Natural` by value.
            ///
            /// Time: worst case O(n)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// where n = max(1, `self.significant_bits()` - `bits`)
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use malachite_base::num::basic::traits::Zero;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!((Natural::ZERO >> 10u8).to_string(), "0");
            /// assert_eq!((Natural::from(492u32) >> 2u32).to_string(), "123");
            /// assert_eq!((Natural::trillion() >> 10u64).to_string(), "976562500");
            /// ```
            #[inline]
            fn shr(mut self, bits: $t) -> Natural {
                self >>= bits;
                self
            }
        }

        impl<'a> Shr<$t> for &'a Natural {
            type Output = Natural;

            /// Shifts a `Natural` right (divides it by a power of 2 and takes the floor), taking
            /// the `Natural` by reference.
            ///
            /// Time: worst case O(n)
            ///
            /// Additional memory: worst case O(n)
            ///
            /// where n = max(1, `self.significant_bits()` - `bits`)
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use malachite_base::num::basic::traits::Zero;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!((&Natural::ZERO >> 10u8).to_string(), "0");
            /// assert_eq!((&Natural::from(492u32) >> 2u32).to_string(), "123");
            /// assert_eq!((&Natural::trillion() >> 10u64).to_string(), "976562500");
            /// ```
            fn shr(self, bits: $t) -> Natural {
                match (self, bits) {
                    (_, 0) | (natural_zero!(), _) => self.clone(),
                    (Natural(Small(_)), bits) if bits >= $t::wrapping_from(Limb::WIDTH) => {
                        Natural::ZERO
                    }
                    (Natural(Small(small)), bits) => Natural(Small(small >> bits)),
                    (Natural(Large(ref limbs)), bits) => {
                        Natural::from_owned_limbs_asc(limbs_shr(limbs, u64::exact_from(bits)))
                    }
                }
            }
        }

        //TODO
        impl ShrAssign<$t> for Natural {
            /// Shifts a `Natural` right (divides it by a power of 2 and takes the floor) in place.
            ///
            /// Time: worst case O(n)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// where n = max(1, `self.significant_bits()` - `bits`)
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
            fn shr_assign(&mut self, bits: $t) {
                match (&mut *self, bits) {
                    (_, 0) | (natural_zero!(), _) => {}
                    (Natural(Small(ref mut small)), bits)
                        if bits >= $t::wrapping_from(Limb::WIDTH) =>
                    {
                        *small = 0;
                    }
                    (Natural(Small(ref mut small)), bits) => {
                        *small >>= bits;
                    }
                    (Natural(Large(ref mut limbs)), bits) => {
                        limbs_vec_shr_in_place(limbs, u64::exact_from(bits));
                        self.trim();
                    }
                }
            }
        }

        impl ShrRound<$t> for Natural {
            type Output = Natural;

            /// Shifts a `Natural` right (divides it by a power of 2) and rounds according to the
            /// specified rounding mode, taking the `Natural` by value. Passing
            /// `RoundingMode::Floor` or `RoundingMode::Down` is equivalent to using `>>`. To test
            /// whether `RoundingMode::Exact` can be passed, use
            /// `self.divisible_by_power_of_two(bits)`.
            ///
            /// Time: worst case O(n)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// where n = `xs.len()`
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
            /// use malachite_base::round::RoundingMode;
            /// use malachite_base::num::arithmetic::traits::ShrRound;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(Natural::from(0x101u32).shr_round(8u8, RoundingMode::Down).to_string(),
            ///     "1");
            /// assert_eq!(
            ///     Natural::from(0x101u32).shr_round(8u16, RoundingMode::Up).to_string(),
            ///     "2"
            /// );
            ///
            /// assert_eq!(Natural::from(0x101u32).shr_round(9u32, RoundingMode::Down).to_string(),
            ///     "0");
            /// assert_eq!(
            ///     Natural::from(0x101u32).shr_round(9u64, RoundingMode::Up).to_string(),
            ///     "1"
            /// );
            /// assert_eq!(
            ///     Natural::from(0x101u32).shr_round(9u8, RoundingMode::Nearest).to_string(),
            ///     "1"
            /// );
            /// assert_eq!(
            ///     Natural::from(0xffu32).shr_round(9u16, RoundingMode::Nearest).to_string(),
            ///     "0"
            /// );
            /// assert_eq!(
            ///     Natural::from(0x100u32).shr_round(9u32, RoundingMode::Nearest).to_string(),
            ///     "0"
            /// );
            ///
            /// assert_eq!(Natural::from(0x100u32).shr_round(8u64, RoundingMode::Exact).to_string(),
            ///     "1");
            /// ```
            #[inline]
            fn shr_round(mut self, bits: $t, rm: RoundingMode) -> Natural {
                self.shr_round_assign(bits, rm);
                self
            }
        }

        impl<'a> ShrRound<$t> for &'a Natural {
            type Output = Natural;

            /// Shifts a `Natural` right (divides it by a power of 2) and rounds according to the
            /// specified rounding mode, taking the `Natural` by reference. Passing
            /// `RoundingMode::Floor` or `RoundingMode::Down` is equivalent to using `>>`. To test
            /// whether `RoundingMode::Exact` can be passed, use
            /// `self.divisible_by_power_of_two(bits)`.
            ///
            /// Time: worst case O(n)
            ///
            /// Additional memory: worst case O(m)
            ///
            /// where n = `self.significant_bits()`, m = max(1, `self.significant_bits()` - `bits`)
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
            /// use malachite_base::round::RoundingMode;
            /// use malachite_base::num::arithmetic::traits::ShrRound;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(
            ///     (&Natural::from(0x101u32)).shr_round(8u8, RoundingMode::Down).to_string(),
            ///     "1"
            /// );
            /// assert_eq!(
            ///     (&Natural::from(0x101u32)).shr_round(8u16, RoundingMode::Up).to_string(),
            ///     "2"
            /// );
            ///
            /// assert_eq!(
            ///     (&Natural::from(0x101u32)).shr_round(9u32, RoundingMode::Down).to_string(),
            ///     "0"
            /// );
            /// assert_eq!((&Natural::from(0x101u32)).shr_round(9u64, RoundingMode::Up).to_string(),
            ///     "1");
            /// assert_eq!(
            ///     (&Natural::from(0x101u32)).shr_round(9u8, RoundingMode::Nearest).to_string(),
            ///     "1"
            /// );
            /// assert_eq!(
            ///     (&Natural::from(0xffu32)).shr_round(9u16, RoundingMode::Nearest).to_string(),
            ///     "0"
            /// );
            /// assert_eq!((&Natural::from(0x100u32)).shr_round(9u32, RoundingMode::Nearest)
            ///     .to_string(), "0");
            ///
            /// assert_eq!(
            ///     (&Natural::from(0x100u32)).shr_round(8u64, RoundingMode::Exact).to_string(),
            ///     "1"
            /// );
            /// ```
            fn shr_round(self, bits: $t, rm: RoundingMode) -> Natural {
                match (self, bits) {
                    (_, 0) | (natural_zero!(), _) => self.clone(),
                    (Natural(Small(ref small)), bits) => Natural(Small(small.shr_round(bits, rm))),
                    (Natural(Large(ref limbs)), bits) => {
                        if let Some(out) = limbs_shr_round(limbs, u64::exact_from(bits), rm) {
                            Natural::from_owned_limbs_asc(out)
                        } else {
                            panic!("Right shift is not exact: {} >> {}", self, bits);
                        }
                    }
                }
            }
        }

        impl ShrRoundAssign<$t> for Natural {
            /// Shifts a `Natural` right (divides it by a power of 2) and rounds according to the
            /// specified rounding mode, in place. Passing `RoundingMode::Floor` or
            /// `RoundingMode::Down` is equivalent to using `>>=`. To test whether
            /// `RoundingMode::Exact` can be passed, use `self.divisible_by_power_of_two(bits)`.
            ///
            /// Time: worst case O(n)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// where n = `xs.len()`
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
            /// use malachite_base::round::RoundingMode;
            /// use malachite_base::num::arithmetic::traits::ShrRoundAssign;
            /// use malachite_nz::natural::Natural;
            ///
            /// let mut n = Natural::from(0x101u32);
            /// n.shr_round_assign(8u8, RoundingMode::Down);
            /// assert_eq!(n.to_string(), "1");
            ///
            /// let mut n = Natural::from(0x101u32);
            /// n.shr_round_assign(8u16, RoundingMode::Up);
            /// assert_eq!(n.to_string(), "2");
            ///
            /// let mut n = Natural::from(0x101u32);
            /// n.shr_round_assign(9u32, RoundingMode::Down);
            /// assert_eq!(n.to_string(), "0");
            ///
            /// let mut n = Natural::from(0x101u32);
            /// n.shr_round_assign(9u64, RoundingMode::Up);
            /// assert_eq!(n.to_string(), "1");
            ///
            /// let mut n = Natural::from(0x101u32);
            /// n.shr_round_assign(9u8, RoundingMode::Nearest);
            /// assert_eq!(n.to_string(), "1");
            ///
            /// let mut n = Natural::from(0xffu32);
            /// n.shr_round_assign(9u16, RoundingMode::Nearest);
            /// assert_eq!(n.to_string(), "0");
            ///
            /// let mut n = Natural::from(0x100u32);
            /// n.shr_round_assign(9u32, RoundingMode::Nearest);
            /// assert_eq!(n.to_string(), "0");
            ///
            /// let mut n = Natural::from(0x100u32);
            /// n.shr_round_assign(8u64, RoundingMode::Exact);
            /// assert_eq!(n.to_string(), "1");
            /// ```
            fn shr_round_assign(&mut self, bits: $t, rm: RoundingMode) {
                match (&mut *self, bits) {
                    (_, 0) | (natural_zero!(), _) => {}
                    (Natural(Small(ref mut small)), bits) => {
                        small.shr_round_assign(bits, rm);
                    }
                    (Natural(Large(ref mut limbs)), bits) => {
                        if !limbs_vec_shr_round_in_place(limbs, u64::exact_from(bits), rm) {
                            panic!("Right shift is not exact.");
                        }
                        self.trim();
                    }
                }
            }
        }
    };
}
impl_natural_shr_unsigned!(u8);
impl_natural_shr_unsigned!(u16);
impl_natural_shr_unsigned!(u32);
impl_natural_shr_unsigned!(u64);
impl_natural_shr_unsigned!(u128);
impl_natural_shr_unsigned!(usize);
