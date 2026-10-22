// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use crate::test_util::natural_matrix::arithmetic::mod_reduced_row_echelon_form::*;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::ModNeg;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;

/// A basis of the right nullspace of `a` modulo the prime `m`, as the columns of a matrix, computed
/// from the reduced row echelon form of `a` found by
/// `natural_matrix_mod_reduced_row_echelon_form_naive`: for each non-pivot column c, the vector
/// with 1 in position c, the negations of column c of the reduced row echelon form in the positions
/// of the pivot columns, and 0 elsewhere.
pub fn natural_matrix_mod_nullspace_naive(a: &NaturalMatrix, m: &Natural) -> NaturalMatrix {
    let r = natural_matrix_mod_reduced_row_echelon_form_naive(a, m);
    let h = r.height();
    let w = usize::exact_from(r.width());
    // The pivot column of each nonzero row.
    let pivots: Vec<usize> = (0..h)
        .filter_map(|i| r.row_iter(i).position(|x| *x != 0u32))
        .collect();
    let nonpivots: Vec<usize> = (0..w).filter(|c| !pivots.contains(c)).collect();
    let nullity = nonpivots.len();
    let mut x = vec![Natural::ZERO; w * nullity];
    for (k, &c) in nonpivots.iter().enumerate() {
        for (i, &p) in pivots.iter().enumerate() {
            x[p * nullity + k] = r
                .get(u64::exact_from(i), u64::exact_from(c))
                .unwrap()
                .mod_neg(m);
        }
        // Modulo 1, every element is 0.
        x[c * nullity + k] = if *m == 1u32 {
            Natural::ZERO
        } else {
            Natural::ONE
        };
    }
    NaturalMatrix::from_owned_elements(u64::exact_from(w), u64::exact_from(nullity), x)
}

/// A basis of the left nullspace of `a` modulo the prime `m`, as the rows of a matrix: the
/// transpose of `natural_matrix_mod_nullspace_naive` applied to the transpose of `a`.
pub fn natural_matrix_mod_left_nullspace_naive(a: &NaturalMatrix, m: &Natural) -> NaturalMatrix {
    natural_matrix_mod_nullspace_naive(&a.transpose(), m).transpose()
}
