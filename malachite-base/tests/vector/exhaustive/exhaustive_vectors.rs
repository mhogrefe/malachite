// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::exhaustive::exhaustive_unsigneds;
use malachite_base::vector::Vector;
use malachite_base::vector::exhaustive::*;
use std::collections::HashSet;

fn strings<T: core::fmt::Display, I: Iterator<Item = Vector<T>>>(xs: I, n: usize) -> Vec<String> {
    xs.take(n).map(|v| v.to_string()).collect()
}

#[test]
fn test_exhaustive_vectors() {
    assert_eq!(
        strings(exhaustive_vectors(exhaustive_unsigneds::<u8>()), 50),
        &[
            "()",
            "(0)",
            "(1)",
            "(0, 0)",
            "(2)",
            "(0, 1)",
            "(3)",
            "(1, 0)",
            "(0, 0, 0)",
            "(0, 0, 0, 0)",
            "(0, 0, 1)",
            "(0, 0, 0, 1)",
            "(0, 1, 0)",
            "(0, 0, 1, 0)",
            "(0, 1, 1)",
            "(0, 0, 1, 1)",
            "(4)",
            "(1, 1)",
            "(5)",
            "(0, 2)",
            "(6)",
            "(0, 3)",
            "(7)",
            "(1, 2)",
            "(1, 0, 0)",
            "(0, 1, 0, 0)",
            "(1, 0, 1)",
            "(0, 1, 0, 1)",
            "(1, 1, 0)",
            "(0, 1, 1, 0)",
            "(1, 1, 1)",
            "(0, 1, 1, 1)",
            "(8)",
            "(1, 3)",
            "(9)",
            "(2, 0)",
            "(10)",
            "(2, 1)",
            "(11)",
            "(3, 0)",
            "(0, 0, 2)",
            "(1, 0, 0, 0)",
            "(0, 0, 3)",
            "(1, 0, 0, 1)",
            "(0, 1, 2)",
            "(1, 0, 1, 0)",
            "(0, 1, 3)",
            "(1, 0, 1, 1)",
            "(12)",
            "(3, 1)",
        ]
    );
    assert_eq!(
        strings(exhaustive_vectors(0..2u8), 30),
        &[
            "()",
            "(0)",
            "(1)",
            "(0, 0)",
            "(0, 1)",
            "(0, 0, 0)",
            "(1, 0)",
            "(0, 0, 1)",
            "(0, 0, 0, 0)",
            "(0, 0, 0, 0, 0)",
            "(0, 0, 0, 1)",
            "(0, 0, 0, 0, 1)",
            "(0, 0, 1, 0)",
            "(0, 0, 0, 1, 0)",
            "(0, 0, 1, 1)",
            "(0, 0, 0, 1, 1)",
            "(1, 1)",
            "(0, 1, 0)",
            "(0, 1, 1)",
            "(0, 1, 0, 0)",
            "(1, 0, 0)",
            "(0, 1, 0, 1)",
            "(1, 0, 1)",
            "(0, 1, 1, 0)",
            "(0, 0, 1, 0, 0)",
            "(0, 0, 0, 0, 0, 0)",
            "(0, 0, 1, 0, 1)",
            "(0, 0, 0, 0, 0, 1)",
            "(0, 0, 1, 1, 0)",
            "(0, 0, 0, 0, 1, 0)",
        ]
    );
    // With no elements, the 0-dimensional vector is the only vector.
    assert_eq!(strings(exhaustive_vectors(0..0u8), 10), &["()"]);
}

#[test]
fn test_exhaustive_vectors_with_dimension() {
    assert_eq!(
        strings(
            exhaustive_vectors_with_dimension(0, exhaustive_unsigneds::<u8>()),
            10
        ),
        &["()"]
    );
    assert_eq!(
        strings(
            exhaustive_vectors_with_dimension(1, exhaustive_unsigneds::<u8>()),
            20
        ),
        &[
            "(0)", "(1)", "(2)", "(3)", "(4)", "(5)", "(6)", "(7)", "(8)", "(9)", "(10)", "(11)",
            "(12)", "(13)", "(14)", "(15)", "(16)", "(17)", "(18)", "(19)",
        ]
    );
    assert_eq!(
        strings(
            exhaustive_vectors_with_dimension(2, exhaustive_unsigneds::<u8>()),
            20
        ),
        &[
            "(0, 0)", "(0, 1)", "(1, 0)", "(1, 1)", "(0, 2)", "(0, 3)", "(1, 2)", "(1, 3)",
            "(2, 0)", "(2, 1)", "(3, 0)", "(3, 1)", "(2, 2)", "(2, 3)", "(3, 2)", "(3, 3)",
            "(0, 4)", "(0, 5)", "(1, 4)", "(1, 5)",
        ]
    );
    assert_eq!(
        strings(
            exhaustive_vectors_with_dimension(3, exhaustive_unsigneds::<u8>()),
            20
        ),
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
    // A finite element iterator gives finitely many vectors.
    assert_eq!(
        strings(exhaustive_vectors_with_dimension(2, 0..3u8), 100),
        &["(0, 0)", "(0, 1)", "(1, 0)", "(1, 1)", "(0, 2)", "(1, 2)", "(2, 0)", "(2, 1)", "(2, 2)",]
    );
}

#[test]
fn exhaustive_vectors_properties() {
    let vs = exhaustive_vectors(exhaustive_unsigneds::<u8>())
        .take(10000)
        .collect_vec();
    assert_eq!(vs.iter().collect::<HashSet<_>>().len(), vs.len());
    for dimension in 0..5 {
        let vs = exhaustive_vectors_with_dimension(dimension, exhaustive_unsigneds::<u8>())
            .take(1000)
            .collect_vec();
        assert!(vs.iter().all(|v| v.dimension() == dimension));
        assert_eq!(vs.iter().collect::<HashSet<_>>().len(), vs.len());
    }
}
