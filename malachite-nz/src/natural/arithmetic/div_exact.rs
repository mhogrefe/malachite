use std::cmp::{max, min, Ordering};

use malachite_base::limbs::{limbs_leading_zero_limbs, limbs_set_zero, limbs_test_zero};
use malachite_base::num::arithmetic::traits::{
    DivExact, DivExactAssign, Parity, WrappingSubAssign,
};

use integer::conversion::to_twos_complement_limbs::limbs_twos_complement_in_place;
use natural::arithmetic::add::{
    limbs_slice_add_greater_in_place_left, limbs_slice_add_same_length_in_place_left,
};
use natural::arithmetic::add_limb::limbs_slice_add_limb_in_place;
use natural::arithmetic::add_mul_limb::limbs_slice_add_mul_limb_same_length_in_place_left;
use natural::arithmetic::div_exact_limb::{limbs_div_exact_limb_to_out, limbs_modular_invert_limb};
use natural::arithmetic::div_mod::MUL_TO_MULMOD_BNM1_FOR_2NXN_THRESHOLD;
use natural::arithmetic::mul::mul_low::limbs_mul_low_same_length;
use natural::arithmetic::mul::mul_mod::{
    _limbs_mul_mod_limb_width_to_n_minus_1, _limbs_mul_mod_limb_width_to_n_minus_1_next_size,
    _limbs_mul_mod_limb_width_to_n_minus_1_scratch_len,
};
use natural::arithmetic::mul::{limbs_mul_greater_to_out, limbs_mul_to_out};
use natural::arithmetic::shr_u::limbs_shr_to_out;
use natural::arithmetic::sub::{
    _limbs_sub_same_length_with_borrow_in_to_out, limbs_sub_in_place_left,
    limbs_sub_same_length_in_place_left, limbs_sub_same_length_to_out,
    limbs_sub_same_length_to_out_with_overlap,
};
use natural::arithmetic::sub_limb::{limbs_sub_limb_in_place, limbs_sub_limb_to_out};
use natural::arithmetic::sub_mul_limb::limbs_sub_mul_limb_same_length_in_place_left;
use natural::comparison::ord::limbs_cmp_same_length;
use natural::Natural;
use platform::{
    Limb, BINV_NEWTON_THRESHOLD, DC_BDIV_QR_THRESHOLD, DC_BDIV_Q_THRESHOLD, MU_BDIV_Q_THRESHOLD,
};

/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// Result is O(`n`)
///
/// This is mpn_binvert_itch from mpn/generic/binvert.c.
pub fn limbs_modular_invert_scratch_len(n: usize) -> usize {
    let itch_local = _limbs_mul_mod_limb_width_to_n_minus_1_next_size(n);
    let itch_out = _limbs_mul_mod_limb_width_to_n_minus_1_scratch_len(itch_local, n, (n + 1) >> 1);
    itch_local + itch_out
}

