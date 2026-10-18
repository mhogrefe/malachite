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
use malachite_base::num::basic::traits::Zero;
use malachite_base::vector::Vector;

/// The product $Av$, computed one element at a time from the definition, without the dot-product
/// kernel.
pub fn natural_matrix_mul_vector_naive(a: &NaturalMatrix, v: &NaturalVector) -> NaturalVector {
    NaturalVector::from_owned_elements(
        (0..a.height())
            .map(|i| {
                let mut sum = Natural::ZERO;
                for (x, y) in a.row_iter(i).zip(&v.elements) {
                    sum += x * y;
                }
                sum
            })
            .collect(),
    )
}

/// The product $vA$, computed one element at a time from the definition, down each column.
pub fn natural_vector_mul_natural_matrix_naive(
    v: &NaturalVector,
    a: &NaturalMatrix,
) -> NaturalVector {
    NaturalVector::from_owned_elements(
        (0..a.width())
            .map(|j| {
                let mut sum = Natural::ZERO;
                for (x, y) in v.elements.iter().zip(a.column_iter(j)) {
                    sum += x * y;
                }
                sum
            })
            .collect(),
    )
}
