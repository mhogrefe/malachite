// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::hash::Hash;
use malachite_base::maps::exhaustive::*;
use malachite_base::test_util::maps::exhaustive::{
    exhaustive_hash_maps_helper_helper, exhaustive_hash_maps_small_helper_helper,
};
use std::collections::HashMap;
use std::fmt::Debug;

fn exhaustive_hash_maps_size_and_unique_value_count_inclusive_range_helper<
    I: Clone + Iterator,
    J: Clone + Iterator,
>(
    size_a: u64,
    size_b: u64,
    unique_value_count_a: u64,
    unique_value_count_b: u64,
    keys: I,
    values: J,
    out: &[HashMap<I::Item, J::Item>],
) where
    I::Item: Clone + Debug + Eq + Hash,
    J::Item: Clone + Debug + Eq,
{
    exhaustive_hash_maps_helper_helper(
        exhaustive_hash_maps_size_and_unique_value_count_inclusive_range(
            size_a,
            size_b,
            unique_value_count_a,
            unique_value_count_b,
            keys,
            values,
        ),
        out,
    );
}

fn exhaustive_hash_maps_size_and_unique_value_count_inclusive_range_small_helper<
    I: Clone + Iterator,
    J: Clone + Iterator,
>(
    size_a: u64,
    size_b: u64,
    unique_value_count_a: u64,
    unique_value_count_b: u64,
    keys: I,
    values: J,
    out_len: usize,
    out: &[HashMap<I::Item, J::Item>],
) where
    I::Item: Clone + Debug + Eq + Hash,
    J::Item: Clone + Debug + Eq,
{
    exhaustive_hash_maps_small_helper_helper(
        exhaustive_hash_maps_size_and_unique_value_count_inclusive_range(
            size_a,
            size_b,
            unique_value_count_a,
            unique_value_count_b,
            keys,
            values,
        ),
        out_len,
        out,
    );
}

#[test]
fn test_exhaustive_hash_maps_size_and_unique_value_count_inclusive_range() {
    exhaustive_hash_maps_size_and_unique_value_count_inclusive_range_small_helper(
        2,
        3,
        1,
        1,
        'a'..='c',
        0..3u8,
        12,
        &[
            hashmap! {'a' => 0, 'b' => 0},
            hashmap! {'a' => 0, 'c' => 0},
            hashmap! {'a' => 1, 'b' => 1},
            hashmap! {'b' => 0, 'c' => 0},
            hashmap! {'a' => 2, 'b' => 2},
            hashmap! {'a' => 1, 'c' => 1},
            hashmap! {'a' => 2, 'c' => 2},
            hashmap! {'a' => 0, 'b' => 0, 'c' => 0},
            hashmap! {'b' => 1, 'c' => 1},
            hashmap! {'a' => 1, 'b' => 1, 'c' => 1},
            hashmap! {'b' => 2, 'c' => 2},
            hashmap! {'a' => 2, 'b' => 2, 'c' => 2},
        ],
    );
    exhaustive_hash_maps_size_and_unique_value_count_inclusive_range_small_helper(
        0,
        2,
        1,
        2,
        'a'..='b',
        0..2u8,
        8,
        &[
            hashmap! {'a' => 0},
            hashmap! {'b' => 0},
            hashmap! {'a' => 1},
            hashmap! {'a' => 0, 'b' => 0},
            hashmap! {'b' => 1},
            hashmap! {'a' => 0, 'b' => 1},
            hashmap! {'a' => 1, 'b' => 1},
            hashmap! {'a' => 1, 'b' => 0},
        ],
    );
    exhaustive_hash_maps_size_and_unique_value_count_inclusive_range_helper(
        3,
        3,
        1,
        1,
        'a'..,
        0u8..,
        &[
            hashmap! {'a' => 0, 'b' => 0, 'c' => 0},
            hashmap! {'a' => 0, 'b' => 0, 'd' => 0},
            hashmap! {'a' => 1, 'b' => 1, 'c' => 1},
            hashmap! {'a' => 0, 'c' => 0, 'd' => 0},
            hashmap! {'a' => 2, 'b' => 2, 'c' => 2},
            hashmap! {'a' => 1, 'b' => 1, 'd' => 1},
            hashmap! {'a' => 3, 'b' => 3, 'c' => 3},
            hashmap! {'b' => 0, 'c' => 0, 'd' => 0},
            hashmap! {'a' => 4, 'b' => 4, 'c' => 4},
            hashmap! {'a' => 2, 'b' => 2, 'd' => 2},
            hashmap! {'a' => 5, 'b' => 5, 'c' => 5},
            hashmap! {'a' => 1, 'c' => 1, 'd' => 1},
            hashmap! {'a' => 6, 'b' => 6, 'c' => 6},
            hashmap! {'a' => 3, 'b' => 3, 'd' => 3},
            hashmap! {'a' => 7, 'b' => 7, 'c' => 7},
            hashmap! {'a' => 0, 'b' => 0, 'e' => 0},
            hashmap! {'a' => 8, 'b' => 8, 'c' => 8},
            hashmap! {'a' => 4, 'b' => 4, 'd' => 4},
            hashmap! {'a' => 9, 'b' => 9, 'c' => 9},
            hashmap! {'a' => 2, 'c' => 2, 'd' => 2},
        ],
    );
}
