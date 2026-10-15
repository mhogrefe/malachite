// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_vector::IntegerVector;
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{AbsSquared, FloorSqrt, Square};

// The norm, found by taking the floor of the square root of the squared norm and checking whether
// it squares back. Nothing is shared with the implementation.
pub fn integer_vector_checked_abs_naive(v: &IntegerVector) -> Option<Natural> {
    let s = v.abs_squared();
    let r = (&s).floor_sqrt();
    if (&r).square() == s { Some(r) } else { None }
}
