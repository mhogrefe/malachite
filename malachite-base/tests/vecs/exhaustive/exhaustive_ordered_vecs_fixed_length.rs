// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
use malachite_base::nevers::nevers;
use malachite_base::num::exhaustive::exhaustive_unsigneds;
use malachite_base::test_util::vecs::exhaustive::{
    exhaustive_vecs_helper_helper, exhaustive_vecs_small_helper_helper,
};
use malachite_base::tuples::exhaustive::exhaustive_units;
use malachite_base::vecs::exhaustive::{
    exhaustive_ordered_vecs_fixed_length, exhaustive_vecs_fixed_length_from_single,
};
use std::fmt::Debug;

fn exhaustive_ordered_vecs_fixed_length_helper<I: Iterator>(len: u64, xs: I, out: &[&[I::Item]])
where
    I::Item: Clone + Debug + Eq,
{
    exhaustive_vecs_helper_helper(exhaustive_ordered_vecs_fixed_length(len, xs), out);
}

fn exhaustive_ordered_vecs_fixed_length_small_helper<I: Clone + Iterator>(
    len: u64,
    xs: I,
    out_len: usize,
    out: &[&[I::Item]],
) where
    I::Item: Clone + Debug + Eq,
{
    exhaustive_vecs_small_helper_helper(
        exhaustive_ordered_vecs_fixed_length(len, xs),
        out_len,
        out,
    );
}

