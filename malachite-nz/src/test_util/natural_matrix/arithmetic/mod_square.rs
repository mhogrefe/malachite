// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use crate::test_util::natural_matrix::arithmetic::mod_mul_matrix::*;

// The square $A^2$ modulo $m$, with `natural_matrix_mod_mul_matrix_naive`.
pub fn natural_matrix_mod_square_naive(a: &NaturalMatrix, m: &Natural) -> NaturalMatrix {
    natural_matrix_mod_mul_matrix_naive(a, a, m)
}
