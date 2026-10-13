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
use malachite_nz::integer_vector::random::*;

#[track_caller]
fn random_ordered_unique_integer_vectors_with_dimension_from_iterator_helper(
    dimension: u64,
    bound: u32,
    expected_values: &[&str],
) {
    assert_eq!(
        random_ordered_unique_integer_vectors_with_dimension_from_iterator(
            dimension,
            uniform_random_integer_inclusive_range(
                EXAMPLE_SEED,
                -Integer::from(bound),
                Integer::from(bound)
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
fn test_random_ordered_unique_integer_vectors_with_dimension_from_iterator() {
    // dimension 0: only the 0-dimensional vector
    random_ordered_unique_integer_vectors_with_dimension_from_iterator_helper(
        0,
        1,
        &[
            "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()",
            "()", "()", "()", "()", "()", "()",
        ],
    );
    // dimension 1, elements in [-8, 8]
    random_ordered_unique_integer_vectors_with_dimension_from_iterator_helper(
        1,
        8,
        &[
            "(-1)", "(1)", "(0)", "(3)", "(6)", "(3)", "(-7)", "(-1)", "(-7)", "(-2)", "(-2)",
            "(-2)", "(-1)", "(-3)", "(4)", "(-1)", "(7)", "(-1)", "(-1)", "(-3)",
        ],
    );
    // dimension 2, elements in [-1, 1]: many repeats are redrawn
    random_ordered_unique_integer_vectors_with_dimension_from_iterator_helper(
        2,
        1,
        &[
            "(0, 1)", "(-1, 1)", "(-1, 1)", "(0, 1)", "(0, 1)", "(-1, 0)", "(0, 1)", "(0, 1)",
            "(-1, 0)", "(-1, 0)", "(-1, 1)", "(0, 1)", "(-1, 0)", "(-1, 0)", "(-1, 0)", "(0, 1)",
            "(-1, 0)", "(0, 1)", "(-1, 1)", "(0, 1)",
        ],
    );
    // dimension 3, elements in [-1, 1]: every vector uses all of them
    random_ordered_unique_integer_vectors_with_dimension_from_iterator_helper(
        3,
        1,
        &[
            "(-1, 0, 1)",
            "(-1, 0, 1)",
            "(-1, 0, 1)",
            "(-1, 0, 1)",
            "(-1, 0, 1)",
            "(-1, 0, 1)",
            "(-1, 0, 1)",
            "(-1, 0, 1)",
            "(-1, 0, 1)",
            "(-1, 0, 1)",
            "(-1, 0, 1)",
            "(-1, 0, 1)",
            "(-1, 0, 1)",
            "(-1, 0, 1)",
            "(-1, 0, 1)",
            "(-1, 0, 1)",
            "(-1, 0, 1)",
            "(-1, 0, 1)",
            "(-1, 0, 1)",
            "(-1, 0, 1)",
        ],
    );
    // dimension 3, elements in [-1000, 1000]
    random_ordered_unique_integer_vectors_with_dimension_from_iterator_helper(
        3,
        1000,
        &[
            "(-913, -907, 905)",
            "(-371, 1, 543)",
            "(-758, 384, 650)",
            "(-916, -845, 515)",
            "(-58, 46, 373)",
            "(-359, -253, 238)",
            "(-865, -715, 139)",
            "(-847, -816, 62)",
            "(158, 350, 591)",
            "(-771, -129, -75)",
            "(-913, 799, 932)",
            "(-919, -633, 47)",
            "(-820, 287, 880)",
            "(-310, 340, 799)",
            "(-829, -723, -387)",
            "(-439, 39, 953)",
            "(-909, 475, 722)",
            "(-267, -248, -5)",
            "(-749, -708, -589)",
            "(-615, -247, 808)",
        ],
    );
}
