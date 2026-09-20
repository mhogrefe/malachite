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
use malachite_base::maps::random::random_b_tree_maps_fixed_size;
use malachite_base::num::random::{random_primitive_ints, random_unsigned_inclusive_range};
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::common_values_map::common_values_map_debug;
use malachite_base::test_util::stats::median;
use std::collections::BTreeMap;
use std::fmt::Debug;

fn random_b_tree_maps_fixed_size_helper<
    K: Clone + Debug + Eq + Hash + Ord,
    V: Clone + Debug + Eq + Hash + Ord,
    I: Clone + Iterator<Item = K>,
    J: Clone + Iterator<Item = V>,
>(
    size: u64,
    keys: I,
    values: J,
    expected_values: &[BTreeMap<K, V>],
    expected_common_values: &[(BTreeMap<K, V>, usize)],
    expected_median: (BTreeMap<K, V>, Option<BTreeMap<K, V>>),
) {
    let xs = random_b_tree_maps_fixed_size(size, keys, values);
    let values = xs.clone().take(20).collect_vec();
    let common_values = common_values_map_debug(1000000, 10, xs.clone());
    let median = median(xs.take(1000000));
    assert_eq!(
        (values.as_slice(), common_values.as_slice(), median),
        (expected_values, expected_common_values, expected_median)
    );
}

#[test]
fn test_random_b_tree_maps_fixed_size() {
    random_b_tree_maps_fixed_size_helper(
        0,
        random_primitive_ints::<u8>(EXAMPLE_SEED.fork("keys")),
        random_primitive_ints::<u8>(EXAMPLE_SEED.fork("values")),
        &repeat_n(btreemap! {}, 20).collect_vec(),
        &[(btreemap! {}, 1000000)],
        (btreemap! {}, None),
    );
    random_b_tree_maps_fixed_size_helper(
        1,
        random_bools(EXAMPLE_SEED.fork("keys")),
        random_bools(EXAMPLE_SEED.fork("values")),
        &[
            btreemap! {true => false},
            btreemap! {true => false},
            btreemap! {true => true},
            btreemap! {true => false},
            btreemap! {false => false},
            btreemap! {false => true},
            btreemap! {true => false},
            btreemap! {false => false},
            btreemap! {false => false},
            btreemap! {false => false},
            btreemap! {false => false},
            btreemap! {false => false},
            btreemap! {true => false},
            btreemap! {false => true},
            btreemap! {true => false},
            btreemap! {false => false},
            btreemap! {true => false},
            btreemap! {true => false},
            btreemap! {false => false},
            btreemap! {false => false},
        ],
        &[
            (btreemap! {true => false}, 250722),
            (btreemap! {true => true}, 250169),
            (btreemap! {false => false}, 249717),
            (btreemap! {false => true}, 249392),
        ],
        (btreemap! {true => false}, None),
    );
    random_b_tree_maps_fixed_size_helper(
        2,
        random_char_inclusive_range(EXAMPLE_SEED.fork("keys"), 'a', 'c'),
        random_unsigned_inclusive_range::<u8>(EXAMPLE_SEED.fork("values"), 0, 2),
        &[
            btreemap! {'a' => 0, 'b' => 1},
            btreemap! {'a' => 2, 'b' => 0},
            btreemap! {'a' => 0, 'b' => 0},
            btreemap! {'a' => 2, 'c' => 0},
            btreemap! {'a' => 0, 'c' => 0},
            btreemap! {'a' => 0, 'c' => 1},
            btreemap! {'a' => 2, 'b' => 1},
            btreemap! {'b' => 1, 'c' => 2},
            btreemap! {'a' => 2, 'c' => 0},
            btreemap! {'b' => 0, 'c' => 1},
            btreemap! {'a' => 0, 'b' => 1},
            btreemap! {'b' => 0, 'c' => 2},
            btreemap! {'b' => 0, 'c' => 2},
            btreemap! {'b' => 1, 'c' => 1},
            btreemap! {'a' => 2, 'c' => 0},
            btreemap! {'b' => 1, 'c' => 0},
            btreemap! {'a' => 1, 'c' => 0},
            btreemap! {'b' => 1, 'c' => 0},
            btreemap! {'a' => 1, 'b' => 2},
            btreemap! {'a' => 2, 'c' => 2},
        ],
        &[
            (btreemap! {'b' => 1, 'c' => 1}, 37460),
            (btreemap! {'a' => 0, 'b' => 0}, 37395),
            (btreemap! {'b' => 1, 'c' => 2}, 37365),
            (btreemap! {'a' => 1, 'c' => 2}, 37351),
            (btreemap! {'b' => 0, 'c' => 0}, 37276),
            (btreemap! {'b' => 2, 'c' => 0}, 37263),
            (btreemap! {'a' => 0, 'b' => 1}, 37169),
            (btreemap! {'b' => 0, 'c' => 2}, 37165),
            (btreemap! {'b' => 2, 'c' => 2}, 37080),
            (btreemap! {'a' => 0, 'b' => 2}, 37059),
        ],
        (btreemap! {'a' => 2, 'b' => 1}, None),
    );
    random_b_tree_maps_fixed_size_helper(
        2,
        random_primitive_ints::<u8>(EXAMPLE_SEED.fork("keys")),
        random_primitive_ints::<u8>(EXAMPLE_SEED.fork("values")),
        &[
            btreemap! {79 => 36, 80 => 32},
            btreemap! {10 => 158, 51 => 64},
            btreemap! {59 => 39, 253 => 71},
            btreemap! {160 => 91, 246 => 8},
            btreemap! {85 => 220, 245 => 18},
            btreemap! {53 => 243, 214 => 153},
            btreemap! {139 => 67, 219 => 134},
            btreemap! {120 => 6, 233 => 107},
            btreemap! {33 => 14, 158 => 198},
            btreemap! {202 => 202, 236 => 14},
            btreemap! {157 => 168, 161 => 185},
            btreemap! {72 => 72, 155 => 166},
            btreemap! {19 => 197, 194 => 97},
            btreemap! {153 => 73, 236 => 27},
            btreemap! {80 => 112, 252 => 53},
            btreemap! {25 => 167, 74 => 234},
            btreemap! {97 => 219, 119 => 127},
            btreemap! {33 => 13, 68 => 55},
            btreemap! {55 => 51, 241 => 208},
            btreemap! {32 => 117, 69 => 202},
        ],
        &[
            (btreemap! {4 => 6, 23 => 221}, 2),
            (btreemap! {11 => 78, 40 => 27}, 2),
            (btreemap! {12 => 87, 99 => 82}, 2),
            (btreemap! {13 => 1, 250 => 42}, 2),
            (btreemap! {13 => 2, 158 => 39}, 2),
            (btreemap! {13 => 97, 255 => 4}, 2),
            (btreemap! {15 => 32, 93 => 94}, 2),
            (btreemap! {17 => 32, 69 => 67}, 2),
            (btreemap! {22 => 93, 56 => 94}, 2),
            (btreemap! {24 => 14, 81 => 26}, 2),
        ],
        (
            btreemap! {74 => 226, 170 => 191},
            Some(btreemap! {74 => 226, 171 => 37}),
        ),
    );
}
