// Copyright © 2026 Mikhail Hogrefe
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
use malachite_base::random::{EXAMPLE_SEED, Seed};
use malachite_base::test_util::vecs::random::random_vecs_helper_helper;
use malachite_base::vecs::random::random_ordered_vecs_from_length_iterator;
use malachite_base::vecs::random_values_from_vec;
use std::fmt::Debug;

fn random_ordered_vecs_from_length_iterator_helper<
    I: Clone + Iterator<Item = u64>,
    J: Clone + Iterator,
>(
    lengths_gen: &dyn Fn(Seed) -> I,
    xs_gen: &dyn Fn(Seed) -> J,
    expected_values: &[&[J::Item]],
    expected_common_values: &[(&[J::Item], usize)],
    expected_median: (&[J::Item], Option<&[J::Item]>),
) where
    J::Item: Clone + Debug + Eq + Hash + Ord,
{
    random_vecs_helper_helper(
        random_ordered_vecs_from_length_iterator(EXAMPLE_SEED, lengths_gen, xs_gen),
        expected_values,
        expected_common_values,
        expected_median,
    );
}

#[test]
fn test_random_ordered_vecs_from_length_iterator() {
    random_ordered_vecs_from_length_iterator_helper(
        &|seed| random_values_from_vec(seed, vec![0, 2]),
        &random_bools,
        &[
            &[false, true],
            &[],
            &[false, true],
            &[false, true],
            &[],
            &[false, true],
            &[true, true],
            &[],
            &[false, true],
            &[false, false],
            &[],
            &[false, false],
            &[],
            &[false, false],
            &[false, true],
            &[],
            &[],
            &[],
            &[false, false],
            &[],
        ],
        &[
            (&[], 499637),
            (&[false, true], 250305),
            (&[false, false], 125346),
            (&[true, true], 124712),
        ],
        (&[false, false], None),
    );
    random_ordered_vecs_from_length_iterator_helper(
        &|seed| geometric_random_unsigneds::<u64>(seed, 2, 1).map(|x| x << 1),
        &random_primitive_ints::<u8>,
        &[
            &[11, 38, 85, 134, 136, 177, 200, 203, 217, 223, 235, 235],
            &[32, 162],
            &[9, 30, 39, 78, 90, 91, 97, 106, 151, 166, 204, 213, 216, 218, 234, 253],
            &[175, 191],
            &[
                2, 17, 22, 25, 32, 35, 52, 65, 69, 73, 79, 91, 114, 114, 115, 121, 137, 144, 148,
                153, 170, 173, 173, 198, 217, 222, 232, 233,
            ],
            &[],
            &[34, 95, 106, 112, 122, 130, 167, 168, 178, 197],
            &[86, 101, 150, 172, 177, 207, 218, 221],
            &[74, 115],
            &[],
            &[9, 48, 52, 109, 123, 133, 159, 196, 201, 235, 247, 250],
            &[40, 68, 97, 104],
            &[],
            &[],
            &[190, 216],
            &[7, 43, 43, 112, 157, 216],
            &[],
            &[24, 217],
            &[],
            &[11, 103],
        ],
        &[
            (&[], 333981),
            (&[71, 131], 19),
            (&[28, 56], 18),
            (&[33, 167], 18),
            (&[12, 53], 17),
            (&[68, 78], 17),
            (&[44, 132], 17),
            (&[44, 174], 17),
            (&[47, 198], 17),
            (&[49, 157], 17),
        ],
        (
            &[13, 17, 37, 49, 60, 71, 79, 93, 187, 243],
            Some(&[13, 17, 37, 75, 84, 127, 169, 192]),
        ),
    );
}
