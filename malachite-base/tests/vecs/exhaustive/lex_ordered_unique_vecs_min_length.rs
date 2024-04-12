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
use malachite_base::vecs::exhaustive::lex_ordered_unique_vecs_min_length;
use std::fmt::Debug;

fn lex_ordered_unique_vecs_min_length_helper<I: Clone + Iterator>(
    min_length: u64,
    xs: I,
    out: &[&[I::Item]],
) where
    I::Item: Clone + Debug + Eq,
{
    exhaustive_vecs_helper_helper(lex_ordered_unique_vecs_min_length(min_length, xs), out);
}

fn lex_ordered_unique_vecs_min_length_small_helper<I: Clone + Iterator>(
    min_length: u64,
    xs: I,
    out_len: usize,
    out: &[&[I::Item]],
) where
    I::Item: Clone + Debug + Eq,
{
    exhaustive_vecs_small_helper_helper(
        lex_ordered_unique_vecs_min_length(min_length, xs),
        out_len,
        out,
    );
}

#[test]
fn test_lex_ordered_unique_vecs_min_length() {
    lex_ordered_unique_vecs_min_length_small_helper(0, nevers(), 1, &[&[]]);
    lex_ordered_unique_vecs_min_length_small_helper(4, nevers(), 0, &[]);
    lex_ordered_unique_vecs_min_length_small_helper(0, exhaustive_units(), 2, &[&[], &[()]]);
    lex_ordered_unique_vecs_min_length_small_helper(5, exhaustive_units(), 0, &[]);
    lex_ordered_unique_vecs_min_length_small_helper(
        0,
        exhaustive_bools(),
        4,
        &[&[], &[false], &[false, true], &[true]],
    );
    lex_ordered_unique_vecs_min_length_small_helper(
        1,
        exhaustive_bools(),
        3,
        &[&[false], &[false, true], &[true]],
    );
    lex_ordered_unique_vecs_min_length_small_helper(
        0,
        'a'..='c',
        8,
        &[&[], &['a'], &['a', 'b'], &['a', 'b', 'c'], &['a', 'c'], &['b'], &['b', 'c'], &['c']],
    );
    lex_ordered_unique_vecs_min_length_small_helper(
        2,
        'a'..='c',
        4,
        &[&['a', 'b'], &['a', 'b', 'c'], &['a', 'c'], &['b', 'c']],
    );
    lex_ordered_unique_vecs_min_length_helper(
        0,
        exhaustive_ascii_chars(),
        &[
            &[],
            &['a'],
            &['a', 'b'],
            &['a', 'b', 'c'],
            &['a', 'b', 'c', 'd'],
            &['a', 'b', 'c', 'd', 'e'],
            &['a', 'b', 'c', 'd', 'e', 'f'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q'],
            &[
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r',
            ],
            &[
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's',
            ],
        ],
    );
    lex_ordered_unique_vecs_min_length_helper(
        3,
        exhaustive_ascii_chars(),
        &[
            &['a', 'b', 'c'],
            &['a', 'b', 'c', 'd'],
            &['a', 'b', 'c', 'd', 'e'],
            &['a', 'b', 'c', 'd', 'e', 'f'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q'],
            &[
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r',
            ],
            &[
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's',
            ],
            &[
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's', 't',
            ],
            &[
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's', 't', 'u',
            ],
            &[
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's', 't', 'u', 'v',
            ],
        ],
    );
}
