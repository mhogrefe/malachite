// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::hash::Hash;
use malachite_base::bools::random::random_bools;
use malachite_base::num::random::{random_primitive_ints, random_unsigned_inclusive_range};
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::vecs::random::random_vecs_helper_helper;
use malachite_base::vecs::random::random_ordered_vecs_fixed_length;
use std::fmt::Debug;

fn random_ordered_vecs_fixed_length_helper<I: Clone + Iterator>(
    len: u64,
    xs: I,
    expected_values: &[&[I::Item]],
    expected_common_values: &[(&[I::Item], usize)],
    expected_median: (&[I::Item], Option<&[I::Item]>),
) where
    I::Item: Clone + Debug + Eq + Hash + Ord,
{
    random_vecs_helper_helper(
        random_ordered_vecs_fixed_length(len, xs),
        expected_values,
        expected_common_values,
        expected_median,
    );
}

#[test]
fn test_random_ordered_vecs_fixed_length() {
    random_ordered_vecs_fixed_length_helper(
        0,
        random_primitive_ints::<u8>(EXAMPLE_SEED),
        &[
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
        ],
        &[(&[], 1000000)],
        (&[], None),
    );
    random_ordered_vecs_fixed_length_helper(
        1,
        random_bools(EXAMPLE_SEED),
        &[
            &[true],
            &[false],
            &[false],
            &[false],
            &[true],
            &[true],
            &[true],
            &[false],
            &[true],
            &[true],
            &[true],
            &[true],
            &[false],
            &[true],
            &[true],
            &[true],
            &[true],
            &[false],
            &[true],
            &[false],
        ],
        &[(&[true], 500473), (&[false], 499527)],
        (&[true], None),
    );
    random_ordered_vecs_fixed_length_helper(
        2,
        random_bools(EXAMPLE_SEED),
        &[
            &[false, true],
            &[false, false],
            &[true, true],
            &[false, true],
            &[true, true],
            &[true, true],
            &[false, true],
            &[true, true],
            &[false, true],
            &[false, true],
            &[false, false],
            &[false, true],
            &[false, false],
            &[true, true],
            &[false, true],
            &[false, true],
            &[false, false],
            &[false, true],
            &[false, true],
            &[true, true],
        ],
        &[(&[false, true], 500269), (&[true, true], 250015), (&[false, false], 249716)],
        (&[false, true], None),
    );
    random_ordered_vecs_fixed_length_helper(
        3,
        random_primitive_ints::<u8>(EXAMPLE_SEED),
        &[
            &[69, 113, 239],
            &[108, 210, 228],
            &[87, 161, 168],
            &[32, 83, 110],
            &[34, 89, 188],
            &[93, 200, 238],
            &[115, 149, 189],
            &[149, 201, 217],
            &[31, 117, 146],
            &[72, 151, 169],
            &[7, 33, 174],
            &[38, 81, 144],
            &[72, 113, 127],
            &[107, 128, 233],
            &[12, 46, 119],
            &[18, 164, 243],
            &[59, 114, 174],
            &[39, 174, 247],
            &[104, 160, 184],
            &[37, 100, 252],
        ],
        &[
            (&[35, 122, 231], 7),
            (&[43, 79, 84], 6),
            (&[19, 93, 252], 6),
            (&[36, 70, 195], 6),
            (&[73, 168, 192], 6),
            (&[1, 44, 56], 5),
            (&[10, 11, 28], 5),
            (&[17, 41, 94], 5),
            (&[23, 38, 62], 5),
            (&[23, 59, 65], 5),
        ],
        (&[52, 167, 253], Some(&[52, 168, 168])),
    );
    random_ordered_vecs_fixed_length_helper(
        3,
        random_unsigned_inclusive_range::<u8>(EXAMPLE_SEED, 1, 3),
        &[
            &[1, 2, 2],
            &[2, 2, 3],
            &[1, 1, 2],
            &[1, 2, 3],
            &[2, 3, 3],
            &[1, 1, 2],
            &[3, 3, 3],
            &[1, 2, 3],
            &[2, 2, 3],
            &[1, 1, 2],
            &[1, 3, 3],
            &[1, 2, 3],
            &[1, 2, 2],
            &[1, 3, 3],
            &[1, 2, 3],
            &[2, 2, 3],
            &[2, 3, 3],
            &[1, 2, 2],
            &[1, 2, 3],
            &[2, 2, 3],
        ],
        &[
            (&[1, 2, 3], 222772),
            (&[1, 2, 2], 111638),
            (&[2, 3, 3], 111135),
            (&[2, 2, 3], 111082),
            (&[1, 1, 3], 111039),
            (&[1, 1, 2], 110779),
            (&[1, 3, 3], 110516),
            (&[2, 2, 2], 37096),
            (&[3, 3, 3], 36993),
            (&[1, 1, 1], 36950),
        ],
        (&[1, 2, 3], None),
    );
}
