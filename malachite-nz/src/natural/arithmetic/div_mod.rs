use std::cmp::{max, min, Ordering};
use std::mem::swap;

use malachite_base::comparison::Max;
use malachite_base::limbs::{limbs_move_left, limbs_set_zero};
use malachite_base::num::arithmetic::traits::{
    CeilingDivAssignNegMod, CeilingDivNegMod, DivAssignMod, DivAssignRem, DivMod, DivRem,
    WrappingAddAssign, WrappingSub, WrappingSubAssign,
};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{CheckedFrom, JoinHalves, SplitInHalf};

use natural::arithmetic::add::{
    _limbs_add_same_length_with_carry_in_in_place_left,
    _limbs_add_same_length_with_carry_in_to_out, limbs_add_same_length_to_out,
    limbs_slice_add_same_length_in_place_left,
};
use natural::arithmetic::add_limb::{limbs_add_limb_to_out, limbs_slice_add_limb_in_place};
use natural::arithmetic::mul::mul_mod::_limbs_mul_mod_limb_width_to_n_minus_1;
use natural::arithmetic::mul::mul_mod::{
    _limbs_mul_mod_limb_width_to_n_minus_1_next_size,
    _limbs_mul_mod_limb_width_to_n_minus_1_scratch_len,
};
use natural::arithmetic::mul::{
    limbs_mul_greater_to_out, limbs_mul_same_length_to_out, limbs_mul_to_out,
};
use natural::arithmetic::shl_u::{limbs_shl_to_out, limbs_slice_shl_in_place};
use natural::arithmetic::shr_u::{limbs_shr_to_out, limbs_slice_shr_in_place};
use natural::arithmetic::sub::{
    _limbs_sub_same_length_with_borrow_in_in_place_left,
    _limbs_sub_same_length_with_borrow_in_in_place_right,
    _limbs_sub_same_length_with_borrow_in_to_out, limbs_sub_in_place_left,
    limbs_sub_same_length_in_place_left, limbs_sub_same_length_in_place_right,
    limbs_sub_same_length_to_out,
};
use natural::arithmetic::sub_limb::limbs_sub_limb_in_place;
use natural::arithmetic::sub_mul_limb::limbs_sub_mul_limb_same_length_in_place_left;
use natural::comparison::ord::limbs_cmp_same_length;
use natural::logic::not::limbs_not_to_out;
use natural::Natural::{self, Large, Small};
use platform::{DoubleLimb, Limb};

// will remove
fn udiv_qrnnd(q: &mut Limb, r: &mut Limb, n_hi: Limb, n_lo: Limb, d: Limb) {
    let n = DoubleLimb::join_halves(n_hi, n_lo);
    let d = DoubleLimb::from(d);
    *r = (n % d).lower_half();
    *q = (n / d).lower_half();
}

