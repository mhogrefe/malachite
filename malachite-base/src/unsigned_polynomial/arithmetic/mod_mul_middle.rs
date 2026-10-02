// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::Parity;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_polynomial::arithmetic::mod_mul::{
    ModData, column_sum, mod_add_assign_slice, mod_sub_assign_slice,
};
use alloc::vec;
use core::cmp::Ordering;

// Middle products of polynomials whose coefficients are words reduced modulo a word $m$. With `xs`
// of length $n$ and `ys` of length $n + k - 1$, the middle product is the $k$ coefficients of the
// product from coefficient $n - 1$ on:
// $$
// r_j = \sum_{i=0}^{n-1} x_i y_{n-1-i+j}, \quad 0 \leq j < k.
// $$
// Each is a full sum of $n$ products, with none of the shorter sums at the ends of a product, and
// they cost about as much as a product of polynomials of length $\max(n, k)$, rather than of length
// $n + k$.

// The length at which Karatsuba middle products overtake classical ones. Measured on an Apple
// M-series machine, 2026-10, for `u64` with moduli of 20 to 64 bits: Karatsuba loses at 32 and 40
// and breaks even or wins from 48.
pub(crate) const MOD_MUL_MIDDLE_KARATSUBA_THRESHOLD: usize = 48;

// Sets `out` to the middle product of `xs` and `ys` modulo $m$, one coefficient at a time. `ys`
// must have length `xs.len() + out.len() - 1`.
pub(crate) fn mod_mul_middle_classical<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    ys: &[T],
    d: &ModData<T>,
) {
    let n = xs.len();
    for (j, o) in out.iter_mut().enumerate() {
        *o = d.reduce_sum(column_sum(xs, &ys[j..j + n], d));
    }
}

// The scratch length needed by `mod_mul_middle_balanced` for `xs` of length `n`.
const fn mod_mul_middle_scratch_len(mut n: usize) -> usize {
    let mut len = 0;
    while n >= MOD_MUL_MIDDLE_KARATSUBA_THRESHOLD {
        let h = n >> 1;
        // A sum of halves of `xs`, a difference of windows of `ys`, and one middle product.
        len += (h << 2) - 1;
        n = h;
    }
    len
}

// Sets `out` to the middle product of `xs` and `ys` modulo $m$, where `xs` and `out` have the same
// nonzero length $n$ and `ys` has length $2n - 1$, by Karatsuba's method for middle products,
// falling back to the classical method below the threshold.
//
// With $n = 2h$, write $x = x_0 + x^h x_1$, and let $Y_0$, $Y_1$, and $Y_2$ be the windows of $y$
// of length $2h - 1$ starting at $0$, $h$, and $2h$. The low half of the middle product is $M(x_0,
// Y_1) + M(x_1, Y_0)$ and the high half is $M(x_0, Y_2) + M(x_1, Y_1)$. With $P = M(x_0 + x_1,
// Y_1)$, these are $P + M(x_1, Y_0 - Y_1)$ and $P + M(x_0, Y_2 - Y_1)$: three middle products of
// half the length in place of four. When $n$ is odd, the top coefficient of $x$ is split off: it
// adds $x_{n-1} y_j$ to each of the first $n - 1$ coefficients, and the last coefficient is
// computed directly.
fn mod_mul_middle_balanced<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    ys: &[T],
    d: &ModData<T>,
    scratch: &mut [T],
) {
    let n = xs.len();
    if n < MOD_MUL_MIDDLE_KARATSUBA_THRESHOLD {
        mod_mul_middle_classical(out, xs, ys, d);
        return;
    }
    let m = d.m;
    if n.odd() {
        let n_1 = n - 1;
        let (out_low, out_last) = out.split_at_mut(n_1);
        mod_mul_middle_balanced(out_low, &xs[..n_1], &ys[1..n_1 << 1], d, scratch);
        let top = xs[n_1];
        for (o, &y) in out_low.iter_mut().zip(ys) {
            let (hi, lo) = T::x_mul_y_to_zz(top, y);
            *o = o.mod_add(d.reduce_2(hi, lo), m);
        }
        out_last[0] = d.reduce_sum(column_sum(xs, &ys[n_1..], d));
        return;
    }
    let h = n >> 1;
    let two_h_1 = (h << 1) - 1;
    let (x0, x1) = xs.split_at(h);
    split_into_chunks_mut!(scratch, h, [x_sum, p], scratch);
    let (y_difference, scratch) = scratch.split_at_mut(two_h_1);
    x_sum.copy_from_slice(x0);
    mod_add_assign_slice(x_sum, x1, m);
    let y1 = &ys[h..h + two_h_1];
    mod_mul_middle_balanced(p, x_sum, y1, d, scratch);
    let (out_low, out_high) = out.split_at_mut(h);
    y_difference.copy_from_slice(&ys[..two_h_1]);
    mod_sub_assign_slice(y_difference, y1, m);
    mod_mul_middle_balanced(out_low, x1, y_difference, d, scratch);
    y_difference.copy_from_slice(&ys[h << 1..]);
    mod_sub_assign_slice(y_difference, y1, m);
    mod_mul_middle_balanced(out_high, x0, y_difference, d, scratch);
    mod_add_assign_slice(out_low, p, m);
    mod_add_assign_slice(out_high, p, m);
}

