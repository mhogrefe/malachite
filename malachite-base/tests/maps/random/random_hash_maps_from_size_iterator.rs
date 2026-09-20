// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::hash::Hash;
use itertools::Itertools;
use malachite_base::maps::random::random_hash_maps_from_size_iterator;
use malachite_base::num::random::geometric::geometric_random_unsigneds;
use malachite_base::num::random::{random_primitive_ints, random_unsigned_inclusive_range};
use malachite_base::random::{EXAMPLE_SEED, Seed};
use std::collections::HashMap;
use std::fmt::Debug;

fn random_hash_maps_from_size_iterator_helper<
    K: Clone + Debug + Eq + Hash,
    V: Clone + Debug + Eq,
    I: Clone + Iterator<Item = u64>,
    J: Clone + Iterator<Item = K>,
    L: Clone + Iterator<Item = V>,
>(
    sizes_gen: &dyn Fn(Seed) -> I,
    keys_gen: &dyn Fn(Seed) -> J,
    values_gen: &dyn Fn(Seed) -> L,
    expected_values: &[HashMap<K, V>],
) {
    let xs = random_hash_maps_from_size_iterator(EXAMPLE_SEED, sizes_gen, keys_gen, values_gen);
    let values = xs.take(20).collect_vec();
    assert_eq!(values.as_slice(), expected_values);
}

#[test]
fn test_random_hash_maps_from_size_iterator() {
    random_hash_maps_from_size_iterator_helper(
        &|seed| geometric_random_unsigneds::<u64>(seed, 2, 1),
        &random_primitive_ints::<u8>,
        &random_primitive_ints::<u8>,
        &[
            hashmap! {},
            hashmap! {},
            hashmap! {51 => 64, 79 => 36, 80 => 32},
            hashmap! {},
            hashmap! {10 => 158},
            hashmap! {59 => 39},
            hashmap! {160 => 91, 246 => 8, 253 => 71},
            hashmap! {245 => 18},
            hashmap! {53 => 243, 85 => 220, 139 => 67, 214 => 153, 219 => 134},
            hashmap! {233 => 107},
            hashmap! {120 => 6},
            hashmap! {33 => 14, 158 => 198, 236 => 14},
            hashmap! {202 => 202},
            hashmap! {157 => 168, 161 => 185},
            hashmap! {},
            hashmap! {},
            hashmap! {19 => 197, 72 => 72, 153 => 73, 155 => 166, 194 => 97},
            hashmap! {236 => 27},
            hashmap! {
                25 => 167, 33 => 13, 55 => 51, 68 => 55, 74 => 234, 80 => 112, 97 => 219,
                119 => 127, 241 => 208, 252 => 53
            },
            hashmap! {69 => 202},
        ],
    );
    random_hash_maps_from_size_iterator_helper(
        &|seed| geometric_random_unsigneds::<u64>(seed, 4, 1),
        &random_primitive_ints::<u8>,
        &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, 2),
        &[
            hashmap! {79 => 0, 80 => 1},
            hashmap! {10 => 0, 51 => 2, 59 => 0, 246 => 2, 253 => 0},
            hashmap! {53 => 0, 85 => 0, 160 => 0, 214 => 1, 245 => 0},
            hashmap! {139 => 1, 219 => 2, 233 => 2},
            hashmap! {},
            hashmap! {120 => 1},
            hashmap! {
                19 => 0, 33 => 2, 72 => 2, 155 => 0, 157 => 1, 158 => 0, 161 => 0, 202 => 0,
                236 => 1
            },
            hashmap! {
                8 => 1, 13 => 2, 25 => 1, 32 => 2, 33 => 1, 55 => 1, 68 => 0, 69 => 2, 74 => 0,
                80 => 2, 91 => 0, 97 => 1, 119 => 0, 153 => 1, 170 => 0, 194 => 2, 216 => 1,
                224 => 0, 236 => 1, 241 => 2, 252 => 0
            },
            hashmap! {},
            hashmap! {},
            hashmap! {},
            hashmap! {},
            hashmap! {127 => 2, 134 => 2, 188 => 1, 196 => 2},
            hashmap! {
                15 => 0, 16 => 2, 30 => 2, 46 => 1, 73 => 2, 86 => 0, 99 => 2, 102 => 0, 108 => 0,
                131 => 0, 152 => 0, 153 => 2, 155 => 1, 163 => 0, 196 => 2, 199 => 2, 206 => 0,
                221 => 2, 241 => 0
            },
            hashmap! {},
            hashmap! {64 => 2, 80 => 2, 81 => 0, 91 => 1, 115 => 0, 122 => 1, 143 => 2},
            hashmap! {},
            hashmap! {41 => 2, 141 => 1, 143 => 2},
            hashmap! {
                39 => 1, 64 => 1, 78 => 1, 84 => 1, 94 => 2, 112 => 2, 126 => 2, 139 => 0, 144 => 1,
                157 => 2, 163 => 1, 188 => 0, 220 => 0
            },
            hashmap! {73 => 1, 233 => 0},
        ],
    );
}
