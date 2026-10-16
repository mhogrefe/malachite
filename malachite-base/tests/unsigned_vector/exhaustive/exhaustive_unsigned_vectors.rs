// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::iterators::prefix_to_string;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::exhaustive::primitive_int_increasing_range;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::unsigned_vector::exhaustive::*;
use malachite_base::vector::Vector;
use std::collections::HashSet;

fn strings<T: PrimitiveUnsigned, I: Iterator<Item = UnsignedVector<T>>>(
    xs: I,
    n: usize,
) -> Vec<String> {
    xs.take(n).map(|v| v.to_string()).collect()
}

#[test]
fn test_exhaustive_unsigned_vectors() {
    assert_eq!(
        strings(exhaustive_unsigned_vectors::<u8>(), 50),
        &[
            "()",
            "(0)",
            "(1)",
            "(0, 0, 0)",
            "(2)",
            "(0, 0)",
            "(3)",
            "(0, 0, 0, 0)",
            "(4)",
            "(0, 1)",
            "(5)",
            "(0, 0, 1)",
            "(6)",
            "(1, 0)",
            "(7)",
            "(0, 0, 0, 0, 0)",
            "(8)",
            "(1, 1)",
            "(9)",
            "(0, 1, 0)",
            "(10)",
            "(0, 2)",
            "(11)",
            "(0, 0, 0, 1)",
            "(12)",
            "(0, 3)",
            "(13)",
            "(0, 1, 1)",
            "(14)",
            "(1, 2)",
            "(15)",
            "(0, 0, 0, 0, 0, 0)",
            "(16)",
            "(1, 3)",
            "(17)",
            "(1, 0, 0)",
            "(18)",
            "(2, 0)",
            "(19)",
            "(0, 0, 1, 0)",
            "(20)",
            "(2, 1)",
            "(21)",
            "(1, 0, 1)",
            "(22)",
            "(3, 0)",
            "(23)",
            "(0, 0, 0, 0, 1)",
            "(24)",
            "(3, 1)",
        ]
    );
}

#[test]
fn test_exhaustive_unsigned_vectors_with_dimension() {
    assert_eq!(
        strings(exhaustive_unsigned_vectors_with_dimension::<u8>(0), 10),
        &["()"]
    );
    assert_eq!(
        strings(exhaustive_unsigned_vectors_with_dimension::<u8>(1), 20),
        &[
            "(0)", "(1)", "(2)", "(3)", "(4)", "(5)", "(6)", "(7)", "(8)", "(9)", "(10)", "(11)",
            "(12)", "(13)", "(14)", "(15)", "(16)", "(17)", "(18)", "(19)",
        ]
    );
    assert_eq!(
        strings(exhaustive_unsigned_vectors_with_dimension::<u8>(2), 20),
        &[
            "(0, 0)", "(0, 1)", "(1, 0)", "(1, 1)", "(0, 2)", "(0, 3)", "(1, 2)", "(1, 3)",
            "(2, 0)", "(2, 1)", "(3, 0)", "(3, 1)", "(2, 2)", "(2, 3)", "(3, 2)", "(3, 3)",
            "(0, 4)", "(0, 5)", "(1, 4)", "(1, 5)",
        ]
    );
    assert_eq!(
        strings(exhaustive_unsigned_vectors_with_dimension::<u8>(3), 20),
        &[
            "(0, 0, 0)",
            "(0, 0, 1)",
            "(0, 1, 0)",
            "(0, 1, 1)",
            "(1, 0, 0)",
            "(1, 0, 1)",
            "(1, 1, 0)",
            "(1, 1, 1)",
            "(0, 0, 2)",
            "(0, 0, 3)",
            "(0, 1, 2)",
            "(0, 1, 3)",
            "(1, 0, 2)",
            "(1, 0, 3)",
            "(1, 1, 2)",
            "(1, 1, 3)",
            "(0, 2, 0)",
            "(0, 2, 1)",
            "(0, 3, 0)",
            "(0, 3, 1)",
        ]
    );
    // u128 elements start the same way.
    assert_eq!(
        strings(exhaustive_unsigned_vectors_with_dimension::<u128>(2), 4),
        &["(0, 0)", "(0, 1)", "(1, 0)", "(1, 1)"]
    );
}

