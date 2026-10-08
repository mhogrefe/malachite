// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural_vector::NaturalVector;
use core::cmp::Ordering::{self, *};

/// The shortlex comparison, written as an explicit walk over the elements rather than as a
/// comparison of lengths followed by a comparison of `Vec`s.
pub fn natural_vector_shortlex_cmp_naive(v: &NaturalVector, w: &NaturalVector) -> Ordering {
    let (xs, ys) = (&v.elements, &w.elements);
    if xs.len() != ys.len() {
        return if xs.len() < ys.len() { Less } else { Greater };
    }
    for i in 0..xs.len() {
        if xs[i] < ys[i] {
            return Less;
        } else if xs[i] > ys[i] {
            return Greater;
        }
    }
    Equal
}