pub fn _limbs_modular_invert_small(
    size: usize,
    is: &mut [Limb],
    scratch: &mut [Limb],
    ds: &[Limb],
    inverse: Limb,
) {
    if size < DC_BDIV_Q_THRESHOLD {
        _limbs_modular_div_schoolbook(is, scratch, ds, inverse);
    } else {
        _limbs_modular_div_divide_and_conquer(is, scratch, ds, inverse);
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
/// This is mpn_binvert from mpn/generic/binvert.c.
pub fn limbs_modular_invert(is: &mut [Limb], ds: &[Limb], scratch: &mut [Limb]) {
    let d_len = ds.len();
    // Compute the computation precisions from highest to lowest, leaving the basecase size in
    // `size`.
    let mut size = d_len;
    let mut sizes = Vec::new();
    while size >= BINV_NEWTON_THRESHOLD {
        sizes.push(size);
        size = (size + 1) >> 1;
    }
    // Compute a base value of `size` limbs.
    let scratch_lo = &mut scratch[..size];
    let ds_lo = &ds[..size];
    limbs_set_zero(scratch_lo);
    scratch_lo[0] = 1;
    let inverse = limbs_modular_invert_limb(ds[0]).wrapping_neg();
    _limbs_modular_invert_small(size, is, scratch_lo, ds_lo, inverse);
    let mut previous_size = size;
    // Use Newton iterations to get the desired precision.
    for &size in sizes.iter().rev() {
        let mul_size = _limbs_mul_mod_limb_width_to_n_minus_1_next_size(size);
        let (scratch_lo, scratch_hi) = scratch.split_at_mut(mul_size);
        let (is_lo, is_hi) = is.split_at_mut(previous_size);
        _limbs_mul_mod_limb_width_to_n_minus_1(
            scratch_lo,
            mul_size,
            &ds[..size],
            is_lo,
            scratch_hi,
        );
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

/// Computes a binary quotient of size `q_len` = `ns.len()` - `ds.len()`. D must be odd. `inverse`
/// is (-D) ^ -1 mod 2 ^ `Limb::WIDTH`, or `limbs_modular_invert_limb(ds[0]).wrapping_neg()`.
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
/// This is mpn_sbpi1_bdiv_qr from mpn/generic/sbpi1_bdiv_qr.c.
pub fn _limbs_modular_div_mod_schoolbook(
    qs: &mut [Limb],
    ns: &mut [Limb],
    ds: &[Limb],
    inverse: Limb,
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
            let q = inverse.wrapping_mul(ns[0]);
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
        let q = inverse.wrapping_mul(ns[0]);
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
/// This is mpn_dcpi1_bdiv_qr_n from mpn/generic/dcpi1_bdiv_qr.c.
fn _limbs_modular_div_mod_divide_and_conquer_helper(
    qs: &mut [Limb],
    ns: &mut [Limb],
    ds: &[Limb],
    inverse: Limb,
    scratch: &mut [Limb],
) -> bool {
    let n = ds.len();
    let ns = &mut ns[..n << 1];
    let scratch = &mut scratch[..n];
    let lo = n >> 1; // floor(n / 2)
    let hi = n - lo; // ceil(n / 2)
    let (ds_lo, ds_hi) = ds.split_at(lo);
    let carry = if lo < DC_BDIV_QR_THRESHOLD {
        _limbs_modular_div_mod_schoolbook(qs, &mut ns[..lo << 1], ds_lo, inverse)
    } else {
        _limbs_modular_div_mod_divide_and_conquer_helper(qs, ns, ds_lo, inverse, scratch)
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
        _limbs_modular_div_mod_schoolbook(qs_hi, &mut ns[..hi << 1], ds_lo, inverse)
    } else {
        _limbs_modular_div_mod_divide_and_conquer_helper(qs_hi, ns, ds_lo, inverse, scratch)
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
/// `rs.len()`.. D must be odd. `inverse` is (-D) ^ -1 mod 2 ^ `Limb::WIDTH`, or
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
/// This is mpn_dcpi1_bdiv_qr from mpn/generic/dcpi1_bdiv_qr.c.
pub fn _limbs_modular_div_mod_divide_and_conquer(
    qs: &mut [Limb],
    ns: &mut [Limb],
    ds: &[Limb],
    inverse: Limb,
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
            _limbs_modular_div_mod_schoolbook(qs, &mut ns[..q_len_mod_d_len << 1], ds_lo, inverse)
        } else {
            _limbs_modular_div_mod_divide_and_conquer_helper(qs, ns, ds_lo, inverse, &mut scratch)
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
                inverse,
                &mut scratch,
            );
            q_len_s -= d_len;
        }
    } else {
        let (ds_lo, ds_hi) = ds.split_at(q_len);
        carry = if q_len < DC_BDIV_QR_THRESHOLD {
            _limbs_modular_div_mod_schoolbook(qs, &mut ns[..q_len << 1], ds_lo, inverse)
        } else {
            _limbs_modular_div_mod_divide_and_conquer_helper(qs, ns, ds_lo, inverse, &mut scratch)
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
/// This is mpn_dcpi1_bdiv_qr_n_itch from mpn/generic/dcpi1_bdiv_qr.c.
#[inline]
pub const fn _limbs_modular_div_mod_divide_and_conquer_helper_scratch_len(n: usize) -> usize {
    n
}

/// This is mpn_mu_bdiv_qr_itch from mpn/generic/mu_bdiv_qr.c.
pub fn _limbs_modular_div_mod_barrett_scratch_len(n_len: usize, d_len: usize) -> usize {
    assert!(DC_BDIV_Q_THRESHOLD < MU_BDIV_Q_THRESHOLD);
    let q_len = n_len - d_len;
    let i_len = if q_len > d_len {
        let blocks = (q_len - 1) / d_len + 1; // ceil(qn / dn), number of blocks
        (q_len - 1) / blocks + 1 // ceil(qn / ceil(qn / dn))
    } else {
        q_len - (q_len >> 1)
    };
    let (mul_len_1, mul_len_2) = if i_len < MUL_TO_MULMOD_BNM1_FOR_2NXN_THRESHOLD {
        (d_len + i_len, 0)
    } else {
        let temp_len = _limbs_mul_mod_limb_width_to_n_minus_1_next_size(d_len);
        (
            temp_len,
            _limbs_mul_mod_limb_width_to_n_minus_1_scratch_len(temp_len, d_len, i_len),
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
    let blocks = (q_len - 1) / d_len + 1; // ceil(qn / dn), number of blocks
    let i_len = (q_len - 1) / blocks + 1; // ceil(qn / b) = ceil(qn / ceil(qn / dn))
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
            let mul_size = _limbs_mul_mod_limb_width_to_n_minus_1_next_size(d_len);
            {
                let (scratch, scratch_out) = scratch.split_at_mut(mul_size);
                _limbs_mul_mod_limb_width_to_n_minus_1(scratch, mul_size, ds, qs, scratch_out);
            }
            //TODO Else is untested!
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
        let tn = _limbs_mul_mod_limb_width_to_n_minus_1_next_size(d_len);
        {
            let (scratch, scratch_out) = scratch.split_at_mut(tn);
            _limbs_mul_mod_limb_width_to_n_minus_1(scratch, tn, ds, qs, scratch_out);
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
    if d_len != q_len_s
        && limbs_sub_same_length_to_out_with_overlap(rs, q_len_s, &scratch_lo[q_len_s..])
    {
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
        let mul_size = _limbs_mul_mod_limb_width_to_n_minus_1_next_size(d_len);
        {
            let (scratch, scratch_out) = scratch.split_at_mut(mul_size);
            _limbs_mul_mod_limb_width_to_n_minus_1(scratch, mul_size, ds, qs_lo, scratch_out);
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
        let mul_size = _limbs_mul_mod_limb_width_to_n_minus_1_next_size(d_len);
        {
            let (scratch, scratch_out) = scratch.split_at_mut(mul_size);
            _limbs_mul_mod_limb_width_to_n_minus_1(scratch, mul_size, ds, qs_hi, scratch_out);
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
    if limbs_sub_same_length_to_out_with_overlap(rs, q_len_s, &scratch_lo[q_len_s..]) {
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
/// `rs.len()`. D must be odd.
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
/// This is mpn_mu_bdiv_qr from mpn/generic/mu_bdiv_qr.c.
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

/// Computes Q = N / D mod 2 ^ (`Limb::WIDTH` * `ns.len()`), destroying N. D must be odd. `inverse`
/// is (-D) ^ -1 mod 2 ^ `Limb::WIDTH`, or `limbs_modular_invert_limb(ds[0]).wrapping_neg()`.
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
/// This is mpn_sbpi1_bdiv_q from mpn/generic/sbpi1_bdiv_q.c.
pub fn _limbs_modular_div_schoolbook(qs: &mut [Limb], ns: &mut [Limb], ds: &[Limb], inverse: Limb) {
    let n_len = ns.len();
    let d_len = ds.len();
    assert_ne!(d_len, 0);
    assert!(n_len >= d_len);
    assert!(ds[0].odd());
    let qs = &mut qs[..n_len];
    let diff = n_len - d_len;
    for i in 0..diff {
        let q = inverse.wrapping_mul(ns[i]);
        let (ns_lo, ns_hi) = ns[i..].split_at_mut(d_len);
        let carry = limbs_slice_add_mul_limb_same_length_in_place_left(ns_lo, ds, q);
        limbs_slice_add_limb_in_place(ns_hi, carry);
        assert_eq!(ns_lo[0], 0);
        qs[i] = !q;
    }
    let last_index = n_len - 1;
    for i in diff..last_index {
        let ns_hi = &mut ns[i..];
        let q = inverse.wrapping_mul(ns_hi[0]);
        limbs_slice_add_mul_limb_same_length_in_place_left(ns_hi, &ds[..n_len - i], q);
        assert_eq!(ns_hi[0], 0);
        qs[i] = !q;
    }
    qs[last_index] = !inverse.wrapping_mul(ns[last_index]);
    limbs_slice_add_limb_in_place(qs, 1);
}

/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// This is mpn_dcpi1_bdiv_q_n_itch from mpn/generic/dcpi1_bdiv_q.c.
pub const fn _limbs_modular_div_divide_and_conquer_helper_scratch_len(n: usize) -> usize {
    n
}

/// Time: worst case O(n * log(n) ^ 2 * log(log(n)))
///
/// Additional memory: worst case O(n * log(n))
///
/// where n = `ds.len()`
///
/// This is mpn_dcpi1_bdiv_q_n from mpn/generic/dcpi1_bdiv_q.c.
fn _limbs_modular_div_divide_and_conquer_helper(
    qs: &mut [Limb],
    ns: &mut [Limb],
    ds: &[Limb],
    inverse: Limb,
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
            _limbs_modular_div_mod_divide_and_conquer_helper(qs, ns, &ds[..lo], inverse, scratch);
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
    _limbs_modular_div_schoolbook(&mut qs[m..], &mut ns[m..n], &ds[..n_rem], inverse);
}

/// Computes Q = N / D mod 2 ^ (`Limb::WIDTH` * `ns.len()`), destroying N. D must be odd. `inverse`
/// is (-D) ^ -1 mod 2 ^ `Limb::WIDTH`, or `limbs_modular_invert_limb(ds[0]).wrapping_neg()`.
///
/// Time: worst case O(n * log(d) ^ 2 * log(log(d)))
///
/// Additional memory: worst case O(n * log(n))
///
/// where n = `ns.len()`, d = `ds.len()`
///
/// This is mpn_dcpi1_bdiv_q from mpn/generic/dcpi1_bdiv_q.c.
pub fn _limbs_modular_div_divide_and_conquer(
    qs: &mut [Limb],
    ns: &mut [Limb],
    ds: &[Limb],
    inverse: Limb,
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
            _limbs_modular_div_mod_schoolbook(qs, &mut ns[..n_len_mod_d_len << 1], ds_lo, inverse)
        } else {
            _limbs_modular_div_mod_divide_and_conquer_helper(qs, ns, ds_lo, inverse, &mut scratch)
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
                inverse,
                &mut scratch,
            );
            m += d_len;
        }
        _limbs_modular_div_divide_and_conquer_helper(
            &mut qs[diff..],
            &mut ns[diff..],
            ds,
            inverse,
            &mut scratch,
        );
    } else if n_len < DC_BDIV_Q_THRESHOLD {
        _limbs_modular_div_schoolbook(qs, ns, ds, inverse);
    } else {
        let mut scratch = vec![0; n_len];
        _limbs_modular_div_divide_and_conquer_helper(qs, ns, ds, inverse, &mut scratch);
    }
}

/// This is mpn_mu_bdiv_q_itch from mpn/generic/mu_bdiv_q.c.
pub fn _limbs_modular_div_barrett_scratch_len(n_len: usize, d_len: usize) -> usize {
    assert!(DC_BDIV_Q_THRESHOLD < MU_BDIV_Q_THRESHOLD);
    let i_len;
    let mul_len = if n_len > d_len {
        let blocks = (n_len - 1) / d_len + 1; // ceil(qn / dn), number of blocks
        i_len = (n_len - 1) / blocks + 1; // ceil(qn / b) = ceil(qn / ceil(qn / dn))
        let (mul_len_1, mul_len_2) = if i_len < MUL_TO_MULMOD_BNM1_FOR_2NXN_THRESHOLD {
            (d_len + i_len, 0)
        } else {
            let mul_len_1 = _limbs_mul_mod_limb_width_to_n_minus_1_next_size(d_len);
            (
                mul_len_1,
                _limbs_mul_mod_limb_width_to_n_minus_1_scratch_len(mul_len_1, d_len, i_len),
            )
        };
        d_len + mul_len_1 + mul_len_2
    } else {
        i_len = n_len - (n_len >> 1);
        let (mul_len_1, mul_len_2) = if i_len < MUL_TO_MULMOD_BNM1_FOR_2NXN_THRESHOLD {
            (n_len + i_len, 0)
        } else {
            let mul_len_1 = _limbs_mul_mod_limb_width_to_n_minus_1_next_size(n_len);
            (
                mul_len_1,
                _limbs_mul_mod_limb_width_to_n_minus_1_scratch_len(mul_len_1, n_len, i_len),
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
    let blocks = (n_len - 1) / d_len + 1; // ceil(qn / dn), number of blocks
    let i_len = (n_len - 1) / blocks + 1; // ceil(qn / b) = ceil(qn / ceil(qn / dn))
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
            let mul_size = _limbs_mul_mod_limb_width_to_n_minus_1_next_size(d_len);
            {
                let (scratch, scratch_out) = scratch.split_at_mut(mul_size);
                _limbs_mul_mod_limb_width_to_n_minus_1(scratch, mul_size, ds, qs_lo, scratch_out);
            }
            //TODO Else is untested!
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
        let mul_size = _limbs_mul_mod_limb_width_to_n_minus_1_next_size(d_len);
        {
            let (scratch, scratch_out) = scratch.split_at_mut(mul_size);
            _limbs_mul_mod_limb_width_to_n_minus_1(scratch, mul_size, ds, qs_lo, scratch_out);
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
        let mul_size = _limbs_mul_mod_limb_width_to_n_minus_1_next_size(n_len);
        {
            let (scratch, scratch_out) = scratch.split_at_mut(mul_size);
            _limbs_mul_mod_limb_width_to_n_minus_1(scratch, mul_size, ds, qs_lo, scratch_out);
        }
        //TODO Else is untested!
        if let Some(wrapped_len) = (n_len + i_len).checked_sub(mul_size) {
            let (scratch_lo, scratch_hi) = scratch.split_at_mut(wrapped_len);
            if wrapped_len != 0
                && limbs_cmp_same_length(scratch_lo, &ns[..wrapped_len]) == Ordering::Less
            {
                assert!(!limbs_sub_limb_in_place(scratch_hi, 1));
            }
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
/// This is mpn_mu_bdiv_q from mpn/generic/mu_bdiv_q.c.
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

/// This is mpn_bdiv_q_itch from mpn/generic/bdiv_q.c.
pub fn _limbs_modular_div_scratch_len(n_len: usize, d_len: usize) -> usize {
    if d_len < MU_BDIV_Q_THRESHOLD {
        n_len
    } else {
        _limbs_modular_div_barrett_scratch_len(n_len, d_len)
    }
}

/// Computes Q = N / D mod 2 ^ (`Limb::WIDTH` * `ns.len()`). D must be odd.
///
/// Time: Worst case O(n * log(n) * log(log(n)))
///
/// Additional memory: Worst case O(n * log(n))
///
/// where n = `ns.len()`
///
/// This is mpn_bdiv_q from mpn/generic/bdiv_q.c.
pub fn _limbs_modular_div(qs: &mut [Limb], ns: &[Limb], ds: &[Limb], scratch: &mut [Limb]) {
    let n_len = ns.len();
    let d_len = ds.len();
    if d_len < DC_BDIV_Q_THRESHOLD {
        let scratch = &mut scratch[..n_len];
        scratch.copy_from_slice(ns);
        let inverse = limbs_modular_invert_limb(ds[0]).wrapping_neg();
        _limbs_modular_div_schoolbook(qs, scratch, ds, inverse);
    } else if d_len < MU_BDIV_Q_THRESHOLD {
        let scratch = &mut scratch[..n_len];
        scratch.copy_from_slice(ns);
        let inverse = limbs_modular_invert_limb(ds[0]).wrapping_neg();
        _limbs_modular_div_divide_and_conquer(qs, scratch, ds, inverse);
    } else {
        _limbs_modular_div_barrett(qs, ns, ds, scratch);
    }
}

/// Interpreting two slices of `Limb`s, `ns` and `ds`, as the limbs (in ascending order) of two
/// `Natural`s, divides them, writing the `ns.len() - ds.len() + 1` limbs of the quotient to `qs`.
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
/// limbs_div_exact_to_out(qs, &[0, 0, 0, 6, 19, 32, 21], &[0, 0, 1, 2, 3]);
/// assert_eq!(qs, &[0, 6, 7, 10]);
///
/// let qs = &mut [10; 4];
/// limbs_div_exact_to_out(qs, &[10_200, 20_402, 30_605, 20_402, 10_200], &[100, 101, 102]);
/// assert_eq!(qs, &[102, 101, 100, 10]);
/// ```
///
/// This is mpn_divexact from mpn/generic/divexact.c.
pub fn limbs_div_exact_to_out(qs: &mut [Limb], ns: &[Limb], ds: &[Limb]) {
    let n_len = ns.len();
    let d_len = ds.len();
    assert_ne!(d_len, 0);
    assert!(n_len >= d_len);
    assert_ne!(ds[d_len - 1], 0);
    let leading_zero_limbs = limbs_leading_zero_limbs(ds);
    assert!(
        limbs_test_zero(&ns[..leading_zero_limbs]),
        "division not exact"
    );
    let mut ns_scratch;
    let mut ds_scratch;
    let mut ns = &ns[leading_zero_limbs..];
    let mut ds = &ds[leading_zero_limbs..];
    let n_len = ns.len();
    let d_len = ds.len();
    if d_len == 1 {
        limbs_div_exact_limb_to_out(qs, ns, ds[0]);
        return;
    }
    let q_len = n_len - d_len + 1;
    let shift = ds[0].trailing_zeros();
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
    let mut scratch = vec![0; _limbs_modular_div_scratch_len(q_len, d_len)];
    _limbs_modular_div(qs, &ns[..q_len], &ds[..d_len], &mut scratch);
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
    /// fn main() {
    ///     // 123 * 456 = 56088
    ///     assert_eq!(Natural::from(56088u32).div_exact(Natural::from(456u32)).to_string(), "123");
    ///
    ///     // 123456789000 * 987654321000 = 121932631112635269000000
    ///     assert_eq!(
    ///         Natural::from_str("121932631112635269000000").unwrap()
    ///             .div_exact(Natural::from_str("987654321000").unwrap()).to_string(),
    ///         "123456789000"
    ///     );
    /// }
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
    /// fn main() {
    ///     // 123 * 456 = 56088
    ///     assert_eq!(
    ///         Natural::from(56088u32).div_exact(&Natural::from(456u32)).to_string(),
    ///         "123"
    ///     );
    ///
    ///     // 123456789000 * 987654321000 = 121932631112635269000000
    ///     assert_eq!(
    ///         Natural::from_str("121932631112635269000000").unwrap()
    ///             .div_exact(&Natural::from_str("987654321000").unwrap()).to_string(),
    ///         "123456789000"
    ///     );
    /// }
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
    /// fn main() {
    ///     // 123 * 456 = 56088
    ///     assert_eq!(
    ///         (&Natural::from(56088u32)).div_exact(Natural::from(456u32)).to_string(),
    ///         "123"
    ///     );
    ///
    ///     // 123456789000 * 987654321000 = 121932631112635269000000
    ///     assert_eq!(
    ///         (&Natural::from_str("121932631112635269000000").unwrap())
    ///             .div_exact(Natural::from_str("987654321000").unwrap()).to_string(),
    ///         "123456789000"
    ///     );
    /// }
    /// ```
    fn div_exact(self, other: Natural) -> Natural {
        //TODO
        self / other
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
    /// fn main() {
    ///     // 123 * 456 = 56088
    ///     assert_eq!(
    ///         (&Natural::from(56088u32)).div_exact(&Natural::from(456u32)).to_string(),
    ///         "123"
    ///     );
    ///
    ///     // 123456789000 * 987654321000 = 121932631112635269000000
    ///     assert_eq!(
    ///         (&Natural::from_str("121932631112635269000000").unwrap())
    ///             .div_exact(&Natural::from_str("987654321000").unwrap()).to_string(),
    ///         "123456789000"
    ///     );
    /// }
    /// ```
    fn div_exact(self, other: &'b Natural) -> Natural {
        //TODO
        self / other
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
    /// fn main() {
    ///     // 123 * 456 = 56088
    ///     let mut x = Natural::from(56088u32);
    ///     x.div_exact_assign(Natural::from(456u32));
    ///     assert_eq!(x.to_string(), "123");
    ///
    ///     // 123456789000 * 987654321000 = 121932631112635269000000
    ///     let mut x = Natural::from_str("121932631112635269000000").unwrap();
    ///     x.div_exact_assign(Natural::from_str("987654321000").unwrap());
    ///     assert_eq!(x.to_string(), "123456789000");
    /// }
    /// ```
    fn div_exact_assign(&mut self, other: Natural) {
        //TODO
        *self /= other;
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
    /// fn main() {
    ///     // 123 * 456 = 56088
    ///     let mut x = Natural::from(56088u32);
    ///     x.div_exact_assign(&Natural::from(456u32));
    ///     assert_eq!(x.to_string(), "123");
    ///
    ///     // 123456789000 * 987654321000 = 121932631112635269000000
    ///     let mut x = Natural::from_str("121932631112635269000000").unwrap();
    ///     x.div_exact_assign(&Natural::from_str("987654321000").unwrap());
    ///     assert_eq!(x.to_string(), "123456789000");
    /// }
    /// ```
    fn div_exact_assign(&mut self, other: &'a Natural) {
        //TODO
        *self /= other;
    }
}
