// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::iterators::prefix_to_string;
use malachite_base::num::basic::traits::{Two, Zero};
use malachite_nz::integer::Integer;
use malachite_nz::integer::exhaustive::integer_increasing_range;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::integer_matrix::exhaustive::*;
use std::collections::HashSet;

fn zero_or_one() -> impl Clone + Iterator<Item = Integer> {
    integer_increasing_range(Integer::ZERO, Integer::TWO)
}

#[test]
fn test_exhaustive_integer_matrices() {
    assert_eq!(
        prefix_to_string(exhaustive_integer_matrices(), 50),
        "[[]_{0,0}, []_{0,1}, []_{1,0}, []_{0,2}, [0], []_{0,3}, [1], [0, 0, 0], [-1], [0, 0], \
        [2], [0, 0, 1], [-2], [0, 1], [3], [0; 0], [-3], [1, 0], [4], [0, 1, 0], [-4], [1, 1], \
        [5], []_{2,0}, [-5], [0, -1], [6], [0, 1, 1], [-6], [0, 2], [7], []_{3,0}, [-7], [1, -1], \
        [8], [1, 0, 0], [-8], [1, 2], [9], [0; 1], [-9], [-1, 0], [10], [1, 0, 1], [-10], [-1, 1], \
        [11], [0; 0; 0], [-11], [2, 0], ...]"
    );
}

#[test]
fn test_exhaustive_integer_matrices_from_iterator() {
    assert_eq!(
        prefix_to_string(exhaustive_integer_matrices_from_iterator(zero_or_one()), 50),
        "[[]_{0,0}, []_{0,1}, []_{1,0}, []_{0,2}, [0], []_{0,3}, [1], [0, 0, 0], [0, 0], [0, 0, \
        1], [0, 1], []_{2,0}, [1, 0], [0, 1, 0], [1, 1], []_{3,0}, [0, 1, 1], [0; 0], [1, 0, 0], \
        [0; 0; 0], [1, 0, 1], [0; 1], [1, 1, 0], [0, 0; 0, 0], [1, 1, 1], [1; 0], [1; 1], [0, 0; \
        0, 1], [0; 0; 1], [0, 0; 1, 0], [0; 1; 0], []_{0,4}, [0; 1; 1], [0, 0; 1, 1], [1; 0; 0], \
        [0, 0, 0; 0, 0, 0], [1; 0; 1], [0, 1; 0, 0], [1; 1; 0], [0, 0; 0, 0; 0, 0], [1; 1; 1], [0, \
        1; 0, 1], [0, 1; 1, 0], [0, 0; 0, 0; 0, 1], [0, 1; 1, 1], [0, 0, 0; 0, 0, 1], [1, 0; 0, \
        0], []_{0,5}, [1, 0; 0, 1], [0, 0, 0; 0, 1, 0], ...]"
    );
}

#[test]
fn test_exhaustive_integer_matrices_with_dimensions() {
    // - no rows or no columns: the empty matrix of that shape, and nothing else
    for (height, width) in [(0, 0), (0, 3), (3, 0)] {
        assert_eq!(
            exhaustive_integer_matrices_with_dimensions(height, width)
                .map(|m| m.to_string())
                .collect_vec(),
            &[format!("[]_{{{height},{width}}}")]
        );
    }
    assert_eq!(
        prefix_to_string(exhaustive_integer_matrices_with_dimensions(1, 1), 20),
        "[[0], [1], [-1], [2], [-2], [3], [-3], [4], [-4], [5], [-5], [6], [-6], [7], [-7], [8], \
        [-8], [9], [-9], [10], ...]"
    );
    assert_eq!(
        prefix_to_string(exhaustive_integer_matrices_with_dimensions(1, 3), 20),
        "[[0, 0, 0], [0, 0, 1], [0, 1, 0], [0, 1, 1], [1, 0, 0], [1, 0, 1], [1, 1, 0], [1, 1, 1], \
        [0, 0, -1], [0, 0, 2], [0, 1, -1], [0, 1, 2], [1, 0, -1], [1, 0, 2], [1, 1, -1], [1, 1, \
        2], [0, -1, 0], [0, -1, 1], [0, 2, 0], [0, 2, 1], ...]"
    );
    assert_eq!(
        prefix_to_string(exhaustive_integer_matrices_with_dimensions(2, 2), 20),
        "[[0, 0; 0, 0], [0, 0; 0, 1], [0, 0; 1, 0], [0, 0; 1, 1], [0, 1; 0, 0], [0, 1; 0, 1], [0, \
        1; 1, 0], [0, 1; 1, 1], [1, 0; 0, 0], [1, 0; 0, 1], [1, 0; 1, 0], [1, 0; 1, 1], [1, 1; 0, \
        0], [1, 1; 0, 1], [1, 1; 1, 0], [1, 1; 1, 1], [0, 0; 0, -1], [0, 0; 0, 2], [0, 0; 1, -1], \
        [0, 0; 1, 2], ...]"
    );
}

