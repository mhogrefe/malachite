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
use malachite_base::num::exhaustive::exhaustive_unsigneds;
use malachite_base::sets::exhaustive::exhaustive_b_tree_sets_fixed_length;
use malachite_base::test_util::sets::exhaustive::{
    exhaustive_b_tree_sets_helper_helper, exhaustive_b_tree_sets_small_helper_helper,
};
use malachite_base::tuples::exhaustive::exhaustive_units;
use malachite_base::vecs::exhaustive::exhaustive_ordered_unique_vecs_fixed_length;
use std::collections::BTreeSet;
use std::fmt::Debug;

fn exhaustive_b_tree_sets_fixed_length_helper<I: Clone + Iterator>(
    len: u64,
    xs: I,
    out: &[BTreeSet<I::Item>],
) where
    I::Item: Clone + Debug + Ord,
{
    exhaustive_b_tree_sets_helper_helper(exhaustive_b_tree_sets_fixed_length(len, xs), out);
}

fn exhaustive_b_tree_sets_fixed_length_small_helper<I: Clone + Iterator>(
    len: u64,
    xs: I,
    out_len: usize,
    out: &[BTreeSet<I::Item>],
) where
    I::Item: Clone + Debug + Ord,
{
    exhaustive_b_tree_sets_small_helper_helper(
        exhaustive_b_tree_sets_fixed_length(len, xs),
        out_len,
        out,
    );
}

