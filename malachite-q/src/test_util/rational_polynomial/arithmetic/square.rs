// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::RationalPolynomial;
use crate::test_util::rational_polynomial::arithmetic::mul::mul_naive;

// Squares a polynomial by multiplying it by itself with schoolbook multiplication, as `Rational`s.
pub fn square_naive(p: &RationalPolynomial) -> RationalPolynomial {
    mul_naive(p, p)
}
