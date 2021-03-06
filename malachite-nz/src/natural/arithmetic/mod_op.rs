use malachite_base::num::arithmetic::traits::{
    Mod, ModAssign, ModPowerOf2, NegMod, NegModAssign, Parity, PowerOf2, WrappingAddAssign,
    WrappingMulAssign, WrappingSubAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{Iverson, Zero};
use malachite_base::num::conversion::traits::{JoinHalves, SplitInHalf};
use malachite_base::num::logic::traits::LeadingZeros;
use malachite_base::slices::{slice_move_left, slice_set_zero};
use natural::arithmetic::add::{
    limbs_add_limb_to_out, limbs_add_same_length_to_out, limbs_slice_add_same_length_in_place_left,
};
use natural::arithmetic::div_mod::{
    _limbs_div_barrett_large_product, _limbs_div_mod_balanced, _limbs_div_mod_barrett_helper,
    _limbs_div_mod_barrett_is_len, _limbs_div_mod_barrett_scratch_len,
    _limbs_div_mod_divide_and_conquer_helper, _limbs_div_mod_schoolbook, _limbs_invert_approx,
    limbs_div_mod_by_two_limb_normalized, limbs_div_mod_three_limb_by_two_limb, limbs_invert_limb,
    limbs_two_limb_inverse_helper, MUL_TO_MULMOD_BNM1_FOR_2NXN_THRESHOLD, MUPI_DIV_QR_THRESHOLD,
};
use natural::arithmetic::mul::mul_mod::_limbs_mul_mod_base_pow_n_minus_1_next_size;
use natural::arithmetic::mul::{
    limbs_mul_greater_to_out, limbs_mul_same_length_to_out, limbs_mul_to_out,
};
use natural::arithmetic::shl::limbs_shl_to_out;
use natural::arithmetic::shr::{limbs_shr_to_out, limbs_slice_shr_in_place};
use natural::arithmetic::sub::{
    _limbs_sub_same_length_with_borrow_in_in_place_left,
    _limbs_sub_same_length_with_borrow_in_in_place_right, limbs_sub_limb_in_place,
    limbs_sub_same_length_in_place_left, limbs_sub_same_length_in_place_right,
    limbs_sub_same_length_to_out,
};
use natural::arithmetic::sub_mul::limbs_sub_mul_limb_same_length_in_place_left;
use natural::comparison::ord::limbs_cmp_same_length;
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::{
    DoubleLimb, Limb, DC_DIV_QR_THRESHOLD, MOD_1N_TO_MOD_1_1_THRESHOLD,
    MOD_1U_TO_MOD_1_1_THRESHOLD, MOD_1_1P_METHOD, MOD_1_1_TO_MOD_1_2_THRESHOLD,
    MOD_1_2_TO_MOD_1_4_THRESHOLD, MOD_1_NORM_THRESHOLD, MOD_1_UNNORM_THRESHOLD,
    MU_DIV_QR_SKEW_THRESHOLD, MU_DIV_QR_THRESHOLD,
};
use std::cmp::Ordering;
use std::mem::swap;
use std::ops::{Rem, RemAssign};

/// Time: O(1)
///
/// Additional memory: O(1)
///
/// This is udiv_qrnnd_preinv from gmp-impl.h, GMP 6.2.1, but not computing the quotient.
pub fn _mod_by_preinversion(n_high: Limb, n_low: Limb, d: Limb, d_inv: Limb) -> Limb {
    let (q_high, q_low) = (DoubleLimb::from(n_high) * DoubleLimb::from(d_inv))
        .wrapping_add(DoubleLimb::join_halves(n_high.wrapping_add(1), n_low))
        .split_in_half();
    let mut r = n_low.wrapping_sub(q_high.wrapping_mul(d));
    if r > q_low {
        r.wrapping_add_assign(d);
    }
    if r >= d {
        r -= d;
    }
    r
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
/// remainder when the `Natural` is divided by a `Limb`.
///
/// The divisor limb cannot be zero and the input limb slice must have at least two elements.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `ns.len()
/// `
///
/// # Panics
/// Panics if the length of `ns` is less than 2 or if `d` is zero.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::mod_op::limbs_mod_limb;
///
/// assert_eq!(limbs_mod_limb(&[123, 456], 789), 636);
/// assert_eq!(limbs_mod_limb(&[u32::MAX, u32::MAX], 3), 0);
/// ```
#[cfg(feature = "32_bit_limbs")]
#[inline]
pub fn limbs_mod_limb(ns: &[Limb], d: Limb) -> Limb {
    _limbs_mod_limb_alt_2(ns, d)
}
#[cfg(not(feature = "32_bit_limbs"))]
#[inline]
pub fn limbs_mod_limb(ns: &[Limb], d: Limb) -> Limb {
    _limbs_mod_limb_alt_1(ns, d)
}

/// Computes the remainder of `[n_2, n_1, n_0]` / `[d_1, d_0]`. Requires the highest bit of `d_1` to
/// be set, and `[n_2, n_1]` < `[d_1, d_0]`. `d_inv` is the inverse of `[d_1, d_0]` computed by
/// `limbs_two_limb_inverse_helper`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::div_mod::limbs_two_limb_inverse_helper;
/// use malachite_nz::natural::arithmetic::mod_op::limbs_mod_three_limb_by_two_limb;
///
/// let d_1 = 0x80000004;
/// let d_0 = 5;
/// assert_eq!(
///     limbs_mod_three_limb_by_two_limb(
///         1, 2, 3, d_1, d_0,
///         limbs_two_limb_inverse_helper(d_1, d_0)),
///     0x7ffffffdfffffffe
/// );
///
/// let d_1 = 0x80000000;
/// let d_0 = 0;
/// assert_eq!(
///     limbs_mod_three_limb_by_two_limb(
///         2, 0x40000000, 4, d_1, d_0,
///         limbs_two_limb_inverse_helper(d_1, d_0)),
///     0x4000000000000004
/// );
/// ```
///
/// This is udiv_qr_3by2 from gmp-impl.h, GMP 6.2.1, returning only the remainder.
pub fn limbs_mod_three_limb_by_two_limb(
    n_2: Limb,
    n_1: Limb,
    n_0: Limb,
    d_1: Limb,
    d_0: Limb,
    d_inv: Limb,
) -> DoubleLimb {
    let (q, q_lo) = (DoubleLimb::from(n_2) * DoubleLimb::from(d_inv))
        .wrapping_add(DoubleLimb::join_halves(n_2, n_1))
        .split_in_half();
    let d = DoubleLimb::join_halves(d_1, d_0);
    // Compute the two most significant limbs of n - q * d
    let r = DoubleLimb::join_halves(n_1.wrapping_sub(d_1.wrapping_mul(q)), n_0)
        .wrapping_sub(d)
        .wrapping_sub(DoubleLimb::from(d_0) * DoubleLimb::from(q));
    // Conditionally adjust the remainder
    if r.upper_half() >= q_lo {
        let (r_plus_d, overflow) = r.overflowing_add(d);
        if overflow {
            return r_plus_d;
        }
    } else if r >= d {
        return r.wrapping_sub(d);
    }
    r
}

/// Divides `ns` by `ds`, returning the limbs of the remainder. `ds` must have length 2, `ns` must
/// have length at least 2, and the most significant bit of `ds[1]` must be set.
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
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::mod_op::limbs_mod_by_two_limb_normalized;
///
/// assert_eq!(
///     limbs_mod_by_two_limb_normalized(&[1, 2, 3, 4, 5], &[3, 0x80000000]),
///     (166, 2147483626)
/// );
/// ```
///
/// This is mpn_divrem_2 from mpn/generic/divrem_2.c, GMP 6.2.1, returning the two limbs of the
/// remainder.
pub fn limbs_mod_by_two_limb_normalized(ns: &[Limb], ds: &[Limb]) -> (Limb, Limb) {
    assert_eq!(ds.len(), 2);
    let n_len = ns.len();
    assert!(n_len >= 2);
    let n_limit = n_len - 2;
    assert!(ds[1].get_highest_bit());
    let d_1 = ds[1];
    let d_0 = ds[0];
    let d = DoubleLimb::join_halves(d_1, d_0);
    let mut r = DoubleLimb::join_halves(ns[n_limit + 1], ns[n_limit]);
    if r >= d {
        r.wrapping_sub_assign(d);
    }
    let (mut r_1, mut r_0) = r.split_in_half();
    let d_inv = limbs_two_limb_inverse_helper(d_1, d_0);
    for &n in ns[..n_limit].iter().rev() {
        let (new_r_1, new_r_0) =
            limbs_mod_three_limb_by_two_limb(r_1, r_0, n, d_1, d_0, d_inv).split_in_half();
        r_1 = new_r_1;
        r_0 = new_r_0;
    }
    (r_0, r_1)
}

/// Divides `ns` by `ds` and writes the `ds.len()` limbs of the remainder to `ns`. `ds` must have
/// length greater than 2, `ns` must be at least as long as `ds`, and the most significant bit of
/// `ds` must be set. `d_inv` should be the result of `limbs_two_limb_inverse_helper` applied to the
/// two highest limbs of the denominator.
///
/// Time: worst case O(d * (n - d + 1)); also, O(n ^ 2)
///
/// Additional memory: worst case O(1)
///
/// where n = `ns.len()`
///       d = `ds.len()`
///
/// # Panics
/// Panics if `ds` has length smaller than 3, `ns` is shorter than `ds`, or the last limb of `ds`
/// does not have its highest bit set.
///
/// This is mpn_sbpi1_div_qr from mpn/generic/sbpi1_div_qr.c, GMP 6.2.1, where only the remainder is
/// calculated.
pub fn _limbs_mod_schoolbook(ns: &mut [Limb], ds: &[Limb], d_inv: Limb) {
    let d_len = ds.len();
    assert!(d_len > 2);
    let n_len = ns.len();
    assert!(n_len >= d_len);
    let (d_1, ds_init) = ds.split_last().unwrap();
    let d_1 = *d_1;
    assert!(d_1.get_highest_bit());
    let (d_0, ds_init_init) = ds_init.split_last().unwrap();
    let d_0 = *d_0;
    let ns_hi = &mut ns[n_len - d_len..];
    if limbs_cmp_same_length(ns_hi, ds) >= Ordering::Equal {
        limbs_sub_same_length_in_place_left(ns_hi, ds);
    }
    let mut n_1 = ns[n_len - 1];
    for i in (d_len..n_len).rev() {
        let j = i - d_len;
        if n_1 == d_1 && ns[i - 1] == d_0 {
            limbs_sub_mul_limb_same_length_in_place_left(&mut ns[j..i], ds, Limb::MAX);
            n_1 = ns[i - 1]; // update n_1, last loop's value will now be invalid
        } else {
            let (ns_lo, ns_hi) = ns.split_at_mut(i - 2);
            let (q, new_n) =
                limbs_div_mod_three_limb_by_two_limb(n_1, ns_hi[1], ns_hi[0], d_1, d_0, d_inv);
            let (new_n_1, mut n_0) = new_n.split_in_half();
            n_1 = new_n_1;
            let local_carry_1 =
                limbs_sub_mul_limb_same_length_in_place_left(&mut ns_lo[j..], ds_init_init, q);
            let local_carry_2 = n_0 < local_carry_1;
            n_0.wrapping_sub_assign(local_carry_1);
            let carry = local_carry_2 && n_1 == 0;
            if local_carry_2 {
                n_1.wrapping_sub_assign(1);
            }
            ns_hi[0] = n_0;
            if carry {
                n_1.wrapping_add_assign(d_1);
                if limbs_slice_add_same_length_in_place_left(&mut ns[j..i - 1], ds_init) {
                    n_1.wrapping_add_assign(1);
                }
            }
        }
    }
    ns[d_len - 1] = n_1;
}

/// `qs` is just used as scratch space.
///
/// Time: worst case O(n * log(n) ^ 2 * log(log(n)))
///
/// Additional memory: worst case O(n * log(n) ^ 2)
///
/// where n = `ds.len()`
///
/// This is mpn_dcpi1_div_qr_n from mpn/generic/dcpi1_div_qr.c, GMP 6.2.1, where only the remainder
/// is calculated.
fn _limbs_mod_divide_and_conquer_helper(
    qs: &mut [Limb],
    ns: &mut [Limb],
    ds: &[Limb],
    d_inv: Limb,
    scratch: &mut [Limb],
) {
    let n = ds.len();
    let lo = n >> 1; // floor(n / 2)
    let hi = n - lo; // ceil(n / 2)
    let qs_hi = &mut qs[lo..];
    let (ds_lo, ds_hi) = ds.split_at(lo);
    let highest_q = if hi < DC_DIV_QR_THRESHOLD {
        _limbs_div_mod_schoolbook(qs_hi, &mut ns[lo << 1..n << 1], ds_hi, d_inv)
    } else {
        _limbs_div_mod_divide_and_conquer_helper(qs_hi, &mut ns[lo << 1..], ds_hi, d_inv, scratch)
    };
    let qs_hi = &mut qs_hi[..hi];
    limbs_mul_greater_to_out(scratch, qs_hi, ds_lo);
    let ns_lo = &mut ns[..n + lo];
    let mut carry = Limb::iverson(limbs_sub_same_length_in_place_left(
        &mut ns_lo[lo..],
        &scratch[..n],
    ));
    if highest_q && limbs_sub_same_length_in_place_left(&mut ns_lo[n..], ds_lo) {
        carry += 1;
    }
    while carry != 0 {
        limbs_sub_limb_in_place(qs_hi, 1);
        if limbs_slice_add_same_length_in_place_left(&mut ns_lo[lo..], ds) {
            carry -= 1;
        }
    }
    let (ds_lo, ds_hi) = ds.split_at(hi);
    let q_lo = if lo < DC_DIV_QR_THRESHOLD {
        _limbs_div_mod_schoolbook(qs, &mut ns[hi..n + lo], ds_hi, d_inv)
    } else {
        _limbs_div_mod_divide_and_conquer_helper(qs, &mut ns[hi..], ds_hi, d_inv, scratch)
    };
    let qs_lo = &mut qs[..lo];
    let ns_lo = &mut ns[..n];
    limbs_mul_greater_to_out(scratch, ds_lo, qs_lo);
    let mut carry = Limb::iverson(limbs_sub_same_length_in_place_left(ns_lo, &scratch[..n]));
    if q_lo && limbs_sub_same_length_in_place_left(&mut ns_lo[lo..], ds_lo) {
        carry += 1;
    }
    while carry != 0 {
        if limbs_slice_add_same_length_in_place_left(ns_lo, ds) {
            carry -= 1;
        }
    }
}

/// `qs` is just used as scratch space.
///
/// Time: worst case O(n * log(n) ^ 2 * log(log(n)))
///
/// Additional memory: worst case O(n * log(n) ^ 2)
///
/// where n = `ds.len()`
///
/// # Panics
/// Panics if `ds` has length smaller than 6, `ns.len()` is less than `ds.len()` + 3, `qs` has
/// length less than `ns.len()` - `ds.len()`, or the last limb of `ds` does not have its highest bit
/// set.
///
/// This is mpn_dcpi1_div_qr from mpn/generic/dcpi1_div_qr.c, GMP 6.2.1, where only the remainder is
/// calculated.
pub fn _limbs_mod_divide_and_conquer(qs: &mut [Limb], ns: &mut [Limb], ds: &[Limb], d_inv: Limb) {
    let n_len = ns.len();
    let d_len = ds.len();
    assert!(d_len >= 6); // to adhere to _limbs_div_mod_schoolbook's limits
    assert!(n_len >= d_len + 3); // to adhere to _limbs_div_mod_schoolbook's limits
    let a = d_len - 1;
    let d_1 = ds[a];
    let b = d_len - 2;
    assert!(d_1.get_highest_bit());
    let mut scratch = vec![0; d_len];
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
        // point at low limb of next quotient block
        let qs_block = &mut qs[q_len - q_len_mod_d_len..q_len];
        if q_len_mod_d_len == 1 {
            // Handle highest_q up front, for simplicity.
            let ns = &mut ns[q_len - 1..];
            let ns_tail = &mut ns[1..];
            if limbs_cmp_same_length(ns_tail, ds) >= Ordering::Equal {
                assert!(!limbs_sub_same_length_in_place_left(ns_tail, ds));
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
                q = Limb::MAX;
                assert_eq!(limbs_sub_mul_limb_same_length_in_place_left(ns, ds, q), n_2);
            } else {
                let (new_q, new_n) =
                    limbs_div_mod_three_limb_by_two_limb(n_2, n_1, n_0, d_1, d_0, d_inv);
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
                let (ns_last, ns_init) = ns.split_last_mut().unwrap();
                if carry {
                    n_1.wrapping_add_assign(d_1);
                    if limbs_slice_add_same_length_in_place_left(ns_init, &ds[..a]) {
                        n_1.wrapping_add_assign(1);
                    }
                    q.wrapping_sub_assign(1);
                }
                *ns_last = n_1;
            }
            qs_block[0] = q;
        } else {
            // Do a 2 * q_len_mod_d_len / q_len_mod_d_len division
            let (ds_lo, ds_hi) = ds.split_at(d_len - q_len_mod_d_len);
            let highest_q = {
                let ns = &mut ns[n_len - (q_len_mod_d_len << 1)..];
                if q_len_mod_d_len == 2 {
                    limbs_div_mod_by_two_limb_normalized(qs_block, ns, ds_hi)
                } else if q_len_mod_d_len < DC_DIV_QR_THRESHOLD {
                    _limbs_div_mod_schoolbook(qs_block, ns, ds_hi, d_inv)
                } else {
                    _limbs_div_mod_divide_and_conquer_helper(
                        qs_block,
                        ns,
                        ds_hi,
                        d_inv,
                        &mut scratch,
                    )
                }
            };
            if q_len_mod_d_len != d_len {
                limbs_mul_to_out(&mut scratch, qs_block, ds_lo);
                let ns = &mut ns[q_len - q_len_mod_d_len..n_len - q_len_mod_d_len];
                let mut carry = Limb::iverson(limbs_sub_same_length_in_place_left(ns, &scratch));
                if highest_q
                    && limbs_sub_same_length_in_place_left(&mut ns[q_len_mod_d_len..], ds_lo)
                {
                    carry += 1;
                }
                while carry != 0 {
                    limbs_sub_limb_in_place(qs_block, 1);
                    if limbs_slice_add_same_length_in_place_left(ns, ds) {
                        carry -= 1;
                    }
                }
            }
        }
        // offset is a multiple of d_len
        let mut offset = n_len.checked_sub(d_len + q_len_mod_d_len).unwrap();
        while offset != 0 {
            offset -= d_len;
            _limbs_mod_divide_and_conquer_helper(
                &mut qs[offset..],
                &mut ns[offset..],
                ds,
                d_inv,
                &mut scratch,
            );
        }
    } else {
        let m = d_len - q_len;
        let (ds_lo, ds_hi) = ds.split_at(m);
        let highest_q = if q_len < DC_DIV_QR_THRESHOLD {
            _limbs_div_mod_schoolbook(qs, &mut ns[m..], ds_hi, d_inv)
        } else {
            _limbs_div_mod_divide_and_conquer_helper(qs, &mut ns[m..], ds_hi, d_inv, &mut scratch)
        };
        if m != 0 {
            let qs = &mut qs[..q_len];
            let ns = &mut ns[..d_len];
            limbs_mul_to_out(&mut scratch, qs, ds_lo);
            let mut carry = Limb::iverson(limbs_sub_same_length_in_place_left(ns, &scratch));
            if highest_q && limbs_sub_same_length_in_place_left(&mut ns[q_len..], ds_lo) {
                carry += 1;
            }
            while carry != 0 {
                if limbs_slice_add_same_length_in_place_left(ns, ds) {
                    carry -= 1;
                }
            }
        }
    }
}

/// `qs` is just used as scratch space.
///
/// Time: Worst case O(n * log(d) * log(log(d)))
///
/// Additional memory: Worst case O(d * log(d))
///
/// where n = `ns.len()`, d = `ds.len()`
///
/// This is mpn_preinv_mu_div_qr from mpn/generic/mu_div_qr.c, GMP 6.2.1, where only the remainder
/// is calculated.
fn _limbs_mod_barrett_preinverted(
    qs: &mut [Limb],
    rs: &mut [Limb],
    ns: &[Limb],
    ds: &[Limb],
    mut is: &[Limb],
    scratch: &mut [Limb],
) {
    let n_len = ns.len();
    let d_len = ds.len();
    assert_eq!(rs.len(), d_len);
    let mut i_len = is.len();
    let q_len = n_len - d_len;
    let qs = &mut qs[..q_len];
    let (ns_lo, ns_hi) = ns.split_at(q_len);
    if limbs_cmp_same_length(ns_hi, ds) >= Ordering::Equal {
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
        limbs_mul_same_length_to_out(scratch, rs_hi, is);
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
        // Check the remainder.
        if carry {
            r.wrapping_sub_assign(1);
        }
        while r != 0 {
            // We loop 0 times with about 69% probability, 1 time with about 31% probability, and 2
            // times with about 0.6% probability, if the inverse is computed as recommended.
            if limbs_sub_same_length_in_place_left(rs, ds) {
                r -= 1;
            }
        }
        if limbs_cmp_same_length(rs, ds) >= Ordering::Equal {
            // This is executed with about 76% probability.
            limbs_sub_same_length_in_place_left(rs, ds);
        }
    }
}

/// `qs` is just used as scratch space.
///
/// Time: Worst case O(n * log(n) * log(log(n)))
///
/// Additional memory: Worst case O(n * log(n))
///
/// where n = `ns.len()`
///
/// This is mpn_mu_div_qr2 from mpn/generic/mu_div_qr.c, GMP 6.2.1, where only the remainder is
/// calculated.
pub fn _limbs_mod_barrett_helper(
    qs: &mut [Limb],
    rs: &mut [Limb],
    ns: &[Limb],
    ds: &[Limb],
    scratch: &mut [Limb],
) {
    let n_len = ns.len();
    let d_len = ds.len();
    assert_eq!(rs.len(), d_len);
    assert!(d_len > 1);
    assert!(n_len > d_len);
    let q_len = n_len - d_len;
    // Compute the inverse size.
    let i_len = _limbs_div_mod_barrett_is_len(q_len, d_len);
    assert!(i_len <= d_len);
    let i_len_plus_1 = i_len + 1;
    let (is, scratch_hi) = scratch.split_at_mut(i_len_plus_1);
    // compute an approximate inverse on i_len + 1 limbs
    if d_len == i_len {
        let (scratch_lo, scratch_hi) = scratch_hi.split_at_mut(i_len_plus_1);
        let (scratch_first, scratch_lo_tail) = scratch_lo.split_first_mut().unwrap();
        scratch_lo_tail.copy_from_slice(&ds[..i_len]);
        *scratch_first = 1;
        _limbs_invert_approx(is, scratch_lo, scratch_hi);
        slice_move_left(is, 1);
    } else if limbs_add_limb_to_out(scratch_hi, &ds[d_len - i_len_plus_1..], 1) {
        slice_set_zero(&mut is[..i_len]);
    } else {
        let (scratch_lo, scratch_hi) = scratch_hi.split_at_mut(i_len_plus_1);
        _limbs_invert_approx(is, scratch_lo, scratch_hi);
        slice_move_left(is, 1);
    }
    let (scratch_lo, scratch_hi) = scratch.split_at_mut(i_len);
    _limbs_mod_barrett_preinverted(qs, rs, ns, ds, scratch_lo, scratch_hi);
}

/// `qs` is just used as scratch space.
fn _limbs_mod_barrett_large_helper(
    qs: &mut [Limb],
    rs: &mut [Limb],
    ns: &[Limb],
    ds: &[Limb],
    scratch: &mut [Limb],
) {
    let n_len = ns.len();
    let d_len = ds.len();
    let q_len = qs.len();
    let q_len_plus_one = q_len + 1;
    let n = n_len - q_len - q_len_plus_one; // 2 * d_len - n_len - 1
    let (ns_lo, ns_hi) = ns.split_at(n);
    let (ds_lo, ds_hi) = ds.split_at(d_len - q_len_plus_one);
    let (rs_lo, rs_hi) = rs.split_at_mut(n);
    let rs_hi = &mut rs_hi[..q_len_plus_one];
    let highest_q = _limbs_div_mod_barrett_helper(qs, rs_hi, ns_hi, ds_hi, scratch);
    // Multiply the quotient by the divisor limbs ignored above.
    // The product is d_len - 1 limbs long.
    limbs_mul_to_out(scratch, ds_lo, qs);
    let (scratch_last, scratch_init) = scratch[..d_len].split_last_mut().unwrap();
    *scratch_last = Limb::iverson(
        highest_q && limbs_slice_add_same_length_in_place_left(&mut scratch_init[q_len..], ds_lo),
    );
    let (scratch_lo, scratch_hi) = scratch.split_at(n);
    let scratch_hi = &scratch_hi[..q_len_plus_one];
    if _limbs_sub_same_length_with_borrow_in_in_place_left(
        rs_hi,
        scratch_hi,
        limbs_sub_same_length_to_out(rs_lo, ns_lo, scratch_lo),
    ) {
        limbs_slice_add_same_length_in_place_left(&mut rs[..d_len], ds);
    }
}

/// `qs` is just used as scratch space.
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
/// less than `ns.len()` - `ds.len()`, or the last limb of `ds` does not have its highest bit set.
///
/// This is mpn_mu_div_qr from mpn/generic/mu_div_qr.c, GMP 6.2.1.
pub fn _limbs_mod_barrett(
    qs: &mut [Limb],
    rs: &mut [Limb],
    ns: &[Limb],
    ds: &[Limb],
    scratch: &mut [Limb],
) {
    let n_len = ns.len();
    let d_len = ds.len();
    let q_len = n_len - d_len;
    let qs = &mut qs[..q_len];
    // Test whether 2 * d_len - n_len > MU_DIV_QR_SKEW_THRESHOLD
    if d_len <= q_len + MU_DIV_QR_SKEW_THRESHOLD {
        _limbs_mod_barrett_helper(qs, &mut rs[..d_len], ns, ds, scratch)
    } else {
        _limbs_mod_barrett_large_helper(qs, rs, ns, ds, scratch)
    }
}

/// `ds` must have length 2, `ns` must have length at least 2, and the most-significant limb of `ds`
/// must be nonzero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `ns.len()`
fn _limbs_mod_by_two_limb(ns: &[Limb], ds: &[Limb]) -> (Limb, Limb) {
    let n_len = ns.len();
    let ds_1 = ds[1];
    let bits = LeadingZeros::leading_zeros(ds_1);
    if bits == 0 {
        limbs_mod_by_two_limb_normalized(ns, ds)
    } else {
        let ds_0 = ds[0];
        let cobits = Limb::WIDTH - bits;
        let mut ns_shifted = vec![0; n_len + 1];
        let ns_shifted = &mut ns_shifted;
        let carry = limbs_shl_to_out(ns_shifted, ns, bits);
        let ds_shifted = &mut [ds_0 << bits, (ds_1 << bits) | (ds_0 >> cobits)];
        let (r_0, r_1) = if carry == 0 {
            limbs_mod_by_two_limb_normalized(&ns_shifted[..n_len], ds_shifted)
        } else {
            ns_shifted[n_len] = carry;
            limbs_mod_by_two_limb_normalized(ns_shifted, ds_shifted)
        };
        ((r_0 >> bits) | (r_1 << cobits), r_1 >> bits)
    }
}

fn divide_and_conquer_condition(n_len: usize, d_len: usize) -> bool {
    let n_64 = n_len as f64;
    let d_64 = d_len as f64;
    d_len < MUPI_DIV_QR_THRESHOLD
        || n_len < MU_DIV_QR_THRESHOLD << 1
        || (((MU_DIV_QR_THRESHOLD - MUPI_DIV_QR_THRESHOLD) << 1) as f64)
            .mul_add(d_64, MUPI_DIV_QR_THRESHOLD as f64 * n_64)
            > d_64 * n_64
}

/// This function is optimized for the case when the numerator has at least twice the length of the
/// denominator.
///
/// `ds` must have length at least 3, `ns` must be at least as long as `ds`, `rs` must have the same
/// length as `ds`, and the most-significant limb of `ds` must be nonzero.
///
/// Time: Worst case O(n * log(n) * log(log(n)))
///
/// Additional memory: Worst case O(n * log(n))
///
/// where n = `ns.len()`
fn _limbs_mod_unbalanced(rs: &mut [Limb], ns: &[Limb], ds: &[Limb], adjusted_n_len: usize) {
    let mut n_len = ns.len();
    let d_len = ds.len();
    let mut ds_shifted_vec;
    let ds_shifted: &[Limb];
    let mut ns_shifted_vec = vec![0; n_len + 1];
    let ns_shifted = &mut ns_shifted_vec;
    let bits = LeadingZeros::leading_zeros(*ds.last().unwrap());
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
    let d_inv = limbs_two_limb_inverse_helper(ds_shifted[d_len - 1], ds_shifted[d_len - 2]);
    let ns_shifted = &mut ns_shifted[..n_len];
    if d_len < DC_DIV_QR_THRESHOLD {
        _limbs_mod_schoolbook(ns_shifted, ds_shifted, d_inv);
        let ns_shifted = &ns_shifted[..d_len];
        if bits == 0 {
            rs.copy_from_slice(ns_shifted);
        } else {
            limbs_shr_to_out(rs, ns_shifted, bits);
        }
    } else if divide_and_conquer_condition(n_len, d_len) {
        let mut qs = vec![0; n_len - d_len];
        _limbs_mod_divide_and_conquer(&mut qs, ns_shifted, ds_shifted, d_inv);
        let ns_shifted = &ns_shifted[..d_len];
        if bits == 0 {
            rs.copy_from_slice(ns_shifted);
        } else {
            limbs_shr_to_out(rs, ns_shifted, bits);
        }
    } else {
        let scratch_len = _limbs_div_mod_barrett_scratch_len(n_len, d_len);
        let mut qs = vec![0; n_len - d_len];
        let mut scratch = vec![0; scratch_len];
        _limbs_mod_barrett(&mut qs, rs, ns_shifted, ds_shifted, &mut scratch);
        if bits != 0 {
            limbs_slice_shr_in_place(rs, bits);
        }
    }
}

/// Interpreting two slices of `Limb`s, `ns` and `ds`, as the limbs (in ascending order) of two
/// `Natural`s, divides them, returning the remainder. The remainder has `ds.len()` limbs.
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
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::mod_op::limbs_mod;
///
/// assert_eq!(limbs_mod(&[1, 2], &[3, 4]), &[1, 2]);
/// assert_eq!(limbs_mod(&[1, 2, 3], &[4, 5]), &[2576980381, 2]);
/// ```
///
/// This is mpn_tdiv_qr from mpn/generic/tdiv_qr.c, GMP 6.2.1, where qp is not calculated and rp is
/// returned.
pub fn limbs_mod(ns: &[Limb], ds: &[Limb]) -> Vec<Limb> {
    let mut rs = vec![0; ds.len()];
    limbs_mod_to_out(&mut rs, ns, ds);
    rs
}

/// Interpreting two slices of `Limb`s, `ns` and `ds`, as the limbs (in ascending order) of two
/// `Natural`s, divides them, writing the `ds.len()` limbs of the remainder to `rs`.
///
/// `ns` must be at least as long as `ds`, `rs` must be at least as long as `ds`, and `ds` must have
/// length at least 2 and its most significant limb must be greater than zero.
///
/// Time: Worst case O(n * log(n) * log(log(n)))
///
/// Additional memory: Worst case O(n * log(n))
///
/// where n = `ns.len()`
///
/// # Panics
/// Panics if `rs` is too short, `ns` is shorter than `ds`, `ds` has length less than 2, or the
/// most-significant limb of `ds` is zero.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::mod_op::limbs_mod_to_out;
///
/// let rs = &mut [10; 4];
/// limbs_mod_to_out(rs, &[1, 2], &[3, 4]);
/// assert_eq!(rs, &[1, 2, 10, 10]);
///
/// let rs = &mut [10; 4];
/// limbs_mod_to_out(rs, &[1, 2, 3], &[4, 5]);
/// assert_eq!(rs, &[2576980381, 2, 10, 10]);
/// ```
///
/// This is mpn_tdiv_qr from mpn/generic/tdiv_qr.c, GMP 6.2.1, where qp is not calculated.
pub fn limbs_mod_to_out(rs: &mut [Limb], ns: &[Limb], ds: &[Limb]) {
    let n_len = ns.len();
    let d_len = ds.len();
    assert!(n_len >= d_len);
    let rs = &mut rs[..d_len];
    let ds_last = *ds.last().unwrap();
    assert!(d_len > 1 && ds_last != 0);
    if d_len == 2 {
        let (r_0, r_1) = _limbs_mod_by_two_limb(ns, ds);
        rs[0] = r_0;
        rs[1] = r_1;
    } else {
        // conservative tests for quotient size
        let adjust = ns[n_len - 1] >= ds_last;
        let adjusted_n_len = if adjust { n_len + 1 } else { n_len };
        if adjusted_n_len < d_len << 1 {
            let mut qs = vec![0; n_len - d_len + 1];
            _limbs_div_mod_balanced(&mut qs, rs, ns, ds, adjust);
        } else {
            _limbs_mod_unbalanced(rs, ns, ds, adjusted_n_len);
        }
    }
}

fn _limbs_rem_naive(ns: &[Limb], d: Limb) -> Limb {
    let d = DoubleLimb::from(d);
    let mut r = 0;
    for &n in ns.iter().rev() {
        r = (DoubleLimb::join_halves(r, n) % d).lower_half();
    }
    r
}

/// The high bit of `d` must be set.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `ns.len()`
///
/// This is mpn_div_qr_1n_pi1 from mpn/generic/div_qr_1n_pi1.c, GMP 6.2.1, with
/// DIV_QR_1N_METHOD == 2, but not computing the quotient.
fn limbs_mod_limb_normalized(ns: &[Limb], ns_high: Limb, d: Limb, d_inv: Limb) -> Limb {
    let len = ns.len();
    if len == 1 {
        return _mod_by_preinversion(ns_high, ns[0], d, d_inv);
    }
    let power_of_2 = d.wrapping_neg().wrapping_mul(d_inv);
    let (sum, mut big_carry) = DoubleLimb::join_halves(ns[len - 1], ns[len - 2])
        .overflowing_add(DoubleLimb::from(power_of_2) * DoubleLimb::from(ns_high));
    let (mut sum_high, mut sum_low) = sum.split_in_half();
    for &n in ns[..len - 2].iter().rev() {
        if big_carry {
            let (sum, carry) = sum_low.overflowing_add(power_of_2);
            sum_low = sum;
            if carry {
                sum_low.wrapping_sub_assign(d);
            }
        }
        let (sum, carry) = DoubleLimb::join_halves(sum_low, n)
            .overflowing_add(DoubleLimb::from(sum_high) * DoubleLimb::from(power_of_2));
        sum_high = sum.upper_half();
        sum_low = sum.lower_half();
        big_carry = carry;
    }
    if big_carry {
        sum_high.wrapping_sub_assign(d);
    }
    if sum_high >= d {
        sum_high.wrapping_sub_assign(d);
    }
    _mod_by_preinversion(sum_high, sum_low, d, d_inv)
}

/// The high bit of `d` must be set.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `ns.len()`
///
/// This is mpn_div_qr_1n_pi1 from mpn/generic/div_qr_1n_pi1.c, GMP 6.2.1, with
/// DIV_QR_1N_METHOD == 2, but not computing the quotient, and where the input is left-shifted by
/// `bits`.
fn limbs_mod_limb_normalized_shl(
    ns: &[Limb],
    ns_high: Limb,
    d: Limb,
    d_inv: Limb,
    bits: u64,
) -> Limb {
    let len = ns.len();
    if len == 1 {
        return _mod_by_preinversion(ns_high, ns[0] << bits, d, d_inv);
    }
    let power_of_2 = d.wrapping_neg().wrapping_mul(d_inv);
    let cobits = Limb::WIDTH - bits;
    let second_highest = ns[len - 2];
    let highest_after_shl = (ns[len - 1] << bits) | (second_highest >> cobits);
    let mut second_highest_after_shl = second_highest << bits;
    if len > 2 {
        second_highest_after_shl |= ns[len - 3] >> cobits;
    }
    let (sum, mut big_carry) = DoubleLimb::join_halves(highest_after_shl, second_highest_after_shl)
        .overflowing_add(DoubleLimb::from(power_of_2) * DoubleLimb::from(ns_high));
    let (mut sum_high, mut sum_low) = sum.split_in_half();
    for j in (0..len - 2).rev() {
        if big_carry {
            let (sum, carry) = sum_low.overflowing_add(power_of_2);
            sum_low = sum;
            if carry {
                sum_low.wrapping_sub_assign(d);
            }
        }
        let mut n = ns[j] << bits;
        if j != 0 {
            n |= ns[j - 1] >> cobits;
        }
        let (sum, carry) = DoubleLimb::join_halves(sum_low, n)
            .overflowing_add(DoubleLimb::from(sum_high) * DoubleLimb::from(power_of_2));
        sum_high = sum.upper_half();
        sum_low = sum.lower_half();
        big_carry = carry;
    }
    if big_carry {
        sum_high.wrapping_sub_assign(d);
    }
    if sum_high >= d {
        sum_high.wrapping_sub_assign(d);
    }
    _mod_by_preinversion(sum_high, sum_low, d, d_inv)
}

/// This is mpn_div_qr_1 from mpn/generic/div_qr_1.c, GMP 6.2.1, where the quotient is not computed
/// and the remainder is returned. Experiments show that this is always slower than
/// `_limbs_mod_limb`.
pub fn _limbs_mod_limb_alt_1(ns: &[Limb], d: Limb) -> Limb {
    assert_ne!(d, 0);
    let len = ns.len();
    assert!(len > 1);
    let len_minus_1 = len - 1;
    let mut ns_high = ns[len_minus_1];
    let bits = LeadingZeros::leading_zeros(d);
    if bits == 0 {
        if ns_high >= d {
            ns_high -= d;
        }
        let d_inv = limbs_invert_limb(d);
        limbs_mod_limb_normalized(&ns[..len_minus_1], ns_high, d, d_inv)
    } else {
        let d = d << bits;
        let cobits = Limb::WIDTH - bits;
        let d_inv = limbs_invert_limb(d);
        let r = _mod_by_preinversion(
            ns_high >> cobits,
            (ns_high << bits) | (ns[len - 2] >> cobits),
            d,
            d_inv,
        );
        limbs_mod_limb_normalized_shl(&ns[..len_minus_1], r, d, d_inv, bits) >> bits
    }
}

/// Dividing (`n_high`, `n_low`) by `d`, returning the remainder only. Unlike `mod_by_preinversion`,
/// works also for the case `n_high` == `d`, where the quotient doesn't quite fit in a single limb.
///
/// Time: O(1)
///
/// Additional memory: O(1)
///
/// This is udiv_rnnd_preinv from gmp-impl.h, GMP 6.2.1.
fn mod_by_preinversion_special(n_high: Limb, n_low: Limb, d: Limb, d_inv: Limb) -> Limb {
    let (q_high, q_low) = ((DoubleLimb::from(n_high) * DoubleLimb::from(d_inv))
        .wrapping_add(DoubleLimb::join_halves(n_high.wrapping_add(1), n_low)))
    .split_in_half();
    let mut r = n_low.wrapping_sub(q_high.wrapping_mul(d));
    // both > and >= are OK
    if r > q_low {
        r.wrapping_add_assign(d);
    }
    if r >= d {
        r.wrapping_sub_assign(d);
    }
    r
}

pub fn _limbs_mod_limb_small_small(ns: &[Limb], d: Limb, mut r: Limb) -> Limb {
    let d = DoubleLimb::from(d);
    for &n in ns.iter().rev() {
        r = (DoubleLimb::join_halves(r, n) % d).lower_half();
    }
    r
}

pub fn _limbs_mod_limb_small_normalized_large(ns: &[Limb], d: Limb, mut r: Limb) -> Limb {
    let d_inv = limbs_invert_limb(d);
    for &n in ns.iter().rev() {
        r = mod_by_preinversion_special(r, n, d, d_inv);
    }
    r
}

/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `ns.len()`
///
/// This is mpn_mod_1_norm from mpn/generic/mod_1.c, GMP 6.2.1.
#[allow(clippy::absurd_extreme_comparisons)]
pub fn _limbs_mod_limb_small_normalized(ns: &[Limb], d: Limb) -> Limb {
    let mut len = ns.len();
    assert_ne!(len, 0);
    assert!(d.get_highest_bit());
    // High limb is initial remainder, possibly with one subtraction of d to get r < d.
    let mut r = ns[len - 1];
    if r >= d {
        r -= d;
    }
    len -= 1;
    if len == 0 {
        r
    } else {
        let ns = &ns[..len];
        if len < MOD_1_NORM_THRESHOLD {
            _limbs_mod_limb_small_small(ns, d, r)
        } else {
            _limbs_mod_limb_small_normalized_large(ns, d, r)
        }
    }
}

pub fn _limbs_mod_limb_small_unnormalized_large(ns: &[Limb], mut d: Limb, mut r: Limb) -> Limb {
    let shift = LeadingZeros::leading_zeros(d);
    d <<= shift;
    let (ns_last, ns_init) = ns.split_last().unwrap();
    let mut previous_n = *ns_last;
    let co_shift = Limb::WIDTH - shift;
    r = (r << shift) | (previous_n >> co_shift);
    let d_inv = limbs_invert_limb(d);
    for &n in ns_init.iter().rev() {
        let shifted_n = (previous_n << shift) | (n >> co_shift);
        r = mod_by_preinversion_special(r, shifted_n, d, d_inv);
        previous_n = n;
    }
    mod_by_preinversion_special(r, previous_n << shift, d, d_inv) >> shift
}

/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `ns.len()`
///
/// This is mpn_mod_1_unnorm from mpn/generic/mod_1.c, GMP 6.2.1, where UDIV_NEEDS_NORMALIZATION is
/// false.
#[allow(clippy::absurd_extreme_comparisons)]
pub fn _limbs_mod_limb_small_unnormalized(ns: &[Limb], d: Limb) -> Limb {
    let mut len = ns.len();
    assert_ne!(len, 0);
    assert_ne!(d, 0);
    assert!(!d.get_highest_bit());
    // Skip a division if high < divisor. Having the test here before normalizing will still skip as
    // often as possible.
    let mut r = ns[len - 1];
    if r < d {
        len -= 1;
        if len == 0 {
            return r;
        }
    } else {
        r = 0;
    }
    let ns = &ns[..len];
    if len < MOD_1_UNNORM_THRESHOLD {
        _limbs_mod_limb_small_small(ns, d, r)
    } else {
        _limbs_mod_limb_small_unnormalized_large(ns, d, r)
    }
}

#[inline]
pub fn _limbs_mod_limb_any_leading_zeros(ns: &[Limb], d: Limb) -> Limb {
    if MOD_1_1P_METHOD {
        _limbs_mod_limb_any_leading_zeros_1(ns, d)
    } else {
        _limbs_mod_limb_any_leading_zeros_2(ns, d)
    }
}

/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `ns.len()`
///
/// This is mpn_mod_1_1p_cps_1 combined with mpn_mod_1_1p_1 from mpn/generic/mod_1.c, GMP 6.2.1.
pub fn _limbs_mod_limb_any_leading_zeros_1(ns: &[Limb], d: Limb) -> Limb {
    let len = ns.len();
    assert!(len >= 2);
    let shift = u64::from(d.leading_zeros());
    let d = d << shift;
    let d_inv = limbs_invert_limb(d);
    let mut base_mod_d = d.wrapping_neg();
    if shift != 0 {
        base_mod_d.wrapping_mul_assign((d_inv >> (Limb::WIDTH - shift)) | Limb::power_of_2(shift));
    }
    assert!(base_mod_d <= d); // not fully reduced mod divisor
    let base_pow_2_mod_d =
        DoubleLimb::from(mod_by_preinversion_special(base_mod_d, 0, d, d_inv) >> shift);
    let base_mod_d = DoubleLimb::from(base_mod_d >> shift);
    let (mut r_hi, mut r_lo) = (DoubleLimb::from(ns[len - 1]) * base_mod_d)
        .wrapping_add(DoubleLimb::from(ns[len - 2]))
        .split_in_half();
    for &n in ns[..len - 2].iter().rev() {
        let (new_r_hi, new_r_lo) = (DoubleLimb::from(r_hi) * base_pow_2_mod_d)
            .wrapping_add(DoubleLimb::from(r_lo) * base_mod_d)
            .wrapping_add(DoubleLimb::from(n))
            .split_in_half();
        r_hi = new_r_hi;
        r_lo = new_r_lo;
    }
    if shift != 0 {
        r_hi = (r_hi << shift) | (r_lo >> (Limb::WIDTH - shift));
    }
    if r_hi >= d {
        r_hi.wrapping_sub_assign(d);
    }
    mod_by_preinversion_special(r_hi, r_lo << shift, d, d_inv) >> shift
}

/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `ns.len()`
///
/// This is mpn_mod_1_1p_cps_2 combined with mpn_mod_1_1p_2 from mpn/generic/mod_1.c, GMP 6.2.1.
pub fn _limbs_mod_limb_any_leading_zeros_2(ns: &[Limb], d: Limb) -> Limb {
    let len = ns.len();
    assert!(len >= 2);
    let shift = LeadingZeros::leading_zeros(d);
    let d = d << shift;
    let d_inv = limbs_invert_limb(d);
    let base_mod_d = if shift == 0 {
        0
    } else {
        let base_mod_d = d
            .wrapping_neg()
            .wrapping_mul((d_inv >> (Limb::WIDTH - shift)) | Limb::power_of_2(shift));
        assert!(base_mod_d <= d); // not fully reduced mod divisor
        DoubleLimb::from(base_mod_d >> shift)
    };
    let small_base_pow_2_mod_d = d.wrapping_neg().wrapping_mul(d_inv);
    // equality iff divisor = 2 ^ (Limb::WIDTH - 1)
    assert!(small_base_pow_2_mod_d <= d);
    let base_pow_2_mod_d = DoubleLimb::from(small_base_pow_2_mod_d);
    let mut r_lo = ns[len - 2];
    let mut r_hi = ns[len - 1];
    if len > 2 {
        let (r, mut carry) = DoubleLimb::join_halves(r_lo, ns[len - 3])
            .overflowing_add(DoubleLimb::from(r_hi) * base_pow_2_mod_d);
        let (new_r_hi, new_r_lo) = r.split_in_half();
        r_hi = new_r_hi;
        r_lo = new_r_lo;
        for &n in ns[..len - 3].iter().rev() {
            if carry {
                let (new_r_lo, carry) = r_lo.overflowing_add(small_base_pow_2_mod_d);
                r_lo = new_r_lo;
                if carry {
                    r_lo.wrapping_sub_assign(d);
                }
            }
            let (r, new_carry) = DoubleLimb::join_halves(r_lo, n)
                .overflowing_add(DoubleLimb::from(r_hi) * base_pow_2_mod_d);
            carry = new_carry;
            let (new_r_hi, new_r_lo) = r.split_in_half();
            r_hi = new_r_hi;
            r_lo = new_r_lo;
        }
        if carry {
            r_hi.wrapping_sub_assign(d);
        }
    }
    if shift != 0 {
        let (new_r_hi, t) = (DoubleLimb::from(r_hi) * base_mod_d).split_in_half();
        let (new_r_hi, new_r_lo) =
            (DoubleLimb::join_halves(new_r_hi, r_lo).wrapping_add(DoubleLimb::from(t)) << shift)
                .split_in_half();
        r_hi = new_r_hi;
        r_lo = new_r_lo;
    } else if r_hi >= d {
        // might get r_hi == divisor here, but `mod_by_preinversion_special` allows that.
        r_hi.wrapping_sub_assign(d);
    }
    mod_by_preinversion_special(r_hi, r_lo, d, d_inv) >> shift
}

/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `ns.len()`
///
/// This is mpn_mod_1s_2p_cps combined with mpn_mod_1s_2p from mpn/generic/mod_1_2.c, GMP 6.2.1.
pub fn _limbs_mod_limb_at_least_1_leading_zero(ns: &[Limb], d: Limb) -> Limb {
    let mut len = ns.len();
    assert_ne!(len, 0);
    let shift = LeadingZeros::leading_zeros(d);
    assert_ne!(shift, 0);
    let co_shift = Limb::WIDTH - shift;
    let d = d << shift;
    let d_inv = limbs_invert_limb(d);
    let base_mod_d = d
        .wrapping_neg()
        .wrapping_mul((d_inv >> co_shift) | Limb::power_of_2(shift));
    assert!(base_mod_d <= d); // not fully reduced mod divisor
    let base_pow_2_mod_d = mod_by_preinversion_special(base_mod_d, 0, d, d_inv);
    let base_mod_d = DoubleLimb::from(base_mod_d >> shift);
    let base_pow_3_mod_d =
        DoubleLimb::from(mod_by_preinversion_special(base_pow_2_mod_d, 0, d, d_inv) >> shift);
    let base_pow_2_mod_d = DoubleLimb::from(base_pow_2_mod_d >> shift);
    let (mut r_hi, mut r_lo) = if len.odd() {
        len -= 1;
        if len == 0 {
            let rl = ns[len];
            return mod_by_preinversion_special(rl >> co_shift, rl << shift, d, d_inv) >> shift;
        }
        (DoubleLimb::from(ns[len]) * base_pow_2_mod_d)
            .wrapping_add(DoubleLimb::from(ns[len - 1]) * base_mod_d)
            .wrapping_add(DoubleLimb::from(ns[len - 2]))
            .split_in_half()
    } else {
        (ns[len - 1], ns[len - 2])
    };
    for chunk in ns[..len - 2].rchunks_exact(2) {
        let (new_r_hi, new_r_lo) = (DoubleLimb::from(r_hi) * base_pow_3_mod_d)
            .wrapping_add(DoubleLimb::from(r_lo) * base_pow_2_mod_d)
            .wrapping_add(DoubleLimb::from(chunk[1]) * base_mod_d)
            .wrapping_add(DoubleLimb::from(chunk[0]))
            .split_in_half();
        r_hi = new_r_hi;
        r_lo = new_r_lo;
    }
    let (r_hi, r_lo) = (DoubleLimb::from(r_hi) * base_mod_d)
        .wrapping_add(DoubleLimb::from(r_lo))
        .split_in_half();
    mod_by_preinversion_special(
        (r_hi << shift) | (r_lo >> co_shift),
        r_lo << shift,
        d,
        d_inv,
    ) >> shift
}

/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `ns.len()`
///
/// This is mpn_mod_1s_4p_cps combined with mpn_mod_1s_4p from mpn/generic/mod_1_4.c, GMP 6.2.1.
pub fn _limbs_mod_limb_at_least_2_leading_zeros(ns: &[Limb], d: Limb) -> Limb {
    let mut len = ns.len();
    assert_ne!(len, 0);
    let shift = LeadingZeros::leading_zeros(d);
    assert!(shift >= 2);
    let co_shift = Limb::WIDTH - shift;
    let d = d << shift;
    let d_inv = limbs_invert_limb(d);
    let base_mod_d = d
        .wrapping_neg()
        .wrapping_mul((d_inv >> co_shift) | Limb::power_of_2(shift));
    assert!(base_mod_d <= d); // not fully reduced mod divisor
    let base_pow_2_mod_d = mod_by_preinversion_special(base_mod_d, 0, d, d_inv);
    let base_mod_d = DoubleLimb::from(base_mod_d >> shift);
    let base_pow_3_mod_d = mod_by_preinversion_special(base_pow_2_mod_d, 0, d, d_inv);
    let base_pow_2_mod_d = DoubleLimb::from(base_pow_2_mod_d >> shift);
    let base_pow_4_mod_d = mod_by_preinversion_special(base_pow_3_mod_d, 0, d, d_inv);
    let base_pow_3_mod_d = DoubleLimb::from(base_pow_3_mod_d >> shift);
    let base_pow_5_mod_d =
        DoubleLimb::from(mod_by_preinversion_special(base_pow_4_mod_d, 0, d, d_inv) >> shift);
    let base_pow_4_mod_d = DoubleLimb::from(base_pow_4_mod_d >> shift);
    let (mut r_hi, mut r_lo) = match len.mod_power_of_2(2) {
        0 => {
            len -= 4;
            (DoubleLimb::from(ns[len + 3]) * base_pow_3_mod_d)
                .wrapping_add(DoubleLimb::from(ns[len + 2]) * base_pow_2_mod_d)
                .wrapping_add(DoubleLimb::from(ns[len + 1]) * base_mod_d)
                .wrapping_add(DoubleLimb::from(ns[len]))
                .split_in_half()
        }
        1 => {
            len -= 1;
            (0, ns[len])
        }
        2 => {
            len -= 2;
            (ns[len + 1], ns[len])
        }
        3 => {
            len -= 3;
            (DoubleLimb::from(ns[len + 2]) * base_pow_2_mod_d)
                .wrapping_add(DoubleLimb::from(ns[len + 1]) * base_mod_d)
                .wrapping_add(DoubleLimb::from(ns[len]))
                .split_in_half()
        }
        _ => unreachable!(),
    };
    for chunk in ns[..len].rchunks_exact(4) {
        let (new_r_hi, new_r_lo) = (DoubleLimb::from(r_hi) * base_pow_5_mod_d)
            .wrapping_add(DoubleLimb::from(r_lo) * base_pow_4_mod_d)
            .wrapping_add(DoubleLimb::from(chunk[3]) * base_pow_3_mod_d)
            .wrapping_add(DoubleLimb::from(chunk[2]) * base_pow_2_mod_d)
            .wrapping_add(DoubleLimb::from(chunk[1]) * base_mod_d)
            .wrapping_add(DoubleLimb::from(chunk[0]))
            .split_in_half();
        r_hi = new_r_hi;
        r_lo = new_r_lo;
    }
    let (r_hi, r_lo) = (DoubleLimb::from(r_hi) * base_mod_d)
        .wrapping_add(DoubleLimb::from(r_lo))
        .split_in_half();
    mod_by_preinversion_special(
        (r_hi << shift) | (r_lo >> co_shift),
        r_lo << shift,
        d,
        d_inv,
    ) >> shift
}

const HIGHEST_TWO_BITS_MASK: Limb = !(Limb::MAX >> 2);

/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `ns.len()`
///
/// This is mpn_mod_1 from mpn/generic/mod_1.c, GMP 6.2.1, where n > 1.
#[allow(clippy::absurd_extreme_comparisons)]
pub fn _limbs_mod_limb_alt_2(ns: &[Limb], d: Limb) -> Limb {
    let len = ns.len();
    assert!(len > 1);
    assert_ne!(d, 0);
    if d.get_highest_bit() {
        if len < MOD_1N_TO_MOD_1_1_THRESHOLD {
            _limbs_mod_limb_small_normalized(ns, d)
        } else {
            _limbs_mod_limb_any_leading_zeros(ns, d)
        }
    } else if len < MOD_1U_TO_MOD_1_1_THRESHOLD {
        _limbs_mod_limb_small_unnormalized(ns, d)
    } else if len < MOD_1_1_TO_MOD_1_2_THRESHOLD {
        _limbs_mod_limb_any_leading_zeros(ns, d)
    } else if len < MOD_1_2_TO_MOD_1_4_THRESHOLD || d & HIGHEST_TWO_BITS_MASK != 0 {
        _limbs_mod_limb_at_least_1_leading_zero(ns, d)
    } else {
        _limbs_mod_limb_at_least_2_leading_zeros(ns, d)
    }
}

impl Natural {
    pub fn _mod_limb_naive(&self, other: Limb) -> Limb {
        match (self, other) {
            (_, 0) => panic!("division by zero"),
            (Natural(Small(small)), other) => small % other,
            (Natural(Large(ref limbs)), other) => _limbs_rem_naive(limbs, other),
        }
    }

    fn rem_limb_ref(&self, other: Limb) -> Limb {
        match (self, other) {
            (_, 0) => panic!("division by zero"),
            (Natural(Small(small)), other) => small % other,
            (Natural(Large(ref limbs)), other) => limbs_mod_limb(limbs, other),
        }
    }

    fn rem_assign_limb(&mut self, other: Limb) {
        match (&mut *self, other) {
            (_, 0) => panic!("division by zero"),
            (Natural(Small(ref mut small)), other) => *small %= other,
            (Natural(Large(ref mut limbs)), other) => {
                *self = Natural(Small(limbs_mod_limb(limbs, other)))
            }
        }
    }
}

impl Mod<Natural> for Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `Natural`, taking both `Natural`s by value and returning the
    /// remainder. The quotient and remainder satisfy `self` = q * `other` + r and 0 <= r < `other`.
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
    /// use malachite_base::num::arithmetic::traits::Mod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(Natural::from(23u32).mod_op(Natural::from(10u32)).to_string(), "3");
    ///
    /// // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    /// assert_eq!(
    ///      Natural::from_str("1000000000000000000000000").unwrap()
    ///          .mod_op(Natural::from_str("1234567890987").unwrap()).to_string(),
    ///      "530068894399"
    /// );
    /// ```
    #[inline]
    fn mod_op(self, other: Natural) -> Natural {
        self % other
    }
}

