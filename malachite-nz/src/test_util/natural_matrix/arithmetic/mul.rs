// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::arithmetic::mul::ProductShape;
use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::AddMulAssign;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;

// Computes each element of the product as a sum of products, adding one product at a time.
pub fn natural_matrix_mul_naive(a: &NaturalMatrix, b: &NaturalMatrix) -> NaturalMatrix {
    let height = usize::exact_from(a.height());
    let inner = usize::exact_from(a.width());
    let width = usize::exact_from(b.width());
    assert_eq!(a.width(), b.height());
    let mut elements = Vec::with_capacity(height * width);
    for i in 0..height {
        for j in 0..width {
            let mut sum = Natural::ZERO;
            for k in 0..inner {
                sum.add_mul_assign(&a.elements[i * inner + k], &b.elements[k * width + j]);
            }
            elements.push(sum);
        }
    }
    NaturalMatrix::from_owned_elements(a.height(), b.width(), elements)
}

// Multiplies two `NaturalMatrix`s with the given kernel, which takes the elements and the shape.
pub fn natural_matrix_mul_with(
    a: &NaturalMatrix,
    b: &NaturalMatrix,
    f: impl Fn(&[Natural], &[Natural], ProductShape) -> Vec<Natural>,
) -> NaturalMatrix {
    assert_eq!(a.width(), b.height());
    let shape = ProductShape {
        ar: usize::exact_from(a.height()),
        br: usize::exact_from(a.width()),
        bc: usize::exact_from(b.width()),
    };
    NaturalMatrix::from_owned_elements(a.height(), b.width(), f(&a.elements, &b.elements, shape))
}
