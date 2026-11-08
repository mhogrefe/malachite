// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::One;
use malachite_base::num::linear_algebra::traits::{AreAffinelyIndependent, AreLinearlyIndependent};
use malachite_base::vector::Vector;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::natural_vector::linear_algebra::are_affinely_independent::*;
use malachite_nz::test_util::generators::natural_matrix_gen;
use std::panic::catch_unwind;

fn vectors(xs: &[&str]) -> Vec<NaturalVector> {
    xs.iter()
        .map(|s| NaturalVector::from_str(s).unwrap())
        .collect()
}

#[test]
fn test_are_affinely_independent() {
    let test = |xs: &[&str], out: bool| {
        let xs = vectors(xs);
        assert_eq!(NaturalVector::are_affinely_independent(&xs), out);
        assert_eq!(
            are_affinely_independent_by_homogeneous_coordinates(&xs),
            out
        );
        assert_eq!(are_affinely_independent_by_subtraction(&xs), out);
    };
    // - no points, or one
    test(&[], true);
    test(&["(1, 2)"], true);
    test(&["()"], true);
    // - a repeated point
    test(&["(1, 1)", "(1, 1)"], false);
    test(&["()", "()"], false);
    test(&["(0, 0)", "(1, 1)", "(2, 2)"], false);
    test(&["(0, 0)", "(1, 0)", "(0, 1)"], true);
    test(&["(1, 0)", "(0, 1)", "(2, 2)"], true);
    // - linearly independent points that are not affinely independent would need more than d
    //   points, but these three are affinely independent although they lie in a plane
    test(&["(1, 0, 0)", "(0, 1, 0)", "(0, 0, 1)"], true);
    // - more than d + 1 points of dimension d
    test(&["(1, 0)", "(0, 1)", "(1, 1)", "(2, 3)"], false);
    // - large elements
    test(
        &["(1267650600228229401496703205376, 0)", "(0, 0)", "(633825300114114700748351602688, 0)"],
        false,
    );
    test(
        &["(1267650600228229401496703205376, 0)", "(0, 0)", "(0, 1)"],
        true,
    );
}

#[test]
fn are_affinely_independent_fail() {
    assert_panic!(NaturalVector::are_affinely_independent(&vectors(&[
        "(1, 2)", "(1)"
    ])));
}

#[test]
fn are_affinely_independent_properties() {
    natural_matrix_gen().test_properties(|matrix| {
        let xs: Vec<NaturalVector> = matrix
            .rows()
            .map(|row| NaturalVector::from_elements(row.as_slice()))
            .collect();
        let independent = NaturalVector::are_affinely_independent(&xs);
        assert_eq!(
            are_affinely_independent_by_homogeneous_coordinates(&xs),
            independent
        );
        assert_eq!(are_affinely_independent_by_subtraction(&xs), independent);
        let integers: Vec<IntegerVector> = xs.iter().map(IntegerVector::from).collect();
        assert_eq!(
            IntegerVector::are_affinely_independent(&integers),
            independent
        );
        // Points are affinely independent exactly when their differences from the first point are
        // linearly independent.
        if let Some((first, rest)) = integers.split_first() {
            let differences: Vec<IntegerVector> = rest.iter().map(|x| x - first).collect();
            assert_eq!(
                IntegerVector::are_linearly_independent(&differences),
                independent
            );
        }
        // The order of the points does not matter.
        let mut reversed = xs.clone();
        reversed.reverse();
        assert_eq!(
            NaturalVector::are_affinely_independent(&reversed),
            independent
        );
        // Translating the points does not matter.
        let translated: Vec<NaturalVector> = xs
            .iter()
            .map(|x| {
                NaturalVector::from_owned_elements(
                    x.elements_ref().iter().map(|e| e + Natural::ONE).collect(),
                )
            })
            .collect();
        assert_eq!(
            NaturalVector::are_affinely_independent(&translated),
            independent
        );
        if let Some(first) = xs.first() {
            // A subset of independent points is independent.
            if independent {
                assert!(NaturalVector::are_affinely_independent(&xs[..xs.len() - 1]));
            }
            // Repeating a point makes the points dependent.
            let mut repeated = xs.clone();
            repeated.push(first.clone());
            assert!(!NaturalVector::are_affinely_independent(&repeated));
        }
    });
}
