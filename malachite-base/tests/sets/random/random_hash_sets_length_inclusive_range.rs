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
use malachite_base::sets::random::random_hash_sets_length_inclusive_range;
use std::collections::HashSet;
use std::fmt::Debug;

fn random_hash_sets_length_inclusive_range_helper<
    T: Clone + Debug + Eq + Hash + Ord,
    I: Clone + Iterator<Item = T>,
>(
    a: u64,
    b: u64,
    xs_gen: &dyn Fn(Seed) -> I,
    expected_values: &[HashSet<T>],
) {
    let xs = random_hash_sets_length_inclusive_range(EXAMPLE_SEED, a, b, xs_gen);
    let values = xs.take(20).collect_vec();
    assert_eq!(values.as_slice(), expected_values);
}

#[test]
fn test_random_hash_sets_length_inclusive_range() {
    random_hash_sets_length_inclusive_range_helper(
        2,
        3,
        &random_primitive_ints::<u8>,
        &[
            hashset! {11, 85, 136},
            hashset! {200, 235},
            hashset! {134, 203, 223},
            hashset! {38, 217, 235},
            hashset! {162, 177},
            hashset! {32, 166, 234},
            hashset! {30, 90, 218},
            hashset! {9, 106},
            hashset! {151, 204, 216},
            hashset! {97, 213, 253},
            hashset! {78, 91},
            hashset! {39, 175, 191},
            hashset! {170, 232},
            hashset! {2, 35, 233},
            hashset! {22, 198, 217},
            hashset! {17, 114},
            hashset! {32, 173},
            hashset! {65, 114},
            hashset! {121, 173, 222},
            hashset! {25, 144},
        ],
    );
    random_hash_sets_length_inclusive_range_helper(
        2,
        3,
        &|seed| geometric_random_unsigneds::<u32>(seed, 2, 1),
        &[
            hashset! {0, 1, 5},
            hashset! {1, 4},
            hashset! {2, 4, 6},
            hashset! {0, 1, 2},
            hashset! {9, 13},
            hashset! {0, 2, 7},
            hashset! {4, 6, 7},
            hashset! {0, 6},
            hashset! {0, 1, 3},
            hashset! {1, 2, 5},
            hashset! {0, 1},
            hashset! {0, 1, 4},
            hashset! {0, 2},
            hashset! {0, 2, 12},
            hashset! {1, 2, 3},
            hashset! {3, 9},
            hashset! {0, 1},
            hashset! {1, 2},
            hashset! {0, 1, 11},
            hashset! {1, 6},
        ],
    );
    random_hash_sets_length_inclusive_range_helper(
        2,
        3,
        &|seed| random_char_inclusive_range(seed, 'a', 'z'),
        &[
            hashset! {'c', 'q', 'v'},
            hashset! {'e', 'i'},
            hashset! {'g', 'p', 's'},
            hashset! {'m', 'n', 't'},
            hashset! {'o', 'z'},
            hashset! {'f', 'k', 'm'},
            hashset! {'q', 'u', 'y'},
            hashset! {'k', 'x'},
            hashset! {'h', 'n', 'u'},
            hashset! {'a', 'j', 'n'},
            hashset! {'w', 'z'},
            hashset! {'b', 'l', 'w'},
            hashset! {'l', 'u'},
            hashset! {'e', 'l', 'n'},
            hashset! {'k', 'u', 'v'},
            hashset! {'c', 'h'},
            hashset! {'i', 'y'},
            hashset! {'m', 'r'},
            hashset! {'m', 's', 'y'},
            hashset! {'e', 'l'},
        ],
    );
}

#[test]
#[should_panic]
fn random_hash_sets_length_inclusive_range_fail() {
    random_hash_sets_length_inclusive_range(EXAMPLE_SEED, 2, 1, &random_primitive_ints::<u32>);
}
