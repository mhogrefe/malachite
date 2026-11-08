// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::OneHalf;
use malachite_base::num::linear_algebra::traits::{AreAffinelyIndependent, AreLinearlyIndependent};
use malachite_base::vector::Vector;
use malachite_q::Rational;
use malachite_q::rational_vector::RationalVector;
use malachite_q::rational_vector::linear_algebra::are_affinely_independent::*;
use malachite_q::test_util::generators::rational_matrix_gen;
use std::panic::catch_unwind;

fn vectors(xs: &[&str]) -> Vec<RationalVector> {
    xs.iter()
        .map(|s| RationalVector::from_str(s).unwrap())
        .collect()
}

#[test]
fn test_are_affinely_independent() {
    let test = |xs: &[&str], out: bool| {
        let xs = vectors(xs);
        assert_eq!(RationalVector::are_affinely_independent(&xs), out);
        assert_eq!(
            are_affinely_independent_by_homogeneous_coordinates(&xs),
            out
        );
        assert_eq!(are_affinely_independent_by_subtraction(&xs), out);
    };
    test(&[], true);
    test(&["(1/2, -1/3)"], true);
    test(&["(1/2, -1/3)", "(1/2, -1/3)"], false);
    test(&["(0, 0)", "(1/2, 1/3)", "(1, 2/3)"], false);
    test(&["(0, 0)", "(1/2, 0)", "(0, 1/3)"], true);
    test(&["(1/2, 1/2)", "(1/3, 2/3)", "(1/4, 3/4)"], false);
    test(&["(1/2, 1/2)", "(1/3, 2/3)", "(1/4, 1/4)"], true);
}

#[test]
fn are_affinely_independent_fail() {
    assert_panic!(RationalVector::are_affinely_independent(&vectors(&[
        "(1, 2)", "(1)"
    ])));
}

#[test]
fn are_affinely_independent_properties() {
    rational_matrix_gen().test_properties(|matrix| {
        let xs: Vec<RationalVector> = matrix
            .rows()
            .map(|row| RationalVector::from_elements(row.as_slice()))
            .collect();
        let independent = RationalVector::are_affinely_independent(&xs);
        assert_eq!(
            are_affinely_independent_by_homogeneous_coordinates(&xs),
            independent
        );
        assert_eq!(are_affinely_independent_by_subtraction(&xs), independent);
        // Points are affinely independent exactly when their differences from the first point are
        // linearly independent.
        if let Some((first, rest)) = xs.split_first() {
            let differences: Vec<RationalVector> = rest.iter().map(|x| x - first).collect();
            assert_eq!(
                RationalVector::are_linearly_independent(&differences),
                independent
            );
            // Translating the points does not matter.
            let translated: Vec<RationalVector> = xs.iter().map(|x| x + first).collect();
            assert_eq!(
                RationalVector::are_affinely_independent(&translated),
                independent
            );
        }
        // Scaling the points by a nonzero constant, or reversing their order, does not matter.
        let mut ys: Vec<RationalVector> = xs.iter().map(|x| x * &Rational::ONE_HALF).collect();
        ys.reverse();
        assert_eq!(RationalVector::are_affinely_independent(&ys), independent);
    });
}
