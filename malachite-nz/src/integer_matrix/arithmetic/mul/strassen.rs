// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2016 Aaditya Thakkar
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::arithmetic::mul::classical::matrix_mul_classical;
use crate::integer_matrix::arithmetic::mul::{ProductShape, matrix_mul};
use crate::integer_vector::arithmetic::add::vec_add_assign;
use crate::integer_vector::arithmetic::sub::vec_sub_assign;
use alloc::vec;
use alloc::vec::Vec;
use core::ops::Range;
use malachite_base::num::arithmetic::traits::NegAssign;
use malachite_base::num::basic::traits::Zero;

// A copy of the block of the matrix with elements `xs` and width `width` made of the given rows and
// columns. This stands in for `fmpz_mat_window_init`, since Malachite's matrices own their
// elements.
pub(crate) fn block(
    xs: &[Integer],
    width: usize,
    rows: Range<usize>,
    columns: Range<usize>,
) -> Vec<Integer> {
    let mut out = Vec::with_capacity(rows.len() * columns.len());
    for i in rows {
        out.extend_from_slice(&xs[i * width + columns.start..i * width + columns.end]);
    }
    out
}

// Moves the elements of a block with `block_width` columns into the matrix with elements `xs` and
// width `width`, with its top-left element in row `row` and column `column`. This writes back what
// FLINT would compute in place through a window.
pub(crate) fn set_block(
    xs: &mut [Integer],
    width: usize,
    row: usize,
    column: usize,
    block: Vec<Integer>,
    block_width: usize,
) {
    if block_width == 0 {
        return;
    }
    let mut block = block.into_iter();
    let mut i = row;
    while block.len() != 0 {
        let start = i * width + column;
        for (x, y) in xs[start..start + block_width].iter_mut().zip(&mut block) {
            *x = y;
        }
        i += 1;
    }
}

// Adds a block with `block_width` columns to the matrix with elements `xs` and width `width`, with
// its top-left element in row `row` and column `column`.
pub(crate) fn add_block(
    xs: &mut [Integer],
    width: usize,
    row: usize,
    column: usize,
    block: &[Integer],
    block_width: usize,
) {
    if block_width == 0 {
        return;
    }
    for (i, block_row) in block.chunks_exact(block_width).enumerate() {
        let start = (row + i) * width + column;
        vec_add_assign(&mut xs[start..start + block_width], block_row);
    }
}

// A block of a matrix, referenced in place: the matrix's elements and width, and the row and column
// of the block's top-left element.
#[derive(Clone, Copy)]
pub(crate) struct BlockRef<'a> {
    pub(crate) xs: &'a [Integer],
    pub(crate) width: usize,
    pub(crate) row: usize,
    pub(crate) column: usize,
}

impl BlockRef<'_> {
    // Row `i` of the block, `columns` elements long.
    pub(crate) fn row(&self, i: usize, columns: usize) -> &[Integer] {
        let start = (self.row + i) * self.width + self.column;
        &self.xs[start..start + columns]
    }
}

// Sets `out`, a matrix with `columns` columns, to the element-by-element sum, or if `subtract`
// difference, of two blocks of its shape, reusing the storage of `out`'s elements.
pub(crate) fn set_sum(
    out: &mut [Integer],
    columns: usize,
    x: BlockRef,
    y: BlockRef,
    subtract: bool,
) {
    if columns == 0 {
        return;
    }
    for (i, out_row) in out.chunks_exact_mut(columns).enumerate() {
        for ((z, a), b) in out_row
            .iter_mut()
            .zip(x.row(i, columns))
            .zip(y.row(i, columns))
        {
            z.clone_from(a);
            if subtract {
                *z -= b;
            } else {
                *z += b;
            }
        }
    }
}

// Adds a block of its shape to `out`, a matrix with `columns` columns, or if `subtract`, subtracts
// it.
pub(crate) fn add_assign_block(out: &mut [Integer], columns: usize, y: BlockRef, subtract: bool) {
    if columns == 0 {
        return;
    }
    for (i, out_row) in out.chunks_exact_mut(columns).enumerate() {
        if subtract {
            vec_sub_assign(out_row, y.row(i, columns));
        } else {
            vec_add_assign(out_row, y.row(i, columns));
        }
    }
}

// Replaces each element `z` of `zs` with `x - z`, where `x` is the corresponding element of `xs`.
pub(crate) fn reverse_sub_assign(zs: &mut [Integer], xs: &[Integer]) {
    for (z, x) in zs.iter_mut().zip(xs) {
        z.neg_assign();
        *z += x;
    }
}

