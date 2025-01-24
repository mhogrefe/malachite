// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::extra_variadic::exhaustive_triples_1_input;
use crate::get_sample_output_types;
use itertools::Itertools;
use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
use malachite_base::nevers::nevers;
use malachite_base::num::exhaustive::{exhaustive_positive_primitive_ints, exhaustive_unsigneds};
use malachite_base::tuples::exhaustive::exhaustive_pairs_1_input;
use std::fmt::Debug;

fn exhaustive_pairs_1_input_helper<T, I: Clone + Iterator<Item = T>>(
    xs: &I,
    out_len: Option<usize>,
    out: &[(T, T)],
) where
    T: Clone + Debug + Eq,
{
    let output_types = get_sample_output_types(2);
    let ps = exhaustive_pairs_1_input(xs.clone(), output_types[0][0], output_types[0][1]);
    assert_eq!(ps.clone().take(20).collect_vec(), out);
    if let Some(out_len) = out_len {
        assert_eq!(ps.count(), out_len);
    }
    for alt_output_types in &output_types[1..] {
        let ps = exhaustive_pairs_1_input(xs.clone(), alt_output_types[0], alt_output_types[1]);
        ps.clone().take(20).for_each(drop);
        if let Some(out_len) = out_len {
            assert_eq!(ps.count(), out_len);
        }
    }
}

#[test]
fn test_exhaustive_pairs_1_input() {
    exhaustive_pairs_1_input_helper(&nevers(), Some(0), &[]);
    exhaustive_pairs_1_input_helper(
        &exhaustive_unsigneds::<u8>(),
        Some(1 << 16),
        &[
            (0, 0),
            (0, 1),
            (1, 0),
            (1, 1),
            (0, 2),
            (0, 3),
            (1, 2),
            (1, 3),
            (2, 0),
            (2, 1),
            (3, 0),
            (3, 1),
            (2, 2),
            (2, 3),
            (3, 2),
            (3, 3),
            (0, 4),
            (0, 5),
            (1, 4),
            (1, 5),
        ],
    );
    exhaustive_pairs_1_input_helper(
        &exhaustive_positive_primitive_ints::<u64>(),
        None,
        &[
            (1, 1),
            (1, 2),
            (2, 1),
            (2, 2),
            (1, 3),
            (1, 4),
            (2, 3),
            (2, 4),
            (3, 1),
            (3, 2),
            (4, 1),
            (4, 2),
            (3, 3),
            (3, 4),
            (4, 3),
            (4, 4),
            (1, 5),
            (1, 6),
            (2, 5),
            (2, 6),
        ],
    );
    exhaustive_pairs_1_input_helper(
        &['x', 'y', 'z'].iter().copied(),
        Some(9),
        &[
            ('x', 'x'),
            ('x', 'y'),
            ('y', 'x'),
            ('y', 'y'),
            ('x', 'z'),
            ('y', 'z'),
            ('z', 'x'),
            ('z', 'y'),
            ('z', 'z'),
        ],
    );
}

fn exhaustive_triples_1_input_helper<T, I: Clone + Iterator<Item = T>>(
    xs: &I,
    out_len: Option<usize>,
    out: &[(T, T, T)],
) where
    T: Clone + Debug + Eq,
{
    let output_types = get_sample_output_types(3);
    let ps = exhaustive_triples_1_input(
        xs.clone(),
        output_types[0][0],
        output_types[0][1],
        output_types[0][2],
    );
    assert_eq!(ps.clone().take(20).collect_vec(), out);
    if let Some(out_len) = out_len {
        assert_eq!(ps.count(), out_len);
    }
    for alt_output_types in &output_types[1..] {
        let ps = exhaustive_triples_1_input(
            xs.clone(),
            alt_output_types[0],
            alt_output_types[1],
            alt_output_types[2],
        );
        ps.clone().take(20).for_each(drop);
        if let Some(out_len) = out_len {
            assert_eq!(ps.count(), out_len);
        }
    }
}

#[test]
fn test_exhaustive_triples_1_input() {
    exhaustive_triples_1_input_helper(
        &['x', 'y', 'z'].iter().copied(),
        Some(27),
        &[
            ('x', 'x', 'x'),
            ('x', 'x', 'y'),
            ('x', 'y', 'x'),
            ('x', 'y', 'y'),
            ('y', 'x', 'x'),
            ('y', 'x', 'y'),
            ('y', 'y', 'x'),
            ('y', 'y', 'y'),
            ('x', 'x', 'z'),
            ('x', 'y', 'z'),
            ('y', 'x', 'z'),
            ('y', 'y', 'z'),
            ('x', 'z', 'x'),
            ('x', 'z', 'y'),
            ('y', 'z', 'x'),
            ('y', 'z', 'y'),
            ('x', 'z', 'z'),
            ('y', 'z', 'z'),
            ('z', 'x', 'x'),
            ('z', 'x', 'y'),
        ],
    );
    exhaustive_triples_1_input_helper(
        &exhaustive_ascii_chars(),
        None,
        &[
            ('a', 'a', 'a'),
            ('a', 'a', 'b'),
            ('a', 'b', 'a'),
            ('a', 'b', 'b'),
            ('b', 'a', 'a'),
            ('b', 'a', 'b'),
            ('b', 'b', 'a'),
            ('b', 'b', 'b'),
            ('a', 'a', 'c'),
            ('a', 'a', 'd'),
            ('a', 'b', 'c'),
            ('a', 'b', 'd'),
            ('b', 'a', 'c'),
            ('b', 'a', 'd'),
            ('b', 'b', 'c'),
            ('b', 'b', 'd'),
            ('a', 'c', 'a'),
            ('a', 'c', 'b'),
            ('a', 'd', 'a'),
            ('a', 'd', 'b'),
        ],
    );
}
