// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural_polynomial::NaturalPolynomial;
use crate::test_util::natural_polynomial::arithmetic::mul::mul_naive;
use malachite_base::num::arithmetic::traits::ModPowerOf2;
use malachite_base::polynomial::Polynomial;

// Multiplies the whole polynomials by schoolbook multiplication, truncates the product, and then
// reduces its coefficients.
pub fn mod_power_of_2_mul_truncated_naive(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    len: u64,
    pow: u64,
) -> NaturalPolynomial {
    mul_naive(p, q).truncate(len).mod_power_of_2(pow)
}
