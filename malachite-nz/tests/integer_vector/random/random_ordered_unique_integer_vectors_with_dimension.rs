// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::integer_vector::random::random_ordered_unique_integer_vectors_with_dimension;

#[track_caller]
fn random_ordered_unique_integer_vectors_with_dimension_helper(
    dimension: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        random_ordered_unique_integer_vectors_with_dimension(
            EXAMPLE_SEED,
            dimension,
            mean_bits_numerator,
            mean_bits_denominator
        )
        .take(20)
        .inspect(|v| assert!(v.elements.is_sorted_by(|x, y| x < y)))
        .map(|v| v.to_string())
        .collect_vec(),
        expected_values
    );
}

#[test]
fn test_random_ordered_unique_integer_vectors_with_dimension() {
    // dimension 0: only the 0-dimensional vector
    random_ordered_unique_integer_vectors_with_dimension_helper(
        0,
        2,
        1,
        &[
            "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()",
            "()", "()", "()", "()", "()", "()",
        ],
    );
    // dimension 1, mean bits = 4
    random_ordered_unique_integer_vectors_with_dimension_helper(
        1,
        4,
        1,
        &[
            "(2)",
            "(152)",
            "(1)",
            "(0)",
            "(-62)",
            "(5282)",
            "(0)",
            "(28)",
            "(4427344568)",
            "(79)",
            "(-11)",
            "(-7330)",
            "(-1)",
            "(-5667523)",
            "(1618)",
            "(120)",
            "(-1)",
            "(-1006)",
            "(-7)",
            "(-45)",
        ],
    );
    // dimension 3, mean bits = 8
    random_ordered_unique_integer_vectors_with_dimension_helper(
        3,
        8,
        1,
        &[
            "(2, 50, 3254)",
            "(-364028554193374, -13474, -188)",
            "(-4162639, -35, 4)",
            "(5, 18, 948931)",
            "(-184, -2, 0)",
            "(-104563169774, -1765048, -23)",
            "(-5, -2, 406)",
            "(-6895, -58, 1757)",
            "(9, 23, 436692)",
            "(3, 15, 26380)",
            "(-16, -2, 560627)",
            "(-2, 1, 22)",
            "(-93, 0, 1029)",
            "(-4, -2, 0)",
            "(-1, 0, 158)",
            "(-2, 102, 1809983)",
            "(0, 1, 412)",
            "(-1135, -3, 24)",
            "(-74535, 7, 10)",
            "(-494098, 171063, 243061)",
        ],
    );
    // dimension 5, mean bits = 3/2: small elements, so many repeats are redrawn
    random_ordered_unique_integer_vectors_with_dimension_helper(
        5,
        3,
        2,
        &[
            "(-24, -1, 0, 1, 12)",
            "(-19, -3, -1, 0, 1)",
            "(-1, 0, 2, 8, 238)",
            "(-6, -1, 0, 2, 3)",
            "(-15, -12, 0, 1, 2)",
            "(-3, -1, 0, 1, 3)",
            "(-5, -1, 0, 1, 1376)",
            "(-612, 0, 1, 2, 5)",
            "(-14, -1, 1, 4, 31)",
            "(-1, 0, 1, 2, 3)",
            "(-3, -1, 0, 1, 3)",
            "(-5, -4, 1, 2, 15)",
            "(-2, -1, 0, 1, 63)",
            "(-5, -1, 1, 3, 24)",
            "(-16, -1, 0, 1, 7)",
            "(-22, -3, -1, 0, 1)",
            "(-7, -2, -1, 0, 3)",
            "(-2, -1, 0, 5, 10)",
            "(-86, -2, 1, 2, 18)",
            "(-57, -14, -1, 0, 1)",
        ],
    );
}

#[test]
#[should_panic]
fn random_ordered_unique_integer_vectors_with_dimension_fail_1() {
    // mean_bits_denominator is zero
    let _ = random_ordered_unique_integer_vectors_with_dimension(EXAMPLE_SEED, 3, 1, 0);
}

#[test]
#[should_panic]
fn random_ordered_unique_integer_vectors_with_dimension_fail_2() {
    // mean_bits_numerator is zero
    let _ = random_ordered_unique_integer_vectors_with_dimension(EXAMPLE_SEED, 3, 0, 1);
}
