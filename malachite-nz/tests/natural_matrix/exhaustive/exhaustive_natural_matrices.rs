// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::iterators::prefix_to_string;
use malachite_base::matrix::Matrix;
use malachite_base::num::basic::traits::{Two, Zero};
use malachite_nz::natural::Natural;
use malachite_nz::natural::exhaustive::exhaustive_natural_range;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::natural_matrix::exhaustive::*;
use std::collections::HashSet;

fn zero_or_one() -> impl Clone + Iterator<Item = Natural> {
    exhaustive_natural_range(Natural::ZERO, Natural::TWO)
}

#[test]
fn test_exhaustive_natural_matrices() {
    assert_eq!(
        prefix_to_string(exhaustive_natural_matrices(), 50),
        "[[]_{0,0}, []_{0,1}, []_{1,0}, []_{0,2}, [0], []_{0,3}, [1], [0, 0, 0], [2], [0, 0], [3], \
        [0, 0, 1], [4], [0, 1], [5], [0; 0], [6], [1, 0], [7], [0, 1, 0], [8], [1, 1], [9], \
        []_{2,0}, [10], [0, 2], [11], [0, 1, 1], [12], [0, 3], [13], []_{3,0}, [14], [1, 2], [15], \
        [1, 0, 0], [16], [1, 3], [17], [0; 1], [18], [2, 0], [19], [1, 0, 1], [20], [2, 1], [21], \
        [0; 0; 0], [22], [3, 0], ...]"
    );
}

