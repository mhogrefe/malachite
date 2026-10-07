// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::iterators::prefix_to_string;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::natural_vector::exhaustive::*;
use std::collections::HashSet;

#[test]
fn test_exhaustive_natural_vectors() {
    assert_eq!(
        prefix_to_string(exhaustive_natural_vectors(), 50),
        "[(), (0), (1), (0, 0), (2), (0, 1), (3), (1, 0), (0, 0, 0), (0, 0, 0, 0), (0, 0, 1), \
        (0, 0, 0, 1), (0, 1, 0), (0, 0, 1, 0), (0, 1, 1), (0, 0, 1, 1), (4), (1, 1), (5), (0, 2), \
        (6), (0, 3), (7), (1, 2), (1, 0, 0), (0, 1, 0, 0), (1, 0, 1), (0, 1, 0, 1), (1, 1, 0), \
        (0, 1, 1, 0), (1, 1, 1), (0, 1, 1, 1), (8), (1, 3), (9), (2, 0), (10), (2, 1), (11), \
        (3, 0), (0, 0, 2), (1, 0, 0, 0), (0, 0, 3), (1, 0, 0, 1), (0, 1, 2), (1, 0, 1, 0), \
        (0, 1, 3), (1, 0, 1, 1), (12), (3, 1), ...]"
    );
}

#[test]
fn test_exhaustive_natural_vectors_with_dimension() {
    // dimension 0: the 0-dimensional vector, and nothing else
    assert_eq!(
        exhaustive_natural_vectors_with_dimension(0)
            .map(|v| v.to_string())
            .collect_vec(),
        &["()"]
    );
    assert_eq!(
        prefix_to_string(exhaustive_natural_vectors_with_dimension(1), 20),
        "[(0), (1), (2), (3), (4), (5), (6), (7), (8), (9), (10), (11), (12), (13), (14), (15), \
        (16), (17), (18), (19), ...]"
    );
    assert_eq!(
        prefix_to_string(exhaustive_natural_vectors_with_dimension(2), 20),
        "[(0, 0), (0, 1), (1, 0), (1, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 0), (2, 1), (3, 0), \
        (3, 1), (2, 2), (2, 3), (3, 2), (3, 3), (0, 4), (0, 5), (1, 4), (1, 5), ...]"
    );
    assert_eq!(
        prefix_to_string(exhaustive_natural_vectors_with_dimension(3), 20),
        "[(0, 0, 0), (0, 0, 1), (0, 1, 0), (0, 1, 1), (1, 0, 0), (1, 0, 1), (1, 1, 0), (1, 1, 1), \
        (0, 0, 2), (0, 0, 3), (0, 1, 2), (0, 1, 3), (1, 0, 2), (1, 0, 3), (1, 1, 2), (1, 1, 3), \
        (0, 2, 0), (0, 2, 1), (0, 3, 0), (0, 3, 1), ...]"
    );
}

#[test]
fn exhaustive_natural_vectors_properties() {
    // No vector is generated twice.
    let vs = exhaustive_natural_vectors().take(10000).collect_vec();
    assert_eq!(vs.iter().collect::<HashSet<_>>().len(), vs.len());

    for dimension in 0..5 {
        let vs: Vec<NaturalVector> = exhaustive_natural_vectors_with_dimension(dimension)
            .take(1000)
            .collect();
        assert!(vs.iter().all(|v| v.dimension() == dimension));
        assert_eq!(vs.iter().collect::<HashSet<_>>().len(), vs.len());
    }
}
