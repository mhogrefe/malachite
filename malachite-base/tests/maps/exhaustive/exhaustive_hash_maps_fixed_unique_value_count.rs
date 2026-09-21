// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::hash::Hash;
#[cfg(not(feature = "std"))]
use hashbrown::HashMap;
use malachite_base::maps::exhaustive::exhaustive_hash_maps_fixed_unique_value_count;
use malachite_base::test_util::maps::exhaustive::exhaustive_hash_maps_small_helper_helper;
#[cfg(feature = "std")]
use std::collections::HashMap;
use std::fmt::Debug;

fn exhaustive_hash_maps_fixed_unique_value_count_small_helper<
    I: Clone + Iterator,
    J: Clone + Iterator,
>(
    unique_value_count: u64,
    keys: I,
    values: J,
    out_len: usize,
    out: &[HashMap<I::Item, J::Item>],
) where
    I::Item: Clone + Debug + Eq + Hash,
    J::Item: Clone + Debug + Eq,
{
    exhaustive_hash_maps_small_helper_helper(
        exhaustive_hash_maps_fixed_unique_value_count(unique_value_count, keys, values),
        out_len,
        out,
    );
}

#[test]
fn test_exhaustive_hash_maps_fixed_unique_value_count() {
    exhaustive_hash_maps_fixed_unique_value_count_small_helper(
        0,
        'a'..='b',
        0..3u8,
        1,
        &[hashmap! {}],
    );
    exhaustive_hash_maps_fixed_unique_value_count_small_helper(
        1,
        'a'..='b',
        0..3u8,
        9,
        &[
            hashmap! {'a' => 0},
            hashmap! {'b' => 0},
            hashmap! {'a' => 1},
            hashmap! {'a' => 0, 'b' => 0},
            hashmap! {'a' => 2},
            hashmap! {'b' => 1},
            hashmap! {'b' => 2},
            hashmap! {'a' => 1, 'b' => 1},
            hashmap! {'a' => 2, 'b' => 2},
        ],
    );
    exhaustive_hash_maps_fixed_unique_value_count_small_helper(
        2,
        'a'..='c',
        0..3u8,
        36,
        &[
            hashmap! {'a' => 0, 'b' => 1},
            hashmap! {'a' => 0, 'c' => 1},
            hashmap! {'a' => 1, 'b' => 0},
            hashmap! {'b' => 0, 'c' => 1},
            hashmap! {'a' => 0, 'b' => 2},
            hashmap! {'a' => 1, 'c' => 0},
            hashmap! {'a' => 2, 'b' => 0},
            hashmap! {'a' => 0, 'b' => 0, 'c' => 1},
            hashmap! {'a' => 1, 'b' => 2},
            hashmap! {'a' => 0, 'c' => 2},
            hashmap! {'a' => 2, 'b' => 1},
            hashmap! {'b' => 1, 'c' => 0},
            hashmap! {'a' => 2, 'c' => 0},
            hashmap! {'b' => 0, 'c' => 2},
            hashmap! {'a' => 1, 'c' => 2},
            hashmap! {'b' => 2, 'c' => 0},
            hashmap! {'a' => 2, 'c' => 1},
            hashmap! {'b' => 1, 'c' => 2},
            hashmap! {'b' => 2, 'c' => 1},
            hashmap! {'a' => 1, 'b' => 1, 'c' => 0},
        ],
    );
}
