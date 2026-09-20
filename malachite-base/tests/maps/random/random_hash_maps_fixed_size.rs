// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::hash::Hash;
use itertools::{Itertools, repeat_n};
use malachite_base::bools::random::random_bools;
use malachite_base::chars::random::random_char_inclusive_range;
use malachite_base::maps::random::random_hash_maps_fixed_size;
use malachite_base::num::random::{random_primitive_ints, random_unsigned_inclusive_range};
use malachite_base::random::EXAMPLE_SEED;
use std::collections::HashMap;
use std::fmt::Debug;

fn random_hash_maps_fixed_size_helper<
    K: Clone + Debug + Eq + Hash,
    V: Clone + Debug + Eq,
    I: Clone + Iterator<Item = K>,
    J: Clone + Iterator<Item = V>,
>(
    size: u64,
    keys: I,
    values: J,
    expected_values: &[HashMap<K, V>],
) {
    let xs = random_hash_maps_fixed_size(size, keys, values);
    let values = xs.take(20).collect_vec();
    assert_eq!(values.as_slice(), expected_values);
}

#[test]
fn test_random_hash_maps_fixed_size() {
    random_hash_maps_fixed_size_helper(
        0,
        random_primitive_ints::<u8>(EXAMPLE_SEED.fork("keys")),
        random_primitive_ints::<u8>(EXAMPLE_SEED.fork("values")),
        &repeat_n(hashmap! {}, 20).collect_vec(),
    );
    random_hash_maps_fixed_size_helper(
        1,
        random_bools(EXAMPLE_SEED.fork("keys")),
        random_bools(EXAMPLE_SEED.fork("values")),
        &[
            hashmap! {true => false},
            hashmap! {true => false},
            hashmap! {true => true},
            hashmap! {true => false},
            hashmap! {false => false},
            hashmap! {false => true},
            hashmap! {true => false},
            hashmap! {false => false},
            hashmap! {false => false},
            hashmap! {false => false},
            hashmap! {false => false},
            hashmap! {false => false},
            hashmap! {true => false},
            hashmap! {false => true},
            hashmap! {true => false},
            hashmap! {false => false},
            hashmap! {true => false},
            hashmap! {true => false},
            hashmap! {false => false},
            hashmap! {false => false},
        ],
    );
    random_hash_maps_fixed_size_helper(
        2,
        random_char_inclusive_range(EXAMPLE_SEED.fork("keys"), 'a', 'c'),
        random_unsigned_inclusive_range::<u8>(EXAMPLE_SEED.fork("values"), 0, 2),
        &[
            hashmap! {'a' => 0, 'b' => 1},
            hashmap! {'a' => 2, 'b' => 0},
            hashmap! {'a' => 0, 'b' => 0},
            hashmap! {'a' => 2, 'c' => 0},
            hashmap! {'a' => 0, 'c' => 0},
            hashmap! {'a' => 0, 'c' => 1},
            hashmap! {'a' => 2, 'b' => 1},
            hashmap! {'b' => 1, 'c' => 2},
            hashmap! {'a' => 2, 'c' => 0},
            hashmap! {'b' => 0, 'c' => 1},
            hashmap! {'a' => 0, 'b' => 1},
            hashmap! {'b' => 0, 'c' => 2},
            hashmap! {'b' => 0, 'c' => 2},
            hashmap! {'b' => 1, 'c' => 1},
            hashmap! {'a' => 2, 'c' => 0},
            hashmap! {'b' => 1, 'c' => 0},
            hashmap! {'a' => 1, 'c' => 0},
            hashmap! {'b' => 1, 'c' => 0},
            hashmap! {'a' => 1, 'b' => 2},
            hashmap! {'a' => 2, 'c' => 2},
        ],
    );
    random_hash_maps_fixed_size_helper(
        2,
        random_primitive_ints::<u8>(EXAMPLE_SEED.fork("keys")),
        random_primitive_ints::<u8>(EXAMPLE_SEED.fork("values")),
        &[
            hashmap! {79 => 36, 80 => 32},
            hashmap! {10 => 158, 51 => 64},
            hashmap! {59 => 39, 253 => 71},
            hashmap! {160 => 91, 246 => 8},
            hashmap! {85 => 220, 245 => 18},
            hashmap! {53 => 243, 214 => 153},
            hashmap! {139 => 67, 219 => 134},
            hashmap! {120 => 6, 233 => 107},
            hashmap! {33 => 14, 158 => 198},
            hashmap! {202 => 202, 236 => 14},
            hashmap! {157 => 168, 161 => 185},
            hashmap! {72 => 72, 155 => 166},
            hashmap! {19 => 197, 194 => 97},
            hashmap! {153 => 73, 236 => 27},
            hashmap! {80 => 112, 252 => 53},
            hashmap! {25 => 167, 74 => 234},
            hashmap! {97 => 219, 119 => 127},
            hashmap! {33 => 13, 68 => 55},
            hashmap! {55 => 51, 241 => 208},
            hashmap! {32 => 117, 69 => 202},
        ],
    );
}
