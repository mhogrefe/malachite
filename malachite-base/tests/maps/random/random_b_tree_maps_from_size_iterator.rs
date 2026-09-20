// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::hash::Hash;
use itertools::Itertools;
use malachite_base::maps::random::random_b_tree_maps_from_size_iterator;
use malachite_base::num::random::geometric::geometric_random_unsigneds;
use malachite_base::num::random::{random_primitive_ints, random_unsigned_inclusive_range};
use malachite_base::random::{EXAMPLE_SEED, Seed};
use malachite_base::test_util::stats::common_values_map::common_values_map_debug;
use malachite_base::test_util::stats::median;
use std::collections::BTreeMap;
use std::fmt::Debug;

fn random_b_tree_maps_from_size_iterator_helper<
    K: Clone + Debug + Eq + Hash + Ord,
    V: Clone + Debug + Eq + Hash + Ord,
    I: Clone + Iterator<Item = u64>,
    J: Clone + Iterator<Item = K>,
    L: Clone + Iterator<Item = V>,
>(
    sizes_gen: &dyn Fn(Seed) -> I,
    keys_gen: &dyn Fn(Seed) -> J,
    values_gen: &dyn Fn(Seed) -> L,
    expected_values: &[BTreeMap<K, V>],
    expected_common_values: &[(BTreeMap<K, V>, usize)],
    expected_median: (BTreeMap<K, V>, Option<BTreeMap<K, V>>),
) {
    let xs = random_b_tree_maps_from_size_iterator(EXAMPLE_SEED, sizes_gen, keys_gen, values_gen);
    let values = xs.clone().take(20).collect_vec();
    let common_values = common_values_map_debug(1000000, 10, xs.clone());
    let median = median(xs.take(1000000));
    assert_eq!(
        (values.as_slice(), common_values.as_slice(), median),
        (expected_values, expected_common_values, expected_median)
    );
}

#[test]
fn test_random_b_tree_maps_from_size_iterator() {
    random_b_tree_maps_from_size_iterator_helper(
        &|seed| geometric_random_unsigneds::<u64>(seed, 2, 1),
        &random_primitive_ints::<u8>,
        &random_primitive_ints::<u8>,
        &[
            btreemap! {},
            btreemap! {},
            btreemap! {51 => 64, 79 => 36, 80 => 32},
            btreemap! {},
            btreemap! {10 => 158},
            btreemap! {59 => 39},
            btreemap! {160 => 91, 246 => 8, 253 => 71},
            btreemap! {245 => 18},
            btreemap! {53 => 243, 85 => 220, 139 => 67, 214 => 153, 219 => 134},
            btreemap! {233 => 107},
            btreemap! {120 => 6},
            btreemap! {33 => 14, 158 => 198, 236 => 14},
            btreemap! {202 => 202},
            btreemap! {157 => 168, 161 => 185},
            btreemap! {},
            btreemap! {},
            btreemap! {19 => 197, 72 => 72, 153 => 73, 155 => 166, 194 => 97},
            btreemap! {236 => 27},
            btreemap! {
                25 => 167, 33 => 13, 55 => 51, 68 => 55, 74 => 234, 80 => 112, 97 => 219,
                119 => 127, 241 => 208, 252 => 53
            },
            btreemap! {69 => 202},
        ],
        &[
            (btreemap! {}, 333261),
            (btreemap! {22 => 51}, 13),
            (btreemap! {202 => 72}, 13),
            (btreemap! {172 => 221}, 13),
            (btreemap! {58 => 46}, 12),
            (btreemap! {180 => 73}, 12),
            (btreemap! {57 => 222}, 12),
            (btreemap! {70 => 207}, 12),
            (btreemap! {77 => 132}, 12),
            (btreemap! {92 => 250}, 12),
        ],
        (
            btreemap! {
                25 => 131, 44 => 140, 62 => 198, 83 => 104, 103 => 246, 177 => 9, 187 => 74,
                189 => 18, 192 => 24, 214 => 111, 239 => 154
            },
            Some(btreemap! {25 => 131, 52 => 233, 91 => 127, 211 => 12, 244 => 222}),
        ),
    );
    random_b_tree_maps_from_size_iterator_helper(
        &|seed| geometric_random_unsigneds::<u64>(seed, 4, 1),
        &random_primitive_ints::<u8>,
        &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, 2),
        &[
            btreemap! {79 => 0, 80 => 1},
            btreemap! {10 => 0, 51 => 2, 59 => 0, 246 => 2, 253 => 0},
            btreemap! {53 => 0, 85 => 0, 160 => 0, 214 => 1, 245 => 0},
            btreemap! {139 => 1, 219 => 2, 233 => 2},
            btreemap! {},
            btreemap! {120 => 1},
            btreemap! {
                19 => 0, 33 => 2, 72 => 2, 155 => 0, 157 => 1, 158 => 0, 161 => 0, 202 => 0,
                236 => 1
            },
            btreemap! {
                8 => 1, 13 => 2, 25 => 1, 32 => 2, 33 => 1, 55 => 1, 68 => 0, 69 => 2, 74 => 0,
                80 => 2, 91 => 0, 97 => 1, 119 => 0, 153 => 1, 170 => 0, 194 => 2, 216 => 1,
                224 => 0, 236 => 1, 241 => 2, 252 => 0
            },
            btreemap! {},
            btreemap! {},
            btreemap! {},
            btreemap! {},
            btreemap! {127 => 2, 134 => 2, 188 => 1, 196 => 2},
            btreemap! {
                15 => 0, 16 => 2, 30 => 2, 46 => 1, 73 => 2, 86 => 0, 99 => 2, 102 => 0, 108 => 0,
                131 => 0, 152 => 0, 153 => 2, 155 => 1, 163 => 0, 196 => 2, 199 => 2, 206 => 0,
                221 => 2, 241 => 0
            },
            btreemap! {},
            btreemap! {64 => 2, 80 => 2, 81 => 0, 91 => 1, 115 => 0, 122 => 1, 143 => 2},
            btreemap! {},
            btreemap! {41 => 2, 141 => 1, 143 => 2},
            btreemap! {
                39 => 1, 64 => 1, 78 => 1, 84 => 1, 94 => 2, 112 => 2, 126 => 2, 139 => 0, 144 => 1,
                157 => 2, 163 => 1, 188 => 0, 220 => 0
            },
            btreemap! {73 => 1, 233 => 0},
        ],
        &[
            (btreemap! {}, 199938),
            (btreemap! {19 => 1}, 256),
            (btreemap! {236 => 2}, 255),
            (btreemap! {64 => 2}, 248),
            (btreemap! {231 => 2}, 248),
            (btreemap! {9 => 1}, 246),
            (btreemap! {143 => 1}, 246),
            (btreemap! {7 => 2}, 245),
            (btreemap! {227 => 0}, 245),
            (btreemap! {25 => 1}, 244),
        ],
        (
            btreemap! {27 => 0, 53 => 1, 127 => 2, 139 => 2, 211 => 0, 252 => 2},
            Some(btreemap! {27 => 0, 53 => 1, 198 => 2}),
        ),
    );
}
