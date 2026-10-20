// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::arithmetic::mul::ProductShape;
use crate::integer_polynomial::arithmetic::coefficient::PolynomialCoefficient;
use crate::integer_vector::arithmetic::dot_general::vec_dot_general_strided;
use alloc::vec;
use alloc::vec::Vec;

// Appends to `out` the product of `row`, a row of `br` elements, and the `br` by `bc` matrix with
// elements `ys` in row-major order: one row of a matrix product. `br` must be positive.
fn mul_row<C: PolynomialCoefficient>(row: &[C], ys: &[C], bc: usize, out: &mut Vec<C>) {
    match *row {
        [ref x] => {
            for y in ys {
                out.push(x.mul_ref(y));
            }
        }
        [ref x_0, ref x_1] => {
            let (ys_0, ys_1) = ys.split_at(bc);
            for (y_0, y_1) in ys_0.iter().zip(ys_1) {
                let mut c = x_0.mul_ref(y_0);
                c.add_mul_assign(x_1, y_1);
                out.push(c);
            }
        }
        _ => {
            // Column `j` of the second matrix starts at element `j` and continues every `bc`
            // elements.
            for j in 0..bc {
                out.push(vec_dot_general_strided(
                    None,
                    false,
                    row,
                    &ys[j..],
                    bc,
                    false,
                ));
            }
        }
    }
}

// Returns the elements of the product of the matrices with elements `xs` and `ys`, both in
// row-major order.
//
// This is equivalent to `fmpz_mat_mul_classical` from `fmpz_mat/mul_classical.c`, FLINT 3.6.0,
// except that the columns of the second matrix are read in place, at a stride, rather than through
// a transposed array of pointers.
crate_test_fn! {matrix_mul_classical<C: PolynomialCoefficient>(
    xs: &[C],
    ys: &[C],
    shape: ProductShape,
) -> Vec<C> {
    let ProductShape { ar, br, bc } = shape;
    if ar == 0 || br == 0 || bc == 0 {
        return vec![C::ZERO; ar * bc];
    }
    let mut out = Vec::with_capacity(ar * bc);
    for row in xs.chunks_exact(br) {
        mul_row(row, ys, bc, &mut out);
    }
    out
}}

// Replaces the elements `xs` of the first factor of a product with those of the product, both in
// row-major order, reusing the storage of `xs`.
//
// Row `i` of the product depends only on row `i` of the first matrix, so each row is computed into
// a one-row buffer and then written back. When the product is no wider than the first matrix, the
// rows are written from the top, and row `i` only overwrites rows that have already been used; when
// it is wider, the elements are extended first and the rows are written from the bottom, for the
// same reason. When the second matrix is square, the shape is unchanged and the only additional
// memory is the buffer.
//
// This is equivalent to `fmpz_mat_mul_classical` from `fmpz_mat/mul_classical.c`, FLINT 3.6.0, with
// the output the same as the first input.
crate_test_fn! {matrix_mul_classical_assign<C: PolynomialCoefficient>(
    xs: &mut Vec<C>,
    ys: &[C],
    shape: ProductShape,
) {
    let ProductShape { ar, br, bc } = shape;
    if ar == 0 || bc == 0 {
        xs.clear();
        return;
    }
    if br == 0 {
        xs.resize(ar * bc, C::ZERO);
        return;
    }
    let mut row = Vec::with_capacity(bc);
    let mut write_row = |xs: &mut Vec<C>, i: usize| {
        mul_row(&xs[i * br..(i + 1) * br], ys, bc, &mut row);
        for (x, y) in xs[i * bc..(i + 1) * bc].iter_mut().zip(row.drain(..)) {
            *x = y;
        }
    };
    if bc <= br {
        for i in 0..ar {
            write_row(xs, i);
        }
        xs.truncate(ar * bc);
    } else {
        xs.resize(ar * bc, C::ZERO);
        for i in (0..ar).rev() {
            write_row(xs, i);
        }
    }
}}
