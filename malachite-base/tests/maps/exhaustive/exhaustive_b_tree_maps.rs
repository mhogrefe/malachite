// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::maps::exhaustive::exhaustive_b_tree_maps;
use malachite_base::nevers::nevers;
use malachite_base::test_util::maps::exhaustive::{
    exhaustive_b_tree_maps_helper_helper, exhaustive_b_tree_maps_small_helper_helper,
};
use malachite_base::tuples::exhaustive::exhaustive_units;
use std::collections::BTreeMap;
use std::fmt::Debug;

fn exhaustive_b_tree_maps_helper<I: Clone + Iterator, J: Clone + Iterator>(
    keys: I,
    values: J,
    out: &[BTreeMap<I::Item, J::Item>],
) where
    I::Item: Clone + Debug + Ord,
    J::Item: Clone + Debug + Eq,
{
    exhaustive_b_tree_maps_helper_helper(exhaustive_b_tree_maps(keys, values), out);
}

fn exhaustive_b_tree_maps_small_helper<I: Clone + Iterator, J: Clone + Iterator>(
    keys: I,
    values: J,
    out_len: usize,
    out: &[BTreeMap<I::Item, J::Item>],
) where
    I::Item: Clone + Debug + Ord,
    J::Item: Clone + Debug + Eq,
{
    exhaustive_b_tree_maps_small_helper_helper(exhaustive_b_tree_maps(keys, values), out_len, out);
}

#[test]
fn test_exhaustive_b_tree_maps() {
    exhaustive_b_tree_maps_small_helper(nevers(), 0..2u8, 1, &[btreemap! {}]);
    exhaustive_b_tree_maps_small_helper(
        exhaustive_units(),
        0..2u8,
        3,
        &[btreemap! {}, btreemap! {() => 0}, btreemap! {() => 1}],
    );
    exhaustive_b_tree_maps_small_helper(exhaustive_bools(), nevers(), 1, &[btreemap! {}]);
    exhaustive_b_tree_maps_small_helper(
        'a'..='b',
        0..2u8,
        9,
        &[
            btreemap! {},
            btreemap! {'a' => 0},
            btreemap! {'a' => 1},
            btreemap! {'a' => 0, 'b' => 0},
            btreemap! {'b' => 0},
            btreemap! {'a' => 0, 'b' => 1},
            btreemap! {'b' => 1},
            btreemap! {'a' => 1, 'b' => 0},
            btreemap! {'a' => 1, 'b' => 1},
        ],
    );
    exhaustive_b_tree_maps_small_helper(
        'a'..='c',
        0..3u8,
        64,
        &[
            btreemap! {},
            btreemap! {'a' => 0},
            btreemap! {'a' => 1},
            btreemap! {'a' => 0, 'b' => 0},
            btreemap! {'a' => 2},
            btreemap! {'b' => 0},
            btreemap! {'b' => 1},
            btreemap! {'a' => 0, 'c' => 0},
            btreemap! {'b' => 2},
            btreemap! {'a' => 0, 'b' => 1},
            btreemap! {'a' => 1, 'b' => 0},
            btreemap! {'a' => 0, 'c' => 1},
            btreemap! {'a' => 1, 'b' => 1},
            btreemap! {'c' => 0},
            btreemap! {'a' => 0, 'b' => 2},
            btreemap! {'a' => 0, 'b' => 0, 'c' => 0},
            btreemap! {'a' => 1, 'b' => 2},
            btreemap! {'c' => 1},
            btreemap! {'a' => 2, 'b' => 0},
            btreemap! {'a' => 1, 'c' => 0},
        ],
    );
    exhaustive_b_tree_maps_helper(
        'a'..,
        0u8..,
        &[
            btreemap! {},
            btreemap! {'a' => 0},
            btreemap! {'a' => 1},
            btreemap! {'a' => 0, 'b' => 0},
            btreemap! {'a' => 2},
            btreemap! {'b' => 0},
            btreemap! {'a' => 3},
            btreemap! {'c' => 0},
            btreemap! {'a' => 4},
            btreemap! {'b' => 1},
            btreemap! {'a' => 5},
            btreemap! {'a' => 0, 'b' => 1},
            btreemap! {'a' => 6},
            btreemap! {'b' => 2},
            btreemap! {'a' => 7},
            btreemap! {'a' => 0, 'c' => 0},
            btreemap! {'a' => 8},
            btreemap! {'b' => 3},
            btreemap! {'a' => 9},
            btreemap! {'a' => 1, 'b' => 0},
        ],
    );
}
