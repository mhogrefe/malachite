// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::natural_matrix::random::random_natural_square_matrices;

fn random_natural_square_matrices_helper(
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_size_numerator: u64,
    mean_size_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        (random_natural_square_matrices(
            EXAMPLE_SEED,
            mean_bits_numerator,
            mean_bits_denominator,
            mean_size_numerator,
            mean_size_denominator
        ))
        .take(20)
        .map(|m| m.to_string())
        .collect_vec(),
        expected_values
    );
}

#[test]
fn test_random_natural_square_matrices() {
    // mean bits = 1/1, mean size = 1/1
    random_natural_square_matrices_helper(
        1,
        1,
        1,
        1,
        &[
            "[]_{0,0}",
            "[]_{0,0}",
            "[]_{0,0}",
            "[2]",
            "[]_{0,0}",
            "[3, 0; 0, 0]",
            "[]_{0,0}",
            "[]_{0,0}",
            "[0]",
            "[0, 2, 0, 3, 0, 99; 3, 0, 1, 0, 2, 0; 8, 1, 0, 2, 0, 4; 10, 3, 2, 0, 0, 0; 1, 11, 1, \
            1, 0, 0; 1, 0, 0, 0, 7, 1]",
            "[0]",
            "[0, 0, 1, 0, 0; 1, 0, 1, 0, 0; 1, 0, 0, 0, 0; 0, 0, 1, 0, 0; 1, 0, 0, 0, 8]",
            "[1]",
            "[0]",
            "[0, 13; 3, 1]",
            "[0, 1; 0, 0]",
            "[0, 22, 0, 1; 0, 0, 5, 0; 0, 0, 0, 0; 0, 4, 6, 3]",
            "[0, 61; 2, 3]",
            "[4, 0, 0, 0; 0, 0, 3, 1; 0, 2, 0, 0; 0, 0, 0, 0]",
            "[]_{0,0}",
        ],
    );
    // mean bits = 2/1, mean size = 1/1
    random_natural_square_matrices_helper(
        2,
        1,
        1,
        1,
        &[
            "[]_{0,0}",
            "[]_{0,0}",
            "[]_{0,0}",
            "[2]",
            "[]_{0,0}",
            "[0, 189; 0, 1]",
            "[]_{0,0}",
            "[]_{0,0}",
            "[0]",
            "[7, 0, 7, 1, 0, 2; 1, 1, 1, 1, 0, 0; 1, 1, 5, 14, 0, 3; 1, 0, 0, 424, 0, 16; 0, 0, 2, \
            3, 1, 0; 2, 6, 0, 1301, 0, 1]",
            "[0]",
            "[0, 6, 4, 0, 1; 3, 1, 15, 4, 1; 1, 1, 1, 4, 1; 21, 3, 0, 0, 0; 12, 3, 1, 0, 3]",
            "[0]",
            "[4]",
            "[0, 0; 8, 2]",
            "[1, 545; 1, 1]",
            "[95, 0, 7, 10; 0, 20, 21, 7; 0, 45, 39, 59; 0, 3, 3, 0]",
            "[3, 4; 0, 63]",
            "[0, 3, 6, 0; 0, 6, 1, 3; 2, 1, 1, 0; 0, 1, 0, 8]",
            "[]_{0,0}",
        ],
    );
    // mean bits = 4/1, mean size = 3/2
    random_natural_square_matrices_helper(
        4,
        1,
        3,
        2,
        &[
            "[]_{0,0}",
            "[]_{0,0}",
            "[2, 7229, 18, 7, 0, 3; 0, 1, 1, 0, 0, 6; 4, 0, 129427, 1, 0, 1; 2, 3, 30, 5, 259, 0; \
            104, 1, 6, 0, 1, 0; 137, 1556, 6, 740629, 5, 14]",
            "[]_{0,0}",
            "[4, 15208; 5, 69]",
            "[1, 1468; 2, 0]",
            "[1]",
            "[]_{0,0}",
            "[25, 0, 9; 4, 0, 643750; 1, 29, 12]",
            "[]_{0,0}",
            "[0, 0, 55, 104, 1; 2, 2806768, 70, 1, 5; 52, 13, 3, 23, 0; 2, 108708, 5, 1, 29; 0, 7, \
            0, 11, 0]",
            "[5, 9, 56319, 4, 1, 327; 822, 2, 12, 3, 12, 0; 15, 101, 4687, 1, 15, 0; 2, 5, 0, 0, \
            2, 0; 0, 24, 3, 107, 101, 294; 0, 53, 1, 9, 334, 60]",
            "[]_{0,0}",
            "[3]",
            "[21]",
            "[0]",
            "[]_{0,0}",
            "[4, 6874, 8; 3, 0, 1; 1, 51, 1]",
            "[7, 0, 4, 10; 4276, 2422, 8, 0; 12, 39, 0, 28034; 21, 0, 49, 27]",
            "[21, 12, 0; 1, 29378, 6; 6, 1, 0]",
        ],
    );
}

#[test]
#[should_panic]
fn random_natural_square_matrices_fail_1() {
    // mean_bits_denominator is zero
    let _ = random_natural_square_matrices(EXAMPLE_SEED, 1, 0, 2, 1);
}

#[test]
#[should_panic]
fn random_natural_square_matrices_fail_2() {
    // mean_bits_numerator is zero
    let _ = random_natural_square_matrices(EXAMPLE_SEED, 0, 1, 2, 1);
}

#[test]
#[should_panic]
fn random_natural_square_matrices_fail_3() {
    // mean_size_denominator is zero
    let _ = random_natural_square_matrices(EXAMPLE_SEED, 4, 1, 2, 0);
}

#[test]
#[should_panic]
fn random_natural_square_matrices_fail_4() {
    // mean_size_numerator is zero
    let _ = random_natural_square_matrices(EXAMPLE_SEED, 4, 1, 0, 1);
}
