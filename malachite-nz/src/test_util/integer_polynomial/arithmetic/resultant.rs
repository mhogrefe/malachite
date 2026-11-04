// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::IntegerMatrix;
use alloc::vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::linear_algebra::traits::Determinant;

// The resultant of the polynomials with coefficients `xs` and `ys`, neither ending with a zero and
// neither empty, as the determinant of their Sylvester matrix: `deg ys` rows of the coefficients of
// `xs` from the top down, each shifted one place right of the one above, followed by `deg xs` rows
// of those of `ys`.
pub fn resultant_sylvester(xs: &[Integer], ys: &[Integer]) -> Integer {
    let deg_x = xs.len() - 1;
    let deg_y = ys.len() - 1;
    let n = deg_x + deg_y;
    let mut elements = vec![Integer::ZERO; n * n];
    for i in 0..deg_y {
        for (j, c) in xs.iter().rev().enumerate() {
            elements[i * n + i + j] = c.clone();
        }
    }
    for i in 0..deg_x {
        for (j, c) in ys.iter().rev().enumerate() {
            elements[(deg_y + i) * n + i + j] = c.clone();
        }
    }
    let n = u64::exact_from(n);
    IntegerMatrix::from_owned_elements(n, n, elements).determinant()
}
