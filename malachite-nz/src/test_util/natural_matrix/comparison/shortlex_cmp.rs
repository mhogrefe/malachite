// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural_matrix::NaturalMatrix;
use core::cmp::Ordering::{self, *};
use malachite_base::matrix::Matrix;
use malachite_base::num::conversion::traits::ExactFrom;

/// The shortlex comparison, written as an explicit walk over the number of elements, the height,
/// the width, and then the elements, rather than as a chain of comparisons ending in a comparison
/// of `Vec`s.
pub fn natural_matrix_shortlex_cmp_naive(m: &NaturalMatrix, n: &NaturalMatrix) -> Ordering {
    let keys = |a: &NaturalMatrix| [u64::exact_from(a.elements.len()), a.height(), a.width()];
    for (k, l) in keys(m).into_iter().zip(keys(n)) {
        match k.cmp(&l) {
            Equal => {}
            c => return c,
        }
    }
    for (x, y) in m.elements.iter().zip(&n.elements) {
        match x.cmp(y) {
            Equal => {}
            c => return c,
        }
    }
    Equal
}
