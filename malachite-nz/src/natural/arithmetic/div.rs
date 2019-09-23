use std::cmp::Ordering;
use std::ops::{Div, DivAssign};

use malachite_base::comparison::Max;
use malachite_base::limbs::limbs_move_left;
use malachite_base::num::arithmetic::traits::{
    DivAssignMod, DivMod, WrappingAddAssign, WrappingSubAssign,
};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::conversion::traits::{CheckedFrom, JoinHalves, SplitInHalf};

use natural::arithmetic::add::limbs_slice_add_same_length_in_place_left;
use natural::arithmetic::div_mod::{
    _limbs_div_mod_divide_and_conquer_helper, _limbs_div_mod_schoolbook,
    limbs_div_mod_by_two_limb_normalized, limbs_div_mod_three_limb_by_two_limb,
};
use natural::arithmetic::mul::{limbs_mul_greater_to_out, limbs_mul_to_out};
use natural::arithmetic::sub::limbs_sub_same_length_in_place_left;
use natural::arithmetic::sub_limb::limbs_sub_limb_in_place;
use natural::arithmetic::sub_mul_limb::limbs_sub_mul_limb_same_length_in_place_left;
use natural::comparison::ord::limbs_cmp_same_length;
use natural::Natural;
use platform::{DoubleLimb, Limb, DC_DIVAPPR_Q_THRESHOLD, DC_DIV_QR_THRESHOLD};

