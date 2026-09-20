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
use malachite_base::maps::random::random_b_tree_maps_size_and_unique_value_count_inclusive_range;
use malachite_base::num::random::{random_primitive_ints, random_unsigned_inclusive_range};
use malachite_base::random::{EXAMPLE_SEED, Seed};
use malachite_base::test_util::stats::common_values_map::common_values_map_debug;
use malachite_base::test_util::stats::median;
use std::collections::BTreeMap;
use std::fmt::Debug;

fn random_b_tree_maps_size_and_unique_value_count_inclusive_range_helper<
    K: Clone + Debug + Eq + Hash + Ord,
    V: Clone + Debug + Eq + Hash + Ord,
    I: Clone + Iterator<Item = K>,
    J: Clone + Iterator<Item = V>,
>(
    size_a: u64,
    size_b: u64,
    unique_value_count_a: u64,
    unique_value_count_b: u64,
    keys_gen: &dyn Fn(Seed) -> I,
    values_gen: &dyn Fn(Seed) -> J,
    expected_values: &[BTreeMap<K, V>],
    expected_common_values: &[(BTreeMap<K, V>, usize)],
    expected_median: (BTreeMap<K, V>, Option<BTreeMap<K, V>>),
) {
    let xs = random_b_tree_maps_size_and_unique_value_count_inclusive_range(
        EXAMPLE_SEED,
        size_a,
        size_b,
        unique_value_count_a,
        unique_value_count_b,
        keys_gen,
        values_gen,
    );
    let values = xs.clone().take(20).collect_vec();
    let common_values = common_values_map_debug(1000000, 10, xs.clone());
    let median = median(xs.take(1000000));
    assert_eq!(
        (values.as_slice(), common_values.as_slice(), median),
        (expected_values, expected_common_values, expected_median)
    );
}

