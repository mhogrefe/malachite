// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2024 Éric Schost
//
//      Copyright © 2024 Vincent Neiger
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::arithmetic::mul::ProductShape;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{AddMulAssign, Parity};
use malachite_base::num::basic::traits::Zero;

// Returns the elements of the product of the `m` by `n` matrix with elements `xs` and the `n` by
// `p` matrix with elements `ys`, where the shape is `ProductShape { ar: m, br: n, bc: p }`, with
// Waksman's algorithm, which takes $mnp/2 + O(mp + np + mn)$ multiplications but many additions; it
// suits small matrices with large elements.
//
// For each pair of columns of the first matrix and the matching pair of rows of the second, every
// element of the product gains $(a_1 + b_2)(a_2 + b_1)$, where $a_1$ and $a_2$ are elements of the
// first matrix and $b_1$ and $b_2$ of the second. This contains the wanted $a_1b_1 + a_2b_2$ plus
// $a_1a_2 + b_1b_2$, which depends only on the row or only on the column; those terms are computed
// once per row and once per column, from the first row and the first column of the product, and
// removed at the end. An odd last column is handled classically.
//
// This is equivalent to `fmpz_mat_mul_waksman` from `fmpz_mat/mul_waksman.c`, FLINT 3.6.0.
crate_test_fn! {matrix_mul_waksman(
    xs: &[Integer],
    ys: &[Integer],
    shape: ProductShape,
) -> Vec<Integer> {
    let ProductShape { ar: m, br: n, bc: p } = shape;
    let mut out = vec![Integer::ZERO; m * p];
    if m == 0 || n == 0 || p == 0 {
        return out;
    }
    let a = |i: usize, j: usize| &xs[i * n + j];
    let b = |i: usize, j: usize| &ys[i * p + j];
    // The row and column corrections share one allocation, as in FLINT.
    let mut corrections = vec![Integer::ZERO; p + m];
    let (c_row, c_col) = corrections.split_at_mut(p);
    let np = n >> 1;
    // As in FLINT, these temporaries are reused, so that their storage is allocated only once.
    let mut val_1 = Integer::ZERO;
    let mut val_2 = Integer::ZERO;
    let mut c = Integer::ZERO;
    // Sets `val` to `x + y`, or if `subtract`, to `x - y`.
    let set = |val: &mut Integer, x: &Integer, y: &Integer, subtract: bool| {
        val.clone_from(x);
        if subtract {
            *val -= y;
        } else {
            *val += y;
        }
    };
    for j in 1..=np {
        let j2 = (j << 1) - 1;
        for k in 0..p {
            set(&mut val_1, a(0, j2 - 1), b(j2, k), false);
            set(&mut val_2, a(0, j2), b(j2 - 1, k), false);
            out[k].add_mul_assign(&val_1, &val_2);
            set(&mut val_1, a(0, j2 - 1), b(j2, k), true);
            set(&mut val_2, a(0, j2), b(j2 - 1, k), true);
            c_row[k].add_mul_assign(&val_1, &val_2);
        }
        for l in 1..m {
            set(&mut val_1, a(l, j2 - 1), b(j2, 0), false);
            set(&mut val_2, a(l, j2), b(j2 - 1, 0), false);
            out[l * p].add_mul_assign(&val_1, &val_2);
            set(&mut val_1, a(l, j2 - 1), b(j2, 0), true);
            set(&mut val_2, a(l, j2), b(j2 - 1, 0), true);
            c_col[l].add_mul_assign(&val_1, &val_2);
        }
        for k in 1..p {
            for l in 1..m {
                set(&mut val_1, a(l, j2 - 1), b(j2, k), false);
                set(&mut val_2, a(l, j2), b(j2 - 1, k), false);
                out[l * p + k].add_mul_assign(&val_1, &val_2);
            }
        }
    }
    // Each of these sums is twice the wanted value, so halving it is exact, and FLINT's truncating
    // halving is the same as a shift.
    for l in 1..m {
        c_col[l] += &out[l * p];
        c_col[l] >>= 1u32;
        out[l * p] -= &c_col[l];
    }
    let mut val_0 = c_row[0].clone();
    val_0 += &out[0];
    val_0 >>= 1u32;
    out[0] -= &val_0;
    for k in 1..p {
        set(&mut val_1, &c_row[k], &out[k], false);
        val_1 >>= 1u32;
        out[k] -= &val_1;
        set(&mut c, &val_1, &val_0, true);
        for l in 1..m {
            out[l * p + k] -= &c;
            out[l * p + k] -= &c_col[l];
        }
    }
    if n.odd() {
        for l in 0..m {
            for k in 0..p {
                out[l * p + k].add_mul_assign(a(l, n - 1), b(n - 1, k));
            }
        }
    }
    out
}}
