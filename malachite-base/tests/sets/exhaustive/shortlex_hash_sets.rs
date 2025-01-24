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
use malachite_base::sets::exhaustive::shortlex_hash_sets;
use malachite_base::test_util::sets::exhaustive::{
    exhaustive_hash_sets_helper_helper, exhaustive_hash_sets_small_helper_helper,
};
use malachite_base::tuples::exhaustive::exhaustive_units;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

fn shortlex_hash_sets_helper<I: Clone + Iterator>(xs: I, out: &[HashSet<I::Item>])
where
    I::Item: Clone + Debug + Eq + Hash,
{
    exhaustive_hash_sets_helper_helper(shortlex_hash_sets(xs).take(20), out);
}

fn shortlex_hash_sets_small_helper<I: Clone + Iterator>(
    xs: I,
    out_len: usize,
    out: &[HashSet<I::Item>],
) where
    I::Item: Clone + Debug + Eq + Hash,
{
    exhaustive_hash_sets_small_helper_helper(shortlex_hash_sets(xs), out_len, out);
}

#[test]
fn test_shortlex_hash_sets() {
    shortlex_hash_sets_small_helper(nevers(), 1, &[hashset! {}]);
    shortlex_hash_sets_small_helper(exhaustive_units(), 2, &[hashset! {}, hashset! {()}]);
    shortlex_hash_sets_small_helper(
        exhaustive_bools(),
        4,
        &[hashset! {}, hashset! {false}, hashset! {true}, hashset! {false, true}],
    );
    shortlex_hash_sets_small_helper(
        1..=6,
        64,
        &[
            hashset! {},
            hashset! {1},
            hashset! {2},
            hashset! {3},
            hashset! {4},
            hashset! {5},
            hashset! {6},
            hashset! {1, 2},
            hashset! {1, 3},
            hashset! {1, 4},
            hashset! {1, 5},
            hashset! {1, 6},
            hashset! {2, 3},
            hashset! {2, 4},
            hashset! {2, 5},
            hashset! {2, 6},
            hashset! {3, 4},
            hashset! {3, 5},
            hashset! {3, 6},
            hashset! {4, 5},
        ],
    );
    shortlex_hash_sets_small_helper(
        'a'..='c',
        8,
        &[
            hashset! {},
            hashset! {'a'},
            hashset! {'b'},
            hashset! {'c'},
            hashset! {'a', 'b'},
            hashset! {'a', 'c'},
            hashset! {'b', 'c'},
            hashset! {'a', 'b', 'c'},
        ],
    );
    shortlex_hash_sets_helper(
        exhaustive_ascii_chars(),
        &[
            hashset! {},
            hashset! {'a'},
            hashset! {'b'},
            hashset! {'c'},
            hashset! {'d'},
            hashset! {'e'},
            hashset! {'f'},
            hashset! {'g'},
            hashset! {'h'},
            hashset! {'i'},
            hashset! {'j'},
            hashset! {'k'},
            hashset! {'l'},
            hashset! {'m'},
            hashset! {'n'},
            hashset! {'o'},
            hashset! {'p'},
            hashset! {'q'},
            hashset! {'r'},
            hashset! {'s'},
        ],
    );
}
