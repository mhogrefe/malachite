// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural_matrix::NaturalMatrix;
use malachite_base::num::arithmetic::traits::{KroneckerProduct, ModPowerOf2};

// The Kronecker product $A \otimes B$ modulo $2^k$: the exact Kronecker product, reduced.
pub fn natural_matrix_mod_power_of_2_kronecker_product_naive(
    a: &NaturalMatrix,
    b: &NaturalMatrix,
    pow: u64,
) -> NaturalMatrix {
    a.kronecker_product(b).mod_power_of_2(pow)
}
