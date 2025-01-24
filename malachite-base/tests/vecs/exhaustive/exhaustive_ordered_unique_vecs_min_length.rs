// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
use malachite_base::nevers::nevers;
use malachite_base::test_util::vecs::exhaustive::{
    exhaustive_vecs_helper_helper, exhaustive_vecs_small_helper_helper,
};
use malachite_base::tuples::exhaustive::exhaustive_units;
use malachite_base::vecs::exhaustive::exhaustive_ordered_unique_vecs_min_length;
use std::fmt::Debug;

fn exhaustive_ordered_unique_vecs_min_length_helper<I: Clone + Iterator>(
    min_length: u64,
    xs: I,
    out: &[&[I::Item]],
) where
    I::Item: Clone + Debug + Eq,
{
    exhaustive_vecs_helper_helper(
        exhaustive_ordered_unique_vecs_min_length(min_length, xs),
        out,
    );
}

fn exhaustive_ordered_unique_vecs_min_length_small_helper<I: Clone + Iterator>(
    min_length: u64,
    xs: I,
    out_len: usize,
    out: &[&[I::Item]],
) where
    I::Item: Clone + Debug + Eq,
{
    exhaustive_vecs_small_helper_helper(
        exhaustive_ordered_unique_vecs_min_length(min_length, xs),
        out_len,
        out,
    );
}

#[test]
fn test_exhaustive_ordered_unique_vecs_min_length() {
    exhaustive_ordered_unique_vecs_min_length_small_helper(0, nevers(), 1, &[&[]]);
    exhaustive_ordered_unique_vecs_min_length_small_helper(4, nevers(), 0, &[]);
    exhaustive_ordered_unique_vecs_min_length_small_helper(0, exhaustive_units(), 2, &[&[], &[()]]);
    exhaustive_ordered_unique_vecs_min_length_small_helper(5, exhaustive_units(), 0, &[]);
    exhaustive_ordered_unique_vecs_min_length_small_helper(
        0,
        exhaustive_bools(),
        4,
        &[&[], &[false], &[true], &[false, true]],
    );
    exhaustive_ordered_unique_vecs_min_length_small_helper(
        1,
        exhaustive_bools(),
        3,
        &[&[false], &[true], &[false, true]],
    );
    exhaustive_ordered_unique_vecs_min_length_small_helper(
        0,
        'a'..='c',
        8,
        &[&[], &['a'], &['b'], &['a', 'b'], &['c'], &['a', 'c'], &['b', 'c'], &['a', 'b', 'c']],
    );
    exhaustive_ordered_unique_vecs_min_length_small_helper(
        2,
        'a'..='c',
        4,
        &[&['a', 'b'], &['a', 'c'], &['b', 'c'], &['a', 'b', 'c']],
    );
    exhaustive_ordered_unique_vecs_min_length_helper(
        0,
        exhaustive_ascii_chars(),
        &[
            &[],
            &['a'],
            &['b'],
            &['a', 'b'],
            &['c'],
            &['a', 'c'],
            &['b', 'c'],
            &['a', 'b', 'c'],
            &['d'],
            &['a', 'd'],
            &['b', 'd'],
            &['a', 'b', 'd'],
            &['c', 'd'],
            &['a', 'c', 'd'],
            &['b', 'c', 'd'],
            &['a', 'b', 'c', 'd'],
            &['e'],
            &['a', 'e'],
            &['b', 'e'],
            &['a', 'b', 'e'],
        ],
    );
    exhaustive_ordered_unique_vecs_min_length_helper(
        3,
        exhaustive_ascii_chars(),
        &[
            &['a', 'b', 'c'],
            &['a', 'b', 'd'],
            &['a', 'c', 'd'],
            &['b', 'c', 'd'],
            &['a', 'b', 'c', 'd'],
            &['a', 'b', 'e'],
            &['a', 'c', 'e'],
            &['b', 'c', 'e'],
            &['a', 'b', 'c', 'e'],
            &['a', 'd', 'e'],
            &['b', 'd', 'e'],
            &['a', 'b', 'd', 'e'],
            &['c', 'd', 'e'],
            &['a', 'c', 'd', 'e'],
            &['b', 'c', 'd', 'e'],
            &['a', 'b', 'c', 'd', 'e'],
            &['a', 'b', 'f'],
            &['a', 'c', 'f'],
            &['b', 'c', 'f'],
            &['a', 'b', 'c', 'f'],
        ],
    );
}
