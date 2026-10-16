// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::integer_matrix::random::random_integer_square_matrices;

fn random_integer_square_matrices_helper(
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_size_numerator: u64,
    mean_size_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        (random_integer_square_matrices(
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
fn test_random_integer_square_matrices() {
    // mean bits = 1/1, mean size = 1/1
    random_integer_square_matrices_helper(
        1,
        1,
        1,
        1,
        &[
            "[]_{0,0}",
            "[]_{0,0}",
            "[]_{0,0}",
            "[0]",
            "[]_{0,0}",
            "[-2, 0; 0, -1]",
            "[]_{0,0}",
            "[]_{0,0}",
            "[-1]",
            "[0, 7, -11, 0, -1, 2; 2, 1, 0, 3, 0, 0; 0, -4, -1, 0, 1, -3; -2350, 13, -1, 0, 0, -1; \
            1, 10, -1, 1, 1, 1; 3, -1, -62, -4, -8, 0]",
            "[-9]",
            "[0, -1, 0, 3, -1; 0, 10, 6, 3, -1; 2, -14, 3, 0, 1; -4, 1, 1, 1, -2; 0, -1, -14, 30, \
            0]",
            "[1]",
            "[0]",
            "[1, 1; -7, 0]",
            "[-1, 0; -6, 2]",
            "[0, -13, 19, -5; -1, 1, 0, 0; -9, 1, 0, 0; 1, -1, 0, -3]",
            "[0, 1; 2, 0]",
            "[-1, 4, -1, 1; -15, 1, 0, -7; 4, 1, 0, -1; 3, 6, 0, -1]",
            "[]_{0,0}",
        ],
    );
    // mean bits = 2/1, mean size = 1/1
    random_integer_square_matrices_helper(
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
            "[-3, -50; -1, 0]",
            "[]_{0,0}",
            "[]_{0,0}",
            "[-3]",
            "[1, 6, -2, -3872, 0, 0; -1, 1, 4, -26, 1913, -14; 1, 7, 8, -4, 2, 0; 0, -1, -73, -4, \
            -2, 85; -1, -2, -32, 8, 9, 1; -1, -1, 1, -1, 1, 9]",
            "[0]",
            "[1, 0, 6, -1, 5; 60, -3, 40, 33, -2; 2, 0, -22, -1, 1; -20, 0, 1, 3, -1; 0, -1, 1, 0, \
            -1]",
            "[7]",
            "[29]",
            "[-1, 11; -5, 17]",
            "[31, -236; 3, 0]",
            "[3, -14, 14, 2; 0, -3, -1, -1; -1, 7, -4, 0; 1, 0, -1, 0]",
            "[-3, -6; 8, 0]",
            "[3, 11, 1, -2; -21, -2, 9, 0; 2, -2, -7, -21; -4, -2, 2, 0]",
            "[]_{0,0}",
        ],
    );
    // mean bits = 4/1, mean size = 3/2
    random_integer_square_matrices_helper(
        4,
        1,
        3,
        2,
        &[
            "[]_{0,0}",
            "[]_{0,0}",
            "[0, 0, -1, 3, -1, -7; 0, 35, 1955, 2354, -1, -34592; 915, -4, -1, -26, -9, 2; 3, \
            -131, 680, -4, 1, -1; 41, -12, -3598, 3, -3, 0; 0, 30, -4, -40, 1, 0]",
            "[]_{0,0}",
            "[3, 743; 60, -1]",
            "[-38, 57; -81, 256]",
            "[-6]",
            "[]_{0,0}",
            "[1, 14621, 124; -3, 1, -1; -1, 4, -1]",
            "[]_{0,0}",
            "[14, 97, -12, 1, 7; -7, 92994, 4, 1, 12931; 429, 0, -154663, -7, 25; -9, 0, 1, 748, \
            -3; 3, 2, 0, -2, 1732]",
            "[-187, 2, 79, -3, -7, -8; 2663, -2, 1, 1, -2, 1107; -363, 5, -22, -21, 0, 26; 1, 0, \
            0, -20814, -284, 1; 0, -21, 20, -474, 2, 0; -149423, -3, -3, -51, -383, 11]",
            "[]_{0,0}",
            "[24148]",
            "[-6]",
            "[-1]",
            "[]_{0,0}",
            "[-118, -107440, -12; -455, 0, 0; -2, -1, -1]",
            "[1, -37, 1, 9; 10, 6, 2, -10; -32, 0, 1, 1; 30, -952, -7029, 7]",
            "[-2, 42, -146; 6, 47, 0; -1, -5, 1]",
        ],
    );
}

#[test]
#[should_panic]
fn random_integer_square_matrices_fail_1() {
    // mean_bits_denominator is zero
    let _ = random_integer_square_matrices(EXAMPLE_SEED, 1, 0, 2, 1);
}

#[test]
#[should_panic]
fn random_integer_square_matrices_fail_2() {
    // mean_bits_numerator is zero
    let _ = random_integer_square_matrices(EXAMPLE_SEED, 0, 1, 2, 1);
}

#[test]
#[should_panic]
fn random_integer_square_matrices_fail_3() {
    // mean_size_denominator is zero
    let _ = random_integer_square_matrices(EXAMPLE_SEED, 4, 1, 2, 0);
}

#[test]
#[should_panic]
fn random_integer_square_matrices_fail_4() {
    // mean_size_numerator is zero
    let _ = random_integer_square_matrices(EXAMPLE_SEED, 4, 1, 0, 1);
}
