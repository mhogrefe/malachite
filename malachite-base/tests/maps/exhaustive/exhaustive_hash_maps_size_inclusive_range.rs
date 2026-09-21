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
use malachite_base::maps::exhaustive::exhaustive_hash_maps_size_inclusive_range;
use malachite_base::test_util::maps::exhaustive::exhaustive_hash_maps_small_helper_helper;
#[cfg(feature = "std")]
use std::collections::HashMap;
use std::fmt::Debug;

fn exhaustive_hash_maps_size_inclusive_range_small_helper<
    I: Clone + Iterator,
    J: Clone + Iterator,
>(
    a: u64,
    b: u64,
    keys: I,
    values: J,
    out_len: usize,
    out: &[HashMap<I::Item, J::Item>],
) where
    I::Item: Clone + Debug + Eq + Hash,
    J::Item: Clone + Debug + Eq,
{
    exhaustive_hash_maps_small_helper_helper(
        exhaustive_hash_maps_size_inclusive_range(a, b, keys, values),
        out_len,
        out,
    );
}

#[test]
fn test_exhaustive_hash_maps_size_inclusive_range() {
    exhaustive_hash_maps_size_inclusive_range_small_helper(
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
            hashmap! {'a' => 1, 'b' => 0},
            hashmap! {'a' => 1, 'b' => 1},
        ],
    );
    exhaustive_hash_maps_size_inclusive_range_small_helper(2, 1, 'a'..='b', 0..2u8, 0, &[]);
}
