// Copyright © 2025 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      `TOOM22_MUL_REC`, `TOOM22_MUL_N_REC`, and `mpn_toom22_mul` contributed to the GNU project by
//      Torbjörn Granlund.
//
//      `mpn_toom32_mul` contributed to the GNU project by Torbjörn Granlund, with improvements by
//      Marco Bodrato and Niels Möller.
//
//      `TOOM33_MUL_N_REC`, `mpn_toom33_mul`, and `mpn_toom42_mul` contributed to the GNU project by
//      Torbjörn Granlund, with additional improvements by Marco Bodrato.
//
//      `mpn_toom43_mul`, `mpn_toom52_mul`, `mpn_toom54_mul`, `abs_sub_n`, `abs_sub_add_n`,
//      `mpn_toom63_mul`, `TOOM6H_MUL_N_REC`, `mpn_toom6h_mul`, `TOOM8H_MUL_N_REC`,
//      `mpn_toom8h_mul`, and `mpn_toom_couple_handling` contributed to the GNU project by Marco
//      Bodrato.
//
//      `mpn_toom44_mul`, `mpn_toom53_mul`, and `mpn_toom62_mul` contributed to the GNU project by
//      Torbjörn Granlund and Marco Bodrato.
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
    limbs_add_same_length_with_carry_in_in_place_left, limbs_add_to_out, limbs_add_to_out_aliased,
    limbs_slice_add_greater_in_place_left, limbs_slice_add_limb_in_place,
    limbs_slice_add_same_length_in_place_left,
};
use crate::natural::arithmetic::add_mul::limbs_slice_add_mul_limb_same_length_in_place_left;
use crate::natural::arithmetic::mul::poly_eval::{
    limbs_mul_toom_evaluate_deg_3_poly_in_1_and_neg_1,
    limbs_mul_toom_evaluate_deg_3_poly_in_2_and_neg_2, limbs_mul_toom_evaluate_poly_in_1_and_neg_1,
    limbs_mul_toom_evaluate_poly_in_2_and_neg_2,
    limbs_mul_toom_evaluate_poly_in_2_pow_and_neg_2_pow,
    limbs_mul_toom_evaluate_poly_in_2_pow_neg_and_neg_2_pow_neg,
};
use crate::natural::arithmetic::mul::poly_interpolate::{
    limbs_mul_toom_interpolate_12_points, limbs_mul_toom_interpolate_16_points,
    limbs_mul_toom_interpolate_5_points, limbs_mul_toom_interpolate_6_points,
    limbs_mul_toom_interpolate_7_points, limbs_mul_toom_interpolate_8_points,
};
use crate::natural::arithmetic::mul::{
    limbs_mul_greater_to_out, limbs_mul_greater_to_out_basecase,
    limbs_mul_greater_to_out_scratch_len, limbs_mul_same_length_to_out,
    limbs_mul_same_length_to_out_scratch_len, limbs_mul_to_out, limbs_mul_to_out_scratch_len,
};
use crate::natural::arithmetic::shl::{limbs_shl_to_out, limbs_slice_shl_in_place};
use crate::natural::arithmetic::shr::limbs_slice_shr_in_place;
use crate::natural::arithmetic::square::{
    limbs_square_to_out_toom_6_scratch_len, limbs_square_to_out_toom_8_scratch_len,
};
use crate::natural::arithmetic::sub::{
    limbs_sub_greater_in_place_left, limbs_sub_greater_to_out, limbs_sub_limb_in_place,
    limbs_sub_same_length_in_place_left, limbs_sub_same_length_in_place_right,
    limbs_sub_same_length_to_out, limbs_sub_same_length_with_borrow_in_in_place_left,
    limbs_sub_same_length_with_borrow_in_to_out,
};
use crate::natural::comparison::cmp::limbs_cmp_same_length;
use crate::platform::{
    Limb, MUL_FFT_THRESHOLD, MUL_TOOM22_THRESHOLD, MUL_TOOM33_THRESHOLD, MUL_TOOM44_THRESHOLD,
    MUL_TOOM6H_THRESHOLD, MUL_TOOM8H_THRESHOLD,
};
use core::cmp::{max, Ordering::*};
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{
    ArithmeticCheckedShl, DivRound, EqModPowerOf2, ShrRound, WrappingAddAssign, WrappingSubAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::logic::traits::NotAssign;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::slices::{slice_set_zero, slice_test_zero};

// TODO tune
pub(crate) const MUL_TOOM33_THRESHOLD_LIMIT: usize = MUL_TOOM33_THRESHOLD;

// Helper function for high degree Toom-Cook algorithms.
//
// Gets {`xs`, `n`} and (`y_sign` ? -1 : 1) * {`ys`, `n`}. Computes at once:
// ```
// {`xs`, `n`} <- ({`xs`, `n`} + {`ys`, `n`}) / 2 ^ {`x_shift` + 1}
// {`ys`, `n`} <- ({`xs`, `n`} - {`ys`, `n`}) / 2 ^ {`y_shift` + 1}
// ```
//
// Finally recompose them obtaining:
// ```
// {`xs`, `n` + `offset`} <- {`xs`, `n`} + {`ys`, `n`} * 2 ^ {`offset` * `Limb::WIDTH`}
// ```
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), ys.len())`.
//
// This is equivalent to `mpn_toom_couple_handling` from `mpn/generic/toom_couple_handling.c`, GMP
// 6.2.1. The argument `n` is excluded as it is just the length of `ys`.
pub(crate) fn limbs_toom_couple_handling(
    xs: &mut [Limb],
    ys: &mut [Limb],
    y_sign: bool,
    offset: usize,
    x_shift: u64,
    y_shift: u64,
) {
    let n = ys.len();
    assert!(xs.len() >= n + offset);
    let (xs_lo, xs_hi) = xs.split_at_mut(n);
    if y_sign {
        limbs_sub_same_length_in_place_right(xs_lo, ys);
    } else {
        limbs_slice_add_same_length_in_place_left(ys, xs_lo);
    }
    limbs_slice_shr_in_place(ys, 1);
    limbs_sub_same_length_in_place_left(xs_lo, ys);
    if x_shift != 0 {
        limbs_slice_shr_in_place(xs_lo, x_shift);
    }
    if y_shift != 0 {
        limbs_slice_shr_in_place(ys, y_shift);
    }
    let (ys_lo, ys_hi) = ys.split_at(n - offset);
    if limbs_slice_add_same_length_in_place_left(&mut xs_lo[offset..], ys_lo) {
        assert!(!limbs_add_limb_to_out(xs_hi, ys_hi, 1));
    } else {
        xs_hi[..offset].copy_from_slice(ys_hi);
    }
}

// TODO make these compiler flags?

// TODO tune
#[cfg(feature = "test_build")]
pub const TUNE_PROGRAM_BUILD: bool = false;
#[cfg(not(feature = "test_build"))]
pub(crate) const TUNE_PROGRAM_BUILD: bool = false;
// TODO tune
#[cfg(feature = "test_build")]
pub const WANT_FAT_BINARY: bool = true;
#[cfg(not(feature = "test_build"))]
pub(crate) const WANT_FAT_BINARY: bool = true;

// TODO tune
#[cfg(feature = "test_build")]
pub const TOOM22_MAYBE_MUL_TOOM22: bool =
    TUNE_PROGRAM_BUILD || WANT_FAT_BINARY || MUL_TOOM33_THRESHOLD >= 2 * MUL_TOOM22_THRESHOLD;
#[cfg(not(feature = "test_build"))]
const TOOM22_MAYBE_MUL_TOOM22: bool =
    TUNE_PROGRAM_BUILD || WANT_FAT_BINARY || MUL_TOOM33_THRESHOLD >= 2 * MUL_TOOM22_THRESHOLD;

fn limbs_mul_same_length_to_out_toom_22_recursive_scratch_len(xs_len: usize) -> usize {
    if !TOOM22_MAYBE_MUL_TOOM22 || xs_len < MUL_TOOM22_THRESHOLD {
        0
    } else {
        limbs_mul_greater_to_out_toom_22_scratch_len(xs_len, xs_len)
    }
}

// A helper function for `limbs_mul_greater_to_out_toom_22`.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `TOOM22_MUL_N_REC` from `mpn/generic/toom22_mul.c`, GMP 6.2.1.
fn limbs_mul_same_length_to_out_toom_22_recursive(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    assert_eq!(xs.len(), ys.len());
    if !TOOM22_MAYBE_MUL_TOOM22 || xs.len() < MUL_TOOM22_THRESHOLD {
        limbs_mul_greater_to_out_basecase(out, xs, ys);
    } else {
        limbs_mul_greater_to_out_toom_22(out, xs, ys, scratch);
    }
}

fn limbs_mul_greater_to_out_toom_22_recursive_scratch_len(xs_len: usize, ys_len: usize) -> usize {
    if !TOOM22_MAYBE_MUL_TOOM22 || ys_len < MUL_TOOM22_THRESHOLD {
        0
    } else if xs_len << 2 < 5 * ys_len {
        limbs_mul_greater_to_out_toom_22_scratch_len(xs_len, ys_len)
    } else if limbs_mul_greater_to_out_toom_32_input_sizes_valid(xs_len, ys_len) {
        limbs_mul_greater_to_out_toom_32_scratch_len(xs_len, ys_len)
    } else {
        limbs_mul_greater_to_out_scratch_len(xs_len, ys_len)
    }
}

// A helper function for `limbs_mul_greater_to_out_toom_22`.
//
// Normally, this calls `limbs_mul_greater_to_out_basecase` or `limbs_mul_greater_to_out_toom_22`.
// But when the fraction MUL_TOOM33_THRESHOLD / MUL_TOOM22_THRESHOLD is large, an initially small
// relative unbalance will become a larger and larger relative unbalance with each recursion (the
// difference s - t will be invariant over recursive calls). Therefore, we need to call
// `limbs_mul_greater_to_out_toom_32`.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `TOOM22_MUL_REC` from `mpn/generic/toom22_mul.c`, GMP 6.2.1.
fn limbs_mul_greater_to_out_toom_22_recursive(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    if !TOOM22_MAYBE_MUL_TOOM22 || ys_len < MUL_TOOM22_THRESHOLD {
        limbs_mul_greater_to_out_basecase(out, xs, ys);
    } else if xs_len << 2 < 5 * ys_len {
        limbs_mul_greater_to_out_toom_22(out, xs, ys, scratch);
    } else if limbs_mul_greater_to_out_toom_32_input_sizes_valid(xs_len, ys_len) {
        limbs_mul_greater_to_out_toom_32(out, xs, ys, scratch);
    } else {
        limbs_mul_greater_to_out(out, xs, ys, scratch);
    }
}

// This function can be used to determine whether the sizes of the input slices to
// `limbs_mul_greater_to_out_toom_22` are valid.
//
// # Worst-case complexity
// Constant time and additional memory.
pub_const_test! {limbs_mul_greater_to_out_toom_22_input_sizes_valid(
    xs_len: usize,
    ys_len: usize,
) -> bool {
    xs_len >= ys_len && xs_len + 1 < ys_len << 1
}}

// This function can be used to determine the length of the input `scratch` slice in
// `limbs_mul_greater_to_out_toom_22`.
//
// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `mpn_toom22_mul_itch` from `gmp-impl.h`, GMP 6.2.1.
pub_crate_test! {limbs_mul_greater_to_out_toom_22_scratch_len(
    xs_len: usize,
    ys_len: usize
) -> usize {
    let s = xs_len >> 1;
    let n = xs_len - s;
    let t = ys_len - n;
    let len = n << 1;
    let mut mul_len = limbs_mul_same_length_to_out_toom_22_recursive_scratch_len(n);
    let n = if s > t {
        limbs_mul_greater_to_out_toom_22_recursive_scratch_len(s, t)
    } else {
        limbs_mul_same_length_to_out_toom_22_recursive_scratch_len(s)
    };
    if n > mul_len {
        mul_len = n;
    }
    len + mul_len
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
// the `xs.len() + ys.len()` least-significant limbs of the product of the `Natural`s to an output
// slice. A scratch slice is provided for the algorithm to use. An upper bound for the number of
// scratch limbs needed is provided by `limbs_mul_greater_to_out_toom_22_scratch_len`. The following
// restrictions on the input slices must be met:
// - `out`.len() >= `xs`.len() + `ys`.len()
// - `xs`.len() >= `ys`.len()
// - `xs`.len() + 1 < 2 * `ys`.len()
//
// Approximately, `ys`.len() < `xs`.len() < 2 * `ys`.len().
//
// The smallest allowable `xs` length is 2. The smallest allowable `ys` length is also 2.
//
// This uses the Toom-22, aka Toom-2, aka Karatsuba algorithm.
//
// Evaluate in: -1, 0, Infinity.
// ```
// <--s--><--n--->
//  ______________
// |_xs1_|__xs0__|
//  |ys1_|__ys0__|
//  <-t--><--n--->
//
// v_0     = xs0         * ys0         # X(0)   * Y(0)
// v_neg_1 = (xs0 - xs1) * (ys0 - ys1) # X(-1)  * Y(-1)
// v_inf   = xs1         * ys1         # X(inf) * Y(inf)
// ```
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// The time would be O(n^{\log_2 3}) \approx O(n^{1.585}), but
// `limbs_mul_same_length_to_out_toom_22_recursive` calls `limbs_mul_greater_to_out`.
//
// # Panics
// May panic if the input slice conditions are not met.
//
// This is equivalent to `mpn_toom22_mul` from `mpn/generic/toom22_mul.c`, GMP 6.2.1.
pub_crate_test! {limbs_mul_greater_to_out_toom_22(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let xs_len = xs.len();
    assert!(xs_len > 1);
    let ys_len = ys.len();
    assert_ne!(ys_len, 0);
    assert!(xs_len >= ys_len);
    let out = &mut out[..xs_len + ys_len];
    let s = xs_len >> 1;
    let n = xs_len - s;
    assert!(ys_len >= n);
    let t = ys_len - n;
    let (xs_0, xs_1) = xs.split_at(n); // xs_0: length n, xs_1: length s
    let (ys_0, ys_1) = ys.split_at(n); // ys_0: length n, ys_1: length t
    let mut v_neg_1_neg = false;
    split_into_chunks_mut!(out, n, [asm1, bsm1], _unused);
    // Compute bsm1.
    if s == n {
        if limbs_cmp_same_length(xs_0, xs_1) == Less {
            limbs_sub_same_length_to_out(asm1, xs_1, xs_0);
            v_neg_1_neg = true;
        } else {
            limbs_sub_same_length_to_out(asm1, xs_0, xs_1);
        }
    } else {
        // n - s == 1
        let (xs_0_last, xs_0_init) = xs_0.split_last().unwrap();
        let (asm1_last, asm1_init) = asm1.split_last_mut().unwrap();
        if *xs_0_last == 0 && limbs_cmp_same_length(xs_0_init, xs_1) == Less {
            limbs_sub_same_length_to_out(asm1_init, xs_1, xs_0_init);
            *asm1_last = 0;
            v_neg_1_neg = true;
        } else {
            *asm1_last = *xs_0_last;
            if limbs_sub_same_length_to_out(asm1_init, xs_0_init, xs_1) {
                asm1_last.wrapping_sub_assign(1);
            }
        }
    }
    // Compute bsm1.
    if t == n {
        if limbs_cmp_same_length(ys_0, ys_1) == Less {
            limbs_sub_same_length_to_out(bsm1, ys_1, ys_0);
            v_neg_1_neg.not_assign();
        } else {
            limbs_sub_same_length_to_out(bsm1, ys_0, ys_1);
        }
    } else {
        let (ys_0_lo, ys_0_hi) = ys_0.split_at(t);
        if slice_test_zero(ys_0_hi) && limbs_cmp_same_length(ys_0_lo, ys_1) == Less {
            limbs_sub_same_length_to_out(bsm1, ys_1, ys_0_lo);
            slice_set_zero(&mut bsm1[t..]);
            v_neg_1_neg.not_assign();
        } else {
            limbs_sub_greater_to_out(bsm1, ys_0, ys_1);
        }
    }
    let (v_neg_1, scratch_out) = scratch.split_at_mut(n << 1);
    // size: n
    limbs_mul_same_length_to_out_toom_22_recursive(v_neg_1, asm1, bsm1, scratch_out);
    let (v_0, v_pos_inf) = out.split_at_mut(n << 1); // v_0: length 2 * n
    if s > t {
        // size: s, t
        limbs_mul_greater_to_out_toom_22_recursive(v_pos_inf, xs_1, ys_1, scratch_out);
    } else {
        // size: s
        limbs_mul_same_length_to_out_toom_22_recursive(v_pos_inf, xs_1, &ys_1[..s], scratch_out);
    }
    // size: n
    limbs_mul_same_length_to_out_toom_22_recursive(v_0, xs_0, ys_0, scratch_out);
    // L(v_pos_inf) + H(v_pos_inf)
    let (v_pos_inf_lo, v_pos_inf_hi) = v_pos_inf.split_at_mut(n); // v_pos_inf_lo: length n
    let (v_0_lo, v_0_hi) = v_0.split_at_mut(n); // v_0_lo: length n, vo_hi: length n
                                                // H(v_0) + L(v_pos_inf)
    let mut carry = Limb::from(limbs_slice_add_same_length_in_place_left(
        v_pos_inf_lo,
        v_0_hi,
    ));
    // L(v_0) + H(v_0)
    let mut carry2 = carry;
    if limbs_add_same_length_to_out(v_0_hi, v_pos_inf_lo, v_0_lo) {
        carry2 += 1;
    }
    // - s + t - n == either ys_len - (xs_len >> 1) or ys_len - (xs_len >> 1) - 2.
    // - n == xs_len - (xs_len >> 1) and xs_len >= ys_len.
    // - So n >= s + t - n.
    if limbs_slice_add_greater_in_place_left(v_pos_inf_lo, &v_pos_inf_hi[..s + t - n]) {
        carry += 1;
    }
    let out_lo = &mut out[n..3 * n];
    if v_neg_1_neg {
        if limbs_slice_add_same_length_in_place_left(out_lo, v_neg_1) {
            carry += 1;
        }
    } else if limbs_sub_same_length_in_place_left(out_lo, v_neg_1) {
        carry.wrapping_sub_assign(1);
    }
    assert!(!limbs_slice_add_limb_in_place(&mut out[n << 1..], carry2));
    let out_hi = &mut out[3 * n..];
    if carry <= 2 {
        assert!(!limbs_slice_add_limb_in_place(out_hi, carry));
    } else {
        assert!(!limbs_sub_limb_in_place(out_hi, 1));
    }
}}

// This function can be used to determine the length of the input `scratch` slice in
// `limbs_mul_greater_to_out_toom_32`.
//
// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `mpn_toom32_mul_itch` from `gmp-impl.h`, GMP 6.2.1.
pub_crate_test! {limbs_mul_greater_to_out_toom_32_scratch_len(
    xs_len: usize,
    ys_len: usize
) -> usize {
    let n = 1 + if xs_len << 1 >= 3 * ys_len {
        (xs_len - 1) / 3
    } else {
        (ys_len - 1) >> 1
    };
    let s = xs_len - (n << 1);
    let t = ys_len - n;
    2 * n
        + 1
        + max(
            limbs_mul_same_length_to_out_scratch_len(n),
            limbs_mul_to_out_scratch_len(s, t),
        )
}}

