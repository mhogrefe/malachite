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
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::natural_matrix_gen;
use std::panic::catch_unwind;

fn vectors(xs: &[&str]) -> Vec<NaturalVector> {
    xs.iter()
        .map(|s| NaturalVector::from_str(s).unwrap())
        .collect()
}

#[test]
fn test_are_linearly_independent() {
    let test = |xs: &[&str], out: bool| {
        assert_eq!(NaturalVector::are_linearly_independent(&vectors(xs)), out);
    };
    // - no vectors
    test(&[], true);
    // - a zero vector, and a vector of dimension 0
    test(&["(0, 0)"], false);
    test(&["()"], false);
    test(&["(1, 2)", "(3, 4)"], true);
    test(&["(1, 2)", "(2, 4)"], false);
    test(&["(1, 0, 0)", "(0, 1, 0)", "(0, 0, 1)"], true);
    // - more vectors than their dimension
    test(&["(1, 0)", "(0, 1)", "(1, 1)"], false);
    // - large elements
    test(&["(1267650600228229401496703205376, 1)", "(1, 0)"], true);
    test(
        &["(1267650600228229401496703205376, 2)", "(633825300114114700748351602688, 1)"],
        false,
    );
}

#[test]
fn are_linearly_independent_fail() {
    assert_panic!(NaturalVector::are_linearly_independent(&vectors(&[
        "(1, 2)", "(1)"
    ])));
}

#[test]
fn are_linearly_independent_properties() {
    natural_matrix_gen().test_properties(|matrix| {
        let xs: Vec<NaturalVector> = matrix
            .rows()
            .map(|row| NaturalVector::from_elements(row.as_slice()))
            .collect();
        let independent = NaturalVector::are_linearly_independent(&xs);
        let integers: Vec<IntegerVector> = xs.iter().map(IntegerVector::from).collect();
        assert_eq!(
            IntegerVector::are_linearly_independent(&integers),
            independent
        );
        // Vectors are independent exactly when their Gram matrix is nonsingular.
        let a = IntegerMatrix::from_rows(&integers);
        assert_eq!((&a * &a.transpose()).determinant() != 0u32, independent);
        // The order of the vectors does not matter.
        let mut reversed = xs.clone();
        reversed.reverse();
        assert_eq!(
            NaturalVector::are_linearly_independent(&reversed),
            independent
        );
        // A subset of independent vectors is independent, and adding a zero vector makes them
        // dependent.
        if let Some(first) = xs.first() {
            if independent {
                assert!(NaturalVector::are_linearly_independent(&xs[..xs.len() - 1]));
            }
            let mut with_zero = xs.clone();
            with_zero.push(NaturalVector::zero(first.dimension()));
            assert!(!NaturalVector::are_linearly_independent(&with_zero));
        }
    });
}
