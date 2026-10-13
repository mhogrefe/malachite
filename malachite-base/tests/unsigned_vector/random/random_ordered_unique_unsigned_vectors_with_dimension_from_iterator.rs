// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::random::random_unsigned_inclusive_range;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::unsigned_vector::random::*;

#[track_caller]
fn random_ordered_unique_unsigned_vectors_with_dimension_from_iterator_helper<
    T: PrimitiveUnsigned,
>(
    dimension: u64,
    bound: T,
    expected_values: &[&str],
) {
    assert_eq!(
        random_ordered_unique_unsigned_vectors_with_dimension_from_iterator(
            dimension,
            random_unsigned_inclusive_range(EXAMPLE_SEED, T::ZERO, bound)
        )
        .take(20)
        .inspect(|v| assert!(v.elements.is_sorted_by(|x, y| x < y)))
        .map(|v| v.to_string())
        .collect_vec(),
        expected_values
    );
}

#[test]
fn test_random_ordered_unique_unsigned_vectors_with_dimension_from_iterator() {
    // dimension 0: only the 0-dimensional vector
    random_ordered_unique_unsigned_vectors_with_dimension_from_iterator_helper::<u8>(
        0,
        1,
        &[
            "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()",
            "()", "()", "()", "()", "()", "()",
        ],
    );
    // dimension 1, u8 elements in [0, 15]
    random_ordered_unique_unsigned_vectors_with_dimension_from_iterator_helper::<u8>(
        1,
        15,
        &[
            "(1)", "(7)", "(15)", "(14)", "(5)", "(4)", "(12)", "(6)", "(4)", "(14)", "(2)",
            "(13)", "(8)", "(10)", "(1)", "(10)", "(7)", "(5)", "(0)", "(2)",
        ],
    );
    // dimension 3, u8 elements in [0, 3]: many repeats are redrawn
    random_ordered_unique_unsigned_vectors_with_dimension_from_iterator_helper::<u8>(
        3,
        3,
        &[
            "(0, 1, 3)",
            "(1, 2, 3)",
            "(0, 1, 3)",
            "(0, 1, 3)",
            "(0, 1, 2)",
            "(1, 2, 3)",
            "(0, 1, 2)",
            "(0, 2, 3)",
            "(0, 1, 2)",
            "(1, 2, 3)",
            "(0, 1, 2)",
            "(0, 2, 3)",
            "(1, 2, 3)",
            "(0, 1, 3)",
            "(0, 2, 3)",
            "(0, 1, 2)",
            "(1, 2, 3)",
            "(1, 2, 3)",
            "(0, 1, 3)",
            "(0, 2, 3)",
        ],
    );
    // dimension 4, u8 elements in [0, 3]: every vector uses all of them
    random_ordered_unique_unsigned_vectors_with_dimension_from_iterator_helper::<u8>(
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
    // dimension 3, u64 elements in [0, 1000]
    random_ordered_unique_unsigned_vectors_with_dimension_from_iterator_helper::<u64>(
        3,
        1000,
        &[
            "(379, 708, 881)",
            "(106, 210, 913)",
            "(129, 378, 878)",
            "(356, 555, 788)",
            "(348, 494, 535)",
            "(445, 462, 613)",
            "(157, 471, 914)",
            "(372, 519, 678)",
            "(430, 456, 608)",
            "(144, 324, 978)",
            "(513, 791, 922)",
            "(18, 49, 882)",
            "(233, 697, 815)",
            "(509, 738, 827)",
            "(184, 346, 642)",
            "(578, 734, 740)",
            "(86, 940, 969)",
            "(317, 760, 935)",
            "(350, 751, 769)",
            "(7, 690, 917)",
        ],
    );
}
