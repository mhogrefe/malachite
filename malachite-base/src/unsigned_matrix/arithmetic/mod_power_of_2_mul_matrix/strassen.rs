// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_matrix::arithmetic::mod_mul_matrix::ProductShape;
use crate::unsigned_matrix::arithmetic::mod_mul_matrix::strassen::{BlockRef, block, set_block};
use crate::unsigned_matrix::arithmetic::mod_power_of_2_mul_matrix::classical::*;
use crate::unsigned_matrix::arithmetic::mod_power_of_2_mul_matrix::matrix_wrapping_mul;
use alloc::vec;
use alloc::vec::Vec;

// Sets `out`, a matrix with `columns` columns, to the element-by-element wrapping sum, or if
// `subtract` difference, of two blocks of its shape.
fn set_sum<T: PrimitiveUnsigned>(
    out: &mut [T],
    columns: usize,
    x: BlockRef<T>,
    y: BlockRef<T>,
    subtract: bool,
) {
    if columns == 0 {
        return;
    }
    for (i, out_row) in out.chunks_exact_mut(columns).enumerate() {
        for ((z, &a), &b) in out_row
            .iter_mut()
            .zip(x.row(i, columns))
            .zip(y.row(i, columns))
        {
            *z = if subtract {
                a.wrapping_sub(b)
            } else {
                a.wrapping_add(b)
            };
        }
    }
}

// Adds a block of its shape to `out`, a matrix with `columns` columns, in wrapping arithmetic, or
// if `subtract`, subtracts it.
fn add_assign_block<T: PrimitiveUnsigned>(
    out: &mut [T],
    columns: usize,
    y: BlockRef<T>,
    subtract: bool,
) {
    if columns == 0 {
        return;
    }
    for (i, out_row) in out.chunks_exact_mut(columns).enumerate() {
        for (z, &b) in out_row.iter_mut().zip(y.row(i, columns)) {
            if subtract {
                z.wrapping_sub_assign(b);
            } else {
                z.wrapping_add_assign(b);
            }
        }
    }
}

// Adds `ys` to `xs` element by element, in wrapping arithmetic.
fn add_assign<T: PrimitiveUnsigned>(xs: &mut [T], ys: &[T]) {
    for (x, &y) in xs.iter_mut().zip(ys) {
        x.wrapping_add_assign(y);
    }
}

// Replaces each element `z` of `zs` with `x - z` in wrapping arithmetic, where `x` is the
// corresponding element of `xs`.
fn reverse_sub_assign<T: PrimitiveUnsigned>(zs: &mut [T], xs: &[T]) {
    for (z, &x) in zs.iter_mut().zip(xs) {
        *z = x.wrapping_sub(*z);
    }
}

