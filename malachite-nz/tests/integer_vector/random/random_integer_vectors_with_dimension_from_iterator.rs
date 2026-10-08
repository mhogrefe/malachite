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
use malachite_nz::integer_vector::random::random_integer_vectors_with_dimension_from_iterator;

fn random_integer_vectors_with_dimension_from_iterator_helper(
    dimension: u64,
    bound: i32,
    expected_values: &[&str],
) {
    assert_eq!(
        random_integer_vectors_with_dimension_from_iterator(
            dimension,
            uniform_random_integer_inclusive_range(
                EXAMPLE_SEED,
                Integer::from(-bound),
                Integer::from(bound)
            )
        )
        .take(20)
        .map(|v| v.to_string())
        .collect_vec(),
        expected_values
    );
}

#[test]
fn test_random_integer_vectors_with_dimension_from_iterator() {
    // dimension 0: only the 0-dimensional vector
    random_integer_vectors_with_dimension_from_iterator_helper(
        0,
        1,
        &[
            "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()",
            "()", "()", "()", "()", "()", "()",
        ],
    );
    // dimension 1, elements in [-8, 8]
    random_integer_vectors_with_dimension_from_iterator_helper(
        1,
        8,
        &[
            "(-1)", "(1)", "(0)", "(3)", "(6)", "(3)", "(-7)", "(-1)", "(-7)", "(-2)", "(-2)",
            "(-2)", "(-1)", "(-3)", "(4)", "(-1)", "(7)", "(-1)", "(-1)", "(-3)",
        ],
    );
    // dimension 3, elements in [-1000, 1000]
    random_integer_vectors_with_dimension_from_iterator_helper(
        3,
        1000,
        &[
            "(905, -913, -907)",
            "(-371, 543, 1)",
            "(650, 384, -758)",
            "(515, -916, -845)",
            "(-58, 373, 46)",
            "(238, -253, -359)",
            "(139, -865, -715)",
            "(-847, -816, 62)",
            "(350, 158, 591)",
            "(-129, -771, -75)",
            "(932, 799, -913)",
            "(-633, -919, 47)",
            "(-820, 880, 287)",
            "(340, 799, -310)",
            "(-387, -723, -829)",
            "(-439, 39, 953)",
            "(-909, 475, 722)",
            "(-248, -267, -5)",
            "(-749, -708, -589)",
            "(-615, 808, -247)",
        ],
    );
}
