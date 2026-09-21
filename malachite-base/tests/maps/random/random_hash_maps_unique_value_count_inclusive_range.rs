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
use itertools::{Itertools, repeat_n};
use malachite_base::maps::random::random_hash_maps_unique_value_count_inclusive_range;
use malachite_base::num::random::{random_primitive_ints, random_unsigned_inclusive_range};
use malachite_base::random::{EXAMPLE_SEED, Seed};
#[cfg(feature = "std")]
use std::collections::HashMap;
use std::fmt::Debug;

fn random_hash_maps_unique_value_count_inclusive_range_helper<
    K: Clone + Debug + Eq + Hash,
    V: Clone + Debug + Eq + Hash,
    I: Clone + Iterator<Item = K>,
    J: Clone + Iterator<Item = V>,
>(
    a: u64,
    b: u64,
    keys_gen: &dyn Fn(Seed) -> I,
    values_gen: &dyn Fn(Seed) -> J,
    mean_size_numerator: u64,
    mean_size_denominator: u64,
    expected_values: &[HashMap<K, V>],
) {
    let xs = random_hash_maps_unique_value_count_inclusive_range(
        EXAMPLE_SEED,
        a,
        b,
        keys_gen,
        values_gen,
        mean_size_numerator,
        mean_size_denominator,
    );
    let values = xs.take(20).collect_vec();
    assert_eq!(values.as_slice(), expected_values);
}

#[test]
fn test_random_hash_maps_unique_value_count_inclusive_range() {
    random_hash_maps_unique_value_count_inclusive_range_helper(
        1,
        2,
        &random_primitive_ints::<u8>,
        &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, 9),
        4,
        1,
        &[
            hashmap! {79 => 4},
            hashmap! {10 => 2, 51 => 0, 80 => 2},
            hashmap! {59 => 2, 85 => 2, 160 => 2, 245 => 2, 246 => 2, 253 => 2},
            hashmap! {53 => 0},
            hashmap! {214 => 4, 219 => 9},
            hashmap! {120 => 7, 139 => 7, 233 => 2},
            hashmap! {33 => 7, 157 => 7, 158 => 7, 161 => 7, 202 => 7, 236 => 7},
            hashmap! {19 => 4, 72 => 4, 153 => 4, 155 => 4, 194 => 4},
            hashmap! {25 => 8, 68 => 8, 74 => 8, 80 => 8, 97 => 8, 119 => 8, 236 => 8, 252 => 8},
            hashmap! {33 => 0, 241 => 5},
            hashmap! {55 => 2, 69 => 2},
            hashmap! {13 => 3, 32 => 1, 91 => 3, 216 => 3, 224 => 1},
            hashmap! {8 => 6, 127 => 9, 134 => 6, 170 => 9},
            hashmap! {108 => 3, 155 => 8, 188 => 8, 196 => 3},
            hashmap! {241 => 4},
            hashmap! {30 => 6},
            hashmap! {
                15 => 6, 16 => 6, 46 => 6, 86 => 6, 99 => 6, 131 => 6, 152 => 6, 206 => 6, 221 => 6
            },
            hashmap! {102 => 0, 163 => 0, 196 => 0},
            hashmap! {
                41 => 6, 64 => 6, 73 => 6, 80 => 0, 81 => 6, 91 => 0, 115 => 0, 122 => 6, 141 => 6,
                143 => 6, 144 => 0, 153 => 0, 157 => 6, 199 => 0
            },
            hashmap! {78 => 0, 139 => 8},
        ],
    );
    random_hash_maps_unique_value_count_inclusive_range_helper(
        0,
        1,
        &random_primitive_ints::<u8>,
        &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, 2),
        2,
        1,
        &[
            hashmap! {},
            hashmap! {},
            hashmap! {51 => 0, 79 => 0, 80 => 0},
            hashmap! {},
            hashmap! {10 => 1},
            hashmap! {59 => 2},
            hashmap! {160 => 0, 246 => 0, 253 => 0},
            hashmap! {245 => 0},
            hashmap! {53 => 0, 85 => 0, 139 => 0, 214 => 0, 219 => 0},
            hashmap! {233 => 2},
            hashmap! {120 => 0},
            hashmap! {33 => 0, 158 => 0, 236 => 0},
            hashmap! {202 => 0},
            hashmap! {157 => 0, 161 => 0},
            hashmap! {},
            hashmap! {},
            hashmap! {19 => 1, 72 => 1, 153 => 1, 155 => 1, 194 => 1},
            hashmap! {236 => 2},
            hashmap! {
                25 => 1, 33 => 1, 55 => 1, 68 => 1, 74 => 1, 80 => 1, 97 => 1, 119 => 1, 241 => 1,
                252 => 1
            },
            hashmap! {69 => 2},
        ],
    );
    random_hash_maps_unique_value_count_inclusive_range_helper(
        0,
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
fn random_hash_maps_unique_value_count_inclusive_range_fail_1() {
    random_hash_maps_unique_value_count_inclusive_range(
        EXAMPLE_SEED,
        2,
        1,
        &random_primitive_ints::<u32>,
        &random_primitive_ints::<u32>,
        5,
        1,
    );
}

#[test]
#[should_panic]
fn random_hash_maps_unique_value_count_inclusive_range_fail_2() {
    random_hash_maps_unique_value_count_inclusive_range(
        EXAMPLE_SEED,
        0,
        2,
        &random_primitive_ints::<u32>,
        &random_primitive_ints::<u32>,
        0,
        1,
    );
}

#[test]
#[should_panic]
fn random_hash_maps_unique_value_count_inclusive_range_fail_3() {
    random_hash_maps_unique_value_count_inclusive_range(
        EXAMPLE_SEED,
        2,
        3,
        &random_primitive_ints::<u32>,
        &random_primitive_ints::<u32>,
        2,
        1,
    );
}
