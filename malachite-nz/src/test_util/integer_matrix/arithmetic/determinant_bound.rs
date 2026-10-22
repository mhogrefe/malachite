// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::IntegerMatrix;
use crate::natural::Natural;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{CeilingSqrt, Square};
use malachite_base::num::basic::traits::{One, Zero};

/// The exact bound that Hadamard's inequality gives for the absolute value of the determinant of
/// `a`: the ceiling of the square root of the smaller of the product of the squared Euclidean norms
/// of the rows and the product of those of the columns. If `include_zero_norms` is false, zero rows
/// and columns are left out of the products, and the bound then holds for every square submatrix.
/// It is 1 for a matrix with no elements.
pub fn integer_matrix_hadamard_bound_naive(a: &IntegerMatrix, include_zero_norms: bool) -> Natural {
    let (height, width) = (a.height(), a.width());
    if height == 0 || width == 0 {
        return Natural::ONE;
    }
    let product = |norms: &mut dyn Iterator<Item = Natural>| {
        norms
            .filter(|n| include_zero_norms || *n != 0u32)
            .fold(Natural::ONE, |p, n| p * n)
    };
    let row_product = product(&mut (0..height).map(|i| {
        a.row_iter(i)
            .map(|x| x.unsigned_abs_ref().square())
            .sum::<Natural>()
    }));
    let column_product = product(&mut (0..width).map(|j| {
        a.column_iter(j)
            .map(|x| x.unsigned_abs_ref().square())
            .sum::<Natural>()
    }));
    row_product.min(column_product).ceiling_sqrt()
}

/// The bounds of `IntegerMatrix::solve_bounds`, computed with the exact Hadamard bound from
/// `integer_matrix_hadamard_bound_naive` in place of the determinant bound: the bound times the
/// ceiling of the largest Euclidean norm of a column of `b`, and the bound.
pub fn integer_matrix_solve_bounds_naive(
    a: &IntegerMatrix,
    b: &IntegerMatrix,
) -> (Natural, Natural) {
    let d = integer_matrix_hadamard_bound_naive(a, true);
    let max = (0..b.width())
        .map(|j| {
            b.column_iter(j)
                .map(|x| x.unsigned_abs_ref().square())
                .sum::<Natural>()
        })
        .max()
        .unwrap_or(Natural::ZERO);
    (&d * max.ceiling_sqrt(), d)
}
