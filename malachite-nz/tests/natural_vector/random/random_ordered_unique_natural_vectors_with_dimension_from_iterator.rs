// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::basic::traits::Zero;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::natural::Natural;
use malachite_nz::natural::random::uniform_random_natural_inclusive_range;
use malachite_nz::natural_vector::random::*;

#[track_caller]
fn random_ordered_unique_natural_vectors_with_dimension_from_iterator_helper(
    dimension: u64,
    bound: u32,
    expected_values: &[&str],
) {
    assert_eq!(
        random_ordered_unique_natural_vectors_with_dimension_from_iterator(
            dimension,
            uniform_random_natural_inclusive_range(
                EXAMPLE_SEED,
                Natural::ZERO,
                Natural::from(bound)
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
fn test_random_ordered_unique_natural_vectors_with_dimension_from_iterator() {
    // dimension 0: only the 0-dimensional vector
    random_ordered_unique_natural_vectors_with_dimension_from_iterator_helper(
        0,
        1,
        &[
            "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()",
            "()", "()", "()", "()", "()", "()",
        ],
    );
    // dimension 1, elements in [0, 15]
    random_ordered_unique_natural_vectors_with_dimension_from_iterator_helper(
        1,
        15,
        &[
            "(1)", "(7)", "(13)", "(5)", "(7)", "(9)", "(2)", "(8)", "(2)", "(11)", "(4)", "(11)",
            "(14)", "(13)", "(6)", "(6)", "(11)", "(1)", "(3)", "(7)",
        ],
    );
    // dimension 3, elements in [0, 3]: many repeats are redrawn
    random_ordered_unique_natural_vectors_with_dimension_from_iterator_helper(
        3,
        3,
        &[
            "(1, 2, 3)",
            "(0, 2, 3)",
            "(0, 2, 3)",
            "(1, 2, 3)",
            "(0, 1, 3)",
            "(1, 2, 3)",
            "(0, 1, 3)",
            "(0, 1, 3)",
            "(0, 2, 3)",
            "(1, 2, 3)",
            "(0, 1, 3)",
            "(0, 1, 3)",
            "(0, 1, 2)",
            "(0, 1, 2)",
            "(0, 1, 3)",
            "(1, 2, 3)",
            "(0, 2, 3)",
            "(1, 2, 3)",
            "(0, 2, 3)",
            "(0, 2, 3)",
        ],
    );
    // dimension 4, elements in [0, 3]: every vector uses all of them
    random_ordered_unique_natural_vectors_with_dimension_from_iterator_helper(
        4,
        3,
        &[
            "(0, 1, 2, 3)",
            "(0, 1, 2, 3)",
            "(0, 1, 2, 3)",
            "(0, 1, 2, 3)",
            "(0, 1, 2, 3)",
            "(0, 1, 2, 3)",
            "(0, 1, 2, 3)",
            "(0, 1, 2, 3)",
            "(0, 1, 2, 3)",
            "(0, 1, 2, 3)",
            "(0, 1, 2, 3)",
            "(0, 1, 2, 3)",
            "(0, 1, 2, 3)",
            "(0, 1, 2, 3)",
            "(0, 1, 2, 3)",
            "(0, 1, 2, 3)",
            "(0, 1, 2, 3)",
            "(0, 1, 2, 3)",
            "(0, 1, 2, 3)",
            "(0, 1, 2, 3)",
        ],
    );
    // dimension 3, elements in [0, 1000]
    random_ordered_unique_natural_vectors_with_dimension_from_iterator_helper(
        3,
        1000,
        &[
            "(87, 93, 881)",
            "(519, 626, 629)",
            "(242, 360, 491)",
            "(84, 155, 942)",
            "(22, 214, 349)",
            "(115, 641, 747)",
            "(135, 153, 285)",
            "(38, 184, 993)",
            "(134, 326, 567)",
            "(229, 871, 925)",
            "(87, 775, 908)",
            "(23, 81, 367)",
            "(180, 263, 856)",
            "(316, 690, 775)",
            "(171, 277, 613)",
            "(15, 561, 929)",
            "(91, 451, 698)",
            "(733, 752, 995)",
            "(251, 292, 411)",
            "(385, 753, 784)",
        ],
    );
}
