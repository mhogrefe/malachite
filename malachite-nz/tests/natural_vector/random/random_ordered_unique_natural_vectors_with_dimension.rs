// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::natural_vector::random::random_ordered_unique_natural_vectors_with_dimension;

#[track_caller]
fn random_ordered_unique_natural_vectors_with_dimension_helper(
    dimension: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        random_ordered_unique_natural_vectors_with_dimension(
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
fn test_random_ordered_unique_natural_vectors_with_dimension() {
    // dimension 0: only the 0-dimensional vector
    random_ordered_unique_natural_vectors_with_dimension_helper(
        0,
        2,
        1,
        &[
            "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()",
            "()", "()", "()", "()", "()", "()",
        ],
    );
    // dimension 1, mean bits = 4
    random_ordered_unique_natural_vectors_with_dimension_helper(
        1,
        4,
        1,
        &[
            "(14)", "(8)", "(10)", "(1)", "(1)", "(0)", "(1)", "(184)", "(15)", "(99)", "(1)",
            "(3)", "(3)", "(6)", "(1)", "(4)", "(494)", "(3)", "(13)", "(1)",
        ],
    );
    // dimension 3, mean bits = 8
    random_ordered_unique_natural_vectors_with_dimension_helper(
        3,
        8,
        1,
        &[
            "(1, 4, 178)",
            "(2, 6, 279521758)",
            "(24, 163, 335)",
            "(1, 19, 103455906)",
            "(0, 1, 298808)",
            "(0, 4, 6)",
            "(1, 11239, 7361485119992205)",
            "(2, 477, 6895)",
            "(775, 980, 73078038663697850)",
            "(1, 3, 58882969)",
            "(0, 2, 780)",
            "(6, 57696, 1126206963)",
            "(2, 30, 603141)",
            "(0, 93, 1134718)",
            "(13, 14, 100)",
            "(0, 14, 15)",
            "(3, 6, 234396)",
            "(72, 1135, 22007)",
            "(0, 33594562, 4036928707)",
            "(2855, 2066962, 5935969653)",
        ],
    );
    // dimension 5, mean bits = 3/2: small elements, so many repeats are redrawn
    random_ordered_unique_natural_vectors_with_dimension_helper(
        5,
        3,
        2,
        &[
            "(0, 1, 2, 8, 14)",
            "(0, 1, 2, 3, 24)",
            "(1, 2, 3, 6, 13)",
            "(0, 1, 2, 15, 87)",
            "(0, 1, 7, 14, 20)",
            "(0, 2, 3, 14, 1029)",
            "(0, 1, 2, 37, 62)",
            "(0, 1, 4, 35, 59)",
            "(0, 1, 6, 7, 10)",
            "(0, 1, 3, 13, 113)",
            "(0, 4, 11, 28, 55)",
            "(0, 2, 3, 4, 63)",
            "(0, 2, 3, 5, 169)",
            "(0, 3, 6, 15, 95)",
            "(0, 1, 2, 3, 16)",
            "(0, 4, 7, 9, 15)",
            "(0, 1, 2, 3, 49)",
            "(0, 8, 14, 15, 52)",
            "(0, 1, 2, 30, 40)",
            "(0, 2, 3, 86, 343)",
        ],
    );
}

#[test]
#[should_panic]
fn random_ordered_unique_natural_vectors_with_dimension_fail_1() {
    // mean_bits_denominator is zero
    let _ = random_ordered_unique_natural_vectors_with_dimension(EXAMPLE_SEED, 3, 1, 0);
}

#[test]
#[should_panic]
fn random_ordered_unique_natural_vectors_with_dimension_fail_2() {
    // mean_bits_numerator is zero
    let _ = random_ordered_unique_natural_vectors_with_dimension(EXAMPLE_SEED, 3, 0, 1);
}
