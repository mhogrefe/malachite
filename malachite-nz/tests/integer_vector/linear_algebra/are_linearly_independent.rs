// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::linear_algebra::traits::{AreLinearlyIndependent, Determinant};
use malachite_base::vector::Vector;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::test_util::generators::integer_matrix_gen;
use std::panic::catch_unwind;

fn vectors(xs: &[&str]) -> Vec<IntegerVector> {
    xs.iter()
        .map(|s| IntegerVector::from_str(s).unwrap())
        .collect()
}

#[test]
fn test_are_linearly_independent() {
    let test = |xs: &[&str], out: bool| {
        assert_eq!(IntegerVector::are_linearly_independent(&vectors(xs)), out);
    };
    test(&[], true);
    test(&["(0, 0)"], false);
    test(&["(1, -2)", "(3, 4)"], true);
    test(&["(1, -2)", "(-2, 4)"], false);
    test(&["(1, 1)", "(1, -1)"], true);
    test(&["(1, 0)", "(0, 1)", "(-1, 1)"], false);
}

#[test]
fn are_linearly_independent_fail() {
    assert_panic!(IntegerVector::are_linearly_independent(&vectors(&[
        "(1, 2)", "(1)"
    ])));
}

#[test]
fn are_linearly_independent_properties() {
    integer_matrix_gen().test_properties(|matrix| {
        let xs: Vec<IntegerVector> = matrix
            .rows()
            .map(|row| IntegerVector::from_elements(row.as_slice()))
            .collect();
        let independent = IntegerVector::are_linearly_independent(&xs);
        // Vectors are independent exactly when their Gram matrix is nonsingular.
        let a = IntegerMatrix::from_rows(&xs);
        assert_eq!((&a * &a.transpose()).determinant() != 0u32, independent);
        // Negating a vector, or reversing their order, does not matter.
        let mut ys = xs.clone();
        ys.reverse();
        if let Some(y) = ys.first_mut() {
            *y = -&*y;
        }
        assert_eq!(IntegerVector::are_linearly_independent(&ys), independent);
    });
}
