// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_polynomial::RationalPolynomial;
use malachite_base::num::arithmetic::traits::Square;

// Sums the squares of the coefficients as `Rational`s, each reduced on its own. Nothing is shared
// with the implementation.
pub fn l2_norm_squared_naive(p: &RationalPolynomial) -> Rational {
    p.to_coefficients_asc()
        .into_iter()
        .map(Square::square)
        .sum()
}
