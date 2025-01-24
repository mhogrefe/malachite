// Copyright © 2025 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      `MAYBE_sqr_toom2`, `TOOM2_SQR_REC`, and `mpn_toom2_sqr` contributed to the GNU project by
//      Torbjörn Granlund.
//
//      `SQR_TOOM6_MAX`, `MAYBE_sqr_basecase`, `MAYBE_sqr_toom2`, `MAYBE_sqr_above_toom2`,
//      `MAYBE_sqr_toom3`, `MAYBE_sqr_above_toom3`, `MAYBE_sqr_above_toom4`, `TOOM6_SQR_REC`,
//      `mpn_toom6_sqr`, `SQR_TOOM8_MAX`, `MAYBE_sqr_basecase`, `MAYBE_sqr_above_basecase`,
//      `MAYBE_sqr_toom2`, `MAYBE_sqr_above_toom2`, `MAYBE_sqr_toom3`, `MAYBE_sqr_above_toom3`,
//      `MAYBE_sqr_toom4`, `MAYBE_sqr_above_toom4`, `MAYBE_sqr_above_toom6`, `TOOM8_SQR_REC`, and
//      `mpn_toom8_sqr` contributed to the GNU project by Marco Bodrato.
//
//      `MAYBE_sqr_toom3`, `MAYBE_sqr_basecase`, `TOOM3_SQR_REC`, and `mpn_toom3_sqr` contributed to
//      the GNU project by Torbjörn Granlund, with additional improvements by Marco Bodrato.
//
//      `MAYBE_sqr_toom2`, `MAYBE_sqr_toom4`, `TOOM4_SQR_REC`, and `mpn_toom4_sqr` contributed to
//      the GNU project by Torbjörn Granlund and Marco Bodrato.
//
//      Copyright © 1991-2018 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::add::{
    limbs_add_limb_to_out, limbs_add_same_length_to_out, limbs_add_to_out,
    limbs_slice_add_greater_in_place_left, limbs_slice_add_limb_in_place,
    limbs_slice_add_same_length_in_place_left,
};
use crate::natural::arithmetic::add_mul::limbs_slice_add_mul_limb_same_length_in_place_left;
use crate::natural::arithmetic::mul::fft::{
    limbs_square_to_out_fft, limbs_square_to_out_fft_scratch_len,
};
use crate::natural::arithmetic::mul::limb::limbs_mul_limb_to_out;
use crate::natural::arithmetic::mul::limbs_mul_greater_to_out_basecase;
use crate::natural::arithmetic::mul::poly_eval::{
    limbs_mul_toom_evaluate_deg_3_poly_in_1_and_neg_1,
    limbs_mul_toom_evaluate_deg_3_poly_in_2_and_neg_2, limbs_mul_toom_evaluate_poly_in_1_and_neg_1,
    limbs_mul_toom_evaluate_poly_in_2_and_neg_2,
    limbs_mul_toom_evaluate_poly_in_2_pow_and_neg_2_pow,
    limbs_mul_toom_evaluate_poly_in_2_pow_neg_and_neg_2_pow_neg,
};
use crate::natural::arithmetic::mul::poly_interpolate::{
    limbs_mul_toom_interpolate_12_points, limbs_mul_toom_interpolate_16_points,
    limbs_mul_toom_interpolate_5_points, limbs_mul_toom_interpolate_7_points,
};
use crate::natural::arithmetic::mul::toom::{
    limbs_toom_couple_handling, BIT_CORRECTION, TUNE_PROGRAM_BUILD, WANT_FAT_BINARY,
};
use crate::natural::arithmetic::shl::{limbs_shl_to_out, limbs_slice_shl_in_place};
use crate::natural::arithmetic::sub::{
    limbs_sub_limb_in_place, limbs_sub_same_length_in_place_left, limbs_sub_same_length_to_out,
};
use crate::natural::comparison::cmp::limbs_cmp_same_length;
use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::{
    DoubleLimb, Limb, SQR_BASECASE_THRESHOLD, SQR_TOOM2_THRESHOLD, SQR_TOOM3_THRESHOLD,
    SQR_TOOM4_THRESHOLD, SQR_TOOM6_THRESHOLD, SQR_TOOM8_THRESHOLD,
};
use alloc::vec::Vec;
use core::cmp::{max, Ordering::*};
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{
    ArithmeticCheckedShl, DivRound, ShrRound, Square, SquareAssign, WrappingAddAssign,
    WrappingSubAssign, XMulYToZZ,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::{SplitInHalf, WrappingFrom};
use malachite_base::rounding_modes::RoundingMode::*;

const SQR_FFT_MODF_THRESHOLD: usize = SQR_TOOM3_THRESHOLD * 3;

// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `MPN_SQR_DIAGONAL` from `mpn/generic/sqr_basecase.c`, GMP 6.2.1.
#[inline]
pub(crate) fn limbs_square_diagonal(out: &mut [Limb], xs: &[Limb]) {
    for (i, &x) in xs.iter().enumerate() {
        let i_2 = i << 1;
        (out[i_2 | 1], out[i_2]) = DoubleLimb::from(x).square().split_in_half();
    }
}

// scratch must have length 2 * xs.len() - 2 and out must have length 2 * xs.len().
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `MPN_SQR_DIAG_ADDLSH1` from `mpn/generic/sqr_basecase.c`, GMP 6.2.1.
pub_test! {limbs_square_diagonal_add_shl_1(out: &mut [Limb], scratch: &mut [Limb], xs: &[Limb]) {
    limbs_square_diagonal(out, xs);
    let (out_last, out_init) = out.split_last_mut().unwrap();
    *out_last += limbs_slice_shl_in_place(scratch, 1);
    if limbs_slice_add_same_length_in_place_left(&mut out_init[1..], scratch) {
        *out_last += 1;
    }
}}

// Interpreting a slices of `Limb`s as the limbs (in ascending order) of a `Natural`s, writes the `2
// * xs.len()` least-significant limbs of the square of the `Natural`s to an output slice. The
// output must be at least twice as long as `xs.len()`, `xs.len()` must be less than
// `SQR_TOOM2_THRESHOLD`, and `xs` cannot be empty.
//
// # Worst-case complexity
// $T(n) = O(n^2)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// # Panics
// Panics if `out` is less than twice the length of `xs`, `xs.len()` > SQR_TOOM2_THRESHOLD, or if
// `xs` is empty.
//
// This is equivalent to `mpn_sqr_basecase` from `mpn/generic/sqr_basecase.c`, GMP 6.2.1.
pub_crate_test! {limbs_square_to_out_basecase(out: &mut [Limb], xs: &[Limb]) {
    let n = xs.len();
    let (xs_head, xs_tail) = xs.split_first().unwrap();
    (out[1], out[0]) = DoubleLimb::from(*xs_head).square().split_in_half();
    if n > 1 {
        assert!(n <= SQR_TOOM2_THRESHOLD);
        let scratch = &mut [0; SQR_TOOM2_THRESHOLD << 1];
        let two_n = n << 1;
        let scratch = &mut scratch[..two_n - 2];
        let (scratch_last, scratch_init) = scratch[..n].split_last_mut().unwrap();
        *scratch_last = limbs_mul_limb_to_out(scratch_init, xs_tail, *xs_head);
        for i in 1..n - 1 {
            let (scratch_last, scratch_init) = scratch[i..][i..n].split_last_mut().unwrap();
            let (xs_head, xs_tail) = xs[i..].split_first().unwrap();
            *scratch_last =
                limbs_slice_add_mul_limb_same_length_in_place_left(scratch_init, xs_tail, *xs_head);
        }
        limbs_square_diagonal_add_shl_1(&mut out[..two_n], scratch, xs);
    }
}}

// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `mpn_toom2_sqr_itch` from `gmp-impl.h`, GMP 6.2.1.
pub_const_test! {limbs_square_to_out_toom_2_scratch_len(xs_len: usize) -> usize {
    (xs_len + Limb::WIDTH as usize) << 1
}}

// TODO tune

/// This is equivalent to `MAYBE_sqr_toom2` from `mpn/generic/toom2_sqr.c`, GMP 6.2.1.
const TOOM2_MAYBE_SQR_TOOM2: bool =
    TUNE_PROGRAM_BUILD || WANT_FAT_BINARY || SQR_TOOM3_THRESHOLD >= 2 * SQR_TOOM2_THRESHOLD;

// # Worst-case complexity
// $T(n) = O(n^{\log_2 3}) \approx O(n^{1.585})$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `TOOM2_SQR_REC` from `mpn/generic/toom2_sqr.c`, GMP 6.2.1.
fn limbs_square_to_out_toom_2_recursive(out: &mut [Limb], xs: &[Limb], scratch: &mut [Limb]) {
    if !TOOM2_MAYBE_SQR_TOOM2 || xs.len() < SQR_TOOM2_THRESHOLD {
        limbs_square_to_out_basecase(out, xs);
    } else {
        limbs_square_to_out_toom_2(out, xs, scratch);
    }
}

// Interpreting a slices of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the `2
// * xs.len()` least-significant limbs of the square of the `Natural` to an output slice. A scratch
// slice is provided for the algorithm to use. An upper bound for the number of scratch limbs needed
// is provided by `limbs_square_to_out_toom_2_scratch_len`. The following restrictions on the input
// slices must be met:
// - `out`.len() >= 2 * `xs`.len()
// - `xs`.len() > 1
//
// The smallest allowable `xs` length is 2.
//
// Evaluate in: -1, 0, infinity.
// ```
// <-s--><--n-->
//  ____ ______
// |xs1_|__xs0_|
//
// v_0     = xs_0 ^ 2          # X(0) ^ 2
// v_neg_1 = (xs_0 - xs_1) ^ 2 # X(-1) ^ 2
// v_inf   = xs_1 ^ 2          # X(inf) ^ 2
// ```
//
// # Worst-case complexity
// $T(n) = O(n^{\log_2 3}) \approx O(n^{1.585})$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// # Panics
// May panic if the input slice conditions are not met.
//
// This is equivalent to `mpn_toom2_sqr` from `mpn/generic/toom2_sqr.c`, GMP 6.2.1.
pub_test! {limbs_square_to_out_toom_2(out: &mut [Limb], xs: &[Limb], scratch: &mut [Limb]) {
    let xs_len = xs.len();
    assert!(xs_len > 1);
    let out = &mut out[..xs_len << 1];
    let s = xs_len >> 1;
    let n = xs_len - s;
    let (xs_0, xs_1) = xs.split_at(n);
    if s == n {
        if limbs_cmp_same_length(xs_0, xs_1) == Less {
            limbs_sub_same_length_to_out(out, xs_1, xs_0);
        } else {
            limbs_sub_same_length_to_out(out, xs_0, xs_1);
        }
    } else {
        // n - s == 1
        let (xs_0_last, xs_0_init) = xs_0.split_last().unwrap();
        let (out_last, out_init) = out[..n].split_last_mut().unwrap();
        if *xs_0_last == 0 && limbs_cmp_same_length(xs_0_init, xs_1) == Less {
            limbs_sub_same_length_to_out(out_init, xs_1, xs_0_init);
            *out_last = 0;
        } else {
            *out_last = *xs_0_last;
            if limbs_sub_same_length_to_out(out_init, xs_0_init, xs_1) {
                out_last.wrapping_sub_assign(1);
            }
        }
    }
    let (v_0, v_inf) = out.split_at_mut(n << 1);
    let (v_neg_1, scratch_out) = scratch.split_at_mut(n << 1);
    limbs_square_to_out_toom_2_recursive(v_neg_1, &v_0[..n], scratch_out);
    limbs_square_to_out_toom_2_recursive(v_inf, xs_1, scratch_out);
    limbs_square_to_out_toom_2_recursive(v_0, xs_0, scratch_out);
    let (v_0_lo, v_0_hi) = v_0.split_at_mut(n);
    let (v_inf_lo, v_inf_hi) = v_inf.split_at_mut(n);
    let mut carry = Limb::from(limbs_slice_add_same_length_in_place_left(v_inf_lo, v_0_hi));
    let mut carry2 = carry;
    if limbs_add_same_length_to_out(v_0_hi, v_inf_lo, v_0_lo) {
        carry2 += 1;
    }
    if limbs_slice_add_greater_in_place_left(v_inf_lo, &v_inf_hi[..s + s - n]) {
        carry += 1;
    }
    if limbs_sub_same_length_in_place_left(&mut out[n..3 * n], v_neg_1) {
        carry.wrapping_sub_assign(1);
    }
    assert!(carry.wrapping_add(1) <= 3);
    assert!(carry2 <= 2);
    let carry3 = limbs_slice_add_limb_in_place(&mut out[n << 1..], carry2);
    let out_hi = &mut out[3 * n..];
    if carry <= 2 {
        assert!(!limbs_slice_add_limb_in_place(out_hi, carry));
    } else if limbs_sub_limb_in_place(out_hi, 1) {
        assert!(carry3);
    }
}}

// This function can be used to determine whether the size of the input slice to
// `limbs_square_to_out_toom_3` is valid.
//
// # Worst-case complexity
// Constant time and additional memory.
pub_const_test! {limbs_square_to_out_toom_3_input_size_valid(xs_len: usize) -> bool {
    xs_len == 3 || xs_len > 4
}}

// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `mpn_toom3_sqr_itch` from `gmp-impl.h`, GMP 6.2.1.
pub_const_test! {limbs_square_to_out_toom_3_scratch_len(xs_len: usize) -> usize {
    3 * xs_len + Limb::WIDTH as usize
}}

// TODO tune

const SMALLER_RECURSION_TOOM_3: bool = true;

// TODO tune

// This is equivalent to `MAYBE_sqr_toom3` from `mpn/generic/toom3_sqr.c`, GMP 6.2.1.
#[cfg(feature = "test_build")]
pub const TOOM3_MAYBE_SQR_TOOM3: bool =
    TUNE_PROGRAM_BUILD || WANT_FAT_BINARY || SQR_TOOM4_THRESHOLD >= 3 * SQR_TOOM3_THRESHOLD;
#[cfg(not(feature = "test_build"))]
const TOOM3_MAYBE_SQR_TOOM3: bool =
    TUNE_PROGRAM_BUILD || WANT_FAT_BINARY || SQR_TOOM4_THRESHOLD >= 3 * SQR_TOOM3_THRESHOLD;

// TODO tune

/// This is equivalent to `MAYBE_sqr_basecase` from `mpn/generic/toom3_sqr.c`, GMP 6.2.1.
#[cfg(feature = "test_build")]
pub const TOOM3_MAYBE_SQR_BASECASE: bool =
    TUNE_PROGRAM_BUILD || WANT_FAT_BINARY || SQR_TOOM3_THRESHOLD < 3 * SQR_TOOM2_THRESHOLD;
#[cfg(not(feature = "test_build"))]
const TOOM3_MAYBE_SQR_BASECASE: bool =
    TUNE_PROGRAM_BUILD || WANT_FAT_BINARY || SQR_TOOM3_THRESHOLD < 3 * SQR_TOOM2_THRESHOLD;

// # Worst-case complexity
// $T(n) = O(n^{\log_3 5}) \approx O(n^{1.465})$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `TOOM3_SQR_REC` from `mpn/generic/toom3_sqr.c`, GMP 6.2.1.
fn limbs_square_to_out_toom_3_recursive(out: &mut [Limb], xs: &[Limb], scratch: &mut [Limb]) {
    let n = xs.len();
    if TOOM3_MAYBE_SQR_BASECASE && n < SQR_TOOM2_THRESHOLD {
        limbs_square_to_out_basecase(out, xs);
    } else if !TOOM3_MAYBE_SQR_TOOM3 || n < SQR_TOOM3_THRESHOLD {
        limbs_square_to_out_toom_2(out, xs, scratch);
    } else {
        limbs_square_to_out_toom_3(out, xs, scratch);
    }
}

// Interpreting a slices of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the `2
// * xs.len()` least-significant limbs of the square of the `Natural` to an output slice. A scratch
// slice is provided for the algorithm to use. An upper bound for the number of scratch limbs needed
// is provided by `limbs_square_to_out_toom_3_scratch_len`. The following restrictions on the input
// slices must be met:
// - `out`.len() >= 2 * `xs`.len()
// - `xs`.len() == 3 or `xs`.len() > 4
//
// The smallest allowable `xs` length is 3.
//
// Evaluate in: -1, 0, +1, +2, +inf
// ```
// <-s--><--n--><--n-->
//  ____ ______ ______
// |xs_2|_xs_1_|_xs_0_|
//
// v_0     = xs_0 ^ 2                         # X(0)^2
// v_1     = (xs_0 + xs_1 + xs_2) ^ 2         # X(1)^2    xh  <= 2
// v_neg_1 = (xs_0 - xs_1 + xs_2) ^ 2         # X(-1)^2  |xh| <= 1
// v_2     = (xs_0 + 2 * xs_1 + 4 * xs_2) ^ 2 # X(2)^2    xh  <= 6
// v_inf   = xs_2 ^ 2                         # X(inf)^2
// ```
//
// # Worst-case complexity
// $T(n) = O(n^{\log_3 5}) \approx O(n^{1.465})$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// # Panics
// May panic if the input slice conditions are not met.
//
// This is equivalent to `mpn_toom3_sqr` from `mpn/generic/toom3_sqr.c`, GMP 6.2.1.
pub_test! {limbs_square_to_out_toom_3(out: &mut [Limb], xs: &[Limb], scratch: &mut [Limb]) {
    let xs_len = xs.len();
    let n = xs_len.div_round(3, Ceiling).0;
    let m = n + 1;
    let k = m + n;
    let s = xs_len - (n << 1);
    assert_ne!(s, 0);
    assert!(s <= n);
    split_into_chunks!(xs, n, [xs_0, xs_1], xs_2);
    split_into_chunks_mut!(scratch, m << 1, [scratch_lo, asm1], as1);
    let (asm1_last, asm1_init) = asm1[..m].split_last_mut().unwrap();
    let (as1_last, as1_init) = as1[..m].split_last_mut().unwrap();
    let scratch_lo = &mut scratch_lo[..n];
    let mut carry = Limb::from(limbs_add_to_out(scratch_lo, xs_0, xs_2));
    *as1_last = carry;
    if limbs_add_same_length_to_out(as1_init, scratch_lo, xs_1) {
        *as1_last += 1;
    }
    if carry == 0 && limbs_cmp_same_length(scratch_lo, xs_1) == Less {
        limbs_sub_same_length_to_out(asm1_init, xs_1, scratch_lo);
    } else if limbs_sub_same_length_to_out(asm1_init, scratch_lo, xs_1) {
        carry.wrapping_sub_assign(1);
    }
    *asm1_last = carry;
    let as2 = &mut out[m..m << 1];
    let (as2_last, as2_init) = as2.split_last_mut().unwrap();
    let (as1_lo, as1_hi) = as1_init.split_at_mut(s);
    let mut carry = Limb::from(limbs_add_same_length_to_out(as2_init, xs_2, as1_lo));
    if s != n {
        carry = Limb::from(limbs_add_limb_to_out(&mut as2_init[s..], as1_hi, carry));
    }
    carry.wrapping_add_assign(*as1_last);
    carry = carry.arithmetic_checked_shl(1).unwrap();
    carry.wrapping_add_assign(limbs_slice_shl_in_place(as2_init, 1));
    if limbs_sub_same_length_in_place_left(as2_init, xs_0) {
        carry.wrapping_sub_assign(1);
    }
    *as2_last = carry;
    assert!(*as1_last <= 2);
    assert!(*asm1_last <= 1);
    let (scratch_lo, scratch_out) = scratch.split_at_mut(5 * m);
    if SMALLER_RECURSION_TOOM_3 {
        let (v_neg_1, asm1) = scratch_lo.split_at_mut(k);
        let (v_neg_1_last, v_neg_1_init) = v_neg_1.split_last_mut().unwrap();
        let (asm1_last, asm1_init) = asm1[1..n + 2].split_last().unwrap();
        limbs_square_to_out_toom_3_recursive(v_neg_1_init, asm1_init, scratch_out);
        *v_neg_1_last = if *asm1_last != 0 {
            asm1_last.wrapping_add(limbs_slice_add_mul_limb_same_length_in_place_left(
                &mut v_neg_1_init[n..],
                asm1_init,
                2,
            ))
        } else {
            0
        };
    } else {
        fail_on_untested_path("limbs_square_to_out_toom_3, !SMALLER_RECURSION");
        let (v_neg_1, asm1) = scratch_lo.split_at_mut(m << 1);
        limbs_square_to_out_toom_3_recursive(v_neg_1, &asm1[..m], scratch_out);
    }
    limbs_square_to_out_toom_3_recursive(&mut scratch_lo[k..], as2, scratch_out);
    let v_inf = &mut out[n << 2..];
    limbs_square_to_out_toom_3_recursive(v_inf, xs_2, scratch_out);
    let v_inf_0 = v_inf[0];
    let (as1, scratch_out) = &mut scratch[m << 2..].split_at_mut(m);
    let out_hi = &mut out[n << 1..];
    if SMALLER_RECURSION_TOOM_3 {
        let (v_1_last, v_1_init) = out_hi[..k].split_last_mut().unwrap();
        let (as1_last, as1_init) = as1.split_last_mut().unwrap();
        limbs_square_to_out_toom_3_recursive(v_1_init, as1_init, scratch_out);
        let v_1_init = &mut v_1_init[n..];
        *v_1_last = if *as1_last == 1 {
            limbs_slice_add_mul_limb_same_length_in_place_left(v_1_init, as1_init, 2)
                .wrapping_add(1)
        } else if *as1_last != 0 {
            as1_last.arithmetic_checked_shl(1u64).unwrap().wrapping_add(
                limbs_slice_add_mul_limb_same_length_in_place_left(v_1_init, as1_init, 4),
            )
        } else {
            0
        };
    } else {
        let carry = out_hi[k];
        limbs_square_to_out_toom_3_recursive(out_hi, as1, scratch_out);
        out_hi[k] = carry;
    }
    let (v_neg_1, remainder) = scratch.split_at_mut(k);
    let (v_2, scratch_out) = remainder.split_at_mut(3 * n + 4);
    limbs_square_to_out_toom_3_recursive(out, xs_0, scratch_out);
    limbs_mul_toom_interpolate_5_points(out, v_2, v_neg_1, n, s << 1, false, v_inf_0);
}}

// This function can be used to determine whether the size of the input slice to
// `limbs_square_to_out_toom_4` is valid.
//
// # Worst-case complexity
// Constant time and additional memory.
pub_const_test! {limbs_square_to_out_toom_4_input_size_valid(xs_len: usize) -> bool {
    xs_len == 4 || xs_len == 7 || xs_len == 8 || xs_len > 9
}}

// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `mpn_toom4_sqr_itch` from `gmp-impl.h`, GMP 6.2.1.
pub_const_test! {limbs_square_to_out_toom_4_scratch_len(xs_len: usize) -> usize {
    3 * xs_len + Limb::WIDTH as usize
}}

// TODO tune

// This is equivalent to `MAYBE_sqr_toom2` from `mpn/generic/toom4_sqr.c`, GMP 6.2.1.
const TOOM4_MAYBE_SQR_TOOM2: bool =
    TUNE_PROGRAM_BUILD || SQR_TOOM4_THRESHOLD < 4 * SQR_TOOM3_THRESHOLD;

// TODO tune

// This is equivalent to `MAYBE_sqr_toom4` from `mpn/generic/toom4_sqr.c`, GMP 6.2.1.
const TOOM4_MAYBE_SQR_TOOM4: bool =
    TUNE_PROGRAM_BUILD || SQR_TOOM6_THRESHOLD >= 4 * SQR_TOOM4_THRESHOLD;

// # Worst-case complexity
// $T(n) = O(n^{\log_4 7}) \approx O(n^{1.404})$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `TOOM4_SQR_REC` from `mpn/generic/toom4_sqr.c`, GMP 6.2.1.
fn limbs_square_to_out_toom_4_recursive(out: &mut [Limb], xs: &[Limb], scratch: &mut [Limb]) {
    let n = xs.len();
    if n < SQR_TOOM2_THRESHOLD {
        // We don't check TOOM4_MAYBE_SQR_BASECASE because we never want the Toom functions to
        // handle very small inputs.
        limbs_square_to_out_basecase(out, xs);
    } else if TOOM4_MAYBE_SQR_TOOM2 && n < SQR_TOOM3_THRESHOLD {
        limbs_square_to_out_toom_2(out, xs, scratch);
    } else if !TOOM4_MAYBE_SQR_TOOM4 || n < SQR_TOOM4_THRESHOLD {
        limbs_square_to_out_toom_3(out, xs, scratch);
    } else {
        limbs_square_to_out_toom_4(out, xs, scratch);
    }
}

// Interpreting a slices of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the `2
// * xs.len()` least-significant limbs of the square of the `Natural` to an output slice. A scratch
// slice is provided for the algorithm to use. An upper bound for the number of scratch limbs needed
// is provided by `limbs_square_to_out_toom_4_scratch_len`. The following restrictions on the input
// slices must be met:
// - `out`.len() >= 2 * `xs`.len()
// - `xs`.len() is 4, 7, or 8, or `xs`.len() > 9.
//
// The smallest allowable `xs` length is 4.
//
// Evaluate in: -1, -1/2, 0, +1/2, +1, +2, +inf
// ```
// <-s--><--n--><--n--><--n-->
//  ____ ______ ______ ______
// |xs_3|_xs_2_|_xs_1_|_xs_0_|
//
// v_0     = xs_0 ^ 2                                    # X(0) ^ 2
// v_1     = (xs_0 + xs_1 + xs_2 + xs_3) ^ 2             # X(1) ^ 2     xh <= 3
// v_neg_1 = (xs_0 - xs_1 + xs_2 - xs_3) ^ 2             # X(-1) ^ 2    |xh| <= 1
// v_2     = (xs_0 + 2 * xs_1 + 4 * xs_2 + 8 * xs_3) ^ 2 # X(2) ^ 2     xh <= 14
// vh      = (8 * xs_0 + 4 * xs_1 + 2 * xs_2 + xs_3) ^ 2 # X(1/2) ^ 2   xh <= 14
// vmh     = (8 * xs_0 - 4 * xs_1 + 2 * xs_2 - xs_3) ^ 2 # X(-1/2) ^ 2  -4 <= xh <= 9
// v_inf   = xs_3 ^ 2                                    # X(inf) ^ 2
// ```
//
// # Worst-case complexity
// $T(n) = O(n^{\log_4 7}) \approx O(n^{1.404})$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// # Panics
// May panic if the input slice conditions are not met.
//
// This is equivalent to `mpn_toom4_sqr` from `mpn/generic/toom4_sqr.c`, GMP 6.2.1.
pub_test! {limbs_square_to_out_toom_4(out: &mut [Limb], xs: &[Limb], scratch: &mut [Limb]) {
    let xs_len = xs.len();
    let n = (xs_len + 3) >> 2;
    let s = xs_len - 3 * n;
    assert_ne!(s, 0);
    assert!(s <= n);
    let m = n + 1;
    let k = m + n;
    split_into_chunks!(xs, n, [xs_0, xs_1, xs_2], xs_3);
    // Total scratch need: 8 * n + 5 + scratch for recursive calls. This gives roughly 32 * n / 3 +
    // log term. Compute apx = xs_0 + 2 * xs_1 + 4 * xs_2 + 8 * xs_3 and amx = xs_0 - 2 * xs_1 + 4
    // * xs_2 - 8 * xs_3.
    let (apx, remainder) = out.split_at_mut(n << 1);
    let apx = &mut apx[..m];
    let (v1, amx) = remainder.split_at_mut(m << 1);
    let amx = &mut amx[..m];
    let (scratch_lo, scratch_hi) = scratch.split_at_mut((k << 2) + 1);
    limbs_mul_toom_evaluate_deg_3_poly_in_2_and_neg_2(apx, amx, xs, n, &mut scratch_hi[..m]);
    limbs_square_to_out_toom_4_recursive(scratch_lo, apx, scratch_hi);
    let scratch_lo = &mut scratch_lo[k..];
    limbs_square_to_out_toom_4_recursive(scratch_lo, amx, scratch_hi);
    // Compute apx = 8 xs_0 + 4 xs_1 + 2 xs_2 + xs_3 = (((2 xs_0 + xs_1) * 2 + xs_2) * 2 + xs_3
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
    carry.wrapping_add_assign(limbs_slice_shl_in_place(apx_init, 1));
    if limbs_slice_add_greater_in_place_left(apx_init, xs_3) {
        carry.wrapping_add_assign(1);
    }
    *apx_last = carry;
    assert!(*apx_last < 15);
    let scratch_lo = &mut scratch_lo[k..];
    limbs_square_to_out_toom_4_recursive(scratch_lo, apx, scratch_hi);
    // Compute apx = xs_0 + xs_1 + xs_2 + xs_3 and amx = xs_0 - xs_1 + xs_2 - xs_3.
    limbs_mul_toom_evaluate_deg_3_poly_in_1_and_neg_1(apx, amx, xs, n, &mut scratch_hi[..m]);
    limbs_square_to_out_toom_4_recursive(v1, apx, scratch_hi);
    let scratch_lo = &mut scratch_lo[k..];
    limbs_square_to_out_toom_4_recursive(scratch_lo, amx, scratch_hi);
    let (v0, vinf) = out.split_at_mut(n << 1);
    let vinf = &mut vinf[n << 2..];
    limbs_square_to_out_toom_4_recursive(v0, xs_0, scratch_hi);
    limbs_square_to_out_toom_4_recursive(vinf, xs_3, scratch_hi);
    split_into_chunks_mut!(scratch, k, [v2, vm2, vh, vm1], scratch_hi);
    limbs_mul_toom_interpolate_7_points(out, n, s << 1, false, vm2, false, vm1, v2, vh, scratch_hi);
}}

// This function can be used to determine whether the size of the input slice to
// `limbs_square_to_out_toom_6` is valid.
//
// # Worst-case complexity
// Constant time and additional memory.
pub_const_test! {limbs_square_to_out_toom_6_input_size_valid(xs_len: usize) -> bool {
    xs_len == 18 || xs_len > 21 && xs_len != 25 && xs_len != 26 && xs_len != 31
}}

// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `mpn_toom6_sqr_itch` from `gmp-impl.h`, GMP 6.2.1.
pub_crate_test! {limbs_square_to_out_toom_6_scratch_len(n: usize) -> usize {
    (n << 1)
        + max(
            (SQR_TOOM6_THRESHOLD << 1) + usize::wrapping_from(Limb::WIDTH) * 6,
            limbs_square_to_out_toom_4_scratch_len(SQR_TOOM6_THRESHOLD),
        )
        - (SQR_TOOM6_THRESHOLD << 1)
}}

// TODO tune

/// This is equivalent to `SQR_TOOM6_MAX` from `mpn/generic/toom6_sqr.c`, GMP 6.2.1.
const SQR_TOOM6_MAX: usize = (SQR_TOOM8_THRESHOLD + 6 * 2 - 1 + 5) / 6;

// TODO tune

/// This is equivalent to `MAYBE_sqr_basecase` from `mpn/generic/toom6_sqr.c`, GMP 6.2.1.
const TOOM6_MAYBE_SQR_BASECASE: bool =
    TUNE_PROGRAM_BUILD || SQR_TOOM6_THRESHOLD < 6 * SQR_TOOM2_THRESHOLD;

// TODO tune

/// This is equivalent to `MAYBE_sqr_toom2` from `mpn/generic/toom6_sqr.c`, GMP 6.2.1.
const TOOM6_MAYBE_SQR_TOOM2: bool =
    TUNE_PROGRAM_BUILD || SQR_TOOM6_THRESHOLD < 6 * SQR_TOOM3_THRESHOLD;

// TODO tune

/// This is equivalent to `MAYBE_sqr_above_toom2` from `mpn/generic/toom6_sqr.c`, GMP 6.2.1.
const TOOM6_MAYBE_SQR_ABOVE_TOOM2: bool =
    TUNE_PROGRAM_BUILD || SQR_TOOM6_MAX >= SQR_TOOM3_THRESHOLD;

// TODO tune

/// This is equivalent to `MAYBE_sqr_toom3` from `mpn/generic/toom6_sqr.c`, GMP 6.2.1.
const TOOM6_MAYBE_SQR_TOOM3: bool =
    TUNE_PROGRAM_BUILD || SQR_TOOM6_THRESHOLD < 6 * SQR_TOOM4_THRESHOLD;

// TODO tune

/// This is equivalent to `MAYBE_sqr_above_toom3` from `mpn/generic/toom6_sqr.c`, GMP 6.2.1.
const TOOM6_MAYBE_SQR_ABOVE_TOOM3: bool =
    TUNE_PROGRAM_BUILD || SQR_TOOM6_MAX >= SQR_TOOM4_THRESHOLD;

// TODO tune

/// This is equivalent to `MAYBE_sqr_above_toom4` from `mpn/generic/toom6_sqr.c`, GMP 6.2.1.
const TOOM6_MAYBE_SQR_ABOVE_TOOM4: bool =
    TUNE_PROGRAM_BUILD || SQR_TOOM6_MAX >= SQR_TOOM6_THRESHOLD;

// # Worst-case complexity
// $T(n) = O(n^{\log_6 11}) \approx O(n^{1.338})$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `TOOM6_SQR_REC` from `mpn/generic/toom6_sqr.c`, GMP 6.2.1.
fn limbs_square_to_out_toom_6_recursive(out: &mut [Limb], xs: &[Limb], scratch: &mut [Limb]) {
    let n = xs.len();
    if TOOM6_MAYBE_SQR_BASECASE && n < SQR_TOOM2_THRESHOLD {
        limbs_square_to_out_basecase(out, xs);
    } else if TOOM6_MAYBE_SQR_TOOM2 && (!TOOM6_MAYBE_SQR_ABOVE_TOOM2 || n < SQR_TOOM3_THRESHOLD) {
        limbs_square_to_out_toom_2(out, xs, scratch);
    } else if TOOM6_MAYBE_SQR_TOOM3 && (!TOOM6_MAYBE_SQR_ABOVE_TOOM3 || n < SQR_TOOM4_THRESHOLD) {
        limbs_square_to_out_toom_3(out, xs, scratch);
    } else if !TOOM6_MAYBE_SQR_ABOVE_TOOM4 || n < SQR_TOOM6_THRESHOLD {
        limbs_square_to_out_toom_4(out, xs, scratch);
    } else {
        limbs_square_to_out_toom_6(out, xs, scratch);
    }
}

// Interpreting a slices of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the `2
// * xs.len()` least-significant limbs of the square of the `Natural` to an output slice. A scratch
// slice is provided for the algorithm to use. An upper bound for the number of scratch limbs needed
// is provided by `limbs_square_to_out_toom_6_scratch_len`. The following restrictions on the input
// slices must be met:
// - `out`.len() >= 2 * `xs`.len()
// - `xs`.len() is 18, or `xs.len()` > 21 but `xs`.len() is not 25, 26, or 31.
//
// The smallest allowable `xs` length is 18.
//
// # Worst-case complexity
// $T(n) = O(n^{\log_6 11}) \approx O(n^{1.338})$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// # Panics
// May panic if the input slice conditions are not met.
//
// This is equivalent to `mpn_toom6_sqr` from `mpn/generic/toom6_sqr.c`, GMP 6.2.1.
pub_test! {limbs_square_to_out_toom_6(out: &mut [Limb], xs: &[Limb], scratch: &mut [Limb]) {
    let xs_len = xs.len();
    assert!(xs_len >= 18);
    let n = 1 + (xs_len - 1) / 6;
    assert!(xs_len > 5 * n);
    let s = xs_len - 5 * n;
    assert!(s <= n);
    assert!(10 * n + 3 <= xs_len << 1);
    let m = n + 1;
    let k = m + n;
    let (out_lo, remainder) = out.split_at_mut(3 * n);
    let (r4, r2) = remainder.split_at_mut(n << 2);
    let (v0, v2) = r2.split_at_mut(m << 1);
    let v0 = &mut v0[..m];
    let v2 = &mut v2[..m];
    // +/- 1/2
    limbs_mul_toom_evaluate_poly_in_2_pow_neg_and_neg_2_pow_neg(
        v2,
        v0,
        5,
        xs,
        n,
        1,
        &mut out_lo[..m],
    );
    split_into_chunks_mut!(scratch, 3 * n + 1, [r5, r3, r1], wse);
    limbs_square_to_out_toom_6_recursive(out_lo, v0, wse); // X(-1/2) ^ 2 * 2 ^
    limbs_square_to_out_toom_6_recursive(r5, v2, wse); // X(1/2) ^ 2 * 2 ^
    limbs_toom_couple_handling(r5, &mut out_lo[..k], false, n, 1, 0);
    // +/- 1
    limbs_mul_toom_evaluate_poly_in_1_and_neg_1(v2, v0, 5, xs, n, &mut out_lo[..m]);
    limbs_square_to_out_toom_6_recursive(out_lo, v0, wse); // X(-1) ^ 2
    limbs_square_to_out_toom_6_recursive(r3, v2, wse); // X(1) ^ 2
    limbs_toom_couple_handling(r3, &mut out_lo[..k], false, n, 0, 0);
    // +/- 4
    limbs_mul_toom_evaluate_poly_in_2_pow_and_neg_2_pow(v2, v0, 5, xs, n, 2, &mut out_lo[..m]);
    limbs_square_to_out_toom_6_recursive(out_lo, v0, wse); // X(-4) ^ 2
    limbs_square_to_out_toom_6_recursive(r1, v2, wse); // X(4) ^ 2
    limbs_toom_couple_handling(r1, &mut out_lo[..k], false, n, 2, 4);
    // +/- 1/4
    limbs_mul_toom_evaluate_poly_in_2_pow_neg_and_neg_2_pow_neg(
        v2,
        v0,
        5,
        xs,
        n,
        2,
        &mut out_lo[..m],
    );
    limbs_square_to_out_toom_6_recursive(out_lo, v0, wse); // X(-1/4) ^ 2 * 4 ^
    limbs_square_to_out_toom_6_recursive(r4, v2, wse); // X(1/4) ^ 2 * 4 ^
    limbs_toom_couple_handling(r4, &mut out_lo[..k], false, n, 2, 0);
    // +/- 2
    limbs_mul_toom_evaluate_poly_in_2_and_neg_2(v2, v0, 5, xs, n, &mut out_lo[..m]);
    limbs_square_to_out_toom_6_recursive(out_lo, v0, wse); // X(-2) ^ 2
    let (v0, v2) = r2.split_at_mut(m << 1);
    limbs_square_to_out_toom_6_recursive(v0, &v2[..m], wse); // X(2) ^ 2
    limbs_toom_couple_handling(r2, &mut out_lo[..k], false, n, 1, 2);
    limbs_square_to_out_toom_6_recursive(out_lo, &xs[..n], wse); // X(0) ^ 2
    limbs_mul_toom_interpolate_12_points(out, r1, r3, r5, n, s << 1, false, wse);
}}

// TODO tune
pub(crate) const SQR_FFT_THRESHOLD: usize = SQR_FFT_MODF_THRESHOLD * 10;

// This function can be used to determine whether the size of the input slice to
// `limbs_square_to_out_toom_8` is valid.
//
// # Worst-case complexity
// Constant time and additional memory.
pub_const_test! {limbs_square_to_out_toom_8_input_size_valid(xs_len: usize) -> bool {
    xs_len == 40 || xs_len > 43 && xs_len != 49 && xs_len != 50 && xs_len != 57
}}

// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `mpn_toom8_sqr_itch` from `gmp-impl.h`, GMP 6.2.1.
pub_crate_test! {limbs_square_to_out_toom_8_scratch_len(n: usize) -> usize {
    ((n * 15) >> 3)
        + max(
            ((SQR_TOOM8_THRESHOLD * 15) >> 3) + usize::wrapping_from(Limb::WIDTH) * 6,
            limbs_square_to_out_toom_6_scratch_len(SQR_TOOM8_THRESHOLD),
        )
        - ((SQR_TOOM8_THRESHOLD * 15) >> 3)
}}

// TODO tune

/// This is equivalent to `SQR_TOOM8_MAX` from `mpn/generic/toom8_sqr.c`, GMP 6.2.1.
const SQR_TOOM8_MAX: usize = if SQR_FFT_THRESHOLD <= usize::MAX - (8 * 2 - 1 + 7) {
    (SQR_FFT_THRESHOLD + 8 * 2 - 1 + 7) / 8
} else {
    usize::MAX
};

// TODO tune

/// This is equivalent to `MAYBE_sqr_basecase` from `mpn/generic/toom8_sqr.c`, GMP 6.2.1.
const TOOM8_MAYBE_SQR_BASECASE: bool =
    TUNE_PROGRAM_BUILD || SQR_TOOM8_THRESHOLD < 8 * SQR_TOOM2_THRESHOLD;

// TODO tune

/// This is equivalent to `MAYBE_sqr_above_basecase` from `mpn/generic/toom8_sqr.c`, GMP 6.2.1.
const TOOM8_MAYBE_SQR_ABOVE_BASECASE: bool =
    TUNE_PROGRAM_BUILD || SQR_TOOM8_MAX >= SQR_TOOM2_THRESHOLD;

// TODO tune

/// This is equivalent to `MAYBE_sqr_toom2` from `mpn/generic/toom8_sqr.c`, GMP 6.2.1.
const TOOM8_MAYBE_SQR_TOOM2: bool =
    TUNE_PROGRAM_BUILD || SQR_TOOM8_THRESHOLD < 8 * SQR_TOOM3_THRESHOLD;

// TODO tune

/// This is equivalent to `MAYBE_sqr_above_toom2` from `mpn/generic/toom8_sqr.c`, GMP 6.2.1.
const TOOM8_MAYBE_SQR_ABOVE_TOOM2: bool =
    TUNE_PROGRAM_BUILD || SQR_TOOM8_MAX >= SQR_TOOM3_THRESHOLD;

// TODO tune

/// This is equivalent to `MAYBE_sqr_toom3` from `mpn/generic/toom8_sqr.c`, GMP 6.2.1.
const TOOM8_MAYBE_SQR_TOOM3: bool =
    TUNE_PROGRAM_BUILD || SQR_TOOM8_THRESHOLD < 8 * SQR_TOOM4_THRESHOLD;

// TODO tune

/// This is equivalent to `MAYBE_sqr_above_toom3` from `mpn/generic/toom8_sqr.c`, GMP 6.2.1.
const TOOM8_MAYBE_SQR_ABOVE_TOOM3: bool =
    TUNE_PROGRAM_BUILD || SQR_TOOM8_MAX >= SQR_TOOM4_THRESHOLD;

// TODO tune

/// This is equivalent to `MAYBE_sqr_toom4` from `mpn/generic/toom8_sqr.c`, GMP 6.2.1.
const TOOM8_MAYBE_SQR_TOOM4: bool =
    TUNE_PROGRAM_BUILD || SQR_TOOM8_THRESHOLD < 8 * SQR_TOOM6_THRESHOLD;

// TODO tune

/// This is equivalent to `MAYBE_sqr_above_toom4` from `mpn/generic/toom8_sqr.c`, GMP 6.2.1.
const TOOM8_MAYBE_SQR_ABOVE_TOOM4: bool =
    TUNE_PROGRAM_BUILD || SQR_TOOM8_MAX >= SQR_TOOM6_THRESHOLD;

// TODO tune

/// This is equivalent to `MAYBE_sqr_above_toom6` from `mpn/generic/toom8_sqr.c`, GMP 6.2.1.
const TOOM8_MAYBE_SQR_ABOVE_TOOM6: bool =
    TUNE_PROGRAM_BUILD || SQR_TOOM8_MAX >= SQR_TOOM8_THRESHOLD;

// # Worst-case complexity
// $T(n) = O(n^{\log_6 11}) \approx O(n^{1.302})$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `TOOM8_SQR_REC` from `mpn/generic/toom8_sqr.c`, GMP 6.2.1, when `f` is
// `false`.
fn limbs_square_to_out_toom_8_recursive(out: &mut [Limb], xs: &[Limb], scratch: &mut [Limb]) {
    let n = xs.len();
    if TOOM8_MAYBE_SQR_BASECASE && (!TOOM8_MAYBE_SQR_ABOVE_BASECASE || n < SQR_TOOM2_THRESHOLD) {
        limbs_square_to_out_basecase(out, xs);
    } else if TOOM8_MAYBE_SQR_TOOM2 && (!TOOM8_MAYBE_SQR_ABOVE_TOOM2 || n < SQR_TOOM3_THRESHOLD) {
        limbs_square_to_out_toom_2(out, xs, scratch);
    } else if TOOM8_MAYBE_SQR_TOOM3 && (!TOOM8_MAYBE_SQR_ABOVE_TOOM3 || n < SQR_TOOM4_THRESHOLD) {
        limbs_square_to_out_toom_3(out, xs, scratch);
    } else if TOOM8_MAYBE_SQR_TOOM4 && (!TOOM8_MAYBE_SQR_ABOVE_TOOM4 || n < SQR_TOOM6_THRESHOLD) {
        limbs_square_to_out_toom_4(out, xs, scratch);
    } else if !TOOM8_MAYBE_SQR_ABOVE_TOOM6 || n < SQR_TOOM8_THRESHOLD {
        limbs_square_to_out_toom_6(out, xs, scratch);
    } else {
        limbs_square_to_out_toom_8(out, xs, scratch);
    }
}

// Interpreting a slices of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the `2
// * xs.len()` least-significant limbs of the square of the `Natural` to an output slice. A scratch
// slice is provided for the algorithm to use. An upper bound for the number of scratch limbs needed
// is provided by `limbs_square_to_out_toom_8_scratch_len`. The following restrictions on the input
// slices must be met:
// - `out`.len() >= 2 * `xs`.len()
// - `xs`.len() is 40, or `xs.len()` > 43 but `xs`.len() is not 49, 50, or 57.
//
// The smallest allowable `xs` length is 40.
//
// # Worst-case complexity
// $T(n) = O(n^{\log_6 11}) \approx O(n^{1.302})$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// # Panics
// May panic if the input slice conditions are not met.
//
// This is equivalent to `mpn_toom8_sqr` from `mpn/generic/toom8_sqr.c`, GMP 6.2.1.
pub_test! {limbs_square_to_out_toom_8(out: &mut [Limb], xs: &[Limb], scratch: &mut [Limb]) {
    let xs_len = xs.len();
    assert!(xs_len >= 40);
    let n: usize = xs_len.shr_round(3, Ceiling).0;
    let m = n + 1;
    let k = m + n;
    let p = k + n;
    assert!(xs_len > 7 * n);
    let s = xs_len - 7 * n;
    assert!(s <= n);
    assert!(s << 1 > 3);
    let (pp_lo, remainder) = out.split_at_mut(3 * n);
    split_into_chunks_mut!(remainder, n << 2, [r6, r4], pp_hi);
    split_into_chunks_mut!(pp_hi, m, [v0, _unused, v2], _unused);
    // +/- 1/8
    limbs_mul_toom_evaluate_poly_in_2_pow_neg_and_neg_2_pow_neg(
        v2,
        v0,
        7,
        xs,
        n,
        3,
        &mut pp_lo[..m],
    );
    let (r7_r5, remainder) = scratch.split_at_mut(p << 1);
    let (r3, r1_wse) = remainder.split_at_mut(p);
    let (r1, wse) = r1_wse.split_at_mut(p);
    // A(-1/8) * B(-1/8) * 8 ^, A(1/8) * B(1/8) * 8 ^
    limbs_square_to_out_toom_8_recursive(pp_lo, v0, wse);
    limbs_square_to_out_toom_8_recursive(r7_r5, v2, wse);
    let limit = if BIT_CORRECTION { m << 1 } else { k };
    limbs_toom_couple_handling(r7_r5, &mut pp_lo[..limit], false, n, 3, 0);
    // +/- 1/4
    limbs_mul_toom_evaluate_poly_in_2_pow_neg_and_neg_2_pow_neg(
        v2,
        v0,
        7,
        xs,
        n,
        2,
        &mut pp_lo[..m],
    );
    // A(-1/4) * B(-1/4) * 4 ^, A(1/4) * B(1/4) * 4^
    limbs_square_to_out_toom_8_recursive(pp_lo, v0, wse);
    let (r7, r5) = r7_r5.split_at_mut(p);
    limbs_square_to_out_toom_8_recursive(r5, v2, wse);
    limbs_toom_couple_handling(r5, &mut pp_lo[..k], false, n, 2, 0);
    // +/- 2
    limbs_mul_toom_evaluate_poly_in_2_and_neg_2(v2, v0, 7, xs, n, &mut pp_lo[..m]);
    // A(-2)*B(-2), A(+2)*B(+2)
    limbs_square_to_out_toom_8_recursive(pp_lo, v0, wse);
    limbs_square_to_out_toom_8_recursive(r3, v2, wse);
    limbs_toom_couple_handling(r3, &mut pp_lo[..k], false, n, 1, 2);
    // +/- 8
    limbs_mul_toom_evaluate_poly_in_2_pow_and_neg_2_pow(v2, v0, 7, xs, n, 3, &mut pp_lo[..m]);
    // A(-8) * B(-8), A(8) * B(8)
    limbs_square_to_out_toom_8_recursive(pp_lo, v0, wse);
    limbs_square_to_out_toom_8_recursive(r1, v2, wse);
    limbs_toom_couple_handling(r1_wse, &mut pp_lo[..limit], false, n, 3, 6);
    // +/- 1/2
    limbs_mul_toom_evaluate_poly_in_2_pow_neg_and_neg_2_pow_neg(
        v2,
        v0,
        7,
        xs,
        n,
        1,
        &mut pp_lo[..m],
    );
    // A(-1/2) * B(-1/2) * 2 ^, A(1/2) * B(1/2) * 2 ^
    let (r1, wse) = r1_wse.split_at_mut(p);
    limbs_square_to_out_toom_8_recursive(pp_lo, v0, wse);
    limbs_square_to_out_toom_8_recursive(r6, v2, wse);
    limbs_toom_couple_handling(r6, &mut pp_lo[..k], false, n, 1, 0);
    // +/- 1
    limbs_mul_toom_evaluate_poly_in_1_and_neg_1(v2, v0, 7, xs, n, &mut pp_lo[..m]);
    // A(-1) * B(-1), A(1) * B(1)
    limbs_square_to_out_toom_8_recursive(pp_lo, v0, wse);
    limbs_square_to_out_toom_8_recursive(r4, v2, wse);
    limbs_toom_couple_handling(r4, &mut pp_lo[..k], false, n, 0, 0);
    // +/- 4
    limbs_mul_toom_evaluate_poly_in_2_pow_and_neg_2_pow(v2, v0, 7, xs, n, 2, &mut pp_lo[..m]);
    // A(-4) * B(-4), A(4) * B(4)
    limbs_square_to_out_toom_8_recursive(pp_lo, v0, wse);
    let (r2, v2) = pp_hi.split_at_mut(m << 1);
    limbs_square_to_out_toom_8_recursive(r2, &v2[..m], wse);
    limbs_toom_couple_handling(pp_hi, &mut pp_lo[..k], false, n, 2, 4);
    // A(0) * B(0)
    limbs_square_to_out_toom_8_recursive(pp_lo, &xs[..n], wse);
    limbs_mul_toom_interpolate_16_points(out, r1, r3, r5, r7, n, s << 1, false, &mut wse[..p]);
}}

pub_crate_test! {
#[allow(clippy::absurd_extreme_comparisons)]
limbs_square_to_out_scratch_len(n: usize) -> usize {
    if n < SQR_BASECASE_THRESHOLD || n < SQR_TOOM2_THRESHOLD {
        0
    } else if n < SQR_TOOM3_THRESHOLD {
        limbs_square_to_out_toom_2_scratch_len(n)
    } else if n < SQR_TOOM4_THRESHOLD {
        limbs_square_to_out_toom_3_scratch_len(n)
    } else if n < SQR_TOOM6_THRESHOLD {
        limbs_square_to_out_toom_4_scratch_len(n)
    } else if n < SQR_TOOM8_THRESHOLD {
        limbs_square_to_out_toom_6_scratch_len(n)
    } else if n < SQR_FFT_THRESHOLD {
        limbs_square_to_out_toom_8_scratch_len(n)
    } else {
        limbs_square_to_out_fft_scratch_len(n)
    }
}}

// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `mpn_sqr` from `mpn/generic/sqr.c`, GMP 6.2.1.
pub_crate_test! {
#[allow(clippy::absurd_extreme_comparisons)]
limbs_square_to_out(out: &mut [Limb], xs: &[Limb], scratch: &mut [Limb]) {
    let n = xs.len();
    assert!(n >= 1);
    if n < SQR_BASECASE_THRESHOLD {
        // limbs_mul_greater_to_out_basecase is faster than limbs_square_to_out_basecase on small
        // sizes sometimes
        limbs_mul_greater_to_out_basecase(out, xs, xs);
    } else if n < SQR_TOOM2_THRESHOLD {
        limbs_square_to_out_basecase(out, xs);
    } else if n < SQR_TOOM3_THRESHOLD {
        limbs_square_to_out_toom_2(out, xs, scratch);
    } else if n < SQR_TOOM4_THRESHOLD {
        limbs_square_to_out_toom_3(out, xs, scratch);
    } else if n < SQR_TOOM6_THRESHOLD {
        limbs_square_to_out_toom_4(out, xs, scratch);
    } else if n < SQR_TOOM8_THRESHOLD {
        limbs_square_to_out_toom_6(out, xs, scratch);
    } else if n < SQR_FFT_THRESHOLD {
        limbs_square_to_out_toom_8(out, xs, scratch);
    } else {
        limbs_square_to_out_fft(out, xs, scratch);
    }
}}

pub_crate_test! {limbs_square(xs: &[Limb]) -> Vec<Limb> {
    let mut out = vec![0; xs.len() << 1];
    let mut square_scratch = vec![0; limbs_square_to_out_scratch_len(xs.len())];
    limbs_square_to_out(&mut out, xs, &mut square_scratch);
    out
}}

impl Square for Natural {
    type Output = Natural;

    /// Squares a [`Natural`], taking it by value.
    ///
    /// $$
    /// f(x) = x^2.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Square;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.square(), 0);
    /// assert_eq!(Natural::from(123u32).square(), 15129);
    /// ```
    #[inline]
    fn square(mut self) -> Natural {
        self.square_assign();
        self
    }
}

impl Square for &Natural {
    type Output = Natural;

    /// Squares a [`Natural`], taking it by reference.
    ///
    /// $$
    /// f(x) = x^2.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Square;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::ZERO).square(), 0);
    /// assert_eq!((&Natural::from(123u32)).square(), 15129);
    /// ```
    #[inline]
    fn square(self) -> Natural {
        match self {
            &Natural::ZERO | &Natural::ONE => self.clone(),
            Natural(Small(x)) => Natural({
                let (upper, lower) = Limb::x_mul_y_to_zz(*x, *x);
                if upper == 0 {
                    Small(lower)
                } else {
                    Large(vec![lower, upper])
                }
            }),
            Natural(Large(ref xs)) => Natural::from_owned_limbs_asc(limbs_square(xs)),
        }
    }
}

impl SquareAssign for Natural {
    /// Squares a [`Natural`] in place.
    ///
    /// $$
    /// x \gets x^2.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::SquareAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::ZERO;
    /// x.square_assign();
    /// assert_eq!(x, 0);
    ///
    /// let mut x = Natural::from(123u32);
    /// x.square_assign();
    /// assert_eq!(x, 15129);
    /// ```
    fn square_assign(&mut self) {
        match self {
            &mut (Natural::ZERO | Natural::ONE) => {}
            Natural(Small(x)) => {
                let (upper, lower) = Limb::x_mul_y_to_zz(*x, *x);
                if upper == 0 {
                    *x = lower;
                } else {
                    *self = Natural(Large(vec![lower, upper]));
                }
            }
            Natural(Large(ref mut xs)) => {
                *xs = limbs_square(xs);
                self.trim();
            }
        }
    }
}
