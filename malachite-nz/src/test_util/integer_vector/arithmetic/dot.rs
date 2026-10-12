// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_vector::IntegerVector;

// The sum of the products of corresponding elements, one product at a time.
pub fn integer_vector_dot_naive(v: &IntegerVector, w: &IntegerVector) -> Integer {
    assert_eq!(v.elements.len(), w.elements.len());
    v.elements.iter().zip(&w.elements).map(|(x, y)| x * y).sum()
}
