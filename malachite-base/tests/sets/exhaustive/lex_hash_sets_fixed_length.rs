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
use malachite_base::num::exhaustive::exhaustive_unsigneds;
use malachite_base::sets::exhaustive::lex_hash_sets_fixed_length;
use malachite_base::test_util::sets::exhaustive::{
    exhaustive_hash_sets_helper_helper, exhaustive_hash_sets_small_helper_helper,
};
use malachite_base::tuples::exhaustive::exhaustive_units;
use malachite_base::vecs::exhaustive::lex_ordered_unique_vecs_fixed_length;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

fn lex_hash_sets_fixed_length_helper<I: Iterator>(len: u64, xs: I, out: &[HashSet<I::Item>])
where
    I::Item: Clone + Debug + Eq + Hash,
{
    exhaustive_hash_sets_helper_helper(lex_hash_sets_fixed_length(len, xs), out);
}

fn lex_hash_sets_fixed_length_small_helper<I: Clone + Iterator>(
    len: u64,
    xs: I,
    out_len: usize,
    out: &[HashSet<I::Item>],
) where
    I::Item: Clone + Debug + Eq + Hash,
{
    exhaustive_hash_sets_small_helper_helper(lex_hash_sets_fixed_length(len, xs), out_len, out);
}

#[test]
fn test_lex_hash_sets_fixed_length() {
    lex_hash_sets_fixed_length_small_helper(0, nevers(), 1, &[hashset! {}]);
    lex_hash_sets_fixed_length_small_helper(1, nevers(), 0, &[]);
    lex_hash_sets_fixed_length_small_helper(2, nevers(), 0, &[]);
    lex_hash_sets_fixed_length_small_helper(5, nevers(), 0, &[]);
    lex_hash_sets_fixed_length_small_helper(1, exhaustive_units(), 1, &[hashset! {()}]);
    lex_hash_sets_fixed_length_small_helper(2, exhaustive_units(), 0, &[]);
    lex_hash_sets_fixed_length_small_helper(5, exhaustive_units(), 0, &[]);
    lex_hash_sets_fixed_length_small_helper(0, exhaustive_unsigneds::<u8>(), 1, &[hashset! {}]);
    lex_hash_sets_fixed_length_small_helper(
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
    lex_hash_sets_fixed_length_helper(
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
    lex_hash_sets_fixed_length_small_helper(
        2,
        exhaustive_unsigneds::<u8>(),
        32640,
        &[
            hashset! {0, 1},
            hashset! {0, 2},
            hashset! {0, 3},
            hashset! {0, 4},
            hashset! {0, 5},
            hashset! {0, 6},
            hashset! {0, 7},
            hashset! {0, 8},
            hashset! {0, 9},
            hashset! {0, 10},
            hashset! {0, 11},
            hashset! {0, 12},
            hashset! {0, 13},
            hashset! {0, 14},
            hashset! {0, 15},
            hashset! {0, 16},
            hashset! {0, 17},
            hashset! {0, 18},
            hashset! {0, 19},
            hashset! {0, 20},
        ],
    );
    lex_hash_sets_fixed_length_helper(
        3,
        exhaustive_unsigneds::<u8>(),
        &[
            hashset! {0, 1, 2},
            hashset! {0, 1, 3},
            hashset! {0, 1, 4},
            hashset! {0, 1, 5},
            hashset! {0, 1, 6},
            hashset! {0, 1, 7},
            hashset! {0, 1, 8},
            hashset! {0, 1, 9},
            hashset! {0, 1, 10},
            hashset! {0, 1, 11},
            hashset! {0, 1, 12},
            hashset! {0, 1, 13},
            hashset! {0, 1, 14},
            hashset! {0, 1, 15},
            hashset! {0, 1, 16},
            hashset! {0, 1, 17},
            hashset! {0, 1, 18},
            hashset! {0, 1, 19},
            hashset! {0, 1, 20},
            hashset! {0, 1, 21},
        ],
    );
    lex_hash_sets_fixed_length_small_helper(
        2,
        exhaustive_ascii_chars(),
        8128,
        &[
            hashset! {'a', 'b'},
            hashset! {'a', 'c'},
            hashset! {'a', 'd'},
            hashset! {'a', 'e'},
            hashset! {'a', 'f'},
            hashset! {'a', 'g'},
            hashset! {'a', 'h'},
            hashset! {'a', 'i'},
            hashset! {'a', 'j'},
            hashset! {'a', 'k'},
            hashset! {'a', 'l'},
            hashset! {'a', 'm'},
            hashset! {'a', 'n'},
            hashset! {'a', 'o'},
            hashset! {'a', 'p'},
            hashset! {'a', 'q'},
            hashset! {'a', 'r'},
            hashset! {'a', 's'},
            hashset! {'a', 't'},
            hashset! {'a', 'u'},
        ],
    );
    lex_hash_sets_fixed_length_small_helper(
        1,
        exhaustive_bools(),
        2,
        &[hashset! {false}, hashset! {true}],
    );
    lex_hash_sets_fixed_length_small_helper(2, exhaustive_bools(), 1, &[hashset! {false, true}]);
    lex_hash_sets_fixed_length_small_helper(4, exhaustive_bools(), 0, &[]);
    lex_hash_sets_fixed_length_small_helper(
        4,
        1..=6,
        15,
        &[
            hashset! {1, 2, 3, 4},
            hashset! {1, 2, 3, 5},
            hashset! {1, 2, 3, 6},
            hashset! {1, 2, 4, 5},
            hashset! {1, 2, 4, 6},
            hashset! {1, 2, 5, 6},
            hashset! {1, 3, 4, 5},
            hashset! {1, 3, 4, 6},
            hashset! {1, 3, 5, 6},
            hashset! {1, 4, 5, 6},
            hashset! {2, 3, 4, 5},
            hashset! {2, 3, 4, 6},
            hashset! {2, 3, 5, 6},
            hashset! {2, 4, 5, 6},
            hashset! {3, 4, 5, 6},
        ],
    );
    lex_hash_sets_fixed_length_helper(
        2,
        lex_ordered_unique_vecs_fixed_length(2, exhaustive_unsigneds::<u8>()),
        &[
            hashset! {vec![0, 1], vec![0, 2]},
            hashset! {vec![0, 1], vec![0, 3]},
            hashset! {vec![0, 1], vec![0, 4]},
            hashset! {vec![0, 1], vec![0, 5]},
            hashset! {vec![0, 1], vec![0, 6]},
            hashset! {vec![0, 1], vec![0, 7]},
            hashset! {vec![0, 1], vec![0, 8]},
            hashset! {vec![0, 1], vec![0, 9]},
            hashset! {vec![0, 1], vec![0, 10]},
            hashset! {vec![0, 1], vec![0, 11]},
            hashset! {vec![0, 1], vec![0, 12]},
            hashset! {vec![0, 1], vec![0, 13]},
            hashset! {vec![0, 1], vec![0, 14]},
            hashset! {vec![0, 1], vec![0, 15]},
            hashset! {vec![0, 1], vec![0, 16]},
            hashset! {vec![0, 1], vec![0, 17]},
            hashset! {vec![0, 1], vec![0, 18]},
            hashset! {vec![0, 1], vec![0, 19]},
            hashset! {vec![0, 1], vec![0, 20]},
            hashset! {vec![0, 1], vec![0, 21]},
        ],
    );
}
