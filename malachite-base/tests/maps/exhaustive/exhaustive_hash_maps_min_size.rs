// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::hash::Hash;
use malachite_base::maps::exhaustive::exhaustive_hash_maps_min_size;
use malachite_base::test_util::maps::exhaustive::exhaustive_hash_maps_small_helper_helper;
use std::collections::HashMap;
use std::fmt::Debug;

fn exhaustive_hash_maps_min_size_small_helper<I: Clone + Iterator, J: Clone + Iterator>(
    min_size: u64,
    keys: I,
    values: J,
    out_len: usize,
    out: &[HashMap<I::Item, J::Item>],
) where
    I::Item: Clone + Debug + Eq + Hash,
    J::Item: Clone + Debug + Eq,
{
    exhaustive_hash_maps_small_helper_helper(
        exhaustive_hash_maps_min_size(min_size, keys, values),
        out_len,
        out,
    );
}

#[test]
fn test_exhaustive_hash_maps_min_size() {
    exhaustive_hash_maps_min_size_small_helper(
        1,
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
            hashmap! {'a' => 1, 'b' => 0},
            hashmap! {'a' => 1, 'b' => 1},
        ],
    );
    exhaustive_hash_maps_min_size_small_helper(
        2,
        'a'..='c',
        0..2u8,
        20,
        &[
            hashmap! {'a' => 0, 'b' => 0},
            hashmap! {'a' => 0, 'c' => 0},
            hashmap! {'a' => 0, 'b' => 1},
            hashmap! {'b' => 0, 'c' => 0},
            hashmap! {'a' => 1, 'b' => 0},
            hashmap! {'a' => 0, 'c' => 1},
            hashmap! {'a' => 1, 'b' => 1},
            hashmap! {'a' => 0, 'b' => 0, 'c' => 0},
            hashmap! {'a' => 1, 'c' => 0},
            hashmap! {'b' => 0, 'c' => 1},
            hashmap! {'a' => 1, 'c' => 1},
            hashmap! {'a' => 0, 'b' => 0, 'c' => 1},
            hashmap! {'b' => 1, 'c' => 0},
            hashmap! {'a' => 0, 'b' => 1, 'c' => 0},
            hashmap! {'b' => 1, 'c' => 1},
            hashmap! {'a' => 0, 'b' => 1, 'c' => 1},
            hashmap! {'a' => 1, 'b' => 0, 'c' => 0},
            hashmap! {'a' => 1, 'b' => 0, 'c' => 1},
            hashmap! {'a' => 1, 'b' => 1, 'c' => 0},
            hashmap! {'a' => 1, 'b' => 1, 'c' => 1},
        ],
    );
}
