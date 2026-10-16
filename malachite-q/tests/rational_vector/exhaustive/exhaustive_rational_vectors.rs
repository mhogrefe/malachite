// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::iterators::prefix_to_string;
use malachite_base::num::basic::traits::{OneHalf, Zero};
use malachite_base::vector::Vector;
use malachite_q::Rational;
use malachite_q::rational_vector::RationalVector;
use malachite_q::rational_vector::exhaustive::*;
use std::collections::HashSet;

#[test]
fn test_exhaustive_rational_vectors() {
    assert_eq!(
        prefix_to_string(exhaustive_rational_vectors(), 50),
        "[(), (0), (1), (0, 0, 0), (-1), (0, 0), (1/2), (0, 0, 0, 0), (-1/2), (0, 1), (2), (0, 0, \
        1), (-2), (1, 0), (1/3), (0, 0, 0, 0, 0), (-1/3), (1, 1), (3/2), (0, 1, 0), (-3/2), (0, \
        -1), (2/3), (0, 0, 0, 1), (-2/3), (0, 1/2), (3), (0, 1, 1), (-3), (1, -1), (1/4), (0, 0, \
        0, 0, 0, 0), (-1/4), (1, 1/2), (4/3), (1, 0, 0), (-4/3), (-1, 0), (3/5), (0, 0, 1, 0), \
        (-3/5), (-1, 1), (5/2), (1, 0, 1), (-5/2), (1/2, 0), (2/5), (0, 0, 0, 0, 1), (-2/5), (1/2, \
        1), ...]"
    );
}

#[test]
fn test_exhaustive_rational_vectors_with_dimension() {
    // dimension 0: the 0-dimensional vector, and nothing else
    assert_eq!(
        exhaustive_rational_vectors_with_dimension(0)
            .map(|v| v.to_string())
            .collect_vec(),
        &["()"]
    );
    assert_eq!(
        prefix_to_string(exhaustive_rational_vectors_with_dimension(1), 20),
        "[(0), (1), (-1), (1/2), (-1/2), (2), (-2), (1/3), (-1/3), (3/2), (-3/2), (2/3), (-2/3), \
        (3), (-3), (1/4), (-1/4), (4/3), (-4/3), (3/5), ...]"
    );
    assert_eq!(
        prefix_to_string(exhaustive_rational_vectors_with_dimension(2), 20),
        "[(0, 0), (0, 1), (1, 0), (1, 1), (0, -1), (0, 1/2), (1, -1), (1, 1/2), (-1, 0), (-1, 1), \
        (1/2, 0), (1/2, 1), (-1, -1), (-1, 1/2), (1/2, -1), (1/2, 1/2), (0, -1/2), (0, 2), (1, \
        -1/2), (1, 2), ...]"
    );
    assert_eq!(
        prefix_to_string(exhaustive_rational_vectors_with_dimension(3), 20),
        "[(0, 0, 0), (0, 0, 1), (0, 1, 0), (0, 1, 1), (1, 0, 0), (1, 0, 1), (1, 1, 0), (1, 1, 1), \
        (0, 0, -1), (0, 0, 1/2), (0, 1, -1), (0, 1, 1/2), (1, 0, -1), (1, 0, 1/2), (1, 1, -1), (1, \
        1, 1/2), (0, -1, 0), (0, -1, 1), (0, 1/2, 0), (0, 1/2, 1), ...]"
    );
}

#[test]
fn exhaustive_rational_vectors_properties() {
    // No vector is generated twice.
    let vs = exhaustive_rational_vectors().take(10000).collect_vec();
    assert_eq!(vs.iter().collect::<HashSet<_>>().len(), vs.len());

    for dimension in 0..5 {
        let vs: Vec<RationalVector> = exhaustive_rational_vectors_with_dimension(dimension)
            .take(1000)
            .collect();
        assert!(vs.iter().all(|v| v.dimension() == dimension));
        assert_eq!(vs.iter().collect::<HashSet<_>>().len(), vs.len());
    }
}

#[test]
fn test_exhaustive_ordered_unique_rational_vectors_with_dimension() {
    // dimension 0: the 0-dimensional vector, and nothing else
    assert_eq!(
        exhaustive_ordered_unique_rational_vectors_with_dimension(0)
            .map(|v| v.to_string())
            .collect_vec(),
        &["()"]
    );
    assert_eq!(
        prefix_to_string(
            exhaustive_ordered_unique_rational_vectors_with_dimension(1),
            20
        ),
        "[(0), (1), (-1), (1/2), (-1/2), (2), (-2), (1/3), (-1/3), (3/2), (-3/2), (2/3), (-2/3), \
        (3), (-3), (1/4), (-1/4), (4/3), (-4/3), (3/5), ...]"
    );
    assert_eq!(
        prefix_to_string(
            exhaustive_ordered_unique_rational_vectors_with_dimension(2),
            20
        ),
        "[(0, 1), (-1, 0), (-1, 1), (0, 1/2), (1/2, 1), (-1, 1/2), (-1/2, 0), (-1/2, 1), (-1, \
        -1/2), (-1/2, 1/2), (0, 2), (1, 2), (-1, 2), (1/2, 2), (-1/2, 2), (-2, 0), (-2, 1), (-2, \
        -1), (-2, 1/2), (-2, -1/2), ...]"
    );
    assert_eq!(
        prefix_to_string(
            exhaustive_ordered_unique_rational_vectors_with_dimension(3),
            20
        ),
        "[(-1, 0, 1), (0, 1/2, 1), (-1, 0, 1/2), (-1, 1/2, 1), (-1/2, 0, 1), (-1, -1/2, 0), (-1, \
        -1/2, 1), (-1/2, 0, 1/2), (-1/2, 1/2, 1), (-1, -1/2, 1/2), (0, 1, 2), (-1, 0, 2), (-1, 1, \
        2), (0, 1/2, 2), (1/2, 1, 2), (-1, 1/2, 2), (-1/2, 0, 2), (-1/2, 1, 2), (-1, -1/2, 2), \
        (-1/2, 1/2, 2), ...]"
    );
}

