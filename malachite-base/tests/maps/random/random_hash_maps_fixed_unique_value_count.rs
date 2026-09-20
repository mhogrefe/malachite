// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::hash::Hash;
use itertools::{Itertools, repeat_n};
use malachite_base::maps::random::random_hash_maps_fixed_unique_value_count;
use malachite_base::num::random::{random_primitive_ints, random_unsigned_inclusive_range};
use malachite_base::random::{EXAMPLE_SEED, Seed};
use std::collections::HashMap;
use std::fmt::Debug;

fn random_hash_maps_fixed_unique_value_count_helper<
    K: Clone + Debug + Eq + Hash,
    V: Clone + Debug + Eq + Hash,
    I: Clone + Iterator<Item = K>,
    J: Clone + Iterator<Item = V>,
>(
    unique_value_count: u64,
    keys_gen: &dyn Fn(Seed) -> I,
    values_gen: &dyn Fn(Seed) -> J,
    mean_size_numerator: u64,
    mean_size_denominator: u64,
    expected_values: &[HashMap<K, V>],
) {
    let xs = random_hash_maps_fixed_unique_value_count(
        EXAMPLE_SEED,
        unique_value_count,
        keys_gen,
        values_gen,
        mean_size_numerator,
        mean_size_denominator,
    );
    let values = xs.take(20).collect_vec();
    assert_eq!(values.as_slice(), expected_values);
}

#[test]
fn test_random_hash_maps_fixed_unique_value_count() {
    random_hash_maps_fixed_unique_value_count_helper(
        2,
        &random_primitive_ints::<u8>,
        &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, 9),
        4,
        1,
        &[
            hashmap! {79 => 4, 80 => 2},
            hashmap! {10 => 2, 51 => 0},
            hashmap! {59 => 0, 160 => 0, 245 => 0, 246 => 4, 253 => 4},
            hashmap! {53 => 7, 85 => 9},
            hashmap! {139 => 7, 214 => 2, 219 => 7},
            hashmap! {33 => 8, 120 => 8, 233 => 4},
            hashmap! {157 => 0, 158 => 0, 161 => 0, 202 => 0, 236 => 5},
            hashmap! {19 => 2, 72 => 2, 155 => 1},
            hashmap! {25 => 3, 74 => 9, 80 => 3, 153 => 9, 194 => 3, 236 => 9, 252 => 9},
            hashmap! {68 => 6, 97 => 6, 119 => 9},
            hashmap! {33 => 8, 55 => 3, 241 => 3},
            hashmap! {13 => 6, 32 => 6, 69 => 4, 216 => 6, 224 => 4},
            hashmap! {8 => 6, 91 => 6, 170 => 0},
            hashmap! {127 => 6, 134 => 0, 188 => 6, 196 => 0},
            hashmap! {108 => 8, 155 => 0},
            hashmap! {30 => 8, 241 => 9},
            hashmap! {15 => 6, 16 => 6, 46 => 6, 86 => 6, 131 => 4, 152 => 4, 221 => 4},
            hashmap! {99 => 1, 196 => 1, 206 => 5},
            hashmap! {
                64 => 9, 73 => 6, 80 => 6, 81 => 9, 91 => 6, 102 => 9, 115 => 9, 122 => 6, 143 => 9,
                153 => 9, 163 => 6, 199 => 6
            },
            hashmap! {41 => 1, 141 => 1, 143 => 4},
        ],
    );
    random_hash_maps_fixed_unique_value_count_helper(
        1,
        &random_primitive_ints::<u8>,
        &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, 2),
        2,
        1,
        &[
            hashmap! {79 => 0},
            hashmap! {80 => 1},
            hashmap! {10 => 2, 51 => 2, 59 => 2, 246 => 2, 253 => 2},
            hashmap! {160 => 0},
            hashmap! {53 => 0, 85 => 0, 245 => 0},
            hashmap! {139 => 0, 214 => 0, 219 => 0},
            hashmap! {120 => 2, 233 => 2},
            hashmap! {33 => 0, 158 => 0, 236 => 0},
            hashmap! {202 => 0},
            hashmap! {157 => 0},
            hashmap! {161 => 0},
            hashmap! {72 => 1},
            hashmap! {19 => 2, 155 => 2},
            hashmap! {194 => 1},
            hashmap! {80 => 2, 153 => 2, 236 => 2},
            hashmap! {25 => 1, 252 => 1},
            hashmap! {74 => 2},
            hashmap! {97 => 0, 119 => 0},
            hashmap! {68 => 1},
            hashmap! {33 => 0, 55 => 0, 241 => 0},
        ],
    );
    random_hash_maps_fixed_unique_value_count_helper(
        0,
        &random_primitive_ints::<u8>,
        &random_primitive_ints::<u8>,
        1,
        1,
        &repeat_n(hashmap! {}, 20).collect_vec(),
    );
}

#[test]
#[should_panic]
fn random_hash_maps_fixed_unique_value_count_fail_1() {
    random_hash_maps_fixed_unique_value_count(
        EXAMPLE_SEED,
        0,
        &random_primitive_ints::<u32>,
        &random_primitive_ints::<u32>,
        0,
        1,
    );
}

#[test]
#[should_panic]
fn random_hash_maps_fixed_unique_value_count_fail_2() {
    random_hash_maps_fixed_unique_value_count(
        EXAMPLE_SEED,
        0,
        &random_primitive_ints::<u32>,
        &random_primitive_ints::<u32>,
        1,
        0,
    );
}

#[test]
#[should_panic]
fn random_hash_maps_fixed_unique_value_count_fail_3() {
    random_hash_maps_fixed_unique_value_count(
        EXAMPLE_SEED,
        2,
        &random_primitive_ints::<u32>,
        &random_primitive_ints::<u32>,
        2,
        1,
    );
}