#[test]
fn exhaustive_unsigned_vectors_properties() {
    let vs = exhaustive_unsigned_vectors::<u8>()
        .take(10000)
        .collect_vec();
    assert_eq!(vs.iter().collect::<HashSet<_>>().len(), vs.len());
    for dimension in 0..5 {
        let vs = exhaustive_unsigned_vectors_with_dimension::<u8>(dimension)
            .take(1000)
            .collect_vec();
        assert!(vs.iter().all(|v| v.dimension() == dimension));
        assert_eq!(vs.iter().collect::<HashSet<_>>().len(), vs.len());
    }
}

#[test]
fn test_exhaustive_ordered_unique_unsigned_vectors_with_dimension() {
    // dimension 0: the 0-dimensional vector, and nothing else
    assert_eq!(
        strings(
            exhaustive_ordered_unique_unsigned_vectors_with_dimension::<u8>(0),
            10
        ),
        &["()"]
    );
    assert_eq!(
        strings(
            exhaustive_ordered_unique_unsigned_vectors_with_dimension::<u8>(1),
            20
        ),
        &[
            "(0)", "(1)", "(2)", "(3)", "(4)", "(5)", "(6)", "(7)", "(8)", "(9)", "(10)", "(11)",
            "(12)", "(13)", "(14)", "(15)", "(16)", "(17)", "(18)", "(19)",
        ]
    );
    assert_eq!(
        strings(
            exhaustive_ordered_unique_unsigned_vectors_with_dimension::<u8>(2),
            20
        ),
        &[
            "(0, 1)", "(0, 2)", "(1, 2)", "(0, 3)", "(1, 3)", "(2, 3)", "(0, 4)", "(1, 4)",
            "(2, 4)", "(3, 4)", "(0, 5)", "(1, 5)", "(2, 5)", "(3, 5)", "(4, 5)", "(0, 6)",
            "(1, 6)", "(2, 6)", "(3, 6)", "(4, 6)",
        ]
    );
    assert_eq!(
        strings(
            exhaustive_ordered_unique_unsigned_vectors_with_dimension::<u64>(3),
            20
        ),
        &[
            "(0, 1, 2)",
            "(0, 1, 3)",
            "(0, 2, 3)",
            "(1, 2, 3)",
            "(0, 1, 4)",
            "(0, 2, 4)",
            "(1, 2, 4)",
            "(0, 3, 4)",
            "(1, 3, 4)",
            "(2, 3, 4)",
            "(0, 1, 5)",
            "(0, 2, 5)",
            "(1, 2, 5)",
            "(0, 3, 5)",
            "(1, 3, 5)",
            "(2, 3, 5)",
            "(0, 4, 5)",
            "(1, 4, 5)",
            "(2, 4, 5)",
            "(3, 4, 5)",
        ]
    );
    // dimension 256: the vector of all u8s, and nothing else
    let vs = exhaustive_ordered_unique_unsigned_vectors_with_dimension::<u8>(256).collect_vec();
    assert_eq!(vs.len(), 1);
    assert_eq!(vs[0].elements, (0..=u8::MAX).collect_vec());
    // dimension 257: no vector
    assert_eq!(
        exhaustive_ordered_unique_unsigned_vectors_with_dimension::<u8>(257).count(),
        0
    );
}

#[test]
fn exhaustive_ordered_unique_unsigned_vectors_with_dimension_properties() {
    for dimension in 0..5 {
        let vs = exhaustive_ordered_unique_unsigned_vectors_with_dimension::<u8>(dimension)
            .take(1000)
            .collect_vec();
        assert!(vs.iter().all(|v| v.dimension() == dimension));
        assert!(vs.iter().all(|v| v.elements.is_sorted_by(|x, y| x < y)));
        assert_eq!(vs.iter().collect::<HashSet<_>>().len(), vs.len());
    }
    // Every 2-subset of the u8s is generated once.
    assert_eq!(
        exhaustive_ordered_unique_unsigned_vectors_with_dimension::<u8>(2).count(),
        256 * 255 / 2
    );
}

