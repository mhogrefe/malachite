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
use malachite_base::vecs::exhaustive::exhaustive_unique_vecs;
use std::fmt::Debug;

fn exhaustive_unique_vecs_helper<I: Clone + Iterator>(xs: I, out: &[&[I::Item]])
where
    I::Item: Clone + Debug + Eq,
{
    exhaustive_vecs_helper_helper(exhaustive_unique_vecs(xs), out);
}

fn exhaustive_unique_vecs_small_helper<I: Clone + Iterator>(
    xs: I,
    out_len: usize,
    out: &[&[I::Item]],
) where
    I::Item: Clone + Debug + Eq,
{
    exhaustive_vecs_small_helper_helper(exhaustive_unique_vecs(xs), out_len, out);
}

#[test]
fn test_exhaustive_unique_vecs() {
    exhaustive_unique_vecs_small_helper(nevers(), 1, &[&[]]);
    exhaustive_unique_vecs_small_helper(exhaustive_units(), 2, &[&[], &[()]]);
    exhaustive_unique_vecs_small_helper(
        exhaustive_bools(),
        5,
        &[&[], &[false], &[true], &[false, true], &[true, false]],
    );
    exhaustive_unique_vecs_small_helper(
        1..=6,
        1957,
        &[
            &[],
            &[1],
            &[2],
            &[3],
            &[1, 2],
            &[1, 3],
            &[2, 1],
            &[1, 2, 3],
            &[3, 1],
            &[2, 3],
            &[3, 2],
            &[4],
            &[1, 3, 2],
            &[1, 4],
            &[2, 1, 3],
            &[3, 4],
            &[2, 3, 1],
            &[4, 1],
            &[3, 1, 2],
            &[2, 4],
        ],
    );
    exhaustive_unique_vecs_small_helper(
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
    exhaustive_unique_vecs_helper(
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
}