// Returns the elements of the product of the `a` by `b` matrix with elements `xs` and the `b` by
// `c` matrix with elements `ys`, in wrapping arithmetic, where the shape is `ProductShape { ar: a,
// br: b, bc: c }`, with Strassen's algorithm: each matrix is split into four blocks, whose product
// takes seven products of blocks instead of eight, at the cost of more additions. Strassen's
// algorithm needs only the ring operations, so it works in the wrapping arithmetic of `T`, whose
// results are correct modulo any power of 2 that fits in `T`. The products of blocks are computed
// with `matrix_wrapping_mul`, which chooses an algorithm for each, recursively. An odd last row or
// column is handled separately.
//
// The sequence of products and additions is the one in `matrix_mod_mul_strassen`: Bodrato's, which
// is equivalent to Winograd's variant of Strassen's algorithm.
crate_test_fn! {matrix_wrapping_mul_strassen<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    shape: ProductShape,
) -> Vec<T> {
    let ProductShape { ar: a, br: b, bc: c } = shape;
    if a <= 1 || b <= 1 || c <= 1 {
        return matrix_wrapping_mul_classical(xs, ys, shape);
    }
    let anr = a >> 1;
    let anc = b >> 1;
    let bnr = anc;
    let bnc = c >> 1;
    let at = |row, column| BlockRef {
        xs,
        width: b,
        row,
        column,
    };
    let bt = |row, column| BlockRef {
        xs: ys,
        width: c,
        row,
        column,
    };
    let a_11 = block(xs, b, 0..anr, 0..anc);
    let a_12 = block(xs, b, 0..anr, anc..anc << 1);
    let a_21 = block(xs, b, anr..anr << 1, 0..anc);
    let b_11 = block(ys, c, 0..bnr, 0..bnc);
    let b_12 = block(ys, c, 0..bnr, bnc..bnc << 1);
    let b_21 = block(ys, c, bnr..bnr << 1, 0..bnc);
    let (a_11_ref, a_12_ref, a_21_ref, a_22_ref) = (at(0, 0), at(0, anc), at(anr, 0), at(anr, anc));
    let (b_11_ref, b_12_ref, b_21_ref, b_22_ref) = (bt(0, 0), bt(0, bnc), bt(bnr, 0), bt(bnr, bnc));
    let half_shape = ProductShape {
        ar: anr,
        br: anc,
        bc: bnc,
    };
    let mut temporaries = vec![T::ZERO; anr * anc + anc * bnc];
    let (x_1, x_2) = temporaries.split_at_mut(anr * anc);
    set_sum(x_1, anc, a_22_ref, a_12_ref, false);
    set_sum(x_2, bnc, b_22_ref, b_12_ref, false);
    let mut c_21 = matrix_wrapping_mul(x_1, x_2, half_shape);
    set_sum(x_1, anc, a_22_ref, a_21_ref, true);
    set_sum(x_2, bnc, b_22_ref, b_21_ref, true);
    let mut c_22 = matrix_wrapping_mul(x_1, x_2, half_shape);
    add_assign_block(x_1, anc, a_12_ref, false);
    add_assign_block(x_2, bnc, b_12_ref, false);
    let mut c_11 = matrix_wrapping_mul(x_1, x_2, half_shape);
    add_assign_block(x_1, anc, a_11_ref, true);
    let mut c_12 = matrix_wrapping_mul(x_1, &b_12, half_shape);
    let x_1 = matrix_wrapping_mul(&a_12, &b_21, half_shape);
    add_assign(&mut c_11, &x_1);
    add_assign(&mut c_12, &c_22);
    // C12 = C11 - C12
    reverse_sub_assign(&mut c_12, &c_11);
    // C11 = C21 - C11
    reverse_sub_assign(&mut c_11, &c_21);
    add_assign_block(x_2, bnc, b_11_ref, true);
    c_21 = matrix_wrapping_mul(&a_21, x_2, half_shape);
    drop(temporaries);
    // C21 = C11 - C21
    reverse_sub_assign(&mut c_21, &c_11);
    add_assign(&mut c_22, &c_11);
    c_11 = matrix_wrapping_mul(&a_11, &b_11, half_shape);
    add_assign(&mut c_11, &x_1);
    let mut out = vec![T::ZERO; a * c];
    set_block(&mut out, c, 0, 0, &c_11, bnc);
    set_block(&mut out, c, 0, bnc, &c_12, bnc);
    set_block(&mut out, c, anr, 0, &c_21, bnc);
    set_block(&mut out, c, anr, bnc, &c_22, bnc);
    if c > bnc << 1 {
        // The first matrix times the last column of the second gives the last column of the
        // product.
        let b_c = block(ys, c, 0..b, bnc << 1..c);
        let c_c = matrix_wrapping_mul(
            xs,
            &b_c,
            ProductShape {
                ar: a,
                br: b,
                bc: c - (bnc << 1),
            },
        );
        set_block(&mut out, c, 0, bnc << 1, &c_c, c - (bnc << 1));
    }
    if a > anr << 1 {
        // The last row of the first matrix times the second gives the last row of the product.
        let a_r = block(xs, b, anr << 1..a, 0..b);
        let c_r = matrix_wrapping_mul(
            &a_r,
            ys,
            ProductShape {
                ar: a - (anr << 1),
                br: b,
                bc: c,
            },
        );
        set_block(&mut out, c, anr << 1, 0, &c_r, c);
    }
    if b > anc << 1 {
        // The last column of the first matrix times the last row of the second is added to the
        // product.
        let a_c = block(xs, b, 0..anr << 1, anc << 1..b);
        let b_r = block(ys, c, bnr << 1..b, 0..bnc << 1);
        let mut c_b = block(&out, c, 0..anr << 1, 0..bnc << 1);
        matrix_wrapping_add_mul_classical(
            &mut c_b,
            &a_c,
            &b_r,
            ProductShape {
                ar: anr << 1,
                br: b - (anc << 1),
                bc: bnc << 1,
            },
        );
        set_block(&mut out, c, 0, 0, &c_b, bnc << 1);
    }
    out
}}
