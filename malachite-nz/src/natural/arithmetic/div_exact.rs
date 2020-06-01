use std::cmp::{max, min, Ordering};
use std::mem::swap;

use malachite_base::num::arithmetic::traits::{
    DivExact, DivExactAssign, ModPowerOfTwo, Parity, ShrRound, ShrRoundAssign, WrappingSubAssign,
};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, SplitInHalf};
use malachite_base::num::logic::traits::TrailingZeros;
use malachite_base::round::RoundingMode;
use malachite_base::slices::{slice_leading_zeros, slice_set_zero, slice_test_zero};

use fail_on_untested_path;
use integer::conversion::to_twos_complement_limbs::limbs_twos_complement_in_place;
use natural::arithmetic::add::{
    limbs_slice_add_greater_in_place_left, limbs_slice_add_limb_in_place,
    limbs_slice_add_same_length_in_place_left,
};
use natural::arithmetic::add_mul::limbs_slice_add_mul_limb_same_length_in_place_left;
use natural::arithmetic::div::{
    limbs_div_divisor_of_limb_max_with_carry_in_place,
    limbs_div_divisor_of_limb_max_with_carry_to_out,
};
use natural::arithmetic::div_mod::MUL_TO_MULMOD_BNM1_FOR_2NXN_THRESHOLD;
use natural::arithmetic::mul::mul_low::limbs_mul_low_same_length;
use natural::arithmetic::mul::mul_mod::{
    _limbs_mul_mod_base_pow_n_minus_1, _limbs_mul_mod_base_pow_n_minus_1_next_size,
    _limbs_mul_mod_base_pow_n_minus_1_scratch_len,
};
use natural::arithmetic::mul::{limbs_mul_greater_to_out, limbs_mul_to_out};
use natural::arithmetic::shr_u::{limbs_shr_to_out, limbs_slice_shr_in_place};
use natural::arithmetic::sub::{
    _limbs_sub_same_length_with_borrow_in_to_out, limbs_sub_in_place_left, limbs_sub_limb_in_place,
    limbs_sub_limb_to_out, limbs_sub_same_length_in_place_left, limbs_sub_same_length_to_out,
    limbs_sub_same_length_to_out_with_overlap,
};
use natural::arithmetic::sub_mul::limbs_sub_mul_limb_same_length_in_place_left;
use natural::comparison::ord::limbs_cmp_same_length;
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::{
    DoubleLimb, Limb, BINV_NEWTON_THRESHOLD, DC_BDIV_QR_THRESHOLD, DC_BDIV_Q_THRESHOLD,
    MU_BDIV_Q_THRESHOLD,
};

const INVERT_LIMB_TABLE_LOG_SIZE: u64 = 7;

const INVERT_LIMB_TABLE_SIZE: usize = 1 << INVERT_LIMB_TABLE_LOG_SIZE;

// The entry at index `i` is the multiplicative inverse of 2 * `i` + 1 mod 2<sup>8</sup>.
const INVERT_LIMB_TABLE: [u8; INVERT_LIMB_TABLE_SIZE] = [
    0x01, 0xAB, 0xCD, 0xB7, 0x39, 0xA3, 0xC5, 0xEF, 0xF1, 0x1B, 0x3D, 0xA7, 0x29, 0x13, 0x35, 0xDF,
    0xE1, 0x8B, 0xAD, 0x97, 0x19, 0x83, 0xA5, 0xCF, 0xD1, 0xFB, 0x1D, 0x87, 0x09, 0xF3, 0x15, 0xBF,
    0xC1, 0x6B, 0x8D, 0x77, 0xF9, 0x63, 0x85, 0xAF, 0xB1, 0xDB, 0xFD, 0x67, 0xE9, 0xD3, 0xF5, 0x9F,
    0xA1, 0x4B, 0x6D, 0x57, 0xD9, 0x43, 0x65, 0x8F, 0x91, 0xBB, 0xDD, 0x47, 0xC9, 0xB3, 0xD5, 0x7F,
    0x81, 0x2B, 0x4D, 0x37, 0xB9, 0x23, 0x45, 0x6F, 0x71, 0x9B, 0xBD, 0x27, 0xA9, 0x93, 0xB5, 0x5F,
    0x61, 0x0B, 0x2D, 0x17, 0x99, 0x03, 0x25, 0x4F, 0x51, 0x7B, 0x9D, 0x07, 0x89, 0x73, 0x95, 0x3F,
    0x41, 0xEB, 0x0D, 0xF7, 0x79, 0xE3, 0x05, 0x2F, 0x31, 0x5B, 0x7D, 0xE7, 0x69, 0x53, 0x75, 0x1F,
    0x21, 0xCB, 0xED, 0xD7, 0x59, 0xC3, 0xE5, 0x0F, 0x11, 0x3B, 0x5D, 0xC7, 0x49, 0x33, 0x55, 0xFF,
];

/// Tests that `INVERT_LIMB_TABLE` is correct.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::div_exact::test_invert_limb_table;
///
/// test_invert_limb_table();
/// ```
pub fn test_invert_limb_table() {
    for (i, &inv) in INVERT_LIMB_TABLE.iter().enumerate() {
        let value = (u8::exact_from(i) << 1) + 1;
        let product = value.wrapping_mul(inv);
        assert_eq!(
            product, 1,
            "INVERT_LIMB_TABLE gives incorrect inverse, {}, for value {}",
            inv, value
        );
    }
}

