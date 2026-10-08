// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::integer_vector::random::random_integer_vectors_with_dimension;

fn random_integer_vectors_with_dimension_helper(
    dimension: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        random_integer_vectors_with_dimension(
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
fn test_random_integer_vectors_with_dimension() {
    // dimension 0: only the 0-dimensional vector
    random_integer_vectors_with_dimension_helper(
        0,
        2,
        1,
        &[
            "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()",
            "()", "()", "()", "()", "()", "()",
        ],
    );
    // dimension 1, mean bits = 2
    random_integer_vectors_with_dimension_helper(
        1,
        2,
        1,
        &[
            "(1)", "(0)", "(-24)", "(18)", "(-6)", "(-18)", "(4)", "(2)", "(0)", "(-3)", "(-1)",
            "(-1)", "(-9)", "(-35)", "(-6)", "(-8)", "(0)", "(0)", "(1)", "(6)",
        ],
    );
    // dimension 3, mean bits = 4
    random_integer_vectors_with_dimension_helper(
        3,
        4,
        1,
        &[
            "(2, 152, 1)",
            "(0, -62, 5282)",
            "(0, 28, 4427344568)",
            "(79, -11, -7330)",
            "(-1, -5667523, 1618)",
            "(120, -1, -1006)",
            "(-7, -45, -1686)",
            "(-206, 3, -1)",
            "(-1, 2516, 775)",
            "(5, 95, 23)",
            "(140, 2, 0)",
            "(499, -4, 14)",
            "(0, 1, -8)",
            "(-5, 0, -3295133)",
            "(0, 1, 2)",
            "(-6, 37, -7)",
            "(-2, 0, 1)",
            "(2, -3, 7)",
            "(-1, 0, 1)",
            "(-11, 2, -15)",
        ],
    );
}

#[test]
#[should_panic]
fn random_integer_vectors_with_dimension_fail_1() {
    // mean_bits_denominator is zero
    let _ = random_integer_vectors_with_dimension(EXAMPLE_SEED, 3, 1, 0);
}

#[test]
#[should_panic]
fn random_integer_vectors_with_dimension_fail_2() {
    // mean_bits_numerator is zero
    let _ = random_integer_vectors_with_dimension(EXAMPLE_SEED, 3, 0, 1);
}
