// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_matrix::arithmetic::mod_mul_matrix::ProductShape;
use alloc::vec;
use alloc::vec::Vec;

// Adds the product of the matrices with elements `xs` and `ys`, both in row-major order, to `out`,
// in wrapping arithmetic. Row $i$ of the product is accumulated as the sum of the rows of the
// second matrix, each times an element of row $i$ of the first, so that both matrices are read in
// storage order.
pub(crate) fn matrix_wrapping_add_mul_classical<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    ys: &[T],
    shape: ProductShape,
) {
    let ProductShape { br, bc, .. } = shape;
    if br == 0 || bc == 0 {
        return;
    }
    for (out_row, row) in out.chunks_exact_mut(bc).zip(xs.chunks_exact(br)) {
        for (&x, y_row) in row.iter().zip(ys.chunks_exact(bc)) {
            if x == T::ZERO {
                continue;
            }
            for (z, &y) in out_row.iter_mut().zip(y_row) {
                z.wrapping_add_assign(x.wrapping_mul(y));
            }
        }
    }
}

// Returns the elements of the product of the matrices with elements `xs` and `ys`, both in
// row-major order, in wrapping arithmetic, with the classical algorithm.
crate_test_fn! {matrix_wrapping_mul_classical<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    shape: ProductShape,
) -> Vec<T> {
    let mut out = vec![T::ZERO; shape.ar * shape.bc];
    matrix_wrapping_add_mul_classical(&mut out, xs, ys, shape);
    out
}}