// will remove
fn umul_ppmm(ph: &mut Limb, pl: &mut Limb, m1: Limb, m2: Limb) {
    let (hi, lo) = (DoubleLimb::from(m1) * DoubleLimb::from(m2)).split_in_half();
    *ph = hi;
    *pl = lo;
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
    let mut inverse = (DoubleLimb::join_halves(!hi, Limb::MAX) / DoubleLimb::from(hi)).lower_half();
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
/// use malachite_nz::natural::arithmetic::div_mod::limbs_div_mod_by_two_limb;
///
/// let qs = &mut [10, 10, 10, 10];
/// let ns = &mut [1, 2, 3, 4, 5];
/// assert_eq!(limbs_div_mod_by_two_limb(qs, ns, &[3, 0x8000_0000]), false);
/// assert_eq!(qs, &[4294967241, 7, 10, 10]);
/// assert_eq!(ns, &[166, 2147483626, 3, 4, 5]);
/// ```
///
/// This is mpn_divrem_2 from mpn/generic/divrem_2.c.
pub fn limbs_div_mod_by_two_limb(qs: &mut [Limb], ns: &mut [Limb], ds: &[Limb]) -> bool {
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
/// Time: worst case O((n - d) * n + d)
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

//TODO tune
const DC_DIV_QR_THRESHOLD: usize = 51;

/// Time: worst case O(n * log(n) ^ 2 * log(log(n)))
///
/// Additional memory: worst case O(n * log(n) ^ 2)
///
/// where n = `ds.len()`
///
/// This is mpn_dcpi1_div_qr_n from mpn/generic/dcpi1_div_qr.c.
fn _limbs_div_mod_divide_and_conquer_helper(
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

/// Recursive divide-and-conquer division for arbitrary-size operands.
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
                        limbs_div_mod_by_two_limb(qs, ns, ds_hi)
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
                    if highest_q {
                        if limbs_sub_same_length_in_place_left(&mut ns[q_len_mod_d_len..], ds_lo) {
                            carry += 1;
                        }
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
            if highest_q {
                if limbs_sub_same_length_in_place_left(&mut ns[q_len..], ds_lo) {
                    carry += 1;
                }
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

/// Schoolbook division using the Möller-Granlund 3/2 division algorithm, returning approximate
/// quotient.
///
/// Divides `ns` by `ds` and writes the `ns.len()` - `ds.len()` least-significant quotient limbs to
/// `qs`. Returns the most significant limb of the quotient; `true` means 1 and `false` means 0. The
/// quotient is either correct, or one too large. `ds` must have length greater than 2, `ns` must be
/// at least as long as `ds`, and the most significant bit of `ds` must be set. `inverse` should be
/// the result of `limbs_two_limb_inverse_helper` applied to the two highest limbs of the
/// denominator.
///
/// Time: worst case O(n ^ 2)
///
/// Additional memory: worst case O(1)
///
/// where n = `ns.len()`
///
/// # Panics
/// Panics if `ds` has length smaller than 3, `ns` is shorter than `ds`, `qs` has length less than
/// `ns.len()` - `ds.len()`, or the last limb of `ds` does not have its highest bit set.
///
/// This is mpn_sbpi1_divappr_q from mpn/generic/sbpi1_divappr_q.c.
pub fn _limbs_div_mod_schoolbook_approx(
    qs: &mut [Limb],
    ns: &mut [Limb],
    mut ds: &[Limb],
    inverse: Limb,
) -> bool {
    let n_len = ns.len();
    let d_len = ds.len();
    assert!(d_len > 2);
    assert!(n_len >= d_len);
    let a = d_len - 1;
    let d_1 = ds[a];
    assert!(d_1.get_highest_bit());
    let b = d_len - 2;
    let d_0 = ds[b];
    let q_len = n_len - d_len;
    assert!(qs.len() >= q_len);
    if q_len + 1 < d_len {
        ds = &ds[d_len - (q_len + 1)..];
    }
    let d_len = ds.len();
    let d_len_minus_1 = d_len - 1;
    let highest_q;
    {
        let ns = &mut ns[n_len - d_len..];
        highest_q = limbs_cmp_same_length(ns, ds) >= Ordering::Equal;
        if highest_q {
            limbs_sub_same_length_in_place_left(ns, ds);
        }
    }
    let mut n_1 = *ns.last().unwrap();
    let mut q;
    let mut n_0;
    for i in (d_len_minus_1..q_len).rev() {
        let j = i + a;
        if n_1 == d_1 && ns[j] == d_0 {
            q = Limb::MAX;
            limbs_sub_mul_limb_same_length_in_place_left(&mut ns[j - d_len_minus_1..j + 1], ds, q);
            n_1 = ns[j]; // update n_1, last loop's value will now be invalid
        } else {
            let (new_q, new_n) =
                limbs_div_mod_three_limb_by_two_limb(n_1, ns[j], ns[j - 1], d_1, d_0, inverse);
            q = new_q;
            let (new_n1, new_n0) = new_n.split_in_half();
            n_1 = new_n1;
            n_0 = new_n0;
            let local_carry_1 = limbs_sub_mul_limb_same_length_in_place_left(
                &mut ns[j - d_len_minus_1..j - 1],
                &ds[..d_len_minus_1 - 1],
                q,
            );
            let local_carry_2 = n_0 < local_carry_1;
            n_0.wrapping_sub_assign(local_carry_1);
            let carry = local_carry_2 && n_1 == 0;
            if local_carry_2 {
                n_1.wrapping_sub_assign(1);
            }
            ns[j - 1] = n_0;
            if carry {
                n_1.wrapping_add_assign(d_1);
                if limbs_slice_add_same_length_in_place_left(
                    &mut ns[j - d_len_minus_1..j],
                    &ds[..d_len_minus_1],
                ) {
                    n_1.wrapping_add_assign(1);
                }
                q.wrapping_sub_assign(1);
            }
        }
        qs[i] = q;
    }
    let mut flag = true;
    if d_len_minus_1 > 0 {
        for i in (1..d_len_minus_1).rev() {
            let j = i + a;
            if !flag || n_1 >= d_1 {
                q = Limb::MAX;
                let carry = limbs_sub_mul_limb_same_length_in_place_left(&mut ns[b..j + 1], ds, q);
                if n_1 != carry {
                    // TODO This branch is untested!
                    if flag && n_1 < carry {
                        q.wrapping_sub_assign(1);
                        limbs_slice_add_same_length_in_place_left(&mut ns[b..j + 1], ds);
                    } else {
                        flag = false;
                    }
                }
                n_1 = ns[j];
            } else {
                let (new_q, new_n) =
                    limbs_div_mod_three_limb_by_two_limb(n_1, ns[j], ns[j - 1], d_1, d_0, inverse);
                q = new_q;
                let (new_n_1, new_n_0) = new_n.split_in_half();
                n_1 = new_n_1;
                n_0 = new_n_0;
                let local_carry_1 =
                    limbs_sub_mul_limb_same_length_in_place_left(&mut ns[b..j - 1], &ds[..i], q);
                let local_carry_2 = n_0 < local_carry_1;
                n_0.wrapping_sub_assign(local_carry_1);
                let carry = local_carry_2 && n_1 == 0;
                if local_carry_2 {
                    n_1.wrapping_sub_assign(1);
                }
                ns[j - 1] = n_0;
                if carry {
                    n_1.wrapping_add_assign(d_1);
                    if limbs_slice_add_same_length_in_place_left(&mut ns[b..j], &ds[..i + 1]) {
                        n_1.wrapping_add_assign(1);
                    }
                    q.wrapping_sub_assign(1);
                }
            }
            qs[i] = q;
            ds = &ds[1..];
        }
        let ns = &mut ns[b..];
        if !flag || n_1 >= d_1 {
            q = Limb::MAX;
            let carry = limbs_sub_mul_limb_same_length_in_place_left(&mut ns[..2], &ds[..2], q);
            if flag && n_1 < carry {
                // TODO This branch is untested!
                q.wrapping_sub_assign(1);
                limbs_slice_add_same_length_in_place_left(&mut ns[..2], &ds[..2]);
            }
            n_1 = ns[1];
        } else {
            let (new_q, new_n) =
                limbs_div_mod_three_limb_by_two_limb(n_1, ns[1], ns[0], d_1, d_0, inverse);
            q = new_q;
            let (new_n_1, n_0) = new_n.split_in_half();
            n_1 = new_n_1;
            ns[1] = n_1;
            ns[0] = n_0;
        }
        qs[0] = q;
    }
    assert_eq!(ns[a], n_1);
    highest_q
}

//TODO tune
const DC_DIVAPPR_Q_THRESHOLD: usize = 171;

/// Time: worst case O(n * log(n) ^ 2 * log(log(n)))
///
/// Additional memory: worst case O(n * log(n) ^ 2)
///
/// where n = `ds.len()`
///
/// This is mpn_dcpi1_divappr_q_n from mpn/generic/dcpi1_divappr_q.c, where ns here is np +
/// (n >> 1).
fn _limbs_div_mod_divide_and_conquer_approx_helper(
    qs: &mut [Limb],
    ns: &mut [Limb],
    ds: &[Limb],
    inverse: Limb,
    scratch: &mut [Limb],
) -> bool {
    let d_len = ds.len();
    let lo = d_len >> 1; // floor(d_len / 2)
    let hi = d_len - lo; // ceil(d_len / 2)
    assert!(ns.len() >= d_len + hi);
    let (ds_lo, ds_hi) = ds.split_at(lo);
    let mut carry;
    let mut highest_q;
    {
        let qs_hi = &mut qs[lo..];
        highest_q = {
            let ns_hi = &mut ns[lo..];
            if hi < DC_DIV_QR_THRESHOLD {
                _limbs_div_mod_schoolbook(qs_hi, &mut ns_hi[..hi << 1], ds_hi, inverse)
            } else {
                _limbs_div_mod_divide_and_conquer_helper(qs_hi, ns_hi, ds_hi, inverse, scratch)
            }
        };
        limbs_mul_greater_to_out(scratch, &qs_hi[..hi], ds_lo);
        let ns_lo = &mut ns[..d_len];
        carry = if limbs_sub_same_length_in_place_left(ns_lo, &scratch[..d_len]) {
            1
        } else {
            0
        };
        if highest_q && limbs_sub_same_length_in_place_left(&mut ns_lo[hi..], ds_lo) {
            carry += 1;
        }
        while carry != 0 {
            if limbs_sub_limb_in_place(&mut qs_hi[..hi], 1) {
                assert!(highest_q);
                highest_q = false;
            }
            if limbs_slice_add_same_length_in_place_left(ns_lo, ds) {
                carry -= 1;
            }
        }
    }
    let ds_hi = &ds[hi..];
    let ns_hi = &mut ns[hi - lo..];
    let q_lo = if lo < DC_DIVAPPR_Q_THRESHOLD {
        _limbs_div_mod_schoolbook_approx(qs, &mut ns_hi[..lo << 1], ds_hi, inverse)
    } else {
        _limbs_div_mod_divide_and_conquer_approx_helper(
            qs,
            &mut ns_hi[lo >> 1..],
            ds_hi,
            inverse,
            scratch,
        )
    };
    if q_lo {
        // TODO This branch is untested!
        for q in qs[..lo].iter_mut() {
            *q = Limb::MAX;
        }
    }
    highest_q
}

/// Recursive divide-and-conquer division, returning approximate quotient.
///
/// Divides `ns` by `ds` and writes the `ns.len()` - `ds.len()` least-significant quotient limbs to
/// `qs`. Returns the most significant limb of the quotient; `true` means 1 and `false` means 0. The
/// quotient is either correct, or one too large. `ds` must have length greater than 2, `ns` must be
/// at least as long as `ds`, and the most significant bit of `ds` must be set. `inverse` should be
/// the result of `limbs_two_limb_inverse_helper` applied to the two highest limbs of the
/// denominator.
///
/// Time: worst case O(n * log(n) ^ 2 * log(log(n)))
///
/// Additional memory: worst case O(n * log(n) ^ 2)
///
/// where n = `ds.len()`
///
/// # Panics
/// Panics if `ds` has length smaller than 6, `ns` is shorter than or the same length as `ds`, `qs`
/// has length less than `ns.len()` - `ds.len()`, or the last limb of `ds` does not have its highest
/// bit set.
///
/// This is mpn_dcpi1_divappr_q from mpn/generic/dcpi1_divappr_q.c.
pub fn _limbs_div_mod_divide_and_conquer_approx(
    qs: &mut [Limb],
    ns: &mut [Limb],
    ds: &[Limb],
    inverse: Limb,
) -> bool {
    let n_len = ns.len();
    let d_len = ds.len();
    assert!(d_len >= 6);
    assert!(n_len > d_len);
    let a = d_len - 1;
    assert!(ds[a].get_highest_bit());
    let b = d_len - 2;
    let q_len = n_len - d_len;
    let mut highest_q;
    if q_len >= d_len {
        let q_len_mod_d_len = {
            let mut m = (q_len + 1) % d_len;
            if m == 0 {
                m = d_len;
            }
            m
        };
        let mut scratch = vec![0; d_len];
        {
            let offset = q_len - q_len_mod_d_len;
            let ns = &mut ns[offset..];
            let qs = &mut qs[offset..];
            let r = d_len - q_len_mod_d_len;
            let (ds_lo, ds_hi) = ds.split_at(r);
            // Perform the typically smaller block first.
            if q_len_mod_d_len == 1 {
                // Handle highest_q up front, for simplicity.
                {
                    let ns = &mut ns[1..d_len + 1];
                    highest_q = limbs_cmp_same_length(ns, ds) >= Ordering::Equal;
                    if highest_q {
                        assert!(!limbs_sub_same_length_in_place_left(ns, ds,));
                    }
                }
                // A single iteration of schoolbook: One 3/2 division, followed by the bignum update
                // and adjustment.
                let n_2 = ns[d_len];
                let mut n_1 = ns[a];
                let mut n_0 = ns[b];
                let d_1 = ds[a];
                let d_0 = ds[b];
                assert!(n_2 < d_1 || (n_2 == d_1 && n_1 <= d_0));
                let mut q;
                if n_2 == d_1 && n_1 == d_0 {
                    // TODO This branch is untested!
                    q = Limb::MAX;
                    assert_eq!(
                        limbs_sub_mul_limb_same_length_in_place_left(&mut ns[..d_len], ds, q,),
                        n_2
                    );
                } else {
                    let (new_q, new_n) =
                        limbs_div_mod_three_limb_by_two_limb(n_2, n_1, n_0, d_1, d_0, inverse);
                    q = new_q;
                    let (new_n_1, new_n_0) = new_n.split_in_half();
                    n_1 = new_n_1;
                    n_0 = new_n_0;
                    // d_len > 2
                    let local_carry_1 =
                        limbs_sub_mul_limb_same_length_in_place_left(&mut ns[..b], &ds[..b], q);
                    let local_carry_2 = n_0 < local_carry_1;
                    n_0.wrapping_sub_assign(local_carry_1);
                    let carry = local_carry_2 && n_1 == 0;
                    if local_carry_2 {
                        n_1.wrapping_sub_assign(1);
                    }
                    ns[b] = n_0;
                    if carry {
                        // TODO This branch is untested!
                        n_1.wrapping_add_assign(d_1);
                        if limbs_slice_add_same_length_in_place_left(&mut ns[..a], &ds[..a]) {
                            n_1.wrapping_add_assign(1);
                        }
                        if q == 0 {
                            assert!(highest_q);
                            highest_q = false;
                        }
                        q.wrapping_sub_assign(1);
                    }
                    ns[a] = n_1;
                }
                qs[0] = q;
            } else {
                {
                    let ns_hi = &mut ns[r..];
                    highest_q = if q_len_mod_d_len == 2 {
                        limbs_div_mod_by_two_limb(qs, &mut ns_hi[..q_len_mod_d_len + 2], ds_hi)
                    } else if q_len_mod_d_len < DC_DIV_QR_THRESHOLD {
                        _limbs_div_mod_schoolbook(qs, ns_hi, ds_hi, inverse)
                    } else {
                        _limbs_div_mod_divide_and_conquer_helper(
                            qs,
                            ns_hi,
                            ds_hi,
                            inverse,
                            &mut scratch,
                        )
                    };
                }
                let qs = &mut qs[..q_len_mod_d_len];
                if q_len_mod_d_len != d_len {
                    limbs_mul_to_out(&mut scratch, qs, ds_lo);
                    let mut carry =
                        if limbs_sub_same_length_in_place_left(&mut ns[..d_len], &scratch[..d_len])
                        {
                            1
                        } else {
                            0
                        };
                    if highest_q {
                        if limbs_sub_same_length_in_place_left(
                            &mut ns[q_len_mod_d_len..d_len],
                            ds_lo,
                        ) {
                            carry += 1;
                        }
                    }
                    while carry != 0 {
                        if limbs_sub_limb_in_place(qs, 1) {
                            assert!(highest_q);
                            highest_q = false;
                        }
                        if limbs_slice_add_same_length_in_place_left(&mut ns[..d_len], ds) {
                            carry -= 1;
                        }
                    }
                }
            }
        }
        let mut offset = q_len.checked_sub(q_len_mod_d_len).unwrap();
        while offset >= d_len {
            offset -= d_len;
            _limbs_div_mod_divide_and_conquer_helper(
                &mut qs[offset..],
                &mut ns[offset..],
                ds,
                inverse,
                &mut scratch,
            );
        }
        // Since we pretended we'd need an extra quotient limb before, we now have made sure the
        // code above left just ds.len() - 1 = qs.len() quotient limbs to develop. Develop that plus
        // a guard limb.
        let ns = &mut ns[offset + (d_len >> 1) - d_len..];
        let q_save = qs[offset];
        _limbs_div_mod_divide_and_conquer_approx_helper(qs, ns, ds, inverse, &mut scratch);
        limbs_move_left(&mut qs[..offset + 1], 1);
        qs[offset] = q_save;
    } else {
        assert!(b >= q_len);
        let offset = b - q_len;
        let q_len_plus_one = q_len + 1;
        let mut qs_2 = vec![0; q_len_plus_one];
        let ns = &mut ns[offset..];
        let ds = &ds[offset + 1..];
        if q_len < DC_DIVAPPR_Q_THRESHOLD {
            highest_q = _limbs_div_mod_schoolbook_approx(&mut qs_2, ns, ds, inverse);
        } else {
            let mut scratch = vec![0; q_len_plus_one];
            highest_q = _limbs_div_mod_divide_and_conquer_approx_helper(
                &mut qs_2,
                &mut ns[q_len_plus_one >> 1..],
                ds,
                inverse,
                &mut scratch,
            );
        }
        qs[..q_len].copy_from_slice(&qs_2[1..]);
    }
    highest_q
}

//TODO tune
const MAYBE_DCP1_DIVAPPR: bool = true;

/// Takes the strictly normalised value ds (i.e., most significant bit must be set) as an input, and
/// computes the approximate reciprocal of ds, with the same length of ds. See documentation for
/// `_limbs_invert_approx` for an explanation of the return value.
///
/// Time: worst case O(n * log(n) ^ 2 * log(log(n)))
///
/// Additional memory: worst case O(n * log(n) ^ 2)
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
        is[0] = (DoubleLimb::join_halves(!d, Limb::MAX) / DoubleLimb::from(d)).lower_half()
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
            limbs_div_mod_by_two_limb(is, scratch, ds);
        } else {
            let inverse = limbs_two_limb_inverse_helper(highest_d, ds[d_len - 2]);
            if !MAYBE_DCP1_DIVAPPR || d_len < DC_DIVAPPR_Q_THRESHOLD {
                _limbs_div_mod_schoolbook_approx(is, scratch, ds, inverse);
            } else {
                _limbs_div_mod_divide_and_conquer_approx(is, scratch, ds, inverse);
            }
            assert!(!limbs_sub_limb_in_place(&mut is[..d_len], 1));
            return false;
        }
    }
    true
}

//TODO tune all
const INV_NEWTON_THRESHOLD: usize = 170;
const INV_MULMOD_BNM1_THRESHOLD: usize = 38;

/// Takes the strictly normalised value ds (i.e., most significant bit must be set) as an input, and
/// computes the approximate reciprocal of ds, with the same length of ds. See documentation for
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
/// Time: O(n * log(n) * log(log(n)))
///
/// Additional memory: O(n * log(n))
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
    let mut previous_d = d_len;
    let mut sizes = vec![previous_d];
    previous_d = (previous_d >> 1) + 1;
    let mut scratch2 = vec![];
    let mut mul_size = 0;
    if d_len >= INV_MULMOD_BNM1_THRESHOLD {
        mul_size = _limbs_mul_mod_limb_width_to_n_minus_1_next_size(d_len + 1);
        scratch2 =
            vec![
                0;
                _limbs_mul_mod_limb_width_to_n_minus_1_scratch_len(mul_size, d_len, previous_d)
            ];
    }
    while previous_d >= INV_NEWTON_THRESHOLD {
        sizes.push(previous_d);
        previous_d = (previous_d >> 1) + 1;
    }
    // We compute the inverse of 0.ds as 1.is.
    // Compute a base value of previous_d limbs.
    _limbs_invert_basecase_approx(
        &mut is[d_len - previous_d..],
        &ds[d_len - previous_d..],
        scratch,
    );
    // Use Newton's iterations to get the desired precision.
    for (i, &d) in sizes.iter().enumerate().rev() {
        // v    d       v
        // +----+-------+
        // ^ previous_d ^
        //
        // Compute i_j * d
        let ds_hi = &ds[d_len - d..];
        let condition = d < INV_MULMOD_BNM1_THRESHOLD || {
            mul_size = _limbs_mul_mod_limb_width_to_n_minus_1_next_size(d + 1);
            mul_size > d + previous_d
        };
        let diff = d - previous_d;
        {
            let is_hi = &mut is[d_len - previous_d..];
            if condition {
                limbs_mul_greater_to_out(scratch, ds_hi, is_hi);
                limbs_slice_add_same_length_in_place_left(
                    &mut scratch[previous_d..d + 1],
                    &ds_hi[..diff + 1],
                );
            // Remember we truncated mod B ^ (d + 1)
            // We computed (truncated) xp of length d + 1 <- 1.is * 0.ds
            } else {
                // Use B ^ mul_size - 1 wraparound
                _limbs_mul_mod_limb_width_to_n_minus_1(
                    scratch,
                    mul_size,
                    ds_hi,
                    is_hi,
                    &mut scratch2,
                );
                let scratch = &mut scratch[..mul_size + 1];
                // We computed {xp, mul_size} <- {is, previous_d} * {ds, d} mod (B ^ mul_size - 1)
                // We know that 2 * |is * ds + ds * B ^ previous_d - B ^ {previous_d + d}| <
                //      B ^ mul_size - 1
                // Add ds * B ^ previous_d mod (B ^ mul_size - 1)
                let mul_diff = mul_size - previous_d;
                assert!(d >= mul_diff);
                let (ds_hi_lo, ds_hi_hi) = ds_hi.split_at(mul_diff);
                let carry = limbs_slice_add_same_length_in_place_left(
                    &mut scratch[previous_d..mul_size],
                    ds_hi_lo,
                );
                // Subtract B ^ {previous_d + d}, maybe only compensate the carry
                scratch[mul_size] = 1; // set a limit for decrement
                {
                    let (scratch_lo, scratch_hi) = scratch.split_at_mut(d - mul_diff);
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
            if scratch[d] < 2 {
                // "positive" residue class
                let (scratch_lo, scratch_hi) = scratch.split_at_mut(d);
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
                assert!(scratch[d] >= Limb::MAX - 1);
                if condition {
                    assert!(!limbs_sub_limb_in_place(&mut scratch[..d + 1], 1));
                }
                let (scratch_lo, scratch_hi) = scratch.split_at_mut(d);
                if scratch_hi[0] != Limb::MAX {
                    assert!(!limbs_slice_add_limb_in_place(is_hi, 1));
                    assert!(limbs_slice_add_same_length_in_place_left(scratch_lo, ds_hi));
                }
                limbs_not_to_out(&mut scratch_hi[diff..d], &scratch_lo[diff..]);
            }
            // Compute x_j * u_j
            let (scratch_lo, scratch_hi) = scratch.split_at_mut(d + diff);
            limbs_mul_same_length_to_out(scratch_lo, &scratch_hi[..previous_d], is_hi);
        }
        let a = (previous_d << 1) - diff;
        let cy = {
            let (scratch_lo, scratch_hi) = scratch.split_at_mut(a);
            limbs_slice_add_same_length_in_place_left(
                &mut scratch_lo[previous_d..],
                &scratch_hi[3 * diff - previous_d..diff << 1],
            )
        };
        if _limbs_add_same_length_with_carry_in_to_out(
            &mut is[d_len - d..],
            &scratch[a..previous_d << 1],
            &scratch[d + previous_d..d << 1],
            cy,
        ) {
            assert!(!limbs_slice_add_limb_in_place(
                &mut is[d_len - previous_d..],
                1
            ));
        }
        if i == 0 {
            // Check for possible carry propagation from below. Be conservative.
            return scratch[a - 1] <= Limb::MAX - 7;
        }
        previous_d = d;
    }
    // The preceding loop always returns when i == 0. Since sizes is nonempty, this always happens.
    unreachable!();
}

/// Takes the strictly normalised value ds (i.e., most significant bit must be set) as an input, and
/// computes the approximate reciprocal of ds, with the same length of ds.
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
/// Time: O(n * log(n) * log(log(n)))
///
/// Additional memory: O(n * log(n))
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

const MUL_TO_MULMOD_BNM1_FOR_2NXN_THRESHOLD: usize = INV_MULMOD_BNM1_THRESHOLD >> 1;

/// This is mpn_preinv_mu_div_qr from mpn/generic/mu_div_qr.c.
fn _limbs_div_mod_barrett_preinverted(
    qs: &mut [Limb],
    rs: &mut [Limb],
    ns: &[Limb],
    ds: &[Limb],
    is: &[Limb],
    scratch: &mut [Limb],
) -> bool {
    let n_len = ns.len();
    let d_len = ds.len();
    // rs has length d_len as well
    let mut i_len = is.len();
    let mut q_len = n_len - d_len;
    let mut np_offset = q_len;
    let mut qp_offset = q_len;
    let qh =
        limbs_cmp_same_length(&ns[np_offset..np_offset + d_len], &ds[..d_len]) >= Ordering::Equal;
    if qh {
        limbs_sub_same_length_to_out(rs, &ns[np_offset..np_offset + d_len], &ds[..d_len]);
    } else {
        rs[..d_len].copy_from_slice(&ns[np_offset..np_offset + d_len]);
    }
    let mut ip_offset = 0;
    while q_len != 0 {
        if q_len < i_len {
            ip_offset += i_len - q_len;
            i_len = q_len;
        }
        np_offset -= i_len;
        qp_offset -= i_len;
        // Compute the next block of quotient limbs by multiplying the inverse I
        // by the upper part of the partial remainder R.
        // mulhi
        limbs_mul_same_length_to_out(
            scratch,
            &rs[d_len - i_len..d_len],
            &is[ip_offset..ip_offset + i_len],
        );
        // I's msb implicit
        let cy = limbs_add_same_length_to_out(
            &mut qs[qp_offset..],
            &scratch[i_len..2 * i_len],
            &rs[d_len - i_len..d_len],
        );
        assert!(!cy);
        q_len -= i_len;
        // Compute the product of the quotient block and the divisor D, to be
        // subtracted from the partial remainder combined with new limbs from the
        // dividend N.  We only really need the low d_len+1 limbs.
        if i_len < MUL_TO_MULMOD_BNM1_FOR_2NXN_THRESHOLD {
            // d_len+in limbs, high 'in' cancels
            limbs_mul_greater_to_out(scratch, &ds[..d_len], &qs[qp_offset..qp_offset + i_len]);
        } else {
            let tn = _limbs_mul_mod_limb_width_to_n_minus_1_next_size(d_len + 1);
            let (tp, scratch_out) = scratch.split_at_mut(tn);
            _limbs_mul_mod_limb_width_to_n_minus_1(
                tp,
                tn,
                &ds[..d_len],
                &qs[qp_offset..qp_offset + i_len],
                scratch_out,
            );
            // number of wrapped limbs
            let wn = isize::checked_from(d_len + i_len).unwrap() - isize::checked_from(tn).unwrap();
            if wn > 0 {
                let wn = usize::checked_from(wn).unwrap();
                let mut cy =
                    if limbs_sub_same_length_in_place_left(&mut tp[..wn], &rs[d_len - wn..d_len]) {
                        1
                    } else {
                        0
                    };
                cy = if limbs_sub_limb_in_place(&mut tp[wn..tn], cy) {
                    1
                } else {
                    0
                };
                let cx = if limbs_cmp_same_length(&rs[d_len - i_len..tn - i_len], &tp[d_len..tn])
                    == Ordering::Less
                {
                    1
                } else {
                    0
                };
                assert!(cx >= cy);
                assert!(!limbs_slice_add_limb_in_place(tp, cx - cy));
            }
        }
        let mut r = rs[d_len - i_len].wrapping_sub(scratch[d_len]);
        // Subtract the product from the partial remainder combined with new
        // limbs from the dividend N, generating a new partial remainder R.
        let mut cy;
        if d_len != i_len {
            // get next 'in' limbs from N
            cy = if limbs_sub_same_length_in_place_right(
                &ns[np_offset..np_offset + i_len],
                &mut scratch[..i_len],
            ) {
                1
            } else {
                0
            };
            cy = if _limbs_sub_same_length_with_borrow_in_in_place_right(
                &rs[..d_len - i_len],
                &mut scratch[i_len..d_len],
                cy != 0,
            ) {
                1
            } else {
                0
            };
            rs[..d_len].copy_from_slice(&scratch[..d_len]);
        } else {
            // get next 'in' limbs from N
            cy = if limbs_sub_same_length_to_out(
                rs,
                &ns[np_offset..np_offset + i_len],
                &scratch[..i_len],
            ) {
                1
            } else {
                0
            };
        }
        // Check the remainder R and adjust the quotient as needed.
        r -= cy;
        while r != 0 {
            // We loop 0 times with about 69% probability, 1 time with about 31%
            // probability, 2 times with about 0.6% probability, if inverse is
            // computed as recommended.
            assert!(!limbs_slice_add_limb_in_place(&mut qs[qp_offset..], 1));
            cy = if limbs_sub_same_length_in_place_left(&mut rs[..d_len], &ds[..d_len]) {
                1
            } else {
                0
            };
            r -= cy;
        }
        if limbs_cmp_same_length(&rs[..d_len], &ds[..d_len]) >= Ordering::Equal {
            // This is executed with about 76% probability.
            assert!(!limbs_slice_add_limb_in_place(&mut qs[qp_offset..], 1));
            limbs_sub_same_length_in_place_left(&mut rs[..d_len], &ds[..d_len]);
        }
    }
    qh
}

/// We distinguish 3 cases:
/// (a) d_len < q_len:              in = ceil(q_len / ceil(q_len / d_len))
/// (b) d_len / 3 < q_len <= d_len: in = ceil(q_len / 2)
/// (c) q_len < d_len / 3:          in = q_len
/// In all cases we have in <= d_len.
///
/// This is mpn_mu_div_qr_choose_in from mpn/generic/mu_div_qr.c, where k == 0.
fn _limbs_div_mod_barrett_is_len(q_len: usize, d_len: usize) -> usize {
    if q_len > d_len {
        // Compute an inverse size that is a nice partition of the quotient.
        let b = (q_len - 1) / d_len + 1; // ceil(q_len / d_len), number of blocks
        (q_len - 1) / b + 1 // ceil(q_len / b) = ceil(q_len / ceil(q_len / d_len))
    } else if 3 * q_len > d_len {
        (q_len - 1) / 2 + 1 // b = 2
    } else {
        (q_len - 1) + 1 // b = 1
    }
}

/// This is mpn_mu_div_qr2 from mpn/generic/mu_div_qr.c.
fn _limbs_div_mod_barrett_helper(
    qs: &mut [Limb],
    rs: &mut [Limb],
    ns: &[Limb],
    ds: &[Limb],
    scratch: &mut [Limb],
) -> bool {
    let n_len = ns.len();
    let d_len = ds.len();
    assert!(d_len > 1);
    assert!(n_len > d_len);
    let q_len = n_len - d_len;
    // Compute the inverse size.
    let i_len = _limbs_div_mod_barrett_is_len(q_len, d_len);
    assert!(i_len <= d_len);
    {
        let (ip, tp) = scratch.split_at_mut(i_len + 1);
        // compute an approximate inverse on (in+1) limbs
        if d_len == i_len {
            tp[1..i_len + 1].copy_from_slice(&ds[..i_len]);
            tp[0] = 1;
            let (tp_lo, tp_hi) = tp.split_at_mut(i_len + 1);
            _limbs_invert_approx(ip, &tp_lo, tp_hi);
            limbs_move_left(ip, 1);
        } else {
            let cy = limbs_add_limb_to_out(tp, &ds[d_len - (i_len + 1)..d_len], 1);
            if cy {
                limbs_set_zero(&mut ip[..i_len]);
            } else {
                let (tp_lo, tp_hi) = tp.split_at_mut(i_len + 1);
                _limbs_invert_approx(ip, tp_lo, tp_hi);
                limbs_move_left(ip, 1);
            }
        }
    }
    let (scratch_lo, scratch_hi) = scratch.split_at_mut(i_len);
    _limbs_div_mod_barrett_preinverted(qs, &mut rs[..d_len], ns, ds, scratch_lo, scratch_hi)
}

//TODO tune
const MU_DIV_QR_SKEW_THRESHOLD: usize = 100;

/// This is mpn_preinv_mu_div_qr_itch from mpn/generic/mu_div_qr.c, but nn is omitted from the
/// arguments as it is unused.
fn _limbs_div_mod_barrett_preinverse_scratch_len(d_len: usize, is_len: usize) -> usize {
    let itch_local = _limbs_mul_mod_limb_width_to_n_minus_1_next_size(d_len + 1);
    let itch_out = _limbs_mul_mod_limb_width_to_n_minus_1_scratch_len(itch_local, d_len, is_len);
    itch_local + itch_out
}

/// This is mpn_invertappr_itch from gmp-impl.h.
const fn _limbs_invert_approx_scratch_len(is_len: usize) -> usize {
    2 * is_len
}

/// This is mpn_mu_div_qr_itch from mpn/generic/mu_div_qr.c, where mua_k == 0.
pub fn _limbs_div_mod_barrett_scratch_len(n_len: usize, d_len: usize) -> usize {
    let is_len = _limbs_div_mod_barrett_is_len(n_len - d_len, d_len);
    let itch_preinv = _limbs_div_mod_barrett_preinverse_scratch_len(d_len, is_len);
    let itch_invapp = _limbs_invert_approx_scratch_len(is_len + 1) + is_len + 2; // 3 * is_len + 4
    assert!(itch_preinv >= itch_invapp);
    is_len + max(itch_invapp, itch_preinv)
}

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
    let mut highest_q;
    if q_len + MU_DIV_QR_SKEW_THRESHOLD < d_len {
        highest_q = _limbs_div_mod_barrett_helper(
            qs,
            &mut rs[n_len - (2 * q_len + 1)..],
            &ns[n_len - (2 * q_len + 1)..n_len],
            &ds[d_len - (q_len + 1)..d_len],
            scratch,
        );
        // Multiply the quotient by the divisor limbs ignored above.
        // prod is d_len-1 limbs
        limbs_mul_to_out(scratch, &ds[..d_len - (q_len + 1)], &qs[..q_len]);
        let mut cy = if highest_q {
            limbs_slice_add_same_length_in_place_left(
                &mut scratch[q_len..d_len - 1],
                &ds[..d_len - (q_len + 1)],
            )
        } else {
            false
        };
        scratch[d_len - 1] = if cy { 1 } else { 0 };
        cy = limbs_sub_same_length_to_out(
            rs,
            &ns[..n_len - (2 * q_len + 1)],
            &scratch[..n_len - (2 * q_len + 1)],
        );
        cy = _limbs_sub_same_length_with_borrow_in_in_place_left(
            &mut rs[n_len - (2 * q_len + 1)..n_len - q_len],
            &scratch[n_len - (2 * q_len + 1)..n_len - q_len],
            cy,
        );
        if cy {
            if limbs_sub_limb_in_place(&mut qs[..q_len], 1) {
                assert!(highest_q);
                highest_q = false;
            }
            limbs_slice_add_same_length_in_place_left(&mut rs[..d_len], &ds[..d_len]);
        }
    } else {
        highest_q = _limbs_div_mod_barrett_helper(qs, rs, &ns[..n_len], &ds[..d_len], scratch);
    }
    highest_q
}

//TODO tune all
const MUPI_DIV_QR_THRESHOLD: usize = 74;
const MU_DIV_QR_THRESHOLD: usize = 1442;

// dn > 1
pub fn mpn_tdiv_qr(qp: &mut [Limb], rp: &mut [Limb], np: &[Limb], dp: &[Limb]) {
    let mut nn = np.len();
    let dn = dp.len();
    assert!(dn > 1 && dp[dn - 1] != 0);
    match dn {
        2 => {
            if !dp[1].get_highest_bit() {
                let cnt = dp[1].leading_zeros();
                let dtmp = &mut [0; 2];
                let d2p = dtmp;
                d2p[1] = (dp[1] << cnt) | (dp[0] >> (Limb::WIDTH - cnt));
                d2p[0] = dp[0] << cnt;
                let mut n2p = vec![0; nn + 1];
                let cy = limbs_shl_to_out(&mut n2p, np, cnt);
                n2p[nn] = cy;
                let qhl = limbs_div_mod_by_two_limb(
                    qp,
                    if cy != 0 {
                        &mut n2p[..nn + 1]
                    } else {
                        &mut n2p[..nn]
                    },
                    d2p,
                );
                if cy == 0 {
                    // always store nn-2+1 quotient limbs
                    qp[nn - 2] = if qhl { 1 } else { 0 };
                }
                rp[0] = (n2p[0] >> cnt) | (n2p[1] << (Limb::WIDTH - cnt));
                rp[1] = n2p[1] >> cnt;
            } else {
                let d2p = dp;
                let mut n2p = vec![0; nn];
                n2p.copy_from_slice(np);
                let qhl = limbs_div_mod_by_two_limb(qp, &mut n2p, d2p);
                // always store nn-2+1 quotient limbs
                qp[nn - 2] = if qhl { 1 } else { 0 };
                rp[0] = n2p[0];
                rp[1] = n2p[1];
            }
        }
        _ => {
            // conservative tests for quotient size
            let adjust = if np[nn - 1] >= dp[dn - 1] { 1 } else { 0 };
            if nn + adjust >= 2 * dn {
                qp[nn - dn] = 0; // zero high quotient limb
                let mut n2p_orig;
                let mut d2p_orig;
                let mut n2p: &mut [Limb];
                let d2p: &[Limb];
                let cnt;
                if !dp[dn - 1].get_highest_bit() {
                    // normalize divisor
                    cnt = dp[dn - 1].leading_zeros();
                    d2p_orig = vec![0; dn];
                    limbs_shl_to_out(&mut d2p_orig, dp, cnt);
                    d2p = &d2p_orig;
                    n2p_orig = vec![0; nn + 1];
                    n2p = &mut n2p_orig;
                    let cy = limbs_shl_to_out(&mut n2p, np, cnt);
                    n2p[nn] = cy;
                    nn += adjust;
                } else {
                    cnt = 0;
                    d2p = dp;
                    n2p_orig = vec![0; nn + 1];
                    n2p = &mut n2p_orig;
                    n2p[0..nn].copy_from_slice(np);
                    n2p[nn] = 0;
                    nn += adjust;
                }
                let dinv = limbs_two_limb_inverse_helper(d2p[dn - 1], d2p[dn - 2]);
                if dn < DC_DIV_QR_THRESHOLD {
                    _limbs_div_mod_schoolbook(qp, &mut n2p[..nn], d2p, dinv);
                    if cnt != 0 {
                        limbs_shr_to_out(rp, &n2p[..dn], cnt);
                    } else {
                        rp[..dn].copy_from_slice(&n2p[..dn]);
                    }
                } else if dn < MUPI_DIV_QR_THRESHOLD ||   // fast condition
             nn < 2 * MU_DIV_QR_THRESHOLD || // fast condition
             (2 * (MU_DIV_QR_THRESHOLD - MUPI_DIV_QR_THRESHOLD)) as f64 * dn as f64 // slow...
             + MUPI_DIV_QR_THRESHOLD as f64 * nn as f64 > dn as f64 * nn as f64
                {
                    // ...condition
                    _limbs_div_mod_divide_and_conquer(qp, &mut n2p[..nn], &d2p[..dn], dinv);

                    if cnt != 0 {
                        limbs_shr_to_out(rp, &n2p[..dn], cnt);
                    } else {
                        rp[..dn].copy_from_slice(&n2p[..dn]);
                    }
                } else {
                    let itch = _limbs_div_mod_barrett_scratch_len(nn, dn);
                    let mut scratch = vec![0; itch];
                    _limbs_div_mod_barrett(qp, rp, &n2p[..nn], &d2p[..dn], &mut scratch);
                    if cnt != 0 {
                        limbs_slice_shr_in_place(&mut rp[..dn], cnt);
                    }
                }
                return;
            }

            // When we come here, the numerator/partial remainder is less
            // than twice the size of the denominator.

            //  Problem:

            //  Divide a numerator N with nn limbs by a denominator D with dn
            //  limbs forming a quotient of qn=nn-dn+1 limbs.  When qn is small
            //  compared to dn, conventional division algorithms perform poorly.
            //  We want an algorithm that has an expected running time that is
            //  dependent only on qn.

            //  Algorithm (very informally stated):

            //  1) Divide the 2 x qn most significant limbs from the numerator
            // by the qn most significant limbs from the denominator.  Call
            // the result qest.  This is either the correct quotient, but
            // might be 1 or 2 too large.  Compute the remainder from the
            // division.  (This step is implemented by an mpn_divrem call.)

            //  2) Is the most significant limb from the remainder < p, where p
            // is the product of the most significant limb from the quotient
            // and the next(d)?  (Next(d) denotes the next ignored limb from
            // the denominator.)  If it is, decrement qest, and adjust the
            // remainder accordingly.

            //  3) Is the remainder >= qest?  If it is, qest is the desired
            // quotient.  The algorithm terminates.

            //  4) Subtract qest x next(d) from the remainder.  If there is
            // borrow out, decrement qest, and adjust the remainder
            // accordingly.

            //  5) Skip one word from the denominator (i.e., let next(d) denote
            // the next less significant limb.

            let mut qn = nn - dn;
            qp[qn] = 0; // zero high quotient limb
            qn += adjust; // qn cannot become bigger

            if qn == 0 {
                rp[..dn].copy_from_slice(&np[..dn]);
                return;
            }

            // (at least partially) ignored # of limbs in ops
            // Normalize denominator by shifting it to the left such that its
            // most significant bit is set.  Then shift the numerator the same
            // amount, to mathematically preserve quotient.
            let mut ilen = dn - qn;
            let mut n2p_orig;
            let mut d2p_orig;
            let n2p: &mut [Limb];
            let d2p: &[Limb];
            let cnt;
            if !dp[dn - 1].get_highest_bit() {
                cnt = dp[dn - 1].leading_zeros();
                d2p_orig = vec![0; qn];
                limbs_shl_to_out(&mut d2p_orig, &dp[ilen..ilen + qn], cnt);
                d2p_orig[0] |= dp[ilen - 1] >> (Limb::WIDTH - cnt);
                d2p = &d2p_orig;
                n2p_orig = vec![0; 2 * qn + 1];
                let cy = limbs_shl_to_out(&mut n2p_orig, &np[nn - 2 * qn..nn], cnt);
                if adjust != 0 {
                    n2p_orig[2 * qn] = cy;
                    n2p = &mut n2p_orig[1..];
                } else {
                    n2p = &mut n2p_orig;
                    n2p[0] |= np[nn - 2 * qn - 1] >> (Limb::WIDTH - cnt);
                }
            } else {
                cnt = 0;
                d2p = &dp[ilen..];

                n2p_orig = vec![0; 2 * qn + 1];
                n2p_orig[..2 * qn].copy_from_slice(&np[nn - 2 * qn..nn]);
                if adjust != 0 {
                    n2p_orig[2 * qn] = 0;
                    n2p = &mut n2p_orig[1..];
                } else {
                    n2p = &mut n2p_orig;
                }
            }

            // Get an approximate quotient using the extracted operands.
            if qn == 1 {
                let mut q0 = 0;
                let mut r0 = 0;
                udiv_qrnnd(&mut q0, &mut r0, n2p[1], n2p[0], d2p[0]);
                n2p[0] = r0;
                qp[0] = q0;
            } else if qn == 2 {
                limbs_div_mod_by_two_limb(qp, n2p, d2p);
            } else {
                let dinv = limbs_two_limb_inverse_helper(d2p[qn - 1], d2p[qn - 2]);
                if qn < DC_DIV_QR_THRESHOLD {
                    _limbs_div_mod_schoolbook(qp, &mut n2p[..2 * qn], &d2p[..qn], dinv);
                } else if qn < MU_DIV_QR_THRESHOLD {
                    _limbs_div_mod_divide_and_conquer(qp, &mut n2p[..2 * qn], &d2p[..qn], dinv);
                } else {
                    let itch = _limbs_div_mod_barrett_scratch_len(2 * qn, qn);
                    let mut scratch = vec![0; itch];
                    // If N and R share space, put ...
                    // intermediate remainder at N's upper end.
                    // if np == r2p {
                    //     r2p += nn - qn;
                    // }
                    _limbs_div_mod_barrett(qp, rp, &n2p[..2 * qn], &d2p[..qn], &mut scratch);
                    n2p[..qn].copy_from_slice(&rp[..qn]);
                }
            }

            let mut rn = qn;
            // Multiply the first ignored divisor limb by the most significant
            // quotient limb.  If that product is > the partial remainder's
            // most significant limb, we know the quotient is too large.  This
            // test quickly catches most cases where the quotient is too large;
            // it catches all cases where the quotient is 2 too large.

            let dl = if isize::checked_from(ilen).unwrap() - 2 < 0 {
                0
            } else {
                dp[ilen - 2]
            };
            let x = (dp[ilen - 1] << cnt) | ((dl >> 1) >> ((!cnt) & Limb::WIDTH_MASK));
            let mut h = 0;
            let mut dummy = 0;
            umul_ppmm(&mut h, &mut dummy, x, qp[qn - 1]);

            if n2p[qn - 1] < h {
                assert!(!limbs_sub_limb_in_place(qp, 1));
                let cy = limbs_slice_add_same_length_in_place_left(&mut n2p[..qn], &d2p[..qn]);
                if cy {
                    // The partial remainder is safely large.
                    n2p[qn] = if cy { 1 } else { 0 };
                    rn += 1;
                }
            }

            let mut quotient_too_large = false;
            if cnt != 0 {
                // Append partially used numerator limb to partial remainder.
                let cy1 = limbs_slice_shl_in_place(&mut n2p[..rn], Limb::WIDTH - cnt);
                n2p[0] |= np[ilen - 1] & (Limb::MAX >> cnt);

                // Update partial remainder with partially used divisor limb.
                let cy2 = limbs_sub_mul_limb_same_length_in_place_left(
                    &mut n2p[..qn],
                    &qp[..qn],
                    dp[ilen - 1] & (Limb::MAX >> cnt),
                );
                if qn != rn {
                    assert!(n2p[qn] >= cy2);
                    n2p[qn].wrapping_sub_assign(cy2);
                } else {
                    n2p[qn] = cy1.wrapping_sub(cy2);
                    quotient_too_large = cy1 < cy2;
                    rn += 1;
                }
                ilen -= 1;
            }
            // True: partial remainder now is neutral, i.e., it is not shifted up.

            let mut tp = vec![0; dn];

            let mut goto_foo = false;
            if ilen < qn {
                if ilen == 0 {
                    rp[..rn].copy_from_slice(&n2p[..rn]);
                    assert_eq!(rn, dn);
                    goto_foo = true;
                } else {
                    limbs_mul_greater_to_out(&mut tp, &qp[..qn], &dp[..ilen]);
                }
            } else {
                limbs_mul_greater_to_out(&mut tp, &dp[..ilen], &qp[..qn]);
            }
            if !goto_foo {
                let mut cy = limbs_sub_in_place_left(&mut n2p[..rn], &tp[ilen..ilen + qn]);
                rp[ilen..dn].copy_from_slice(&n2p[..dn - ilen]);
                quotient_too_large |= cy;
                cy = limbs_sub_same_length_to_out(rp, &np[..ilen], &tp[..ilen]);
                cy = limbs_sub_limb_in_place(
                    &mut rp[ilen..min(dp.len(), ilen + rn)],
                    if cy { 1 } else { 0 },
                );
                quotient_too_large |= cy;
            }
            if quotient_too_large {
                assert!(!limbs_sub_limb_in_place(qp, 1));
                limbs_slice_add_same_length_in_place_left(&mut rp[..dn], &dp[..dn]);
            }
        }
    }
}

impl DivMod<Natural> for Natural {
    type DivOutput = Natural;
    type ModOutput = Natural;

    #[inline]
    fn div_mod(mut self, other: Natural) -> (Natural, Natural) {
        let remainder = self.div_assign_mod(other);
        (self, remainder)
    }
}

impl<'a> DivMod<&'a Natural> for Natural {
    type DivOutput = Natural;
    type ModOutput = Natural;

    #[inline]
    fn div_mod(mut self, other: &'a Natural) -> (Natural, Natural) {
        let remainder = self.div_assign_mod(other);
        (self, remainder)
    }
}

impl<'a> DivMod<Natural> for &'a Natural {
    type DivOutput = Natural;
    type ModOutput = Natural;

    #[inline]
    fn div_mod(self, other: Natural) -> (Natural, Natural) {
        //TODO
        let mut x = self.clone();
        let remainder = x.div_assign_mod(other);
        (x, remainder)
    }
}

impl<'a, 'b> DivMod<&'b Natural> for &'a Natural {
    type DivOutput = Natural;
    type ModOutput = Natural;

    #[inline]
    fn div_mod(self, other: &'b Natural) -> (Natural, Natural) {
        //TODO
        let mut x = self.clone();
        let remainder = x.div_assign_mod(other);
        (x, remainder)
    }
}

impl DivAssignMod<Natural> for Natural {
    type ModOutput = Natural;

    fn div_assign_mod(&mut self, other: Natural) -> Natural {
        //TODO
        self.div_assign_mod(&other)
    }
}

impl<'a> DivAssignMod<&'a Natural> for Natural {
    type ModOutput = Natural;

    /// Divides a `Natural` by a `Limb` in place, returning the remainder. The quotient is rounded
    /// towards negative infinity. The quotient and remainder satisfy `self` = q * `other` + r and
    /// 0 <= r < `other`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `other.significant_bits()`
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
            let (qs, rs) = match (&mut *self, other) {
                (x, &Small(y)) => {
                    return Small(x.div_assign_mod(y));
                }
                (&mut Small(mut x), y) => {
                    return Small(x.div_assign_mod(y));
                }
                (&mut Large(ref mut xs), Large(ref ys)) => {
                    let mut qp = vec![0; xs.len() - ys.len() + 1];
                    let mut rp = vec![0; ys.len()];
                    mpn_tdiv_qr(&mut qp, &mut rp, xs, ys);
                    (qp, rp)
                }
            };
            let mut q = Large(qs);
            q.trim();
            *self = q;
            let mut r = Large(rs);
            r.trim();
            r
        }
    }
}

