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
    ModAreAffinelyIndependent, ModPowerOf2AreAffinelyIndependent, ModPowerOf2AreLinearlyIndependent,
};
use malachite_base::num::modular::traits::ModPowerOf2Sub;
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_4;
use malachite_base::test_util::unsigned_vector::linear_algebra::mod_are_linearly_independent::*;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::unsigned_vector::linear_algebra::mod_power_of_2_are_affinely_independent::*;
use std::panic::catch_unwind;

fn vectors(xs: &[&str]) -> Vec<UnsignedVector<u64>> {
    xs.iter()
        .map(|s| UnsignedVector::from_str(s).unwrap())
        .collect()
}

#[test]
fn test_mod_power_of_2_are_affinely_independent() {
    let test = |xs: &[&str], pow: u64, out: bool| {
        let xs = vectors(xs);
        assert_eq!(
            UnsignedVector::mod_power_of_2_are_affinely_independent(&xs, pow),
            out
        );
        assert_eq!(
            mod_power_of_2_are_affinely_independent_by_homogeneous_coordinates(&xs, pow),
            out
        );
        assert_eq!(
            mod_power_of_2_are_affinely_independent_by_subtraction(&xs, pow),
            out
        );
        assert_eq!(
            mod_power_of_2_are_affinely_independent_by_reduction_mod_2(&xs, pow),
            out
        );
        if pow < 64 {
            assert_eq!(
                UnsignedVector::mod_are_affinely_independent(&xs, u64::power_of_2(pow)),
                out
            );
        }
    };
    // (expected values computed independently with python, by trying every combination of
    // coefficients that sum to zero)
    // - no points, or one
    test(&[], 3, true);
    test(&["(3, 1)"], 2, true);
    // - pow == 0: modulo 1, where the only coefficient is 0
    test(&["(0, 0)", "(0, 0)"], 0, true);
    // - a repeated point
    test(&["(1)", "(1)"], 2, false);
    // - independent
    test(&["(0, 0)", "(1, 0)", "(0, 1)"], 3, true);
    test(&["(1, 0)", "(0, 1)", "(1, 1)"], 1, true);
    // - dependent
    test(&["(2, 1)", "(0, 1)"], 2, false);
    test(&["(0, 0)", "(2, 0)"], 3, false);
    test(&["(0, 0)", "(1, 0)", "(0, 4)"], 3, false);
    test(&["(1, 3)", "(3, 1)", "(2, 2)"], 2, false);
    // - more than d + 1 points of dimension d
    test(&["(0)", "(1)", "(2)"], 3, false);
    // - pow == 64, with differences that wrap around
    test(&["(18446744073709551615, 0)", "(0, 0)"], 64, true);
    test(&["(18446744073709551615, 0)", "(1, 0)"], 64, false);
}

#[test]
fn mod_power_of_2_are_affinely_independent_fail() {
    assert_panic!(
        UnsignedVector::<u64>::mod_power_of_2_are_affinely_independent(
            &vectors(&["(1, 2)", "(1)"]),
            3
        )
    );
    assert_panic!(
        UnsignedVector::<u64>::mod_power_of_2_are_affinely_independent(&vectors(&["(1, 8)"]), 3)
    );
}

#[test]
fn mod_power_of_2_are_affinely_independent_properties() {
    unsigned_matrix_unsigned_pair_gen_var_4().test_properties(|(matrix, pow)| {
        let xs = rows_as_vectors(&matrix);
        let independent = UnsignedVector::mod_power_of_2_are_affinely_independent(&xs, pow);
        assert_eq!(
            mod_power_of_2_are_affinely_independent_by_homogeneous_coordinates(&xs, pow),
            independent
        );
        assert_eq!(
            mod_power_of_2_are_affinely_independent_by_subtraction(&xs, pow),
            independent
        );
        assert_eq!(
            mod_power_of_2_are_affinely_independent_by_reduction_mod_2(&xs, pow),
            independent
        );
        if pow < 64 {
            assert_eq!(
                UnsignedVector::mod_are_affinely_independent(&xs, u64::power_of_2(pow)),
                independent
            );
        }
        // Points are affinely independent exactly when their differences from the first point are
        // linearly independent.
        if let Some((first, rest)) = xs.split_first() {
            let differences: Vec<UnsignedVector<u64>> = rest
                .iter()
                .map(|x| x.clone().mod_power_of_2_sub(first, pow))
                .collect();
            assert_eq!(
                UnsignedVector::mod_power_of_2_are_linearly_independent(&differences, pow),
                independent
            );
        }
        // The order of the points does not matter.
        let mut reversed = xs.clone();
        reversed.reverse();
        assert_eq!(
            UnsignedVector::mod_power_of_2_are_affinely_independent(&reversed, pow),
            independent
        );
    });
}
