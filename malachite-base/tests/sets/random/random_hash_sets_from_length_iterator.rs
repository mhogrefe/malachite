// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::bools::random::random_bools;
use malachite_base::num::random::geometric::geometric_random_unsigneds;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::{Seed, EXAMPLE_SEED};
use malachite_base::sets::random::random_hash_sets_from_length_iterator;
use malachite_base::vecs::random_values_from_vec;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

fn random_hash_sets_from_length_iterator_helper<
    T: Clone + Debug + Eq + Hash,
    I: Clone + Iterator<Item = u64>,
    J: Clone + Iterator<Item = T>,
>(
    lengths_gen: &dyn Fn(Seed) -> I,
    xs_gen: &dyn Fn(Seed) -> J,
    expected_values: &[HashSet<T>],
) {
    let xs = random_hash_sets_from_length_iterator(EXAMPLE_SEED, lengths_gen, xs_gen);
    let values = xs.take(20).collect_vec();
    assert_eq!(values.as_slice(), expected_values);
}

#[test]
fn test_random_hash_sets_from_length_iterator() {
    random_hash_sets_from_length_iterator_helper(
        &|seed| random_values_from_vec(seed, vec![0, 2]),
        &random_bools,
        &[
            hashset! {false, true},
            hashset! {},
            hashset! {false, true},
            hashset! {false, true},
            hashset! {},
            hashset! {false, true},
            hashset! {false, true},
            hashset! {},
            hashset! {false, true},
            hashset! {false, true},
            hashset! {},
            hashset! {false, true},
            hashset! {},
            hashset! {false, true},
            hashset! {false, true},
            hashset! {},
            hashset! {},
            hashset! {},
            hashset! {false, true},
            hashset! {},
        ],
    );
    random_hash_sets_from_length_iterator_helper(
        &|seed| geometric_random_unsigneds::<u64>(seed, 2, 1).map(|x| x << 1),
        &random_primitive_ints::<u8>,
        &[
            hashset! {11, 38, 85, 134, 136, 162, 177, 200, 203, 217, 223, 235},
            hashset! {32, 166},
            hashset! {9, 30, 39, 78, 90, 91, 97, 106, 151, 191, 204, 213, 216, 218, 234, 253},
            hashset! {170, 175},
            hashset! {
                2, 17, 22, 25, 32, 34, 35, 52, 65, 69, 73, 79, 91, 112, 114, 115, 121, 137, 144,
                148, 153, 173, 178, 198, 217, 222, 232, 233,
            },
            hashset! {},
            hashset! {95, 106, 122, 130, 167, 168, 172, 177, 197, 207},
            hashset! {9, 74, 86, 101, 115, 150, 218, 221},
            hashset! {109, 123},
            hashset! {},
            hashset! {40, 48, 52, 97, 104, 133, 159, 196, 201, 235, 247, 250},
            hashset! {7, 68, 190, 216},
            hashset! {},
            hashset! {},
            hashset! {157, 216},
            hashset! {11, 24, 43, 103, 112, 217},
            hashset! {},
            hashset! {84, 211},
            hashset! {},
            hashset! {55, 135},
        ],
    );
}