#[test]
fn test_random_b_tree_maps_size_and_unique_value_count_inclusive_range() {
    random_b_tree_maps_size_and_unique_value_count_inclusive_range_helper(
        2,
        3,
        1,
        2,
        &|seed| random_char_inclusive_range(seed, 'a', 'c'),
        &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, 2),
        &[
            btreemap! {'a' => 0, 'b' => 1},
            btreemap! {'a' => 2, 'b' => 2},
            btreemap! {'a' => 0, 'b' => 0, 'c' => 0},
            btreemap! {'a' => 2, 'b' => 2, 'c' => 0},
            btreemap! {'a' => 0, 'b' => 0, 'c' => 0},
            btreemap! {'a' => 0, 'b' => 0, 'c' => 0},
            btreemap! {'a' => 0, 'b' => 0},
            btreemap! {'b' => 0, 'c' => 0},
            btreemap! {'a' => 1, 'b' => 1, 'c' => 2},
            btreemap! {'a' => 1, 'b' => 2, 'c' => 1},
            btreemap! {'b' => 2, 'c' => 1},
            btreemap! {'a' => 1, 'b' => 0, 'c' => 1},
            btreemap! {'a' => 0, 'b' => 0, 'c' => 1},
            btreemap! {'a' => 2, 'c' => 0},
            btreemap! {'a' => 0, 'b' => 0, 'c' => 0},
            btreemap! {'b' => 0, 'c' => 2},
            btreemap! {'a' => 1, 'b' => 1, 'c' => 1},
            btreemap! {'a' => 1, 'b' => 1, 'c' => 1},
            btreemap! {'b' => 0, 'c' => 2},
            btreemap! {'a' => 1, 'c' => 1},
        ],
        &[
            (btreemap! {'a' => 2, 'b' => 2, 'c' => 2}, 83678),
            (btreemap! {'a' => 1, 'b' => 1, 'c' => 1}, 83371),
            (btreemap! {'a' => 0, 'b' => 0, 'c' => 0}, 83354),
            (btreemap! {'a' => 2, 'c' => 2}, 28063),
            (btreemap! {'b' => 0, 'c' => 0}, 28026),
            (btreemap! {'a' => 1, 'c' => 1}, 27883),
            (btreemap! {'b' => 1, 'c' => 1}, 27867),
            (btreemap! {'a' => 2, 'b' => 2}, 27826),
            (btreemap! {'a' => 0, 'b' => 0}, 27743),
            (btreemap! {'a' => 0, 'c' => 0}, 27652),
        ],
        (btreemap! {'a' => 1, 'c' => 0}, None),
    );
    random_b_tree_maps_size_and_unique_value_count_inclusive_range_helper(
        0,
        3,
        0,
        3,
        &|seed| random_char_inclusive_range(seed, 'a', 'c'),
        &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, 2),
        &[
            btreemap! {},
            btreemap! {'a' => 0, 'b' => 1, 'c' => 0},
            btreemap! {'a' => 2, 'b' => 2, 'c' => 2},
            btreemap! {},
            btreemap! {'a' => 0, 'b' => 0, 'c' => 0},
            btreemap! {'a' => 0, 'c' => 2},
            btreemap! {'c' => 0},
            btreemap! {'b' => 0},
            btreemap! {'a' => 1, 'b' => 0, 'c' => 2},
            btreemap! {},
            btreemap! {},
            btreemap! {'b' => 1, 'c' => 1},
            btreemap! {},
            btreemap! {'a' => 0, 'b' => 2, 'c' => 1},
            btreemap! {'b' => 1, 'c' => 1},
            btreemap! {},
            btreemap! {'c' => 0},
            btreemap! {'a' => 0, 'c' => 1},
            btreemap! {'c' => 2},
            btreemap! {'a' => 0, 'b' => 0, 'c' => 0},
        ],
        &[
            (btreemap! {}, 249831),
            (btreemap! {'c' => 0}, 28020),
            (btreemap! {'a' => 2, 'b' => 2, 'c' => 2}, 27966),
            (btreemap! {'b' => 1}, 27919),
            (btreemap! {'a' => 0}, 27881),
            (btreemap! {'b' => 0}, 27872),
            (btreemap! {'c' => 2}, 27858),
            (btreemap! {'a' => 1, 'b' => 1, 'c' => 1}, 27837),
            (btreemap! {'a' => 0, 'b' => 0, 'c' => 0}, 27830),
            (btreemap! {'c' => 1}, 27824),
        ],
        (btreemap! {'a' => 1, 'b' => 1, 'c' => 1}, None),
    );
    random_b_tree_maps_size_and_unique_value_count_inclusive_range_helper(
        2,
        4,
        2,
        2,
        &random_primitive_ints::<u8>,
        &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, 9),
        &[
            btreemap! {79 => 4, 80 => 2},
            btreemap! {10 => 2, 51 => 0},
            btreemap! {59 => 0, 160 => 0, 246 => 4, 253 => 4},
            btreemap! {53 => 9, 85 => 7, 245 => 9},
            btreemap! {139 => 7, 214 => 2, 219 => 7},
            btreemap! {120 => 8, 233 => 4},
            btreemap! {33 => 0, 158 => 5},
            btreemap! {157 => 1, 161 => 2, 202 => 1, 236 => 2},
            btreemap! {72 => 3, 155 => 9},
            btreemap! {19 => 9, 153 => 9, 194 => 6, 236 => 9},
            btreemap! {80 => 8, 252 => 3},
            btreemap! {25 => 4, 74 => 6, 119 => 4},
            btreemap! {33 => 0, 68 => 0, 97 => 6, 241 => 6},
            btreemap! {32 => 6, 55 => 0, 69 => 6},
            btreemap! {216 => 0, 224 => 8},
            btreemap! {13 => 9, 91 => 8, 170 => 9},
            btreemap! {8 => 4, 134 => 6},
            btreemap! {127 => 5, 155 => 1, 188 => 1, 196 => 1},
            btreemap! {30 => 9, 108 => 6, 131 => 9, 241 => 9},
            btreemap! {46 => 4, 86 => 4, 152 => 1, 221 => 1},
        ],
        &[
            (btreemap! {16 => 7, 35 => 8}, 4),
            (btreemap! {40 => 6, 59 => 0}, 4),
            (btreemap! {16 => 9, 116 => 7}, 4),
            (btreemap! {22 => 9, 241 => 8}, 4),
            (btreemap! {32 => 6, 155 => 5}, 4),
            (btreemap! {101 => 2, 165 => 4}, 4),
            (btreemap! {103 => 4, 120 => 7}, 4),
            (btreemap! {130 => 5, 196 => 0}, 4),
            (btreemap! {140 => 0, 233 => 6}, 4),
            (btreemap! {161 => 1, 213 => 9}, 4),
        ],
        (
            btreemap! {53 => 9, 67 => 9, 75 => 3, 81 => 3},
            Some(btreemap! {53 => 9, 67 => 9, 96 => 1}),
        ),
    );
}

#[test]
#[should_panic]
fn random_b_tree_maps_size_and_unique_value_count_inclusive_range_fail_1() {
    random_b_tree_maps_size_and_unique_value_count_inclusive_range(
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
fn random_b_tree_maps_size_and_unique_value_count_inclusive_range_fail_2() {
    random_b_tree_maps_size_and_unique_value_count_inclusive_range(
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
fn random_b_tree_maps_size_and_unique_value_count_inclusive_range_fail_3() {
    random_b_tree_maps_size_and_unique_value_count_inclusive_range(
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
fn random_b_tree_maps_size_and_unique_value_count_inclusive_range_fail_4() {
    random_b_tree_maps_size_and_unique_value_count_inclusive_range(
        EXAMPLE_SEED,
        1,
        3,
        0,
        0,
        &random_primitive_ints::<u32>,
        &random_primitive_ints::<u32>,
    );
}
