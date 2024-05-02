// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::hash::Hash;
use malachite_base::bools::random::random_bools;
use malachite_base::num::random::geometric::geometric_random_unsigneds;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::{Seed, EXAMPLE_SEED};
use malachite_base::test_util::vecs::random::random_vecs_helper_helper;
use malachite_base::vecs::random::random_ordered_unique_vecs_from_length_iterator;
use malachite_base::vecs::random_values_from_vec;
use std::fmt::Debug;

fn random_ordered_unique_vecs_from_length_iterator_helper<
    T: Clone + Debug + Eq + Hash + Ord,
    I: Clone + Iterator<Item = u64>,
    J: Clone + Iterator<Item = T>,
>(
    lengths_gen: &dyn Fn(Seed) -> I,
    xs_gen: &dyn Fn(Seed) -> J,
    expected_values: &[&[T]],
    expected_common_values: &[(&[T], usize)],
    expected_median: (&[T], Option<&[T]>),
) {
    random_vecs_helper_helper(
        random_ordered_unique_vecs_from_length_iterator(EXAMPLE_SEED, lengths_gen, xs_gen),
        expected_values,
        expected_common_values,
        expected_median,
    );
}

#[test]
fn test_random_ordered_unique_vecs_from_length_iterator() {
    random_ordered_unique_vecs_from_length_iterator_helper(
        &|seed| random_values_from_vec(seed, vec![0, 2]),
        &random_bools,
        &[
            &[false, true],
            &[],
            &[false, true],
            &[false, true],
            &[],
            &[false, true],
            &[false, true],
            &[],
            &[false, true],
            &[false, true],
            &[],
            &[false, true],
            &[],
            &[false, true],
            &[false, true],
            &[],
            &[],
            &[],
            &[false, true],
            &[],
        ],
        &[(&[false, true], 500363), (&[], 499637)],
        (&[false, true], None),
    );
    random_ordered_unique_vecs_from_length_iterator_helper(
        &|seed| geometric_random_unsigneds::<u64>(seed, 2, 1).map(|x| x << 1),
        &random_primitive_ints::<u8>,
        &[
            &[11, 38, 85, 134, 136, 162, 177, 200, 203, 217, 223, 235],
            &[32, 166],
            &[9, 30, 39, 78, 90, 91, 97, 106, 151, 191, 204, 213, 216, 218, 234, 253],
            &[170, 175],
            &[
                2, 17, 22, 25, 32, 34, 35, 52, 65, 69, 73, 79, 91, 112, 114, 115, 121, 137, 144,
                148, 153, 173, 178, 198, 217, 222, 232, 233,
            ],
            &[],
            &[95, 106, 122, 130, 167, 168, 172, 177, 197, 207],
            &[9, 74, 86, 101, 115, 150, 218, 221],
            &[109, 123],
            &[],
            &[40, 48, 52, 97, 104, 133, 159, 196, 201, 235, 247, 250],
            &[7, 68, 190, 216],
            &[],
            &[],
            &[157, 216],
            &[11, 24, 43, 103, 112, 217],
            &[],
            &[84, 211],
            &[],
            &[55, 135],
        ],
        &[
            (&[], 333981),
            (&[33, 163], 22),
            (&[76, 233], 19),
            (&[5, 42], 18),
            (&[76, 79], 18),
            (&[32, 134], 18),
            (&[69, 234], 18),
            (&[74, 164], 18),
            (&[86, 192], 18),
            (&[99, 145], 18),
        ],
        (&[12, 190], None),
    );
}
