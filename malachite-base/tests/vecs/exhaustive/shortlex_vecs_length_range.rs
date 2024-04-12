// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::nevers::nevers;
use malachite_base::test_util::vecs::exhaustive::exhaustive_vecs_small_helper_helper;
use malachite_base::tuples::exhaustive::exhaustive_units;
use malachite_base::vecs::exhaustive::shortlex_vecs_length_range;
use std::fmt::Debug;

fn shortlex_vecs_length_range_small_helper<I: Clone + Iterator>(
    a: u64,
    b: u64,
    xs: I,
    out_len: usize,
    out: &[&[I::Item]],
) where
    I::Item: Clone + Debug + Eq,
{
    exhaustive_vecs_small_helper_helper(shortlex_vecs_length_range(a, b, xs), out_len, out);
}

#[test]
fn test_shortlex_vecs_length_range() {
    shortlex_vecs_length_range_small_helper(0, 5, nevers(), 1, &[&[]]);
    shortlex_vecs_length_range_small_helper(6, 10, nevers(), 0, &[]);
    shortlex_vecs_length_range_small_helper(
        0,
        5,
        exhaustive_units(),
        5,
        &[&[], &[()], &[(), ()], &[(), (), ()], &[(), (), (), ()]],
    );
    shortlex_vecs_length_range_small_helper(1, 1, exhaustive_bools(), 0, &[]);
    shortlex_vecs_length_range_small_helper(5, 3, exhaustive_bools(), 0, &[]);
    shortlex_vecs_length_range_small_helper(
        0,
        3,
        exhaustive_bools(),
        7,
        &[&[], &[false], &[true], &[false, false], &[false, true], &[true, false], &[true, true]],
    );
    shortlex_vecs_length_range_small_helper(
        2,
        4,
        exhaustive_bools(),
        12,
        &[
            &[false, false],
            &[false, true],
            &[true, false],
            &[true, true],
            &[false, false, false],
            &[false, false, true],
            &[false, true, false],
            &[false, true, true],
            &[true, false, false],
            &[true, false, true],
            &[true, true, false],
            &[true, true, true],
        ],
    );
    shortlex_vecs_length_range_small_helper(
        5,
        8,
        'a'..='c',
        3159,
        &[
            &['a', 'a', 'a', 'a', 'a'],
            &['a', 'a', 'a', 'a', 'b'],
            &['a', 'a', 'a', 'a', 'c'],
            &['a', 'a', 'a', 'b', 'a'],
            &['a', 'a', 'a', 'b', 'b'],
            &['a', 'a', 'a', 'b', 'c'],
            &['a', 'a', 'a', 'c', 'a'],
            &['a', 'a', 'a', 'c', 'b'],
            &['a', 'a', 'a', 'c', 'c'],
            &['a', 'a', 'b', 'a', 'a'],
            &['a', 'a', 'b', 'a', 'b'],
            &['a', 'a', 'b', 'a', 'c'],
            &['a', 'a', 'b', 'b', 'a'],
            &['a', 'a', 'b', 'b', 'b'],
            &['a', 'a', 'b', 'b', 'c'],
            &['a', 'a', 'b', 'c', 'a'],
            &['a', 'a', 'b', 'c', 'b'],
            &['a', 'a', 'b', 'c', 'c'],
            &['a', 'a', 'c', 'a', 'a'],
            &['a', 'a', 'c', 'a', 'b'],
        ],
    );
}
