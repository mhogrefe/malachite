// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_matrix::RationalMatrix;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::AddMulAssign;
use malachite_base::num::basic::traits::Zero;

/// The product $AB$, computed one element at a time from the definition in rational arithmetic,
/// without clearing denominators.
///
/// This is equivalent to `fmpq_mat_mul_direct` from `fmpq_mat/mul_direct.c`, FLINT 3.6.0.
pub fn rational_matrix_mul_naive(a: &RationalMatrix, b: &RationalMatrix) -> RationalMatrix {
    assert_eq!(a.width(), b.height());
    let elements = (0..a.height())
        .flat_map(|i| {
            (0..b.width()).map(move |j| {
                let mut sum = Rational::ZERO;
                for (x, y) in a.row_iter(i).zip(b.column_iter(j)) {
                    sum.add_mul_assign(x, y);
                }
                sum
            })
        })
        .collect();
    RationalMatrix::from_owned_elements(a.height(), b.width(), elements)
}

/// The product $AB$, with each matrix written as integers over a single common denominator, so that
/// the integer product is divided by one denominator, the product of the two.
pub fn rational_matrix_mul_matwise(a: &RationalMatrix, b: &RationalMatrix) -> RationalMatrix {
    let (a_numerators, d) = a.to_numerators_and_denominator();
    let (b_numerators, e) = b.to_numerators_and_denominator();
    RationalMatrix::from_numerators_and_denominator(&(a_numerators * b_numerators), &(d * e))
}
