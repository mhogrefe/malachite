// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::hash::Hash;
use itertools::Itertools;
use malachite_base::bools::random::random_bools;
use malachite_base::num::random::geometric::geometric_random_unsigneds;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::{Seed, EXAMPLE_SEED};
use malachite_base::sets::random::random_b_tree_sets_from_length_iterator;
use malachite_base::test_util::stats::common_values_map::common_values_map_debug;
use malachite_base::test_util::stats::median;
use malachite_base::vecs::random_values_from_vec;
use std::collections::BTreeSet;
use std::fmt::Debug;

fn random_b_tree_sets_from_length_iterator_helper<
    T: Clone + Debug + Eq + Hash + Ord,
    I: Clone + Iterator<Item = u64>,
    J: Clone + Iterator<Item = T>,
>(
    lengths_gen: &dyn Fn(Seed) -> I,
    xs_gen: &dyn Fn(Seed) -> J,
    expected_values: &[BTreeSet<T>],
    expected_common_values: &[(BTreeSet<T>, usize)],
    expected_median: (BTreeSet<T>, Option<BTreeSet<T>>),
) {
    let xs = random_b_tree_sets_from_length_iterator(EXAMPLE_SEED, lengths_gen, xs_gen);
    let values = xs.clone().take(20).collect_vec();
    let common_values = common_values_map_debug(1000000, 10, xs.clone());
    let median = median(xs.take(1000000));
    assert_eq!(
        (values.as_slice(), common_values.as_slice(), median),
        (expected_values, expected_common_values, expected_median)
    );
}

#[test]
fn test_random_b_tree_sets_from_length_iterator() {
    random_b_tree_sets_from_length_iterator_helper(
        &|seed| random_values_from_vec(seed, vec![0, 2]),
        &random_bools,
        &[
            btreeset! {false, true},
            btreeset! {},
            btreeset! {false, true},
            btreeset! {false, true},
            btreeset! {},
            btreeset! {false, true},
            btreeset! {false, true},
            btreeset! {},
            btreeset! {false, true},
            btreeset! {false, true},
            btreeset! {},
            btreeset! {false, true},
            btreeset! {},
            btreeset! {false, true},
            btreeset! {false, true},
            btreeset! {},
            btreeset! {},
            btreeset! {},
            btreeset! {false, true},
            btreeset! {},
        ],
        &[(btreeset! {false, true}, 500363), (btreeset! {}, 499637)],
        (btreeset! {false, true}, None),
    );
    random_b_tree_sets_from_length_iterator_helper(
        &|seed| geometric_random_unsigneds::<u64>(seed, 2, 1).map(|x| x << 1),
        &random_primitive_ints::<u8>,
        &[
            btreeset! {11, 38, 85, 134, 136, 162, 177, 200, 203, 217, 223, 235},
            btreeset! {32, 166},
            btreeset! {9, 30, 39, 78, 90, 91, 97, 106, 151, 191, 204, 213, 216, 218, 234, 253},
            btreeset! {170, 175},
            btreeset! {
                2, 17, 22, 25, 32, 34, 35, 52, 65, 69, 73, 79, 91, 112, 114, 115, 121, 137, 144,
                148, 153, 173, 178, 198, 217, 222, 232, 233,
            },
            btreeset! {},
            btreeset! {95, 106, 122, 130, 167, 168, 172, 177, 197, 207},
            btreeset! {9, 74, 86, 101, 115, 150, 218, 221},
            btreeset! {109, 123},
            btreeset! {},
            btreeset! {40, 48, 52, 97, 104, 133, 159, 196, 201, 235, 247, 250},
            btreeset! {7, 68, 190, 216},
            btreeset! {},
            btreeset! {},
            btreeset! {157, 216},
            btreeset! {11, 24, 43, 103, 112, 217},
            btreeset! {},
            btreeset! {84, 211},
            btreeset! {},
            btreeset! {55, 135},
        ],
        &[
            (btreeset! {}, 333981),
            (btreeset! {33, 163}, 22),
            (btreeset! {76, 233}, 19),
            (btreeset! {5, 42}, 18),
            (btreeset! {76, 79}, 18),
            (btreeset! {32, 134}, 18),
            (btreeset! {69, 234}, 18),
            (btreeset! {74, 164}, 18),
            (btreeset! {86, 192}, 18),
            (btreeset! {99, 145}, 18),
        ],
        (btreeset! {12, 190}, None),
    );
}
