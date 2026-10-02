// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::test_util::natural_polynomial::arithmetic::mul::mul_naive;
use malachite_base::num::arithmetic::traits::Mod;
use malachite_base::polynomial::Polynomial;

// Multiplies by schoolbook multiplication and then reduces the product's coefficients modulo `m`.
pub fn mod_mul_naive(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    m: &Natural,
) -> NaturalPolynomial {
    NaturalPolynomial::from_coefficients_asc(
        mul_naive(p, q)
            .coefficients_asc()
            .iter()
            .map(|x| x.mod_op(m))
            .collect(),
    )
}
