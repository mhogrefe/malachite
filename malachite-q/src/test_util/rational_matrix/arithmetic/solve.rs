// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_matrix::RationalMatrix;
use malachite_base::matrix::Matrix;

/// A matrix of `a`'s shape with rank at most `b`'s width, if that is smaller than its height and
/// width: `b` times the first `b.width()` rows of `a`. Systems with this matrix and the right-hand
/// side `b` often have no unique solution, and some have none.
pub fn rational_matrix_low_rank(a: &RationalMatrix, b: &RationalMatrix) -> RationalMatrix {
    let (h, w) = (a.height(), a.width());
    let k = b.width();
    if k >= h.min(w) {
        return a.clone();
    }
    b * a.window(0..k, 0..w)
}