impl<'a> Mod<&'a Natural> for Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `Natural`, taking the first `Natural` by value and the second by
    /// reference, and returning the remainder. The quotient and remainder satisfy `self` =
    /// q * `other` + r and 0 <= r < `other`.
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
    /// use malachite_base::num::arithmetic::traits::Mod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(Natural::from(23u32).mod_op(&Natural::from(10u32)).to_string(), "3");
    ///
    /// // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    /// assert_eq!(
    ///      Natural::from_str("1000000000000000000000000").unwrap()
    ///          .mod_op(&Natural::from_str("1234567890987").unwrap()).to_string(),
    ///      "530068894399"
    /// );
    /// ```
    #[inline]
    fn mod_op(self, other: &'a Natural) -> Natural {
        self % other
    }
}

impl<'a> Mod<Natural> for &'a Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `Natural`, taking the first `Natural` by reference and the second
    /// by value, and returning the remainder. The quotient and remainder satisfy `self` =
    /// q * `other` + r and 0 <= r < `other`.
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
    /// use malachite_base::num::arithmetic::traits::Mod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!((&Natural::from(23u32)).mod_op(Natural::from(10u32)).to_string(), "3");
    ///
    /// // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    /// assert_eq!(
    ///      (&Natural::from_str("1000000000000000000000000").unwrap())
    ///          .mod_op(Natural::from_str("1234567890987").unwrap()).to_string(),
    ///      "530068894399"
    /// );
    /// ```
    #[inline]
    fn mod_op(self, other: Natural) -> Natural {
        self % other
    }
}

