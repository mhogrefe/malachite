// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_vector::RationalVector;
use malachite_base::num::arithmetic::traits::Square;

// The sum of the squares of the elements, one square at a time, each reduced.
pub fn rational_vector_abs_squared_naive(v: &RationalVector) -> Rational {
    v.elements.iter().map(Square::square).sum()
}