#[test]
fn exhaustive_ordered_unique_rational_vectors_with_dimension_properties() {
    for dimension in 0..5 {
        let vs: Vec<RationalVector> =
            exhaustive_ordered_unique_rational_vectors_with_dimension(dimension)
                .take(1000)
                .collect();
        assert!(vs.iter().all(|v| v.dimension() == dimension));
        assert!(vs.iter().all(|v| v.elements.is_sorted_by(|x, y| x < y)));
        assert_eq!(vs.iter().collect::<HashSet<_>>().len(), vs.len());
    }
}

fn small_elements() -> impl Clone + Iterator<Item = Rational> {
    [Rational::ZERO, Rational::ONE_HALF].into_iter()
}

#[test]
fn test_exhaustive_rational_vectors_from_iterator() {
    assert_eq!(
        prefix_to_string(
            exhaustive_rational_vectors_from_iterator(small_elements()),
            50
        ),
        "[(), (0), (1/2), (0, 0, 0), (0, 0), (0, 0, 1/2), (0, 1/2), (0, 0, 0, 0, 0), (1/2, 0), (0, \
        1/2, 0), (1/2, 1/2), (0, 0, 0, 0), (0, 1/2, 1/2), (0, 0, 0, 1/2), (1/2, 0, 0), (0, 0, 0, \
        0, 0, 0, 0), (1/2, 0, 1/2), (0, 0, 1/2, 0), (1/2, 1/2, 0), (0, 0, 0, 0, 1/2), (1/2, 1/2, \
        1/2), (0, 0, 1/2, 1/2), (0, 1/2, 0, 0), (0, 0, 0, 0, 0, 0, 1/2), (0, 1/2, 0, 1/2), (0, 0, \
        0, 1/2, 0), (0, 1/2, 1/2, 0), (0, 0, 0, 0, 0, 0), (0, 1/2, 1/2, 1/2), (0, 0, 0, 1/2, 1/2), \
        (1/2, 0, 0, 0), (0, 0, 0, 0, 0, 0, 0, 0, 0), (1/2, 0, 0, 1/2), (0, 0, 1/2, 0, 0), (1/2, 0, \
        1/2, 0), (0, 0, 0, 0, 0, 1/2), (1/2, 0, 1/2, 1/2), (0, 0, 1/2, 0, 1/2), (1/2, 1/2, 0, 0), \
        (0, 0, 0, 0, 0, 1/2, 0), (1/2, 1/2, 0, 1/2), (0, 0, 1/2, 1/2, 0), (1/2, 1/2, 1/2, 0), (0, \
        0, 0, 0, 1/2, 0), (1/2, 1/2, 1/2, 1/2), (0, 0, 1/2, 1/2, 1/2), (0, 1/2, 0, 0, 0), (0, 0, \
        0, 0, 0, 0, 0, 0, 1/2), (0, 1/2, 0, 0, 1/2), (0, 0, 0, 0, 1/2, 1/2), ...]"
    );
    // - no elements: the 0-dimensional vector, and nothing else
    assert_eq!(
        exhaustive_rational_vectors_from_iterator(Vec::<Rational>::new().into_iter())
            .take(10)
            .map(|v| v.to_string())
            .collect_vec(),
        &["()"]
    );
}

#[test]
fn test_exhaustive_rational_vectors_with_dimension_from_iterator() {
    // dimension 0: the 0-dimensional vector, and nothing else
    assert_eq!(
        exhaustive_rational_vectors_with_dimension_from_iterator(0, small_elements())
            .map(|v| v.to_string())
            .collect_vec(),
        &["()"]
    );
    assert_eq!(
        prefix_to_string(
            exhaustive_rational_vectors_with_dimension_from_iterator(1, small_elements()),
            20
        ),
        "[(0), (1/2)]"
    );
    assert_eq!(
        prefix_to_string(
            exhaustive_rational_vectors_with_dimension_from_iterator(2, small_elements()),
            20
        ),
        "[(0, 0), (0, 1/2), (1/2, 0), (1/2, 1/2)]"
    );
    // - no elements and a positive dimension: nothing
    assert_eq!(
        exhaustive_rational_vectors_with_dimension_from_iterator(
            2,
            Vec::<Rational>::new().into_iter()
        )
        .count(),
        0
    );
}

#[test]
fn exhaustive_rational_vectors_from_iterator_properties() {
    // Every element is from the set, and no vector is generated twice.
    let vs = exhaustive_rational_vectors_from_iterator(small_elements())
        .take(10000)
        .collect_vec();
    assert!(vs.iter().all(|v| {
        v.elements
            .iter()
            .all(|x| *x == 0u32 || *x == Rational::ONE_HALF)
    }));
    assert_eq!(vs.iter().collect::<HashSet<_>>().len(), vs.len());

    // With 2 possible elements, there are 2^n vectors of each dimension n.
    for dimension in 0..6 {
        let vs =
            exhaustive_rational_vectors_with_dimension_from_iterator(dimension, small_elements())
                .collect_vec();
        assert_eq!(vs.len(), 2usize.pow(u32::try_from(dimension).unwrap()));
        assert!(vs.iter().all(|v| v.dimension() == dimension));
        assert_eq!(vs.iter().collect::<HashSet<_>>().len(), vs.len());
    }
}
