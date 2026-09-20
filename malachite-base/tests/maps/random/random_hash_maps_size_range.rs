// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::hash::Hash;
use itertools::Itertools;
use malachite_base::bools::random::random_bools;
use malachite_base::chars::random::random_char_inclusive_range;
use malachite_base::maps::random::random_hash_maps_size_range;
use malachite_base::num::random::{random_primitive_ints, random_unsigned_inclusive_range};
use malachite_base::random::{EXAMPLE_SEED, Seed};
use std::collections::HashMap;
use std::fmt::Debug;

fn random_hash_maps_size_range_helper<
    K: Clone + Debug + Eq + Hash,
    V: Clone + Debug + Eq,
    I: Clone + Iterator<Item = K>,
    J: Clone + Iterator<Item = V>,
>(
    a: u64,
    b: u64,
    keys_gen: &dyn Fn(Seed) -> I,
    values_gen: &dyn Fn(Seed) -> J,
    expected_values: &[HashMap<K, V>],
) {
    let xs = random_hash_maps_size_range(EXAMPLE_SEED, a, b, keys_gen, values_gen);
    let values = xs.take(20).collect_vec();
    assert_eq!(values.as_slice(), expected_values);
}

#[test]
fn test_random_hash_maps_size_range() {
    random_hash_maps_size_range_helper(
        2,
        4,
        &random_primitive_ints::<u8>,
        &random_primitive_ints::<u8>,
        &[
            hashmap! {79 => 36, 80 => 32},
            hashmap! {10 => 158, 51 => 64},
            hashmap! {59 => 39, 246 => 8, 253 => 71},
            hashmap! {85 => 220, 160 => 91, 245 => 18},
            hashmap! {53 => 243, 214 => 153, 219 => 134},
            hashmap! {120 => 6, 139 => 67, 233 => 107},
            hashmap! {33 => 14, 158 => 198},
            hashmap! {202 => 202, 236 => 14},
            hashmap! {72 => 72, 157 => 168, 161 => 185},
            hashmap! {19 => 197, 155 => 166, 194 => 97},
            hashmap! {153 => 73, 236 => 27},
            hashmap! {25 => 167, 80 => 112, 252 => 53},
            hashmap! {74 => 234, 97 => 219, 119 => 127},
            hashmap! {33 => 13, 68 => 55},
            hashmap! {55 => 51, 69 => 202, 241 => 208},
            hashmap! {32 => 117, 216 => 181},
            hashmap! {13 => 149, 91 => 244, 224 => 68},
            hashmap! {8 => 213, 134 => 164, 170 => 39},
            hashmap! {127 => 165, 188 => 65},
            hashmap! {155 => 156, 196 => 248},
        ],
    );
    random_hash_maps_size_range_helper(
        1,
        3,
        &|seed| random_char_inclusive_range(seed, 'a', 'c'),
        &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, 1),
        &[
            hashmap! {'a' => 0},
            hashmap! {'b' => 0},
            hashmap! {'a' => 1, 'b' => 0},
            hashmap! {'a' => 1, 'b' => 0},
            hashmap! {'a' => 0, 'c' => 0},
            hashmap! {'a' => 0, 'c' => 0},
            hashmap! {'a' => 0},
            hashmap! {'c' => 0},
            hashmap! {'a' => 0, 'b' => 1},
            hashmap! {'b' => 0, 'c' => 0},
            hashmap! {'a' => 0},
            hashmap! {'a' => 0, 'c' => 0},
            hashmap! {'b' => 0, 'c' => 0},
            hashmap! {'b' => 0},
            hashmap! {'a' => 0, 'b' => 1},
            hashmap! {'c' => 0},
            hashmap! {'b' => 1, 'c' => 1},
            hashmap! {'b' => 1, 'c' => 1},
            hashmap! {'a' => 0},
            hashmap! {'c' => 0},
        ],
    );
    random_hash_maps_size_range_helper(
        0,
        2,
        &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, 5),
        &random_bools,
        &[
            hashmap! {},
            hashmap! {},
            hashmap! {1 => false},
            hashmap! {1 => false},
            hashmap! {0 => true},
            hashmap! {5 => false},
            hashmap! {},
            hashmap! {},
            hashmap! {4 => false},
            hashmap! {1 => true},
            hashmap! {},
            hashmap! {2 => false},
            hashmap! {1 => false},
            hashmap! {},
            hashmap! {4 => false},
            hashmap! {},
            hashmap! {5 => false},
            hashmap! {3 => false},
            hashmap! {},
            hashmap! {},
        ],
    );
}

#[test]
#[should_panic]
fn random_hash_maps_size_range_fail() {
    random_hash_maps_size_range(
        EXAMPLE_SEED,
        2,
        2,
        &random_primitive_ints::<u32>,
        &random_primitive_ints::<u32>,
    );
}
