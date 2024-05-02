// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::nevers::nevers;
use malachite_base::sets::exhaustive::shortlex_hash_sets_length_range;
use malachite_base::test_util::sets::exhaustive::exhaustive_hash_sets_small_helper_helper;
use malachite_base::tuples::exhaustive::exhaustive_units;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

fn shortlex_hash_sets_length_range_small_helper<I: Clone + Iterator>(
    a: u64,
    b: u64,
    xs: I,
    out_len: usize,
    out: &[HashSet<I::Item>],
) where
    I::Item: Clone + Debug + Eq + Hash,
{
    exhaustive_hash_sets_small_helper_helper(
        shortlex_hash_sets_length_range(a, b, xs),
        out_len,
        out,
    );
}

#[test]
fn test_shortlex_hash_sets_length_range() {
    shortlex_hash_sets_length_range_small_helper(0, 5, nevers(), 1, &[hashset! {}]);
    shortlex_hash_sets_length_range_small_helper(6, 10, nevers(), 0, &[]);
    shortlex_hash_sets_length_range_small_helper(
        0,
        5,
        exhaustive_units(),
        2,
        &[hashset! {}, hashset! {()}],
    );
    shortlex_hash_sets_length_range_small_helper(1, 0, exhaustive_bools(), 0, &[]);
    shortlex_hash_sets_length_range_small_helper(1, 1, exhaustive_bools(), 0, &[]);
    shortlex_hash_sets_length_range_small_helper(
        0,
        2,
        exhaustive_bools(),
        3,
        &[hashset! {}, hashset! {false}, hashset! {true}],
    );
    shortlex_hash_sets_length_range_small_helper(
        2,
        4,
        exhaustive_bools(),
        1,
        &[hashset! {false, true}],
    );
    shortlex_hash_sets_length_range_small_helper(
        1,
        2,
        'a'..='c',
        3,
        &[hashset! {'a'}, hashset! {'b'}, hashset! {'c'}],
    );
}
