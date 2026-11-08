// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::linear_algebra::traits::ModAreLinearlyIndependent;
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_9;
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
fn test_mod_are_linearly_independent() {
    let test = |xs: &[&str], m: u64, out: bool| {
        let xs = vectors(xs);
        assert_eq!(UnsignedVector::mod_are_linearly_independent(&xs, m), out);
        assert_eq!(mod_are_linearly_independent_naive(&xs, m), out);
        assert_eq!(mod_are_linearly_independent_by_prime_factors(&xs, m), out);
    };
    // (expected values computed independently with python, by trying every combination)
    // - no vectors
    test(&[], 5, true);
    // - modulo 1, where the only coefficient is 0
    test(&["(0, 0)"], 1, true);
    test(&["()"], 1, true);
    // - vectors of dimension 0 are zero vectors
    test(&["()"], 5, false);
    // - a prime modulus
    test(&["(1, 2)", "(0, 1)"], 7, true);
    test(&["(1, 2)", "(2, 4)"], 7, false);
    // - more vectors than their dimension
    test(&["(1, 0)", "(0, 1)", "(1, 1)"], 7, false);
    test(&["(2)", "(3)"], 6, false);
    // - a composite modulus, where a vector whose elements share a factor with it is dependent
    test(&["(2, 0)"], 4, false);
    test(&["(3)"], 6, false);
    test(&["(5)"], 6, true);
    test(&["(2, 3)"], 6, true);
    test(&["(2, 1)"], 4, true);
    test(&["(2, 1)", "(1, 2)"], 4, true);
    test(&["(2, 0)", "(1, 3)"], 4, false);
    test(&["(3, 0)", "(0, 2)"], 6, false);
    test(&["(4, 6)", "(6, 9)"], 12, false);
    test(&["(1, 4)", "(2, 1)"], 8, true);
}

#[test]
fn test_mod_are_linearly_independent_large_modulus() {
    let test = |xs: &[&str], m: u64, out: bool| {
        let xs = vectors(xs);
        assert_eq!(UnsignedVector::mod_are_linearly_independent(&xs, m), out);
        assert_eq!(mod_are_linearly_independent_by_prime_factors(&xs, m), out);
    };
    // - u64::MAX = 3 * 5 * 17 * 257 * 641 * 65537 * 6700417
    test(&["(1, 0)", "(0, 1)"], u64::MAX, true);
    test(&["(3, 0)"], u64::MAX, false);
    test(&["(3, 5)"], u64::MAX, true);
    // - 2^63, whose only prime factor is 2
    test(&["(9223372036854775806, 1)"], 1 << 63, true);
    test(&["(9223372036854775806, 2)"], 1 << 63, false);
}

#[test]
fn mod_are_linearly_independent_fail() {
    assert_panic!(UnsignedVector::<u64>::mod_are_linearly_independent(
        &vectors(&["(1)"]),
        0
    ));
    assert_panic!(UnsignedVector::<u64>::mod_are_linearly_independent(
        &vectors(&["(1, 2)", "(1)"]),
        5
    ));
    assert_panic!(UnsignedVector::<u64>::mod_are_linearly_independent(
        &vectors(&["(1, 5)"]),
        5
    ));
}

#[test]
fn mod_are_linearly_independent_properties() {
    unsigned_matrix_unsigned_pair_gen_var_9().test_properties(|(matrix, m)| {
        let xs = rows_as_vectors(&matrix);
        let independent = UnsignedVector::mod_are_linearly_independent(&xs, m);
        // Vectors are independent modulo m exactly when they are independent modulo every prime
        // factor of m.
        assert_eq!(
            mod_are_linearly_independent_by_prime_factors(&xs, m),
            independent
        );
        if u32::try_from(xs.len()).is_ok_and(|r| (m as f64).powi(r as i32) <= 4096.0) {
            assert_eq!(mod_are_linearly_independent_naive(&xs, m), independent);
        }
        // The order of the vectors does not matter.
        let mut reversed = xs.clone();
        reversed.reverse();
        assert_eq!(
            UnsignedVector::mod_are_linearly_independent(&reversed, m),
            independent
        );
        // A subset of independent vectors is independent.
        if independent && !xs.is_empty() {
            assert!(UnsignedVector::mod_are_linearly_independent(
                &xs[..xs.len() - 1],
                m
            ));
        }
        // Adding a zero vector makes the vectors dependent, unless m is 1.
        if m != 1
            && let Some(first) = xs.first()
        {
            let mut with_zero = xs.clone();
            with_zero.push(UnsignedVector::from_elements(&vec![
                0;
                first.elements.len()
            ]));
            assert!(!UnsignedVector::mod_are_linearly_independent(&with_zero, m));
        }
    });
}