// This function can be used to determine whether the sizes of the input slices to
// `limbs_mul_greater_to_out_toom_32` are valid.
//
// # Worst-case complexity
// Constant time and additional memory.
pub_const_test! {limbs_mul_greater_to_out_toom_32_input_sizes_valid(
    xs_len: usize,
    ys_len: usize,
) -> bool {
    xs_len > ys_len + 1 && (xs_len == 6 || ys_len > 4) && xs_len << 1 < 3 * (ys_len + 1)
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
// the `xs.len() + ys.len()` least-significant limbs of the product of the `Natural`s to an output
// slice. A scratch slice is provided for the algorithm to use. An upper bound for the number of
// scratch limbs needed is provided by `limbs_mul_greater_to_out_toom_32_scratch_len`. The following
// restrictions on the input slices must be met:
// - `out`.len() >= `xs`.len() + `ys`.len()
// - `xs`.len() > `ys`.len() + 1
// - 2 * `xs`.len() < 3 * (`ys`.len() + 1)
// - `xs`.len() == 6 or `ys`.len() > 4
//
// Approximately, `ys`.len() < `xs`.len() < 3 / 2 * `ys`.len().
//
// The smallest allowable `xs` length is 6. The smallest allowable `ys` length is 4.
//
// This uses the Toom-32 aka Toom-2.5 algorithm.
//
// Evaluate in: -1, 0, 1, Infinity.
// ```
// <-s-><--n--><--n-->
//  ___________________
// |xs2_|__xs1_|__xs0_|
//        |ys1_|__ys0_|
//        <-t--><--n-->
//
// v0   =  xs0              * ys0         # X(0)   * Y(0)
// v1   = (xs0 + xs1 + xs2) * (ys0 + ys1) # X(1)   * Y(1)    xh  <= 2  yh <= 1
// vm1  = (xs0 - xs1 + xs2) * (ys0 - ys1) # X(-1)  * Y(-1)  |xh| <= 1  yh = 0
// vinf =               xs2 * ys1         # X(inf) * Y(inf)
// ```
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// The time would be O(n^{\log_3 4}) \approx O(n^{1.262}), but `limbs_mul_same_length_to_out` is
// called.
//
// # Panics
// May panic if the input slice conditions are not met.
//
// This is equivalent to `mpn_toom32_mul` from `mpn/generic/toom32_mul.c`, GMP 6.2.1.
pub_crate_test! {limbs_mul_greater_to_out_toom_32(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);
    let n = 1 + if xs_len << 1 >= 3 * ys_len {
        (xs_len - 1) / 3
    } else {
        (ys_len - 1) >> 1
    };
    // Required, to ensure that s + t >= n.
    assert!(ys_len + 2 <= xs_len && xs_len + 6 <= 3 * ys_len);
    split_into_chunks!(xs, n, [xs_0, xs_1], xs_2);
    let s = xs_2.len();
    assert_ne!(s, 0);
    assert!(s <= n);
    split_into_chunks!(ys, n, [ys_0], ys_1);
    let t = ys_1.len();
    assert_ne!(t, 0);
    assert!(t <= n);
    assert!(s + t >= n);
    // Product area of size xs_len + ys_len = 3 * n + s + t >= 4 * n + 2.
    split_into_chunks_mut!(out, n << 1, [out_lo, out_hi], _unused);
    let (ap1, bp1) = out_lo.split_at_mut(n);
    let (am1, bm1) = out_hi.split_at_mut(n);
    // Compute ap1 = xs0 + xs1 + a3, am1 = xs0 - xs1 + a3
    let mut hi = limbs_add_to_out(ap1, xs_0, xs_2);
    let mut ap1_hi = Limb::from(hi);
    let mut v_neg_1_neg = ap1_hi == 0 && limbs_cmp_same_length(ap1, xs_1) == Less;
    if v_neg_1_neg {
        assert!(!limbs_sub_same_length_to_out(am1, xs_1, ap1));
    } else if limbs_sub_same_length_to_out(am1, ap1, xs_1) {
        hi.not_assign();
    }
    if limbs_slice_add_same_length_in_place_left(ap1, xs_1) {
        ap1_hi += 1;
    }
    let bp1_hi;
    // Compute bp1 = ys0 + ys1 and bm1 = ys0 - ys1.
    if t == n {
        bp1_hi = limbs_add_same_length_to_out(bp1, ys_0, ys_1);
        if limbs_cmp_same_length(ys_0, ys_1) == Less {
            assert!(!limbs_sub_same_length_to_out(bm1, ys_1, ys_0));
            v_neg_1_neg.not_assign();
        } else {
            assert!(!limbs_sub_same_length_to_out(bm1, ys_0, ys_1));
        }
    } else {
        bp1_hi = limbs_add_to_out(bp1, ys_0, ys_1);
        let (ys_0_lo, ys_0_hi) = ys_0.split_at(t);
        if slice_test_zero(ys_0_hi) && limbs_cmp_same_length(ys_0_lo, ys_1) == Less {
            let (bm1_lo, bm1_hi) = bm1.split_at_mut(t);
            assert!(!limbs_sub_same_length_to_out(bm1_lo, ys_1, ys_0_lo));
            slice_set_zero(bm1_hi);
            v_neg_1_neg.not_assign();
        } else {
            assert!(!limbs_sub_greater_to_out(bm1, ys_0, ys_1));
        }
    }
    let m = (n << 1) + 1;
    let (scratch, mul_scratch) = scratch.split_at_mut(m);
    // size: n
    limbs_mul_same_length_to_out(scratch, ap1, bp1, mul_scratch);
    split_into_chunks_mut!(scratch, n, [_unused, scratch_lo], scratch_hi);
    let mut carry = 0;
    match ap1_hi {
        1 => {
            if limbs_slice_add_same_length_in_place_left(scratch_lo, bp1) {
                carry = 1;
            }
            if bp1_hi {
                carry += 1;
            }
        }
        2 => {
            carry = limbs_slice_add_mul_limb_same_length_in_place_left(scratch_lo, bp1, 2);
            if bp1_hi {
                carry += 2;
            }
        }
        _ => {}
    }
    if bp1_hi && limbs_slice_add_same_length_in_place_left(scratch_lo, ap1) {
        carry += 1;
    }
    scratch_hi[0] = carry;
    // size: n
    limbs_mul_same_length_to_out(out_lo, am1, bm1, mul_scratch);
    out_hi[0] =
        Limb::from(hi && limbs_slice_add_same_length_in_place_left(&mut out_lo[n..], bm1));
    // v1 <-- (v1 + vm1) / 2 = x0 + x2
    let out_b = &out[..m];
    if v_neg_1_neg {
        limbs_sub_same_length_in_place_left(scratch, out_b);
    } else {
        limbs_slice_add_same_length_in_place_left(scratch, out_b);
    }
    assert_eq!(limbs_slice_shr_in_place(scratch, 1), 0);
    split_into_chunks_mut!(out, n, [out_0, out_1, out_2], _unused);
    // We get x1 + x3 = (x0 + x2) - (x0 - x1 + x2 - x3), and hence
    // ```
    // y = x1 + x3 + (x0 + x2) * B
    //   = (x0 + x2) * B + (x0 + x2) - vm1.
    // ```
    //
    // y is 3 * n + 1 limbs, y = y0 + y1 B + y2 B^2. We store them as follows: y0 at scratch, y1 at
    // out + 2 * n, and y2 at scratch + n (already in place, except for carry propagation).
    //
    // We thus add
    // ```
    //    B^3  B^2   B    1
    //     |    |    |    |
    //    +-----+----+
    //  + |  x0 + x2 |
    //    +----+-----+----+
    //  +      |  x0 + x2 |
    //         +----------+
    //  -      |  vm1     |
    //  --+----++----+----+-
    //    | y2  | y1 | y0 |
    //    +-----+----+----+
    // ```
    //
    // Since we store y0 at the same location as the low half of x0 + x2, we need to do the middle
    // sum first.
    let mut hi = out_2[0];
    let (scratch_lo, scratch_hi) = scratch.split_at_mut(n);
    let scratch_hi = &mut scratch_hi[..=n];
    let (scratch_hi_last, scratch_hi_init) = scratch_hi.split_last().unwrap();
    let mut x = *scratch_hi_last;
    if limbs_add_same_length_to_out(out_2, scratch_lo, scratch_hi_init) {
        x += 1;
    }
    assert!(!limbs_slice_add_limb_in_place(scratch_hi, x));
    if v_neg_1_neg {
        let carry = limbs_slice_add_same_length_in_place_left(scratch_lo, out_0);
        // out_lo: length 2 * n
        if limbs_add_same_length_with_carry_in_in_place_left(out_2, out_1, carry) {
            hi.wrapping_add_assign(1);
        }
        assert!(!limbs_slice_add_limb_in_place(scratch_hi, hi));
    } else {
        let carry = limbs_sub_same_length_in_place_left(scratch_lo, out_0);
        // out_lo: length 2 * n
        if limbs_sub_same_length_with_borrow_in_in_place_left(out_2, out_1, carry) {
            hi.wrapping_add_assign(1);
        }
        assert!(!limbs_sub_limb_in_place(scratch_hi, hi));
    }
    // size: n
    limbs_mul_same_length_to_out(out, xs_0, ys_0, mul_scratch);
    // size: s, t
    limbs_mul_to_out(&mut out[3 * n..], xs_2, ys_1, mul_scratch);
    // Remaining interpolation.
    //
    // ```
    //    y * B + x0 + x3 B^3 - x0 B^2 - x3 B
    //    = (x1 + x3) B + (x0 + x2) B^2 + x0 + x3 B^3 - x0 B^2 - x3 B
    //    = y0 B + y1 B^2 + y3 B^3 + Lx0 + H x0 B
    //      + L x3 B^3 + H x3 B^4 - Lx0 B^2 - H x0 B^3 - L x3 B - H x3 B^2
    //    = L x0 + (y0 + H x0 - L x3) B + (y1 - L x0 - H x3) B^2
    //      + (y2 - (H x0 - L x3)) B^3 + H x3 B^4
    //
    //     B^4       B^3       B^2        B         1
    //  |         |         |         |         |         |
    //  +-------+                   +---------+---------+
    //  |  Hx3  |                   | Hx0-Lx3 |    Lx0  |
    //  +------+----------+---------+---------+---------+
    //     |    y2    |  y1     |   y0    |
    //     ++---------+---------+---------+
    //     -| Hx0-Lx3 | - Lx0   |
    //      +---------+---------+
    //             | - Hx3  |
    //             +--------+
    // ```
    // We must take into account the carry from Hx0 - Lx3.
    split_into_chunks_mut!(out, n, [out_0, out_1, out_2], out_3);
    let carry = limbs_sub_same_length_in_place_left(out_1, &out_3[..n]);
    let (scratch_hi_last, scratch_hi_init) = scratch_hi.split_last().unwrap();
    let mut hi = *scratch_hi_last;
    if carry {
        hi.wrapping_add_assign(1);
    }
    let borrow = limbs_sub_same_length_with_borrow_in_in_place_left(out_2, out_0, carry);
    if limbs_sub_same_length_with_borrow_in_to_out(out_3, scratch_hi_init, out_1, borrow) {
        hi.wrapping_sub_assign(1);
    }
    if limbs_slice_add_greater_in_place_left(&mut out[n..n << 2], scratch_lo) {
        hi.wrapping_add_assign(1);
    }
    if s + t > n {
        split_into_chunks_mut!(out, n << 1, [_unused, out_lo], out_hi);
        let out_hi = &mut out_hi[..s + t - n];
        if limbs_sub_greater_in_place_left(out_lo, out_hi) {
            hi.wrapping_sub_assign(1);
        }
        if hi.get_highest_bit() {
            fail_on_untested_path("limbs_mul_greater_to_out_toom_32, hi.get_highest_bit()");
            assert!(!limbs_sub_limb_in_place(out_hi, hi.wrapping_neg()));
        } else {
            assert!(!limbs_slice_add_limb_in_place(out_hi, hi));
        }
    } else {
        assert_eq!(hi, 0);
    }
}}

// TODO tune
#[cfg(feature = "test_build")]
pub const TOOM33_MAYBE_MUL_BASECASE: bool =
    TUNE_PROGRAM_BUILD || WANT_FAT_BINARY || MUL_TOOM33_THRESHOLD < 3 * MUL_TOOM22_THRESHOLD;
#[cfg(not(feature = "test_build"))]
const TOOM33_MAYBE_MUL_BASECASE: bool =
    TUNE_PROGRAM_BUILD || WANT_FAT_BINARY || MUL_TOOM33_THRESHOLD < 3 * MUL_TOOM22_THRESHOLD;
// TODO tune
#[cfg(feature = "test_build")]
#[allow(clippy::absurd_extreme_comparisons)]
pub const TOOM33_MAYBE_MUL_TOOM33: bool =
    TUNE_PROGRAM_BUILD || WANT_FAT_BINARY || MUL_TOOM44_THRESHOLD >= 3 * MUL_TOOM33_THRESHOLD;
#[cfg(not(feature = "test_build"))]
#[allow(clippy::absurd_extreme_comparisons)]
const TOOM33_MAYBE_MUL_TOOM33: bool =
    TUNE_PROGRAM_BUILD || WANT_FAT_BINARY || MUL_TOOM44_THRESHOLD >= 3 * MUL_TOOM33_THRESHOLD;

fn limbs_mul_same_length_to_out_toom_33_recursive_scratch_len(xs_len: usize) -> usize {
    if TOOM33_MAYBE_MUL_BASECASE && xs_len < MUL_TOOM22_THRESHOLD {
        0
    } else if !TOOM33_MAYBE_MUL_TOOM33 || xs_len < MUL_TOOM33_THRESHOLD {
        limbs_mul_greater_to_out_toom_22_scratch_len(xs_len, xs_len)
    } else {
        limbs_mul_greater_to_out_toom_33_scratch_len(xs_len, xs_len)
    }
}

// A helper function for `limbs_mul_greater_to_out_toom_33`.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `TOOM33_MUL_N_REC` from `mpn/generic/toom33_mul.c`, GMP 6.2.1.
pub_test! {limbs_mul_same_length_to_out_toom_33_recursive(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let n = xs.len();
    assert_eq!(xs.len(), n);
    if TOOM33_MAYBE_MUL_BASECASE && n < MUL_TOOM22_THRESHOLD {
        limbs_mul_greater_to_out_basecase(out, xs, ys);
    } else if !TOOM33_MAYBE_MUL_TOOM33 || n < MUL_TOOM33_THRESHOLD {
        limbs_mul_greater_to_out_toom_22(out, xs, ys, scratch);
    } else {
        limbs_mul_greater_to_out_toom_33(out, xs, ys, scratch);
    }
}}

// TODO tune
const SMALLER_RECURSION_TOOM_33_AND_53: bool = true;

// This function can be used to determine whether the sizes of the input slices to
// `limbs_mul_greater_to_out_toom_33` are valid.
//
// # Worst-case complexity
// Constant time and additional memory.
pub_test! {limbs_mul_greater_to_out_toom_33_input_sizes_valid(
    xs_len: usize,
    ys_len: usize
) -> bool {
    xs_len >= ys_len && xs_len.div_round(3, Ceiling).0 << 1 < ys_len
}}

// This function can be used to determine the length of the input `scratch` slice in
// `limbs_mul_greater_to_out_toom_33`.
//
// Scratch need is 5 * xs_len / 2 + 10 * k, where k is the recursion depth. We use 3 * xs_len + C,
// so that we can use a smaller constant.
//
// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `mpn_toom33_mul_itch` from `gmp-impl.h`, GMP 6.2.1.
pub_crate_test! {limbs_mul_greater_to_out_toom_33_scratch_len(
    xs_len: usize,
    ys_len: usize
) -> usize {
    let n = xs_len.div_round(3, Ceiling).0;
    let m = n + 1;
    assert!(m < xs_len);
    let s = xs_len - (n << 1);
    let t = ys_len - (n << 1);
    let st_mul_scratch_len = if s > t {
        limbs_mul_greater_to_out_scratch_len(s, t)
    } else {
        limbs_mul_same_length_to_out_toom_33_recursive_scratch_len(s)
    };
    5 * m
        + max!(
            limbs_mul_same_length_to_out_toom_33_recursive_scratch_len(m),
            limbs_mul_same_length_to_out_toom_33_recursive_scratch_len(n),
            st_mul_scratch_len
        )
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
// the `xs.len() + ys.len()` least-significant limbs of the product of the `Natural`s to an output
// slice. A scratch slice is provided for the algorithm to use. An upper bound for the number of
// scratch limbs needed is provided by `limbs_mul_greater_to_out_toom_33_scratch_len`. The following
// restrictions on the input slices must be met:
// - `out`.len() >= `xs`.len() + `ys`.len()
// - `xs`.len() >= `ys`.len()
// - 2 * ceiling(`xs`.len() / 3) < `ys`.len()
//
// Approximately, `ys`.len() < `xs`.len() < 3 / 2 * `ys`.len().
//
// The smallest allowable `xs` length is 3. The smallest allowable `ys` length is also 3.
//
// This uses the Toom-33 aka Toom-3 algorithm.
//
// Evaluate in: -1, 0, 1, 2, Infinity.
// ```
// <--s--><--n--><--n-->
//  ____________________
// |_xs2_|__xs1_|__xs0_|
//  |ys2_|__ys1_|__ys0_|
//  <-t--><--n--><--n-->
//
// v0   =  xs0           *  b0                                 # X(0)   * Y(0)
// v1   = (xs0 +   * xs1 +  a2)    * (ys0 +  ys1+ ys2)         # X(1)   * Y(1)    xh  <= 2, yh <= 2
// vm1  = (xs0 -   * xs1 +  a2)    * (ys0 -  ys1+ ys2)         # X(-1)  * Y(-1)  |xh| <= 1, yh <= 1
// v2   = (xs0 + 2 * xs1 + 4 * a2) * (ys0 + 2 * ys1 + 4 * ys2) # X(2)   * Y(2)    xh  <= 6, yh <= 6
// vinf =            xs2           *  ys2                      # X(inf) * Y(inf)
// ```
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// The time would be O(n^{\log_3 5}) \approx O(n^{1.465}), but
// `limbs_mul_same_length_to_out_toom_33_recursive` ultimately calls `limbs_mul_greater_to_out`.
//
// # Panics
// May panic if the input slice conditions are not met.
//
// This is equivalent to `mpn_toom33_mul` from `mpn/generic/toom33_mul.c`, GMP 6.2.1.
pub_crate_test! {limbs_mul_greater_to_out_toom_33(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);
    let n = xs_len.div_round(3, Ceiling).0;
    let m = n + 1;
    split_into_chunks!(xs, n, [xs_0, xs_1], xs_2);
    let s = xs_2.len();
    split_into_chunks!(ys, n, [ys_0, ys_1], ys_2);
    let t = ys_2.len();
    assert_ne!(t, 0);
    split_into_chunks_mut!(out, m, [bs1, as2, bs2], _unused);
    // we need 4 * n + 4 <= 4 * n + s + t
    let (v_neg_1, remainder) = scratch.split_at_mut(m << 1);
    split_into_chunks_mut!(remainder, m, [asm1, bsm1, as1], mul_scratch);
    let gp = &mut v_neg_1[..n];
    // Compute as1 and asm1.
    let mut carry = Limb::from(limbs_add_to_out(gp, xs_0, xs_2));
    let (as1_last, as1_init) = as1.split_last_mut().unwrap();
    *as1_last = carry;
    if limbs_add_same_length_to_out(as1_init, gp, xs_1) {
        as1_last.wrapping_add_assign(1);
    }
    let mut v_neg_1_neg = carry == 0 && limbs_cmp_same_length(gp, xs_1) == Less;
    let (asm1_last, asm1_init) = asm1.split_last_mut().unwrap();
    if v_neg_1_neg {
        limbs_sub_same_length_to_out(asm1_init, xs_1, gp);
        *asm1_last = 0;
    } else {
        if limbs_sub_same_length_to_out(asm1_init, gp, xs_1) {
            carry.wrapping_sub_assign(1);
        }
        *asm1_last = carry;
    }
    // Compute as2.
    let (as2_last, as2_init) = as2.split_last_mut().unwrap();
    let carry = if s == n {
        limbs_add_same_length_to_out(as2_init, xs_2, &as1_init[..s])
    } else if limbs_add_same_length_to_out(as2_init, xs_2, &as1_init[..s]) {
        limbs_add_limb_to_out(&mut as2_init[s..], &as1_init[s..], 1)
    } else {
        as2_init[s..].copy_from_slice(&as1_init[s..]);
        false
    };
    let mut carry = Limb::from(carry)
        .wrapping_add(*as1_last)
        .arithmetic_checked_shl(1u64)
        .unwrap()
        .wrapping_add(limbs_slice_shl_in_place(as2_init, 1));
    if limbs_sub_same_length_in_place_left(as2_init, xs_0) {
        carry.wrapping_sub_assign(1);
    }
    *as2_last = carry;
    // Compute bs1 and bsm1.
    let (bs1_last, bs1_init) = bs1.split_last_mut().unwrap();
    let mut carry = Limb::from(limbs_add_to_out(gp, ys_0, ys_2));
    *bs1_last = carry;
    if limbs_add_same_length_to_out(bs1_init, gp, ys_1) {
        *bs1_last += 1;
    }
    let (bsm1_last, bsm1_init) = bsm1.split_last_mut().unwrap();
    if carry == 0 && limbs_cmp_same_length(gp, ys_1) == Less {
        limbs_sub_same_length_to_out(bsm1_init, ys_1, gp);
        *bsm1_last = 0;
        v_neg_1_neg.not_assign();
    } else {
        if limbs_sub_same_length_to_out(bsm1_init, gp, ys_1) {
            carry.wrapping_sub_assign(1);
        }
        *bsm1_last = carry;
    }
    // Compute bs2.
    let (bs2_last, bs2_init) = bs2.split_last_mut().unwrap();
    let carry = if t == n {
        limbs_add_same_length_to_out(bs2_init, &bs1_init[..t], ys_2)
    } else if limbs_add_same_length_to_out(bs2_init, &bs1_init[..t], ys_2) {
        limbs_add_limb_to_out(&mut bs2_init[t..], &bs1_init[t..], 1)
    } else {
        bs2_init[t..].copy_from_slice(&bs1_init[t..]);
        false
    };
    let mut carry = Limb::from(carry)
        .wrapping_add(*bs1_last)
        .arithmetic_checked_shl(1u64)
        .unwrap()
        .wrapping_add(limbs_slice_shl_in_place(bs2_init, 1));
    if limbs_sub_same_length_in_place_left(bs2_init, ys_0) {
        carry.wrapping_sub_assign(1);
    }
    *bs2_last = carry;
    assert!(*as1_last <= 2);
    assert!(*bs1_last <= 2);
    assert!(*asm1_last <= 1);
    assert!(*bsm1_last <= 1);
    assert!(*as2_last <= 6);
    assert!(*bs2_last <= 6);
    if SMALLER_RECURSION_TOOM_33_AND_53 {
        // size: n
        limbs_mul_same_length_to_out_toom_33_recursive(v_neg_1, asm1_init, bsm1_init, mul_scratch);
        let (v_neg_1_last, v_neg_1_init) = v_neg_1[n..=n<<1].split_last_mut().unwrap();
        let mut carry = 0;
        if *asm1_last != 0 {
            carry = *bsm1_last;
            if limbs_slice_add_same_length_in_place_left(v_neg_1_init, bsm1_init) {
                carry.wrapping_add_assign(1);
            }
        }
        if *bsm1_last != 0 && limbs_slice_add_same_length_in_place_left(v_neg_1_init, asm1_init) {
            carry.wrapping_add_assign(1);
        }
        *v_neg_1_last = carry;
    } else {
        fail_on_untested_path("limbs_mul_greater_to_out_toom_33, !SMALLER_RECURSION");
        // size: m
        limbs_mul_same_length_to_out_toom_33_recursive(v_neg_1, asm1, bsm1, mul_scratch);
    }
    let (v_2, mul_scratch) = scratch[2 * n + 1..].split_at_mut(3 * n + 4);
    // size: m
    limbs_mul_same_length_to_out_toom_33_recursive(v_2, as2, bs2, mul_scratch);
    let v_inf = &mut out[n << 2..];
    // v_inf, s + t limbs
    if s > t {
        // size: s, t
        limbs_mul_greater_to_out(v_inf, xs_2, ys_2, mul_scratch);
    } else {
        // size: s
        limbs_mul_same_length_to_out_toom_33_recursive(v_inf, xs_2, &ys_2[..s], mul_scratch);
    }
    let v_inf0 = v_inf[0]; // v1 overlaps with this
    let (as1, mul_scratch) = scratch[m << 2..].split_at_mut(m);
    let (bs1, v_1) = out.split_at_mut(n << 1);
    let bs1 = &bs1[..m];
    if SMALLER_RECURSION_TOOM_33_AND_53 {
        let (as1_last, as1_init) = as1.split_last().unwrap();
        let (bs1_last, bs1_init) = bs1.split_last().unwrap();
        // size: n
        limbs_mul_same_length_to_out_toom_33_recursive(v_1, as1_init, bs1_init, mul_scratch);
        let (v_1_lo, v_1_hi) = v_1[n..].split_at_mut(n);
        let mut carry = 0;
        if *as1_last == 1 {
            carry = *bs1_last;
            if limbs_slice_add_same_length_in_place_left(v_1_lo, bs1_init) {
                carry += 1;
            }
        } else if *as1_last != 0 {
            carry = bs1_last.arithmetic_checked_shl(1).unwrap();
            carry.wrapping_add_assign(limbs_slice_add_mul_limb_same_length_in_place_left(
                v_1_lo, bs1_init, 2,
            ));
        }
        if *bs1_last == 1 {
            if limbs_slice_add_same_length_in_place_left(v_1_lo, as1_init) {
                carry += 1;
            }
        } else if *bs1_last != 0 {
            carry += limbs_slice_add_mul_limb_same_length_in_place_left(v_1_lo, as1_init, 2);
        }
        v_1_hi[0] = carry;
    } else {
        let carry = v_1[2 * n + 1];
        // size: m
        limbs_mul_same_length_to_out_toom_33_recursive(v_1, as1, bs1, mul_scratch);
        v_1[2 * n + 1] = carry;
    }
    // size: n
    limbs_mul_same_length_to_out_toom_33_recursive(out, &xs[..n], &ys[..n], mul_scratch);
    let (v_neg_1, v_2) = scratch.split_at_mut(2 * n + 1);
    limbs_mul_toom_interpolate_5_points(out, v_2, v_neg_1, n, s + t, v_neg_1_neg, v_inf0);
}}

// This function can be used to determine whether the sizes of the input slices to
// `limbs_mul_greater_to_out_toom_42` are valid.
//
// # Worst-case complexity
// Constant time and additional memory.
pub_test! {
    limbs_mul_greater_to_out_toom_42_input_sizes_valid(xs_len: usize, ys_len: usize) -> bool {
    !(xs_len == 9 && ys_len == 4)
        && xs_len + 3 < ys_len << 2
        && xs_len.div_round(3, Ceiling).0
            > ys_len.shr_round(1, Ceiling).0
}}

// This function can be used to determine the length of the input `scratch` slice in
// `limbs_mul_greater_to_out_toom_42`.
//
// This is equivalent to `mpn_toom42_mul_itch` from `gmp-impl.h`, GMP 6.2.1.
//
// # Worst-case complexity
// Constant time and additional memory.
pub_crate_test! {limbs_mul_greater_to_out_toom_42_scratch_len(
    xs_len: usize,
    ys_len: usize
) -> usize {
    let n = if xs_len >= ys_len << 1 {
        xs_len.shr_round(2, Ceiling).0
    } else {
        ys_len.shr_round(1, Ceiling).0
    };
    let s = xs_len - 3 * n;
    let t = ys_len - n;
    assert!(n + 1 < xs_len);
    10 * n
        + 8
        + max!(
            limbs_mul_same_length_to_out_scratch_len(n),
            limbs_mul_same_length_to_out_scratch_len(n + 1),
            limbs_mul_to_out_scratch_len(s, t)
        )
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
// the `xs.len() + ys.len()` least-significant limbs of the product of the `Natural`s to an output
// slice. A scratch slice is provided for the algorithm to use. An upper bound for the number of
// scratch limbs needed is provided by `limbs_mul_greater_to_out_toom_42_scratch_len`. The following
// restrictions on the input slices must be met:
// - `out`.len() >= `xs`.len() + `ys`.len()
// - ceiling(`xs`.len() / 3) > ceiling(`ys`.len() / 2)
// - `xs`.len() + 3 < 4 * `ys`.len()
// - (`xs`.len(), `ys`.len()) != (9, 4)
//
// Approximately, 3 / 2 * `ys`.len() < `xs`.len() < 4 * `ys`.len().
//
// The smallest allowable `xs` length is 4. The smallest allowable `ys` length is 2.
//
// This uses the Toom-42 algorithm.
//
// Evaluate in: -1, 0, 1, 2, Infinity.
// ```
// <-s--><--n---><--n---><--n--->
//  _____________________________
// |xs3_|__xs2__|__xs1__|__xs0__|
//               |_ys1__|__ys0__|
//               <--t--><---n--->
//
// v_0     =  xs0                          *  ys0          # X(0)  * Y(0)
// v_1     = (xs0 +   xs1 +   xs2 +   xs3) * (ys0 + ys1)   # X(1)  * Y(1)   xh  <= 3  yh <= 1
// v_neg_1 = (xs0 -   xs1 +   xs2 -   xs3) * (ys0 - ys1)   # X(-1) * Y(-1) |xh| <= 1  yh  = 0
// v_2     = (xs0 + 2*xs1 + 4*xs2 + 8*xs3) * (ys0 + 2*ys1) # X(2)  * Y(2)   xh  <= 14 yh <= 2
// v_inf   =  xs3 *     b1  # A(inf)*B(inf)
// ```
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// The time would be O(n^{\log_4 5}) \approx O(n^{1.161}), but `limbs_mul_same_length_to_out` is
// called.
//
// # Panics
// May panic if the input slice conditions are not met.
//
// This is equivalent to `mpn_toom42_mul` from `mpn/generic/toom42_mul.c`, GMP 6.2.1.
pub_crate_test! {limbs_mul_greater_to_out_toom_42(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let n = if xs_len >= ys_len << 1 {
        xs_len.shr_round(2, Ceiling).0
    } else {
        ys_len.shr_round(1, Ceiling).0
    };
    split_into_chunks!(xs, n, [xs_0, xs_1, xs_2], xs_3);
    let s = xs_3.len();
    assert_ne!(s, 0);
    assert!(s <= n);
    split_into_chunks!(ys, n, [ys_0], ys_1);
    let t = ys_1.len();
    assert_ne!(t, 0);
    assert!(t <= n);
    let (scratch, mul_scratch) = scratch.split_at_mut(10 * n + 8);
    let (scratch_lo, scratch_hi) = scratch.split_at_mut(6 * n + 5);
    split_into_chunks_mut!(scratch_lo, n + 1, [as1, asm1, as2, bs1], remainder);
    let (bsm1, bs2) = remainder.split_at_mut(n);
    // Compute as1 and asm1.
    let mut v_neg_1_neg =
        limbs_mul_toom_evaluate_deg_3_poly_in_1_and_neg_1(as1, asm1, xs, n, &mut out[..=n]);
    // Compute as2.
    let (as2_last, as2_init) = as2.split_last_mut().unwrap();
    let mut carry = limbs_shl_to_out(as2_init, xs_3, 1);
    if limbs_slice_add_same_length_in_place_left(&mut as2_init[..s], &xs_2[..s]) {
        carry += 1;
    }
    if s != n {
        carry = Limb::from(limbs_add_limb_to_out(&mut as2_init[s..], &xs_2[s..], carry));
    }
    carry = carry.arithmetic_checked_shl(1).unwrap();
    carry.wrapping_add_assign(limbs_slice_shl_in_place(as2_init, 1));
    if limbs_slice_add_same_length_in_place_left(as2_init, xs_1) {
        carry += 1;
    }
    carry = carry.arithmetic_checked_shl(1).unwrap();
    carry.wrapping_add_assign(limbs_slice_shl_in_place(as2_init, 1));
    if limbs_slice_add_same_length_in_place_left(as2_init, xs_0) {
        carry += 1;
    }
    *as2_last = carry;
    // Compute bs1 and bsm1.
    let (bs1_last, bs1_init) = bs1.split_last_mut().unwrap();
    if t == n {
        *bs1_last = Limb::from(limbs_add_same_length_to_out(bs1_init, ys_0, ys_1));
        if limbs_cmp_same_length(ys_0, ys_1) == Less {
            limbs_sub_same_length_to_out(bsm1, ys_1, ys_0);
            v_neg_1_neg.not_assign();
        } else {
            limbs_sub_same_length_to_out(bsm1, ys_0, ys_1);
        }
    } else {
        *bs1_last = Limb::from(limbs_add_to_out(bs1_init, ys_0, ys_1));
        if slice_test_zero(&ys_0[t..]) && limbs_cmp_same_length(&ys_0[..t], ys_1) == Less
        {
            limbs_sub_same_length_to_out(bsm1, ys_1, &ys_0[..t]);
            slice_set_zero(&mut bsm1[t..]);
            v_neg_1_neg.not_assign();
        } else {
            limbs_sub_greater_to_out(bsm1, ys_0, ys_1);
        }
    }
    // Compute bs2, recycling bs1. bs2 = bs1 + ys_1
    limbs_add_to_out(bs2, bs1, ys_1);
    let (as1_last, as1_init) = as1.split_last().unwrap();
    let (bs1_last, bs1_init) = bs1.split_last().unwrap();
    let (asm1_last, asm1_init) = asm1.split_last().unwrap();
    assert!(*as1_last <= 3);
    assert!(*bs1_last <= 1);
    assert!(*asm1_last <= 1);
    assert!(*as2_last <= 14);
    assert!(bs2[n] <= 2);
    let (v_neg_1, v_2) = scratch_hi.split_at_mut(2 * n + 1);
    split_into_chunks_mut!(out, n << 1, [v_0, v_1], v_inf);
    // size: n
    limbs_mul_same_length_to_out(v_neg_1, asm1_init, bsm1, mul_scratch);
    let (v_neg_1_last, v_neg_1_init) = v_neg_1.split_last_mut().unwrap();
    *v_neg_1_last = Limb::from(
        *asm1_last != 0 && limbs_slice_add_same_length_in_place_left(&mut v_neg_1_init[n..], bsm1),
    );
    // size: n + 1
    limbs_mul_same_length_to_out(v_2, as2, bs2, mul_scratch);
    // size: s, t
    limbs_mul_to_out(v_inf, xs_3, ys_1, mul_scratch);
    // size: n + 1
    limbs_mul_same_length_to_out(v_1, as1_init, bs1_init, mul_scratch);
    let v_1 = &mut v_1[n..];
    let mut carry = match *as1_last {
        1 => {
            let mut carry = *bs1_last;
            if limbs_slice_add_same_length_in_place_left(v_1, bs1_init) {
                carry += 1;
            }
            carry
        }
        2 => (*bs1_last << 1).wrapping_add(limbs_slice_add_mul_limb_same_length_in_place_left(
            v_1, bs1_init, 2,
        )),
        3 => bs1_last.wrapping_mul(3).wrapping_add(
            limbs_slice_add_mul_limb_same_length_in_place_left(v_1, bs1_init, 3),
        ),
        _ => *v_neg_1_last,
    };
    if *bs1_last != 0 && limbs_slice_add_same_length_in_place_left(v_1, as1_init) {
        carry += 1;
    }
    let v_inf_0 = v_inf[0]; // v_1 overlaps with this
    v_inf[0] = carry;
    // size: n
    limbs_mul_same_length_to_out(v_0, xs_0, ys_0, mul_scratch);
    limbs_mul_toom_interpolate_5_points(out, v_2, v_neg_1, n, s + t, v_neg_1_neg, v_inf_0);
}}

// This function can be used to determine whether the sizes of the input slices to
// `limbs_mul_greater_to_out_toom_43` are valid.
//
// # Worst-case complexity
// Constant time and additional memory.
pub_test! {
    limbs_mul_greater_to_out_toom_43_input_sizes_valid(xs_len: usize, ys_len: usize) -> bool {
    !(xs_len == 16 && ys_len == 13)
        && xs_len.div_round(3, Ceiling).0
            > ys_len.div_round(3, Ceiling).0
        && {
            let xs_len_div_4: usize = xs_len.shr_round(2, Ceiling).0;
            xs_len_div_4 < ys_len.shr_round(1, Ceiling).0
                && xs_len + ys_len >= 5 * (xs_len_div_4 + 1)
        }
}}

// This function can be used to determine the length of the input `scratch` slice in
// `limbs_mul_greater_to_out_toom_43`.
//
// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `mpn_toom43_mul_itch` from `gmp-impl.h`, GMP 6.2.1.
pub_crate_test! {limbs_mul_greater_to_out_toom_43_scratch_len(
    xs_len: usize,
    ys_len: usize
) -> usize {
    let n = 1 + if 3 * xs_len >= ys_len << 2 {
        (xs_len - 1) >> 2
    } else {
        (ys_len - 1) / 3
    };
    let s = xs_len - 3 * n;
    let t = ys_len - (n << 1);
    assert!(n + 1 < xs_len);
    6 * n
        + 4
        + max!(
            limbs_mul_same_length_to_out_scratch_len(n),
            limbs_mul_same_length_to_out_scratch_len(n + 1),
            limbs_mul_to_out_scratch_len(s, t)
        )
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
// the `xs.len() + ys.len()` least-significant limbs of the product of the `Natural`s to an output
// slice. A scratch slice is provided for the algorithm to use. An upper bound for the number of
// scratch limbs needed is provided by `limbs_mul_greater_to_out_toom_43_scratch_len`. The following
// restrictions on the input slices must be met:
// - `out`.len() >= `xs`.len() + `ys`.len()
// - ceiling(`xs`.len() / 3) > ceiling(`ys`.len() / 3)
// - ceiling(`xs`.len() / 4) < ceiling(`ys`.len() / 2)
// - (`xs`.len(), `ys`.len()) != (16, 13)
// - `xs`.len() + `ys`.len() >= 5 * (ceiling(`xs`.len() / 4) + 1)
//
// Approximately, `ys`.len() < `xs`.len() < 2 * `ys`.len().
//
// The smallest allowable `xs` length is 11. The smallest allowable `ys` length is 8.
//
// This uses the Toom-43 algorithm.
// ```
// <-s--><--n--><--n--><--n-->
//  __________________________
// |xs3_|__xs2_|__xs1_|__xs0_|
//       |_ys2_|__ys1_|__ys0_|
//       <-t--><--n--><--n-->
//
// v_0     =  xs0                          * ys0                   # X(0) *Y(0)
// v_1     = (xs0 +   xs1 +   xs2 +   xs3) * (ys0 +   ys1 +   ys2) # X(1) *Y(1)   xh  <= 3  yh <= 2
// v_neg_1 = (xs0 -   xs1 +   xs2 -   xs3) * (ys0 -   ys1 +   ys2) # X(-1)*Y(-1) |xh| <= 1 |yh|<= 1
// v_2     = (xs0 + 2*xs1 + 4*xs2 + 8*xs3) * (ys0 + 2*ys1 + 4*ys2) # X(2) *Y(2)   xh  <= 14 yh <= 6
// v_neg_2 = (xs0 - 2*xs1 + 4*xs2 - 8*xs3) * (ys0 - 2*ys1 + 4*ys2) # X(-2)*Y(-2) |xh| <= 9 |yh|<= 4
// v_inf   =                          xs3 *                   ys2  # X(inf)*Y(inf)
// ```
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// The time would be O(n^{\log_4 6}) \approx O(n^{1.292}), but this function calls
// `limbs_mul_same_length_to_out`.
//
// # Panics
// May panic if the input slice conditions are not met.
//
// This is equivalent to `mpn_toom43_mul` from `mpn/generic/toom43_mul.c`, GMP 6.2.1.
pub_crate_test! {limbs_mul_greater_to_out_toom_43(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let n = 1 + if 3 * xs_len >= ys_len << 2 {
        (xs_len - 1) >> 2
    } else {
        (ys_len - 1) / 3
    };
    let xs_3 = &xs[3 * n..];
    let s = xs_3.len();
    assert_ne!(s, 0);
    assert!(s <= n);
    split_into_chunks!(ys, n, [ys_0, ys_1], ys_2);
    let t = ys_2.len();
    assert_ne!(t, 0);
    assert!(t <= n);
    // This is probably true whenever `xs_len` >= 25 or `ys_len` >= 19. It guarantees that we can
    // fit 5 values of size n + 1 in the product area.
    assert!(s + t >= 5);
    let m = n + 1;
    let mut v_neg_1_neg = false;
    let mut v_neg_2_neg = false;
    split_into_chunks_mut!(out, m, [bs1, bsm2, bs2, as2, as1], _unused);
    let (scratch, mul_scratch) = scratch.split_at_mut(6 * n + 4);
    split_into_chunks_mut!(scratch, m, [small_scratch, _unused, bsm1, asm1], asm2);
    // Compute as2 and asm2.
    if limbs_mul_toom_evaluate_deg_3_poly_in_2_and_neg_2(as2, asm2, xs, n, asm1) {
        v_neg_2_neg = true;
    }
    // Compute bs2 and bsm2.
    bsm1[n] = limbs_shl_to_out(bsm1, ys_1, 1); // 2 * ys_1
    let mut carry = limbs_shl_to_out(small_scratch, ys_2, 2); // 4 * ys_2
    if limbs_slice_add_same_length_in_place_left(&mut small_scratch[..t], &ys_0[..t]) {
        carry += 1;
    }
    // 4 * ys_2 + ys_0
    if t != n {
        carry = Limb::from(limbs_add_limb_to_out(
            &mut small_scratch[t..],
            &ys_0[t..],
            carry,
        ));
    }
    small_scratch[n] = carry;
    limbs_add_same_length_to_out(bs2, small_scratch, bsm1);
    if limbs_cmp_same_length(small_scratch, bsm1) == Less {
        limbs_sub_same_length_to_out(bsm2, bsm1, small_scratch);
        v_neg_2_neg.not_assign();
    } else {
        limbs_sub_same_length_to_out(bsm2, small_scratch, bsm1);
    }
    // Compute as1 and asm1.
    if limbs_mul_toom_evaluate_deg_3_poly_in_1_and_neg_1(as1, asm1, xs, n, small_scratch) {
        v_neg_1_neg = true;
    }
    let (bsm1_last, bsm1_init) = bsm1.split_last_mut().unwrap();
    // Compute bs1 and bsm1.
    *bsm1_last = Limb::from(limbs_add_to_out(bsm1_init, ys_0, ys_2));
    let (bs1_last, bs1_init) = bs1.split_last_mut().unwrap();
    *bs1_last = *bsm1_last;
    if limbs_add_same_length_to_out(bs1_init, bsm1_init, ys_1) {
        *bs1_last += 1;
    }
    if *bsm1_last == 0 && limbs_cmp_same_length(bsm1_init, ys_1) == Less {
        limbs_sub_same_length_in_place_right(ys_1, bsm1_init);
        v_neg_1_neg.not_assign();
    } else if limbs_sub_same_length_in_place_left(bsm1_init, ys_1) {
        bsm1_last.wrapping_sub_assign(1);
    }
    assert!(as1[n] <= 3);
    assert!(*bs1_last <= 2);
    assert!(asm1[n] <= 1);
    assert!(*bsm1_last <= 1);
    assert!(as2[n] <= 14);
    assert!(bs2[n] <= 6);
    assert!(asm2[n] <= 9);
    assert!(bsm2[n] <= 4);
    let (v_neg_1, remainder) = scratch.split_at_mut(m << 1);
    split_into_chunks_mut!(remainder, m, [bsm1, asm1], _unused);
    // size: m
    limbs_mul_same_length_to_out(v_neg_1, asm1, bsm1, mul_scratch); // W4
    let two_n = n << 1;
    let (v_neg_2, asm2) = scratch[two_n + 1..].split_at_mut(two_n + 3);
    split_into_chunks_mut!(out, m, [_unused, out_lo, bs2, as2], _unused);
    // size: m
    limbs_mul_same_length_to_out(v_neg_2, &asm2[..m], out_lo, mul_scratch);
    // size: m
    limbs_mul_same_length_to_out(&mut scratch[(n << 2) + 2..], as2, bs2, mul_scratch);
    let (bs1, remainder) = out.split_at_mut(two_n);
    let (v_1, as1) = remainder.split_at_mut(two_n + 4);
    // size: m
    limbs_mul_same_length_to_out(v_1, &as1[..m], &bs1[..m], mul_scratch);
    // size: s, t
    limbs_mul_to_out(&mut out[5 * n..], xs_3, ys_2, mul_scratch);
    // size: n
    limbs_mul_same_length_to_out(&mut out[..two_n], &xs[..n], ys_0, mul_scratch);
    split_into_chunks_mut!(scratch, two_n + 1, [v_neg_1, v_neg_2, v_2], _unused);
    limbs_mul_toom_interpolate_6_points(
        out,
        n,
        t + s,
        v_neg_1_neg,
        v_neg_1,
        v_neg_2_neg,
        v_neg_2,
        v_2,
    );
}}

// This function can be used to determine whether the sizes of the input slices to
// `limbs_mul_greater_to_out_toom_44` are valid.
//
// # Worst-case complexity
// Constant time and additional memory.
pub_test! {limbs_mul_greater_to_out_toom_44_input_sizes_valid(
    xs_len: usize,
    ys_len: usize
) -> bool {
    xs_len >= ys_len && 3usize * xs_len.shr_round(2, Ceiling).0 < ys_len
}}

// This function can be used to determine the length of the input `scratch` slice in
// `limbs_mul_greater_to_out_toom_44`.
//
// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `mpn_toom44_mul_itch` from `gmp-impl.h`, GMP 6.2.1.
pub_crate_test! {limbs_mul_greater_to_out_toom_44_scratch_len(
    xs_len: usize,
    ys_len: usize
) -> usize {
    let n = xs_len.shr_round(2, Ceiling).0;
    let s = xs_len - 3 * n;
    let t = ys_len - 3 * n;
    assert!(n + 1 < xs_len);
    let alt_mul_scratch_len = if s > t {
        limbs_mul_greater_to_out_scratch_len(s, t)
    } else {
        limbs_mul_same_length_to_out_scratch_len(s)
    };
    max(
        9 * n
            + 6
            + max!(
                limbs_mul_same_length_to_out_scratch_len(n),
                limbs_mul_same_length_to_out_scratch_len(n + 1),
                alt_mul_scratch_len
            ),
        (n << 3) + 6usize + max(n << 1, s + t),
    )
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
// the `xs.len() + ys.len()` least-significant limbs of the product of the `Natural`s to an output
// slice. A scratch slice is provided for the algorithm to use. An upper bound for the number of
// scratch limbs needed is provided by `limbs_mul_greater_to_out_toom_44_scratch_len`. The following
// restrictions on the input slices must be met:
// - `out`.len() >= `xs`.len() + `ys`.len()
// - `xs`.len() >= `ys`.len()
// - 3 * ceiling(`xs`.len() / 4) < `ys`.len()
//
// Approximately, `ys`.len() < `xs`.len() < 4 / 3 * `ys`.len().
//
// The smallest allowable `xs` length is 4. The smallest allowable `ys` length is also 4.
//
// This uses the Toom-44 algorithm.
//
// Evaluate in: 0, 1, -1, 2, -2, 1 / 2, Infinity.
// ```
// <-s--><--n--><--n--><--n-->
//  __________________________
// |_xs3|__xs2_|__xs1_|__xs0_|
//  |ys3|__ys2_|__ys1_|__ys0_|
//  <-t-><--n--><--n--><--n-->
//
// v_0     =   x0             *  y0              #   X(0)*Y(0)
// v_1     = ( x0+ x1+ x2+ x3)*( y0+ y1+ y2+ y3) #   X(1)*Y(1)    xh  <= 3   yh  <= 3
// v_neg_1 = ( x0- x1+ x2- x3)*( y0- y1+ y2- y3) #  X(-1)*Y(-1)  |xh| <= 1  |yh| <= 1
// v_2     = ( x0+2x1+4x2+8x3)*( y0+2y1+4y2+8y3) #   X(2)*Y(2)    xh  <= 14  yh  <= 14
// v_neg_2 = ( x0-2x1+4x2-8x3)*( y0-2y1+4y2-8y3) #   X(2)*Y(2)    xh  <= 9  |yh| <= 9
// v_half  = (8x0+4x1+2x2+ x3)*(8y0+4y1+2y2+ y3) # X(1/2)*Y(1/2)  xh  <= 14  yh  <= 14
// v_inf   =               x3 *          y2      # X(inf)*Y(inf)
// ```
//
// Use of scratch space: In the product area, we store
// ```
//    _______________________
//   |v_inf|____|_v_1_|_v_0_|
//    s+t   2n-1  2n+1  2n
// ```
//
// The other recursive products, v_neg_1, v_2, v_neg_2, and v_half, are stored in the scratch area.
// When computing them, we use the product area for intermediate values.
//
// Next, we compute v_1. We can store the intermediate factors at v_0 and at v_half + 2 * n + 2.
//
// Finally, for v_0 and v_inf, factors are parts of the input operands, and we need scratch space
// only for the recursive multiplication.
//
// In all, if S(xs_len) is the scratch need, the needed space is bounded by
//
// S(xs_len) <= 4 (2 * ceil(xs_len / 4) + 1) + 1 + S(ceil(xs_len / 4) + 1)
//
// which should give S(n) = 8 * n / 3 + c * log(n) for some constant c.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// The time would be O(n^{\log_4 7}) \approx O(n^{1.404}), but
// `limbs_mul_same_length_to_out_toom_44_recursive` calls `limbs_mul_same_length_to_out`.
//
// # Panics
// May panic if the input slice conditions are not met.
//
// This is equivalent to `mpn_toom44_mul` from `mpn/generic/toom44_mul.c`, GMP 6.2.1.
pub_crate_test! {limbs_mul_greater_to_out_toom_44(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);
    let n = xs_len.shr_round(2, Ceiling).0;
    let m = 2 * n + 1;
    split_into_chunks!(xs, n, [xs_0, xs_1, xs_2], xs_3);
    let s = xs_3.len();
    assert_ne!(s, 0);
    assert!(s <= n);
    split_into_chunks!(ys, n, [ys_0, ys_1, ys_2], ys_3);
    let t = ys_3.len();
    assert_ne!(t, 0);
    assert!(t <= n);
    let k = n + 1;
    // NOTE: The multiplications to v_2, v_neg_2, v_half, and v_neg_1 overwrite the following limb,
    // so these must be computed in order, and we need a one limb gap to scratch2. apx and bpx must
    // not overlap with v1
    split_into_chunks_mut!(out, k, [apx, amx], remainder);
    let (bmx, bpx) = remainder.split_at_mut(n << 1);
    let bmx = &mut bmx[..k];
    let bpx = &mut bpx[..k];
    // Total scratch need: 8 * n + 5 + scratch for recursive calls. This gives roughly 32 * n / 3 +
    // log term.
    let (scratch_lo, mul_scratch) = scratch.split_at_mut(9 * n + 6);
    let (v_2, scratch_hi) = scratch_lo.split_at_mut((n << 3) + 5);
    let scratch_hi = &mut scratch_hi[..k];
    // Compute apx = xs_0 + 2 * xs_1 + 4 * xs_2 + 8 xs_3 and amx = xs_0 - 2 * xs_1 + 4 * xs_2 - 8 *
    // xs_3.
    let mut w1_neg =
        limbs_mul_toom_evaluate_deg_3_poly_in_2_and_neg_2(&mut apx[..k], amx, xs, n, scratch_hi);
    // Compute bpx = ys_0 + 2 * ys_1 + 4 * ys_2 + 8 * ys_3 and bmx = ys_0 - 2 * ys_1 + 4 * ys_2 - 8
    // * ys_3.
    if limbs_mul_toom_evaluate_deg_3_poly_in_2_and_neg_2(bpx, bmx, ys, n, scratch_hi) {
        w1_neg.not_assign();
    }
    // size: n + 1
    limbs_mul_same_length_to_out(v_2, apx, bpx, mul_scratch);
    // size: n + 1
    limbs_mul_same_length_to_out(&mut scratch_lo[m..], amx, bmx, mul_scratch);
    // Compute apx = 8 * xs_0 + 4 * xs_1 + 2 * xs_2 + xs_3 = (((2 * xs_0 + xs_1) * 2 + xs_2) * 2 +
    // xs_3
    let (apx_last, apx_init) = apx.split_last_mut().unwrap();
    let mut carry = limbs_shl_to_out(apx_init, xs_0, 1);
    if limbs_slice_add_same_length_in_place_left(apx_init, xs_1) {
        carry.wrapping_add_assign(1);
    }
    carry = carry.arithmetic_checked_shl(1).unwrap();
    carry.wrapping_add_assign(limbs_slice_shl_in_place(apx_init, 1));
    if limbs_slice_add_same_length_in_place_left(apx_init, xs_2) {
        carry.wrapping_add_assign(1);
    }
    carry = carry.arithmetic_checked_shl(1).unwrap();
    *apx_last = carry.wrapping_add(limbs_slice_shl_in_place(apx_init, 1));
    if limbs_slice_add_greater_in_place_left(apx_init, xs_3) {
        apx_last.wrapping_add_assign(1);
    }
    // Compute bpx = 8 ys_0 + 4 ys_1 + 2 ys_2 + ys_3 = (((2*ys_0 + ys_1) * 2 + ys_2) * 2 + ys_3
    let (bpx_last, bpx_init) = bpx.split_last_mut().unwrap();
    let mut carry = limbs_shl_to_out(bpx_init, ys_0, 1);
    if limbs_slice_add_same_length_in_place_left(bpx_init, ys_1) {
        carry.wrapping_add_assign(1);
    }
    carry = carry.arithmetic_checked_shl(1).unwrap();
    carry.wrapping_add_assign(limbs_slice_shl_in_place(bpx_init, 1));
    if limbs_slice_add_same_length_in_place_left(bpx_init, ys_2) {
        carry.wrapping_add_assign(1);
    }
    carry = carry.arithmetic_checked_shl(1).unwrap();
    *bpx_last = carry.wrapping_add(limbs_slice_shl_in_place(bpx_init, 1));
    if limbs_slice_add_greater_in_place_left(bpx_init, ys_3) {
        bpx_last.wrapping_add_assign(1);
    }
    assert!(*apx_last < 15);
    assert!(*bpx_last < 15);
    let (v_half, scratch_hi) = scratch_lo[m << 1..].split_at_mut((n << 2) + 3);
    let scratch_hi = &mut scratch_hi[..k];
    // size: n + 1
    limbs_mul_same_length_to_out(v_half, apx, bpx, mul_scratch);
    // Compute apx = xs_0 + xs_1 + xs_2 + xs_3 and amx = xs_0 - xs_1 + xs_2 - xs_3.
    let mut w3_neg = limbs_mul_toom_evaluate_deg_3_poly_in_1_and_neg_1(apx, amx, xs, n, scratch_hi);
    // Compute bpx = ys_0 + ys_1 + ys_2 + ys_3 and bmx = ys_0 - ys_1 + ys_2 - ys_3.
    if limbs_mul_toom_evaluate_deg_3_poly_in_1_and_neg_1(bpx, bmx, ys, n, scratch_hi) {
        w3_neg.not_assign();
    }
    // size: n + 1
    limbs_mul_same_length_to_out(&mut scratch_lo[3 * m..], amx, bmx, mul_scratch);
    let (apx, remainder) = out.split_at_mut(n << 1);
    let (v_1, bpx) = remainder.split_at_mut(m + 1);
    // - Clobbers amx, bmx.
    // - size: n + 1
    limbs_mul_same_length_to_out(v_1, &apx[..k], &bpx[..k], mul_scratch);
    let (v_0, v_inf) = out.split_at_mut(n << 1); // v_0 length: 2 * n
    let v_inf = &mut v_inf[n << 2..];
    // size: n
    limbs_mul_same_length_to_out(v_0, xs_0, ys_0, mul_scratch);
    if s > t {
        // size: s, t
        limbs_mul_greater_to_out(v_inf, xs_3, ys_3, mul_scratch);
    } else {
        // size: s, s
        limbs_mul_same_length_to_out(v_inf, xs_3, ys_3, mul_scratch);
    }
    split_into_chunks_mut!(scratch, m, [v_2, v_neg_2, v_half], remainder);
    let (v_neg_1, scratch_hi) = remainder.split_at_mut(m + 1);
    limbs_mul_toom_interpolate_7_points(
        out,
        n,
        s + t,
        w1_neg,
        v_neg_2,
        w3_neg,
        &mut v_neg_1[..m],
        v_2,
        v_half,
        scratch_hi,
    );
}}

// This function can be used to determine whether the sizes of the input slices to
// `limbs_mul_greater_to_out_toom_52` are valid.
//
// # Worst-case complexity
// Constant time and additional memory.
pub_test! {
    limbs_mul_greater_to_out_toom_52_input_sizes_valid(xs_len: usize, ys_len: usize) -> bool {
    xs_len + 4 < 5 * ys_len
        && xs_len.shr_round(2, Ceiling).0
            > ys_len.shr_round(1, Ceiling).0
        && xs_len + ys_len >= 5 * (xs_len.div_round(5, Ceiling).0 + 1)
}}

// This function can be used to determine the length of the input `scratch` slice in
// `limbs_mul_greater_to_out_toom_52`.
//
// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `mpn_toom52_mul_itch` from `gmp-impl.h`, GMP 6.2.1.
pub_test! {limbs_mul_greater_to_out_toom_52_scratch_len(
    xs_len: usize,
    ys_len: usize
) -> usize {
    let n = 1 + if xs_len << 1 >= 5 * ys_len {
        (xs_len - 1) / 5
    } else {
        (ys_len - 1) >> 1
    };
    let s = xs_len - (n << 2);
    let t = ys_len - n;
    let m = n + 1;
    assert!(m < xs_len);
    6 * n
        + 4
        + max!(
            limbs_mul_same_length_to_out_scratch_len(n),
            limbs_mul_same_length_to_out_scratch_len(m),
            limbs_mul_greater_to_out_scratch_len(m, n),
            limbs_mul_to_out_scratch_len(s, t)
        )
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
// the `xs.len() + ys.len()` least-significant limbs of the product of the `Natural`s to an output
// slice. A scratch slice is provided for the algorithm to use. An upper bound for the number of
// scratch limbs needed is provided by `limbs_mul_greater_to_out_toom_52_scratch_len`. The following
// restrictions on the input slices must be met:
// - `out`.len() >= `xs`.len() + `ys`.len()
// - ceiling(`xs`.len() / 4) > ceiling(`ys`.len() / 2)
// - `xs`.len() + 4 < 5 * `ys`.len()
// - `xs`.len() + `ys`.len() >= 5 * (ceiling(`xs`.len() / 5) + 1)
//
// Approximately, 2 * `ys`.len() < `xs`.len() < 5 * `ys`.len().
//
// The smallest allowable `xs` length is 14. The smallest allowable `ys` length is 5.
//
// This uses the Toom-52 algorithm.
//
// Evaluate in: -2, -1, 0, 1, 2, Infinity.
// ```
// <-s--><--n--><--n--><--n--><--n-->
//  _________________________________
// |xs4_|__xs3_|__xs2_|__xs1_|__xs0_|
//                       |ys1|__ys0_|
//                       <-t-><--n-->
//
// v_0     =  xs0                               * ys0          # X(0)   * Y(0)
// v_1     = (xs0 +  xs1 +  xs2 +  xs3 +   xs4) * (ys0 +  ys1) # X(1)   * Y(1)   xh  <= 4   yh <= 1
// v_neg_1 = (xs0 -  xs1 +  xs2 -  xs3 +   xs4) * (ys0 -  ys1) # X(-1)  * Y(-1) |xh| <= 2   yh  = 0
// v_2     = (xs0 + 2xs1 + 4xs2 + 8xs3 + 16xs4) * (ys0 + 2ys1) # X(2)   * Y(2)   xh  <= 30  yh <= 2
// v_neg_2 = (xs0 - 2xs1 + 4xs2 - 8xs3 + 16xs4) * (ys0 - 2ys1) # X(-2)  * Y(-2) |xh| <= 20 |yh|<= 1
// v_inf   =                               xs4  *         ys1  # X(inf) * Y(inf)
// ```
//
// Some slight optimization in evaluation are taken from the paper: "Towards Optimal Toom-Cook
// Multiplication for Univariate and Multivariate Polynomials in Characteristic 2 and 0."
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// The time would be O(n^{\log_5 6}) \approx O(n^{1.113}), but this function calls
// `limbs_mul_same_length_to_out`.
//
// # Panics
// May panic if the input slice conditions are not met.
//
// This is equivalent to `mpn_toom52_mul` from `mpn/generic/toom52_mul.c`, GMP 6.2.1.
pub_test! {limbs_mul_greater_to_out_toom_52(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);
    let n = 1 + if xs_len << 1 >= 5 * ys_len {
        (xs_len - 1) / 5
    } else {
        (ys_len - 1) >> 1
    };
    let (ys_0, ys_1) = ys.split_at(n);
    let t = ys_1.len();
    assert_ne!(t, 0);
    assert!(t <= n);
    let s = xs_len - (n << 2);
    assert_ne!(s, 0);
    assert!(s <= n);
    // Ensures that 5 values of n + 1 limbs each fits in the product area. Borderline cases are
    // xs_len = 32, ys_len = 8, n = 7, and xs_len = 36, ys_len = 9, n = 8.
    assert!(s + t >= 5);
    // - Scratch need is 6 * n + 4. We need one extra limb, because products will overwrite 2 * n +
    //   2 limbs.
    // - Compute as2 and asm2.
    let mut v_neg_1_neg = false;
    let m = n + 1;
    let (scratch, mul_scratch) = scratch.split_at_mut(6 * n + 4);
    split_into_chunks_mut!(out, m, [bs1, bsm2, bs2, as2, as1], _unused);
    let (v_neg_1, scratch_hi) = scratch.split_at_mut(m << 1);
    split_into_chunks_mut!(scratch_hi, m, [bsm1, asm1, asm2], _unused);
    let bsm1 = &mut bsm1[..n];
    let mut v_neg_2_neg = limbs_mul_toom_evaluate_poly_in_2_and_neg_2(as2, asm2, 4, xs, n, asm1);
    // Compute bs1 and bsm1.
    let (bs1_last, bs1_init) = bs1.split_last_mut().unwrap();
    *bs1_last = 0;
    if t == n {
        if limbs_add_same_length_to_out(bs1_init, ys_0, ys_1) {
            *bs1_last = 1;
        }
        if limbs_cmp_same_length(ys_0, ys_1) == Less {
            limbs_sub_same_length_to_out(bsm1, ys_1, ys_0);
            v_neg_1_neg = true;
        } else {
            limbs_sub_same_length_to_out(bsm1, ys_0, ys_1);
        }
    } else {
        if limbs_add_to_out(bs1_init, ys_0, ys_1) {
            *bs1_last = 1;
        }
        let (ys_0_lo, ys_0_hi) = ys_0.split_at(t);
        if slice_test_zero(ys_0_hi) && limbs_cmp_same_length(ys_0_lo, ys_1) == Less {
            let (bsm1_lo, bsm1_hi) = bsm1.split_at_mut(t);
            limbs_sub_same_length_to_out(bsm1_lo, ys_1, ys_0_lo);
            slice_set_zero(bsm1_hi);
            v_neg_1_neg.not_assign();
        } else {
            limbs_sub_greater_to_out(bsm1, ys_0, ys_1);
        }
    }
    // Compute bs2 and bsm2, recycling bs1 and bsm1. bs2 = bs1 + ys_1; bsm2 = bsm1 - ys_1
    limbs_add_to_out(bs2, bs1, ys_1);
    let (bsm2_last, bsm2_init) = bsm2.split_last_mut().unwrap();
    *bsm2_last = 0;
    if v_neg_1_neg {
        if limbs_add_to_out(bsm2_init, bsm1, ys_1) {
            *bsm2_last = 1;
        }
        v_neg_2_neg.not_assign();
    } else if t == n {
        if limbs_cmp_same_length(bsm1, ys_1) == Less {
            limbs_sub_same_length_to_out(bsm2_init, ys_1, bsm1);
            v_neg_2_neg.not_assign();
        } else {
            limbs_sub_same_length_to_out(bsm2_init, bsm1, ys_1);
        }
    } else {
        let (bsm1_lo, bsm1_hi) = bsm1.split_at(t);
        if slice_test_zero(bsm1_hi) && limbs_cmp_same_length(bsm1_lo, ys_1) == Less {
            limbs_sub_same_length_to_out(bsm2_init, ys_1, bsm1_lo);
            slice_set_zero(&mut bsm2_init[t..]);
            v_neg_2_neg.not_assign();
        } else {
            limbs_sub_greater_to_out(bsm2_init, bsm1, ys_1);
        }
    }
    // Compute as1 and asm1.
    if limbs_mul_toom_evaluate_poly_in_1_and_neg_1(as1, asm1, 4, xs, n, &mut v_neg_1[..m]) {
        v_neg_1_neg.not_assign();
    }
    assert!(as1[n] <= 4);
    assert!(bs1[n] <= 1);
    assert!(asm1[n] <= 2);
    assert!(as2[n] <= 30);
    assert!(bs2[n] <= 2);
    assert!(asm2[n] <= 20);
    assert!(*bsm2_last <= 1);
    // size: m, n
    limbs_mul_greater_to_out(v_neg_1, asm1, bsm1, mul_scratch); // W4
    let k = (n << 1) + 1;
    let (v_neg_2, asm2) = scratch[k..].split_at_mut(k + 2);
    // size: m
    limbs_mul_same_length_to_out(v_neg_2, &asm2[..m], bsm2, mul_scratch); // W2
                                                                          // size: m
    limbs_mul_same_length_to_out(&mut scratch[k << 1..], as2, bs2, mul_scratch); // W1
    let (bs1, remainder) = out.split_at_mut(n << 1);
    let (v_1, as1) = remainder.split_at_mut(k + 3);
    // size: m
    limbs_mul_same_length_to_out(v_1, &as1[..m], &bs1[..m], mul_scratch); // W3
    let (v_0, v_inf) = out.split_at_mut(5 * n);
    // size: s, t
    limbs_mul_to_out(v_inf, &xs[n << 2..], ys_1, mul_scratch); // W0
                                                               // size: n
    limbs_mul_same_length_to_out(v_0, &xs[..n], ys_0, mul_scratch); // W5
    split_into_chunks_mut!(scratch, k, [v_neg_1, v_neg_2, v_2], _unused);
    limbs_mul_toom_interpolate_6_points(
        out,
        n,
        s + t,
        v_neg_1_neg,
        v_neg_1,
        v_neg_2_neg,
        v_neg_2,
        v_2,
    );
}}

// This function can be used to determine whether the sizes of the input slices to
// `limbs_mul_greater_to_out_toom_53` are valid.
//
// # Worst-case complexity
// Constant time and additional memory.
pub_test! {
    limbs_mul_greater_to_out_toom_53_input_sizes_valid(xs_len: usize, ys_len: usize) -> bool {
    !(xs_len == 16 && ys_len == 9)
        && xs_len.shr_round(2, Ceiling).0
            > ys_len.div_round(3, Ceiling).0
        && xs_len.div_round(5, Ceiling).0
            < ys_len.shr_round(1, Ceiling).0
}}

// This function can be used to determine the length of the input `scratch` slice in
// `limbs_mul_greater_to_out_toom_53`.
//
// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `mpn_toom53_mul_itch` from `gmp-impl.h`, GMP 6.2.1.
pub_crate_test! {limbs_mul_greater_to_out_toom_53_scratch_len(
    xs_len: usize,
    ys_len: usize
) -> usize {
    let n = 1 + if 3 * xs_len >= 5 * ys_len {
        (xs_len - 1) / 5
    } else {
        (ys_len - 1) / 3
    };
    let s = xs_len - (n << 2);
    let t = ys_len - (n << 1);
    let m = n + 1;
    assert!(m < xs_len);
    18 * n
        + 15
        + max!(
            limbs_mul_same_length_to_out_scratch_len(n),
            limbs_mul_same_length_to_out_scratch_len(m),
            limbs_mul_to_out_scratch_len(s, t)
        )
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
// the `xs.len() + ys.len()` least-significant limbs of the product of the `Natural`s to an output
// slice. A scratch slice is provided for the algorithm to use. An upper bound for the number of
// scratch limbs needed is provided by `limbs_mul_greater_to_out_toom_53_scratch_len`. The following
// restrictions on the input slices must be met:
// - `out`.len() >= `xs`.len() + `ys`.len()
// - ceiling(`xs`.len() / 4) > ceiling(`xs`.len() / 3)
// - ceiling(`xs`.len() / 5) < ceiling(`ys`.len() / 2)
// - (`xs`.len(), `ys`.len()) != (16, 9)
//
// Approximately, 4 / 3 * `ys`.len() < `xs`.len() < 5 / 2 * `ys`.len().
//
// The smallest allowable `xs` length is 5. The smallest allowable `ys` length is 3.
//
// This uses the Toom-53 algorithm.
//
// Evaluate in: 0, 1, -1, 2, -2, 1 / 2, Infinity.
// ```
// <-s-><--n--><--n--><--n--><--n-->
//  _________________________________
// |xs4_|__xs3_|__xs2_|__xs1_|__xs0_|
//               |ys2_|__ys1_|__ys0_|
//               <-t--><--n--><--n-->
//
// v_0  =       x0                   *   y0          #    X(0) * Y(0)
// v_1  =    (  x0+ x1+ x2+ x3+  x4) * ( y0+ y1+ y2) #    X(1) * Y(1)      xh  <= 4      yh <= 2
// v_neg_1 = (  x0- x1+ x2- x3+  x4) * ( y0- y1+ y2) #   X(-1) * Y(-1)    |xh| <= 2      yh <= 1
// v_2  =    (  x0+2x1+4x2+8x3+16x4) * ( y0+2y1+4y2) #    X(2) * Y(2)      xh  <= 30     yh <= 6
// v_neg_2 = (  x0-2x1+4x2-8x3+16x4) * ( y0-2y1+4y2) #    X(2) * Y(2)    -9<=xh<=20  -1<=yh <= 4
// v_half  = (16x0+8x1+4x2+2x3+  x4) * (4y0+2y1+ y2) #  X(1/2) * Y(1/2)    xh  <= 30     yh <= 6
// v_inf=                        x4  *           y2  #  X(inf) * Y(inf)
// ```
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// The time would be O(n^{\log_5 7}) \approx O(n^{1.209}), but this function calls
// `limbs_mul_same_length_to_out`.
//
// # Panics
// May panic if the input slice conditions are not met.
//
// This is equivalent to `mpn_toom53_mul` from `mpn/generic/toom53_mul.c`, GMP 6.2.1.
pub_crate_test! {limbs_mul_greater_to_out_toom_53(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);
    let n = 1 + if 3 * xs_len >= 5 * ys_len {
        (xs_len - 1) / 5
    } else {
        (ys_len - 1) / 3
    };
    split_into_chunks!(xs, n, [xs_0, xs_1, xs_2, xs_3], xs_4);
    let s = xs_4.len();
    assert_ne!(s, 0);
    assert!(s <= n);
    split_into_chunks!(ys, n, [ys_0, ys_1], ys_2);
    let t = ys_2.len();
    assert_ne!(t, 0);
    assert!(t <= n);
    let (scratch, mul_scratch) = scratch.split_at_mut(18 * n + 15);
    let (scratch2, scratch) = scratch.split_at_mut(10 * (n + 1));
    split_into_chunks_mut!(
        scratch2,
        n + 1,
        [as1, asm1, as2, asm2, ash, bs1, bsm1, bs2, bsm2, bsh],
        _unused
    );
    let out_lo = &mut out[..=n];
    // Compute as1 and asm1.
    let mut v_neg_1_neg = limbs_mul_toom_evaluate_poly_in_1_and_neg_1(as1, asm1, 4, xs, n, out_lo);
    // Compute as2 and asm2.
    let mut v_neg_2_neg = limbs_mul_toom_evaluate_poly_in_2_and_neg_2(as2, asm2, 4, xs, n, out_lo);
    // Compute ash = 16 * xs_0 + 8 * xs_1 + 4 * xs_2 + 2 * xs_3 + xs_4 = 2 * (2 * (2 * (2 * xs_0 +
    // xs_1) + xs_2) + xs_3) + xs_4
    let (ash_last, ash_init) = ash.split_last_mut().unwrap();
    let mut carry = limbs_shl_to_out(ash_init, xs_0, 1);
    if limbs_slice_add_same_length_in_place_left(ash_init, xs_1) {
        carry.wrapping_add_assign(1);
    }
    carry = carry.arithmetic_checked_shl(1).unwrap();
    carry.wrapping_add_assign(limbs_slice_shl_in_place(ash_init, 1));
    if limbs_slice_add_same_length_in_place_left(ash_init, xs_2) {
        carry.wrapping_add_assign(1);
    }
    carry = carry.arithmetic_checked_shl(1).unwrap();
    carry.wrapping_add_assign(limbs_slice_shl_in_place(ash_init, 1));
    if limbs_slice_add_same_length_in_place_left(ash_init, xs_3) {
        carry.wrapping_add_assign(1);
    }
    carry = carry.arithmetic_checked_shl(1).unwrap();
    carry.wrapping_add_assign(limbs_slice_shl_in_place(ash_init, 1));
    if limbs_slice_add_greater_in_place_left(ash_init, xs_4) {
        carry.wrapping_add_assign(1);
    }
    *ash_last = carry;
    // Compute bs1 and bsm1.
    let (bs1_last, bs1_init) = bs1.split_last_mut().unwrap();
    // ys_0 + ys_2
    *bs1_last = Limb::from(limbs_add_to_out(bs1_init, ys_0, ys_2));
    let (bsm1_last, bsm1_init) = bsm1.split_last_mut().unwrap();
    if *bs1_last == 0 && limbs_cmp_same_length(bs1_init, ys_1) == Less {
        limbs_sub_same_length_to_out(bsm1_init, ys_1, bs1_init);
        *bsm1_last = 0;
        v_neg_1_neg.not_assign();
    } else {
        *bsm1_last = *bs1_last;
        if limbs_sub_same_length_to_out(bsm1_init, bs1_init, ys_1) {
            bsm1_last.wrapping_sub_assign(1);
        }
    }
    // ys_0 + ys_1 + ys_2
    if limbs_slice_add_same_length_in_place_left(bs1_init, ys_1) {
        bs1_last.wrapping_add_assign(1);
    }
    // Compute bs2 and bsm2.
    let (out_lo_last, out_lo_init) = out_lo.split_last_mut().unwrap();
    let carry = limbs_shl_to_out(out_lo_init, ys_2, 2);
    let (bs2_last, bs2_init) = bs2.split_last_mut().unwrap();
    *bs2_last = Limb::from(limbs_add_to_out(bs2_init, ys_0, &out_lo_init[..t]));
    assert!(!limbs_slice_add_limb_in_place(&mut bs2[t..], carry));
    *out_lo_last = limbs_shl_to_out(out_lo_init, ys_1, 1);
    if limbs_cmp_same_length(bs2, out_lo) == Less {
        assert!(!limbs_sub_same_length_to_out(bsm2, out_lo, bs2));
        v_neg_2_neg.not_assign();
    } else {
        assert!(!limbs_sub_same_length_to_out(bsm2, bs2, out_lo));
    }
    limbs_slice_add_same_length_in_place_left(bs2, out_lo);
    // Compute bsh = 4 * ys_0 + 2 * ys_1 + ys_2 = 2 * (2 * ys_0 + ys_1) + ys_2.
    let (bsh_last, bsh_init) = bsh.split_last_mut().unwrap();
    let mut carry = limbs_shl_to_out(bsh_init, ys_0, 1);
    if limbs_slice_add_same_length_in_place_left(bsh_init, ys_1) {
        carry.wrapping_add_assign(1);
    }
    carry = carry.arithmetic_checked_shl(1).unwrap();
    carry.wrapping_add_assign(limbs_slice_shl_in_place(bsh_init, 1));
    if limbs_slice_add_greater_in_place_left(bsh_init, ys_2) {
        carry.wrapping_add_assign(1);
    }
    *bsh_last = carry;
    let (as1_last, as1_init) = as1.split_last().unwrap();
    let (asm1_last, asm1_init) = asm1.split_last().unwrap();
    assert!(*as1_last <= 4);
    assert!(*bs1_last <= 2);
    assert!(*asm1_last <= 2);
    assert!(*bsm1_last <= 1);
    assert!(as2[n] <= 30);
    assert!(bs2[n] <= 6);
    assert!(asm2[n] <= 20);
    assert!(bsm2[n] <= 4);
    assert!(*ash_last <= 30);
    assert!(*bsh_last <= 6);
    let (v_0, remainder) = out.split_at_mut(n << 1);
    let (v_1, v_inf) = remainder.split_at_mut(n << 2);
    // size: n + 1
    limbs_mul_same_length_to_out(scratch, as2, bs2, mul_scratch);
    let m = 2 * n + 1;
    // size: n + 1
    limbs_mul_same_length_to_out(&mut scratch[m..], asm2, bsm2, mul_scratch);
    // size: n + 1
    limbs_mul_same_length_to_out(&mut scratch[m << 1..], ash, bsh, mul_scratch);
    let v_neg_1 = &mut scratch[3 * m..m << 2];
    let (v_neg_1_last, v_neg_1_init) = v_neg_1.split_last_mut().unwrap();
    if SMALLER_RECURSION_TOOM_33_AND_53 {
        // size: n
        limbs_mul_same_length_to_out(v_neg_1_init, asm1_init, bsm1_init, mul_scratch);
        let v_neg_1_init = &mut v_neg_1_init[n..];
        let mut carry = match *asm1_last {
            1 => {
                let mut carry = *bsm1_last;
                if limbs_slice_add_same_length_in_place_left(v_neg_1_init, bsm1_init) {
                    carry.wrapping_add_assign(1);
                }
                carry
            }
            2 => (*bsm1_last << 1).wrapping_add(
                limbs_slice_add_mul_limb_same_length_in_place_left(v_neg_1_init, bsm1_init, 2),
            ),
            _ => 0,
        };
        if *bsm1_last != 0 && limbs_slice_add_same_length_in_place_left(v_neg_1_init, asm1_init) {
            carry.wrapping_add_assign(1);
        }
        *v_neg_1_last = carry;
        // size: n
        limbs_mul_same_length_to_out(v_1, as1_init, bs1_init, mul_scratch);
        split_into_chunks_mut!(v_1, n, [_unused, v_1_lo], v_1_hi);
        let mut carry = match *as1_last {
            1 => {
                let mut carry = *bs1_last;
                if limbs_slice_add_same_length_in_place_left(v_1_lo, bs1_init) {
                    carry.wrapping_add_assign(1);
                }
                carry
            }
            2 => (*bs1_last << 1).wrapping_add(limbs_slice_add_mul_limb_same_length_in_place_left(
                v_1_lo, bs1_init, 2,
            )),
            0 => 0,
            _ => {
                as1_last.wrapping_mul(*bs1_last)
                    + limbs_slice_add_mul_limb_same_length_in_place_left(
                        v_1_lo, bs1_init, *as1_last,
                    )
            }
        };
        if *bs1_last == 1 && limbs_slice_add_same_length_in_place_left(v_1_lo, as1_init) {
            carry.wrapping_add_assign(1);
        } else if *bs1_last == 2 {
            carry.wrapping_add_assign(limbs_slice_add_mul_limb_same_length_in_place_left(
                v_1_lo, as1_init, 2,
            ));
        }
        v_1_hi[0] = carry;
    } else {
        fail_on_untested_path("limbs_mul_greater_to_out_toom_53, !SMALLER_RECURSION");
        *v_neg_1_last = 0;
        if (*asm1_last | *bsm1_last) == 0 {
            // size: n
            limbs_mul_same_length_to_out(v_neg_1_init, asm1_init, bsm1_init, mul_scratch);
        } else {
            // size: n + 1
            limbs_mul_same_length_to_out(&mut scratch[3 * m..8 * n + 5], asm1, bsm1, mul_scratch);
        }
        v_1[n << 1] = 0;
        if (*as1_last | *bs1_last) == 0 {
            // size: n
            limbs_mul_same_length_to_out(v_1, as1_init, bs1_init, mul_scratch);
        } else {
            // size: n + 1
            limbs_mul_same_length_to_out(v_1, as1, bs1, mul_scratch);
        }
    }
    // size: n
    limbs_mul_same_length_to_out(v_0, xs_0, ys_0, mul_scratch); // v_0, 2 * n limbs
    // size: s, t
    limbs_mul_to_out(v_inf, xs_4, ys_2, mul_scratch); // v_inf, s + t limbs
    split_into_chunks_mut!(
        scratch,
        m,
        [v_2, v_neg_2, v_half, v_neg_1],
        _unused
    );
    limbs_mul_toom_interpolate_7_points(
        out,
        n,
        s + t,
        v_neg_2_neg,
        v_neg_2,
        v_neg_1_neg,
        v_neg_1,
        v_2,
        v_half,
        scratch2,
    );
}}

// This function can be used to determine whether the sizes of the input slices to
// `limbs_mul_greater_to_out_toom_54` are valid.
//
// # Worst-case complexity
// Constant time and additional memory.
pub_const_test! {limbs_mul_greater_to_out_toom_54_input_sizes_valid(
    xs_len: usize,
    ys_len: usize,
) -> bool {
    xs_len != 0 && xs_len >= ys_len && {
        let sum = xs_len + ys_len;
        let n = 1 + if xs_len << 2 >= 5 * ys_len {
            (xs_len - 1) / 5
        } else {
            (ys_len - 1) >> 2
        };
        n > 2
            && xs_len > n << 2
            && xs_len <= 5 * n
            && ys_len > 3 * n
            && ys_len <= n << 2
            && sum >= n << 3
            && sum > 7 * n + 4
    }
}}

// This function can be used to determine the length of the input `scratch` slice in
// `limbs_mul_greater_to_out_toom_54`.
//
// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `mpn_toom54_mul_itch` from `gmp-impl.h`, GMP 6.2.1.
pub_test! {limbs_mul_greater_to_out_toom_54_scratch_len(
    xs_len: usize,
    ys_len: usize
) -> usize {
    let n = 1
        + (if xs_len << 2 >= 5 * ys_len {
            (xs_len - 1) / 5
        } else {
            (ys_len - 1) >> 2
        });
    let m = n + 1;
    assert!(m < xs_len);
    let s = xs_len - (n << 2);
    let t = ys_len - 3 * n;
    6 * n
        + 2
        + max!(
            3 * n + 1,
            limbs_mul_same_length_to_out_scratch_len(n),
            limbs_mul_same_length_to_out_scratch_len(m),
            limbs_mul_to_out_scratch_len(s, t)
        )
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
// the `xs.len() + ys.len()` least-significant limbs of the product of the `Natural`s to an output
// slice. A scratch slice is provided for the algorithm to use. An upper bound for the number of
// scratch limbs needed is provided by `limbs_mul_greater_to_out_toom_54_scratch_len`. The following
// restrictions on the input slices must be met:
// - `out`.len() >= `xs`.len() + `ys`.len()
// - `xs`.len() >= `ys`.len()
// - Others; see `limbs_mul_greater_to_out_toom_54_input_sizes_valid`. The gist is that 3 times
//   `xs.len()` must be less than 5 times `ys.len()`.
//
// This uses the Toom-54 algorithm (the splitting unbalanced version).
//
// Evaluate in: Infinity, 4, -4, 2, -2, 1, -1, 0.
// ```
// <--s-><--n--><--n--><--n--><--n-->
//  _________________________________
// |xs4_|_xs3__|_xs2__|_xs1__|_xs0__|
//        |ys3_|_ys2__|_ys1__|_ys0__|
//         <-t-><--n--><--n--><--n-->
// ```
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// The time would be O(n^{\log_5 8}) \approx O(n^{1.292}), but `limbs_mul_same_length_to_out` is
// called.
//
// # Panics
// May panic if the input slice conditions are not met.
//
// This is equivalent to `mpn_toom54_mul` from `mpn/generic/toom54_mul.c`, GMP 6.2.1.
#[cfg(feature = "test_build")]
pub fn limbs_mul_greater_to_out_toom_54(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);
    // Decomposition
    let n = 1 + if xs_len << 2 >= 5 * ys_len {
        (xs_len - 1) / 5
    } else {
        (ys_len - 1) >> 2
    };
    assert!(n > 2);
    let m = n + 1;
    let a4 = &xs[n << 2..];
    let b3 = &ys[3 * n..];
    let s = a4.len();
    assert_ne!(s, 0);
    assert!(s <= n);
    let t = b3.len();
    assert_ne!(t, 0);
    assert!(t <= n);
    let sum = s + t;
    assert!(sum >= n);
    assert!(sum > 4);
    // Also allocate 3 * `n` + 1 limbs for `scratch_hi`. `limbs_mul_toom_interpolate_8_points` may
    // need all of them, when `shl_and_sub_same_length` uses a scratch.
    let (scratch, mul_scratch) = scratch.split_at_mut(6 * n + 2);
    let (r7, r3) = scratch.split_at_mut(3 * n + 1);
    let (out_lo, out_hi) = out.split_at_mut(3 * n);
    split_into_chunks_mut!(out_hi, m, [v0, v1, v2, v3], _unused);
    // - Evaluation and recursive calls
    // - 4, -4
    let out_lo_lo = &mut out_lo[..m];
    let neg_2_pow_sign =
        limbs_mul_toom_evaluate_poly_in_2_pow_and_neg_2_pow(v2, v0, 4, xs, n, 2, out_lo_lo)
            != limbs_mul_toom_evaluate_poly_in_2_pow_and_neg_2_pow(v3, v1, 3, ys, n, 2, out_lo_lo);
    // - X(-4) * Y(-4)
    // - size: m
    limbs_mul_same_length_to_out(out_lo, v0, v1, mul_scratch);
    // - X(+4) * Y(+4)
    // - size: m
    limbs_mul_same_length_to_out(r3, v2, v3, mul_scratch);
    limbs_toom_couple_handling(r3, &mut out_lo[..=n << 1], neg_2_pow_sign, n, 2, 4);
    // 1, -1
    let out_lo_lo = &mut out_lo[..m];
    let neg_1_sign = limbs_mul_toom_evaluate_poly_in_1_and_neg_1(v2, v0, 4, xs, n, out_lo_lo)
        != limbs_mul_toom_evaluate_deg_3_poly_in_1_and_neg_1(v3, v1, ys, n, out_lo_lo);
    // - X(-1) * Y(-1)
    // - size: m
    limbs_mul_same_length_to_out(out_lo, v0, v1, mul_scratch);
    // - X(1) * Y(1)
    // - size: m
    limbs_mul_same_length_to_out(r7, v2, v3, mul_scratch);
    limbs_toom_couple_handling(r7, &mut out_lo[..=n << 1], neg_1_sign, n, 0, 0);
    // 2, -2
    let out_lo_lo = &mut out_lo[..m];
    let neg_2_sign = limbs_mul_toom_evaluate_poly_in_2_and_neg_2(v2, v0, 4, xs, n, out_lo_lo)
        != limbs_mul_toom_evaluate_deg_3_poly_in_2_and_neg_2(v3, v1, ys, n, out_lo_lo);
    // - X(-2) * Y(-2)
    // - size: m
    limbs_mul_same_length_to_out(out_lo, v0, v1, mul_scratch);
    let (r5, remainder) = out[3 * n..].split_at_mut(m << 1);
    split_into_chunks_mut!(remainder, m, [v2, v3], _unused);
    // - X(2) * Y(2)
    // - size: m
    limbs_mul_same_length_to_out(r5, v2, v3, mul_scratch);
    let (out_lo, r5) = out.split_at_mut(3 * n);
    limbs_toom_couple_handling(r5, &mut out_lo[..=n << 1], neg_2_sign, n, 1, 2);
    // - X(0) * Y(0)
    // - size: n
    limbs_mul_same_length_to_out(out, &xs[..n], &ys[..n], mul_scratch);
    // - Infinity
    // - size: s, t
    limbs_mul_to_out(&mut out[7 * n..], a4, b3, mul_scratch);
    limbs_mul_toom_interpolate_8_points(out, n, sum, r3, r7, mul_scratch);
}

// This function can be used to determine whether the sizes of the input slices to
// `limbs_mul_greater_to_out_toom_62` are valid.
//
// # Worst-case complexity
// Constant time and additional memory.
pub_const_test! {limbs_mul_greater_to_out_toom_62_input_sizes_valid(
    xs_len: usize,
    ys_len: usize,
) -> bool {
    xs_len != 0 && xs_len >= ys_len && {
        let n = 1 + if xs_len >= 3 * ys_len {
            (xs_len - 1) / 6
        } else {
            (ys_len - 1) >> 1
        };
        xs_len > 5 * n && xs_len <= 6 * n && ys_len > n && ys_len <= n << 1
    }
}}

// This function can be used to determine the length of the input `scratch` slice in
// `limbs_mul_greater_to_out_toom_62`.
//
// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `mpn_toom62_mul_itch` from `gmp-impl.h`, GMP 6.2.1.
pub_test! {limbs_mul_greater_to_out_toom_62_scratch_len(
    xs_len: usize,
    ys_len: usize
) -> usize {
    let n = 1 + if xs_len >= 3 * ys_len {
        (xs_len - 1) / 6
    } else {
        (ys_len - 1) >> 1
    };
    let s = xs_len - 5 * n;
    let t = ys_len - n;
    let m = n + 1;
    assert!(m < xs_len);
    20 * n
        + 14
        + max!(
            limbs_mul_same_length_to_out_scratch_len(n),
            limbs_mul_same_length_to_out_scratch_len(m),
            limbs_mul_to_out_scratch_len(s, t)
        )
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
// the `xs.len() + ys.len()` least-significant limbs of the product of the `Natural`s to an output
// slice. A scratch slice is provided for the algorithm to use. An upper bound for the number of
// scratch limbs needed is provided by `limbs_mul_greater_to_out_toom_62_scratch_len`. The following
// restrictions on the input slices must be met:
// - `out`.len() >= `xs`.len() + `ys`.len()
// - `xs`.len() >= `ys`.len()
// - See `limbs_mul_greater_to_out_toom_62_input_sizes_valid`. The gist is that `xs.len()` must be
//   less than 6 times `ys.len()`.
//
// This uses the Toom-62 algorithm.
//
// Evaluate in: 0, 1, -1, 2, -2, 1 / 2, Infinity.
// ```
// <-s-><--n--><--n--><--n--><--n--><--n-->
//  ________________________________________
// |xs5_|__xs4_|__xs3_|__xs2_|__xs1_|__xs0_|
//                             |ys1_|__ys0_|
//                             <-t--><--n-->
//
// v_0     =    x0                         *    y0     #   X(0) * Y(0)
// v_1     = (  x0+  x1+ x2+ x3+  x4+  x5) * ( y0+ y1) #   X(1) * Y(1)        xh <= 5      yh <= 1
// v_neg_1 = (  x0-  x1+ x2- x3+  x4-  x5) * ( y0- y1) #  X(-1) * Y(-1)      |xh|<= 2      yh =  0
// v_2     = (  x0+ 2x1+4x2+8x3+16x4+32x5) * ( y0+2y1) #   X(2) * Y(2)        xh <= 62     yh <= 2
// v_neg_2 = (  x0- 2x1+4x2-8x3+16x4-32x5) * ( y0-2y1) #  X(-2) * Y(-2)  -41<=xh <= 20 -1<=yh <= 0
// v_half  = (32x0+16x1+8x2+4x3+ 2x4+  x5) * (2y0+ y1) # X(1/2) * Y(1/2)      xh <= 62     yh <= 2
// v_inf   =                           x5  *       y1  # X(inf) * Y(inf)
// ```
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// The time would be O(n^{\log_6 7}) \approx O(n^{1.086}), but this function calls
// `limbs_mul_same_length_to_out`.
//
// # Panics
// May panic if the input slice conditions are not met.
//
// This is equivalent to `mpn_toom62_mul` from `mpn/generic/toom62_mul.c`, GMP 6.2.1.
pub_test! {limbs_mul_greater_to_out_toom_62(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);
    let n = 1 + if xs_len >= 3 * ys_len {
        (xs_len - 1) / 6
    } else {
        (ys_len - 1) >> 1
    };
    split_into_chunks!(xs, n, [xs_0, xs_1, xs_2, xs_3, xs_4], xs_5);
    let s = xs_5.len();
    assert_ne!(s, 0);
    assert!(s <= n);
    split_into_chunks!(ys, n, [ys_0], ys_1);
    let t = ys_1.len();
    assert_ne!(t, 0);
    assert!(t <= n);
    let m = n + 1;
    let (scratch, mul_scratch) = scratch.split_at_mut(20 * n + 14);
    let (scratch2, scratch) = scratch.split_at_mut(10 * n + 9);
    split_into_chunks_mut!(
        scratch2,
        m,
        [as1, asm1, as2, asm2, ash, bs1, bs2, bsm2, bsh],
        bsm1
    );
    // Compute as1 and asm1.
    let out_lo = &mut out[..m];
    let v_neg_1_neg_a = limbs_mul_toom_evaluate_poly_in_1_and_neg_1(as1, asm1, 5, xs, n, out_lo);
    // Compute as2 and asm2.
    let v_neg_2_neg_a = limbs_mul_toom_evaluate_poly_in_2_and_neg_2(as2, asm2, 5, xs, n, out_lo);
    let (ash_last, ash_init) = ash.split_last_mut().unwrap();
    // Compute ash = 32 * xs_0 + 16 * xs_1 + 8 * xs_2 + 4 * xs_3 + 2 * xs_4 + xs_5 = 2 * (2 * (2 *
    // (2 * (2 * xs_0 + xs_1) + xs_2) + xs_3) + xs_4) + xs_5
    let mut carry = limbs_shl_to_out(ash_init, xs_0, 1);
    for xs_i in &[xs_1, xs_2, xs_3, xs_4] {
        if limbs_slice_add_same_length_in_place_left(ash_init, xs_i) {
            carry += 1;
        }
        carry <<= 1;
        carry |= limbs_slice_shl_in_place(ash_init, 1);
    }
    *ash_last = carry;
    if limbs_slice_add_greater_in_place_left(ash_init, xs_5) {
        *ash_last += 1;
    }
    // Compute bs1 and bsm1.
    let (bs1_last, bs1_init) = bs1.split_last_mut().unwrap();
    let v_neg_1_neg_b = if t == n {
        *bs1_last = Limb::from(limbs_add_same_length_to_out(bs1_init, ys_0, ys_1));
        if limbs_cmp_same_length(ys_0, ys_1) == Less {
            limbs_sub_same_length_to_out(bsm1, ys_1, ys_0);
            true
        } else {
            limbs_sub_same_length_to_out(bsm1, ys_0, ys_1);
            false
        }
    } else {
        *bs1_last = Limb::from(limbs_add_to_out(bs1_init, ys_0, ys_1));
        let (ys_0_lo, ys_0_hi) = ys_0.split_at(t);
        if slice_test_zero(ys_0_hi) && limbs_cmp_same_length(ys_0_lo, ys_1) == Less {
            let (bsm1_lo, bsm1_hi) = bsm1.split_at_mut(t);
            limbs_sub_same_length_to_out(bsm1_lo, ys_1, ys_0_lo);
            slice_set_zero(bsm1_hi);
            true
        } else {
            limbs_sub_greater_to_out(bsm1, ys_0, ys_1);
            false
        }
    };
    // Compute bs2 and bsm2. Recycling bs1 and bsm1: bs2 = bs1 + ys_1, bsm2 = bsm1 - ys_1
    limbs_add_to_out(bs2, bs1, ys_1);
    let v_neg_2_neg_b = if v_neg_1_neg_b {
        bsm2[n] = Limb::from(limbs_add_to_out(bsm2, bsm1, ys_1));
        true
    } else if t < n {
        let (bsm1_lo, bsm1_hi) = bsm1.split_at(t);
        if slice_test_zero(bsm1_hi) && limbs_cmp_same_length(bsm1_lo, ys_1) == Less {
            let (bsm2_lo, bsm2_hi) = bsm2.split_at_mut(t);
            assert!(!limbs_sub_same_length_to_out(bsm2_lo, ys_1, bsm1_lo));
            slice_set_zero(bsm2_hi);
            true
        } else {
            assert!(!limbs_sub_greater_to_out(bsm2, bsm1, ys_1));
            bsm2[n] = 0;
            false
        }
    } else {
        bsm2[n] = 0;
        if limbs_cmp_same_length(bsm1, ys_1) == Less {
            assert!(!limbs_sub_same_length_to_out(bsm2, ys_1, bsm1));
            true
        } else {
            assert!(!limbs_sub_same_length_to_out(bsm2, bsm1, ys_1));
            false
        }
    };
    // Compute bsh, recycling bs1. bsh = bs1 + ys_0
    let (bs1_last, bs1_init) = bs1.split_last_mut().unwrap();
    let (bsh_last, bsh_init) = bsh.split_last_mut().unwrap();
    *bsh_last = *bs1_last;
    if limbs_add_same_length_to_out(bsh_init, bs1_init, ys_0) {
        bsh_last.wrapping_add_assign(1);
    }
    assert!(as1[n] <= 5);
    assert!(*bs1_last <= 1);
    assert!(asm1[n] <= 2);
    assert!(as2[n] <= 62);
    assert!(bs2[n] <= 2);
    assert!(asm2[n] <= 41);
    assert!(bsm2[n] <= 1);
    assert!(*ash_last <= 62);
    assert!(*bsh_last <= 2);
    let (as1_last, as1_init) = as1.split_last_mut().unwrap();
    let (asm1_last, asm1_init) = asm1.split_last_mut().unwrap();
    let (bs1_last, bs1_init) = bs1.split_last_mut().unwrap();
    let p = 2 * n + 1;
    // size: m
    limbs_mul_same_length_to_out(scratch, as2, bs2, mul_scratch);
    // size: m
    limbs_mul_same_length_to_out(&mut scratch[p..], asm2, bsm2, mul_scratch);
    // size: m
    limbs_mul_same_length_to_out(&mut scratch[p << 1..], ash, bsh, mul_scratch);
    split_into_chunks_mut!(
        scratch,
        p,
        [v_2, v_neg_2, v_half, v_neg_1, scratch_out],
        _unused
    );
    // size: n
    limbs_mul_same_length_to_out(v_neg_1, asm1_init, bsm1, mul_scratch);
    let (v_neg_1_last, v_neg_1_hi) = v_neg_1[n..p].split_last_mut().unwrap();
    *v_neg_1_last = if *asm1_last == 2 {
        limbs_slice_add_mul_limb_same_length_in_place_left(v_neg_1_hi, bsm1, 2)
    } else {
        Limb::from(
            *asm1_last == 1 && limbs_slice_add_same_length_in_place_left(v_neg_1_hi, bsm1),
        )
    };
    let (v_0, remainder) = out.split_at_mut(n << 1);
    let (v_1, v_inf) = remainder.split_at_mut(n << 2);
    // size: n
    limbs_mul_same_length_to_out(v_1, as1_init, bs1_init, mul_scratch);
    let (v_1_last, v_1_hi) = v_1[n..p].split_last_mut().unwrap();
    *v_1_last = match *as1_last {
        1 => {
            let mut carry = *bs1_last;
            if limbs_slice_add_same_length_in_place_left(v_1_hi, bs1_init) {
                carry.wrapping_add_assign(1);
            }
            carry
        }
        2 => bs1_last.arithmetic_checked_shl(1u64).unwrap().wrapping_add(
            limbs_slice_add_mul_limb_same_length_in_place_left(v_1_hi, bs1_init, 2),
        ),
        0 => 0,
        _ => as1_last.wrapping_mul(*bs1_last).wrapping_add(
            limbs_slice_add_mul_limb_same_length_in_place_left(v_1_hi, bs1_init, *as1_last),
        ),
    };
    if *bs1_last != 0 && limbs_slice_add_same_length_in_place_left(v_1_hi, as1_init) {
        v_1_last.wrapping_add_assign(1);
    }
    // size: n
    limbs_mul_same_length_to_out(v_0, xs_0, ys_0, mul_scratch);
    // size: s, t
    limbs_mul_to_out(v_inf, xs_5, ys_1, mul_scratch);
    limbs_mul_toom_interpolate_7_points(
        out,
        n,
        s + t,
        v_neg_2_neg_a != v_neg_2_neg_b,
        v_neg_2,
        v_neg_1_neg_a != v_neg_1_neg_b,
        v_neg_1,
        v_2,
        v_half,
        scratch_out,
    );
}}

// Stores |{xs,n}-{ys,n}| in {out,n}, returns the sign.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `abs_sub_n` from `mpn/generic/toom63_mul.c`, GMP 6.2.1.
fn limbs_abs_sub_same_length(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) -> bool {
    let n = xs.len();
    assert_eq!(ys.len(), n);
    for i in (0..n).rev() {
        let x = xs[i];
        let y = ys[i];
        if x != y {
            let n = i + 1;
            return if x > y {
                limbs_sub_same_length_to_out(out, &xs[..n], &ys[..n]);
                false
            } else {
                limbs_sub_same_length_to_out(out, &ys[..n], &xs[..n]);
                true
            };
        }
        out[i] = 0;
    }
    false
}

// Given the limbs `xs` and `ys` of two integers, writes the limbs of the absolute difference to
// `out_diff` and the limbs of the sum to `xs`.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `abs_sub_add_n` from `mpn/generic/toom63_mul.c`, GMP 6.2.1.
fn limbs_abs_sub_add_same_length(out_diff: &mut [Limb], xs: &mut [Limb], ys: &[Limb]) -> bool {
    let result = limbs_abs_sub_same_length(out_diff, xs, ys);
    assert!(!limbs_slice_add_same_length_in_place_left(xs, ys));
    result
}

// This function can be used to determine whether the sizes of the input slices to
// `limbs_mul_greater_to_out_toom_63` are valid.
//
// # Worst-case complexity
// Constant time and additional memory.
pub_const_test! {limbs_mul_greater_to_out_toom_63_input_sizes_valid(
    xs_len: usize,
    ys_len: usize,
) -> bool {
    xs_len != 0 && xs_len >= ys_len && {
        let sum = xs_len + ys_len;
        let n = 1 + if xs_len >= ys_len << 1 {
            (xs_len - 1) / 6
        } else {
            (ys_len - 1) / 3
        };
        n > 2
            && xs_len > 5 * n
            && xs_len <= 6 * n
            && ys_len > n << 1
            && ys_len <= 3 * n
            && sum >= n << 3
            && sum > 7 * n + 4
    }
}}

// This function can be used to determine the length of the input `scratch` slice in
// `limbs_mul_greater_to_out_toom_63`.
//
// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `mpn_toom63_mul_itch` from `gmp-impl.h`, GMP 6.2.1.
pub_crate_test! {limbs_mul_greater_to_out_toom_63_scratch_len(
    xs_len: usize,
    ys_len: usize
) -> usize {
    let n = 1 + if xs_len >= ys_len << 1 {
        (xs_len - 1) / 6
    } else {
        (ys_len - 1) / 3
    };
    let m = n + 1;
    assert!(m < xs_len);
    let s = xs_len - 5 * n;
    let t = ys_len - (n << 1);
    9 * n
        + 3
        + max!(
            limbs_mul_same_length_to_out_scratch_len(n),
            limbs_mul_same_length_to_out_scratch_len(m),
            limbs_mul_to_out_scratch_len(s, t)
        )
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
// the `xs.len() + ys.len()` least-significant limbs of the product of the `Natural`s to an output
// slice. A scratch slice is provided for the algorithm to use. An upper bound for the number of
// scratch limbs needed is provided by `limbs_mul_greater_to_out_toom_63_scratch_len`. The following
// restrictions on the input slices must be met:
// - `out`.len() >= `xs`.len() + `ys`.len()
// - `xs`.len() >= `ys`.len()
// - Others; see `limbs_mul_greater_to_out_toom_63_input_sizes_valid`. The gist is that
// `xs.len()` must be less than 3 times `ys.len()`.
//
// This uses the Toom-63 algorithm (aka Toom-4.5, the splitting 6x3 unbalanced version).
//
// Evaluate in: Infinity, 4, -4, 2, -2, 1, -1, 0.
// ```
// <--s-><--n--><--n--><--n--><--n--><--n-->
//  ________________________________________
// |xs5_|_xs4__|_xs3__|_xs2__|_xs1__|_xs0__|
//                      |ys2_|_ys1__|_ys0__|
//                      <--t-><--n--><--n-->
// ```
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// The time would be O(n^{\log_6 8}) \approx O(n^{1.161}), but
// `limbs_mul_same_length_to_out_toom_63_recursive` calls `limbs_mul_same_length_to_out`.
//
// # Panics
// May panic if the input slice conditions are not met.
//
// This is equivalent to `mpn_toom63_mul` from `mpn/generic/toom63_mul.c`, GMP 6.2.1.
pub_crate_test! {limbs_mul_greater_to_out_toom_63(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);
    let n = 1 + if xs_len >= ys_len << 1 {
        (xs_len - 1) / 6
    } else {
        (ys_len - 1) / 3
    };
    assert!(n > 2);
    let m = n + 1;
    // Decomposition
    split_into_chunks!(ys, n, [ys_0, ys_1], ys_2);
    let t = ys_2.len();
    assert_ne!(t, 0);
    assert!(t <= n);
    let s = xs_len - 5 * n;
    assert_ne!(s, 0);
    assert!(s <= n);
    assert!(s + t >= n);
    assert!(s + t > 4);
    // - Also allocate 3 * n + 1 limbs for `scratch2`. `limbs_mul_toom_interpolate_8_points` may
    //   need all of them, when `shl_and_sub_same_length` uses a scratch.
    // - Evaluation and recursive calls
    // - 4, -4
    let (scratch, mul_scratch) = scratch.split_at_mut(9 * n + 3);
    split_into_chunks_mut!(scratch, 3 * n + 1, [r7, r3], scratch2);
    let (r8, remainder) = out.split_at_mut(3 * n);
    split_into_chunks_mut!(remainder, m, [v0, v1, v2, v3], _unused);
    let r8_lo = &mut r8[..m];
    let mut v_neg_2_neg =
        limbs_mul_toom_evaluate_poly_in_2_pow_and_neg_2_pow(v2, v0, 5, xs, n, 2, r8_lo);
    let (r8_lo_last, r8_lo_init) = r8_lo.split_last_mut().unwrap();
    *r8_lo_last = limbs_shl_to_out(r8_lo_init, ys_1, 2); // 4 * ys_1
    v3[t] = limbs_shl_to_out(v3, ys_2, 4); // 16 * ys_2
    let (v3_last, v3_init) = v3.split_last_mut().unwrap();
    if n != t {
        // 16 * ys_2 + ys_0
        *v3_last = Limb::from(limbs_add_to_out_aliased(v3_init, t + 1, ys_0));
    } else if limbs_slice_add_same_length_in_place_left(v3_init, ys_0) {
        // 16 * ys_2 + ys_0
        *v3_last += 1;
    }
    if limbs_abs_sub_add_same_length(v1, v3, r8_lo) {
        v_neg_2_neg.not_assign();
    }
    // - A(-4) * B(-4)
    // - size: m
    limbs_mul_same_length_to_out(r8, v0, v1, mul_scratch);
    // - A(+4) * B(+4)
    // - size: m
    limbs_mul_same_length_to_out(r3, v2, v3, mul_scratch);
    limbs_toom_couple_handling(r3, &mut r8[..=n<<1], v_neg_2_neg, n, 2, 4);
    // $pm1$
    v_neg_2_neg = limbs_mul_toom_evaluate_poly_in_1_and_neg_1(v2, v0, 5, xs, n, &mut r8[..m]);
    // Compute bs1 and bsm1. Code taken from toom33
    let scratch2_lo = &mut scratch2[..n];
    let (v3_last, v3_init) = v3.split_last_mut().unwrap();
    *v3_last = 0;
    let carry = limbs_add_to_out(scratch2_lo, ys_0, ys_2);
    *v3_last = Limb::from(carry);
    if limbs_add_same_length_to_out(v3_init, scratch2_lo, ys_1) {
        *v3_last += 1;
    }
    let (v1_last, v1_init) = v1.split_last_mut().unwrap();
    if !carry && limbs_cmp_same_length(scratch2_lo, ys_1) == Less {
        limbs_sub_same_length_to_out(v1_init, ys_1, scratch2_lo);
        *v1_last = 0;
        v_neg_2_neg.not_assign();
    } else {
        *v1_last = Limb::from(carry);
        if limbs_sub_same_length_to_out(v1_init, scratch2_lo, ys_1) {
            v1_last.wrapping_sub_assign(1);
        }
    }
    // - A(-1) * B(-1)
    // - size: m
    limbs_mul_same_length_to_out(r8, v0, v1, mul_scratch);
    // - A(1) * B(1)
    // - size: m
    limbs_mul_same_length_to_out(r7, v2, v3, mul_scratch);
    limbs_toom_couple_handling(r7, &mut r8[..=n<<1], v_neg_2_neg, n, 0, 0);
    // 2, -2
    let r8_lo = &mut r8[..m];
    v_neg_2_neg = limbs_mul_toom_evaluate_poly_in_2_and_neg_2(v2, v0, 5, xs, n, r8_lo);
    let (r8_lo_last, r8_lo_init) = r8_lo.split_last_mut().unwrap();
    *r8_lo_last = limbs_shl_to_out(r8_lo_init, ys_1, 1); // 2 * ys_1
    v3[t] = limbs_shl_to_out(v3, ys_2, 2); // 4 * ys_2
    if n == t {
        // 4 * ys_2 + ys_0
        let (v3_last, v3_init) = v3.split_last_mut().unwrap();
        if limbs_slice_add_same_length_in_place_left(v3_init, ys_0) {
            v3_last.wrapping_add_assign(1);
        }
    } else {
        // 4 * ys_2 + ys_0
        v3[n] = Limb::from(limbs_add_to_out_aliased(v3, t + 1, ys_0));
    }
    if limbs_abs_sub_add_same_length(v1, v3, &r8[..m]) {
        v_neg_2_neg.not_assign();
    }
    // - A(-2) * B(-2)
    // - size: m
    limbs_mul_same_length_to_out(r8, v0, v1, mul_scratch);
    let (r8, r5) = out.split_at_mut(3 * n);
    let (r5_lo, remainder) = r5.split_at_mut(m << 1);
    split_into_chunks_mut!(remainder, m, [v2, v3], _unused);
    // - A(2) * B(2)
    // - size: m
    limbs_mul_same_length_to_out(r5_lo, v2, v3, mul_scratch);
    limbs_toom_couple_handling(r5, &mut r8[..=n<<1], v_neg_2_neg, n, 1, 2);
    // - A(0) * B(0)
    // - size: n
    limbs_mul_same_length_to_out(r8, &xs[..n], &ys[..n], mul_scratch);
    // Infinity
    let xs_5 = &xs[5 * n..];
    let r1 = &mut out[7 * n..];
    // size: s, t
    limbs_mul_to_out(r1, xs_5, ys_2, mul_scratch);
    limbs_mul_toom_interpolate_8_points(out, n, s + t, r3, r7, scratch2);
}}

// - The limit is a rational number between (12 / 11) ^ (log(4) / log(2 * 4 - 1)) and
// - (12 / 11) ^ (log(6) / log(2 * 6 - 1))
const TOOM_6H_LIMIT_NUMERATOR: usize = 18;
const TOOM_6H_LIMIT_DENOMINATOR: usize = 17;

// This function can be used to determine whether the sizes of the input slices to
// `limbs_mul_greater_to_out_toom_6h` are valid.
//
// # Worst-case complexity
// Constant time and additional memory.
pub_test! {limbs_mul_greater_to_out_toom_6h_input_sizes_valid(
    xs_len: usize,
    ys_len: usize
) -> bool {
    if xs_len == 0
        || xs_len < ys_len
        || ys_len < 42
        || xs_len * 3 >= ys_len << 3 && (ys_len < 46 || xs_len * 6 >= ys_len * 17)
    {
        return false;
    }
    let n;
    let mut half = false;
    let mut xl = xs_len;
    let mut yl = ys_len;
    let (xr, yr) = if xs_len * TOOM_6H_LIMIT_DENOMINATOR < TOOM_6H_LIMIT_NUMERATOR * ys_len {
        // xs.len() < 18 / 17 * ys.len()
        n = 1 + (xs_len - 1) / 6;
        let n5 = 5 * n;
        (n5, n5)
    } else {
        let (p, q) = if xs_len * 5 * TOOM_6H_LIMIT_NUMERATOR
            < TOOM_6H_LIMIT_DENOMINATOR * 7 * ys_len
        {
            (7, 6) // xs.len() < 119 / 90 * ys.len()
        } else if xs_len * 5 * TOOM_6H_LIMIT_DENOMINATOR < TOOM_6H_LIMIT_NUMERATOR * 7 * ys_len {
            (7, 5) // xs.len() < 126 / 85 * ys.len()
        } else if xs_len * TOOM_6H_LIMIT_NUMERATOR < (TOOM_6H_LIMIT_DENOMINATOR * ys_len) << 1 {
            (8, 5) // xs.len() < 17 / 9 * ys.len()
        } else if xs_len * TOOM_6H_LIMIT_DENOMINATOR < (TOOM_6H_LIMIT_NUMERATOR * ys_len) << 1 {
            (8, 4) // xs.len() < 36 / 17 * ys.len()
        } else {
            (9, 4) // xs.len() >= 36 / 17 * ys.len()
        };
        // With LIMIT = 16 / 15, the following recovery is needed only if `ys_len` <= 73.
        n = 1 + if q * xs_len >= p * ys_len {
            (xs_len - 1) / p
        } else {
            (ys_len - 1) / q
        };
        let pn = (p - 1) * n;
        let qn = (q - 1) * n;
        if !p.eq_mod_power_of_2(q, 1) {
            // Recover from-badly chosen splitting
            if xs_len <= pn {
                xl += n;
            } else if ys_len <= qn {
                yl += n;
            } else {
                half = true;
            }
        }
        (pn, qn)
    };
    xl > xr && yl > yr && {
        let (s, t) = (xl - xr, yl - yr);
        let limit = 12 * n + 6;
        s <= n
            && t <= n
            && (half || s + t > 3)
            && n > 2
            && limit <= limbs_mul_greater_to_out_toom_6h_scratch_len(xs_len, ys_len)
            && limit <= limbs_square_to_out_toom_6_scratch_len(n * 6)
    }
}}

// TODO tune
const TOOM_6H_MAYBE_MUL_BASECASE: bool =
    TUNE_PROGRAM_BUILD || MUL_TOOM6H_THRESHOLD < 6 * MUL_TOOM22_THRESHOLD;
// TODO tune
const TOOM_6H_MAYBE_MUL_TOOM22: bool =
    TUNE_PROGRAM_BUILD || MUL_TOOM6H_THRESHOLD < 6 * MUL_TOOM33_THRESHOLD;
const TOOM_6H_MAYBE_MUL_TOOM33: bool =
    TUNE_PROGRAM_BUILD || MUL_TOOM6H_THRESHOLD < 6usize.saturating_mul(MUL_TOOM44_THRESHOLD);
const TOOM_6H_MAYBE_MUL_TOOM6H: bool =
    TUNE_PROGRAM_BUILD || MUL_FFT_THRESHOLD >= 6 * MUL_TOOM6H_THRESHOLD;

fn limbs_mul_same_length_to_out_toom_6h_recursive_scratch_len(n: usize) -> usize {
    if TOOM_6H_MAYBE_MUL_BASECASE && n < MUL_TOOM22_THRESHOLD {
        0
    } else if TOOM_6H_MAYBE_MUL_TOOM22 && n < MUL_TOOM33_THRESHOLD {
        limbs_mul_greater_to_out_toom_22_scratch_len(n, n)
    } else if TOOM_6H_MAYBE_MUL_TOOM33 && n < MUL_TOOM44_THRESHOLD {
        limbs_mul_greater_to_out_toom_33_scratch_len(n, n)
    } else if !TOOM_6H_MAYBE_MUL_TOOM6H || n < MUL_TOOM6H_THRESHOLD {
        limbs_mul_greater_to_out_toom_44_scratch_len(n, n)
    } else {
        limbs_mul_greater_to_out_toom_6h_scratch_len(n, n)
    }
}

// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `TOOM6H_MUL_N_REC` from `mpn/generic/toom6h_mul.c`, GMP 6.2.1, when `f` is
// false.
fn limbs_mul_same_length_to_out_toom_6h_recursive(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let n = xs.len();
    assert_eq!(ys.len(), n);
    if TOOM_6H_MAYBE_MUL_BASECASE && n < MUL_TOOM22_THRESHOLD {
        limbs_mul_greater_to_out_basecase(out, xs, ys);
    } else if TOOM_6H_MAYBE_MUL_TOOM22 && n < MUL_TOOM33_THRESHOLD {
        limbs_mul_greater_to_out_toom_22(out, xs, ys, scratch);
    } else if TOOM_6H_MAYBE_MUL_TOOM33 && n < MUL_TOOM44_THRESHOLD {
        limbs_mul_greater_to_out_toom_33(out, xs, ys, scratch);
    } else if !TOOM_6H_MAYBE_MUL_TOOM6H || n < MUL_TOOM6H_THRESHOLD {
        limbs_mul_greater_to_out_toom_44(out, xs, ys, scratch);
    } else {
        limbs_mul_greater_to_out_toom_6h(out, xs, ys, scratch);
    }
}

// This function can be used to determine the length of the input `scratch` slice in
// `limbs_mul_greater_to_out_toom_6h`.
//
// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `mpn_toom6h_mul_itch` from `gmp-impl.h`, GMP 6.2.1.
pub_crate_test! {limbs_mul_greater_to_out_toom_6h_scratch_len(
    xs_len: usize,
    ys_len: usize
) -> usize {
    let n;
    let mut half = false;
    let (pn, qn) = if xs_len * TOOM_6H_LIMIT_DENOMINATOR < TOOM_6H_LIMIT_NUMERATOR * ys_len {
        // - xs.len() < 18 / 17 * ys.len()
        // - This is the slowest variation
        n = 1 + (xs_len - 1) / 6;
        let n5 = 5 * n;
        (n5, n5)
    } else {
        let (mut p, mut q) = if xs_len * 5 * TOOM_6H_LIMIT_NUMERATOR
            < TOOM_6H_LIMIT_DENOMINATOR * 7 * ys_len
        {
            // xs.len() < 119 / 90 * ys.len(), half
            (7, 6)
        } else if xs_len * 5 * TOOM_6H_LIMIT_DENOMINATOR < TOOM_6H_LIMIT_NUMERATOR * 7 * ys_len {
            // xs.len() < 126 / 85 * ys.len(), !half
            (7, 5)
        } else if xs_len * TOOM_6H_LIMIT_NUMERATOR < (TOOM_6H_LIMIT_DENOMINATOR * ys_len) << 1 {
            // xs.len() < 17 / 9 * ys.len(), half
            (8, 5)
        } else if xs_len * TOOM_6H_LIMIT_DENOMINATOR < (TOOM_6H_LIMIT_NUMERATOR * ys_len) << 1 {
            // xs.len() < 36 / 17 * ys.len(), !half
            (8, 4)
        } else {
            // xs.len() >= 36 / 17 * ys.len(), half
            (9, 4)
        };
        n = 1 + if q * xs_len >= p * ys_len {
            (xs_len - 1) / p
        } else {
            (ys_len - 1) / q
        };
        p -= 1;
        q -= 1;
        let mut pn = p * n;
        let mut qn = q * n;
        // With LIMIT = 16 / 15, the following recovery is needed only if `ys_len` <= 73.
        if !p.eq_mod_power_of_2(q, 1) {
            // Recover from badly-chosen splitting
            if xs_len <= pn {
                pn -= n;
            } else if ys_len <= qn {
                qn -= n;
            } else {
                half = true;
            }
        }
        (pn, qn)
    };
    let m = n + 1;
    assert!(m < xs_len);
    let s = xs_len - pn;
    let t = ys_len - qn;
    let mut mul_scratch_len = max!(
        3 * n - 1,
        limbs_mul_same_length_to_out_toom_6h_recursive_scratch_len(n),
        limbs_mul_same_length_to_out_toom_6h_recursive_scratch_len(m)
    );
    if half {
        mul_scratch_len = max(mul_scratch_len, limbs_mul_to_out_scratch_len(s, t));
    }
    10 * n + 4 + mul_scratch_len
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
// the `xs.len() + ys.len()` least-significant limbs of the product of the `Natural`s to an output
// slice. A scratch slice is provided for the algorithm to use. An upper bound for the number of
// scratch limbs needed is provided by `limbs_mul_greater_to_out_toom_6h_scratch_len`. The following
// restrictions on the input slices must be met:
// - `out`.len() >= `xs`.len() + `ys`.len()
// - `xs`.len() >= `ys`.len()
// - Others; see `limbs_mul_greater_to_out_toom_6h_input_sizes_valid`.
//
// This uses the Toom-6h algorithm (Toom-6.5).
//
// Evaluate in: Infinity, 4, -4, 2, -2, 1, -1, 1 / 2, -1 / 2, 1 / 4, -1 / 4, 0.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// The time would be O(n^{\log_5 11}) \approx O(n^{1.490}) (assuming worst-possible splitting), but
// `limbs_mul_same_length_to_out_toom_6h_recursive` ultimately calls `limbs_mul_greater_to_out`.
//
// # Panics
// May panic if the input slice conditions are not met.
//
// This is equivalent to `mpn_toom6h_mul` from `mpn/generic/toom6h_mul.c`, GMP 6.2.1.
pub_crate_test! {limbs_mul_greater_to_out_toom_6h(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);
    // - Decomposition
    // - Can not handle too much unbalance
    assert!(ys_len >= 42);
    assert!(xs_len * 3 < ys_len << 3 || ys_len >= 46 && xs_len * 6 < ys_len * 17);
    let n;
    let mut half = false;
    let (p, q, pn, qn) = if xs_len * TOOM_6H_LIMIT_DENOMINATOR < TOOM_6H_LIMIT_NUMERATOR * ys_len {
        // - xs.len() < 18 / 17 * ys.len()
        // - This is the slowest variation
        n = 1 + (xs_len - 1) / 6;
        let n5 = 5 * n;
        (5, 5, n5, n5)
    } else {
        let (mut p, mut q) = if xs_len * 5 * TOOM_6H_LIMIT_NUMERATOR
            < TOOM_6H_LIMIT_DENOMINATOR * 7 * ys_len
        {
            // xs.len() < 119 / 90 * ys.len(), half
            (7, 6)
        } else if xs_len * 5 * TOOM_6H_LIMIT_DENOMINATOR < TOOM_6H_LIMIT_NUMERATOR * 7 * ys_len {
            // xs.len() < 126 / 85 * ys.len(), !half
            (7, 5)
        } else if xs_len * TOOM_6H_LIMIT_NUMERATOR < (TOOM_6H_LIMIT_DENOMINATOR * ys_len) << 1 {
            // xs.len() < 17 / 9 * ys.len(), half
            (8, 5)
        } else if xs_len * TOOM_6H_LIMIT_DENOMINATOR < (TOOM_6H_LIMIT_NUMERATOR * ys_len) << 1 {
            // xs.len() < 36 / 17 * ys.len(), !half
            (8, 4)
        } else {
            // xs.len() >= 36 / 17 * ys.len(), half
            (9, 4)
        };
        n = 1 + if q * xs_len >= p * ys_len {
            (xs_len - 1) / p
        } else {
            (ys_len - 1) / q
        };
        p -= 1;
        q -= 1;
        let mut pn = p * n;
        let mut qn = q * n;
        // With LIMIT = 16 / 15, the following recovery is needed only if `ys_len` <= 73.
        if !p.eq_mod_power_of_2(q, 1) {
            // Recover from badly-chosen splitting
            if xs_len <= pn {
                p -= 1;
                pn -= n;
            } else if ys_len <= qn {
                q -= 1;
                qn -= n;
            } else {
                half = true;
            }
        }
        (p, q, pn, qn)
    };
    assert!(n > 2);
    assert!(xs_len > pn);
    let s = xs_len - pn;
    assert!(s <= n);
    assert!(ys_len > qn);
    let t = ys_len - qn;
    assert!(t <= n);
    assert!(half || s + t > 3);
    let m = n + 1;
    // r == 2 * n + 1
    let r = m + n;
    split_into_chunks_mut!(scratch, 3 * n + 1, [r5, r3, r1], scratch2);
    let (out_lo, remainder) = out.split_at_mut(3 * n);
    let (r4, remainder) = remainder.split_at_mut(n << 2);
    split_into_chunks_mut!(remainder, m, [v0, v1, v2], _unused);
    let (v3, mul_scratch) = scratch2.split_at_mut(m);
    // - Evaluation and recursive calls
    // - 1/2, -1/2
    let out_lo_lo = &mut out_lo[..m];
    let v_neg_half_neg =
        limbs_mul_toom_evaluate_poly_in_2_pow_neg_and_neg_2_pow_neg(v2, v0, p, xs, n, 1, out_lo_lo)
            != limbs_mul_toom_evaluate_poly_in_2_pow_neg_and_neg_2_pow_neg(
                v3, v1, q, ys, n, 1, out_lo_lo,
            );
    // - X(-1/2) * Y(-1/2) * 2^
    // - X(1/2) * Y(1/2) * 2^
    // - size: m
    limbs_mul_same_length_to_out_toom_6h_recursive(out_lo, v0, v1, mul_scratch);
    // size: m
    limbs_mul_same_length_to_out_toom_6h_recursive(r5, v2, v3, mul_scratch);
    if half {
        limbs_toom_couple_handling(r5, &mut out_lo[..r], v_neg_half_neg, n, 2, 1);
    } else {
        limbs_toom_couple_handling(r5, &mut out_lo[..r], v_neg_half_neg, n, 1, 0);
    }
    // 1, -1
    let out_lo_lo = &mut out_lo[..m];
    let mut v_neg_1_neg = limbs_mul_toom_evaluate_poly_in_1_and_neg_1(v2, v0, p, xs, n, out_lo_lo);
    let flip = if q == 3 {
        limbs_mul_toom_evaluate_deg_3_poly_in_1_and_neg_1(v3, v1, ys, n, out_lo_lo)
    } else {
        limbs_mul_toom_evaluate_poly_in_1_and_neg_1(v3, v1, q, ys, n, out_lo_lo)
    };
    if flip {
        v_neg_1_neg.not_assign();
    }
    // - X(-1) * Y(-1)
    // - X(1) * Y(1)
    // - size: m
    limbs_mul_same_length_to_out_toom_6h_recursive(out_lo, v0, v1, mul_scratch);
    // size: m
    limbs_mul_same_length_to_out_toom_6h_recursive(r3, v2, v3, mul_scratch);
    limbs_toom_couple_handling(r3, &mut out_lo[..r], v_neg_1_neg, n, 0, 0);
    // 4, -4
    let out_lo_lo = &mut out_lo[..m];
    let v_neg_4_neg =
        limbs_mul_toom_evaluate_poly_in_2_pow_and_neg_2_pow(v2, v0, p, xs, n, 2, out_lo_lo)
            != limbs_mul_toom_evaluate_poly_in_2_pow_and_neg_2_pow(v3, v1, q, ys, n, 2, out_lo_lo);
    // - X(-4) * Y(-4)
    // - size: m
    limbs_mul_same_length_to_out_toom_6h_recursive(out_lo, v0, v1, mul_scratch);
    // size: m
    limbs_mul_same_length_to_out_toom_6h_recursive(r1, v2, v3, mul_scratch);
    // X(4) * B(4)
    limbs_toom_couple_handling(r1, &mut out_lo[..r], v_neg_4_neg, n, 2, 4);
    // 1/4, -1/4
    let out_lo_lo = &mut out_lo[..m];
    let v_neg_quarter_neg =
        limbs_mul_toom_evaluate_poly_in_2_pow_neg_and_neg_2_pow_neg(v2, v0, p, xs, n, 2, out_lo_lo)
            != limbs_mul_toom_evaluate_poly_in_2_pow_neg_and_neg_2_pow_neg(
                v3, v1, q, ys, n, 2, out_lo_lo,
            );
    // - X(-1/4) * Y(-1/4) * 4^
    // - X(1/4) * Y(1/4) * 4^
    // - size: m
    limbs_mul_same_length_to_out_toom_6h_recursive(out_lo, v0, v1, mul_scratch);
    // size: m
    limbs_mul_same_length_to_out_toom_6h_recursive(r4, v2, v3, mul_scratch);
    if half {
        limbs_toom_couple_handling(r4, &mut out_lo[..r], v_neg_quarter_neg, n, 4, 2);
    } else {
        limbs_toom_couple_handling(r4, &mut out_lo[..r], v_neg_quarter_neg, n, 2, 0);
    }
    let out_lo = &mut out_lo[..m];
    // 2, -2
    let v_neg_2_neg = limbs_mul_toom_evaluate_poly_in_2_and_neg_2(v2, v0, p, xs, n, out_lo)
        != limbs_mul_toom_evaluate_poly_in_2_and_neg_2(v3, v1, q, ys, n, out_lo);
    // - X(-2) * Y(-2)
    // - X(2) * Y(2)
    let (out_lo, r2) = out.split_at_mut(7 * n);
    let (r2_v1, v2) = r2.split_at_mut(m << 1);
    let v2 = &mut v2[..m];
    {
        let (r2, v1) = r2_v1.split_at_mut(m);
        // size: m
        limbs_mul_same_length_to_out_toom_6h_recursive(out_lo, r2, v1, mul_scratch);
    }
    // size: m
    limbs_mul_same_length_to_out_toom_6h_recursive(r2_v1, v2, v3, mul_scratch);
    limbs_toom_couple_handling(r2, &mut out_lo[..r], v_neg_2_neg, n, 1, 2);
    // - X(0) * Y(0)
    // - size: n
    limbs_mul_same_length_to_out_toom_6h_recursive(out, &xs[..n], &ys[..n], mul_scratch);
    // Infinity
    if half {
        // size: s, t
        limbs_mul_to_out(&mut out[11 * n..], &xs[pn..], &ys[qn..], mul_scratch);
    }
    limbs_mul_toom_interpolate_12_points(out, r1, r3, r5, n, s + t, half, scratch2);
}}

// Limit num/den is a rational number between (16 / 15) ^ (log(6) / log(2 * 6 - 1)) and (16 / 15) ^
// (log(8) / log(2 * 8 - 1))
const TOOM_8H_LIMIT_NUMERATOR: usize = 21;
const TOOM_8H_LIMIT_DENOMINATOR: usize = 20;

// # Worst-case complexity
// Constant time and additional memory.
//
// This function can be used to determine whether the sizes of the input slices to
// `limbs_mul_greater_to_out_toom_8h` are valid.
pub_test! {limbs_mul_greater_to_out_toom_8h_input_sizes_valid(
    xs_len: usize,
    ys_len: usize
) -> bool {
    if xs_len == 0
        || ys_len < 86
        || xs_len < ys_len
        || xs_len > ys_len << 2
        || Limb::WIDTH <= 11 * 3 && xs_len << 2 > ys_len * 11
        || Limb::WIDTH <= 10 * 3 && xs_len > ys_len << 1
        || Limb::WIDTH <= 9 * 3 && xs_len << 1 > ys_len * 3
    {
        return false;
    }
    let n;
    let mut half = false;
    let mut xl = xs_len;
    let mut yl = ys_len;
    let (xr, yr) = if xs_len == ys_len
        || xs_len * (TOOM_8H_LIMIT_DENOMINATOR >> 1) < TOOM_8H_LIMIT_NUMERATOR * (ys_len >> 1)
    {
        // - xs_len == ys_len || xs_len < 21 / 20 * ys_len, !half
        // - This is the slowest variation
        n = 1 + ((xs_len - 1) >> 3);
        let n7 = 7 * n;
        (n7, n7)
    } else {
        let (p, q) = if xs_len * 13 < ys_len << 4 {
            // xs_len < 16 / 13 * ys_len, half
            (9, 8)
        } else if Limb::WIDTH <= 9 * 3
            || xs_len * (TOOM_8H_LIMIT_DENOMINATOR >> 1)
                < (TOOM_8H_LIMIT_NUMERATOR / 7 * 9) * (ys_len >> 1)
        {
            // Limb::WIDTH <= 27 || xs_len < 27 / 20 * ys_len, !half
            (9, 7)
        } else if xs_len * 10 < 33 * (ys_len >> 1) {
            // xs_len < 33 / 20 * ys_len, half
            (10, 7)
        } else if Limb::WIDTH <= 10 * 3
            || xs_len * (TOOM_8H_LIMIT_DENOMINATOR / 5) < (TOOM_8H_LIMIT_NUMERATOR / 3) * ys_len
        {
            // Limb::WIDTH <= 30 || xs_len < 7 / 4 * ys_len, !half
            (10, 6)
        } else if xs_len * 6 < 13 * ys_len {
            // xs_len < 13 / 6 * ys_len, half
            (11, 6)
        } else if Limb::WIDTH <= 11 * 3 || xs_len << 2 < 9 * ys_len {
            // Limb::WIDTH <= 33 || xs_len < 9 / 4 * ys_len, !half
            (11, 5)
        } else if xs_len * (TOOM_8H_LIMIT_NUMERATOR / 3) < TOOM_8H_LIMIT_DENOMINATOR * ys_len {
            // xs_len < 20 / 7 * ys_len, half
            (12, 5)
        } else if Limb::WIDTH <= 12 * 3 || xs_len * 9 < 28 * ys_len {
            // Limb::WIDTH <= 36 || xs_len < 28 / 9 * ys_len, !half
            (12, 4)
        } else {
            // half
            (13, 4)
        };
        n = 1 + if q * xs_len >= p * ys_len {
            (xs_len - 1) / p
        } else {
            (ys_len - 1) / q
        };
        let pn = (p - 1) * n;
        let qn = (q - 1) * n;
        if !p.eq_mod_power_of_2(q, 1) {
            // Recover from badly chosen splitting
            if xs_len <= pn {
                xl += n;
            } else if ys_len <= qn {
                yl += n;
            } else {
                half = true;
            }
        }
        (pn, qn)
    };
    xl > xr && yl > yr && {
        let (s, t) = (xl - xr, yl - yr);
        let limit = 15 * n + 6;
        s <= n
            && t <= n
            && (half || s + t > 3)
            && n > 2
            && limit <= limbs_mul_greater_to_out_toom_8h_scratch_len(xs_len, ys_len)
            && limit <= limbs_square_to_out_toom_8_scratch_len(n << 3)
    }
}}

#[cfg(feature = "32_bit_limbs")]
pub(crate) const BIT_CORRECTION: bool = true;
#[cfg(not(feature = "32_bit_limbs"))]
pub(crate) const BIT_CORRECTION: bool = false;

// TODO tune
const TOOM_8H_MAYBE_MUL_BASECASE: bool =
    TUNE_PROGRAM_BUILD || MUL_TOOM8H_THRESHOLD < MUL_TOOM22_THRESHOLD << 3;
// TODO tune
const TOOM_8H_MAYBE_MUL_TOOM22: bool =
    TUNE_PROGRAM_BUILD || MUL_TOOM8H_THRESHOLD < MUL_TOOM33_THRESHOLD << 3;
// TODO tune
const TOOM_8H_MAYBE_MUL_TOOM33: bool =
    TUNE_PROGRAM_BUILD || MUL_TOOM8H_THRESHOLD < MUL_TOOM44_THRESHOLD << 3;
// TODO tune
const TOOM_8H_MAYBE_MUL_TOOM44: bool =
    TUNE_PROGRAM_BUILD || MUL_TOOM8H_THRESHOLD < MUL_TOOM6H_THRESHOLD << 3;
// TODO tune
const TOOM_8H_MAYBE_MUL_TOOM8H: bool =
    TUNE_PROGRAM_BUILD || MUL_FFT_THRESHOLD >= MUL_TOOM8H_THRESHOLD << 3;

fn limbs_mul_same_length_to_out_toom_8h_recursive_scratch_len(n: usize) -> usize {
    if TOOM_8H_MAYBE_MUL_BASECASE && n < MUL_TOOM22_THRESHOLD {
        0
    } else if TOOM_8H_MAYBE_MUL_TOOM22 && n < MUL_TOOM33_THRESHOLD {
        limbs_mul_greater_to_out_toom_22_scratch_len(n, n)
    } else if TOOM_8H_MAYBE_MUL_TOOM33 && n < MUL_TOOM44_THRESHOLD {
        limbs_mul_greater_to_out_toom_33_scratch_len(n, n)
    } else if TOOM_8H_MAYBE_MUL_TOOM44 && n < MUL_TOOM6H_THRESHOLD {
        limbs_mul_greater_to_out_toom_44_scratch_len(n, n)
    } else if !TOOM_8H_MAYBE_MUL_TOOM8H || n < MUL_TOOM8H_THRESHOLD {
        limbs_mul_greater_to_out_toom_6h_scratch_len(n, n)
    } else {
        limbs_mul_greater_to_out_toom_8h_scratch_len(n, n)
    }
}

// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// The time would be O(n^{\log_7 15}) \approx O(n^{1.392}), but
// `limbs_mul_same_length_to_out_toom_22` ultimately calls `limbs_mul_greater_to_out`.
//
// This is equivalent to `TOOM8H_MUL_N_REC` from `mpn/generic/toom8h_mul.c`, GMP 6.2.1, when `f` is
// false.
fn limbs_mul_same_length_to_out_toom_8h_recursive(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let n = xs.len();
    assert_eq!(ys.len(), n);
    if TOOM_8H_MAYBE_MUL_BASECASE && n < MUL_TOOM22_THRESHOLD {
        limbs_mul_greater_to_out_basecase(out, xs, ys);
    } else if TOOM_8H_MAYBE_MUL_TOOM22 && n < MUL_TOOM33_THRESHOLD {
        limbs_mul_greater_to_out_toom_22(out, xs, ys, scratch);
    } else if TOOM_8H_MAYBE_MUL_TOOM33 && n < MUL_TOOM44_THRESHOLD {
        limbs_mul_greater_to_out_toom_33(out, xs, ys, scratch);
    } else if TOOM_8H_MAYBE_MUL_TOOM44 && n < MUL_TOOM6H_THRESHOLD {
        limbs_mul_greater_to_out_toom_44(out, xs, ys, scratch);
    } else if !TOOM_8H_MAYBE_MUL_TOOM8H || n < MUL_TOOM8H_THRESHOLD {
        limbs_mul_greater_to_out_toom_6h(out, xs, ys, scratch);
    } else {
        limbs_mul_greater_to_out_toom_8h(out, xs, ys, scratch);
    }
}

// This function can be used to determine the length of the input `scratch` slice in
// `limbs_mul_greater_to_out_toom_8h`.
//
// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `mpn_toom8h_mul_itch` from `gmp-impl.h`, GMP 6.2.1.
pub_crate_test! {limbs_mul_greater_to_out_toom_8h_scratch_len(
    xs_len: usize,
    ys_len: usize
) -> usize {
    let n;
    let mut half = false;
    let (pn, qn) = if xs_len == ys_len
        || xs_len * (TOOM_8H_LIMIT_DENOMINATOR >> 1) < TOOM_8H_LIMIT_NUMERATOR * (ys_len >> 1)
    {
        // - xs_len == ys_len || xs_len < 21 / 20 * ys_len, !half
        // - This is the slowest variation
        n = 1 + ((xs_len - 1) >> 3);
        let n7 = 7 * n;
        (n7, n7)
    } else {
        let (mut p, mut q) = if xs_len * 13 < ys_len << 4 {
            // xs_len < 16 / 13 * ys_len, half
            (9, 8)
        } else if Limb::WIDTH <= 9 * 3
            || xs_len * (TOOM_8H_LIMIT_DENOMINATOR >> 1)
                < (TOOM_8H_LIMIT_NUMERATOR / 7 * 9) * (ys_len >> 1)
        {
            // Limb::WIDTH <= 27 || xs_len < 27 / 20 * ys_len, !half
            (9, 7)
        } else if xs_len * 10 < 33 * (ys_len >> 1) {
            // xs_len < 33 / 20 * ys_len, half
            (10, 7)
        } else if Limb::WIDTH <= 10 * 3
            || xs_len * (TOOM_8H_LIMIT_DENOMINATOR / 5) < (TOOM_8H_LIMIT_NUMERATOR / 3) * ys_len
        {
            // Limb::WIDTH <= 30 || xs_len < 7 / 4 * ys_len, !half
            (10, 6)
        } else if xs_len * 6 < 13 * ys_len {
            // xs_len < 13 / 6 * ys_len, half
            (11, 6)
        } else if Limb::WIDTH <= 11 * 3 || xs_len << 2 < 9 * ys_len {
            // Limb::WIDTH <= 33 || xs_len < 9 / 4 * ys_len, !half
            (11, 5)
        } else if xs_len * (TOOM_8H_LIMIT_NUMERATOR / 3) < TOOM_8H_LIMIT_DENOMINATOR * ys_len {
            // xs_len < 20 / 7 * ys_len, half
            (12, 5)
        } else if Limb::WIDTH <= 12 * 3 || xs_len * 9 < 28 * ys_len {
            // Limb::WIDTH <= 36 || xs_len < 28 / 9 * ys_len, !half
            (12, 4)
        } else {
            // half
            (13, 4)
        };
        n = 1 + if q * xs_len >= p * ys_len {
            (xs_len - 1) / p
        } else {
            (ys_len - 1) / q
        };
        p -= 1;
        q -= 1;
        let mut pn = p * n;
        let mut qn = q * n;
        if !p.eq_mod_power_of_2(q, 1) {
            // Recover from badly chosen splitting
            if xs_len <= pn {
                pn -= n;
            } else if ys_len <= qn {
                qn -= n;
            } else {
                half = true;
            }
        }
        (pn, qn)
    };
    let s = xs_len - pn;
    let t = ys_len - qn;
    let m = n + 1;
    assert!(m < xs_len);
    let mut mul_scratch_len = max!(
        3 * n + 1,
        limbs_mul_same_length_to_out_toom_8h_recursive_scratch_len(n),
        limbs_mul_same_length_to_out_toom_8h_recursive_scratch_len(m)
    );
    if half {
        mul_scratch_len = max(mul_scratch_len, limbs_mul_to_out_scratch_len(s, t));
    }
    13 * n + 5 + mul_scratch_len
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
// the `xs.len() + ys.len()` least-significant limbs of the product of the `Natural`s to an output
// slice. A scratch slice is provided for the algorithm to use. An upper bound for the number of
// scratch limbs needed is provided by `limbs_mul_greater_to_out_toom_8h_scratch_len`. The following
// restrictions on the input slices must be met:
// - `out`.len() >= `xs`.len() + `ys`.len()
// - `xs`.len() >= `ys`.len()
// - Others; see `limbs_mul_greater_to_out_toom_8h_input_sizes_valid`.
//
// This uses the Toom-8h algorithm (Toom-8.5).
//
// Evaluate in: Infinity, 8, -8, 4, -4, 2, -2, 1, -1, 1 / 2, -1 / 2, 1 / 4, -1 / 4, 1 / 8, -1 / 8,
// 0.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// The time would be O(n^{\log_7 15}) \approx O(n^{1.392}), but
// `limbs_mul_same_length_to_out_toom_8h_recursive` ultimately calls `limbs_mul_greater_to_out`.
//
// # Panics
// May panic if the input slice conditions are not met.
//
// This is equivalent to `mpn_toom8h_mul` from `mpn/generic/toom8h_mul.c`, GMP 6.2.1.
pub_crate_test! {limbs_mul_greater_to_out_toom_8h(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);
    // - Decomposition
    // - Can not handle too small operands
    assert!(ys_len >= 86);
    // Can not handle too much unbalance
    assert!(xs_len <= ys_len << 2);
    assert!(Limb::WIDTH > 11 * 3 || xs_len << 2 <= ys_len * 11);
    assert!(Limb::WIDTH > 10 * 3 || xs_len <= ys_len << 1);
    assert!(Limb::WIDTH > 9 * 3 || xs_len << 1 <= ys_len * 3);
    let n;
    let mut half = false;
    let (p, q, pn, qn) = if xs_len == ys_len
        || xs_len * (TOOM_8H_LIMIT_DENOMINATOR >> 1) < TOOM_8H_LIMIT_NUMERATOR * (ys_len >> 1)
    {
        // - xs_len == ys_len || xs_len < 21 / 20 * ys_len, !half
        // - This is the slowest variation
        n = 1 + ((xs_len - 1) >> 3);
        let n7 = 7 * n;
        (7, 7, n7, n7)
    } else {
        let (mut p, mut q) = if xs_len * 13 < ys_len << 4 {
            // xs_len < 16 / 13 * ys_len, half
            (9, 8)
        } else if Limb::WIDTH <= 9 * 3
            || xs_len * (TOOM_8H_LIMIT_DENOMINATOR >> 1)
                < (TOOM_8H_LIMIT_NUMERATOR / 7 * 9) * (ys_len >> 1)
        {
            // Limb::WIDTH <= 27 || xs_len < 27 / 20 * ys_len, !half
            (9, 7)
        } else if xs_len * 10 < 33 * (ys_len >> 1) {
            // xs_len < 33 / 20 * ys_len, half
            (10, 7)
        } else if Limb::WIDTH <= 10 * 3
            || xs_len * (TOOM_8H_LIMIT_DENOMINATOR / 5) < (TOOM_8H_LIMIT_NUMERATOR / 3) * ys_len
        {
            // Limb::WIDTH <= 30 || xs_len < 7 / 4 * ys_len, !half
            (10, 6)
        } else if xs_len * 6 < 13 * ys_len {
            // xs_len < 13 / 6 * ys_len, half
            (11, 6)
        } else if Limb::WIDTH <= 11 * 3 || xs_len << 2 < 9 * ys_len {
            // Limb::WIDTH <= 33 || xs_len < 9 / 4 * ys_len, !half
            (11, 5)
        } else if xs_len * (TOOM_8H_LIMIT_NUMERATOR / 3) < TOOM_8H_LIMIT_DENOMINATOR * ys_len {
            // xs_len < 20 / 7 * ys_len, half
            (12, 5)
        } else if Limb::WIDTH <= 12 * 3 || xs_len * 9 < 28 * ys_len {
            // Limb::WIDTH <= 36 || xs_len < 28 / 9 * ys_len, !half
            (12, 4)
        } else {
            // half
            (13, 4)
        };
        n = 1 + if q * xs_len >= p * ys_len {
            (xs_len - 1) / p
        } else {
            (ys_len - 1) / q
        };
        p -= 1;
        q -= 1;
        let mut pn = p * n;
        let mut qn = q * n;
        if !p.eq_mod_power_of_2(q, 1) {
            // Recover from badly chosen splitting
            if xs_len <= pn {
                p -= 1;
                pn -= n;
            } else if ys_len <= qn {
                q -= 1;
                qn -= n;
            } else {
                half = true;
            }
        }
        (p, q, pn, qn)
    };
    assert!(n > 2);
    assert!(xs_len > pn);
    let s = xs_len - pn;
    assert!(s <= n);
    assert!(ys_len > qn);
    let t = ys_len - qn;
    assert!(t <= n);
    assert!(half || s + t > 3);
    let m = n + 1;
    let u = m + n; // 2 * n + 1
    let r = 3 * n + 1;
    // Evaluation and recursive calls
    let (pp_lo, remainder) = out.split_at_mut(3 * n);
    split_into_chunks_mut!(remainder, n << 2, [r6, r4], remainder);
    split_into_chunks_mut!(remainder, m, [v0, v1, v2], _unused);
    let (r7, remainder) = scratch.split_at_mut(r);
    let (v3, mul_scratch) = remainder[3 * r..].split_at_mut(m);
    // 1/8, -1/8
    let pp_lo_lo = &mut pp_lo[..m];
    let v_neg_8th_neg =
        limbs_mul_toom_evaluate_poly_in_2_pow_neg_and_neg_2_pow_neg(v2, v0, p, xs, n, 3, pp_lo_lo)
            != limbs_mul_toom_evaluate_poly_in_2_pow_neg_and_neg_2_pow_neg(
                v3, v1, q, ys, n, 3, pp_lo_lo,
            );
    // - X(-1/8) * Y(-1/8) * 8^
    // - X(1/8) * Y(1/8) * 8^
    // - size: m
    limbs_mul_same_length_to_out_toom_8h_recursive(pp_lo, v0, v1, mul_scratch);
    // size: m
    limbs_mul_same_length_to_out_toom_8h_recursive(r7, v2, v3, mul_scratch);
    let limit = if BIT_CORRECTION { m << 1 } else { u };
    let pp_lo_lo = &mut pp_lo[..limit];
    if half {
        limbs_toom_couple_handling(scratch, pp_lo_lo, v_neg_8th_neg, n, 6, 3);
    } else {
        limbs_toom_couple_handling(scratch, pp_lo_lo, v_neg_8th_neg, n, 3, 0);
    }
    split_into_chunks_mut!(scratch, r, [_unused, r5, r3, r1], remainder);
    let (v3, mul_scratch) = remainder.split_at_mut(m);
    // 1/4, -1/4
    let pp_lo_lo = &mut pp_lo[..m];
    let v_neg_quarter_neg =
        limbs_mul_toom_evaluate_poly_in_2_pow_neg_and_neg_2_pow_neg(v2, v0, p, xs, n, 2, pp_lo_lo)
            != limbs_mul_toom_evaluate_poly_in_2_pow_neg_and_neg_2_pow_neg(
                v3, v1, q, ys, n, 2, pp_lo_lo,
            );
    // - X(-1/4) * Y(-1/4) * 4^
    // - X(1/4) * Y(1/4) * 4^
    // - size: m
    limbs_mul_same_length_to_out_toom_8h_recursive(pp_lo, v0, v1, mul_scratch);
    // size: m
    limbs_mul_same_length_to_out_toom_8h_recursive(r5, v2, v3, mul_scratch);
    let pp_lo_lo = &mut pp_lo[..u];
    if half {
        limbs_toom_couple_handling(r5, pp_lo_lo, v_neg_quarter_neg, n, 4, 2);
    } else {
        limbs_toom_couple_handling(r5, pp_lo_lo, v_neg_quarter_neg, n, 2, 0);
    }
    // 2, -2
    let pp_lo_lo = &mut pp_lo[..m];
    let v_neg_2_neg = limbs_mul_toom_evaluate_poly_in_2_and_neg_2(v2, v0, p, xs, n, pp_lo_lo)
        != limbs_mul_toom_evaluate_poly_in_2_and_neg_2(v3, v1, q, ys, n, pp_lo_lo);
    // - X(-2) * Y(-2)
    // - X(2) * Y(2)
    // - size: m
    limbs_mul_same_length_to_out_toom_8h_recursive(pp_lo, v0, v1, mul_scratch);
    // size: m
    limbs_mul_same_length_to_out_toom_8h_recursive(r3, v2, v3, mul_scratch);
    limbs_toom_couple_handling(r3, &mut pp_lo[..u], v_neg_2_neg, n, 1, 2);
    // 8, -8
    let pp_lo_lo = &mut pp_lo[..m];
    let v_neg_8_neg =
        limbs_mul_toom_evaluate_poly_in_2_pow_and_neg_2_pow(v2, v0, p, xs, n, 3, pp_lo_lo)
            != limbs_mul_toom_evaluate_poly_in_2_pow_and_neg_2_pow(v3, v1, q, ys, n, 3, pp_lo_lo);
    // - X(-8) * Y(-8)
    // - X(8) * Y(8)
    // - size: m
    limbs_mul_same_length_to_out_toom_8h_recursive(pp_lo, v0, v1, mul_scratch);
    // size: m
    limbs_mul_same_length_to_out_toom_8h_recursive(r1, v2, v3, mul_scratch);
    limbs_toom_couple_handling(
        &mut scratch[3 * r..],
        &mut pp_lo[..limit],
        v_neg_8_neg,
        n,
        3,
        6,
    );
    let (v3, mul_scratch) = scratch[r << 2..].split_at_mut(m);
    // 1/2, -1/2
    let pp_lo_lo = &mut pp_lo[..m];
    let v_neg_half_neg =
        limbs_mul_toom_evaluate_poly_in_2_pow_neg_and_neg_2_pow_neg(v2, v0, p, xs, n, 1, pp_lo_lo)
            != limbs_mul_toom_evaluate_poly_in_2_pow_neg_and_neg_2_pow_neg(
                v3, v1, q, ys, n, 1, pp_lo_lo,
            );
    // - X(-1/2) * Y(-1/2) * 2^
    // - X(1/2) * Y(1/2) * 2^
    // - size: m
    limbs_mul_same_length_to_out_toom_8h_recursive(pp_lo, v0, v1, mul_scratch);
    // size: m
    limbs_mul_same_length_to_out_toom_8h_recursive(r6, v2, v3, mul_scratch);
    let pp_lo_lo = &mut pp_lo[..u];
    if half {
        limbs_toom_couple_handling(r6, pp_lo_lo, v_neg_half_neg, n, 2, 1);
    } else {
        limbs_toom_couple_handling(r6, pp_lo_lo, v_neg_half_neg, n, 1, 0);
    }
    // 1, -1
    let pp_lo_lo = &mut pp_lo[..m];
    let mut v_neg_1_neg = limbs_mul_toom_evaluate_poly_in_1_and_neg_1(v2, v0, p, xs, n, pp_lo_lo);
    if if Limb::WIDTH > 36 && q == 3 {
        limbs_mul_toom_evaluate_deg_3_poly_in_1_and_neg_1(v3, v1, ys, n, pp_lo_lo)
    } else {
        limbs_mul_toom_evaluate_poly_in_1_and_neg_1(v3, v1, q, ys, n, pp_lo_lo)
    } {
        v_neg_1_neg.not_assign();
    }
    // - X(-1) * Y(-1)
    // - X(1) * Y(1)
    // - size: m
    limbs_mul_same_length_to_out_toom_8h_recursive(pp_lo, v0, v1, mul_scratch);
    // size: m
    limbs_mul_same_length_to_out_toom_8h_recursive(r4, v2, v3, mul_scratch);
    limbs_toom_couple_handling(r4, &mut pp_lo[..u], v_neg_1_neg, n, 0, 0);
    // 4, -4
    let pp_lo_lo = &mut pp_lo[..m];
    let v_neg_4_neg =
        limbs_mul_toom_evaluate_poly_in_2_pow_and_neg_2_pow(v2, v0, p, xs, n, 2, pp_lo_lo)
            != limbs_mul_toom_evaluate_poly_in_2_pow_and_neg_2_pow(v3, v1, q, ys, n, 2, pp_lo_lo);
    // - X(-4) * Y(-4)
    // - X(4) * Y(4)
    // - size: m
    limbs_mul_same_length_to_out_toom_8h_recursive(pp_lo, v0, v1, mul_scratch);
    split_into_chunks_mut!(scratch, r, [r7, r5, r3, r1], mul_scratch);
    let (v3, mul_scratch) = mul_scratch.split_at_mut(m);
    let (r2, v2) = out[11 * n..].split_at_mut(m << 1);
    // size: m
    limbs_mul_same_length_to_out_toom_8h_recursive(r2, &v2[..m], v3, mul_scratch);
    let (pp_lo, r2) = out.split_at_mut(11 * n);
    limbs_toom_couple_handling(r2, &mut pp_lo[..u], v_neg_4_neg, n, 2, 4);
    // - X(0) * Y(0)
    // - size: n
    limbs_mul_same_length_to_out_toom_8h_recursive(out, &xs[..n], &ys[..n], mul_scratch);
    // Infinity
    if half {
        // size: s, t
        limbs_mul_to_out(&mut out[15 * n..], &xs[pn..], &ys[qn..], mul_scratch);
    }
    limbs_mul_toom_interpolate_16_points(
        out,
        r1,
        r3,
        r5,
        r7,
        n,
        s + t,
        half,
        &mut mul_scratch[..r],
    );
}}
