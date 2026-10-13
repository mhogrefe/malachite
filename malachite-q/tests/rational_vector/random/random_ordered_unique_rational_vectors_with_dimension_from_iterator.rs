// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_q::Rational;
use malachite_q::rational::random::random_rational_range;
use malachite_q::rational_vector::random::*;

#[track_caller]
fn random_ordered_unique_rational_vectors_with_dimension_from_iterator_helper(
    dimension: u64,
    bound: i32,
    mean_bits: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        random_ordered_unique_rational_vectors_with_dimension_from_iterator(
            dimension,
            random_rational_range(
                EXAMPLE_SEED,
                -Rational::from(bound),
                Rational::from(bound),
                mean_bits,
                1,
                mean_bits,
                1
            )
        )
        .take(20)
        .inspect(|v| assert!(v.elements.is_sorted_by(|x, y| x < y)))
        .map(|v| v.to_string())
        .collect_vec(),
        expected_values
    );
}

#[test]
fn test_random_ordered_unique_rational_vectors_with_dimension_from_iterator() {
    // dimension 0: only the 0-dimensional vector
    random_ordered_unique_rational_vectors_with_dimension_from_iterator_helper(
        0,
        1,
        3,
        &[
            "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()",
            "()", "()", "()", "()", "()", "()",
        ],
    );
    // dimension 1, elements in [-1, 1), mean bits = 3
    random_ordered_unique_rational_vectors_with_dimension_from_iterator_helper(
        1,
        1,
        3,
        &[
            "(1/2)", "(0)", "(0)", "(-1/2)", "(4/11)", "(-1)", "(-1)", "(-1)", "(-1/2)", "(1/8)",
            "(-9/11)", "(-5/9)", "(1/2)", "(1/8)", "(-2/5)", "(-1)", "(-1/5)", "(11/15)", "(-1/3)",
            "(-1)",
        ],
    );
    // dimension 3, elements in [-1, 1), mean bits = 3
    random_ordered_unique_rational_vectors_with_dimension_from_iterator_helper(
        3,
        1,
        3,
        &[
            "(-1/2, 0, 1/2)",
            "(-1, -1/2, 4/11)",
            "(-9/11, -5/9, 1/8)",
            "(-2/5, 1/8, 1/2)",
            "(-1, -1/5, 11/15)",
            "(-1, -1/3, 1/3)",
            "(-9/10, 1/2, 3/5)",
            "(-1/2, -2/7, 1/5)",
            "(-1, -1/10, 1/2)",
            "(-1/2, -1/4, 3/8)",
            "(-1/3, -4/25, -1/8)",
            "(-1, 0, 1/3)",
            "(-1, 0, 1/6)",
            "(-1, 0, 1/2)",
            "(-5/7, -1/6, 3/8)",
            "(-2/3, 0, 4/5)",
            "(-4/5, 1/11, 1/2)",
            "(-1, -1/10, 1/3)",
            "(-1/2, -1/3, -1/9)",
            "(-1, -2/3, -1/3)",
        ],
    );
    // dimension 5, elements in [-1000, 1000), mean bits = 16
    random_ordered_unique_rational_vectors_with_dimension_from_iterator_helper(
        5,
        1000,
        16,
        &[
            "(-504/5, -107/12, -5/21, 1, 7/3)",
            "(-23/5, -29/28, 0, 11/12, 219/20)",
            "(-3931/10, -2/3, 2/7, 3/4, 82/51)",
            "(-1389/7, -15, -14/17, -5/8, 419/15)",
            "(3/55, 1/4, 32/29, 13/8, 1533/5)",
            "(-1, 1/62, 3/10, 47/67, 7/2)",
            "(-3, -44/23, 17/42, 9/8, 15/4)",
            "(-39/2, -5/9, -1/7, 1/6, 1/2)",
            "(1/24, 1/9, 1/8, 25/6, 58/3)",
            "(-7/4, 5/33, 1/5, 30/37, 1021/17)",
            "(-82/9, -25/84, 1/8, 4/3, 27/13)",
            "(-122, -2939/41, -11/36, 7/9, 229/2)",
            "(-130/17, -26/9, -3/50, 6/19, 8780/27)",
            "(-1652/11, 0, 25/2, 232/9, 1883/8)",
            "(-143/42, -1/15, 3/2, 57/10, 127/4)",
            "(-5654/7, 1/22, 1/13, 5/36, 25/7)",
            "(-1/33, 1/27, 49/41, 13/8, 69/10)",
            "(-41/12, 2, 69/10, 189/10, 14591/33)",
            "(-4794/5, -3, -1/3, 1/10, 101/14)",
            "(-85/6, -3/4, -3/16, 3116/25, 3587/4)",
        ],
    );
}
