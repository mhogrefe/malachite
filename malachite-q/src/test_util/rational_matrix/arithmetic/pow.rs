// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_matrix::RationalMatrix;
use crate::test_util::rational_matrix::arithmetic::mul::rational_matrix_mul_naive;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::Pow;
use malachite_base::num::logic::traits::BitIterable;

/// The power $A^k$, by right-to-left binary powering, with each product computed one element at a
/// time from the definition in rational arithmetic.
pub fn rational_matrix_pow_naive(a: &RationalMatrix, exp: u64) -> RationalMatrix {
    let mut result = RationalMatrix::identity(a.height());
    let mut base = a.clone();
    for bit in exp.bits() {
        if bit {
            result = rational_matrix_mul_naive(&result, &base);
        }
        base = rational_matrix_mul_naive(&base, &base);
    }
    result
}

/// The power $A^k$, with the matrix written as integers over a single common denominator $d$, so
/// that the integer matrix is raised to the power, as by `IntegerMatrix`'s `Pow`, and divided by
/// $d^k$ once.
pub fn rational_matrix_pow_matwise(a: &RationalMatrix, exp: u64) -> RationalMatrix {
    let (numerators, d) = a.to_numerators_and_denominator();
    RationalMatrix::from_numerators_and_denominator(&numerators.pow(exp), &d.pow(exp))
}
