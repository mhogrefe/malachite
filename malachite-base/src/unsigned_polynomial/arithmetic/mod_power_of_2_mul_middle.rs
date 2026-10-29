// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::Parity;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_mul::{
    add_wrapping_assign, mask_coefficients, sub_wrapping_assign,
};
use crate::unsigned_polynomial::arithmetic::ntt::{NttOp, mul_middle_ntt, ntt_preferred};
use alloc::vec;
use core::cmp::{Ordering, min};

// Middle products of polynomials with word coefficients, in wrapping arithmetic, so correct modulo
// $2^k$ for every $k$ up to the width of the word. With `xs` of length $n$ and `ys` of length $n +
// k - 1$, the middle product is the $k$ coefficients of the product from coefficient $n - 1$ on, as
// for the middle products modulo $m$ in `mod_mul_middle`, whose algorithms these follow.

// The length at which Karatsuba middle products overtake classical ones in wrapping arithmetic.
// Measured on an Apple M-series machine, 2026-10, for `u64` with exponents of 8 to 64 and `u128`
// with 100 and 128: they break even below 32 and Karatsuba wins from 35.
const MUL_MIDDLE_WRAPPING_KARATSUBA_THRESHOLD: usize = 32;

// Sets `out` to the middle product of `xs` and `ys` modulo $2^W$, one coefficient at a time. `ys`
// must have length `xs.len() + out.len() - 1`.
fn mul_middle_classical_wrapping<T: PrimitiveUnsigned>(out: &mut [T], xs: &[T], ys: &[T]) {
    let n = xs.len();
    for (j, o) in out.iter_mut().enumerate() {
        let mut sum = T::ZERO;
        for (&x, &y) in xs.iter().zip(ys[j..j + n].iter().rev()) {
            sum.wrapping_add_assign(x.wrapping_mul(y));
        }
        *o = sum;
    }
}

// The scratch length needed by `mul_middle_balanced_wrapping` for `xs` of length `n`.
const fn mul_middle_wrapping_scratch_len(mut n: usize) -> usize {
    let mut len = 0;
    while n >= MUL_MIDDLE_WRAPPING_KARATSUBA_THRESHOLD {
        let h = n >> 1;
        // A sum of halves of `xs`, a difference of windows of `ys`, and one middle product.
        len += (h << 2) - 1;
        n = h;
    }
    len
}

// Sets `out` to the middle product of `xs` and `ys` modulo $2^W$, where `xs` and `out` have the
// same nonzero length $n$ and `ys` has length $2n - 1$, by Karatsuba's method for middle products
// (see `mod_mul_middle_balanced`), falling back to the classical method below the threshold.
fn mul_middle_balanced_wrapping<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    ys: &[T],
    scratch: &mut [T],
) {
    let n = xs.len();
    if n < MUL_MIDDLE_WRAPPING_KARATSUBA_THRESHOLD {
        mul_middle_classical_wrapping(out, xs, ys);
        return;
    }
    if n.odd() {
        let n_1 = n - 1;
        let (out_low, out_last) = out.split_at_mut(n_1);
        mul_middle_balanced_wrapping(out_low, &xs[..n_1], &ys[1..n_1 << 1], scratch);
        let top = xs[n_1];
        for (o, &y) in out_low.iter_mut().zip(ys) {
            o.wrapping_add_assign(top.wrapping_mul(y));
        }
        mul_middle_classical_wrapping(out_last, xs, &ys[n_1..]);
        return;
    }
    let h = n >> 1;
    let two_h_1 = (h << 1) - 1;
    let (x0, x1) = xs.split_at(h);
    split_into_chunks_mut!(scratch, h, [x_sum, p], scratch);
    let (y_difference, scratch) = scratch.split_at_mut(two_h_1);
    x_sum.copy_from_slice(x0);
    add_wrapping_assign(x_sum, x1);
    let y1 = &ys[h..h + two_h_1];
    mul_middle_balanced_wrapping(p, x_sum, y1, scratch);
    let (out_low, out_high) = out.split_at_mut(h);
    y_difference.copy_from_slice(&ys[..two_h_1]);
    sub_wrapping_assign(y_difference, y1);
    mul_middle_balanced_wrapping(out_low, x1, y_difference, scratch);
    y_difference.copy_from_slice(&ys[h << 1..]);
    sub_wrapping_assign(y_difference, y1);
    mul_middle_balanced_wrapping(out_high, x0, y_difference, scratch);
    add_wrapping_assign(out_low, p);
    add_wrapping_assign(out_high, p);
}

