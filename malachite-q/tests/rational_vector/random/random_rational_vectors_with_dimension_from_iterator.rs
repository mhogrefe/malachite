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
use malachite_q::rational_vector::random::random_rational_vectors_with_dimension_from_iterator;

fn random_rational_vectors_with_dimension_from_iterator_helper(
    dimension: u64,
    a: i32,
    b: i32,
    expected_values: &[&str],
) {
    assert_eq!(
        random_rational_vectors_with_dimension_from_iterator(
            dimension,
            random_rational_range(
                EXAMPLE_SEED,
                Rational::from(a),
                Rational::from(b),
                3,
                1,
                3,
                1
            )
        )
        .take(20)
        .map(|v| v.to_string())
        .collect_vec(),
        expected_values
    );
}

#[test]
fn test_random_rational_vectors_with_dimension_from_iterator() {
    // dimension 0: only the 0-dimensional vector
    random_rational_vectors_with_dimension_from_iterator_helper(
        0,
        -1,
        1,
        &[
            "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()",
            "()", "()", "()", "()", "()", "()",
        ],
    );
    // dimension 1, elements in [-1, 1)
    random_rational_vectors_with_dimension_from_iterator_helper(
        1,
        -1,
        1,
        &[
            "(1/2)", "(0)", "(0)", "(-1/2)", "(4/11)", "(-1)", "(-1)", "(-1)", "(-1/2)", "(1/8)",
            "(-9/11)", "(-5/9)", "(1/2)", "(1/8)", "(-2/5)", "(-1)", "(-1/5)", "(11/15)", "(-1/3)",
            "(-1)",
        ],
    );
    // dimension 3, elements in [-100, 100)
    random_rational_vectors_with_dimension_from_iterator_helper(
        3,
        -100,
        100,
        &[
            "(99/2, 0, -1)",
            "(-1/2, 4/11, -7)",
            "(0, 5, 7/2)",
            "(753/8, 256/11, -5/9)",
            "(-1/2, -1/8, -2/5)",
            "(-7, -7/5, 19/15)",
            "(31/3, -6, -7/3)",
            "(15/2, -1/5, -11/10)",
            "(1/2, 54/7, -229/5)",
            "(-1/2, 58, -3/2)",
            "(-1/10, 259/4, 1/2)",
            "(-1/8, 44/25, 3/8)",
            "(218/3, 0, 1)",
            "(0, 0, 1/3)",
            "(1/6, 1, -10)",
            "(-1, -1, -15)",
            "(1/2, 49/8, -5/7)",
            "(-1/6, 1, 1)",
            "(1/3, 2/5, -67/5)",
            "(-45/11, -27/2, 223/3)",
        ],
    );
}
