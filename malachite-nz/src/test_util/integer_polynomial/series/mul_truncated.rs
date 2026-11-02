// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::IntegerPolynomial;
use crate::test_util::integer_polynomial::arithmetic::mul::mul_naive;
use malachite_base::polynomial::Polynomial;

// Multiplies the whole polynomials by schoolbook multiplication and then truncates the product.
pub fn mul_truncated_naive(
    p: &IntegerPolynomial,
    q: &IntegerPolynomial,
    len: u64,
) -> IntegerPolynomial {
    mul_naive(p, q).truncate(len)
}
