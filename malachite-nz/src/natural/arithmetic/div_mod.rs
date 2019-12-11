use std::cmp::{min, Ordering};
use std::mem::swap;

use malachite_base::comparison::Max;
use malachite_base::crement::Crementable;
use malachite_base::limbs::{limbs_move_left, limbs_set_zero};
use malachite_base::num::arithmetic::traits::{
    CeilingDivAssignNegMod, CeilingDivNegMod, DivAssignMod, DivAssignRem, DivMod, DivRem,
    WrappingAddAssign, WrappingSub, WrappingSubAssign,
};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::{JoinHalves, SplitInHalf};

use natural::arithmetic::add::{
    _limbs_add_same_length_with_carry_in_in_place_left,
    _limbs_add_same_length_with_carry_in_to_out, limbs_add_limb_to_out,
    limbs_add_same_length_to_out, limbs_slice_add_limb_in_place,
    limbs_slice_add_same_length_in_place_left,
};
use natural::arithmetic::div::{
    _limbs_div_divide_and_conquer_approx, _limbs_div_schoolbook_approx,
};
use natural::arithmetic::mul::mul_mod::{
    _limbs_mul_mod_base_pow_n_minus_1, _limbs_mul_mod_base_pow_n_minus_1_next_size,
    _limbs_mul_mod_base_pow_n_minus_1_scratch_len,
};
use natural::arithmetic::mul::{
    limbs_mul_greater_to_out, limbs_mul_same_length_to_out, limbs_mul_to_out,
};
use natural::arithmetic::shl_u::{limbs_shl_to_out, limbs_slice_shl_in_place};
use natural::arithmetic::shr_u::{limbs_shr_to_out, limbs_slice_shr_in_place};
use natural::arithmetic::sub::{
    _limbs_sub_same_length_with_borrow_in_in_place_left,
    _limbs_sub_same_length_with_borrow_in_in_place_right,
    _limbs_sub_same_length_with_borrow_in_to_out, limbs_sub_in_place_left, limbs_sub_limb_in_place,
    limbs_sub_same_length_in_place_left, limbs_sub_same_length_in_place_right,
    limbs_sub_same_length_to_out,
};
use natural::arithmetic::sub_mul::limbs_sub_mul_limb_same_length_in_place_left;
use natural::comparison::ord::limbs_cmp_same_length;
use natural::logic::not::limbs_not_to_out;
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::{
    DoubleLimb, Limb, DC_DIVAPPR_Q_THRESHOLD, DC_DIV_QR_THRESHOLD, INV_MULMOD_BNM1_THRESHOLD,
    INV_NEWTON_THRESHOLD, MAYBE_DCP1_DIVAPPR, MU_DIV_QR_SKEW_THRESHOLD, MU_DIV_QR_THRESHOLD,
};

/// The highest bit of the input must be set.
///
/// Time: O(1)
///
/// Additional memory: O(1)
///
/// # Panics
/// Panics if `divisor` is zero.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::div_mod::limbs_invert_limb;
///
/// assert_eq!(limbs_invert_limb(0x8000_0002), 0xffff_fff8);
/// assert_eq!(limbs_invert_limb(0xffff_fffe), 2);
/// ```
///
/// This is mpn_invert_limb, or invert_limb, from gmp-impl.h.
#[inline]
pub fn limbs_invert_limb(divisor: Limb) -> Limb {
    (DoubleLimb::join_halves(!divisor, Limb::MAX) / DoubleLimb::from(divisor)).lower_half()
}