impl<'a, 'b> Mod<&'b Natural> for &'a Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `Natural`, taking both `Natural`s by reference and returning the
    /// remainder. The quotient and remainder satisfy `self` = q * `other` + r and 0 <= r < `other`.
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
    /// use malachite_base::num::arithmetic::traits::Mod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!((&Natural::from(23u32)).mod_op(&Natural::from(10u32)).to_string(), "3");
    ///
    /// // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    /// assert_eq!(
    ///      (&Natural::from_str("1000000000000000000000000").unwrap())
    ///          .mod_op(&Natural::from_str("1234567890987").unwrap()).to_string(),
    ///      "530068894399"
    /// );
    /// ```
    #[inline]
    fn mod_op(self, other: &'b Natural) -> Natural {
        self % other
    }
}

impl ModAssign<Natural> for Natural {
    /// Divides a `Natural` by a `Natural` in place, taking the second `Natural` by value and
    /// replacing `self` with the remainder. The quotient and remainder satisfy `self` =
    /// q * `other` + r and 0 <= r < `other`.
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
    /// use malachite_base::num::arithmetic::traits::ModAssign;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// // 2 * 10 + 3 = 23
    /// let mut x = Natural::from(23u32);
    /// x.mod_assign(Natural::from(10u32));
    /// assert_eq!(x.to_string(), "3");
    ///
    /// // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    /// let mut x = Natural::from_str("1000000000000000000000000").unwrap();
    /// x.mod_assign(Natural::from_str("1234567890987").unwrap());
    /// assert_eq!(x.to_string(), "530068894399");
    /// ```
    #[inline]
    fn mod_assign(&mut self, other: Natural) {
        *self %= other;
    }
}