/// Finds the inverse of a `Limb` mod 2<sup>`Limb::WIDTH`</sup>; given x, returns y such that
/// x * y === 1 mod 2<sup>`Limb::WIDTH`</sup>. This inverse only exists for odd `Limb`s, so `limb`
/// must be odd.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Panics
/// Panics if `limb` is even.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::div_exact::limbs_modular_invert_limb;
///
/// assert_eq!(limbs_modular_invert_limb(3), 2_863_311_531);
/// assert_eq!(limbs_modular_invert_limb(1_000_000_001), 2_211_001_857);
/// ```
///
/// This is binvert_limb from gmp-impl.h, GMP 6.1.2.
pub fn limbs_modular_invert_limb(limb: Limb) -> Limb {
    assert!(limb.odd());
    let index = (limb >> 1).mod_power_of_two(INVERT_LIMB_TABLE_LOG_SIZE);
    let mut inv = Limb::from(INVERT_LIMB_TABLE[usize::exact_from(index)]);
    inv = (inv << 1).wrapping_sub((inv * inv).wrapping_mul(limb));
    inv = (inv << 1).wrapping_sub(inv.wrapping_mul(inv).wrapping_mul(limb));
    if !cfg!(feature = "32_bit_limbs") {
        inv = (inv << 1).wrapping_sub(inv.wrapping_mul(inv).wrapping_mul(limb));
    }
    inv
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
/// quotient limbs of the `Natural` divided by a `Limb`. The divisor limb cannot be zero and the
/// limb slice must be nonempty. The `Natural` must be exactly divisible by the `Limb`. If it isn't,
/// the behavior of this function is undefined.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `limbs` is empty or if `d` is zero.
///
/// This is mpn_divexact_1 from mpn/generic/dive_1.c, GMP 6.1.2, where the result is returned.
pub fn _limbs_div_exact_limb_no_special_3(xs: &[Limb], d: Limb) -> Vec<Limb> {
    let mut q = vec![0; xs.len()];
    limbs_div_exact_limb_to_out(&mut q, xs, d);
    q
}

/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `out` is shorter than `in_limbs`, `in_limbs` is empty, or if `d` is zero.
///
/// This is mpn_divexact_1 from mpn/generic/dive_1.c, GMP 6.1.2.
pub fn _limbs_div_exact_limb_to_out_no_special_3(out: &mut [Limb], xs: &[Limb], d: Limb) {
    assert_ne!(d, 0);
    let len = xs.len();
    assert_ne!(len, 0);
    assert!(out.len() >= len);
    if d.even() {
        let shift = TrailingZeros::trailing_zeros(d);
        let shift_complement = Limb::WIDTH - shift;
        let shifted_d = d >> shift;
        let d_inv = limbs_modular_invert_limb(shifted_d);
        let mut upper_half = 0;
        let mut previous_in_limb = xs[0];
        for i in 1..len {
            let in_limb = xs[i];
            let shifted_in_limb = (previous_in_limb >> shift) | (in_limb << shift_complement);
            previous_in_limb = in_limb;
            let (diff, carry) = shifted_in_limb.overflowing_sub(upper_half);
            let out_limb = diff.wrapping_mul(d_inv);
            out[i - 1] = out_limb;
            upper_half = (DoubleLimb::from(out_limb) * DoubleLimb::from(shifted_d)).upper_half();
            if carry {
                upper_half += 1;
            }
        }
        out[len - 1] = (previous_in_limb >> shift)
            .wrapping_sub(upper_half)
            .wrapping_mul(d_inv);
    } else {
        let d_inv = limbs_modular_invert_limb(d);
        let mut out_limb = xs[0].wrapping_mul(d_inv);
        out[0] = out_limb;
        let mut previous_carry = false;
        for i in 1..len {
            let mut upper_half = (DoubleLimb::from(out_limb) * DoubleLimb::from(d)).upper_half();
            if previous_carry {
                upper_half += 1;
            }
            let (diff, carry) = xs[i].overflowing_sub(upper_half);
            previous_carry = carry;
            out_limb = diff.wrapping_mul(d_inv);
            out[i] = out_limb;
        }
    }
}

/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `limbs` is empty or if `d` is zero.
///
/// This is mpn_divexact_1 from mpn/generic/dive_1.c, GMP 6.1.2, where dst == src.
pub fn _limbs_div_exact_limb_in_place_no_special_3(xs: &mut [Limb], d: Limb) {
    assert_ne!(d, 0);
    let len = xs.len();
    assert_ne!(len, 0);
    if d.even() {
        let shift = TrailingZeros::trailing_zeros(d);
        let shift_complement = Limb::WIDTH - shift;
        let shifted_d = d >> shift;
        let d_inv = limbs_modular_invert_limb(shifted_d);
        let shifted_d = DoubleLimb::from(shifted_d);
        let mut upper_half = 0;
        let mut previous_in_limb = xs[0];
        for i in 1..len {
            let in_limb = xs[i];
            let shifted_in_limb = (previous_in_limb >> shift) | (in_limb << shift_complement);
            previous_in_limb = in_limb;
            let (diff, carry) = shifted_in_limb.overflowing_sub(upper_half);
            let out_limb = diff.wrapping_mul(d_inv);
            xs[i - 1] = out_limb;
            upper_half = (DoubleLimb::from(out_limb) * shifted_d).upper_half();
            if carry {
                upper_half += 1;
            }
        }
        xs[len - 1] = (previous_in_limb >> shift)
            .wrapping_sub(upper_half)
            .wrapping_mul(d_inv);
    } else {
        let d_inv = limbs_modular_invert_limb(d);
        let d = DoubleLimb::from(d);
        let mut out_limb = xs[0].wrapping_mul(d_inv);
        xs[0] = out_limb;
        let mut previous_carry = false;
        for limb in xs[1..].iter_mut() {
            let mut upper_half = (DoubleLimb::from(out_limb) * d).upper_half();
            if previous_carry {
                upper_half += 1;
            }
            let (diff, carry) = limb.overflowing_sub(upper_half);
            previous_carry = carry;
            out_limb = diff.wrapping_mul(d_inv);
            *limb = out_limb;
        }
    }
}

pub const MAX_OVER_3: Limb = Limb::MAX / 3;

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
/// quotient limbs of the `Natural` divided by 3. The limb slice must be nonempty. The `Natural`
/// must be exactly divisible by 3. If it isn't, the behavior of this function is undefined.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `limbs` is empty.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::div_exact::limbs_div_exact_3;
///
/// assert_eq!(limbs_div_exact_3(&[8, 7]), &[1_431_655_768, 2]);
/// assert_eq!(limbs_div_exact_3(&[0xffff_ffff, 0xffff_ffff]), &[0x5555_5555, 0x5555_5555]);
/// ```
///
/// This is mpn_divexact_by3c from mpn/generic diveby3.c, GMP 6.1.2, with DIVEXACT_BY3_METHOD == 0
/// and no carry-in, where the result is returned.
pub fn limbs_div_exact_3(xs: &[Limb]) -> Vec<Limb> {
    let mut q = vec![0; xs.len()];
    limbs_div_exact_3_to_out(&mut q, xs);
    q
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the quotient of the `Natural` and 3 to an output slice. The output slice must be at
/// least as long as the input slice. The input limb slice must be nonempty. The `Natural` must be
/// exactly divisible by 3. If it isn't, the behavior of this function is undefined.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `out` is shorter than `xs` or if `xs` is empty.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::div_exact::limbs_div_exact_3_to_out;
///
/// let mut out = vec![10, 10, 10, 10];
/// limbs_div_exact_3_to_out(&mut out, &[8, 7]);
/// assert_eq!(out, &[1_431_655_768, 2, 10, 10]);
///
/// let mut out = vec![10, 10, 10, 10];
/// limbs_div_exact_3_to_out(&mut out, &[0xffff_ffff, 0xffff_ffff]);
/// assert_eq!(out, &[0x5555_5555, 0x5555_5555, 10, 10]);
/// ```
///
/// This is mpn_divexact_by3c from mpn/generic diveby3.c, GMP 6.1.2, with DIVEXACT_BY3_METHOD == 0,
/// no carry-in, and no return value.
pub fn limbs_div_exact_3_to_out(out: &mut [Limb], xs: &[Limb]) {
    assert!(out.len() >= xs.len());
    let (xs_last, xs_init) = xs.split_last().unwrap();
    let out_limb = limbs_div_divisor_of_limb_max_with_carry_to_out(out, xs_init, MAX_OVER_3, 0);
    let lower = (DoubleLimb::from(*xs_last) * DoubleLimb::from(MAX_OVER_3)).lower_half();
    out[xs.len() - 1] = out_limb.wrapping_sub(lower);
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the quotient of the `Natural` and 3 to the input slice. The input limb slice must be
/// nonempty. The `Natural` must be exactly divisible by 3. If it isn't, the behavior of this
/// function is undefined.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `limbs` is empty.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::div_exact::limbs_div_exact_3_in_place;
///
/// let mut limbs = vec![8, 7];
/// limbs_div_exact_3_in_place(&mut limbs);
/// assert_eq!(limbs, &[1_431_655_768, 2]);
///
/// let mut limbs = vec![0xffff_ffff, 0xffff_ffff];
/// limbs_div_exact_3_in_place(&mut limbs);
/// assert_eq!(limbs, &[0x5555_5555, 0x5555_5555]);
/// ```
/// This is mpn_divexact_by3c from mpn/generic diveby3.c, GMP 6.1.2, with DIVEXACT_BY3_METHOD == 0,
/// no carry-in, and no return value, where rp == up.
pub fn limbs_div_exact_3_in_place(xs: &mut [Limb]) {
    let (xs_last, xs_init) = xs.split_last_mut().unwrap();
    let out_limb = limbs_div_divisor_of_limb_max_with_carry_in_place(xs_init, MAX_OVER_3, 0);
    let lower = (DoubleLimb::from(*xs_last) * DoubleLimb::from(MAX_OVER_3)).lower_half();
    *xs_last = out_limb.wrapping_sub(lower);
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the quotient of the `Natural` and a `Limb` to an output slice. The output slice must be
/// at least as long as the input slice. The divisor limb cannot be zero and the input limb slice
/// must be nonempty. The `Natural` must be exactly divisible by the `Limb`. If it isn't, the
/// behavior of this function is undefined.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `out` is shorter than `in_limbs`, `in_limbs` is empty, or if `d` is zero.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::div_exact::limbs_div_exact_limb_to_out;
///
/// let mut out = vec![10, 10, 10, 10];
/// limbs_div_exact_limb_to_out(&mut out, &[6, 7], 2);
/// assert_eq!(out, &[2_147_483_651, 3, 10, 10]);
///
/// let mut out = vec![10, 10, 10, 10];
/// limbs_div_exact_limb_to_out(&mut out, &[0xffff_ffff, 0xffff_ffff], 3);
/// assert_eq!(out, &[0x5555_5555, 0x5555_5555, 10, 10]);
/// ```
pub fn limbs_div_exact_limb_to_out(out: &mut [Limb], xs: &[Limb], d: Limb) {
    if d == 3 {
        limbs_div_exact_3_to_out(out, xs)
    } else {
        _limbs_div_exact_limb_to_out_no_special_3(out, xs, d);
    }
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
/// quotient limbs of the `Natural` divided by a `Limb`. The divisor limb cannot be zero and the
/// limb slice must be nonempty. The `Natural` must be exactly divisible by the `Limb`. If it isn't,
/// the behavior of this function is undefined.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `limbs` is empty or if `d` is zero.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::div_exact::limbs_div_exact_limb;
///
/// assert_eq!(limbs_div_exact_limb(&[6, 7], 2), &[2_147_483_651, 3]);
/// assert_eq!(limbs_div_exact_limb(&[0xffff_ffff, 0xffff_ffff], 3), &[0x5555_5555, 0x5555_5555]);
/// ```
///
/// This is mpn_divexact_1 from mpn/generic/dive_1.c, GMP 6.1.2, where the result is returned.
pub fn limbs_div_exact_limb(xs: &[Limb], d: Limb) -> Vec<Limb> {
    if d == 3 {
        limbs_div_exact_3(xs)
    } else {
        _limbs_div_exact_limb_no_special_3(xs, d)
    }
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the quotient of the `Natural` and a `Limb` to the input slice. The divisor limb cannot
/// be zero and the input limb slice must be nonempty. The `Natural` must be exactly divisible by
/// the `Limb`. If it isn't, the behavior of this function is undefined.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `limbs` is empty or if `d` is zero.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::div_exact::limbs_div_exact_limb_in_place;
///
/// let mut limbs = vec![6, 7];
/// limbs_div_exact_limb_in_place(&mut limbs, 2);
/// assert_eq!(limbs, &[2_147_483_651, 3]);
///
/// let mut limbs = vec![0xffff_ffff, 0xffff_ffff];
/// limbs_div_exact_limb_in_place(&mut limbs, 3);
/// assert_eq!(limbs, &[0x5555_5555, 0x5555_5555]);
/// ```
pub fn limbs_div_exact_limb_in_place(xs: &mut [Limb], d: Limb) {
    if d == 3 {
        limbs_div_exact_3_in_place(xs)
    } else {
        _limbs_div_exact_limb_in_place_no_special_3(xs, d)
    }
}

/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// Result is O(`n`)
///
/// This is mpn_binvert_itch from mpn/generic/binvert.c, GMP 6.1.2.
pub fn limbs_modular_invert_scratch_len(n: usize) -> usize {
    let itch_local = _limbs_mul_mod_base_pow_n_minus_1_next_size(n);
    let itch_out = _limbs_mul_mod_base_pow_n_minus_1_scratch_len(
        itch_local,
        n,
        n.shr_round(1, RoundingMode::Ceiling),
    );
    itch_local + itch_out
}

pub fn _limbs_modular_invert_small(
    size: usize,
    is: &mut [Limb],
    scratch: &mut [Limb],
    ds: &[Limb],
    d_inv: Limb,
) {
    if size < DC_BDIV_Q_THRESHOLD {
        _limbs_modular_div_schoolbook(is, scratch, ds, d_inv);
    } else {
        _limbs_modular_div_divide_and_conquer(is, scratch, ds, d_inv);
    }
}

/// Finds the inverse of a slice `Limb` mod 2<sup>`ds.len() * Limb::WIDTH`</sup>; given x, returns y
/// such that x * y === 1 mod 2<sup>`ds.len() * Limb::WIDTH`</sup>. This inverse only exists for odd
/// x, so the least-significant limb of `ds` must be odd.
///
/// Time: O(n * log(n) * log(log(n)))
///
/// Additional memory: O(n * log(n))
///
/// where n = `ds.len()`
///
/// # Panics
/// Panics if `is` is shorter than `ds`, if `ds` is empty, or if `scratch` is too short.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::div_exact::*;
///
/// let ds = &[1, 2, 3, 4];
/// let mut scratch = vec![0; limbs_modular_invert_scratch_len(ds.len())];
/// let is = &mut [10; 4];
/// limbs_modular_invert(is, ds, &mut scratch);
/// assert_eq!(is, &[1, 4294967294, 0, 0]);
/// ```
///
/// This is mpn_binvert from mpn/generic/binvert.c, GMP 6.1.2.
pub fn limbs_modular_invert(is: &mut [Limb], ds: &[Limb], scratch: &mut [Limb]) {
    let d_len = ds.len();
    // Compute the computation precisions from highest to lowest, leaving the basecase size in
    // `size`.
    let mut size = d_len;
    let mut sizes = Vec::new();
    while size >= BINV_NEWTON_THRESHOLD {
        sizes.push(size);
        size.shr_round_assign(1, RoundingMode::Ceiling);
    }
    // Compute a base value of `size` limbs.
    let scratch_lo = &mut scratch[..size];
    let ds_lo = &ds[..size];
    slice_set_zero(scratch_lo);
    scratch_lo[0] = 1;
    let d_inv = limbs_modular_invert_limb(ds[0]).wrapping_neg();
    _limbs_modular_invert_small(size, is, scratch_lo, ds_lo, d_inv);
    let mut previous_size = size;
    // Use Newton iterations to get the desired precision.
    for &size in sizes.iter().rev() {
        let mul_size = _limbs_mul_mod_base_pow_n_minus_1_next_size(size);
        let (scratch_lo, scratch_hi) = scratch.split_at_mut(mul_size);
        let (is_lo, is_hi) = is.split_at_mut(previous_size);
        _limbs_mul_mod_base_pow_n_minus_1(scratch_lo, mul_size, &ds[..size], is_lo, scratch_hi);
        limbs_sub_limb_to_out(
            scratch_hi,
            &scratch_lo[..previous_size - (mul_size - size)],
            1,
        );
        let diff = size - previous_size;
        limbs_mul_low_same_length(is_hi, &is_lo[..diff], &scratch[previous_size..size]);
        limbs_twos_complement_in_place(&mut is_hi[..diff]);
        previous_size = size;
    }
}

/// Computes a binary quotient of size `q_len` = `ns.len()` - `ds.len()`. D must be odd. `d_inv` is
/// (-D) ^ -1 mod 2 ^ `Limb::WIDTH`, or `limbs_modular_invert_limb(ds[0]).wrapping_neg()`.
///
/// Output:
///    Q = N / D mod 2 ^ (`Limb::WIDTH` * `q_len`)
///    R = (N - Q * D) / 2 ^ (`Limb::WIDTH` * `q_len`)
///
/// Stores the `ds.len()` least-significant limbs of R at `&np[q_len..]` and returns the borrow from
/// the subtraction N - Q * D.
///
/// Time: worst case O(n ^ 2)
///
/// where n = `ns.len()`
///
/// Additional memory: worst case O(1)
///
/// This is mpn_sbpi1_bdiv_qr from mpn/generic/sbpi1_bdiv_qr.c, GMP 6.1.2.
pub fn _limbs_modular_div_mod_schoolbook(
    qs: &mut [Limb],
    ns: &mut [Limb],
    ds: &[Limb],
    d_inv: Limb,
) -> bool {
    let n_len = ns.len();
    let d_len = ds.len();
    assert_ne!(d_len, 0);
    assert!(n_len > d_len);
    assert!(ds[0].odd());
    let q_len = n_len - d_len;
    let qs = &mut qs[..q_len];
    let mut highest_r = false;
    // To complete the negation, this value is added to the quotient.
    let mut lowest_q = true;
    let mut q_len_s = q_len;
    while q_len_s > d_len {
        let q_diff = q_len - q_len_s;
        for i in q_diff..n_len - q_len_s {
            let ns = &mut ns[i..i + d_len];
            let q = d_inv.wrapping_mul(ns[0]);
            ns[0] = limbs_slice_add_mul_limb_same_length_in_place_left(ns, ds, q);
            qs[i] = !q;
        }
        let (np_lo, np_hi) = ns[q_diff..].split_at_mut(d_len);
        if limbs_slice_add_greater_in_place_left(&mut np_hi[..q_len_s], np_lo) {
            highest_r = true;
        }
        if lowest_q && !limbs_slice_add_limb_in_place(&mut qs[q_diff..n_len - q_len_s], 1) {
            lowest_q = false;
        }
        q_len_s -= d_len;
    }
    let q_len_s = q_len_s;
    let q_diff = q_len - q_len_s;
    for i in q_diff..q_len {
        let ns = &mut ns[i..i + d_len];
        let q = d_inv.wrapping_mul(ns[0]);
        ns[0] = limbs_slice_add_mul_limb_same_length_in_place_left(ns, ds, q);
        qs[i] = !q;
    }
    let (np_lo, np_hi) = ns[q_diff..].split_at_mut(d_len);
    if limbs_slice_add_same_length_in_place_left(&mut np_hi[..q_len_s], &np_lo[..q_len_s]) {
        assert!(!highest_r);
        highest_r = true;
    }
    if lowest_q && limbs_slice_add_limb_in_place(&mut qs[q_diff..], 1) {
        // quotient is zero
        assert!(!highest_r);
        false
    } else {
        let carry = limbs_sub_same_length_in_place_left(&mut ns[q_len..], ds);
        assert!(carry || !highest_r);
        carry != highest_r
    }
}

/// Time: worst case O(n * log(n) ^ 2 * log(log(n)))
///
/// Additional memory: worst case O(n * log(n))
///
/// where n = `ds.len()`
///
/// This is mpn_dcpi1_bdiv_qr_n from mpn/generic/dcpi1_bdiv_qr.c, GMP 6.1.2.
fn _limbs_modular_div_mod_divide_and_conquer_helper(
    qs: &mut [Limb],
    ns: &mut [Limb],
    ds: &[Limb],
    d_inv: Limb,
    scratch: &mut [Limb],
) -> bool {
    let n = ds.len();
    let ns = &mut ns[..n << 1];
    let scratch = &mut scratch[..n];
    let lo = n >> 1; // floor(n / 2)
    let hi = n - lo; // ceil(n / 2)
    let (ds_lo, ds_hi) = ds.split_at(lo);
    let carry = if lo < DC_BDIV_QR_THRESHOLD {
        _limbs_modular_div_mod_schoolbook(qs, &mut ns[..lo << 1], ds_lo, d_inv)
    } else {
        _limbs_modular_div_mod_divide_and_conquer_helper(qs, ns, ds_lo, d_inv, scratch)
    };
    let (qs_lo, qs_hi) = qs.split_at_mut(lo);
    limbs_mul_greater_to_out(scratch, ds_hi, qs_lo);
    if carry {
        assert!(!limbs_slice_add_limb_in_place(&mut scratch[lo..], 1));
    }
    let ns = &mut ns[lo..];
    let highest_r = limbs_sub_in_place_left(ns, scratch);
    let (ds_lo, ds_hi) = ds.split_at(hi);
    let carry = if hi < DC_BDIV_QR_THRESHOLD {
        _limbs_modular_div_mod_schoolbook(qs_hi, &mut ns[..hi << 1], ds_lo, d_inv)
    } else {
        _limbs_modular_div_mod_divide_and_conquer_helper(qs_hi, ns, ds_lo, d_inv, scratch)
    };
    limbs_mul_greater_to_out(scratch, &qs_hi[..hi], ds_hi);
    if carry {
        assert!(!limbs_slice_add_limb_in_place(&mut scratch[hi..], 1));
    }
    if limbs_sub_same_length_in_place_left(&mut ns[hi..], scratch) {
        assert!(!highest_r);
        true
    } else {
        highest_r
    }
}

/// Computes a binary quotient of size `q_len` = `ns.len()` - `ds.len()` and a remainder of size
/// `rs.len()`. D must be odd. `d_inv` is (-D) ^ -1 mod 2 ^ `Limb::WIDTH`, or
/// `limbs_modular_invert_limb(ds[0]).wrapping_neg()`.
///
/// Output:
///    Q = N / D mod 2 ^ (`Limb::WIDTH` * `q_len`)
///    R = (N - Q * D) / 2 ^ (`Limb::WIDTH` * `q_len`)
///
/// Stores the `ds.len()` least-significant limbs of R at `&np[q_len..]` and returns the borrow from
/// the subtraction N - Q * D.
///
/// Time: worst case O(n * log(d) ^ 2 * log(log(d)))
///
/// Additional memory: worst case O(n * log(n))
///
/// where n = `ns.len()`, d = `ds.len()`
///
/// This is mpn_dcpi1_bdiv_qr from mpn/generic/dcpi1_bdiv_qr.c, GMP 6.1.2.
pub fn _limbs_modular_div_mod_divide_and_conquer(
    qs: &mut [Limb],
    ns: &mut [Limb],
    ds: &[Limb],
    d_inv: Limb,
) -> bool {
    let n_len = ns.len();
    let d_len = ds.len();
    assert!(d_len >= 2); // to adhere to _limbs_modular_div_mod_schoolbook's limits
    assert!(n_len > d_len); // to adhere to _limbs_modular_div_mod_schoolbook's limits
    assert!(ds[0].odd());
    let mut scratch = vec![0; d_len];
    let q_len = n_len - d_len;
    let qs = &mut qs[..q_len];
    let mut borrow = false;
    let mut carry;
    if q_len > d_len {
        let q_len_mod_d_len = {
            let mut m = q_len % d_len;
            if m == 0 {
                m = d_len;
            }
            m
        };
        let (ds_lo, ds_hi) = ds.split_at(q_len_mod_d_len);
        // Perform the typically smaller block first.
        carry = if q_len_mod_d_len < DC_BDIV_QR_THRESHOLD {
            _limbs_modular_div_mod_schoolbook(qs, &mut ns[..q_len_mod_d_len << 1], ds_lo, d_inv)
        } else {
            _limbs_modular_div_mod_divide_and_conquer_helper(qs, ns, ds_lo, d_inv, &mut scratch)
        };
        if q_len_mod_d_len != d_len {
            limbs_mul_to_out(&mut scratch, ds_hi, &qs[..q_len_mod_d_len]);
            if carry {
                assert!(!limbs_slice_add_limb_in_place(
                    &mut scratch[q_len_mod_d_len..],
                    1
                ));
            }
            borrow = limbs_sub_in_place_left(&mut ns[q_len_mod_d_len..], &scratch[..d_len]);
            carry = false;
        }
        let mut q_len_s = q_len - q_len_mod_d_len; // q_len_s is a multiple of d_len
        while q_len_s != 0 {
            let q_diff = q_len - q_len_s;
            let ns = &mut ns[q_diff..];
            if carry && limbs_sub_limb_in_place(&mut ns[d_len..], 1) {
                assert!(!borrow);
                borrow = true;
            }
            carry = _limbs_modular_div_mod_divide_and_conquer_helper(
                &mut qs[q_diff..],
                ns,
                ds,
                d_inv,
                &mut scratch,
            );
            q_len_s -= d_len;
        }
    } else {
        let (ds_lo, ds_hi) = ds.split_at(q_len);
        carry = if q_len < DC_BDIV_QR_THRESHOLD {
            _limbs_modular_div_mod_schoolbook(qs, &mut ns[..q_len << 1], ds_lo, d_inv)
        } else {
            _limbs_modular_div_mod_divide_and_conquer_helper(qs, ns, ds_lo, d_inv, &mut scratch)
        };
        if q_len != d_len {
            limbs_mul_to_out(&mut scratch, ds_hi, qs);
            if carry {
                assert!(!limbs_slice_add_limb_in_place(&mut scratch[q_len..], 1));
            }
            borrow = limbs_sub_in_place_left(&mut ns[q_len..], &scratch[..d_len]);
            carry = false;
        }
    }
    if carry {
        assert!(!borrow);
        borrow = true;
    }
    borrow
}

/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// This is mpn_dcpi1_bdiv_qr_n_itch from mpn/generic/dcpi1_bdiv_qr.c, GMP 6.1.2.
#[inline]
pub const fn _limbs_modular_div_mod_divide_and_conquer_helper_scratch_len(n: usize) -> usize {
    n
}

/// This is mpn_mu_bdiv_qr_itch from mpn/generic/mu_bdiv_qr.c, GMP 6.1.2.
pub fn _limbs_modular_div_mod_barrett_scratch_len(n_len: usize, d_len: usize) -> usize {
    assert!(DC_BDIV_Q_THRESHOLD < MU_BDIV_Q_THRESHOLD);
    let q_len = n_len - d_len;
    let i_len = if q_len > d_len {
        let blocks = (q_len - 1) / d_len + 1; // ceil(q_len / d_len), number of blocks
        (q_len - 1) / blocks + 1 // ceil(q_len / ceil(q_len / d_len))
    } else {
        q_len - (q_len >> 1)
    };
    let (mul_len_1, mul_len_2) = if i_len < MUL_TO_MULMOD_BNM1_FOR_2NXN_THRESHOLD {
        (d_len + i_len, 0)
    } else {
        let t_len = _limbs_mul_mod_base_pow_n_minus_1_next_size(d_len);
        (
            t_len,
            _limbs_mul_mod_base_pow_n_minus_1_scratch_len(t_len, d_len, i_len),
        )
    };
    let modular_invert_scratch_len = limbs_modular_invert_scratch_len(i_len);
    let scratch_len = mul_len_1 + mul_len_2;
    i_len + max(scratch_len, modular_invert_scratch_len)
}

fn _limbs_modular_div_mod_barrett_unbalanced(
    qs: &mut [Limb],
    rs: &mut [Limb],
    ns: &[Limb],
    ds: &[Limb],
    scratch: &mut [Limb],
) -> bool {
    let n_len = ns.len();
    let d_len = ds.len();
    let q_len = n_len - d_len;
    let qs = &mut qs[..q_len];
    let rs = &mut rs[..d_len];
    // |_______________________| dividend
    // |________| divisor
    // Compute an inverse size that is a nice partition of the quotient.
    let blocks = (q_len - 1) / d_len + 1; // ceil(q_len / d_len), number of blocks
    let i_len = (q_len - 1) / blocks + 1; // ceil(q_len / b) = ceil(q_len / ceil(q_len / d_len))
    let (is, scratch) = scratch.split_at_mut(i_len);
    limbs_modular_invert(is, &ds[..i_len], scratch);
    rs.copy_from_slice(&ns[..d_len]);
    let mut carry = false;
    let mut q_len_s = q_len;
    while q_len_s > i_len {
        let qs = &mut qs[q_len - q_len_s..];
        let qs = &mut qs[..i_len];
        let ns = &ns[n_len - q_len_s..];
        limbs_mul_low_same_length(qs, &rs[..i_len], is);
        if i_len < MUL_TO_MULMOD_BNM1_FOR_2NXN_THRESHOLD {
            limbs_mul_greater_to_out(scratch, ds, qs);
        } else {
            let mul_size = _limbs_mul_mod_base_pow_n_minus_1_next_size(d_len);
            {
                let (scratch, scratch_out) = scratch.split_at_mut(mul_size);
                _limbs_mul_mod_base_pow_n_minus_1(scratch, mul_size, ds, qs, scratch_out);
            }
            if let Some(wrapped_len) = (d_len + i_len).checked_sub(mul_size) {
                if wrapped_len != 0 {
                    let (scratch_lo, scratch_hi) = scratch.split_at_mut(mul_size);
                    if limbs_sub_same_length_to_out(
                        scratch_hi,
                        &scratch_lo[..wrapped_len],
                        &rs[..wrapped_len],
                    ) {
                        assert!(!limbs_sub_limb_in_place(&mut scratch[wrapped_len..], 1));
                    }
                }
            } else {
                fail_on_untested_path(
                    "_limbs_modular_div_mod_barrett_unbalanced, wrapped_len is None",
                );
            }
        }
        let (scratch_lo, scratch_hi) = scratch.split_at_mut(d_len);
        if d_len != i_len {
            let (rp_lo, rp_hi) = rs.split_at_mut(i_len);
            if limbs_sub_same_length_to_out(rp_lo, &rp_hi[..d_len - i_len], &scratch_lo[i_len..]) {
                if carry {
                    assert!(!limbs_slice_add_limb_in_place(scratch_hi, 1));
                } else {
                    carry = true;
                }
            }
        }
        carry = _limbs_sub_same_length_with_borrow_in_to_out(
            &mut rs[d_len - i_len..],
            &ns[..i_len],
            &scratch_hi[..i_len],
            carry,
        );
        q_len_s -= i_len;
    }
    // high q_len quotient limbs
    let qs = &mut qs[q_len - q_len_s..];
    limbs_mul_low_same_length(qs, &rs[..q_len_s], &is[..q_len_s]);
    if q_len_s < MUL_TO_MULMOD_BNM1_FOR_2NXN_THRESHOLD {
        limbs_mul_greater_to_out(scratch, ds, qs);
    } else {
        let tn = _limbs_mul_mod_base_pow_n_minus_1_next_size(d_len);
        {
            let (scratch, scratch_out) = scratch.split_at_mut(tn);
            _limbs_mul_mod_base_pow_n_minus_1(scratch, tn, ds, qs, scratch_out);
        }
        if let Some(wrapped_len) = (d_len + q_len_s).checked_sub(tn) {
            if wrapped_len != 0 {
                let (scratch_lo, scratch_hi) = scratch.split_at_mut(tn);
                if limbs_sub_same_length_to_out(
                    scratch_hi,
                    &scratch_lo[..wrapped_len],
                    &rs[..wrapped_len],
                ) {
                    assert!(!limbs_sub_limb_in_place(&mut scratch[wrapped_len..], 1));
                }
            }
        }
    }
    let (scratch_lo, scratch_hi) = scratch.split_at_mut(d_len);
    if d_len != q_len_s && limbs_sub_same_length_to_out_with_overlap(rs, &scratch_lo[q_len_s..]) {
        if carry {
            assert!(!limbs_slice_add_limb_in_place(scratch_hi, 1));
        } else {
            carry = true;
        }
    }
    _limbs_sub_same_length_with_borrow_in_to_out(
        &mut rs[d_len - q_len_s..],
        &ns[n_len - q_len_s..],
        &scratch_hi[..q_len_s],
        carry,
    )
}

fn _limbs_modular_div_mod_barrett_balanced(
    qs: &mut [Limb],
    rs: &mut [Limb],
    ns: &[Limb],
    ds: &[Limb],
    scratch: &mut [Limb],
) -> bool {
    let n_len = ns.len();
    let d_len = ds.len();
    let q_len = n_len - d_len;
    let qs = &mut qs[..q_len];
    let rs = &mut rs[..d_len];
    // |_______________________| dividend
    // |________________| divisor
    // Compute half-sized inverse.
    let i_len = q_len - (q_len >> 1);
    let (is, scratch) = scratch.split_at_mut(i_len);
    let (qs_lo, qs_hi) = qs.split_at_mut(i_len);
    limbs_modular_invert(is, &ds[..i_len], scratch);
    limbs_mul_low_same_length(qs_lo, &ns[..i_len], is); // low i_len quotient limbs
    if i_len < MUL_TO_MULMOD_BNM1_FOR_2NXN_THRESHOLD {
        limbs_mul_greater_to_out(scratch, ds, qs_lo);
    } else {
        let mul_size = _limbs_mul_mod_base_pow_n_minus_1_next_size(d_len);
        {
            let (scratch, scratch_out) = scratch.split_at_mut(mul_size);
            _limbs_mul_mod_base_pow_n_minus_1(scratch, mul_size, ds, qs_lo, scratch_out);
        }
        if let Some(wrapped_len) = (d_len + i_len).checked_sub(mul_size) {
            if wrapped_len != 0 {
                let (scratch_lo, scratch_hi) = scratch.split_at_mut(mul_size);
                if limbs_sub_same_length_to_out(
                    scratch_hi,
                    &scratch_lo[..wrapped_len],
                    &ns[..wrapped_len],
                ) {
                    assert!(!limbs_sub_limb_in_place(&mut scratch[wrapped_len..], 1));
                }
            }
        }
    }
    let q_len_s = q_len - i_len;
    let (ns_lo, ns_hi) = ns.split_at(i_len + d_len);
    let mut carry =
        limbs_sub_same_length_to_out(rs, &ns_lo[i_len..], &scratch[i_len..i_len + d_len]);
    // high q_len quotient limbs
    limbs_mul_low_same_length(qs_hi, &rs[..q_len_s], &is[..q_len_s]);
    if q_len_s < MUL_TO_MULMOD_BNM1_FOR_2NXN_THRESHOLD {
        limbs_mul_greater_to_out(scratch, ds, qs_hi);
    } else {
        let mul_size = _limbs_mul_mod_base_pow_n_minus_1_next_size(d_len);
        {
            let (scratch, scratch_out) = scratch.split_at_mut(mul_size);
            _limbs_mul_mod_base_pow_n_minus_1(scratch, mul_size, ds, qs_hi, scratch_out);
        }
        if let Some(wrapped_len) = (d_len + q_len_s).checked_sub(mul_size) {
            if wrapped_len != 0 {
                let (scratch_lo, scratch_hi) = scratch.split_at_mut(mul_size);
                if limbs_sub_same_length_to_out(
                    scratch_hi,
                    &scratch_lo[..wrapped_len],
                    &rs[..wrapped_len],
                ) {
                    assert!(!limbs_sub_limb_in_place(&mut scratch[wrapped_len..], 1));
                }
            }
        }
    }
    let (scratch_lo, scratch_hi) = scratch.split_at_mut(d_len);
    if limbs_sub_same_length_to_out_with_overlap(rs, &scratch_lo[q_len_s..]) {
        if carry {
            assert!(!limbs_slice_add_limb_in_place(scratch_hi, 1));
        } else {
            carry = true;
        }
    }
    _limbs_sub_same_length_with_borrow_in_to_out(
        &mut rs[d_len - q_len_s..],
        ns_hi,
        &scratch_hi[..q_len_s],
        carry,
    )
}

/// Computes a binary quotient of size `q_len` = `ns.len()` - `ds.len()` and a remainder of size
/// `ds.len()`. D must be odd.
///
/// Output:
///    Q = N / D mod 2 ^ (`Limb::WIDTH` * `q_len`)
///    R = (N - Q * D) / 2 ^ (`Limb::WIDTH` * `q_len`)
///
/// Time: Worst case O(n * log(n) * log(log(n)))
///
/// Additional memory: Worst case O(n * log(n))
///
/// where n = `ns.len()`
///
/// # Panics
/// Panics if `ds` has length smaller than 2, `ns.len()` is less than `ds.len()` + 2, `qs` has
/// length less than `ns.len()` - `ds.len()`, `rs` is shorter than `ds`, `scratch` is to short, or
/// the last limb of `ds` is even.
///
/// This is mpn_mu_bdiv_qr from mpn/generic/mu_bdiv_qr.c, GMP 6.1.2.
pub fn _limbs_modular_div_mod_barrett(
    qs: &mut [Limb],
    rs: &mut [Limb],
    ns: &[Limb],
    ds: &[Limb],
    scratch: &mut [Limb],
) -> bool {
    let n_len = ns.len();
    let d_len = ds.len();
    assert!(d_len >= 2);
    assert!(n_len >= d_len + 2);
    if n_len > d_len << 1 {
        _limbs_modular_div_mod_barrett_unbalanced(qs, rs, ns, ds, scratch)
    } else {
        _limbs_modular_div_mod_barrett_balanced(qs, rs, ns, ds, scratch)
    }
}

/// Computes Q = N / D mod 2 ^ (`Limb::WIDTH` * `ns.len()`), destroying N. D must be odd. `d_inv` is
/// (-D) ^ -1 mod 2 ^ `Limb::WIDTH`, or `limbs_modular_invert_limb(ds[0]).wrapping_neg()`.
///
/// The straightforward way to compute Q is to cancel one limb at a time, using
///     qs[i] = D ^ (-1) * ns[i] mod 2 ^ `Limb::WIDTH`
///     N -= 2 ^ (Limb::WIDTH * i) * qs[i] * D
///
/// But we prefer addition to subtraction, since
/// `limbs_slice_add_mul_limb_same_length_in_place_left` is often faster than
/// `limbs_sub_mul_limb_same_length_in_place_left`. Q = -N / D can be computed by iterating
///     qs[i] = (-D) ^ (-1) * ns[i] mod 2 ^ `Limb::WIDTH`
///     N += 2 ^ (Limb::WIDTH * i) * qs[i] * D
///
/// And then we flip the sign: -Q = ~Q + 1.
///
/// Time: worst case O(n ^ 2)
///
/// Additional memory: worst case O(1)
///
/// where n = `ns.len()`
///
/// This is mpn_sbpi1_bdiv_q from mpn/generic/sbpi1_bdiv_q.c, GMP 6.1.2.
pub fn _limbs_modular_div_schoolbook(qs: &mut [Limb], ns: &mut [Limb], ds: &[Limb], d_inv: Limb) {
    let n_len = ns.len();
    let d_len = ds.len();
    assert_ne!(d_len, 0);
    assert!(n_len >= d_len);
    assert!(ds[0].odd());
    let qs = &mut qs[..n_len];
    let diff = n_len - d_len;
    for i in 0..diff {
        let q = d_inv.wrapping_mul(ns[i]);
        let (ns_lo, ns_hi) = ns[i..].split_at_mut(d_len);
        let carry = limbs_slice_add_mul_limb_same_length_in_place_left(ns_lo, ds, q);
        limbs_slice_add_limb_in_place(ns_hi, carry);
        assert_eq!(ns_lo[0], 0);
        qs[i] = !q;
    }
    let last_index = n_len - 1;
    for i in diff..last_index {
        let ns_hi = &mut ns[i..];
        let q = d_inv.wrapping_mul(ns_hi[0]);
        limbs_slice_add_mul_limb_same_length_in_place_left(ns_hi, &ds[..n_len - i], q);
        assert_eq!(ns_hi[0], 0);
        qs[i] = !q;
    }
    qs[last_index] = !d_inv.wrapping_mul(ns[last_index]);
    limbs_slice_add_limb_in_place(qs, 1);
}

/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// This is mpn_dcpi1_bdiv_q_n_itch from mpn/generic/dcpi1_bdiv_q.c, GMP 6.1.2.
pub const fn _limbs_modular_div_divide_and_conquer_helper_scratch_len(n: usize) -> usize {
    n
}

/// Time: worst case O(n * log(n) ^ 2 * log(log(n)))
///
/// Additional memory: worst case O(n * log(n))
///
/// where n = `ds.len()`
///
/// This is mpn_dcpi1_bdiv_q_n from mpn/generic/dcpi1_bdiv_q.c, GMP 6.1.2.
fn _limbs_modular_div_divide_and_conquer_helper(
    qs: &mut [Limb],
    ns: &mut [Limb],
    ds: &[Limb],
    d_inv: Limb,
    scratch: &mut [Limb],
) {
    let n = ds.len();
    let mut n_rem = n;
    while n_rem >= DC_BDIV_Q_THRESHOLD {
        let m = n - n_rem;
        let lo = n_rem >> 1; // floor(n / 2)
        let hi = n_rem - lo; // ceil(n / 2)
        let qs = &mut qs[m..];
        let ns = &mut ns[m..];
        let carry_1 =
            _limbs_modular_div_mod_divide_and_conquer_helper(qs, ns, &ds[..lo], d_inv, scratch);
        let qs = &qs[..lo];
        limbs_mul_low_same_length(scratch, qs, &ds[hi..n_rem]);
        limbs_sub_same_length_in_place_left(&mut ns[hi..n_rem], &scratch[..lo]);
        if lo < hi {
            let carry_2 =
                limbs_sub_mul_limb_same_length_in_place_left(&mut ns[lo..lo << 1], qs, ds[lo]);
            let n_limb = &mut ns[n_rem - 1];
            n_limb.wrapping_sub_assign(carry_2);
            if carry_1 {
                n_limb.wrapping_sub_assign(1);
            }
        }
        n_rem = hi;
    }
    let m = n - n_rem;
    _limbs_modular_div_schoolbook(&mut qs[m..], &mut ns[m..n], &ds[..n_rem], d_inv);
}

/// Computes Q = N / D mod 2 ^ (`Limb::WIDTH` * `ns.len()`), destroying N. D must be odd. `d_inv` is
/// (-D) ^ -1 mod 2 ^ `Limb::WIDTH`, or `limbs_modular_invert_limb(ds[0]).wrapping_neg()`.
///
/// Time: worst case O(n * log(d) ^ 2 * log(log(d)))
///
/// Additional memory: worst case O(n * log(n))
///
/// where n = `ns.len()`, d = `ds.len()`
///
/// This is mpn_dcpi1_bdiv_q from mpn/generic/dcpi1_bdiv_q.c, GMP 6.1.2.
pub fn _limbs_modular_div_divide_and_conquer(
    qs: &mut [Limb],
    ns: &mut [Limb],
    ds: &[Limb],
    d_inv: Limb,
) {
    let n_len = ns.len();
    let d_len = ds.len();
    assert!(d_len >= 2);
    assert!(n_len >= d_len);
    assert!(ds[0].odd());
    if n_len > d_len {
        let n_len_mod_d_len = {
            let mut m = n_len % d_len;
            if m == 0 {
                m = d_len;
            }
            m
        };
        let mut scratch = vec![0; d_len];
        // Perform the typically smaller block first.
        let (ds_lo, ds_hi) = ds.split_at(n_len_mod_d_len);
        let mut carry = if n_len_mod_d_len < DC_BDIV_QR_THRESHOLD {
            _limbs_modular_div_mod_schoolbook(qs, &mut ns[..n_len_mod_d_len << 1], ds_lo, d_inv)
        } else {
            _limbs_modular_div_mod_divide_and_conquer_helper(qs, ns, ds_lo, d_inv, &mut scratch)
        };
        if n_len_mod_d_len != d_len {
            limbs_mul_to_out(&mut scratch, ds_hi, &qs[..n_len_mod_d_len]);
            if carry {
                assert!(!limbs_slice_add_limb_in_place(
                    &mut scratch[n_len_mod_d_len..],
                    1
                ));
            }
            limbs_sub_in_place_left(&mut ns[n_len_mod_d_len..], &scratch[..d_len]);
            carry = false;
        }
        let mut m = n_len_mod_d_len;
        let diff = n_len - d_len;
        while m != diff {
            if carry {
                limbs_sub_limb_in_place(&mut ns[m + d_len..], 1);
            }
            carry = _limbs_modular_div_mod_divide_and_conquer_helper(
                &mut qs[m..],
                &mut ns[m..],
                ds,
                d_inv,
                &mut scratch,
            );
            m += d_len;
        }
        _limbs_modular_div_divide_and_conquer_helper(
            &mut qs[diff..],
            &mut ns[diff..],
            ds,
            d_inv,
            &mut scratch,
        );
    } else if n_len < DC_BDIV_Q_THRESHOLD {
        _limbs_modular_div_schoolbook(qs, ns, ds, d_inv);
    } else {
        let mut scratch = vec![0; n_len];
        _limbs_modular_div_divide_and_conquer_helper(qs, ns, ds, d_inv, &mut scratch);
    }
}

/// This is mpn_mu_bdiv_q_itch from mpn/generic/mu_bdiv_q.c, GMP 6.1.2.
pub fn _limbs_modular_div_barrett_scratch_len(n_len: usize, d_len: usize) -> usize {
    assert!(DC_BDIV_Q_THRESHOLD < MU_BDIV_Q_THRESHOLD);
    let i_len;
    let mul_len = if n_len > d_len {
        let blocks = (n_len - 1) / d_len + 1; // ceil(q_len / d_len), number of blocks
        i_len = (n_len - 1) / blocks + 1; // ceil(q_len / b) = ceil(q_len / ceil(q_len / d_len))
        let (mul_len_1, mul_len_2) = if i_len < MUL_TO_MULMOD_BNM1_FOR_2NXN_THRESHOLD {
            (d_len + i_len, 0)
        } else {
            let mul_len_1 = _limbs_mul_mod_base_pow_n_minus_1_next_size(d_len);
            (
                mul_len_1,
                _limbs_mul_mod_base_pow_n_minus_1_scratch_len(mul_len_1, d_len, i_len),
            )
        };
        d_len + mul_len_1 + mul_len_2
    } else {
        i_len = n_len - (n_len >> 1);
        let (mul_len_1, mul_len_2) = if i_len < MUL_TO_MULMOD_BNM1_FOR_2NXN_THRESHOLD {
            (n_len + i_len, 0)
        } else {
            let mul_len_1 = _limbs_mul_mod_base_pow_n_minus_1_next_size(n_len);
            (
                mul_len_1,
                _limbs_mul_mod_base_pow_n_minus_1_scratch_len(mul_len_1, n_len, i_len),
            )
        };
        mul_len_1 + mul_len_2
    };
    let invert_len = limbs_modular_invert_scratch_len(i_len);
    i_len + max(mul_len, invert_len)
}

fn _limbs_modular_div_barrett_greater(
    qs: &mut [Limb],
    ns: &[Limb],
    ds: &[Limb],
    scratch: &mut [Limb],
) {
    let n_len = ns.len();
    let d_len = ds.len();
    // |_______________________| dividend
    // |________| divisor
    // Compute an inverse size that is a nice partition of the quotient.
    let blocks = (n_len - 1) / d_len + 1; // ceil(q_len / d_len), number of blocks
    let i_len = (n_len - 1) / blocks + 1; // ceil(q_len / b) = ceil(q_len / ceil(q_len / d_len))
    let (is, rs) = scratch.split_at_mut(i_len);
    limbs_modular_invert(is, &ds[..i_len], rs);
    let mut carry = false;
    let (rs, scratch) = rs.split_at_mut(d_len);
    rs.copy_from_slice(&ns[..d_len]);
    limbs_mul_low_same_length(qs, &rs[..i_len], is);
    let mut n_len_s = n_len;
    let limit = i_len << 1;
    while n_len_s > limit {
        let diff = n_len - n_len_s;
        let (qs_lo, qs_hi) = qs[diff..].split_at_mut(i_len);
        if i_len < MUL_TO_MULMOD_BNM1_FOR_2NXN_THRESHOLD {
            limbs_mul_greater_to_out(scratch, ds, qs_lo);
        } else {
            let mul_size = _limbs_mul_mod_base_pow_n_minus_1_next_size(d_len);
            {
                let (scratch, scratch_out) = scratch.split_at_mut(mul_size);
                _limbs_mul_mod_base_pow_n_minus_1(scratch, mul_size, ds, qs_lo, scratch_out);
            }
            if let Some(wrapped_len) = (d_len + i_len).checked_sub(mul_size) {
                if wrapped_len != 0 {
                    let (scratch_lo, scratch_hi) = scratch.split_at_mut(mul_size);
                    if limbs_sub_same_length_to_out(
                        scratch_hi,
                        &scratch_lo[..wrapped_len],
                        &rs[..wrapped_len],
                    ) {
                        assert!(!limbs_sub_limb_in_place(&mut scratch[wrapped_len..], 1));
                    }
                }
            } else {
                fail_on_untested_path(
                    "_limbs_modular_div_mod_barrett_greater, wrapped_len is None",
                );
            }
        }
        let (scratch_lo, scratch_hi) = scratch.split_at_mut(d_len);
        if d_len != i_len {
            let (rs_lo, rs_hi) = rs.split_at_mut(i_len);
            if limbs_sub_same_length_to_out(rs_lo, &rs_hi[..d_len - i_len], &scratch_lo[i_len..]) {
                if carry {
                    assert!(!limbs_slice_add_limb_in_place(scratch_hi, 1));
                } else {
                    carry = true;
                }
            }
        }
        let ns = &ns[diff + d_len..];
        carry = _limbs_sub_same_length_with_borrow_in_to_out(
            &mut rs[d_len - i_len..],
            &ns[..i_len],
            &scratch_hi[..i_len],
            carry,
        );
        limbs_mul_low_same_length(qs_hi, &rs[..i_len], is);
        n_len_s -= i_len;
    }
    let n_len_s = n_len_s;
    let diff = n_len - n_len_s;
    let (qs_lo, qs_hi) = qs[diff..].split_at_mut(i_len);
    // Generate last q_len limbs.
    if i_len < MUL_TO_MULMOD_BNM1_FOR_2NXN_THRESHOLD {
        limbs_mul_greater_to_out(scratch, ds, qs_lo);
    } else {
        let mul_size = _limbs_mul_mod_base_pow_n_minus_1_next_size(d_len);
        {
            let (scratch, scratch_out) = scratch.split_at_mut(mul_size);
            _limbs_mul_mod_base_pow_n_minus_1(scratch, mul_size, ds, qs_lo, scratch_out);
        }
        if let Some(wrapped_len) = (d_len + i_len).checked_sub(mul_size) {
            if wrapped_len != 0 {
                let (scratch_lo, scratch_hi) = scratch.split_at_mut(mul_size);
                if limbs_sub_same_length_to_out(
                    scratch_hi,
                    &scratch_lo[..wrapped_len],
                    &rs[..wrapped_len],
                ) {
                    assert!(!limbs_sub_limb_in_place(&mut scratch[wrapped_len..], 1));
                }
            }
        }
    }
    if d_len != i_len {
        let (rs_lo, rs_hi) = rs.split_at_mut(i_len);
        let (scratch_lo, scratch_hi) = scratch.split_at_mut(d_len);
        if limbs_sub_same_length_to_out(rs_lo, rs_hi, &scratch_lo[i_len..]) {
            if carry {
                assert!(!limbs_slice_add_limb_in_place(scratch_hi, 1));
            } else {
                carry = true;
            }
        }
    }
    _limbs_sub_same_length_with_borrow_in_to_out(
        &mut rs[d_len - i_len..],
        &ns[diff + d_len..],
        &scratch[d_len..n_len_s],
        carry,
    );
    let limit = n_len_s - i_len;
    limbs_mul_low_same_length(qs_hi, &rs[..limit], &is[..limit]);
}

fn _limbs_modular_div_barrett_same_length(
    qs: &mut [Limb],
    ns: &[Limb],
    ds: &[Limb],
    scratch: &mut [Limb],
) {
    let n_len = ns.len();
    // |________________| dividend
    // |________________| divisor
    // Compute half-sized inverse.
    let i_len = n_len - (n_len >> 1);
    let (is, scratch) = scratch.split_at_mut(i_len);
    limbs_modular_invert(is, &ds[..i_len], scratch);
    let (ns_lo, ns_hi) = ns.split_at(i_len);
    limbs_mul_low_same_length(qs, ns_lo, is); // low i_len quotient limbs
    let (qs_lo, qs_hi) = qs.split_at_mut(i_len);
    if i_len < MUL_TO_MULMOD_BNM1_FOR_2NXN_THRESHOLD {
        limbs_mul_greater_to_out(scratch, ds, qs_lo);
    } else {
        let mul_size = _limbs_mul_mod_base_pow_n_minus_1_next_size(n_len);
        {
            let (scratch, scratch_out) = scratch.split_at_mut(mul_size);
            _limbs_mul_mod_base_pow_n_minus_1(scratch, mul_size, ds, qs_lo, scratch_out);
        }
        if let Some(wrapped_len) = (n_len + i_len).checked_sub(mul_size) {
            let (scratch_lo, scratch_hi) = scratch.split_at_mut(wrapped_len);
            if wrapped_len != 0
                && limbs_cmp_same_length(scratch_lo, &ns[..wrapped_len]) == Ordering::Less
            {
                assert!(!limbs_sub_limb_in_place(scratch_hi, 1));
            }
        } else {
            fail_on_untested_path(
                "_limbs_modular_div_mod_barrett_same_length, wrapped_len is None",
            );
        }
    }
    let (scratch_lo, scratch_hi) = scratch.split_at_mut(i_len);
    let diff = n_len - i_len;
    limbs_sub_same_length_to_out(scratch_lo, ns_hi, &scratch_hi[..diff]);
    // high n_len - i_len quotient limbs
    limbs_mul_low_same_length(qs_hi, &scratch[..diff], &is[..diff]);
}

/// Computes Q = N / D mod 2 ^ (`Limb::WIDTH` * `ns.len()`). D must be odd.
///
/// Time: Worst case O(n * log(n) * log(log(n)))
///
/// Additional memory: Worst case O(n * log(n))
///
/// where n = `ns.len()`
///
/// This is mpn_mu_bdiv_q from mpn/generic/mu_bdiv_q.c, GMP 6.1.2.
pub fn _limbs_modular_div_barrett(qs: &mut [Limb], ns: &[Limb], ds: &[Limb], scratch: &mut [Limb]) {
    let n_len = ns.len();
    let d_len = ds.len();
    assert!(d_len >= 2);
    assert!(n_len >= d_len);
    if n_len > d_len {
        _limbs_modular_div_barrett_greater(qs, ns, ds, scratch);
    } else {
        _limbs_modular_div_barrett_same_length(qs, ns, ds, scratch);
    }
}

/// This is mpn_bdiv_q_itch from mpn/generic/bdiv_q.c, GMP 6.1.2, where nothing is allocated for
/// inputs that are too small for Barrett division.
pub fn _limbs_modular_div_scratch_len(n_len: usize, d_len: usize) -> usize {
    if d_len < MU_BDIV_Q_THRESHOLD {
        0
    } else {
        _limbs_modular_div_barrett_scratch_len(n_len, d_len)
    }
}

/// Computes Q = N / D mod 2 ^ (`Limb::WIDTH` * `ns.len()`), taking N by value. D must be odd.
///
/// Time: Worst case O(n * log(n) * log(log(n)))
///
/// Additional memory: Worst case O(n * log(n))
///
/// where n = `ns.len()`
///
/// This is mpn_bdiv_q from mpn/generic/bdiv_q.c, GMP 6.1.2.
pub fn _limbs_modular_div(qs: &mut [Limb], ns: &mut [Limb], ds: &[Limb], scratch: &mut [Limb]) {
    let d_len = ds.len();
    if d_len < DC_BDIV_Q_THRESHOLD {
        let d_inv = limbs_modular_invert_limb(ds[0]).wrapping_neg();
        _limbs_modular_div_schoolbook(qs, ns, ds, d_inv);
    } else if d_len < MU_BDIV_Q_THRESHOLD {
        let d_inv = limbs_modular_invert_limb(ds[0]).wrapping_neg();
        _limbs_modular_div_divide_and_conquer(qs, ns, ds, d_inv);
    } else {
        _limbs_modular_div_barrett(qs, ns, ds, scratch);
    }
}

/// This is mpn_bdiv_q_itch from mpn/generic/bdiv_q.c, GMP 6.1.2.
pub fn _limbs_modular_div_ref_scratch_len(n_len: usize, d_len: usize) -> usize {
    if d_len < MU_BDIV_Q_THRESHOLD {
        n_len
    } else {
        _limbs_modular_div_barrett_scratch_len(n_len, d_len)
    }
}

/// Computes Q = N / D mod 2 ^ (`Limb::WIDTH` * `ns.len()`), taking N by reference. D must be odd.
///
/// Time: Worst case O(n * log(n) * log(log(n)))
///
/// Additional memory: Worst case O(n * log(n))
///
/// where n = `ns.len()`
///
/// This is mpn_bdiv_q from mpn/generic/bdiv_q.c, GMP 6.1.2.
pub fn _limbs_modular_div_ref(qs: &mut [Limb], ns: &[Limb], ds: &[Limb], scratch: &mut [Limb]) {
    let n_len = ns.len();
    let d_len = ds.len();
    if d_len < DC_BDIV_Q_THRESHOLD {
        let scratch = &mut scratch[..n_len];
        scratch.copy_from_slice(ns);
        let d_inv = limbs_modular_invert_limb(ds[0]).wrapping_neg();
        _limbs_modular_div_schoolbook(qs, scratch, ds, d_inv);
    } else if d_len < MU_BDIV_Q_THRESHOLD {
        let scratch = &mut scratch[..n_len];
        scratch.copy_from_slice(ns);
        let d_inv = limbs_modular_invert_limb(ds[0]).wrapping_neg();
        _limbs_modular_div_divide_and_conquer(qs, scratch, ds, d_inv);
    } else {
        _limbs_modular_div_barrett(qs, ns, ds, scratch);
    }
}

/// Interpreting two slices of `Limb`s, `ns` and `ds`, as the limbs (in ascending order) of two
/// `Natural`s, divides them, returning the quotient. The quotient has `ns.len() - ds.len() + 1`
/// limbs.
///
/// `ns` must be exactly divisible by `ds`! If it isn't, the function will panic or return a
/// meaningless result.
///
/// `ns` must be at least as long as `ds` and `ds` must have length at least 2 and its most
/// significant limb must be greater than zero.
///
/// Time: Worst case O(n * log(n) * log(log(n)))
///
/// Additional memory: Worst case O(n * log(n))
///
/// where n = `ns.len()`
///
/// # Panics
/// Panics if `ns` is shorter than `ds`, `ds` is empty, or the most-significant limb of `ds` is
/// zero.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::div_exact::limbs_div_exact;
///
/// assert_eq!(limbs_div_exact(&[0, 0, 0, 6, 19, 32, 21], &[0, 0, 1, 2, 3]), vec![0, 6, 7]);
/// assert_eq!(
///     limbs_div_exact(&[10_200, 20_402, 30_605, 20_402, 10_200], &[100, 101, 102]),
///     vec![102, 101, 100]
/// );
/// ```
///
/// This is mpn_divexact from mpn/generic/divexact.c, GMP 6.1.2, where scratch is allocated
/// internally and qp is returned.
pub fn limbs_div_exact(ns: &[Limb], ds: &[Limb]) -> Vec<Limb> {
    let mut qs = vec![0; ns.len() - ds.len() + 1];
    limbs_div_exact_to_out_ref_ref(&mut qs, ns, ds);
    qs
}

/// Interpreting two slices of `Limb`s, `ns` and `ds`, as the limbs (in ascending order) of two
/// `Natural`s, divides them, writing the `ns.len() - ds.len() + 1` limbs of the quotient to `qs`.
/// `ns` and `ds` are taken by value.
///
/// `ns` must be exactly divisible by `ds`! If it isn't, the function will panic or return a
/// meaningless result.
///
/// `ns` must be at least as long as `ds`, `qs` must have length at least `ns.len() - ds.len() + 1`,
/// and `ds` must be nonempty and its most significant limb must be greater than zero.
///
/// Time: Worst case O(n * log(n) * log(log(n)))
///
/// Additional memory: Worst case O(n * log(n))
///
/// where n = `ns.len()`
///
/// # Panics
/// Panics if `qs` is too short, `ns` is shorter than `ds`, `ds` is empty, or the most-significant
/// limb of `ds` is zero.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::div_exact::limbs_div_exact_to_out;
///
/// let qs = &mut [10; 4];
/// limbs_div_exact_to_out(qs, &mut [0, 0, 0, 6, 19, 32, 21], &mut [0, 0, 1, 2, 3]);
/// assert_eq!(qs, &[0, 6, 7, 10]);
///
/// let qs = &mut [10; 4];
/// limbs_div_exact_to_out(qs, &mut [10_200, 20_402, 30_605, 20_402, 10_200], &mut [100, 101, 102]);
/// assert_eq!(qs, &[102, 101, 100, 10]);
/// ```
///
/// This is mpn_divexact from mpn/generic/divexact.c, GMP 6.1.2.
pub fn limbs_div_exact_to_out(qs: &mut [Limb], ns: &mut [Limb], ds: &mut [Limb]) {
    let n_len = ns.len();
    let d_len = ds.len();
    assert_ne!(d_len, 0);
    assert!(n_len >= d_len);
    assert_ne!(ds[d_len - 1], 0);
    let leading_zero_limbs = slice_leading_zeros(ds);
    let (ns_lo, ns) = ns.split_at_mut(leading_zero_limbs);
    assert!(slice_test_zero(ns_lo), "division not exact");
    let ds = &mut ds[leading_zero_limbs..];
    let n_len = ns.len();
    let d_len = ds.len();
    if d_len == 1 {
        limbs_div_exact_limb_to_out(qs, ns, ds[0]);
        return;
    }
    let q_len = n_len - d_len + 1;
    let shift = TrailingZeros::trailing_zeros(ds[0]);
    if shift != 0 {
        let q_len_plus_1 = q_len + 1;
        let ds_limit_len = if d_len > q_len { q_len_plus_1 } else { d_len };
        limbs_slice_shr_in_place(&mut ds[..ds_limit_len], shift);
        // Since we have excluded d_len == 1, we have n_len > q_len, and we need to shift one limb
        // beyond q_len.
        limbs_slice_shr_in_place(&mut ns[..q_len_plus_1], shift);
    }
    let d_len = min(d_len, q_len);
    let mut scratch = vec![0; _limbs_modular_div_scratch_len(q_len, d_len)];
    _limbs_modular_div(qs, &mut ns[..q_len], &ds[..d_len], &mut scratch);
}

/// Interpreting two slices of `Limb`s, `ns` and `ds`, as the limbs (in ascending order) of two
/// `Natural`s, divides them, writing the `ns.len() - ds.len() + 1` limbs of the quotient to `qs`.
/// `ns` is taken by value and `ds` by reference.
///
/// `ns` must be exactly divisible by `ds`! If it isn't, the function will panic or return a
/// meaningless result.
///
/// `ns` must be at least as long as `ds`, `qs` must have length at least `ns.len() - ds.len() + 1`,
/// and `ds` must be nonempty and its most significant limb must be greater than zero.
///
/// Time: Worst case O(n * log(n) * log(log(n)))
///
/// Additional memory: Worst case O(n * log(n))
///
/// where n = `ns.len()`
///
/// # Panics
/// Panics if `qs` is too short, `ns` is shorter than `ds`, `ds` is empty, or the most-significant
/// limb of `ds` is zero.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::div_exact::limbs_div_exact_to_out_val_ref;
///
/// let qs = &mut [10; 4];
/// limbs_div_exact_to_out_val_ref(qs, &mut [0, 0, 0, 6, 19, 32, 21], &[0, 0, 1, 2, 3]);
/// assert_eq!(qs, &[0, 6, 7, 10]);
///
/// let qs = &mut [10; 4];
/// limbs_div_exact_to_out_val_ref(
///     qs,
///     &mut [10_200, 20_402, 30_605, 20_402, 10_200],
///     &[100, 101, 102]
/// );
/// assert_eq!(qs, &[102, 101, 100, 10]);
/// ```
///
/// This is mpn_divexact from mpn/generic/divexact.c, GMP 6.1.2.
pub fn limbs_div_exact_to_out_val_ref(qs: &mut [Limb], ns: &mut [Limb], ds: &[Limb]) {
    let n_len = ns.len();
    let d_len = ds.len();
    assert_ne!(d_len, 0);
    assert!(n_len >= d_len);
    assert_ne!(ds[d_len - 1], 0);
    let leading_zero_limbs = slice_leading_zeros(ds);
    let (ns_lo, ns) = ns.split_at_mut(leading_zero_limbs);
    assert!(slice_test_zero(ns_lo), "division not exact");
    let mut ds_scratch;
    let mut ds = &ds[leading_zero_limbs..];
    let n_len = ns.len();
    let d_len = ds.len();
    if d_len == 1 {
        limbs_div_exact_limb_to_out(qs, ns, ds[0]);
        return;
    }
    let q_len = n_len - d_len + 1;
    let shift = TrailingZeros::trailing_zeros(ds[0]);
    if shift != 0 {
        let q_len_plus_1 = q_len + 1;
        let ds_scratch_len = if d_len > q_len { q_len_plus_1 } else { d_len };
        ds_scratch = vec![0; ds_scratch_len];
        limbs_shr_to_out(&mut ds_scratch, &ds[..ds_scratch_len], shift);
        ds = &ds_scratch;
        // Since we have excluded d_len == 1, we have n_len > q_len, and we need to shift one limb
        // beyond q_len.
        limbs_slice_shr_in_place(&mut ns[..q_len_plus_1], shift);
    }
    let d_len = min(d_len, q_len);
    let mut scratch = vec![0; _limbs_modular_div_scratch_len(q_len, d_len)];
    _limbs_modular_div(qs, &mut ns[..q_len], &ds[..d_len], &mut scratch);
}

/// Interpreting two slices of `Limb`s, `ns` and `ds`, as the limbs (in ascending order) of two
/// `Natural`s, divides them, writing the `ns.len() - ds.len() + 1` limbs of the quotient to `qs`.
/// `ns` is taken by reference and `ds` by value.
///
/// `ns` must be exactly divisible by `ds`! If it isn't, the function will panic or return a
/// meaningless result.
///
/// `ns` must be at least as long as `ds`, `qs` must have length at least `ns.len() - ds.len() + 1`,
/// and `ds` must be nonempty and its most significant limb must be greater than zero.
///
/// Time: Worst case O(n * log(n) * log(log(n)))
///
/// Additional memory: Worst case O(n * log(n))
///
/// where n = `ns.len()`
///
/// # Panics
/// Panics if `qs` is too short, `ns` is shorter than `ds`, `ds` is empty, or the most-significant
/// limb of `ds` is zero.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::div_exact::limbs_div_exact_to_out_ref_val;
///
/// let qs = &mut [10; 4];
/// limbs_div_exact_to_out_ref_val(qs, &[0, 0, 0, 6, 19, 32, 21], &mut [0, 0, 1, 2, 3]);
/// assert_eq!(qs, &[0, 6, 7, 10]);
///
/// let qs = &mut [10; 4];
/// limbs_div_exact_to_out_ref_val(
///     qs,
///     &[10_200, 20_402, 30_605, 20_402, 10_200],
///     &mut [100, 101, 102]
/// );
/// assert_eq!(qs, &[102, 101, 100, 10]);
/// ```
///
/// This is mpn_divexact from mpn/generic/divexact.c, GMP 6.1.2.
pub fn limbs_div_exact_to_out_ref_val(qs: &mut [Limb], ns: &[Limb], ds: &mut [Limb]) {
    let n_len = ns.len();
    let d_len = ds.len();
    assert_ne!(d_len, 0);
    assert!(n_len >= d_len);
    assert_ne!(ds[d_len - 1], 0);
    let leading_zero_limbs = slice_leading_zeros(ds);
    let (ns_lo, ns_hi) = ns.split_at(leading_zero_limbs);
    assert!(slice_test_zero(ns_lo), "division not exact");
    let mut ns_scratch;
    let mut ns = ns_hi;
    let ds = &mut ds[leading_zero_limbs..];
    let n_len = ns.len();
    let d_len = ds.len();
    if d_len == 1 {
        limbs_div_exact_limb_to_out(qs, ns, ds[0]);
        return;
    }
    let q_len = n_len - d_len + 1;
    let shift = TrailingZeros::trailing_zeros(ds[0]);
    if shift != 0 {
        let q_len_plus_1 = q_len + 1;
        let ds_limit_len = if d_len > q_len { q_len_plus_1 } else { d_len };
        limbs_slice_shr_in_place(&mut ds[..ds_limit_len], shift);
        // Since we have excluded d_len == 1, we have n_len > q_len, and we need to shift one limb
        // beyond q_len.
        ns_scratch = vec![0; q_len_plus_1];
        limbs_shr_to_out(&mut ns_scratch, &ns[..q_len_plus_1], shift);
        ns = &ns_scratch;
    }
    let d_len = min(d_len, q_len);
    let mut scratch = vec![0; _limbs_modular_div_ref_scratch_len(q_len, d_len)];
    _limbs_modular_div_ref(qs, &ns[..q_len], &ds[..d_len], &mut scratch);
}

/// Interpreting two slices of `Limb`s, `ns` and `ds`, as the limbs (in ascending order) of two
/// `Natural`s, divides them, writing the `ns.len() - ds.len() + 1` limbs of the quotient to `qs`.
/// `ns` and `ds` are taken by reference.
///
/// `ns` must be exactly divisible by `ds`! If it isn't, the function will panic or return a
/// meaningless result.
///
/// `ns` must be at least as long as `ds`, `qs` must have length at least `ns.len() - ds.len() + 1`,
/// and `ds` must be nonempty and its most significant limb must be greater than zero.
///
/// Time: Worst case O(n * log(n) * log(log(n)))
///
/// Additional memory: Worst case O(n * log(n))
///
/// where n = `ns.len()`
///
/// # Panics
/// Panics if `qs` is too short, `ns` is shorter than `ds`, `ds` is empty, or the most-significant
/// limb of `ds` is zero.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::div_exact::limbs_div_exact_to_out_ref_ref;
///
/// let qs = &mut [10; 4];
/// limbs_div_exact_to_out_ref_ref(qs, &[0, 0, 0, 6, 19, 32, 21], &[0, 0, 1, 2, 3]);
/// assert_eq!(qs, &[0, 6, 7, 10]);
///
/// let qs = &mut [10; 4];
/// limbs_div_exact_to_out_ref_ref(qs, &[10_200, 20_402, 30_605, 20_402, 10_200], &[100, 101, 102]);
/// assert_eq!(qs, &[102, 101, 100, 10]);
/// ```
///
/// This is mpn_divexact from mpn/generic/divexact.c, GMP 6.1.2.
pub fn limbs_div_exact_to_out_ref_ref(qs: &mut [Limb], ns: &[Limb], ds: &[Limb]) {
    let n_len = ns.len();
    let d_len = ds.len();
    assert_ne!(d_len, 0);
    assert!(n_len >= d_len);
    assert_ne!(ds[d_len - 1], 0);
    let leading_zero_limbs = slice_leading_zeros(ds);
    let (ns_lo, ns_hi) = ns.split_at(leading_zero_limbs);
    assert!(slice_test_zero(ns_lo), "division not exact");
    let mut ns_scratch;
    let mut ds_scratch;
    let mut ns = ns_hi;
    let mut ds = &ds[leading_zero_limbs..];
    let n_len = ns.len();
    let d_len = ds.len();
    if d_len == 1 {
        limbs_div_exact_limb_to_out(qs, ns, ds[0]);
        return;
    }
    let q_len = n_len - d_len + 1;
    let shift = TrailingZeros::trailing_zeros(ds[0]);
    if shift != 0 {
        let q_len_plus_1 = q_len + 1;
        let ds_scratch_len = if d_len > q_len { q_len_plus_1 } else { d_len };
        ds_scratch = vec![0; ds_scratch_len];
        limbs_shr_to_out(&mut ds_scratch, &ds[..ds_scratch_len], shift);
        ds = &ds_scratch;
        // Since we have excluded d_len == 1, we have n_len > q_len, and we need to shift one limb
        // beyond q_len.
        ns_scratch = vec![0; q_len_plus_1];
        limbs_shr_to_out(&mut ns_scratch, &ns[..q_len_plus_1], shift);
        ns = &ns_scratch;
    }
    let d_len = min(d_len, q_len);
    let mut scratch = vec![0; _limbs_modular_div_ref_scratch_len(q_len, d_len)];
    _limbs_modular_div_ref(qs, &ns[..q_len], &ds[..d_len], &mut scratch);
}

impl Natural {
    fn div_exact_limb_ref(&self, other: Limb) -> Natural {
        if other == 0 {
            panic!("division by zero");
        } else if other == 1 {
            self.clone()
        } else {
            match *self {
                Natural(Small(small)) => Natural(Small(small / other)),
                Natural(Large(ref limbs)) => {
                    Natural::from_owned_limbs_asc(limbs_div_exact_limb(limbs, other))
                }
            }
        }
    }

    fn div_exact_assign_limb(&mut self, other: Limb) {
        if other == 0 {
            panic!("division by zero");
        } else if other != 1 {
            match *self {
                Natural(Small(ref mut small)) => {
                    *small /= other;
                    return;
                }
                Natural(Large(ref mut limbs)) => limbs_div_exact_limb_in_place(limbs, other),
            }
            self.trim();
        }
    }
}

impl DivExact<Natural> for Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `Natural`, taking both `Natural`s by value. The first `Natural`
    /// must be exactly divisible by the second. If it isn't, this function will crash or return
    /// a meaningless result.
    ///
    /// If you are unsure whether the division will be exact use `self / other` instead. If you're
    /// unsure and you want to know, use `self.div_mod(other)` and check whether the remainder is
    /// zero. If you want a function that panics if the division is not exact, use
    /// `self.div_round(other, RoundingMode::Exact)`.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero. May panic if `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivExact;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// // 123 * 456 = 56088
    /// assert_eq!(Natural::from(56088u32).div_exact(Natural::from(456u32)).to_string(), "123");
    ///
    /// // 123456789000 * 987654321000 = 121932631112635269000000
    /// assert_eq!(
    ///     Natural::from_str("121932631112635269000000").unwrap()
    ///         .div_exact(Natural::from_str("987654321000").unwrap()).to_string(),
    ///     "123456789000"
    /// );
    /// ```
    #[inline]
    fn div_exact(mut self, other: Natural) -> Natural {
        self.div_exact_assign(other);
        self
    }
}

impl<'a> DivExact<&'a Natural> for Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `Natural`, taking the first `Natural` by value and the second by
    /// reference. The first `Natural` must be exactly divisible by the second. If it isn't, this
    /// function will crash or return a meaningless result.
    ///
    /// If you are unsure whether the division will be exact use `self / other` instead. If you're
    /// unsure and you want to know, use `self.div_mod(other)` and check whether the remainder is
    /// zero. If you want a function that panics if the division is not exact, use
    /// `self.div_round(other, RoundingMode::Exact)`.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero. May panic if `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivExact;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// // 123 * 456 = 56088
    /// assert_eq!(
    ///     Natural::from(56088u32).div_exact(&Natural::from(456u32)).to_string(),
    ///     "123"
    /// );
    ///
    /// // 123456789000 * 987654321000 = 121932631112635269000000
    /// assert_eq!(
    ///     Natural::from_str("121932631112635269000000").unwrap()
    ///         .div_exact(&Natural::from_str("987654321000").unwrap()).to_string(),
    ///     "123456789000"
    /// );
    /// ```
    #[inline]
    fn div_exact(mut self, other: &'a Natural) -> Natural {
        self.div_exact_assign(other);
        self
    }
}

impl<'a> DivExact<Natural> for &'a Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `Natural`, taking the first `Natural` by reference and the second
    /// by value. The first `Natural` must be exactly divisible by the second. If it isn't, this
    /// function will crash or return a meaningless result.
    ///
    /// If you are unsure whether the division will be exact use `self / other` instead. If you're
    /// unsure and you want to know, use `self.div_mod(other)` and check whether the remainder is
    /// zero. If you want a function that panics if the division is not exact, use
    /// `self.div_round(other, RoundingMode::Exact)`.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero. May panic if `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivExact;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// // 123 * 456 = 56088
    /// assert_eq!(
    ///     (&Natural::from(56088u32)).div_exact(Natural::from(456u32)).to_string(),
    ///     "123"
    /// );
    ///
    /// // 123456789000 * 987654321000 = 121932631112635269000000
    /// assert_eq!(
    ///     (&Natural::from_str("121932631112635269000000").unwrap())
    ///         .div_exact(Natural::from_str("987654321000").unwrap()).to_string(),
    ///     "123456789000"
    /// );
    /// ```
    fn div_exact(self, mut other: Natural) -> Natural {
        if other == 0 {
            panic!("division by zero");
        } else if other == 1 {
            self.clone()
        } else if *self == 0 {
            Natural::ZERO
        } else if self.limb_count() < other.limb_count() {
            panic!("division not exact");
        } else {
            Natural::from_owned_limbs_asc(match (self, &mut other) {
                (x, &mut Natural(Small(y))) => {
                    return x.div_exact_limb_ref(y);
                }
                (&Natural(Large(ref xs)), &mut Natural(Large(ref mut ys))) => {
                    let mut qs = vec![0; xs.len() - ys.len() + 1];
                    limbs_div_exact_to_out_ref_val(&mut qs, xs, ys);
                    qs
                }
                _ => unreachable!(),
            })
        }
    }
}

impl<'a, 'b> DivExact<&'b Natural> for &'a Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `Natural`, taking both `Natural`s by reference. The first `Natural`
    /// must be exactly divisible by the second. If it isn't, this function will crash or return
    /// a meaningless result.
    ///
    /// If you are unsure whether the division will be exact use `self / other` instead. If you're
    /// unsure and you want to know, use `self.div_mod(other)` and check whether the remainder is
    /// zero. If you want a function that panics if the division is not exact, use
    /// `self.div_round(other, RoundingMode::Exact)`.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero. May panic if `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivExact;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// // 123 * 456 = 56088
    /// assert_eq!(
    ///     (&Natural::from(56088u32)).div_exact(&Natural::from(456u32)).to_string(),
    ///     "123"
    /// );
    ///
    /// // 123456789000 * 987654321000 = 121932631112635269000000
    /// assert_eq!(
    ///     (&Natural::from_str("121932631112635269000000").unwrap())
    ///         .div_exact(&Natural::from_str("987654321000").unwrap()).to_string(),
    ///     "123456789000"
    /// );
    /// ```
    fn div_exact(self, other: &'b Natural) -> Natural {
        if *other == 0 {
            panic!("division by zero");
        } else if *other == 1 {
            self.clone()
        } else if *self == 0 {
            Natural::ZERO
        } else if self as *const Natural == other as *const Natural {
            Natural::ONE
        } else if self.limb_count() < other.limb_count() {
            panic!("division not exact");
        } else {
            Natural::from_owned_limbs_asc(match (self, other) {
                (x, &Natural(Small(y))) => {
                    return x.div_exact_limb_ref(y);
                }
                (&Natural(Large(ref xs)), &Natural(Large(ref ys))) => {
                    let mut qs = vec![0; xs.len() - ys.len() + 1];
                    limbs_div_exact_to_out_ref_ref(&mut qs, xs, ys);
                    qs
                }
                _ => unreachable!(),
            })
        }
    }
}

impl DivExactAssign<Natural> for Natural {
    /// Divides a `Natural` by a `Natural` in place, taking the second `Natural` by value. The
    /// `Natural` being assigned to must be exactly divisible by the `Natural` on the RHS. If it
    /// isn't, this function will crash or assign the first `Natural` to a meaningless value.
    ///
    /// If you are unsure whether the division will be exact use `self /= other` instead. If you're
    /// unsure and you want to know, use `self.div_assign_mod(other)` and check whether the
    /// remainder is zero. If you want a function that panics if the division is not exact, use
    /// `self.div_round_assign(other, RoundingMode::Exact)`.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero. May panic if `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivExactAssign;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// // 123 * 456 = 56088
    /// let mut x = Natural::from(56088u32);
    /// x.div_exact_assign(Natural::from(456u32));
    /// assert_eq!(x.to_string(), "123");
    ///
    /// // 123456789000 * 987654321000 = 121932631112635269000000
    /// let mut x = Natural::from_str("121932631112635269000000").unwrap();
    /// x.div_exact_assign(Natural::from_str("987654321000").unwrap());
    /// assert_eq!(x.to_string(), "123456789000");
    /// ```
    fn div_exact_assign(&mut self, other: Natural) {
        if other == 0 {
            panic!("division by zero");
        } else if other == 1 {
        } else if *self == 0 {
            *self = Natural::ZERO;
        } else if self.limb_count() < other.limb_count() {
            panic!("division not exact");
        } else {
            match (&mut *self, other) {
                (x, Natural(Small(y))) => {
                    x.div_exact_assign_limb(y);
                    return;
                }
                (&mut Natural(Large(ref mut xs)), Natural(Large(ref mut ys))) => {
                    let mut qs = vec![0; xs.len() - ys.len() + 1];
                    limbs_div_exact_to_out(&mut qs, xs, ys);
                    swap(&mut qs, xs);
                }
                _ => unreachable!(),
            };
            self.trim();
        }
    }
}

impl<'a> DivExactAssign<&'a Natural> for Natural {
    /// Divides a `Natural` by a `Natural` in place, taking the second `Natural` by reference. The
    /// `Natural` being assigned to must be exactly divisible by the `Natural` on the RHS. If it
    /// isn't, this function will crash or assign the first `Natural` to a meaningless value.
    ///
    /// If you are unsure whether the division will be exact use `self /= other` instead. If you're
    /// unsure and you want to know, use `self.div_assign_mod(other)` and check whether the
    /// remainder is zero. If you want a function that panics if the division is not exact, use
    /// `self.div_round_assign(other, RoundingMode::Exact)`.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero. May panic if `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivExactAssign;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// // 123 * 456 = 56088
    /// let mut x = Natural::from(56088u32);
    /// x.div_exact_assign(&Natural::from(456u32));
    /// assert_eq!(x.to_string(), "123");
    ///
    /// // 123456789000 * 987654321000 = 121932631112635269000000
    /// let mut x = Natural::from_str("121932631112635269000000").unwrap();
    /// x.div_exact_assign(&Natural::from_str("987654321000").unwrap());
    /// assert_eq!(x.to_string(), "123456789000");
    /// ```
    fn div_exact_assign(&mut self, other: &'a Natural) {
        if *other == 0 {
            panic!("division by zero");
        } else if *other == 1 {
        } else if *self == 0 {
            *self = Natural::ZERO;
        } else if self.limb_count() < other.limb_count() {
            panic!("division not exact");
        } else {
            match (&mut *self, other) {
                (x, &Natural(Small(y))) => {
                    x.div_exact_assign_limb(y);
                    return;
                }
                (&mut Natural(Large(ref mut xs)), Natural(Large(ref ys))) => {
                    let mut qs = vec![0; xs.len() - ys.len() + 1];
                    limbs_div_exact_to_out_val_ref(&mut qs, xs, ys);
                    swap(&mut qs, xs);
                }
                _ => unreachable!(),
            };
            self.trim();
        }
    }
}
