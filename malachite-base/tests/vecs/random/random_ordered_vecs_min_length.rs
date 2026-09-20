// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::hash::Hash;
use malachite_base::num::random::geometric::geometric_random_unsigneds;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::{EXAMPLE_SEED, Seed};
use malachite_base::test_util::vecs::random::random_vecs_helper_helper;
use malachite_base::vecs::random::random_ordered_vecs_min_length;
use std::fmt::Debug;

fn random_ordered_vecs_min_length_helper<I: Clone + Iterator>(
    min_length: u64,
    xs_gen: &dyn Fn(Seed) -> I,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[&[I::Item]],
    expected_common_values: &[(&[I::Item], usize)],
    expected_median: (&[I::Item], Option<&[I::Item]>),
) where
    I::Item: Clone + Debug + Eq + Hash + Ord,
{
    random_vecs_helper_helper(
        random_ordered_vecs_min_length(
            EXAMPLE_SEED,
            min_length,
            xs_gen,
            mean_length_numerator,
            mean_length_denominator,
        ),
        expected_values,
        expected_common_values,
        expected_median,
    );
}

#[test]
fn test_random_ordered_vecs_min_length() {
    random_ordered_vecs_min_length_helper(
        0,
        &random_primitive_ints::<u8>,
        4,
        1,
        &[
            &[],
            &[11, 32, 38, 85, 134, 136, 162, 177, 200, 203, 217, 223, 235, 235],
            &[30, 166, 218, 234],
            &[9, 90, 106, 216],
            &[204],
            &[],
            &[78, 97, 151, 213, 253],
            &[39, 91],
            &[170, 175, 191, 232],
            &[],
            &[2, 22, 35, 198, 217, 233],
            &[],
            &[],
            &[17, 25, 32, 65, 114, 114, 121, 144, 173, 173, 222],
            &[52, 69, 73, 79, 91, 115, 137, 148],
            &[],
            &[112, 153, 178],
            &[],
            &[34, 95, 106, 167, 197],
            &[86, 122, 130, 150, 168, 172, 177, 207, 221],
        ],
        &[
            (&[], 199913),
            (&[146], 693),
            (&[26], 692),
            (&[185], 688),
            (&[58], 683),
            (&[196], 683),
            (&[81], 678),
            (&[229], 675),
            (&[192], 673),
            (&[233], 673),
        ],
        (
            &[27, 52, 108, 156, 187, 196, 197, 231],
            Some(&[27, 52, 109, 208, 221, 243]),
        ),
    );
    random_ordered_vecs_min_length_helper(
        3,
        &random_primitive_ints::<u8>,
        7,
        1,
        &[
            &[11, 85, 136],
            &[30, 32, 38, 90, 106, 134, 162, 166, 177, 200, 203, 217, 218, 223, 234, 235, 235],
            &[9, 97, 151, 204, 213, 216, 253],
            &[39, 78, 91, 170, 175, 191, 232],
            &[2, 22, 35, 233],
            &[114, 198, 217],
            &[17, 32, 65, 114, 121, 173, 173, 222],
            &[25, 79, 115, 144, 148],
            &[52, 69, 73, 91, 137, 153, 178],
            &[34, 95, 112],
            &[106, 122, 130, 167, 168, 172, 177, 197, 207],
            &[86, 150, 221],
            &[101, 115, 218],
            &[9, 40, 48, 52, 74, 109, 123, 133, 159, 196, 201, 235, 247, 250],
            &[7, 43, 43, 68, 97, 104, 112, 157, 190, 216, 216],
            &[11, 24, 217],
            &[29, 55, 84, 103, 135, 211],
            &[65, 89, 206],
            &[9, 22, 22, 34, 51, 79, 148, 191],
            &[3, 20, 32, 47, 50, 62, 114, 118, 120, 166, 176, 194],
        ],
        &[
            (&[28, 36, 225], 4),
            (&[34, 39, 234], 4),
            (&[84, 171, 239], 4),
            (&[1, 18, 93], 3),
            (&[2, 3, 198], 3),
            (&[1, 91, 245], 3),
            (&[14, 24, 54], 3),
            (&[14, 47, 70], 3),
            (&[19, 81, 87], 3),
            (&[2, 62, 212], 3),
        ],
        (
            &[27, 39, 53, 61, 120, 121, 135, 163, 185],
            Some(&[27, 39, 53, 73, 94, 131, 149, 157, 177, 181, 207, 208]),
        ),
    );
    random_ordered_vecs_min_length_helper(
        0,
        &|seed| geometric_random_unsigneds::<u32>(seed, 32, 1),
        4,
        1,
        &[
            &[],
            &[1, 9, 12, 14, 16, 17, 19, 21, 41, 42, 68, 79, 124, 141],
            &[0, 1, 10, 99],
            &[2, 12, 36, 77],
            &[1],
            &[],
            &[1, 5, 9, 19, 103],
            &[6, 7],
            &[15, 18, 51, 159],
            &[],
            &[2, 26, 40, 52, 64, 75],
            &[],
            &[],
            &[3, 4, 5, 7, 30, 31, 34, 43, 49, 51, 67],
            &[1, 14, 16, 24, 29, 41, 47, 52],
            &[],
            &[11, 13, 62],
            &[],
            &[3, 14, 42, 47, 109],
            &[5, 13, 16, 25, 37, 41, 42, 86, 96],
        ],
        &[
            (&[], 199913),
            (&[0], 4842),
            (&[1], 4831),
            (&[2], 4623),
            (&[3], 4460),
            (&[4], 4197),
            (&[5], 4141),
            (&[6], 4106),
            (&[7], 3872),
            (&[8], 3839),
        ],
        (
            &[3, 12, 20, 23, 24, 68, 70, 99],
            Some(&[3, 12, 20, 23, 26, 113]),
        ),
    );
    random_ordered_vecs_min_length_helper(
        3,
        &random_primitive_ints::<u8>,
        13,
        4,
        &[
            &[11, 85, 136],
            &[134, 200, 235],
            &[38, 203, 223, 235],
            &[32, 162, 177, 217],
            &[30, 90, 166, 218, 234],
            &[9, 106, 216],
            &[151, 204, 213],
            &[78, 97, 253],
            &[39, 91, 191],
            &[170, 175, 232],
            &[2, 35, 233],
            &[22, 198, 217],
            &[17, 32, 114, 114, 173],
            &[65, 121, 173, 222],
            &[25, 144, 148],
            &[52, 69, 73, 79, 115],
            &[91, 137, 153],
            &[34, 112, 178],
            &[95, 106, 167],
            &[130, 168, 197],
        ],
        &[
            (&[1, 75, 139], 6),
            (&[96, 160, 226], 6),
            (&[31, 47, 60], 5),
            (&[36, 54, 98], 5),
            (&[7, 39, 239], 5),
            (&[9, 61, 109], 5),
            (&[10, 21, 199], 5),
            (&[10, 94, 152], 5),
            (&[25, 83, 122], 5),
            (&[35, 65, 120], 5),
        ],
        (&[49, 100, 219], Some(&[49, 100, 223])),
    );
}
