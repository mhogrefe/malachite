// Copyright © 2024 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      `mpn_dcpi1_div_qr`, `mpn_dcpi1_div_qr_n`, `mpn_preinv_mu_div_qr_itch`,
//      `mpn_preinv_mu_div_qr`, `mpn_mu_div_qr_choose_in`, `mpn_mu_div_qr2`, `mpn_mu_div_qr`,
//      `mpn_mu_div_qr_itch`, and `mpn_sbpi1_div_qr` contributed to the GNU project by Torbjörn
//      Granlund.
//
//      `mpn_invertappr`, `mpn_bc_invertappr`, and `mpn_ni_invertappr` contributed to the GNU
//      project by Marco Bodrato. The algorithm used here was inspired by ApproximateReciprocal from
//      "Modern Computer Arithmetic", by Richard P. Brent and Paul Zimmermann. Special thanks to
//      Paul Zimmermann for his very valuable suggestions on all the theoretical aspects during the
//      work on this code.
//
//      Copyright © 1991-2018 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::add::{
    limbs_add_limb_to_out, limbs_add_same_length_to_out,
    limbs_add_same_length_with_carry_in_in_place_left, limbs_add_same_length_with_carry_in_to_out,
    limbs_slice_add_limb_in_place, limbs_slice_add_same_length_in_place_left,
};
use crate::natural::arithmetic::div::{
    limbs_div_divide_and_conquer_approx, limbs_div_schoolbook_approx,
};
use crate::natural::arithmetic::mul::mul_mod::{
    limbs_mul_mod_base_pow_n_minus_1, limbs_mul_mod_base_pow_n_minus_1_next_size,
    limbs_mul_mod_base_pow_n_minus_1_scratch_len,
};
use crate::natural::arithmetic::mul::{
    limbs_mul_greater_to_out, limbs_mul_greater_to_out_scratch_len, limbs_mul_same_length_to_out,
    limbs_mul_same_length_to_out_scratch_len, limbs_mul_to_out, limbs_mul_to_out_scratch_len,
};
use crate::natural::arithmetic::shl::{limbs_shl_to_out, limbs_slice_shl_in_place};
use crate::natural::arithmetic::shr::{limbs_shr_to_out, limbs_slice_shr_in_place};
use crate::natural::arithmetic::sub::{
    limbs_sub_greater_in_place_left, limbs_sub_limb_in_place, limbs_sub_same_length_in_place_left,
    limbs_sub_same_length_in_place_right, limbs_sub_same_length_to_out,
    limbs_sub_same_length_with_borrow_in_in_place_left,
    limbs_sub_same_length_with_borrow_in_in_place_right,
    limbs_sub_same_length_with_borrow_in_to_out,
};
use crate::natural::arithmetic::sub_mul::limbs_sub_mul_limb_same_length_in_place_left;
use crate::natural::comparison::cmp::limbs_cmp_same_length;
use crate::natural::logic::not::limbs_not_to_out;
use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::{
    DoubleLimb, Limb, DC_DIVAPPR_Q_THRESHOLD, DC_DIV_QR_THRESHOLD, INV_MULMOD_BNM1_THRESHOLD,
    INV_NEWTON_THRESHOLD, MAYBE_DCP1_DIVAPPR, MU_DIV_QR_SKEW_THRESHOLD, MU_DIV_QR_THRESHOLD,
};
use alloc::vec::Vec;
use core::cmp::{min, Ordering::*};
use core::mem::swap;
use malachite_base::num::arithmetic::traits::{
    CeilingDivAssignNegMod, CeilingDivNegMod, DivAssignMod, DivAssignRem, DivMod, DivRem,
    WrappingAddAssign, WrappingSub, WrappingSubAssign, XMulYToZZ, XXDivModYToQR,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::{JoinHalves, SplitInHalf};
use malachite_base::num::logic::traits::LeadingZeros;
use malachite_base::slices::{slice_move_left, slice_set_zero};

// The highest bit of the input must be set.
//
// # Worst-case complexity
// Constant time and additional memory.
//
// # Panics
// Panics if `d` is zero.
//
// This is equivalent to `mpn_invert_limb`, or `invert_limb`, from `gmp-impl.h`, GMP 6.2.1.
pub_crate_test! {limbs_invert_limb(d: Limb) -> Limb {
    (DoubleLimb::join_halves(!d, Limb::MAX) / DoubleLimb::from(d)).lower_half()
}}

// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `udiv_qrnnd_preinv` from `gmp-impl.h`, GMP 6.2.1.
pub_crate_test! {div_mod_by_preinversion(
    n_high: Limb,
    n_low: Limb,
    d: Limb,
    d_inv: Limb
) -> (Limb, Limb) {
    let (mut q_high, q_low) = (DoubleLimb::from(n_high) * DoubleLimb::from(d_inv))
        .wrapping_add(DoubleLimb::join_halves(n_high.wrapping_add(1), n_low))
        .split_in_half();
    let mut r = n_low.wrapping_sub(q_high.wrapping_mul(d));
    if r > q_low {
        let (r_plus_d, overflow) = r.overflowing_add(d);
        if overflow {
            q_high.wrapping_sub_assign(1);
            r = r_plus_d;
        }
    } else if r >= d {
        q_high.wrapping_add_assign(1);
        r -= d;
    }
    (q_high, r)
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
// quotient limbs and remainder of the `Natural` divided by a `Limb`. The divisor limb cannot be
// zero and the limb slice must have at least two elements.
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
// This is equivalent to `mpn_divrem_1` from `mpn/generic/divrem_1.c`, GMP 6.2.1, where `qxn == 0`,
// `un > 1`, and both results are returned. Experiments show that `DIVREM_1_NORM_THRESHOLD` and
// `DIVREM_1_UNNORM_THRESHOLD` are unnecessary (they would always be 0).
pub_test! {limbs_div_limb_mod(ns: &[Limb], d: Limb) -> (Vec<Limb>, Limb) {
    let mut qs = vec![0; ns.len()];
    let r = limbs_div_limb_to_out_mod(&mut qs, ns, d);
    (qs, r)
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
// limbs of the quotient of the `Natural` and a `Limb` to an output slice, and returns the
// remainder. The output slice must be at least as long as the input slice. The divisor limb cannot
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
// Panics if `out` is shorter than `ns`, the length of `ns` is less than 2, or if `d` is zero.
//
// This is equivalent to `mpn_divrem_1` from `mpn/generic/divrem_1.c`, GMP 6.2.1, where `qxn == 0`
// and `un > 1`. Experiments show that `DIVREM_1_NORM_THRESHOLD` and `DIVREM_1_UNNORM_THRESHOLD` are
// unnecessary (they would always be 0).
pub_crate_test! {limbs_div_limb_to_out_mod(out: &mut [Limb], ns: &[Limb], d: Limb) -> Limb {
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
        r
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
        let (out_head, out_tail) = out.split_first_mut().unwrap();
        for (out_q, &n) in out_tail.iter_mut().zip(ns_init.iter()).rev() {
            let n_shifted = (previous_n << bits) | (n >> cobits);
            (*out_q, r) = div_mod_by_preinversion(r, n_shifted, d, d_inv);
            previous_n = n;
        }
        let out_r;
        (*out_head, out_r) = div_mod_by_preinversion(r, previous_n << bits, d, d_inv);
        out_r >> bits
    }
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
// limbs of the quotient of the `Natural` and a `Limb` to the input slice and returns the remainder.
// The divisor limb cannot be zero and the input limb slice must have at least two elements.
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
// `qxn == 0`, and `un > 1`. Experiments show that `DIVREM_1_NORM_THRESHOLD` and
// `DIVREM_1_UNNORM_THRESHOLD` are unnecessary (they would always be 0).
pub_crate_test! {limbs_div_limb_in_place_mod(ns: &mut [Limb], d: Limb) -> Limb {
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
        r
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
        let out_r;
        (ns[0], out_r) = div_mod_by_preinversion(r, previous_n << bits, d, d_inv);
        out_r >> bits
    }
}}

// Let `ns` be the limbs of a `Natural` $n$, and let $f$ be `fraction_len`. This function performs
// the integer division $B^fn / d$, writing the `ns.len() + fraction_len` limbs of the quotient to
// `out` and returning the remainder.
//
// `shift` must be the number of leading zeros of `d`, and `d_inv` must be `limbs_invert_limb(d <<
// shift)`.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len() + fraction_len`.
//
// # Panics
// Panics if `out` is shorter than `ns.len()` + `fraction_len`, if `ns` is empty, or if `d` is zero.
//
// This is equivalent to `mpn_preinv_divrem_1` from `mpn/generic/pre_divrem_1.c`, GMP 6.2.1, where
// `qp != ap`.
pub_test! {limbs_div_mod_extra(
    out: &mut [Limb],
    fraction_len: usize,
    mut ns: &[Limb],
    d: Limb,
    d_inv: Limb,
    shift: u64,
) -> Limb {
    assert!(!ns.is_empty());
    assert_ne!(d, 0);
    let (ns_last, ns_init) = ns.split_last().unwrap();
    let ns_last = *ns_last;
    let d_norm = d << shift;
    let (fraction_out, integer_out) = out.split_at_mut(fraction_len);
    let mut integer_out = &mut integer_out[..ns.len()];
    let mut r;
    if shift == 0 {
        r = ns_last;
        let q_high = r >= d_norm;
        if r >= d_norm {
            r -= d_norm;
        }
        let (integer_out_last, integer_out_init) = integer_out.split_last_mut().unwrap();
        *integer_out_last = Limb::from(q_high);
        for (q, &n) in integer_out_init.iter_mut().zip(ns_init.iter()).rev() {
            (*q, r) = div_mod_by_preinversion(r, n, d_norm, d_inv);
        }
    } else {
        r = 0;
        if ns_last < d {
            r = ns_last << shift;
            let integer_out_last;
            (integer_out_last, integer_out) = integer_out.split_last_mut().unwrap();
            *integer_out_last = 0;
            ns = ns_init;
        }
        if !ns.is_empty() {
            let co_shift = Limb::WIDTH - shift;
            let (ns_last, ns_init) = ns.split_last().unwrap();
            let mut previous_n = *ns_last;
            r |= previous_n >> co_shift;
            let (integer_out_head, integer_out_tail) = integer_out.split_first_mut().unwrap();
            for (q, &n) in integer_out_tail.iter_mut().zip(ns_init.iter()).rev() {
                assert!(r < d_norm);
                (*q, r) = div_mod_by_preinversion(
                    r,
                    (previous_n << shift) | (n >> co_shift),
                    d_norm,
                    d_inv,
                );
                previous_n = n;
            }
            (*integer_out_head, r) = div_mod_by_preinversion(r, previous_n << shift, d_norm, d_inv);
        }
    }
    for q in fraction_out.iter_mut().rev() {
        (*q, r) = div_mod_by_preinversion(r, 0, d_norm, d_inv);
    }
    r >> shift
}}

// Let `&ns[fraction_len..]` be the limbs of a `Natural` $n$, and let $f$ be `fraction_len`. This
// function performs the integer division $B^fn / d$, writing the `ns.len() + fraction_len` limbs of
// the quotient to `ns` and returning the remainder.
//
// `shift` must be the number of leading zeros of `d`, and `d_inv` must be `limbs_invert_limb(d <<
// shift)`.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len() + fraction_len`.
//
// # Panics
// Panics if `ns` is empty, if `ns.len()` is less than `fraction_len`, or if `d` is zero.
//
// This is equivalent to `mpn_preinv_divrem_1` from `mpn/generic/pre_divrem_1.c`, GMP 6.2.1, where
// `qp == ap`.
pub_crate_test! {limbs_div_mod_extra_in_place(
    ns: &mut [Limb],
    fraction_len: usize,
    d: Limb,
    d_inv: Limb,
    shift: u64,
) -> Limb {
    assert_ne!(d, 0);
    let (fraction_ns, mut integer_ns) = ns.split_at_mut(fraction_len);
    let ns_last = *integer_ns.last().unwrap();
    let d_norm = d << shift;
    let mut r;
    if shift == 0 {
        r = ns_last;
        let q_high = r >= d_norm;
        if r >= d_norm {
            r -= d_norm;
        }
        let (integer_ns_last, integer_ns_init) = integer_ns.split_last_mut().unwrap();
        *integer_ns_last = Limb::from(q_high);
        for q in integer_ns_init.iter_mut().rev() {
            (*q, r) = div_mod_by_preinversion(r, *q, d_norm, d_inv);
        }
    } else {
        r = 0;
        if ns_last < d {
            r = ns_last << shift;
            let integer_ns_last;
            (integer_ns_last, integer_ns) = integer_ns.split_last_mut().unwrap();
            *integer_ns_last = 0;
        }
        if !integer_ns.is_empty() {
            let co_shift = Limb::WIDTH - shift;
            let mut previous_n = *integer_ns.last().unwrap();
            r |= previous_n >> co_shift;
            for i in (1..integer_ns.len()).rev() {
                assert!(r < d_norm);
                let n = integer_ns[i - 1];
                (integer_ns[i], r) = div_mod_by_preinversion(
                    r,
                    (previous_n << shift) | (n >> co_shift),
                    d_norm,
                    d_inv,
                );
                previous_n = n;
            }
            (integer_ns[0], r) = div_mod_by_preinversion(r, previous_n << shift, d_norm, d_inv);
        }
    }
    for q in fraction_ns.iter_mut().rev() {
        (*q, r) = div_mod_by_preinversion(r, 0, d_norm, d_inv);
    }
    r >> shift
}}

// Computes floor((B ^ 3 - 1) / (`hi` * B + `lo`)) - B, where B = 2 ^ `Limb::WIDTH`, assuming the
// highest bit of `hi` is set.
//
// # Worst-case complexity
// Constant time and additional memory.
//
// # Panics
// Panics if `hi` is zero.
//
// This is equivalent to `invert_pi1` from `gmp-impl.h`, GMP 6.2.1, where the result is returned
// instead of being written to `dinv`.
pub_crate_test! {limbs_two_limb_inverse_helper(hi: Limb, lo: Limb) -> Limb {
    let mut d_inv = limbs_invert_limb(hi);
    let mut hi_product = hi.wrapping_mul(d_inv);
    hi_product.wrapping_add_assign(lo);
    if hi_product < lo {
        d_inv.wrapping_sub_assign(1);
        if hi_product >= hi {
            hi_product.wrapping_sub_assign(hi);
            d_inv.wrapping_sub_assign(1);
        }
        hi_product.wrapping_sub_assign(hi);
    }
    let (lo_product_hi, lo_product_lo) = Limb::x_mul_y_to_zz(lo, d_inv);
    hi_product.wrapping_add_assign(lo_product_hi);
    if hi_product < lo_product_hi {
        d_inv.wrapping_sub_assign(1);
        if hi_product > hi || hi_product == hi && lo_product_lo >= lo {
            d_inv.wrapping_sub_assign(1);
        }
    }
    d_inv
}}

// Computes the quotient and remainder of `[n_2, n_1, n_0]` / `[d_1, d_0]`. Requires the highest bit
// of `d_1` to be set, and `[n_2, n_1]` < `[d_1, d_0]`. `d_inv` is the inverse of `[d_1, d_0]`
// computed by `limbs_two_limb_inverse_helper`.
//
// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `udiv_qr_3by2` from `gmp-impl.h`, GMP 6.2.1.
pub_crate_test! {limbs_div_mod_three_limb_by_two_limb(
    n_2: Limb,
    n_1: Limb,
    n_0: Limb,
    d_1: Limb,
    d_0: Limb,
    d_inv: Limb,
) -> (Limb, DoubleLimb) {
    let (mut q, q_lo) = (DoubleLimb::from(n_2) * DoubleLimb::from(d_inv))
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
}}

// Divides `ns` by `ds` and writes the `ns.len()` - 2 least-significant quotient limbs to `qs` and
// the 2-long remainder to `ns`. Returns the most significant limb of the quotient; `true` means 1
// and `false` means 0. `ds` must have length 2, `ns` must have length at least 2, and the most
// significant bit of `ds[1]` must be set.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// # Panics
// Panics if `ds` does not have length 2, `ns` has length less than 2, `qs` has length less than
// `ns.len() - 2`, or `ds[1]` does not have its highest bit set.
//
// This is equivalent to `mpn_divrem_2` from `mpn/generic/divrem_2.c`, GMP 6.2.1.
pub_crate_test! {limbs_div_mod_by_two_limb_normalized(
    qs: &mut [Limb],
    ns: &mut [Limb],
    ds: &[Limb]
) -> bool {
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
    let d_inv = limbs_two_limb_inverse_helper(d_1, d_0);
    for (&n, q) in ns[..n_limit].iter().zip(qs[..n_limit].iter_mut()).rev() {
        let r;
        (*q, r) = limbs_div_mod_three_limb_by_two_limb(r_1, r_0, n, d_1, d_0, d_inv);
        (r_1, r_0) = r.split_in_half();
    }
    ns[1] = r_1;
    ns[0] = r_0;
    highest_q
}}

// Schoolbook division using the Möller-Granlund 3/2 division algorithm.
//
// Divides `ns` by `ds` and writes the `ns.len()` - `ds.len()` least-significant quotient limbs to
// `qs` and the `ds.len()` limbs of the remainder to `ns`. Returns the most significant limb of the
// quotient; `true` means 1 and `false` means 0. `ds` must have length greater than 2, `ns` must be
// at least as long as `ds`, and the most significant bit of `ds` must be set. `d_inv` should be the
// result of `limbs_two_limb_inverse_helper` applied to the two highest limbs of the denominator.
//
// # Worst-case complexity
// $T(n, d) = O(d(n - d + 1)) = O(n^2)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, $n$ is `ns.len()`, and $d$ is `ds.len()`.
//
// # Panics
// Panics if `ds` has length smaller than 3, `ns` is shorter than `ds`, `qs` has length less than
// `ns.len()` - `ds.len()`, or the last limb of `ds` does not have its highest bit set.
//
// This is equivalent to `mpn_sbpi1_div_qr` from `mpn/generic/sbpi1_div_qr.c`, GMP 6.2.1.
pub_crate_test! {limbs_div_mod_schoolbook(
    qs: &mut [Limb],
    ns: &mut [Limb],
    ds: &[Limb],
    d_inv: Limb,
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
    let ns_hi = &mut ns[n_len - d_len..];
    let highest_q = limbs_cmp_same_length(ns_hi, ds) >= Equal;
    if highest_q {
        limbs_sub_same_length_in_place_left(ns_hi, ds);
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
            let (ns_lo, ns_hi) = ns.split_at_mut(i - 2);
            let n;
            (q, n) = limbs_div_mod_three_limb_by_two_limb(n_1, ns_hi[1], ns_hi[0], d_1, d_0, d_inv);
            let mut n_0;
            (n_1, n_0) = n.split_in_half();
            let local_carry_1 = limbs_sub_mul_limb_same_length_in_place_left(
                &mut ns_lo[j..],
                ds_except_last_two,
                q,
            );
            let local_carry_2 = n_0 < local_carry_1;
            n_0.wrapping_sub_assign(local_carry_1);
            let carry = local_carry_2 && n_1 == 0;
            if local_carry_2 {
                n_1.wrapping_sub_assign(1);
            }
            ns_hi[0] = n_0;
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
}}

// # Worst-case complexity
// $T(n) = O(n (\log n)^2 \log \log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ds.len()`.
//
// This is equivalent to `mpn_dcpi1_div_qr_n` from `mpn/generic/dcpi1_div_qr.c`, GMP 6.2.1.
pub(crate) fn limbs_div_mod_divide_and_conquer_helper(
    qs: &mut [Limb],
    ns: &mut [Limb],
    ds: &[Limb],
    d_inv: Limb,
    scratch: &mut [Limb],
) -> bool {
    let n = ds.len();
    let lo = n >> 1; // floor(n / 2)
    let hi = n - lo; // ceil(n / 2)
    let qs_hi = &mut qs[lo..];
    let (ds_lo, ds_hi) = ds.split_at(lo);
    let mut highest_q = if hi < DC_DIV_QR_THRESHOLD {
        limbs_div_mod_schoolbook(qs_hi, &mut ns[lo << 1..n << 1], ds_hi, d_inv)
    } else {
        limbs_div_mod_divide_and_conquer_helper(qs_hi, &mut ns[lo << 1..], ds_hi, d_inv, scratch)
    };
    let qs_hi = &mut qs_hi[..hi];
    let mut mul_scratch = vec![0; limbs_mul_greater_to_out_scratch_len(qs_hi.len(), ds_lo.len())];
    limbs_mul_greater_to_out(scratch, qs_hi, ds_lo, &mut mul_scratch);
    let ns_lo = &mut ns[..n + lo];
    let mut carry = Limb::from(limbs_sub_same_length_in_place_left(
        &mut ns_lo[lo..],
        &scratch[..n],
    ));
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
    let (ds_lo, ds_hi) = ds.split_at(hi);
    let q_lo = if lo < DC_DIV_QR_THRESHOLD {
        limbs_div_mod_schoolbook(qs, &mut ns[hi..n + lo], ds_hi, d_inv)
    } else {
        limbs_div_mod_divide_and_conquer_helper(qs, &mut ns[hi..], ds_hi, d_inv, scratch)
    };
    let qs_lo = &mut qs[..lo];
    let ns_lo = &mut ns[..n];
    let mut mul_scratch = vec![0; limbs_mul_greater_to_out_scratch_len(ds_lo.len(), lo)];
    limbs_mul_greater_to_out(scratch, ds_lo, qs_lo, &mut mul_scratch);
    let mut carry = Limb::from(limbs_sub_same_length_in_place_left(ns_lo, &scratch[..n]));
    if q_lo && limbs_sub_same_length_in_place_left(&mut ns_lo[lo..], ds_lo) {
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

// # Worst-case complexity
// $T(n) = O(n (\log n)^2 \log \log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ds.len()`.
pub_test! {limbs_div_dc_helper(
    qs: &mut [Limb],
    ns: &mut [Limb],
    ds: &[Limb],
    d_inv: Limb,
    scratch: &mut [Limb],
) -> bool {
    if qs.len() < DC_DIV_QR_THRESHOLD {
        limbs_div_mod_schoolbook(qs, ns, ds, d_inv)
    } else {
        limbs_div_mod_divide_and_conquer_helper(qs, ns, ds, d_inv, scratch)
    }
}}

// Recursive divide-and-conquer division.
//
// # Worst-case complexity
// $T(n) = O(n (\log n)^2 \log \log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ds.len()`.
//
// # Panics
// Panics if `ds` has length smaller than 6, `ns.len()` is less than `ds.len()` + 3, `qs` has length
// less than `ns.len()` - `ds.len()`, or the last limb of `ds` does not have its highest bit set.
//
// This is equivalent to `mpn_dcpi1_div_qr` from `mpn/generic/dcpi1_div_qr.c`, GMP 6.2.1.
pub_test! {limbs_div_mod_divide_and_conquer(
    qs: &mut [Limb],
    ns: &mut [Limb],
    ds: &[Limb],
    d_inv: Limb,
) -> bool {
    let n_len = ns.len();
    let d_len = ds.len();
    assert!(d_len >= 6); // to adhere to limbs_div_mod_schoolbook's limits
    assert!(n_len >= d_len + 3); // to adhere to limbs_div_mod_schoolbook's limits
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
        // Perform the typically smaller block first. Point at low limb of next quotient block
        let qs_alt = &mut qs[q_len - q_len_mod_d_len..q_len];
        if q_len_mod_d_len == 1 {
            // Handle highest_q up front, for simplicity.
            let ns_hi = &mut ns[q_len - 1..];
            let ns_hi_hi = &mut ns_hi[1..];
            highest_q = limbs_cmp_same_length(ns_hi_hi, ds) >= Equal;
            if highest_q {
                assert!(!limbs_sub_same_length_in_place_left(ns_hi_hi, ds));
            }
            // A single iteration of schoolbook: One 3/2 division, followed by the bignum update and
            // adjustment.
            let (last_n, ns) = ns_hi.split_last_mut().unwrap();
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
                let n;
                (q, n) = limbs_div_mod_three_limb_by_two_limb(n_2, n_1, n_0, d_1, d_0, d_inv);
                (n_1, n_0) = n.split_in_half();
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
            qs_alt[0] = q;
        } else {
            // Do a 2 * q_len_mod_d_len / q_len_mod_d_len division
            let (ds_lo, ds_hi) = ds.split_at(d_len - q_len_mod_d_len);
            highest_q = {
                let ns = &mut ns[n_len - (q_len_mod_d_len << 1)..];
                if q_len_mod_d_len == 2 {
                    limbs_div_mod_by_two_limb_normalized(qs_alt, ns, ds_hi)
                } else {
                    limbs_div_dc_helper(qs_alt, ns, ds_hi, d_inv, &mut scratch)
                }
            };
            if q_len_mod_d_len != d_len {
                let mut mul_scratch =
                    vec![0; limbs_mul_to_out_scratch_len(qs_alt.len(), ds_lo.len())];
                limbs_mul_to_out(&mut scratch, qs_alt, ds_lo, &mut mul_scratch);
                let ns = &mut ns[q_len - q_len_mod_d_len..n_len - q_len_mod_d_len];
                let mut carry = Limb::from(limbs_sub_same_length_in_place_left(ns, &scratch));
                if highest_q
                    && limbs_sub_same_length_in_place_left(&mut ns[q_len_mod_d_len..], ds_lo)
                {
                    carry += 1;
                }
                while carry != 0 {
                    if limbs_sub_limb_in_place(qs_alt, 1) {
                        assert!(highest_q);
                        highest_q = false;
                    }
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
            limbs_div_mod_divide_and_conquer_helper(
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
        highest_q = if q_len < DC_DIV_QR_THRESHOLD {
            limbs_div_mod_schoolbook(qs, &mut ns[m..], ds_hi, d_inv)
        } else {
            limbs_div_mod_divide_and_conquer_helper(qs, &mut ns[m..], ds_hi, d_inv, &mut scratch)
        };
        if m != 0 {
            let qs = &mut qs[..q_len];
            let ns = &mut ns[..d_len];
            let mut mul_scratch = vec![0; limbs_mul_to_out_scratch_len(q_len, ds_lo.len())];
            limbs_mul_to_out(&mut scratch, qs, ds_lo, &mut mul_scratch);
            let mut carry = Limb::from(limbs_sub_same_length_in_place_left(ns, &scratch));
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
}}

pub_test! {limbs_div_approx_helper(qs: &mut [Limb], ns: &mut [Limb], ds: &[Limb], d_inv: Limb) {
    if ds.len() < DC_DIVAPPR_Q_THRESHOLD {
        limbs_div_schoolbook_approx(qs, ns, ds, d_inv);
    } else {
        limbs_div_divide_and_conquer_approx(qs, ns, ds, d_inv);
    }
}}

// Takes the strictly normalized value ds (i.e., most significant bit must be set) as an input, and
// computes the approximate reciprocal of `ds`, with the same length as `ds`. See documentation for
// `limbs_invert_approx` for an explanation of the return value.
//
// # Worst-case complexity
// $T(n) = O(n (\log n)^2 \log \log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ds.len()`.
//
// # Panics
// Panics if `ds` is empty, `is` is shorter than `ds`, `scratch` is shorter than twice the length of
// `ds`, or the last limb of `ds` does not have its highest bit set.
//
// This is equivalent to `mpn_bc_invertappr` from `mpn/generic/invertappr.c`, GMP 6.2.1, where the
// return value is `true` iff the return value of `mpn_bc_invertappr` would be 0.
pub_test! {limbs_invert_basecase_approx(
    is: &mut [Limb],
    ds: &[Limb],
    scratch: &mut [Limb]
) -> bool {
    let d_len = ds.len();
    assert_ne!(d_len, 0);
    let highest_d = ds[d_len - 1];
    assert!(highest_d.get_highest_bit());
    if d_len == 1 {
        let d = ds[0];
        is[0] = limbs_invert_limb(d);
    } else {
        let scratch = &mut scratch[..d_len << 1];
        let (scratch_lo, scratch_hi) = scratch.split_at_mut(d_len);
        for s in &mut *scratch_lo {
            *s = Limb::MAX;
        }
        limbs_not_to_out(scratch_hi, ds);
        // Now scratch contains 2 ^ (2 * d_len * Limb::WIDTH) - d * 2 ^ (d_len * Limb::WIDTH) - 1
        if d_len == 2 {
            limbs_div_mod_by_two_limb_normalized(is, scratch, ds);
        } else {
            let d_inv = limbs_two_limb_inverse_helper(highest_d, ds[d_len - 2]);
            if MAYBE_DCP1_DIVAPPR {
                limbs_div_approx_helper(is, scratch, ds, d_inv);
            } else {
                limbs_div_schoolbook_approx(is, scratch, ds, d_inv);
            }
            assert!(!limbs_sub_limb_in_place(&mut is[..d_len], 1));
            return false;
        }
    }
    true
}}

// Takes the strictly normalized value ds (i.e., most significant bit must be set) as an input, and
// computes the approximate reciprocal of `ds`, with the same length as `ds`. See documentation for
// `limbs_invert_approx` for an explanation of the return value.
//
// Uses Newton's iterations (at least one). Inspired by Algorithm "ApproximateReciprocal", published
// in "Modern Computer Arithmetic" by Richard P. Brent and Paul Zimmermann, algorithm 3.5, page 121
// in version 0.4 of the book.
//
// Some adaptations were introduced, to allow product mod B ^ m - 1 and return the value e.
//
// We introduced a correction in such a way that "the value of B ^ {n + h} - T computed at step 8
// cannot exceed B ^ n - 1" (the book reads "2 * B ^ n - 1").
//
// Maximum scratch needed by this branch <= 2 * n, but have to fit 3 * rn in the scratch, i.e. 3 *
// rn <= 2 * n: we require n > 4.
//
// We use a wrapped product modulo B ^ m - 1.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log \log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `is.len()`.
//
// # Panics
// Panics if `ds` has length less than 5, `is` is shorter than `ds`, `scratch` is shorter than twice
// the length of `ds`, or the last limb of `ds` does not have its highest bit set.
//
// This is equivalent to `mpn_ni_invertappr` from `mpn/generic/invertappr.c`, GMP 6.2.1, where the
// return value is `true` iff the return value of `mpn_ni_invertappr` would be 0.
pub_test! {limbs_invert_newton_approx(is: &mut [Limb], ds: &[Limb], scratch: &mut [Limb]) -> bool {
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
        mul_size = limbs_mul_mod_base_pow_n_minus_1_next_size(d_len + 1);
        scratch2 = vec![0; limbs_mul_mod_base_pow_n_minus_1_scratch_len(mul_size, d_len, size)];
    }
    while size >= INV_NEWTON_THRESHOLD {
        sizes.push(size);
        size = (size >> 1) + 1;
    }
    // We compute the inverse of 0.ds as 1.is. Compute a base value of previous_d limbs.
    limbs_invert_basecase_approx(&mut is[d_len - size..], &ds[d_len - size..], scratch);
    let mut previous_size = size;
    let mut a = 0;
    // Use Newton's iterations to get the desired precision.
    for &size in sizes.iter().rev() {
        // ```
        // v    d       v
        // +----+-------+
        // ^ previous_d ^
        // ```
        //
        // Compute i_j * d
        let ds_hi = &ds[d_len - size..];
        let condition = size < INV_MULMOD_BNM1_THRESHOLD || {
            mul_size = limbs_mul_mod_base_pow_n_minus_1_next_size(size + 1);
            mul_size > size + previous_size
        };
        let diff = size - previous_size;
        let is_hi = &mut is[d_len - previous_size..];
        if condition {
            let mut mul_scratch =
                vec![0; limbs_mul_greater_to_out_scratch_len(ds_hi.len(), is_hi.len())];
            limbs_mul_greater_to_out(scratch, ds_hi, is_hi, &mut mul_scratch);
            limbs_slice_add_same_length_in_place_left(
                &mut scratch[previous_size..=size],
                &ds_hi[..=diff],
            );
        } else {
            // Remember we truncated mod B ^ (d + 1) We computed (truncated) xp of length d + 1 <-
            // 1.is * 0.ds Use B ^ mul_size - 1 wraparound
            limbs_mul_mod_base_pow_n_minus_1(scratch, mul_size, ds_hi, is_hi, &mut scratch2);
            let scratch = &mut scratch[..=mul_size];
            // We computed {xp, mul_size} <- {is, previous_d} * {ds, d} mod (B ^ mul_size - 1) We
            // know that 2 * |is * ds + ds * B ^ previous_d - B ^ {previous_d + d}| < B ^ mul_size
            // - 1 Add ds * B ^ previous_d mod (B ^ mul_size - 1)
            let mul_diff = mul_size - previous_size;
            assert!(size >= mul_diff);
            let (ds_hi_lo, ds_hi_hi) = ds_hi.split_at(mul_diff);
            let carry = limbs_slice_add_same_length_in_place_left(
                &mut scratch[previous_size..mul_size],
                ds_hi_lo,
            );
            // Subtract B ^ {previous_d + d}, maybe only compensate the carry
            scratch[mul_size] = 1; // set a limit for decrement
            let (scratch_lo, scratch_hi) = scratch.split_at_mut(size - mul_diff);
            if !limbs_add_same_length_with_carry_in_in_place_left(scratch_lo, ds_hi_hi, carry) {
                assert!(!limbs_sub_limb_in_place(scratch_hi, 1));
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
            if limbs_cmp_same_length(scratch_lo, ds_hi) == Greater {
                assert!(!limbs_sub_same_length_in_place_left(scratch_lo, ds_hi));
                carry += 1;
            }
            let (scratch_lo, scratch_mid) = scratch_lo.split_at_mut(diff);
            let (ds_hi_lo, ds_hi_hi) = ds_hi.split_at(diff);
            let borrow = limbs_cmp_same_length(scratch_lo, ds_hi_lo) == Greater;
            assert!(!limbs_sub_same_length_with_borrow_in_to_out(
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
                assert!(!limbs_sub_limb_in_place(&mut scratch[..=size], 1));
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
        let mut mul_scratch = vec![0; limbs_mul_same_length_to_out_scratch_len(is_hi.len())];
        limbs_mul_same_length_to_out(
            scratch_lo,
            &scratch_hi[..previous_size],
            is_hi,
            &mut mul_scratch,
        );
        a = (previous_size << 1) - diff;
        let carry = {
            let (scratch_lo, scratch_hi) = scratch.split_at_mut(a);
            limbs_slice_add_same_length_in_place_left(
                &mut scratch_lo[previous_size..],
                &scratch_hi[3 * diff - previous_size..diff << 1],
            )
        };
        if limbs_add_same_length_with_carry_in_to_out(
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
        previous_size = size;
    }
    // Check for possible carry propagation from below. Be conservative.
    scratch[a - 1] <= Limb::MAX - 7
}}

// Takes the strictly normalized value ds (i.e., most significant bit must be set) as an input, and
// computes the approximate reciprocal of `ds`, with the same length as `ds`.
//
// Let result_definitely_exact = limbs_invert_basecase_approx(is, ds, scratch) be the returned
// value. If result_definitely_exact is `true`, the error e is 0; otherwise, it may be 0 or 1. The
// following condition is satisfied by the output:
//
// ds * (2 ^ (n * Limb::WIDTH) + is) < 2 ^ (2 * n * Limb::WIDTH) <= ds * (2 ^ (n * Limb::WIDTH) + is
// + 1 + e), where n = `ds.len()`.
//
// When the strict result is needed, i.e., e = 0 in the relation above, the function `mpn_invert`
// (TODO!) should be used instead.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log \log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `is.len()`.
//
// # Panics
// Panics if `ds` is empty, `is` is shorter than `ds`, `scratch` is shorter than twice the length of
// `ds`, or the last limb of `ds` does not have its highest bit set.
//
// This is equivalent to `mpn_invertappr` from `mpn/generic/invertappr.c`, GMP 6.2.1, where the
// return value is `true` iff the return value of `mpn_invertappr` would be 0.
pub_crate_test! {limbs_invert_approx(is: &mut [Limb], ds: &[Limb], scratch: &mut [Limb]) -> bool {
    if ds.len() < INV_NEWTON_THRESHOLD {
        limbs_invert_basecase_approx(is, ds, scratch)
    } else {
        limbs_invert_newton_approx(is, ds, scratch)
    }
}}

// TODO tune
pub(crate) const MUL_TO_MULMOD_BNM1_FOR_2NXN_THRESHOLD: usize = INV_MULMOD_BNM1_THRESHOLD >> 1;

// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ds.len()`.
//
// - ds.len() >= 2
// - n_len >= 3
// - n_len >= ds.len()
// - i_len == limbs_div_mod_barrett_is_len(n_len - ds.len(), ds.len())
// - qs.len() == i_len
// - scratch_len ==  limbs_mul_mod_base_pow_n_minus_1_next_size(ds.len() + 1)
// - scratch.len() == limbs_div_mod_barrett_scratch_len(n_len, d_len) - i_len
// - rs_hi.len() == i_len
pub_crate_test! {limbs_div_barrett_large_product(
    scratch: &mut [Limb],
    ds: &[Limb],
    qs: &[Limb],
    rs_hi: &[Limb],
    scratch_len: usize,
    i_len: usize,
) {
    let d_len = ds.len();
    let (scratch, scratch_out) = scratch.split_at_mut(scratch_len);
    limbs_mul_mod_base_pow_n_minus_1(scratch, scratch_len, ds, qs, scratch_out);
    if d_len + i_len > scratch_len {
        let (rs_hi_lo, rs_hi_hi) = rs_hi.split_at(scratch_len - d_len);
        let carry_1 = limbs_sub_greater_in_place_left(scratch, rs_hi_hi);
        let carry_2 = limbs_cmp_same_length(rs_hi_lo, &scratch[d_len..]) == Less;
        if !carry_1 && carry_2 {
            assert!(!limbs_slice_add_limb_in_place(scratch, 1));
        } else {
            assert_eq!(carry_1, carry_2);
        }
    }
}}

// # Worst-case complexity
// $T(n, d) = O(n \log d \log\log d)$
//
// $M(n) = O(d \log d)$
//
// where $T$ is time, $M$ is additional memory, $n$ is `ns.len()`, and $d$ is `ds.len()`.
//
// This is equivalent to `mpn_preinv_mu_div_qr` from `mpn/generic/mu_div_qr.c`, GMP 6.2.1.
fn limbs_div_mod_barrett_preinverted(
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
    let highest_q = limbs_cmp_same_length(ns_hi, ds) >= Equal;
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
        let mut mul_scratch = vec![0; limbs_mul_same_length_to_out_scratch_len(is.len())];
        limbs_mul_same_length_to_out(scratch, rs_hi, is, &mut mul_scratch);
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
            let mut mul_scratch = vec![0; limbs_mul_greater_to_out_scratch_len(ds.len(), qs.len())];
            limbs_mul_greater_to_out(scratch, ds, qs, &mut mul_scratch);
        } else {
            limbs_div_barrett_large_product(scratch, ds, qs, rs_hi, scratch_len, i_len);
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
            let carry = limbs_sub_same_length_with_borrow_in_in_place_right(
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
        if limbs_cmp_same_length(rs, ds) >= Equal {
            // This is executed with about 76% probability.
            assert!(!limbs_slice_add_limb_in_place(qs, 1));
            limbs_sub_same_length_in_place_left(rs, ds);
        }
    }
    highest_q
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
// Result is O(`q_len`)
//
// This is equivalent to `mpn_mu_div_qr_choose_in` from `mpn/generic/mu_div_qr.c`, GMP 6.2.1, where
// `k == 0`.
pub_const_crate_test! {limbs_div_mod_barrett_is_len(q_len: usize, d_len: usize) -> usize {
    let q_len_minus_1 = q_len - 1;
    if q_len > d_len {
        // Compute an inverse size that is a nice partition of the quotient.
        let b = q_len_minus_1 / d_len + 1; // ceil(q_len / d_len), number of blocks
        q_len_minus_1 / b + 1 // ceil(q_len / b) = ceil(q_len / ceil(q_len / d_len))
    } else if 3 * q_len > d_len {
        (q_len_minus_1 >> 1) + 1 // b = 2
    } else {
        q_len_minus_1 + 1 // b = 1
    }
}}

// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// This is equivalent to `mpn_mu_div_qr2` from `mpn/generic/mu_div_qr.c`, GMP 6.2.1.
pub_crate_test! {limbs_div_mod_barrett_helper(
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
    let i_len = limbs_div_mod_barrett_is_len(q_len, d_len);
    assert!(i_len <= d_len);
    let i_len_plus_1 = i_len + 1;
    let (is, scratch_hi) = scratch.split_at_mut(i_len_plus_1);
    // compute an approximate inverse on i_len + 1 limbs
    if d_len == i_len {
        let (scratch_lo, scratch_hi) = scratch_hi.split_at_mut(i_len_plus_1);
        let (scratch_first, scratch_lo_tail) = scratch_lo.split_first_mut().unwrap();
        scratch_lo_tail.copy_from_slice(&ds[..i_len]);
        *scratch_first = 1;
        limbs_invert_approx(is, scratch_lo, scratch_hi);
        slice_move_left(is, 1);
    } else if limbs_add_limb_to_out(scratch_hi, &ds[d_len - i_len_plus_1..], 1) {
        slice_set_zero(&mut is[..i_len]);
    } else {
        let (scratch_lo, scratch_hi) = scratch_hi.split_at_mut(i_len_plus_1);
        limbs_invert_approx(is, scratch_lo, scratch_hi);
        slice_move_left(is, 1);
    }
    let (scratch_lo, scratch_hi) = scratch.split_at_mut(i_len);
    limbs_div_mod_barrett_preinverted(qs, rs, ns, ds, scratch_lo, scratch_hi)
}}

// # Worst-case complexity
// Constant time and additional memory.
//
// Result is O(`d_len`)
//
// This is equivalent to `mpn_preinv_mu_div_qr_itch` from `mpn/generic/mu_div_qr.c`, GMP 6.2.1, but
// `nn` is omitted from the arguments as it is unused.
fn limbs_div_mod_barrett_preinverse_scratch_len(d_len: usize, is_len: usize) -> usize {
    let itch_local = limbs_mul_mod_base_pow_n_minus_1_next_size(d_len + 1);
    let itch_out = limbs_mul_mod_base_pow_n_minus_1_scratch_len(itch_local, d_len, is_len);
    itch_local + itch_out
}

// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `mpn_invertappr_itch` from `gmp-impl.h`, GMP 6.2.1.
pub(crate) const fn limbs_invert_approx_scratch_len(is_len: usize) -> usize {
    is_len << 1
}

// # Worst-case complexity
// Constant time and additional memory.
//
// Result is O(`n_len`)
//
// This is equivalent to `mpn_mu_div_qr_itch` from `mpn/generic/mu_div_qr.c`, GMP 6.2.1, where
// `mua_k == 0`.
pub_crate_test! {limbs_div_mod_barrett_scratch_len(n_len: usize, d_len: usize) -> usize {
    let is_len = limbs_div_mod_barrett_is_len(n_len - d_len, d_len);
    let preinverse_len = limbs_div_mod_barrett_preinverse_scratch_len(d_len, is_len);
    // 3 * is_len + 4
    let inv_approx_len = limbs_invert_approx_scratch_len(is_len + 1) + is_len + 2;
    assert!(preinverse_len >= inv_approx_len);
    is_len + preinverse_len
}}

// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ds.len()`.
pub_test! {limbs_div_mod_barrett_large_helper(
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
    let mut highest_q = limbs_div_mod_barrett_helper(qs, rs_hi, ns_hi, ds_hi, scratch);
    // Multiply the quotient by the divisor limbs ignored above. The product is d_len - 1 limbs
    // long.
    let mut mul_scratch = vec![0; limbs_mul_to_out_scratch_len(ds_lo.len(), qs.len())];
    limbs_mul_to_out(scratch, ds_lo, qs, &mut mul_scratch);
    let (scratch_last, scratch_init) = scratch[..d_len].split_last_mut().unwrap();
    *scratch_last = Limb::from(
        highest_q && limbs_slice_add_same_length_in_place_left(&mut scratch_init[q_len..], ds_lo),
    );
    let (scratch_lo, scratch_hi) = scratch.split_at(n);
    let scratch_hi = &scratch_hi[..q_len_plus_one];
    if limbs_sub_same_length_with_borrow_in_in_place_left(
        rs_hi,
        scratch_hi,
        limbs_sub_same_length_to_out(rs_lo, ns_lo, scratch_lo),
    ) {
        if limbs_sub_limb_in_place(qs, 1) {
            assert!(highest_q);
            highest_q = false;
        }
        limbs_slice_add_same_length_in_place_left(&mut rs[..d_len], ds);
    }
    highest_q
}}

// Block-wise Barrett division. The idea of the algorithm used herein is to compute a smaller
// inverted value than used in the standard Barrett algorithm, and thus save time in the Newton
// iterations, and pay just a small price when using the inverted value for developing quotient
// bits. This algorithm was presented at ICMS 2006.
//
// `ns` must have length at least 3, `ds` must have length at least 2 and be no longer than `ns`,
// and the most significant bit of `ds` must be set.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// # Panics
// Panics if `ds` has length smaller than 2, `ns.len()` is less than `ds.len()`, `qs` has length
// less than `ns.len()` - `ds.len()`, `scratch` is too short, or the last limb of `ds` does not have
// its highest bit set.
//
// This is equivalent to `mpn_mu_div_qr` from `mpn/generic/mu_div_qr.c`, GMP 6.2.1.
pub_test! {limbs_div_mod_barrett(
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
        limbs_div_mod_barrett_helper(qs, &mut rs[..d_len], ns, ds, scratch)
    } else {
        limbs_div_mod_barrett_large_helper(qs, rs, ns, ds, scratch)
    }
}}

// `ds` must have length 2, `ns` must have length at least 2, `qs` must have length at least
// `ns.len() - 2`, `rs` must have length at least 2, and the most-significant limb of `ds` must be
// nonzero.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
fn limbs_div_mod_by_two_limb(qs: &mut [Limb], rs: &mut [Limb], ns: &[Limb], ds: &[Limb]) {
    let n_len = ns.len();
    let ds_1 = ds[1];
    let bits = LeadingZeros::leading_zeros(ds_1);
    if bits == 0 {
        let mut ns = ns.to_vec();
        // always store n_len - 1 quotient limbs
        qs[n_len - 2] = Limb::from(limbs_div_mod_by_two_limb_normalized(qs, &mut ns, ds));
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
            qs[n_len - 2] = Limb::from(limbs_div_mod_by_two_limb_normalized(
                qs,
                &mut ns_shifted[..n_len],
                ds_shifted,
            ));
        } else {
            ns_shifted[n_len] = carry;
            limbs_div_mod_by_two_limb_normalized(qs, ns_shifted, ds_shifted);
        }
        let ns_shifted_1 = ns_shifted[1];
        rs[0] = (ns_shifted[0] >> bits) | (ns_shifted_1 << cobits);
        rs[1] = ns_shifted_1 >> bits;
    }
}

// TODO tune
pub(crate) const MUPI_DIV_QR_THRESHOLD: usize = 74;

// # Worst-case complexity
// Constant time and additional memory.
fn limbs_div_mod_dc_condition(n_len: usize, d_len: usize) -> bool {
    let n_64 = n_len as f64;
    let d_64 = d_len as f64;
    d_len < MUPI_DIV_QR_THRESHOLD
        || n_len < MU_DIV_QR_THRESHOLD << 1
        || libm::fma(
            ((MU_DIV_QR_THRESHOLD - MUPI_DIV_QR_THRESHOLD) << 1) as f64,
            d_64,
            MUPI_DIV_QR_THRESHOLD as f64 * n_64,
        ) > d_64 * n_64
}

// This function is optimized for the case when the numerator has at least twice the length of the
// denominator.
//
// `ds` must have length at least 3, `ns` must be at least as long as `ds`, `qs` must have length at
// least `ns.len() - ds.len() + 1`, `rs` must have the same length as `ds`, and the most-
// significant limb of `ds` must be nonzero.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
fn limbs_div_mod_unbalanced(
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
        limbs_div_mod_schoolbook(qs, ns_shifted, ds_shifted, d_inv);
        let ns_shifted = &ns_shifted[..d_len];
        if bits == 0 {
            rs.copy_from_slice(ns_shifted);
        } else {
            limbs_shr_to_out(rs, ns_shifted, bits);
        }
    } else if limbs_div_mod_dc_condition(n_len, d_len) {
        limbs_div_mod_divide_and_conquer(qs, ns_shifted, ds_shifted, d_inv);
        let ns_shifted = &ns_shifted[..d_len];
        if bits == 0 {
            rs.copy_from_slice(ns_shifted);
        } else {
            limbs_shr_to_out(rs, ns_shifted, bits);
        }
    } else {
        let scratch_len = limbs_div_mod_barrett_scratch_len(n_len, d_len);
        let mut scratch = vec![0; scratch_len];
        limbs_div_mod_barrett(qs, rs, ns_shifted, ds_shifted, &mut scratch);
        if bits != 0 {
            limbs_slice_shr_in_place(rs, bits);
        }
    }
}

// The numerator must have less than twice the length of the denominator.
//
// Problem:
//
// Divide a numerator N with `n_len` limbs by a denominator D with `d_len` limbs, forming a quotient
// of `q_len` = `n_len` - `d_len` + 1 limbs. When `q_len` is small compared to `d_len`, conventional
// division algorithms perform poorly. We want an algorithm that has an expected running time that
// is dependent only on `q_len`.
//
// Algorithm (very informally stated):
//
// 1) Divide the 2 * `q_len` most significant limbs from the numerator by the `q_len` most-
// significant limbs from the denominator. Call the result `qest`. This is either the correct
// quotient, or 1 or 2 too large. Compute the remainder from the division.
//
// 2) Is the most significant limb from the remainder < p, where p is the product of the most-
// significant limb from the quotient and the next(d)? (Next(d) denotes the next ignored limb from
// the denominator.)  If it is, decrement `qest`, and adjust the remainder accordingly.
//
// 3) Is the remainder >= `qest`?  If it is, `qest` is the desired quotient. The algorithm
// terminates.
//
// 4) Subtract `qest` * next(d) from the remainder. If there is borrow out, decrement `qest`, and
// adjust the remainder accordingly.
//
// 5) Skip one word from the denominator (i.e., let next(d) denote the next less significant limb).
//
// `ds` must have length at least 3, `ns` must be at least as long as `ds` but no more than twice as
// long, `qs` must have length at least `ns.len() - ds.len() + 1`,`rs` must have the same length as
// `ds`, and the most-significant limb of `ds` must be nonzero.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
pub(crate) fn limbs_div_mod_balanced(
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
    let bits = LeadingZeros::leading_zeros(ds[d_len - 1]);
    let cobits = Limb::WIDTH - bits;
    let q_len_2 = q_len << 1;
    let m = n_len - q_len_2;
    let mut ns_shifted_vec = vec![0; q_len_2 + 1];
    let mut ds_shifted_vec;
    let ds_shifted: &[Limb];
    let ds_hi = &ds[i_len..];
    let ds_lo_last = ds[i_len - 1];
    let carry = if bits == 0 {
        ds_shifted = ds_hi;
        ns_shifted_vec[..q_len_2].copy_from_slice(&ns[m..]);
        0
    } else {
        ds_shifted_vec = vec![0; q_len];
        limbs_shl_to_out(&mut ds_shifted_vec, ds_hi, bits);
        ds_shifted_vec[0] |= ds_lo_last >> cobits;
        ds_shifted = &ds_shifted_vec;
        let carry = limbs_shl_to_out(&mut ns_shifted_vec, &ns[m..], bits);
        if !adjust {
            ns_shifted_vec[0] |= ns[m - 1] >> cobits;
        }
        carry
    };
    let ns_shifted = if adjust {
        ns_shifted_vec[q_len_2] = carry;
        &mut ns_shifted_vec[1..]
    } else {
        &mut ns_shifted_vec
    };
    // Get an approximate quotient using the extracted operands.
    if q_len == 1 {
        (qs[0], ns_shifted[0]) =
            Limb::xx_div_mod_y_to_qr(ns_shifted[1], ns_shifted[0], ds_shifted[0]);
    } else if q_len == 2 {
        limbs_div_mod_by_two_limb_normalized(qs, ns_shifted, ds_shifted);
    } else {
        let ns_shifted = &mut ns_shifted[..q_len_2];
        let d_inv = limbs_two_limb_inverse_helper(ds_shifted[q_len - 1], ds_shifted[q_len - 2]);
        if q_len < DC_DIV_QR_THRESHOLD {
            limbs_div_mod_schoolbook(qs, ns_shifted, ds_shifted, d_inv);
        } else if q_len < MU_DIV_QR_THRESHOLD {
            limbs_div_mod_divide_and_conquer(qs, ns_shifted, ds_shifted, d_inv);
        } else {
            let mut scratch = vec![0; limbs_div_mod_barrett_scratch_len(q_len_2, q_len)];
            limbs_div_mod_barrett(qs, rs, ns_shifted, ds_shifted, &mut scratch);
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
        x |= ds[i_len - 2] >> 1 >> (!bits & Limb::WIDTH_MASK);
    }
    if ns_shifted[q_len - 1] < (DoubleLimb::from(x) * DoubleLimb::from(qs[q_len - 1])).upper_half()
    {
        assert!(!limbs_sub_limb_in_place(qs, 1));
        let carry = limbs_slice_add_same_length_in_place_left(&mut ns_shifted[..q_len], ds_shifted);
        if carry {
            // The partial remainder is safely large.
            ns_shifted[q_len] = 1;
            r_len += 1;
        }
    }
    let mut q_too_large = false;
    let mut do_extra_cleanup = true;
    let mut scratch = vec![0; d_len];
    let mut i_len_alt = i_len;
    let qs_lo = &mut qs[..q_len];
    if bits != 0 {
        // Append the partially used numerator limb to the partial remainder.
        let carry_1 = limbs_slice_shl_in_place(&mut ns_shifted[..r_len], cobits);
        let mask = Limb::MAX >> bits;
        ns_shifted[0] |= ns[i_len - 1] & mask;
        // Update partial remainder with partially used divisor limb.
        let (ns_shifted_last, ns_shifted_init) = ns_shifted[..=q_len].split_last_mut().unwrap();
        let carry_2 = limbs_sub_mul_limb_same_length_in_place_left(
            ns_shifted_init,
            qs_lo,
            ds[i_len - 1] & mask,
        );
        if q_len == r_len {
            (*ns_shifted_last, q_too_large) = carry_1.overflowing_sub(carry_2);
            r_len += 1;
        } else {
            assert!(*ns_shifted_last >= carry_2);
            ns_shifted_last.wrapping_sub_assign(carry_2);
        }
        i_len_alt -= 1;
    }
    // True: partial remainder now is neutral, i.e., it is not shifted up.
    if i_len_alt == 0 {
        rs.copy_from_slice(&ns_shifted[..r_len]);
        do_extra_cleanup = false;
    } else {
        let mut mul_scratch = vec![0; limbs_mul_to_out_scratch_len(qs_lo.len(), i_len_alt)];
        limbs_mul_to_out(&mut scratch, qs_lo, &ds[..i_len_alt], &mut mul_scratch);
    }
    if do_extra_cleanup {
        let (scratch_lo, scratch_hi) = scratch.split_at_mut(i_len_alt);
        q_too_large |=
            limbs_sub_greater_in_place_left(&mut ns_shifted[..r_len], &scratch_hi[..q_len]);
        let (rs_lo, rs_hi) = rs.split_at_mut(i_len_alt);
        let rs_hi_len = rs_hi.len();
        rs_hi.copy_from_slice(&ns_shifted[..rs_hi_len]);
        q_too_large |= limbs_sub_same_length_to_out(rs_lo, &ns[..i_len_alt], scratch_lo)
            && limbs_sub_limb_in_place(&mut rs_hi[..min(rs_hi_len, r_len)], 1);
    }
    if q_too_large {
        assert!(!limbs_sub_limb_in_place(qs, 1));
        limbs_slice_add_same_length_in_place_left(rs, ds);
    }
}

// Interpreting two slices of `Limb`s, `ns` and `ds`, as the limbs (in ascending order) of two
// `Natural`s, divides them, returning the quotient and remainder. The quotient has `ns.len() -
// ds.len() + 1` limbs and the remainder `ds.len()` limbs.
//
// `ns` must be at least as long as `ds` and `ds` must have length at least 2 and its most
// significant limb must be greater than zero.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// # Panics
// Panics if `ns` is shorter than `ds`, `ds` has length less than 2, or the most-significant limb of
// `ds` is zero.
//
// This is equivalent to `mpn_tdiv_qr` from `mpn/generic/tdiv_qr.c`, GMP 6.2.1, where `dn > 1` and
// `qp` and `rp` are returned.
pub_test! {limbs_div_mod(ns: &[Limb], ds: &[Limb]) -> (Vec<Limb>, Vec<Limb>) {
    let d_len = ds.len();
    let mut qs = vec![0; ns.len() - d_len + 1];
    let mut rs = vec![0; d_len];
    limbs_div_mod_to_out(&mut qs, &mut rs, ns, ds);
    (qs, rs)
}}

// Interpreting two slices of `Limb`s, `ns` and `ds`, as the limbs (in ascending order) of two
// `Natural`s, divides them, writing the `ns.len() - ds.len() + 1` limbs of the quotient to `qs` and
// the `ds.len()` limbs of the remainder to `rs`.
//
// `ns` must be at least as long as `ds`, `qs` must have length at least `ns.len() - ds.len() + 1`,
// `rs` must be at least as long as `ds`, and `ds` must have length at least 2 and its most
// significant limb must be greater than zero.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// # Panics
// Panics if `qs` or `rs` are too short, `ns` is shorter than `ds`, `ds` has length less than 2, or
// the most-significant limb of `ds` is zero.
//
// This is equivalent to `mpn_tdiv_qr` from `mpn/generic/tdiv_qr.c`, GMP 6.2.1, where `dn > 1`.
pub_crate_test! {limbs_div_mod_to_out(qs: &mut [Limb], rs: &mut [Limb], ns: &[Limb], ds: &[Limb]) {
    let n_len = ns.len();
    let d_len = ds.len();
    assert!(d_len > 1);
    assert!(n_len >= d_len);
    assert!(qs.len() > n_len - d_len);
    let rs = &mut rs[..d_len];
    let ds_last = *ds.last().unwrap();
    assert!(ds_last != 0);
    if d_len == 2 {
        limbs_div_mod_by_two_limb(qs, rs, ns, ds);
    } else {
        // conservative tests for quotient size
        let adjust = ns[n_len - 1] >= ds_last;
        let adjusted_n_len = if adjust { n_len + 1 } else { n_len };
        if adjusted_n_len < d_len << 1 {
            limbs_div_mod_balanced(qs, rs, ns, ds, adjust);
        } else {
            limbs_div_mod_unbalanced(qs, rs, ns, ds, adjusted_n_len);
        }
    }
}}

// TODO improve!
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
pub_crate_test! {limbs_div_mod_qs_to_out_rs_to_ns(qs: &mut [Limb], ns: &mut [Limb], ds: &[Limb]) {
    let ns_copy = ns.to_vec();
    limbs_div_mod_to_out(qs, ns, &ns_copy, ds);
}}

impl Natural {
    fn div_mod_limb_ref(&self, other: Limb) -> (Natural, Limb) {
        match (self, other) {
            (_, 0) => panic!("division by zero"),
            (n, 1) => (n.clone(), 0),
            (Natural(Small(small)), other) => {
                let (q, r) = small.div_rem(other);
                (Natural(Small(q)), r)
            }
            (Natural(Large(ref limbs)), other) => {
                let (qs, r) = limbs_div_limb_mod(limbs, other);
                (Natural::from_owned_limbs_asc(qs), r)
            }
        }
    }

    pub_test! {div_assign_mod_limb(&mut self, other: Limb) -> Limb {
        match (&mut *self, other) {
            (_, 0) => panic!("division by zero"),
            (_, 1) => 0,
            (Natural(Small(ref mut small)), other) => small.div_assign_rem(other),
            (Natural(Large(ref mut limbs)), other) => {
                let r = limbs_div_limb_in_place_mod(limbs, other);
                self.trim();
                r
            }
        }
    }}
}

impl DivMod<Natural> for Natural {
    type DivOutput = Natural;
    type ModOutput = Natural;

    /// Divides a [`Natural`] by another [`Natural`], taking both by value and returning the
    /// quotient and remainder. The quotient is rounded towards negative infinity.
    ///
    /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq r < y$.
    ///
    /// $$
    /// f(x, y) = \left ( \left \lfloor \frac{x}{y} \right \rfloor, \space
    /// x - y\left \lfloor \frac{x}{y} \right \rfloor \right ).
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
    /// use malachite_base::num::arithmetic::traits::DivMod;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(
    ///     Natural::from(23u32)
    ///         .div_mod(Natural::from(10u32))
    ///         .to_debug_string(),
    ///     "(2, 3)"
    /// );
    ///
    /// // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    /// assert_eq!(
    ///     Natural::from_str("1000000000000000000000000")
    ///         .unwrap()
    ///         .div_mod(Natural::from_str("1234567890987").unwrap())
    ///         .to_debug_string(),
    ///     "(810000006723, 530068894399)"
    /// );
    /// ```
    #[inline]
    fn div_mod(mut self, other: Natural) -> (Natural, Natural) {
        let r = self.div_assign_mod(other);
        (self, r)
    }
}

impl<'a> DivMod<&'a Natural> for Natural {
    type DivOutput = Natural;
    type ModOutput = Natural;

    /// Divides a [`Natural`] by another [`Natural`], taking the first by value and the second by
    /// reference and returning the quotient and remainder. The quotient is rounded towards negative
    /// infinity.
    ///
    /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq r < y$.
    ///
    /// $$
    /// f(x, y) = \left ( \left \lfloor \frac{x}{y} \right \rfloor, \space
    /// x - y\left \lfloor \frac{x}{y} \right \rfloor \right ).
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
    /// use malachite_base::num::arithmetic::traits::DivMod;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(
    ///     Natural::from(23u32)
    ///         .div_mod(&Natural::from(10u32))
    ///         .to_debug_string(),
    ///     "(2, 3)"
    /// );
    ///
    /// // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    /// assert_eq!(
    ///     Natural::from_str("1000000000000000000000000")
    ///         .unwrap()
    ///         .div_mod(&Natural::from_str("1234567890987").unwrap())
    ///         .to_debug_string(),
    ///     "(810000006723, 530068894399)"
    /// );
    /// ```
    #[inline]
    fn div_mod(mut self, other: &'a Natural) -> (Natural, Natural) {
        let r = self.div_assign_mod(other);
        (self, r)
    }
}

impl<'a> DivMod<Natural> for &'a Natural {
    type DivOutput = Natural;
    type ModOutput = Natural;

    /// Divides a [`Natural`] by another [`Natural`], taking the first by reference and the second
    /// by value and returning the quotient and remainder. The quotient is rounded towards negative
    /// infinity.
    ///
    /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq r < y$.
    ///
    /// $$
    /// f(x, y) = \left ( \left \lfloor \frac{x}{y} \right \rfloor, \space
    /// x - y\left \lfloor \frac{x}{y} \right \rfloor \right ).
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
    /// use malachite_base::num::arithmetic::traits::DivMod;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(
    ///     (&Natural::from(23u32))
    ///         .div_mod(Natural::from(10u32))
    ///         .to_debug_string(),
    ///     "(2, 3)"
    /// );
    ///
    /// // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    /// assert_eq!(
    ///     (&Natural::from_str("1000000000000000000000000").unwrap())
    ///         .div_mod(Natural::from_str("1234567890987").unwrap())
    ///         .to_debug_string(),
    ///     "(810000006723, 530068894399)"
    /// );
    /// ```
    fn div_mod(self, mut other: Natural) -> (Natural, Natural) {
        if *self == other {
            return (Natural::ONE, Natural::ZERO);
        }
        match (self, &mut other) {
            (_, &mut Natural::ZERO) => panic!("division by zero"),
            (n, &mut Natural::ONE) => (n.clone(), Natural::ZERO),
            (n, &mut Natural(Small(d))) => {
                let (q, r) = n.div_mod_limb_ref(d);
                (q, Natural(Small(r)))
            }
            (Natural(Small(_)), _) => (Natural::ZERO, self.clone()),
            (&Natural(Large(ref ns)), &mut Natural(Large(ref mut ds))) => {
                if ns.len() < ds.len() {
                    (Natural::ZERO, self.clone())
                } else {
                    let (qs, mut rs) = limbs_div_mod(ns, ds);
                    swap(&mut rs, ds);
                    other.trim();
                    (Natural::from_owned_limbs_asc(qs), other)
                }
            }
        }
    }
}

impl<'a, 'b> DivMod<&'b Natural> for &'a Natural {
    type DivOutput = Natural;
    type ModOutput = Natural;

    /// Divides a [`Natural`] by another [`Natural`], taking both by reference and returning the
    /// quotient and remainder. The quotient is rounded towards negative infinity.
    ///
    /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq r < y$.
    ///
    /// $$
    /// f(x, y) = \left ( \left \lfloor \frac{x}{y} \right \rfloor, \space
    /// x - y\left \lfloor \frac{x}{y} \right \rfloor \right ).
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
    /// use malachite_base::num::arithmetic::traits::DivMod;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(
    ///     (&Natural::from(23u32))
    ///         .div_mod(&Natural::from(10u32))
    ///         .to_debug_string(),
    ///     "(2, 3)"
    /// );
    ///
    /// // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    /// assert_eq!(
    ///     (&Natural::from_str("1000000000000000000000000").unwrap())
    ///         .div_mod(&Natural::from_str("1234567890987").unwrap())
    ///         .to_debug_string(),
    ///     "(810000006723, 530068894399)"
    /// );
    /// ```
    fn div_mod(self, other: &'b Natural) -> (Natural, Natural) {
        if self == other {
            return (Natural::ONE, Natural::ZERO);
        }
        match (self, other) {
            (_, &Natural::ZERO) => panic!("division by zero"),
            (n, &Natural::ONE) => (n.clone(), Natural::ZERO),
            (n, Natural(Small(d))) => {
                let (q, r) = n.div_mod_limb_ref(*d);
                (q, Natural(Small(r)))
            }
            (Natural(Small(_)), _) => (Natural::ZERO, self.clone()),
            (&Natural(Large(ref ns)), Natural(Large(ref ds))) => {
                if ns.len() < ds.len() {
                    (Natural::ZERO, self.clone())
                } else {
                    let (qs, rs) = limbs_div_mod(ns, ds);
                    (
                        Natural::from_owned_limbs_asc(qs),
                        Natural::from_owned_limbs_asc(rs),
                    )
                }
            }
        }
    }
}

impl DivAssignMod<Natural> for Natural {
    type ModOutput = Natural;

    /// Divides a [`Natural`] by another [`Natural`] in place, taking the [`Natural`] on the
    /// right-hand side by value and returning the remainder. The quotient is rounded towards
    /// negative infinity.
    ///
    /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq r < y$.
    ///
    /// $$
    /// f(x, y) = x - y\left \lfloor \frac{x}{y} \right \rfloor,
    /// $$
    /// $$
    /// x \gets \left \lfloor \frac{x}{y} \right \rfloor.
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
    /// use malachite_base::num::arithmetic::traits::DivAssignMod;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 10 + 3 = 23
    /// let mut x = Natural::from(23u32);
    /// assert_eq!(x.div_assign_mod(Natural::from(10u32)), 3);
    /// assert_eq!(x, 2);
    ///
    /// // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    /// let mut x = Natural::from_str("1000000000000000000000000").unwrap();
    /// assert_eq!(
    ///     x.div_assign_mod(Natural::from_str("1234567890987").unwrap()),
    ///     530068894399u64
    /// );
    /// assert_eq!(x, 810000006723u64);
    /// ```
    fn div_assign_mod(&mut self, mut other: Natural) -> Natural {
        if *self == other {
            *self = Natural::ONE;
            return Natural::ZERO;
        }
        match (&mut *self, &mut other) {
            (_, &mut Natural::ZERO) => panic!("division by zero"),
            (_, &mut Natural::ONE) => Natural::ZERO,
            (n, &mut Natural(Small(d))) => Natural(Small(n.div_assign_mod_limb(d))),
            (Natural(Small(_)), _) => {
                let mut r = Natural::ZERO;
                swap(self, &mut r);
                r
            }
            (&mut Natural(Large(ref mut ns)), &mut Natural(Large(ref mut ds))) => {
                if ns.len() < ds.len() {
                    let mut r = Natural::ZERO;
                    swap(self, &mut r);
                    r
                } else {
                    let (mut qs, mut rs) = limbs_div_mod(ns, ds);
                    swap(&mut qs, ns);
                    swap(&mut rs, ds);
                    self.trim();
                    other.trim();
                    other
                }
            }
        }
    }
}

impl<'a> DivAssignMod<&'a Natural> for Natural {
    type ModOutput = Natural;

    /// Divides a [`Natural`] by another [`Natural`] in place, taking the [`Natural`] on the
    /// right-hand side by value and returning the remainder. The quotient is rounded towards
    /// negative infinity.
    ///
    /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq r < y$.
    ///
    /// $$
    /// f(x, y) = x - y\left \lfloor \frac{x}{y} \right \rfloor,
    /// $$
    /// $$
    /// x \gets \left \lfloor \frac{x}{y} \right \rfloor.
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
    /// use malachite_base::num::arithmetic::traits::DivAssignMod;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 10 + 3 = 23
    /// let mut x = Natural::from(23u32);
    /// assert_eq!(x.div_assign_mod(&Natural::from(10u32)), 3);
    /// assert_eq!(x, 2);
    ///
    /// // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    /// let mut x = Natural::from_str("1000000000000000000000000").unwrap();
    /// assert_eq!(
    ///     x.div_assign_mod(&Natural::from_str("1234567890987").unwrap()),
    ///     530068894399u64
    /// );
    /// assert_eq!(x, 810000006723u64);
    /// ```
    fn div_assign_mod(&mut self, other: &'a Natural) -> Natural {
        if self == other {
            *self = Natural::ONE;
            return Natural::ZERO;
        }
        match (&mut *self, other) {
            (_, &Natural::ZERO) => panic!("division by zero"),
            (_, &Natural::ONE) => Natural::ZERO,
            (_, Natural(Small(d))) => Natural(Small(self.div_assign_mod_limb(*d))),
            (Natural(Small(_)), _) => {
                let mut r = Natural::ZERO;
                swap(self, &mut r);
                r
            }
            (&mut Natural(Large(ref mut ns)), Natural(Large(ref ds))) => {
                if ns.len() < ds.len() {
                    let mut r = Natural::ZERO;
                    swap(self, &mut r);
                    r
                } else {
                    let (mut qs, rs) = limbs_div_mod(ns, ds);
                    swap(&mut qs, ns);
                    self.trim();
                    Natural::from_owned_limbs_asc(rs)
                }
            }
        }
    }
}

impl DivRem<Natural> for Natural {
    type DivOutput = Natural;
    type RemOutput = Natural;

    /// Divides a [`Natural`] by another [`Natural`], taking both by value and returning the
    /// quotient and remainder. The quotient is rounded towards zero.
    ///
    /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq r < y$.
    ///
    /// $$
    /// f(x, y) = \left ( \left \lfloor \frac{x}{y} \right \rfloor, \space
    /// x - y\left \lfloor \frac{x}{y} \right \rfloor \right ).
    /// $$
    ///
    /// For [`Natural`]s, `div_rem` is equivalent to
    /// [`div_mod`](malachite_base::num::arithmetic::traits::DivMod::div_mod).
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
    /// use malachite_base::num::arithmetic::traits::DivRem;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(
    ///     Natural::from(23u32)
    ///         .div_rem(Natural::from(10u32))
    ///         .to_debug_string(),
    ///     "(2, 3)"
    /// );
    ///
    /// // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    /// assert_eq!(
    ///     Natural::from_str("1000000000000000000000000")
    ///         .unwrap()
    ///         .div_rem(Natural::from_str("1234567890987").unwrap())
    ///         .to_debug_string(),
    ///     "(810000006723, 530068894399)"
    /// );
    /// ```
    #[inline]
    fn div_rem(self, other: Natural) -> (Natural, Natural) {
        self.div_mod(other)
    }
}

impl<'a> DivRem<&'a Natural> for Natural {
    type DivOutput = Natural;
    type RemOutput = Natural;

    /// Divides a [`Natural`] by another [`Natural`], taking the first by value and the second by
    /// reference and returning the quotient and remainder. The quotient is rounded towards zero.
    ///
    /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq r < y$.
    ///
    /// $$
    /// f(x, y) = \left ( \left \lfloor \frac{x}{y} \right \rfloor, \space
    /// x - y\left \lfloor \frac{x}{y} \right \rfloor \right ).
    /// $$
    ///
    /// For [`Natural`]s, `div_rem` is equivalent to
    /// [`div_mod`](malachite_base::num::arithmetic::traits::DivMod::div_mod).
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
    /// use malachite_base::num::arithmetic::traits::DivRem;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(
    ///     Natural::from(23u32)
    ///         .div_rem(&Natural::from(10u32))
    ///         .to_debug_string(),
    ///     "(2, 3)"
    /// );
    ///
    /// // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    /// assert_eq!(
    ///     Natural::from_str("1000000000000000000000000")
    ///         .unwrap()
    ///         .div_rem(&Natural::from_str("1234567890987").unwrap())
    ///         .to_debug_string(),
    ///     "(810000006723, 530068894399)"
    /// );
    /// ```
    #[inline]
    fn div_rem(self, other: &'a Natural) -> (Natural, Natural) {
        self.div_mod(other)
    }
}

impl<'a> DivRem<Natural> for &'a Natural {
    type DivOutput = Natural;
    type RemOutput = Natural;

    /// Divides a [`Natural`] by another [`Natural`], taking the first by reference and the second
    /// by value and returning the quotient and remainder. The quotient is rounded towards zero.
    ///
    /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq r < y$.
    ///
    /// $$
    /// f(x, y) = \left ( \left \lfloor \frac{x}{y} \right \rfloor, \space
    /// x - y\left \lfloor \frac{x}{y} \right \rfloor \right ).
    /// $$
    ///
    /// For [`Natural`]s, `div_rem` is equivalent to
    /// [`div_mod`](malachite_base::num::arithmetic::traits::DivMod::div_mod).
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
    /// use malachite_base::num::arithmetic::traits::DivRem;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(
    ///     (&Natural::from(23u32))
    ///         .div_rem(Natural::from(10u32))
    ///         .to_debug_string(),
    ///     "(2, 3)"
    /// );
    ///
    /// // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    /// assert_eq!(
    ///     (&Natural::from_str("1000000000000000000000000").unwrap())
    ///         .div_rem(Natural::from_str("1234567890987").unwrap())
    ///         .to_debug_string(),
    ///     "(810000006723, 530068894399)"
    /// );
    /// ```
    #[inline]
    fn div_rem(self, other: Natural) -> (Natural, Natural) {
        self.div_mod(other)
    }
}

impl<'a, 'b> DivRem<&'b Natural> for &'a Natural {
    type DivOutput = Natural;
    type RemOutput = Natural;

    /// Divides a [`Natural`] by another [`Natural`], taking both by reference and returning the
    /// quotient and remainder. The quotient is rounded towards zero.
    ///
    /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq r < y$.
    ///
    /// $$
    /// f(x, y) = \left ( \left \lfloor \frac{x}{y} \right \rfloor, \space
    /// x - y\left \lfloor \frac{x}{y} \right \rfloor \right ).
    /// $$
    ///
    /// For [`Natural`]s, `div_rem` is equivalent to
    /// [`div_mod`](malachite_base::num::arithmetic::traits::DivMod::div_mod).
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
    /// use malachite_base::num::arithmetic::traits::DivRem;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(
    ///     (&Natural::from(23u32))
    ///         .div_rem(&Natural::from(10u32))
    ///         .to_debug_string(),
    ///     "(2, 3)"
    /// );
    ///
    /// // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    /// assert_eq!(
    ///     (&Natural::from_str("1000000000000000000000000").unwrap())
    ///         .div_rem(&Natural::from_str("1234567890987").unwrap())
    ///         .to_debug_string(),
    ///     "(810000006723, 530068894399)"
    /// );
    /// ```
    #[inline]
    fn div_rem(self, other: &'b Natural) -> (Natural, Natural) {
        self.div_mod(other)
    }
}

impl DivAssignRem<Natural> for Natural {
    type RemOutput = Natural;

    /// Divides a [`Natural`] by another [`Natural`] in place, taking the [`Natural`] on the
    /// right-hand side by value and returning the remainder. The quotient is rounded towards zero.
    ///
    /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq r < y$.
    ///
    /// $$
    /// f(x, y) = x - y\left \lfloor \frac{x}{y} \right \rfloor,
    /// $$
    /// $$
    /// x \gets \left \lfloor \frac{x}{y} \right \rfloor.
    /// $$
    ///
    /// For [`Natural`]s, `div_assign_rem` is equivalent to
    /// [`div_assign_mod`](malachite_base::num::arithmetic::traits::DivAssignMod::div_assign_mod).
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
    /// use malachite_base::num::arithmetic::traits::DivAssignRem;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 10 + 3 = 23
    /// let mut x = Natural::from(23u32);
    /// assert_eq!(x.div_assign_rem(Natural::from(10u32)), 3);
    /// assert_eq!(x, 2);
    ///
    /// // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    /// let mut x = Natural::from_str("1000000000000000000000000").unwrap();
    /// assert_eq!(
    ///     x.div_assign_rem(Natural::from_str("1234567890987").unwrap()),
    ///     530068894399u64
    /// );
    /// assert_eq!(x, 810000006723u64);
    /// ```
    #[inline]
    fn div_assign_rem(&mut self, other: Natural) -> Natural {
        self.div_assign_mod(other)
    }
}

impl<'a> DivAssignRem<&'a Natural> for Natural {
    type RemOutput = Natural;

    /// Divides a [`Natural`] by another [`Natural`] in place, taking the [`Natural`] on the
    /// right-hand side by reference and returning the remainder. The quotient is rounded towards
    /// zero.
    ///
    /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq r < y$.
    ///
    /// $$
    /// f(x, y) = x - y\left \lfloor \frac{x}{y} \right \rfloor,
    /// $$
    /// $$
    /// x \gets \left \lfloor \frac{x}{y} \right \rfloor.
    /// $$
    ///
    /// For [`Natural`]s, `div_assign_rem` is equivalent to
    /// [`div_assign_mod`](malachite_base::num::arithmetic::traits::DivAssignMod::div_assign_mod).
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
    /// use malachite_base::num::arithmetic::traits::DivAssignRem;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 10 + 3 = 23
    /// let mut x = Natural::from(23u32);
    /// assert_eq!(x.div_assign_rem(&Natural::from(10u32)), 3);
    /// assert_eq!(x, 2);
    ///
    /// // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    /// let mut x = Natural::from_str("1000000000000000000000000").unwrap();
    /// assert_eq!(
    ///     x.div_assign_rem(&Natural::from_str("1234567890987").unwrap()),
    ///     530068894399u64
    /// );
    /// assert_eq!(x, 810000006723u64);
    /// ```
    #[inline]
    fn div_assign_rem(&mut self, other: &'a Natural) -> Natural {
        self.div_assign_mod(other)
    }
}

impl CeilingDivNegMod<Natural> for Natural {
    type DivOutput = Natural;
    type ModOutput = Natural;

    /// Divides a [`Natural`] by another [`Natural`], taking both by value and returning the ceiling
    /// of the quotient and the remainder of the negative of the first [`Natural`] divided by the
    /// second.
    ///
    /// The quotient and remainder satisfy $x = qy - r$ and $0 \leq r < y$.
    ///
    /// $$
    /// f(x, y) = \left ( \left \lceil \frac{x}{y} \right \rceil, \space
    /// y\left \lceil \frac{x}{y} \right \rceil - x \right ).
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
    /// use malachite_base::num::arithmetic::traits::CeilingDivNegMod;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 3 * 10 - 7 = 23
    /// assert_eq!(
    ///     Natural::from(23u32)
    ///         .ceiling_div_neg_mod(Natural::from(10u32))
    ///         .to_debug_string(),
    ///     "(3, 7)"
    /// );
    ///
    /// // 810000006724 * 1234567890987 - 704498996588 = 1000000000000000000000000
    /// assert_eq!(
    ///     Natural::from_str("1000000000000000000000000")
    ///         .unwrap()
    ///         .ceiling_div_neg_mod(Natural::from_str("1234567890987").unwrap())
    ///         .to_debug_string(),
    ///     "(810000006724, 704498996588)"
    /// );
    /// ```
    #[inline]
    fn ceiling_div_neg_mod(mut self, other: Natural) -> (Natural, Natural) {
        let r = self.ceiling_div_assign_neg_mod(other);
        (self, r)
    }
}

impl<'a> CeilingDivNegMod<&'a Natural> for Natural {
    type DivOutput = Natural;
    type ModOutput = Natural;

    /// Divides a [`Natural`] by another [`Natural`], taking the first by value and the second by
    /// reference and returning the ceiling of the quotient and the remainder of the negative of the
    /// first [`Natural`] divided by the second.
    ///
    /// The quotient and remainder satisfy $x = qy - r$ and $0 \leq r < y$.
    ///
    /// $$
    /// f(x, y) = \left ( \left \lceil \frac{x}{y} \right \rceil, \space
    /// y\left \lceil \frac{x}{y} \right \rceil - x \right ).
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
    /// use malachite_base::num::arithmetic::traits::CeilingDivNegMod;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 3 * 10 - 7 = 23
    /// assert_eq!(
    ///     Natural::from(23u32)
    ///         .ceiling_div_neg_mod(&Natural::from(10u32))
    ///         .to_debug_string(),
    ///     "(3, 7)"
    /// );
    ///
    /// // 810000006724 * 1234567890987 - 704498996588 = 1000000000000000000000000
    /// assert_eq!(
    ///     Natural::from_str("1000000000000000000000000")
    ///         .unwrap()
    ///         .ceiling_div_neg_mod(&Natural::from_str("1234567890987").unwrap())
    ///         .to_debug_string(),
    ///     "(810000006724, 704498996588)"
    /// );
    /// ```
    #[inline]
    fn ceiling_div_neg_mod(mut self, other: &'a Natural) -> (Natural, Natural) {
        let r = self.ceiling_div_assign_neg_mod(other);
        (self, r)
    }
}

impl<'a> CeilingDivNegMod<Natural> for &'a Natural {
    type DivOutput = Natural;
    type ModOutput = Natural;

    /// Divides a [`Natural`] by another [`Natural`], taking the first by reference and the second
    /// by value and returning the ceiling of the quotient and the remainder of the negative of the
    /// first [`Natural`] divided by the second.
    ///
    /// The quotient and remainder satisfy $x = qy - r$ and $0 \leq r < y$.
    ///
    /// $$
    /// f(x, y) = \left ( \left \lceil \frac{x}{y} \right \rceil, \space
    /// y\left \lceil \frac{x}{y} \right \rceil - x \right ).
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
    /// use malachite_base::num::arithmetic::traits::CeilingDivNegMod;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 3 * 10 - 7 = 23
    /// assert_eq!(
    ///     (&Natural::from(23u32))
    ///         .ceiling_div_neg_mod(Natural::from(10u32))
    ///         .to_debug_string(),
    ///     "(3, 7)"
    /// );
    ///
    /// // 810000006724 * 1234567890987 - 704498996588 = 1000000000000000000000000
    /// assert_eq!(
    ///     (&Natural::from_str("1000000000000000000000000").unwrap())
    ///         .ceiling_div_neg_mod(Natural::from_str("1234567890987").unwrap())
    ///         .to_debug_string(),
    ///     "(810000006724, 704498996588)"
    /// );
    /// ```
    fn ceiling_div_neg_mod(self, other: Natural) -> (Natural, Natural) {
        let (q, r) = self.div_mod(&other);
        if r == 0 {
            (q, r)
        } else {
            (q.add_limb(1), other - r)
        }
    }
}

impl<'a, 'b> CeilingDivNegMod<&'b Natural> for &'a Natural {
    type DivOutput = Natural;
    type ModOutput = Natural;

    /// Divides a [`Natural`] by another [`Natural`], taking both by reference and returning the
    /// ceiling of the quotient and the remainder of the negative of the first [`Natural`] divided
    /// by the second.
    ///
    /// The quotient and remainder satisfy $x = qy - r$ and $0 \leq r < y$.
    ///
    /// $$
    /// f(x, y) = \left ( \left \lceil \frac{x}{y} \right \rceil, \space
    /// y\left \lceil \frac{x}{y} \right \rceil - x \right ).
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
    /// use malachite_base::num::arithmetic::traits::CeilingDivNegMod;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 3 * 10 - 7 = 23
    /// assert_eq!(
    ///     (&Natural::from(23u32))
    ///         .ceiling_div_neg_mod(&Natural::from(10u32))
    ///         .to_debug_string(),
    ///     "(3, 7)"
    /// );
    ///
    /// // 810000006724 * 1234567890987 - 704498996588 = 1000000000000000000000000
    /// assert_eq!(
    ///     (&Natural::from_str("1000000000000000000000000").unwrap())
    ///         .ceiling_div_neg_mod(&Natural::from_str("1234567890987").unwrap())
    ///         .to_debug_string(),
    ///     "(810000006724, 704498996588)"
    /// );
    /// ```
    fn ceiling_div_neg_mod(self, other: &'b Natural) -> (Natural, Natural) {
        let (q, r) = self.div_mod(other);
        if r == 0 {
            (q, r)
        } else {
            (q.add_limb(1), other - r)
        }
    }
}

impl CeilingDivAssignNegMod<Natural> for Natural {
    type ModOutput = Natural;

    /// Divides a [`Natural`] by another [`Natural`] in place, taking the [`Natural`] on the
    /// right-hand side by value and returning the remainder of the negative of the first number
    /// divided by the second.
    ///
    /// The quotient and remainder satisfy $x = qy - r$ and $0 \leq r < y$.
    ///
    /// $$
    /// f(x, y) = y\left \lceil \frac{x}{y} \right \rceil - x,
    /// $$
    /// $$
    /// x \gets \left \lceil \frac{x}{y} \right \rceil.
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
    /// use malachite_base::num::arithmetic::traits::CeilingDivAssignNegMod;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 3 * 10 - 7 = 23
    /// let mut x = Natural::from(23u32);
    /// assert_eq!(x.ceiling_div_assign_neg_mod(Natural::from(10u32)), 7);
    /// assert_eq!(x, 3);
    ///
    /// // 810000006724 * 1234567890987 - 704498996588 = 1000000000000000000000000
    /// let mut x = Natural::from_str("1000000000000000000000000").unwrap();
    /// assert_eq!(
    ///     x.ceiling_div_assign_neg_mod(Natural::from_str("1234567890987").unwrap()),
    ///     704498996588u64,
    /// );
    /// assert_eq!(x, 810000006724u64);
    /// ```
    fn ceiling_div_assign_neg_mod(&mut self, other: Natural) -> Natural {
        let r = self.div_assign_mod(&other);
        if r == 0 {
            Natural::ZERO
        } else {
            *self += Natural::ONE;
            other - r
        }
    }
}

impl<'a> CeilingDivAssignNegMod<&'a Natural> for Natural {
    type ModOutput = Natural;

    /// Divides a [`Natural`] by another [`Natural`] in place, taking the [`Natural`] on the
    /// right-hand side by reference and returning the remainder of the negative of the first number
    /// divided by the second.
    ///
    /// The quotient and remainder satisfy $x = qy - r$ and $0 \leq r < y$.
    ///
    /// $$
    /// f(x, y) = y\left \lceil \frac{x}{y} \right \rceil - x,
    /// $$
    /// $$
    /// x \gets \left \lceil \frac{x}{y} \right \rceil.
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
    /// use malachite_base::num::arithmetic::traits::CeilingDivAssignNegMod;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 3 * 10 - 7 = 23
    /// let mut x = Natural::from(23u32);
    /// assert_eq!(x.ceiling_div_assign_neg_mod(&Natural::from(10u32)), 7);
    /// assert_eq!(x, 3);
    ///
    /// // 810000006724 * 1234567890987 - 704498996588 = 1000000000000000000000000
    /// let mut x = Natural::from_str("1000000000000000000000000").unwrap();
    /// assert_eq!(
    ///     x.ceiling_div_assign_neg_mod(&Natural::from_str("1234567890987").unwrap()),
    ///     704498996588u64,
    /// );
    /// assert_eq!(x, 810000006724u64);
    /// ```
    fn ceiling_div_assign_neg_mod(&mut self, other: &'a Natural) -> Natural {
        let r = self.div_assign_mod(other);
        if r == 0 {
            Natural::ZERO
        } else {
            *self += Natural::ONE;
            other - r
        }
    }
}
