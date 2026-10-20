// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2008 Martin Albrecht
//
//      Copyright © 2008, 2009 William Hart
//
//      Copyright © 2010, 2011 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_matrix::arithmetic::mod_mul_matrix::classical::matrix_mod_mul_classical_op;
use crate::unsigned_matrix::arithmetic::mod_mul_matrix::{ModMulOp, ProductShape, matrix_mod_mul};
use alloc::vec;
use alloc::vec::Vec;
use core::ops::Range;

// A copy of the block of the matrix with elements `xs` and width `width` made of the given rows and
// columns. This stands in for `nmod_mat_window_init`, since Malachite's matrices own their
// elements.
fn block<T: PrimitiveUnsigned>(
    xs: &[T],
    width: usize,
    rows: Range<usize>,
    columns: Range<usize>,
) -> Vec<T> {
    let mut out = Vec::with_capacity(rows.len() * columns.len());
    for i in rows {
        out.extend_from_slice(&xs[i * width + columns.start..i * width + columns.end]);
    }
    out
}

// Copies a block with `block_width` columns into the matrix with elements `xs` and width `width`,
// with its top-left element in row `row` and column `column`.
fn set_block<T: PrimitiveUnsigned>(
    xs: &mut [T],
    width: usize,
    row: usize,
    column: usize,
    block: &[T],
    block_width: usize,
) {
    if block_width == 0 {
        return;
    }
    for (i, block_row) in block.chunks_exact(block_width).enumerate() {
        let start = (row + i) * width + column;
        xs[start..start + block_width].copy_from_slice(block_row);
    }
}

// A block of a matrix, referenced in place: the matrix's elements and width, and the row and column
// of the block's top-left element.
#[derive(Clone, Copy)]
struct BlockRef<'a, T> {
    xs: &'a [T],
    width: usize,
    row: usize,
    column: usize,
}

impl<T> BlockRef<'_, T> {
    // Row `i` of the block, `columns` elements long.
    fn row(&self, i: usize, columns: usize) -> &[T] {
        let start = (self.row + i) * self.width + self.column;
        &self.xs[start..start + columns]
    }
}

// Sets `out`, a matrix with `columns` columns, to the element-by-element sum, or if `subtract`
// difference, of two blocks of its shape, modulo `m`.
fn set_sum<T: PrimitiveUnsigned>(
    out: &mut [T],
    columns: usize,
    x: BlockRef<T>,
    y: BlockRef<T>,
    subtract: bool,
    m: T,
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
                a.mod_sub(b, m)
            } else {
                a.mod_add(b, m)
            };
        }
    }
}

// Adds a block of its shape to `out`, a matrix with `columns` columns, modulo `m`, or if
// `subtract`, subtracts it.
fn add_assign_block<T: PrimitiveUnsigned>(
    out: &mut [T],
    columns: usize,
    y: BlockRef<T>,
    subtract: bool,
    m: T,
) {
    if columns == 0 {
        return;
    }
    for (i, out_row) in out.chunks_exact_mut(columns).enumerate() {
        for (z, &b) in out_row.iter_mut().zip(y.row(i, columns)) {
            if subtract {
                z.mod_sub_assign(b, m);
            } else {
                z.mod_add_assign(b, m);
            }
        }
    }
}

// Adds `ys` to `xs` element by element, modulo `m`.
fn add_assign<T: PrimitiveUnsigned>(xs: &mut [T], ys: &[T], m: T) {
    for (x, &y) in xs.iter_mut().zip(ys) {
        x.mod_add_assign(y, m);
    }
}

// Replaces each element `z` of `zs` with `x - z` modulo `m`, where `x` is the corresponding element
// of `xs`.
fn reverse_sub_assign<T: PrimitiveUnsigned>(zs: &mut [T], xs: &[T], m: T) {
    for (z, &x) in zs.iter_mut().zip(xs) {
        *z = x.mod_sub(*z, m);
    }
}

// Adds the product of the matrices with elements `xs` and `ys` to `out`, modulo `m`, with the
// classical algorithm. FLINT's `nmod_mat_addmul` computes the product separately when every
// dimension is at least the cutoff for Strassen's algorithm, but here the inner dimension is always
// 1, so the classical algorithm, fused with the addition, is always used.
//
// This is equivalent to `nmod_mat_addmul` from `nmod_mat/addmul.c`, FLINT 3.6.0, with one thread,
// with the output the same as the matrix that is added to, and with an inner dimension of 1.
fn matrix_mod_add_mul<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    ys: &[T],
    shape: ProductShape,
    m: T,
) {
    debug_assert_eq!(shape.br, 1);
    matrix_mod_mul_classical_op(out, ModMulOp::Add, xs, ys, shape, m);
}