impl DivRem<Natural> for Natural {
    type DivOutput = Natural;
    type RemOutput = Natural;

    #[inline]
    fn div_rem(self, other: Natural) -> (Natural, Natural) {
        self.div_mod(other)
    }
}

impl<'a> DivRem<&'a Natural> for Natural {
    type DivOutput = Natural;
    type RemOutput = Natural;

    #[inline]
    fn div_rem(self, other: &'a Natural) -> (Natural, Natural) {
        self.div_mod(other)
    }
}

impl<'a> DivRem<Natural> for &'a Natural {
    type DivOutput = Natural;
    type RemOutput = Natural;

    #[inline]
    fn div_rem(self, other: Natural) -> (Natural, Natural) {
        self.div_mod(other)
    }
}

impl<'a, 'b> DivRem<&'b Natural> for &'a Natural {
    type DivOutput = Natural;
    type RemOutput = Natural;

    #[inline]
    fn div_rem(self, other: &'b Natural) -> (Natural, Natural) {
        self.div_mod(other)
    }
}

impl DivAssignRem<Natural> for Natural {
    type RemOutput = Natural;

    #[inline]
    fn div_assign_rem(&mut self, other: Natural) -> Natural {
        self.div_assign_mod(other)
    }
}

impl<'a> DivAssignRem<&'a Natural> for Natural {
    type RemOutput = Natural;

