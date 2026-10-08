// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::basic::traits::{NegativeOne, One};
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::integer::Integer;
use malachite_nz::integer::random::uniform_random_integer_inclusive_range;
use malachite_nz::integer_vector::random::random_integer_vectors_from_iterator;

fn random_integer_vectors_from_iterator_helper(
    bound: i32,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        random_integer_vectors_from_iterator(
            EXAMPLE_SEED,
            &|seed| uniform_random_integer_inclusive_range(
                seed,
                Integer::from(-bound),
                Integer::from(bound)
            ),
            mean_length_numerator,
            mean_length_denominator
        )
        .take(20)
        .map(|v| v.to_string())
        .collect_vec(),
        expected_values
    );
}

#[test]
fn test_random_integer_vectors_from_iterator() {
    // elements in [-1, 1], mean dimension = 1
    random_integer_vectors_from_iterator_helper(
        1,
        1,
        1,
        &[
            "(0)",
            "(1, 1)",
            "(0, 1)",
            "(1, 0)",
            "(0)",
            "(1, 0)",
            "()",
            "()",
            "(0)",
            "()",
            "()",
            "()",
            "(-1)",
            "(1, 0, 0, -1)",
            "()",
            "(-1, 0)",
            "()",
            "(0, -1, 0, 1, 1)",
            "(1)",
            "(-1)",
        ],
    );
    // elements in [-8, 8], mean dimension = 2
    random_integer_vectors_from_iterator_helper(
        8,
        2,
        1,
        &[
            "(-2, 2, 5, -3, 2, 1)",
            "(8)",
            "(1, -5, 6, 7, -8, -5, 7, -4)",
            "(7)",
            "(6, -5, 8, 3, 4, -3, -1, 1, -7, 2, -6, 6, -2, 8)",
            "()",
            "(8, 0, -1, 7, -8)",
            "(-5, 2, -5, 6)",
            "(-4)",
            "()",
            "(-4, -7, 6, 4, 7, -2)",
            "(1, -3)",
            "()",
            "()",
            "(6)",
            "(-2, 3, 8)",
            "()",
            "(-2)",
            "()",
            "(-6)",
        ],
    );
    // elements in [-1000, 1000], mean dimension = 4
    random_integer_vectors_from_iterator_helper(
        1000,
        4,
        1,
        &[
            "()",
            "(-147, -194, -458, -531, -830, -630, -571, -675, 898, 713, -223, 328, -810, -783)",
            "(565, -340, 812, 47)",
            "(-631, -749, -639, 443)",
            "(-972)",
            "()",
            "(977, -698, 914, -522, -936)",
            "(-773, 263)",
            "(-772, -145, -527, 109)",
            "()",
            "(-50, -906, -825, -186, 563, -5)",
            "()",
            "()",
            "(-373, -728, -457, 739, -412, -355, 703, 853, -671, -305, 976)",
            "(-268, -7, -55, 937, 594, -937, -958, 90)",
            "()",
            "(-282, 41, 335)",
            "()",
            "(-626, 651, 884, 111, 333)",
            "(848, 254, 744, 872, -480, 959, -825, -819, 952)",
        ],
    );
}

#[test]
#[should_panic]
fn random_integer_vectors_from_iterator_fail_1() {
    // mean_length_denominator is zero
    let _ = random_integer_vectors_from_iterator(
        EXAMPLE_SEED,
        &|seed| uniform_random_integer_inclusive_range(seed, Integer::NEGATIVE_ONE, Integer::ONE),
        1,
        0,
    );
}

#[test]
#[should_panic]
fn random_integer_vectors_from_iterator_fail_2() {
    // mean_length_numerator is zero
    let _ = random_integer_vectors_from_iterator(
        EXAMPLE_SEED,
        &|seed| uniform_random_integer_inclusive_range(seed, Integer::NEGATIVE_ONE, Integer::ONE),
        0,
        1,
    );
}
