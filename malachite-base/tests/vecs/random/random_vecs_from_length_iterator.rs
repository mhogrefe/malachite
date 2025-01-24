// Copyright © 2025 Mikhail Hogrefe
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
use malachite_base::vecs::random::random_vecs_from_length_iterator;
use malachite_base::vecs::random_values_from_vec;
use std::fmt::Debug;

fn random_vecs_from_length_iterator_helper<
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
        random_vecs_from_length_iterator(EXAMPLE_SEED, lengths_gen, xs_gen),
        expected_values,
        expected_common_values,
        expected_median,
    );
}

#[test]
fn test_random_vecs_from_length_iterator() {
    random_vecs_from_length_iterator_helper(
        &|seed| random_values_from_vec(seed, vec![0, 2, 4]),
        &random_bools,
        &[
            &[true, false],
            &[true, false, true, false],
            &[true, false],
            &[true, true, false, true],
            &[false, false, false, false],
            &[false, false],
            &[],
            &[false, true],
            &[],
            &[false, false, false, true],
            &[false, false, false, true],
            &[false, false],
            &[true, true, true, true],
            &[false, true],
            &[false, true, true, true],
            &[false, true, true, false],
            &[false, false, false, true],
            &[],
            &[true, true, false, true],
            &[],
        ],
        &[
            (&[], 333820),
            (&[true, false], 83553),
            (&[false, false], 83348),
            (&[false, true], 83319),
            (&[true, true], 82905),
            (&[true, true, true, false], 21126),
            (&[false, false, false, true], 20940),
            (&[false, false, true, false], 20931),
            (&[true, true, false, true], 20925),
            (&[true, false, false, false], 20899),
        ],
        (&[false, false, true, true], None),
    );
    random_vecs_from_length_iterator_helper(
        &|seed| geometric_random_unsigneds::<u64>(seed, 2, 1).map(|x| x << 1),
        &random_primitive_ints::<u8>,
        &[
            &[85, 11, 136, 200, 235, 134, 203, 223, 38, 235, 217, 177],
            &[162, 32],
            &[166, 234, 30, 218, 90, 106, 9, 216, 204, 151, 213, 97, 253, 78, 91, 39],
            &[191, 175],
            &[
                170, 232, 233, 2, 35, 22, 217, 198, 114, 17, 32, 173, 114, 65, 121, 222, 173, 25,
                144, 148, 79, 115, 52, 73, 69, 137, 91, 153,
            ],
            &[],
            &[178, 112, 34, 95, 106, 167, 197, 130, 168, 122],
            &[207, 172, 177, 86, 150, 221, 218, 101],
            &[115, 74],
            &[],
            &[9, 123, 109, 52, 201, 159, 247, 250, 48, 133, 235, 196],
            &[40, 97, 104, 68],
            &[],
            &[],
            &[190, 216],
            &[7, 216, 157, 43, 43, 112],
            &[],
            &[217, 24],
            &[],
            &[11, 103],
        ],
        &[
            (&[], 333981),
            (&[198, 47], 13),
            (&[203, 121], 13),
            (&[77, 29], 12),
            (&[97, 58], 12),
            (&[174, 43], 12),
            (&[80, 107], 12),
            (&[100, 118], 12),
            (&[176, 218], 12),
            (&[203, 110], 12),
        ],
        (&[63, 135], None),
    );
}
