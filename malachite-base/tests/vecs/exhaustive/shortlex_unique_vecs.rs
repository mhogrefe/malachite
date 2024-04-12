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
use malachite_base::vecs::exhaustive::shortlex_unique_vecs;
use std::fmt::Debug;

fn shortlex_unique_vecs_helper<I: Clone + Iterator>(xs: I, out: &[&[I::Item]])
where
    I::Item: Clone + Debug + Eq,
{
    exhaustive_vecs_helper_helper(shortlex_unique_vecs(xs), out);
}

fn shortlex_unique_vecs_small_helper<I: Clone + Iterator>(xs: I, out_len: usize, out: &[&[I::Item]])
where
    I::Item: Clone + Debug + Eq,
{
    exhaustive_vecs_small_helper_helper(shortlex_unique_vecs(xs), out_len, out);
}

#[test]
fn test_shortlex_unique_vecs() {
    shortlex_unique_vecs_small_helper(nevers(), 1, &[&[]]);
    shortlex_unique_vecs_small_helper(exhaustive_units(), 2, &[&[], &[()]]);
    shortlex_unique_vecs_small_helper(
        exhaustive_bools(),
        5,
        &[&[], &[false], &[true], &[false, true], &[true, false]],
    );
    shortlex_unique_vecs_small_helper(
        1..=6,
        1957,
        &[
            &[],
            &[1],
            &[2],
            &[3],
            &[4],
            &[5],
            &[6],
            &[1, 2],
            &[1, 3],
            &[1, 4],
            &[1, 5],
            &[1, 6],
            &[2, 1],
            &[2, 3],
            &[2, 4],
            &[2, 5],
            &[2, 6],
            &[3, 1],
            &[3, 2],
            &[3, 4],
        ],
    );
    shortlex_unique_vecs_small_helper(
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
    shortlex_unique_vecs_helper(
        exhaustive_ascii_chars(),
        &[
            &[],
            &['a'],
            &['b'],
            &['c'],
            &['d'],
            &['e'],
            &['f'],
            &['g'],
            &['h'],
            &['i'],
            &['j'],
            &['k'],
            &['l'],
            &['m'],
            &['n'],
            &['o'],
            &['p'],
            &['q'],
            &['r'],
            &['s'],
        ],
    );
}
