// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::hash::Hash;
use itertools::{Itertools, repeat_n};
use malachite_base::maps::random::random_b_tree_maps_fixed_unique_value_count;
use malachite_base::num::random::{random_primitive_ints, random_unsigned_inclusive_range};
use malachite_base::random::{EXAMPLE_SEED, Seed};
use malachite_base::test_util::stats::common_values_map::common_values_map_debug;
use malachite_base::test_util::stats::median;
use std::collections::BTreeMap;
use std::fmt::Debug;

fn random_b_tree_maps_fixed_unique_value_count_helper<
    K: Clone + Debug + Eq + Hash + Ord,
    V: Clone + Debug + Eq + Hash + Ord,
    I: Clone + Iterator<Item = K>,
    J: Clone + Iterator<Item = V>,
>(
    unique_value_count: u64,
    keys_gen: &dyn Fn(Seed) -> I,
    values_gen: &dyn Fn(Seed) -> J,
    mean_size_numerator: u64,
    mean_size_denominator: u64,
    expected_values: &[BTreeMap<K, V>],
    expected_common_values: &[(BTreeMap<K, V>, usize)],
    expected_median: (BTreeMap<K, V>, Option<BTreeMap<K, V>>),
) {
    let xs = random_b_tree_maps_fixed_unique_value_count(
        EXAMPLE_SEED,
        unique_value_count,
        keys_gen,
        values_gen,
        mean_size_numerator,
        mean_size_denominator,
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
fn test_random_b_tree_maps_fixed_unique_value_count() {
    random_b_tree_maps_fixed_unique_value_count_helper(
        2,
        &random_primitive_ints::<u8>,
        &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, 9),
        4,
        1,
        &[
            btreemap! {79 => 4, 80 => 2},
            btreemap! {10 => 2, 51 => 0},
            btreemap! {59 => 0, 160 => 0, 245 => 0, 246 => 4, 253 => 4},
            btreemap! {53 => 7, 85 => 9},
            btreemap! {139 => 7, 214 => 2, 219 => 7},
            btreemap! {33 => 8, 120 => 8, 233 => 4},
            btreemap! {157 => 0, 158 => 0, 161 => 0, 202 => 0, 236 => 5},
            btreemap! {19 => 2, 72 => 2, 155 => 1},
            btreemap! {25 => 3, 74 => 9, 80 => 3, 153 => 9, 194 => 3, 236 => 9, 252 => 9},
            btreemap! {68 => 6, 97 => 6, 119 => 9},
            btreemap! {33 => 8, 55 => 3, 241 => 3},
            btreemap! {13 => 6, 32 => 6, 69 => 4, 216 => 6, 224 => 4},
            btreemap! {8 => 6, 91 => 6, 170 => 0},
            btreemap! {127 => 6, 134 => 0, 188 => 6, 196 => 0},
            btreemap! {108 => 8, 155 => 0},
            btreemap! {30 => 8, 241 => 9},
            btreemap! {15 => 6, 16 => 6, 46 => 6, 86 => 6, 131 => 4, 152 => 4, 221 => 4},
            btreemap! {99 => 1, 196 => 1, 206 => 5},
            btreemap! {
                64 => 9, 73 => 6, 80 => 6, 81 => 9, 91 => 6, 102 => 9, 115 => 9, 122 => 6, 143 => 9,
                153 => 9, 163 => 6, 199 => 6
            },
            btreemap! {41 => 1, 141 => 1, 143 => 4},
        ],
        &[
            (btreemap! {5 => 2, 45 => 7}, 4),
            (btreemap! {13 => 6, 43 => 8}, 4),
            (btreemap! {15 => 4, 97 => 5}, 4),
            (btreemap! {20 => 6, 72 => 3}, 4),
            (btreemap! {18 => 4, 117 => 3}, 4),
            (btreemap! {29 => 1, 136 => 0}, 4),
            (btreemap! {35 => 9, 244 => 5}, 4),
            (btreemap! {41 => 0, 208 => 5}, 4),
            (btreemap! {54 => 6, 121 => 5}, 4),
            (btreemap! {58 => 7, 174 => 1}, 4),
        ],
        (
            btreemap! {45 => 0, 163 => 3, 164 => 3, 231 => 0},
            Some(btreemap! {45 => 0, 163 => 4, 221 => 4}),
        ),
    );
    random_b_tree_maps_fixed_unique_value_count_helper(
        1,
        &random_primitive_ints::<u8>,
        &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, 2),
        2,
        1,
        &[
            btreemap! {79 => 0},
            btreemap! {80 => 1},
            btreemap! {10 => 2, 51 => 2, 59 => 2, 246 => 2, 253 => 2},
            btreemap! {160 => 0},
            btreemap! {53 => 0, 85 => 0, 245 => 0},
            btreemap! {139 => 0, 214 => 0, 219 => 0},
            btreemap! {120 => 2, 233 => 2},
            btreemap! {33 => 0, 158 => 0, 236 => 0},
            btreemap! {202 => 0},
            btreemap! {157 => 0},
            btreemap! {161 => 0},
            btreemap! {72 => 1},
            btreemap! {19 => 2, 155 => 2},
            btreemap! {194 => 1},
            btreemap! {80 => 2, 153 => 2, 236 => 2},
            btreemap! {25 => 1, 252 => 1},
            btreemap! {74 => 2},
            btreemap! {97 => 0, 119 => 0},
            btreemap! {68 => 1},
            btreemap! {33 => 0, 55 => 0, 241 => 0},
        ],
        &[
            (btreemap! {150 => 0}, 727),
            (btreemap! {244 => 0}, 725),
            (btreemap! {202 => 0}, 717),
            (btreemap! {38 => 0}, 712),
            (btreemap! {87 => 0}, 711),
            (btreemap! {72 => 1}, 709),
            (btreemap! {229 => 2}, 709),
            (btreemap! {12 => 1}, 707),
            (btreemap! {210 => 2}, 706),
            (btreemap! {99 => 0}, 705),
        ],
        (btreemap! {85 => 0, 115 => 0}, None),
    );
    random_b_tree_maps_fixed_unique_value_count_helper(
        0,
        &random_primitive_ints::<u8>,
        &random_primitive_ints::<u8>,
        1,
        1,
        &repeat_n(btreemap! {}, 20).collect_vec(),
        &[(btreemap! {}, 1000000)],
        (btreemap! {}, None),
    );
}

#[test]
#[should_panic]
fn random_b_tree_maps_fixed_unique_value_count_fail_1() {
    random_b_tree_maps_fixed_unique_value_count(
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
fn random_b_tree_maps_fixed_unique_value_count_fail_2() {
    random_b_tree_maps_fixed_unique_value_count(
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
fn random_b_tree_maps_fixed_unique_value_count_fail_3() {
    random_b_tree_maps_fixed_unique_value_count(
        EXAMPLE_SEED,
        2,
        &random_primitive_ints::<u32>,
        &random_primitive_ints::<u32>,
        2,
        1,
    );
}
