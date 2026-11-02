// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_matrix::RationalMatrix;
use malachite_base::matrix::Matrix;

// The matrix that `fmpq_mat_similarity` conjugates by: the n by n identity matrix with the element
// in row `r` and column `j` replaced by `d` for every `j` other than `r - 1` and `r`.
fn similarity_matrix(n: u64, r: u64, d: &Rational) -> RationalMatrix {
    let mut p = RationalMatrix::identity(n);
    for j in 0..n {
        if j != r && j + 1 != r {
            *p.get_mut(r, j).unwrap() = d.clone();
        }
    }
    p
}

// The similarity transform of `fmpq_mat_similarity` from its definition, $P^{-1}AP$, with $P$ and
// $P^{-1}$ formed explicitly and multiplied out.
pub fn rational_matrix_similarity_transform_by_definition(
    a: &RationalMatrix,
    r: u64,
    d: &Rational,
) -> RationalMatrix {
    let n = a.height();
    let p = similarity_matrix(n, r, d);
    let p_inverse = similarity_matrix(n, r, &-d);
    &p_inverse * &(a * &p)
}