impl<'a> ModAssign<&'a Natural> for Natural {
    /// Divides a `Natural` by a `Natural` in place, taking the second `Natural` by reference and
    /// replacing `self` with the remainder. The quotient and remainder satisfy `self` =
    /// q * `other` + r and 0 <= r < `other`.
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
    /// use malachite_base::num::arithmetic::traits::ModAssign;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// // 2 * 10 + 3 = 23
    /// let mut x = Natural::from(23u32);
    /// x.mod_assign(&Natural::from(10u32));
    /// assert_eq!(x.to_string(), "3");
    ///
    /// // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    /// let mut x = Natural::from_str("1000000000000000000000000").unwrap();
    /// x.mod_assign(&Natural::from_str("1234567890987").unwrap());
    /// assert_eq!(x.to_string(), "530068894399");
    /// ```
    fn mod_assign(&mut self, other: &'a Natural) {
        *self %= other;
    }
}

impl Rem<Natural> for Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `Natural`, taking both `Natural`s by value and returning the
    /// remainder. The quotient and remainder satisfy `self` = q * `other` + r and 0 <= r < `other`.
    /// For `Natural`s, rem is equivalent to mod.
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
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!((Natural::from(23u32) % Natural::from(10u32)).to_string(), "3");
    ///
    /// // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    /// assert_eq!(
    ///     (Natural::from_str("1000000000000000000000000").unwrap() %
    ///             Natural::from_str("1234567890987").unwrap()).to_string(),
    ///     "530068894399"
    /// );
    /// ```
    #[inline]
    fn rem(mut self, other: Natural) -> Natural {
        self %= other;
        self
    }
}

