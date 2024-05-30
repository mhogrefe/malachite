// Copyright © 2024 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      `mpn_dcpi1_divappr_q`, `mpn_dcpi1_divappr_q_n`, `mpn_div_q`, `mpn_mu_divappr_q`,
//      `mpn_mu_divappr_q_choose_in`, `mpn_mu_divappr_q_itch`, `mpn_preinv_mu_divappr_q`,
//      `mpn_sbpi1_div_q`, and `mpn_sbpi1_divappr_q` contributed to the GNU project by Torbjörn
//      Granlund.
//
//      `mpn_dcpi1_div_q`, `mpn_mu_div_q`, and `mpn_mu_div_q_itch` contributed to the GNU project by
//      Torbjörn Granlund and Marco Bodrato.
//
//      `mpn_div_qr_1` contributed to the GNU project by Niels Möller and Torbjörn Granlund.
//
//      Copyright © 1991-2018 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::add::{
    limbs_add_limb_to_out, limbs_add_same_length_to_out, limbs_slice_add_limb_in_place,
    limbs_slice_add_same_length_in_place_left,
};
use crate::natural::arithmetic::div_mod::{
    div_mod_by_preinversion, limbs_div_barrett_large_product, limbs_div_mod_by_two_limb_normalized,
    limbs_div_mod_divide_and_conquer_helper, limbs_div_mod_schoolbook,
    limbs_div_mod_three_limb_by_two_limb, limbs_invert_approx, limbs_invert_approx_scratch_len,
    limbs_invert_limb, limbs_two_limb_inverse_helper, MUL_TO_MULMOD_BNM1_FOR_2NXN_THRESHOLD,
    MUPI_DIV_QR_THRESHOLD,
};
use crate::natural::arithmetic::mul::mul_mod::{
    limbs_mul_mod_base_pow_n_minus_1_next_size, limbs_mul_mod_base_pow_n_minus_1_scratch_len,
};
use crate::natural::arithmetic::mul::{
    limbs_mul_greater_to_out, limbs_mul_greater_to_out_scratch_len, limbs_mul_same_length_to_out,
    limbs_mul_same_length_to_out_scratch_len, limbs_mul_to_out, limbs_mul_to_out_scratch_len,
};
use crate::natural::arithmetic::shl::{limbs_shl_to_out, limbs_slice_shl_in_place};
use crate::natural::arithmetic::sub::{
    limbs_sub_limb_in_place, limbs_sub_limb_to_out, limbs_sub_same_length_in_place_left,
    limbs_sub_same_length_in_place_right, limbs_sub_same_length_to_out,
    limbs_sub_same_length_with_borrow_in_in_place_right,
};
use crate::natural::arithmetic::sub_mul::limbs_sub_mul_limb_same_length_in_place_left;
use crate::natural::comparison::cmp::limbs_cmp_same_length;
use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::{
    DoubleLimb, Limb, DC_DIVAPPR_Q_THRESHOLD, DC_DIV_QR_THRESHOLD, FUDGE, MU_DIVAPPR_Q_THRESHOLD,
};
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::cmp::Ordering::*;
use core::iter::once;
use core::mem::swap;
use core::ops::{Div, DivAssign};
use malachite_base::fail_on_untested_path;
#[cfg(feature = "test_build")]
use malachite_base::num::arithmetic::traits::DivRem;
use malachite_base::num::arithmetic::traits::{
    CheckedDiv, WrappingAddAssign, WrappingMulAssign, WrappingSubAssign, XMulYToZZ, XXAddYYToZZ,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, JoinHalves, SplitInHalf};
use malachite_base::num::logic::traits::LeadingZeros;
use malachite_base::slices::{slice_move_left, slice_set_zero};

// Divide an number by a divisor of B - 1, where B is the limb base.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// # Panics
// Panics if `out` is shorter than `ns`.
//
// This is equivalent to `mpn_bdiv_dbm1c` from `mpn/generic/bdiv_dbm1c.c`, GMP 6.2.1.
pub_crate_test! {limbs_div_divisor_of_limb_max_with_carry_to_out(
    out: &mut [Limb],
    ns: &[Limb],
    d: Limb,
    mut carry: Limb,
) -> Limb {
    assert!(out.len() >= ns.len());
    let d = DoubleLimb::from(d);
    for (q, &n) in out.iter_mut().zip(ns.iter()) {
        let (hi, lo) = (DoubleLimb::from(n) * d).split_in_half();
        let inner_carry = carry < lo;
        carry.wrapping_sub_assign(lo);
        *q = carry;
        carry.wrapping_sub_assign(hi);
        if inner_carry {
            carry.wrapping_sub_assign(1);
        }
    }
    carry
}}

// Divide an number by a divisor of B - 1, where B is the limb base.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// This is equivalent to `mpn_bdiv_dbm1c` from `mpn/generic/bdiv_dbm1c.c`, GMP 6.2.1, where `qp ==
// ap`.
pub_crate_test! {limbs_div_divisor_of_limb_max_with_carry_in_place(
    ns: &mut [Limb],
    d: Limb,
    mut carry: Limb,
) -> Limb {
    let d = DoubleLimb::from(d);
    for n in &mut *ns {
        let (hi, lo) = (DoubleLimb::from(*n) * d).split_in_half();
        let inner_carry = carry < lo;
        carry.wrapping_sub_assign(lo);
        *n = carry;
        carry.wrapping_sub_assign(hi);
        if inner_carry {
            carry.wrapping_sub_assign(1);
        }
    }
    carry
}}

// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `udiv_qrnnd_preinv` from `gmp-impl.h`, GMP 6.2.1, but not computing the
// remainder.
pub_test! {div_by_preinversion(n_high: Limb, n_low: Limb, d: Limb, d_inv: Limb) -> Limb {
    let (mut q_high, q_low) = (DoubleLimb::from(n_high) * DoubleLimb::from(d_inv))
        .wrapping_add(DoubleLimb::join_halves(n_high.wrapping_add(1), n_low))
        .split_in_half();
    let mut r = n_low.wrapping_sub(q_high.wrapping_mul(d));
    if r > q_low {
        q_high.wrapping_sub_assign(1);
        r.wrapping_add_assign(d);
    }
    if r >= d {
        q_high.wrapping_add_assign(1);
    }
    q_high
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
// quotient limbs of the `Natural` divided by a `Limb`. The divisor limb cannot be zero and the limb
// slice must have at least two elements.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// # Panics
// Panics if the length of `ns` is less than 2 or if `d` is zero.
//
// This is equivalent to `mpn_div_qr_1` from `mpn/generic/div_qr_1.c`, GMP 6.2.1, where the quotient
// is returned, but not computing the remainder.
pub_test! {limbs_div_limb(ns: &[Limb], d: Limb) -> Vec<Limb> {
    let mut qs = vec![0; ns.len()];
    limbs_div_limb_to_out(&mut qs, ns, d);
    qs
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
// limbs of the quotient of the `Natural` and a `Limb` to an output slice. The output slice must be
// at least as long as the input slice. The divisor limb cannot be zero and the input limb slice
// must have at least two elements.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// # Panics
// Panics if `out` is shorter than `ns`, the length of `ns` is less than 2, or if `d` is zero.
//
// This is equivalent to `mpn_divrem_1` from `mpn/generic/divrem_1.c`, GMP 6.2.1, where `qxn == 0`.
// and `un > 1`, but not computing the remainder.
pub_crate_test! {limbs_div_limb_to_out(out: &mut [Limb], ns: &[Limb], d: Limb) {
    assert_ne!(d, 0);
    let len = ns.len();
    assert!(len > 1);
    let out = &mut out[..len];
    let bits = LeadingZeros::leading_zeros(d);
    if bits == 0 {
        // High quotient limb is 0 or 1, skip a divide step.
        let (r, ns_init) = ns.split_last().unwrap();
        let mut r = *r;
        let (out_last, out_init) = out.split_last_mut().unwrap();
        let adjust = r >= d;
        if adjust {
            r -= d;
        }
        *out_last = Limb::from(adjust);
        // Multiply-by-inverse, divisor already normalized.
        let d_inv = limbs_invert_limb(d);
        for (out_q, &n) in out_init.iter_mut().zip(ns_init.iter()).rev() {
            (*out_q, r) = div_mod_by_preinversion(r, n, d, d_inv);
        }
    } else {
        // Skip a division if high < divisor (high quotient 0). Testing here before normalizing will
        // still skip as often as possible.
        let (ns_last, ns_init) = ns.split_last().unwrap();
        let (ns, mut r) = if *ns_last < d {
            *out.last_mut().unwrap() = 0;
            (ns_init, *ns_last)
        } else {
            (ns, 0)
        };
        let d = d << bits;
        r <<= bits;
        let d_inv = limbs_invert_limb(d);
        let (previous_n, ns_init) = ns.split_last().unwrap();
        let mut previous_n = *previous_n;
        let cobits = Limb::WIDTH - bits;
        r |= previous_n >> cobits;
        let (out_first, out_tail) = out.split_first_mut().unwrap();
        for (out_q, &n) in out_tail.iter_mut().zip(ns_init.iter()).rev() {
            let shifted_n = (previous_n << bits) | (n >> cobits);
            (*out_q, r) = div_mod_by_preinversion(r, shifted_n, d, d_inv);
            previous_n = n;
        }
        *out_first = div_by_preinversion(r, previous_n << bits, d, d_inv);
    }
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
// limbs of the quotient of the `Natural` and a `Limb` to the input slice. The divisor limb cannot
// be zero and the input limb slice must have at least two elements.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// # Panics
// Panics if the length of `ns` is less than 2 or if `d` is zero.
//
// This is equivalent to `mpn_divrem_1` from `mpn/generic/divrem_1.c`, GMP 6.2.1, where `qp == up`,
// `qxn == 0`, and `un > 1`, but not computing the remainder.
pub_test! {limbs_div_limb_in_place(ns: &mut [Limb], d: Limb) {
    assert_ne!(d, 0);
    let len = ns.len();
    assert!(len > 1);
    let bits = LeadingZeros::leading_zeros(d);
    let (ns_last, ns_init) = ns.split_last_mut().unwrap();
    if bits == 0 {
        // High quotient limb is 0 or 1, skip a divide step.
        let mut r = *ns_last;
        let adjust = r >= d;
        if adjust {
            r -= d;
        }
        *ns_last = Limb::from(adjust);
        // Multiply-by-inverse, divisor already normalized.
        let d_inv = limbs_invert_limb(d);
        for n in ns_init.iter_mut().rev() {
            (*n, r) = div_mod_by_preinversion(r, *n, d, d_inv);
        }
    } else {
        // Skip a division if high < divisor (high quotient 0). Testing here before normalizing will
        // still skip as often as possible.
        let (ns, mut r) = if *ns_last < d {
            let r = *ns_last;
            *ns_last = 0;
            (ns_init, r)
        } else {
            (ns, 0)
        };
        let d = d << bits;
        r <<= bits;
        let d_inv = limbs_invert_limb(d);
        let last_index = ns.len() - 1;
        let mut previous_n = ns[last_index];
        let cobits = Limb::WIDTH - bits;
        r |= previous_n >> cobits;
        for i in (0..last_index).rev() {
            let n = ns[i];
            let shifted_n = (previous_n << bits) | (n >> cobits);
            (ns[i + 1], r) = div_mod_by_preinversion(r, shifted_n, d, d_inv);
            previous_n = n;
        }
        ns[0] = div_by_preinversion(r, previous_n << bits, d, d_inv);
    }
}}

// Schoolbook division using the Möller-Granlund 3/2 division algorithm.
//
// Divides `ns` by `ds` and writes the `ns.len()` - `ds.len()` least-significant quotient limbs to
// `qs`. Returns the most significant limb of the quotient; `true` means 1 and `false` means 0. `ds`
// must have length greater than 2, `ns` must be at least as long as `ds`, and the most significant
// bit of `ds` must be set. `d_inv` should be the result of `limbs_two_limb_inverse_helper` applied
// to the two highest limbs of the denominator.
//
// # Worst-case complexity
// $T(n) = O(n^2)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// # Panics
// Panics if `ds` has length smaller than 3, `ns` is shorter than `ds`, `qs` has length less than
// `ns.len()` - `ds.len()`, or the last limb of `ds` does not have its highest bit set.
//
// This is equivalent to `mpn_sbpi1_div_q` from `mpn/generic/sbpi1_div_q.c`, GMP 6.2.1.
pub_test! {limbs_div_schoolbook(
    qs: &mut [Limb],
    ns: &mut [Limb],
    ds: &[Limb],
    d_inv: Limb
) -> bool {
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
    let ns_hi = &mut ns[n_len - d_len_s..];
    let highest_q = limbs_cmp_same_length(ns_hi, ds_s) >= Equal;
    if highest_q {
        limbs_sub_same_length_in_place_left(ns_hi, ds_s);
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
        let mut q;
        if n_1 == d_1 && ns[d_len_s_m_1] == d_2 {
            q = Limb::MAX;
            limbs_sub_mul_limb_same_length_in_place_left(ns, ds_s, q);
            n_1 = ns[d_len_s_m_1]; // update n_1; last loop's value will now be invalid
        } else {
            let n;
            (q, n) = limbs_div_mod_three_limb_by_two_limb(
                n_1,
                ns[d_len_s_m_1],
                ns[d_len_s - 2],
                d_1,
                d_2,
                d_inv,
            );
            let mut n_0;
            (n_1, n_0) = n.split_in_half();
            let carry = limbs_sub_mul_limb_same_length_in_place_left(
                &mut ns[..d_len_s - 2],
                &ds_s[..d_len_s - 2],
                q,
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
                q.wrapping_sub_assign(1);
            }
        }
        qs[i - d_len] = q;
    }
    let mut flag = true;
    let offset = if d_len_s >= 2 {
        let mut ds_suffix = &ds[d_diff..];
        for i in (1..d_len_s_m_1).rev() {
            let ns = &mut ns[d_len_m_2..d_len + i];
            let mut q;
            if !flag || n_1 >= d_1 {
                q = Limb::MAX;
                let carry = limbs_sub_mul_limb_same_length_in_place_left(ns, ds_suffix, q);
                if n_1 != carry {
                    if flag && n_1 < carry {
                        q.wrapping_sub_assign(1);
                        limbs_slice_add_same_length_in_place_left(ns, ds_suffix);
                    } else {
                        flag = false;
                    }
                }
                n_1 = ns[i + 1];
            } else {
                let n;
                (q, n) =
                    limbs_div_mod_three_limb_by_two_limb(n_1, ns[i + 1], ns[i], d_1, d_2, d_inv);
                let mut n_0;
                (n_1, n_0) = n.split_in_half();
                let carry = limbs_sub_mul_limb_same_length_in_place_left(
                    &mut ns[..i],
                    &ds_suffix[..ds_suffix.len() - 2],
                    q,
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
                        &mut ns[..=i],
                        &ds_suffix[..ds_suffix.len() - 1],
                    ) {
                        n_1.wrapping_add_assign(1);
                    }
                    q.wrapping_sub_assign(1);
                }
            }
            qs[i] = q;
            ds_suffix = &ds_suffix[1..];
        }
        let mut q;
        let ns = &mut ns[d_len_m_2..d_len];
        if !flag || n_1 >= d_1 {
            q = Limb::MAX;
            let ds_hi = &ds[d_len_m_2..];
            let carry = limbs_sub_mul_limb_same_length_in_place_left(ns, &ds_hi[..2], q);
            if n_1 != carry {
                if flag && n_1 < carry {
                    q.wrapping_sub_assign(1);
                    (ns[1], ns[0]) = Limb::xx_add_yy_to_zz(ns[1], ns[0], d_2, ds_hi[0]);
                } else {
                    flag = false;
                }
            }
            n_1 = ns[1];
        } else {
            let new_n;
            (q, new_n) = limbs_div_mod_three_limb_by_two_limb(n_1, ns[1], ns[0], d_1, d_2, d_inv);
            (n_1, ns[0]) = new_n.split_in_half();
            ns[1] = n_1;
        }
        qs[0] = q;
        d_len
    } else {
        d_sum - 1
    };
    let (ns_last, ns_init) = ns[..offset].split_last_mut().unwrap();
    assert_eq!(*ns_last, n_1);
    if flag && n_1 < Limb::exact_from(d_len) {
        let qs = &mut qs[offset - d_len..];
        let qs = &mut qs[..q_len];
        // The quotient may be too large if the remainder is small. Recompute for above ignored
        // operand parts, until the remainder spills. Compensate for triangularization.
        let ns = ns_init;
        {
            let (ns_last, ns_init) = ns.split_last_mut().unwrap();
            for i in 3..=d_len_s {
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
}}

// Recursive divide-and-conquer division.
//
// Divides `ns` by `ds` and writes the `ns.len()` - `ds.len()` least-significant quotient limbs to
// `qs`. Returns the most significant limb of the quotient; `true` means 1 and `false` means 0. `ds`
// must have length greater than 2, `ns` must be at least as long as `ds`, and the most significant
// bit of `ds` must be set. `d_inv` should be the result of `limbs_two_limb_inverse_helper` applied
// to the two highest limbs of the denominator.
//
// # Worst-case complexity
// $T(n) = O(n (\log n)^2 \log \log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// # Panics
// Panics if `ds` has length smaller than 6, `ns` is shorter than or the same length as `ds`, `qs`
// has length less than `ns.len()` - `ds.len()`, or the last limb of `ds` does not have its highest
// bit set.
//
// This is equivalent to `mpn_dcpi1_div_q` from `mpn/generic/dcpi1_div_q.c`, GMP 6.2.1.
pub_test! {limbs_div_divide_and_conquer(
    qs: &mut [Limb],
    ns: &[Limb],
    ds: &[Limb],
    d_inv: Limb,
) -> bool {
    let n_len = ns.len();
    let d_len = ds.len();
    assert!(d_len >= 6);
    assert!(n_len - d_len >= 3);
    let q_len = n_len - d_len;
    assert!(ds[d_len - 1].get_highest_bit());
    let qs = &mut qs[..q_len];
    let mut scratch = Vec::with_capacity(n_len + 1);
    scratch.push(1);
    scratch.extend_from_slice(ns);
    let mut scratch_2 = vec![0; q_len + 1];
    let highest_q = limbs_div_divide_and_conquer_approx(&mut scratch_2, &mut scratch, ds, d_inv);
    let (scratch_2_head, scratch_2_tail) = scratch_2.split_first_mut().unwrap();
    if *scratch_2_head == 0 {
        let mut mul_scratch = vec![0; limbs_mul_to_out_scratch_len(q_len, d_len)];
        limbs_mul_to_out(&mut scratch, scratch_2_tail, ds, &mut mul_scratch);
        let scratch_init = &mut scratch[..n_len];
        // At most is wrong by one, no cycle.
        if highest_q && limbs_slice_add_same_length_in_place_left(&mut scratch_init[q_len..], ds)
            || limbs_cmp_same_length(scratch_init, ns) == Greater
        {
            return if limbs_sub_limb_to_out(qs, scratch_2_tail, 1) {
                assert!(highest_q);
                false
            } else {
                highest_q
            };
        }
    }
    qs.copy_from_slice(scratch_2_tail);
    highest_q
}}

// Divides `ns` by `ds` and writes the `ns.len()` - `ds.len()` least-significant quotient limbs to
// `qs`. Returns the most significant limb of the quotient; `true` means 1 and `false` means 0. `ds`
// must have length greater than 2, `ns` must be longer than `ds`, and the most significant bit of
// `ds` must be set.
//
// The idea of the algorithm used herein is to compute a smaller inverted value than used in the
// standard Barrett algorithm, and thus save time in the Newton iterations, and pay just a small
// price when using the inverted value for developing quotient bits. This algorithm was presented at
// ICMS 2006.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log \log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// This is equivalent to `mpn_mu_div_q` from `mpn/generic/mu_div_q.c`, GMP 6.2.1.
pub_test! {limbs_div_barrett(
    qs: &mut [Limb],
    ns: &[Limb],
    ds: &[Limb],
    scratch: &mut [Limb]
) -> bool {
    let n_len = ns.len();
    let d_len = ds.len();
    assert!(n_len > d_len);
    let q_len = n_len - d_len;
    let qs = &mut qs[..q_len];
    let q_len_plus_1 = q_len + 1;
    let mut scratch_2 = vec![0; q_len_plus_1];
    let highest_q;
    if q_len >= d_len {
        // ```
        // |_______________________|   dividend
        // |________|   divisor
        // ```
        let mut rs = Vec::with_capacity(n_len + 1);
        rs.push(0);
        rs.extend_from_slice(ns);
        let rs_hi = &mut rs[q_len_plus_1..];
        highest_q = limbs_cmp_same_length(rs_hi, ds) >= Equal;
        if highest_q {
            limbs_sub_same_length_in_place_left(rs_hi, ds);
        }
        if limbs_div_barrett_approx(&mut scratch_2, &rs, ds, scratch) {
            // Since the partial remainder fed to limbs_div_barrett_approx_preinverted was
            // canonically reduced, replace the returned value of B ^ (q_len - d_len) + epsilon by
            // the largest possible value.
            for s in &mut scratch_2 {
                *s = Limb::MAX;
            }
        }
        // The max error of limbs_div_barrett_approx is +4. If the low quotient limb is smaller than
        // the max error, we cannot trust the quotient.
        let (scratch_2_head, scratch_2_tail) = scratch_2.split_first().unwrap();
        if *scratch_2_head > 4 {
            qs.copy_from_slice(scratch_2_tail);
        } else {
            let rs = &mut rs[..n_len];
            let mut mul_scratch =
                vec![0; limbs_mul_greater_to_out_scratch_len(scratch_2_tail.len(), ds.len())];
            limbs_mul_greater_to_out(rs, scratch_2_tail, ds, &mut mul_scratch);
            if highest_q && limbs_slice_add_same_length_in_place_left(&mut rs[q_len..], ds)
                || limbs_cmp_same_length(rs, ns) == Greater
            {
                // At most is wrong by one, no cycle.
                if limbs_sub_limb_to_out(qs, scratch_2_tail, 1) {
                    fail_on_untested_path("limbs_div_barrett, limbs_sub_greater_to_out 1");
                    assert!(highest_q);
                    return false;
                }
            } else {
                qs.copy_from_slice(scratch_2_tail);
            }
        }
    } else {
        // ```
        // |_______________________|   dividend
        // |________________|   divisor
        // ```
        let ghost_n = n_len == (d_len << 1) - 1;
        highest_q = limbs_div_barrett_approx_helper(
            &mut scratch_2,
            &ns[if ghost_n {
                0
            } else {
                n_len - (q_len_plus_1 << 1)
            }..],
            ghost_n,
            &ds[d_len - q_len_plus_1..],
            scratch,
        );
        // The max error of limbs_div_barrett_approx is +4, but we get an additional error from the
        // divisor truncation.
        let (scratch_2_head, scratch_2_tail) = scratch_2.split_first().unwrap();
        if *scratch_2_head > 6 {
            qs.copy_from_slice(scratch_2_tail);
        } else {
            let mut rs = vec![0; n_len];
            let mut mul_scratch =
                vec![0; limbs_mul_greater_to_out_scratch_len(ds.len(), scratch_2_tail.len())];
            limbs_mul_greater_to_out(&mut rs, ds, scratch_2_tail, &mut mul_scratch);
            if highest_q && limbs_slice_add_same_length_in_place_left(&mut rs[q_len..], ds)
                || limbs_cmp_same_length(&rs, ns) == Greater
            {
                // At most is wrong by one, no cycle.
                if limbs_sub_limb_to_out(qs, scratch_2_tail, 1) {
                    assert!(highest_q);
                    return false;
                }
            } else {
                qs.copy_from_slice(scratch_2_tail);
            }
        }
    }
    highest_q
}}

// # Worst-case complexity
// Constant time and additional memory.
//
// The result is $O(n)$, where $n$ is `n_len`.
//
// This is equivalent to `mpn_mu_div_q_itch` from `mpn/generic/mu_div_q.c`, GMP 6.2.1, where `mua_k
// == 0`.
pub_test! {limbs_div_barrett_scratch_len(n_len: usize, d_len: usize) -> usize {
    let q_len = n_len - d_len;
    if q_len >= d_len {
        limbs_div_barrett_approx_scratch_len(n_len + 1, d_len)
    } else {
        let q_len_plus_1 = q_len + 1;
        limbs_div_barrett_approx_scratch_len(q_len_plus_1 << 1, q_len_plus_1)
    }
}}

// Schoolbook division using the Möller-Granlund 3/2 division algorithm, returning approximate
// quotient.
//
// Divides `ns` by `ds` and writes the `ns.len()` - `ds.len()` least-significant quotient limbs to
// `qs`. Returns the most significant limb of the quotient; `true` means 1 and `false` means 0. The
// quotient is either correct, or one too large. `ds` must have length greater than 2, `ns` must be
// at least as long as `ds`, and the most significant bit of `ds` must be set. `d_inv` should be the
// result of `limbs_two_limb_inverse_helper` applied to the two highest limbs of the denominator.
//
// # Worst-case complexity
// $T(n) = O(n^2)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// # Panics
// Panics if `ds` has length smaller than 3, `ns` is shorter than `ds`, `qs` has length less than
// `ns.len()` - `ds.len()`, or the last limb of `ds` does not have its highest bit set.
//
// This is equivalent to `mpn_sbpi1_divappr_q` from `mpn/generic/sbpi1_divappr_q.c`, GMP 6.2.1.
pub_crate_test! {limbs_div_schoolbook_approx(
    qs: &mut [Limb],
    ns: &mut [Limb],
    mut ds: &[Limb],
    d_inv: Limb,
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
    let ns_hi = &mut ns[n_len - d_len..];
    let highest_q = limbs_cmp_same_length(ns_hi, ds) >= Equal;
    if highest_q {
        limbs_sub_same_length_in_place_left(ns_hi, ds);
    }
    let mut n_1 = *ns.last().unwrap();
    let mut q;
    let mut n_0;
    for i in (d_len_minus_1..q_len).rev() {
        let j = i + a;
        if n_1 == d_1 && ns[j] == d_0 {
            q = Limb::MAX;
            limbs_sub_mul_limb_same_length_in_place_left(&mut ns[j - d_len_minus_1..=j], ds, q);
            n_1 = ns[j]; // update n_1, last loop's value will now be invalid
        } else {
            let n;
            (q, n) = limbs_div_mod_three_limb_by_two_limb(n_1, ns[j], ns[j - 1], d_1, d_0, d_inv);
            (n_1, n_0) = n.split_in_half();
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
                let carry = limbs_sub_mul_limb_same_length_in_place_left(&mut ns[b..=j], ds, q);
                if n_1 != carry {
                    if flag && n_1 < carry {
                        q.wrapping_sub_assign(1);
                        limbs_slice_add_same_length_in_place_left(&mut ns[b..=j], ds);
                    } else {
                        flag = false;
                    }
                }
                n_1 = ns[j];
            } else {
                let n;
                (q, n) =
                    limbs_div_mod_three_limb_by_two_limb(n_1, ns[j], ns[j - 1], d_1, d_0, d_inv);
                (n_1, n_0) = n.split_in_half();
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
                    if limbs_slice_add_same_length_in_place_left(&mut ns[b..j], &ds[..=i]) {
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
                q.wrapping_sub_assign(1);
                limbs_slice_add_same_length_in_place_left(&mut ns[..2], &ds[..2]);
            }
            n_1 = ns[1];
        } else {
            let n;
            (q, n) = limbs_div_mod_three_limb_by_two_limb(n_1, ns[1], ns[0], d_1, d_0, d_inv);
            (n_1, ns[0]) = n.split_in_half();
            ns[1] = n_1;
        }
        qs[0] = q;
    }
    assert_eq!(ns[a], n_1);
    highest_q
}}

// # Worst-case complexity
// $T(n) = O(n (\log n)^2 \log \log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ds.len()`.
//
// This is equivalent to `mpn_dcpi1_divappr_q_n` from `mpn/generic/dcpi1_divappr_q.c`, GMP 6.2.1,
// where `ns` here is `np + (n >> 1)`.
fn limbs_div_divide_and_conquer_approx_helper(
    qs: &mut [Limb],
    ns: &mut [Limb],
    ds: &[Limb],
    d_inv: Limb,
    scratch: &mut [Limb],
) -> bool {
    let d_len = ds.len();
    let lo = d_len >> 1; // floor(d_len / 2)
    let hi = d_len - lo; // ceil(d_len / 2)
    assert!(ns.len() >= d_len + hi);
    let (ds_lo, ds_hi) = ds.split_at(lo);
    let qs_hi = &mut qs[lo..];
    let ns_hi = &mut ns[lo..];
    let mut highest_q = if hi < DC_DIV_QR_THRESHOLD {
        limbs_div_mod_schoolbook(qs_hi, &mut ns_hi[..hi << 1], ds_hi, d_inv)
    } else {
        limbs_div_mod_divide_and_conquer_helper(qs_hi, ns_hi, ds_hi, d_inv, scratch)
    };
    let mut mul_scratch = vec![0; limbs_mul_greater_to_out_scratch_len(hi, ds_lo.len())];
    limbs_mul_greater_to_out(scratch, &qs_hi[..hi], ds_lo, &mut mul_scratch);
    let ns_lo = &mut ns[..d_len];
    let mut carry = Limb::from(limbs_sub_same_length_in_place_left(
        ns_lo,
        &scratch[..d_len],
    ));
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
    let ds_hi = &ds[hi..];
    let ns_hi = &mut ns[hi - lo..];
    let q_lo = if lo < DC_DIVAPPR_Q_THRESHOLD {
        limbs_div_schoolbook_approx(qs, &mut ns_hi[..lo << 1], ds_hi, d_inv)
    } else {
        limbs_div_divide_and_conquer_approx_helper(qs, &mut ns_hi[lo >> 1..], ds_hi, d_inv, scratch)
    };
    if q_lo {
        for q in &mut qs[..lo] {
            *q = Limb::MAX;
        }
    }
    highest_q
}

// Recursive divide-and-conquer division, returning approximate quotient.
//
// Divides `ns` by `ds` and writes the `ns.len()` - `ds.len()` least-significant quotient limbs to
// `qs`. Returns the most significant limb of the quotient; `true` means 1 and `false` means 0. The
// quotient is either correct, or one too large. `ds` must have length greater than 2, `ns` must be
// at least as long as `ds`, and the most significant bit of `ds` must be set. `d_inv` should be the
// result of `limbs_two_limb_inverse_helper` applied to the two highest limbs of the denominator.
//
// # Worst-case complexity
// $T(n) = O(n (\log n)^2 \log \log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// # Panics
// Panics if `ds` has length smaller than 6, `ns` is shorter than or the same length as `ds`, `qs`
// has length less than `ns.len()` - `ds.len()`, or the last limb of `ds` does not have its highest
// bit set.
//
// This is equivalent to `mpn_dcpi1_divappr_q` from `mpn/generic/dcpi1_divappr_q.c`, GMP 6.2.1.
pub_crate_test! {limbs_div_divide_and_conquer_approx(
    qs: &mut [Limb],
    ns: &mut [Limb],
    ds: &[Limb],
    d_inv: Limb,
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
        let offset = q_len - q_len_mod_d_len;
        let ns_hi = &mut ns[offset..];
        let qs_hi = &mut qs[offset..];
        let r = d_len - q_len_mod_d_len;
        let (ds_lo, ds_hi) = ds.split_at(r);
        // Perform the typically smaller block first.
        if q_len_mod_d_len == 1 {
            // Handle highest_q up front, for simplicity.
            let ns_2 = &mut ns_hi[1..=d_len];
            highest_q = limbs_cmp_same_length(ns_2, ds) >= Equal;
            if highest_q {
                assert!(!limbs_sub_same_length_in_place_left(ns_2, ds));
            }
            // A single iteration of schoolbook: One 3/2 division, followed by the bignum update and
            // adjustment.
            let n_2 = ns_hi[d_len];
            let mut n_1 = ns_hi[a];
            let mut n_0 = ns_hi[b];
            let d_1 = ds[a];
            let d_0 = ds[b];
            assert!(n_2 < d_1 || (n_2 == d_1 && n_1 <= d_0));
            let mut q;
            if n_2 == d_1 && n_1 == d_0 {
                q = Limb::MAX;
                assert_eq!(
                    limbs_sub_mul_limb_same_length_in_place_left(&mut ns_hi[..d_len], ds, q),
                    n_2
                );
            } else {
                let n;
                (q, n) = limbs_div_mod_three_limb_by_two_limb(n_2, n_1, n_0, d_1, d_0, d_inv);
                (n_1, n_0) = n.split_in_half();
                // d_len > 2
                let local_carry_1 =
                    limbs_sub_mul_limb_same_length_in_place_left(&mut ns_hi[..b], &ds[..b], q);
                let local_carry_2 = n_0 < local_carry_1;
                n_0.wrapping_sub_assign(local_carry_1);
                let carry = local_carry_2 && n_1 == 0;
                if local_carry_2 {
                    n_1.wrapping_sub_assign(1);
                }
                ns_hi[b] = n_0;
                if carry {
                    n_1.wrapping_add_assign(d_1);
                    if limbs_slice_add_same_length_in_place_left(&mut ns_hi[..a], &ds[..a]) {
                        n_1.wrapping_add_assign(1);
                    }
                    if q == 0 {
                        fail_on_untested_path("limbs_div_divide_and_conquer_approx, q == 0");
                        assert!(highest_q);
                        highest_q = false;
                    }
                    q.wrapping_sub_assign(1);
                }
                ns_hi[a] = n_1;
            }
            qs_hi[0] = q;
        } else {
            let ns_hi_2 = &mut ns_hi[r..];
            highest_q = if q_len_mod_d_len == 2 {
                limbs_div_mod_by_two_limb_normalized(
                    qs_hi,
                    &mut ns_hi_2[..q_len_mod_d_len + 2],
                    ds_hi,
                )
            } else if q_len_mod_d_len < DC_DIV_QR_THRESHOLD {
                limbs_div_mod_schoolbook(qs_hi, ns_hi_2, ds_hi, d_inv)
            } else {
                limbs_div_mod_divide_and_conquer_helper(qs_hi, ns_hi_2, ds_hi, d_inv, &mut scratch)
            };
            let qs = &mut qs_hi[..q_len_mod_d_len];
            if q_len_mod_d_len != d_len {
                let mut mul_scratch = vec![0; limbs_mul_to_out_scratch_len(qs.len(), ds_lo.len())];
                limbs_mul_to_out(&mut scratch, qs, ds_lo, &mut mul_scratch);
                let mut carry = Limb::from(limbs_sub_same_length_in_place_left(
                    &mut ns_hi[..d_len],
                    &scratch[..d_len],
                ));
                if highest_q
                    && limbs_sub_same_length_in_place_left(
                        &mut ns_hi[q_len_mod_d_len..d_len],
                        ds_lo,
                    )
                {
                    carry += 1;
                }
                while carry != 0 {
                    if limbs_sub_limb_in_place(qs, 1) {
                        assert!(highest_q);
                        highest_q = false;
                    }
                    if limbs_slice_add_same_length_in_place_left(&mut ns_hi[..d_len], ds) {
                        carry -= 1;
                    }
                }
            }
        }
        let mut offset = q_len.checked_sub(q_len_mod_d_len).unwrap();
        while offset >= d_len {
            offset -= d_len;
            limbs_div_mod_divide_and_conquer_helper(
                &mut qs[offset..],
                &mut ns[offset..],
                ds,
                d_inv,
                &mut scratch,
            );
        }
        // Since we pretended we'd need an extra quotient limb before, we now have made sure the
        // code above left just ds.len() - 1 = qs.len() quotient limbs to develop. Develop that plus
        // a guard limb.
        let ns = &mut ns[offset + (d_len >> 1) - d_len..];
        let q_save = qs[offset];
        limbs_div_divide_and_conquer_approx_helper(qs, ns, ds, d_inv, &mut scratch);
        slice_move_left(&mut qs[..=offset], 1);
        qs[offset] = q_save;
    } else {
        let offset = a - q_len;
        let q_len_plus_one = q_len + 1;
        let mut qs_2 = vec![0; q_len_plus_one];
        let ds = &ds[offset..];
        highest_q = if q_len < DC_DIVAPPR_Q_THRESHOLD && offset > 0 {
            limbs_div_schoolbook_approx(&mut qs_2, &mut ns[offset - 1..], ds, d_inv)
        } else {
            let mut scratch = vec![0; q_len_plus_one];
            limbs_div_divide_and_conquer_approx_helper(
                &mut qs_2,
                &mut ns[offset + (q_len_plus_one >> 1) - 1..],
                ds,
                d_inv,
                &mut scratch,
            )
        };
        qs[..q_len].copy_from_slice(&qs_2[1..]);
    }
    highest_q
}}

// Compute Q = floor(N / D) + e. N has n_len limbs, D has d_len limbs and must be normalized, and Q
// must have n_len - d_len limbs, 0 <= e <= 4. The requirement that Q has n_len - d_len limbs (and
// not n_len - d_len + 1 limbs) was put in place in order to allow us to let N be unmodified during
// the operation.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log \log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// This is equivalent to `mpn_mu_divappr_q` from `mpn/generic/mu_divappr_q.c`, GMP 6.2.1.
pub_crate_test! {limbs_div_barrett_approx(
    qs: &mut [Limb],
    ns: &[Limb],
    ds: &[Limb],
    scratch: &mut [Limb],
) -> bool {
    limbs_div_barrett_approx_helper(qs, ns, false, ds, scratch)
}}

fn limbs_div_barrett_approx_helper(
    qs: &mut [Limb],
    ns: &[Limb],
    mut ns_ghost_limb: bool,
    ds: &[Limb],
    scratch: &mut [Limb],
) -> bool {
    let n_len = if ns_ghost_limb {
        ns.len() + 1
    } else {
        ns.len()
    };
    let d_len = ds.len();
    assert!(d_len > 1);
    assert!(n_len >= d_len);
    assert!(ds[d_len - 1].get_highest_bit());
    let q_len = n_len - d_len;
    // If Q is smaller than D, truncate operands.
    let (ns, ds) = if q_len + 1 < d_len {
        let start = d_len - q_len - 1; // start > 0
        if ns_ghost_limb {
            ns_ghost_limb = false;
            (&ns[start - 1..], &ds[start..])
        } else {
            (&ns[start..], &ds[start..])
        }
    } else {
        (ns, ds)
    };
    let d_len_s = ds.len();
    // Compute the inverse size.
    let i_len = limbs_div_barrett_approx_is_len(q_len, d_len_s);
    assert!(i_len <= d_len_s);
    let n = i_len + 1;
    let (is, scratch_2) = scratch.split_at_mut(n);
    // compute an approximate inverse on i_len + 1 limbs
    if d_len_s == i_len {
        scratch_2[1..n].copy_from_slice(&ds[..i_len]);
        scratch_2[0] = 1;
        let (scratch_2_lo, scratch_2_hi) = scratch_2.split_at_mut(n);
        limbs_invert_approx(is, scratch_2_lo, scratch_2_hi);
        slice_move_left(is, 1);
    } else if limbs_add_limb_to_out(scratch_2, &ds[d_len_s - n..], 1) {
        slice_set_zero(&mut is[..i_len]);
    } else {
        let (scratch_2_lo, scratch_2_hi) = scratch_2.split_at_mut(n);
        limbs_invert_approx(is, scratch_2_lo, scratch_2_hi);
        slice_move_left(is, 1);
    }
    let (is, scratch_hi) = scratch.split_at_mut(i_len);
    limbs_div_barrett_approx_preinverted(qs, ns, ns_ghost_limb, ds, is, scratch_hi)
}

// $T(n, d) = O(n \log d \log\log d)$
//
// $M(n) = O(d \log d)$
//
// where $T$ is time, $M$ is additional memory, $n$ is `ns.len()`, and $d$ is `ds.len()`.
//
// This is equivalent to `mpn_preinv_mu_divappr_q` from `mpn/generic/mu_divappr_q.c`, GMP 6.2.1.
fn limbs_div_barrett_approx_preinverted(
    qs: &mut [Limb],
    ns: &[Limb],
    ns_ghost_limb: bool,
    ds: &[Limb],
    mut is: &[Limb],
    scratch: &mut [Limb],
) -> bool {
    let n_len = if ns_ghost_limb {
        ns.len() + 1
    } else {
        ns.len()
    };
    let d_len = ds.len();
    let mut i_len = is.len();
    let mut q_len = n_len - d_len;
    let qs = &mut qs[..q_len];
    if ns_ghost_limb {
        assert_ne!(q_len, 0);
        assert_ne!(i_len, 0);
    }
    let (ns_lo, ns_hi) = ns.split_at(if ns_ghost_limb { q_len - 1 } else { q_len });
    let highest_q = limbs_cmp_same_length(ns_hi, ds) >= Equal;
    if q_len == 0 {
        return highest_q;
    }
    let (rs, scratch) = scratch.split_at_mut(d_len);
    if highest_q {
        limbs_sub_same_length_to_out(rs, ns_hi, ds);
    } else {
        rs.copy_from_slice(ns_hi);
    }
    let scratch_len = if i_len < MUL_TO_MULMOD_BNM1_FOR_2NXN_THRESHOLD {
        0
    } else {
        limbs_mul_mod_base_pow_n_minus_1_next_size(d_len + 1)
    };
    let mut carry = false; // This value is never used
    let mut n = d_len - i_len;
    let empty_slice: &[Limb] = &[];
    let ns_iter: Box<dyn Iterator<Item = &[Limb]>> = if ns_ghost_limb {
        Box::new(ns_lo.rchunks(i_len).chain(once(empty_slice)))
    } else {
        Box::new(ns_lo.rchunks(i_len))
    };
    let mut mul_scratch = vec![0; limbs_mul_same_length_to_out_scratch_len(i_len)];
    for (ns, qs) in ns_iter.zip(qs.rchunks_mut(i_len)) {
        let chunk_len = qs.len();
        if i_len != chunk_len {
            // last iteration
            is = &is[i_len - chunk_len..];
            i_len = chunk_len;
            n = d_len - i_len;
        }
        let (rs_lo, rs_hi) = rs.split_at_mut(n);
        // Compute the next block of quotient limbs by multiplying the inverse I by the upper part
        // of the partial remainder R.
        limbs_mul_same_length_to_out(scratch, rs_hi, is, &mut mul_scratch);
        // i's highest bit is implicit
        carry = limbs_add_same_length_to_out(qs, &scratch[i_len..i_len << 1], rs_hi);
        assert!(!carry);
        q_len -= i_len;
        if q_len == 0 {
            break;
        }
        // Compute the product of the quotient block and the divisor D, to be subtracted from the
        // partial remainder combined with new limbs from the dividend N. We only really need the
        // low d_len limbs.
        if i_len < MUL_TO_MULMOD_BNM1_FOR_2NXN_THRESHOLD {
            let mut mul_scratch = vec![0; limbs_mul_greater_to_out_scratch_len(ds.len(), qs.len())];
            limbs_mul_greater_to_out(scratch, ds, qs, &mut mul_scratch);
        } else {
            limbs_div_barrett_large_product(scratch, ds, qs, rs_hi, scratch_len, i_len);
        }
        let mut r = rs_hi[0].wrapping_sub(scratch[d_len]);
        // Subtract the product from the partial remainder combined with new limbs from the dividend
        // N, generating a new partial remainder R.
        let scratch = &mut scratch[..d_len];
        if n == 0 {
            // get next i_len limbs from N
            carry = limbs_sub_same_length_to_out(rs, ns, scratch);
        } else {
            // get next i_len limbs from N.
            let (scratch_lo, scratch_hi) = scratch.split_at_mut(i_len);
            carry = limbs_sub_same_length_with_borrow_in_in_place_right(
                rs_lo,
                scratch_hi,
                limbs_sub_same_length_in_place_right(ns, scratch_lo),
            );
            rs.copy_from_slice(scratch);
        }
        // Check the remainder R and adjust the quotient as needed.
        if carry {
            r.wrapping_sub_assign(1);
        }
        while r != 0 {
            // We loop 0 times with about 69% probability, 1 time with about 31% probability, and 2
            // times with about 0.6% probability, if inverse is computed as recommended.
            assert!(!limbs_slice_add_limb_in_place(qs, 1));
            carry = limbs_sub_same_length_in_place_left(rs, ds);
            if carry {
                r -= 1;
            }
        }
        if limbs_cmp_same_length(rs, ds) >= Equal {
            // This is executed with about 76% probability.
            assert!(!limbs_slice_add_limb_in_place(qs, 1));
            carry = limbs_sub_same_length_in_place_left(rs, ds);
        }
    }
    if limbs_slice_add_limb_in_place(qs, 3) || carry {
        if highest_q {
            // Return a quotient of just 1-bits, with highest_q set.
            for q in &mut *qs {
                *q = Limb::MAX;
            }
        }
        true
    } else {
        highest_q
    }
}

// We distinguish 3 cases:
//
// - d_len < q_len:              i_len = ceil(q_len / ceil(q_len / d_len))
// - d_len / 3 < q_len <= d_len: i_len = ceil(q_len / 2)
// - q_len < d_len / 3:          i_len = q_len
//
// In all cases we have i_len <= d_len.
//
// # Worst-case complexity
// Constant time and additional memory.
//
// The result is $O(n)$, where $n$ is `q_len`.
//
// This is equivalent to `mpn_mu_divappr_q_choose_in` from `mpn/generic/mu_divappr_q.c`, GMP 6.2.1,
// where `k == 0`.
#[allow(clippy::missing_const_for_fn)]
fn limbs_div_barrett_approx_is_len(q_len: usize, d_len: usize) -> usize {
    if q_len > d_len {
        // Compute an inverse size that is a nice partition of the quotient.
        let b = q_len.saturating_sub(1) / d_len + 1; // ceil(q_len / d_len), number of blocks
        q_len.saturating_sub(1) / b + 1 // ceil(q_len / b) = ceil(q_len / ceil(q_len / d_len))
    } else if 3 * q_len > d_len {
        q_len.saturating_sub(1) / 2 + 1 // b = 2
    } else {
        q_len.saturating_sub(1) + 1 // b = 1
    }
}

// # Worst-case complexity
// Constant time and additional memory.
//
// The result is $O(n)$, where $n$ is `n_len`.
//
// This is equivalent to `mpn_mu_divappr_q_itch` from `mpn/generic/mu_divappr_q.c`, GMP 6.2.1, where
// `mua_k == 0`.
pub_crate_test! {limbs_div_barrett_approx_scratch_len(n_len: usize, mut d_len: usize) -> usize {
    let qn = n_len - d_len;
    if qn + 1 < d_len {
        d_len = qn + 1;
    }
    let is_len = limbs_div_barrett_approx_is_len(qn, d_len);
    let local_len = limbs_mul_mod_base_pow_n_minus_1_next_size(d_len + 1);
    let out_len = limbs_mul_mod_base_pow_n_minus_1_scratch_len(local_len, d_len, is_len);
    // 3 * is_len + 4
    let inv_approx_len = limbs_invert_approx_scratch_len(is_len + 1) + is_len + 2;
    assert!(d_len + local_len + out_len >= inv_approx_len);
    is_len + d_len + local_len + out_len
}}

// TODO tune
const DC_DIV_Q_THRESHOLD: usize = DC_DIVAPPR_Q_THRESHOLD;
// TODO tune
const MU_DIV_Q_THRESHOLD: usize = MU_DIVAPPR_Q_THRESHOLD;
// TODO tune
const MUPI_DIV_Q_THRESHOLD: usize = MUPI_DIVAPPR_Q_THRESHOLD;
// TODO tune
const MUPI_DIVAPPR_Q_THRESHOLD: usize = MUPI_DIV_QR_THRESHOLD;

fn limbs_div_dc_condition(n_len: usize, d_len: usize) -> bool {
    let n_64 = n_len as f64;
    let d_64 = d_len as f64;
    d_len < MUPI_DIV_Q_THRESHOLD
        || n_len < MU_DIV_Q_THRESHOLD << 1
        || libm::fma(
            ((MU_DIV_Q_THRESHOLD - MUPI_DIV_Q_THRESHOLD) << 1) as f64,
            d_64,
            MUPI_DIV_Q_THRESHOLD as f64 * n_64,
        ) > d_64 * n_64
}

// Division when n_len >= 2 * d_len - FUDGE.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log \log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
pub_crate_test! {limbs_div_to_out_unbalanced(qs: &mut [Limb], ns: &mut [Limb], ds: &mut [Limb]) {
    // ```
    // |________________________|
    //                  |_______|
    // ```
    let n_len = ns.len();
    let d_len = ds.len();
    let highest_d = ds[d_len - 1];
    let bits = LeadingZeros::leading_zeros(highest_d);
    if bits == 0 {
        let highest_q = if d_len == 2 {
            limbs_div_mod_by_two_limb_normalized(qs, ns, ds)
        } else if d_len < DC_DIV_Q_THRESHOLD || n_len - d_len < DC_DIV_Q_THRESHOLD {
            let d_inv = limbs_two_limb_inverse_helper(highest_d, ds[d_len - 2]);
            limbs_div_schoolbook(qs, ns, ds, d_inv)
        } else if limbs_div_dc_condition(n_len, d_len) {
            let d_inv = limbs_two_limb_inverse_helper(highest_d, ds[d_len - 2]);
            limbs_div_divide_and_conquer(qs, ns, ds, d_inv)
        } else {
            let mut scratch = vec![0; limbs_div_barrett_scratch_len(n_len, d_len)];
            limbs_div_barrett(qs, ns, ds, &mut scratch)
        };
        qs[n_len - d_len] = Limb::from(highest_q);
    } else {
        let mut scratch = vec![0; n_len + 1];
        let carry = limbs_shl_to_out(&mut scratch, ns, bits);
        scratch[n_len] = carry;
        let new_n_len = if carry == 0 { n_len } else { n_len + 1 };
        let new_ns = &mut scratch[..new_n_len];
        limbs_slice_shl_in_place(ds, bits);
        let highest_q = if d_len == 2 {
            limbs_div_mod_by_two_limb_normalized(qs, new_ns, ds)
        } else if d_len < DC_DIV_Q_THRESHOLD || new_n_len - d_len < DC_DIV_Q_THRESHOLD {
            let d_inv = limbs_two_limb_inverse_helper(ds[d_len - 1], ds[d_len - 2]);
            limbs_div_schoolbook(qs, new_ns, ds, d_inv)
        } else if limbs_div_dc_condition(n_len, d_len) {
            let d_inv = limbs_two_limb_inverse_helper(ds[d_len - 1], ds[d_len - 2]);
            limbs_div_divide_and_conquer(qs, new_ns, ds, d_inv)
        } else {
            let mut scratch = vec![0; limbs_div_barrett_scratch_len(new_n_len, d_len)];
            limbs_div_barrett(qs, new_ns, ds, &mut scratch)
        };
        if carry == 0 {
            qs[n_len - d_len] = Limb::from(highest_q);
        } else {
            assert!(!highest_q);
        }
    }
}}

pub_test! {limbs_div_q_dc_helper(qs: &mut [Limb], ns: &mut [Limb], ds: &[Limb]) -> bool {
    let n_len = ns.len();
    let d_len = ds.len();
    let highest_d = ds[d_len - 1];
    if d_len < DC_DIV_Q_THRESHOLD || ns.len() - d_len < DC_DIV_Q_THRESHOLD {
        let d_inv = limbs_two_limb_inverse_helper(highest_d, ds[d_len - 2]);
        limbs_div_schoolbook(qs, ns, ds, d_inv)
    } else if limbs_div_dc_condition(n_len, d_len) {
        let d_inv = limbs_two_limb_inverse_helper(highest_d, ds[d_len - 2]);
        limbs_div_divide_and_conquer(qs, ns, ds, d_inv)
    } else {
        let mut scratch = vec![0; limbs_div_barrett_scratch_len(n_len, d_len)];
        limbs_div_barrett(qs, ns, ds, &mut scratch)
    }
}}

/// Division when n_len >= 2 * d_len - FUDGE.
///
// # Worst-case complexity
// $T(n) = O(n \log n \log \log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
fn limbs_div_to_out_unbalanced_val_ref(qs: &mut [Limb], ns: &mut [Limb], ds: &[Limb]) {
    // ```
    // |________________________|
    //                  |_______|
    // ```
    let n_len = ns.len();
    let d_len = ds.len();
    let highest_d = ds[d_len - 1];
    let bits = LeadingZeros::leading_zeros(highest_d);
    if bits == 0 {
        let highest_q = if d_len == 2 {
            limbs_div_mod_by_two_limb_normalized(qs, ns, ds)
        } else {
            limbs_div_q_dc_helper(qs, ns, ds)
        };
        qs[n_len - d_len] = Limb::from(highest_q);
    } else {
        let mut scratch = vec![0; n_len + 1];
        let carry = limbs_shl_to_out(&mut scratch, ns, bits);
        scratch[n_len] = carry;
        let new_n_len = if carry == 0 { n_len } else { n_len + 1 };
        let new_ns = &mut scratch[..new_n_len];
        let mut new_ds = vec![0; d_len];
        limbs_shl_to_out(&mut new_ds, ds, bits);
        let highest_q = if d_len == 2 {
            limbs_div_mod_by_two_limb_normalized(qs, new_ns, &new_ds)
        } else if d_len < DC_DIV_Q_THRESHOLD || new_n_len - d_len < DC_DIV_Q_THRESHOLD {
            let d_inv = limbs_two_limb_inverse_helper(new_ds[d_len - 1], new_ds[d_len - 2]);
            limbs_div_schoolbook(qs, new_ns, &new_ds, d_inv)
        } else if limbs_div_dc_condition(n_len, d_len) {
            let d_inv = limbs_two_limb_inverse_helper(new_ds[d_len - 1], new_ds[d_len - 2]);
            limbs_div_divide_and_conquer(qs, new_ns, &new_ds, d_inv)
        } else {
            let mut scratch = vec![0; limbs_div_barrett_scratch_len(new_n_len, d_len)];
            limbs_div_barrett(qs, new_ns, &new_ds, &mut scratch)
        };
        if carry == 0 {
            qs[n_len - d_len] = Limb::from(highest_q);
        } else {
            assert!(!highest_q);
        }
    }
}

// Division when n_len >= 2 * d_len - FUDGE.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log \log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
fn limbs_div_to_out_unbalanced_ref_val(qs: &mut [Limb], ns: &[Limb], ds: &mut [Limb]) {
    // ```
    // |________________________|
    //                  |_______|
    // ```
    let n_len = ns.len();
    let d_len = ds.len();
    let highest_d = ds[d_len - 1];
    let bits = LeadingZeros::leading_zeros(highest_d);
    if bits == 0 {
        let highest_q = if d_len == 2 {
            let mut new_ns = ns.to_vec();
            limbs_div_mod_by_two_limb_normalized(qs, &mut new_ns, ds)
        } else if d_len < DC_DIV_Q_THRESHOLD || n_len - d_len < DC_DIV_Q_THRESHOLD {
            let d_inv = limbs_two_limb_inverse_helper(highest_d, ds[d_len - 2]);
            let mut new_ns = ns.to_vec();
            limbs_div_schoolbook(qs, &mut new_ns, ds, d_inv)
        } else if limbs_div_dc_condition(n_len, d_len) {
            let d_inv = limbs_two_limb_inverse_helper(highest_d, ds[d_len - 2]);
            limbs_div_divide_and_conquer(qs, ns, ds, d_inv)
        } else {
            let mut scratch = vec![0; limbs_div_barrett_scratch_len(n_len, d_len)];
            limbs_div_barrett(qs, ns, ds, &mut scratch)
        };
        qs[n_len - d_len] = Limb::from(highest_q);
    } else {
        let mut scratch = vec![0; n_len + 1];
        let carry = limbs_shl_to_out(&mut scratch, ns, bits);
        scratch[n_len] = carry;
        let new_n_len = if carry == 0 { n_len } else { n_len + 1 };
        let new_ns = &mut scratch[..new_n_len];
        limbs_slice_shl_in_place(ds, bits);
        let highest_q = if d_len == 2 {
            limbs_div_mod_by_two_limb_normalized(qs, new_ns, ds)
        } else if d_len < DC_DIV_Q_THRESHOLD || new_n_len - d_len < DC_DIV_Q_THRESHOLD {
            let d_inv = limbs_two_limb_inverse_helper(ds[d_len - 1], ds[d_len - 2]);
            limbs_div_schoolbook(qs, new_ns, ds, d_inv)
        } else if limbs_div_dc_condition(n_len, d_len) {
            let d_inv = limbs_two_limb_inverse_helper(ds[d_len - 1], ds[d_len - 2]);
            limbs_div_divide_and_conquer(qs, new_ns, ds, d_inv)
        } else {
            let mut scratch = vec![0; limbs_div_barrett_scratch_len(new_n_len, d_len)];
            limbs_div_barrett(qs, new_ns, ds, &mut scratch)
        };
        if carry == 0 {
            qs[n_len - d_len] = Limb::from(highest_q);
        } else {
            assert!(!highest_q);
        }
    }
}

// Division when n_len >= 2 * d_len - FUDGE.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log \log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
fn limbs_div_to_out_unbalanced_ref_ref(qs: &mut [Limb], ns: &[Limb], ds: &[Limb]) {
    // ```
    // |________________________|
    //                  |_______|
    // ```
    let n_len = ns.len();
    let d_len = ds.len();
    let highest_d = ds[d_len - 1];
    let bits = LeadingZeros::leading_zeros(highest_d);
    if bits == 0 {
        let highest_q = if d_len == 2 {
            let mut new_ns = ns.to_vec();
            limbs_div_mod_by_two_limb_normalized(qs, &mut new_ns, ds)
        } else if d_len < DC_DIV_Q_THRESHOLD || n_len - d_len < DC_DIV_Q_THRESHOLD {
            let d_inv = limbs_two_limb_inverse_helper(highest_d, ds[d_len - 2]);
            let mut new_ns = ns.to_vec();
            limbs_div_schoolbook(qs, &mut new_ns, ds, d_inv)
        } else if limbs_div_dc_condition(n_len, d_len) {
            let d_inv = limbs_two_limb_inverse_helper(highest_d, ds[d_len - 2]);
            limbs_div_divide_and_conquer(qs, ns, ds, d_inv)
        } else {
            let mut scratch = vec![0; limbs_div_barrett_scratch_len(n_len, d_len)];
            limbs_div_barrett(qs, ns, ds, &mut scratch)
        };
        qs[n_len - d_len] = Limb::from(highest_q);
    } else {
        let mut scratch = vec![0; n_len + 1];
        let carry = limbs_shl_to_out(&mut scratch, ns, bits);
        scratch[n_len] = carry;
        let new_n_len = if carry == 0 { n_len } else { n_len + 1 };
        let new_ns = &mut scratch[..new_n_len];
        let mut new_ds = vec![0; d_len];
        limbs_shl_to_out(&mut new_ds, ds, bits);
        let highest_q = if d_len == 2 {
            limbs_div_mod_by_two_limb_normalized(qs, new_ns, &new_ds)
        } else if d_len < DC_DIV_Q_THRESHOLD || new_n_len - d_len < DC_DIV_Q_THRESHOLD {
            let d_inv = limbs_two_limb_inverse_helper(new_ds[d_len - 1], new_ds[d_len - 2]);
            limbs_div_schoolbook(qs, new_ns, &new_ds, d_inv)
        } else if limbs_div_dc_condition(n_len, d_len) {
            let d_inv = limbs_two_limb_inverse_helper(new_ds[d_len - 1], new_ds[d_len - 2]);
            limbs_div_divide_and_conquer(qs, new_ns, &new_ds, d_inv)
        } else {
            let mut scratch = vec![0; limbs_div_barrett_scratch_len(new_n_len, d_len)];
            limbs_div_barrett(qs, new_ns, &new_ds, &mut scratch)
        };
        if carry == 0 {
            qs[n_len - d_len] = Limb::from(highest_q);
        } else {
            assert!(!highest_q);
        }
    }
}

// Division when n_len < 2 * d_len - FUDGE.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log \log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
pub_test! {limbs_div_to_out_balanced(qs: &mut [Limb], ns: &[Limb], ds: &[Limb]) {
    // ```
    // |________________________|
    //        |_________________|
    // ```
    let n_len = ns.len();
    let d_len = ds.len();
    let q_len = n_len - d_len + 1;
    let q_len_plus_1 = q_len + 1;
    let mut scratch_2 = vec![0; q_len_plus_1];
    let new_n_len = q_len + q_len_plus_1;
    let ns_tail = &ns[n_len.checked_sub(new_n_len).unwrap()..];
    let highest_d = ds[d_len - 1];
    let bits = LeadingZeros::leading_zeros(highest_d);
    if bits == 0 {
        let new_ds = &ds[d_len - q_len_plus_1..];
        let highest_q = if q_len_plus_1 == 2 {
            let mut new_ns = ns_tail.to_vec();
            limbs_div_mod_by_two_limb_normalized(&mut scratch_2, &mut new_ns, new_ds)
        } else if q_len_plus_1 < DC_DIVAPPR_Q_THRESHOLD {
            let d_inv = limbs_two_limb_inverse_helper(highest_d, new_ds[q_len - 1]);
            let mut new_ns = ns_tail.to_vec();
            limbs_div_schoolbook_approx(&mut scratch_2, &mut new_ns, new_ds, d_inv)
        } else if q_len_plus_1 < MU_DIVAPPR_Q_THRESHOLD {
            let d_inv = limbs_two_limb_inverse_helper(highest_d, new_ds[q_len - 1]);
            let mut new_ns = ns_tail.to_vec();
            limbs_div_divide_and_conquer_approx(&mut scratch_2, &mut new_ns, new_ds, d_inv)
        } else {
            let mut scratch =
                vec![0; limbs_div_barrett_approx_scratch_len(new_n_len, q_len_plus_1)];
            limbs_div_barrett_approx(&mut scratch_2, ns_tail, new_ds, &mut scratch)
        };
        scratch_2[q_len] = Limb::from(highest_q);
    } else {
        let mut scratch = vec![0; n_len + 1];
        let carry = limbs_shl_to_out(&mut scratch, ns_tail, bits);
        scratch[new_n_len] = carry;
        let new_n_len = if carry == 0 { new_n_len } else { new_n_len + 1 };
        let new_ns = &mut scratch[..new_n_len];
        let mut new_ds = vec![0; q_len_plus_1];
        limbs_shl_to_out(&mut new_ds, &ds[d_len - q_len_plus_1..], bits);
        new_ds[0] |= ds[d_len - q_len_plus_1 - 1] >> (Limb::WIDTH - bits);
        let highest_q = if q_len_plus_1 == 2 {
            limbs_div_mod_by_two_limb_normalized(&mut scratch_2, new_ns, &new_ds)
        } else if q_len_plus_1 < DC_DIVAPPR_Q_THRESHOLD {
            let d_inv = limbs_two_limb_inverse_helper(new_ds[q_len], new_ds[q_len - 1]);
            limbs_div_schoolbook_approx(&mut scratch_2, new_ns, &new_ds, d_inv)
        } else if q_len_plus_1 < MU_DIVAPPR_Q_THRESHOLD {
            let d_inv = limbs_two_limb_inverse_helper(new_ds[q_len], new_ds[q_len - 1]);
            limbs_div_divide_and_conquer_approx(&mut scratch_2, new_ns, &new_ds, d_inv)
        } else {
            let mut scratch =
                vec![0; limbs_div_barrett_approx_scratch_len(new_n_len, q_len_plus_1)];
            limbs_div_barrett_approx(&mut scratch_2, new_ns, &new_ds, &mut scratch)
        };
        if carry == 0 {
            scratch_2[q_len] = Limb::from(highest_q);
        } else if highest_q {
            fail_on_untested_path("limbs_div_to_out_balanced, highest_q");
            // This happens only when the quotient is close to B ^ n and one of the approximate
            // division functions returned B ^ n.
            for s in &mut scratch_2[..new_n_len - q_len_plus_1] {
                *s = Limb::MAX;
            }
        }
    }
    let (scratch_2_head, scratch_2_tail) = scratch_2.split_first().unwrap();
    qs[..q_len].copy_from_slice(scratch_2_tail);
    if *scratch_2_head <= 4 {
        let mut rs = vec![0; n_len + 1];
        let mut mul_scratch =
            vec![0; limbs_mul_greater_to_out_scratch_len(ds.len(), scratch_2_tail.len())];
        limbs_mul_greater_to_out(&mut rs, ds, scratch_2_tail, &mut mul_scratch);
        let r_len = if rs[n_len] == 0 { n_len } else { n_len + 1 };
        if r_len > n_len || limbs_cmp_same_length(ns, &rs[..n_len]) == Less {
            assert!(!limbs_sub_limb_in_place(qs, 1));
        }
    }
}}

// Interpreting two slices of `Limb`s, `ns` and `ds`, as the limbs (in ascending order) of two
// `Natural`s, divides them, returning the quotient. The quotient has `ns.len() - ds.len() + 1`
// limbs.
//
// `ns` must be at least as long as `ds` and `ds` must have length at least 2 and its most
// significant limb must be greater than zero.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log \log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// # Panics
// Panics if `ns` is shorter than `ds`, `ds` has length less than 2, or the most-significant limb of
// `ds` is zero.
//
// This is equivalent to `mpn_div_q` from `mpn/generic/div_q.c`, GMP 6.2.1, where `scratch` is
// allocated internally and `qp` is returned.
pub_test! {limbs_div(ns: &[Limb], ds: &[Limb]) -> Vec<Limb> {
    let mut qs = vec![0; ns.len() - ds.len() + 1];
    limbs_div_to_out_ref_ref(&mut qs, ns, ds);
    qs
}}

// Interpreting two slices of `Limb`s, `ns` and `ds`, as the limbs (in ascending order) of two
// `Natural`s, divides them, writing the `ns.len() - ds.len() + 1` limbs of the quotient to `qs`.
//
// `ns` must be at least as long as `ds`, `qs` must have length at least `ns.len() - ds.len() + 1`,
// and `ds` must have length at least 2 and its most significant limb must be greater than zero.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log \log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// # Panics
// Panics if `qs` is too short, `ns` is shorter than `ds`, `ds` has length less than 2, or the
// most-significant limb of `ds` is zero.
//
// This is equivalent to `mpn_div_q` from `mpn/generic/div_q.c`, GMP 6.2.1, where `scratch` is
// allocated internally and `np` and `dp` are consumed, saving some memory allocations.
pub_crate_test! {limbs_div_to_out(qs: &mut [Limb], ns: &mut [Limb], ds: &mut [Limb]) {
    let n_len = ns.len();
    let d_len = ds.len();
    assert!(n_len >= d_len);
    assert!(d_len > 1);
    assert_ne!(ds[d_len - 1], 0);
    assert!(FUDGE >= 2);
    let q_len = n_len - d_len + 1; // Quotient size, high limb might be zero
    if q_len + FUDGE >= d_len {
        limbs_div_to_out_unbalanced(qs, ns, ds);
    } else {
        limbs_div_to_out_balanced(qs, ns, ds);
    }
}}

// Interpreting two slices of `Limb`s, `ns` and `ds`, as the limbs (in ascending order) of two
// `Natural`s, divides them, writing the `ns.len() - ds.len() + 1` limbs of the quotient to `qs`.
//
// `ns` must be at least as long as `ds`, `qs` must have length at least `ns.len() - ds.len() + 1`,
// and `ds` must have length at least 2 and its most significant limb must be greater than zero.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log \log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// # Panics
// Panics if `qs` is too short, `ns` is shorter than `ds`, `ds` has length less than 2, or the
// most-significant limb of `ds` is zero.
//
// This is equivalent to `mpn_div_q` from `mpn/generic/div_q.c`, GMP 6.2.1, where `scratch` is
// allocated internally and `np` is consumed, saving some memory allocations.
pub_test! {limbs_div_to_out_val_ref(qs: &mut [Limb], ns: &mut [Limb], ds: &[Limb]) {
    let n_len = ns.len();
    let d_len = ds.len();
    assert!(n_len >= d_len);
    assert!(d_len > 1);
    assert_ne!(ds[d_len - 1], 0);
    assert!(FUDGE >= 2);
    let q_len = n_len - d_len + 1; // Quotient size, high limb might be zero
    if q_len + FUDGE >= d_len {
        limbs_div_to_out_unbalanced_val_ref(qs, ns, ds);
    } else {
        limbs_div_to_out_balanced(qs, ns, ds);
    }
}}

// Interpreting two slices of `Limb`s, `ns` and `ds`, as the limbs (in ascending order) of two
// `Natural`s, divides them, writing the `ns.len() - ds.len() + 1` limbs of the quotient to `qs`.
//
// `ns` must be at least as long as `ds`, `qs` must have length at least `ns.len() - ds.len() + 1`,
// and `ds` must have length at least 2 and its most significant limb must be greater than zero.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log \log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// # Panics
// Panics if `qs` is too short, `ns` is shorter than `ds`, `ds` has length less than 2, or the
// most-significant limb of `ds` is zero.
//
// This is equivalent to `mpn_div_q` from `mpn/generic/div_q.c`, GMP 6.2.1, where `scratch` is
// allocated internally and `dp` is consumed, saving some memory allocations.
pub_test! {limbs_div_to_out_ref_val(qs: &mut [Limb], ns: &[Limb], ds: &mut [Limb]) {
    let n_len = ns.len();
    let d_len = ds.len();
    assert!(n_len >= d_len);
    assert!(d_len > 1);
    assert_ne!(ds[d_len - 1], 0);
    assert!(FUDGE >= 2);
    let q_len = n_len - d_len + 1; // Quotient size, high limb might be zero
    if q_len + FUDGE >= d_len {
        limbs_div_to_out_unbalanced_ref_val(qs, ns, ds);
    } else {
        limbs_div_to_out_balanced(qs, ns, ds);
    }
}}

// Interpreting two slices of `Limb`s, `ns` and `ds`, as the limbs (in ascending order) of two
// `Natural`s, divides them, writing the `ns.len() - ds.len() + 1` limbs of the quotient to `qs`.
//
// `ns` must be at least as long as `ds`, `qs` must have length at least `ns.len() - ds.len() + 1`,
// and `ds` must have length at least 2 and its most significant limb must be greater than zero.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log \log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// # Panics
// Panics if `qs` is too short, `ns` is shorter than `ds`, `ds` has length less than 2, or the
// most-significant limb of `ds` is zero.
//
// This is equivalent to `mpn_div_q` from `mpn/generic/div_q.c`, GMP 6.2.1, where `scratch` is
// allocated internally.
pub_test! {limbs_div_to_out_ref_ref(qs: &mut [Limb], ns: &[Limb], ds: &[Limb]) {
    let n_len = ns.len();
    let d_len = ds.len();
    assert!(n_len >= d_len);
    assert!(d_len > 1);
    assert_ne!(ds[d_len - 1], 0);
    assert!(FUDGE >= 2);
    let q_len = n_len - d_len + 1; // Quotient size, high limb might be zero
    if q_len + FUDGE >= d_len {
        limbs_div_to_out_unbalanced_ref_ref(qs, ns, ds);
    } else {
        limbs_div_to_out_balanced(qs, ns, ds);
    }
}}

// Divides using the naive (schoolbook) algorithm.
//
// # Worst-case complexity
// Constant time and additional memory.
#[cfg(feature = "test_build")]
fn limbs_div_in_place_naive(ns: &mut [Limb], d: Limb) {
    let limb = DoubleLimb::from(d);
    let mut upper = 0;
    for n in ns.iter_mut().rev() {
        let lower = *n;
        let (q, r) = DoubleLimb::join_halves(upper, lower).div_rem(limb);
        *n = q.lower_half();
        upper = r.lower_half();
    }
}

// This is equivalent to `mpn_pi1_bdiv_q_1` from `mpn/generic/bdiv_q_1.c`, GMP 6.2.1, where rp ==
// up.
pub(crate) fn limbs_hensel_div_limb_in_place(
    ns: &mut [Limb],
    d: Limb,
    d_inv: Limb,
    shift: u64,
) -> bool {
    let n_len = ns.len();
    assert_ne!(n_len, 0);
    assert_ne!(d, 0);
    let mut carry = 0;
    if shift == 0 {
        let (ns_head, ns_tail) = ns.split_first_mut().unwrap();
        let mut l = ns_head.wrapping_mul(d_inv);
        *ns_head = l;
        let mut carry_2 = false;
        for n in ns_tail {
            let mut carry = Limb::x_mul_y_to_zz(l, d).0;
            if carry_2 {
                carry += 1;
            }
            (l, carry_2) = n.overflowing_sub(carry);
            l.wrapping_mul_assign(d_inv);
            *n = l;
        }
        carry_2
    } else {
        for i in 0..n_len - 1 {
            let n = (ns[i] >> shift) | (ns[i + 1] << (Limb::WIDTH - shift));
            let (mut l, carry_2) = n.overflowing_sub(carry);
            l.wrapping_mul_assign(d_inv);
            ns[i] = l;
            carry = Limb::x_mul_y_to_zz(l, d).0;
            if carry_2 {
                carry += 1;
            }
        }
        let ns_last = ns.last_mut().unwrap();
        let (l, carry_2) = (*ns_last >> shift).overflowing_sub(carry);
        *ns_last = l.wrapping_mul(d_inv);
        carry_2
    }
}

impl Natural {
    fn div_limb_ref(&self, other: Limb) -> Natural {
        match (self, other) {
            (_, 0) => panic!("division by zero"),
            (n, 1) => n.clone(),
            (Natural(Small(small)), other) => Natural(Small(small / other)),
            (Natural(Large(ref limbs)), other) => {
                Natural::from_owned_limbs_asc(limbs_div_limb(limbs, other))
            }
        }
    }

    #[cfg(feature = "test_build")]
    #[inline]
    pub fn div_limb_naive(mut self, other: Limb) -> Natural {
        self.div_assign_limb_naive(other);
        self
    }

    fn div_assign_limb(&mut self, other: Limb) {
        match (&mut *self, other) {
            (_, 0) => panic!("division by zero"),
            (_, 1) => {}
            (Natural(Small(ref mut small)), other) => *small /= other,
            (Natural(Large(ref mut limbs)), other) => {
                limbs_div_limb_in_place(limbs, other);
                self.trim();
            }
        }
    }

    #[cfg(feature = "test_build")]
    pub fn div_assign_limb_naive(&mut self, other: Limb) {
        match (&mut *self, other) {
            (_, 0) => panic!("division by zero"),
            (_, 1) => {}
            (Natural(Small(ref mut small)), other) => {
                *small /= other;
            }
            (Natural(Large(ref mut limbs)), other) => {
                limbs_div_in_place_naive(limbs, other);
                self.trim();
            }
        }
    }
}

impl Div<Natural> for Natural {
    type Output = Natural;

    /// Divides a [`Natural`] by another [`Natural`], taking both by value. The quotient is rounded
    /// towards negative infinity. The quotient and remainder (which is not computed) satisfy $x =
    /// qy + r$ and $0 \leq r < y$.
    ///
    /// $$
    /// f(x, y) = \left \lfloor \frac{x}{y} \right \rfloor.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(Natural::from(23u32) / Natural::from(10u32), 2);
    ///
    /// // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    /// assert_eq!(
    ///     Natural::from_str("1000000000000000000000000").unwrap()
    ///         / Natural::from_str("1234567890987").unwrap(),
    ///     810000006723u64
    /// );
    /// ```
    #[inline]
    fn div(mut self, other: Natural) -> Natural {
        self /= other;
        self
    }
}

impl<'a> Div<&'a Natural> for Natural {
    type Output = Natural;

    /// Divides a [`Natural`] by another [`Natural`], taking the first by value and the second by
    /// reference. The quotient is rounded towards negative infinity. The quotient and remainder
    /// (which is not computed) satisfy $x = qy + r$ and $0 \leq r < y$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(Natural::from(23u32) / &Natural::from(10u32), 2);
    ///
    /// // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    /// assert_eq!(
    ///     Natural::from_str("1000000000000000000000000").unwrap()
    ///         / &Natural::from_str("1234567890987").unwrap(),
    ///     810000006723u64
    /// );
    /// ```
    #[inline]
    fn div(mut self, other: &'a Natural) -> Natural {
        self /= other;
        self
    }
}

impl<'a> Div<Natural> for &'a Natural {
    type Output = Natural;

    /// Divides a [`Natural`] by another [`Natural`], taking the first by reference and the second
    /// by value. The quotient is rounded towards negative infinity. The quotient and remainder
    /// (which is not computed) satisfy $x = qy + r$ and $0 \leq r < y$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(&Natural::from(23u32) / Natural::from(10u32), 2);
    ///
    /// // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    /// assert_eq!(
    ///     &Natural::from_str("1000000000000000000000000").unwrap()
    ///         / Natural::from_str("1234567890987").unwrap(),
    ///     810000006723u64
    /// );
    /// ```
    fn div(self, mut other: Natural) -> Natural {
        match (self, &mut other) {
            (_, &mut Natural::ZERO) => panic!("division by zero"),
            (x, y) if x == y => Natural::ONE,
            (n, &mut Natural::ONE) => n.clone(),
            (n, &mut Natural(Small(d))) => n.div_limb_ref(d),
            (Natural(Small(_)), _) => Natural::ZERO,
            (&Natural(Large(ref ns)), &mut Natural(Large(ref mut ds))) => {
                let ns_len = ns.len();
                let ds_len = ds.len();
                if ns_len < ds_len {
                    Natural::ZERO
                } else {
                    let mut qs = vec![0; ns_len - ds_len + 1];
                    limbs_div_to_out_ref_val(&mut qs, ns, ds);
                    Natural::from_owned_limbs_asc(qs)
                }
            }
        }
    }
}

impl<'a, 'b> Div<&'b Natural> for &'a Natural {
    type Output = Natural;

    /// Divides a [`Natural`] by another [`Natural`], taking both by reference. The quotient is
    /// rounded towards negative infinity. The quotient and remainder (which is not computed)
    /// satisfy $x = qy + r$ and $0 \leq r < y$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(&Natural::from(23u32) / &Natural::from(10u32), 2);
    ///
    /// // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    /// assert_eq!(
    ///     &Natural::from_str("1000000000000000000000000").unwrap()
    ///         / &Natural::from_str("1234567890987").unwrap(),
    ///     810000006723u64
    /// );
    /// ```
    fn div(self, other: &'b Natural) -> Natural {
        match (self, other) {
            (_, &Natural::ZERO) => panic!("division by zero"),
            (x, y) if x == y => Natural::ONE,
            (n, &Natural::ONE) => n.clone(),
            (n, &Natural(Small(d))) => n.div_limb_ref(d),
            (Natural(Small(_)), _) => Natural::ZERO,
            (&Natural(Large(ref ns)), &Natural(Large(ref ds))) => {
                if ns.len() < ds.len() {
                    Natural::ZERO
                } else {
                    Natural::from_owned_limbs_asc(limbs_div(ns, ds))
                }
            }
        }
    }
}

impl DivAssign<Natural> for Natural {
    /// Divides a [`Natural`] by another [`Natural`] in place, taking the [`Natural`] on the
    /// right-hand side by value. The quotient is rounded towards negative infinity. The quotient
    /// and remainder (which is not computed) satisfy $x = qy + r$ and $0 \leq r < y$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 10 + 3 = 23
    /// let mut x = Natural::from(23u32);
    /// x /= Natural::from(10u32);
    /// assert_eq!(x, 2);
    ///
    /// // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    /// let mut x = Natural::from_str("1000000000000000000000000").unwrap();
    /// x /= Natural::from_str("1234567890987").unwrap();
    /// assert_eq!(x, 810000006723u64);
    /// ```
    fn div_assign(&mut self, other: Natural) {
        match (&mut *self, other) {
            (_, Natural::ZERO) => panic!("division by zero"),
            (x, y) if *x == y => {
                *self = Natural::ONE;
            }
            (_, Natural::ONE) => {}
            (n, Natural(Small(d))) => n.div_assign_limb(d),
            (Natural(Small(_)), _) => *self = Natural::ZERO,
            (&mut Natural(Large(ref mut ns)), Natural(Large(ref mut ds))) => {
                let ns_len = ns.len();
                let ds_len = ds.len();
                if ns_len < ds_len {
                    *self = Natural::ZERO;
                } else {
                    let mut qs = vec![0; ns_len - ds_len + 1];
                    limbs_div_to_out(&mut qs, ns, ds);
                    swap(&mut qs, ns);
                    self.trim();
                }
            }
        }
    }
}

impl<'a> DivAssign<&'a Natural> for Natural {
    /// Divides a [`Natural`] by another [`Natural`] in place, taking the [`Natural`] on the
    /// right-hand side by reference. The quotient is rounded towards negative infinity. The
    /// quotient and remainder (which is not computed) satisfy $x = qy + r$ and $0 \leq r < y$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 10 + 3 = 23
    /// let mut x = Natural::from(23u32);
    /// x /= &Natural::from(10u32);
    /// assert_eq!(x, 2);
    ///
    /// // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    /// let mut x = Natural::from_str("1000000000000000000000000").unwrap();
    /// x /= &Natural::from_str("1234567890987").unwrap();
    /// assert_eq!(x, 810000006723u64);
    /// ```
    fn div_assign(&mut self, other: &'a Natural) {
        match (&mut *self, other) {
            (_, &Natural::ZERO) => panic!("division by zero"),
            (x, y) if x == y => {
                *self = Natural::ONE;
            }
            (_, &Natural::ONE) => {}
            (n, &Natural(Small(d))) => n.div_assign_limb(d),
            (Natural(Small(_)), _) => *self = Natural::ZERO,
            (&mut Natural(Large(ref mut ns)), Natural(Large(ref ds))) => {
                let ns_len = ns.len();
                let ds_len = ds.len();
                if ns_len < ds_len {
                    *self = Natural::ZERO;
                } else {
                    let mut qs = vec![0; ns_len - ds_len + 1];
                    limbs_div_to_out_val_ref(&mut qs, ns, ds);
                    swap(&mut qs, ns);
                    self.trim();
                }
            }
        }
    }
}

impl CheckedDiv<Natural> for Natural {
    type Output = Natural;

    /// Divides a [`Natural`] by another [`Natural`], taking both by value. The quotient is rounded
    /// towards negative infinity. The quotient and remainder (which is not computed) satisfy $x =
    /// qy + r$ and $0 \leq r < y$. Returns `None` when the second [`Natural`] is zero, `Some`
    /// otherwise.
    ///
    /// $$
    /// f(x, y) = \begin{cases}
    ///     \operatorname{Some}\left ( \left \lfloor \frac{x}{y} \right \rfloor \right ) &
    ///         \text{if} \\quad y \neq 0 \\\\
    ///     \text{None} & \text{otherwise}
    /// \end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CheckedDiv;
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(
    ///     Natural::from(23u32)
    ///         .checked_div(Natural::from(10u32))
    ///         .to_debug_string(),
    ///     "Some(2)"
    /// );
    /// assert_eq!(Natural::ONE.checked_div(Natural::ZERO), None);
    /// ```
    #[inline]
    fn checked_div(self, mut other: Natural) -> Option<Natural> {
        match (self, &mut other) {
            (_, &mut Natural::ZERO) => None,
            (x, y) if x == *y => Some(Natural::ONE),
            (n, &mut Natural::ONE) => Some(n),
            (mut n, &mut Natural(Small(d))) => {
                n.div_assign_limb(d);
                Some(n)
            }
            (Natural(Small(_)), _) => Some(Natural::ZERO),
            (Natural(Large(mut ns)), &mut Natural(Large(ref mut ds))) => {
                let ns_len = ns.len();
                let ds_len = ds.len();
                Some(if ns_len < ds_len {
                    Natural::ZERO
                } else {
                    let mut qs = vec![0; ns_len - ds_len + 1];
                    limbs_div_to_out(&mut qs, &mut ns, ds);
                    Natural::from_owned_limbs_asc(qs)
                })
            }
        }
    }
}

impl<'a> CheckedDiv<&'a Natural> for Natural {
    type Output = Natural;

    /// Divides a [`Natural`] by another [`Natural`], taking the first by value and the second by
    /// reference. The quotient is rounded towards negative infinity. The quotient and remainder
    /// (which is not computed) satisfy $x = qy + r$ and $0 \leq r < y$. Returns `None` when the
    /// second [`Natural`] is zero, `Some` otherwise.
    ///
    /// $$
    /// f(x, y) = \begin{cases}
    ///     \operatorname{Some}\left ( \left \lfloor \frac{x}{y} \right \rfloor \right ) &
    ///         \text{if} \\quad y \neq 0 \\\\
    ///     \text{None} & \text{otherwise}
    /// \end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CheckedDiv;
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(
    ///     Natural::from(23u32)
    ///         .checked_div(&Natural::from(10u32))
    ///         .to_debug_string(),
    ///     "Some(2)"
    /// );
    /// assert_eq!(Natural::ONE.checked_div(&Natural::ZERO), None);
    /// ```
    #[inline]
    fn checked_div(self, other: &'a Natural) -> Option<Natural> {
        match (self, other) {
            (_, &Natural::ZERO) => None,
            (x, y) if x == *y => Some(Natural::ONE),
            (n, &Natural::ONE) => Some(n.clone()),
            (mut n, &Natural(Small(d))) => {
                n.div_assign_limb(d);
                Some(n)
            }
            (Natural(Small(_)), _) => Some(Natural::ZERO),
            (Natural(Large(mut ns)), &Natural(Large(ref ds))) => {
                let ns_len = ns.len();
                let ds_len = ds.len();
                Some(if ns_len < ds_len {
                    Natural::ZERO
                } else {
                    let mut qs = vec![0; ns_len - ds_len + 1];
                    limbs_div_to_out_val_ref(&mut qs, &mut ns, ds);
                    Natural::from_owned_limbs_asc(qs)
                })
            }
        }
    }
}

impl<'a> CheckedDiv<Natural> for &'a Natural {
    type Output = Natural;

    /// Divides a [`Natural`] by another [`Natural`], taking the first by reference and the second
    /// by value. The quotient is rounded towards negative infinity. The quotient and remainder
    /// (which is not computed) satisfy $x = qy + r$ and $0 \leq r < y$. Returns `None` when the
    /// second [`Natural`] is zero, `Some` otherwise.
    ///
    /// $$
    /// f(x, y) = \begin{cases}
    ///     \operatorname{Some}\left ( \left \lfloor \frac{x}{y} \right \rfloor \right ) &
    ///         \text{if} \\quad y \neq 0 \\\\
    ///     \text{None} & \text{otherwise}
    /// \end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CheckedDiv;
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(
    ///     (&Natural::from(23u32))
    ///         .checked_div(Natural::from(10u32))
    ///         .to_debug_string(),
    ///     "Some(2)"
    /// );
    /// assert_eq!((&Natural::ONE).checked_div(Natural::ZERO), None);
    /// ```
    fn checked_div(self, mut other: Natural) -> Option<Natural> {
        match (self, &mut other) {
            (_, &mut Natural::ZERO) => None,
            (x, y) if x == y => Some(Natural::ONE),
            (n, &mut Natural::ONE) => Some(n.clone()),
            (n, &mut Natural(Small(d))) => Some(n.div_limb_ref(d)),
            (Natural(Small(_)), _) => Some(Natural::ZERO),
            (&Natural(Large(ref ns)), &mut Natural(Large(ref mut ds))) => {
                let ns_len = ns.len();
                let ds_len = ds.len();
                Some(if ns_len < ds_len {
                    Natural::ZERO
                } else {
                    let mut qs = vec![0; ns_len - ds_len + 1];
                    limbs_div_to_out_ref_val(&mut qs, ns, ds);
                    Natural::from_owned_limbs_asc(qs)
                })
            }
        }
    }
}

impl<'a, 'b> CheckedDiv<&'b Natural> for &'a Natural {
    type Output = Natural;

    /// Divides a [`Natural`] by another [`Natural`], taking both by reference. The quotient is
    /// rounded towards negative infinity. The quotient and remainder (which is not computed)
    /// satisfy $x = qy + r$ and $0 \leq r < y$. Returns `None` when the second [`Natural`] is zero,
    /// `Some` otherwise.
    ///
    /// $$
    /// f(x, y) = \begin{cases}
    ///     \operatorname{Some}\left ( \left \lfloor \frac{x}{y} \right \rfloor \right ) &
    /// \text{if} \\quad y \neq 0 \\\\
    ///     \text{None} & \text{otherwise}
    /// \end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CheckedDiv;
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(
    ///     (&Natural::from(23u32))
    ///         .checked_div(&Natural::from(10u32))
    ///         .to_debug_string(),
    ///     "Some(2)"
    /// );
    /// assert_eq!((&Natural::ONE).checked_div(&Natural::ZERO), None);
    /// ```
    fn checked_div(self, other: &'b Natural) -> Option<Natural> {
        match (self, other) {
            (_, &Natural::ZERO) => None,
            (x, y) if x == y => Some(Natural::ONE),
            (n, &Natural::ONE) => Some(n.clone()),
            (n, &Natural(Small(d))) => Some(n.div_limb_ref(d)),
            (Natural(Small(_)), _) => Some(Natural::ZERO),
            (&Natural(Large(ref ns)), &Natural(Large(ref ds))) => Some(if ns.len() < ds.len() {
                Natural::ZERO
            } else {
                Natural::from_owned_limbs_asc(limbs_div(ns, ds))
            }),
        }
    }
}
