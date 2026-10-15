// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::natural_matrix::random::striped_random_natural_matrices;

fn striped_random_natural_matrices_helper(
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_dimension_numerator: u64,
    mean_dimension_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        (striped_random_natural_matrices(
            EXAMPLE_SEED,
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_bits_numerator,
            mean_bits_denominator,
            mean_dimension_numerator,
            mean_dimension_denominator
        ))
        .take(20)
        .map(|m| m.to_string())
        .collect_vec(),
        expected_values
    );
}

#[test]
fn test_striped_random_natural_matrices() {
    // mean stripe = 2/1, mean bits = 2/1, mean dimension = 1/1
    striped_random_natural_matrices_helper(
        2,
        1,
        2,
        1,
        1,
        1,
        &[
            "[]_{0,0}",
            "[]_{0,1}",
            "[]_{0,2}",
            "[]_{0,0}",
            "[3, 0, 177, 0, 1, 0]",
            "[5, 0, 4, 1, 0]",
            "[3]",
            "[1, 1; 1, 1]",
            "[0, 0; 1, 1; 5, 9; 0, 2]",
            "[]_{4,0}",
            "[]_{1,0}",
            "[]_{1,0}",
            "[]_{0,0}",
            "[1, 0]",
            "[]_{2,0}",
            "[]_{3,0}",
            "[0; 321; 0]",
            "[]_{2,0}",
            "[]_{0,1}",
            "[18; 0]",
        ],
    );
    // mean stripe = 16/1, mean bits = 4/1, mean dimension = 2/1
    striped_random_natural_matrices_helper(
        16,
        1,
        4,
        1,
        2,
        1,
        &[
            "[]_{0,1}",
            "[]_{0,11}",
            "[]_{2,0}",
            "[3, 7999; 24, 7; 0, 2; 0, 1]",
            "[]_{6,0}",
            "[]_{0,2}",
            "[]_{1,0}",
            "[]_{0,3}",
            "[]_{6,0}",
            "[1]",
            "[0, 0, 4, 4; 0, 65536, 1, 0]",
            "[]_{0,1}",
            "[]_{0,3}",
            "[]_{5,0}",
            "[]_{4,0}",
            "[]_{0,6}",
            "[1, 3]",
            "[]_{0,1}",
            "[]_{0,0}",
            "[]_{0,0}",
        ],
    );
    // mean stripe = 4/1, mean bits = 8/1, mean dimension = 2/1
    striped_random_natural_matrices_helper(
        4,
        1,
        8,
        1,
        2,
        1,
        &[
            "[]_{0,1}",
            "[]_{0,11}",
            "[]_{2,0}",
            "[3, 127; 62, 3; 46, 79; 3, 1]",
            "[]_{6,0}",
            "[]_{0,2}",
            "[]_{1,0}",
            "[]_{0,3}",
            "[]_{6,0}",
            "[4160]",
            "[0, 8, 0, 1; 2323511445475, 5, 8321966083, 0]",
            "[]_{0,1}",
            "[]_{0,3}",
            "[]_{5,0}",
            "[]_{4,0}",
            "[]_{0,6}",
            "[15, 0]",
            "[]_{0,1}",
            "[]_{0,0}",
            "[]_{0,0}",
        ],
    );
}

#[test]
#[should_panic]
fn striped_random_natural_matrices_fail_1() {
    // mean_stripe_denominator is zero
    let _ = striped_random_natural_matrices(EXAMPLE_SEED, 1, 0, 4, 1, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_natural_matrices_fail_2() {
    // mean_stripe_numerator < mean_stripe_denominator
    let _ = striped_random_natural_matrices(EXAMPLE_SEED, 1, 2, 4, 1, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_natural_matrices_fail_3() {
    // mean_bits_numerator is zero
    let _ = striped_random_natural_matrices(EXAMPLE_SEED, 4, 1, 0, 1, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_natural_matrices_fail_4() {
    // mean_dimension_denominator is zero
    let _ = striped_random_natural_matrices(EXAMPLE_SEED, 4, 1, 4, 1, 2, 0);
}
