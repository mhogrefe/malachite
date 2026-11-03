// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::IntegerPolynomial;
use crate::test_util::integer_polynomial::arithmetic::mul::integers_mul_naive;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::basic::traits::One;
use malachite_base::polynomial::Polynomial;

// The coefficients of the product of the factors $x - r$, multiplied in one at a time by schoolbook
// multiplication.
pub fn from_roots_coefficients_naive(roots: &[Integer]) -> Vec<Integer> {
    let mut product = vec![Integer::ONE];
    for r in roots {
        product = integers_mul_naive(&product, &[-r, Integer::ONE]);
    }
    product
}

// The product of the factors $x - r$, multiplied in one at a time by schoolbook multiplication.
pub fn from_roots_naive(roots: &[Integer]) -> IntegerPolynomial {
    IntegerPolynomial::from_coefficients_asc(from_roots_coefficients_naive(roots))
}
