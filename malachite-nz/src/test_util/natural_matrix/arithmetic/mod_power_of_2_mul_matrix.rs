// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural_matrix::NaturalMatrix;
use crate::test_util::natural_matrix::arithmetic::mul::natural_matrix_mul_naive;
use malachite_base::num::arithmetic::traits::ModPowerOf2Assign;

// The product $AB$ modulo $2^k$: the exact product with `natural_matrix_mul_naive`, with each
// element reduced.
pub fn natural_matrix_mod_power_of_2_mul_matrix_naive(
    a: &NaturalMatrix,
    b: &NaturalMatrix,
    pow: u64,
) -> NaturalMatrix {
    let mut c = natural_matrix_mul_naive(a, b);
    for x in &mut c.elements {
        x.mod_power_of_2_assign(pow);
    }
    c
}
