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
    exhaustive_ordered_vecs_length_inclusive_range, lex_ordered_vecs_length_inclusive_range,
};
use std::fmt::Debug;

fn lex_ordered_vecs_length_inclusive_range_helper<I: Clone + Iterator>(
    a: u64,
    b: u64,
    xs: I,
    out: &[&[I::Item]],
) where
    I::Item: Clone + Debug + Eq,
{
    exhaustive_vecs_helper_helper(lex_ordered_vecs_length_inclusive_range(a, b, xs), out);
}

fn lex_ordered_vecs_length_inclusive_range_small_helper<I: Clone + Iterator>(
    a: u64,
    b: u64,
    xs: I,
    out_len: usize,
    out: &[&[I::Item]],
) where
    I::Item: Clone + Debug + Eq,
{
    exhaustive_vecs_small_helper_helper(
        lex_ordered_vecs_length_inclusive_range(a, b, xs),
        out_len,
        out,
    );
}

#[test]
fn test_lex_ordered_vecs_length_inclusive_range() {
    lex_ordered_vecs_length_inclusive_range_small_helper(0, 0, nevers(), 1, &[&[]]);
    lex_ordered_vecs_length_inclusive_range_small_helper(1, 3, nevers(), 0, &[]);
    lex_ordered_vecs_length_inclusive_range_small_helper(0, 0, exhaustive_units(), 1, &[&[]]);
    lex_ordered_vecs_length_inclusive_range_small_helper(
        1,
        2,
        exhaustive_units(),
        2,
        &[&[()], &[(), ()]],
    );
    lex_ordered_vecs_length_inclusive_range_small_helper(2, 1, exhaustive_units(), 0, &[]);
    lex_ordered_vecs_length_inclusive_range_small_helper(
        0,
        2,
        exhaustive_bools(),
        6,
        &[&[], &[false], &[false, false], &[false, true], &[true], &[true, true]],
    );
    lex_ordered_vecs_length_inclusive_range_small_helper(
        1,
        2,
        'a'..='c',
        9,
        &[
            &['a'],
            &['a', 'a'],
            &['a', 'b'],
            &['a', 'c'],
            &['b'],
            &['b', 'b'],
            &['b', 'c'],
            &['c'],
            &['c', 'c'],
        ],
    );
    lex_ordered_vecs_length_inclusive_range_small_helper(
        2,
        3,
        1..=4,
        30,
        &[
            &[1, 1],
            &[1, 1, 1],
            &[1, 1, 2],
            &[1, 1, 3],
            &[1, 1, 4],
            &[1, 2],
            &[1, 2, 2],
            &[1, 2, 3],
            &[1, 2, 4],
            &[1, 3],
            &[1, 3, 3],
            &[1, 3, 4],
            &[1, 4],
            &[1, 4, 4],
            &[2, 2],
            &[2, 2, 2],
            &[2, 2, 3],
            &[2, 2, 4],
            &[2, 3],
            &[2, 3, 3],
        ],
    );
    lex_ordered_vecs_length_inclusive_range_small_helper(0, 0, 1..=3, 1, &[&[]]);
    lex_ordered_vecs_length_inclusive_range_small_helper(
        1,
        3,
        1..=3,
        19,
        &[
            &[1],
            &[1, 1],
            &[1, 1, 1],
            &[1, 1, 2],
            &[1, 1, 3],
            &[1, 2],
            &[1, 2, 2],
            &[1, 2, 3],
            &[1, 3],
            &[1, 3, 3],
            &[2],
            &[2, 2],
            &[2, 2, 2],
            &[2, 2, 3],
            &[2, 3],
            &[2, 3, 3],
            &[3],
            &[3, 3],
            &[3, 3, 3],
        ],
    );
    lex_ordered_vecs_length_inclusive_range_small_helper(
        2,
        3,
        1..=3,
        16,
        &[
            &[1, 1],
            &[1, 1, 1],
            &[1, 1, 2],
            &[1, 1, 3],
            &[1, 2],
            &[1, 2, 2],
            &[1, 2, 3],
            &[1, 3],
            &[1, 3, 3],
            &[2, 2],
            &[2, 2, 2],
            &[2, 2, 3],
            &[2, 3],
            &[2, 3, 3],
            &[3, 3],
            &[3, 3, 3],
        ],
    );
    lex_ordered_vecs_length_inclusive_range_helper(
        1,
        2,
        exhaustive_ascii_chars(),
        &[
            &['a'],
            &['a', 'a'],
            &['a', 'b'],
            &['a', 'c'],
            &['a', 'd'],
            &['a', 'e'],
            &['a', 'f'],
            &['a', 'g'],
            &['a', 'h'],
            &['a', 'i'],
            &['a', 'j'],
            &['a', 'k'],
            &['a', 'l'],
            &['a', 'm'],
            &['a', 'n'],
            &['a', 'o'],
            &['a', 'p'],
            &['a', 'q'],
            &['a', 'r'],
            &['a', 's'],
        ],
    );
    lex_ordered_vecs_length_inclusive_range_helper(
        2,
        3,
        exhaustive_unsigneds::<u8>(),
        &[
            &[0, 0],
            &[0, 0, 0],
            &[0, 0, 1],
            &[0, 0, 2],
            &[0, 0, 3],
            &[0, 0, 4],
            &[0, 0, 5],
            &[0, 0, 6],
            &[0, 0, 7],
            &[0, 0, 8],
            &[0, 0, 9],
            &[0, 0, 10],
            &[0, 0, 11],
            &[0, 0, 12],
            &[0, 0, 13],
            &[0, 0, 14],
            &[0, 0, 15],
            &[0, 0, 16],
            &[0, 0, 17],
            &[0, 0, 18],
        ],
    );
}

// A `Vec` sorts before every longer `Vec` that begins with it, so sorting the exhaustive output
// must give the lex output.
#[test]
fn lex_ordered_vecs_length_inclusive_range_is_sorted_exhaustive() {
    for n in 0..=4u8 {
        for a in 0..=3u64 {
            for b in a..=4 {
                let mut expected =
                    exhaustive_ordered_vecs_length_inclusive_range(a, b, 0..n).collect_vec();
                expected.sort_unstable();
                assert_eq!(
                    lex_ordered_vecs_length_inclusive_range(a, b, 0..n).collect_vec(),
                    expected
                );
            }
        }
    }
}
