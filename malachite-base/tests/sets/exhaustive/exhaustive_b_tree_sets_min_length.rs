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
use malachite_base::sets::exhaustive::exhaustive_b_tree_sets_min_length;
use malachite_base::test_util::sets::exhaustive::{
    exhaustive_b_tree_sets_helper_helper, exhaustive_b_tree_sets_small_helper_helper,
};
use malachite_base::tuples::exhaustive::exhaustive_units;
use std::collections::BTreeSet;
use std::fmt::Debug;

fn exhaustive_b_tree_sets_min_length_helper<I: Clone + Iterator>(
    min_length: u64,
    xs: I,
    out: &[BTreeSet<I::Item>],
) where
    I::Item: Clone + Debug + Ord,
{
    exhaustive_b_tree_sets_helper_helper(exhaustive_b_tree_sets_min_length(min_length, xs), out);
}

fn exhaustive_b_tree_sets_min_length_small_helper<I: Clone + Iterator>(
    min_length: u64,
    xs: I,
    out_len: usize,
    out: &[BTreeSet<I::Item>],
) where
    I::Item: Clone + Debug + Ord,
{
    exhaustive_b_tree_sets_small_helper_helper(
        exhaustive_b_tree_sets_min_length(min_length, xs),
        out_len,
        out,
    );
}

#[test]
fn test_exhaustive_b_tree_sets_min_length() {
    exhaustive_b_tree_sets_min_length_small_helper(0, nevers(), 1, &[btreeset! {}]);
    exhaustive_b_tree_sets_min_length_small_helper(4, nevers(), 0, &[]);
    exhaustive_b_tree_sets_min_length_small_helper(
        0,
        exhaustive_units(),
        2,
        &[btreeset! {}, btreeset! {()}],
    );
    exhaustive_b_tree_sets_min_length_small_helper(5, exhaustive_units(), 0, &[]);
    exhaustive_b_tree_sets_min_length_small_helper(
        0,
        exhaustive_bools(),
        4,
        &[btreeset! {}, btreeset! {false}, btreeset! {true}, btreeset! {false, true}],
    );
    exhaustive_b_tree_sets_min_length_small_helper(
        1,
        exhaustive_bools(),
        3,
        &[btreeset! {false}, btreeset! {true}, btreeset! {false, true}],
    );
    exhaustive_b_tree_sets_min_length_small_helper(
        0,
        'a'..='c',
        8,
        &[
            btreeset! {},
            btreeset! {'a'},
            btreeset! {'b'},
            btreeset! {'a', 'b'},
            btreeset! {'c'},
            btreeset! {'a', 'c'},
            btreeset! {'b', 'c'},
            btreeset! {'a', 'b', 'c'},
        ],
    );
    exhaustive_b_tree_sets_min_length_small_helper(
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
    exhaustive_b_tree_sets_min_length_helper(
        0,
        exhaustive_ascii_chars(),
        &[
            btreeset! {},
            btreeset! {'a'},
            btreeset! {'b'},
            btreeset! {'a', 'b'},
            btreeset! {'c'},
            btreeset! {'a', 'c'},
            btreeset! {'b', 'c'},
            btreeset! {'a', 'b', 'c'},
            btreeset! {'d'},
            btreeset! {'a', 'd'},
            btreeset! {'b', 'd'},
            btreeset! {'a', 'b', 'd'},
            btreeset! {'c', 'd'},
            btreeset! {'a', 'c', 'd'},
            btreeset! {'b', 'c', 'd'},
            btreeset! {'a', 'b', 'c', 'd'},
            btreeset! {'e'},
            btreeset! {'a', 'e'},
            btreeset! {'b', 'e'},
            btreeset! {'a', 'b', 'e'},
        ],
    );
    exhaustive_b_tree_sets_min_length_helper(
        3,
        exhaustive_ascii_chars(),
        &[
            btreeset! {'a', 'b', 'c'},
            btreeset! {'a', 'b', 'd'},
            btreeset! {'a', 'c', 'd'},
            btreeset! {'b', 'c', 'd'},
            btreeset! {'a', 'b', 'c', 'd'},
            btreeset! {'a', 'b', 'e'},
            btreeset! {'a', 'c', 'e'},
            btreeset! {'b', 'c', 'e'},
            btreeset! {'a', 'b', 'c', 'e'},
            btreeset! {'a', 'd', 'e'},
            btreeset! {'b', 'd', 'e'},
            btreeset! {'a', 'b', 'd', 'e'},
            btreeset! {'c', 'd', 'e'},
            btreeset! {'a', 'c', 'd', 'e'},
            btreeset! {'b', 'c', 'd', 'e'},
            btreeset! {'a', 'b', 'c', 'd', 'e'},
            btreeset! {'a', 'b', 'f'},
            btreeset! {'a', 'c', 'f'},
            btreeset! {'b', 'c', 'f'},
            btreeset! {'a', 'b', 'c', 'f'},
        ],
    );
}
