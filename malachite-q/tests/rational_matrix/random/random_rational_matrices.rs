// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_q::rational_matrix::random::random_rational_matrices;

fn random_rational_matrices_helper(
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_dimension_numerator: u64,
    mean_dimension_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        (random_rational_matrices(
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
fn test_random_rational_matrices() {
    // mean bits = 2/1, mean dimension = 1/1
    random_rational_matrices_helper(
        2,
        1,
        1,
        1,
        &[
            "[]_{0,0}",
            "[]_{0,1}",
            "[]_{0,2}",
            "[]_{0,0}",
            "[0, -49/4, 1, 0, 0, -4/3]",
            "[5, 1, 5/4, 0, -1]",
            "[-1]",
            "[-1, 7/3; 0, -1]",
            "[-4, 15; -19/7, -1/4; 1, -1; 0, 0]",
            "[]_{4,0}",
            "[]_{1,0}",
            "[]_{1,0}",
            "[]_{0,0}",
            "[0, -3]",
            "[]_{2,0}",
            "[]_{3,0}",
            "[5/7; -8; -3]",
            "[]_{2,0}",
            "[]_{0,1}",
            "[-31; 0]",
        ],
    );
    // mean bits = 3/1, mean dimension = 2/1
    random_rational_matrices_helper(
        3,
        1,
        2,
        1,
        &[
            "[]_{0,1}",
            "[]_{0,11}",
            "[]_{2,0}",
            "[0, -113/4; 1/2, 0; 0, -4; 9/28, 1]",
            "[]_{6,0}",
            "[]_{0,2}",
            "[]_{1,0}",
            "[]_{0,3}",
            "[]_{6,0}",
            "[2863/12]",
            "[1/3, -1/5, -40/3, -1; 5/7, 1/3, -1, -11/2]",
            "[]_{0,1}",
            "[]_{0,3}",
            "[]_{5,0}",
            "[]_{4,0}",
            "[]_{0,6}",
            "[38, -46/3]",
            "[]_{0,1}",
            "[]_{0,0}",
            "[]_{0,0}",
        ],
    );
    // mean bits = 4/1, mean dimension = 2/1
    random_rational_matrices_helper(
        4,
        1,
        2,
        1,
        &[
            "[]_{0,1}",
            "[]_{0,11}",
            "[]_{2,0}",
            "[-9/91, 0; 82, -8; 1/3, -5; 47/28, 44]",
            "[]_{6,0}",
            "[]_{0,2}",
            "[]_{1,0}",
            "[]_{0,3}",
            "[]_{6,0}",
            "[1979435/4]",
            "[4, -3/5, -3, -2; 3/31, 1/3, -1, -5]",
            "[]_{0,1}",
            "[]_{0,3}",
            "[]_{5,0}",
            "[]_{4,0}",
            "[]_{0,6}",
            "[83, -1]",
            "[]_{0,1}",
            "[]_{0,0}",
            "[]_{0,0}",
        ],
    );
}

#[test]
#[should_panic]
fn random_rational_matrices_fail_1() {
    // mean_bits_denominator is zero
    let _ = random_rational_matrices(EXAMPLE_SEED, 1, 0, 2, 1);
}

#[test]
#[should_panic]
fn random_rational_matrices_fail_2() {
    // mean_bits_numerator is zero
    let _ = random_rational_matrices(EXAMPLE_SEED, 0, 1, 2, 1);
}

#[test]
#[should_panic]
fn random_rational_matrices_fail_3() {
    // mean_dimension_denominator is zero
    let _ = random_rational_matrices(EXAMPLE_SEED, 4, 1, 2, 0);
}

#[test]
#[should_panic]
fn random_rational_matrices_fail_4() {
    // mean_dimension_numerator is zero
    let _ = random_rational_matrices(EXAMPLE_SEED, 4, 1, 0, 1);
}
