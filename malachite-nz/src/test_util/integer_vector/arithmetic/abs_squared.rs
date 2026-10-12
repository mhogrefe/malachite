// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_vector::IntegerVector;
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::Square;

// The sum of the squares of the elements' absolute values, one square at a time.
pub fn integer_vector_abs_squared_naive(v: &IntegerVector) -> Natural {
    v.elements
        .iter()
        .map(|x| x.unsigned_abs_ref().square())
        .sum()
}
