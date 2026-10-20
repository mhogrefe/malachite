// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use crate::test_util::natural_matrix::arithmetic::mul::natural_matrix_mul_naive;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::conversion::traits::ExactFrom;

// Multiplies the matrix by itself with `natural_matrix_mul_naive`.
pub fn natural_matrix_square_naive(a: &NaturalMatrix) -> NaturalMatrix {
    natural_matrix_mul_naive(a, a)
}

// Squares a square `NaturalMatrix` with the given kernel, which takes the elements and the
// dimension.
pub fn natural_matrix_square_with(
    a: &NaturalMatrix,
    f: impl Fn(&[Natural], usize) -> Vec<Natural>,
) -> NaturalMatrix {
    assert!(a.is_square());
    NaturalMatrix::from_owned_elements(
        a.height(),
        a.width(),
        f(&a.elements, usize::exact_from(a.height())),
    )
}
