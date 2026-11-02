// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use malachite_base::num::arithmetic::traits::KroneckerProduct;

// The Kronecker product $A \otimes B$ modulo $m$: the exact Kronecker product, reduced.
pub fn natural_matrix_mod_kronecker_product_naive(
    a: &NaturalMatrix,
    b: &NaturalMatrix,
    m: &Natural,
) -> NaturalMatrix {
    a.kronecker_product(b) % m
}
