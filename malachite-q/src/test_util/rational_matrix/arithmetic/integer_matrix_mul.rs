// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_matrix::RationalMatrix;
use crate::test_util::rational_matrix::arithmetic::mul::rational_matrix_mul_naive;
use malachite_nz::integer_matrix::IntegerMatrix;

/// The product $AB$, with the [`IntegerMatrix`] converted to a [`RationalMatrix`] and the product
/// computed one element at a time in rational arithmetic.
pub fn integer_matrix_mul_rational_matrix_naive(
    a: &IntegerMatrix,
    b: &RationalMatrix,
) -> RationalMatrix {
    rational_matrix_mul_naive(&RationalMatrix::from(a.clone()), b)
}