impl<'a> Rem<&'a Natural> for Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `Natural`, taking the first `Natural` by value and the second by
    /// reference, and returning the remainder. The quotient and remainder satisfy `self` =
    /// q * `other` + r and 0 <= r < `other`. For `Natural`s, rem is equivalent to mod.
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
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!((Natural::from(23u32) % &Natural::from(10u32)).to_string(), "3");
    ///
    /// // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    /// assert_eq!(
    ///     (Natural::from_str("1000000000000000000000000").unwrap() %
    ///             &Natural::from_str("1234567890987").unwrap()).to_string(),
    ///     "530068894399"
    /// );
    /// ```
    #[inline]
    fn rem(mut self, other: &'a Natural) -> Natural {
        self %= other;
        self
    }
}

impl<'a> Rem<Natural> for &'a Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `Natural`, taking the first `Natural` by reference and the second
    /// by value, and returning the remainder. The quotient and remainder satisfy `self` =
    /// q * `other` + r and 0 <= r < `other`. For `Natural`s, rem is equivalent to mod.
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
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!((&Natural::from(23u32) % Natural::from(10u32)).to_string(), "3");
    ///
    /// // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    /// assert_eq!(
    ///     (&Natural::from_str("1000000000000000000000000").unwrap() %
    ///             Natural::from_str("1234567890987").unwrap()).to_string(),
    ///     "530068894399"
    /// );
    /// ```
    fn rem(self, other: Natural) -> Natural {
        match (self, other) {
            (_, natural_zero!()) => panic!("division by zero"),
            (_, natural_one!()) => Natural::ZERO,
            (n, Natural(Small(d))) => Natural(Small(n.rem_limb_ref(d))),
            (Natural(Small(_)), _) => self.clone(),
            (&Natural(Large(ref ns)), Natural(Large(ref ds))) => {
                if ns.len() >= ds.len() {
                    Natural::from_owned_limbs_asc(limbs_mod(ns, ds))
                } else {
                    self.clone()
                }
            }
        }
    }
}