/// Schoolbook division using the Möller-Granlund 3/2 division algorithm.
///
/// Divides `ns` by `ds` and writes the `ns.len()` - `ds.len()` least-significant quotient limbs to
/// `qs`. Returns the most significant limb of the quotient; `true` means 1 and `false` means 0.
/// `ds` must have length greater than 2, `ns` must be at least as long as `ds`, and the most
/// significant bit of `ds` must be set. `inverse` should be the result of
/// `limbs_two_limb_inverse_helper` applied to the two highest limbs of the denominator.
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
/// This is mpn_sbpi1_div_q from mpn/generic/sbpi1_div_q.c.
pub fn _limbs_div_schoolbook(qs: &mut [Limb], ns: &mut [Limb], ds: &[Limb], inverse: Limb) -> bool {
    let n_len = ns.len();
    let d_len = ds.len();
    assert!(d_len > 2);
    assert!(n_len >= d_len);
    let d_len_m_1 = d_len - 1;
    let d_1 = ds[d_len_m_1];
    assert!(d_1.get_highest_bit());
    let q_len = n_len - d_len;
    let ds_s = if q_len < d_len_m_1 {
        &ds[d_len_m_1 - q_len..]
    } else {
        ds
    };
    let d_len_s = ds_s.len(); // d_len or n_len - d_len + 1
    let d_sum = d_len + d_len_s; // 2 * d_len or n_len + 1
    let d_diff = d_len - d_len_s; // 0 or 2 * d_len - n_len - 1
    let highest_q = limbs_cmp_same_length(&ns[n_len - d_len_s..], ds_s) >= Ordering::Equal;
    if highest_q {
        limbs_sub_same_length_in_place_left(&mut ns[n_len - d_len_s..], ds_s);
    }
    // Offset d_len by 2 for main division loops, saving two iterations in
    // limbs_sub_mul_limb_same_length_in_place_left.
    let d_len_m_2 = d_len - 2;
    let d_len_s_m_1 = d_len_s - 1;
    let d_len_s_m_2 = d_len_s.wrapping_sub(2); // only used when d_len_s >= 2
    let d_2 = ds[d_len_m_2];
    let mut n_1 = ns[n_len - 1];
    for i in (d_sum - 1..n_len).rev() {
        let ns = &mut ns[i - d_len_s..i];
        let mut quotient;
        if n_1 == d_1 && ns[d_len_s_m_1] == d_2 {
            // TODO This branch is untested!
            quotient = Limb::MAX;
            limbs_sub_mul_limb_same_length_in_place_left(ns, ds_s, quotient);
            n_1 = ns[d_len_s_m_1]; // update n_1; last loop's value will now be invalid
        } else {
            let (new_q, n) = limbs_div_mod_three_limb_by_two_limb(
                n_1,
                ns[d_len_s_m_1],
                ns[d_len_s - 2],
                d_1,
                d_2,
                inverse,
            );
            quotient = new_q;
            let (new_n_1, mut n_0) = n.split_in_half();
            n_1 = new_n_1;
            let carry = limbs_sub_mul_limb_same_length_in_place_left(
                &mut ns[..d_len_s - 2],
                &ds_s[..d_len_s - 2],
                quotient,
            );
            let carry_2 = n_0 < carry;
            n_0.wrapping_sub_assign(carry);
            let carry = carry_2 && n_1 == 0;
            if carry_2 {
                n_1.wrapping_sub_assign(1);
            }
            ns[d_len_s_m_2] = n_0;
            if carry {
                n_1.wrapping_add_assign(d_1);
                if limbs_slice_add_same_length_in_place_left(
                    &mut ns[..d_len_s_m_1],
                    &ds_s[..d_len_s_m_1],
                ) {
                    n_1.wrapping_add_assign(1);
                }
                quotient.wrapping_sub_assign(1);
            }
        }
        qs[i - d_len] = quotient;
    }
    let mut flag = true;
    let offset = if d_len_s >= 2 {
        let mut ds_suffix = &ds[d_diff..];
        for i in (1..d_len_s_m_1).rev() {
            let ns = &mut ns[d_len_m_2..d_len + i];
            let mut quotient;
            if !flag || n_1 >= d_1 {
                quotient = Limb::MAX;
                let carry = limbs_sub_mul_limb_same_length_in_place_left(ns, ds_suffix, quotient);
                if n_1 != carry {
                    if flag && n_1 < carry {
                        quotient.wrapping_sub_assign(1);
                        limbs_slice_add_same_length_in_place_left(ns, ds_suffix);
                    } else {
                        // TODO This branch is untested!
                        flag = false;
                    }
                }
                n_1 = ns[i + 1];
            } else {
                let (new_quotient, new_n) =
                    limbs_div_mod_three_limb_by_two_limb(n_1, ns[i + 1], ns[i], d_1, d_2, inverse);
                quotient = new_quotient;
                let (new_n_1, mut n_0) = new_n.split_in_half();
                n_1 = new_n_1;
                let carry = limbs_sub_mul_limb_same_length_in_place_left(
                    &mut ns[..i],
                    &ds_suffix[..ds_suffix.len() - 2],
                    quotient,
                );
                let carry_2 = n_0 < carry;
                n_0.wrapping_sub_assign(carry);
                let carry = carry_2 && n_1 == 0;
                if carry_2 {
                    n_1.wrapping_sub_assign(1);
                }
                ns[i] = n_0;
                if carry {
                    n_1.wrapping_add_assign(d_1);
                    if limbs_slice_add_same_length_in_place_left(
                        &mut ns[..i + 1],
                        &ds_suffix[..ds_suffix.len() - 1],
                    ) {
                        n_1.wrapping_add_assign(1);
                    }
                    quotient.wrapping_sub_assign(1);
                }
            }
            qs[i] = quotient;
            ds_suffix = &ds_suffix[1..];
        }
        let mut quotient;
        let ns = &mut ns[d_len_m_2..d_len];
        if !flag || n_1 >= d_1 {
            quotient = Limb::MAX;
            let ds_hi = &ds[d_len - 3..];
            let carry = limbs_sub_mul_limb_same_length_in_place_left(ns, &ds_hi[..2], quotient);
            if n_1 != carry {
                if flag && n_1 < carry {
                    quotient.wrapping_sub_assign(1);
                    let (new_n_1, new_n_0) = DoubleLimb::join_halves(ns[1], ns[0])
                        .wrapping_add(DoubleLimb::join_halves(d_2, ds_hi[0]))
                        .split_in_half();
                    ns[1] = new_n_1;
                    ns[0] = new_n_0;
                } else {
                    // TODO This branch is untested!
                    flag = false;
                }
            }
            n_1 = ns[1];
        } else {
            let (new_quotient, new_n) =
                limbs_div_mod_three_limb_by_two_limb(n_1, ns[1], ns[0], d_1, d_2, inverse);
            quotient = new_quotient;
            let (new_n_1, n_0) = new_n.split_in_half();
            n_1 = new_n_1;
            ns[0] = n_0;
            ns[1] = n_1;
        }
        qs[0] = quotient;
        d_len
    } else {
        d_sum - 1
    };
    let (ns_last, ns_init) = ns[..offset].split_last_mut().unwrap();
    assert_eq!(*ns_last, n_1);
    if !flag || n_1 < Limb::checked_from(d_len).unwrap() {
        let qs = &mut qs[offset - d_len..];
        let qs = &mut qs[..q_len];
        // The quotient may be too large if the remainder is small. Recompute for above ignored
        // operand parts, until the remainder spills. Compensate for triangularization.
        let ns = ns_init;
        {
            let (ns_last, ns_init) = ns.split_last_mut().unwrap();
            for i in 3..d_len_s + 1 {
                let q = qs[d_len_s - i];
                let carry = limbs_sub_mul_limb_same_length_in_place_left(
                    &mut ns_init[offset - i..],
                    &ds_s[..i - 2],
                    q,
                );
                if *ns_last < carry {
                    if n_1 == 0 {
                        assert!(!limbs_sub_limb_in_place(qs, 1));
                        return highest_q;
                    }
                    n_1 -= 1;
                }
                ns_last.wrapping_sub_assign(carry);
            }
        }
        if d_diff != 0 {
            // Compensate for ignored dividend and divisor tails.
            if highest_q {
                let mut carry =
                    limbs_sub_same_length_in_place_left(&mut ns[q_len..d_len_m_1], &ds[..d_diff]);
                if carry {
                    if n_1 == 0 {
                        // TODO This branch is untested! (else)
                        if q_len != 0 {
                            carry = limbs_sub_limb_in_place(qs, 1);
                        }
                        assert!(highest_q || !carry);
                        return highest_q != carry;
                    }
                    n_1 -= 1;
                }
            }
            if q_len == 0 {
                return highest_q;
            }
            let ns = &mut ns[..d_len_m_1];
            for i in (0..d_diff).rev() {
                let (ns_lo, ns_hi) = ns[i..].split_at_mut(q_len);
                if limbs_sub_limb_in_place(
                    ns_hi,
                    limbs_sub_mul_limb_same_length_in_place_left(ns_lo, qs, ds[i]),
                ) {
                    if n_1 == 0 {
                        limbs_sub_limb_in_place(qs, 1);
                        return highest_q;
                    }
                    n_1 -= 1;
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
pub fn _limbs_div_schoolbook_approx(
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

/// Time: worst case O(n * log(n) ^ 2 * log(log(n)))
///
/// Additional memory: worst case O(n * log(n) ^ 2)
///
/// where n = `ds.len()`
///
/// This is mpn_dcpi1_divappr_q_n from mpn/generic/dcpi1_divappr_q.c, where ns here is np +
/// (n >> 1).
fn _limbs_div_divide_and_conquer_approx_helper(
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
        _limbs_div_schoolbook_approx(qs, &mut ns_hi[..lo << 1], ds_hi, inverse)
    } else {
        _limbs_div_divide_and_conquer_approx_helper(
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
pub fn _limbs_div_divide_and_conquer_approx(
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
                        limbs_div_mod_by_two_limb_normalized(
                            qs,
                            &mut ns_hi[..q_len_mod_d_len + 2],
                            ds_hi,
                        )
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
        _limbs_div_divide_and_conquer_approx_helper(qs, ns, ds, inverse, &mut scratch);
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
            highest_q = _limbs_div_schoolbook_approx(&mut qs_2, ns, ds, inverse);
        } else {
            let mut scratch = vec![0; q_len_plus_one];
            highest_q = _limbs_div_divide_and_conquer_approx_helper(
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

impl Div<Natural> for Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `Natural`, taking both `Natural`s by value. The quotient is rounded
    /// towards negative infinity. The quotient and remainder satisfy `self` = q * `other` + r and
    /// 0 <= r < `other`.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!((Natural::from(23u32) / Natural::from(10u32)).to_string(), "2");
    ///
    ///     // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    ///     assert_eq!(
    ///         (Natural::from_str("1000000000000000000000000").unwrap() /
    ///         Natural::from_str("1234567890987").unwrap()).to_string(),
    ///         "810000006723"
    ///     );
    /// }
    /// ```
    #[inline]
    fn div(mut self, other: Natural) -> Natural {
        self /= other;
        self
    }
}

impl<'a> Div<&'a Natural> for Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `Natural`, taking the first `Natural` by value and the second by
    /// reference. The quotient is rounded towards negative infinity. The quotient and remainder
    /// satisfy `self` = q * `other` + r and 0 <= r < `other`.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!((Natural::from(23u32) / &Natural::from(10u32)).to_string(), "2");
    ///
    ///     // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    ///     assert_eq!(
    ///         (Natural::from_str("1000000000000000000000000").unwrap() /
    ///         &Natural::from_str("1234567890987").unwrap()).to_string(),
    ///         "810000006723"
    ///     );
    /// }
    /// ```
    #[inline]
    fn div(mut self, other: &'a Natural) -> Natural {
        self /= other;
        self
    }
}

impl<'a> Div<Natural> for &'a Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `Natural`, taking the first `Natural` by reference and the second
    /// by value. The quotient is rounded towards negative infinity. The quotient and remainder
    /// satisfy `self` = q * `other` + r and 0 <= r < `other`.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
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
    ///     assert_eq!((&Natural::from(23u32) / Natural::from(10u32)).to_string(), "2");
    ///
    ///     // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    ///     assert_eq!(
    ///         (&Natural::from_str("1000000000000000000000000").unwrap() /
    ///         Natural::from_str("1234567890987").unwrap()).to_string(),
    ///         "810000006723"
    ///     );
    /// }
    /// ```
    #[inline]
    fn div(self, other: Natural) -> Natural {
        self.div_mod(other).0
    }
}

impl<'a, 'b> Div<&'b Natural> for &'a Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `Natural`, taking both `Natural`s by reference. The quotient is
    /// rounded towards negative infinity. The quotient and remainder satisfy `self` =
    /// q * `other` + r and 0 <= r < `other`.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
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
    ///     assert_eq!((&Natural::from(23u32) / &Natural::from(10u32)).to_string(), "2");
    ///
    ///     // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    ///     assert_eq!(
    ///         (&Natural::from_str("1000000000000000000000000").unwrap() /
    ///         &Natural::from_str("1234567890987").unwrap()).to_string(),
    ///         "810000006723"
    ///     );
    /// }
    /// ```
    #[inline]
    fn div(self, other: &'b Natural) -> Natural {
        self.div_mod(other).0
    }
}

impl DivAssign<Natural> for Natural {
    /// Divides a `Natural` by a `Natural` in place, taking the second `Natural` by value. The
    /// quotient is rounded towards negative infinity. The quotient and remainder satisfy `self` =
    /// q * `other` + r and 0 <= r < `other`.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     let mut x = Natural::from(23u32);
    ///     x /= Natural::from(10u32);
    ///     assert_eq!(x.to_string(), "2");
    ///
    ///     // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    ///     let mut x = Natural::from_str("1000000000000000000000000").unwrap();
    ///     x /= Natural::from_str("1234567890987").unwrap();
    ///     assert_eq!(x.to_string(), "810000006723");
    /// }
    /// ```
    #[inline]
    fn div_assign(&mut self, other: Natural) {
        self.div_assign_mod(other);
    }
}

impl<'a> DivAssign<&'a Natural> for Natural {
    /// Divides a `Natural` by a `Natural` in place, taking the second `Natural` by reference. The
    /// quotient is rounded towards negative infinity. The quotient and remainder satisfy `self` =
    /// q * `other` + r and 0 <= r < `other`.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     let mut x = Natural::from(23u32);
    ///     x /= &Natural::from(10u32);
    ///     assert_eq!(x.to_string(), "2");
    ///
    ///     // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    ///     let mut x = Natural::from_str("1000000000000000000000000").unwrap();
    ///     x /= &Natural::from_str("1234567890987").unwrap();
    ///     assert_eq!(x.to_string(), "810000006723");
    /// }
    /// ```
    #[inline]
    fn div_assign(&mut self, other: &'a Natural) {
        self.div_assign_mod(other);
    }
}
