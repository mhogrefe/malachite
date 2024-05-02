// Copyright © 2024 Mikhail Hogrefe
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
use malachite_base::vecs::exhaustive::exhaustive_unique_vecs_min_length;
use std::fmt::Debug;

fn exhaustive_unique_vecs_min_length_helper<I: Clone + Iterator>(
    min_length: u64,
    xs: I,
    out: &[&[I::Item]],
) where
    I::Item: Clone + Debug + Eq,
{
    exhaustive_vecs_helper_helper(exhaustive_unique_vecs_min_length(min_length, xs), out);
}

fn exhaustive_unique_vecs_min_length_small_helper<I: Clone + Iterator>(
    min_length: u64,
    xs: I,
    out_len: usize,
    out: &[&[I::Item]],
) where
    I::Item: Clone + Debug + Eq,
{
    exhaustive_vecs_small_helper_helper(
        exhaustive_unique_vecs_min_length(min_length, xs),
        out_len,
        out,
    );
}

#[test]
fn test_exhaustive_unique_vecs_min_length() {
    exhaustive_unique_vecs_min_length_small_helper(0, nevers(), 1, &[&[]]);
    exhaustive_unique_vecs_min_length_small_helper(4, nevers(), 0, &[]);
    exhaustive_unique_vecs_min_length_small_helper(0, exhaustive_units(), 2, &[&[], &[()]]);
    exhaustive_unique_vecs_min_length_small_helper(5, exhaustive_units(), 0, &[]);
    exhaustive_unique_vecs_min_length_small_helper(
        0,
        exhaustive_bools(),
        5,
        &[&[], &[false], &[true], &[false, true], &[true, false]],
    );
    exhaustive_unique_vecs_min_length_small_helper(
        1,
        exhaustive_bools(),
        4,
        &[&[false], &[true], &[false, true], &[true, false]],
    );
    exhaustive_unique_vecs_min_length_small_helper(
        0,
        'a'..='c',
        16,
        &[
            &[],
            &['a'],
            &['b'],
            &['c'],
            &['a', 'b'],
            &['a', 'c'],
            &['b', 'a'],
            &['a', 'b', 'c'],
            &['c', 'a'],
            &['b', 'c'],
            &['c', 'b'],
            &['a', 'c', 'b'],
            &['b', 'a', 'c'],
            &['b', 'c', 'a'],
            &['c', 'a', 'b'],
            &['c', 'b', 'a'],
        ],
    );
    exhaustive_unique_vecs_min_length_small_helper(
        2,
        'a'..='c',
        12,
        &[
            &['a', 'b'],
            &['a', 'c'],
            &['b', 'a'],
            &['b', 'c'],
            &['c', 'a'],
            &['c', 'b'],
            &['a', 'b', 'c'],
            &['a', 'c', 'b'],
            &['b', 'a', 'c'],
            &['b', 'c', 'a'],
            &['c', 'a', 'b'],
            &['c', 'b', 'a'],
        ],
    );
    exhaustive_unique_vecs_min_length_helper(
        0,
        exhaustive_ascii_chars(),
        &[
            &[],
            &['a'],
            &['b'],
            &['c'],
            &['a', 'b'],
            &['a', 'c'],
            &['b', 'a'],
            &['a', 'b', 'c'],
            &['c', 'a'],
            &['b', 'c'],
            &['c', 'b'],
            &['d'],
            &['a', 'c', 'b'],
            &['a', 'd'],
            &['b', 'a', 'c'],
            &['c', 'd'],
            &['b', 'c', 'a'],
            &['d', 'a'],
            &['c', 'a', 'b'],
            &['b', 'd'],
        ],
    );
    exhaustive_unique_vecs_min_length_helper(
        3,
        exhaustive_ascii_chars(),
        &[
            &['a', 'b', 'c'],
            &['a', 'b', 'd'],
            &['a', 'c', 'b'],
            &['a', 'c', 'd'],
            &['b', 'a', 'c'],
            &['a', 'd', 'b'],
            &['b', 'c', 'a'],
            &['b', 'c', 'd'],
            &['c', 'a', 'b'],
            &['b', 'a', 'd'],
            &['c', 'b', 'a'],
            &['a', 'd', 'c'],
            &['b', 'd', 'a'],
            &['c', 'a', 'd'],
            &['d', 'a', 'b'],
            &['a', 'b', 'e'],
            &['d', 'b', 'a'],
            &['c', 'd', 'a'],
            &['d', 'a', 'c'],
            &['a', 'b', 'c', 'd'],
        ],
    );
}
