// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::linear_algebra::traits::{
    ModAreLinearlyIndependent, ModPowerOf2AreLinearlyIndependent,
};
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::natural_vector::linear_algebra::mod_power_of_2_are_linearly_independent::*;
use malachite_nz::test_util::generators::natural_matrix_unsigned_pair_gen_var_4;
use std::panic::catch_unwind;

fn vectors(xs: &[&str]) -> Vec<NaturalVector> {
    xs.iter()
        .map(|s| NaturalVector::from_str(s).unwrap())
        .collect()
}

#[test]
fn test_mod_power_of_2_are_linearly_independent() {
    let test = |xs: &[&str], pow: u64, out: bool| {
        let xs = vectors(xs);
        assert_eq!(
            NaturalVector::mod_power_of_2_are_linearly_independent(&xs, pow),
            out
        );
        assert_eq!(
            mod_power_of_2_are_linearly_independent_by_natural_rank(&xs, pow),
            out
        );
        assert_eq!(
            NaturalVector::mod_are_linearly_independent(&xs, Natural::power_of_2(pow)),
            out
        );
    };
    test(&[], 3, true);
    test(&["(0, 0)"], 0, true);
    test(&["(1)", "(1)"], 2, false);
    test(&["(2, 1)"], 2, true);
    test(&["(2, 0)"], 2, false);
    test(&["(1, 2)", "(3, 2)"], 2, false);
    test(&["(1, 4)", "(2, 1)"], 3, true);
    // - pow > 64
    test(&["(633825300114114700748351602688, 1)"], 100, true);
    test(&["(633825300114114700748351602688, 0)"], 100, false);
}

#[test]
fn mod_power_of_2_are_linearly_independent_fail() {
    assert_panic!(NaturalVector::mod_power_of_2_are_linearly_independent(
        &vectors(&["(1, 2)", "(1)"]),
        3
    ));
    assert_panic!(NaturalVector::mod_power_of_2_are_linearly_independent(
        &vectors(&["(1, 8)"]),
        3
    ));
}

#[test]
fn mod_power_of_2_are_linearly_independent_properties() {
    natural_matrix_unsigned_pair_gen_var_4().test_properties(|(matrix, pow)| {
        let xs: Vec<NaturalVector> = matrix
            .rows()
            .map(|row| NaturalVector::from_elements(row.as_slice()))
            .collect();
        let independent = NaturalVector::mod_power_of_2_are_linearly_independent(&xs, pow);
        assert_eq!(
            mod_power_of_2_are_linearly_independent_by_natural_rank(&xs, pow),
            independent
        );
        assert_eq!(
            NaturalVector::mod_are_linearly_independent(&xs, Natural::power_of_2(pow)),
            independent
        );
        if pow <= 64 {
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
                UnsignedVector::mod_power_of_2_are_linearly_independent(&ys, pow),
                independent
            );
        }
    });
}
