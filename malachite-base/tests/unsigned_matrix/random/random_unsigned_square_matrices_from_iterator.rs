// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::random::random_unsigned_inclusive_range;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::unsigned_matrix::random::random_unsigned_square_matrices_from_iterator;

fn random_unsigned_square_matrices_from_iterator_helper(
    bound: u8,
    mean_size_numerator: u64,
    mean_size_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        (random_unsigned_square_matrices_from_iterator(
            EXAMPLE_SEED,
            &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, bound),
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
fn test_random_unsigned_square_matrices_from_iterator() {
    // elements up to 1, mean size = 1/1
    random_unsigned_square_matrices_from_iterator_helper(
        1,
        1,
        1,
        &[
            "[]_{0,0}",
            "[]_{0,0}",
            "[]_{0,0}",
            "[0]",
            "[]_{0,0}",
            "[0, 1; 0, 0]",
            "[]_{0,0}",
            "[]_{0,0}",
            "[1]",
            "[0, 1, 0, 0, 1, 1; 0, 0, 1, 0, 0, 1; 1, 1, 1, 0, 1, 0; 0, 1, 1, 1, 1, 1; 1, 0, 0, 0, \
            1, 1; 1, 0, 0, 0, 0, 1]",
            "[1]",
            "[1, 0, 1, 1, 0; 1, 1, 1, 1, 0; 1, 0, 1, 1, 0; 0, 0, 1, 0, 0; 0, 1, 1, 0, 1]",
            "[0]",
            "[0]",
            "[0, 0; 0, 1]",
            "[1, 1; 0, 1]",
            "[1, 1, 1, 0; 0, 0, 0, 0; 1, 0, 0, 1; 1, 1, 1, 1]",
            "[0, 0; 1, 0]",
            "[0, 0, 0, 1; 1, 1, 0, 0; 1, 1, 1, 1; 0, 1, 1, 0]",
            "[]_{0,0}",
        ],
    );
    // elements up to 8, mean size = 1/1
    random_unsigned_square_matrices_from_iterator_helper(
        8,
        1,
        1,
        &[
            "[]_{0,0}",
            "[]_{0,0}",
            "[]_{0,0}",
            "[4]",
            "[]_{0,0}",
            "[4, 5; 7, 1]",
            "[]_{0,0}",
            "[]_{0,0}",
            "[6]",
            "[1, 1, 0, 1, 4, 3; 1, 1, 4, 4, 0, 6; 7, 1, 1, 1, 7, 5; 2, 2, 2, 4, 8, 5; 3, 3, 7, 0, \
            0, 8; 4, 0, 3, 7, 8, 4]",
            "[5]",
            "[7, 0, 6, 5, 5; 4, 5, 1, 0, 0; 8, 5, 3, 8, 8; 3, 2, 7, 7, 0; 7, 2, 5, 4, 2]",
            "[1]",
            "[3]",
            "[0, 5; 7, 3]",
            "[8, 3; 5, 7]",
            "[7, 4, 2, 4; 0, 5, 1, 0; 5, 6, 0, 5; 2, 8, 4, 0]",
            "[8, 5; 3, 1]",
            "[7, 6, 1, 1; 4, 4, 3, 7; 5, 0, 6, 7; 3, 5, 0, 0]",
            "[]_{0,0}",
        ],
    );
    // elements up to 100, mean size = 3/2
    random_unsigned_square_matrices_from_iterator_helper(
        100,
        3,
        2,
        &[
            "[]_{0,0}",
            "[]_{0,0}",
            "[36, 25, 71, 67, 91, 87; 17, 22, 56, 15, 100, 39; 56, 94, 65, 22, 1, 83; 50, 68, 15, \
            71, 75, 8; 93, 46, 92, 18, 70, 91; 30, 71, 3, 0, 9, 76]",
            "[]_{0,0}",
            "[13, 86; 3, 90]",
            "[92, 53; 85, 25]",
            "[85]",
            "[]_{0,0}",
            "[26, 64, 3; 84, 53, 24; 34, 35, 90]",
            "[]_{0,0}",
            "[95, 63, 90, 59, 46; 56, 47, 60, 57, 38; 18, 49, 54, 66, 62; 78, 58, 71, 4, 80; 2, \
            20, 3, 78, 75]",
            "[32, 78, 84, 3, 44, 19; 54, 54, 19, 96, 28, 18; 45, 26, 87, 30, 88, 27; 61, 65, 6, \
            66, 31, 47; 70, 49, 27, 66, 83, 26; 58, 79, 89, 23, 25, 7]",
            "[]_{0,0}",
            "[30]",
            "[97]",
            "[58]",
            "[]_{0,0}",
            "[28, 79, 4; 43, 46, 18; 77, 77, 46]",
            "[47, 45, 73, 8; 9, 86, 59, 7; 59, 28, 2, 53; 97, 25, 6, 80]",
            "[65, 49, 54; 58, 67, 15; 59, 82, 9]",
        ],
    );
}

#[test]
#[should_panic]
fn random_unsigned_square_matrices_from_iterator_fail_1() {
    // mean_size_denominator is zero
    let _ = random_unsigned_square_matrices_from_iterator(
        EXAMPLE_SEED,
        &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, 8),
        2,
        0,
    );
}

#[test]
#[should_panic]
fn random_unsigned_square_matrices_from_iterator_fail_2() {
    // mean_size_numerator is zero
    let _ = random_unsigned_square_matrices_from_iterator(
        EXAMPLE_SEED,
        &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, 8),
        0,
        1,
    );
}
