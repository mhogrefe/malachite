// Copyright © 2024 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      `DO_mpn_sublsh_n`, `DO_mpn_subrsh`, `mpn_toom_interpolate_6pts`,
//      `mpn_toom_interpolate_8pts`, `mpn_toom_interpolate_12pts`, and `mpn_toom_interpolate_16pts`
//      contributed to the GNU project by Marco Bodrato.
//
//      `mpn_toom_interpolate_5pts` and `mpn_toom_interpolate_7pts` contributed to the GNU project
//      by Robert Harley, with improvements by Paul Zimmermann and Marco Bodrato.
//
//      Copyright © 2000-2003, 2005-2007, 2009, 2010, 2011, 2012, 2015, 2020 Free Software
//      Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::add::{
    limbs_add_limb_to_out, limbs_add_same_length_to_out, limbs_slice_add_greater_in_place_left,
    limbs_slice_add_limb_in_place, limbs_slice_add_same_length_in_place_left,
};
use crate::natural::arithmetic::add_mul::limbs_slice_add_mul_limb_same_length_in_place_left;
use crate::natural::arithmetic::div::limbs_div_divisor_of_limb_max_with_carry_in_place;
use crate::natural::arithmetic::div_exact::{
    limbs_div_exact_3_in_place, limbs_div_exact_limb_in_place,
};
use crate::natural::arithmetic::mul::poly_eval::limbs_shl_and_add_same_length_in_place_left;
use crate::natural::arithmetic::mul::toom::BIT_CORRECTION;
use crate::natural::arithmetic::shl::limbs_shl_to_out;
use crate::natural::arithmetic::shr::limbs_slice_shr_in_place;
use crate::natural::arithmetic::sub::{
    limbs_sub_greater_in_place_left, limbs_sub_limb_in_place, limbs_sub_same_length_in_place_left,
    limbs_sub_same_length_in_place_right, limbs_sub_same_length_in_place_with_overlap,
    limbs_sub_same_length_to_out,
};
use crate::natural::arithmetic::sub_mul::limbs_sub_mul_limb_same_length_in_place_left;
use crate::platform::{
    Limb, AORSMUL_FASTER_2AORSLSH, AORSMUL_FASTER_3AORSLSH, AORSMUL_FASTER_AORS_2AORSLSH,
    AORSMUL_FASTER_AORS_AORSLSH,
};
use core::mem::swap;
use malachite_base::num::arithmetic::traits::{
    DivisibleByPowerOf2, Parity, WrappingAddAssign, WrappingSubAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::slices::slice_test_zero;

// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `k`.
//
// This is equivalent to `mpn_toom_interpolate_5pts` in `mpn/generic/toom_interpolate_5pts.c`, GMP
// 6.2.1.
pub(crate) fn limbs_mul_toom_interpolate_5_points(
    c: &mut [Limb],
    v_2: &mut [Limb],
    v_neg_1: &mut [Limb],
    k: usize,
    two_r: usize,
    v_neg_1_neg: bool,
    mut v_inf_0: Limb,
) {
    let two_k = k << 1;
    let two_k_plus_1 = two_k + 1;
    let four_k_plus_1 = two_k_plus_1 + two_k;
    assert_eq!(v_neg_1.len(), two_k_plus_1);
    assert!(two_r <= two_k);
    let v_1 = &c[two_k..four_k_plus_1]; // v_1 length: 2 * k + 1
    let v_2 = &mut v_2[..two_k_plus_1];
    // ```
    // (1) v_2 <- v_2 - v_neg_1 < v_2 + |v_neg_1|,            (16 8 4 2 1) - (1 -1 1 -1  1) =
    // thus 0 <= v_2 < 50 * B ^ (2 * k) < 2 ^ 6 * B ^ (2 * k) (15 9 3  3  0)
    // ```
    if v_neg_1_neg {
        assert!(!limbs_slice_add_same_length_in_place_left(v_2, v_neg_1));
    } else {
        assert!(!limbs_sub_same_length_in_place_left(v_2, v_neg_1));
    }
    // ```
    // {c,2k} {c + 2k,2k + 1} {c + 4k + 1,2r - 1} {t,2k + 1} {t + 2k + 1,2k + 1} {t + 4k + 2,2r}
    //   v0        v_1             hi(v_inf)      |v_neg_1|     v_2-v_neg_1          EMPTY
    // v_2 <- v_2 / 3
    // (5 3 1 1 0)
    // {c,2k} {c + 2k,2k + 1} {c + 4k + 1,2r - 1} {t,2k + 1} {t + 2k + 1,2k + 1} {t + 4k + 2,2r}
    //   v0       v_1             hi(v_inf)        |v_neg_1|    (v_2-v_neg_1)/3       EMPTY
    //
    // (2) v_neg_1 <- tm1 := (v_1 - v_neg_1) / 2  [(1 1 1 1 1) - (1 -1 1 -1 1)] / 2 =
    // tm1 >= 0                                    (0  1 0  1 0)
    // ```
    // No carry comes out from {v_1, two_k_plus_1} +/- {v_neg_1, two_k_plus_1}, and the division by
    // two is exact. If v_neg_1_neg the sign of v_neg_1 is negative
    limbs_div_exact_3_in_place(v_2);
    if v_neg_1_neg {
        assert!(!limbs_slice_add_same_length_in_place_left(v_neg_1, v_1));
    } else {
        assert!(!limbs_sub_same_length_in_place_right(v_1, v_neg_1));
    }
    assert_eq!(limbs_slice_shr_in_place(v_neg_1, 1), 0);
    // ```
    // {c,2k} {c + 2k,2k + 1} {c + 4k + 1,2r - 1} {t,2k + 1} {t + 2k + 1,2k + 1} {t + 4k + 2,2r}
    //   v0       v_1             hi(v_inf)          tm1       (v_2-v_neg_1)/3        EMPTY
    //
    // (3) v_1 <- t1 := v_1 - v0  (1 1 1 1 1) - (0 0 0 0 1) = (1 1 1 1 0)
    // t1 >= 0
    // ```
    let (c_lo, v_1) = c.split_at_mut(two_k);
    if limbs_sub_same_length_in_place_left(&mut v_1[..two_k], c_lo) {
        v_1[two_k].wrapping_sub_assign(1);
    }
    let v_1 = &mut v_1[..two_k_plus_1];
    // ```
    // {c,2k} {c + 2k,2k + 1} {c + 4k + 1,2r - 1} {t,2k + 1} {t + 2k + 1,2k + 1} {t + 4k + 2,2r}
    //   v0       v_1-v0           hi(v_inf)          tm1      (v_2-v_neg_1)/3        EMPTY
    //
    // (4) v_2 <- t2 := ((v_2 - v_neg_1) / 3 - t1) / 2 = (v_2 - v_neg_1 - 3 * t1) / 6
    // t2 >= 0                  [(5 3 1 1 0) - (1 1 1 1 0)]/2 = (2 1 0 0 0)
    // ```
    assert!(!limbs_sub_same_length_in_place_left(v_2, v_1));
    assert_eq!(limbs_slice_shr_in_place(v_2, 1), 0);
    // ```
    // {c,2k} {c + 2k,2k + 1} {c + 4k + 1,2r - 1} {t,2k + 1} {t + 2k + 1,2k + 1} {t + 4k + 2,2r}
    //   v0      v_1 - v0        hi(v_inf)          tm1    (v_2 - v_neg_1 - 3t1) / 6    EMPTY
    //
    // (5) v_1 <- t1 - tm1           (1 1 1 1 0) - (0 1 0 1 0) = (1 0 1 0 0)
    // result is v_1 >= 0
    // ```
    assert!(!limbs_sub_same_length_in_place_left(v_1, v_neg_1));
    // We do not need to read the value in v_neg_1, so we add it in {c + k, ..}
    let (c_lo, c_hi) = c.split_at_mut(3 * k + 1);
    if limbs_slice_add_same_length_in_place_left(&mut c_lo[k..], v_neg_1) {
        // ```
        // 2 * n - (3 * k + 1) = 2 * r + k - 1
        // ```
        //
        // Memory allocated for v_neg_1 is now free, it can be recycled
        assert!(!limbs_slice_add_limb_in_place(
            &mut c_hi[..two_r + k - 1],
            1,
        ));
    }
    let v_inf = &mut c_hi[k - 1..two_r + k - 1];
    // ```
    // (6) v_2 <- v_2 - 2 * v_inf, (2 1 0 0 0) - 2 * (1 0 0 0 0) = (0 1 0 0 0)
    // ```
    //
    // result is v_2 >= 0
    let saved = v_inf[0]; // Remember v1's highest byte (will be overwritten).
    v_inf[0] = v_inf_0; // Set the right value for v_inf_0
                        // Overwrite unused v_neg_1
    let mut carry = limbs_shl_to_out(v_neg_1, &v_inf[..two_r], 1);
    if limbs_sub_same_length_in_place_left(&mut v_2[..two_r], &v_neg_1[..two_r]) {
        carry += 1;
    }
    assert!(!limbs_sub_limb_in_place(&mut v_2[two_r..], carry));
    // Current matrix is
    // ```
    // [1 0 0 0 0; v_inf
    //  0 1 0 0 0; v_2
    //  1 0 1 0 0; v1
    //  0 1 0 1 0; v_neg_1
    //  0 0 0 0 1] v0
    // ```
    // Some values already are in-place (we added v_neg_1 in the correct position)
    // ```
    // | v_inf|  v1 |  v0 |
    // | v_neg_1 |
    // ```
    // One still is in a separated area
    // ```
    // | +v_2 |
    // ```
    // We have to compute v1-=v_inf; v_neg_1 -= v_2,
    // ```
    // | -v_inf|
    // | -v_2 |
    // ```
    // Carefully reordering operations we can avoid to compute twice the sum of the high half of v_2
    // plus the low half of v_inf.
    //
    // Add the high half of t2 in {v_inf}
    if two_r > k + 1 {
        // This is the expected flow
        let (c_lo, c_hi) = c[k << 2..].split_at_mut(k + 1);
        if limbs_slice_add_same_length_in_place_left(c_lo, &v_2[k..]) {
            // 2n-(5k+1) = 2r-k-1
            assert!(!limbs_slice_add_limb_in_place(
                &mut c_hi[..two_r - k - 1],
                1,
            ));
        }
    } else {
        // - triggered only by very unbalanced cases like (k+k+(k-2))x(k+k+1), should be handled by
        //   toom32
        // - two_r < k + 1 so k + two_r < two_k, the size of v_2
        assert!(!limbs_slice_add_same_length_in_place_left(
            &mut c[k << 2..(k << 2) + two_r],
            &v_2[k..k + two_r],
        ));
    }
    split_into_chunks_mut!(c, k << 1, [_unused, v_1], v_inf);
    // - (7) v_1 <- v_1 - v_inf,       (1 0 1 0 0) - (1 0 0 0 0) = (0 0 1 0 0)
    // - result is >= 0
    // - Side effect: we also subtracted (high half) v_neg_1 -= v_2
    // - v_inf is at most two_r long.
    let carry = limbs_sub_same_length_in_place_left(&mut v_1[..two_r], &v_inf[..two_r]);
    v_inf_0 = v_inf[0]; // Save again the right value for v_inf_0
    v_inf[0] = saved;
    split_into_chunks_mut!(c, k, [_unused, c1], v1);
    let v1 = &mut v1[..two_k_plus_1];
    if carry {
        assert!(!limbs_sub_limb_in_place(&mut v1[two_r..], 1)); // Treat the last bytes.
    }
    // - (8) v_neg_1 <- v_neg_1 - v_2 (0 1 0 1 0) - (0 1 0 0 0) = (0 0 0 1 0)
    // - Operate only on the low half.
    if limbs_sub_same_length_in_place_left(c1, &v_2[..k]) {
        assert!(!limbs_sub_limb_in_place(v1, 1));
    }
    let (c3, v_inf) = c[3 * k..].split_at_mut(k);
    // - Beginning the final phase
    // - Most of the recomposition was done
    // - add t2 in {c + 3 * k, ...}, but only the low half
    if limbs_slice_add_same_length_in_place_left(c3, &v_2[..k]) {
        v_inf[0].wrapping_add_assign(1);
        assert!(v_inf[0] >= 1); // No carry
    }
    // Add v_inf_0, propagate carry.
    assert!(!limbs_slice_add_limb_in_place(&mut v_inf[..two_r], v_inf_0));
}

// Interpolation for Toom-3.5, using the evaluation points infinity, 1, -1, 2, -2. More precisely,
// we want to compute f(2 ^ (`Limb::WIDTH` * n)) for a polynomial f of degree 5, given the six
// values
//
// ```
// w5 = f(0),
// w4 = f(-1),
// w3 = f(1)
// w2 = f(-2),
// w1 = f(2),
// w0 = limit at infinity of f(x) / x^5,
// ```
//
// The result is stored in {out, 5 * n + n_high}. At entry, w5 is stored at {out, 2 * n}, w3 is
// stored at {out + 2 * n, 2 * n + 1}, and w0 is stored at {out + 5 * n, n_high}. The other values
// are 2 * n + 1 limbs each (with most significant limbs small). f(-1) and f(-2) may be negative;
// signs are passed in. All intermediate results are positive. Inputs are destroyed.
//
// Interpolation sequence was taken from the paper: "Integer and Polynomial Multiplication: Towards
// Optimal Toom-Cook Matrices". Some slight variations were introduced: adaptation to "gmp
// instruction set", and a final saving of an operation by interlacing interpolation and
// recomposition phases.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `n`.
//
// This is equivalent to `mpn_toom_interpolate_6pts` from `mpn/generic/toom_interpolate_6pts.c`, GMP
// 6.2.1, but the argument `w0n == n_high` is moved to immediately after `n`.
pub(crate) fn limbs_mul_toom_interpolate_6_points(
    out: &mut [Limb],
    n: usize,
    n_high: usize,
    w4_neg: bool,
    w4: &mut [Limb],
    w2_neg: bool,
    w2: &mut [Limb],
    w1: &mut [Limb],
) {
    assert_ne!(n, 0);
    let m = 2 * n + 1;
    assert_ne!(n_high, 0);
    assert!(n_high < m);
    assert_eq!(w1.len(), m);
    assert_eq!(w2.len(), m);
    assert_eq!(w4.len(), m);
    // w5 length: 2 * n
    //
    // Interpolate with sequence:
    // - w2 = (w1 - w2) >> 2
    // - w1 = (w1 - w5) >> 1
    // - w1 = (w1 - w2) >> 1
    // - w4 = (w3 - w4) >> 1
    // - w2 = (w2 - w4) / 3
    // - w3 =  w3 - w4 - w5
    // - w1 = (w1 - w3) / 3
    //
    // Last steps are mixed with recomposition:
    // - w2 = w2 - w0 << 2
    // - w4 = w4 - w2
    // - w3 = w3 - w1
    // - w2 = w2 - w0
    //
    // w2 = (w1 - w2) >> 2
    let (w5, w3) = out[..=n << 2].split_at_mut(n << 1);
    if w2_neg {
        limbs_slice_add_same_length_in_place_left(w2, w1);
    } else {
        limbs_sub_same_length_in_place_right(w1, w2);
    }
    limbs_slice_shr_in_place(w2, 2);
    // w1 = (w1 - w5) >> 1
    let (w1_last, w1_init) = w1.split_last_mut().unwrap();
    if limbs_sub_same_length_in_place_left(w1_init, w5) {
        w1_last.wrapping_sub_assign(1);
    }
    limbs_slice_shr_in_place(w1, 1);
    // w1 = (w1 - w2) >> 1
    limbs_sub_same_length_in_place_left(w1, w2);
    limbs_slice_shr_in_place(w1, 1);
    // w4 = (w3 - w4) >> 1
    if w4_neg {
        limbs_slice_add_same_length_in_place_left(w4, w3);
    } else {
        limbs_sub_same_length_in_place_right(w3, w4);
    }
    limbs_slice_shr_in_place(w4, 1);
    // w2 = (w2 - w4) / 3
    limbs_sub_same_length_in_place_left(w2, w4);
    limbs_div_exact_3_in_place(w2);
    // w3 = w3 - w4 - w5
    limbs_sub_same_length_in_place_left(w3, w4);
    let (w3_last, w3_init) = w3.split_last_mut().unwrap();
    if limbs_sub_same_length_in_place_left(w3_init, w5) {
        w3_last.wrapping_sub_assign(1);
    }
    // w1 = (w1 - w3) / 3
    limbs_sub_same_length_in_place_left(w1, w3);
    limbs_div_exact_3_in_place(w1);
    // ```
    // [1 0 0 0 0 0;
    //  0 1 0 0 0 0;
    //  1 0 1 0 0 0;
    //  0 1 0 1 0 0;
    //  1 0 1 0 1 0;
    //  0 0 0 0 0 1]
    // ```
    //
    // out[] prior to operations:
    // ```
    // |_H w0__|_L w0__|______||_H w3__|_L w3__|_H w5__|_L w5__|
    // ```
    //
    // summation scheme for remaining operations:
    // ```
    // |______________5|n_____4|n_____3|n_____2|n______|n______| out
    // |_H w0__|_L w0__|______||_H w3__|_L w3__|_H w5__|_L w5__|
    //                || H w4  | L w4  |
    //        || H w2  | L w2  |
    //    || H w1  | L w1  |
    //            ||-H w1  |-L w1  |
    //         |-H w0  |-L w0 ||-H w2  |-L w2  |
    // ```
    let out = &mut out[n..];
    let (out_lo, out_hi) = out.split_at_mut(m);
    if limbs_slice_add_same_length_in_place_left(out_lo, w4) {
        assert!(!limbs_slice_add_limb_in_place(&mut out_hi[..n], 1));
    }
    // ```
    // w2 -= w0 << 2
    // ```
    //
    // {w4, 2 * n + 1} is now free and can be overwritten.
    let out_hi = &out[n << 2..];
    let mut carry = limbs_shl_to_out(w4, &out_hi[..n_high], 2);
    let (w2_lo, w2_hi) = w2.split_at_mut(n_high);
    if limbs_sub_same_length_in_place_left(w2_lo, &w4[..n_high]) {
        carry += 1;
    }
    assert!(!limbs_sub_limb_in_place(w2_hi, carry));
    // w4L = w4L - w2L
    let (w2_lo, w2_hi) = w2.split_at(n);
    let (out_lo, out) = out.split_at_mut(n);
    if limbs_sub_same_length_in_place_left(out_lo, w2_lo) {
        assert!(!limbs_sub_limb_in_place(&mut out[..m], 1));
    }
    let (out_lo, out_hi) = out.split_at_mut(n << 1);
    let carry = Limb::from(limbs_slice_add_same_length_in_place_left(
        &mut out_lo[n..],
        w2_lo,
    ));
    // w3H = w3H + w2L
    let carry_1 = out_hi[0] + carry;
    // w1L + w2H
    let (w2_hi_last, w2_hi_init) = w2_hi.split_last().unwrap();
    let mut carry = *w2_hi_last;
    let (w1_lo, w1_hi) = w1.split_at_mut(n);
    if limbs_add_same_length_to_out(out_hi, w1_lo, w2_hi_init) {
        carry += 1;
    }
    assert!(!limbs_slice_add_limb_in_place(w1_hi, carry));
    // w0 = w0 + w1H
    let mut carry_2 = 0;
    let (w1_last, w1_init) = w1.split_last().unwrap();
    let w1_init = &w1_init[n..];
    let out_hi = &mut out[3 * n..];
    if n_high > n {
        carry_2 = *w1_last;
        if limbs_slice_add_same_length_in_place_left(&mut out_hi[..n], w1_init) {
            carry_2.wrapping_add_assign(1);
        }
    } else if limbs_slice_add_same_length_in_place_left(&mut out_hi[..n_high], &w1_init[..n_high]) {
        carry_2 = 1;
    }
    // summation scheme for the next operation:
    // ```
    // |...____5|n_____4|n_____3|n_____2|n______|n______| out
    // |...w0___|_w1_w2_|_H w3__|_L w3__|_H w5__|_L w5__|
    //         ...-w0___|-w1_w2 |
    // ```
    //
    // if (LIKELY(n_high > n)) the two operands below DO overlap!
    let out = &mut out[..3 * n + n_high];
    let carry = limbs_sub_same_length_in_place_with_overlap(out, n << 1);
    let out_high = out.last_mut().unwrap();
    let embankment = out_high.wrapping_sub(1);
    *out_high = 1;
    let out = &mut out[n..];
    if n_high > n {
        if carry_1 > carry_2 {
            assert!(!limbs_slice_add_limb_in_place(
                &mut out[n..],
                carry_1 - carry_2,
            ));
        } else {
            assert!(!limbs_sub_limb_in_place(&mut out[n..], carry_2 - carry_1));
        }
        if carry {
            assert!(!limbs_sub_limb_in_place(&mut out[n_high..], 1));
        }
        assert!(!limbs_slice_add_limb_in_place(&mut out[3 * n..], carry_2));
    } else {
        assert!(!limbs_slice_add_limb_in_place(&mut out[n..], carry_1));
        if carry {
            carry_2.wrapping_add_assign(1);
        }
        assert!(!limbs_sub_limb_in_place(&mut out[n_high..], carry_2));
    }
    out.last_mut().unwrap().wrapping_add_assign(embankment);
}

const WANT_ASSERT: bool = true;

// Interpolation for toom4, using the evaluation points 0, infinity, 1, -1, 2, -2, 1 / 2. More
// precisely, we want to compute f(2 ^ (Limb::WIDTH * n)) for a polynomial f of degree 6, given the
// seven values
// - w0 = f(0),
// - w1 = f(-2),
// - w2 = f(1),
// - w3 = f(-1),
// - w4 = f(2)
// - w5 = 64 * f(1/2)
// - w6 = limit at infinity of f(x) / x ^ 6,
//
// The result is 6 * n + n_high limbs. At entry, w0 is stored at {out, 2 * n}, w2 is stored at {out
// + 2 * n, 2 * n + 1}, and w6 is stored at {out + 6 * n, n_high}. The other values are 2 * n + 1
// limbs each (with most significant limbs small). f(-1) and f(-1/2) may be negative, signs
// determined by the flag bits. Inputs are destroyed.
//
// Needs 2 * n + 1 limbs of temporary storage.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `n`.
//
// This is equivalent to `mpn_toom_interpolate_7pts` from `mpn/generic/toom_interpolate_7pts.c`, GMP
// 6.2.1, but the argument `w6n == n_high` is moved to immediately after `n`.
pub(crate) fn limbs_mul_toom_interpolate_7_points(
    out: &mut [Limb],
    n: usize,
    n_high: usize,
    w1_neg: bool,
    w1: &mut [Limb],
    w3_neg: bool,
    w3: &mut [Limb],
    w4: &mut [Limb],
    w5: &mut [Limb],
    scratch: &mut [Limb],
) {
    let m = 2 * n + 1;
    assert_ne!(n_high, 0);
    assert!(n_high < m);
    assert_eq!(w1.len(), m);
    assert_eq!(w3.len(), m);
    assert_eq!(w4.len(), m);
    assert_eq!(w5.len(), m);
    let (w0, remainder) = out.split_at_mut(n << 1);
    let (w2, w6) = remainder.split_at_mut(n << 2);
    let w2 = &mut w2[..m];
    let w6 = &mut w6[..n_high];
    // Using formulas similar to Marco Bodrato's
    //
    // - w5 =  w5 + w4
    // - w1 = (w4 - w1) / 2
    // - w4 =  w4 - w0
    // - w4 = (w4 - w1) / 4 - w6 * 16
    // - w3 = (w2 - w3) / 2
    // - w2 =  w2 - w3
    //
    // - w5 =  w5 - w2 * 65 May be negative.
    // - w2 =  w2 - w6 - w0
    // - w5 = (w5 + w2 * 45) / 2 Now >= 0 again.
    // - w4 = (w4 - w2) / 3
    // - w2 =  w2 - w4
    //
    // - w1 =  w5 - w1 May be negative.
    // - w5 = (w5 - w3 * 8) / 9
    // - w3 =  w3 - w5
    // - w1 = (w1 / 15 + w5) / 2 Now >= 0 again.
    // - w5 =  w5 - w1
    //
    // where w0 = f(0), w1 = f(-2), w2 = f(1), w3 = f(-1), w4 = f(2), w5 = f(1/2), w6 = f(infinity).
    //
    // Note that most intermediate results are positive; the ones that may be negative are
    // represented in two's complement. We must never shift right a value that may be negative,
    // since that would invalidate the sign bit. On the other hand, divexact by odd numbers works
    // fine with two's complement.
    limbs_slice_add_same_length_in_place_left(w5, w4);
    if w1_neg {
        limbs_slice_add_same_length_in_place_left(w1, w4);
    } else {
        limbs_sub_same_length_in_place_right(w4, w1);
    }
    assert!(w1[0].even());
    limbs_slice_shr_in_place(w1, 1);
    limbs_sub_greater_in_place_left(w4, w0);
    limbs_sub_same_length_in_place_left(w4, w1);
    assert!(w4[0].divisible_by_power_of_2(2));
    limbs_slice_shr_in_place(w4, 2); // w4 >= 0
    scratch[n_high] = limbs_shl_to_out(scratch, w6, 4);
    limbs_sub_greater_in_place_left(w4, &scratch[..=n_high]);
    if w3_neg {
        limbs_slice_add_same_length_in_place_left(w3, w2);
    } else {
        limbs_sub_same_length_in_place_right(w2, w3);
    }
    assert!(w3[0].even());
    limbs_slice_shr_in_place(w3, 1);
    limbs_sub_same_length_in_place_left(w2, w3);
    limbs_sub_mul_limb_same_length_in_place_left(w5, w2, 65);
    limbs_sub_greater_in_place_left(w2, w6);
    limbs_sub_greater_in_place_left(w2, w0);
    limbs_slice_add_mul_limb_same_length_in_place_left(w5, w2, 45);
    assert!(w5[0].even());
    limbs_slice_shr_in_place(w5, 1);
    limbs_sub_same_length_in_place_left(w4, w2);
    limbs_div_exact_3_in_place(w4);
    limbs_sub_same_length_in_place_left(w2, w4);
    limbs_sub_same_length_in_place_right(w5, w1);
    limbs_shl_to_out(scratch, w3, 3);
    limbs_sub_same_length_in_place_left(w5, &scratch[..m]);
    limbs_div_exact_limb_in_place(w5, 9);
    limbs_sub_same_length_in_place_left(w3, w5);
    limbs_div_exact_limb_in_place(w1, 15);
    limbs_slice_add_same_length_in_place_left(w1, w5);
    assert!(w1[0].even());
    limbs_slice_shr_in_place(w1, 1); // w1 >= 0 now
    limbs_sub_same_length_in_place_left(w5, w1);
    // These bounds are valid for the 4x4 polynomial product of toom44, and they are conservative
    // for toom53 and toom62.
    let two_n = n << 1;
    assert!(w1[two_n] < 2);
    assert!(w2[two_n] < 3);
    assert!(w3[two_n] < 4);
    assert!(w4[two_n] < 3);
    assert!(w5[two_n] < 2);
    // Addition chain. Note carries and the 2n'th limbs that need to be added in.
    //
    // Special care is needed for w2[2 * n] and the corresponding carry, since the "simple" way of
    // adding it all together would overwrite the limb at wp[2 * n] and out[4 * n] (same location)
    // with the sum of the high half of w3 and the low half of w4.
    // ```
    //
    //         7    6    5    4    3    2    1    0
    //    |    |    |    |    |    |    |    |    |
    //                  ||w3 (2n+1)|
    //             ||w4 (2n+1)|
    //        ||w5 (2n+1)|        ||w1 (2n+1)|
    //  +     |w6(n_high)|        ||w2 (2n+1)| w0 (2n) |  (share storage with r)
    //  -----------------------------------------------
    //  r |    |    |    |    |    |    |    |    |
    //        c7   c6   c5   c4   c3                 Carries to propagate
    // ```
    let (out_lo, out_hi) = out[n..].split_at_mut(m);
    if limbs_slice_add_same_length_in_place_left(out_lo, w1) {
        assert!(!limbs_slice_add_limb_in_place(&mut out_hi[..n], 1));
    }
    split_into_chunks_mut!(&mut out[3 * n..], n, [out_3, out_4, out_5], remainder);
    let mut addend = out_4[0];
    let (w3_lo, w3_hi) = w3.split_at_mut(n);
    if limbs_slice_add_same_length_in_place_left(out_3, w3_lo) {
        addend.wrapping_add_assign(1);
    }
    assert!(!limbs_slice_add_limb_in_place(w3_hi, addend));
    let (w3_hi_last, w3_hi_init) = w3_hi.split_last_mut().unwrap();
    let mut addend = *w3_hi_last;
    let (w4_lo, w4_hi) = w4.split_at_mut(n);
    if limbs_add_same_length_to_out(out_4, w3_hi_init, w4_lo) {
        addend += 1;
    }
    assert!(!limbs_slice_add_limb_in_place(w4_hi, addend));
    let (w4_last, w4_init) = w4_hi.split_last_mut().unwrap();
    let mut addend = *w4_last;
    let (w5_lo, w5_hi) = w5.split_at_mut(n);
    if limbs_add_same_length_to_out(out_5, w4_init, w5_lo) {
        addend += 1;
    }
    assert!(!limbs_slice_add_limb_in_place(w5_hi, addend));
    if n_high > n + 1 {
        assert!(!limbs_slice_add_greater_in_place_left(remainder, w5_hi));
    } else {
        let (w5_hi_lo, w5_hi_hi) = w5_hi.split_at_mut(n_high);
        assert!(!limbs_slice_add_same_length_in_place_left(
            &mut remainder[..n_high],
            w5_hi_lo,
        ));
        if WANT_ASSERT && n + n_high < m {
            slice_test_zero(w5_hi_hi);
        }
    }
}

// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ys.len()`.
//
// This is equivalent to `DO_mpn_sublsh_n` from `mpn/generic/toom_interpolate_8pts.c`, GMP 6.2.1.
pub_test! {limbs_shl_and_sub_same_length(
    xs: &mut [Limb],
    ys: &[Limb],
    shift: u64,
    scratch: &mut [Limb],
) -> Limb {
    let n = ys.len();
    let mut carry = limbs_shl_to_out(scratch, ys, shift);
    if limbs_sub_same_length_in_place_left(&mut xs[..n], &scratch[..n]) {
        carry.wrapping_add_assign(1);
    }
    carry
}}

// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), ys.len())`.
//
// This is equivalent to `DO_mpn_subrsh` from `mpn/generic/toom_interpolate_8pts.c`, GMP 6.2.1.
fn limbs_shl_and_sub(xs: &mut [Limb], ys: &[Limb], shift: u64, scratch: &mut [Limb]) {
    let (ys_head, ys_tail) = ys.split_first().unwrap();
    assert!(!limbs_sub_limb_in_place(xs, *ys_head >> shift));
    let carry = limbs_shl_and_sub_same_length(xs, ys_tail, Limb::WIDTH - shift, scratch);
    assert!(!limbs_sub_limb_in_place(&mut xs[ys.len() - 1..], carry));
}

fn limbs_shl_and_sub_special(
    xs_init: &mut [Limb],
    xs_last: &mut Limb,
    ys: &[Limb],
    shift: u64,
    scratch: &mut [Limb],
) {
    let (ys_head, ys_tail) = ys.split_first().unwrap();
    if limbs_sub_limb_in_place(xs_init, *ys_head >> shift) {
        *xs_last = xs_last.checked_sub(1).unwrap();
    }
    let carry = limbs_shl_and_sub_same_length(xs_init, ys_tail, Limb::WIDTH - shift, scratch);
    if limbs_sub_limb_in_place(&mut xs_init[ys_tail.len()..], carry) {
        *xs_last = xs_last.checked_sub(1).unwrap();
    }
}

// Interpolation for Toom-4.5 (or Toom-4), using the evaluation points: infinity(4.5 only), 4, -4,
// 2, -2, 1, -1, 0. More precisely, we want to compute f(2 ^ (`Limb::WIDTH` * n)) for a polynomial f
// of degree 7 (or 6), given the 8 (rsp. 7) values:
//
// - r1 = limit at infinity of f(x) / x ^ 7,
// - r2 = f(4),
// - r3 = f(-4),
// - r4 = f(2),
// - r5 = f(-2),
// - r6 = f(1),
// - r7 = f(-1),
// - r8 = f(0).
//
// All couples of the form f(n),f(-n) must be already mixed with
// `limbs_toom_couple_handling`(f(n),..., f(-n), ...)
//
// The result is stored in {`out`, `s_plus_t` + 7 * n (or 6 * n)}. At entry, `r8` is stored at
// {`out`, 2 * `n`}, and r5 is stored at {`out` + 3 * `n`, 3 * `n` + 1}.
//
// The other values are 2 * `n` + ... limbs each (with most significant limbs small).
//
// All intermediate results are positive. Inputs are destroyed.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `n`.
//
// This is equivalent to `mpn_toom_interpolate_8pts` from `mpn/generic/toom_interpolate_8pts.c`, GMP
// 6.2.1, but the argument spt == `s_plus_t` is moved to immediately after `n`.
pub(crate) fn limbs_mul_toom_interpolate_8_points(
    out: &mut [Limb],
    n: usize,
    s_plus_t: usize,
    r3: &mut [Limb],
    r7: &mut [Limb],
    scratch: &mut [Limb],
) {
    assert!(s_plus_t >= n);
    let m = 3 * n + 1;
    assert_eq!(r3.len(), m);
    assert_eq!(r7.len(), m);
    let (out, remainder) = out.split_at_mut(n << 1);
    let (out_2, remainder) = remainder.split_at_mut(n);
    let (r5, r1) = remainder.split_at_mut(n << 2);
    let r1 = &mut r1[..s_plus_t];
    let r5_lo = &mut r5[..m];
    // Interpolation
    limbs_shl_and_sub(&mut r3[n..], out, 4, scratch);
    let carry = limbs_shl_and_sub_same_length(r3, r1, 12, scratch);
    assert!(!limbs_sub_limb_in_place(&mut r3[s_plus_t..], carry));
    limbs_shl_and_sub(&mut r5_lo[n..], out, 2, scratch);
    let carry = limbs_shl_and_sub_same_length(r5_lo, r1, 6, scratch);
    assert!(!limbs_sub_limb_in_place(&mut r5_lo[s_plus_t..], carry));
    let (r7_last, r7_init) = r7.split_last_mut().unwrap();
    if limbs_sub_same_length_in_place_left(&mut r7_init[n..], out) {
        r7_last.wrapping_sub_assign(1);
    }
    let (r7_lo, r7_hi) = r7.split_at_mut(s_plus_t);
    if limbs_sub_same_length_in_place_left(r7_lo, r1) {
        assert!(!limbs_sub_limb_in_place(r7_hi, 1));
    }
    assert!(!limbs_sub_same_length_in_place_left(r3, r5_lo));
    assert_eq!(limbs_slice_shr_in_place(r3, 2), 0);
    assert!(!limbs_sub_same_length_in_place_left(r5_lo, r7));
    assert!(!limbs_sub_same_length_in_place_left(r3, r5_lo));
    limbs_div_exact_limb_in_place(r3, 45);
    limbs_div_exact_3_in_place(r5_lo);
    assert_eq!(limbs_shl_and_sub_same_length(r5_lo, r3, 2, scratch), 0);
    // Last interpolation steps are mixed with recomposition.
    //
    // out[] prior to operations:
    // ```
    // |_H r1|_L r1|____||_H r5|_M_r5|_L r5|_____|_H r8|_L r8|out
    // ```
    //
    // summation scheme for remaining operations:
    // ```
    // |____8|n___7|n___6|n___5|n___4|n___3|n___2|n____|n____|out
    // |_H r1|_L r1|____||_H*r5|_M r5|_L r5|_____|_H_r8|_L r8|out
    //  ||_H r3|_M r3|_L*r3|
    //              ||_H_r7|_M_r7|_L_r7|
    //          ||-H r3|-M r3|-L*r3|
    //              ||-H*r5|-M_r5|-L_r5|
    // Hr8+Lr7-Lr5
    // ```
    split_into_chunks_mut!(r5_lo, n, [r5_lo_0, r5_lo_1, r5_lo_2], r5_lo_3);
    let (r7_lo, r7) = r7.split_at_mut(n);
    let out = &mut out[n..];
    let carry_1 = limbs_slice_add_same_length_in_place_left(out, r7_lo);
    let carry_2 = limbs_sub_same_length_in_place_left(out, r5_lo_0);
    if carry_1 && !carry_2 {
        assert!(!limbs_slice_add_limb_in_place(r7, 1));
    } else if !carry_1 && carry_2 {
        assert!(!limbs_sub_limb_in_place(r7, 1));
    }
    let (r7_lo, r7_hi) = r7.split_at_mut(n);
    // Mr7-Mr5
    if limbs_sub_same_length_to_out(out_2, r7_lo, r5_lo_1) {
        assert!(!limbs_sub_limb_in_place(r7_hi, 1));
    }
    // Hr5+Lr3
    let (r3_lo, r3) = r3.split_at_mut(n);
    if limbs_slice_add_same_length_in_place_left(r5_lo_2, r3_lo) {
        r5_lo_3[0].wrapping_add_assign(1);
    }
    // Hr7+Lr5
    let carry_1 = limbs_slice_add_same_length_in_place_left(&mut r5_lo[..=n], r7_hi);
    // Hr7-Hr5+Lr5-Lr3
    let (r5_lo_lo, r5_lo_hi) = r5_lo.split_at_mut(n << 1);
    let carry_2 = limbs_sub_same_length_in_place_left(&mut r5_lo_lo[..=n], r5_lo_hi);
    if carry_1 && !carry_2 {
        assert!(!limbs_slice_add_limb_in_place(&mut r5_lo[n + 1..], 1));
    } else if !carry_1 && carry_2 {
        assert!(!limbs_sub_limb_in_place(&mut r5_lo[n + 1..], 1));
    }
    // Mr5-Mr3,Hr5-Hr3
    assert!(!limbs_sub_same_length_in_place_left(&mut r5_lo[n..], r3));
    let r5_3n = r5[3 * n];
    let (r3_lo, r3) = r3.split_at_mut(n);
    if limbs_add_limb_to_out(&mut r5[3 * n..], r3_lo, r5_3n) {
        assert!(!limbs_slice_add_limb_in_place(r3, 1));
    }
    let mut r3_n = r3[n];
    let (r1_lo, r1_hi) = r1.split_at_mut(n);
    if limbs_slice_add_same_length_in_place_left(r1_lo, &r3[..n]) {
        r3_n.wrapping_add_assign(1);
    }
    if s_plus_t == n {
        assert_eq!(r3_n, 0);
    } else {
        assert!(!limbs_slice_add_limb_in_place(r1_hi, r3_n));
    }
}

// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
fn limbs_div_255_in_place(xs: &mut [Limb]) {
    limbs_div_divisor_of_limb_max_with_carry_in_place(xs, Limb::MAX / 255, 0);
}

fn limbs_aors_mul_or_two_sh_aors_helper(
    xs: &mut [Limb],
    ys: &[Limb],
    s: Limb,
    sign: bool,
    s1: u64,
    sign1: bool,
    s2: u64,
    sign2: bool,
    scratch: &mut [Limb],
) {
    if AORSMUL_FASTER_2AORSLSH {
        if sign {
            limbs_slice_add_mul_limb_same_length_in_place_left(xs, ys, s);
        } else {
            limbs_sub_mul_limb_same_length_in_place_left(xs, ys, s);
        }
    } else {
        if sign1 {
            limbs_shl_and_add_same_length_in_place_left(xs, ys, s1, scratch);
        } else {
            limbs_shl_and_sub_same_length(xs, ys, s1, scratch);
        }
        if sign2 {
            limbs_shl_and_add_same_length_in_place_left(xs, ys, s2, scratch);
        } else {
            limbs_shl_and_sub_same_length(xs, ys, s2, scratch);
        }
    }
}

fn limbs_aors_mul_or_three_sh_aors_helper(
    xs: &mut [Limb],
    ys: &[Limb],
    s: Limb,
    s1: u64,
    s2: u64,
    s3: u64,
    no_carry: bool,
    scratch: &mut [Limb],
) {
    if AORSMUL_FASTER_3AORSLSH {
        let c = limbs_sub_mul_limb_same_length_in_place_left(xs, ys, s);
        if no_carry {
            assert_eq!(c, 0);
        }
    } else {
        let c = limbs_shl_and_sub_same_length(xs, ys, s1, scratch);
        if no_carry {
            assert_eq!(c, 0);
        }
        let c = limbs_shl_and_sub_same_length(xs, ys, s2, scratch);
        if no_carry {
            assert_eq!(c, 0);
        }
        let c = limbs_shl_and_sub_same_length(xs, ys, s3, scratch);
        if no_carry {
            assert_eq!(c, 0);
        }
    }
}

fn limbs_aors_mul_or_aors_and_two_sh_aors_helper(
    xs: &mut [Limb],
    ys: &[Limb],
    s: Limb,
    s1: u64,
    s2: u64,
    scratch: &mut [Limb],
) {
    if AORSMUL_FASTER_AORS_2AORSLSH {
        assert_eq!(limbs_sub_mul_limb_same_length_in_place_left(xs, ys, s), 0);
    } else {
        assert!(!limbs_sub_same_length_in_place_left(xs, ys));
        assert_eq!(
            limbs_shl_and_add_same_length_in_place_left(xs, ys, s1, scratch),
            0
        );
        assert_eq!(limbs_shl_and_sub_same_length(xs, ys, s2, scratch), 0);
    }
}

fn limbs_aors_mul_or_aors_and_sh_aors_helper(
    xs: &mut [Limb],
    ys: &[Limb],
    s: Limb,
    sign1: bool,
    s2: u64,
    scratch: &mut [Limb],
) {
    if AORSMUL_FASTER_AORS_AORSLSH {
        limbs_sub_mul_limb_same_length_in_place_left(xs, ys, s);
    } else {
        if sign1 {
            limbs_slice_add_same_length_in_place_left(xs, ys);
        } else {
            limbs_sub_same_length_in_place_left(xs, ys);
        }
        limbs_shl_and_sub_same_length(xs, ys, s2, scratch);
    }
}

// Interpolation for Toom-6.5 (or Toom-6), using the evaluation points:
// - Infinity(6.5 only), +-4, +-2, +-1, +-1/4, +-1/2, 0.
//
// More precisely, we want to compute f(2 ^ (`Limb::WIDTH` * n)) for a polynomial f of degree 11 (or
// 10), given the 12 (resp. 11) values:
//
// - r0 = limit at infinity of f(x) / x ^ 7,
// - r1 = f(4),f(-4),
// - r2 = f(2),f(-2),
// - r3 = f(1),f(-1),
// - r4 = f(1 / 4), f(-1 / 4),
// - r5 = f(1 / 2), f(-1 / 2),
// - r6 = f(0).
//
// All couples of the form f(n),f(-n) must be already mixed with `limbs_toom_couple_handling`(f(n),
// ..., f(-n),...)
//
// - The result is stored in {out, s_plus_t + 7 * n (or 6 * n)}.
// - At entry, r6 is stored at {out, 2 * n},
// - r4 is stored at {out +  3 * n, 3 * n + 1}.
// - r2 is stored at {out +  7 * n, 3 * n + 1}.
// - r0 is stored at {out + 11 * n, s_plus_t}.
//
// The other values are 3 * n + 1 limbs each (with most significant limbs small).
//
// Negative intermediate results are stored two-complemented. Inputs are destroyed.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `n`.
//
// This is equivalent to `mpn_toom_interpolate_12pts` from `mpn/generic/toom_interpolate_12pts.c`,
// GMP 6.2.1.
pub_crate_test! {limbs_mul_toom_interpolate_12_points<'a>(
    out: &mut [Limb],
    mut r1: &'a mut [Limb],
    r3: &mut [Limb],
    mut r5: &'a mut [Limb],
    n: usize,
    s_plus_t: usize,
    half: bool,
    mut scratch: &'a mut [Limb],
) {
    let m = 3 * n + 1;
    assert_eq!(r1.len(), m);
    assert_eq!(r3.len(), m);
    assert_eq!(r5.len(), m);
    let (out_lo, remainder) = out.split_at_mut(3 * n);
    let out_lo = &mut out_lo[..n << 1];
    let (r4, r2) = remainder.split_at_mut(n << 2);
    let r4 = &mut r4[..m];
    // Interpolation
    if half {
        let (r2, r0) = r2.split_at_mut(4 * n);
        let r0 = &mut r0[..s_plus_t];
        let (r3_lo, r3_hi) = r3.split_at_mut(s_plus_t);
        if limbs_sub_same_length_in_place_left(r3_lo, r0) {
            assert!(!limbs_sub_limb_in_place(r3_hi, 1));
        }
        let carry = limbs_shl_and_sub_same_length(r2, r0, 10, scratch);
        assert!(!limbs_sub_limb_in_place(&mut r2[s_plus_t..m], carry));
        limbs_shl_and_sub(r5, r0, 2, scratch);
        let carry = limbs_shl_and_sub_same_length(r1, r0, 20, scratch);
        assert!(!limbs_sub_limb_in_place(&mut r1[s_plus_t..], carry));
        limbs_shl_and_sub(r4, r0, 4, scratch);
    };
    let r2 = &mut r2[..m];
    let carry = limbs_shl_and_sub_same_length(&mut r4[n..], out_lo, 20, scratch);
    r4.last_mut().unwrap().wrapping_sub_assign(carry);
    limbs_shl_and_sub(&mut r1[n..], out_lo, 4, scratch);
    assert!(!limbs_add_same_length_to_out(scratch, r1, r4));
    limbs_sub_same_length_in_place_left(r4, r1); // can be negative
    swap(&mut r1, &mut scratch);
    let r1 = &mut r1[..m];
    let carry = limbs_shl_and_sub_same_length(&mut r5[n..], out_lo, 10, scratch);
    r5.last_mut().unwrap().wrapping_sub_assign(carry);
    limbs_shl_and_sub(&mut r2[n..], out_lo, 2, scratch);
    limbs_sub_same_length_to_out(scratch, r5, r2); // can be negative
    assert!(!limbs_slice_add_same_length_in_place_left(r2, r5));
    swap(&mut r5, &mut scratch);
    let (r3_last, r3_init) = r3.split_last_mut().unwrap();
    if limbs_sub_same_length_in_place_left(&mut r3_init[n..], out_lo) {
        r3_last.wrapping_sub_assign(1);
    }
    limbs_aors_mul_or_aors_and_sh_aors_helper(r4, r5, 257, false, 8, scratch);
    // A division by 2835 * 4 follows. Warning: the operand can be negative!
    limbs_div_exact_limb_in_place(r4, 2835 << 2);
    let r4_last = r4.last_mut().unwrap();
    if r4_last.leading_zeros() < 3 {
        *r4_last |= Limb::MAX << (Limb::WIDTH - 2);
    }
    limbs_aors_mul_or_two_sh_aors_helper(r5, r4, 60, true, 2, false, 6, true, scratch);
    limbs_div_255_in_place(r5);
    assert_eq!(limbs_shl_and_sub_same_length(r2, r3, 5, scratch), 0);
    limbs_aors_mul_or_three_sh_aors_helper(r1, r2, 100, 6, 5, 2, true, scratch);
    assert_eq!(limbs_shl_and_sub_same_length(r1, r3, 9, scratch), 0);
    limbs_div_exact_limb_in_place(r1, 42525);
    limbs_aors_mul_or_aors_and_two_sh_aors_helper(r2, r1, 225, 5, 8, scratch);
    limbs_div_exact_limb_in_place(r2, 9 << 2);
    assert!(!limbs_sub_same_length_in_place_left(r3, r2));
    limbs_sub_same_length_in_place_right(r2, r4);
    assert_eq!(limbs_slice_shr_in_place(r4, 1), 0);
    assert!(!limbs_sub_same_length_in_place_left(r2, r4));
    let r1 = &mut r1[..m];
    limbs_slice_add_same_length_in_place_left(r5, r1);
    assert_eq!(limbs_slice_shr_in_place(r5, 1), 0);
    // Last interpolation steps...
    assert!(!limbs_sub_same_length_in_place_left(r3, r1));
    assert!(!limbs_sub_same_length_in_place_left(r1, r5));
    // ...could be mixed with recomposition
    // ```
    // ||H-r5|M-r5|L-r5|   ||H-r1|M-r1|L-r1|
    // ```
    //
    // Recomposition
    //
    // out[] prior to operations:
    // ```
    // |M r0|L r0|___||H r2|M r2|L r2|___||H r4|M r4|L r4|____|H_r6|L r6|out
    // ```
    //
    // Summation scheme for remaining operations:
    // ```
    // |__12|n_11|n_10|n__9|n__8|n__7|n__6|n__5|n__4|n__3|n__2|n___|n___|out
    // |M r0|L r0|___||H r2|M r2|L r2|___||H r4|M r4|L r4|____|H_r6|L r6|out
    // ||H r1|M r1|L r1|   ||H r3|M r3|L r3|   ||H_r5|M_r5|L_r5|
    // ```
    split_into_chunks_mut!(out, n, [_unused, out_1, out_2, out_3], out_4);
    split_into_chunks_mut!(r5, n, [r5_0, r5_1], r5_2);
    if limbs_slice_add_same_length_in_place_left(out_1, r5_0) {
        if limbs_add_limb_to_out(out_2, r5_1, 1) {
            assert!(!limbs_slice_add_limb_in_place(r5_2, 1));
        }
    } else {
        out_2.copy_from_slice(r5_1);
    }
    let (r5_last, r5_2) = r5_2.split_last_mut().unwrap();
    let mut carry = *r5_last;
    if limbs_slice_add_same_length_in_place_left(out_3, r5_2) {
        carry.wrapping_add_assign(1);
    }
    assert!(!limbs_slice_add_limb_in_place(
        &mut out_4[..=n<<1],
        carry,
    ));
    split_into_chunks_mut!(out_4, n, [_unused, out_5, out_6, out_7], out_8);
    split_into_chunks_mut!(r3, n, [r3_0, r3_1], r3_2);
    if limbs_slice_add_same_length_in_place_left(out_5, r3_0) {
        out_6[0].wrapping_add_assign(1);
    }
    let out_6_first = out_6[0];
    if limbs_add_limb_to_out(out_6, r3_1, out_6_first) {
        assert!(!limbs_slice_add_limb_in_place(r3_2, 1));
    }
    let (r3_last, r3_2) = r3_2.split_last_mut().unwrap();
    let mut carry = *r3_last;
    if limbs_slice_add_same_length_in_place_left(out_7, r3_2) {
        carry.wrapping_add_assign(1);
    }
    assert!(!limbs_slice_add_limb_in_place(
        &mut out_8[..=n<<1],
        carry,
    ));
    split_into_chunks_mut!(out_8, n, [_unused, out_9], out_10);
    let (r1_0, r1_1) = r1.split_at_mut(n);
    if limbs_slice_add_same_length_in_place_left(out_9, r1_0) {
        out_10[0].wrapping_add_assign(1);
    }
    let out_10_first = out_10[0];
    if half {
        let (out_10, out_11) = out_10.split_at_mut(n);
        let (r1_1, r1_2) = r1_1.split_at_mut(n);
        if limbs_add_limb_to_out(out_10, r1_1, out_10_first) {
            assert!(!limbs_slice_add_limb_in_place(r1_2, 1));
        }
        if s_plus_t > n {
            let (out_11, out_12) = out_11.split_at_mut(n);
            let (r1_last, r1_2) = r1_2.split_last_mut().unwrap();
            let mut carry = *r1_last;
            if limbs_slice_add_same_length_in_place_left(out_11, r1_2) {
                carry.wrapping_add_assign(1);
            }
            assert!(!limbs_slice_add_limb_in_place(
                &mut out_12[..s_plus_t - n],
                carry,
            ));
        } else {
            assert!(!limbs_slice_add_same_length_in_place_left(
                &mut out_11[..s_plus_t],
                &r1_2[..s_plus_t],
            ));
        }
    } else {
        assert!(!limbs_add_limb_to_out(
            out_10,
            &r1_1[..s_plus_t],
            out_10_first,
        ));
    }
}}