impl<'a, 'b> Rem<&'b Natural> for &'a Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `Natural`, taking both `Natural`s by reference and returning the
    /// remainder. The quotient and remainder satisfy `self` = q * `other` + r and 0 <= r < `other`.
    /// For `Natural`s, rem is equivalent to mod.
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
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!((&Natural::from(23u32) % &Natural::from(10u32)).to_string(), "3");
    ///
    /// // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    /// assert_eq!(
    ///     (&Natural::from_str("1000000000000000000000000").unwrap() %
    ///             &Natural::from_str("1234567890987").unwrap()).to_string(),
    ///     "530068894399"
    /// );
    /// ```
    fn rem(self, other: &'b Natural) -> Natural {
        match (self, other) {
            (_, natural_zero!()) => panic!("division by zero"),
            (_, natural_one!()) => Natural::ZERO,
            (n, d) if std::ptr::eq(n, d) => Natural::ZERO,
            (n, Natural(Small(d))) => Natural(Small(n.rem_limb_ref(*d))),
            (Natural(Small(_)), _) => self.clone(),
            (&Natural(Large(ref ns)), Natural(Large(ref ds))) => {
                if ns.len() >= ds.len() {
                    Natural::from_owned_limbs_asc(limbs_mod(ns, ds))
                } else {
                    self.clone()
                }
            }
        }
    }
}

