// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::linear_algebra::traits::{
    ModAreAffinelyIndependent, ModAreLinearlyIndependent,
};
use malachite_base::num::modular::traits::{ModAdd, ModSub};
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_9;
use malachite_base::test_util::unsigned_vector::linear_algebra::mod_are_linearly_independent::*;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::unsigned_vector::linear_algebra::mod_are_affinely_independent::*;
use malachite_base::vector::Vector;
use std::panic::catch_unwind;

fn vectors(xs: &[&str]) -> Vec<UnsignedVector<u64>> {
    xs.iter()
        .map(|s| UnsignedVector::from_str(s).unwrap())
        .collect()
}

// The points in homogeneous coordinates, with a 1 appended to each.
fn homogeneous(xs: &[UnsignedVector<u64>]) -> Vec<UnsignedVector<u64>> {
    xs.iter()
        .map(|x| {
            let mut y = x.clone();
            y.push(1);
            y
        })
        .collect()
}

#[test]
fn test_mod_are_affinely_independent() {
    let test = |xs: &[&str], m: u64, out: bool| {
        let xs = vectors(xs);
        assert_eq!(UnsignedVector::mod_are_affinely_independent(&xs, m), out);
        assert_eq!(
            mod_are_affinely_independent_by_homogeneous_coordinates(&xs, m),
            out
        );
        assert_eq!(mod_are_affinely_independent_by_subtraction(&xs, m), out);
    };
    // (expected values computed independently with python, by trying every combination of
    // coefficients that sum to zero)
    // - no points, or one
    test(&[], 5, true);
    test(&["(1, 2)"], 5, true);
    test(&["()"], 5, true);
    // - modulo 1, where the only coefficient is 0
    test(&["(0)", "(0)", "(0)"], 1, true);
    // - a repeated point
    test(&["()", "()"], 5, false);
    test(&["(5, 5)", "(5, 5)"], 7, false);
    // - a prime modulus
    test(&["(0, 0)", "(1, 1)", "(2, 2)"], 7, false);
    test(&["(0, 0)", "(1, 0)", "(0, 1)"], 7, true);
    test(&["(1, 0)", "(0, 1)", "(1, 1)"], 2, true);
    test(&["(1, 2, 3)", "(2, 3, 4)", "(3, 4, 0)"], 5, false);
    test(&["(1, 2, 3)", "(2, 3, 0)", "(3, 4, 4)"], 5, true);
    // - more than d + 1 points of dimension d
    test(&["(0, 0)", "(1, 0)", "(0, 1)", "(1, 1)"], 7, false);
    // - a composite modulus, where a difference whose elements share a factor with it is dependent
    test(&["(0, 0)", "(2, 0)"], 4, false);
    test(&["(0, 0)", "(1, 0)"], 4, true);
    test(&["(1, 1)", "(3, 3)"], 4, false);
    test(&["(2, 1)", "(0, 3)"], 4, false);
    test(&["(3)", "(5)"], 6, false);
    test(&["(3)", "(4)"], 6, true);
    test(&["(0, 0)", "(3, 0)", "(0, 2)"], 6, false);
    // - the simplest point is not first
    test(&["(1, 2)", "(3, 1)", "(0, 0)"], 4, true);
    test(&["(1, 2)", "(3, 1)", "(2, 0)"], 4, true);
}

#[test]
fn test_mod_are_affinely_independent_large_modulus() {
    let test = |xs: &[&str], m: u64, out: bool| {
        let xs = vectors(xs);
        assert_eq!(UnsignedVector::mod_are_affinely_independent(&xs, m), out);
        assert_eq!(
            mod_are_affinely_independent_by_homogeneous_coordinates(&xs, m),
            out
        );
    };
    // - u64::MAX = 3 * 5 * 17 * 257 * 641 * 65537 * 6700417
    test(&["(0, 0)", "(1, 0)", "(0, 1)"], u64::MAX, true);
    test(&["(1, 1)", "(4, 1)"], u64::MAX, false);
    test(&["(1, 1)", "(4, 6)"], u64::MAX, true);
    // - differences that wrap around
    test(&["(4, 0)", "(1, 3)"], u64::MAX, false);
    test(&["(4, 0)", "(1, 1)"], u64::MAX, true);
}

#[test]
fn mod_are_affinely_independent_fail() {
    assert_panic!(UnsignedVector::<u64>::mod_are_affinely_independent(
        &vectors(&["(1)"]),
        0
    ));
    assert_panic!(UnsignedVector::<u64>::mod_are_affinely_independent(
        &vectors(&["(1, 2)", "(1)"]),
        5
    ));
    assert_panic!(UnsignedVector::<u64>::mod_are_affinely_independent(
        &vectors(&["(1, 5)"]),
        5
    ));
}

#[test]
fn mod_are_affinely_independent_properties() {
    unsigned_matrix_unsigned_pair_gen_var_9().test_properties(|(matrix, m)| {
        let xs = rows_as_vectors(&matrix);
        let independent = UnsignedVector::mod_are_affinely_independent(&xs, m);
        assert_eq!(
            mod_are_affinely_independent_by_homogeneous_coordinates(&xs, m),
            independent
        );
        assert_eq!(
            mod_are_affinely_independent_by_subtraction(&xs, m),
            independent
        );
        // Points are affinely independent exactly when their differences from the first point are
        // linearly independent.
        if let Some((first, rest)) = xs.split_first() {
            let differences: Vec<UnsignedVector<u64>> =
                rest.iter().map(|x| x.clone().mod_sub(first, m)).collect();
            assert_eq!(
                UnsignedVector::mod_are_linearly_independent(&differences, m),
                independent
            );
        }
        if u32::try_from(xs.len()).is_ok_and(|r| (m as f64).powi(r as i32) <= 4096.0) {
            assert_eq!(
                mod_are_linearly_independent_naive(&homogeneous(&xs), m),
                independent
            );
        }
        // The order of the points does not matter.
        let mut reversed = xs.clone();
        reversed.reverse();
        assert_eq!(
            UnsignedVector::mod_are_affinely_independent(&reversed, m),
            independent
        );
        // A subset of independent points is independent.
        if independent && !xs.is_empty() {
            assert!(UnsignedVector::mod_are_affinely_independent(
                &xs[..xs.len() - 1],
                m
            ));
        }
        if m != 1 {
            // Translating the points does not matter.
            let translated: Vec<UnsignedVector<u64>> = xs
                .iter()
                .map(|x| {
                    UnsignedVector::from_owned_elements(
                        x.elements.iter().map(|&e| e.mod_add(1, m)).collect(),
                    )
                })
                .collect();
            assert_eq!(
                UnsignedVector::mod_are_affinely_independent(&translated, m),
                independent
            );
            // Repeating a point makes the points dependent.
            if let Some(first) = xs.first() {
                let mut repeated = xs.clone();
                repeated.push(first.clone());
                assert!(!UnsignedVector::mod_are_affinely_independent(&repeated, m));
            }
        }
    });
}
