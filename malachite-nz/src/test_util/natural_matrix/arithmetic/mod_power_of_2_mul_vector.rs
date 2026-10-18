// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use crate::natural_vector::NaturalVector;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{ModPowerOf2Add, ModPowerOf2Mul};
use malachite_base::num::basic::traits::Zero;
use malachite_base::vector::Vector;

/// The product $Av$ modulo $2^k$, one modular operation at a time.
pub fn natural_matrix_mod_power_of_2_mul_vector_naive(
    a: &NaturalMatrix,
    v: &NaturalVector,
    pow: u64,
) -> NaturalVector {
    NaturalVector::from_owned_elements(
        (0..a.height())
            .map(|i| {
                let mut sum = Natural::ZERO;
                for (x, y) in a.row_iter(i).zip(&v.elements) {
                    sum = sum.mod_power_of_2_add(x.mod_power_of_2_mul(y, pow), pow);
                }
                sum
            })
            .collect(),
    )
}

/// The product $vA$ modulo $2^k$, one modular operation at a time, down each column.
pub fn natural_vector_mod_power_of_2_mul_natural_matrix_naive(
    v: &NaturalVector,
    a: &NaturalMatrix,
    pow: u64,
) -> NaturalVector {
    NaturalVector::from_owned_elements(
        (0..a.width())
            .map(|j| {
                let mut sum = Natural::ZERO;
                for (x, y) in v.elements.iter().zip(a.column_iter(j)) {
                    sum = sum.mod_power_of_2_add(x.mod_power_of_2_mul(y, pow), pow);
                }
                sum
            })
            .collect(),
    )
}
