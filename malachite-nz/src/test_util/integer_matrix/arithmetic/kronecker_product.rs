// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::IntegerMatrix;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::conversion::traits::ExactFrom;

/// The Kronecker product $A \otimes B$, computed one element at a time from the definition: element
/// $(r, c)$ is $A_{\lfloor r/p \rfloor, \lfloor c/q \rfloor}B_{r \bmod p, c \bmod q}$, where $B$ is
/// $p \times q$.
pub fn integer_matrix_kronecker_product_naive(
    a: &IntegerMatrix,
    b: &IntegerMatrix,
) -> IntegerMatrix {
    let p = usize::exact_from(b.height());
    let q = usize::exact_from(b.width());
    let height = usize::exact_from(a.height()) * p;
    let width = usize::exact_from(a.width()) * q;
    let mut elements = Vec::with_capacity(height * width);
    for r in 0..height {
        for c in 0..width {
            elements.push(&a[(r / p, c / q)] * &b[(r % p, c % q)]);
        }
    }
    IntegerMatrix::from_owned_elements(a.height() * b.height(), a.width() * b.width(), elements)
}