// Sets `out`, which is nonempty, to the middle product of `xs`, which is nonempty, and `ys` modulo
// $2^W$, where `ys` has length `xs.len() + out.len() - 1`, by Karatsuba's method, cutting `xs` or
// `out` into pieces when their lengths differ (see `mod_mul_middle_karatsuba`).
fn mul_middle_karatsuba_wrapping<T: PrimitiveUnsigned>(out: &mut [T], xs: &[T], ys: &[T]) {
    let n = xs.len();
    let k = out.len();
    match n.cmp(&k) {
        Ordering::Equal => {
            let mut scratch = vec![T::ZERO; mul_middle_wrapping_scratch_len(n)];
            mul_middle_balanced_wrapping(out, xs, ys, &mut scratch);
        }
        Ordering::Greater => {
            // Piece c holds the coefficients of x from ck on, and its middle product reads ys from
            // n - ck - (its length) on.
            out.fill(T::ZERO);
            let mut piece_out = vec![T::ZERO; k];
            for (c, piece) in xs.chunks(k).enumerate() {
                let len = piece.len();
                let start = n - c * k - len;
                mul_middle_karatsuba_wrapping(
                    &mut piece_out,
                    piece,
                    &ys[start..start + len + k - 1],
                );
                add_wrapping_assign(out, &piece_out);
            }
        }
        Ordering::Less => {
            for (b, block) in out.chunks_mut(n).enumerate() {
                let start = b * n;
                let len = block.len();
                mul_middle_karatsuba_wrapping(block, xs, &ys[start..start + n + len - 1]);
            }
        }
    }
}

fn assert_lengths<T>(out: &[T], xs: &[T], ys: &[T]) {
    assert!(!out.is_empty());
    assert!(!xs.is_empty());
    assert_eq!(ys.len(), xs.len() + out.len() - 1);
}

// Sets `out` to the middle product of `xs` and `ys`, correct modulo $2^k$, where $k$ is `pow`, but
// not reduced: by Karatsuba's method, which falls back to the classical method for short inputs,
// or, for long ones, by number-theoretic transforms (see `ntt_preferred`). `out` and `xs` must be
// nonempty, and `ys` must have length `xs.len() + out.len() - 1`. The inputs need not be reduced.
pub(crate) fn mul_middle_wrapping<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    ys: &[T],
    pow: u64,
) {
    assert_lengths(out, xs, ys);
    if ntt_preferred::<T>(NttOp::MulMiddle, min(xs.len(), out.len()), pow) {
        mul_middle_ntt(out, xs, ys, pow);
    } else {
        mul_middle_karatsuba_wrapping(out, xs, ys);
    }
}

// Sets `out` to the middle product of `xs` and `ys` modulo $2^k$, where $k$ is `pow`, by the
// classical method: the `out.len()` coefficients of their product from coefficient `xs.len() - 1`
// on. `out` and `xs` must be nonempty, the inputs must be reduced modulo $2^k$, `ys` must have
// length `xs.len() + out.len() - 1`, and `pow` must be no greater than `T::WIDTH`.
crate_test_fn! {
#[allow(dead_code)]
mod_power_of_2_mul_middle_to_out_classical<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    ys: &[T],
    pow: u64,
) {
    assert_lengths(out, xs, ys);
    assert!(pow <= T::WIDTH);
    mul_middle_classical_wrapping(out, xs, ys);
    mask_coefficients(out, pow);
}}

// Sets `out` to the middle product of `xs` and `ys` modulo $2^k$, where $k$ is `pow`, by
// Karatsuba's method for middle products, under the same conditions as
// `mod_power_of_2_mul_middle_to_out_classical`.
crate_test_fn! {
#[allow(dead_code)]
mod_power_of_2_mul_middle_to_out_karatsuba<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    ys: &[T],
    pow: u64,
) {
    assert_lengths(out, xs, ys);
    assert!(pow <= T::WIDTH);
    mul_middle_karatsuba_wrapping(out, xs, ys);
    mask_coefficients(out, pow);
}}

// Sets `out` to the middle product of `xs` and `ys` modulo $2^k$, where $k$ is `pow`, by the
// dispatcher `mul_middle_wrapping`, under the same conditions as
// `mod_power_of_2_mul_middle_to_out_classical`.
crate_test_fn! {
#[allow(dead_code)]
mod_power_of_2_mul_middle_to_out<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    ys: &[T],
    pow: u64,
) {
    assert!(pow <= T::WIDTH);
    mul_middle_wrapping(out, xs, ys, pow);
    mask_coefficients(out, pow);
}}
