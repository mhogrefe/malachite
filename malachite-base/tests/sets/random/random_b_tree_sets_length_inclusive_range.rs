// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::hash::Hash;
use itertools::Itertools;
use malachite_base::chars::random::random_char_inclusive_range;
use malachite_base::num::random::geometric::geometric_random_unsigneds;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::{Seed, EXAMPLE_SEED};
use malachite_base::sets::random::random_b_tree_sets_length_inclusive_range;
use malachite_base::test_util::stats::common_values_map::common_values_map_debug;
use malachite_base::test_util::stats::median;
use std::collections::BTreeSet;
use std::fmt::Debug;

fn random_b_tree_sets_length_inclusive_range_helper<
    T: Clone + Debug + Eq + Hash + Ord,
    I: Clone + Iterator<Item = T>,
>(
    a: u64,
    b: u64,
    xs_gen: &dyn Fn(Seed) -> I,
    expected_values: &[BTreeSet<T>],
    expected_common_values: &[(BTreeSet<T>, usize)],
    expected_median: (BTreeSet<T>, Option<BTreeSet<T>>),
) {
    let xs = random_b_tree_sets_length_inclusive_range(EXAMPLE_SEED, a, b, xs_gen);
    let values = xs.clone().take(20).collect_vec();
    let common_values = common_values_map_debug(1000000, 10, xs.clone());
    let median = median(xs.take(1000000));
    assert_eq!(
        (values.as_slice(), common_values.as_slice(), median),
        (expected_values, expected_common_values, expected_median)
    );
}

#[test]
fn test_random_b_tree_sets_length_inclusive_range() {
    random_b_tree_sets_length_inclusive_range_helper(
        2,
        3,
        &random_primitive_ints::<u8>,
        &[
            btreeset! {11, 85, 136},
            btreeset! {200, 235},
            btreeset! {134, 203, 223},
            btreeset! {38, 217, 235},
            btreeset! {162, 177},
            btreeset! {32, 166, 234},
            btreeset! {30, 90, 218},
            btreeset! {9, 106},
            btreeset! {151, 204, 216},
            btreeset! {97, 213, 253},
            btreeset! {78, 91},
            btreeset! {39, 175, 191},
            btreeset! {170, 232},
            btreeset! {2, 35, 233},
            btreeset! {22, 198, 217},
            btreeset! {17, 114},
            btreeset! {32, 173},
            btreeset! {65, 114},
            btreeset! {121, 173, 222},
            btreeset! {25, 144},
        ],
        &[
            (btreeset! {106, 108}, 34),
            (btreeset! {224, 237}, 34),
            (btreeset! {51, 132}, 32),
            (btreeset! {82, 117}, 32),
            (btreeset! {72, 108}, 31),
            (btreeset! {142, 194}, 31),
            (btreeset! {0, 34}, 30),
            (btreeset! {12, 208}, 30),
            (btreeset! {15, 141}, 30),
            (btreeset! {30, 248}, 30),
        ],
        (btreeset! {62, 131, 203}, Some(btreeset! {62, 131, 205})),
    );
    random_b_tree_sets_length_inclusive_range_helper(
        2,
        3,
        &|seed| geometric_random_unsigneds::<u32>(seed, 2, 1),
        &[
            btreeset! {0, 1, 5},
            btreeset! {1, 4},
            btreeset! {2, 4, 6},
            btreeset! {0, 1, 2},
            btreeset! {9, 13},
            btreeset! {0, 2, 7},
            btreeset! {4, 6, 7},
            btreeset! {0, 6},
            btreeset! {0, 1, 3},
            btreeset! {1, 2, 5},
            btreeset! {0, 1},
            btreeset! {0, 1, 4},
            btreeset! {0, 2},
            btreeset! {0, 2, 12},
            btreeset! {1, 2, 3},
            btreeset! {3, 9},
            btreeset! {0, 1},
            btreeset! {1, 2},
            btreeset! {0, 1, 11},
            btreeset! {1, 6},
        ],
        &[
            (btreeset! {0, 1}, 103032),
            (btreeset! {0, 1, 2}, 84142),
            (btreeset! {0, 2}, 66185),
            (btreeset! {0, 1, 3}, 52638),
            (btreeset! {0, 3}, 42990),
            (btreeset! {1, 2}, 40380),
            (btreeset! {0, 1, 4}, 33815),
            (btreeset! {0, 2, 3}, 31257),
            (btreeset! {0, 4}, 28088),
            (btreeset! {1, 3}, 26214),
        ],
        (btreeset! {0, 3}, None),
    );
    random_b_tree_sets_length_inclusive_range_helper(
        2,
        3,
        &|seed| random_char_inclusive_range(seed, 'a', 'z'),
        &[
            btreeset! {'c', 'q', 'v'},
            btreeset! {'e', 'i'},
            btreeset! {'g', 'p', 's'},
            btreeset! {'m', 'n', 't'},
            btreeset! {'o', 'z'},
            btreeset! {'f', 'k', 'm'},
            btreeset! {'q', 'u', 'y'},
            btreeset! {'k', 'x'},
            btreeset! {'h', 'n', 'u'},
            btreeset! {'a', 'j', 'n'},
            btreeset! {'w', 'z'},
            btreeset! {'b', 'l', 'w'},
            btreeset! {'l', 'u'},
            btreeset! {'e', 'l', 'n'},
            btreeset! {'k', 'u', 'v'},
            btreeset! {'c', 'h'},
            btreeset! {'i', 'y'},
            btreeset! {'m', 'r'},
            btreeset! {'m', 's', 'y'},
            btreeset! {'e', 'l'},
        ],
        &[
            (btreeset! {'l', 'x'}, 1640),
            (btreeset! {'o', 't'}, 1636),
            (btreeset! {'b', 'p'}, 1630),
            (btreeset! {'m', 'v'}, 1623),
            (btreeset! {'h', 'u'}, 1621),
            (btreeset! {'a', 'x'}, 1614),
            (btreeset! {'d', 'f'}, 1613),
            (btreeset! {'e', 'r'}, 1613),
            (btreeset! {'o', 'p'}, 1612),
            (btreeset! {'c', 'i'}, 1611),
        ],
        (btreeset! {'g', 'j'}, None),
    );
}

#[test]
#[should_panic]
fn random_b_tree_sets_length_inclusive_range_fail() {
    random_b_tree_sets_length_inclusive_range(EXAMPLE_SEED, 2, 1, &random_primitive_ints::<u32>);
}
