// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::IntegerMatrix;
use crate::test_util::integer_matrix::arithmetic::mul::integer_matrix_mul_naive;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::conversion::traits::ExactFrom;

// Multiplies the matrix by itself with `integer_matrix_mul_naive`.
pub fn integer_matrix_square_naive(a: &IntegerMatrix) -> IntegerMatrix {
    integer_matrix_mul_naive(a, a)
}

// Squares a square `IntegerMatrix` with the given kernel, which takes the elements and the
// dimension.
pub fn integer_matrix_square_with(
    a: &IntegerMatrix,
    f: impl Fn(&[Integer], usize) -> Vec<Integer>,
) -> IntegerMatrix {
    assert!(a.is_square());
    IntegerMatrix::from_owned_elements(
        a.height(),
        a.width(),
        f(&a.elements, usize::exact_from(a.height())),
    )
}
