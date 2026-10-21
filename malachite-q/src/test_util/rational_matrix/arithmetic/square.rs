// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_matrix::RationalMatrix;
use crate::test_util::rational_matrix::arithmetic::mul::rational_matrix_mul_naive;
use malachite_base::num::arithmetic::traits::Square;

/// The square $A^2$, computed one element at a time from the definition in rational arithmetic.
pub fn rational_matrix_square_naive(a: &RationalMatrix) -> RationalMatrix {
    rational_matrix_mul_naive(a, a)
}

/// The square $A^2$, with the matrix written as integers over a single common denominator $d$, so
/// that the integer matrix is squared, as by `IntegerMatrix`'s `Square`, and divided by $d^2$.
pub fn rational_matrix_square_matwise(a: &RationalMatrix) -> RationalMatrix {
    let (numerators, d) = a.to_numerators_and_denominator();
    RationalMatrix::from_numerators_and_denominator(&numerators.square(), &d.square())
}