// Returns the elements of the product of the `a` by `b` matrix with elements `xs` and the `b` by
// `c` matrix with elements `ys`, all reduced modulo `m`, where the shape is `ProductShape { ar: a,
// br: b, bc: c }`, with Strassen's algorithm: each matrix is split into four blocks, whose product
// takes seven products of blocks instead of eight, at the cost of more additions. The products of
// blocks are computed with `matrix_mod_mul`, which chooses an algorithm for each, recursively. An
// odd last row or column is handled separately.
//
// The sequence of products and additions is Bodrato's, which is equivalent to Winograd's variant of
// Strassen's algorithm, scheduled as in Dumas, Pernet, and Zhou, "Memory efficient scheduling of
// Strassen-Winograd's matrix multiplication algorithm".
//
// This is equivalent to `nmod_mat_mul_strassen` from `nmod_mat/mul_strassen.c`, FLINT 3.6.0, with
// the blocks that are multiplied directly copied rather than referenced through windows, since
// Malachite's matrices own their elements.
crate_test_fn! {matrix_mod_mul_strassen<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    shape: ProductShape,
    m: T,
) -> Vec<T> {
    let ProductShape { ar: a, br: b, bc: c } = shape;
    if a <= 4 || b <= 4 || c <= 4 {
        return matrix_mod_mul(xs, ys, shape, m);
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
    // Only the blocks that are multiplied directly are copied; the others are read in place. FLINT
    // reads them all through windows.
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
    // As in FLINT, `x_1` and `x_2` are reused for each sum and difference; they share one
    // allocation.
    let mut temporaries = vec![T::ZERO; anr * anc + anc * bnc];
    let (x_1, x_2) = temporaries.split_at_mut(anr * anc);
    set_sum(x_1, anc, a_22_ref, a_12_ref, false, m);
    set_sum(x_2, bnc, b_22_ref, b_12_ref, false, m);
    let mut c_21 = matrix_mod_mul(x_1, x_2, half_shape, m);
    set_sum(x_1, anc, a_22_ref, a_21_ref, true, m);
    set_sum(x_2, bnc, b_22_ref, b_21_ref, true, m);
    let mut c_22 = matrix_mod_mul(x_1, x_2, half_shape, m);
    add_assign_block(x_1, anc, a_12_ref, false, m);
    add_assign_block(x_2, bnc, b_12_ref, false, m);
    let mut c_11 = matrix_mod_mul(x_1, x_2, half_shape, m);
    add_assign_block(x_1, anc, a_11_ref, true, m);
    let mut c_12 = matrix_mod_mul(x_1, &b_12, half_shape, m);
    let x_1 = matrix_mod_mul(&a_12, &b_21, half_shape, m);
    add_assign(&mut c_11, &x_1, m);
    add_assign(&mut c_12, &c_22, m);
    // C12 = C11 - C12
    reverse_sub_assign(&mut c_12, &c_11, m);
    // C11 = C21 - C11
    reverse_sub_assign(&mut c_11, &c_21, m);
    add_assign_block(x_2, bnc, b_11_ref, true, m);
    c_21 = matrix_mod_mul(&a_21, x_2, half_shape, m);
    drop(temporaries);
    // C21 = C11 - C21
    reverse_sub_assign(&mut c_21, &c_11, m);
    add_assign(&mut c_22, &c_11, m);
    c_11 = matrix_mod_mul(&a_11, &b_11, half_shape, m);
    add_assign(&mut c_11, &x_1, m);
    let mut out = vec![T::ZERO; a * c];
    set_block(&mut out, c, 0, 0, &c_11, bnc);
    set_block(&mut out, c, 0, bnc, &c_12, bnc);
    set_block(&mut out, c, anr, 0, &c_21, bnc);
    set_block(&mut out, c, anr, bnc, &c_22, bnc);
    if c > bnc << 1 {
        // The first matrix times the last column of the second gives the last column of the
        // product.
        let b_c = block(ys, c, 0..b, bnc << 1..c);
        let c_c = matrix_mod_mul(
            xs,
            &b_c,
            ProductShape {
                ar: a,
                br: b,
                bc: c - (bnc << 1),
            },
            m,
        );
        set_block(&mut out, c, 0, bnc << 1, &c_c, c - (bnc << 1));
    }
    if a > anr << 1 {
        // The last row of the first matrix times the second gives the last row of the product.
        let a_r = block(xs, b, anr << 1..a, 0..b);
        let c_r = matrix_mod_mul(
            &a_r,
            ys,
            ProductShape {
                ar: a - (anr << 1),
                br: b,
                bc: c,
            },
            m,
        );
        set_block(&mut out, c, anr << 1, 0, &c_r, c);
    }
    if b > anc << 1 {
        // The last column of the first matrix times the last row of the second is added to the
        // product.
        let a_c = block(xs, b, 0..anr << 1, anc << 1..b);
        let b_r = block(ys, c, bnr << 1..b, 0..bnc << 1);
        let mut c_b = block(&out, c, 0..anr << 1, 0..bnc << 1);
        matrix_mod_add_mul(
            &mut c_b,
            &a_c,
            &b_r,
            ProductShape {
                ar: anr << 1,
                br: b - (anc << 1),
                bc: bnc << 1,
            },
            m,
        );
        set_block(&mut out, c, 0, 0, &c_b, bnc << 1);
    }
    out
}}
