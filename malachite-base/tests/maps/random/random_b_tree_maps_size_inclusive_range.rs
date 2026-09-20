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
use malachite_base::chars::random::random_char_inclusive_range;
use malachite_base::maps::random::random_b_tree_maps_size_inclusive_range;
use malachite_base::num::random::{random_primitive_ints, random_unsigned_inclusive_range};
use malachite_base::random::{EXAMPLE_SEED, Seed};
use malachite_base::test_util::stats::common_values_map::common_values_map_debug;
use malachite_base::test_util::stats::median;
use std::collections::BTreeMap;
use std::fmt::Debug;

fn random_b_tree_maps_size_inclusive_range_helper<
    K: Clone + Debug + Eq + Hash + Ord,
    V: Clone + Debug + Eq + Hash + Ord,
    I: Clone + Iterator<Item = K>,
    J: Clone + Iterator<Item = V>,
>(
    a: u64,
    b: u64,
    keys_gen: &dyn Fn(Seed) -> I,
    values_gen: &dyn Fn(Seed) -> J,
    expected_values: &[BTreeMap<K, V>],
    expected_common_values: &[(BTreeMap<K, V>, usize)],
    expected_median: (BTreeMap<K, V>, Option<BTreeMap<K, V>>),
) {
    let xs = random_b_tree_maps_size_inclusive_range(EXAMPLE_SEED, a, b, keys_gen, values_gen);
    let values = xs.clone().take(20).collect_vec();
    let common_values = common_values_map_debug(1000000, 10, xs.clone());
    let median = median(xs.take(1000000));
    assert_eq!(
        (values.as_slice(), common_values.as_slice(), median),
        (expected_values, expected_common_values, expected_median)
    );
}

#[test]
fn test_random_b_tree_maps_size_inclusive_range() {
    random_b_tree_maps_size_inclusive_range_helper(
        2,
        3,
        &random_primitive_ints::<u8>,
        &random_primitive_ints::<u8>,
        &[
            btreemap! {79 => 36, 80 => 32},
            btreemap! {10 => 158, 51 => 64},
            btreemap! {59 => 39, 246 => 8, 253 => 71},
            btreemap! {85 => 220, 160 => 91, 245 => 18},
            btreemap! {53 => 243, 214 => 153, 219 => 134},
            btreemap! {120 => 6, 139 => 67, 233 => 107},
            btreemap! {33 => 14, 158 => 198},
            btreemap! {202 => 202, 236 => 14},
            btreemap! {72 => 72, 157 => 168, 161 => 185},
            btreemap! {19 => 197, 155 => 166, 194 => 97},
            btreemap! {153 => 73, 236 => 27},
            btreemap! {25 => 167, 80 => 112, 252 => 53},
            btreemap! {74 => 234, 97 => 219, 119 => 127},
            btreemap! {33 => 13, 68 => 55},
            btreemap! {55 => 51, 69 => 202, 241 => 208},
            btreemap! {32 => 117, 216 => 181},
            btreemap! {13 => 149, 91 => 244, 224 => 68},
            btreemap! {8 => 213, 134 => 164, 170 => 39},
            btreemap! {127 => 165, 188 => 65},
            btreemap! {155 => 156, 196 => 248},
        ],
        &[
            (btreemap! {23 => 35, 48 => 15}, 2),
            (btreemap! {39 => 108, 68 => 3}, 2),
            (btreemap! {65 => 42, 160 => 5}, 2),
            (btreemap! {70 => 3, 79 => 134}, 2),
            (btreemap! {82 => 4, 206 => 97}, 2),
            (btreemap! {93 => 0, 186 => 71}, 2),
            (btreemap! {26 => 98, 183 => 31}, 2),
            (btreemap! {2 => 50, 222 => 120}, 2),
            (btreemap! {3 => 112, 177 => 32}, 2),
            (btreemap! {40 => 58, 170 => 56}, 2),
        ],
        (
            btreemap! {62 => 176, 136 => 109, 146 => 199},
            Some(btreemap! {62 => 176, 151 => 43, 209 => 171}),
        ),
    );
    random_b_tree_maps_size_inclusive_range_helper(
        1,
        2,
        &|seed| random_char_inclusive_range(seed, 'a', 'c'),
        &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, 1),
        &[
            btreemap! {'a' => 0},
            btreemap! {'b' => 0},
            btreemap! {'a' => 1, 'b' => 0},
            btreemap! {'a' => 1, 'b' => 0},
            btreemap! {'a' => 0, 'c' => 0},
            btreemap! {'a' => 0, 'c' => 0},
            btreemap! {'a' => 0},
            btreemap! {'c' => 0},
            btreemap! {'a' => 0, 'b' => 1},
            btreemap! {'b' => 0, 'c' => 0},
            btreemap! {'a' => 0},
            btreemap! {'a' => 0, 'c' => 0},
            btreemap! {'b' => 0, 'c' => 0},
            btreemap! {'b' => 0},
            btreemap! {'a' => 0, 'b' => 1},
            btreemap! {'c' => 0},
            btreemap! {'b' => 1, 'c' => 1},
            btreemap! {'b' => 1, 'c' => 1},
            btreemap! {'a' => 0},
            btreemap! {'c' => 0},
        ],
        &[
            (btreemap! {'b' => 0}, 83848),
            (btreemap! {'c' => 0}, 83765),
            (btreemap! {'c' => 1}, 83301),
            (btreemap! {'a' => 0}, 83077),
            (btreemap! {'a' => 1}, 82973),
            (btreemap! {'b' => 1}, 82869),
            (btreemap! {'a' => 1, 'b' => 0}, 41840),
            (btreemap! {'b' => 1, 'c' => 1}, 41821),
            (btreemap! {'b' => 0, 'c' => 1}, 41786),
            (btreemap! {'a' => 1, 'c' => 1}, 41776),
        ],
        (btreemap! {'b' => 0}, None),
    );
    random_b_tree_maps_size_inclusive_range_helper(
        0,
        1,
        &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, 5),
        &random_bools,
        &[
            btreemap! {},
            btreemap! {},
            btreemap! {1 => false},
            btreemap! {1 => false},
            btreemap! {0 => true},
            btreemap! {5 => false},
            btreemap! {},
            btreemap! {},
            btreemap! {4 => false},
            btreemap! {1 => true},
            btreemap! {},
            btreemap! {2 => false},
            btreemap! {1 => false},
            btreemap! {},
            btreemap! {4 => false},
            btreemap! {},
            btreemap! {5 => false},
            btreemap! {3 => false},
            btreemap! {},
            btreemap! {},
        ],
        &[
            (btreemap! {}, 499833),
            (btreemap! {5 => true}, 42178),
            (btreemap! {2 => false}, 41881),
            (btreemap! {4 => true}, 41849),
            (btreemap! {5 => false}, 41690),
            (btreemap! {0 => false}, 41674),
            (btreemap! {1 => true}, 41670),
            (btreemap! {3 => false}, 41665),
            (btreemap! {2 => true}, 41595),
            (btreemap! {0 => true}, 41583),
        ],
        (btreemap! {0 => false}, None),
    );
}

#[test]
#[should_panic]
fn random_b_tree_maps_size_inclusive_range_fail() {
    random_b_tree_maps_size_inclusive_range(
        EXAMPLE_SEED,
        2,
        1,
        &random_primitive_ints::<u32>,
        &random_primitive_ints::<u32>,
    );
}
