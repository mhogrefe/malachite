// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural_matrix::NaturalMatrix;
use crate::test_util::natural_matrix::arithmetic::mul::natural_matrix_mul_naive;
use malachite_base::matrix::Matrix;

// Multiplies the identity matrix by the matrix `exp` times, with `natural_matrix_mul_naive`.
pub fn natural_matrix_pow_naive(a: &NaturalMatrix, exp: u64) -> NaturalMatrix {
    assert!(a.is_square());
    let mut out = NaturalMatrix::identity(a.height());
    for _ in 0..exp {
        out = natural_matrix_mul_naive(&out, a);
    }
    out
}
