// Copyright © 2024 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      `mpn_bdiv_q`, `mpn_bdiv_q_itch`, `mpn_binvert`, `mpn_binvert_itch`, `mpn_divexact`,
//      `mpn_mu_bdiv_q`, `mpn_mu_bdiv_q_itch`, `mpn_dcpi1_bdiv_qr`, `mpn_dcpi1_bdiv_qr_n`,
//      `mpn_dcpi1_bdiv_qr_n_itch`, and `mpn_sbpi1_bdiv_q` contributed to the GNU project by
//      Torbjörn Granlund.
//
//      `mpn_dcpi1_bdiv_q`, `mpn_dcpi1_bdiv_q_n`, `mpn_dcpi1_bdiv_q_n_itch`, `mpn_dcpi1_bdiv_qr`,
//      `mpn_dcpi1_bdiv_qr_n`, `mpn_dcpi1_bdiv_qr_n_itch`, and `mpn_sbpi1_bdiv_qr` contributed to
//      the GNU project by Niels Möller and Torbjörn Granlund.
//
//      Copyright © 1991-2018 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::conversion::to_twos_complement_limbs::limbs_twos_complement_in_place;
use crate::natural::arithmetic::add::{
    limbs_slice_add_greater_in_place_left, limbs_slice_add_limb_in_place,
    limbs_slice_add_same_length_in_place_left,
};
use crate::natural::arithmetic::add_mul::limbs_slice_add_mul_limb_same_length_in_place_left;
use crate::natural::arithmetic::div::{
    limbs_div_divisor_of_limb_max_with_carry_in_place,
    limbs_div_divisor_of_limb_max_with_carry_to_out,
};
use crate::natural::arithmetic::div_mod::MUL_TO_MULMOD_BNM1_FOR_2NXN_THRESHOLD;
use crate::natural::arithmetic::mul::mul_low::limbs_mul_low_same_length;
use crate::natural::arithmetic::mul::mul_mod::{
    limbs_mul_mod_base_pow_n_minus_1, limbs_mul_mod_base_pow_n_minus_1_next_size,
    limbs_mul_mod_base_pow_n_minus_1_scratch_len,
};
use crate::natural::arithmetic::mul::{
    limbs_mul_greater_to_out, limbs_mul_greater_to_out_scratch_len, limbs_mul_to_out,
    limbs_mul_to_out_scratch_len,
};
use crate::natural::arithmetic::neg::limbs_neg_in_place;
use crate::natural::arithmetic::shr::{limbs_shr_to_out, limbs_slice_shr_in_place};
use crate::natural::arithmetic::sub::{
    limbs_sub_greater_in_place_left, limbs_sub_limb_in_place, limbs_sub_limb_to_out,
    limbs_sub_same_length_in_place_left, limbs_sub_same_length_to_out,
    limbs_sub_same_length_to_out_with_overlap, limbs_sub_same_length_with_borrow_in_to_out,
};
use crate::natural::arithmetic::sub_mul::limbs_sub_mul_limb_same_length_in_place_left;
use crate::natural::comparison::cmp::limbs_cmp_same_length;
use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::{
    DoubleLimb, Limb, BINV_NEWTON_THRESHOLD, DC_BDIV_QR_THRESHOLD, DC_BDIV_Q_THRESHOLD,
    MU_BDIV_Q_THRESHOLD,
};
use alloc::vec::Vec;
use core::cmp::{max, min, Ordering::*};
use core::mem::swap;
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{
    DivExact, DivExactAssign, ModPowerOf2, Parity, ShrRound, ShrRoundAssign, WrappingAddAssign,
    WrappingMulAssign, WrappingSubAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, SplitInHalf};
use malachite_base::num::logic::traits::TrailingZeros;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::slices::{slice_leading_zeros, slice_set_zero, slice_test_zero};

const INVERT_LIMB_TABLE_LOG_SIZE: u64 = 7;

const INVERT_LIMB_TABLE_SIZE: usize = 1 << INVERT_LIMB_TABLE_LOG_SIZE;

// The entry at index `i` is the multiplicative inverse of `2 * i + 1 mod 2 ^ 8`.
const INVERT_LIMB_TABLE: [u8; INVERT_LIMB_TABLE_SIZE] = [
    0x01, 0xab, 0xcd, 0xb7, 0x39, 0xa3, 0xc5, 0xef, 0xf1, 0x1b, 0x3d, 0xa7, 0x29, 0x13, 0x35, 0xdf,
    0xe1, 0x8b, 0xad, 0x97, 0x19, 0x83, 0xa5, 0xcf, 0xd1, 0xfb, 0x1d, 0x87, 0x09, 0xf3, 0x15, 0xbf,
    0xc1, 0x6b, 0x8d, 0x77, 0xf9, 0x63, 0x85, 0xaf, 0xb1, 0xdb, 0xfd, 0x67, 0xe9, 0xd3, 0xf5, 0x9f,
    0xa1, 0x4b, 0x6d, 0x57, 0xd9, 0x43, 0x65, 0x8f, 0x91, 0xbb, 0xdd, 0x47, 0xc9, 0xb3, 0xd5, 0x7f,
    0x81, 0x2b, 0x4d, 0x37, 0xb9, 0x23, 0x45, 0x6f, 0x71, 0x9b, 0xbd, 0x27, 0xa9, 0x93, 0xb5, 0x5f,
    0x61, 0x0b, 0x2d, 0x17, 0x99, 0x03, 0x25, 0x4f, 0x51, 0x7b, 0x9d, 0x07, 0x89, 0x73, 0x95, 0x3f,
    0x41, 0xeb, 0x0d, 0xf7, 0x79, 0xe3, 0x05, 0x2f, 0x31, 0x5b, 0x7d, 0xe7, 0x69, 0x53, 0x75, 0x1f,
    0x21, 0xcb, 0xed, 0xd7, 0x59, 0xc3, 0xe5, 0x0f, 0x11, 0x3b, 0x5d, 0xc7, 0x49, 0x33, 0x55, 0xff,
];

// Tests that `INVERT_LIMB_TABLE` is correct.
#[cfg(feature = "test_build")]
pub fn test_invert_limb_table() {
    for (i, &inv) in INVERT_LIMB_TABLE.iter().enumerate() {
        let value = (u8::exact_from(i) << 1) + 1;
        let product = value.wrapping_mul(inv);
        assert_eq!(
            product, 1,
            "INVERT_LIMB_TABLE gives incorrect inverse, {inv}, for value {value}",
        );
    }
}