#[test]
fn test_exhaustive_ordered_vecs_fixed_length() {
    exhaustive_ordered_vecs_fixed_length_small_helper(0, nevers(), 1, &[&[]]);
    exhaustive_ordered_vecs_fixed_length_small_helper(1, nevers(), 0, &[]);
    exhaustive_ordered_vecs_fixed_length_small_helper(2, nevers(), 0, &[]);
    exhaustive_ordered_vecs_fixed_length_small_helper(5, nevers(), 0, &[]);
    exhaustive_ordered_vecs_fixed_length_small_helper(1, exhaustive_units(), 1, &[&[()]]);
    exhaustive_ordered_vecs_fixed_length_small_helper(2, exhaustive_units(), 1, &[&[(), ()]]);
    exhaustive_ordered_vecs_fixed_length_small_helper(
        5,
        exhaustive_units(),
        1,
        &[&[(), (), (), (), ()]],
    );
    exhaustive_ordered_vecs_fixed_length_small_helper(0, exhaustive_unsigneds::<u8>(), 1, &[&[]]);
    exhaustive_ordered_vecs_fixed_length_helper(
        1,
        exhaustive_unsigneds::<u8>(),
        &[
            &[0],
            &[1],
            &[2],
            &[4],
            &[3],
            &[5],
            &[6],
            &[9],
            &[7],
            &[8],
            &[10],
            &[12],
            &[11],
            &[13],
            &[14],
            &[18],
            &[15],
            &[16],
            &[17],
            &[19],
        ],
    );
    exhaustive_ordered_vecs_fixed_length_helper(
        2,
        exhaustive_unsigneds::<u8>(),
        &[
            &[0, 0],
            &[1, 1],
            &[0, 1],
            &[0, 2],
            &[2, 2],
            &[1, 2],
            &[3, 3],
            &[2, 3],
            &[0, 3],
            &[1, 3],
            &[4, 4],
            &[1, 4],
            &[0, 4],
            &[2, 4],
            &[3, 4],
            &[2, 5],
            &[5, 5],
            &[0, 5],
            &[1, 5],
            &[3, 5],
        ],
    );
    exhaustive_ordered_vecs_fixed_length_helper(
        3,
        exhaustive_unsigneds::<u8>(),
        &[
            &[0, 0, 0],
            &[1, 1, 1],
            &[0, 1, 1],
            &[0, 2, 2],
            &[0, 0, 1],
            &[2, 2, 2],
            &[0, 0, 2],
            &[3, 3, 3],
            &[1, 2, 2],
            &[0, 1, 2],
            &[1, 1, 2],
            &[0, 3, 3],
            &[0, 0, 3],
            &[1, 3, 3],
            &[1, 1, 3],
            &[1, 2, 3],
            &[0, 1, 3],
            &[2, 3, 3],
            &[2, 2, 3],
            &[4, 4, 4],
        ],
    );
    exhaustive_ordered_vecs_fixed_length_small_helper(
        2,
        exhaustive_bools(),
        3,
        &[&[false, false], &[true, true], &[false, true]],
    );
    exhaustive_ordered_vecs_fixed_length_small_helper(
        3,
        exhaustive_bools(),
        4,
        &[&[false, false, false], &[true, true, true], &[false, true, true], &[false, false, true]],
    );
    exhaustive_ordered_vecs_fixed_length_small_helper(
        2,
        'a'..='c',
        6,
        &[&['a', 'a'], &['b', 'b'], &['a', 'b'], &['a', 'c'], &['c', 'c'], &['b', 'c']],
    );
    exhaustive_ordered_vecs_fixed_length_small_helper(
        3,
        'a'..='c',
        10,
        &[
            &['a', 'a', 'a'],
            &['b', 'b', 'b'],
            &['a', 'b', 'b'],
            &['a', 'c', 'c'],
            &['a', 'a', 'b'],
            &['c', 'c', 'c'],
            &['a', 'a', 'c'],
            &['a', 'b', 'c'],
            &['b', 'c', 'c'],
            &['b', 'b', 'c'],
        ],
    );
    exhaustive_ordered_vecs_fixed_length_small_helper(
        2,
        1..=6,
        21,
        &[
            &[1, 1],
            &[2, 2],
            &[1, 2],
            &[1, 3],
            &[3, 3],
            &[2, 3],
            &[4, 4],
            &[3, 4],
            &[1, 4],
            &[2, 4],
            &[5, 5],
            &[2, 5],
            &[1, 5],
            &[3, 5],
            &[4, 5],
            &[3, 6],
            &[6, 6],
            &[1, 6],
            &[2, 6],
            &[4, 6],
        ],
    );
    exhaustive_ordered_vecs_fixed_length_small_helper(
        3,
        1..=4,
        20,
        &[
            &[1, 1, 1],
            &[2, 2, 2],
            &[1, 2, 2],
            &[1, 3, 3],
            &[1, 1, 2],
            &[3, 3, 3],
            &[1, 1, 3],
            &[4, 4, 4],
            &[2, 3, 3],
            &[1, 2, 3],
            &[2, 2, 3],
            &[1, 4, 4],
            &[1, 1, 4],
            &[2, 4, 4],
            &[2, 2, 4],
            &[2, 3, 4],
            &[1, 2, 4],
            &[3, 4, 4],
            &[3, 3, 4],
            &[1, 3, 4],
        ],
    );
    exhaustive_ordered_vecs_fixed_length_helper(
        2,
        exhaustive_ascii_chars(),
        &[
            &['a', 'a'],
            &['b', 'b'],
            &['a', 'b'],
            &['a', 'c'],
            &['c', 'c'],
            &['b', 'c'],
            &['d', 'd'],
            &['c', 'd'],
            &['a', 'd'],
            &['b', 'd'],
            &['e', 'e'],
            &['b', 'e'],
            &['a', 'e'],
            &['c', 'e'],
            &['d', 'e'],
            &['c', 'f'],
            &['f', 'f'],
            &['a', 'f'],
            &['b', 'f'],
            &['d', 'f'],
        ],
    );
    exhaustive_ordered_vecs_fixed_length_helper(
        3,
        exhaustive_ascii_chars(),
        &[
            &['a', 'a', 'a'],
            &['b', 'b', 'b'],
            &['a', 'b', 'b'],
            &['a', 'c', 'c'],
            &['a', 'a', 'b'],
            &['c', 'c', 'c'],
            &['a', 'a', 'c'],
            &['d', 'd', 'd'],
            &['b', 'c', 'c'],
            &['a', 'b', 'c'],
            &['b', 'b', 'c'],
            &['a', 'd', 'd'],
            &['a', 'a', 'd'],
            &['b', 'd', 'd'],
            &['b', 'b', 'd'],
            &['b', 'c', 'd'],
            &['a', 'b', 'd'],
            &['c', 'd', 'd'],
            &['c', 'c', 'd'],
            &['e', 'e', 'e'],
        ],
    );
}

// Sorting every fixed-length `Vec` and removing the duplicates leaves exactly the multisets, so the
// two generators must agree as sets, and the ordered generator must produce no duplicates.
#[test]
fn exhaustive_ordered_vecs_fixed_length_matches_sorted_vecs() {
    for n in 0..=5u8 {
        for k in 0..=4u64 {
            let mut expected = exhaustive_vecs_fixed_length_from_single(k, 0..n)
                .map(|mut xs| {
                    xs.sort_unstable();
                    xs
                })
                .collect_vec();
            expected.sort_unstable();
            expected.dedup();
            let mut actual = exhaustive_ordered_vecs_fixed_length(k, 0..n).collect_vec();
            let len = actual.len();
            actual.sort_unstable();
            actual.dedup();
            assert_eq!(actual.len(), len);
            assert_eq!(actual, expected);
        }
    }
}
