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
use malachite_base::unsigned_matrix::random::random_unsigned_matrices_with_dimensions_from_iterator;

fn random_unsigned_matrices_with_dimensions_from_iterator_helper(
    height: u64,
    width: u64,
    bound: u8,
    expected_values: &[&str],
) {
    assert_eq!(
        (random_unsigned_matrices_with_dimensions_from_iterator(
            height,
            width,
            random_unsigned_inclusive_range::<u8>(EXAMPLE_SEED, 0, bound)
        ))
        .take(20)
        .map(|m| m.to_string())
        .collect_vec(),
        expected_values
    );
}

#[test]
fn test_random_unsigned_matrices_with_dimensions_from_iterator() {
    // 0-by-3, elements up to 8
    random_unsigned_matrices_with_dimensions_from_iterator_helper(
        0,
        3,
        8,
        &[
            "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}",
            "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}",
            "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}",
        ],
    );
    // 2-by-2, elements up to 1
    random_unsigned_matrices_with_dimensions_from_iterator_helper(
        2,
        2,
        1,
        &[
            "[1, 0; 0, 0]",
            "[1, 1; 1, 0]",
            "[1, 1; 1, 1]",
            "[0, 1; 1, 1]",
            "[1, 0; 1, 0]",
            "[0, 0; 1, 0]",
            "[0, 0; 1, 1]",
            "[0, 1; 1, 0]",
            "[0, 0; 1, 0]",
            "[0, 1; 1, 1]",
            "[0, 1; 0, 0]",
            "[1, 0; 1, 1]",
            "[0, 0; 0, 1]",
            "[0, 1; 0, 1]",
            "[1, 0; 0, 0]",
            "[0, 1; 0, 1]",
            "[1, 1; 1, 0]",
            "[1, 0; 1, 0]",
            "[0, 0; 0, 0]",
            "[0, 1; 0, 0]",
        ],
    );
    // 2-by-3, elements up to 8
    random_unsigned_matrices_with_dimensions_from_iterator_helper(
        2,
        3,
        8,
        &[
            "[1, 7, 5; 4, 6, 4]",
            "[2, 8, 1; 7, 5, 0]",
            "[2, 6, 3; 5, 2, 2]",
            "[5, 5, 8; 5, 3, 7]",
            "[5, 5, 7; 2, 1, 8]",
            "[4, 7, 1; 2, 7, 0]",
            "[6, 2, 1; 5, 0, 8]",
            "[4, 7, 1; 7, 0, 8]",
            "[6, 2, 7; 7, 0, 2]",
            "[1, 4, 3; 2, 7, 3]",
            "[7, 7, 2; 0, 8, 8]",
            "[6, 5, 2; 4, 6, 4]",
            "[7, 6, 5; 4, 2, 8]",
            "[3, 8, 3; 5, 5, 0]",
            "[7, 0, 8; 5, 2, 4]",
            "[5, 0, 0; 7, 5, 5]",
            "[8, 1, 5; 7, 3, 0]",
            "[0, 5, 1; 4, 2, 8]",
            "[6, 6, 5; 6, 7, 2]",
            "[2, 5, 0; 6, 5, 8]",
        ],
    );
    // 3-by-1, elements up to 100
    random_unsigned_matrices_with_dimensions_from_iterator_helper(
        3,
        1,
        100,
        &[
            "[94; 23; 98]",
            "[70; 92; 52]",
            "[84; 33; 47]",
            "[1; 54; 10]",
            "[47; 17; 89]",
            "[92; 66; 92]",
            "[92; 94; 21]",
            "[51; 39; 46]",
            "[39; 7; 36]",
            "[23; 83; 58]",
            "[13; 64; 73]",
            "[40; 16; 17]",
            "[11; 7; 48]",
            "[53; 46; 49]",
            "[16; 65; 60]",
            "[57; 46; 92]",
            "[63; 98; 21]",
            "[40; 92; 74]",
            "[16; 99; 79]",
            "[92; 94; 53]",
        ],
    );
}

#[test]
#[should_panic]
fn random_unsigned_matrices_with_dimensions_from_iterator_fail_1() {
    // height * width overflows
    let _ = random_unsigned_matrices_with_dimensions_from_iterator(
        u64::MAX,
        2,
        random_unsigned_inclusive_range::<u8>(EXAMPLE_SEED, 0, 8),
    );
}
