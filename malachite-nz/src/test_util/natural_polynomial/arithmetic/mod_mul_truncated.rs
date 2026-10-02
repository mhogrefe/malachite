// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::test_util::natural_polynomial::arithmetic::mod_mul::mod_mul_naive;
use malachite_base::polynomial::Polynomial;

// Multiplies the whole polynomials by schoolbook multiplication, reduces the product's coefficients
// modulo `m`, and then truncates it.
pub fn mod_mul_truncated_naive(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    len: u64,
    m: &Natural,
) -> NaturalPolynomial {
    mod_mul_naive(p, q, m).truncate(len)
}
