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
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_4;
use malachite_base::test_util::unsigned_vector::linear_algebra::mod_are_linearly_independent::*;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;
use std::panic::catch_unwind;

fn vectors(xs: &[&str]) -> Vec<UnsignedVector<u64>> {
    xs.iter()
        .map(|s| UnsignedVector::from_str(s).unwrap())
        .collect()
}

#[test]
fn test_mod_power_of_2_are_linearly_independent() {
    let test = |xs: &[&str], pow: u64, out: bool| {
        let xs = vectors(xs);
        assert_eq!(
            UnsignedVector::mod_power_of_2_are_linearly_independent(&xs, pow),
            out
        );
        if pow < 64 {
            let m = u64::power_of_2(pow);
            assert_eq!(UnsignedVector::mod_are_linearly_independent(&xs, m), out);
            assert_eq!(mod_are_linearly_independent_naive(&xs, m), out);
        }
    };
    // (expected values computed independently with python, by trying every combination)
    // - no vectors
    test(&[], 3, true);
    // - pow == 0: modulo 1, where the only coefficient is 0
    test(&["(0, 0)"], 0, true);
    // - more vectors than their dimension
    test(&["(1)", "(1)"], 2, false);
    test(&["()"], 2, false);
    // - full rank modulo 2
    test(&["(2, 1)"], 2, true);
    test(&["(1, 2)", "(3, 1)"], 2, true);
    test(&["(2, 1)", "(1, 2)"], 2, true);
    test(&["(1, 4)", "(2, 1)"], 3, true);
    // - not full rank modulo 2
    test(&["(2, 0)"], 2, false);
    test(&["(1, 2)", "(3, 2)"], 2, false);
    // - pow == 64
    test(&["(18446744073709551614, 1)"], 64, true);
    test(&["(18446744073709551614, 2)"], 64, false);
}

#[test]
fn mod_power_of_2_are_linearly_independent_fail() {
    assert_panic!(
        UnsignedVector::<u64>::mod_power_of_2_are_linearly_independent(
            &vectors(&["(1, 2)", "(1)"]),
            3
        )
    );
    assert_panic!(
        UnsignedVector::<u64>::mod_power_of_2_are_linearly_independent(&vectors(&["(1, 8)"]), 3)
    );
}

#[test]
fn mod_power_of_2_are_linearly_independent_properties() {
    unsigned_matrix_unsigned_pair_gen_var_4().test_properties(|(matrix, pow)| {
        let xs = rows_as_vectors(&matrix);
        let independent = UnsignedVector::mod_power_of_2_are_linearly_independent(&xs, pow);
        if pow < 64 {
            assert_eq!(
                UnsignedVector::mod_are_linearly_independent(&xs, u64::power_of_2(pow)),
                independent
            );
        }
        // For pow > 0, the vectors are independent modulo 2^pow exactly when they are independent
        // modulo 2.
        if pow > 0 {
            let reduced: Vec<UnsignedVector<u64>> = xs
                .iter()
                .map(|x| {
                    UnsignedVector::from_elements(
                        &x.elements.iter().map(|e| e & 1).collect::<Vec<_>>(),
                    )
                })
                .collect();
            assert_eq!(
                UnsignedVector::mod_power_of_2_are_linearly_independent(&reduced, 1),
                independent
            );
        }
    });
}