// Finds the inverse of a `Limb` mod `2 ^ Limb::WIDTH`; given x, returns y such that x * y ≡ 1 mod
// `2 ^ Limb::WIDTH`. This inverse only exists for odd `Limb`s, so `x` must be odd.
//
// # Worst-case complexity
// Constant time and additional memory.
//
// # Panics
// Panics if `x` is even.
//
// This is equivalent to `binvert_limb` from `gmp-impl.h`, GMP 6.2.1.
pub_crate_test! {limbs_modular_invert_limb(x: Limb) -> Limb {
    assert!(x.odd());
    let index = (x >> 1).mod_power_of_2(INVERT_LIMB_TABLE_LOG_SIZE);
    let mut inv = Limb::from(INVERT_LIMB_TABLE[usize::exact_from(index)]);
    inv = (inv << 1).wrapping_sub((inv * inv).wrapping_mul(x));
    inv = (inv << 1).wrapping_sub(inv.wrapping_mul(inv).wrapping_mul(x));
    if !cfg!(feature = "32_bit_limbs") {
        inv = (inv << 1).wrapping_sub(inv.wrapping_mul(inv).wrapping_mul(x));
    }
    inv
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
// quotient limbs of the `Natural` divided by a `Limb`. The divisor limb cannot be zero and the limb
// slice must be nonempty. The `Natural` must be exactly divisible by the `Limb`. If it isn't, the
// behavior of this function is undefined.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// # Panics
// Panics if `ns` is empty or if `d` is zero.
//
// This is equivalent to `mpn_divexact_1` from `mpn/generic/dive_1.c`, GMP 6.2.1, where the result
// is returned.
pub_test! {limbs_div_exact_limb_no_special_3(ns: &[Limb], d: Limb) -> Vec<Limb> {
    let mut q = vec![0; ns.len()];
    limbs_div_exact_limb_to_out(&mut q, ns, d);
    q
}}

// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// # Panics
// Panics if `out` is shorter than `ns`, `ns` is empty, or if `d` is zero.
//
// This is equivalent to `mpn_divexact_1` from `mpn/generic/dive_1.c`, GMP 6.2.1.
pub_test! {limbs_div_exact_limb_to_out_no_special_3(out: &mut [Limb], ns: &[Limb], d: Limb) {
    assert_ne!(d, 0);
    let len = ns.len();
    assert_ne!(len, 0);
    let out = &mut out[..len];
    let (ns_head, ns_tail) = ns.split_first().unwrap();
    if d.even() {
        let shift = TrailingZeros::trailing_zeros(d);
        let shift_complement = Limb::WIDTH - shift;
        let shifted_d = d >> shift;
        let d_inv = limbs_modular_invert_limb(shifted_d);
        let (out_last, out_init) = out.split_last_mut().unwrap();
        let mut upper_half = 0;
        let mut previous_n = *ns_head;
        for (out_q, n) in out_init.iter_mut().zip(ns_tail.iter()) {
            let shifted_n = (previous_n >> shift) | (n << shift_complement);
            previous_n = *n;
            let (diff, carry) = shifted_n.overflowing_sub(upper_half);
            let q = diff.wrapping_mul(d_inv);
            *out_q = q;
            upper_half = (DoubleLimb::from(q) * DoubleLimb::from(shifted_d)).upper_half();
            if carry {
                upper_half += 1;
            }
        }
        *out_last = (previous_n >> shift)
            .wrapping_sub(upper_half)
            .wrapping_mul(d_inv);
    } else {
        let d_inv = limbs_modular_invert_limb(d);
        let (out_head, out_tail) = out.split_first_mut().unwrap();
        let mut q = ns_head.wrapping_mul(d_inv);
        *out_head = q;
        let mut previous_carry = false;
        for (out_q, n) in out_tail.iter_mut().zip(ns_tail.iter()) {
            let mut upper_half = (DoubleLimb::from(q) * DoubleLimb::from(d)).upper_half();
            if previous_carry {
                upper_half += 1;
            }
            let diff;
            (diff, previous_carry) = n.overflowing_sub(upper_half);
            q = diff.wrapping_mul(d_inv);
            *out_q = q;
        }
    }
}}

// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// # Panics
// Panics if `ns` is empty or if `d` is zero.
//
// This is equivalent to `mpn_divexact_1` from `mpn/generic/dive_1.c`, GMP 6.2.1, where `dst ==
// src`.
pub_test! {limbs_div_exact_limb_in_place_no_special_3(ns: &mut [Limb], d: Limb) {
    assert_ne!(d, 0);
    let len = ns.len();
    assert_ne!(len, 0);
    if d.even() {
        let shift = TrailingZeros::trailing_zeros(d);
        let shift_complement = Limb::WIDTH - shift;
        let shifted_d = d >> shift;
        let d_inv = limbs_modular_invert_limb(shifted_d);
        let shifted_d = DoubleLimb::from(shifted_d);
        let mut upper_half = 0;
        let mut previous_n = ns[0];
        for i in 1..len {
            let n = ns[i];
            let shifted_n = (previous_n >> shift) | (n << shift_complement);
            previous_n = n;
            let (diff, carry) = shifted_n.overflowing_sub(upper_half);
            let q = diff.wrapping_mul(d_inv);
            ns[i - 1] = q;
            upper_half = (DoubleLimb::from(q) * shifted_d).upper_half();
            if carry {
                upper_half += 1;
            }
        }
        ns[len - 1] = (previous_n >> shift)
            .wrapping_sub(upper_half)
            .wrapping_mul(d_inv);
    } else {
        let d_inv = limbs_modular_invert_limb(d);
        let d = DoubleLimb::from(d);
        let (ns_head, ns_tail) = ns.split_first_mut().unwrap();
        let mut q = ns_head.wrapping_mul(d_inv);
        *ns_head = q;
        let mut previous_carry = false;
        for n in &mut *ns_tail {
            let mut upper_half = (DoubleLimb::from(q) * d).upper_half();
            if previous_carry {
                upper_half += 1;
            }
            let diff;
            (diff, previous_carry) = n.overflowing_sub(upper_half);
            q = diff.wrapping_mul(d_inv);
            *n = q;
        }
    }
}}

#[cfg(feature = "test_build")]
pub(crate) const MAX_OVER_3: Limb = Limb::MAX / 3;

#[cfg(not(feature = "test_build"))]
const MAX_OVER_3: Limb = Limb::MAX / 3;

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
// quotient limbs of the `Natural` divided by 3. The limb slice must be nonempty. The `Natural` must
// be exactly divisible by 3. If it isn't, the behavior of this function is undefined.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// # Panics
// Panics if `ns` is empty.
//
// This is equivalent to `mpn_divexact_by3c` from `mpn/generic/diveby3.c`, GMP 6.2.1, with
// `DIVEXACT_BY3_METHOD == 0` and no carry-in, where the result is returned.
pub_test! {limbs_div_exact_3(ns: &[Limb]) -> Vec<Limb> {
    let mut q = vec![0; ns.len()];
    limbs_div_exact_3_to_out(&mut q, ns);
    q
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
// limbs of the quotient of the `Natural` and 3 to an output slice. The output slice must be at
// least as long as the input slice. The input limb slice must be nonempty. The `Natural` must be
// exactly divisible by 3. If it isn't, the behavior of this function is undefined.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// # Panics
// Panics if `out` is shorter than `ns` or if `ns` is empty.
//
// This is equivalent to `mpn_divexact_by3c` from `mpn/generic/diveby3.c`, GMP 6.2.1, with
// `DIVEXACT_BY3_METHOD == 0`, no carry-in, and no return value.
pub_test! {limbs_div_exact_3_to_out(out: &mut [Limb], ns: &[Limb]) {
    let (out_last, out_init) = out[..ns.len()].split_last_mut().unwrap();
    let (ns_last, ns_init) = ns.split_last().unwrap();
    let q = limbs_div_divisor_of_limb_max_with_carry_to_out(out_init, ns_init, MAX_OVER_3, 0);
    let lower = (DoubleLimb::from(*ns_last) * DoubleLimb::from(MAX_OVER_3)).lower_half();
    *out_last = q.wrapping_sub(lower);
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
// limbs of the quotient of the `Natural` and 3 to the input slice. The input limb slice must be
// nonempty. The `Natural` must be exactly divisible by 3. If it isn't, the behavior of this
// function is undefined.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// # Panics
// Panics if `ns` is empty.
//
// This is equivalent to `mpn_divexact_by3c` from `mpn/generic/diveby3.c`, GMP 6.2.1, with
// `DIVEXACT_BY3_METHOD == 0`, no carry-in, and no return value, where `rp == up`.
pub_crate_test! {limbs_div_exact_3_in_place(ns: &mut [Limb]) {
    let (ns_last, ns_init) = ns.split_last_mut().unwrap();
    let q = limbs_div_divisor_of_limb_max_with_carry_in_place(ns_init, MAX_OVER_3, 0);
    let lower = (DoubleLimb::from(*ns_last) * DoubleLimb::from(MAX_OVER_3)).lower_half();
    *ns_last = q.wrapping_sub(lower);
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
// limbs of the quotient of the `Natural` and a `Limb` to an output slice. The output slice must be
// at least as long as the input slice. The divisor limb cannot be zero and the input limb slice
// must be nonempty. The `Natural` must be exactly divisible by the `Limb`. If it isn't, the
// behavior of this function is undefined.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// # Panics
// Panics if `out` is shorter than `ns`, `ns` is empty, or if `d` is zero.
//
// This is equivalent to `mpn_divexact_1` from `mpn/generic/dive_1.c`, GMP 6.2.1.
pub_test! {limbs_div_exact_limb_to_out(out: &mut [Limb], ns: &[Limb], d: Limb) {
    if d == 3 {
        limbs_div_exact_3_to_out(out, ns);
    } else {
        limbs_div_exact_limb_to_out_no_special_3(out, ns, d);
    }
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
// quotient limbs of the `Natural` divided by a `Limb`. The divisor limb cannot be zero and the limb
// slice must be nonempty. The `Natural` must be exactly divisible by the `Limb`. If it isn't, the
// behavior of this function is undefined.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// # Panics
// Panics if `ns` is empty or if `d` is zero.
//
// This is equivalent to `mpn_divexact_1` from `mpn/generic/dive_1.c`, GMP 6.2.1, where the result
// is returned.
pub_test! {limbs_div_exact_limb(ns: &[Limb], d: Limb) -> Vec<Limb> {
    if d == 3 {
        limbs_div_exact_3(ns)
    } else {
        limbs_div_exact_limb_no_special_3(ns, d)
    }
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
// limbs of the quotient of the `Natural` and a `Limb` to the input slice. The divisor limb cannot
// be zero and the input limb slice must be nonempty. The `Natural` must be exactly divisible by the
// `Limb`. If it isn't, the behavior of this function is undefined.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// # Panics
// Panics if `ns` is empty or if `d` is zero.
//
// This is equivalent to `mpn_divexact_1` from `mpn/generic/dive_1.c`, GMP 6.2.1, where `dest ==
// src`.
pub_crate_test! {limbs_div_exact_limb_in_place(ns: &mut [Limb], d: Limb) {
    if d == 3 {
        limbs_div_exact_3_in_place(ns);
    } else {
        limbs_div_exact_limb_in_place_no_special_3(ns, d);
    }
}}

// # Worst-case complexity
// Constant time and additional memory.
//
// The result is $O(n)$.
//
// This is equivalent to `mpn_binvert_itch` from `mpn/generic/binvert.c`, GMP 6.2.1.
pub_crate_test! {limbs_modular_invert_scratch_len(n: usize) -> usize {
    let itch_local = limbs_mul_mod_base_pow_n_minus_1_next_size(n);
    let itch_out = limbs_mul_mod_base_pow_n_minus_1_scratch_len(
        itch_local,
        n,
        n.shr_round(1, Ceiling).0,
    );
    itch_local + itch_out
}}

// # Worst-case complexity
// $T(n) = O(n (\log n)^2 \log \log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ds.len()`.
pub_test! {limbs_modular_invert_small(
    size: usize,
    is: &mut [Limb],
    scratch: &mut [Limb],
    ds: &[Limb],
    d_inv: Limb,
) {
    if size < DC_BDIV_Q_THRESHOLD {
        limbs_modular_div_schoolbook(is, scratch, ds, d_inv);
        limbs_neg_in_place(is);
    } else {
        limbs_modular_div_divide_and_conquer(is, scratch, ds, d_inv);
    }
}}

// Finds the inverse of a slice `Limb` mod `2 ^ (ds.len() * Limb::WIDTH)`; given x, returns y such
// that x * y ≡ 1 mod `2 ^ (ds.len() * Limb::WIDTH)`. This inverse only exists for odd x, so the
// least-significant limb of `ds` must be odd.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log \log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ds.len()`.
//
// # Panics
// Panics if `is` is shorter than `ds`, if `ds` is empty, or if `scratch` is too short.
//
// This is equivalent to `mpn_binvert` from `mpn/generic/binvert.c`, GMP 6.2.1.
pub_crate_test! {limbs_modular_invert(is: &mut [Limb], ds: &[Limb], scratch: &mut [Limb]) {
    let d_len = ds.len();
    // Compute the computation precisions from highest to lowest, leaving the basecase size in
    // `size`.
    let mut size = d_len;
    let mut sizes = Vec::new();
    while size >= BINV_NEWTON_THRESHOLD {
        sizes.push(size);
        size.shr_round_assign(1, Ceiling);
    }
    // Compute a base value of `size` limbs.
    let scratch_lo = &mut scratch[..size];
    let ds_lo = &ds[..size];
    slice_set_zero(scratch_lo);
    scratch_lo[0] = 1;
    let d_inv = limbs_modular_invert_limb(ds[0]).wrapping_neg();
    limbs_modular_invert_small(size, is, scratch_lo, ds_lo, d_inv);
    let mut previous_size = size;
    // Use Newton iterations to get the desired precision.
    for &size in sizes.iter().rev() {
        let mul_size = limbs_mul_mod_base_pow_n_minus_1_next_size(size);
        let (scratch_lo, scratch_hi) = scratch.split_at_mut(mul_size);
        let (is_lo, is_hi) = is.split_at_mut(previous_size);
        limbs_mul_mod_base_pow_n_minus_1(scratch_lo, mul_size, &ds[..size], is_lo, scratch_hi);
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
}}

// Computes a binary quotient of size `q_len` = `ns.len()` - `ds.len()`. D must be odd. `d_inv` is
// (-D) ^ -1 mod 2 ^ `Limb::WIDTH`, or `limbs_modular_invert_limb(ds[0]).wrapping_neg()`.
//
// Output:
// ```
//    Q = N / D mod 2 ^ (`Limb::WIDTH` * `q_len`)
//    R = (N - Q * D) / 2 ^ (`Limb::WIDTH` * `q_len`)
// ```
//
// Stores the `ds.len()` least-significant limbs of R at `&np[q_len..]` and returns the borrow from
// the subtraction N - Q * D.
//
// # Worst-case complexity
// $T(n) = O(n^2)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// This is equivalent to `mpn_sbpi1_bdiv_qr` from `mpn/generic/sbpi1_bdiv_qr.c`, GMP 6.2.1.
// Investigate changes from 6.1.2?
pub_crate_test! {limbs_modular_div_mod_schoolbook(
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
}}

// # Worst-case complexity
// $T(n) = O(n (\log n)^2 \log \log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ds.len()`.
fn limbs_modular_div_mod_helper(
    qs: &mut [Limb],
    ns: &mut [Limb],
    len: usize,
    ds_lo: &[Limb],
    d_inv: Limb,
    scratch: &mut [Limb],
) -> bool {
    if len < DC_BDIV_QR_THRESHOLD {
        limbs_modular_div_mod_schoolbook(qs, &mut ns[..len << 1], ds_lo, d_inv)
    } else {
        limbs_modular_div_mod_divide_and_conquer_helper(qs, ns, ds_lo, d_inv, scratch)
    }
}

// # Worst-case complexity
// $T(n) = O(n (\log n)^2 \log \log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ds.len()`.
//
// This is equivalent to `mpn_dcpi1_bdiv_qr_n` from `mpn/generic/dcpi1_bdiv_qr.c`, GMP 6.2.1.
fn limbs_modular_div_mod_divide_and_conquer_helper(
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
    let carry = limbs_modular_div_mod_helper(qs, ns, lo, ds_lo, d_inv, scratch);
    let (qs_lo, qs_hi) = qs.split_at_mut(lo);
    let mut mul_scratch = vec![0; limbs_mul_greater_to_out_scratch_len(ds_hi.len(), qs_lo.len())];
    limbs_mul_greater_to_out(scratch, ds_hi, qs_lo, &mut mul_scratch);
    if carry {
        assert!(!limbs_slice_add_limb_in_place(&mut scratch[lo..], 1));
    }
    let ns = &mut ns[lo..];
    let highest_r = limbs_sub_greater_in_place_left(ns, scratch);
    let (ds_lo, ds_hi) = ds.split_at(hi);
    let carry = limbs_modular_div_mod_helper(qs_hi, ns, hi, ds_lo, d_inv, scratch);
    let mut mul_scratch = vec![0; limbs_mul_greater_to_out_scratch_len(hi, ds_hi.len())];
    limbs_mul_greater_to_out(scratch, &qs_hi[..hi], ds_hi, &mut mul_scratch);
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

// Computes a binary quotient of size `q_len` = `ns.len()` - `ds.len()` and a remainder of size
// `rs.len()`. D must be odd. `d_inv` is (-D) ^ -1 mod 2 ^ `Limb::WIDTH`, or
// `limbs_modular_invert_limb(ds[0]).wrapping_neg()`.
//
// Output:
// ```
//    Q = N / D mod 2 ^ (`Limb::WIDTH` * `q_len`)
//    R = (N - Q * D) / 2 ^ (`Limb::WIDTH` * `q_len`)
// ```
//
// Stores the `ds.len()` least-significant limbs of R at `&np[q_len..]` and returns the borrow from
// the subtraction N - Q * D.
//
// # Worst-case complexity
// $T(n, d) = O(n (\log d)^2 \log \log d)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, $n$ is `ns.len()`, and $d$ is `ds.len()`.
//
// This is equivalent to `mpn_dcpi1_bdiv_qr` from `mpn/generic/dcpi1_bdiv_qr.c`, GMP 6.2.1.
pub_crate_test! {limbs_modular_div_mod_divide_and_conquer(
    qs: &mut [Limb],
    ns: &mut [Limb],
    ds: &[Limb],
    d_inv: Limb,
) -> bool {
    let n_len = ns.len();
    let d_len = ds.len();
    assert!(d_len >= 2); // to adhere to limbs_modular_div_mod_schoolbook's limits
    assert!(n_len > d_len); // to adhere to limbs_modular_div_mod_schoolbook's limits
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
        carry = limbs_modular_div_mod_helper(qs, ns, q_len_mod_d_len, ds_lo, d_inv, &mut scratch);
        if q_len_mod_d_len != d_len {
            let mut mul_scratch =
                vec![0; limbs_mul_to_out_scratch_len(ds_hi.len(), q_len_mod_d_len)];
            limbs_mul_to_out(
                &mut scratch,
                ds_hi,
                &qs[..q_len_mod_d_len],
                &mut mul_scratch,
            );
            if carry {
                assert!(!limbs_slice_add_limb_in_place(
                    &mut scratch[q_len_mod_d_len..],
                    1
                ));
            }
            borrow = limbs_sub_greater_in_place_left(&mut ns[q_len_mod_d_len..], &scratch[..d_len]);
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
            carry = limbs_modular_div_mod_divide_and_conquer_helper(
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
        carry = limbs_modular_div_mod_helper(qs, ns, q_len, ds_lo, d_inv, &mut scratch);
        if q_len != d_len {
            let mut mul_scratch = vec![0; limbs_mul_to_out_scratch_len(ds_hi.len(), qs.len())];
            limbs_mul_to_out(&mut scratch, ds_hi, qs, &mut mul_scratch);
            if carry {
                assert!(!limbs_slice_add_limb_in_place(&mut scratch[q_len..], 1));
            }
            borrow = limbs_sub_greater_in_place_left(&mut ns[q_len..], &scratch[..d_len]);
            carry = false;
        }
    }
    if carry {
        assert!(!borrow);
        borrow = true;
    }
    borrow
}}

// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `mpn_dcpi1_bdiv_qr_n_itch` from `mpn/generic/dcpi1_bdiv_qr.c`, GMP 6.2.1.
pub_const_test! {limbs_modular_div_mod_divide_and_conquer_helper_scratch_len(n: usize) -> usize {
    n
}}

// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `mpn_mu_bdiv_qr_itch` from `mpn/generic/mu_bdiv_qr.c`, GMP 6.2.1.
pub_crate_test! {limbs_modular_div_mod_barrett_scratch_len(n_len: usize, d_len: usize) -> usize {
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
        let t_len = limbs_mul_mod_base_pow_n_minus_1_next_size(d_len);
        (
            t_len,
            limbs_mul_mod_base_pow_n_minus_1_scratch_len(t_len, d_len, i_len),
        )
    };
    let modular_invert_scratch_len = limbs_modular_invert_scratch_len(i_len);
    let scratch_len = mul_len_1 + mul_len_2;
    i_len + max(scratch_len, modular_invert_scratch_len)
}}

// # Worst-case complexity
// $T(n) = O(n \log n \log \log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
fn limbs_modular_div_mod_barrett_unbalanced(
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
    // ```
    // |_______________________| dividend
    // |________| divisor
    // ```
    //
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
            let mut mul_scratch = vec![0; limbs_mul_greater_to_out_scratch_len(ds.len(), qs.len())];
            limbs_mul_greater_to_out(scratch, ds, qs, &mut mul_scratch);
        } else {
            let mul_size = limbs_mul_mod_base_pow_n_minus_1_next_size(d_len);
            let (scratch_lo, scratch_hi) = scratch.split_at_mut(mul_size);
            limbs_mul_mod_base_pow_n_minus_1(scratch_lo, mul_size, ds, qs, scratch_hi);
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
                    "limbs_modular_div_mod_barrett_unbalanced, wrapped_len is None",
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
        carry = limbs_sub_same_length_with_borrow_in_to_out(
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
        let mut mul_scratch = vec![0; limbs_mul_greater_to_out_scratch_len(ds.len(), qs.len())];
        limbs_mul_greater_to_out(scratch, ds, qs, &mut mul_scratch);
    } else {
        let tn = limbs_mul_mod_base_pow_n_minus_1_next_size(d_len);
        let (scratch_lo, scratch_hi) = scratch.split_at_mut(tn);
        limbs_mul_mod_base_pow_n_minus_1(scratch_lo, tn, ds, qs, scratch_hi);
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
    limbs_sub_same_length_with_borrow_in_to_out(
        &mut rs[d_len - q_len_s..],
        &ns[n_len - q_len_s..],
        &scratch_hi[..q_len_s],
        carry,
    )
}

// # Worst-case complexity
// $T(n) = O(n \log n \log \log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
fn limbs_modular_div_mod_barrett_balanced(
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
    // ```
    // |_______________________| dividend
    // |________________| divisor
    // ```
    //
    // Compute a half-sized inverse.
    let i_len = q_len - (q_len >> 1);
    let (is, scratch) = scratch.split_at_mut(i_len);
    let (qs_lo, qs_hi) = qs.split_at_mut(i_len);
    limbs_modular_invert(is, &ds[..i_len], scratch);
    limbs_mul_low_same_length(qs_lo, &ns[..i_len], is); // low i_len quotient limbs
    if i_len < MUL_TO_MULMOD_BNM1_FOR_2NXN_THRESHOLD {
        let mut mul_scratch = vec![0; limbs_mul_greater_to_out_scratch_len(ds.len(), qs_lo.len())];
        limbs_mul_greater_to_out(scratch, ds, qs_lo, &mut mul_scratch);
    } else {
        let mul_size = limbs_mul_mod_base_pow_n_minus_1_next_size(d_len);
        let (scratch_lo, scratch_hi) = scratch.split_at_mut(mul_size);
        limbs_mul_mod_base_pow_n_minus_1(scratch_lo, mul_size, ds, qs_lo, scratch_hi);
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
        let mut mul_scratch = vec![0; limbs_mul_greater_to_out_scratch_len(ds.len(), qs_hi.len())];
        limbs_mul_greater_to_out(scratch, ds, qs_hi, &mut mul_scratch);
    } else {
        let mul_size = limbs_mul_mod_base_pow_n_minus_1_next_size(d_len);
        let (scratch_lo, scratch_hi) = scratch.split_at_mut(mul_size);
        limbs_mul_mod_base_pow_n_minus_1(scratch_lo, mul_size, ds, qs_hi, scratch_hi);
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
    limbs_sub_same_length_with_borrow_in_to_out(
        &mut rs[d_len - q_len_s..],
        ns_hi,
        &scratch_hi[..q_len_s],
        carry,
    )
}

// Computes a binary quotient of size `q_len` = `ns.len()` - `ds.len()` and a remainder of size
// `ds.len()`. D must be odd.
//
// Output:
// ```
//    Q = N / D mod 2 ^ (`Limb::WIDTH` * `q_len`)
//    R = (N - Q * D) / 2 ^ (`Limb::WIDTH` * `q_len`)
// ```
//
// # Worst-case complexity
// $T(n) = O(n \log n \log \log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// # Panics
// Panics if `ds` has length smaller than 2, `ns.len()` is less than `ds.len()` + 2, `qs` has length
// less than `ns.len()` - `ds.len()`, `rs` is shorter than `ds`, `scratch` is to short, or the last
// limb of `ds` is even.
//
// This is equivalent to `mpn_mu_bdiv_qr` from `mpn/generic/mu_bdiv_qr.c`, GMP 6.2.1.
pub_crate_test! {limbs_modular_div_mod_barrett(
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
        limbs_modular_div_mod_barrett_unbalanced(qs, rs, ns, ds, scratch)
    } else {
        limbs_modular_div_mod_barrett_balanced(qs, rs, ns, ds, scratch)
    }
}}

// Computes Q = -N/D mod B^un, destroys N.
//
// D must be odd. d_inv is (-D)^-1 mod B.
//
// # Worst-case complexity
// $T(n) = O(n^2)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// This is equivalent to `mpn_sbpi1_bdiv_q` from `mpn/generic/sbpi1_bdiv_q.c`, GMP 6.2.1.
pub_crate_test! {limbs_modular_div_schoolbook(
    mut qs: &mut [Limb],
    mut ns: &mut [Limb],
    ds: &[Limb],
    d_inv: Limb,
) {
    let n_len = ns.len();
    let d_len = ds.len();
    assert_ne!(d_len, 0);
    assert!(n_len >= d_len);
    assert!(ds[0].odd());
    if n_len > d_len {
        let mut carry = 0;
        let limit = n_len - d_len - 1;
        for i in 0..limit {
            let (ns_lo, ns_hi) = ns[i..].split_at_mut(d_len);
            let q = d_inv.wrapping_mul(ns_lo[0]);
            let mut hi = limbs_slice_add_mul_limb_same_length_in_place_left(ns_lo, ds, q);
            assert_eq!(ns_lo[0], 0);
            qs[i] = q;
            let mut carry_b;
            (hi, carry_b) = hi.overflowing_add(carry);
            carry = Limb::from(carry_b);
            (hi, carry_b) = hi.overflowing_add(ns_hi[0]);
            if carry_b {
                carry += 1;
            }
            ns_hi[0] = hi;
        }
        ns = &mut ns[limit..];
        qs = &mut qs[limit..];
        let q = d_inv.wrapping_mul(ns[0]);
        let (ns_lo, ns_hi) = ns.split_at_mut(d_len);
        let hi = carry + limbs_slice_add_mul_limb_same_length_in_place_left(ns_lo, ds, q);
        qs[0] = q;
        ns_hi[0].wrapping_add_assign(hi);
        ns = &mut ns[1..];
        qs = &mut qs[1..];
    }
    let ns = &mut ns[..d_len];
    for i in 0..d_len - 1 {
        let ns_hi = &mut ns[i..];
        let q = d_inv.wrapping_mul(ns_hi[0]);
        limbs_slice_add_mul_limb_same_length_in_place_left(ns_hi, &ds[..d_len - i], q);
        qs[i] = q;
    }
    let last_index = d_len - 1;
    qs[last_index] = d_inv.wrapping_mul(ns[last_index]);
}}

// This is equivalent to `mpn_sbpi1_bdiv_q` from `mpn/generic/sbpi1_bdiv_q.c`, GMP 6.2.1, where qp
// == up.
pub fn limbs_modular_div_schoolbook_in_place(mut ns: &mut [Limb], ds: &[Limb], d_inv: Limb) {
    let n_len = ns.len();
    let d_len = ds.len();
    assert_ne!(d_len, 0);
    assert!(n_len >= d_len);
    assert!(ds[0].odd());
    if n_len > d_len {
        let mut carry = 0;
        let limit = n_len - d_len - 1;
        for i in 0..limit {
            let (ns_lo, ns_hi) = ns[i..].split_at_mut(d_len);
            let q = d_inv.wrapping_mul(ns_lo[0]);
            let mut hi = limbs_slice_add_mul_limb_same_length_in_place_left(ns_lo, ds, q);
            assert_eq!(ns_lo[0], 0);
            ns_lo[0] = q;
            let mut carry_b;
            (hi, carry_b) = hi.overflowing_add(carry);
            carry = Limb::from(carry_b);
            (hi, carry_b) = hi.overflowing_add(ns_hi[0]);
            if carry_b {
                carry += 1;
            }
            ns_hi[0] = hi;
        }
        ns = &mut ns[limit..];
        let q = d_inv.wrapping_mul(ns[0]);
        let (ns_lo, ns_hi) = ns.split_at_mut(d_len);
        let hi = carry + limbs_slice_add_mul_limb_same_length_in_place_left(ns_lo, ds, q);
        ns_lo[0] = q;
        ns_hi[0].wrapping_add_assign(hi);
        ns = &mut ns[1..];
    }
    let ns = &mut ns[..d_len];
    for i in 0..d_len - 1 {
        let ns_hi = &mut ns[i..];
        let q = d_inv.wrapping_mul(ns_hi[0]);
        limbs_slice_add_mul_limb_same_length_in_place_left(ns_hi, &ds[..d_len - i], q);
        ns_hi[0] = q;
    }
    let last_index = d_len - 1;
    ns[last_index].wrapping_mul_assign(d_inv);
}

// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `mpn_dcpi1_bdiv_q_n_itch` from `mpn/generic/dcpi1_bdiv_q.c`, GMP 6.2.1.
pub_const_test! {limbs_modular_div_divide_and_conquer_helper_scratch_len(n: usize) -> usize {
    n
}}

// # Worst-case complexity
// $T(n) = O(n (\log n)^2 \log \log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ds.len()`.
//
// This is equivalent to `mpn_dcpi1_bdiv_q_n` from `mpn/generic/dcpi1_bdiv_q.c`, GMP 6.2.1.
// Investigate changes from 6.1.2?
fn limbs_modular_div_divide_and_conquer_helper(
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
            limbs_modular_div_mod_divide_and_conquer_helper(qs, ns, &ds[..lo], d_inv, scratch);
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
    limbs_modular_div_schoolbook(&mut qs[m..], &mut ns[m..n], &ds[..n_rem], d_inv);
    limbs_neg_in_place(&mut qs[m..]);
}

// Computes Q = N / D mod 2 ^ (`Limb::WIDTH` * `ns.len()`), destroying N. D must be odd. `d_inv` is
// (-D) ^ -1 mod 2 ^ `Limb::WIDTH`, or `limbs_modular_invert_limb(ds[0]).wrapping_neg()`.
//
// # Worst-case complexity
// $T(n, d) = O(n (\log d)^2 \log \log d)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, $n$ is `ns.len()`, and $d$ is `ds.len()`.
//
// This is equivalent to `mpn_dcpi1_bdiv_q` from `mpn/generic/dcpi1_bdiv_q.c`, GMP 6.2.1.
// Investigate changes from 6.1.2?
pub_test! {limbs_modular_div_divide_and_conquer(
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
            limbs_modular_div_mod_schoolbook(qs, &mut ns[..n_len_mod_d_len << 1], ds_lo, d_inv)
        } else {
            limbs_modular_div_mod_divide_and_conquer_helper(qs, ns, ds_lo, d_inv, &mut scratch)
        };
        if n_len_mod_d_len != d_len {
            let mut mul_scratch =
                vec![0; limbs_mul_to_out_scratch_len(ds_hi.len(), n_len_mod_d_len)];
            limbs_mul_to_out(
                &mut scratch,
                ds_hi,
                &qs[..n_len_mod_d_len],
                &mut mul_scratch,
            );
            if carry {
                assert!(!limbs_slice_add_limb_in_place(
                    &mut scratch[n_len_mod_d_len..],
                    1
                ));
            }
            limbs_sub_greater_in_place_left(&mut ns[n_len_mod_d_len..], &scratch[..d_len]);
            carry = false;
        }
        let mut m = n_len_mod_d_len;
        let diff = n_len - d_len;
        while m != diff {
            if carry {
                limbs_sub_limb_in_place(&mut ns[m + d_len..], 1);
            }
            carry = limbs_modular_div_mod_divide_and_conquer_helper(
                &mut qs[m..],
                &mut ns[m..],
                ds,
                d_inv,
                &mut scratch,
            );
            m += d_len;
        }
        limbs_modular_div_divide_and_conquer_helper(
            &mut qs[diff..],
            &mut ns[diff..],
            ds,
            d_inv,
            &mut scratch,
        );
    } else if n_len < DC_BDIV_Q_THRESHOLD {
        limbs_modular_div_schoolbook(qs, ns, ds, d_inv);
        limbs_neg_in_place(qs);
    } else {
        let mut scratch = vec![0; n_len];
        limbs_modular_div_divide_and_conquer_helper(qs, ns, ds, d_inv, &mut scratch);
    }
}}

// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `mpn_mu_bdiv_q_itch` from `mpn/generic/mu_bdiv_q.c`, GMP 6.2.1.
pub_test! {limbs_modular_div_barrett_scratch_len(n_len: usize, d_len: usize) -> usize {
    assert!(DC_BDIV_Q_THRESHOLD < MU_BDIV_Q_THRESHOLD);
    let i_len;
    let mul_len = if n_len > d_len {
        let blocks = (n_len - 1) / d_len + 1; // ceil(q_len / d_len), number of blocks
        i_len = (n_len - 1) / blocks + 1; // ceil(q_len / b) = ceil(q_len / ceil(q_len / d_len))
        let (mul_len_1, mul_len_2) = if i_len < MUL_TO_MULMOD_BNM1_FOR_2NXN_THRESHOLD {
            (d_len + i_len, 0)
        } else {
            let mul_len_1 = limbs_mul_mod_base_pow_n_minus_1_next_size(d_len);
            (
                mul_len_1,
                limbs_mul_mod_base_pow_n_minus_1_scratch_len(mul_len_1, d_len, i_len),
            )
        };
        d_len + mul_len_1 + mul_len_2
    } else {
        i_len = n_len - (n_len >> 1);
        let (mul_len_1, mul_len_2) = if i_len < MUL_TO_MULMOD_BNM1_FOR_2NXN_THRESHOLD {
            (n_len + i_len, 0)
        } else {
            let mul_len_1 = limbs_mul_mod_base_pow_n_minus_1_next_size(n_len);
            (
                mul_len_1,
                limbs_mul_mod_base_pow_n_minus_1_scratch_len(mul_len_1, n_len, i_len),
            )
        };
        mul_len_1 + mul_len_2
    };
    let invert_len = limbs_modular_invert_scratch_len(i_len);
    i_len + max(mul_len, invert_len)
}}

// # Worst-case complexity
// $T(n) = O(n \log n \log \log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
fn limbs_modular_div_barrett_greater(
    qs: &mut [Limb],
    ns: &[Limb],
    ds: &[Limb],
    scratch: &mut [Limb],
) {
    let n_len = ns.len();
    let d_len = ds.len();
    // ```
    // |_______________________| dividend
    // |________| divisor
    // ```
    //
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
            let mut mul_scratch =
                vec![0; limbs_mul_greater_to_out_scratch_len(ds.len(), qs_lo.len())];
            limbs_mul_greater_to_out(scratch, ds, qs_lo, &mut mul_scratch);
        } else {
            let mul_size = limbs_mul_mod_base_pow_n_minus_1_next_size(d_len);
            let (scratch_lo, scratch_hi) = scratch.split_at_mut(mul_size);
            limbs_mul_mod_base_pow_n_minus_1(scratch_lo, mul_size, ds, qs_lo, scratch_hi);
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
                fail_on_untested_path("limbs_modular_div_mod_barrett_greater, wrapped_len is None");
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
        carry = limbs_sub_same_length_with_borrow_in_to_out(
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
        let mut mul_scratch = vec![0; limbs_mul_greater_to_out_scratch_len(ds.len(), qs_lo.len())];
        limbs_mul_greater_to_out(scratch, ds, qs_lo, &mut mul_scratch);
    } else {
        let mul_size = limbs_mul_mod_base_pow_n_minus_1_next_size(d_len);
        let (scratch_lo, scratch_hi) = scratch.split_at_mut(mul_size);
        limbs_mul_mod_base_pow_n_minus_1(scratch_lo, mul_size, ds, qs_lo, scratch_hi);
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
    limbs_sub_same_length_with_borrow_in_to_out(
        &mut rs[d_len - i_len..],
        &ns[diff + d_len..],
        &scratch[d_len..n_len_s],
        carry,
    );
    let limit = n_len_s - i_len;
    limbs_mul_low_same_length(qs_hi, &rs[..limit], &is[..limit]);
}

// # Worst-case complexity
// $T(n) = O(n \log n \log \log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
fn limbs_modular_div_barrett_same_length(
    qs: &mut [Limb],
    ns: &[Limb],
    ds: &[Limb],
    scratch: &mut [Limb],
) {
    let n_len = ns.len();
    // ```
    // |________________| dividend
    // |________________| divisor
    // ```
    //
    // Compute a half-sized inverse.
    let i_len = n_len - (n_len >> 1);
    let (is, scratch) = scratch.split_at_mut(i_len);
    limbs_modular_invert(is, &ds[..i_len], scratch);
    let (ns_lo, ns_hi) = ns.split_at(i_len);
    limbs_mul_low_same_length(qs, ns_lo, is); // low i_len quotient limbs
    let (qs_lo, qs_hi) = qs.split_at_mut(i_len);
    if i_len < MUL_TO_MULMOD_BNM1_FOR_2NXN_THRESHOLD {
        let mut mul_scratch = vec![0; limbs_mul_greater_to_out_scratch_len(ds.len(), qs_lo.len())];
        limbs_mul_greater_to_out(scratch, ds, qs_lo, &mut mul_scratch);
    } else {
        let mul_size = limbs_mul_mod_base_pow_n_minus_1_next_size(n_len);
        let (scratch_lo, scratch_hi) = scratch.split_at_mut(mul_size);
        limbs_mul_mod_base_pow_n_minus_1(scratch_lo, mul_size, ds, qs_lo, scratch_hi);
        if let Some(wrapped_len) = (n_len + i_len).checked_sub(mul_size) {
            let (scratch_lo, scratch_hi) = scratch.split_at_mut(wrapped_len);
            if wrapped_len != 0 && limbs_cmp_same_length(scratch_lo, &ns[..wrapped_len]) == Less {
                assert!(!limbs_sub_limb_in_place(scratch_hi, 1));
            }
        } else {
            fail_on_untested_path("limbs_modular_div_mod_barrett_same_length, wrapped_len is None");
        }
    }
    let (scratch_lo, scratch_hi) = scratch.split_at_mut(i_len);
    let diff = n_len - i_len;
    limbs_sub_same_length_to_out(scratch_lo, ns_hi, &scratch_hi[..diff]);
    // high n_len - i_len quotient limbs
    limbs_mul_low_same_length(qs_hi, &scratch[..diff], &is[..diff]);
}

// Computes Q = N / D mod 2 ^ (`Limb::WIDTH` * `ns.len()`). D must be odd.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log \log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// This is equivalent to `mpn_mu_bdiv_q` from `mpn/generic/mu_bdiv_q.c`, GMP 6.2.1.
pub_test! {limbs_modular_div_barrett(
    qs: &mut [Limb],
    ns: &[Limb],
    ds: &[Limb],
    scratch: &mut [Limb]
) {
    let n_len = ns.len();
    let d_len = ds.len();
    assert!(d_len >= 2);
    assert!(n_len >= d_len);
    if n_len > d_len {
        limbs_modular_div_barrett_greater(qs, ns, ds, scratch);
    } else {
        limbs_modular_div_barrett_same_length(qs, ns, ds, scratch);
    }
}}

// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `mpn_bdiv_q_itch` from `mpn/generic/bdiv_q.c`, GMP 6.2.1, where nothing is
// allocated for inputs that are too small for Barrett division. Investigate changes from 6.1.2?
pub_test! {limbs_modular_div_scratch_len(n_len: usize, d_len: usize) -> usize {
    if d_len < MU_BDIV_Q_THRESHOLD {
        0
    } else {
        limbs_modular_div_barrett_scratch_len(n_len, d_len)
    }
}}

// Computes Q = N / D mod 2 ^ (`Limb::WIDTH` * `ns.len()`), taking N by value. D must be odd.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log \log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// This is equivalent to `mpn_bdiv_q` from `mpn/generic/bdiv_q.c`, GMP 6.2.1. Investigate changes
// from 6.1.2?
pub_test! {limbs_modular_div(qs: &mut [Limb], ns: &mut [Limb], ds: &[Limb], scratch: &mut [Limb]) {
    let d_len = ds.len();
    if d_len < DC_BDIV_Q_THRESHOLD {
        let d_inv = limbs_modular_invert_limb(ds[0]).wrapping_neg();
        limbs_modular_div_schoolbook(qs, ns, ds, d_inv);
        limbs_neg_in_place(qs);
    } else if d_len < MU_BDIV_Q_THRESHOLD {
        let d_inv = limbs_modular_invert_limb(ds[0]).wrapping_neg();
        limbs_modular_div_divide_and_conquer(qs, ns, ds, d_inv);
    } else {
        limbs_modular_div_barrett(qs, ns, ds, scratch);
    }
}}

// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `mpn_bdiv_q_itch` from `mpn/generic/bdiv_q.c`, GMP 6.2.1.
pub_test! {limbs_modular_div_ref_scratch_len(n_len: usize, d_len: usize) -> usize {
    if d_len < MU_BDIV_Q_THRESHOLD {
        n_len
    } else {
        limbs_modular_div_barrett_scratch_len(n_len, d_len)
    }
}}

// Computes Q = N / D mod 2 ^ (`Limb::WIDTH` * `ns.len()`), taking N by reference. D must be odd.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log \log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// This is equivalent to `mpn_bdiv_q` from `mpn/generic/bdiv_q.c`, GMP 6.2.1.
pub_test! {limbs_modular_div_ref(qs: &mut [Limb], ns: &[Limb], ds: &[Limb], scratch: &mut [Limb]) {
    let n_len = ns.len();
    let d_len = ds.len();
    if d_len < DC_BDIV_Q_THRESHOLD {
        let scratch = &mut scratch[..n_len];
        scratch.copy_from_slice(ns);
        let d_inv = limbs_modular_invert_limb(ds[0]).wrapping_neg();
        limbs_modular_div_schoolbook(qs, scratch, ds, d_inv);
        limbs_neg_in_place(qs);
    } else if d_len < MU_BDIV_Q_THRESHOLD {
        let scratch = &mut scratch[..n_len];
        scratch.copy_from_slice(ns);
        let d_inv = limbs_modular_invert_limb(ds[0]).wrapping_neg();
        limbs_modular_div_divide_and_conquer(qs, scratch, ds, d_inv);
    } else {
        limbs_modular_div_barrett(qs, ns, ds, scratch);
    }
}}

// Interpreting two slices of `Limb`s, `ns` and `ds`, as the limbs (in ascending order) of two
// `Natural`s, divides them, returning the quotient. The quotient has `ns.len() - ds.len() + 1`
// limbs.
//
// `ns` must be exactly divisible by `ds`! If it isn't, the function will panic or return a
// meaningless result.
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
// Panics if `ns` is shorter than `ds`, `ds` is empty, or the most-significant limb of `ds` is zero.
//
// This is equivalent to `mpn_divexact` from `mpn/generic/divexact.c`, GMP 6.2.1, where `scratch` is
// allocated internally and `qp` is returned.
pub_test! {limbs_div_exact(ns: &[Limb], ds: &[Limb]) -> Vec<Limb> {
    let mut qs = vec![0; ns.len() - ds.len() + 1];
    limbs_div_exact_to_out_ref_ref(&mut qs, ns, ds);
    qs
}}

// Interpreting two slices of `Limb`s, `ns` and `ds`, as the limbs (in ascending order) of two
// `Natural`s, divides them, writing the `ns.len() - ds.len() + 1` limbs of the quotient to `qs`.
// `ns` and `ds` are taken by value.
//
// `ns` must be exactly divisible by `ds`! If it isn't, the function will panic or return a
// meaningless result.
//
// `ns` must be at least as long as `ds`, `qs` must have length at least `ns.len() - ds.len() + 1`,
// and `ds` must be nonempty and its most significant limb must be greater than zero.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log \log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// # Panics
// Panics if `qs` is too short, `ns` is shorter than `ds`, `ds` is empty, or the most-significant
// limb of `ds` is zero.
//
// This is equivalent to `mpn_divexact` from `mpn/generic/divexact.c`, GMP 6.2.1, except that `np`
// and `dp` are consumed.
pub_crate_test! {limbs_div_exact_to_out(qs: &mut [Limb], ns: &mut [Limb], ds: &mut [Limb]) {
    let n_len = ns.len();
    let d_len = ds.len();
    assert_ne!(d_len, 0);
    assert!(n_len >= d_len);
    assert_ne!(ds[d_len - 1], 0);
    let leading_zeros = slice_leading_zeros(ds);
    let (ns_lo, ns) = ns.split_at_mut(leading_zeros);
    assert!(slice_test_zero(ns_lo), "division not exact");
    let ds = &mut ds[leading_zeros..];
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
    let mut scratch = vec![0; limbs_modular_div_scratch_len(q_len, d_len)];
    limbs_modular_div(qs, &mut ns[..q_len], &ds[..d_len], &mut scratch);
}}

// Interpreting two slices of `Limb`s, `ns` and `ds`, as the limbs (in ascending order) of two
// `Natural`s, divides them, writing the `ns.len() - ds.len() + 1` limbs of the quotient to `qs`.
// `ns` is taken by value and `ds` by reference.
//
// `ns` must be exactly divisible by `ds`! If it isn't, the function will panic or return a
// meaningless result.
//
// `ns` must be at least as long as `ds`, `qs` must have length at least `ns.len() - ds.len() + 1`,
// and `ds` must be nonempty and its most significant limb must be greater than zero.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log \log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// # Panics
// Panics if `qs` is too short, `ns` is shorter than `ds`, `ds` is empty, or the most-significant
// limb of `ds` is zero.
//
// This is equivalent to `mpn_divexact` from `mpn/generic/divexact.c`, GMP 6.2.1, except that `np`
// is consumed.
pub_test! {limbs_div_exact_to_out_val_ref(qs: &mut [Limb], ns: &mut [Limb], ds: &[Limb]) {
    let n_len = ns.len();
    let d_len = ds.len();
    assert_ne!(d_len, 0);
    assert!(n_len >= d_len);
    assert_ne!(ds[d_len - 1], 0);
    let leading_zeros = slice_leading_zeros(ds);
    let (ns_lo, ns) = ns.split_at_mut(leading_zeros);
    assert!(slice_test_zero(ns_lo), "division not exact");
    let mut ds_scratch;
    let mut ds = &ds[leading_zeros..];
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
    let mut scratch = vec![0; limbs_modular_div_scratch_len(q_len, d_len)];
    limbs_modular_div(qs, &mut ns[..q_len], &ds[..d_len], &mut scratch);
}}

// Interpreting two slices of `Limb`s, `ns` and `ds`, as the limbs (in ascending order) of two
// `Natural`s, divides them, writing the `ns.len() - ds.len() + 1` limbs of the quotient to `qs`.
// `ns` is taken by reference and `ds` by value.
//
// `ns` must be exactly divisible by `ds`! If it isn't, the function will panic or return a
// meaningless result.
//
// `ns` must be at least as long as `ds`, `qs` must have length at least `ns.len() - ds.len() + 1`,
// and `ds` must be nonempty and its most significant limb must be greater than zero.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log \log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// # Panics
// Panics if `qs` is too short, `ns` is shorter than `ds`, `ds` is empty, or the most-significant
// limb of `ds` is zero.
//
// This is equivalent to `mpn_divexact` from `mpn/generic/divexact.c`, GMP 6.2.1, except that `dp`
// is consumed.
pub_test! {limbs_div_exact_to_out_ref_val(qs: &mut [Limb], ns: &[Limb], ds: &mut [Limb]) {
    let n_len = ns.len();
    let d_len = ds.len();
    assert_ne!(d_len, 0);
    assert!(n_len >= d_len);
    assert_ne!(ds[d_len - 1], 0);
    let leading_zeros = slice_leading_zeros(ds);
    let (ns_lo, ns_hi) = ns.split_at(leading_zeros);
    assert!(slice_test_zero(ns_lo), "division not exact");
    let mut ns_scratch;
    let mut ns = ns_hi;
    let ds = &mut ds[leading_zeros..];
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
    let mut scratch = vec![0; limbs_modular_div_ref_scratch_len(q_len, d_len)];
    limbs_modular_div_ref(qs, &ns[..q_len], &ds[..d_len], &mut scratch);
}}

// Interpreting two slices of `Limb`s, `ns` and `ds`, as the limbs (in ascending order) of two
// `Natural`s, divides them, writing the `ns.len() - ds.len() + 1` limbs of the quotient to `qs`.
// `ns` and `ds` are taken by reference.
//
// `ns` must be exactly divisible by `ds`! If it isn't, the function will panic or return a
// meaningless result.
//
// `ns` must be at least as long as `ds`, `qs` must have length at least `ns.len() - ds.len() + 1`,
// and `ds` must be nonempty and its most significant limb must be greater than zero.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log \log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// # Panics
// Panics if `qs` is too short, `ns` is shorter than `ds`, `ds` is empty, or the most-significant
// limb of `ds` is zero.
//
// This is equivalent to `mpn_divexact` from `mpn/generic/divexact.c`, GMP 6.2.1.
pub_test! {limbs_div_exact_to_out_ref_ref(qs: &mut [Limb], ns: &[Limb], ds: &[Limb]) {
    let n_len = ns.len();
    let d_len = ds.len();
    assert_ne!(d_len, 0);
    assert!(n_len >= d_len);
    assert_ne!(ds[d_len - 1], 0);
    let leading_zeros = slice_leading_zeros(ds);
    let (ns_lo, ns_hi) = ns.split_at(leading_zeros);
    assert!(slice_test_zero(ns_lo), "division not exact");
    let mut ns_scratch;
    let mut ds_scratch;
    let mut ns = ns_hi;
    let mut ds = &ds[leading_zeros..];
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
    let mut scratch = vec![0; limbs_modular_div_ref_scratch_len(q_len, d_len)];
    limbs_modular_div_ref(qs, &ns[..q_len], &ds[..d_len], &mut scratch);
}}

impl Natural {
    fn div_exact_limb_ref(&self, other: Limb) -> Natural {
        match (self, other) {
            (_, 0) => panic!("division by zero"),
            (x, 1) => x.clone(),
            (Natural(Small(small)), other) => Natural(Small(small / other)),
            (Natural(Large(ref limbs)), other) => {
                Natural::from_owned_limbs_asc(limbs_div_exact_limb(limbs, other))
            }
        }
    }

    fn div_exact_assign_limb(&mut self, other: Limb) {
        match (&mut *self, other) {
            (_, 0) => panic!("division by zero"),
            (_, 1) => {}
            (Natural(Small(ref mut small)), other) => *small /= other,
            (Natural(Large(ref mut limbs)), other) => {
                limbs_div_exact_limb_in_place(limbs, other);
                self.trim();
            }
        }
    }
}

impl DivExact<Natural> for Natural {
    type Output = Natural;

    /// Divides a [`Natural`] by another [`Natural`], taking both by value. The first [`Natural`]
    /// must be exactly divisible by the second. If it isn't, this function may panic or return a
    /// meaningless result.
    ///
    /// $$
    /// f(x, y) = \frac{x}{y}.
    /// $$
    ///
    /// If you are unsure whether the division will be exact, use `self / other` instead. If you're
    /// unsure and you want to know, use `self.div_mod(other)` and check whether the remainder is
    /// zero. If you want a function that panics if the division is not exact, use
    /// `self.div_round(other, Exact)`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero. May panic if `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivExact;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 123 * 456 = 56088
    /// assert_eq!(
    ///     Natural::from(56088u32).div_exact(Natural::from(456u32)),
    ///     123
    /// );
    ///
    /// // 123456789000 * 987654321000 = 121932631112635269000000
    /// assert_eq!(
    ///     Natural::from_str("121932631112635269000000")
    ///         .unwrap()
    ///         .div_exact(Natural::from_str("987654321000").unwrap()),
    ///     123456789000u64
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

    /// Divides a [`Natural`] by another [`Natural`], taking the first by value and the second by
    /// reference. The first [`Natural`] must be exactly divisible by the second. If it isn't, this
    /// function may panic or return a meaningless result.
    ///
    /// $$
    /// f(x, y) = \frac{x}{y}.
    /// $$
    ///
    /// If you are unsure whether the division will be exact, use `self / &other` instead. If you're
    /// unsure and you want to know, use `self.div_mod(&other)` and check whether the remainder is
    /// zero. If you want a function that panics if the division is not exact, use
    /// `self.div_round(&other, Exact)`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero. May panic if `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivExact;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 123 * 456 = 56088
    /// assert_eq!(
    ///     Natural::from(56088u32).div_exact(&Natural::from(456u32)),
    ///     123
    /// );
    ///
    /// // 123456789000 * 987654321000 = 121932631112635269000000
    /// assert_eq!(
    ///     Natural::from_str("121932631112635269000000")
    ///         .unwrap()
    ///         .div_exact(&Natural::from_str("987654321000").unwrap()),
    ///     123456789000u64
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

    /// Divides a [`Natural`] by another [`Natural`], taking the first by reference and the second
    /// by value. The first [`Natural`] must be exactly divisible by the second. If it isn't, this
    /// function may panic or return a meaningless result.
    ///
    /// $$
    /// f(x, y) = \frac{x}{y}.
    /// $$
    ///
    /// If you are unsure whether the division will be exact, use `&self / other` instead. If you're
    /// unsure and you want to know, use `self.div_mod(other)` and check whether the remainder is
    /// zero. If you want a function that panics if the division is not exact, use
    /// `(&self).div_round(other, Exact)`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero. May panic if `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivExact;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 123 * 456 = 56088
    /// assert_eq!(
    ///     (&Natural::from(56088u32)).div_exact(Natural::from(456u32)),
    ///     123
    /// );
    ///
    /// // 123456789000 * 987654321000 = 121932631112635269000000
    /// assert_eq!(
    ///     (&Natural::from_str("121932631112635269000000").unwrap())
    ///         .div_exact(Natural::from_str("987654321000").unwrap()),
    ///     123456789000u64
    /// );
    /// ```
    fn div_exact(self, mut other: Natural) -> Natural {
        if *self == other {
            return Natural::ONE;
        }
        match (self, &mut other) {
            (_, &mut Natural::ZERO) => panic!("division by zero"),
            (n, &mut Natural::ONE) => n.clone(),
            (&Natural::ZERO, _) => Natural::ZERO,
            (n, &mut Natural(Small(d))) => n.div_exact_limb_ref(d),
            (Natural(Small(_)), Natural(Large(_))) => panic!("division not exact"),
            (Natural(Large(ref ns)), &mut Natural(Large(ref mut ds))) => {
                let ns_len = ns.len();
                let ds_len = ds.len();
                if ns_len < ds_len {
                    panic!("division not exact");
                } else {
                    let mut qs = vec![0; ns_len - ds_len + 1];
                    limbs_div_exact_to_out_ref_val(&mut qs, ns, ds);
                    Natural::from_owned_limbs_asc(qs)
                }
            }
        }
    }
}

impl<'a, 'b> DivExact<&'b Natural> for &'a Natural {
    type Output = Natural;

    /// Divides a [`Natural`] by another [`Natural`], taking both by reference. The first
    /// [`Natural`] must be exactly divisible by the second. If it isn't, this function may panic or
    /// return a meaningless result.
    ///
    /// $$
    /// f(x, y) = \frac{x}{y}.
    /// $$
    ///
    /// If you are unsure whether the division will be exact, use `&self / &other` instead. If
    /// you're unsure and you want to know, use `(&self).div_mod(&other)` and check whether the
    /// remainder is zero. If you want a function that panics if the division is not exact, use
    /// `(&self).div_round(&other, Exact)`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero. May panic if `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivExact;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 123 * 456 = 56088
    /// assert_eq!(
    ///     (&Natural::from(56088u32)).div_exact(&Natural::from(456u32)),
    ///     123
    /// );
    ///
    /// // 123456789000 * 987654321000 = 121932631112635269000000
    /// assert_eq!(
    ///     (&Natural::from_str("121932631112635269000000").unwrap())
    ///         .div_exact(&Natural::from_str("987654321000").unwrap()),
    ///     123456789000u64
    /// );
    /// ```
    fn div_exact(self, other: &'b Natural) -> Natural {
        if self == other {
            return Natural::ONE;
        }
        match (self, other) {
            (_, &Natural::ZERO) => panic!("division by zero"),
            (n, &Natural::ONE) => n.clone(),
            (&Natural::ZERO, _) => Natural::ZERO,
            (n, Natural(Small(d))) => n.div_exact_limb_ref(*d),
            (Natural(Small(_)), Natural(Large(_))) => panic!("division not exact"),
            (Natural(Large(ref ns)), Natural(Large(ref ds))) => {
                let ns_len = ns.len();
                let ds_len = ds.len();
                if ns_len < ds_len {
                    panic!("division not exact");
                } else {
                    let mut qs = vec![0; ns_len - ds_len + 1];
                    limbs_div_exact_to_out_ref_ref(&mut qs, ns, ds);
                    Natural::from_owned_limbs_asc(qs)
                }
            }
        }
    }
}

impl DivExactAssign<Natural> for Natural {
    /// Divides a [`Natural`] by another [`Natural`] in place, taking the [`Natural`] on the
    /// right-hand side by value. The first [`Natural`] must be exactly divisible by the second. If
    /// it isn't, this function may panic or return a meaningless result.
    ///
    /// $$
    /// x \gets \frac{x}{y}.
    /// $$
    ///
    /// If you are unsure whether the division will be exact, use `self /= other` instead. If you're
    /// unsure and you want to know, use `self.div_assign_mod(other)` and check whether the
    /// remainder is zero. If you want a function that panics if the division is not exact, use
    /// `self.div_round_assign(other, Exact)`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero. May panic if `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivExactAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 123 * 456 = 56088
    /// let mut x = Natural::from(56088u32);
    /// x.div_exact_assign(Natural::from(456u32));
    /// assert_eq!(x, 123);
    ///
    /// // 123456789000 * 987654321000 = 121932631112635269000000
    /// let mut x = Natural::from_str("121932631112635269000000").unwrap();
    /// x.div_exact_assign(Natural::from_str("987654321000").unwrap());
    /// assert_eq!(x, 123456789000u64);
    /// ```
    fn div_exact_assign(&mut self, mut other: Natural) {
        if *self == other {
            *self = Natural::ONE;
            return;
        }
        match (&mut *self, &mut other) {
            (_, &mut Natural::ZERO) => panic!("division by zero"),
            (_, &mut Natural::ONE) | (&mut Natural::ZERO, _) => {}
            (n, &mut Natural(Small(d))) => n.div_exact_assign_limb(d),
            (Natural(Small(_)), Natural(Large(_))) => panic!("division not exact"),
            (Natural(Large(ref mut ns)), &mut Natural(Large(ref mut ds))) => {
                let ns_len = ns.len();
                let ds_len = ds.len();
                if ns_len < ds_len {
                    panic!("division not exact");
                } else {
                    let mut qs = vec![0; ns_len - ds_len + 1];
                    limbs_div_exact_to_out(&mut qs, ns, ds);
                    swap(&mut qs, ns);
                    self.trim();
                }
            }
        }
    }
}

impl<'a> DivExactAssign<&'a Natural> for Natural {
    /// Divides a [`Natural`] by another [`Natural`] in place, taking the [`Natural`] on the
    /// right-hand side by reference. The first [`Natural`] must be exactly divisible by the second.
    /// If it isn't, this function may panic or return a meaningless result.
    ///
    /// $$
    /// x \gets \frac{x}{y}.
    /// $$
    ///
    /// If you are unsure whether the division will be exact, use `self /= &other` instead. If
    /// you're unsure and you want to know, use `self.div_assign_mod(&other)` and check whether the
    /// remainder is zero. If you want a function that panics if the division is not exact, use
    /// `self.div_round_assign(&other, Exact)`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero. May panic if `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivExactAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 123 * 456 = 56088
    /// let mut x = Natural::from(56088u32);
    /// x.div_exact_assign(&Natural::from(456u32));
    /// assert_eq!(x, 123);
    ///
    /// // 123456789000 * 987654321000 = 121932631112635269000000
    /// let mut x = Natural::from_str("121932631112635269000000").unwrap();
    /// x.div_exact_assign(&Natural::from_str("987654321000").unwrap());
    /// assert_eq!(x, 123456789000u64);
    /// ```
    fn div_exact_assign(&mut self, other: &'a Natural) {
        if self == other {
            *self = Natural::ONE;
            return;
        }
        match (&mut *self, other) {
            (_, &Natural::ZERO) => panic!("division by zero"),
            (_, &Natural::ONE) | (&mut Natural::ZERO, _) => {}
            (_, Natural(Small(d))) => self.div_exact_assign_limb(*d),
            (Natural(Small(_)), Natural(Large(_))) => panic!("division not exact"),
            (Natural(Large(ref mut ns)), Natural(Large(ref ds))) => {
                let ns_len = ns.len();
                let ds_len = ds.len();
                if ns_len < ds_len {
                    panic!("division not exact");
                } else {
                    let mut qs = vec![0; ns_len - ds_len + 1];
                    limbs_div_exact_to_out_val_ref(&mut qs, ns, ds);
                    swap(&mut qs, ns);
                    self.trim();
                }
            }
        }
    }
}
