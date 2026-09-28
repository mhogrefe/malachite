// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::IntegerPolynomial;
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::Square;
use malachite_base::num::conversion::traits::ExactFrom;

// Sums the squares of the coefficients as `Integer`s. Nothing is shared with the implementation.
pub fn l2_norm_squared_naive(p: &IntegerPolynomial) -> Natural {
    Natural::exact_from(
        p.coefficients_asc()
            .iter()
            .map(Square::square)
            .sum::<Integer>(),
    )
}
