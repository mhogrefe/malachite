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
use malachite_base::sets::exhaustive::shortlex_b_tree_sets_min_length;
use malachite_base::test_util::sets::exhaustive::{
    exhaustive_b_tree_sets_helper_helper, exhaustive_b_tree_sets_small_helper_helper,
};
use malachite_base::tuples::exhaustive::exhaustive_units;
use std::collections::BTreeSet;
use std::fmt::Debug;

fn shortlex_b_tree_sets_min_length_helper<I: Clone + Iterator>(
    min_length: u64,
    xs: I,
    out: &[BTreeSet<I::Item>],
) where
    I::Item: Clone + Debug + Eq + Ord,
{
    exhaustive_b_tree_sets_helper_helper(shortlex_b_tree_sets_min_length(min_length, xs), out);
}

fn shortlex_b_tree_sets_min_length_small_helper<I: Clone + Iterator>(
    min_length: u64,
    xs: I,
    out_len: usize,
    out: &[BTreeSet<I::Item>],
) where
    I::Item: Clone + Debug + Eq + Ord,
{
    exhaustive_b_tree_sets_small_helper_helper(
        shortlex_b_tree_sets_min_length(min_length, xs),
        out_len,
        out,
    );
}

#[test]
fn test_shortlex_b_tree_sets_min_length() {
    shortlex_b_tree_sets_min_length_small_helper(0, nevers(), 1, &[btreeset! {}]);
    shortlex_b_tree_sets_min_length_small_helper(4, nevers(), 0, &[]);
    shortlex_b_tree_sets_min_length_small_helper(
        0,
        exhaustive_units(),
        2,
        &[btreeset! {}, btreeset! {()}],
    );
    shortlex_b_tree_sets_min_length_small_helper(5, exhaustive_units(), 0, &[]);
    shortlex_b_tree_sets_min_length_small_helper(
        0,
        exhaustive_bools(),
        4,
        &[btreeset! {}, btreeset! {false}, btreeset! {true}, btreeset! {false, true}],
    );
    shortlex_b_tree_sets_min_length_small_helper(
        1,
        exhaustive_bools(),
        3,
        &[btreeset! {false}, btreeset! {true}, btreeset! {false, true}],
    );
    shortlex_b_tree_sets_min_length_small_helper(
        0,
        'a'..='c',
        8,
        &[
            btreeset! {},
            btreeset! {'a'},
            btreeset! {'b'},
            btreeset! {'c'},
            btreeset! {'a', 'b'},
            btreeset! {'a', 'c'},
            btreeset! {'b', 'c'},
            btreeset! {'a', 'b', 'c'},
        ],
    );
    shortlex_b_tree_sets_min_length_small_helper(
        2,
        'a'..='c',
        4,
        &[
            btreeset! {'a', 'b'},
            btreeset! {'a', 'c'},
            btreeset! {'b', 'c'},
            btreeset! {'a', 'b', 'c'},
        ],
    );
    shortlex_b_tree_sets_min_length_helper(
        0,
        exhaustive_ascii_chars(),
        &[
            btreeset! {},
            btreeset! {'a'},
            btreeset! {'b'},
            btreeset! {'c'},
            btreeset! {'d'},
            btreeset! {'e'},
            btreeset! {'f'},
            btreeset! {'g'},
            btreeset! {'h'},
            btreeset! {'i'},
            btreeset! {'j'},
            btreeset! {'k'},
            btreeset! {'l'},
            btreeset! {'m'},
            btreeset! {'n'},
            btreeset! {'o'},
            btreeset! {'p'},
            btreeset! {'q'},
            btreeset! {'r'},
            btreeset! {'s'},
        ],
    );
    shortlex_b_tree_sets_min_length_helper(
        3,
        exhaustive_ascii_chars(),
        &[
            btreeset! {'a', 'b', 'c'},
            btreeset! {'a', 'b', 'd'},
            btreeset! {'a', 'b', 'e'},
            btreeset! {'a', 'b', 'f'},
            btreeset! {'a', 'b', 'g'},
            btreeset! {'a', 'b', 'h'},
            btreeset! {'a', 'b', 'i'},
            btreeset! {'a', 'b', 'j'},
            btreeset! {'a', 'b', 'k'},
            btreeset! {'a', 'b', 'l'},
            btreeset! {'a', 'b', 'm'},
            btreeset! {'a', 'b', 'n'},
            btreeset! {'a', 'b', 'o'},
            btreeset! {'a', 'b', 'p'},
            btreeset! {'a', 'b', 'q'},
            btreeset! {'a', 'b', 'r'},
            btreeset! {'a', 'b', 's'},
            btreeset! {'a', 'b', 't'},
            btreeset! {'a', 'b', 'u'},
            btreeset! {'a', 'b', 'v'},
        ],
    );
}
