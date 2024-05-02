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
use malachite_base::sets::exhaustive::exhaustive_hash_sets_fixed_length;
use malachite_base::test_util::sets::exhaustive::{
    exhaustive_hash_sets_helper_helper, exhaustive_hash_sets_small_helper_helper,
};
use malachite_base::tuples::exhaustive::exhaustive_units;
use malachite_base::vecs::exhaustive::exhaustive_ordered_unique_vecs_fixed_length;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

fn exhaustive_hash_sets_fixed_length_helper<I: Clone + Iterator>(
    len: u64,
    xs: I,
    out: &[HashSet<I::Item>],
) where
    I::Item: Clone + Debug + Eq + Hash,
{
    exhaustive_hash_sets_helper_helper(exhaustive_hash_sets_fixed_length(len, xs), out);
}

fn exhaustive_hash_sets_fixed_length_small_helper<I: Clone + Iterator>(
    len: u64,
    xs: I,
    out_len: usize,
    out: &[HashSet<I::Item>],
) where
    I::Item: Clone + Debug + Eq + Hash,
{
    exhaustive_hash_sets_small_helper_helper(
        exhaustive_hash_sets_fixed_length(len, xs),
        out_len,
        out,
    );
}

#[test]
fn test_exhaustive_hash_sets_fixed_length() {
    // This demonstrates that 0 ^ 0 == 1:
    exhaustive_hash_sets_fixed_length_small_helper(0, nevers(), 1, &[hashset! {}]);
    exhaustive_hash_sets_fixed_length_small_helper(1, nevers(), 0, &[]);
    exhaustive_hash_sets_fixed_length_small_helper(2, nevers(), 0, &[]);
    exhaustive_hash_sets_fixed_length_small_helper(5, nevers(), 0, &[]);
    exhaustive_hash_sets_fixed_length_small_helper(1, exhaustive_units(), 1, &[hashset! {()}]);
    exhaustive_hash_sets_fixed_length_small_helper(2, exhaustive_units(), 0, &[]);
    exhaustive_hash_sets_fixed_length_small_helper(5, exhaustive_units(), 0, &[]);
    exhaustive_hash_sets_fixed_length_small_helper(
        0,
        exhaustive_unsigneds::<u8>(),
        1,
        &[hashset! {}],
    );
    exhaustive_hash_sets_fixed_length_small_helper(
        1,
        exhaustive_unsigneds::<u8>(),
        256,
        &[
            hashset! {0},
            hashset! {1},
            hashset! {2},
            hashset! {3},
            hashset! {4},
            hashset! {5},
            hashset! {6},
            hashset! {7},
            hashset! {8},
            hashset! {9},
            hashset! {10},
            hashset! {11},
            hashset! {12},
            hashset! {13},
            hashset! {14},
            hashset! {15},
            hashset! {16},
            hashset! {17},
            hashset! {18},
            hashset! {19},
        ],
    );
    exhaustive_hash_sets_fixed_length_helper(
        1,
        exhaustive_unsigneds::<u64>(),
        &[
            hashset! {0},
            hashset! {1},
            hashset! {2},
            hashset! {3},
            hashset! {4},
            hashset! {5},
            hashset! {6},
            hashset! {7},
            hashset! {8},
            hashset! {9},
            hashset! {10},
            hashset! {11},
            hashset! {12},
            hashset! {13},
            hashset! {14},
            hashset! {15},
            hashset! {16},
            hashset! {17},
            hashset! {18},
            hashset! {19},
        ],
    );
    exhaustive_hash_sets_fixed_length_small_helper(
        2,
        exhaustive_unsigneds::<u8>(),
        32640,
        &[
            hashset! {0, 1},
            hashset! {0, 2},
            hashset! {1, 2},
            hashset! {0, 3},
            hashset! {1, 3},
            hashset! {2, 3},
            hashset! {0, 4},
            hashset! {1, 4},
            hashset! {2, 4},
            hashset! {3, 4},
            hashset! {0, 5},
            hashset! {1, 5},
            hashset! {2, 5},
            hashset! {3, 5},
            hashset! {4, 5},
            hashset! {0, 6},
            hashset! {1, 6},
            hashset! {2, 6},
            hashset! {3, 6},
            hashset! {4, 6},
        ],
    );
    exhaustive_hash_sets_fixed_length_helper(
        3,
        exhaustive_unsigneds::<u8>(),
        &[
            hashset! {0, 1, 2},
            hashset! {0, 1, 3},
            hashset! {0, 2, 3},
            hashset! {1, 2, 3},
            hashset! {0, 1, 4},
            hashset! {0, 2, 4},
            hashset! {1, 2, 4},
            hashset! {0, 3, 4},
            hashset! {1, 3, 4},
            hashset! {2, 3, 4},
            hashset! {0, 1, 5},
            hashset! {0, 2, 5},
            hashset! {1, 2, 5},
            hashset! {0, 3, 5},
            hashset! {1, 3, 5},
            hashset! {2, 3, 5},
            hashset! {0, 4, 5},
            hashset! {1, 4, 5},
            hashset! {2, 4, 5},
            hashset! {3, 4, 5},
        ],
    );
    exhaustive_hash_sets_fixed_length_small_helper(
        2,
        exhaustive_ascii_chars(),
        8128,
        &[
            hashset! {'a', 'b'},
            hashset! {'a', 'c'},
            hashset! {'b', 'c'},
            hashset! {'a', 'd'},
            hashset! {'b', 'd'},
            hashset! {'c', 'd'},
            hashset! {'a', 'e'},
            hashset! {'b', 'e'},
            hashset! {'c', 'e'},
            hashset! {'d', 'e'},
            hashset! {'a', 'f'},
            hashset! {'b', 'f'},
            hashset! {'c', 'f'},
            hashset! {'d', 'f'},
            hashset! {'e', 'f'},
            hashset! {'a', 'g'},
            hashset! {'b', 'g'},
            hashset! {'c', 'g'},
            hashset! {'d', 'g'},
            hashset! {'e', 'g'},
        ],
    );
    exhaustive_hash_sets_fixed_length_small_helper(
        1,
        exhaustive_bools(),
        2,
        &[hashset! {false}, hashset! {true}],
    );
    exhaustive_hash_sets_fixed_length_small_helper(
        2,
        exhaustive_bools(),
        1,
        &[hashset! {false, true}],
    );
    exhaustive_hash_sets_fixed_length_small_helper(4, exhaustive_bools(), 0, &[]);
    exhaustive_hash_sets_fixed_length_small_helper(
        4,
        1..=6,
        15,
        &[
            hashset! {1, 2, 3, 4},
            hashset! {1, 2, 3, 5},
            hashset! {1, 2, 4, 5},
            hashset! {1, 3, 4, 5},
            hashset! {2, 3, 4, 5},
            hashset! {1, 2, 3, 6},
            hashset! {1, 2, 4, 6},
            hashset! {1, 3, 4, 6},
            hashset! {2, 3, 4, 6},
            hashset! {1, 2, 5, 6},
            hashset! {1, 3, 5, 6},
            hashset! {2, 3, 5, 6},
            hashset! {1, 4, 5, 6},
            hashset! {2, 4, 5, 6},
            hashset! {3, 4, 5, 6},
        ],
    );
    exhaustive_hash_sets_fixed_length_helper(
        2,
        exhaustive_ordered_unique_vecs_fixed_length(2, exhaustive_unsigneds::<u8>()),
        &[
            hashset! {vec![0, 1], vec![0, 2]},
            hashset! {vec![0, 1], vec![1, 2]},
            hashset! {vec![0, 2], vec![1, 2]},
            hashset! {vec![0, 1], vec![0, 3]},
            hashset! {vec![0, 2], vec![0, 3]},
            hashset! {vec![1, 2], vec![0, 3]},
            hashset! {vec![0, 1], vec![1, 3]},
            hashset! {vec![0, 2], vec![1, 3]},
            hashset! {vec![1, 2], vec![1, 3]},
            hashset! {vec![0, 3], vec![1, 3]},
            hashset! {vec![0, 1], vec![2, 3]},
            hashset! {vec![0, 2], vec![2, 3]},
            hashset! {vec![1, 2], vec![2, 3]},
            hashset! {vec![0, 3], vec![2, 3]},
            hashset! {vec![1, 3], vec![2, 3]},
            hashset! {vec![0, 1], vec![0, 4]},
            hashset! {vec![0, 2], vec![0, 4]},
            hashset! {vec![1, 2], vec![0, 4]},
            hashset! {vec![0, 3], vec![0, 4]},
            hashset! {vec![1, 3], vec![0, 4]},
        ],
    );
}
