// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::RationalPolynomial;
use crate::test_util::rational_polynomial::arithmetic::mul::mul_naive;
use malachite_base::polynomial::Polynomial;

// Multiplies the whole polynomials by schoolbook multiplication, as `Rational`s, and then keeps the
// first `len` coefficients of the product.
pub fn mul_truncated_naive(
    p: &RationalPolynomial,
    q: &RationalPolynomial,
    len: u64,
) -> RationalPolynomial {
    mul_naive(p, q).truncate(len)
}
