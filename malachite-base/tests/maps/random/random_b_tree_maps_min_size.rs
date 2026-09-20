// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::hash::Hash;
use itertools::Itertools;
use malachite_base::maps::random::random_b_tree_maps_min_size;
use malachite_base::num::random::{random_primitive_ints, random_unsigned_inclusive_range};
use malachite_base::random::{EXAMPLE_SEED, Seed};
use malachite_base::test_util::stats::common_values_map::common_values_map_debug;
use malachite_base::test_util::stats::median;
use std::collections::BTreeMap;
use std::fmt::Debug;

fn random_b_tree_maps_min_size_helper<
    K: Clone + Debug + Eq + Hash + Ord,
    V: Clone + Debug + Eq + Hash + Ord,
    I: Clone + Iterator<Item = K>,
    J: Clone + Iterator<Item = V>,
>(
    min_size: u64,
    keys_gen: &dyn Fn(Seed) -> I,
    values_gen: &dyn Fn(Seed) -> J,
    mean_size_numerator: u64,
    mean_size_denominator: u64,
    expected_values: &[BTreeMap<K, V>],
    expected_common_values: &[(BTreeMap<K, V>, usize)],
    expected_median: (BTreeMap<K, V>, Option<BTreeMap<K, V>>),
) {
    let xs = random_b_tree_maps_min_size(
        EXAMPLE_SEED,
        min_size,
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
fn test_random_b_tree_maps_min_size() {
    random_b_tree_maps_min_size_helper(
        2,
        &random_primitive_ints::<u8>,
        &random_primitive_ints::<u8>,
        6,
        1,
        &[
            btreemap! {10 => 158, 51 => 64, 79 => 36, 80 => 32},
            btreemap! {53 => 243, 59 => 39, 85 => 220, 160 => 91, 245 => 18, 246 => 8, 253 => 71},
            btreemap! {
                33 => 14, 120 => 6, 139 => 67, 158 => 198, 214 => 153, 219 => 134, 233 => 107
            },
            btreemap! {72 => 72, 157 => 168, 161 => 185, 202 => 202, 236 => 14},
            btreemap! {19 => 197, 155 => 166},
            btreemap! {153 => 73, 194 => 97, 236 => 27},
            btreemap! {
                25 => 167, 33 => 13, 55 => 51, 68 => 55, 69 => 202, 74 => 234, 80 => 112, 97 => 219,
                119 => 127, 241 => 208, 252 => 53
            },
            btreemap! {
                8 => 213, 13 => 149, 15 => 119, 16 => 225, 30 => 145, 32 => 117, 46 => 134,
                86 => 237, 91 => 244, 108 => 223, 127 => 165, 131 => 236, 134 => 164, 152 => 209,
                155 => 156, 170 => 39, 188 => 65, 196 => 248, 206 => 98, 216 => 181, 221 => 220,
                224 => 68, 241 => 72
            },
            btreemap! {99 => 145, 196 => 73},
            btreemap! {102 => 88, 163 => 159},
            btreemap! {73 => 39, 153 => 182},
            btreemap! {122 => 127, 199 => 54},
            btreemap! {64 => 93, 80 => 99, 81 => 39, 91 => 176, 115 => 88, 143 => 134},
            btreemap! {
                39 => 18, 41 => 250, 64 => 139, 73 => 15, 78 => 140, 84 => 64, 94 => 79, 104 => 37,
                112 => 24, 126 => 28, 133 => 34, 139 => 148, 141 => 100, 143 => 245, 144 => 69,
                149 => 225, 157 => 201, 163 => 187, 188 => 60, 220 => 179, 233 => 106
            },
            btreemap! {13 => 233, 151 => 99},
            btreemap! {
                2 => 78, 12 => 253, 61 => 247, 102 => 218, 127 => 36, 193 => 47, 203 => 189,
                231 => 174, 253 => 101
            },
            btreemap! {43 => 248, 182 => 142},
            btreemap! {71 => 8, 141 => 17, 232 => 12, 241 => 178, 253 => 111},
            btreemap! {
                2 => 4, 7 => 163, 31 => 254, 42 => 128, 51 => 231, 74 => 15, 93 => 79, 108 => 52,
                141 => 127, 142 => 72, 176 => 66, 197 => 201, 215 => 127, 231 => 134, 249 => 198
            },
            btreemap! {23 => 108, 104 => 143, 105 => 234, 227 => 212},
        ],
        &[
            (btreemap! {40 => 56, 218 => 49}, 2),
            (btreemap! {95 => 186, 247 => 68}, 2),
            (btreemap! {131 => 97, 136 => 204}, 2),
            (btreemap! {145 => 151, 245 => 39}, 2),
            (btreemap! {14 => 159, 243 => 248}, 2),
            (btreemap! {98 => 175, 141 => 204}, 2),
            (btreemap! {111 => 117, 117 => 231}, 2),
            (btreemap! {150 => 215, 169 => 216}, 2),
            (btreemap! {160 => 151, 194 => 238}, 2),
            (btreemap! {0 => 3, 20 => 0}, 1),
        ],
        (
            btreemap! {32 => 204, 97 => 61, 120 => 233},
            Some(btreemap! {32 => 204, 101 => 234, 109 => 48, 122 => 176, 154 => 108}),
        ),
    );
    random_b_tree_maps_min_size_helper(
        1,
        &random_primitive_ints::<u8>,
        &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, 2),
        4,
        1,
        &[
            btreemap! {79 => 0},
            btreemap! {10 => 0, 51 => 2, 80 => 1},
            btreemap! {59 => 0, 85 => 0, 160 => 0, 245 => 0, 246 => 2, 253 => 0},
            btreemap! {53 => 0},
            btreemap! {214 => 1, 219 => 2},
            btreemap! {120 => 1, 139 => 1, 233 => 2},
            btreemap! {33 => 2, 157 => 1, 158 => 0, 161 => 0, 202 => 0, 236 => 1},
            btreemap! {19 => 0, 72 => 2, 153 => 1, 155 => 0, 194 => 2},
            btreemap! {25 => 1, 68 => 0, 74 => 0, 80 => 2, 97 => 1, 119 => 0, 236 => 1, 252 => 0},
            btreemap! {33 => 1, 241 => 2},
            btreemap! {55 => 1, 69 => 2},
            btreemap! {13 => 2, 32 => 2, 91 => 0, 216 => 1, 224 => 0},
            btreemap! {8 => 1, 127 => 2, 134 => 2, 170 => 0},
            btreemap! {108 => 0, 155 => 1, 188 => 1, 196 => 2},
            btreemap! {241 => 0},
            btreemap! {30 => 2},
            btreemap! {
                15 => 0, 16 => 2, 46 => 1, 86 => 0, 99 => 2, 131 => 0, 152 => 0, 206 => 0, 221 => 2
            },
            btreemap! {102 => 0, 163 => 0, 196 => 2},
            btreemap! {
                41 => 1, 64 => 2, 73 => 2, 80 => 2, 81 => 0, 91 => 1, 115 => 0, 122 => 1, 141 => 2,
                143 => 2, 144 => 2, 153 => 2, 157 => 2, 199 => 2
            },
            btreemap! {78 => 1, 139 => 1},
        ],
        &[
            (btreemap! {20 => 1}, 377),
            (btreemap! {95 => 1}, 376),
            (btreemap! {115 => 2}, 376),
            (btreemap! {91 => 1}, 375),
            (btreemap! {231 => 2}, 374),
            (btreemap! {30 => 2}, 373),
            (btreemap! {44 => 1}, 373),
            (btreemap! {219 => 0}, 372),
            (btreemap! {204 => 1}, 370),
            (btreemap! {57 => 1}, 369),
        ],
        (
            btreemap! {50 => 2, 138 => 0, 183 => 1, 188 => 0},
            Some(btreemap! {50 => 2, 138 => 1}),
        ),
    );
    random_b_tree_maps_min_size_helper(
        0,
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
}

#[test]
#[should_panic]
fn random_b_tree_maps_min_size_fail_1() {
    random_b_tree_maps_min_size(
        EXAMPLE_SEED,
        3,
        &random_primitive_ints::<u32>,
        &random_primitive_ints::<u32>,
        3,
        1,
    );
}

#[test]
#[should_panic]
fn random_b_tree_maps_min_size_fail_2() {
    random_b_tree_maps_min_size(
        EXAMPLE_SEED,
        1,
        &random_primitive_ints::<u32>,
        &random_primitive_ints::<u32>,
        1,
        0,
    );
}

#[test]
#[should_panic]
fn random_b_tree_maps_min_size_fail_3() {
    random_b_tree_maps_min_size(
        EXAMPLE_SEED,
        0,
        &random_primitive_ints::<u32>,
        &random_primitive_ints::<u32>,
        u64::MAX,
        u64::MAX - 1,
    );
}
