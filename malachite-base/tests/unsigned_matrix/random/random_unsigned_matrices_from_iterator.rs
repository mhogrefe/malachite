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
use malachite_base::unsigned_matrix::random::random_unsigned_matrices_from_iterator;

fn random_unsigned_matrices_from_iterator_helper(
    bound: u8,
    mean_dimension_numerator: u64,
    mean_dimension_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        (random_unsigned_matrices_from_iterator(
            EXAMPLE_SEED,
            &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, bound),
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
fn test_random_unsigned_matrices_from_iterator() {
    // elements up to 1, mean dimension = 1/1
    random_unsigned_matrices_from_iterator_helper(
        1,
        1,
        1,
        &[
            "[]_{0,0}",
            "[]_{0,1}",
            "[]_{0,2}",
            "[]_{0,0}",
            "[0, 0, 1, 0, 0, 1]",
            "[0, 1, 0, 0, 1]",
            "[1]",
            "[0, 0; 1, 0]",
            "[0, 1; 1, 1; 1, 0; 1, 0]",
            "[]_{4,0}",
            "[]_{1,0}",
            "[]_{1,0}",
            "[]_{0,0}",
            "[0, 1]",
            "[]_{2,0}",
            "[]_{3,0}",
            "[1; 1; 1]",
            "[]_{2,0}",
            "[]_{0,1}",
            "[1; 1]",
        ],
    );
    // elements up to 8, mean dimension = 2/1
    random_unsigned_matrices_from_iterator_helper(
        8,
        2,
        1,
        &[
            "[]_{0,1}",
            "[]_{0,11}",
            "[]_{2,0}",
            "[4, 4; 5, 7; 1, 6; 1, 1]",
            "[]_{6,0}",
            "[]_{0,2}",
            "[]_{1,0}",
            "[]_{0,3}",
            "[]_{6,0}",
            "[0]",
            "[1, 4, 3, 1; 1, 4, 4, 0]",
            "[]_{0,1}",
            "[]_{0,3}",
            "[]_{5,0}",
            "[]_{4,0}",
            "[]_{0,6}",
            "[6, 7]",
            "[]_{0,1}",
            "[]_{0,0}",
            "[]_{0,0}",
        ],
    );
    // elements up to 100, mean dimension = 2/1
    random_unsigned_matrices_from_iterator_helper(
        100,
        2,
        1,
        &[
            "[]_{0,1}",
            "[]_{0,11}",
            "[]_{2,0}",
            "[36, 25; 71, 67; 91, 87; 17, 22]",
            "[]_{6,0}",
            "[]_{0,2}",
            "[]_{1,0}",
            "[]_{0,3}",
            "[]_{6,0}",
            "[56]",
            "[15, 100, 39, 56; 94, 65, 22, 1]",
            "[]_{0,1}",
            "[]_{0,3}",
            "[]_{5,0}",
            "[]_{4,0}",
            "[]_{0,6}",
            "[83, 50]",
            "[]_{0,1}",
            "[]_{0,0}",
            "[]_{0,0}",
        ],
    );
}

#[test]
#[should_panic]
fn random_unsigned_matrices_from_iterator_fail_1() {
    // mean_dimension_denominator is zero
    let _ = random_unsigned_matrices_from_iterator(
        EXAMPLE_SEED,
        &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, 8),
        2,
        0,
    );
}

#[test]
#[should_panic]
fn random_unsigned_matrices_from_iterator_fail_2() {
    // mean_dimension_numerator is zero
    let _ = random_unsigned_matrices_from_iterator(
        EXAMPLE_SEED,
        &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, 8),
        0,
        1,
    );
}
