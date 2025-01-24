// Copyright © 2025 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      `mpn_div_qr_1n_pi1` contributed to the GNU project by Niels Möller
//
//      `mpn_div_qr_1` contributed to the GNU project by Niels Möller and Torbjörn Granlund
//
//      Copyright © 1991, 1993, 1994, 1996, 1998-2000, 2002, 2003, 2013 Free Software Foundation,
//      Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::add::limbs_slice_add_limb_in_place;
use crate::natural::arithmetic::div_mod::{div_mod_by_preinversion, limbs_invert_limb};
use crate::natural::arithmetic::shl::{limbs_shl_to_out, limbs_slice_shl_in_place};
use crate::platform::{DoubleLimb, Limb};
use malachite_base::num::arithmetic::traits::{
    DivRem, OverflowingAddAssign, WrappingAddAssign, WrappingSubAssign, XMulYToZZ,
};
use malachite_base::num::conversion::traits::{JoinHalves, SplitInHalf};
use malachite_base::num::logic::traits::LeadingZeros;

pub fn rug_ceiling_div_neg_mod(x: rug::Integer, y: rug::Integer) -> (rug::Integer, rug::Integer) {
    let (quotient, remainder) = x.div_rem_ceil(y);
    (quotient, -remainder)
}

pub fn limbs_div_limb_to_out_mod_naive(out: &mut [Limb], xs: &[Limb], d: Limb) -> Limb {
    assert!(out.len() >= xs.len());
    let d = DoubleLimb::from(d);
    let mut upper = 0;
    for (out_limb, &in_limb) in out.iter_mut().zip(xs.iter()).rev() {
        let (q, r) = DoubleLimb::join_halves(upper, in_limb).div_rem(d);
        *out_limb = q.lower_half();
        upper = r.lower_half();
    }
    upper
}

pub fn limbs_div_limb_in_place_mod_naive(xs: &mut [Limb], d: Limb) -> Limb {
    let d = DoubleLimb::from(d);
    let mut upper = 0;
    for limb in xs.iter_mut().rev() {
        let (q, r) = DoubleLimb::join_halves(upper, *limb).div_rem(d);
        *limb = q.lower_half();
        upper = r.lower_half();
    }
    upper
}

// The high bit of `d` must be set.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// This is equivalent to `mpn_div_qr_1n_pi1` from `mpn/generic/div_qr_1n_pi1.c`, GMP 6.2.1, with
// `DIV_QR_1N_METHOD == 2`, where `qp == up`.
fn limbs_div_limb_normalized_in_place_mod(
    ns: &mut [Limb],
    ns_high: Limb,
    d: Limb,
    d_inv: Limb,
) -> Limb {
    let len = ns.len();
    if len == 1 {
        let r;
        (ns[0], r) = div_mod_by_preinversion(ns_high, ns[0], d, d_inv);
        return r;
    }
    let power_of_2 = d.wrapping_neg().wrapping_mul(d_inv);
    let (mut q_high, mut q_low) = Limb::x_mul_y_to_zz(d_inv, ns_high);
    q_high.wrapping_add_assign(ns_high);
    let ns_2nd_to_last = ns[len - 1];
    ns[len - 1] = q_high;
    let (sum, mut big_carry) = DoubleLimb::join_halves(ns_2nd_to_last, ns[len - 2])
        .overflowing_add(DoubleLimb::from(power_of_2) * DoubleLimb::from(ns_high));
    let (mut sum_high, mut sum_low) = sum.split_in_half();
    for j in (0..len - 2).rev() {
        let (t, r) = Limb::x_mul_y_to_zz(sum_high, d_inv);
        let mut q = DoubleLimb::from(sum_high) + DoubleLimb::from(t) + DoubleLimb::from(q_low);
        q_low = r;
        if big_carry {
            q.wrapping_add_assign(DoubleLimb::join_halves(1, d_inv));
            if sum_low.overflowing_add_assign(power_of_2) {
                sum_low.wrapping_sub_assign(d);
                q.wrapping_add_assign(1);
            }
        }
        let (q_higher, q_high) = q.split_in_half();
        ns[j + 1] = q_high;
        assert!(!limbs_slice_add_limb_in_place(&mut ns[j + 2..], q_higher));
        let (sum, carry) = DoubleLimb::join_halves(sum_low, ns[j])
            .overflowing_add(DoubleLimb::from(sum_high) * DoubleLimb::from(power_of_2));
        sum_high = sum.upper_half();
        sum_low = sum.lower_half();
        big_carry = carry;
    }
    let mut q_high = 0;
    if big_carry {
        q_high += 1;
        sum_high.wrapping_sub_assign(d);
    }
    if sum_high >= d {
        q_high += 1;
        sum_high.wrapping_sub_assign(d);
    }
    let (t, r) = div_mod_by_preinversion(sum_high, sum_low, d, d_inv);
    let (q_high, q_low) = DoubleLimb::join_halves(q_high, q_low)
        .wrapping_add(DoubleLimb::from(t))
        .split_in_half();
    assert!(!limbs_slice_add_limb_in_place(&mut ns[1..], q_high));
    ns[0] = q_low;
    r
}