impl RemAssign<Natural> for Natural {
    /// Divides a `Natural` by a `Natural` in place, taking the second `Natural` by value and
    /// replacing `self` with the remainder. The quotient and remainder satisfy `self` =
    /// q * `other` + r and 0 <= r < `other`. For `Natural`s, rem is equivalent to mod.
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
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// // 2 * 10 + 3 = 23
    /// let mut x = Natural::from(23u32);
    /// x %= Natural::from(10u32);
    /// assert_eq!(x.to_string(), "3");
    ///
    /// // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    /// let mut x = Natural::from_str("1000000000000000000000000").unwrap();
    /// x %= Natural::from_str("1234567890987").unwrap();
    /// assert_eq!(x.to_string(), "530068894399");
    /// ```
    #[inline]
    fn rem_assign(&mut self, other: Natural) {
        *self %= &other;
    }
}

impl<'a> RemAssign<&'a Natural> for Natural {
    /// Divides a `Natural` by a `Natural` in place, taking the second `Natural` by reference and
    /// replacing `self` with the remainder. The quotient and remainder satisfy `self` =
    /// q * `other` + r and 0 <= r < `other`. For `Natural`s, rem is equivalent to mod.
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
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// // 2 * 10 + 3 = 23
    /// let mut x = Natural::from(23u32);
    /// x %= &Natural::from(10u32);
    /// assert_eq!(x.to_string(), "3");
    ///
    /// // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    /// let mut x = Natural::from_str("1000000000000000000000000").unwrap();
    /// x %= &Natural::from_str("1234567890987").unwrap();
    /// assert_eq!(x.to_string(), "530068894399");
    /// ```
    fn rem_assign(&mut self, other: &'a Natural) {
        match (&mut *self, other) {
            (_, natural_zero!()) => panic!("division by zero"),
            (_, natural_one!()) => *self = Natural::ZERO,
            (_, Natural(Small(d))) => self.rem_assign_limb(*d),
            (Natural(Small(_)), _) => {}
            (&mut Natural(Large(ref mut ns)), Natural(Large(ref ds))) => {
                if ns.len() >= ds.len() {
                    let mut rs = vec![0; ds.len()];
                    limbs_mod_to_out(&mut rs, ns, ds);
                    swap(&mut rs, ns);
                    self.trim();
                }
            }
        }
    }
}

