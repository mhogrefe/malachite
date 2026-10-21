// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{ModMulAssign, ModSquareAssign};
use malachite_base::num::logic::traits::BitIterable;

// The power $A^e$ modulo $m$, by right-to-left binary powering with `ModMulAssign` and
// `ModSquareAssign`.
pub fn natural_matrix_mod_pow_naive(a: &NaturalMatrix, exp: u64, m: &Natural) -> NaturalMatrix {
    let mut result = NaturalMatrix::identity(a.height()) % m;
    let mut base = a.clone();
    for bit in exp.bits() {
        if bit {
            result.mod_mul_assign(&base, m);
        }
        base.mod_square_assign(m);
    }
    result
}
