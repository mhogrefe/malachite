// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{DivModPrecomputed, ModAddAssign, ModMul};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::unsigned_matrix::arithmetic::mod_strong_echelon_form::*;
use malachite_base::unsigned_matrix::UnsignedMatrix;

/// The strong echelon form of `a` modulo `m`, computed from the definition by enumerating the row
/// span, as for an `UnsignedMatrix<u64>`. `m` must fit in a `u64`, and the computation is only
/// practical for small matrices and moduli.
pub fn natural_matrix_mod_strong_echelon_form_naive(
    a: &NaturalMatrix,
    m: &Natural,
) -> NaturalMatrix {
    let a = UnsignedMatrix::<u64>::try_from(a).unwrap();
    NaturalMatrix::from(unsigned_matrix_mod_strong_echelon_form_naive(
        &a,
        u64::exact_from(m),
    ))
}

/// A matrix with the same row span as `a` modulo `m`: the rows in reverse order, then each row plus
/// a multiple of the row above it.
pub fn natural_matrix_mod_row_equivalent(a: &NaturalMatrix, m: &Natural) -> NaturalMatrix {
    let mut rows: Vec<Vec<Natural>> = (0..a.height())
        .rev()
        .map(|i| a.row_iter(i).cloned().collect())
        .collect();
    let data = Natural::precompute_div_mod_data(m);
    for i in 1..rows.len() {
        let c = (&Natural::from(i + 1)).div_mod_precomputed(m, &data).1;
        let above = rows[i - 1].clone();
        for (x, y) in rows[i].iter_mut().zip(above) {
            x.mod_add_assign((&c).mod_mul(y, m), m);
        }
    }
    let elements = if a.width() == 0 {
        Vec::new()
    } else {
        rows.concat()
    };
    NaturalMatrix::from_owned_elements(a.height(), a.width(), elements)
}
