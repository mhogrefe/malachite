// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::basic::traits::Zero;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::natural::Natural;
use malachite_nz::natural::random::uniform_random_natural_inclusive_range;
use malachite_nz::natural_matrix::random::random_natural_matrices_with_dimensions_from_iterator;

fn random_natural_matrices_with_dimensions_from_iterator_helper(
    height: usize,
    width: usize,
    bound: u32,
    expected_values: &[&str],
) {
    assert_eq!(
        (random_natural_matrices_with_dimensions_from_iterator(
            height,
            width,
            uniform_random_natural_inclusive_range(
                EXAMPLE_SEED,
                Natural::ZERO,
                Natural::from(bound)
            )
        ))
        .take(20)
        .map(|m| m.to_string())
        .collect_vec(),
        expected_values
    );
}

#[test]
fn test_random_natural_matrices_with_dimensions_from_iterator() {
    // 0-by-3, elements bounded by 8
    random_natural_matrices_with_dimensions_from_iterator_helper(
        0,
        3,
        8,
        &[
            "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}",
            "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}",
            "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}",
        ],
    );
    // 2-by-2, elements bounded by 1
    random_natural_matrices_with_dimensions_from_iterator_helper(
        2,
        2,
        1,
        &[
            "[1, 1; 1, 1]",
            "[1, 1; 0, 0]",
            "[0, 1; 0, 1]",
            "[0, 1; 0, 0]",
            "[1, 1; 1, 1]",
            "[1, 1; 0, 1]",
            "[0, 0; 0, 1]",
            "[1, 1; 1, 0]",
            "[1, 1; 1, 1]",
            "[1, 0; 0, 1]",
            "[0, 1; 0, 1]",
            "[1, 1; 1, 1]",
            "[1, 1; 1, 0]",
            "[0, 1; 1, 1]",
            "[0, 1; 1, 0]",
            "[1, 0; 0, 0]",
            "[1, 0; 1, 0]",
            "[1, 0; 1, 0]",
            "[1, 1; 1, 0]",
            "[0, 1; 0, 0]",
        ],
    );
    // 2-by-3, elements bounded by 8
    random_natural_matrices_with_dimensions_from_iterator_helper(
        2,
        3,
        8,
        &[
            "[1, 7, 5; 7, 2, 8]",
            "[2, 4, 6; 6, 1, 3]",
            "[7, 8, 1; 6, 6, 6]",
            "[7, 7, 5; 7, 7, 1]",
            "[7, 4, 8; 7, 7, 2]",
            "[5, 5, 1; 1, 3, 0]",
            "[3, 4, 1; 0, 1, 2]",
            "[1, 0, 7; 0, 5, 3]",
            "[7, 5, 6; 7, 3, 3]",
            "[4, 7, 0; 2, 2, 6]",
            "[2, 6, 6; 6, 8, 7]",
            "[3, 4, 1; 8, 4, 3]",
            "[8, 1, 8; 1, 0, 7]",
            "[1, 2, 6; 0, 5, 0]",
            "[1, 7, 8; 2, 6, 4]",
            "[0, 7, 6; 4, 4, 3]",
            "[7, 3, 5; 6, 7, 3]",
            "[3, 7, 2; 8, 3, 8]",
            "[3, 3, 0; 7, 4, 3]",
            "[4, 4, 1; 6, 3, 4]",
        ],
    );
    // 3-by-1, elements bounded by 100
    random_natural_matrices_with_dimensions_from_iterator_helper(
        3,
        1,
        100,
        &[
            "[87; 93; 7]",
            "[84; 27; 46]",
            "[93; 22; 86]",
            "[1; 7; 29]",
            "[25; 56; 97]",
            "[38; 70; 6]",
            "[55; 29; 12]",
            "[7; 87; 81]",
            "[23; 52; 88]",
            "[7; 60; 7]",
            "[50; 21; 43]",
            "[49; 15; 33]",
            "[91; 67; 58]",
            "[93; 99; 36]",
            "[27; 1; 16]",
            "[42; 82; 78]",
            "[65; 48; 92]",
            "[55; 80; 5]",
            "[14; 3; 71]",
            "[21; 12; 63]",
        ],
    );
}

#[test]
#[should_panic]
fn random_natural_matrices_with_dimensions_from_iterator_fail_1() {
    // height * width overflows
    let _ = random_natural_matrices_with_dimensions_from_iterator(
        usize::MAX,
        2,
        uniform_random_natural_inclusive_range(EXAMPLE_SEED, Natural::ZERO, Natural::from(8u32)),
    );
}
