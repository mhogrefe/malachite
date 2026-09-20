// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::hash::Hash;
use itertools::Itertools;
use malachite_base::maps::random::random_b_tree_maps_unique_value_count_range;
use malachite_base::num::random::{random_primitive_ints, random_unsigned_inclusive_range};
use malachite_base::random::{EXAMPLE_SEED, Seed};
use malachite_base::test_util::stats::common_values_map::common_values_map_debug;
use malachite_base::test_util::stats::median;
use std::collections::BTreeMap;
use std::fmt::Debug;

fn random_b_tree_maps_unique_value_count_range_helper<
    K: Clone + Debug + Eq + Hash + Ord,
    V: Clone + Debug + Eq + Hash + Ord,
    I: Clone + Iterator<Item = K>,
    J: Clone + Iterator<Item = V>,
>(
    a: u64,
    b: u64,
    keys_gen: &dyn Fn(Seed) -> I,
    values_gen: &dyn Fn(Seed) -> J,
    mean_size_numerator: u64,
    mean_size_denominator: u64,
    expected_values: &[BTreeMap<K, V>],
    expected_common_values: &[(BTreeMap<K, V>, usize)],
    expected_median: (BTreeMap<K, V>, Option<BTreeMap<K, V>>),
) {
    let xs = random_b_tree_maps_unique_value_count_range(
        EXAMPLE_SEED,
        a,
        b,
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
fn test_random_b_tree_maps_unique_value_count_range() {
    random_b_tree_maps_unique_value_count_range_helper(
        1,
        3,
        &random_primitive_ints::<u8>,
        &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, 9),
        4,
        1,
        &[
            btreemap! {79 => 4},
            btreemap! {10 => 2, 51 => 0, 80 => 2},
            btreemap! {59 => 2, 85 => 2, 160 => 2, 245 => 2, 246 => 2, 253 => 2},
            btreemap! {53 => 0},
            btreemap! {214 => 4, 219 => 9},
            btreemap! {120 => 7, 139 => 7, 233 => 2},
            btreemap! {33 => 7, 157 => 7, 158 => 7, 161 => 7, 202 => 7, 236 => 7},
            btreemap! {19 => 4, 72 => 4, 153 => 4, 155 => 4, 194 => 4},
            btreemap! {25 => 8, 68 => 8, 74 => 8, 80 => 8, 97 => 8, 119 => 8, 236 => 8, 252 => 8},
            btreemap! {33 => 0, 241 => 5},
            btreemap! {55 => 2, 69 => 2},
            btreemap! {13 => 3, 32 => 1, 91 => 3, 216 => 3, 224 => 1},
            btreemap! {8 => 6, 127 => 9, 134 => 6, 170 => 9},
            btreemap! {108 => 3, 155 => 8, 188 => 8, 196 => 3},
            btreemap! {241 => 4},
            btreemap! {30 => 6},
            btreemap! {
                15 => 6, 16 => 6, 46 => 6, 86 => 6, 99 => 6, 131 => 6, 152 => 6, 206 => 6, 221 => 6
            },
            btreemap! {102 => 0, 163 => 0, 196 => 0},
            btreemap! {
                41 => 6, 64 => 6, 73 => 6, 80 => 0, 81 => 6, 91 => 0, 115 => 0, 122 => 6, 141 => 6,
                143 => 6, 144 => 0, 153 => 0, 157 => 6, 199 => 0
            },
            btreemap! {78 => 0, 139 => 8},
        ],
        &[
            (btreemap! {65 => 9}, 133),
            (btreemap! {30 => 2}, 131),
            (btreemap! {222 => 0}, 128),
            (btreemap! {74 => 5}, 127),
            (btreemap! {204 => 7}, 127),
            (btreemap! {36 => 6}, 126),
            (btreemap! {187 => 1}, 126),
            (btreemap! {231 => 5}, 126),
            (btreemap! {5 => 4}, 125),
            (btreemap! {126 => 4}, 125),
        ],
        (
            btreemap! {50 => 9, 60 => 9, 111 => 3, 123 => 3, 201 => 3, 221 => 9, 229 => 3},
            Some(btreemap! {
                    50 => 9, 60 => 9, 115 => 9, 146 => 9, 164 => 9, 172 => 9, 207 => 9, 234 => 9,
                    241 => 9, 243 => 9
            }),
        ),
    );
    random_b_tree_maps_unique_value_count_range_helper(
        0,
        2,
        &random_primitive_ints::<u8>,
        &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, 2),
        2,
        1,
        &[
            btreemap! {},
            btreemap! {},
            btreemap! {51 => 0, 79 => 0, 80 => 0},
            btreemap! {},
            btreemap! {10 => 1},
            btreemap! {59 => 2},
            btreemap! {160 => 0, 246 => 0, 253 => 0},
            btreemap! {245 => 0},
            btreemap! {53 => 0, 85 => 0, 139 => 0, 214 => 0, 219 => 0},
            btreemap! {233 => 2},
            btreemap! {120 => 0},
            btreemap! {33 => 0, 158 => 0, 236 => 0},
            btreemap! {202 => 0},
            btreemap! {157 => 0, 161 => 0},
            btreemap! {},
            btreemap! {},
            btreemap! {19 => 1, 72 => 1, 153 => 1, 155 => 1, 194 => 1},
            btreemap! {236 => 2},
            btreemap! {
                25 => 1, 33 => 1, 55 => 1, 68 => 1, 74 => 1, 80 => 1, 97 => 1, 119 => 1, 241 => 1,
                252 => 1
            },
            btreemap! {69 => 2},
        ],
        &[
            (btreemap! {}, 333261),
            (btreemap! {209 => 2}, 337),
            (btreemap! {239 => 1}, 336),
            (btreemap! {168 => 2}, 335),
            (btreemap! {178 => 0}, 335),
            (btreemap! {44 => 2}, 333),
            (btreemap! {70 => 2}, 333),
            (btreemap! {202 => 2}, 333),
            (btreemap! {16 => 2}, 330),
            (btreemap! {34 => 2}, 330),
        ],
        (
            btreemap! {25 => 1, 69 => 1, 82 => 1, 134 => 1},
            Some(btreemap! {25 => 1, 69 => 1, 82 => 1, 160 => 1, 210 => 1, 229 => 1}),
        ),
    );
    random_b_tree_maps_unique_value_count_range_helper(
        2,
        4,
        &random_primitive_ints::<u8>,
        &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, 9),
        5,
        1,
        &[
            btreemap! {79 => 4, 80 => 2},
            btreemap! {10 => 2, 51 => 0, 59 => 4, 253 => 0},
            btreemap! {53 => 9, 85 => 7, 160 => 7, 214 => 2, 219 => 2, 245 => 2, 246 => 9},
            btreemap! {139 => 7, 233 => 4},
            btreemap! {33 => 0, 120 => 8, 158 => 0},
            btreemap! {157 => 1, 161 => 1, 202 => 2, 236 => 5},
            btreemap! {19 => 9, 72 => 3, 80 => 3, 153 => 3, 155 => 9, 194 => 3, 236 => 9},
            btreemap! {25 => 6, 68 => 6, 74 => 8, 97 => 9, 119 => 6, 252 => 9},
            btreemap! {
                13 => 6, 32 => 6, 33 => 3, 55 => 6, 69 => 6, 91 => 4, 216 => 4, 224 => 6, 241 => 4
            },
            btreemap! {8 => 0, 134 => 8, 170 => 6},
            btreemap! {127 => 9, 188 => 8, 196 => 4},
            btreemap! {30 => 6, 86 => 6, 108 => 5, 131 => 6, 155 => 6, 241 => 6},
            btreemap! {15 => 1, 16 => 1, 46 => 6, 152 => 9, 221 => 1},
            btreemap! {99 => 1, 102 => 1, 163 => 4, 196 => 0, 206 => 4},
            btreemap! {73 => 5, 153 => 7},
            btreemap! {122 => 7, 199 => 3},
            btreemap! {
                41 => 3, 64 => 3, 80 => 3, 81 => 3, 91 => 3, 115 => 3, 141 => 3, 143 => 7, 144 => 7,
                157 => 3
            },
            btreemap! {64 => 5, 78 => 0, 139 => 3, 188 => 5},
            btreemap! {
                13 => 5, 39 => 7, 73 => 7, 78 => 7, 84 => 5, 94 => 5, 104 => 5, 112 => 7, 126 => 5,
                133 => 7, 149 => 7, 151 => 7, 163 => 5, 220 => 7, 233 => 7
            },
            btreemap! {2 => 4, 102 => 9, 193 => 5},
        ],
        &[
            (btreemap! {57 => 3, 118 => 9}, 4),
            (btreemap! {1 => 2, 21 => 3}, 3),
            (btreemap! {2 => 4, 30 => 2}, 3),
            (btreemap! {2 => 9, 87 => 3}, 3),
            (btreemap! {3 => 6, 44 => 8}, 3),
            (btreemap! {4 => 3, 80 => 8}, 3),
            (btreemap! {4 => 7, 17 => 5}, 3),
            (btreemap! {4 => 9, 65 => 0}, 3),
            (btreemap! {0 => 3, 118 => 2}, 3),
            (btreemap! {0 => 3, 137 => 0}, 3),
        ],
        (
            btreemap! {37 => 9, 55 => 9, 81 => 7, 83 => 6, 86 => 6, 181 => 6, 218 => 7, 252 => 9},
            Some(btreemap! {37 => 9, 55 => 9, 85 => 0, 172 => 9, 188 => 9, 214 => 0}),
        ),
    );
}

#[test]
#[should_panic]
fn random_b_tree_maps_unique_value_count_range_fail_1() {
    random_b_tree_maps_unique_value_count_range(
        EXAMPLE_SEED,
        2,
        2,
        &random_primitive_ints::<u32>,
        &random_primitive_ints::<u32>,
        5,
        1,
    );
}

#[test]
#[should_panic]
fn random_b_tree_maps_unique_value_count_range_fail_2() {
    random_b_tree_maps_unique_value_count_range(
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
fn random_b_tree_maps_unique_value_count_range_fail_3() {
    random_b_tree_maps_unique_value_count_range(
        EXAMPLE_SEED,
        1,
        3,
        &random_primitive_ints::<u32>,
        &random_primitive_ints::<u32>,
        1,
        1,
    );
}