impl NegMod<Natural> for Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `Natural`, taking both `Natural`s by value and returning the
    /// remainder of the negative of the first `Natural` divided by the second. The quotient and
    /// remainder satisfy `self` = q * `other` - r and 0 <= r < `other`.
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
    /// use malachite_base::num::arithmetic::traits::NegMod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// // 3 * 10 - 7 = 23
    /// assert_eq!(Natural::from(23u32).neg_mod(Natural::from(10u32)).to_string(), "7");
    ///
    /// // 810000006724 * 1234567890987 - 704498996588 = 1000000000000000000000000
    /// assert_eq!(
    ///      Natural::from_str("1000000000000000000000000").unwrap()
    ///             .neg_mod(Natural::from_str("1234567890987").unwrap()).to_string(),
    ///      "704498996588"
    /// );
    /// ```
    #[inline]
    fn neg_mod(mut self, other: Natural) -> Natural {
        self.neg_mod_assign(other);
        self
    }
}

impl<'a> NegMod<&'a Natural> for Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `Natural`, taking the first `Natural` by value and the second by
    /// reference, and returning the remainder of the negative of the first `Natural` divided by the
    /// second. The quotient and remainder satisfy `self` = q * `other` - r and 0 <= r < `other`.
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
    /// use malachite_base::num::arithmetic::traits::NegMod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// // 3 * 10 - 7 = 23
    /// assert_eq!(Natural::from(23u32).neg_mod(&Natural::from(10u32)).to_string(), "7");
    ///
    /// // 810000006724 * 1234567890987 - 704498996588 = 1000000000000000000000000
    /// assert_eq!(
    ///      Natural::from_str("1000000000000000000000000").unwrap()
    ///             .neg_mod(&Natural::from_str("1234567890987").unwrap()).to_string(),
    ///      "704498996588"
    /// );
    /// ```
    #[inline]
    fn neg_mod(mut self, other: &'a Natural) -> Natural {
        self.neg_mod_assign(other);
        self
    }
}

impl<'a> NegMod<Natural> for &'a Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `Natural`, taking the first `Natural` by reference and the second
    /// by value, and returning the remainder of the negative of the first `Natural` divided by the
    /// second. The quotient and remainder satisfy `self` = q * `other` - r and 0 <= r < `other`.
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
    /// use malachite_base::num::arithmetic::traits::NegMod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// // 3 * 10 - 7 = 23
    /// assert_eq!((&Natural::from(23u32)).neg_mod(Natural::from(10u32)).to_string(), "7");
    ///
    /// // 810000006724 * 1234567890987 - 704498996588 = 1000000000000000000000000
    /// assert_eq!(
    ///      (&Natural::from_str("1000000000000000000000000").unwrap())
    ///             .neg_mod(Natural::from_str("1234567890987").unwrap()).to_string(),
    ///      "704498996588"
    /// );
    /// ```
    fn neg_mod(self, other: Natural) -> Natural {
        let r = self % &other;
        if r == 0 {
            r
        } else {
            other - r
        }
    }
}

impl<'a, 'b> NegMod<&'b Natural> for &'a Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `Natural`, taking both `Natural`s by reference and returning the
    /// remainder of the negative of the first `Natural` divided by the second. The quotient and
    /// remainder satisfy `self` = q * `other` - r and 0 <= r < `other`.
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
    /// use malachite_base::num::arithmetic::traits::NegMod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// // 3 * 10 - 7 = 23
    /// assert_eq!((&Natural::from(23u32)).neg_mod(&Natural::from(10u32)).to_string(), "7");
    ///
    /// // 810000006724 * 1234567890987 - 704498996588 = 1000000000000000000000000
    /// assert_eq!(
    ///      (&Natural::from_str("1000000000000000000000000").unwrap())
    ///             .neg_mod(&Natural::from_str("1234567890987").unwrap()).to_string(),
    ///      "704498996588"
    /// );
    /// ```
    fn neg_mod(self, other: &'b Natural) -> Natural {
        let r = self % other;
        if r == 0 {
            r
        } else {
            other - r
        }
    }
}

impl NegModAssign<Natural> for Natural {
    /// Divides a `Natural` by a `Natural` in place, taking the second `Natural` by value, replacing
    /// `self` with the remainder of the negative of the first `Natural` divided by the second. The
    /// quotient and remainder satisfy `self` = q * `other` - r and 0 <= r < `other`.
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
    /// use malachite_base::num::arithmetic::traits::NegModAssign;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// // 3 * 10 - 7 = 23
    /// let mut x = Natural::from(23u32);
    /// x.neg_mod_assign(Natural::from(10u32));
    /// assert_eq!(x.to_string(), "7");
    ///
    /// // 810000006724 * 1234567890987 - 704498996588 = 1000000000000000000000000
    /// let mut x = Natural::from_str("1000000000000000000000000").unwrap();
    /// x.neg_mod_assign(Natural::from_str("1234567890987").unwrap());
    /// assert_eq!(x.to_string(), "704498996588");
    /// ```
    fn neg_mod_assign(&mut self, other: Natural) {
        *self %= &other;
        if *self != 0 {
            self.sub_right_assign_no_panic(&other);
        }
    }
}

impl<'a> NegModAssign<&'a Natural> for Natural {
    /// Divides a `Natural` by a `Natural` in place, taking the second `Natural` by reference,
    /// and replacing `self` with the remainder of the negative of the first `Natural` divided by
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
    /// use malachite_base::num::arithmetic::traits::NegModAssign;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// // 3 * 10 - 7 = 23
    /// let mut x = Natural::from(23u32);
    /// x.neg_mod_assign(&Natural::from(10u32));
    /// assert_eq!(x.to_string(), "7");
    ///
    /// // 810000006724 * 1234567890987 - 704498996588 = 1000000000000000000000000
    /// let mut x = Natural::from_str("1000000000000000000000000").unwrap();
    /// x.neg_mod_assign(&Natural::from_str("1234567890987").unwrap());
    /// assert_eq!(x.to_string(), "704498996588");
    /// ```
    fn neg_mod_assign(&mut self, other: &'a Natural) {
        *self %= other;
        if *self != 0 {
            self.sub_right_assign_no_panic(other);
        }
    }
}
