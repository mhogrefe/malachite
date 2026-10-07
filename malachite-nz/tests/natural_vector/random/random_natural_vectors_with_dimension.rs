// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::natural_vector::random::random_natural_vectors_with_dimension;

fn random_natural_vectors_with_dimension_helper(
    dimension: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        random_natural_vectors_with_dimension(
            EXAMPLE_SEED,
            dimension,
            mean_bits_numerator,
            mean_bits_denominator
        )
        .take(20)
        .map(|v| v.to_string())
        .collect_vec(),
        expected_values
    );
}

#[test]
fn test_random_natural_vectors_with_dimension() {
    // dimension 0: only the 0-dimensional vector
    random_natural_vectors_with_dimension_helper(
        0,
        2,
        1,
        &[
            "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()",
            "()", "()", "()", "()", "()", "()",
        ],
    );
    // dimension 1, mean bits = 2
    random_natural_vectors_with_dimension_helper(
        1,
        2,
        1,
        &[
            "(54)", "(0)", "(0)", "(2)", "(2)", "(0)", "(2)", "(18)", "(4)", "(1)", "(1)", "(0)",
            "(3)", "(2)", "(9)", "(3)", "(1)", "(1)", "(1)", "(30)",
        ],
    );
    // dimension 3, mean bits = 4
    random_natural_vectors_with_dimension_helper(
        3,
        4,
        1,
        &[
            "(14, 8, 10)",
            "(1, 1, 0)",
            "(1, 184, 15)",
            "(99, 1, 3)",
            "(3, 6, 1)",
            "(4, 494, 3)",
            "(13, 1, 1)",
            "(1757, 1775, 3514)",
            "(2, 1799, 0)",
            "(5, 0, 7)",
            "(1, 2, 0)",
            "(4724, 3, 24928)",
            "(0, 1, 0)",
            "(0, 6, 1)",
            "(133, 3, 0)",
            "(6, 0, 0)",
            "(1, 6, 325)",
            "(7, 102, 2)",
            "(60, 59, 23)",
            "(776, 15, 3)",
        ],
    );
}

#[test]
#[should_panic]
fn random_natural_vectors_with_dimension_fail_1() {
    // mean_bits_denominator is zero
    let _ = random_natural_vectors_with_dimension(EXAMPLE_SEED, 3, 1, 0);
}

#[test]
#[should_panic]
fn random_natural_vectors_with_dimension_fail_2() {
    // mean_bits_numerator is zero
    let _ = random_natural_vectors_with_dimension(EXAMPLE_SEED, 3, 0, 1);
}
