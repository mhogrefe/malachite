// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural_matrix::NaturalMatrix;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, ModPowerOf2MulAssign, ModPowerOf2SquareAssign,
};
use malachite_base::num::logic::traits::BitIterable;

// The power $A^e$ modulo $2^k$, by right-to-left binary powering with `ModPowerOf2MulAssign` and
// `ModPowerOf2SquareAssign`.
pub fn natural_matrix_mod_power_of_2_pow_naive(
    a: &NaturalMatrix,
    exp: u64,
    pow: u64,
) -> NaturalMatrix {
    let mut result = NaturalMatrix::identity(a.height()).mod_power_of_2(pow);
    let mut base = a.clone();
    for bit in exp.bits() {
        if bit {
            result.mod_power_of_2_mul_assign(&base, pow);
        }
        base.mod_power_of_2_square_assign(pow);
    }
    result
}
