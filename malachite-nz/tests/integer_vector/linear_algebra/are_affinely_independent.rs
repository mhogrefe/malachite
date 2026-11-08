// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::linear_algebra::traits::{AreAffinelyIndependent, AreLinearlyIndependent};
use malachite_base::vector::Vector;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::integer_vector::linear_algebra::are_affinely_independent::*;
use malachite_nz::test_util::generators::integer_matrix_gen;
use std::panic::catch_unwind;

fn vectors(xs: &[&str]) -> Vec<IntegerVector> {
    xs.iter()
        .map(|s| IntegerVector::from_str(s).unwrap())
        .collect()
}

#[test]
fn test_are_affinely_independent() {
    let test = |xs: &[&str], out: bool| {
        let xs = vectors(xs);
        assert_eq!(IntegerVector::are_affinely_independent(&xs), out);
        assert_eq!(
            are_affinely_independent_by_homogeneous_coordinates(&xs),
            out
        );
        assert_eq!(are_affinely_independent_by_subtraction(&xs), out);
    };
    test(&[], true);
    test(&["(-1, 2)"], true);
    test(&["(-1, 2)", "(-1, 2)"], false);
    test(&["(-1, -1)", "(0, 0)", "(1, 1)"], false);
    test(&["(-1, 0)", "(1, 0)", "(0, 1)"], true);
    test(&["(1, -2)", "(-3, 4)", "(5, -8)"], false);
    test(&["(1, -2)", "(-3, 4)", "(5, -7)"], true);
    test(&["(-1, 0)", "(1, 0)", "(0, 1)", "(0, -1)"], false);
}

#[test]
fn are_affinely_independent_fail() {
    assert_panic!(IntegerVector::are_affinely_independent(&vectors(&[
        "(1, 2)", "(1)"
    ])));
}

#[test]
fn are_affinely_independent_properties() {
    integer_matrix_gen().test_properties(|matrix| {
        let xs: Vec<IntegerVector> = matrix
            .rows()
            .map(|row| IntegerVector::from_elements(row.as_slice()))
            .collect();
        let independent = IntegerVector::are_affinely_independent(&xs);
        assert_eq!(
            are_affinely_independent_by_homogeneous_coordinates(&xs),
            independent
        );
        assert_eq!(are_affinely_independent_by_subtraction(&xs), independent);
        // Points are affinely independent exactly when their differences from the first point are
        // linearly independent.
        if let Some((first, rest)) = xs.split_first() {
            let differences: Vec<IntegerVector> = rest.iter().map(|x| x - first).collect();
            assert_eq!(
                IntegerVector::are_linearly_independent(&differences),
                independent
            );
            // Translating the points does not matter.
            let translated: Vec<IntegerVector> = xs.iter().map(|x| x + first).collect();
            assert_eq!(
                IntegerVector::are_affinely_independent(&translated),
                independent
            );
        }
        // Negating the points, or reversing their order, does not matter.
        let mut ys: Vec<IntegerVector> = xs.iter().map(|x| -x).collect();
        ys.reverse();
        assert_eq!(IntegerVector::are_affinely_independent(&ys), independent);
    });
}
