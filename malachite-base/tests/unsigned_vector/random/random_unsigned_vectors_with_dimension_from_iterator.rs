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
use malachite_base::unsigned_vector::random::random_unsigned_vectors_with_dimension_from_iterator;

fn random_unsigned_vectors_with_dimension_from_iterator_helper<T: PrimitiveUnsigned>(
    dimension: u64,
    bound: T,
    expected_values: &[&str],
) {
    assert_eq!(
        random_unsigned_vectors_with_dimension_from_iterator(
            dimension,
            random_unsigned_inclusive_range(EXAMPLE_SEED, T::ZERO, bound)
        )
        .take(20)
        .map(|v| v.to_string())
        .collect_vec(),
        expected_values
    );
}

#[test]
fn test_random_unsigned_vectors_with_dimension_from_iterator() {
    // dimension 0: only the 0-dimensional vector
    random_unsigned_vectors_with_dimension_from_iterator_helper::<u8>(
        0,
        1,
        &[
            "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()",
            "()", "()", "()", "()", "()", "()",
        ],
    );
    // dimension 1, u8 elements in [0, 15]
    random_unsigned_vectors_with_dimension_from_iterator_helper::<u8>(
        1,
        15,
        &[
            "(1)", "(7)", "(15)", "(14)", "(5)", "(4)", "(12)", "(6)", "(4)", "(14)", "(2)",
            "(13)", "(8)", "(10)", "(1)", "(10)", "(7)", "(5)", "(0)", "(2)",
        ],
    );
    // dimension 3, u64 elements in [0, 1000]
    random_unsigned_vectors_with_dimension_from_iterator_helper::<u64>(
        3,
        1000,
        &[
            "(881, 379, 708)",
            "(913, 210, 106)",
            "(378, 129, 878)",
            "(788, 555, 356)",
            "(494, 535, 348)",
            "(462, 445, 613)",
            "(157, 471, 914)",
            "(519, 372, 678)",
            "(430, 456, 608)",
            "(324, 144, 978)",
            "(791, 513, 922)",
            "(882, 49, 18)",
            "(233, 815, 697)",
            "(827, 509, 738)",
            "(642, 184, 346)",
            "(578, 740, 734)",
            "(86, 969, 940)",
            "(760, 935, 317)",
            "(751, 350, 769)",
            "(7, 690, 917)",
        ],
    );
}
