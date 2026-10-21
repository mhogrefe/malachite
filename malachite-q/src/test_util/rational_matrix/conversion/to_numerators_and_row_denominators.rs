// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_matrix::RationalMatrix;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::natural_vector::NaturalVector;

/// Clears the denominators of each row of `a` by clearing the denominators of each row as a vector,
/// with `RationalVector::to_numerators_and_denominator`.
pub fn rational_matrix_to_numerators_and_row_denominators_naive(
    a: &RationalMatrix,
) -> (IntegerMatrix, NaturalVector) {
    let mut rows = Vec::new();
    let mut denominators = Vec::new();
    for i in 0..a.height() {
        let (ns, d) = a.row(i).to_numerators_and_denominator();
        rows.push(ns);
        denominators.push(d);
    }
    let numerators = if rows.is_empty() {
        IntegerMatrix::zero(a.height(), a.width())
    } else {
        IntegerMatrix::from_rows(&rows)
    };
    (
        numerators,
        NaturalVector {
            elements: denominators,
        },
    )
}
