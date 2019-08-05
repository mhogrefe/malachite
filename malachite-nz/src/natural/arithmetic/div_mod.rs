use std::cmp::{max, min, Ordering};
use std::mem::{size_of, swap};

use malachite_base::comparison::Max;
use malachite_base::limbs::{limbs_move_left, limbs_set_zero};
use malachite_base::num::arithmetic::traits::{
    CeilingDivAssignNegMod, CeilingDivNegMod, CeilingLogTwo, DivAssignMod, DivAssignRem, DivMod,
    DivRem, WrappingAddAssign, WrappingSub, WrappingSubAssign,
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
    _limbs_mul_mod_limb_width_to_n_minus_1_scratch_size,
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
                    // TODO This branch is untested!
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

//TODO test
// checked
// docs preserved
// In case k == 0 (automatic choice), we distinguish 3 cases:
// (a) dn < qn:           in = ceil(qn / ceil(qn / dn))
// (b) dn / 3 < qn <= dn: in = ceil(qn / 2)
// (c) qn < dn / 3:       in = qn
// In all cases we have in <= dn.
// mpn_mu_div_qr_choose_in from mpn/generic/mu_div_qr.c
pub fn mpn_mu_div_qr_choose_in(qn: usize, dn: usize, k: Limb) -> usize {
    if k == 0 {
        if qn > dn {
            // Compute an inverse size that is a nice partition of the quotient.
            let b = (qn - 1) / dn + 1; // ceil(qn / dn), number of blocks
            (qn - 1) / b + 1 // ceil(qn / b) = ceil(qn / ceil(qn / dn))
        } else if 3 * qn > dn {
            (qn - 1) / 2 + 1 // b = 2
        } else {
            (qn - 1) + 1 // b = 1
        }
    } else {
        let xn = min(dn, qn);
        (xn - 1) / usize::checked_from(k).unwrap() + 1
    }
}

//TODO test
// checked
// docs preserved
// mpn_preinv_mu_div_qr_itch from mpn/generic/mu_div_qr.c
pub fn mpn_preinv_mu_div_qr_itch(dn: usize, in_size: usize) -> usize {
    let itch_local = _limbs_mul_mod_limb_width_to_n_minus_1_next_size(dn + 1);
    let itch_out = _limbs_mul_mod_limb_width_to_n_minus_1_scratch_size(itch_local, dn, in_size);
    itch_local + itch_out
}

//TODO test
// checked
// docs preserved
// mpn_invertappr_itch from gmp-impl.h
pub fn mpn_invertappr_itch(n: usize) -> usize {
    2 * n
}

//TODO test
// checked
// docs preserved
// mpn_mu_div_qr_itch from mpn/generic/mu_div_qr.c
pub fn mpn_mu_div_qr_itch(nn: usize, dn: usize, mua_k: Limb) -> usize {
    let in_size = mpn_mu_div_qr_choose_in(nn - dn, dn, mua_k);
    let itch_preinv = mpn_preinv_mu_div_qr_itch(dn, in_size);
    let itch_invapp = mpn_invertappr_itch(in_size + 1) + in_size + 2; // 3 * in_size + 4

    assert!(itch_preinv >= itch_invapp);
    in_size + max(itch_invapp, itch_preinv)
}

//TODO tune
const MAYBE_DCP1_DIVAPPR: bool = true;

//TODO test
// docs preserved
// mpn_bc_invertappr (ip, dp, scratch), takes the strictly normalised value dp (i.e., most
// significant bit must be set) as an input, and computes ip of length n: the approximate reciprocal
// of dp.
//
// Let e = mpn_bc_invertappr(ip, dp, scratch) be the returned value; the following conditions are
// satisfied by the output:
//   a) 0 <= e <= 1
//   b) dp * (B ^ n + ip) < B ^ {2n} <= dp * (B ^ n + ip + 1 + e)
//      i.e. e=0 means that the result ip equals the one given by mpn_invert. e=1 means that the
//      result may be one less than expected. e=1 most of the time.
//
// When the strict result is needed, i.e., e = 0 in the relation above:
//   dp * (B ^ n + ip) < B ^ {2n} <= dp * (B ^ n + ip + 1)
// the function mpn_invert(ip, dp, scratch) should be used instead.
// mpn_bc_invertappr from mpn/generic/invertappr.c
pub fn mpn_bc_invertappr(ip: &mut [Limb], dp: &[Limb], xp: &mut [Limb]) -> Limb {
    let n = dp.len();
    assert_ne!(n, 0);
    assert!(dp[n - 1].get_highest_bit());

    // Compute a base value of r limbs.
    if n == 1 {
        ip[0] = (DoubleLimb::join_halves(!dp[0], Limb::MAX) / DoubleLimb::from(dp[0])).lower_half()
    } else {
        // n > 1 here
        let mut i = n;
        loop {
            i -= 1;
            xp[i] = Limb::MAX;
            if i == 0 {
                break;
            }
        }
        limbs_not_to_out(&mut xp[n..], &dp[..n]);

        // Now xp contains B ^ 2n - dp * B ^ n - 1
        if n == 2 {
            limbs_div_mod_by_two_limb(ip, &mut xp[..4], dp);
        } else {
            let inv = limbs_two_limb_inverse_helper(dp[n - 1], dp[n - 2]);
            if !MAYBE_DCP1_DIVAPPR || n < DC_DIVAPPR_Q_THRESHOLD {
                _limbs_div_mod_schoolbook_approx(ip, &mut xp[..2 * n], &dp[..n], inv);
            } else {
                _limbs_div_mod_divide_and_conquer_approx(ip, &mut xp[..2 * n], &dp[..n], inv);
            }
            assert!(!limbs_sub_limb_in_place(&mut ip[..n], 1));
            return 1;
        }
    }
    0
}

//TODO tune all
const INV_NEWTON_THRESHOLD: usize = 170;
const INV_MULMOD_BNM1_THRESHOLD: usize = 38;

fn npows() -> usize {
    (if size_of::<usize>() > 6 {
        48
    } else {
        8 * size_of::<usize>()
    }) - usize::checked_from(INV_NEWTON_THRESHOLD.ceiling_log_two()).unwrap()
}

// Computes the approximate reciprocal using Newton's iterations (at least one).
//
// Inspired by Algorithm "ApproximateReciprocal", published in "Modern Computer Arithmetic" by
// Richard P. Brent and Paul Zimmermann, algorithm 3.5, page 121 in version 0.4 of the book.
//
// Some adaptations were introduced, to allow product mod B ^ m - 1 and return the value e.
//
// We introduced a correction in such a way that "the value of B ^ {n + h} - T computed at step 8
// cannot exceed B ^ n - 1" (the book reads "2B ^ n - 1").
//
// Maximum scratch needed by this branch <= 2 * n, but have to fit 3 * rn in the scratch, i.e.
// 3 * rn <= 2 * n: we require n > 4.
//
// We use a wrapped product modulo B ^ m - 1. NOTE: is there any normalisation problem for the [0]
// class? It shouldn't: we compute 2 * |A * X_h - B ^ {n + h}| < B ^ m - 1. We may get [0] if and
// only if we get AX_h = B ^ {n + h}. This can happen only if A = B ^ {n} / 2, but this implies
// X_h = B ^ {h} * 2 - 1 i.e., AX_h = B ^ {n + h} - A, then we get into the "negative" branch, where
// X_h is not incremented (because A < B ^ n).
// mpn_ni_invertappr from mpn/generic/invertappr.c
pub fn mpn_ni_invertappr(ip: &mut [Limb], dp: &[Limb], scratch: &mut [Limb]) -> Limb {
    let mut n = dp.len();

    assert!(n > 4);
    assert!(dp[n - 1].get_highest_bit());
    let mut sizes = vec![0; npows()];
    let mut sizp = 0;

    // Compute the computation precisions from highest to lowest, leaving the base case size in
    // 'rn'.
    let mut rn = n;
    loop {
        sizes[sizp] = rn;
        rn = (rn >> 1) + 1;
        sizp += 1;
        if rn < INV_NEWTON_THRESHOLD {
            break;
        }
    }

    // We search the inverse of 0.dp, we compute it as 1.ip
    let dp_offset = n;
    let ip_offset = n;

    // Compute a base value of rn limbs.
    mpn_bc_invertappr(
        &mut ip[ip_offset - rn..],
        &dp[dp_offset - rn..dp_offset],
        scratch,
    );

    let mut tp = vec![];
    let mut mn = 0;
    if n >= INV_MULMOD_BNM1_THRESHOLD {
        mn = _limbs_mul_mod_limb_width_to_n_minus_1_next_size(n + 1);
        tp = vec![0; _limbs_mul_mod_limb_width_to_n_minus_1_scratch_size(mn, n, (n >> 1) + 1)];
    }
    // Use Newton's iterations to get the desired precision.

    let mut cy;
    loop {
        sizp -= 1;
        n = sizes[sizp];
        //
        // v    n  v
        // +----+--+
        // ^ rn ^
        //
        // Compute i_jd
        let condition = if n < INV_MULMOD_BNM1_THRESHOLD {
            true
        } else {
            mn = _limbs_mul_mod_limb_width_to_n_minus_1_next_size(n + 1);
            mn > n + rn
        };
        if condition {
            limbs_mul_greater_to_out(
                scratch,
                &dp[dp_offset - n..dp_offset],
                &ip[ip_offset - rn..ip_offset],
            );
            limbs_slice_add_same_length_in_place_left(
                &mut scratch[rn..],
                &dp[dp_offset - n..dp_offset - rn + 1],
            );
            cy = 1;
        // Remember we truncated, mod B ^ (n + 1)
        // We computed (truncated) xp of length n + 1 <- 1.ip * 0.dp
        } else {
            // Use B ^ mn - 1 wraparound
            _limbs_mul_mod_limb_width_to_n_minus_1(
                scratch,
                mn,
                &dp[dp_offset - n..dp_offset],
                &ip[ip_offset - rn..ip_offset],
                &mut tp,
            );
            // We computed {xp,mn} <- {ip,rn} * {dp,n} mod (B^mn-1)
            // We know that 2*|ip*dp + dp*B^rn - B^{rn+n}| < B^mn-1
            // Add dp*B^rn mod (B^mn-1)
            assert!(n >= mn - rn);
            let mut bcy = limbs_slice_add_same_length_in_place_left(
                &mut scratch[rn..mn],
                &dp[dp_offset - n..dp_offset - n + mn - rn],
            );
            bcy = _limbs_add_same_length_with_carry_in_in_place_left(
                &mut scratch[..n - (mn - rn)],
                &dp[dp_offset - (n - (mn - rn))..dp_offset],
                bcy,
            );
            cy = if bcy { 1 } else { 0 };
            // Subtract B^{rn+n}, maybe only compensate the carry
            scratch[mn] = 1; // set a limit for DECR_U
            assert!(!limbs_sub_limb_in_place(
                &mut scratch[rn + n - mn..mn + 1],
                WrappingSub::wrapping_sub(1, cy)
            ));
            // if DECR_U eroded xp[mn]
            let scratch_mn = scratch[mn];
            assert!(!limbs_sub_limb_in_place(
                &mut scratch[..mn],
                1.wrapping_sub(scratch_mn)
            ));
            cy = 0; // Remember we are working Mod B^mn-1
        }
        if scratch[n] < 2 {
            // "positive" residue class
            let mut cy = scratch[n]; // 0 <= cy <= 1 here.
            if cy != 0
                && !limbs_sub_same_length_in_place_left(
                    &mut scratch[..n],
                    &dp[dp_offset - n..dp_offset],
                )
            {
                cy += 1;
                assert!(limbs_sub_same_length_in_place_left(
                    &mut scratch[..n],
                    &dp[dp_offset - n..dp_offset]
                ));
                cy += 1;
            } else {
                cy += 1;
            }
            // 1 <= cy <= 3 here.
            if limbs_cmp_same_length(&scratch[..n], &dp[dp_offset - n..dp_offset])
                == Ordering::Greater
            {
                assert!(!limbs_sub_same_length_in_place_left(
                    &mut scratch[..n],
                    &dp[dp_offset - n..dp_offset]
                ));
                cy += 1;
            }
            let cmp = limbs_cmp_same_length(&scratch[..n - rn], &dp[dp_offset - n..dp_offset - rn]);
            let (scratch_lo, scratch_hi) = scratch.split_at_mut(n);
            assert!(!_limbs_sub_same_length_with_borrow_in_to_out(
                &mut scratch_hi[n - rn..],
                &dp[dp_offset - rn..dp_offset],
                &scratch_lo[n - rn..],
                cmp == Ordering::Greater
            ));
            assert!(!limbs_sub_limb_in_place(
                &mut ip[ip_offset - rn..ip_offset],
                cy
            )); // 1 <= cy <= 4 here
        } else {
            // "negative" residue class
            assert!(scratch[n] >= Limb::MAX - 1);
            assert!(!limbs_sub_limb_in_place(&mut scratch[..n + 1], cy));
            if scratch[n] != Limb::MAX {
                assert!(!limbs_slice_add_limb_in_place(
                    &mut ip[ip_offset - rn..ip_offset],
                    1
                ));
                assert!(limbs_slice_add_same_length_in_place_left(
                    &mut scratch[..n],
                    &dp[dp_offset - n..dp_offset]
                ));
            }
            let (scratch_lo, scratch_hi) = scratch.split_at_mut(n);
            limbs_not_to_out(&mut scratch_hi[n - rn..n], &scratch_lo[n - rn..]);
        }

        // Compute x_ju_j
        {
            let (scratch_lo, scratch_hi) = scratch.split_at_mut(2 * n - rn);
            limbs_mul_same_length_to_out(
                scratch_lo,
                &scratch_hi[..rn],
                &ip[ip_offset - rn..ip_offset],
            );
        }
        {
            let (scratch_lo, scratch_hi) = scratch.split_at_mut(3 * rn - n);
            cy = if limbs_slice_add_same_length_in_place_left(
                &mut scratch_lo[rn..],
                &scratch_hi[3 * n - 4 * rn..2 * (n - rn)],
            ) {
                1
            } else {
                0
            };
        }
        cy = if _limbs_add_same_length_with_carry_in_to_out(
            &mut ip[ip_offset - n..],
            &scratch[3 * rn - n..2 * rn],
            &scratch[n + rn..2 * n],
            cy != 0,
        ) {
            1
        } else {
            0
        };
        assert!(!limbs_slice_add_limb_in_place(
            &mut ip[ip_offset - rn..ip_offset],
            cy
        ));
        if sizp == 0 {
            // Get out of the cycle
            // Check for possible carry propagation from below.
            cy = if scratch[3 * rn - n - 1] > Limb::MAX - 7 {
                1
            } else {
                0
            }; // Be conservative.
            break;
        }
        rn = n;
    }
    cy
}

fn mpn_invertappr(ip: &mut [Limb], dp: &[Limb], scratch: &mut [Limb]) -> Limb {
    let n = dp.len();
    assert_ne!(n, 0);
    assert!(dp.last().unwrap().get_highest_bit());

    if n < INV_NEWTON_THRESHOLD {
        mpn_bc_invertappr(ip, dp, scratch)
    } else {
        mpn_ni_invertappr(ip, dp, scratch)
    }
}

pub fn mpn_mu_div_qr2(
    qp: &mut [Limb],
    rp: &mut [Limb],
    np: &[Limb],
    dp: &[Limb],
    scratch: &mut [Limb],
) -> bool {
    let nn = np.len();
    let dn = dp.len();

    assert!(dn > 1);

    let qn = nn - dn;

    // Compute the inverse size.
    let ip_len = mpn_mu_div_qr_choose_in(qn, dn, 0);
    assert!(ip_len <= dn);
    {
        let (ip, tp) = scratch.split_at_mut(ip_len + 1);
        // compute an approximate inverse on (in+1) limbs
        if dn == ip_len {
            tp[1..].copy_from_slice(&dp[..ip_len]);
            tp[0] = 1;
            let (tp_lo, tp_hi) = tp.split_at_mut(ip_len + 1);
            mpn_invertappr(ip, &tp_lo, tp_hi);
            limbs_move_left(ip, 1);
        } else {
            let cy = limbs_add_limb_to_out(tp, &dp[dn - (ip_len + 1)..dn], 1);
            if cy {
                limbs_set_zero(&mut ip[..ip_len]);
            } else {
                let (tp_lo, tp_hi) = tp.split_at_mut(ip_len + 1);
                mpn_invertappr(ip, tp_lo, tp_hi);
                limbs_move_left(ip, 1);
            }
        }
    }
    let (scratch_lo, scratch_hi) = scratch.split_at_mut(ip_len);
    mpn_preinv_mu_div_qr(qp, rp, np, dp, scratch_lo, scratch_hi)
}

const MUL_TO_MULMOD_BNM1_FOR_2NXN_THRESHOLD: usize = INV_MULMOD_BNM1_THRESHOLD >> 1;

pub fn mpn_preinv_mu_div_qr(
    qp: &mut [Limb],
    rp: &mut [Limb],
    np: &[Limb],
    dp: &[Limb],
    ip: &[Limb],
    scratch: &mut [Limb],
) -> bool {
    let nn = np.len();
    let dn = dp.len();
    let mut ip_len = ip.len();

    let mut qn = nn - dn;

    let mut np_offset = qn;
    let mut qp_offset = qn;

    let qh = limbs_cmp_same_length(&np[np_offset..np_offset + dn], &dp[..dn]) >= Ordering::Greater;
    if qh {
        limbs_sub_same_length_to_out(rp, &np[np_offset..np_offset + dn], &dp[..dn]);
    } else {
        rp[..dn].copy_from_slice(&np[np_offset..np_offset + dn]);
    }

    let mut ip_offset = 0;
    while qn != 0 {
        if qn < ip_len {
            ip_offset += ip_len - qn;
            ip_len = qn;
        }
        np_offset -= ip_len;
        qp_offset -= ip_len;

        // Compute the next block of quotient limbs by multiplying the inverse I
        // by the upper part of the partial remainder R.
        // mulhi
        limbs_mul_same_length_to_out(
            scratch,
            &rp[dn - ip_len..dn],
            &ip[ip_offset..ip_offset + ip_len],
        );
        // I's msb implicit
        let cy = limbs_add_same_length_to_out(
            &mut qp[qp_offset..],
            &scratch[ip_len..2 * ip_len],
            &rp[dn - ip_len..dn],
        );
        assert!(!cy);

        qn -= ip_len;

        // Compute the product of the quotient block and the divisor D, to be
        // subtracted from the partial remainder combined with new limbs from the
        // dividend N.  We only really need the low dn+1 limbs.
        if ip_len < MUL_TO_MULMOD_BNM1_FOR_2NXN_THRESHOLD {
            // dn+in limbs, high 'in' cancels
            limbs_mul_greater_to_out(scratch, &dp[..dn], &qp[qp_offset..qp_offset + ip_len]);
        } else {
            let tn = _limbs_mul_mod_limb_width_to_n_minus_1_next_size(dn + 1);
            let (tp, scratch_out) = scratch.split_at_mut(tn);
            _limbs_mul_mod_limb_width_to_n_minus_1(
                tp,
                tn,
                &dp[..dn],
                &qp[qp_offset..qp_offset + ip_len],
                scratch_out,
            );
            // number of wrapped limbs
            let wn = isize::checked_from(dn + ip_len).unwrap() - isize::checked_from(tn).unwrap();
            if wn > 0 {
                let wn = usize::checked_from(wn).unwrap();
                let mut cy = if limbs_sub_same_length_in_place_left(&mut tp[..wn], &rp[dn - wn..dn])
                {
                    1
                } else {
                    0
                };
                cy = if limbs_sub_limb_in_place(&mut tp[wn..tn], cy) {
                    1
                } else {
                    0
                };
                let cx = if limbs_cmp_same_length(&rp[dn - ip_len..tn - ip_len], &tp[dn..tn])
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
        let mut r = rp[dn - ip_len].wrapping_sub(scratch[dn]);
        // Subtract the product from the partial remainder combined with new
        // limbs from the dividend N, generating a new partial remainder R.
        let mut cy;
        if dn != ip_len {
            // get next 'in' limbs from N
            cy = if limbs_sub_same_length_in_place_right(
                &np[np_offset..np_offset + ip_len],
                &mut scratch[..ip_len],
            ) {
                1
            } else {
                0
            };
            cy = if _limbs_sub_same_length_with_borrow_in_in_place_right(
                &rp[..dn - ip_len],
                &mut scratch[ip_len..dn],
                cy != 0,
            ) {
                1
            } else {
                0
            };
            rp[..dn].copy_from_slice(&scratch[..dn]);
        } else {
            // get next 'in' limbs from N
            cy = if limbs_sub_same_length_to_out(
                rp,
                &np[np_offset..np_offset + ip_len],
                &scratch[..ip_len],
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
            assert!(!limbs_slice_add_limb_in_place(&mut qp[qp_offset..], 1));
            cy = if limbs_sub_same_length_in_place_left(&mut rp[..dn], &dp[..dn]) {
                1
            } else {
                0
            };
            r -= cy;
        }
        if limbs_cmp_same_length(&rp[..dn], &dp[..dn]) == Ordering::Greater {
            // This is executed with about 76% probability.
            assert!(!limbs_slice_add_limb_in_place(&mut qp[qp_offset..], 1));
            limbs_sub_same_length_in_place_left(&mut rp[..dn], &dp[..dn]);
        }
    }
    qh
}

//TODO tune
const MU_DIV_QR_SKEW_THRESHOLD: usize = 100;

pub fn mpn_mu_div_qr(
    qp: &mut [Limb],
    rp: &mut [Limb],
    np: &[Limb],
    dp: &[Limb],
    scratch: &mut [Limb],
) -> Limb {
    let nn = np.len();
    let dn = dp.len();
    let qn = nn - dn;
    let mut qh;
    if qn + MU_DIV_QR_SKEW_THRESHOLD < dn {
        qh = if mpn_mu_div_qr2(
            qp,
            &mut rp[nn - (2 * qn + 1)..],
            &np[nn - (2 * qn + 1)..nn],
            &dp[dn - (qn + 1)..dn],
            scratch,
        ) {
            1
        } else {
            0
        };

        // Multiply the quotient by the divisor limbs ignored above.
        // prod is dn-1 limbs
        limbs_mul_to_out(scratch, &dp[..dn - (qn + 1)], &qp[..qn]);

        let mut cy = if qh != 0 {
            limbs_slice_add_same_length_in_place_left(
                &mut scratch[qn..dn - 1],
                &dp[..dn - (qn + 1)],
            )
        } else {
            false
        };
        scratch[dn - 1] = if cy { 1 } else { 0 };

        cy = limbs_sub_same_length_to_out(
            rp,
            &np[..nn - (2 * qn + 1)],
            &scratch[..nn - (2 * qn + 1)],
        );
        cy = _limbs_sub_same_length_with_borrow_in_in_place_left(
            &mut rp[nn - (2 * qn + 1)..nn - qn],
            &scratch[nn - (2 * qn + 1)..nn - qn],
            cy,
        );
        if cy {
            qh -= if limbs_sub_limb_in_place(&mut qp[..qn], 1) {
                1
            } else {
                0
            };
            limbs_slice_add_same_length_in_place_left(&mut rp[..dn], &dp[..dn]);
        }
    } else {
        qh = if mpn_mu_div_qr2(qp, rp, &np[..nn], &dp[..dn], scratch) {
            1
        } else {
            0
        };
    }
    qh
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
                    let itch = mpn_mu_div_qr_itch(nn, dn, 0);
                    let mut scratch = vec![0; itch];
                    mpn_mu_div_qr(qp, rp, &n2p[..nn], &d2p[..dn], &mut scratch);
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
                    let itch = mpn_mu_div_qr_itch(2 * qn, qn, 0);
                    let mut scratch = vec![0; itch];
                    // If N and R share space, put ...
                    // intermediate remainder at N's upper end.
                    // if np == r2p {
                    //     r2p += nn - qn;
                    // }
                    mpn_mu_div_qr(qp, rp, &n2p[..2 * qn], &d2p[..qn], &mut scratch);
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
