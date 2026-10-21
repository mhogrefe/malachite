// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::test_util::unsigned_matrix::arithmetic::mod_mul_matrix::*;
use crate::unsigned_matrix::UnsignedMatrix;

/// The square $A^2$ modulo $m$, with `unsigned_matrix_mod_mul_matrix_naive`.
pub fn unsigned_matrix_mod_square_naive<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    m: T,
) -> UnsignedMatrix<T> {
    unsigned_matrix_mod_mul_matrix_naive(a, a, m)
}
