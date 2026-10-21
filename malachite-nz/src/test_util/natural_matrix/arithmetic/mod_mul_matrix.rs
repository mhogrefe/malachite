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
use malachite_base::num::arithmetic::traits::{ModAddAssign, ModMul};
use malachite_base::num::basic::traits::Zero;

/// The product $AB$ modulo $m$, one modular operation at a time.
pub fn natural_matrix_mod_mul_matrix_naive(
    a: &NaturalMatrix,
    b: &NaturalMatrix,
    m: &Natural,
) -> NaturalMatrix {
    let elements = (0..a.height())
        .flat_map(|i| {
            (0..b.width()).map(move |j| {
                let mut sum = Natural::ZERO;
                for (x, y) in a.row_iter(i).zip(b.column_iter(j)) {
                    sum.mod_add_assign(x.mod_mul(y, m), m);
                }
                sum
            })
        })
        .collect();
    NaturalMatrix::from_owned_elements(a.height(), b.width(), elements)
}