// Returns the elements of the product of the `a` by `b` matrix with elements `xs` and the `b` by
// `c` matrix with elements `ys`, where the shape is `ProductShape { ar: a, br: b, bc: c }`, with
// Strassen's algorithm: each matrix is split into four blocks, whose product takes seven products
// of blocks instead of eight, at the cost of more additions. The products of blocks are computed
// with `matrix_mul`, which chooses an algorithm for each, recursively. An odd last row or column is
// handled separately.
//
// The sequence of products and additions is Bodrato's, which is equivalent to Winograd's variant of
// Strassen's algorithm.
//
// This is equivalent to `fmpz_mat_mul_strassen` from `fmpz_mat/mul_strassen.c`, FLINT 3.6.0, with
// the blocks copied rather than referenced through windows, since Malachite's matrices own their
// elements.
crate_test_fn! {matrix_mul_strassen(
    xs: &[Integer],
    ys: &[Integer],
    shape: ProductShape,
) -> Vec<Integer> {
    let ProductShape { ar: a, br: b, bc: c } = shape;
    if a <= 1 || b <= 1 || c <= 1 {
        return matrix_mul_classical(xs, ys, shape);
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
    // The products of an `anr` by `anc` block and an `anc` by `bnc` block, and of the first
    // matrix's `anr` by `anc` blocks with the second matrix's `bnr` by `bnc` blocks.
    let half_shape = ProductShape {
        ar: anr,
        br: anc,
        bc: bnc,
    };
    // As in FLINT, `x_1` and `x_2` are reused for each sum and difference; they share one
    // allocation.
    let mut temporaries = vec![Integer::ZERO; anr * anc + anc * bnc];
    let (x_1, x_2) = temporaries.split_at_mut(anr * anc);
    set_sum(x_1, anc, a_22_ref, a_12_ref, false);
    set_sum(x_2, bnc, b_22_ref, b_12_ref, false);
    let mut c_21 = matrix_mul(x_1, x_2, half_shape);
    set_sum(x_1, anc, a_22_ref, a_21_ref, true);
    set_sum(x_2, bnc, b_22_ref, b_21_ref, true);
    let mut c_22 = matrix_mul(x_1, x_2, half_shape);
    add_assign_block(x_1, anc, a_12_ref, false);
    add_assign_block(x_2, bnc, b_12_ref, false);
    let mut c_11 = matrix_mul(x_1, x_2, half_shape);
    add_assign_block(x_1, anc, a_11_ref, true);
    let mut c_12 = matrix_mul(x_1, &b_12, half_shape);
    let x_1 = matrix_mul(&a_12, &b_21, half_shape);
    vec_add_assign(&mut c_11, &x_1);
    vec_add_assign(&mut c_12, &c_22);
    // C12 = C11 - C12
    reverse_sub_assign(&mut c_12, &c_11);
    // C11 = C21 - C11
    reverse_sub_assign(&mut c_11, &c_21);
    add_assign_block(x_2, bnc, b_11_ref, true);
    c_21 = matrix_mul(&a_21, x_2, half_shape);
    drop(temporaries);
    // C21 = C11 - C21
    reverse_sub_assign(&mut c_21, &c_11);
    vec_add_assign(&mut c_22, &c_11);
    c_11 = matrix_mul(&a_11, &b_11, half_shape);
    vec_add_assign(&mut c_11, &x_1);
    let mut out = vec![Integer::ZERO; a * c];
    set_block(&mut out, c, 0, 0, c_11, bnc);
    set_block(&mut out, c, 0, bnc, c_12, bnc);
    set_block(&mut out, c, anr, 0, c_21, bnc);
    set_block(&mut out, c, anr, bnc, c_22, bnc);
    if c > bnc << 1 {
        // The last column of the second matrix.
        let b_c = block(ys, c, 0..b, bnc << 1..c);
        let c_c = matrix_mul(
            xs,
            &b_c,
            ProductShape {
                ar: a,
                br: b,
                bc: c - (bnc << 1),
            },
        );
        set_block(&mut out, c, 0, bnc << 1, c_c, c - (bnc << 1));
    }
    if a > anr << 1 {
        // The last row of the first matrix; the overlapping elements are not computed twice.
        let a_r = block(xs, b, anr << 1..a, 0..b);
        let shape_r = ProductShape {
            ar: a - (anr << 1),
            br: b,
            bc: bnc << 1,
        };
        let c_r = if c > bnc << 1 {
            matrix_mul(&a_r, &block(ys, c, 0..b, 0..bnc << 1), shape_r)
        } else {
            matrix_mul(&a_r, ys, shape_r)
        };
        set_block(&mut out, c, anr << 1, 0, c_r, bnc << 1);
    }
    if b > anc << 1 {
        // The last column of the first matrix and the last row of the second.
        let a_c = block(xs, b, 0..anr << 1, anc << 1..b);
        let b_r = block(ys, c, bnr << 1..b, 0..bnc << 1);
        let tmp = matrix_mul(
            &a_c,
            &b_r,
            ProductShape {
                ar: anr << 1,
                br: b - (anc << 1),
                bc: bnc << 1,
            },
        );
        add_block(&mut out, c, 0, 0, &tmp, bnc << 1);
    }
    out
}}
