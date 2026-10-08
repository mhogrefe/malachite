// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;
use core::cmp::Ordering::{self, *};

/// The shortlex comparison, written as an explicit walk over the elements rather than as a
/// comparison of lengths followed by a comparison of `Vec`s.
pub fn unsigned_vector_shortlex_cmp_naive<T: PrimitiveUnsigned>(
    v: &UnsignedVector<T>,
    w: &UnsignedVector<T>,
) -> Ordering {
    let (xs, ys) = (&v.elements, &w.elements);
    if xs.len() != ys.len() {
        return if xs.len() < ys.len() { Less } else { Greater };
    }
    for i in 0..xs.len() {
        match xs[i].cmp(&ys[i]) {
            Equal => {}
            c => return c,
        }
    }
    Equal
}