fn small_elements() -> impl Clone + Iterator<Item = u8> {
    primitive_int_increasing_range(0u8, 3)
}

#[test]
fn test_exhaustive_unsigned_vectors_from_iterator() {
    assert_eq!(
        prefix_to_string(
            exhaustive_unsigned_vectors_from_iterator(small_elements()),
            50
        ),
        "[(), (0), (1), (0, 0, 0), (2), (0, 0), (0, 1), (0, 0, 0, 0, 0), (1, 0), (0, 0, 1), (1, \
        1), (0, 0, 0, 0), (0, 2), (0, 1, 0), (1, 2), (0, 0, 0, 0, 0, 0), (2, 0), (0, 1, 1), (2, \
        1), (0, 0, 0, 1), (2, 2), (1, 0, 0), (1, 0, 1), (0, 0, 0, 0, 0, 1), (1, 1, 0), (0, 0, 1, \
        0), (1, 1, 1), (0, 0, 0, 0, 1), (0, 0, 2), (0, 0, 1, 1), (0, 1, 2), (0, 0, 0, 0, 0, 0, 0, \
        0), (1, 0, 2), (0, 1, 0, 0), (1, 1, 2), (0, 0, 0, 1, 0), (0, 2, 0), (0, 1, 0, 1), (0, 2, \
        1), (0, 0, 0, 0, 1, 0), (1, 2, 0), (0, 1, 1, 0), (1, 2, 1), (0, 0, 0, 1, 1), (0, 2, 2), \
        (0, 1, 1, 1), (1, 2, 2), (0, 0, 0, 0, 0, 0, 0), (2, 0, 0), (1, 0, 0, 0), ...]"
    );
    // - no elements: the 0-dimensional vector, and nothing else
    assert_eq!(
        exhaustive_unsigned_vectors_from_iterator(primitive_int_increasing_range(0u8, 0))
            .take(10)
            .map(|v| v.to_string())
            .collect_vec(),
        &["()"]
    );
}

#[test]
fn test_exhaustive_unsigned_vectors_with_dimension_from_iterator() {
    // dimension 0: the 0-dimensional vector, and nothing else
    assert_eq!(
        exhaustive_unsigned_vectors_with_dimension_from_iterator(0, small_elements())
            .map(|v| v.to_string())
            .collect_vec(),
        &["()"]
    );
    assert_eq!(
        prefix_to_string(
            exhaustive_unsigned_vectors_with_dimension_from_iterator(1, small_elements()),
            20
        ),
        "[(0), (1), (2)]"
    );
    assert_eq!(
        prefix_to_string(
            exhaustive_unsigned_vectors_with_dimension_from_iterator(2, small_elements()),
            20
        ),
        "[(0, 0), (0, 1), (1, 0), (1, 1), (0, 2), (1, 2), (2, 0), (2, 1), (2, 2)]"
    );
    // - no elements and a positive dimension: nothing
    assert_eq!(
        exhaustive_unsigned_vectors_with_dimension_from_iterator(
            2,
            primitive_int_increasing_range(0u8, 0)
        )
        .count(),
        0
    );
}

#[test]
fn exhaustive_unsigned_vectors_from_iterator_properties() {
    // Every element is from the set, and no vector is generated twice.
    let vs = exhaustive_unsigned_vectors_from_iterator(small_elements())
        .take(10000)
        .collect_vec();
    assert!(vs.iter().all(|v| v.elements.iter().all(|x| *x < 3)));
    assert_eq!(vs.iter().collect::<HashSet<_>>().len(), vs.len());

    // With 3 possible elements, there are 3^n vectors of each dimension n.
    for dimension in 0..6 {
        let vs =
            exhaustive_unsigned_vectors_with_dimension_from_iterator(dimension, small_elements())
                .collect_vec();
        assert_eq!(vs.len(), 3usize.pow(u32::try_from(dimension).unwrap()));
        assert!(vs.iter().all(|v| v.dimension() == dimension));
        assert_eq!(vs.iter().collect::<HashSet<_>>().len(), vs.len());
    }
}
