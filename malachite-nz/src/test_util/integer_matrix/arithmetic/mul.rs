// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::IntegerMatrix;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;

// Computes each element of the product as a sum of products, adding one product at a time.
pub fn integer_matrix_mul_naive(a: &IntegerMatrix, b: &IntegerMatrix) -> IntegerMatrix {
    let height = usize::exact_from(a.height());
    let inner = usize::exact_from(a.width());
    let width = usize::exact_from(b.width());
    assert_eq!(a.width(), b.height());
    let mut elements = Vec::with_capacity(height * width);
    for i in 0..height {
        for j in 0..width {
            let mut sum = Integer::ZERO;
            for k in 0..inner {
                sum += &a.elements[i * inner + k] * &b.elements[k * width + j];
            }
            elements.push(sum);
        }
    }
    IntegerMatrix::from_owned_elements(a.height(), b.width(), elements)
}
