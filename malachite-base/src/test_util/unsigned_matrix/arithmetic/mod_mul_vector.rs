// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::matrix::Matrix;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_vector::UnsignedVector;
use crate::vector::Vector;

/// The product $Av$ modulo $m$, one modular operation at a time.
pub fn unsigned_matrix_mod_mul_vector_naive<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    v: &UnsignedVector<T>,
    m: T,
) -> UnsignedVector<T> {
    UnsignedVector::from_owned_elements(
        (0..a.height())
            .map(|i| {
                let mut sum = T::ZERO;
                for (&x, &y) in a.row_iter(i).zip(&v.elements) {
                    sum = sum.mod_add(x.mod_mul(y, m), m);
                }
                sum
            })
            .collect(),
    )
}

/// The product $vA$ modulo $m$, one modular operation at a time, down each column.
pub fn unsigned_vector_mod_mul_unsigned_matrix_naive<T: PrimitiveUnsigned>(
    v: &UnsignedVector<T>,
    a: &UnsignedMatrix<T>,
    m: T,
) -> UnsignedVector<T> {
    UnsignedVector::from_owned_elements(
        (0..a.width())
            .map(|j| {
                let mut sum = T::ZERO;
                for (&x, &y) in v.elements.iter().zip(a.column_iter(j)) {
                    sum = sum.mod_add(x.mod_mul(y, m), m);
                }
                sum
            })
            .collect(),
    )
}
