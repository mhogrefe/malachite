// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_q::rational_vector::random::random_rational_vectors_with_dimension;

fn random_rational_vectors_with_dimension_helper(
    dimension: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        random_rational_vectors_with_dimension(
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
fn test_random_rational_vectors_with_dimension() {
    // dimension 0: only the 0-dimensional vector
    random_rational_vectors_with_dimension_helper(
        0,
        2,
        1,
        &[
            "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()",
            "()", "()", "()", "()", "()", "()",
        ],
    );
    // dimension 1, mean bits = 2
    random_rational_vectors_with_dimension_helper(
        1,
        2,
        1,
        &[
            "(-1)", "(-5/3)", "(0)", "(1)", "(0)", "(1/2)", "(356)", "(0)", "(0)", "(3/2)",
            "(3/5)", "(-14/3)", "(0)", "(-1/3)", "(-19/3)", "(-1/2)", "(0)", "(-1)", "(0)",
            "(-10)",
        ],
    );
    // dimension 3, mean bits = 4
    random_rational_vectors_with_dimension_helper(
        3,
        4,
        1,
        &[
            "(0, -1/3, 1)",
            "(4/323, 0, 7/12)",
            "(1/17, 7, 1/97)",
            "(15/19, 0, -79/21)",
            "(7/6, -9912911/5, 0)",
            "(-5/786, -986/97, -12/5)",
            "(0, 0, 0)",
            "(256/15, 0, -1)",
            "(-9/7, -1062790142, -2)",
            "(6/113, -184/31, 29)",
            "(56/20583, -717, 1625/619)",
            "(6, -5/66, 0)",
            "(0, 22/7, 1/4)",
            "(-1/3, 5/3, 7)",
            "(-3/5, -1/5, -16/115)",
            "(3, 3/46, 1/63)",
            "(-126, -1/61, 0)",
            "(0, -13, -15/22)",
            "(0, 37, -39/1328)",
            "(-1/2, 0, 11)",
        ],
    );
}

#[test]
#[should_panic]
fn random_rational_vectors_with_dimension_fail_1() {
    // mean_bits_denominator is zero
    let _ = random_rational_vectors_with_dimension(EXAMPLE_SEED, 3, 1, 0);
}

#[test]
#[should_panic]
fn random_rational_vectors_with_dimension_fail_2() {
    // mean_bits_numerator is zero
    let _ = random_rational_vectors_with_dimension(EXAMPLE_SEED, 3, 0, 1);
}

#[test]
#[should_panic]
fn random_rational_vectors_with_dimension_fail_3() {
    // mean_bits_numerator <= mean_bits_denominator
    let _ = random_rational_vectors_with_dimension(EXAMPLE_SEED, 3, 1, 1);
}
