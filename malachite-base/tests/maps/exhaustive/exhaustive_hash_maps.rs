// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::hash::Hash;
use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::maps::exhaustive::exhaustive_hash_maps;
use malachite_base::nevers::nevers;
use malachite_base::test_util::maps::exhaustive::{
    exhaustive_hash_maps_helper_helper, exhaustive_hash_maps_small_helper_helper,
};
use malachite_base::tuples::exhaustive::exhaustive_units;
use std::collections::HashMap;
use std::fmt::Debug;

fn exhaustive_hash_maps_helper<I: Clone + Iterator, J: Clone + Iterator>(
    keys: I,
    values: J,
    out: &[HashMap<I::Item, J::Item>],
) where
    I::Item: Clone + Debug + Eq + Hash,
    J::Item: Clone + Debug + Eq,
{
    exhaustive_hash_maps_helper_helper(exhaustive_hash_maps(keys, values), out);
}

fn exhaustive_hash_maps_small_helper<I: Clone + Iterator, J: Clone + Iterator>(
    keys: I,
    values: J,
    out_len: usize,
    out: &[HashMap<I::Item, J::Item>],
) where
    I::Item: Clone + Debug + Eq + Hash,
    J::Item: Clone + Debug + Eq,
{
    exhaustive_hash_maps_small_helper_helper(exhaustive_hash_maps(keys, values), out_len, out);
}

#[test]
fn test_exhaustive_hash_maps() {
    exhaustive_hash_maps_small_helper(nevers(), 0..2u8, 1, &[hashmap! {}]);
    exhaustive_hash_maps_small_helper(
        exhaustive_units(),
        0..2u8,
        3,
        &[hashmap! {}, hashmap! {() => 0}, hashmap! {() => 1}],
    );
    exhaustive_hash_maps_small_helper(exhaustive_bools(), nevers(), 1, &[hashmap! {}]);
    exhaustive_hash_maps_small_helper(
        'a'..='b',
        0..2u8,
        9,
        &[
            hashmap! {},
            hashmap! {'a' => 0},
            hashmap! {'a' => 1},
            hashmap! {'a' => 0, 'b' => 0},
            hashmap! {'b' => 0},
            hashmap! {'a' => 0, 'b' => 1},
            hashmap! {'b' => 1},
            hashmap! {'a' => 1, 'b' => 0},
            hashmap! {'a' => 1, 'b' => 1},
        ],
    );
    exhaustive_hash_maps_small_helper(
        'a'..='c',
        0..3u8,
        64,
        &[
            hashmap! {},
            hashmap! {'a' => 0},
            hashmap! {'a' => 1},
            hashmap! {'a' => 0, 'b' => 0},
            hashmap! {'a' => 2},
            hashmap! {'b' => 0},
            hashmap! {'b' => 1},
            hashmap! {'a' => 0, 'c' => 0},
            hashmap! {'b' => 2},
            hashmap! {'a' => 0, 'b' => 1},
            hashmap! {'a' => 1, 'b' => 0},
            hashmap! {'a' => 0, 'c' => 1},
            hashmap! {'a' => 1, 'b' => 1},
            hashmap! {'c' => 0},
            hashmap! {'a' => 0, 'b' => 2},
            hashmap! {'a' => 0, 'b' => 0, 'c' => 0},
            hashmap! {'a' => 1, 'b' => 2},
            hashmap! {'c' => 1},
            hashmap! {'a' => 2, 'b' => 0},
            hashmap! {'a' => 1, 'c' => 0},
        ],
    );
    exhaustive_hash_maps_helper(
        'a'..,
        0u8..,
        &[
            hashmap! {},
            hashmap! {'a' => 0},
            hashmap! {'a' => 1},
            hashmap! {'a' => 0, 'b' => 0},
            hashmap! {'a' => 2},
            hashmap! {'b' => 0},
            hashmap! {'a' => 3},
            hashmap! {'c' => 0},
            hashmap! {'a' => 4},
            hashmap! {'b' => 1},
            hashmap! {'a' => 5},
            hashmap! {'a' => 0, 'b' => 1},
            hashmap! {'a' => 6},
            hashmap! {'b' => 2},
            hashmap! {'a' => 7},
            hashmap! {'a' => 0, 'c' => 0},
            hashmap! {'a' => 8},
            hashmap! {'b' => 3},
            hashmap! {'a' => 9},
            hashmap! {'a' => 1, 'b' => 0},
        ],
    );
}
