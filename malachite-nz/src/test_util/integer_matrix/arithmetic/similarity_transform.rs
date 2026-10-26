// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2015 William Hart
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::IntegerMatrix;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{AddMulAssign, SubMulAssign};

// The similarity transform of `fmpz_mat_similarity`, translated loop for loop: one multiplication
// by `d` per element touched.
//
// This is equivalent to `fmpz_mat_similarity` from `fmpz_mat/similarity.c`, FLINT 3.6.0.
pub fn integer_matrix_similarity_transform_flint_loops(
    a: &IntegerMatrix,
    r: u64,
    d: &Integer,
) -> IntegerMatrix {
    let n = a.height();
    let mut a = a.clone();
    for i in 0..n {
        for j in (0..r.saturating_sub(1)).chain(r + 1..n) {
            let x = a.get(i, r).unwrap().clone();
            a.get_mut(i, j).unwrap().add_mul_assign(x, d);
        }
    }
    for i in 0..n {
        for j in (0..r.saturating_sub(1)).chain(r + 1..n) {
            let x = a.get(j, i).unwrap().clone();
            a.get_mut(r, i).unwrap().sub_mul_assign(x, d);
        }
    }
    a
}

// The matrix that `fmpz_mat_similarity` conjugates by: the n by n identity matrix with the element
// in row `r` and column `j` replaced by `d` for every `j` other than `r - 1` and `r`.
fn similarity_matrix(n: u64, r: u64, d: &Integer) -> IntegerMatrix {
    let mut p = IntegerMatrix::identity(n);
    for j in 0..n {
        if j != r && j + 1 != r {
            *p.get_mut(r, j).unwrap() = d.clone();
        }
    }
    p
}

// The similarity transform of `fmpz_mat_similarity` from its definition, $P^{-1}AP$, with $P$ and
// $P^{-1}$ formed explicitly and multiplied out.
pub fn integer_matrix_similarity_transform_by_definition(
    a: &IntegerMatrix,
    r: u64,
    d: &Integer,
) -> IntegerMatrix {
    let n = a.height();
    let p = similarity_matrix(n, r, d);
    let p_inverse = similarity_matrix(n, r, &-d);
    &p_inverse * &(a * &p)
}
