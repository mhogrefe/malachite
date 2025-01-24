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
use malachite_base::vecs::exhaustive::exhaustive_ordered_unique_vecs;
use std::fmt::Debug;

fn exhaustive_ordered_unique_vecs_helper<I: Clone + Iterator>(xs: I, out: &[&[I::Item]])
where
    I::Item: Clone + Debug + Eq,
{
    exhaustive_vecs_helper_helper(exhaustive_ordered_unique_vecs(xs), out);
}

fn exhaustive_ordered_unique_vecs_small_helper<I: Clone + Iterator>(
    xs: I,
    out_len: usize,
    out: &[&[I::Item]],
) where
    I::Item: Clone + Debug + Eq,
{
    exhaustive_vecs_small_helper_helper(exhaustive_ordered_unique_vecs(xs), out_len, out);
}

#[test]
fn test_exhaustive_ordered_unique_vecs() {
    exhaustive_ordered_unique_vecs_small_helper(nevers(), 1, &[&[]]);
    exhaustive_ordered_unique_vecs_small_helper(exhaustive_units(), 2, &[&[], &[()]]);
    exhaustive_ordered_unique_vecs_small_helper(
        exhaustive_bools(),
        4,
        &[&[], &[false], &[true], &[false, true]],
    );
    exhaustive_ordered_unique_vecs_small_helper(
        1..=6,
        64,
        &[
            &[],
            &[1],
            &[2],
            &[1, 2],
            &[3],
            &[1, 3],
            &[2, 3],
            &[1, 2, 3],
            &[4],
            &[1, 4],
            &[2, 4],
            &[1, 2, 4],
            &[3, 4],
            &[1, 3, 4],
            &[2, 3, 4],
            &[1, 2, 3, 4],
            &[5],
            &[1, 5],
            &[2, 5],
            &[1, 2, 5],
        ],
    );
    exhaustive_ordered_unique_vecs_small_helper(
        'a'..='c',
        8,
        &[&[], &['a'], &['b'], &['a', 'b'], &['c'], &['a', 'c'], &['b', 'c'], &['a', 'b', 'c']],
    );
    exhaustive_ordered_unique_vecs_helper(
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
}
