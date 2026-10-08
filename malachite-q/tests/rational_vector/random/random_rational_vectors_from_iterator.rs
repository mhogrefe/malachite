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
use malachite_q::Rational;
use malachite_q::rational::random::random_rational_range;
use malachite_q::rational_vector::random::random_rational_vectors_from_iterator;

fn random_rational_vectors_from_iterator_helper(
    a: i32,
    b: i32,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        random_rational_vectors_from_iterator(
            EXAMPLE_SEED,
            &|seed| random_rational_range(seed, Rational::from(a), Rational::from(b), 3, 1, 3, 1),
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
fn test_random_rational_vectors_from_iterator() {
    // elements in [0, 1), mean dimension = 1
    random_rational_vectors_from_iterator_helper(
        0,
        1,
        1,
        1,
        &[
            "(1/3)",
            "(3/4, 0)",
            "(1/6, 1/10)",
            "(1/7, 0)",
            "(2/9)",
            "(1/3, 1/6)",
            "()",
            "()",
            "(1/8)",
            "()",
            "()",
            "()",
            "(1/2)",
            "(1/2, 2/3, 0, 1/5)",
            "()",
            "(0, 6/7)",
            "()",
            "(1/10, 0, 1/2, 0, 0)",
            "(0)",
            "(1/3)",
        ],
    );
    // elements in [-1, 1), mean dimension = 2
    random_rational_vectors_from_iterator_helper(
        -1,
        1,
        2,
        1,
        &[
            "(1/3, -3/4, 0, -1/6, 1/10, 6/7)",
            "(0)",
            "(-5/9, 1/3, 5/6, 1/8, 1/2, 1/2, -1/3, -1)",
            "(-4/5)",
            "(0, -2/7, -1/10, -1, 1/2, 0, -1, 0, -1/3, 1/3, 0, 1/2, 0, -1/5)",
            "()",
            "(-1/2, 0, -1, -1/3, -1/8)",
            "(-1/5, -1, 0, 0)",
            "(-1)",
            "()",
            "(-1/10, -1/2, 1/2, 0, 0, 1/3)",
            "(0, -5/6)",
            "()",
            "()",
            "(-1/2)",
            "(-1/8, -1/4, -3/5)",
            "()",
            "(1/3)",
            "()",
            "(-1/4)",
        ],
    );
    // elements in [-100, 100), mean dimension = 4
    random_rational_vectors_from_iterator_helper(
        -100,
        100,
        4,
        1,
        &[
            "()",
            "(-5/3, 1/4, -1, -1/6, 1/10, 4/7, 7, -5/9, -1/3, -5/6, 1/8, -5/2, 1/2, 1/3)",
            "(-3, 13/5, 1, -3/7)",
            "(7/10, 0, -3/2, 1)",
            "(1)",
            "()",
            "(2, 1/3, 272/3, -6, 5/2)",
            "(40, 8/5)",
            "(-1/2, 1, -31, -38/3)",
            "()",
            "(-1/8, -7/5, -2, -52, 27, -7)",
            "()",
            "()",
            "(1/10, -5/2, -9/2, 13, -7, 1/3, 0, 1/6, 3/2, 7/8, 1/4)",
            "(21/5, 2/3, -1/4, 14/3, 1/13, 3/2, -1/5, -3/2)",
            "()",
            "(-1, 2, 1/2)",
            "()",
            "(-4/3, 0, 12, -73/7, -1/7)",
            "(1/3, -2/9, -1/5, 26/3, -1/4, 1/2, 1/4, 7/4, 1/2)",
        ],
    );
}

#[test]
#[should_panic]
fn random_rational_vectors_from_iterator_fail_1() {
    // mean_length_denominator is zero
    let _ = random_rational_vectors_from_iterator(
        EXAMPLE_SEED,
        &|seed| random_rational_range(seed, Rational::NEGATIVE_ONE, Rational::ONE, 3, 1, 3, 1),
        1,
        0,
    );
}

#[test]
#[should_panic]
fn random_rational_vectors_from_iterator_fail_2() {
    // mean_length_numerator is zero
    let _ = random_rational_vectors_from_iterator(
        EXAMPLE_SEED,
        &|seed| random_rational_range(seed, Rational::NEGATIVE_ONE, Rational::ONE, 3, 1, 3, 1),
        0,
        1,
    );
}
