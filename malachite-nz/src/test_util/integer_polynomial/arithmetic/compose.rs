// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::IntegerPolynomial;
use malachite_base::num::basic::traits::Zero;

// Composes two polynomials with Horner's rule over whole polynomials, using only the public
// arithmetic: $p(q) = (\cdots(p_{n-1} q + p_{n-2}) q + \cdots) q + p_0$.
pub fn compose_naive(p: &IntegerPolynomial, q: &IntegerPolynomial) -> IntegerPolynomial {
    let mut result = IntegerPolynomial::ZERO;
    for c in p.coefficients_asc().iter().rev() {
        result = result * q + IntegerPolynomial::from(c.clone());
    }
    result
}