/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// This is udiv_qrnnd_preinv from gmp-impl.h.
pub(crate) fn div_mod_by_preinversion(
    n_high: Limb,
    n_low: Limb,
    divisor: Limb,
    divisor_inverse: Limb,
) -> (Limb, Limb) {
    let (mut quotient_high, quotient_low) = (DoubleLimb::from(n_high)
        * DoubleLimb::from(divisor_inverse))
    .wrapping_add(DoubleLimb::join_halves(n_high.wrapping_add(1), n_low))
    .split_in_half();
    let mut remainder = n_low.wrapping_sub(quotient_high.wrapping_mul(divisor));
    if remainder > quotient_low {
        let (r_plus_d, overflow) = remainder.overflowing_add(divisor);
        if overflow {
            quotient_high.wrapping_sub_assign(1);
            remainder = r_plus_d;
        }
    } else if remainder >= divisor {
        quotient_high.wrapping_add_assign(1);
        remainder -= divisor;
    }
    (quotient_high, remainder)
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
/// quotient limbs and remainder of the `Natural` divided by a `Limb`. The divisor limb cannot be
/// zero and the limb slice must have at least two elements.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if the length of `limbs` is less than 2 or if `divisor` is zero.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::div_mod::limbs_div_limb_mod;
///
/// assert_eq!(limbs_div_limb_mod(&[123, 456], 789), (vec![2_482_262_467, 0], 636));
/// assert_eq!(limbs_div_limb_mod(&[0xffff_ffff, 0xffff_ffff], 3),
///     (vec![0x5555_5555, 0x5555_5555], 0));
/// ```
///
/// This is mpn_divrem_1 from mpn/generic/divrem_1.c where qxn is 0, un > 1, and both results are
/// returned. Experiments show that DIVREM_1_NORM_THRESHOLD and DIVREM_1_UNNORM_THRESHOLD are
/// unnecessary (they would always be 0).
pub fn limbs_div_limb_mod(limbs: &[Limb], divisor: Limb) -> (Vec<Limb>, Limb) {
    let mut quotient_limbs = vec![0; limbs.len()];
    let remainder = limbs_div_limb_to_out_mod(&mut quotient_limbs, limbs, divisor);
    (quotient_limbs, remainder)
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the quotient of the `Natural` and a `Limb` to an output slice, and returns the
/// remainder. The output slice must be at least as long as the input slice. The divisor limb cannot
/// be zero and the input limb slice must have at least two elements.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `out` is shorter than `in_limbs`, the length of `in_limbs` is less than 2, or if
/// `divisor` is zero.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::div_mod::limbs_div_limb_to_out_mod;
///
/// let mut out = vec![10, 10, 10, 10];
/// assert_eq!(limbs_div_limb_to_out_mod(&mut out, &[123, 456], 789), 636);
/// assert_eq!(out, &[2_482_262_467, 0, 10, 10]);
///
/// let mut out = vec![10, 10, 10, 10];
/// assert_eq!(limbs_div_limb_to_out_mod(&mut out, &[0xffff_ffff, 0xffff_ffff], 3), 0);
/// assert_eq!(out, &[0x5555_5555, 0x5555_5555, 10, 10]);
/// ```
///
/// This is mpn_divrem_1 from mpn/generic/divrem_1.c where qxn is 0 and un > 1. Experiments show
/// that DIVREM_1_NORM_THRESHOLD and DIVREM_1_UNNORM_THRESHOLD are unnecessary (they would always be
/// 0).
pub fn limbs_div_limb_to_out_mod(out: &mut [Limb], in_limbs: &[Limb], divisor: Limb) -> Limb {
    assert_ne!(divisor, 0);
    let len = in_limbs.len();
    assert!(len > 1);
    let out = &mut out[..len];
    let bits = divisor.leading_zeros();
    if bits == 0 {
        // High quotient limb is 0 or 1, skip a divide step.
        let (remainder, in_limbs_init) = in_limbs.split_last().unwrap();
        let mut remainder = *remainder;
        let (out_last, out_init) = out.split_last_mut().unwrap();
        *out_last = if remainder >= divisor {
            remainder -= divisor;
            1
        } else {
            0
        };
        // Multiply-by-inverse, divisor already normalized.
        let inverse = limbs_invert_limb(divisor);
        for (out_limb, &limb) in out_init.iter_mut().zip(in_limbs_init.iter()).rev() {
            let (quotient, new_remainder) =
                div_mod_by_preinversion(remainder, limb, divisor, inverse);
            *out_limb = quotient;
            remainder = new_remainder;
        }
        remainder
    } else {
        // Skip a division if high < divisor (high quotient 0). Testing here before normalizing will
        // still skip as often as possible.
        let (in_limbs_last, in_limbs_init) = in_limbs.split_last().unwrap();
        let (in_limbs, mut remainder) = if *in_limbs_last < divisor {
            *out.last_mut().unwrap() = 0;
            (in_limbs_init, *in_limbs_last)
        } else {
            (in_limbs, 0)
        };
        let divisor = divisor << bits;
        remainder <<= bits;
        let inverse = limbs_invert_limb(divisor);
        let (previous_limb, in_limbs_init) = in_limbs.split_last().unwrap();
        let mut previous_limb = *previous_limb;
        let cobits = Limb::WIDTH - bits;
        remainder |= previous_limb >> cobits;
        let (out_first, out_tail) = out.split_first_mut().unwrap();
        for (out_limb, &limb) in out_tail.iter_mut().zip(in_limbs_init.iter()).rev() {
            let nshift = (previous_limb << bits) | (limb >> cobits);
            let (quotient, new_remainder) =
                div_mod_by_preinversion(remainder, nshift, divisor, inverse);
            *out_limb = quotient;
            remainder = new_remainder;
            previous_limb = limb;
        }
        let (quotient, remainder) =
            div_mod_by_preinversion(remainder, previous_limb << bits, divisor, inverse);
        *out_first = quotient;
        remainder >> bits
    }
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the quotient of the `Natural` and a `Limb` to the input slice and returns the
/// remainder. The divisor limb cannot be zero and the input limb slice must have at least two
/// elements.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if the length of `limbs` is less than 2 or if `divisor` is zero.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::div_mod::limbs_div_limb_in_place_mod;
///
/// let mut limbs = vec![123, 456];
/// assert_eq!(limbs_div_limb_in_place_mod(&mut limbs, 789), 636);
/// assert_eq!(limbs, &[2_482_262_467, 0]);
///
/// let mut limbs = vec![0xffff_ffff, 0xffff_ffff];
/// assert_eq!(limbs_div_limb_in_place_mod(&mut limbs, 3), 0);
/// assert_eq!(limbs, &[0x5555_5555, 0x5555_5555]);
/// ```
///
/// This is mpn_divrem_1 from mpn/generic/divrem_1.c where qp == up, qxn is 0, and un > 1.
/// Experiments show that DIVREM_1_NORM_THRESHOLD and DIVREM_1_UNNORM_THRESHOLD are unnecessary
/// (they would always be 0).
pub fn limbs_div_limb_in_place_mod(limbs: &mut [Limb], divisor: Limb) -> Limb {
    assert_ne!(divisor, 0);
    let len = limbs.len();
    assert!(len > 1);
    let bits = divisor.leading_zeros();
    let (limbs_last, limbs_init) = limbs.split_last_mut().unwrap();
    if bits == 0 {
        // High quotient limb is 0 or 1, skip a divide step.
        let mut remainder = *limbs_last;
        *limbs_last = if remainder >= divisor {
            remainder -= divisor;
            1
        } else {
            0
        };
        // Multiply-by-inverse, divisor already normalized.
        let inverse = limbs_invert_limb(divisor);
        for limb in limbs_init.iter_mut().rev() {
            let (quotient, new_remainder) =
                div_mod_by_preinversion(remainder, *limb, divisor, inverse);
            *limb = quotient;
            remainder = new_remainder;
        }
        remainder
    } else {
        // Skip a division if high < divisor (high quotient 0). Testing here before normalizing will
        // still skip as often as possible.
        let (limbs, mut remainder) = if *limbs_last < divisor {
            let remainder = *limbs_last;
            *limbs_last = 0;
            (limbs_init, remainder)
        } else {
            (limbs, 0)
        };
        let divisor = divisor << bits;
        remainder <<= bits;
        let inverse = limbs_invert_limb(divisor);
        let last_index = limbs.len() - 1;
        let mut previous_limb = limbs[last_index];
        let cobits = Limb::WIDTH - bits;
        remainder |= previous_limb >> cobits;
        for i in (0..last_index).rev() {
            let limb = limbs[i];
            let shifted_limb = (previous_limb << bits) | (limb >> cobits);
            let (quotient, new_remainder) =
                div_mod_by_preinversion(remainder, shifted_limb, divisor, inverse);
            limbs[i + 1] = quotient;
            remainder = new_remainder;
            previous_limb = limb;
        }
        let (quotient, remainder) =
            div_mod_by_preinversion(remainder, previous_limb << bits, divisor, inverse);
        limbs[0] = quotient;
        remainder >> bits
    }
}

/// Computes floor((B ^ 3 - 1) / (`hi` * B + `lo`)) - B, where B = 2 ^ `Limb::WIDTH`, assuming the
/// highest bit of `hi` is set.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Panics
/// Panics if `hi` is zero.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::div_mod::limbs_two_limb_inverse_helper;
///
/// assert_eq!(limbs_two_limb_inverse_helper(0x8000_0001, 3), 0xffff_fffb);
/// assert_eq!(limbs_two_limb_inverse_helper(2325651385, 3907343530), 3636893938);
/// ```
///
/// This is invert_pi1 from gmp-impl.h, where the result is returned instead of being written to
/// dinv.
pub fn limbs_two_limb_inverse_helper(hi: Limb, lo: Limb) -> Limb {
    let mut inverse = limbs_invert_limb(hi);
    let mut hi_product = hi.wrapping_mul(inverse);
    hi_product.wrapping_add_assign(lo);
    if hi_product < lo {
        inverse.wrapping_sub_assign(1);
        if hi_product >= hi {
            hi_product.wrapping_sub_assign(hi);
            inverse.wrapping_sub_assign(1);
        }
        hi_product.wrapping_sub_assign(hi);
    }
    let (lo_product_hi, lo_product_lo) =
        (DoubleLimb::from(lo) * DoubleLimb::from(inverse)).split_in_half();
    hi_product.wrapping_add_assign(lo_product_hi);
    if hi_product < lo_product_hi {
        inverse.wrapping_sub_assign(1);
        if hi_product > hi || hi_product == hi && lo_product_lo >= lo {
            inverse.wrapping_sub_assign(1);
        }
    }
    inverse
}

/// Computes the quotient and remainder of `[n_2, n_1, n_0]` / `[d_1, d_0]`. Requires the highest
/// bit of `d_1` to be set, and `[n_2, n_1]` < `[d_1, d_0]`. `inverse` is the inverse of
/// `[d_1, d_0]` computed by `limbs_two_limb_inverse_helper`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::div_mod::*;
///
/// let d_1 = 0x8000_0004;
/// let d_0 = 5;
/// assert_eq!(
///     limbs_div_mod_three_limb_by_two_limb(
///         1, 2, 3, d_1, d_0,
///         limbs_two_limb_inverse_helper(d_1, d_0)),
///     (1, 0x7fff_fffd_ffff_fffe)
/// );
///
/// let d_1 = 0x8000_0000;
/// let d_0 = 0;
/// assert_eq!(
///     limbs_div_mod_three_limb_by_two_limb(
///         2, 0x4000_0000, 4, d_1, d_0,
///         limbs_two_limb_inverse_helper(d_1, d_0)),
///     (4, 0x4000_0000_0000_0004)
/// );
/// ```
///
/// This is udiv_qr_3by2 from gmp-impl.h.
pub fn limbs_div_mod_three_limb_by_two_limb(
    n_2: Limb,
    n_1: Limb,
    n_0: Limb,
    d_1: Limb,
    d_0: Limb,
    inverse: Limb,
) -> (Limb, DoubleLimb) {
    let (mut q, q_lo) = (DoubleLimb::from(n_2) * DoubleLimb::from(inverse))
        .wrapping_add(DoubleLimb::join_halves(n_2, n_1))
        .split_in_half();
    let d = DoubleLimb::join_halves(d_1, d_0);
    // Compute the two most significant limbs of n - q * d
    let mut r = DoubleLimb::join_halves(n_1.wrapping_sub(d_1.wrapping_mul(q)), n_0)
        .wrapping_sub(d)
        .wrapping_sub(DoubleLimb::from(d_0) * DoubleLimb::from(q));
    q.wrapping_add_assign(1);
    // Conditionally adjust q and the remainder
    if r.upper_half() >= q_lo {
        let (r_plus_d, overflow) = r.overflowing_add(d);
        if overflow {
            q.wrapping_sub_assign(1);
            r = r_plus_d;
        }
    } else if r >= d {
        q.wrapping_add_assign(1);
        r.wrapping_sub_assign(d);
    }
    (q, r)
}

/// Divides `ns` by `ds` and writes the `ns.len()` - 2 least-significant quotient limbs to `qs` and
/// the 2-long remainder to `ns`. Returns the most significant limb of the quotient; `true` means 1
/// and `false` means 0. `ds` must have length 2, `ns` must have length at least 2, and the most
/// significant bit of `ds[1]` must be set.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `ns.len()`
///
/// # Panics
/// Panics if `ds` does not have length 2, `ns` has length less than 2, `qs` has length less than
/// `ns.len() - 2`, or `ds[1]` does not have its highest bit set.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::div_mod::limbs_div_mod_by_two_limb_normalized;
///
/// let qs = &mut [10, 10, 10, 10];
/// let ns = &mut [1, 2, 3, 4, 5];
/// assert_eq!(limbs_div_mod_by_two_limb_normalized(qs, ns, &[3, 0x8000_0000]), false);
/// assert_eq!(qs, &[4294967241, 7, 10, 10]);
/// assert_eq!(ns, &[166, 2147483626, 3, 4, 5]);
/// ```
///
/// This is mpn_divrem_2 from mpn/generic/divrem_2.c.
pub fn limbs_div_mod_by_two_limb_normalized(qs: &mut [Limb], ns: &mut [Limb], ds: &[Limb]) -> bool {
    assert_eq!(ds.len(), 2);
    let n_len = ns.len();
    assert!(n_len >= 2);
    let n_limit = n_len - 2;
    assert!(ds[1].get_highest_bit());
    let d_1 = ds[1];
    let d_0 = ds[0];
    let d = DoubleLimb::join_halves(d_1, d_0);
    let mut r = DoubleLimb::join_halves(ns[n_limit + 1], ns[n_limit]);
    let highest_q = r >= d;
    if highest_q {
        r.wrapping_sub_assign(d);
    }
    let (mut r_1, mut r_0) = r.split_in_half();
    let inverse = limbs_two_limb_inverse_helper(d_1, d_0);
    for (&n, q) in ns[..n_limit].iter().zip(qs[..n_limit].iter_mut()).rev() {
        let (new_q, r) = limbs_div_mod_three_limb_by_two_limb(r_1, r_0, n, d_1, d_0, inverse);
        let (new_r_1, new_r_0) = r.split_in_half();
        r_1 = new_r_1;
        r_0 = new_r_0;
        *q = new_q;
    }
    ns[1] = r_1;
    ns[0] = r_0;
    highest_q
}

/// Schoolbook division using the Möller-Granlund 3/2 division algorithm.
///
/// Divides `ns` by `ds` and writes the `ns.len()` - `ds.len()` least-significant quotient limbs to
/// `qs` and the `ds.len()` limbs of the remainder to `ns`. Returns the most significant limb of the
/// quotient; `true` means 1 and `false` means 0. `ds` must have length greater than 2, `ns` must be
/// at least as long as `ds`, and the most significant bit of `ds` must be set. `inverse` should be
/// the result of `limbs_two_limb_inverse_helper` applied to the two highest limbs of the
/// denominator.
///
/// Time: worst case O(d * (n - d + 1)); also, O(n ^ 2)
///
/// Additional memory: worst case O(1)
///
/// where n = `ns.len()`
///       d = `ds.len()`
///
/// # Panics
/// Panics if `ds` has length smaller than 3, `ns` is shorter than `ds`, `qs` has length less than
/// `ns.len()` - `ds.len()`, or the last limb of `ds` does not have its highest bit set.
///
/// This is mpn_sbpi1_div_qr from mpn/generic/sbpi1_div_qr.c.
pub fn _limbs_div_mod_schoolbook(
    qs: &mut [Limb],
    ns: &mut [Limb],
    ds: &[Limb],
    inverse: Limb,
) -> bool {
    let d_len = ds.len();
    assert!(d_len > 2);
    let n_len = ns.len();
    assert!(n_len >= d_len);
    let d_1 = ds[d_len - 1];
    assert!(d_1.get_highest_bit());
    let d_0 = ds[d_len - 2];
    let ds_except_last = &ds[..d_len - 1];
    let ds_except_last_two = &ds[..d_len - 2];
    let highest_q;
    {
        let ns_hi = &mut ns[n_len - d_len..];
        highest_q = limbs_cmp_same_length(ns_hi, ds) >= Ordering::Equal;
        if highest_q {
            limbs_sub_same_length_in_place_left(ns_hi, ds);
        }
    }
    let mut n_1 = ns[n_len - 1];
    for i in (d_len..n_len).rev() {
        let j = i - d_len;
        let mut q;
        if n_1 == d_1 && ns[i - 1] == d_0 {
            q = Limb::MAX;
            limbs_sub_mul_limb_same_length_in_place_left(&mut ns[j..i], ds, q);
            n_1 = ns[i - 1]; // update n_1, last loop's value will now be invalid
        } else {
            let carry;
            {
                let (ns_lo, ns_hi) = ns.split_at_mut(i - 2);
                let (new_q, new_n) = limbs_div_mod_three_limb_by_two_limb(
                    n_1, ns_hi[1], ns_hi[0], d_1, d_0, inverse,
                );
                let (new_n_1, mut n_0) = new_n.split_in_half();
                q = new_q;
                n_1 = new_n_1;
                let local_carry_1 = limbs_sub_mul_limb_same_length_in_place_left(
                    &mut ns_lo[j..],
                    ds_except_last_two,
                    q,
                );
                let local_carry_2 = n_0 < local_carry_1;
                n_0.wrapping_sub_assign(local_carry_1);
                carry = local_carry_2 && n_1 == 0;
                if local_carry_2 {
                    n_1.wrapping_sub_assign(1);
                }
                ns_hi[0] = n_0;
            }
            if carry {
                n_1.wrapping_add_assign(d_1);
                if limbs_slice_add_same_length_in_place_left(&mut ns[j..i - 1], ds_except_last) {
                    n_1.wrapping_add_assign(1);
                }
                q.wrapping_sub_assign(1);
            }
        }
        qs[j] = q;
    }
    ns[d_len - 1] = n_1;
    highest_q
}

/// Time: worst case O(n * log(n) ^ 2 * log(log(n)))
///
/// Additional memory: worst case O(n * log(n))
///
/// where n = `ds.len()`
///
/// This is mpn_dcpi1_div_qr_n from mpn/generic/dcpi1_div_qr.c.
pub(crate) fn _limbs_div_mod_divide_and_conquer_helper(
    qs: &mut [Limb],
    ns: &mut [Limb],
    ds: &[Limb],
    inverse: Limb,
    scratch: &mut [Limb],
) -> bool {
    let n = ds.len();
    let lo = n >> 1; // floor(n / 2)
    let hi = n - lo; // ceil(n / 2)
    let mut highest_q;
    {
        let qs_hi = &mut qs[lo..];
        let (ds_lo, ds_hi) = ds.split_at(lo);
        highest_q = if hi < DC_DIV_QR_THRESHOLD {
            _limbs_div_mod_schoolbook(qs_hi, &mut ns[2 * lo..2 * n], ds_hi, inverse)
        } else {
            _limbs_div_mod_divide_and_conquer_helper(
                qs_hi,
                &mut ns[2 * lo..],
                ds_hi,
                inverse,
                scratch,
            )
        };
        let qs_hi = &mut qs_hi[..hi];
        limbs_mul_greater_to_out(scratch, qs_hi, ds_lo);
        let ns_lo = &mut ns[..n + lo];
        let mut carry = if limbs_sub_same_length_in_place_left(&mut ns_lo[lo..], &scratch[..n]) {
            1
        } else {
            0
        };
        if highest_q && limbs_sub_same_length_in_place_left(&mut ns_lo[n..], ds_lo) {
            carry += 1;
        }
        while carry != 0 {
            if limbs_sub_limb_in_place(qs_hi, 1) {
                assert!(highest_q);
                highest_q = false;
            }
            if limbs_slice_add_same_length_in_place_left(&mut ns_lo[lo..], ds) {
                carry -= 1;
            }
        }
    }
    let (ds_lo, ds_hi) = ds.split_at(hi);
    let q_lo = if lo < DC_DIV_QR_THRESHOLD {
        _limbs_div_mod_schoolbook(qs, &mut ns[hi..n + lo], ds_hi, inverse)
    } else {
        _limbs_div_mod_divide_and_conquer_helper(qs, &mut ns[hi..], ds_hi, inverse, scratch)
    };
    let qs_lo = &mut qs[..lo];
    let ns_lo = &mut ns[..n];
    limbs_mul_greater_to_out(scratch, ds_lo, qs_lo);
    let mut carry = if limbs_sub_same_length_in_place_left(ns_lo, &scratch[..n]) {
        1
    } else {
        0
    };
    if q_lo && limbs_sub_same_length_in_place_left(&mut ns_lo[lo..], ds_lo) {
        // TODO This branch is untested!
        carry += 1;
    }
    while carry != 0 {
        limbs_sub_limb_in_place(qs_lo, 1);
        if limbs_slice_add_same_length_in_place_left(ns_lo, ds) {
            carry -= 1;
        }
    }
    highest_q
}

/// Recursive divide-and-conquer division.
///
/// Time: worst case O(n * log(n) ^ 2 * log(log(n)))
///
/// Additional memory: worst case O(n * log(n))
///
/// where n = `ds.len()`
///
/// # Panics
/// Panics if `ds` has length smaller than 6, `ns.len()` is less than `ds.len()` + 3, `qs` has
/// length less than `ns.len()` - `ds.len()`, or the last limb of `ds` does not have its highest bit
/// set.
///
/// This is mpn_dcpi1_div_qr from mpn/generic/dcpi1_div_qr.c.
pub fn _limbs_div_mod_divide_and_conquer(
    qs: &mut [Limb],
    ns: &mut [Limb],
    ds: &[Limb],
    inverse: Limb,
) -> bool {
    let n_len = ns.len();
    let d_len = ds.len();
    assert!(d_len >= 6); // to adhere to _limbs_div_mod_schoolbook's limits
    assert!(n_len >= d_len + 3); // to adhere to _limbs_div_mod_schoolbook's limits
    let a = d_len - 1;
    let d_1 = ds[a];
    let b = d_len - 2;
    assert!(d_1.get_highest_bit());
    let mut scratch = vec![0; d_len];
    let mut highest_q;
    let q_len = n_len - d_len;
    if q_len > d_len {
        let q_len_mod_d_len = {
            let mut m = q_len % d_len;
            if m == 0 {
                m = d_len;
            }
            m
        };
        // Perform the typically smaller block first.
        {
            // point at low limb of next quotient block
            let qs = &mut qs[q_len - q_len_mod_d_len..q_len];
            if q_len_mod_d_len == 1 {
                // Handle highest_q up front, for simplicity.
                let ns = &mut ns[q_len - 1..];
                {
                    let ns = &mut ns[1..];
                    highest_q = limbs_cmp_same_length(ns, ds) >= Ordering::Equal;
                    if highest_q {
                        assert!(!limbs_sub_same_length_in_place_left(ns, ds));
                    }
                }
                // A single iteration of schoolbook: One 3/2 division, followed by the bignum update
                // and adjustment.
                let (last_n, ns) = ns.split_last_mut().unwrap();
                let n_2 = *last_n;
                let mut n_1 = ns[a];
                let mut n_0 = ns[b];
                let d_0 = ds[b];
                assert!(n_2 < d_1 || n_2 == d_1 && n_1 <= d_0);
                let mut q;
                if n_2 == d_1 && n_1 == d_0 {
                    // TODO This branch is untested!
                    q = Limb::MAX;
                    assert_eq!(limbs_sub_mul_limb_same_length_in_place_left(ns, ds, q), n_2);
                } else {
                    let (new_q, new_n) =
                        limbs_div_mod_three_limb_by_two_limb(n_2, n_1, n_0, d_1, d_0, inverse);
                    q = new_q;
                    let (new_n_1, new_n_0) = new_n.split_in_half();
                    n_1 = new_n_1;
                    n_0 = new_n_0;
                    // d_len > 2 because of precondition. No need to check
                    let local_carry_1 =
                        limbs_sub_mul_limb_same_length_in_place_left(&mut ns[..b], &ds[..b], q);
                    let local_carry_2 = n_0 < local_carry_1;
                    n_0.wrapping_sub_assign(local_carry_1);
                    let carry = local_carry_2 && n_1 == 0;
                    if local_carry_2 {
                        n_1.wrapping_sub_assign(1);
                    }
                    ns[b] = n_0;
                    let (last_n, ns) = ns.split_last_mut().unwrap();
                    if carry {
                        n_1.wrapping_add_assign(d_1);
                        if limbs_slice_add_same_length_in_place_left(ns, &ds[..a]) {
                            n_1.wrapping_add_assign(1);
                        }
                        if q == 0 {
                            assert!(highest_q);
                            highest_q = false;
                        }
                        q.wrapping_sub_assign(1);
                    }
                    *last_n = n_1;
                }
                qs[0] = q;
            } else {
                // Do a 2 * q_len_mod_d_len / q_len_mod_d_len division
                let (ds_lo, ds_hi) = ds.split_at(d_len - q_len_mod_d_len);
                highest_q = {
                    let ns = &mut ns[n_len - (q_len_mod_d_len << 1)..];
                    if q_len_mod_d_len == 2 {
                        limbs_div_mod_by_two_limb_normalized(qs, ns, ds_hi)
                    } else if q_len_mod_d_len < DC_DIV_QR_THRESHOLD {
                        _limbs_div_mod_schoolbook(qs, ns, ds_hi, inverse)
                    } else {
                        _limbs_div_mod_divide_and_conquer_helper(
                            qs,
                            ns,
                            ds_hi,
                            inverse,
                            &mut scratch,
                        )
                    }
                };
                if q_len_mod_d_len != d_len {
                    limbs_mul_to_out(&mut scratch, qs, ds_lo);
                    let ns = &mut ns[q_len - q_len_mod_d_len..n_len - q_len_mod_d_len];
                    let mut carry = if limbs_sub_same_length_in_place_left(ns, &scratch) {
                        1
                    } else {
                        0
                    };
                    if highest_q
                        && limbs_sub_same_length_in_place_left(&mut ns[q_len_mod_d_len..], ds_lo)
                    {
                        carry += 1;
                    }
                    while carry != 0 {
                        if limbs_sub_limb_in_place(qs, 1) {
                            assert!(highest_q);
                            highest_q = false;
                        }
                        if limbs_slice_add_same_length_in_place_left(ns, ds) {
                            carry -= 1;
                        }
                    }
                }
            }
        }
        // offset is a multiple of d_len
        let mut offset = n_len.checked_sub(d_len + q_len_mod_d_len).unwrap();
        while offset != 0 {
            offset -= d_len;
            _limbs_div_mod_divide_and_conquer_helper(
                &mut qs[offset..],
                &mut ns[offset..],
                ds,
                inverse,
                &mut scratch,
            );
        }
    } else {
        let m = d_len - q_len;
        let (ds_lo, ds_hi) = ds.split_at(m);
        highest_q = if q_len < DC_DIV_QR_THRESHOLD {
            _limbs_div_mod_schoolbook(qs, &mut ns[m..], ds_hi, inverse)
        } else {
            _limbs_div_mod_divide_and_conquer_helper(qs, &mut ns[m..], ds_hi, inverse, &mut scratch)
        };
        if m != 0 {
            let qs = &mut qs[..q_len];
            let ns = &mut ns[..d_len];
            limbs_mul_to_out(&mut scratch, &qs, ds_lo);
            let mut carry = if limbs_sub_same_length_in_place_left(ns, &scratch) {
                1
            } else {
                0
            };
            if highest_q && limbs_sub_same_length_in_place_left(&mut ns[q_len..], ds_lo) {
                carry += 1;
            }
            while carry != 0 {
                if limbs_sub_limb_in_place(qs, 1) {
                    assert!(highest_q);
                    highest_q = false;
                }
                if limbs_slice_add_same_length_in_place_left(ns, ds) {
                    carry -= 1;
                }
            }
        }
    }
    highest_q
}

/// Takes the strictly normalized value ds (i.e., most significant bit must be set) as an input, and
/// computes the approximate reciprocal of `ds`, with the same length as `ds`. See documentation for
/// `_limbs_invert_approx` for an explanation of the return value.
///
/// Time: worst case O(n * log(n) ^ 2 * log(log(n)))
///
/// Additional memory: worst case O(n * log(n))
///
/// where n = `ds.len()`
///
/// # Panics
/// Panics if `ds` is empty, `is` is shorter than `ds`, `scratch` is shorter than twice the length
/// of `ds`, or the last limb of `ds` does not have its highest bit set.
///
/// This is mpn_bc_invertappr from mpn/generic/invertappr.c, where the return value is `true` iff
/// the return value of mpn_bc_invertappr would be 0.
pub fn _limbs_invert_basecase_approx(is: &mut [Limb], ds: &[Limb], scratch: &mut [Limb]) -> bool {
    let d_len = ds.len();
    assert_ne!(d_len, 0);
    let highest_d = ds[d_len - 1];
    assert!(highest_d.get_highest_bit());
    if d_len == 1 {
        let d = ds[0];
        is[0] = limbs_invert_limb(d);
    } else {
        let scratch = &mut scratch[..d_len << 1];
        {
            let (scratch_lo, scratch_hi) = scratch.split_at_mut(d_len);
            for s in scratch_lo.iter_mut() {
                *s = Limb::MAX;
            }
            limbs_not_to_out(scratch_hi, ds);
        }
        // Now scratch contains 2 ^ (2 * d_len * Limb::WIDTH) - d * 2 ^ (d_len * Limb::WIDTH) - 1
        if d_len == 2 {
            limbs_div_mod_by_two_limb_normalized(is, scratch, ds);
        } else {
            let inverse = limbs_two_limb_inverse_helper(highest_d, ds[d_len - 2]);
            if !MAYBE_DCP1_DIVAPPR || d_len < DC_DIVAPPR_Q_THRESHOLD {
                _limbs_div_schoolbook_approx(is, scratch, ds, inverse);
            } else {
                _limbs_div_divide_and_conquer_approx(is, scratch, ds, inverse);
            }
            assert!(!limbs_sub_limb_in_place(&mut is[..d_len], 1));
            return false;
        }
    }
    true
}

/// Takes the strictly normalized value ds (i.e., most significant bit must be set) as an input, and
/// computes the approximate reciprocal of `ds`, with the same length as `ds`. See documentation for
/// `_limbs_invert_approx` for an explanation of the return value.
///
/// Uses Newton's iterations (at least one). Inspired by Algorithm "ApproximateReciprocal",
/// published in "Modern Computer Arithmetic" by Richard P. Brent and Paul Zimmermann, algorithm
/// 3.5, page 121 in version 0.4 of the book.
///
/// Some adaptations were introduced, to allow product mod B ^ m - 1 and return the value e.
///
/// We introduced a correction in such a way that "the value of B ^ {n + h} - T computed at step 8
/// cannot exceed B ^ n - 1" (the book reads "2 * B ^ n - 1").
///
/// Maximum scratch needed by this branch <= 2 * n, but have to fit 3 * rn in the scratch, i.e.
/// 3 * rn <= 2 * n: we require n > 4.
///
/// We use a wrapped product modulo B ^ m - 1.
///
/// Time: Worst case O(n * log(n) * log(log(n)))
///
/// Additional memory: Worst case O(n * log(n))
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `ds` has length less than 5, `is` is shorter than `ds`, `scratch` is shorter than
/// twice the length of `ds`, or the last limb of `ds` does not have its highest bit set.
///
/// This is mpn_ni_invertappr from mpn/generic/invertappr.c, where the return value is `true` iff
/// the return value of mpn_ni_invertappr would be 0.
pub fn _limbs_invert_newton_approx(is: &mut [Limb], ds: &[Limb], scratch: &mut [Limb]) -> bool {
    let d_len = ds.len();
    assert!(d_len > 4);
    assert!(ds[d_len - 1].get_highest_bit());
    let is = &mut is[..d_len];
    // Compute the computation precisions from highest to lowest, leaving the base case size in
    // 'previous_d'.
    let mut size = d_len;
    let mut sizes = vec![size];
    size = (size >> 1) + 1;
    let mut scratch2 = vec![];
    let mut mul_size = 0;
    if d_len >= INV_MULMOD_BNM1_THRESHOLD {
        mul_size = _limbs_mul_mod_base_pow_n_minus_1_next_size(d_len + 1);
        scratch2 = vec![0; _limbs_mul_mod_base_pow_n_minus_1_scratch_len(mul_size, d_len, size)];
    }
    while size >= INV_NEWTON_THRESHOLD {
        sizes.push(size);
        size = (size >> 1) + 1;
    }
    // We compute the inverse of 0.ds as 1.is.
    // Compute a base value of previous_d limbs.
    _limbs_invert_basecase_approx(&mut is[d_len - size..], &ds[d_len - size..], scratch);
    let mut previous_size = size;
    // Use Newton's iterations to get the desired precision.
    for (i, &size) in sizes.iter().enumerate().rev() {
        // v    d       v
        // +----+-------+
        // ^ previous_d ^
        //
        // Compute i_j * d
        let ds_hi = &ds[d_len - size..];
        let condition = size < INV_MULMOD_BNM1_THRESHOLD || {
            mul_size = _limbs_mul_mod_base_pow_n_minus_1_next_size(size + 1);
            mul_size > size + previous_size
        };
        let diff = size - previous_size;
        {
            let is_hi = &mut is[d_len - previous_size..];
            if condition {
                limbs_mul_greater_to_out(scratch, ds_hi, is_hi);
                limbs_slice_add_same_length_in_place_left(
                    &mut scratch[previous_size..size + 1],
                    &ds_hi[..diff + 1],
                );
            // Remember we truncated mod B ^ (d + 1)
            // We computed (truncated) xp of length d + 1 <- 1.is * 0.ds
            } else {
                // Use B ^ mul_size - 1 wraparound
                _limbs_mul_mod_base_pow_n_minus_1(scratch, mul_size, ds_hi, is_hi, &mut scratch2);
                let scratch = &mut scratch[..mul_size + 1];
                // We computed {xp, mul_size} <- {is, previous_d} * {ds, d} mod (B ^ mul_size - 1)
                // We know that 2 * |is * ds + ds * B ^ previous_d - B ^ {previous_d + d}| <
                //      B ^ mul_size - 1
                // Add ds * B ^ previous_d mod (B ^ mul_size - 1)
                let mul_diff = mul_size - previous_size;
                assert!(size >= mul_diff);
                let (ds_hi_lo, ds_hi_hi) = ds_hi.split_at(mul_diff);
                let carry = limbs_slice_add_same_length_in_place_left(
                    &mut scratch[previous_size..mul_size],
                    ds_hi_lo,
                );
                // Subtract B ^ {previous_d + d}, maybe only compensate the carry
                scratch[mul_size] = 1; // set a limit for decrement
                {
                    let (scratch_lo, scratch_hi) = scratch.split_at_mut(size - mul_diff);
                    if !_limbs_add_same_length_with_carry_in_in_place_left(
                        scratch_lo, ds_hi_hi, carry,
                    ) {
                        assert!(!limbs_sub_limb_in_place(scratch_hi, 1));
                    }
                }
                // if decrement eroded xp[mul_size]
                let (scratch_last, scratch_init) = scratch.split_last_mut().unwrap();
                assert!(!limbs_sub_limb_in_place(
                    scratch_init,
                    1.wrapping_sub(*scratch_last)
                ));
                // Remember we are working mod B ^ mul_size - 1
            }
            if scratch[size] < 2 {
                // "positive" residue class
                let (scratch_lo, scratch_hi) = scratch.split_at_mut(size);
                let mut carry = scratch_hi[0] + 1; // 1 <= carry <= 2 here.
                if carry == 2 && !limbs_sub_same_length_in_place_left(scratch_lo, ds_hi) {
                    carry = 3;
                    assert!(limbs_sub_same_length_in_place_left(scratch_lo, ds_hi));
                }
                // 1 <= carry <= 3 here.
                if limbs_cmp_same_length(scratch_lo, ds_hi) == Ordering::Greater {
                    assert!(!limbs_sub_same_length_in_place_left(scratch_lo, ds_hi));
                    carry += 1;
                }
                let (scratch_lo, scratch_mid) = scratch_lo.split_at_mut(diff);
                let (ds_hi_lo, ds_hi_hi) = ds_hi.split_at(diff);
                let borrow = limbs_cmp_same_length(scratch_lo, ds_hi_lo) == Ordering::Greater;
                assert!(!_limbs_sub_same_length_with_borrow_in_to_out(
                    &mut scratch_hi[diff..],
                    ds_hi_hi,
                    scratch_mid,
                    borrow
                ));
                assert!(!limbs_sub_limb_in_place(is_hi, carry)); // 1 <= carry <= 4 here
            } else {
                // "negative" residue class
                assert!(scratch[size] >= Limb::MAX - 1);
                if condition {
                    assert!(!limbs_sub_limb_in_place(&mut scratch[..size + 1], 1));
                }
                let (scratch_lo, scratch_hi) = scratch.split_at_mut(size);
                if scratch_hi[0] != Limb::MAX {
                    assert!(!limbs_slice_add_limb_in_place(is_hi, 1));
                    assert!(limbs_slice_add_same_length_in_place_left(scratch_lo, ds_hi));
                }
                limbs_not_to_out(&mut scratch_hi[diff..size], &scratch_lo[diff..]);
            }
            // Compute x_j * u_j
            let (scratch_lo, scratch_hi) = scratch.split_at_mut(size + diff);
            limbs_mul_same_length_to_out(scratch_lo, &scratch_hi[..previous_size], is_hi);
        }
        let a = (previous_size << 1) - diff;
        let carry = {
            let (scratch_lo, scratch_hi) = scratch.split_at_mut(a);
            limbs_slice_add_same_length_in_place_left(
                &mut scratch_lo[previous_size..],
                &scratch_hi[3 * diff - previous_size..diff << 1],
            )
        };
        if _limbs_add_same_length_with_carry_in_to_out(
            &mut is[d_len - size..],
            &scratch[a..previous_size << 1],
            &scratch[size + previous_size..size << 1],
            carry,
        ) {
            assert!(!limbs_slice_add_limb_in_place(
                &mut is[d_len - previous_size..],
                1
            ));
        }
        if i == 0 {
            // Check for possible carry propagation from below. Be conservative.
            return scratch[a - 1] <= Limb::MAX - 7;
        }
        previous_size = size;
    }
    // The preceding loop always returns when i == 0. Since sizes is nonempty, this always happens.
    unreachable!();
}

/// Takes the strictly normalized value ds (i.e., most significant bit must be set) as an input, and
/// computes the approximate reciprocal of `ds`, with the same length as `ds`.
///
/// Let result_definitely_exact = _limbs_invert_basecase_approx(is, ds, scratch) be the returned
/// value. If result_definitely_exact is `true`, the error e is 0; otherwise, it may be 0 or 1. The
/// following condition is satisfied by the output:
///
/// ds * (2 ^ (n * Limb::WIDTH) + is) < 2 ^ (2 * n * Limb::WIDTH) <=
/// ds * (2 ^ (n * Limb::WIDTH) + is + 1 + e),
/// where n = `ds.len()`.
///
/// When the strict result is needed, i.e., e = 0 in the relation above, the function `mpn_invert`
/// (TODO!) should be used instead.
///
/// Time: Worst case O(n * log(n) * log(log(n)))
///
/// Additional memory: Worst case O(n * log(n))
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `ds` is empty, `is` is shorter than `ds`, `scratch` is shorter than twice the length
/// of `ds`, or the last limb of `ds` does not have its highest bit set.
///
/// This is mpn_invertappr from mpn/generic/invertappr.c, where the return value is `true` iff
/// the return value of mpn_invertappr would be 0.
pub fn _limbs_invert_approx(is: &mut [Limb], ds: &[Limb], scratch: &mut [Limb]) -> bool {
    if ds.len() < INV_NEWTON_THRESHOLD {
        _limbs_invert_basecase_approx(is, ds, scratch)
    } else {
        _limbs_invert_newton_approx(is, ds, scratch)
    }
}

pub(crate) const MUL_TO_MULMOD_BNM1_FOR_2NXN_THRESHOLD: usize = INV_MULMOD_BNM1_THRESHOLD >> 1;

// ds.len() >= 2
// n_len >= 3
// n_len >= ds.len()
// i_len == _limbs_div_mod_barrett_is_len(n_len - ds.len(), ds.len())
// qs.len() == i_len
// scratch_len ==  _limbs_mul_mod_base_pow_n_minus_1_next_size(ds.len() + 1)
// scratch.len() == _limbs_div_mod_barrett_scratch_len(n_len, d_len) - i_len
// rs_hi.len() == i_len
pub fn _limbs_div_barrett_large_product(
    scratch: &mut [Limb],
    ds: &[Limb],
    qs: &[Limb],
    rs_hi: &[Limb],
    scratch_len: usize,
    i_len: usize,
) {
    let d_len = ds.len();
    let (scratch, scratch_out) = scratch.split_at_mut(scratch_len);
    _limbs_mul_mod_base_pow_n_minus_1(scratch, scratch_len, ds, qs, scratch_out);
    if d_len + i_len > scratch_len {
        let (rs_hi_lo, rs_hi_hi) = rs_hi.split_at(scratch_len - d_len);
        let carry_1 = limbs_sub_in_place_left(scratch, rs_hi_hi);
        let carry_2 = limbs_cmp_same_length(rs_hi_lo, &scratch[d_len..]) == Ordering::Less;
        if !carry_1 && carry_2 {
            assert!(!limbs_slice_add_limb_in_place(scratch, 1));
        } else {
            assert_eq!(carry_1, carry_2);
        }
    }
}

/// Time: Worst case O(n * log(d) * log(log(d)))
///
/// Additional memory: Worst case O(d * log(d))
///
/// where n = `ns.len()`, d = `ds.len()`
///
/// This is mpn_preinv_mu_div_qr from mpn/generic/mu_div_qr.c.
fn _limbs_div_mod_barrett_preinverted(
    qs: &mut [Limb],
    rs: &mut [Limb],
    ns: &[Limb],
    ds: &[Limb],
    mut is: &[Limb],
    scratch: &mut [Limb],
) -> bool {
    let n_len = ns.len();
    let d_len = ds.len();
    assert_eq!(rs.len(), d_len);
    let mut i_len = is.len();
    let q_len = n_len - d_len;
    let qs = &mut qs[..q_len];
    let (ns_lo, ns_hi) = ns.split_at(q_len);
    let highest_q = limbs_cmp_same_length(ns_hi, ds) >= Ordering::Equal;
    if highest_q {
        limbs_sub_same_length_to_out(rs, ns_hi, ds);
    } else {
        rs.copy_from_slice(ns_hi);
    }
    let scratch_len = if i_len < MUL_TO_MULMOD_BNM1_FOR_2NXN_THRESHOLD {
        0
    } else {
        _limbs_mul_mod_base_pow_n_minus_1_next_size(d_len + 1)
    };
    let mut n = d_len - i_len;
    for (ns, qs) in ns_lo.rchunks(i_len).zip(qs.rchunks_mut(i_len)) {
        let chunk_len = ns.len();
        if i_len != chunk_len {
            // last iteration
            is = &is[i_len - chunk_len..];
            i_len = chunk_len;
            n = d_len - i_len;
        }
        let (rs_lo, rs_hi) = rs.split_at_mut(n);
        // Compute the next block of quotient limbs by multiplying the inverse by the upper part of
        // the partial remainder.
        limbs_mul_same_length_to_out(scratch, rs_hi, &is);
        // The inverse's most significant bit is implicit.
        assert!(!limbs_add_same_length_to_out(
            qs,
            &scratch[i_len..i_len << 1],
            rs_hi,
        ));
        // Compute the product of the quotient block and the divisor, to be subtracted from the
        // partial remainder combined with new limbs from the dividend. We only really need the low
        // d_len + 1 limbs.
        if i_len < MUL_TO_MULMOD_BNM1_FOR_2NXN_THRESHOLD {
            limbs_mul_greater_to_out(scratch, ds, qs);
        } else {
            _limbs_div_barrett_large_product(scratch, ds, qs, rs_hi, scratch_len, i_len)
        }
        let mut r = rs_hi[0].wrapping_sub(scratch[d_len]);
        // Subtract the product from the partial remainder combined with new limbs from the
        // dividend, generating a new partial remainder.
        let scratch = &mut scratch[..d_len];
        let carry = if n == 0 {
            // Get next i_len limbs from n.
            limbs_sub_same_length_to_out(rs, ns, scratch)
        } else {
            let (scratch_lo, scratch_hi) = scratch.split_at_mut(i_len);
            // Get next i_len limbs from n.
            let carry = _limbs_sub_same_length_with_borrow_in_in_place_right(
                rs_lo,
                scratch_hi,
                limbs_sub_same_length_in_place_right(ns, scratch_lo),
            );
            rs.copy_from_slice(scratch);
            carry
        };
        // Check the remainder and adjust the quotient as needed.
        if carry {
            r.wrapping_sub_assign(1);
        }
        while r != 0 {
            // We loop 0 times with about 69% probability, 1 time with about 31% probability, and 2
            // times with about 0.6% probability, if the inverse is computed as recommended.
            assert!(!limbs_slice_add_limb_in_place(qs, 1));
            if limbs_sub_same_length_in_place_left(rs, ds) {
                r -= 1;
            }
        }
        if limbs_cmp_same_length(rs, ds) >= Ordering::Equal {
            // This is executed with about 76% probability.
            assert!(!limbs_slice_add_limb_in_place(qs, 1));
            limbs_sub_same_length_in_place_left(rs, ds);
        }
    }
    highest_q
}

/// We distinguish 3 cases:
///
/// (a) d_len < q_len:              i_len = ceil(q_len / ceil(q_len / d_len))
/// (b) d_len / 3 < q_len <= d_len: i_len = ceil(q_len / 2)
/// (c) q_len < d_len / 3:          i_len = q_len
///
/// In all cases we have i_len <= d_len.
///
/// Time: Worst case O(1)
///
/// Additional memory: Worst case O(1)
///
/// Result is O(`q_len`)
///
/// This is mpn_mu_div_qr_choose_in from mpn/generic/mu_div_qr.c, where k == 0.
pub fn _limbs_div_mod_barrett_is_len(q_len: usize, d_len: usize) -> usize {
    let q_len_minus_1 = q_len - 1;
    if q_len > d_len {
        // Compute an inverse size that is a nice partition of the quotient.
        let b = q_len_minus_1 / d_len + 1; // ceil(q_len / d_len), number of blocks
        q_len_minus_1 / b + 1 // ceil(q_len / b) = ceil(q_len / ceil(q_len / d_len))
    } else if 3 * q_len > d_len {
        q_len_minus_1 / 2 + 1 // b = 2
    } else {
        q_len_minus_1 + 1 // b = 1
    }
}

/// Time: Worst case O(n * log(n) * log(log(n)))
///
/// Additional memory: Worst case O(n * log(n))
///
/// where n = `ns.len()`
///
/// This is mpn_mu_div_qr2 from mpn/generic/mu_div_qr.c.
pub fn _limbs_div_mod_barrett_helper(
    qs: &mut [Limb],
    rs: &mut [Limb],
    ns: &[Limb],
    ds: &[Limb],
    scratch: &mut [Limb],
) -> bool {
    let n_len = ns.len();
    let d_len = ds.len();
    assert_eq!(rs.len(), d_len);
    assert!(d_len > 1);
    assert!(n_len > d_len);
    let q_len = n_len - d_len;
    // Compute the inverse size.
    let i_len = _limbs_div_mod_barrett_is_len(q_len, d_len);
    assert!(i_len <= d_len);
    {
        let i_len_plus_1 = i_len + 1;
        let (is, scratch) = scratch.split_at_mut(i_len_plus_1);
        // compute an approximate inverse on i_len + 1 limbs
        if d_len == i_len {
            let (scratch_lo, scratch_hi) = scratch.split_at_mut(i_len_plus_1);
            {
                let (scratch_first, scratch_lo_tail) = scratch_lo.split_first_mut().unwrap();
                scratch_lo_tail.copy_from_slice(&ds[..i_len]);
                *scratch_first = 1;
            }
            _limbs_invert_approx(is, &scratch_lo, scratch_hi);
            limbs_move_left(is, 1);
        } else if limbs_add_limb_to_out(scratch, &ds[d_len - i_len_plus_1..], 1) {
            // TODO This branch is untested!
            limbs_set_zero(&mut is[..i_len]);
        } else {
            let (scratch_lo, scratch_hi) = scratch.split_at_mut(i_len_plus_1);
            _limbs_invert_approx(is, scratch_lo, scratch_hi);
            limbs_move_left(is, 1);
        }
    }
    let (scratch_lo, scratch_hi) = scratch.split_at_mut(i_len);
    _limbs_div_mod_barrett_preinverted(qs, rs, ns, ds, scratch_lo, scratch_hi)
}

/// Time: Worst case O(1)
///
/// Additional memory: Worst case O(1)
///
/// Result is O(`d_len`)
///
/// This is mpn_preinv_mu_div_qr_itch from mpn/generic/mu_div_qr.c, but nn is omitted from the
/// arguments as it is unused.
fn _limbs_div_mod_barrett_preinverse_scratch_len(d_len: usize, is_len: usize) -> usize {
    let itch_local = _limbs_mul_mod_base_pow_n_minus_1_next_size(d_len + 1);
    let itch_out = _limbs_mul_mod_base_pow_n_minus_1_scratch_len(itch_local, d_len, is_len);
    itch_local + itch_out
}

/// Time: Worst case O(1)
///
/// Additional memory: Worst case O(1)
///
/// This is mpn_invertappr_itch from gmp-impl.h.
pub(crate) const fn _limbs_invert_approx_scratch_len(is_len: usize) -> usize {
    is_len << 1
}

/// Time: Worst case O(1)
///
/// Additional memory: Worst case O(1)
///
/// Result is O(`n_len`)
///
/// This is mpn_mu_div_qr_itch from mpn/generic/mu_div_qr.c, where mua_k == 0.
pub fn _limbs_div_mod_barrett_scratch_len(n_len: usize, d_len: usize) -> usize {
    let is_len = _limbs_div_mod_barrett_is_len(n_len - d_len, d_len);
    let preinverse_len = _limbs_div_mod_barrett_preinverse_scratch_len(d_len, is_len);
    // 3 * is_len + 4
    let inverse_approx_len = _limbs_invert_approx_scratch_len(is_len + 1) + is_len + 2;
    assert!(preinverse_len >= inverse_approx_len);
    is_len + preinverse_len
}

pub fn _limbs_div_mod_barrett_large_helper(
    qs: &mut [Limb],
    rs: &mut [Limb],
    ns: &[Limb],
    ds: &[Limb],
    scratch: &mut [Limb],
) -> bool {
    let n_len = ns.len();
    let d_len = ds.len();
    let q_len = qs.len();
    let q_len_plus_one = q_len + 1;
    let n = n_len - q_len - q_len_plus_one; // 2 * d_len - n_len - 1
    let (ns_lo, ns_hi) = ns.split_at(n);
    let (ds_lo, ds_hi) = ds.split_at(d_len - q_len_plus_one);
    let (rs_lo, rs_hi) = rs.split_at_mut(n);
    let rs_hi = &mut rs_hi[..q_len_plus_one];
    let mut highest_q = _limbs_div_mod_barrett_helper(qs, rs_hi, ns_hi, ds_hi, scratch);
    // Multiply the quotient by the divisor limbs ignored above.
    // The product is d_len - 1 limbs long.
    limbs_mul_to_out(scratch, ds_lo, qs);
    let (scratch_last, scratch_init) = scratch[..d_len].split_last_mut().unwrap();
    *scratch_last = if highest_q
        && limbs_slice_add_same_length_in_place_left(&mut scratch_init[q_len..], ds_lo)
    {
        1
    } else {
        0
    };
    let (scratch_lo, scratch_hi) = scratch.split_at(n);
    let scratch_hi = &scratch_hi[..q_len_plus_one];
    if _limbs_sub_same_length_with_borrow_in_in_place_left(
        rs_hi,
        scratch_hi,
        limbs_sub_same_length_to_out(rs_lo, ns_lo, scratch_lo),
    ) {
        // TODO This branch is untested!
        if limbs_sub_limb_in_place(qs, 1) {
            assert!(highest_q);
            highest_q = false;
        }
        limbs_slice_add_same_length_in_place_left(&mut rs[..d_len], ds);
    }
    highest_q
}

/// Block-wise Barrett division. The idea of the algorithm used herein is to compute a smaller
/// inverted value than used in the standard Barrett algorithm, and thus save time in the Newton
/// iterations, and pay just a small price when using the inverted value for developing quotient
/// bits. This algorithm was presented at ICMS 2006.
///
/// `ns` must have length at least 3, `ds` must have length at least 2 and be no longer than `ns`,
/// and the most significant bit of `ds` must be set.
///
/// Time: Worst case O(n * log(n) * log(log(n)))
///
/// Additional memory: Worst case O(n * log(n))
///
/// where n = `ns.len()`
///
/// # Panics
/// Panics if `ds` has length smaller than 2, `ns.len()` is less than `ds.len()`, `qs` has length
/// less than `ns.len()` - `ds.len()`, `scratch` is too short, or the last limb of `ds` does not
/// have its highest bit set.
///
/// This is mpn_mu_div_qr from mpn/generic/mu_div_qr.c.
pub fn _limbs_div_mod_barrett(
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
    // Test whether 2 * d_len - n_len > MU_DIV_QR_SKEW_THRESHOLD
    if d_len <= q_len + MU_DIV_QR_SKEW_THRESHOLD {
        _limbs_div_mod_barrett_helper(qs, &mut rs[..d_len], ns, ds, scratch)
    } else {
        _limbs_div_mod_barrett_large_helper(qs, rs, ns, ds, scratch)
    }
}

/// `ds` must have length 2, `ns` must have length at least 2, `qs` must have length at least
/// `ns.len() - 2`, `rs` must have length at least 2, and the most-significant limb of `ds` must be
/// nonzero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `ns.len()`
fn _limbs_div_mod_by_two_limb(qs: &mut [Limb], rs: &mut [Limb], ns: &[Limb], ds: &[Limb]) {
    let n_len = ns.len();
    let ds_1 = ds[1];
    let bits = ds_1.leading_zeros();
    if bits == 0 {
        let mut ns = ns.to_vec();
        // always store n_len - 1 quotient limbs
        qs[n_len - 2] = if limbs_div_mod_by_two_limb_normalized(qs, &mut ns, ds) {
            1
        } else {
            0
        };
        rs[0] = ns[0];
        rs[1] = ns[1];
    } else {
        let ds_0 = ds[0];
        let cobits = Limb::WIDTH - bits;
        let mut ns_shifted = vec![0; n_len + 1];
        let ns_shifted = &mut ns_shifted;
        let carry = limbs_shl_to_out(ns_shifted, ns, bits);
        let ds_shifted = &mut [ds_0 << bits, (ds_1 << bits) | (ds_0 >> cobits)];
        if carry == 0 {
            // always store n_len - 1 quotient limbs
            qs[n_len - 2] =
                if limbs_div_mod_by_two_limb_normalized(qs, &mut ns_shifted[..n_len], ds_shifted) {
                    1
                } else {
                    0
                };
        } else {
            ns_shifted[n_len] = carry;
            limbs_div_mod_by_two_limb_normalized(qs, ns_shifted, ds_shifted);
        }
        let ns_shifted_1 = ns_shifted[1];
        rs[0] = (ns_shifted[0] >> bits) | (ns_shifted_1 << cobits);
        rs[1] = ns_shifted_1 >> bits;
    }
}

//TODO tune
pub(crate) const MUPI_DIV_QR_THRESHOLD: usize = 74;

/// This function is optimized for the case when the numerator has at least twice the length of the
/// denominator.
///
/// `ds` must have length at least 3, `ns` must be at least as long as `ds`, `qs` must have length
/// at least `ns.len() - ds.len() + 1`, `rs` must have the same length as `ds`, and the most-
/// significant limb of `ds` must be nonzero.
///
/// Time: Worst case O(n * log(n) * log(log(n)))
///
/// Additional memory: Worst case O(n * log(n))
///
/// where n = `ns.len()`
fn _limbs_div_mod_unbalanced(
    qs: &mut [Limb],
    rs: &mut [Limb],
    ns: &[Limb],
    ds: &[Limb],
    adjusted_n_len: usize,
) {
    let mut n_len = ns.len();
    let d_len = ds.len();
    qs[n_len - d_len] = 0; // zero high quotient limb
    let mut ds_shifted_vec;
    let ds_shifted: &[Limb];
    let mut ns_shifted_vec = vec![0; n_len + 1];
    let ns_shifted = &mut ns_shifted_vec;
    let bits = ds.last().unwrap().leading_zeros();
    if bits == 0 {
        ds_shifted = ds;
        ns_shifted[..n_len].copy_from_slice(ns);
    } else {
        // normalize divisor
        ds_shifted_vec = vec![0; d_len];
        limbs_shl_to_out(&mut ds_shifted_vec, ds, bits);
        ds_shifted = &ds_shifted_vec;
        let (ns_shifted_last, ns_shifted_init) = ns_shifted.split_last_mut().unwrap();
        *ns_shifted_last = limbs_shl_to_out(ns_shifted_init, ns, bits);
    }
    n_len = adjusted_n_len;
    let inverse = limbs_two_limb_inverse_helper(ds_shifted[d_len - 1], ds_shifted[d_len - 2]);
    let ns_shifted = &mut ns_shifted[..n_len];
    if d_len < DC_DIV_QR_THRESHOLD {
        _limbs_div_mod_schoolbook(qs, ns_shifted, ds_shifted, inverse);
        let ns_shifted = &ns_shifted[..d_len];
        if bits == 0 {
            rs.copy_from_slice(ns_shifted);
        } else {
            limbs_shr_to_out(rs, ns_shifted, bits);
        }
    } else if d_len < MUPI_DIV_QR_THRESHOLD
        || n_len < 2 * MU_DIV_QR_THRESHOLD
        || (2 * (MU_DIV_QR_THRESHOLD - MUPI_DIV_QR_THRESHOLD)) as f64 * d_len as f64
            + MUPI_DIV_QR_THRESHOLD as f64 * n_len as f64
            > d_len as f64 * n_len as f64
    {
        _limbs_div_mod_divide_and_conquer(qs, ns_shifted, ds_shifted, inverse);
        let ns_shifted = &ns_shifted[..d_len];
        if bits == 0 {
            rs.copy_from_slice(ns_shifted);
        } else {
            limbs_shr_to_out(rs, ns_shifted, bits);
        }
    } else {
        let scratch_len = _limbs_div_mod_barrett_scratch_len(n_len, d_len);
        let mut scratch = vec![0; scratch_len];
        _limbs_div_mod_barrett(qs, rs, ns_shifted, ds_shifted, &mut scratch);
        if bits != 0 {
            limbs_slice_shr_in_place(rs, bits);
        }
    }
}

/// The numerator must have less than twice the length of the denominator.
///
/// Problem:
///
/// Divide a numerator N with `n_len` limbs by a denominator D with `d_len` limbs, forming a
/// quotient of `q_len` = `n_len` - `d_len` + 1 limbs. When `q_len` is small compared to `d_len`,
/// conventional division algorithms perform poorly. We want an algorithm that has an expected
/// running time that is dependent only on `q_len`.
///
/// Algorithm (very informally stated):
///
/// 1) Divide the 2 * `q_len` most significant limbs from the numerator by the `q_len` most-
/// significant limbs from the denominator. Call the result `qest`. This is either the correct
/// quotient, or 1 or 2 too large. Compute the remainder from the division.
///
/// 2) Is the most significant limb from the remainder < p, where p is the product of the most-
/// significant limb from the quotient and the next(d)? (Next(d) denotes the next ignored limb from
/// the denominator.)  If it is, decrement `qest`, and adjust the remainder accordingly.
///
/// 3) Is the remainder >= `qest`?  If it is, `qest` is the desired quotient. The algorithm
/// terminates.
///
/// 4) Subtract `qest` * next(d) from the remainder. If there is borrow out, decrement `qest`, and
/// adjust the remainder accordingly.
///
/// 5) Skip one word from the denominator (i.e., let next(d) denote the next less significant limb).
///
/// `ds` must have length at least 3, `ns` must be at least as long as `ds` but no more than twice
/// as long, `qs` must have length at least `ns.len() - ds.len() + 1`,`rs` must have the same length
/// as `ds`, and the most-significant limb of `ds` must be nonzero.
///
/// Time: Worst case O(n * log(n) * log(log(n)))
///
/// Additional memory: Worst case O(n * log(n))
///
/// where n = `ns.len()`
pub(crate) fn _limbs_div_mod_balanced(
    qs: &mut [Limb],
    rs: &mut [Limb],
    ns: &[Limb],
    ds: &[Limb],
    adjust: bool,
) {
    let n_len = ns.len();
    let d_len = ds.len();
    let mut q_len = n_len - d_len;
    assert!(d_len >= q_len);
    qs[q_len] = 0; // zero high quotient limb
    if adjust {
        q_len += 1;
    } else if q_len == 0 {
        rs.copy_from_slice(&ns[..d_len]);
        return;
    }
    let q_len = q_len;
    // `i_len` is the (at least partially) ignored number of limbs.
    let i_len = d_len - q_len;
    // Normalize the denominator by shifting it to the left such that its most significant bit is
    // set. Then shift the numerator the same amount, to mathematically preserve the quotient.
    let bits = ds[d_len - 1].leading_zeros();
    let cobits = Limb::WIDTH - bits;
    let q_len_2 = q_len << 1;
    let m = n_len - q_len_2;
    let mut ns_shifted_vec = vec![0; q_len_2 + 1];
    let mut ds_shifted_vec;
    let ds_shifted: &[Limb];
    let ds_hi = &ds[i_len..];
    let ds_lo_last = ds[i_len - 1];
    let cy = if bits == 0 {
        ds_shifted = ds_hi;
        ns_shifted_vec[..q_len_2].copy_from_slice(&ns[m..]);
        0
    } else {
        ds_shifted_vec = vec![0; q_len];
        limbs_shl_to_out(&mut ds_shifted_vec, ds_hi, bits);
        ds_shifted_vec[0] |= ds_lo_last >> cobits;
        ds_shifted = &ds_shifted_vec;
        let cy = limbs_shl_to_out(&mut ns_shifted_vec, &ns[m..], bits);
        if !adjust {
            ns_shifted_vec[0] |= ns[m - 1] >> cobits;
        }
        cy
    };
    let ns_shifted = if adjust {
        ns_shifted_vec[q_len_2] = cy;
        &mut ns_shifted_vec[1..]
    } else {
        &mut ns_shifted_vec
    };
    // Get an approximate quotient using the extracted operands.
    if q_len == 1 {
        let n = DoubleLimb::join_halves(ns_shifted[1], ns_shifted[0]);
        let d = DoubleLimb::from(ds_shifted[0]);
        qs[0] = (n / d).lower_half();
        ns_shifted[0] = (n % d).lower_half();
    } else if q_len == 2 {
        limbs_div_mod_by_two_limb_normalized(qs, ns_shifted, ds_shifted);
    } else {
        let ns_shifted = &mut ns_shifted[..q_len_2];
        let inverse = limbs_two_limb_inverse_helper(ds_shifted[q_len - 1], ds_shifted[q_len - 2]);
        if q_len < DC_DIV_QR_THRESHOLD {
            _limbs_div_mod_schoolbook(qs, ns_shifted, ds_shifted, inverse);
        } else if q_len < MU_DIV_QR_THRESHOLD {
            _limbs_div_mod_divide_and_conquer(qs, ns_shifted, ds_shifted, inverse);
        } else {
            // TODO This branch is untested!
            let mut scratch = vec![0; _limbs_div_mod_barrett_scratch_len(q_len_2, q_len)];
            _limbs_div_mod_barrett(qs, rs, ns_shifted, ds_shifted, &mut scratch);
            ns_shifted[..q_len].copy_from_slice(&rs[..q_len]);
        }
    }
    // Multiply the first ignored divisor limb by the most significant quotient limb. If that
    // product is > the partial remainder's most significant limb, we know the quotient is too
    // large. This test quickly catches most cases where the quotient is too large; it catches all
    // cases where the quotient is 2 too large.
    let mut r_len = q_len;
    let mut x = ds_lo_last << bits;
    if i_len >= 2 {
        x |= ds[i_len - 2] >> 1 >> ((!bits) & Limb::WIDTH_MASK);
    }
    if ns_shifted[q_len - 1] < (DoubleLimb::from(x) * DoubleLimb::from(qs[q_len - 1])).upper_half()
    {
        assert!(!limbs_sub_limb_in_place(qs, 1));
        let carry = limbs_slice_add_same_length_in_place_left(&mut ns_shifted[..q_len], ds_shifted);
        if carry {
            // The partial remainder is safely large.
            ns_shifted[q_len] = if carry { 1 } else { 0 };
            r_len += 1;
        }
    }
    let mut quotient_too_large = false;
    let mut do_extra_cleanup = true;
    let mut scratch = vec![0; d_len];
    let mut i_len_alt = i_len;
    {
        let qs = &mut qs[..q_len];
        if bits != 0 {
            // Append the partially used numerator limb to the partial remainder.
            let carry_1 = limbs_slice_shl_in_place(&mut ns_shifted[..r_len], cobits);
            let mask = Limb::MAX >> bits;
            ns_shifted[0] |= ns[i_len - 1] & mask;
            // Update partial remainder with partially used divisor limb.
            let (ns_shifted_last, ns_shifted_init) =
                ns_shifted[..q_len + 1].split_last_mut().unwrap();
            let carry_2 = limbs_sub_mul_limb_same_length_in_place_left(
                ns_shifted_init,
                qs,
                ds[i_len - 1] & mask,
            );
            if q_len != r_len {
                assert!(*ns_shifted_last >= carry_2);
                ns_shifted_last.wrapping_sub_assign(carry_2);
            } else {
                let (difference, overflow) = carry_1.overflowing_sub(carry_2);
                *ns_shifted_last = difference;
                quotient_too_large = overflow;
                r_len += 1;
            }
            i_len_alt -= 1;
        }
        // True: partial remainder now is neutral, i.e., it is not shifted up.
        if i_len_alt == 0 {
            rs.copy_from_slice(&ns_shifted[..r_len]);
            do_extra_cleanup = false;
        } else {
            limbs_mul_to_out(&mut scratch, qs, &ds[..i_len_alt]);
        }
    }
    if do_extra_cleanup {
        let (scratch_lo, scratch_hi) = scratch.split_at_mut(i_len_alt);
        quotient_too_large |=
            limbs_sub_in_place_left(&mut ns_shifted[..r_len], &scratch_hi[..q_len]);
        let (rs_lo, rs_hi) = rs.split_at_mut(i_len_alt);
        let rs_hi_len = rs_hi.len();
        rs_hi.copy_from_slice(&ns_shifted[..rs_hi_len]);
        quotient_too_large |= limbs_sub_same_length_to_out(rs_lo, &ns[..i_len_alt], &scratch_lo)
            && limbs_sub_limb_in_place(&mut rs_hi[..min(rs_hi_len, r_len)], 1);
    }
    if quotient_too_large {
        assert!(!limbs_sub_limb_in_place(qs, 1));
        limbs_slice_add_same_length_in_place_left(rs, ds);
    }
}

/// Interpreting two slices of `Limb`s, `ns` and `ds`, as the limbs (in ascending order) of two
/// `Natural`s, divides them, returning the quotient and remainder. The quotient has
/// `ns.len() - ds.len() + 1` limbs and the remainder `ds.len()` limbs.
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
/// Panics if `ns` is shorter than `ds`, `ds` has length less than 2, or the most-significant limb
/// of `ds` is zero.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::div_mod::limbs_div_mod;
///
/// assert_eq!(limbs_div_mod(&[1, 2], &[3, 4]), (vec![0], vec![1, 2]));
/// assert_eq!(limbs_div_mod(&[1, 2, 3], &[4, 5]), (vec![2576980377, 0], vec![2576980381, 2]));
/// ```
///
/// This is mpn_tdiv_qr from mpn/generic/tdiv_qr.c, where qp and rp are returned.
pub fn limbs_div_mod(ns: &[Limb], ds: &[Limb]) -> (Vec<Limb>, Vec<Limb>) {
    let d_len = ds.len();
    let mut qs = vec![0; ns.len() - d_len + 1];
    let mut rs = vec![0; d_len];
    limbs_div_mod_to_out(&mut qs, &mut rs, ns, ds);
    (qs, rs)
}

/// Interpreting two slices of `Limb`s, `ns` and `ds`, as the limbs (in ascending order) of two
/// `Natural`s, divides them, writing the `ns.len() - ds.len() + 1` limbs of the quotient to `qs`
/// and the `ds.len()` limbs of the remainder to `rs`.
///
/// `ns` must be at least as long as `ds`, `qs` must have length at least `ns.len() - ds.len() + 1`,
/// `rs` must be at least as long as `ds`, and `ds` must have length at least 2 and its most
/// significant limb must be greater than zero.
///
/// Time: Worst case O(n * log(n) * log(log(n)))
///
/// Additional memory: Worst case O(n * log(n))
///
/// where n = `ns.len()`
///
/// # Panics
/// Panics if `qs` or `rs` are too short, `ns` is shorter than `ds`, `ds` has length less than 2, or
/// the most-significant limb of `ds` is zero.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::div_mod::limbs_div_mod_to_out;
///
/// let qs = &mut [10; 4];
/// let rs = &mut [10; 4];
/// limbs_div_mod_to_out(qs, rs, &[1, 2], &[3, 4]);
/// assert_eq!(qs, &[0, 10, 10, 10]);
/// assert_eq!(rs, &[1, 2, 10, 10]);
///
/// let qs = &mut [10; 4];
/// let rs = &mut [10; 4];
/// limbs_div_mod_to_out(qs, rs, &[1, 2, 3], &[4, 5]);
/// assert_eq!(qs, &[2576980377, 0, 10, 10]);
/// assert_eq!(rs, &[2576980381, 2, 10, 10]);
/// ```
///
/// This is mpn_tdiv_qr from mpn/generic/tdiv_qr.c.
pub fn limbs_div_mod_to_out(qs: &mut [Limb], rs: &mut [Limb], ns: &[Limb], ds: &[Limb]) {
    let n_len = ns.len();
    let d_len = ds.len();
    assert!(n_len >= d_len);
    assert!(qs.len() > n_len - d_len);
    let rs = &mut rs[..d_len];
    let ds_last = *ds.last().unwrap();
    assert!(d_len > 1 && ds_last != 0);
    if d_len == 2 {
        _limbs_div_mod_by_two_limb(qs, rs, ns, ds);
    } else {
        // conservative tests for quotient size
        let adjust = ns[n_len - 1] >= ds_last;
        let adjusted_n_len = if adjust { n_len + 1 } else { n_len };
        if adjusted_n_len < d_len << 1 {
            _limbs_div_mod_balanced(qs, rs, ns, ds, adjust);
        } else {
            _limbs_div_mod_unbalanced(qs, rs, ns, ds, adjusted_n_len);
        }
    }
}

impl Natural {
    pub(crate) fn div_mod_limb_ref(&self, other: Limb) -> (Natural, Limb) {
        if other == 0 {
            panic!("division by zero");
        } else if other == 1 {
            (self.clone(), 0)
        } else {
            match *self {
                Natural(Small(small)) => {
                    let (quotient, remainder) = small.div_rem(other);
                    (Natural(Small(quotient)), remainder)
                }
                Natural(Large(ref limbs)) => {
                    let (quotient_limbs, remainder) = limbs_div_limb_mod(limbs, other);
                    let mut quotient = Natural(Large(quotient_limbs));
                    quotient.trim();
                    (quotient, remainder)
                }
            }
        }
    }

    pub(crate) fn div_assign_mod_limb(&mut self, other: Limb) -> Limb {
        if other == 0 {
            panic!("division by zero");
        } else if other == 1 {
            0
        } else {
            let remainder = match *self {
                Natural(Small(ref mut small)) => {
                    return small.div_assign_rem(other);
                }
                Natural(Large(ref mut limbs)) => limbs_div_limb_in_place_mod(limbs, other),
            };
            self.trim();
            remainder
        }
    }

    pub(crate) fn ceiling_div_neg_mod_limb_ref(&self, other: Limb) -> (Natural, Limb) {
        let (quotient, remainder) = self.div_mod_limb_ref(other);
        if remainder == 0 {
            (quotient, 0)
        } else {
            (quotient.add_limb(1), other - remainder)
        }
    }

    pub(crate) fn ceiling_div_assign_neg_mod_limb(&mut self, other: Limb) -> Limb {
        let remainder = self.div_assign_mod_limb(other);
        if remainder == 0 {
            0
        } else {
            self.increment();
            other - remainder
        }
    }
}

impl DivMod<Natural> for Natural {
    type DivOutput = Natural;
    type ModOutput = Natural;

    /// Divides a `Natural` by a `Natural`, taking both `Natural`s by value and returning the
    /// quotient and remainder. The quotient is rounded towards negative infinity. The quotient and
    /// remainder satisfy `self` = q * `other` + r and 0 <= r < `other`.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivMod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(
    ///         format!("{:?}", Natural::from(23u32).div_mod(Natural::from(10u32))),
    ///         "(2, 3)"
    ///     );
    ///
    ///     // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    ///     assert_eq!(
    ///          format!("{:?}", Natural::from_str("1000000000000000000000000").unwrap()
    ///              .div_mod(Natural::from_str("1234567890987").unwrap())),
    ///          "(810000006723, 530068894399)"
    ///     );
    /// }
    /// ```
    #[inline]
    fn div_mod(mut self, other: Natural) -> (Natural, Natural) {
        let remainder = self.div_assign_mod(other);
        (self, remainder)
    }
}

impl<'a> DivMod<&'a Natural> for Natural {
    type DivOutput = Natural;
    type ModOutput = Natural;

    /// Divides a `Natural` by a `Natural`, taking the first `Natural` by value and the second by
    /// reference, and returning the quotient and remainder. The quotient is rounded towards
    /// negative infinity. The quotient and remainder satisfy `self` = q * `other` + r and
    /// 0 <= r < `other`.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivMod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(
    ///         format!("{:?}", Natural::from(23u32).div_mod(&Natural::from(10u32))),
    ///         "(2, 3)"
    ///     );
    ///
    ///     // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    ///     assert_eq!(
    ///          format!("{:?}", Natural::from_str("1000000000000000000000000").unwrap()
    ///              .div_mod(&Natural::from_str("1234567890987").unwrap())),
    ///          "(810000006723, 530068894399)"
    ///     );
    /// }
    /// ```
    #[inline]
    fn div_mod(mut self, other: &'a Natural) -> (Natural, Natural) {
        let remainder = self.div_assign_mod(other);
        (self, remainder)
    }
}

impl<'a> DivMod<Natural> for &'a Natural {
    type DivOutput = Natural;
    type ModOutput = Natural;

    /// Divides a `Natural` by a `Natural`, taking the first `Natural` by reference and the second
    /// by value, and returning the quotient and remainder. The quotient is rounded towards negative
    /// infinity. The quotient and remainder satisfy `self` = q * `other` + r and 0 <= r < `other`.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivMod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(
    ///         format!("{:?}", (&Natural::from(23u32)).div_mod(Natural::from(10u32))),
    ///         "(2, 3)"
    ///     );
    ///
    ///     // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    ///     assert_eq!(
    ///          format!("{:?}", (&Natural::from_str("1000000000000000000000000").unwrap())
    ///              .div_mod(Natural::from_str("1234567890987").unwrap())),
    ///          "(810000006723, 530068894399)"
    ///     );
    /// }
    /// ```
    fn div_mod(self, mut other: Natural) -> (Natural, Natural) {
        if other == 0 as Limb {
            panic!("division by zero");
        } else if other == 1 as Limb {
            (self.clone(), Natural::ZERO)
        } else if self.limb_count() < other.limb_count() {
            (Natural::ZERO, self.clone())
        } else {
            let qs = match (self, &mut other) {
                (x, &mut Natural(Small(y))) => {
                    let (q, r) = x.div_mod_limb_ref(y);
                    return (q, Natural(Small(r)));
                }
                (&Natural(Large(ref xs)), &mut Natural(Large(ref mut ys))) => {
                    let (qs, mut rs) = limbs_div_mod(xs, ys);
                    swap(&mut rs, ys);
                    qs
                }
                _ => unreachable!(),
            };
            let mut q = Natural(Large(qs));
            q.trim();
            other.trim();
            (q, other)
        }
    }
}

impl<'a, 'b> DivMod<&'b Natural> for &'a Natural {
    type DivOutput = Natural;
    type ModOutput = Natural;

    /// Divides a `Natural` by a `Natural`, taking both `Natural`s by reference and returning the
    /// quotient and remainder. The quotient is rounded towards negative infinity. The quotient and
    /// remainder satisfy `self` = q * `other` + r and 0 <= r < `other`.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivMod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(
    ///         format!("{:?}", (&Natural::from(23u32)).div_mod(&Natural::from(10u32))),
    ///         "(2, 3)"
    ///     );
    ///
    ///     // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    ///     assert_eq!(
    ///          format!("{:?}", (&Natural::from_str("1000000000000000000000000").unwrap())
    ///              .div_mod(&Natural::from_str("1234567890987").unwrap())),
    ///          "(810000006723, 530068894399)"
    ///     );
    /// }
    /// ```
    fn div_mod(self, other: &'b Natural) -> (Natural, Natural) {
        if *other == 0 as Limb {
            panic!("division by zero");
        } else if *other == 1 as Limb {
            (self.clone(), Natural::ZERO)
        } else if self as *const Natural == other as *const Natural {
            (Natural::ONE, Natural::ZERO)
        } else if self.limb_count() < other.limb_count() {
            (Natural::ZERO, self.clone())
        } else {
            let (qs, rs) = match (self, other) {
                (x, &Natural(Small(y))) => {
                    let (q, r) = x.div_mod_limb_ref(y);
                    return (q, Natural(Small(r)));
                }
                (&Natural(Large(ref xs)), &Natural(Large(ref ys))) => limbs_div_mod(xs, ys),
                _ => unreachable!(),
            };
            let mut q = Natural(Large(qs));
            q.trim();
            let mut r = Natural(Large(rs));
            r.trim();
            (q, r)
        }
    }
}

impl DivAssignMod<Natural> for Natural {
    type ModOutput = Natural;

    /// Divides a `Natural` by a `Natural` in place, taking the second `Natural` by value and
    /// returning the remainder. The quotient is rounded towards negative infinity. The quotient and
    /// remainder satisfy `self` = q * `other` + r and 0 <= r < `other`.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivAssignMod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     let mut x = Natural::from(23u32);
    ///     assert_eq!(x.div_assign_mod(Natural::from(10u32)).to_string(), "3");
    ///     assert_eq!(x.to_string(), "2");
    ///
    ///     // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    ///     let mut x = Natural::from_str("1000000000000000000000000").unwrap();
    ///     assert_eq!(x.div_assign_mod(Natural::from_str("1234567890987").unwrap()).to_string(),
    ///         "530068894399");
    ///     assert_eq!(x.to_string(), "810000006723");
    /// }
    /// ```
    fn div_assign_mod(&mut self, mut other: Natural) -> Natural {
        if other == 0 as Limb {
            panic!("division by zero");
        } else if other == 1 as Limb {
            Natural::ZERO
        } else if self.limb_count() < other.limb_count() {
            let mut r = Natural::ZERO;
            swap(self, &mut r);
            r
        } else {
            match (&mut *self, &mut other) {
                (x, &mut Natural(Small(y))) => {
                    return Natural(Small(x.div_assign_mod_limb(y)));
                }
                (&mut Natural(Large(ref mut xs)), &mut Natural(Large(ref mut ys))) => {
                    let (mut qs, mut rs) = limbs_div_mod(xs, ys);
                    swap(&mut qs, xs);
                    swap(&mut rs, ys);
                }
                _ => unreachable!(),
            };
            self.trim();
            other.trim();
            other
        }
    }
}

impl<'a> DivAssignMod<&'a Natural> for Natural {
    type ModOutput = Natural;

    /// Divides a `Natural` by a `Natural` in place, taking the second `Natural` by reference and
    /// returning the remainder. The quotient is rounded towards negative infinity. The quotient and
    /// remainder satisfy `self` = q * `other` + r and 0 <= r < `other`.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivAssignMod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     let mut x = Natural::from(23u32);
    ///     assert_eq!(x.div_assign_mod(&Natural::from(10u32)).to_string(), "3");
    ///     assert_eq!(x.to_string(), "2");
    ///
    ///     // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    ///     let mut x = Natural::from_str("1000000000000000000000000").unwrap();
    ///     assert_eq!(x.div_assign_mod(&Natural::from_str("1234567890987").unwrap()).to_string(),
    ///         "530068894399");
    ///     assert_eq!(x.to_string(), "810000006723");
    /// }
    /// ```
    fn div_assign_mod(&mut self, other: &'a Natural) -> Natural {
        if *other == 0 as Limb {
            panic!("division by zero");
        } else if *other == 1 as Limb {
            Natural::ZERO
        } else if self.limb_count() < other.limb_count() {
            let mut r = Natural::ZERO;
            swap(self, &mut r);
            r
        } else {
            let rs = match (&mut *self, other) {
                (x, &Natural(Small(y))) => {
                    return Natural(Small(x.div_assign_mod_limb(y)));
                }
                (&mut Natural(Large(ref mut xs)), &Natural(Large(ref ys))) => {
                    let (mut qs, rs) = limbs_div_mod(xs, ys);
                    swap(&mut qs, xs);
                    rs
                }
                _ => unreachable!(),
            };
            self.trim();
            let mut r = Natural(Large(rs));
            r.trim();
            r
        }
    }
}

impl DivRem<Natural> for Natural {
    type DivOutput = Natural;
    type RemOutput = Natural;

    /// Divides a `Natural` by a `Natural`, taking both `Natural`s by value and returning the
    /// quotient and remainder. The quotient is rounded towards zero. The quotient and remainder
    /// satisfy `self` = q * `other` + r and 0 <= r < `other`. For `Natural`s, rem is equivalent to
    /// mod.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivRem;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(
    ///         format!("{:?}", Natural::from(23u32).div_rem(Natural::from(10u32))),
    ///         "(2, 3)"
    ///     );
    ///
    ///     // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    ///     assert_eq!(
    ///          format!("{:?}", Natural::from_str("1000000000000000000000000").unwrap()
    ///              .div_rem(Natural::from_str("1234567890987").unwrap())),
    ///          "(810000006723, 530068894399)"
    ///     );
    /// }
    /// ```
    #[inline]
    fn div_rem(self, other: Natural) -> (Natural, Natural) {
        self.div_mod(other)
    }
}

impl<'a> DivRem<&'a Natural> for Natural {
    type DivOutput = Natural;
    type RemOutput = Natural;

    /// Divides a `Natural` by a `Natural`, taking the first `Natural` by value and the second by
    /// reference, and returning the quotient and remainder. The quotient is rounded towards zero.
    /// The quotient and remainder satisfy `self` = q * `other` + r and 0 <= r < `other`. For
    /// `Natural`s, rem is equivalent to mod.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivRem;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(
    ///         format!("{:?}", Natural::from(23u32).div_rem(&Natural::from(10u32))),
    ///         "(2, 3)"
    ///     );
    ///
    ///     // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    ///     assert_eq!(
    ///          format!("{:?}", Natural::from_str("1000000000000000000000000").unwrap()
    ///              .div_rem(&Natural::from_str("1234567890987").unwrap())),
    ///          "(810000006723, 530068894399)"
    ///     );
    /// }
    /// ```
    #[inline]
    fn div_rem(self, other: &'a Natural) -> (Natural, Natural) {
        self.div_mod(other)
    }
}

impl<'a> DivRem<Natural> for &'a Natural {
    type DivOutput = Natural;
    type RemOutput = Natural;

    /// Divides a `Natural` by a `Natural`, taking the first `Natural` by reference and the second
    /// by value, and returning the quotient and remainder. The quotient is rounded towards zero.
    /// The quotient and remainder satisfy `self` = q * `other` + r and 0 <= r < `other`. For
    /// `Natural`s, rem is equivalent to mod.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivRem;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(
    ///         format!("{:?}", (&Natural::from(23u32)).div_rem(Natural::from(10u32))),
    ///         "(2, 3)"
    ///     );
    ///
    ///     // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    ///     assert_eq!(
    ///          format!("{:?}", (&Natural::from_str("1000000000000000000000000").unwrap())
    ///              .div_rem(Natural::from_str("1234567890987").unwrap())),
    ///          "(810000006723, 530068894399)"
    ///     );
    /// }
    /// ```
    #[inline]
    fn div_rem(self, other: Natural) -> (Natural, Natural) {
        self.div_mod(other)
    }
}

impl<'a, 'b> DivRem<&'b Natural> for &'a Natural {
    type DivOutput = Natural;
    type RemOutput = Natural;

    /// Divides a `Natural` by a `Natural`, taking both `Natural`s by reference and returning the
    /// quotient and remainder. The quotient is rounded towards zero. The quotient and remainder
    /// satisfy `self` = q * `other` + r and 0 <= r < `other`. For `Natural`s, rem is equivalent to
    /// mod.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivRem;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(
    ///         format!("{:?}", (&Natural::from(23u32)).div_rem(&Natural::from(10u32))),
    ///         "(2, 3)"
    ///     );
    ///
    ///     // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    ///     assert_eq!(
    ///          format!("{:?}", (&Natural::from_str("1000000000000000000000000").unwrap())
    ///              .div_rem(&Natural::from_str("1234567890987").unwrap())),
    ///          "(810000006723, 530068894399)"
    ///     );
    /// }
    /// ```
    #[inline]
    fn div_rem(self, other: &'b Natural) -> (Natural, Natural) {
        self.div_mod(other)
    }
}

impl DivAssignRem<Natural> for Natural {
    type RemOutput = Natural;

    /// Divides a `Natural` by a `Natural` in place, taking the second `Natural` by value and
    /// returning the remainder. The quotient is rounded towards zero. The quotient and remainder
    /// satisfy `self` = q * `other` + r and 0 <= r < `other`. For `Natural`s, rem is equivalent to
    /// mod.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivAssignRem;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     let mut x = Natural::from(23u32);
    ///     assert_eq!(x.div_assign_rem(Natural::from(10u32)).to_string(), "3");
    ///     assert_eq!(x.to_string(), "2");
    ///
    ///     // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    ///     let mut x = Natural::from_str("1000000000000000000000000").unwrap();
    ///     assert_eq!(x.div_assign_rem(Natural::from_str("1234567890987").unwrap()).to_string(),
    ///         "530068894399");
    ///     assert_eq!(x.to_string(), "810000006723");
    /// }
    /// ```
    #[inline]
    fn div_assign_rem(&mut self, other: Natural) -> Natural {
        self.div_assign_mod(other)
    }
}

impl<'a> DivAssignRem<&'a Natural> for Natural {
    type RemOutput = Natural;

    /// Divides a `Natural` by a `Natural` in place, taking the second `Natural` by reference and
    /// returning the remainder. The quotient is rounded towards zero. The quotient and remainder
    /// satisfy `self` = q * `other` + r and 0 <= r < `other`. For `Natural`s, rem is equivalent to
    /// mod.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivAssignRem;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     let mut x = Natural::from(23u32);
    ///     assert_eq!(x.div_assign_rem(&Natural::from(10u32)).to_string(), "3");
    ///     assert_eq!(x.to_string(), "2");
    ///
    ///     // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    ///     let mut x = Natural::from_str("1000000000000000000000000").unwrap();
    ///     assert_eq!(x.div_assign_rem(&Natural::from_str("1234567890987").unwrap()).to_string(),
    ///         "530068894399");
    ///     assert_eq!(x.to_string(), "810000006723");
    /// }
    /// ```
    #[inline]
    fn div_assign_rem(&mut self, other: &'a Natural) -> Natural {
        self.div_assign_mod(other)
    }
}

impl CeilingDivNegMod<Natural> for Natural {
    type DivOutput = Natural;
    type ModOutput = Natural;

    /// Divides a `Natural` by a `Natural`, taking both `Natural`s by value and returning the
    /// ceiling of the quotient and the remainder of the negative of the first `Natural` divided by
    /// the second. The quotient and remainder satisfy `self` = q * `other` - r and
    /// 0 <= r < `other`.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CeilingDivNegMod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 3 * 10 - 7 = 23
    ///     assert_eq!(
    ///         format!("{:?}", Natural::from(23u32).ceiling_div_neg_mod(Natural::from(10u32))),
    ///         "(3, 7)"
    ///     );
    ///
    ///     // 810000006724 * 1234567890987 - 704498996588 = 1000000000000000000000000
    ///     assert_eq!(
    ///          format!("{:?}", Natural::from_str("1000000000000000000000000").unwrap()
    ///              .ceiling_div_neg_mod(Natural::from_str("1234567890987").unwrap())),
    ///          "(810000006724, 704498996588)"
    ///     );
    /// }
    /// ```
    #[inline]
    fn ceiling_div_neg_mod(mut self, other: Natural) -> (Natural, Natural) {
        let remainder = self.ceiling_div_assign_neg_mod(other);
        (self, remainder)
    }
}

impl<'a> CeilingDivNegMod<&'a Natural> for Natural {
    type DivOutput = Natural;
    type ModOutput = Natural;

    /// Divides a `Natural` by a `Natural`, taking the first `Natural` by value and the second by
    /// reference, and returning the ceiling of the quotient and the remainder of the negative of
    /// the first `Natural` divided by the second. The quotient and remainder satisfy `self` =
    /// q * `other` - r and 0 <= r < `other`.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CeilingDivNegMod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 3 * 10 - 7 = 23
    ///     assert_eq!(
    ///         format!("{:?}", Natural::from(23u32).ceiling_div_neg_mod(&Natural::from(10u32))),
    ///         "(3, 7)"
    ///     );
    ///
    ///     // 810000006724 * 1234567890987 - 704498996588 = 1000000000000000000000000
    ///     assert_eq!(
    ///          format!("{:?}", Natural::from_str("1000000000000000000000000").unwrap()
    ///              .ceiling_div_neg_mod(&Natural::from_str("1234567890987").unwrap())),
    ///          "(810000006724, 704498996588)"
    ///     );
    /// }
    /// ```
    #[inline]
    fn ceiling_div_neg_mod(mut self, other: &'a Natural) -> (Natural, Natural) {
        let remainder = self.ceiling_div_assign_neg_mod(other);
        (self, remainder)
    }
}

impl<'a> CeilingDivNegMod<Natural> for &'a Natural {
    type DivOutput = Natural;
    type ModOutput = Natural;

    /// Divides a `Natural` by a `Natural`, taking the first `Natural` by reference and the second
    /// by value, and returning the ceiling of the quotient and the remainder of the negative of the
    /// first `Natural` divided by the second. The quotient and remainder satisfy `self` =
    /// q * `other` - r and 0 <= r < `other`.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CeilingDivNegMod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 3 * 10 - 7 = 23
    ///     assert_eq!(
    ///         format!("{:?}", (&Natural::from(23u32)).ceiling_div_neg_mod(Natural::from(10u32))),
    ///         "(3, 7)"
    ///     );
    ///
    ///     // 810000006724 * 1234567890987 - 704498996588 = 1000000000000000000000000
    ///     assert_eq!(
    ///          format!("{:?}", (&Natural::from_str("1000000000000000000000000").unwrap())
    ///              .ceiling_div_neg_mod(Natural::from_str("1234567890987").unwrap())),
    ///          "(810000006724, 704498996588)"
    ///     );
    /// }
    /// ```
    fn ceiling_div_neg_mod(self, other: Natural) -> (Natural, Natural) {
        let (quotient, remainder) = self.div_mod(&other);
        if remainder == 0 as Limb {
            (quotient, remainder)
        } else {
            (quotient.add_limb(1), other - remainder)
        }
    }
}

impl<'a, 'b> CeilingDivNegMod<&'b Natural> for &'a Natural {
    type DivOutput = Natural;
    type ModOutput = Natural;

    /// Divides a `Natural` by a `Natural`, taking both `Natural`s by reference and returning the
    /// ceiling of the quotient and the remainder of the negative of the first `Natural` divided by
    /// the second. The quotient and remainder satisfy `self` = q * `other` - r and
    /// 0 <= r < `other`.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CeilingDivNegMod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 3 * 10 - 7 = 23
    ///     assert_eq!(
    ///         format!("{:?}", (&Natural::from(23u32)).ceiling_div_neg_mod(&Natural::from(10u32))),
    ///         "(3, 7)"
    ///     );
    ///
    ///     // 810000006724 * 1234567890987 - 704498996588 = 1000000000000000000000000
    ///     assert_eq!(
    ///          format!("{:?}", (&Natural::from_str("1000000000000000000000000").unwrap())
    ///              .ceiling_div_neg_mod(&Natural::from_str("1234567890987").unwrap())),
    ///          "(810000006724, 704498996588)"
    ///     );
    /// }
    /// ```
    fn ceiling_div_neg_mod(self, other: &'b Natural) -> (Natural, Natural) {
        let (quotient, remainder) = self.div_mod(other);
        if remainder == 0 as Limb {
            (quotient, remainder)
        } else {
            (quotient.add_limb(1), other - remainder)
        }
    }
}

impl CeilingDivAssignNegMod<Natural> for Natural {
    type ModOutput = Natural;

    /// Divides a `Natural` by a `Natural` in place, taking the second `Natural` by value, taking
    /// the ceiling of the quotient, and returning the remainder of the negative of the first
    /// `Natural` divided by the second. The quotient and remainder satisfy `self` = q * `other` - r
    /// and 0 <= r < `other`.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CeilingDivAssignNegMod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 3 * 10 - 7 = 23
    ///     let mut x = Natural::from(23u32);
    ///     assert_eq!(x.ceiling_div_assign_neg_mod(Natural::from(10u32)).to_string(), "7");
    ///     assert_eq!(x.to_string(), "3");
    ///
    ///     // 810000006724 * 1234567890987 - 704498996588 = 1000000000000000000000000
    ///     let mut x = Natural::from_str("1000000000000000000000000").unwrap();
    ///     assert_eq!(
    ///         x.ceiling_div_assign_neg_mod(
    ///             Natural::from_str("1234567890987").unwrap()
    ///         ).to_string(),
    ///         "704498996588",
    ///     );
    ///     assert_eq!(x.to_string(), "810000006724");
    /// }
    /// ```
    fn ceiling_div_assign_neg_mod(&mut self, other: Natural) -> Natural {
        let remainder = self.div_assign_mod(&other);
        if remainder == 0 as Limb {
            Natural::ZERO
        } else {
            self.increment();
            other - remainder
        }
    }
}

impl<'a> CeilingDivAssignNegMod<&'a Natural> for Natural {
    type ModOutput = Natural;

    /// Divides a `Natural` by a `Natural` in place, taking the second `Natural` by reference,
    /// taking the ceiling of the quotient, and returning the remainder of the negative of the first
    /// `Natural` divided by the second. The quotient and remainder satisfy `self` = q * `other` - r
    /// and 0 <= r < `other`.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CeilingDivAssignNegMod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 3 * 10 - 7 = 23
    ///     let mut x = Natural::from(23u32);
    ///     assert_eq!(x.ceiling_div_assign_neg_mod(&Natural::from(10u32)).to_string(), "7");
    ///     assert_eq!(x.to_string(), "3");
    ///
    ///     // 810000006724 * 1234567890987 - 704498996588 = 1000000000000000000000000
    ///     let mut x = Natural::from_str("1000000000000000000000000").unwrap();
    ///     assert_eq!(
    ///         x.ceiling_div_assign_neg_mod(
    ///             &Natural::from_str("1234567890987").unwrap()
    ///         ).to_string(),
    ///         "704498996588",
    ///     );
    ///     assert_eq!(x.to_string(), "810000006724");
    /// }
    /// ```
    fn ceiling_div_assign_neg_mod(&mut self, other: &'a Natural) -> Natural {
        let remainder = self.div_assign_mod(other);
        if remainder == 0 as Limb {
            Natural::ZERO
        } else {
            self.increment();
            other - remainder
        }
    }
}

pub fn _limbs_div_limb_to_out_mod_naive(
    out: &mut [Limb],
    in_limbs: &[Limb],
    divisor: Limb,
) -> Limb {
    assert!(out.len() >= in_limbs.len());
    let divisor = DoubleLimb::from(divisor);
    let mut upper = 0;
    for (out_limb, &in_limb) in out.iter_mut().zip(in_limbs.iter()).rev() {
        let (quotient, remainder) = DoubleLimb::join_halves(upper, in_limb).div_rem(divisor);
        *out_limb = quotient.lower_half();
        upper = remainder.lower_half();
    }
    upper
}

pub fn _limbs_div_limb_in_place_mod_naive(limbs: &mut [Limb], divisor: Limb) -> Limb {
    let divisor = DoubleLimb::from(divisor);
    let mut upper = 0;
    for limb in limbs.iter_mut().rev() {
        let (quotient, remainder) = DoubleLimb::join_halves(upper, *limb).div_rem(divisor);
        *limb = quotient.lower_half();
        upper = remainder.lower_half();
    }
    upper
}

/// The high bit of `divisor` must be set.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// This is mpn_div_qr_1n_pi1 from mpn/generic/div_qr_1n_pi1.c with DIV_QR_1N_METHOD == 2, where
/// qp == up.
fn limbs_div_limb_normalized_in_place_mod(
    limbs: &mut [Limb],
    highest_limb: Limb,
    divisor: Limb,
    divisor_inverse: Limb,
) -> Limb {
    let len = limbs.len();
    if len == 1 {
        let (quotient, remainder) =
            div_mod_by_preinversion(highest_limb, limbs[0], divisor, divisor_inverse);
        limbs[0] = quotient;
        return remainder;
    }
    let power_of_two = divisor.wrapping_neg().wrapping_mul(divisor_inverse);
    let (mut quotient_high, mut quotient_low) =
        (DoubleLimb::from(divisor_inverse) * DoubleLimb::from(highest_limb)).split_in_half();
    quotient_high.wrapping_add_assign(highest_limb);
    let second_highest_limb = limbs[len - 1];
    limbs[len - 1] = quotient_high;
    let (sum, mut big_carry) = DoubleLimb::join_halves(second_highest_limb, limbs[len - 2])
        .overflowing_add(DoubleLimb::from(power_of_two) * DoubleLimb::from(highest_limb));
    let (mut sum_high, mut sum_low) = sum.split_in_half();
    for j in (0..len - 2).rev() {
        let (temp, remainder) =
            (DoubleLimb::from(sum_high) * DoubleLimb::from(divisor_inverse)).split_in_half();
        let mut quotient =
            DoubleLimb::from(sum_high) + DoubleLimb::from(temp) + DoubleLimb::from(quotient_low);
        quotient_low = remainder;
        if big_carry {
            quotient.wrapping_add_assign(DoubleLimb::join_halves(1, divisor_inverse));
            let (sum, carry) = sum_low.overflowing_add(power_of_two);
            sum_low = sum;
            if carry {
                sum_low.wrapping_sub_assign(divisor);
                quotient.wrapping_add_assign(1);
            }
        }
        let (quotient_higher, quotient_high) = quotient.split_in_half();
        limbs[j + 1] = quotient_high;
        assert!(!limbs_slice_add_limb_in_place(
            &mut limbs[j + 2..],
            quotient_higher,
        ));
        let (sum, carry) = DoubleLimb::join_halves(sum_low, limbs[j])
            .overflowing_add(DoubleLimb::from(sum_high) * DoubleLimb::from(power_of_two));
        sum_high = sum.upper_half();
        sum_low = sum.lower_half();
        big_carry = carry;
    }
    let mut quotient_high = 0;
    if big_carry {
        quotient_high += 1;
        sum_high.wrapping_sub_assign(divisor);
    }
    if sum_high >= divisor {
        quotient_high += 1;
        sum_high.wrapping_sub_assign(divisor);
    }
    let (temp, remainder) = div_mod_by_preinversion(sum_high, sum_low, divisor, divisor_inverse);
    let (quotient_high, quotient_low) = DoubleLimb::join_halves(quotient_high, quotient_low)
        .wrapping_add(DoubleLimb::from(temp))
        .split_in_half();
    assert!(!limbs_slice_add_limb_in_place(
        &mut limbs[1..],
        quotient_high,
    ));
    limbs[0] = quotient_low;
    remainder
}

/// The high bit of `divisor` must be set.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `limbs.len()`
///
/// This is mpn_div_qr_1n_pi1 from mpn/generic/div_qr_1n_pi1.c with DIV_QR_1N_METHOD == 2.
fn limbs_div_limb_normalized_to_out_mod(
    out: &mut [Limb],
    in_limbs: &[Limb],
    highest_limb: Limb,
    divisor: Limb,
    divisor_inverse: Limb,
) -> Limb {
    let len = in_limbs.len();
    if len == 1 {
        let (quotient, remainder) =
            div_mod_by_preinversion(highest_limb, in_limbs[0], divisor, divisor_inverse);
        out[0] = quotient;
        return remainder;
    }
    let power_of_two = divisor.wrapping_neg().wrapping_mul(divisor_inverse);
    let (mut quotient_high, mut quotient_low) =
        (DoubleLimb::from(divisor_inverse) * DoubleLimb::from(highest_limb)).split_in_half();
    quotient_high.wrapping_add_assign(highest_limb);
    out[len - 1] = quotient_high;
    let (sum, mut big_carry) = DoubleLimb::join_halves(in_limbs[len - 1], in_limbs[len - 2])
        .overflowing_add(DoubleLimb::from(power_of_two) * DoubleLimb::from(highest_limb));
    let (mut sum_high, mut sum_low) = sum.split_in_half();
    for j in (0..len - 2).rev() {
        let (temp, remainder) =
            (DoubleLimb::from(sum_high) * DoubleLimb::from(divisor_inverse)).split_in_half();
        let mut quotient =
            DoubleLimb::from(sum_high) + DoubleLimb::from(temp) + DoubleLimb::from(quotient_low);
        quotient_low = remainder;
        if big_carry {
            quotient.wrapping_add_assign(DoubleLimb::join_halves(1, divisor_inverse));
            let (sum, carry) = sum_low.overflowing_add(power_of_two);
            sum_low = sum;
            if carry {
                sum_low.wrapping_sub_assign(divisor);
                quotient.wrapping_add_assign(1);
            }
        }
        let (quotient_higher, quotient_high) = quotient.split_in_half();
        out[j + 1] = quotient_high;
        assert!(!limbs_slice_add_limb_in_place(
            &mut out[j + 2..],
            quotient_higher,
        ));
        let (sum, carry) = DoubleLimb::join_halves(sum_low, in_limbs[j])
            .overflowing_add(DoubleLimb::from(sum_high) * DoubleLimb::from(power_of_two));
        sum_high = sum.upper_half();
        sum_low = sum.lower_half();
        big_carry = carry;
    }
    let mut quotient_high = 0;
    if big_carry {
        quotient_high += 1;
        sum_high.wrapping_sub_assign(divisor);
    }
    if sum_high >= divisor {
        quotient_high += 1;
        sum_high.wrapping_sub_assign(divisor);
    }
    let (temp, remainder) = div_mod_by_preinversion(sum_high, sum_low, divisor, divisor_inverse);
    let (quotient_high, quotient_low) = DoubleLimb::join_halves(quotient_high, quotient_low)
        .wrapping_add(DoubleLimb::from(temp))
        .split_in_half();
    assert!(!limbs_slice_add_limb_in_place(&mut out[1..], quotient_high));
    out[0] = quotient_low;
    remainder
}

/// This is mpn_div_qr_1 from mpn/generic/div_qr_1.c where len > 1. Experiments show that this is
/// always slower than `_limbs_div_limb_to_out_mod`.
pub fn _limbs_div_limb_to_out_mod_alt(out: &mut [Limb], in_limbs: &[Limb], divisor: Limb) -> Limb {
    assert_ne!(divisor, 0);
    let len = in_limbs.len();
    assert!(len > 1);
    let out = &mut out[..len];
    assert!(out.len() >= len);
    let (highest_limb, in_limbs_init) = in_limbs.split_last().unwrap();
    let mut highest_limb = *highest_limb;
    let bits = divisor.leading_zeros();
    if bits == 0 {
        let (out_last, out_init) = out.split_last_mut().unwrap();
        *out_last = if highest_limb >= divisor {
            highest_limb -= divisor;
            1
        } else {
            0
        };
        let inverse = limbs_invert_limb(divisor);
        limbs_div_limb_normalized_to_out_mod(
            out_init,
            in_limbs_init,
            highest_limb,
            divisor,
            inverse,
        )
    } else {
        let divisor = divisor << bits;
        let highest_limb = limbs_shl_to_out(out, in_limbs, bits);
        let inverse = limbs_invert_limb(divisor);
        let (out_last, out_init) = out.split_last_mut().unwrap();
        let (quotient, remainder) =
            div_mod_by_preinversion(highest_limb, *out_last, divisor, inverse);
        *out_last = quotient;
        limbs_div_limb_normalized_in_place_mod(out_init, remainder, divisor, inverse) >> bits
    }
}

/// This is mpn_div_qr_1 from mpn/generic/div_qr_1.c where qp == up and len > 1. Experiments show
/// that this is always slower than `_limbs_div_limb_in_place_mod`.
pub fn _limbs_div_limb_in_place_mod_alt(limbs: &mut [Limb], divisor: Limb) -> Limb {
    assert_ne!(divisor, 0);
    let len = limbs.len();
    assert!(len > 1);
    let len_minus_1 = len - 1;
    let mut highest_limb = limbs[len_minus_1];
    let bits = divisor.leading_zeros();
    if bits == 0 {
        limbs[len_minus_1] = if highest_limb >= divisor {
            highest_limb -= divisor;
            1
        } else {
            0
        };
        let limb_inverse = limbs_invert_limb(divisor);
        limbs_div_limb_normalized_in_place_mod(
            &mut limbs[..len_minus_1],
            highest_limb,
            divisor,
            limb_inverse,
        )
    } else {
        let divisor = divisor << bits;
        let highest_limb = limbs_slice_shl_in_place(limbs, bits);
        let limb_inverse = limbs_invert_limb(divisor);
        let (quotient, remainder) =
            div_mod_by_preinversion(highest_limb, limbs[len_minus_1], divisor, limb_inverse);
        limbs[len_minus_1] = quotient;
        limbs_div_limb_normalized_in_place_mod(
            &mut limbs[..len_minus_1],
            remainder,
            divisor,
            limb_inverse,
        ) >> bits
    }
}
