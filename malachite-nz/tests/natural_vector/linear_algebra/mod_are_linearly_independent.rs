// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::linear_algebra::traits::ModAreLinearlyIndependent;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::natural_matrix_natural_pair_gen_var_4;
use std::panic::catch_unwind;

fn vectors(xs: &[&str]) -> Vec<NaturalVector> {
    xs.iter()
        .map(|s| NaturalVector::from_str(s).unwrap())
        .collect()
}

#[test]
fn test_mod_are_linearly_independent() {
    let test = |xs: &[&str], m: &str, out: bool| {
        let xs = vectors(xs);
        let m = Natural::from_str(m).unwrap();
        assert_eq!(NaturalVector::mod_are_linearly_independent(&xs, &m), out);
        assert_eq!(NaturalVector::mod_are_linearly_independent(&xs, m), out);
    };
    // (expected values computed independently with python, by trying every combination)
    test(&[], "5", true);
    test(&["(0, 0)"], "1", true);
    test(&["()"], "5", false);
    test(&["(1, 2)", "(2, 4)"], "7", false);
    test(&["(2, 0)"], "4", false);
    test(&["(2, 1)"], "4", true);
    test(&["(3, 0)", "(0, 2)"], "6", false);
    test(&["(2, 1)", "(1, 2)"], "4", true);
    // - moduli of more than 64 bits
    test(
        &["(633825300114114700748351602688, 1)"],
        "1267650600228229401496703205376",
        true,
    );
    test(
        &["(633825300114114700748351602688, 0)"],
        "1267650600228229401496703205376",
        false,
    );
    test(&["(3, 0)"], "717897987691852588770249", false);
    test(&["(3, 1)"], "717897987691852588770249", true);
    test(
        &["(1, 2)", "(0, 1)"],
        "170141183460469231731687303715884105727",
        true,
    );
    test(
        &["(1, 2)", "(2, 4)"],
        "170141183460469231731687303715884105727",
        false,
    );
}

#[test]
fn mod_are_linearly_independent_fail() {
    assert_panic!(NaturalVector::mod_are_linearly_independent(
        &vectors(&["(1)"]),
        Natural::ZERO
    ));
    assert_panic!(NaturalVector::mod_are_linearly_independent(
        &vectors(&["(1, 2)", "(1)"]),
        Natural::from(5u32)
    ));
    assert_panic!(NaturalVector::mod_are_linearly_independent(
        &vectors(&["(1, 5)"]),
        Natural::from(5u32)
    ));
}

#[test]
fn mod_are_linearly_independent_properties() {
    natural_matrix_natural_pair_gen_var_4().test_properties(|(matrix, m)| {
        let xs: Vec<NaturalVector> = matrix
            .rows()
            .map(|row| NaturalVector::from_elements(row.as_slice()))
            .collect();
        let independent = NaturalVector::mod_are_linearly_independent(&xs, &m);
        assert_eq!(
            NaturalVector::mod_are_linearly_independent(&xs, m.clone()),
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
                UnsignedVector::mod_are_linearly_independent(&ys, m64),
                independent
            );
        }
        // The order of the vectors does not matter.
        let mut reversed = xs.clone();
        reversed.reverse();
        assert_eq!(
            NaturalVector::mod_are_linearly_independent(&reversed, &m),
            independent
        );
        if let Some(first) = xs.first() {
            if independent {
                assert!(NaturalVector::mod_are_linearly_independent(
                    &xs[..xs.len() - 1],
                    &m
                ));
            }
            if m != 1u32 {
                let mut with_zero = xs.clone();
                with_zero.push(NaturalVector::zero(first.dimension()));
                assert!(!NaturalVector::mod_are_linearly_independent(&with_zero, &m));
            }
        }
    });
}
