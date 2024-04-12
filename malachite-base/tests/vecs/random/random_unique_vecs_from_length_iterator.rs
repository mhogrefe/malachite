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
use malachite_base::vecs::random::random_unique_vecs_from_length_iterator;
use malachite_base::vecs::random_values_from_vec;
use std::fmt::Debug;

fn random_unique_vecs_from_length_iterator_helper<
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
        random_unique_vecs_from_length_iterator(EXAMPLE_SEED, lengths_gen, xs_gen),
        expected_values,
        expected_common_values,
        expected_median,
    );
}

#[test]
fn test_random_unique_vecs_from_length_iterator() {
    random_unique_vecs_from_length_iterator_helper(
        &|seed| random_values_from_vec(seed, vec![0, 2]),
        &random_bools,
        &[
            &[true, false],
            &[],
            &[true, false],
            &[true, false],
            &[],
            &[true, false],
            &[true, false],
            &[],
            &[true, false],
            &[false, true],
            &[],
            &[false, true],
            &[],
            &[false, true],
            &[false, true],
            &[],
            &[],
            &[],
            &[true, false],
            &[],
        ],
        &[(&[], 499637), (&[false, true], 250413), (&[true, false], 249950)],
        (&[false, true], None),
    );
    random_unique_vecs_from_length_iterator_helper(
        &|seed| geometric_random_unsigneds::<u64>(seed, 2, 1).map(|x| x << 1),
        &random_primitive_ints::<u8>,
        &[
            &[85, 11, 136, 200, 235, 134, 203, 223, 38, 217, 177, 162],
            &[32, 166],
            &[234, 30, 218, 90, 106, 9, 216, 204, 151, 213, 97, 253, 78, 91, 39, 191],
            &[175, 170],
            &[
                232, 233, 2, 35, 22, 217, 198, 114, 17, 32, 173, 65, 121, 222, 25, 144, 148, 79,
                115, 52, 73, 69, 137, 91, 153, 178, 112, 34,
            ],
            &[],
            &[95, 106, 167, 197, 130, 168, 122, 207, 172, 177],
            &[86, 150, 221, 218, 101, 115, 74, 9],
            &[123, 109],
            &[],
            &[52, 201, 159, 247, 250, 48, 133, 235, 196, 40, 97, 104],
            &[68, 190, 216, 7],
            &[],
            &[],
            &[216, 157],
            &[43, 112, 217, 24, 11, 103],
            &[],
            &[211, 84],
            &[],
            &[135, 55],
        ],
        &[
            (&[], 333981),
            (&[79, 76], 14),
            (&[234, 129], 14),
            (&[119, 62], 13),
            (&[33, 163], 13),
            (&[5, 42], 12),
            (&[28, 91], 12),
            (&[55, 25], 12),
            (&[152, 55], 12),
            (&[224, 77], 12),
        ],
        (&[63, 197, 169, 69, 240, 201], Some(&[63, 197, 181, 249])),
    );
}
