// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::hash::Hash;
use itertools::Itertools;
use malachite_base::chars::random::random_char_inclusive_range;
use malachite_base::maps::random::random_hash_maps_size_and_unique_value_count_inclusive_range;
use malachite_base::num::random::{random_primitive_ints, random_unsigned_inclusive_range};
use malachite_base::random::{EXAMPLE_SEED, Seed};
use std::collections::HashMap;
use std::fmt::Debug;

fn random_hash_maps_size_and_unique_value_count_inclusive_range_helper<
    K: Clone + Debug + Eq + Hash,
    V: Clone + Debug + Eq + Hash,
    I: Clone + Iterator<Item = K>,
    J: Clone + Iterator<Item = V>,
>(
    size_a: u64,
    size_b: u64,
    unique_value_count_a: u64,
    unique_value_count_b: u64,
    keys_gen: &dyn Fn(Seed) -> I,
    values_gen: &dyn Fn(Seed) -> J,
    expected_values: &[HashMap<K, V>],
) {
    let xs = random_hash_maps_size_and_unique_value_count_inclusive_range(
        EXAMPLE_SEED,
        size_a,
        size_b,
        unique_value_count_a,
        unique_value_count_b,
        keys_gen,
        values_gen,
    );
    let values = xs.take(20).collect_vec();
    assert_eq!(values.as_slice(), expected_values);
}

#[test]
fn test_random_hash_maps_size_and_unique_value_count_inclusive_range() {
    random_hash_maps_size_and_unique_value_count_inclusive_range_helper(
        2,
        3,
        1,
        2,
        &|seed| random_char_inclusive_range(seed, 'a', 'c'),
        &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, 2),
        &[
            hashmap! {'a' => 0, 'b' => 1},
            hashmap! {'a' => 2, 'b' => 2},
            hashmap! {'a' => 0, 'b' => 0, 'c' => 0},
            hashmap! {'a' => 2, 'b' => 2, 'c' => 0},
            hashmap! {'a' => 0, 'b' => 0, 'c' => 0},
            hashmap! {'a' => 0, 'b' => 0, 'c' => 0},
            hashmap! {'a' => 0, 'b' => 0},
            hashmap! {'b' => 0, 'c' => 0},
            hashmap! {'a' => 1, 'b' => 1, 'c' => 2},
            hashmap! {'a' => 1, 'b' => 2, 'c' => 1},
            hashmap! {'b' => 2, 'c' => 1},
            hashmap! {'a' => 1, 'b' => 0, 'c' => 1},
            hashmap! {'a' => 0, 'b' => 0, 'c' => 1},
            hashmap! {'a' => 2, 'c' => 0},
            hashmap! {'a' => 0, 'b' => 0, 'c' => 0},
            hashmap! {'b' => 0, 'c' => 2},
            hashmap! {'a' => 1, 'b' => 1, 'c' => 1},
            hashmap! {'a' => 1, 'b' => 1, 'c' => 1},
            hashmap! {'b' => 0, 'c' => 2},
            hashmap! {'a' => 1, 'c' => 1},
        ],
    );
    random_hash_maps_size_and_unique_value_count_inclusive_range_helper(
        0,
        3,
        0,
        3,
        &|seed| random_char_inclusive_range(seed, 'a', 'c'),
        &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, 2),
        &[
            hashmap! {},
            hashmap! {'a' => 0, 'b' => 1, 'c' => 0},
            hashmap! {'a' => 2, 'b' => 2, 'c' => 2},
            hashmap! {},
            hashmap! {'a' => 0, 'b' => 0, 'c' => 0},
            hashmap! {'a' => 0, 'c' => 2},
            hashmap! {'c' => 0},
            hashmap! {'b' => 0},
            hashmap! {'a' => 1, 'b' => 0, 'c' => 2},
            hashmap! {},
            hashmap! {},
            hashmap! {'b' => 1, 'c' => 1},
            hashmap! {},
            hashmap! {'a' => 0, 'b' => 2, 'c' => 1},
            hashmap! {'b' => 1, 'c' => 1},
            hashmap! {},
            hashmap! {'c' => 0},
            hashmap! {'a' => 0, 'c' => 1},
            hashmap! {'c' => 2},
            hashmap! {'a' => 0, 'b' => 0, 'c' => 0},
        ],
    );
    random_hash_maps_size_and_unique_value_count_inclusive_range_helper(
        2,
        4,
        2,
        2,
        &random_primitive_ints::<u8>,
        &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, 9),
        &[
            hashmap! {79 => 4, 80 => 2},
            hashmap! {10 => 2, 51 => 0},
            hashmap! {59 => 0, 160 => 0, 246 => 4, 253 => 4},
            hashmap! {53 => 9, 85 => 7, 245 => 9},
            hashmap! {139 => 7, 214 => 2, 219 => 7},
            hashmap! {120 => 8, 233 => 4},
            hashmap! {33 => 0, 158 => 5},
            hashmap! {157 => 1, 161 => 2, 202 => 1, 236 => 2},
            hashmap! {72 => 3, 155 => 9},
            hashmap! {19 => 9, 153 => 9, 194 => 6, 236 => 9},
            hashmap! {80 => 8, 252 => 3},
            hashmap! {25 => 4, 74 => 6, 119 => 4},
            hashmap! {33 => 0, 68 => 0, 97 => 6, 241 => 6},
            hashmap! {32 => 6, 55 => 0, 69 => 6},
            hashmap! {216 => 0, 224 => 8},
            hashmap! {13 => 9, 91 => 8, 170 => 9},
            hashmap! {8 => 4, 134 => 6},
            hashmap! {127 => 5, 155 => 1, 188 => 1, 196 => 1},
            hashmap! {30 => 9, 108 => 6, 131 => 9, 241 => 9},
            hashmap! {46 => 4, 86 => 4, 152 => 1, 221 => 1},
        ],
    );
}

#[test]
#[should_panic]
fn random_hash_maps_size_and_unique_value_count_inclusive_range_fail_1() {
    random_hash_maps_size_and_unique_value_count_inclusive_range(
        EXAMPLE_SEED,
        3,
        2,
        1,
        1,
        &random_primitive_ints::<u32>,
        &random_primitive_ints::<u32>,
    );
}

#[test]
#[should_panic]
fn random_hash_maps_size_and_unique_value_count_inclusive_range_fail_2() {
    random_hash_maps_size_and_unique_value_count_inclusive_range(
        EXAMPLE_SEED,
        0,
        3,
        2,
        1,
        &random_primitive_ints::<u32>,
        &random_primitive_ints::<u32>,
    );
}

#[test]
#[should_panic]
fn random_hash_maps_size_and_unique_value_count_inclusive_range_fail_3() {
    random_hash_maps_size_and_unique_value_count_inclusive_range(
        EXAMPLE_SEED,
        0,
        0,
        1,
        2,
        &random_primitive_ints::<u32>,
        &random_primitive_ints::<u32>,
    );
}

#[test]
#[should_panic]
fn random_hash_maps_size_and_unique_value_count_inclusive_range_fail_4() {
    random_hash_maps_size_and_unique_value_count_inclusive_range(
        EXAMPLE_SEED,
        1,
        3,
        0,
        0,
        &random_primitive_ints::<u32>,
        &random_primitive_ints::<u32>,
    );
}
