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
use itertools::Itertools;
use malachite_base::bools::random::random_bools;
use malachite_base::maps::random::random_hash_maps;
use malachite_base::num::random::{random_primitive_ints, random_unsigned_inclusive_range};
use malachite_base::random::{EXAMPLE_SEED, Seed};
#[cfg(feature = "std")]
use std::collections::HashMap;
use std::fmt::Debug;

fn random_hash_maps_helper<
    K: Clone + Debug + Eq + Hash,
    V: Clone + Debug + Eq,
    I: Clone + Iterator<Item = K>,
    J: Clone + Iterator<Item = V>,
>(
    keys_gen: &dyn Fn(Seed) -> I,
    values_gen: &dyn Fn(Seed) -> J,
    mean_size_numerator: u64,
    mean_size_denominator: u64,
    expected_values: &[HashMap<K, V>],
) {
    let xs = random_hash_maps(
        EXAMPLE_SEED,
        keys_gen,
        values_gen,
        mean_size_numerator,
        mean_size_denominator,
    );
    let values = xs.take(20).collect_vec();
    assert_eq!(values.as_slice(), expected_values);
}

#[test]
fn test_random_hash_maps() {
    random_hash_maps_helper(
        &random_primitive_ints::<u8>,
        &random_primitive_ints::<u8>,
        4,
        1,
        &[
            hashmap! {79 => 36, 80 => 32},
            hashmap! {10 => 158, 51 => 64, 59 => 39, 246 => 8, 253 => 71},
            hashmap! {53 => 243, 85 => 220, 160 => 91, 214 => 153, 245 => 18},
            hashmap! {139 => 67, 219 => 134, 233 => 107},
            hashmap! {},
            hashmap! {120 => 6},
            hashmap! {
                19 => 197, 33 => 14, 72 => 72, 155 => 166, 157 => 168, 158 => 198, 161 => 185,
                202 => 202, 236 => 14
            },
            hashmap! {
                8 => 213, 13 => 149, 25 => 167, 32 => 117, 33 => 13, 55 => 51, 68 => 55, 69 => 202,
                74 => 234, 80 => 112, 91 => 244, 97 => 219, 119 => 127, 153 => 73, 170 => 39,
                194 => 97, 216 => 181, 224 => 68, 236 => 27, 241 => 208, 252 => 53
            },
            hashmap! {},
            hashmap! {},
            hashmap! {},
            hashmap! {},
            hashmap! {127 => 165, 134 => 164, 188 => 65, 196 => 248},
            hashmap! {
                15 => 119, 16 => 225, 30 => 145, 46 => 134, 73 => 39, 86 => 237, 99 => 145,
                102 => 88, 108 => 223, 131 => 236, 152 => 209, 153 => 182, 155 => 156, 163 => 159,
                196 => 73, 199 => 54, 206 => 98, 221 => 220, 241 => 72
            },
            hashmap! {},
            hashmap! {64 => 93, 80 => 99, 81 => 39, 91 => 176, 115 => 88, 122 => 127, 143 => 134},
            hashmap! {},
            hashmap! {41 => 250, 141 => 100, 143 => 245},
            hashmap! {
                39 => 18, 64 => 139, 78 => 140, 84 => 64, 94 => 79, 112 => 24, 126 => 28,
                139 => 148, 144 => 69, 157 => 201, 163 => 187, 188 => 60, 220 => 179
            },
            hashmap! {73 => 15, 233 => 106},
        ],
    );
    random_hash_maps_helper(
        &random_primitive_ints::<u8>,
        &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, 2),
        2,
        1,
        &[
            hashmap! {},
            hashmap! {},
            hashmap! {51 => 2, 79 => 0, 80 => 1},
            hashmap! {},
            hashmap! {10 => 0},
            hashmap! {59 => 0},
            hashmap! {160 => 0, 246 => 2, 253 => 0},
            hashmap! {245 => 0},
            hashmap! {53 => 0, 85 => 0, 139 => 1, 214 => 1, 219 => 2},
            hashmap! {233 => 2},
            hashmap! {120 => 1},
            hashmap! {33 => 2, 158 => 0, 236 => 1},
            hashmap! {202 => 0},
            hashmap! {157 => 1, 161 => 0},
            hashmap! {},
            hashmap! {},
            hashmap! {19 => 0, 72 => 2, 153 => 1, 155 => 0, 194 => 2},
            hashmap! {236 => 1},
            hashmap! {
                25 => 1, 33 => 1, 55 => 1, 68 => 0, 74 => 0, 80 => 2, 97 => 1, 119 => 0, 241 => 2,
                252 => 0
            },
            hashmap! {69 => 2},
        ],
    );
    random_hash_maps_helper(
        &|seed| random_unsigned_inclusive_range::<u32>(seed, 1, 100),
        &random_bools,
        1,
        1,
        &[
            hashmap! {},
            hashmap! {},
            hashmap! {33 => false, 78 => true, 80 => false, 82 => false},
            hashmap! {},
            hashmap! {40 => true, 49 => false},
            hashmap! {33 => false, 64 => false},
            hashmap! {88 => false},
            hashmap! {43 => false, 100 => false},
            hashmap! {},
            hashmap! {},
            hashmap! {},
            hashmap! {},
            hashmap! {70 => false},
            hashmap! {},
            hashmap! {6 => false, 74 => true},
            hashmap! {94 => false},
            hashmap! {},
            hashmap! {79 => false},
            hashmap! {},
            hashmap! {18 => false, 34 => false},
        ],
    );
}

#[test]
#[should_panic]
fn random_hash_maps_fail_1() {
    random_hash_maps(
        EXAMPLE_SEED,
        &random_primitive_ints::<u32>,
        &random_primitive_ints::<u32>,
        0,
        1,
    );
}

#[test]
#[should_panic]
fn random_hash_maps_fail_2() {
    random_hash_maps(
        EXAMPLE_SEED,
        &random_primitive_ints::<u32>,
        &random_primitive_ints::<u32>,
        1,
        0,
    );
}

#[test]
#[should_panic]
fn random_hash_maps_fail_3() {
    random_hash_maps(
        EXAMPLE_SEED,
        &random_primitive_ints::<u32>,
        &random_primitive_ints::<u32>,
        u64::MAX,
        u64::MAX - 1,
    );
}