    #[inline]
    fn div_assign_rem(&mut self, other: &'a Natural) -> Natural {
        self.div_assign_mod(other)
    }
}

impl CeilingDivNegMod<Natural> for Natural {
    type DivOutput = Natural;
    type ModOutput = Natural;

    #[inline]
    fn ceiling_div_neg_mod(mut self, other: Natural) -> (Natural, Natural) {
        let remainder = self.ceiling_div_assign_neg_mod(other);
        (self, remainder)
    }
}

impl<'a> CeilingDivNegMod<&'a Natural> for Natural {
    type DivOutput = Natural;
    type ModOutput = Natural;

    #[inline]
    fn ceiling_div_neg_mod(mut self, other: &'a Natural) -> (Natural, Natural) {
        let remainder = self.ceiling_div_assign_neg_mod(other);
        (self, remainder)
    }
}

impl<'a> CeilingDivNegMod<Natural> for &'a Natural {
    type DivOutput = Natural;
    type ModOutput = Natural;

    #[inline]
    fn ceiling_div_neg_mod(self, other: Natural) -> (Natural, Natural) {
        //TODO
        let mut x = self.clone();
        let remainder = x.ceiling_div_assign_neg_mod(other);
        (x, remainder)
    }
}

impl<'a, 'b> CeilingDivNegMod<&'b Natural> for &'a Natural {
    type DivOutput = Natural;
    type ModOutput = Natural;

    #[inline]
    fn ceiling_div_neg_mod(self, other: &'b Natural) -> (Natural, Natural) {
        //TODO
        let mut x = self.clone();
        let remainder = x.ceiling_div_assign_neg_mod(other);
        (x, remainder)
    }
}

impl CeilingDivAssignNegMod<Natural> for Natural {
    type ModOutput = Natural;

    #[inline]
    fn ceiling_div_assign_neg_mod(&mut self, other: Natural) -> Natural {
        //TODO
        self.ceiling_div_assign_neg_mod(&other)
    }
}

impl<'a> CeilingDivAssignNegMod<&'a Natural> for Natural {
    type ModOutput = Natural;

    fn ceiling_div_assign_neg_mod(&mut self, other: &'a Natural) -> Natural {
        //TODO
        let remainder = self.div_assign_mod(other);
        if remainder == 0 as Limb {
            Natural::ZERO
        } else {
            *self += 1 as Limb;
            other - remainder
        }
    }
}