#[test]
fn test_exhaustive_integer_matrices_with_dimensions_from_iterator() {
    // - finitely many elements: every filling of the shape, once
    assert_eq!(
        prefix_to_string(
            exhaustive_integer_matrices_with_dimensions_from_iterator(2, 2, zero_or_one()),
            20
        ),
        "[[0, 0; 0, 0], [0, 0; 0, 1], [0, 0; 1, 0], [0, 0; 1, 1], [0, 1; 0, 0], [0, 1; 0, 1], [0, \
        1; 1, 0], [0, 1; 1, 1], [1, 0; 0, 0], [1, 0; 0, 1], [1, 0; 1, 0], [1, 0; 1, 1], [1, 1; 0, \
        0], [1, 1; 0, 1], [1, 1; 1, 0], [1, 1; 1, 1]]"
    );
    assert_eq!(
        exhaustive_integer_matrices_with_dimensions_from_iterator(0, 2, zero_or_one())
            .map(|m| m.to_string())
            .collect_vec(),
        &["[]_{0,2}"]
    );
    // - no elements at all: only shapes with no elements are filled
    assert_eq!(
        exhaustive_integer_matrices_with_dimensions_from_iterator(2, 2, [].into_iter()).count(),
        0
    );
}

#[test]
#[should_panic]
fn exhaustive_integer_matrices_with_dimensions_fail() {
    // height * width overflows
    exhaustive_integer_matrices_with_dimensions(usize::MAX, 2);
}

#[test]
#[should_panic]
fn exhaustive_integer_matrices_with_dimensions_from_iterator_fail() {
    // height * width overflows
    exhaustive_integer_matrices_with_dimensions_from_iterator(usize::MAX, 2, zero_or_one());
}

#[test]
fn exhaustive_integer_matrices_properties() {
    // No matrix is generated twice.
    let ms = exhaustive_integer_matrices().take(10000).collect_vec();
    assert!(ms.iter().all(IntegerMatrix::is_valid));
    assert_eq!(ms.iter().collect::<HashSet<_>>().len(), ms.len());

    // With elements from a set, every element is in the set, and no matrix is generated twice.
    let ms = exhaustive_integer_matrices_from_iterator(zero_or_one())
        .take(10000)
        .collect_vec();
    assert!(
        ms.iter()
            .all(|m| m.is_valid() && m.elements.iter().all(|x| *x >= 0u32 && *x < 2u32))
    );
    assert_eq!(ms.iter().collect::<HashSet<_>>().len(), ms.len());

    for height in 0..4 {
        for width in 0..4 {
            let ms: Vec<IntegerMatrix> = exhaustive_integer_matrices_with_dimensions(height, width)
                .take(1000)
                .collect();
            assert!(
                ms.iter()
                    .all(|m| m.is_valid() && m.height() == height && m.width() == width)
            );
            assert_eq!(ms.iter().collect::<HashSet<_>>().len(), ms.len());

            // With two possible elements, there are 2^(height * width) matrices of each shape.
            let ms: Vec<IntegerMatrix> = exhaustive_integer_matrices_with_dimensions_from_iterator(
                height,
                width,
                zero_or_one(),
            )
            .collect();
            assert_eq!(ms.len(), 1 << (height * width));
            assert_eq!(ms.iter().collect::<HashSet<_>>().len(), ms.len());
        }
    }
}
