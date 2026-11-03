// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_polynomial::RationalPolynomial;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::SubMulAssign;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::polynomial::Polynomial;

// The coefficients of the product of the factors $x - r$, multiplied in one at a time, each
// multiplication a shift minus a scalar multiple, with every coefficient a separate `Rational`.
pub fn from_roots_coefficients_naive(roots: &[Rational]) -> Vec<Rational> {
    let mut product = vec![Rational::ONE];
    for r in roots {
        let mut next = vec![Rational::ZERO; product.len() + 1];
        for (i, c) in product.iter().enumerate() {
            next[i + 1] += c;
            next[i].sub_mul_assign(c, r);
        }
        product = next;
    }
    product
}

// The product of the factors $x - r$, multiplied in one at a time.
pub fn from_roots_naive(roots: &[Rational]) -> RationalPolynomial {
    RationalPolynomial::from_coefficients_asc(from_roots_coefficients_naive(roots))
}
