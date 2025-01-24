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
use malachite_base::sets::exhaustive::exhaustive_b_tree_sets;
use malachite_base::test_util::sets::exhaustive::{
    exhaustive_b_tree_sets_helper_helper, exhaustive_b_tree_sets_small_helper_helper,
};
use malachite_base::tuples::exhaustive::exhaustive_units;
use std::collections::BTreeSet;
use std::fmt::Debug;

fn exhaustive_b_tree_sets_helper<I: Clone + Iterator>(xs: I, out: &[BTreeSet<I::Item>])
where
    I::Item: Clone + Debug + Ord,
{
    exhaustive_b_tree_sets_helper_helper(exhaustive_b_tree_sets(xs), out);
}

fn exhaustive_b_tree_sets_small_helper<I: Clone + Iterator>(
    xs: I,
    out_len: usize,
    out: &[BTreeSet<I::Item>],
) where
    I::Item: Clone + Debug + Ord,
{
    exhaustive_b_tree_sets_small_helper_helper(exhaustive_b_tree_sets(xs), out_len, out);
}

#[test]
fn test_exhaustive_b_tree_sets() {
    exhaustive_b_tree_sets_small_helper(nevers(), 1, &[btreeset! {}]);
    exhaustive_b_tree_sets_small_helper(exhaustive_units(), 2, &[btreeset! {}, btreeset! {()}]);
    exhaustive_b_tree_sets_small_helper(
        exhaustive_bools(),
        4,
        &[btreeset! {}, btreeset! {false}, btreeset! {true}, btreeset! {false, true}],
    );
    exhaustive_b_tree_sets_small_helper(
        1..=6,
        64,
        &[
            btreeset! {},
            btreeset! {1},
            btreeset! {2},
            btreeset! {1, 2},
            btreeset! {3},
            btreeset! {1, 3},
            btreeset! {2, 3},
            btreeset! {1, 2, 3},
            btreeset! {4},
            btreeset! {1, 4},
            btreeset! {2, 4},
            btreeset! {1, 2, 4},
            btreeset! {3, 4},
            btreeset! {1, 3, 4},
            btreeset! {2, 3, 4},
            btreeset! {1, 2, 3, 4},
            btreeset! {5},
            btreeset! {1, 5},
            btreeset! {2, 5},
            btreeset! {1, 2, 5},
        ],
    );
    exhaustive_b_tree_sets_small_helper(
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
    exhaustive_b_tree_sets_helper(
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
}
