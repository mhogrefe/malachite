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
use malachite_nz::natural_matrix::random::random_natural_square_matrices_from_iterator;

fn random_natural_square_matrices_from_iterator_helper(
    bound: u32,
    mean_size_numerator: u64,
    mean_size_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        (random_natural_square_matrices_from_iterator(
            EXAMPLE_SEED,
            &|seed| uniform_random_natural_inclusive_range(
                seed,
                Natural::ZERO,
                Natural::from(bound)
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
fn test_random_natural_square_matrices_from_iterator() {
    // elements bounded by 1, mean size = 1/1
    random_natural_square_matrices_from_iterator_helper(
        1,
        1,
        1,
        &[
            "[]_{0,0}",
            "[]_{0,0}",
            "[]_{0,0}",
            "[0]",
            "[]_{0,0}",
            "[1, 0; 1, 1]",
            "[]_{0,0}",
            "[]_{0,0}",
            "[1]",
            "[1, 1, 1, 1, 1, 1; 1, 1, 0, 0, 0, 0; 0, 0, 1, 0, 1, 1; 0, 1, 1, 0, 1, 1; 1, 1, 0, 1, \
            1, 1; 0, 0, 0, 1, 0, 1]",
            "[0]",
            "[0, 1, 0, 0, 0; 0, 1, 1, 1, 0; 0, 0, 1, 1, 0; 0, 1, 0, 1, 1; 1, 0, 0, 1, 1]",
            "[1]",
            "[1]",
            "[1, 1; 1, 0]",
            "[0, 0; 0, 0]",
            "[1, 1, 1, 1; 1, 0, 1, 1; 0, 0, 0, 1; 0, 0, 0, 1]",
            "[0, 0; 0, 0]",
            "[1, 1, 1, 1; 1, 0, 0, 1; 1, 1, 1, 1; 1, 0, 1, 1]",
            "[]_{0,0}",
        ],
    );
    // elements bounded by 8, mean size = 1/1
    random_natural_square_matrices_from_iterator_helper(
        8,
        1,
        1,
        &[
            "[]_{0,0}",
            "[]_{0,0}",
            "[]_{0,0}",
            "[4]",
            "[]_{0,0}",
            "[0, 5; 5, 1]",
            "[]_{0,0}",
            "[]_{0,0}",
            "[8]",
            "[4, 4, 7, 2, 7, 4; 7, 1, 7, 0, 8, 7; 0, 6, 2, 7, 2, 0; 6, 3, 1, 6, 2, 8; 3, 3, 5, 2, \
            4, 3; 3, 7, 3, 7, 2, 8]",
            "[1]",
            "[5, 5, 1, 7, 8; 0, 2, 8, 5, 6; 6, 2, 1, 5, 4; 6, 3, 7, 8, 3; 1, 0, 3, 1, 1]",
            "[6]",
            "[2]",
            "[7, 1; 5, 0]",
            "[6, 4; 7, 8]",
            "[3, 4, 4, 5; 4, 1, 8, 4; 4, 5, 4, 0; 5, 7, 6, 3]",
            "[3, 1; 1, 8]",
            "[4, 7, 2, 7; 6, 3, 2, 3; 2, 4, 8, 5; 5, 5, 1, 0]",
            "[]_{0,0}",
        ],
    );
    // elements bounded by 100, mean size = 3/2
    random_natural_square_matrices_from_iterator_helper(
        100,
        3,
        2,
        &[
            "[]_{0,0}",
            "[]_{0,0}",
            "[36, 11, 96, 63, 91, 53; 29, 73, 21, 15, 65, 88; 60, 76, 20, 57, 55, 2; 55, 36, 7, \
            61, 42, 75; 64, 7, 80, 9, 54, 82; 82, 70, 10, 99, 65, 79]",
            "[]_{0,0}",
            "[22, 90; 98, 73]",
            "[29, 92; 73, 78]",
            "[19]",
            "[]_{0,0}",
            "[19, 53, 18; 36, 19, 3; 73, 73, 39]",
            "[]_{0,0}",
            "[3, 7, 82, 46, 78; 44, 24, 97, 37, 37; 97, 71, 13, 48, 18; 61, 44, 40, 42, 85; 92, \
            54, 54, 2, 81]",
            "[91, 5, 73, 57, 52, 86; 3, 11, 13, 23, 59, 88; 73, 51, 11, 62, 49, 61; 16, 19, 17, \
            79, 70, 26; 17, 91, 11, 64, 90, 78; 22, 92, 52, 23, 42, 44]",
            "[]_{0,0}",
            "[8]",
            "[45]",
            "[51]",
            "[]_{0,0}",
            "[36, 14, 37; 68, 81, 88; 29, 76, 4]",
            "[91, 73, 36, 61; 53, 89, 79, 4; 5, 90, 38, 63; 99, 14, 81, 97]",
            "[56, 36, 30; 55, 82, 74; 14, 39, 54]",
        ],
    );
}

#[test]
#[should_panic]
fn random_natural_square_matrices_from_iterator_fail_1() {
    // mean_size_denominator is zero
    let _ = random_natural_square_matrices_from_iterator(
        EXAMPLE_SEED,
        &|seed| uniform_random_natural_inclusive_range(seed, Natural::ZERO, Natural::from(8u32)),
        2,
        0,
    );
}

#[test]
#[should_panic]
fn random_natural_square_matrices_from_iterator_fail_2() {
    // mean_size_numerator is zero
    let _ = random_natural_square_matrices_from_iterator(
        EXAMPLE_SEED,
        &|seed| uniform_random_natural_inclusive_range(seed, Natural::ZERO, Natural::from(8u32)),
        0,
        1,
    );
}
