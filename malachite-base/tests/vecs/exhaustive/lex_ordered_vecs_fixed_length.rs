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
    exhaustive_ordered_vecs_fixed_length, lex_ordered_vecs_fixed_length,
};
use std::fmt::Debug;

fn lex_ordered_vecs_fixed_length_helper<I: Clone + Iterator>(len: u64, xs: I, out: &[&[I::Item]])
where
    I::Item: Clone + Debug + Eq,
{
    exhaustive_vecs_helper_helper(lex_ordered_vecs_fixed_length(len, xs), out);
}

fn lex_ordered_vecs_fixed_length_small_helper<I: Clone + Iterator>(
    len: u64,
    xs: I,
    out_len: usize,
    out: &[&[I::Item]],
) where
    I::Item: Clone + Debug + Eq,
{
    exhaustive_vecs_small_helper_helper(lex_ordered_vecs_fixed_length(len, xs), out_len, out);
}

#[test]
fn test_lex_ordered_vecs_fixed_length() {
    lex_ordered_vecs_fixed_length_small_helper(0, nevers(), 1, &[&[]]);
    lex_ordered_vecs_fixed_length_small_helper(1, nevers(), 0, &[]);
    lex_ordered_vecs_fixed_length_small_helper(2, nevers(), 0, &[]);
    lex_ordered_vecs_fixed_length_small_helper(1, exhaustive_units(), 1, &[&[()]]);
    lex_ordered_vecs_fixed_length_small_helper(2, exhaustive_units(), 1, &[&[(), ()]]);
    lex_ordered_vecs_fixed_length_small_helper(5, exhaustive_units(), 1, &[&[(), (), (), (), ()]]);
    lex_ordered_vecs_fixed_length_small_helper(0, exhaustive_unsigneds::<u8>(), 1, &[&[]]);
    lex_ordered_vecs_fixed_length_helper(
        1,
        exhaustive_unsigneds::<u8>(),
        &[
            &[0],
            &[1],
            &[2],
            &[3],
            &[4],
            &[5],
            &[6],
            &[7],
            &[8],
            &[9],
            &[10],
            &[11],
            &[12],
            &[13],
            &[14],
            &[15],
            &[16],
            &[17],
            &[18],
            &[19],
        ],
    );
    lex_ordered_vecs_fixed_length_helper(
        2,
        exhaustive_unsigneds::<u8>(),
        &[
            &[0, 0],
            &[0, 1],
            &[0, 2],
            &[0, 3],
            &[0, 4],
            &[0, 5],
            &[0, 6],
            &[0, 7],
            &[0, 8],
            &[0, 9],
            &[0, 10],
            &[0, 11],
            &[0, 12],
            &[0, 13],
            &[0, 14],
            &[0, 15],
            &[0, 16],
            &[0, 17],
            &[0, 18],
            &[0, 19],
        ],
    );
    lex_ordered_vecs_fixed_length_helper(
        3,
        exhaustive_unsigneds::<u8>(),
        &[
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
            &[0, 0, 19],
        ],
    );
    lex_ordered_vecs_fixed_length_small_helper(
        2,
        exhaustive_bools(),
        3,
        &[&[false, false], &[false, true], &[true, true]],
    );
    lex_ordered_vecs_fixed_length_small_helper(
        3,
        exhaustive_bools(),
        4,
        &[&[false, false, false], &[false, false, true], &[false, true, true], &[true, true, true]],
    );
    lex_ordered_vecs_fixed_length_small_helper(
        2,
        'a'..='c',
        6,
        &[&['a', 'a'], &['a', 'b'], &['a', 'c'], &['b', 'b'], &['b', 'c'], &['c', 'c']],
    );
    lex_ordered_vecs_fixed_length_small_helper(
        3,
        'a'..='c',
        10,
        &[
            &['a', 'a', 'a'],
            &['a', 'a', 'b'],
            &['a', 'a', 'c'],
            &['a', 'b', 'b'],
            &['a', 'b', 'c'],
            &['a', 'c', 'c'],
            &['b', 'b', 'b'],
            &['b', 'b', 'c'],
            &['b', 'c', 'c'],
            &['c', 'c', 'c'],
        ],
    );
    lex_ordered_vecs_fixed_length_small_helper(
        2,
        1..=6,
        21,
        &[
            &[1, 1],
            &[1, 2],
            &[1, 3],
            &[1, 4],
            &[1, 5],
            &[1, 6],
            &[2, 2],
            &[2, 3],
            &[2, 4],
            &[2, 5],
            &[2, 6],
            &[3, 3],
            &[3, 4],
            &[3, 5],
            &[3, 6],
            &[4, 4],
            &[4, 5],
            &[4, 6],
            &[5, 5],
            &[5, 6],
        ],
    );
    lex_ordered_vecs_fixed_length_small_helper(
        3,
        1..=4,
        20,
        &[
            &[1, 1, 1],
            &[1, 1, 2],
            &[1, 1, 3],
            &[1, 1, 4],
            &[1, 2, 2],
            &[1, 2, 3],
            &[1, 2, 4],
            &[1, 3, 3],
            &[1, 3, 4],
            &[1, 4, 4],
            &[2, 2, 2],
            &[2, 2, 3],
            &[2, 2, 4],
            &[2, 3, 3],
            &[2, 3, 4],
            &[2, 4, 4],
            &[3, 3, 3],
            &[3, 3, 4],
            &[3, 4, 4],
            &[4, 4, 4],
        ],
    );
    lex_ordered_vecs_fixed_length_helper(
        2,
        exhaustive_ascii_chars(),
        &[
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
            &['a', 't'],
        ],
    );
    lex_ordered_vecs_fixed_length_helper(
        3,
        exhaustive_ascii_chars(),
        &[
            &['a', 'a', 'a'],
            &['a', 'a', 'b'],
            &['a', 'a', 'c'],
            &['a', 'a', 'd'],
            &['a', 'a', 'e'],
            &['a', 'a', 'f'],
            &['a', 'a', 'g'],
            &['a', 'a', 'h'],
            &['a', 'a', 'i'],
            &['a', 'a', 'j'],
            &['a', 'a', 'k'],
            &['a', 'a', 'l'],
            &['a', 'a', 'm'],
            &['a', 'a', 'n'],
            &['a', 'a', 'o'],
            &['a', 'a', 'p'],
            &['a', 'a', 'q'],
            &['a', 'a', 'r'],
            &['a', 'a', 's'],
            &['a', 'a', 't'],
        ],
    );
    lex_ordered_vecs_fixed_length_small_helper(
        3,
        1..=3,
        10,
        &[
            &[1, 1, 1],
            &[1, 1, 2],
            &[1, 1, 3],
            &[1, 2, 2],
            &[1, 2, 3],
            &[1, 3, 3],
            &[2, 2, 2],
            &[2, 2, 3],
            &[2, 3, 3],
            &[3, 3, 3],
        ],
    );
}

// Lexicographic order is sorted order, so sorting the exhaustive output must give the lex output.
#[test]
fn lex_ordered_vecs_fixed_length_is_sorted_exhaustive() {
    for n in 0..=5u8 {
        for k in 0..=4u64 {
            let mut expected = exhaustive_ordered_vecs_fixed_length(k, 0..n).collect_vec();
            expected.sort_unstable();
            assert_eq!(
                lex_ordered_vecs_fixed_length(k, 0..n).collect_vec(),
                expected
            );
        }
    }
}
