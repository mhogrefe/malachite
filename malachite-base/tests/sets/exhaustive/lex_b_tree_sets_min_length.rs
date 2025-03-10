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
use malachite_base::sets::exhaustive::lex_b_tree_sets_min_length;
use malachite_base::test_util::sets::exhaustive::{
    exhaustive_b_tree_sets_helper_helper, exhaustive_b_tree_sets_small_helper_helper,
};
use malachite_base::tuples::exhaustive::exhaustive_units;
use std::collections::BTreeSet;
use std::fmt::Debug;

fn lex_b_tree_sets_min_length_helper<I: Clone + Iterator>(
    min_length: u64,
    xs: I,
    out: &[BTreeSet<I::Item>],
) where
    I::Item: Clone + Debug + Eq + Ord,
{
    exhaustive_b_tree_sets_helper_helper(lex_b_tree_sets_min_length(min_length, xs), out);
}

fn lex_b_tree_sets_min_length_small_helper<I: Clone + Iterator>(
    min_length: u64,
    xs: I,
    out_len: usize,
    out: &[BTreeSet<I::Item>],
) where
    I::Item: Clone + Debug + Eq + Ord,
{
    exhaustive_b_tree_sets_small_helper_helper(
        lex_b_tree_sets_min_length(min_length, xs),
        out_len,
        out,
    );
}

#[test]
fn test_lex_b_tree_sets_min_length() {
    lex_b_tree_sets_min_length_small_helper(0, nevers(), 1, &[btreeset! {}]);
    lex_b_tree_sets_min_length_small_helper(4, nevers(), 0, &[]);
    lex_b_tree_sets_min_length_small_helper(
        0,
        exhaustive_units(),
        2,
        &[btreeset! {}, btreeset! {()}],
    );
    lex_b_tree_sets_min_length_small_helper(5, exhaustive_units(), 0, &[]);
    lex_b_tree_sets_min_length_small_helper(
        0,
        exhaustive_bools(),
        4,
        &[btreeset! {}, btreeset! {false}, btreeset! {false, true}, btreeset! {true}],
    );
    lex_b_tree_sets_min_length_small_helper(
        1,
        exhaustive_bools(),
        3,
        &[btreeset! {false}, btreeset! {false, true}, btreeset! {true}],
    );
    lex_b_tree_sets_min_length_small_helper(
        0,
        'a'..='c',
        8,
        &[
            btreeset! {},
            btreeset! {'a'},
            btreeset! {'a', 'b'},
            btreeset! {'a', 'b', 'c'},
            btreeset! {'a', 'c'},
            btreeset! {'b'},
            btreeset! {'b', 'c'},
            btreeset! {'c'},
        ],
    );
    lex_b_tree_sets_min_length_small_helper(
        2,
        'a'..='c',
        4,
        &[
            btreeset! {'a', 'b'},
            btreeset! {'a', 'b', 'c'},
            btreeset! {'a', 'c'},
            btreeset! {'b', 'c'},
        ],
    );
    lex_b_tree_sets_min_length_helper(
        0,
        exhaustive_ascii_chars(),
        &[
            btreeset! {},
            btreeset! {'a'},
            btreeset! {'a', 'b'},
            btreeset! {'a', 'b', 'c'},
            btreeset! {'a', 'b', 'c', 'd'},
            btreeset! {'a', 'b', 'c', 'd', 'e'},
            btreeset! {'a', 'b', 'c', 'd', 'e', 'f'},
            btreeset! {'a', 'b', 'c', 'd', 'e', 'f', 'g'},
            btreeset! {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'},
            btreeset! {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i'},
            btreeset! {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'},
            btreeset! {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k'},
            btreeset! {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l'},
            btreeset! {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm'},
            btreeset! {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n'},
            btreeset! {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o'},
            btreeset! {
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p'
            },
            btreeset! {
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q'
            },
            btreeset! {
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r',
            },
            btreeset! {
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's',
            },
        ],
    );
    lex_b_tree_sets_min_length_helper(
        3,
        exhaustive_ascii_chars(),
        &[
            btreeset! {'a', 'b', 'c'},
            btreeset! {'a', 'b', 'c', 'd'},
            btreeset! {'a', 'b', 'c', 'd', 'e'},
            btreeset! {'a', 'b', 'c', 'd', 'e', 'f'},
            btreeset! {'a', 'b', 'c', 'd', 'e', 'f', 'g'},
            btreeset! {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'},
            btreeset! {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i'},
            btreeset! {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'},
            btreeset! {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k'},
            btreeset! {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l'},
            btreeset! {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm'},
            btreeset! {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n'},
            btreeset! {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o'},
            btreeset! {
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p'
            },
            btreeset! {
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q'
            },
            btreeset! {
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r',
            },
            btreeset! {
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's',
            },
            btreeset! {
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's', 't',
            },
            btreeset! {
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's', 't', 'u',
            },
            btreeset! {
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's', 't', 'u', 'v',
            },
        ],
    );
}
