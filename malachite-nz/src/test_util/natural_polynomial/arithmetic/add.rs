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
use core::cmp::max;
use malachite_base::polynomial::Polynomial;

// Adds two polynomials coefficient by coefficient, reading each coefficient through `coefficient`,
// which gives zero past the degree, and building the result with `from_coefficients_asc`, which
// trims. Nothing is shared with the in-place implementation.
pub fn add_naive(p: &NaturalPolynomial, q: &NaturalPolynomial) -> NaturalPolynomial {
    let len = max(p.len(), q.len());
    NaturalPolynomial::from_coefficients_asc(
        (0..len)
            .map(|i| p.coefficient(i) + q.coefficient(i))
            .collect::<Vec<Natural>>(),
    )
}
