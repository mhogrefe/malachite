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
use malachite_nz::natural_vector::random::random_natural_vectors_with_dimension_from_iterator;

fn random_natural_vectors_with_dimension_from_iterator_helper(
    dimension: u64,
    bound: u32,
    expected_values: &[&str],
) {
    assert_eq!(
        random_natural_vectors_with_dimension_from_iterator(
            dimension,
            uniform_random_natural_inclusive_range(
                EXAMPLE_SEED,
                Natural::ZERO,
                Natural::from(bound)
            )
        )
        .take(20)
        .map(|v| v.to_string())
        .collect_vec(),
        expected_values
    );
}

#[test]
fn test_random_natural_vectors_with_dimension_from_iterator() {
    // dimension 0: only the 0-dimensional vector
    random_natural_vectors_with_dimension_from_iterator_helper(
        0,
        1,
        &[
            "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()",
            "()", "()", "()", "()", "()", "()",
        ],
    );
    // dimension 1, elements in [0, 15]
    random_natural_vectors_with_dimension_from_iterator_helper(
        1,
        15,
        &[
            "(1)", "(7)", "(13)", "(5)", "(7)", "(9)", "(2)", "(8)", "(2)", "(11)", "(4)", "(11)",
            "(14)", "(13)", "(6)", "(6)", "(11)", "(1)", "(3)", "(7)",
        ],
    );
    // dimension 3, elements in [0, 1000]
    random_natural_vectors_with_dimension_from_iterator_helper(
        3,
        1000,
        &[
            "(881, 87, 93)",
            "(629, 519, 626)",
            "(360, 242, 491)",
            "(84, 155, 942)",
            "(349, 22, 214)",
            "(747, 641, 115)",
            "(135, 285, 153)",
            "(184, 993, 38)",
            "(326, 134, 567)",
            "(871, 229, 925)",
            "(908, 775, 87)",
            "(367, 81, 23)",
            "(180, 856, 263)",
            "(316, 775, 690)",
            "(613, 277, 171)",
            "(561, 15, 929)",
            "(91, 451, 698)",
            "(752, 733, 995)",
            "(251, 292, 411)",
            "(385, 784, 753)",
        ],
    );
}