#[cfg(feature = "32_bit_limbs")]
const CORRECTED_WIDTH: u64 = 42 - Limb::WIDTH;
#[cfg(not(feature = "32_bit_limbs"))]
const CORRECTED_WIDTH: u64 = 42;

// Interpolation for Toom-8.5 (or Toom-8), using the evaluation points: Infinity(8.5 only), +-8,
// +-4, +-2, +-1, +-1/4, +-1/2, +-1/8, 0.
//
// More precisely, we want to compute f(2 ^ (`Limb::WIDTH` * n)) for a polynomial f of degree 15 (or
// 14), given the 16 (rsp. 15) values:
//
// - r0 = limit at infinity of f(x) / x ^ 7,
// - r1 = f(8), f(-8),
// - r2 = f(4), f(-4),
// - r3 = f(2), f(-2),
// - r4 = f(1), f(-1),
// - r5 = f(1/4), f(-1/4),
// - r6 = f(1/2), f(-1/2),
// - r7 = f(1/8), f(-1/8),
// - r8 = f(0).
//
// All couples of the form f(n),f(-n) must be already mixed with
// toom_couple_handling(f(n),...,f(-n),...)
//
// - The result is stored in {out, s_plus_t + 7 * n (or 8 * n)}.
// - At entry, r8 is stored at {out, 2 * n},
// - r6 is stored at {out + 3 * n, 3 * n + 1}.
// - r4 is stored at {out + 7 * n, 3 * n + 1}.
// - r2 is stored at {out + 11 * n, 3 * n + 1}.
// - r0 is stored at {out + 15 * n, s_plus_t}.
//
// The other values are 3 * n + 1 limbs each (with most significant limbs small).
//
// Negative intermediate results are stored two-complemented. Inputs are destroyed.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `n`.
//
// This is equivalent to `mpn_toom_interpolate_16pts` from `mpn/generic/toom_interpolate_16pts.c`,
// GMP 6.2.1.
pub_crate_test! {limbs_mul_toom_interpolate_16_points<'a>(
    out: &mut [Limb],
    r1: &mut [Limb],
    mut r3: &'a mut [Limb],
    mut r5: &'a mut [Limb],
    mut r7: &'a mut [Limb],
    n: usize,
    s_plus_t: usize,
    half: bool,
    mut scratch: &'a mut [Limb],
) {
    let m = 3 * n + 1;
    assert!(s_plus_t <= n << 1);
    assert_eq!(r1.len(), m);
    assert_eq!(r3.len(), m);
    assert_eq!(r5.len(), m);
    assert_eq!(r7.len(), m);
    let (pp_lo, remainder) = out.split_at_mut(3 * n);
    let pp_lo = &mut pp_lo[..n << 1];
    split_into_chunks_mut!(remainder, n << 2, [r6, r4], r2);
    let r4 = &mut r4[..m];
    let r6 = &mut r6[..m];
    // Interpolation
    if half {
        let (r2, r0) = r2.split_at_mut(n << 2);
        let r0 = &mut r0[..s_plus_t];
        let r2 = &mut r2[..m];
        let (r4_lo, r4_hi) = r4.split_at_mut(s_plus_t);
        if limbs_sub_same_length_in_place_left(r4_lo, r0) {
            assert!(!limbs_sub_limb_in_place(r4_hi, 1));
        }
        let carry = limbs_shl_and_sub_same_length(r3, r0, 14, scratch);
        assert!(!limbs_sub_limb_in_place(&mut r3[s_plus_t..], carry));
        limbs_shl_and_sub(r6, r0, 2, scratch);
        let carry = limbs_shl_and_sub_same_length(r2, r0, 28, scratch);
        assert!(!limbs_sub_limb_in_place(&mut r2[s_plus_t..], carry));
        limbs_shl_and_sub(r5, r0, 4, scratch);
        if BIT_CORRECTION {
            let carry = limbs_shl_and_sub_same_length(&mut r1[1..], r0, CORRECTED_WIDTH, scratch);
            limbs_sub_limb_in_place(&mut r1[s_plus_t + 1..], carry);
            let r5_first = r5.first_mut().unwrap();
            let carry = *r5_first;
            *r5_first = 0x80;
            limbs_shl_and_sub_special(r7, r5_first, r0, 6, scratch);
            *r5_first = carry;
        } else {
            let carry = limbs_shl_and_sub_same_length(r1, r0, CORRECTED_WIDTH, scratch);
            assert!(!limbs_sub_limb_in_place(&mut r1[s_plus_t..], carry));
            limbs_shl_and_sub(r7, r0, 6, scratch);
        }
    }
    let r2 = &mut r2[..m];
    let (r5_last, r5_init) = r5[n..].split_last_mut().unwrap();
    r5_last.wrapping_sub_assign(limbs_shl_and_sub_same_length(r5_init, pp_lo, 28, scratch));
    limbs_shl_and_sub(&mut r2[n..], pp_lo, 4, scratch);
    limbs_sub_same_length_to_out(scratch, r5, r2); // can be negative
    assert!(!limbs_slice_add_same_length_in_place_left(r2, r5));
    swap(&mut r5, &mut scratch);
    let (r6_last, r6_init) = r6[n..].split_last_mut().unwrap();
    r6_last.wrapping_sub_assign(limbs_shl_and_sub_same_length(r6_init, pp_lo, 14, scratch));
    limbs_shl_and_sub(&mut r3[n..], pp_lo, 2, scratch);
    assert!(!limbs_add_same_length_to_out(scratch, r3, r6));
    limbs_sub_same_length_in_place_left(r6, r3); // can be negative
    swap(&mut r3, &mut scratch);
    let r1_hi = &mut r1[n..];
    if BIT_CORRECTION {
        limbs_shl_and_sub_same_length(&mut r7[n + 1..], pp_lo, CORRECTED_WIDTH, scratch);
        let (pp_lo_first, pp_lo_tail) = pp_lo.split_first().unwrap();
        assert!(!limbs_sub_limb_in_place(r1_hi, pp_lo_first >> 6));
        let carry = limbs_shl_and_sub_same_length(r1_hi, pp_lo_tail, Limb::WIDTH - 6, scratch);
        limbs_sub_limb_in_place(&mut r1_hi[2 * n - 1..], carry);
    } else {
        let carry = limbs_shl_and_sub_same_length(&mut r7[n..], pp_lo, CORRECTED_WIDTH, scratch);
        r7.last_mut().unwrap().wrapping_sub_assign(carry);
        limbs_shl_and_sub(r1_hi, pp_lo, 6, scratch);
    }
    // can be negative
    limbs_sub_same_length_to_out(scratch, r7, r1);
    // if BIT_CORRECTION, can give a carry.
    limbs_slice_add_same_length_in_place_left(r1, r7);
    swap(&mut r7, &mut scratch);
    let (r4_last, r4_init) = r4[n..].split_last_mut().unwrap();
    if limbs_sub_same_length_in_place_left(r4_init, pp_lo) {
        r4_last.wrapping_sub_assign(1);
    }
    limbs_aors_mul_or_two_sh_aors_helper(r5, r6, 1028, false, 2, false, 10, false, scratch);
    limbs_sub_mul_limb_same_length_in_place_left(r7, r5, 1300); // can be negative
    limbs_aors_mul_or_three_sh_aors_helper(r7, r6, 1052688, 4, 12, 20, false, scratch);
    limbs_div_exact_limb_in_place(r7, 188513325);
    limbs_div_255_in_place(r7);
    // can be negative
    limbs_sub_mul_limb_same_length_in_place_left(r5, r7, 12567555);
    // A division by 2835x64 follows. Warning: the operand can be negative!
    limbs_div_exact_limb_in_place(r5, 2835 << 6);
    let r5_last = r5.last_mut().unwrap();
    if r5_last.leading_zeros() < 7 {
        *r5_last |= Limb::MAX << (Limb::WIDTH - 6);
    }
    limbs_aors_mul_or_aors_and_sh_aors_helper(r6, r7, 4095, true, 12, scratch);
    limbs_aors_mul_or_two_sh_aors_helper(r6, r5, 240, true, 8, true, 4, false, scratch);
    // A division by 255x4 follows. Warning: the operand can be negative!
    limbs_div_exact_limb_in_place(r6, 255 << 2);
    let r6_last = r6.last_mut().unwrap();
    if r6_last.leading_zeros() < 3 {
        *r6_last |= Limb::MAX << (Limb::WIDTH - 2);
    }
    assert_eq!(limbs_shl_and_sub_same_length(r3, r4, 7, scratch), 0);
    assert_eq!(limbs_shl_and_sub_same_length(r2, r4, 13, scratch), 0);
    assert_eq!(limbs_sub_mul_limb_same_length_in_place_left(r2, r3, 400), 0);
    // If `Limb::WIDTH` < 42 next operations on r1 can give a carry!
    limbs_shl_and_sub_same_length(r1, r4, 19, scratch);
    limbs_sub_mul_limb_same_length_in_place_left(r1, r2, 1428);
    limbs_sub_mul_limb_same_length_in_place_left(r1, r3, 112896);
    limbs_div_exact_limb_in_place(r1, 182712915);
    limbs_div_255_in_place(r1);
    assert_eq!(
        limbs_sub_mul_limb_same_length_in_place_left(r2, r1, 15181425),
        0
    );
    limbs_div_exact_limb_in_place(r2, 42525 << 4);
    limbs_aors_mul_or_aors_and_two_sh_aors_helper(r3, r1, 3969, 7, 12, scratch);
    assert_eq!(limbs_sub_mul_limb_same_length_in_place_left(r3, r2, 900), 0);
    limbs_div_exact_limb_in_place(r3, 9 << 4);
    assert!(!limbs_sub_same_length_in_place_left(r4, r1));
    assert!(!limbs_sub_same_length_in_place_left(r4, r3));
    assert!(!limbs_sub_same_length_in_place_left(r4, r2));
    limbs_slice_add_same_length_in_place_left(r6, r2);
    assert_eq!(limbs_slice_shr_in_place(r6, 1), 0);
    assert!(!limbs_sub_greater_in_place_left(r2, r6));
    limbs_sub_same_length_in_place_right(r3, r5);
    assert_eq!(limbs_slice_shr_in_place(r5, 1), 0);
    assert!(!limbs_sub_same_length_in_place_left(r3, r5));
    limbs_slice_add_same_length_in_place_left(r7, r1);
    assert_eq!(limbs_slice_shr_in_place(r7, 1), 0);
    assert!(!limbs_sub_same_length_in_place_left(r1, r7));
    // Last interpolation steps could be mixed with recomposition.
    // ```
    // ||H-r7|M-r7|L-r7|   ||H-r5|M-r5|L-r5|
    // ```
    //
    // Recomposition
    //
    // out[] prior to operations:
    // ```
    // |M r0|L r0|___||H r2|M r2|L r2|___||H r4|M r4|L r4|___||H r6|M r6|L r6|____|H_r8|L r8|out
    // ```
    //
    // summation scheme for remaining operations:
    // ```
    // |__16|n_15|n_14|n_13|n_12|n_11|n_10|n__9|n__8|n__7|n__6|n__5|n__4|n__3|n__2|n___|n___|out
    // |M r0|L r0|___||H r2|M r2|L r2|___||H r4|M r4|L r4|___||H r6|M r6|L r6|____|H_r8|L r8|out
    // ||H r1|M r1|L r1|   ||H r3|M r3|L r3|   ||H_r5|M_r5|L_r5|   ||H r7|M r7|L r7|
    // ```
    split_into_chunks_mut!(out, n, [_unused, out_1, out_2, out_3], out_4);
    split_into_chunks_mut!(r7, n, [r7_0, r7_1], r7_2);
    if limbs_slice_add_same_length_in_place_left(out_1, r7_0) {
        if limbs_add_limb_to_out(out_2, r7_1, 1) {
            assert!(!limbs_slice_add_limb_in_place(r7_2, 1));
        }
    } else {
        out_2.copy_from_slice(r7_1);
    }
    let (r7_last, r7_2) = r7_2.split_last_mut().unwrap();
    let mut carry = *r7_last;
    if limbs_slice_add_same_length_in_place_left(out_3, r7_2) {
        carry.wrapping_add_assign(1);
    }
    assert!(!limbs_slice_add_limb_in_place(out_4, carry));
    split_into_chunks_mut!(out_4, n, [_unused, out_5, out_6, out_7], out_8);
    split_into_chunks_mut!(r5, n, [r5_0, r5_1], r5_2);
    if limbs_slice_add_same_length_in_place_left(out_5, r5_0) {
        out_6[0].wrapping_add_assign(1);
    }
    let out_6_first = out_6[0];
    if limbs_add_limb_to_out(out_6, r5_1, out_6_first) {
        assert!(!limbs_slice_add_limb_in_place(r5_2, 1));
    }
    let (r5_last, r5_2) = r5_2.split_last_mut().unwrap();
    let mut carry = *r5_last;
    if limbs_slice_add_same_length_in_place_left(out_7, r5_2) {
        carry.wrapping_add_assign(1);
    }
    assert!(!limbs_slice_add_limb_in_place(out_8, carry));
    split_into_chunks_mut!(out_8, n, [_unused, out_9, out_10, out_11], out_12);
    split_into_chunks_mut!(r3, n, [r3_0, r3_1], r3_2);
    if limbs_slice_add_same_length_in_place_left(out_9, r3_0) {
        out_10[0].wrapping_add_assign(1);
    }
    let out_10_first = out_10[0];
    if limbs_add_limb_to_out(out_10, r3_1, out_10_first) {
        assert!(!limbs_slice_add_limb_in_place(r3_2, 1));
    }
    let (r3_last, r3_2) = r3_2.split_last_mut().unwrap();
    let mut carry = *r3_last;
    if limbs_slice_add_same_length_in_place_left(out_11, r3_2) {
        carry.wrapping_add_assign(1);
    }
    assert!(!limbs_slice_add_limb_in_place(out_12, carry));
    split_into_chunks_mut!(out_12, n, [_unused, out_13], out_14);
    let (r1_0, r1_1) = r1.split_at_mut(n);
    if limbs_slice_add_same_length_in_place_left(out_13, r1_0) {
        out_14[0].wrapping_add_assign(1);
    }
    let out_14_first = out_14[0];
    if half {
        let (out_14, out_15) = out_14.split_at_mut(n);
        let (r1_1, r1_2) = r1_1.split_at_mut(n);
        if limbs_add_limb_to_out(out_14, r1_1, out_14_first) {
            assert!(!limbs_slice_add_limb_in_place(r1_2, 1));
        }
        if s_plus_t > n {
            let (out_15, out_16) = out_15.split_at_mut(n);
            let (r1_last, r1_2) = r1_2.split_last_mut().unwrap();
            let mut carry = *r1_last;
            if limbs_slice_add_same_length_in_place_left(out_15, r1_2) {
                carry.wrapping_add_assign(1);
            }
            assert!(!limbs_slice_add_limb_in_place(
                &mut out_16[..s_plus_t - n],
                carry,
            ));
        } else {
            assert!(!limbs_slice_add_same_length_in_place_left(
                &mut out_15[..s_plus_t],
                &r1_2[..s_plus_t],
            ));
        }
    } else {
        assert!(!limbs_add_limb_to_out(
            &mut out_14[..s_plus_t],
            &r1_1[..s_plus_t],
            out_14_first,
        ));
    }
}}
