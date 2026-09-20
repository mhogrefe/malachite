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
use malachite_base::maps::random::random_b_tree_maps;
use malachite_base::num::random::{random_primitive_ints, random_unsigned_inclusive_range};
use malachite_base::random::{EXAMPLE_SEED, Seed};
use malachite_base::test_util::stats::common_values_map::common_values_map_debug;
use malachite_base::test_util::stats::median;
use std::collections::BTreeMap;
use std::fmt::Debug;

fn random_b_tree_maps_helper<
    K: Clone + Debug + Eq + Hash + Ord,
    V: Clone + Debug + Eq + Hash + Ord,
    I: Clone + Iterator<Item = K>,
    J: Clone + Iterator<Item = V>,
>(
    keys_gen: &dyn Fn(Seed) -> I,
    values_gen: &dyn Fn(Seed) -> J,
    mean_size_numerator: u64,
    mean_size_denominator: u64,
    expected_values: &[BTreeMap<K, V>],
    expected_common_values: &[(BTreeMap<K, V>, usize)],
    expected_median: (BTreeMap<K, V>, Option<BTreeMap<K, V>>),
) {
    let xs = random_b_tree_maps(
        EXAMPLE_SEED,
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
fn test_random_b_tree_maps() {
    random_b_tree_maps_helper(
        &random_primitive_ints::<u8>,
        &random_primitive_ints::<u8>,
        4,
        1,
        &[
            btreemap! {79 => 36, 80 => 32},
            btreemap! {10 => 158, 51 => 64, 59 => 39, 246 => 8, 253 => 71},
            btreemap! {53 => 243, 85 => 220, 160 => 91, 214 => 153, 245 => 18},
            btreemap! {139 => 67, 219 => 134, 233 => 107},
            btreemap! {},
            btreemap! {120 => 6},
            btreemap! {
                19 => 197, 33 => 14, 72 => 72, 155 => 166, 157 => 168, 158 => 198, 161 => 185,
                202 => 202, 236 => 14
            },
            btreemap! {
                8 => 213, 13 => 149, 25 => 167, 32 => 117, 33 => 13, 55 => 51, 68 => 55, 69 => 202,
                74 => 234, 80 => 112, 91 => 244, 97 => 219, 119 => 127, 153 => 73, 170 => 39,
                194 => 97, 216 => 181, 224 => 68, 236 => 27, 241 => 208, 252 => 53
            },
            btreemap! {},
            btreemap! {},
            btreemap! {},
            btreemap! {},
            btreemap! {127 => 165, 134 => 164, 188 => 65, 196 => 248},
            btreemap! {
                15 => 119, 16 => 225, 30 => 145, 46 => 134, 73 => 39, 86 => 237, 99 => 145,
                102 => 88, 108 => 223, 131 => 236, 152 => 209, 153 => 182, 155 => 156, 163 => 159,
                196 => 73, 199 => 54, 206 => 98, 221 => 220, 241 => 72
            },
            btreemap! {},
            btreemap! {64 => 93, 80 => 99, 81 => 39, 91 => 176, 115 => 88, 122 => 127, 143 => 134},
            btreemap! {},
            btreemap! {41 => 250, 141 => 100, 143 => 245},
            btreemap! {
                39 => 18, 64 => 139, 78 => 140, 84 => 64, 94 => 79, 112 => 24, 126 => 28,
                139 => 148, 144 => 69, 157 => 201, 163 => 187, 188 => 60, 220 => 179
            },
            btreemap! {73 => 15, 233 => 106},
        ],
        &[
            (btreemap! {}, 199938),
            (btreemap! {158 => 98}, 11),
            (btreemap! {71 => 3}, 10),
            (btreemap! {91 => 54}, 10),
            (btreemap! {102 => 21}, 10),
            (btreemap! {106 => 99}, 10),
            (btreemap! {18 => 149}, 10),
            (btreemap! {228 => 53}, 10),
            (btreemap! {26 => 184}, 10),
            (btreemap! {132 => 147}, 10),
        ],
        (
            btreemap! {27 => 39, 119 => 51, 149 => 64, 192 => 125},
            Some(btreemap! {
                    27 => 39, 127 => 60, 140 => 17, 191 => 12, 201 => 249, 203 => 86, 228 => 159,
                    254 => 234
            }),
        ),
    );
    random_b_tree_maps_helper(
        &random_primitive_ints::<u8>,
        &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, 2),
        2,
        1,
        &[
            btreemap! {},
            btreemap! {},
            btreemap! {51 => 2, 79 => 0, 80 => 1},
            btreemap! {},
            btreemap! {10 => 0},
            btreemap! {59 => 0},
            btreemap! {160 => 0, 246 => 2, 253 => 0},
            btreemap! {245 => 0},
            btreemap! {53 => 0, 85 => 0, 139 => 1, 214 => 1, 219 => 2},
            btreemap! {233 => 2},
            btreemap! {120 => 1},
            btreemap! {33 => 2, 158 => 0, 236 => 1},
            btreemap! {202 => 0},
            btreemap! {157 => 1, 161 => 0},
            btreemap! {},
            btreemap! {},
            btreemap! {19 => 0, 72 => 2, 153 => 1, 155 => 0, 194 => 2},
            btreemap! {236 => 1},
            btreemap! {
                25 => 1, 33 => 1, 55 => 1, 68 => 0, 74 => 0, 80 => 2, 97 => 1, 119 => 0, 241 => 2,
                252 => 0
            },
            btreemap! {69 => 2},
        ],
        &[
            (btreemap! {}, 333261),
            (btreemap! {52 => 0}, 344),
            (btreemap! {175 => 1}, 342),
            (btreemap! {114 => 0}, 339),
            (btreemap! {59 => 2}, 334),
            (btreemap! {147 => 1}, 330),
            (btreemap! {169 => 1}, 329),
            (btreemap! {22 => 1}, 328),
            (btreemap! {174 => 1}, 327),
            (btreemap! {75 => 2}, 326),
        ],
        (
            btreemap! {25 => 1, 70 => 2, 180 => 0, 191 => 0, 250 => 0},
            Some(btreemap! {25 => 1, 70 => 2, 240 => 1, 250 => 0}),
        ),
    );
    random_b_tree_maps_helper(
        &|seed| random_unsigned_inclusive_range::<u32>(seed, 1, 100),
        &random_bools,
        1,
        1,
        &[
            btreemap! {},
            btreemap! {},
            btreemap! {33 => false, 78 => true, 80 => false, 82 => false},
            btreemap! {},
            btreemap! {40 => true, 49 => false},
            btreemap! {33 => false, 64 => false},
            btreemap! {88 => false},
            btreemap! {43 => false, 100 => false},
            btreemap! {},
            btreemap! {},
            btreemap! {},
            btreemap! {},
            btreemap! {70 => false},
            btreemap! {},
            btreemap! {6 => false, 74 => true},
            btreemap! {94 => false},
            btreemap! {},
            btreemap! {79 => false},
            btreemap! {},
            btreemap! {18 => false, 34 => false},
        ],
        &[
            (btreemap! {}, 499459),
            (btreemap! {37 => false}, 1386),
            (btreemap! {95 => false}, 1320),
            (btreemap! {56 => true}, 1318),
            (btreemap! {39 => false}, 1318),
            (btreemap! {74 => true}, 1313),
            (btreemap! {64 => false}, 1313),
            (btreemap! {30 => false}, 1306),
            (btreemap! {22 => true}, 1304),
            (btreemap! {90 => true}, 1303),
        ],
        (btreemap! {1 => false}, None),
    );
}

#[test]
#[should_panic]
fn random_b_tree_maps_fail_1() {
    random_b_tree_maps(
        EXAMPLE_SEED,
        &random_primitive_ints::<u32>,
        &random_primitive_ints::<u32>,
        0,
        1,
    );
}

#[test]
#[should_panic]
fn random_b_tree_maps_fail_2() {
    random_b_tree_maps(
        EXAMPLE_SEED,
        &random_primitive_ints::<u32>,
        &random_primitive_ints::<u32>,
        1,
        0,
    );
}

#[test]
#[should_panic]
fn random_b_tree_maps_fail_3() {
    random_b_tree_maps(
        EXAMPLE_SEED,
        &random_primitive_ints::<u32>,
        &random_primitive_ints::<u32>,
        u64::MAX,
        u64::MAX - 1,
    );
}
