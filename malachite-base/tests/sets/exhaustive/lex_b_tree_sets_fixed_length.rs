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
use malachite_base::sets::exhaustive::lex_b_tree_sets_fixed_length;
use malachite_base::test_util::sets::exhaustive::{
    exhaustive_b_tree_sets_helper_helper, exhaustive_b_tree_sets_small_helper_helper,
};
use malachite_base::tuples::exhaustive::exhaustive_units;
use malachite_base::vecs::exhaustive::lex_ordered_unique_vecs_fixed_length;
use std::collections::BTreeSet;
use std::fmt::Debug;

fn lex_b_tree_sets_fixed_length_helper<I: Iterator>(len: u64, xs: I, out: &[BTreeSet<I::Item>])
where
    I::Item: Clone + Debug + Eq + Ord,
{
    exhaustive_b_tree_sets_helper_helper(lex_b_tree_sets_fixed_length(len, xs), out);
}

fn lex_b_tree_sets_fixed_length_small_helper<I: Clone + Iterator>(
    len: u64,
    xs: I,
    out_len: usize,
    out: &[BTreeSet<I::Item>],
) where
    I::Item: Clone + Debug + Eq + Ord,
{
    exhaustive_b_tree_sets_small_helper_helper(lex_b_tree_sets_fixed_length(len, xs), out_len, out);
}

