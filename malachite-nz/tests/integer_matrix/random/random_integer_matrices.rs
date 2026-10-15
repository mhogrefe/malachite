// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::integer_matrix::random::random_integer_matrices;

fn random_integer_matrices_helper(
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_dimension_numerator: u64,
    mean_dimension_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        (random_integer_matrices(
            EXAMPLE_SEED,
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
fn test_random_integer_matrices() {
    // mean bits = 1/1, mean dimension = 1/1
    random_integer_matrices_helper(
        1,
        1,
        1,
        1,
        &[
            "[]_{0,0}",
            "[]_{0,1}",
            "[]_{0,2}",
            "[]_{0,0}",
            "[0, -2, 0, 0, -1, -1]",
            "[0, 7, -11, 0, -1]",
            "[2]",
            "[2, 1; 0, 3]",
            "[0, 0; 0, -4; -1, 0; 1, -3]",
            "[]_{4,0}",
            "[]_{1,0}",
            "[]_{1,0}",
            "[]_{0,0}",
            "[-2350, 13]",
            "[]_{2,0}",
            "[]_{3,0}",
            "[-1; 0; 0]",
            "[]_{2,0}",
            "[]_{0,1}",
            "[-1; 1]",
        ],
    );
    // mean bits = 2/1, mean dimension = 2/1
    random_integer_matrices_helper(
        2,
        1,
        2,
        1,
        &[
            "[]_{0,1}",
            "[]_{0,11}",
            "[]_{2,0}",
            "[2, -3; -50, -1; 0, -3; 1, 6]",
            "[]_{6,0}",
            "[]_{0,2}",
            "[]_{1,0}",
            "[]_{0,3}",
            "[]_{6,0}",
            "[-2]",
            "[-3872, 0, 0, -1; 1, 4, -26, 1913]",
            "[]_{0,1}",
            "[]_{0,3}",
            "[]_{5,0}",
            "[]_{4,0}",
            "[]_{0,6}",
            "[-14, 1]",
            "[]_{0,1}",
            "[]_{0,0}",
            "[]_{0,0}",
        ],
    );
    // mean bits = 4/1, mean dimension = 2/1
    random_integer_matrices_helper(
        4,
        1,
        2,
        1,
        &[
            "[]_{0,1}",
            "[]_{0,11}",
            "[]_{2,0}",
            "[0, 0; -1, 3; -1, -7; 0, 35]",
            "[]_{6,0}",
            "[]_{0,2}",
            "[]_{1,0}",
            "[]_{0,3}",
            "[]_{6,0}",
            "[1955]",
            "[2354, -1, -34592, 915; -4, -1, -26, -9]",
            "[]_{0,1}",
            "[]_{0,3}",
            "[]_{5,0}",
            "[]_{4,0}",
            "[]_{0,6}",
            "[2, 3]",
            "[]_{0,1}",
            "[]_{0,0}",
            "[]_{0,0}",
        ],
    );
}

#[test]
#[should_panic]
fn random_integer_matrices_fail_1() {
    // mean_bits_denominator is zero
    let _ = random_integer_matrices(EXAMPLE_SEED, 1, 0, 2, 1);
}

#[test]
#[should_panic]
fn random_integer_matrices_fail_2() {
    // mean_bits_numerator is zero
    let _ = random_integer_matrices(EXAMPLE_SEED, 0, 1, 2, 1);
}

#[test]
#[should_panic]
fn random_integer_matrices_fail_3() {
    // mean_dimension_denominator is zero
    let _ = random_integer_matrices(EXAMPLE_SEED, 4, 1, 2, 0);
}

#[test]
#[should_panic]
fn random_integer_matrices_fail_4() {
    // mean_dimension_numerator is zero
    let _ = random_integer_matrices(EXAMPLE_SEED, 4, 1, 0, 1);
}