// Sets `out`, which is nonempty, to the middle product of `xs`, which is nonempty, and `ys` modulo
// $m$, where `ys` has length `xs.len() + out.len() - 1`. When there are fewer outputs than
// coefficients of `xs`, `xs` is cut into pieces as long as `out` and their middle products are
// added; when there are more, `out` is cut into pieces as long as `xs`, each a middle product of
// its own window of `ys`.
pub(crate) fn mod_mul_middle_karatsuba<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    ys: &[T],
    d: &ModData<T>,
) {
    let n = xs.len();
    let k = out.len();
    match n.cmp(&k) {
        Ordering::Equal => {
            let mut scratch = vec![T::ZERO; mod_mul_middle_scratch_len(n)];
            mod_mul_middle_balanced(out, xs, ys, d, &mut scratch);
        }
        Ordering::Greater => {
            // Piece c holds the coefficients of x from ck on, and its middle product reads ys from
            // n - ck - (its length) on.
            out.fill(T::ZERO);
            let mut piece_out = vec![T::ZERO; k];
            for (c, piece) in xs.chunks(k).enumerate() {
                let len = piece.len();
                let start = n - c * k - len;
                mod_mul_middle_karatsuba(&mut piece_out, piece, &ys[start..start + len + k - 1], d);
                mod_add_assign_slice(out, &piece_out, d.m);
            }
        }
        Ordering::Less => {
            for (b, block) in out.chunks_mut(n).enumerate() {
                let start = b * n;
                let len = block.len();
                mod_mul_middle_karatsuba(block, xs, &ys[start..start + n + len - 1], d);
            }
        }
    }
}

fn assert_lengths<T>(out: &[T], xs: &[T], ys: &[T]) {
    assert!(!out.is_empty());
    assert!(!xs.is_empty());
    assert_eq!(ys.len(), xs.len() + out.len() - 1);
}

// Sets `out` to the middle product of `xs` and `ys` modulo `m`, by the classical method: the
// `out.len()` coefficients of their product from coefficient `xs.len() - 1` on. `out` and `xs` must
// be nonempty, the inputs must be reduced modulo `m`, and `ys` must have length `xs.len() +
// out.len() - 1`.
crate_test_fn! {
#[allow(dead_code)]
mod_mul_middle_to_out_classical<T: PrimitiveUnsigned>(out: &mut [T], xs: &[T], ys: &[T], m: T) {
    assert_lengths(out, xs, ys);
    mod_mul_middle_classical(out, xs, ys, &ModData::new(m, xs.len()));
}}

// Sets `out` to the middle product of `xs` and `ys` modulo `m`, by Karatsuba's method for middle
// products, under the same conditions as `mod_mul_middle_to_out_classical`.
crate_test_fn! {
#[allow(dead_code)]
mod_mul_middle_to_out_karatsuba<T: PrimitiveUnsigned>(out: &mut [T], xs: &[T], ys: &[T], m: T) {
    assert_lengths(out, xs, ys);
    mod_mul_middle_karatsuba(out, xs, ys, &ModData::new(m, xs.len()));
}}
