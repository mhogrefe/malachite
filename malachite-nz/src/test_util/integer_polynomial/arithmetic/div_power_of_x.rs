// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::IntegerPolynomial;
use alloc::vec::Vec;
use malachite_base::polynomial::Polynomial;

// Builds the result coefficient by coefficient, reading the coefficient `n` places up through
// `coefficient`. Nothing is shared with the implementation.
pub fn div_power_of_x_naive(p: &IntegerPolynomial, n: u64) -> IntegerPolynomial {
    IntegerPolynomial::from_coefficients_asc(
        (n..p.len().max(n))
            .map(|i| p.coefficient(i).clone())
            .collect::<Vec<Integer>>(),
    )
}
