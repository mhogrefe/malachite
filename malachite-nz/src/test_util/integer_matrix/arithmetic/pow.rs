// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::IntegerMatrix;
use crate::test_util::integer_matrix::arithmetic::mul::integer_matrix_mul_naive;
use malachite_base::matrix::Matrix;

// Multiplies the identity matrix by the matrix `exp` times, with `integer_matrix_mul_naive`.
pub fn integer_matrix_pow_naive(a: &IntegerMatrix, exp: u64) -> IntegerMatrix {
    assert!(a.is_square());
    let mut out = IntegerMatrix::identity(a.height());
    for _ in 0..exp {
        out = integer_matrix_mul_naive(&out, a);
    }
    out
}
