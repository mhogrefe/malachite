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
use malachite_base::test_util::vecs::random::random_vecs_helper_helper;
use malachite_base::vecs::random::random_unique_vecs_fixed_length;
use std::fmt::Debug;

fn random_unique_vecs_fixed_length_helper<I: Clone + Iterator>(
    len: u64,
    xs: I,
    expected_values: &[&[I::Item]],
    expected_common_values: &[(&[I::Item], usize)],
    expected_median: (&[I::Item], Option<&[I::Item]>),
) where
    I::Item: Clone + Debug + Eq + Hash + Ord,
{
    random_vecs_helper_helper(
        random_unique_vecs_fixed_length(len, xs),
        expected_values,
        expected_common_values,
        expected_median,
    );
}

#[test]
fn test_random_unique_vecs_fixed_length() {
    random_unique_vecs_fixed_length_helper(
        0,
        random_primitive_ints::<u8>(EXAMPLE_SEED),
        repeat_n(&[][..], 20).collect_vec().as_slice(),
        &[(&[], 1000000)],
        (&[], None),
    );
    random_unique_vecs_fixed_length_helper(
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
    random_unique_vecs_fixed_length_helper(
        3,
        random_primitive_ints::<u8>(EXAMPLE_SEED),
        &[
            &[113, 239, 69],
            &[108, 228, 210],
            &[168, 161, 87],
            &[32, 110, 83],
            &[188, 34, 89],
            &[238, 93, 200],
            &[149, 115, 189],
            &[149, 217, 201],
            &[117, 146, 31],
            &[72, 151, 169],
            &[174, 33, 7],
            &[38, 81, 144],
            &[72, 127, 113],
            &[128, 233, 107],
            &[46, 119, 12],
            &[18, 164, 243],
            &[114, 174, 59],
            &[247, 39, 174],
            &[160, 184, 104],
            &[37, 100, 252],
        ],
        &[
            (&[205, 0, 97], 4),
            (&[102, 18, 19], 4),
            (&[105, 70, 13], 4),
            (&[22, 45, 192], 4),
            (&[87, 100, 26], 4),
            (&[15, 107, 109], 4),
            (&[134, 245, 157], 4),
            (&[138, 164, 179], 4),
            (&[219, 253, 196], 4),
            (&[237, 197, 239], 4),
        ],
        (&[128, 16, 107], Some(&[128, 16, 116])),
    );
    random_unique_vecs_fixed_length_helper(
        2,
        random_unique_vecs_fixed_length(2, random_primitive_ints::<u8>(EXAMPLE_SEED)),
        &[
            &[vec![113, 239], vec![69, 108]],
            &[vec![228, 210], vec![168, 161]],
            &[vec![87, 32], vec![110, 83]],
            &[vec![188, 34], vec![89, 238]],
            &[vec![93, 200], vec![149, 115]],
            &[vec![189, 149], vec![217, 201]],
            &[vec![117, 146], vec![31, 72]],
            &[vec![151, 169], vec![174, 33]],
            &[vec![7, 38], vec![81, 144]],
            &[vec![72, 127], vec![113, 128]],
            &[vec![233, 107], vec![46, 119]],
            &[vec![12, 18], vec![164, 243]],
            &[vec![114, 174], vec![59, 247]],
            &[vec![39, 174], vec![160, 184]],
            &[vec![104, 37], vec![100, 252]],
            &[vec![228, 122], vec![107, 69]],
            &[vec![242, 248], vec![179, 142]],
            &[vec![239, 233], vec![61, 189]],
            &[vec![235, 85], vec![192, 7]],
            &[vec![200, 90], vec![185, 178]],
        ],
        &[
            (&[vec![60, 12], vec![3, 32]], 2),
            (&[vec![0, 80], vec![88, 210]], 2),
            (&[vec![1, 3], vec![216, 183]], 2),
            (&[vec![159, 0], vec![69, 30]], 2),
            (&[vec![199, 6], vec![95, 79]], 2),
            (&[vec![2, 98], vec![221, 19]], 2),
            (&[vec![212, 65], vec![99, 2]], 2),
            (&[vec![3, 14], vec![61, 170]], 2),
            (&[vec![41, 155], vec![3, 72]], 2),
            (&[vec![47, 85], vec![69, 66]], 2),
        ],
        (
            &[vec![128, 41], vec![252, 44]],
            Some(&[vec![128, 42], vec![8, 241]]),
        ),
    );
}