// The high bit of `d` must be set.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// This is equivalent to `mpn_div_qr_1n_pi1` from `mpn/generic/div_qr_1n_pi1.c`, GMP 6.2.1, with
// `DIV_QR_1N_METHOD == 2`.
fn limbs_div_limb_normalized_to_out_mod(
    out: &mut [Limb],
    ns: &[Limb],
    n_last: Limb,
    d: Limb,
    d_inv: Limb,
) -> Limb {
    let len = ns.len();
    if len == 1 {
        let (q, r) = div_mod_by_preinversion(n_last, ns[0], d, d_inv);
        out[0] = q;
        return r;
    }
    let power_of_2 = d.wrapping_neg().wrapping_mul(d_inv);
    let (mut q_high, mut q_low) = Limb::x_mul_y_to_zz(d_inv, n_last);
    q_high.wrapping_add_assign(n_last);
    out[len - 1] = q_high;
    let (sum, mut big_carry) = DoubleLimb::join_halves(ns[len - 1], ns[len - 2])
        .overflowing_add(DoubleLimb::from(power_of_2) * DoubleLimb::from(n_last));
    let (mut sum_high, mut sum_low) = sum.split_in_half();
    for j in (0..len - 2).rev() {
        let (t, r) = Limb::x_mul_y_to_zz(sum_high, d_inv);
        let mut q = DoubleLimb::from(sum_high) + DoubleLimb::from(t) + DoubleLimb::from(q_low);
        q_low = r;
        if big_carry {
            q.wrapping_add_assign(DoubleLimb::join_halves(1, d_inv));
            let (sum, carry) = sum_low.overflowing_add(power_of_2);
            sum_low = sum;
            if carry {
                sum_low.wrapping_sub_assign(d);
                q.wrapping_add_assign(1);
            }
        }
        let (q_higher, q_high) = q.split_in_half();
        out[j + 1] = q_high;
        assert!(!limbs_slice_add_limb_in_place(&mut out[j + 2..], q_higher));
        let (sum, carry) = DoubleLimb::join_halves(sum_low, ns[j])
            .overflowing_add(DoubleLimb::from(sum_high) * DoubleLimb::from(power_of_2));
        sum_high = sum.upper_half();
        sum_low = sum.lower_half();
        big_carry = carry;
    }
    let mut q_high = 0;
    if big_carry {
        q_high += 1;
        sum_high.wrapping_sub_assign(d);
    }
    if sum_high >= d {
        q_high += 1;
        sum_high.wrapping_sub_assign(d);
    }
    let (t, r) = div_mod_by_preinversion(sum_high, sum_low, d, d_inv);
    let (q_high, q_low) = DoubleLimb::join_halves(q_high, q_low)
        .wrapping_add(DoubleLimb::from(t))
        .split_in_half();
    assert!(!limbs_slice_add_limb_in_place(&mut out[1..], q_high));
    out[0] = q_low;
    r
}

/// This is equivalent to `mpn_div_qr_1` from `mpn/generic/div_qr_1.c`, GMP 6.2.1, where `len > 1`.
/// Experiments show that this is always slower than `limbs_div_limb_to_out_mod`.
pub fn limbs_div_limb_to_out_mod_alt(out: &mut [Limb], ns: &[Limb], d: Limb) -> Limb {
    assert_ne!(d, 0);
    let len = ns.len();
    assert!(len > 1);
    let out = &mut out[..len];
    assert!(out.len() >= len);
    let (ns_last, ns_init) = ns.split_last().unwrap();
    let mut ns_last = *ns_last;
    let bits = LeadingZeros::leading_zeros(d);
    if bits == 0 {
        let (out_last, out_init) = out.split_last_mut().unwrap();
        *out_last = if ns_last >= d {
            ns_last -= d;
            1
        } else {
            0
        };
        let d_inv = limbs_invert_limb(d);
        limbs_div_limb_normalized_to_out_mod(out_init, ns_init, ns_last, d, d_inv)
    } else {
        let d = d << bits;
        let ns_last = limbs_shl_to_out(out, ns, bits);
        let d_inv = limbs_invert_limb(d);
        let (out_last, out_init) = out.split_last_mut().unwrap();
        let (q, r) = div_mod_by_preinversion(ns_last, *out_last, d, d_inv);
        *out_last = q;
        limbs_div_limb_normalized_in_place_mod(out_init, r, d, d_inv) >> bits
    }
}

/// This is equivalent to `mpn_div_qr_1` from `mpn/generic/div_qr_1.c`, GMP 6.2.1, where `qp == up`
/// and `len > 1`. Experiments show that this is always slower than `limbs_div_limb_in_place_mod`.
pub fn limbs_div_limb_in_place_mod_alt(ns: &mut [Limb], d: Limb) -> Limb {
    assert_ne!(d, 0);
    let len = ns.len();
    assert!(len > 1);
    let len_minus_1 = len - 1;
    let mut ns_last = ns[len_minus_1];
    let bits = LeadingZeros::leading_zeros(d);
    if bits == 0 {
        ns[len_minus_1] = if ns_last >= d {
            ns_last -= d;
            1
        } else {
            0
        };
        let d_inv = limbs_invert_limb(d);
        limbs_div_limb_normalized_in_place_mod(&mut ns[..len_minus_1], ns_last, d, d_inv)
    } else {
        let d = d << bits;
        let ns_last = limbs_slice_shl_in_place(ns, bits);
        let d_inv = limbs_invert_limb(d);
        let (q, r) = div_mod_by_preinversion(ns_last, ns[len_minus_1], d, d_inv);
        ns[len_minus_1] = q;
        limbs_div_limb_normalized_in_place_mod(&mut ns[..len_minus_1], r, d, d_inv) >> bits
    }
}
