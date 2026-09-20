// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::nevers::nevers;
use malachite_base::num::exhaustive::exhaustive_unsigneds;
use malachite_base::test_util::vecs::exhaustive::{
    exhaustive_vecs_helper_helper, exhaustive_vecs_small_helper_helper,
};
use malachite_base::vecs::exhaustive::exhaustive_ordered_vecs_from_length_iterator;
use std::fmt::Debug;
use std::iter::empty;

fn exhaustive_ordered_vecs_from_length_iterator_helper<
    I: Clone + Iterator<Item = u64>,
    J: Clone + Iterator,
>(
    lengths: I,
    xs: J,
    out: &[&[J::Item]],
) where
    J::Item: Clone + Debug + Eq,
{
    exhaustive_vecs_helper_helper(
        exhaustive_ordered_vecs_from_length_iterator(lengths, xs),
        out,
    );
}

fn exhaustive_ordered_vecs_from_length_iterator_small_helper<
    I: Clone + Iterator<Item = u64>,
    J: Clone + Iterator,
>(
    lengths: I,
    xs: J,
    out_len: usize,
    out: &[&[J::Item]],
) where
    J::Item: Clone + Debug + Eq,
{
    exhaustive_vecs_small_helper_helper(
        exhaustive_ordered_vecs_from_length_iterator(lengths, xs),
        out_len,
        out,
    );
}

#[test]
fn test_exhaustive_ordered_vecs_from_length_iterator() {
    exhaustive_ordered_vecs_from_length_iterator_small_helper(
        [2, 1, 2].iter().copied(),
        exhaustive_bools(),
        8,
        &[
            &[false, false],
            &[false],
            &[true, true],
            &[false, false],
            &[false, true],
            &[true],
            &[true, true],
            &[false, true],
        ],
    );
    exhaustive_ordered_vecs_from_length_iterator_small_helper(
        [0, 0, 1, 0].iter().copied(),
        nevers(),
        2,
        &[&[], &[]],
    );
    exhaustive_ordered_vecs_from_length_iterator_small_helper(
        [0, 1, 2].iter().copied(),
        1..=3,
        10,
        &[&[], &[1], &[2], &[3], &[1, 1], &[2, 2], &[1, 2], &[1, 3], &[3, 3], &[2, 3]],
    );
    exhaustive_ordered_vecs_from_length_iterator_small_helper(empty::<u64>(), 1..=3, 0, &[]);
    exhaustive_ordered_vecs_from_length_iterator_small_helper(
        [1, 1].iter().copied(),
        'a'..='c',
        6,
        &[&['a'], &['a'], &['b'], &['c'], &['b'], &['c']],
    );
    exhaustive_ordered_vecs_from_length_iterator_small_helper(
        [3].iter().copied(),
        1..=2,
        4,
        &[&[1, 1, 1], &[2, 2, 2], &[1, 2, 2], &[1, 1, 2]],
    );
    exhaustive_ordered_vecs_from_length_iterator_small_helper(
        [5].iter().copied(),
        nevers(),
        0,
        &[],
    );
    exhaustive_ordered_vecs_from_length_iterator_helper(
        exhaustive_unsigneds::<u64>(),
        1..=2,
        &[
            &[],
            &[1],
            &[2],
            &[1, 1, 1],
            &[1, 1],
            &[2, 2, 2],
            &[2, 2],
            &[1, 1, 1, 1, 1],
            &[1, 2],
            &[1, 2, 2],
            &[1, 1, 2],
            &[2, 2, 2, 2, 2],
            &[1, 1, 1, 1],
            &[1, 2, 2, 2, 2],
            &[2, 2, 2, 2],
            &[1, 1, 1, 1, 1, 1, 1, 1],
            &[1, 2, 2, 2],
            &[1, 1, 2, 2, 2],
            &[1, 1, 2, 2],
            &[1, 1, 1, 1, 1, 1],
        ],
    );
}