#[test]
fn test_lex_b_tree_sets_fixed_length() {
    lex_b_tree_sets_fixed_length_small_helper(0, nevers(), 1, &[btreeset! {}]);
    lex_b_tree_sets_fixed_length_small_helper(1, nevers(), 0, &[]);
    lex_b_tree_sets_fixed_length_small_helper(2, nevers(), 0, &[]);
    lex_b_tree_sets_fixed_length_small_helper(5, nevers(), 0, &[]);
    lex_b_tree_sets_fixed_length_small_helper(1, exhaustive_units(), 1, &[btreeset! {()}]);
    lex_b_tree_sets_fixed_length_small_helper(2, exhaustive_units(), 0, &[]);
    lex_b_tree_sets_fixed_length_small_helper(5, exhaustive_units(), 0, &[]);
    lex_b_tree_sets_fixed_length_small_helper(0, exhaustive_unsigneds::<u8>(), 1, &[btreeset! {}]);
    lex_b_tree_sets_fixed_length_small_helper(
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
    lex_b_tree_sets_fixed_length_helper(
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
    lex_b_tree_sets_fixed_length_small_helper(
        2,
        exhaustive_unsigneds::<u8>(),
        32640,
        &[
            btreeset! {0, 1},
            btreeset! {0, 2},
            btreeset! {0, 3},
            btreeset! {0, 4},
            btreeset! {0, 5},
            btreeset! {0, 6},
            btreeset! {0, 7},
            btreeset! {0, 8},
            btreeset! {0, 9},
            btreeset! {0, 10},
            btreeset! {0, 11},
            btreeset! {0, 12},
            btreeset! {0, 13},
            btreeset! {0, 14},
            btreeset! {0, 15},
            btreeset! {0, 16},
            btreeset! {0, 17},
            btreeset! {0, 18},
            btreeset! {0, 19},
            btreeset! {0, 20},
        ],
    );
    lex_b_tree_sets_fixed_length_helper(
        3,
        exhaustive_unsigneds::<u8>(),
        &[
            btreeset! {0, 1, 2},
            btreeset! {0, 1, 3},
            btreeset! {0, 1, 4},
            btreeset! {0, 1, 5},
            btreeset! {0, 1, 6},
            btreeset! {0, 1, 7},
            btreeset! {0, 1, 8},
            btreeset! {0, 1, 9},
            btreeset! {0, 1, 10},
            btreeset! {0, 1, 11},
            btreeset! {0, 1, 12},
            btreeset! {0, 1, 13},
            btreeset! {0, 1, 14},
            btreeset! {0, 1, 15},
            btreeset! {0, 1, 16},
            btreeset! {0, 1, 17},
            btreeset! {0, 1, 18},
            btreeset! {0, 1, 19},
            btreeset! {0, 1, 20},
            btreeset! {0, 1, 21},
        ],
    );
    lex_b_tree_sets_fixed_length_small_helper(
        2,
        exhaustive_ascii_chars(),
        8128,
        &[
            btreeset! {'a', 'b'},
            btreeset! {'a', 'c'},
            btreeset! {'a', 'd'},
            btreeset! {'a', 'e'},
            btreeset! {'a', 'f'},
            btreeset! {'a', 'g'},
            btreeset! {'a', 'h'},
            btreeset! {'a', 'i'},
            btreeset! {'a', 'j'},
            btreeset! {'a', 'k'},
            btreeset! {'a', 'l'},
            btreeset! {'a', 'm'},
            btreeset! {'a', 'n'},
            btreeset! {'a', 'o'},
            btreeset! {'a', 'p'},
            btreeset! {'a', 'q'},
            btreeset! {'a', 'r'},
            btreeset! {'a', 's'},
            btreeset! {'a', 't'},
            btreeset! {'a', 'u'},
        ],
    );
    lex_b_tree_sets_fixed_length_small_helper(
        1,
        exhaustive_bools(),
        2,
        &[btreeset! {false}, btreeset! {true}],
    );
    lex_b_tree_sets_fixed_length_small_helper(2, exhaustive_bools(), 1, &[btreeset! {false, true}]);
    lex_b_tree_sets_fixed_length_small_helper(4, exhaustive_bools(), 0, &[]);
    lex_b_tree_sets_fixed_length_small_helper(
        4,
        1..=6,
        15,
        &[
            btreeset! {1, 2, 3, 4},
            btreeset! {1, 2, 3, 5},
            btreeset! {1, 2, 3, 6},
            btreeset! {1, 2, 4, 5},
            btreeset! {1, 2, 4, 6},
            btreeset! {1, 2, 5, 6},
            btreeset! {1, 3, 4, 5},
            btreeset! {1, 3, 4, 6},
            btreeset! {1, 3, 5, 6},
            btreeset! {1, 4, 5, 6},
            btreeset! {2, 3, 4, 5},
            btreeset! {2, 3, 4, 6},
            btreeset! {2, 3, 5, 6},
            btreeset! {2, 4, 5, 6},
            btreeset! {3, 4, 5, 6},
        ],
    );
    lex_b_tree_sets_fixed_length_helper(
        2,
        lex_ordered_unique_vecs_fixed_length(2, exhaustive_unsigneds::<u8>()),
        &[
            btreeset! {vec![0, 1], vec![0, 2]},
            btreeset! {vec![0, 1], vec![0, 3]},
            btreeset! {vec![0, 1], vec![0, 4]},
            btreeset! {vec![0, 1], vec![0, 5]},
            btreeset! {vec![0, 1], vec![0, 6]},
            btreeset! {vec![0, 1], vec![0, 7]},
            btreeset! {vec![0, 1], vec![0, 8]},
            btreeset! {vec![0, 1], vec![0, 9]},
            btreeset! {vec![0, 1], vec![0, 10]},
            btreeset! {vec![0, 1], vec![0, 11]},
            btreeset! {vec![0, 1], vec![0, 12]},
            btreeset! {vec![0, 1], vec![0, 13]},
            btreeset! {vec![0, 1], vec![0, 14]},
            btreeset! {vec![0, 1], vec![0, 15]},
            btreeset! {vec![0, 1], vec![0, 16]},
            btreeset! {vec![0, 1], vec![0, 17]},
            btreeset! {vec![0, 1], vec![0, 18]},
            btreeset! {vec![0, 1], vec![0, 19]},
            btreeset! {vec![0, 1], vec![0, 20]},
            btreeset! {vec![0, 1], vec![0, 21]},
        ],
    );
}
