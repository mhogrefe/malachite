// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_vector::RationalVector;
use malachite_base::num::arithmetic::traits::AddMulAssign;
use malachite_base::num::basic::traits::Zero;

// The sum of the products of corresponding elements, adding one product at a time, each addition
// reducing to lowest terms, as FLINT's `_fmpq_vec_dot` does.
pub fn rational_vector_dot_naive(v: &RationalVector, w: &RationalVector) -> Rational {
    assert_eq!(v.elements.len(), w.elements.len());
    let mut sum = Rational::ZERO;
    for (x, y) in v.elements.iter().zip(&w.elements) {
        sum.add_mul_assign(x, y);
    }
    sum
}
