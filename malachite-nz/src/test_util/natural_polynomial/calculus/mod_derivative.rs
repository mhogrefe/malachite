// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use alloc::vec::Vec;
use malachite_base::polynomial::Polynomial;

// Multiplies the coefficient of x^i, read one at a time through `coefficient`, by i without a
// modulus, reduces the product modulo `m`, and normalizes the result. Nothing is shared with the
// implementation.
pub fn mod_derivative_naive(p: &NaturalPolynomial, m: &Natural) -> NaturalPolynomial {
    NaturalPolynomial::from_coefficients_asc(
        (1..p.len())
            .map(|i| p.coefficient(i) * Natural::from(i) % m)
            .collect::<Vec<Natural>>(),
    )
}
