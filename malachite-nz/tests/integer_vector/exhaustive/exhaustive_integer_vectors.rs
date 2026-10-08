// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::iterators::prefix_to_string;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::integer_vector::exhaustive::*;
use std::collections::HashSet;

#[test]
fn test_exhaustive_integer_vectors() {
    assert_eq!(
        prefix_to_string(exhaustive_integer_vectors(), 50),
        "[(), (0), (1), (0, 0), (-1), (0, 1), (2), (1, 0), (0, 0, 0), (0, 0, 0, 0), (0, 0, 1), \
        (0, 0, 0, 1), (0, 1, 0), (0, 0, 1, 0), (0, 1, 1), (0, 0, 1, 1), (-2), (1, 1), (3), \
        (0, -1), (-3), (0, 2), (4), (1, -1), (1, 0, 0), (0, 1, 0, 0), (1, 0, 1), (0, 1, 0, 1), \
        (1, 1, 0), (0, 1, 1, 0), (1, 1, 1), (0, 1, 1, 1), (-4), (1, 2), (5), (-1, 0), (-5), \
        (-1, 1), (6), (2, 0), (0, 0, -1), (1, 0, 0, 0), (0, 0, 2), (1, 0, 0, 1), (0, 1, -1), \
        (1, 0, 1, 0), (0, 1, 2), (1, 0, 1, 1), (-6), (2, 1), ...]"
    );
}

#[test]
fn test_exhaustive_integer_vectors_with_dimension() {
    // dimension 0: the 0-dimensional vector, and nothing else
    assert_eq!(
        exhaustive_integer_vectors_with_dimension(0)
            .map(|v| v.to_string())
            .collect_vec(),
        &["()"]
    );
    assert_eq!(
        prefix_to_string(exhaustive_integer_vectors_with_dimension(1), 20),
        "[(0), (1), (-1), (2), (-2), (3), (-3), (4), (-4), (5), (-5), (6), (-6), (7), (-7), (8), \
        (-8), (9), (-9), (10), ...]"
    );
    assert_eq!(
        prefix_to_string(exhaustive_integer_vectors_with_dimension(2), 20),
        "[(0, 0), (0, 1), (1, 0), (1, 1), (0, -1), (0, 2), (1, -1), (1, 2), (-1, 0), (-1, 1), \
        (2, 0), (2, 1), (-1, -1), (-1, 2), (2, -1), (2, 2), (0, -2), (0, 3), (1, -2), \
        (1, 3), ...]"
    );
    assert_eq!(
        prefix_to_string(exhaustive_integer_vectors_with_dimension(3), 20),
        "[(0, 0, 0), (0, 0, 1), (0, 1, 0), (0, 1, 1), (1, 0, 0), (1, 0, 1), (1, 1, 0), (1, 1, 1), \
        (0, 0, -1), (0, 0, 2), (0, 1, -1), (0, 1, 2), (1, 0, -1), (1, 0, 2), (1, 1, -1), \
        (1, 1, 2), (0, -1, 0), (0, -1, 1), (0, 2, 0), (0, 2, 1), ...]"
    );
}

#[test]
fn exhaustive_integer_vectors_properties() {
    // No vector is generated twice.
    let vs = exhaustive_integer_vectors().take(10000).collect_vec();
    assert_eq!(vs.iter().collect::<HashSet<_>>().len(), vs.len());

    for dimension in 0..5 {
        let vs: Vec<IntegerVector> = exhaustive_integer_vectors_with_dimension(dimension)
            .take(1000)
            .collect();
        assert!(vs.iter().all(|v| v.dimension() == dimension));
        assert_eq!(vs.iter().collect::<HashSet<_>>().len(), vs.len());
    }
}
