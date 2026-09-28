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

// Builds the result coefficient by coefficient, reading the coefficient `n` places up through
// `coefficient`. Nothing is shared with the implementation.
pub fn div_power_of_x_naive(p: &NaturalPolynomial, n: u64) -> NaturalPolynomial {
    NaturalPolynomial::from_coefficients_asc(
        (n..p.len().max(n))
            .map(|i| p.coefficient(i).clone())
            .collect::<Vec<Natural>>(),
    )
}
