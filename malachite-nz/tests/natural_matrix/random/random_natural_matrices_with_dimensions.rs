// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::natural_matrix::random::random_natural_matrices_with_dimensions;

fn random_natural_matrices_with_dimensions_helper(
    height: u64,
    width: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        (random_natural_matrices_with_dimensions(
            EXAMPLE_SEED,
            height,
            width,
            mean_bits_numerator,
            mean_bits_denominator
        ))
        .take(20)
        .map(|m| m.to_string())
        .collect_vec(),
        expected_values
    );
}

#[test]
fn test_random_natural_matrices_with_dimensions() {
    // 0-by-3, mean bits = 4/1
    random_natural_matrices_with_dimensions_helper(
        0,
        3,
        4,
        1,
        &[
            "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}",
            "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}",
            "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}",
        ],
    );
    // 2-by-2, mean bits = 1/1
    random_natural_matrices_with_dimensions_helper(
        2,
        2,
        1,
        1,
        &[
            "[0, 14; 0, 8]",
            "[2, 6; 1, 0]",
            "[0, 0; 0, 0]",
            "[1, 1; 0, 0]",
            "[1, 1; 0, 0]",
            "[0, 0; 1, 1]",
            "[0, 0; 0, 3]",
            "[1, 0; 8, 0]",
            "[0, 0; 2, 0]",
            "[1, 0; 0, 1]",
            "[0, 5; 0, 0]",
            "[0, 0; 0, 2]",
            "[0, 0; 0, 1]",
            "[1, 0; 1, 0]",
            "[6, 1; 71, 0]",
            "[1, 15; 1, 0]",
            "[1, 0; 0, 1]",
            "[0, 0; 0, 1]",
            "[0, 2; 1, 2]",
            "[0, 1; 1, 0]",
        ],
    );
    // 2-by-3, mean bits = 2/1
    random_natural_matrices_with_dimensions_helper(
        2,
        3,
        2,
        1,
        &[
            "[54, 0, 0; 2, 2, 0]",
            "[2, 18, 4; 1, 1, 0]",
            "[3, 2, 9; 3, 1, 1]",
            "[1, 30, 7; 3, 1, 1]",
            "[13, 7, 0; 6, 84, 1]",
            "[57, 0, 0; 0, 0, 1]",
            "[0, 87, 0; 44, 12, 1]",
            "[0, 0, 0; 32, 1, 62]",
            "[0, 1, 0; 5, 3, 6]",
            "[3684, 2, 0; 3, 0, 3]",
            "[0, 1, 58; 1, 1, 55]",
            "[264, 3, 0; 11, 6, 1]",
            "[13, 82, 3; 0, 0, 26]",
            "[0, 0, 0; 49, 1, 0]",
            "[0, 2, 0; 1, 0, 2]",
            "[43, 15, 2; 1, 1, 1]",
            "[5, 4, 5; 3, 9, 307]",
            "[0, 3, 1; 1, 0, 0]",
            "[0, 0, 17; 0, 0, 1]",
            "[3, 3, 44; 0, 1, 1]",
        ],
    );
    // 3-by-1, mean bits = 4/1
    random_natural_matrices_with_dimensions_helper(
        3,
        1,
        4,
        1,
        &[
            "[14; 8; 10]",
            "[1; 1; 0]",
            "[1; 184; 15]",
            "[99; 1; 3]",
            "[3; 6; 1]",
            "[4; 494; 3]",
            "[13; 1; 1]",
            "[1757; 1775; 3514]",
            "[2; 1799; 0]",
            "[5; 0; 7]",
            "[1; 2; 0]",
            "[4724; 3; 24928]",
            "[0; 1; 0]",
            "[0; 6; 1]",
            "[133; 3; 0]",
            "[6; 0; 0]",
            "[1; 6; 325]",
            "[7; 102; 2]",
            "[60; 59; 23]",
            "[776; 15; 3]",
        ],
    );
}

#[test]
#[should_panic]
fn random_natural_matrices_with_dimensions_fail_1() {
    // mean_bits_denominator is zero
    let _ = random_natural_matrices_with_dimensions(EXAMPLE_SEED, 2, 2, 1, 0);
}

#[test]
#[should_panic]
fn random_natural_matrices_with_dimensions_fail_2() {
    // mean_bits_numerator is zero
    let _ = random_natural_matrices_with_dimensions(EXAMPLE_SEED, 2, 2, 0, 1);
}

#[test]
#[should_panic]
fn random_natural_matrices_with_dimensions_fail_3() {
    // height * width overflows
    let _ = random_natural_matrices_with_dimensions(EXAMPLE_SEED, u64::MAX, 2, 4, 1);
}