#[test]
fn test_exhaustive_b_tree_sets_fixed_length() {
    // This demonstrates that 0 ^ 0 == 1:
    exhaustive_b_tree_sets_fixed_length_small_helper(0, nevers(), 1, &[btreeset! {}]);
    exhaustive_b_tree_sets_fixed_length_small_helper(1, nevers(), 0, &[]);
    exhaustive_b_tree_sets_fixed_length_small_helper(2, nevers(), 0, &[]);
    exhaustive_b_tree_sets_fixed_length_small_helper(5, nevers(), 0, &[]);
    exhaustive_b_tree_sets_fixed_length_small_helper(1, exhaustive_units(), 1, &[btreeset! {()}]);
    exhaustive_b_tree_sets_fixed_length_small_helper(2, exhaustive_units(), 0, &[]);
    exhaustive_b_tree_sets_fixed_length_small_helper(5, exhaustive_units(), 0, &[]);
    exhaustive_b_tree_sets_fixed_length_small_helper(
        0,
        exhaustive_unsigneds::<u8>(),
        1,
        &[btreeset! {}],
    );
    exhaustive_b_tree_sets_fixed_length_small_helper(
        1,
        exhaustive_unsigneds::<u8>(),
        256,
        &[
            btreeset! {0},
            btreeset! {1},
            btreeset! {2},
            btreeset! {3},
            btreeset! {4},
            btreeset! {5},
            btreeset! {6},
            btreeset! {7},
            btreeset! {8},
            btreeset! {9},
            btreeset! {10},
            btreeset! {11},
            btreeset! {12},
            btreeset! {13},
            btreeset! {14},
            btreeset! {15},
            btreeset! {16},
            btreeset! {17},
            btreeset! {18},
            btreeset! {19},
        ],
    );
    exhaustive_b_tree_sets_fixed_length_helper(
        1,
        exhaustive_unsigneds::<u64>(),
        &[
            btreeset! {0},
            btreeset! {1},
            btreeset! {2},
            btreeset! {3},
            btreeset! {4},
            btreeset! {5},
            btreeset! {6},
            btreeset! {7},
            btreeset! {8},
            btreeset! {9},
            btreeset! {10},
            btreeset! {11},
            btreeset! {12},
            btreeset! {13},
            btreeset! {14},
            btreeset! {15},
            btreeset! {16},
            btreeset! {17},
            btreeset! {18},
            btreeset! {19},
        ],
    );
    exhaustive_b_tree_sets_fixed_length_small_helper(
        2,
        exhaustive_unsigneds::<u8>(),
        32640,
        &[
            btreeset! {0, 1},
            btreeset! {0, 2},
            btreeset! {1, 2},
            btreeset! {0, 3},
            btreeset! {1, 3},
            btreeset! {2, 3},
            btreeset! {0, 4},
            btreeset! {1, 4},
            btreeset! {2, 4},
            btreeset! {3, 4},
            btreeset! {0, 5},
            btreeset! {1, 5},
            btreeset! {2, 5},
            btreeset! {3, 5},
            btreeset! {4, 5},
            btreeset! {0, 6},
            btreeset! {1, 6},
            btreeset! {2, 6},
            btreeset! {3, 6},
            btreeset! {4, 6},
        ],
    );
    exhaustive_b_tree_sets_fixed_length_helper(
        3,
        exhaustive_unsigneds::<u8>(),
        &[
            btreeset! {0, 1, 2},
            btreeset! {0, 1, 3},
            btreeset! {0, 2, 3},
            btreeset! {1, 2, 3},
            btreeset! {0, 1, 4},
            btreeset! {0, 2, 4},
            btreeset! {1, 2, 4},
            btreeset! {0, 3, 4},
            btreeset! {1, 3, 4},
            btreeset! {2, 3, 4},
            btreeset! {0, 1, 5},
            btreeset! {0, 2, 5},
            btreeset! {1, 2, 5},
            btreeset! {0, 3, 5},
            btreeset! {1, 3, 5},
            btreeset! {2, 3, 5},
            btreeset! {0, 4, 5},
            btreeset! {1, 4, 5},
            btreeset! {2, 4, 5},
            btreeset! {3, 4, 5},
        ],
    );
    exhaustive_b_tree_sets_fixed_length_small_helper(
        2,
        exhaustive_ascii_chars(),
        8128,
        &[
            btreeset! {'a', 'b'},
            btreeset! {'a', 'c'},
            btreeset! {'b', 'c'},
            btreeset! {'a', 'd'},
            btreeset! {'b', 'd'},
            btreeset! {'c', 'd'},
            btreeset! {'a', 'e'},
            btreeset! {'b', 'e'},
            btreeset! {'c', 'e'},
            btreeset! {'d', 'e'},
            btreeset! {'a', 'f'},
            btreeset! {'b', 'f'},
            btreeset! {'c', 'f'},
            btreeset! {'d', 'f'},
            btreeset! {'e', 'f'},
            btreeset! {'a', 'g'},
            btreeset! {'b', 'g'},
            btreeset! {'c', 'g'},
            btreeset! {'d', 'g'},
            btreeset! {'e', 'g'},
        ],
    );
    exhaustive_b_tree_sets_fixed_length_small_helper(
        1,
        exhaustive_bools(),
        2,
        &[btreeset! {false}, btreeset! {true}],
    );
    exhaustive_b_tree_sets_fixed_length_small_helper(
        2,
        exhaustive_bools(),
        1,
        &[btreeset! {false, true}],
    );
    exhaustive_b_tree_sets_fixed_length_small_helper(4, exhaustive_bools(), 0, &[]);
    exhaustive_b_tree_sets_fixed_length_small_helper(
        4,
        1..=6,
        15,
        &[
            btreeset! {1, 2, 3, 4},
            btreeset! {1, 2, 3, 5},
            btreeset! {1, 2, 4, 5},
            btreeset! {1, 3, 4, 5},
            btreeset! {2, 3, 4, 5},
            btreeset! {1, 2, 3, 6},
            btreeset! {1, 2, 4, 6},
            btreeset! {1, 3, 4, 6},
            btreeset! {2, 3, 4, 6},
            btreeset! {1, 2, 5, 6},
            btreeset! {1, 3, 5, 6},
            btreeset! {2, 3, 5, 6},
            btreeset! {1, 4, 5, 6},
            btreeset! {2, 4, 5, 6},
            btreeset! {3, 4, 5, 6},
        ],
    );
    exhaustive_b_tree_sets_fixed_length_helper(
        2,
        exhaustive_ordered_unique_vecs_fixed_length(2, exhaustive_unsigneds::<u8>()),
        &[
            btreeset! {vec![0, 1], vec![0, 2]},
            btreeset! {vec![0, 1], vec![1, 2]},
            btreeset! {vec![0, 2], vec![1, 2]},
            btreeset! {vec![0, 1], vec![0, 3]},
            btreeset! {vec![0, 2], vec![0, 3]},
            btreeset! {vec![1, 2], vec![0, 3]},
            btreeset! {vec![0, 1], vec![1, 3]},
            btreeset! {vec![0, 2], vec![1, 3]},
            btreeset! {vec![1, 2], vec![1, 3]},
            btreeset! {vec![0, 3], vec![1, 3]},
            btreeset! {vec![0, 1], vec![2, 3]},
            btreeset! {vec![0, 2], vec![2, 3]},
            btreeset! {vec![1, 2], vec![2, 3]},
            btreeset! {vec![0, 3], vec![2, 3]},
            btreeset! {vec![1, 3], vec![2, 3]},
            btreeset! {vec![0, 1], vec![0, 4]},
            btreeset! {vec![0, 2], vec![0, 4]},
            btreeset! {vec![1, 2], vec![0, 4]},
            btreeset! {vec![0, 3], vec![0, 4]},
            btreeset! {vec![1, 3], vec![0, 4]},
        ],
    );
}
