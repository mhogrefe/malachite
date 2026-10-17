// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::integer_matrix::random::random_integer_matrices_with_dimensions;

fn random_integer_matrices_with_dimensions_helper(
    height: u64,
    width: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        (random_integer_matrices_with_dimensions(
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
fn test_random_integer_matrices_with_dimensions() {
    // 0-by-3, mean bits = 4/1
    random_integer_matrices_with_dimensions_helper(
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
    random_integer_matrices_with_dimensions_helper(
        2,
        2,
        1,
        1,
        &[
            "[2, -2; -6, 0]",
            "[0, -1; -2, -2]",
            "[2, 0; 0, -1]",
            "[-7, -2; 5, 0]",
            "[1, 1; 1, -2]",
            "[-1, 1; 1, 1]",
            "[-78, 0; -1, 1]",
            "[0, -1; 0, 0]",
            "[1, -1; 0, 0]",
            "[3, 0; 1, -7]",
            "[2, -1; 0, -3]",
            "[1, 0; 0, 2]",
            "[2, 48; 0, 1]",
            "[1, 1; 0, 0]",
            "[1, 2; 5, 0]",
            "[0, 0; 63, 2]",
            "[1, 0; 12, 0]",
            "[1, 15; -1, 7]",
            "[0, 1; 0, 1]",
            "[0, -3; 3, -6]",
        ],
    );
    // 2-by-3, mean bits = 2/1
    random_integer_matrices_with_dimensions_helper(
        2,
        3,
        2,
        1,
        &[
            "[1, 0, -24; 18, -6, -18]",
            "[4, 2, 0; -3, -1, -1]",
            "[-9, -35, -6; -8, 0, 0]",
            "[1, 6, 1; -13, 1, -14]",
            "[0, 61, 7; -10, 4, -15]",
            "[-3, 0, -7; 15, -1, 12]",
            "[-3, 4, 6; 0, 14, -2]",
            "[5, 1, -2; 228, -14, 1]",
            "[-1, 0, 102; -90, 28, -1]",
            "[15, 1, 31; -35, 0, -1]",
            "[205607, 0, -1; 1, 3, -2]",
            "[1, -1, 1; 3, -1, 619]",
            "[3, 0, -1; 8, -868, 0]",
            "[-1, 41, 8; 9, 9, 0]",
            "[-9, 0, 0; 11, 13, 7]",
            "[-2, 0, 0; 0, 33, 0]",
            "[3, -3, 15; 12, -4, 5510]",
            "[-1, -28, -1; 2, -1, 15]",
            "[-1, 0, -707; 0, 0, 0]",
            "[9, -16, 3; 0, 0, 0]",
        ],
    );
    // 3-by-1, mean bits = 4/1
    random_integer_matrices_with_dimensions_helper(
        3,
        1,
        4,
        1,
        &[
            "[2; 152; 1]",
            "[0; -62; 5282]",
            "[0; 28; 4427344568]",
            "[79; -11; -7330]",
            "[-1; -5667523; 1618]",
            "[120; -1; -1006]",
            "[-7; -45; -1686]",
            "[-206; 3; -1]",
            "[-1; 2516; 775]",
            "[5; 95; 23]",
            "[140; 2; 0]",
            "[499; -4; 14]",
            "[0; 1; -8]",
            "[-5; 0; -3295133]",
            "[0; 1; 2]",
            "[-6; 37; -7]",
            "[-2; 0; 1]",
            "[2; -3; 7]",
            "[-1; 0; 1]",
            "[-11; 2; -15]",
        ],
    );
}

#[test]
#[should_panic]
fn random_integer_matrices_with_dimensions_fail_1() {
    // mean_bits_denominator is zero
    let _ = random_integer_matrices_with_dimensions(EXAMPLE_SEED, 2, 2, 1, 0);
}

#[test]
#[should_panic]
fn random_integer_matrices_with_dimensions_fail_2() {
    // mean_bits_numerator is zero
    let _ = random_integer_matrices_with_dimensions(EXAMPLE_SEED, 2, 2, 0, 1);
}

#[test]
#[should_panic]
fn random_integer_matrices_with_dimensions_fail_3() {
    // height * width overflows
    let _ = random_integer_matrices_with_dimensions(EXAMPLE_SEED, u64::MAX, 2, 4, 1);
}
