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
use malachite_base::vecs::random::random_vecs_fixed_length_from_single;
use std::fmt::Debug;

fn random_vecs_fixed_length_from_single_helper<I: Clone + Iterator>(
    len: u64,
    xs: I,
    expected_values: &[&[I::Item]],
    expected_common_values: &[(&[I::Item], usize)],
    expected_median: (&[I::Item], Option<&[I::Item]>),
) where
    I::Item: Clone + Debug + Eq + Hash + Ord,
{
    random_vecs_helper_helper(
        random_vecs_fixed_length_from_single(len, xs),
        expected_values,
        expected_common_values,
        expected_median,
    );
}

#[test]
fn test_random_vecs_fixed_length_from_single() {
    random_vecs_fixed_length_from_single_helper(
        0,
        random_primitive_ints::<u8>(EXAMPLE_SEED),
        repeat_n(&[][..], 20).collect_vec().as_slice(),
        &[(&[], 1000000)],
        (&[], None),
    );
    random_vecs_fixed_length_from_single_helper(
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
    random_vecs_fixed_length_from_single_helper(
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
            (&[222, 60, 79], 4),
            (&[26, 110, 13], 4),
            (&[41, 254, 55], 4),
            (&[109, 134, 76], 4),
            (&[165, 174, 73], 4),
            (&[236, 57, 174], 4),
            (&[73, 168, 192], 4),
            (&[89, 197, 244], 4),
            (&[91, 170, 115], 4),
            (&[142, 168, 231], 4),
        ],
        (&[127, 253, 76], Some(&[127, 253, 86])),
    );
    random_vecs_fixed_length_from_single_helper(
        2,
        random_vecs_fixed_length_from_single(2, random_primitive_ints::<u8>(EXAMPLE_SEED)),
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
            (&[vec![28, 96], vec![0, 11]], 2),
            (&[vec![2, 43], vec![64, 233]], 2),
            (&[vec![20, 33], vec![14, 10]], 2),
            (&[vec![223, 84], vec![7, 22]], 2),
            (&[vec![43, 33], vec![131, 6]], 2),
            (&[vec![6, 233], vec![45, 89]], 2),
            (&[vec![65, 26], vec![6, 146]], 2),
            (&[vec![71, 80], vec![68, 88]], 2),
            (&[vec![9, 85], vec![186, 55]], 2),
            (&[vec![96, 254], vec![9, 37]], 2),
        ],
        (
            &[vec![127, 243], vec![125, 130]],
            Some(&[vec![127, 243], vec![134, 100]]),
        ),
    );
}