#[test]
fn test_exhaustive_natural_matrices_from_iterator() {
    assert_eq!(
        prefix_to_string(exhaustive_natural_matrices_from_iterator(zero_or_one()), 50),
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
fn test_exhaustive_natural_matrices_with_dimensions() {
    // - no rows or no columns: the empty matrix of that shape, and nothing else
    for (height, width) in [(0, 0), (0, 3), (3, 0)] {
        assert_eq!(
            exhaustive_natural_matrices_with_dimensions(height, width)
                .map(|m| m.to_string())
                .collect_vec(),
            &[format!("[]_{{{height},{width}}}")]
        );
    }
    assert_eq!(
        prefix_to_string(exhaustive_natural_matrices_with_dimensions(1, 1), 20),
        "[[0], [1], [2], [3], [4], [5], [6], [7], [8], [9], [10], [11], [12], [13], [14], [15], \
        [16], [17], [18], [19], ...]"
    );
    assert_eq!(
        prefix_to_string(exhaustive_natural_matrices_with_dimensions(1, 3), 20),
        "[[0, 0, 0], [0, 0, 1], [0, 1, 0], [0, 1, 1], [1, 0, 0], [1, 0, 1], [1, 1, 0], [1, 1, 1], \
        [0, 0, 2], [0, 0, 3], [0, 1, 2], [0, 1, 3], [1, 0, 2], [1, 0, 3], [1, 1, 2], [1, 1, 3], \
        [0, 2, 0], [0, 2, 1], [0, 3, 0], [0, 3, 1], ...]"
    );
    assert_eq!(
        prefix_to_string(exhaustive_natural_matrices_with_dimensions(2, 2), 20),
        "[[0, 0; 0, 0], [0, 0; 0, 1], [0, 0; 1, 0], [0, 0; 1, 1], [0, 1; 0, 0], [0, 1; 0, 1], [0, \
        1; 1, 0], [0, 1; 1, 1], [1, 0; 0, 0], [1, 0; 0, 1], [1, 0; 1, 0], [1, 0; 1, 1], [1, 1; 0, \
        0], [1, 1; 0, 1], [1, 1; 1, 0], [1, 1; 1, 1], [0, 0; 0, 2], [0, 0; 0, 3], [0, 0; 1, 2], \
        [0, 0; 1, 3], ...]"
    );
}

#[test]
fn test_exhaustive_natural_matrices_with_dimensions_from_iterator() {
    // - finitely many elements: every filling of the shape, once
    assert_eq!(
        prefix_to_string(
            exhaustive_natural_matrices_with_dimensions_from_iterator(2, 2, zero_or_one()),
            20
        ),
        "[[0, 0; 0, 0], [0, 0; 0, 1], [0, 0; 1, 0], [0, 0; 1, 1], [0, 1; 0, 0], [0, 1; 0, 1], [0, \
        1; 1, 0], [0, 1; 1, 1], [1, 0; 0, 0], [1, 0; 0, 1], [1, 0; 1, 0], [1, 0; 1, 1], [1, 1; 0, \
        0], [1, 1; 0, 1], [1, 1; 1, 0], [1, 1; 1, 1]]"
    );
    assert_eq!(
        exhaustive_natural_matrices_with_dimensions_from_iterator(0, 2, zero_or_one())
            .map(|m| m.to_string())
            .collect_vec(),
        &["[]_{0,2}"]
    );
    // - no elements at all: only shapes with no elements are filled
    assert_eq!(
        exhaustive_natural_matrices_with_dimensions_from_iterator(2, 2, [].into_iter()).count(),
        0
    );
}

#[test]
#[should_panic]
fn exhaustive_natural_matrices_with_dimensions_fail() {
    // height * width overflows
    exhaustive_natural_matrices_with_dimensions(u64::MAX, 2);
}

#[test]
#[should_panic]
fn exhaustive_natural_matrices_with_dimensions_from_iterator_fail() {
    // height * width overflows
    exhaustive_natural_matrices_with_dimensions_from_iterator(u64::MAX, 2, zero_or_one());
}

#[test]
fn exhaustive_natural_matrices_properties() {
    // No matrix is generated twice.
    let ms = exhaustive_natural_matrices().take(10000).collect_vec();
    assert!(ms.iter().all(NaturalMatrix::is_valid));
    assert_eq!(ms.iter().collect::<HashSet<_>>().len(), ms.len());

    // With elements from a set, every element is in the set, and no matrix is generated twice.
    let ms = exhaustive_natural_matrices_from_iterator(zero_or_one())
        .take(10000)
        .collect_vec();
    assert!(
        ms.iter()
            .all(|m| m.is_valid() && m.elements_ref().iter().all(|x| *x >= 0u32 && *x < 2u32))
    );
    assert_eq!(ms.iter().collect::<HashSet<_>>().len(), ms.len());

    for height in 0..4 {
        for width in 0..4 {
            let ms: Vec<NaturalMatrix> = exhaustive_natural_matrices_with_dimensions(height, width)
                .take(1000)
                .collect();
            assert!(
                ms.iter()
                    .all(|m| m.is_valid() && m.height() == height && m.width() == width)
            );
            assert_eq!(ms.iter().collect::<HashSet<_>>().len(), ms.len());

            // With two possible elements, there are 2^(height * width) matrices of each shape.
            let ms: Vec<NaturalMatrix> = exhaustive_natural_matrices_with_dimensions_from_iterator(
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

#[test]
fn test_exhaustive_natural_square_matrices() {
    assert_eq!(
        prefix_to_string(exhaustive_natural_square_matrices(), 30),
        "[[]_{0,0}, [0], [1], [0, 0, 0; 0, 0, 0; 0, 0, 0], [2], [0, 0; 0, 0], [3], [0, 0, 0, 0; 0, \
        0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0], [4], [0, 0; 0, 1], [5], [0, 0, 0; 0, 0, 0; 0, 0, 1], \
        [6], [0, 0; 1, 0], [7], [0, 0, 0, 0, 0; 0, 0, 0, 0, 0; 0, 0, 0, 0, 0; 0, 0, 0, 0, 0; 0, 0, \
        0, 0, 0], [8], [0, 0; 1, 1], [9], [0, 0, 0; 0, 0, 0; 0, 1, 0], [10], [0, 1; 0, 0], [11], \
        [0, 0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 1], [12], [0, 1; 0, 1], [13], [0, 0, 0; 0, \
        0, 0; 0, 1, 1], [14], [0, 1; 1, 0], ...]"
    );
}

#[test]
fn test_exhaustive_natural_square_matrices_from_iterator() {
    assert_eq!(
        prefix_to_string(
            exhaustive_natural_square_matrices_from_iterator(zero_or_one()),
            30
        ),
        "[[]_{0,0}, [0], [1], [0, 0, 0; 0, 0, 0; 0, 0, 0], [0, 0; 0, 0], [0, 0, 0; 0, 0, 0; 0, 0, \
        1], [0, 0; 0, 1], [0, 0, 0, 0, 0; 0, 0, 0, 0, 0; 0, 0, 0, 0, 0; 0, 0, 0, 0, 0; 0, 0, 0, 0, \
        0], [0, 0; 1, 0], [0, 0, 0; 0, 0, 0; 0, 1, 0], [0, 0; 1, 1], [0, 0, 0, 0; 0, 0, 0, 0; 0, \
        0, 0, 0; 0, 0, 0, 0], [0, 1; 0, 0], [0, 0, 0; 0, 0, 0; 0, 1, 1], [0, 1; 0, 1], [0, 0, 0, \
        0, 0, 0; 0, 0, 0, 0, 0, 0; 0, 0, 0, 0, 0, 0; 0, 0, 0, 0, 0, 0; 0, 0, 0, 0, 0, 0; 0, 0, 0, \
        0, 0, 0], [0, 1; 1, 0], [0, 0, 0; 0, 0, 0; 1, 0, 0], [0, 1; 1, 1], [0, 0, 0, 0; 0, 0, 0, \
        0; 0, 0, 0, 0; 0, 0, 0, 1], [1, 0; 0, 0], [0, 0, 0; 0, 0, 0; 1, 0, 1], [1, 0; 0, 1], [0, \
        0, 0, 0, 0; 0, 0, 0, 0, 0; 0, 0, 0, 0, 0; 0, 0, 0, 0, 0; 0, 0, 0, 0, 1], [1, 0; 1, 0], [0, \
        0, 0; 0, 0, 0; 1, 1, 0], [1, 0; 1, 1], [0, 0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0; 0, 0, 1, 0], \
        [1, 1; 0, 0], [0, 0, 0; 0, 0, 0; 1, 1, 1], ...]"
    );
}

#[test]
fn exhaustive_natural_square_matrices_properties() {
    // Every matrix is square, and no matrix is generated twice.
    let ms = exhaustive_natural_square_matrices()
        .take(10000)
        .collect_vec();
    assert!(ms.iter().all(|m| m.is_valid() && m.is_square()));
    assert_eq!(ms.iter().collect::<HashSet<_>>().len(), ms.len());

    // With elements from a set, every element is in the set as well.
    let ms = exhaustive_natural_square_matrices_from_iterator(zero_or_one())
        .take(10000)
        .collect_vec();
    assert!(
        ms.iter()
            .all(|m| m.is_valid() && m.is_square() && m.elements_ref().iter().all(|x| *x < 2u32))
    );
    assert_eq!(ms.iter().collect::<HashSet<_>>().len(), ms.len());

    // The square matrices of each size are exactly the matrices with those dimensions.
    for size in 0..3 {
        let mut ms: Vec<_> = exhaustive_natural_square_matrices_from_iterator(zero_or_one())
            .filter(|m| m.height() == size)
            .take(1 << (size * size))
            .collect();
        let mut ns: Vec<_> =
            exhaustive_natural_matrices_with_dimensions_from_iterator(size, size, zero_or_one())
                .collect();
        ms.sort_by_key(ToString::to_string);
        ns.sort_by_key(ToString::to_string);
        assert_eq!(ms, ns);
    }
}
