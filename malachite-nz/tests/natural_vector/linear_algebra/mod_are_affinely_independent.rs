// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::linear_algebra::traits::{
    ModAreAffinelyIndependent, ModAreLinearlyIndependent,
};
use malachite_base::num::modular::traits::{ModAdd, ModSub};
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::natural_vector::linear_algebra::mod_are_affinely_independent::*;
use malachite_nz::test_util::generators::natural_matrix_natural_pair_gen_var_4;
use std::panic::catch_unwind;

fn vectors(xs: &[&str]) -> Vec<NaturalVector> {
    xs.iter()
        .map(|s| NaturalVector::from_str(s).unwrap())
        .collect()
}

#[test]
fn test_mod_are_affinely_independent() {
    let test = |xs: &[&str], m: &str, out: bool| {
        let xs = vectors(xs);
        let m = Natural::from_str(m).unwrap();
        assert_eq!(NaturalVector::mod_are_affinely_independent(&xs, &m), out);
        assert_eq!(
            mod_are_affinely_independent_by_homogeneous_coordinates(&xs, &m),
            out
        );
        assert_eq!(mod_are_affinely_independent_by_subtraction(&xs, &m), out);
        assert_eq!(NaturalVector::mod_are_affinely_independent(&xs, m), out);
    };
    // (expected values computed independently with python, by trying every combination of
    // coefficients that sum to zero)
    test(&[], "5", true);
    test(&["(1, 2)"], "5", true);
    test(&["(0)", "(0)", "(0)"], "1", true);
    test(&["()", "()"], "5", false);
    test(&["(0, 0)", "(1, 1)", "(2, 2)"], "7", false);
    test(&["(0, 0)", "(1, 0)", "(0, 1)"], "7", true);
    test(&["(0, 0)", "(2, 0)"], "4", false);
    test(&["(0, 0)", "(1, 0)"], "4", true);
    test(&["(0, 0)", "(3, 0)", "(0, 2)"], "6", false);
    test(&["(1, 2)", "(3, 1)", "(2, 0)"], "4", true);
    // - moduli of more than 64 bits
    test(
        &["(0, 0)", "(633825300114114700748351602688, 0)"],
        "1267650600228229401496703205376",
        false,
    );
    test(
        &["(0, 0)", "(633825300114114700748351602688, 1)"],
        "1267650600228229401496703205376",
        true,
    );
    test(&["(4, 0)", "(1, 3)"], "717897987691852588770249", false);
    test(&["(4, 0)", "(1, 1)"], "717897987691852588770249", true);
}

#[test]
fn mod_are_affinely_independent_fail() {
    assert_panic!(NaturalVector::mod_are_affinely_independent(
        &vectors(&["(1)"]),
        Natural::ZERO
    ));
    assert_panic!(NaturalVector::mod_are_affinely_independent(
        &vectors(&["(1, 2)", "(1)"]),
        Natural::from(5u32)
    ));
    assert_panic!(NaturalVector::mod_are_affinely_independent(
        &vectors(&["(1, 5)"]),
        Natural::from(5u32)
    ));
}

#[test]
fn mod_are_affinely_independent_properties() {
    natural_matrix_natural_pair_gen_var_4().test_properties(|(matrix, m)| {
        let xs: Vec<NaturalVector> = matrix
            .rows()
            .map(|row| NaturalVector::from_elements(row.as_slice()))
            .collect();
        let independent = NaturalVector::mod_are_affinely_independent(&xs, &m);
        assert_eq!(
            NaturalVector::mod_are_affinely_independent(&xs, m.clone()),
            independent
        );
        assert_eq!(
            mod_are_affinely_independent_by_homogeneous_coordinates(&xs, &m),
            independent
        );
        assert_eq!(
            mod_are_affinely_independent_by_subtraction(&xs, &m),
            independent
        );
        // Moduli that fit in a u64 give the same answer as for UnsignedVectors.
        if let Ok(m64) = u64::try_from(&m) {
            let ys: Vec<UnsignedVector<u64>> = xs
                .iter()
                .map(|x| {
                    UnsignedVector::from_owned_elements(
                        x.elements_ref()
                            .iter()
                            .map(|e| u64::try_from(e).unwrap())
                            .collect(),
                    )
                })
                .collect();
            assert_eq!(
                UnsignedVector::mod_are_affinely_independent(&ys, m64),
                independent
            );
        }
        // Points are affinely independent exactly when their differences from the first point are
        // linearly independent.
        if let Some((first, rest)) = xs.split_first() {
            let differences: Vec<NaturalVector> =
                rest.iter().map(|x| x.clone().mod_sub(first, &m)).collect();
            assert_eq!(
                NaturalVector::mod_are_linearly_independent(&differences, &m),
                independent
            );
        }
        // The order of the points does not matter.
        let mut reversed = xs.clone();
        reversed.reverse();
        assert_eq!(
            NaturalVector::mod_are_affinely_independent(&reversed, &m),
            independent
        );
        if m != 1u32 {
            // Translating the points does not matter.
            let translated: Vec<NaturalVector> = xs
                .iter()
                .map(|x| {
                    NaturalVector::from_owned_elements(
                        x.elements_ref()
                            .iter()
                            .map(|e| e.mod_add(Natural::ONE, &m))
                            .collect(),
                    )
                })
                .collect();
            assert_eq!(
                NaturalVector::mod_are_affinely_independent(&translated, &m),
                independent
            );
            // Repeating a point makes the points dependent.
            if let Some(first) = xs.first() {
                let mut repeated = xs.clone();
                repeated.push(first.clone());
                assert!(!NaturalVector::mod_are_affinely_independent(&repeated, &m));
            }
        }
    });
}
