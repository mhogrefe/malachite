// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::hash::Hash;
use itertools::{repeat_n, Itertools};
use malachite_base::bools::random::random_bools;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::sets::random::random_b_tree_sets_fixed_length;
use malachite_base::test_util::stats::common_values_map::common_values_map_debug;
use malachite_base::test_util::stats::median;
use std::collections::BTreeSet;
use std::fmt::Debug;

fn random_b_tree_sets_fixed_length_helper<I: Clone + Iterator>(
    len: u64,
    xs: I,
    expected_values: &[BTreeSet<I::Item>],
    expected_common_values: &[(BTreeSet<I::Item>, usize)],
    expected_median: (BTreeSet<I::Item>, Option<BTreeSet<I::Item>>),
) where
    I::Item: Clone + Debug + Eq + Hash + Ord,
{
    let xs = random_b_tree_sets_fixed_length(len, xs);
    let values = xs.clone().take(20).collect_vec();
    let common_values = common_values_map_debug(1000000, 10, xs.clone());
    let median = median(xs.take(1000000));
    assert_eq!(
        (values.as_slice(), common_values.as_slice(), median),
        (expected_values, expected_common_values, expected_median)
    );
}

#[test]
fn test_random_b_tree_sets_fixed_length() {
    random_b_tree_sets_fixed_length_helper(
        0,
        random_primitive_ints::<u8>(EXAMPLE_SEED),
        &repeat_n(btreeset! {}, 20).collect_vec(),
        &[(btreeset! {}, 1000000)],
        (btreeset! {}, None),
    );
    random_b_tree_sets_fixed_length_helper(
        1,
        random_bools(EXAMPLE_SEED),
        &[
            btreeset! {true},
            btreeset! {false},
            btreeset! {false},
            btreeset! {false},
            btreeset! {true},
            btreeset! {true},
            btreeset! {true},
            btreeset! {false},
            btreeset! {true},
            btreeset! {true},
            btreeset! {true},
            btreeset! {true},
            btreeset! {false},
            btreeset! {true},
            btreeset! {true},
            btreeset! {true},
            btreeset! {true},
            btreeset! {false},
            btreeset! {true},
            btreeset! {false},
        ],
        &[(btreeset! {true}, 500473), (btreeset! {false}, 499527)],
        (btreeset! {true}, None),
    );
    random_b_tree_sets_fixed_length_helper(
        3,
        random_primitive_ints::<u8>(EXAMPLE_SEED),
        &[
            btreeset! {69, 113, 239},
            btreeset! {108, 210, 228},
            btreeset! {87, 161, 168},
            btreeset! {32, 83, 110},
            btreeset! {34, 89, 188},
            btreeset! {93, 200, 238},
            btreeset! {115, 149, 189},
            btreeset! {149, 201, 217},
            btreeset! {31, 117, 146},
            btreeset! {72, 151, 169},
            btreeset! {7, 33, 174},
            btreeset! {38, 81, 144},
            btreeset! {72, 113, 127},
            btreeset! {107, 128, 233},
            btreeset! {12, 46, 119},
            btreeset! {18, 164, 243},
            btreeset! {59, 114, 174},
            btreeset! {39, 174, 247},
            btreeset! {104, 160, 184},
            btreeset! {37, 100, 252},
        ],
        &[
            (btreeset! {57, 142, 207}, 7),
            (btreeset! {32, 68, 169}, 6),
            (btreeset! {36, 70, 195}, 6),
            (btreeset! {125, 168, 194}, 6),
            (btreeset! {0, 97, 205}, 5),
            (btreeset! {2, 33, 227}, 5),
            (btreeset! {5, 46, 239}, 5),
            (btreeset! {9, 68, 189}, 5),
            (btreeset! {9, 78, 240}, 5),
            (btreeset! {1, 110, 203}, 5),
        ],
        (btreeset! {52, 133, 241}, Some(btreeset! {52, 133, 242})),
    );
    random_b_tree_sets_fixed_length_helper(
        2,
        random_b_tree_sets_fixed_length(2, random_primitive_ints::<u8>(EXAMPLE_SEED)),
        &[
            btreeset! {btreeset!{69, 108}, btreeset!{113, 239}},
            btreeset! {btreeset!{161, 168}, btreeset!{210, 228}},
            btreeset! {btreeset!{32, 87}, btreeset!{83, 110}},
            btreeset! {btreeset!{34, 188}, btreeset!{89, 238}},
            btreeset! {btreeset!{93, 200}, btreeset!{115, 149}},
            btreeset! {btreeset!{149, 189}, btreeset!{201, 217}},
            btreeset! {btreeset!{31, 72}, btreeset!{117, 146}},
            btreeset! {btreeset!{33, 174}, btreeset!{151, 169}},
            btreeset! {btreeset!{7, 38}, btreeset!{81, 144}},
            btreeset! {btreeset!{72, 127}, btreeset!{113, 128}},
            btreeset! {btreeset!{46, 119}, btreeset!{107, 233}},
            btreeset! {btreeset!{12, 18}, btreeset!{164, 243}},
            btreeset! {btreeset!{59, 247}, btreeset!{114, 174}},
            btreeset! {btreeset!{39, 174}, btreeset!{160, 184}},
            btreeset! {btreeset!{37, 104}, btreeset!{100, 252}},
            btreeset! {btreeset!{69, 107}, btreeset!{122, 228}},
            btreeset! {btreeset!{142, 179}, btreeset!{242, 248}},
            btreeset! {btreeset!{61, 189}, btreeset!{233, 239}},
            btreeset! {btreeset!{7, 192}, btreeset!{85, 235}},
            btreeset! {btreeset!{90, 200}, btreeset!{178, 185}},
        ],
        &[
            (btreeset! {btreeset!{0, 78}, btreeset!{34, 52}}, 2),
            (btreeset! {btreeset!{1, 58}, btreeset!{6, 112}}, 2),
            (btreeset! {btreeset!{1, 63}, btreeset!{8, 154}}, 2),
            (btreeset! {btreeset!{1, 97}, btreeset!{7, 250}}, 2),
            (btreeset! {btreeset!{2, 33}, btreeset!{40, 81}}, 2),
            (btreeset! {btreeset!{3, 160}, btreeset!{7, 29}}, 2),
            (btreeset! {btreeset!{3, 32}, btreeset!{12, 60}}, 2),
            (btreeset! {btreeset!{6, 130}, btreeset!{7, 20}}, 2),
            (btreeset! {btreeset!{6, 68}, btreeset!{7, 126}}, 2),
            (btreeset! {btreeset!{6, 77}, btreeset!{36, 54}}, 2),
        ],
        (
            btreeset! {btreeset!{40, 193}, btreeset!{94, 142}},
            Some(btreeset! {btreeset!{40, 193}, btreeset!{97, 243}}),
        ),
    );
}
