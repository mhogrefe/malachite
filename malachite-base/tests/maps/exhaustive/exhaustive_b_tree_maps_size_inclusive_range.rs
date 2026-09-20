// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::maps::exhaustive::exhaustive_b_tree_maps_size_inclusive_range;
use malachite_base::test_util::maps::exhaustive::exhaustive_b_tree_maps_small_helper_helper;
use std::collections::BTreeMap;
use std::fmt::Debug;

fn exhaustive_b_tree_maps_size_inclusive_range_small_helper<
    I: Clone + Iterator,
    J: Clone + Iterator,
>(
    a: u64,
    b: u64,
    keys: I,
    values: J,
    out_len: usize,
    out: &[BTreeMap<I::Item, J::Item>],
) where
    I::Item: Clone + Debug + Ord,
    J::Item: Clone + Debug + Eq,
{
    exhaustive_b_tree_maps_small_helper_helper(
        exhaustive_b_tree_maps_size_inclusive_range(a, b, keys, values),
        out_len,
        out,
    );
}

#[test]
fn test_exhaustive_b_tree_maps_size_inclusive_range() {
    exhaustive_b_tree_maps_size_inclusive_range_small_helper(
        1,
        2,
        'a'..='b',
        0..2u8,
        8,
        &[
            btreemap! {'a' => 0},
            btreemap! {'b' => 0},
            btreemap! {'a' => 1},
            btreemap! {'a' => 0, 'b' => 0},
            btreemap! {'b' => 1},
            btreemap! {'a' => 0, 'b' => 1},
            btreemap! {'a' => 1, 'b' => 0},
            btreemap! {'a' => 1, 'b' => 1},
        ],
    );
    exhaustive_b_tree_maps_size_inclusive_range_small_helper(2, 1, 'a'..='b', 0..2u8, 0, &[]);
}
