// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::integer::Integer;
use malachite_nz::integer::random::uniform_random_integer_inclusive_range;
use malachite_nz::integer_matrix::random::random_integer_square_matrices_from_iterator;

fn random_integer_square_matrices_from_iterator_helper(
    bound: i32,
    mean_size_numerator: u64,
    mean_size_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        (random_integer_square_matrices_from_iterator(
            EXAMPLE_SEED,
            &|seed| uniform_random_integer_inclusive_range(
                seed,
                Integer::from(-bound),
                Integer::from(bound)
            ),
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
fn test_random_integer_square_matrices_from_iterator() {
    // elements bounded by 1, mean size = 1/1
    random_integer_square_matrices_from_iterator_helper(
        1,
        1,
        1,
        &[
            "[]_{0,0}",
            "[]_{0,0}",
            "[]_{0,0}",
            "[-1]",
            "[]_{0,0}",
            "[-1, 0; 0, 0]",
            "[]_{0,0}",
            "[]_{0,0}",
            "[0]",
            "[0, 0, 0, 1, -1, -1; -1, -1, -1, 0, 1, 0; 1, 0, -1, 0, 0, 0; 1, 0, -1, -1, -1, -1; 0, \
            1, 1, 1, -1, 1; 1, 0, 1, 1, 1, 0]",
            "[0]",
            "[-1, -1, 0, 1, 0; 1, -1, 0, 0, 1; 1, 1, -1, -1, 0; 0, 0, 0, 0, 1; 0, -1, -1, 1, 0]",
            "[-1]",
            "[-1]",
            "[1, 0; -1, 1]",
            "[1, 1; 0, 0]",
            "[0, 0, -1, 1; 0, -1, 0, 1; 0, 0, -1, 0; 0, 1, 1, 1]",
            "[0, 0; -1, 1]",
            "[1, 1, -1, -1; 1, -1, -1, 0; -1, -1, 1, 0; -1, 0, 0, -1]",
            "[]_{0,0}",
        ],
    );
    // elements bounded by 8, mean size = 1/1
    random_integer_square_matrices_from_iterator_helper(
        8,
        1,
        1,
        &[
            "[]_{0,0}",
            "[]_{0,0}",
            "[]_{0,0}",
            "[-4]",
            "[]_{0,0}",
            "[3, -8; 3, 5]",
            "[]_{0,0}",
            "[]_{0,0}",
            "[1]",
            "[5, 7, -7, 6, 4, 2; -6, 5, -4, -1, 2, 1; -1, 3, -8, 0, -1, 8; 1, 8, -2, 2, -5, -7; 7, \
            -6, 1, 0, 1, 6; -4, -5, 1, 1, -1, -5]",
            "[-1]",
            "[6, 6, 4, -7, -3; -3, -7, -1, 5, 0; 8, 4, 0, 2, -6; -3, 1, -5, 3, 5; 1, 3, 8, 7, -2]",
            "[-1]",
            "[3]",
            "[-8, 6; 2, 4]",
            "[0, 5; -4, 6]",
            "[-3, -4, 3, 5; 4, -4, 1, -4; 7, -4, 8, -3; -1, -2, -5, 6]",
            "[-7, -4; 2, 6]",
            "[-1, 6, -5, 4; -5, -6, -4, 6; 4, 3, -3, -7; 8, 6, -7, 8]",
            "[]_{0,0}",
        ],
    );
    // elements bounded by 100, mean size = 3/2
    random_integer_square_matrices_from_iterator_helper(
        100,
        3,
        2,
        &[
            "[]_{0,0}",
            "[]_{0,0}",
            "[64, -89, -4, -37, -9, -47; 57, -27, -79, -85, 93, -40; 16, 48, -43, -45, -98, -45; \
            64, 21, 35, -39, 13, 70; 5, -25, -36, 24, 4, 35; -91, -46, 19, 12, 98, 38]",
            "[]_{0,0}",
            "[-1, -35; -21, 50]",
            "[-10, -2; -71, -27]",
            "[-22]",
            "[]_{0,0}",
            "[-81, 47, -47; -82, -64, -81; 31, -27, -27]",
            "[]_{0,0}",
            "[67, 31, 35, -18, -54; 72, -76, -63, 65, -29; -87, 4, -52, 46, -39; 72, 68, 70, 82, \
            82; 30, -19, -95, 85, -48]",
            "[31, 39, 41, -77, -41, -27; 79, -89, 90, -51, -39, -84; 47, -83, 13, -30, -74, 14; 3, \
            45, -9, -89, 17, 92; -10, -22, -78, 80, 51, -58; 72, 36, 73, 79, -64, -86]",
            "[]_{0,0}",
            "[23]",
            "[-63]",
            "[96]",
            "[]_{0,0}",
            "[-19, -71, 9; -96, -27, 64; -39, -47, -11]",
            "[-21, -96, 12, -95; -62, -37, -1, 15; 42, -19, 84, -64; -70, -45, -26, -86]",
            "[67, 82, 74; 58, 95, -70; -82, -1, -66]",
        ],
    );
}

#[test]
#[should_panic]
fn random_integer_square_matrices_from_iterator_fail_1() {
    // mean_size_denominator is zero
    let _ = random_integer_square_matrices_from_iterator(
        EXAMPLE_SEED,
        &|seed| uniform_random_integer_inclusive_range(seed, Integer::from(-8), Integer::from(8)),
        2,
        0,
    );
}

#[test]
#[should_panic]
fn random_integer_square_matrices_from_iterator_fail_2() {
    // mean_size_numerator is zero
    let _ = random_integer_square_matrices_from_iterator(
        EXAMPLE_SEED,
        &|seed| uniform_random_integer_inclusive_range(seed, Integer::from(-8), Integer::from(8)),
        0,
        1,
    );
}
