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
use malachite_base::vecs::random::random_ordered_unique_vecs_fixed_length;
use std::fmt::Debug;

fn random_ordered_unique_vecs_fixed_length_helper<I: Clone + Iterator>(
    len: u64,
    xs: I,
    expected_values: &[&[I::Item]],
    expected_common_values: &[(&[I::Item], usize)],
    expected_median: (&[I::Item], Option<&[I::Item]>),
) where
    I::Item: Clone + Debug + Eq + Hash + Ord,
{
    random_vecs_helper_helper(
        random_ordered_unique_vecs_fixed_length(len, xs),
        expected_values,
        expected_common_values,
        expected_median,
    );
}

#[test]
fn test_random_ordered_unique_vecs_fixed_length() {
    random_ordered_unique_vecs_fixed_length_helper(
        0,
        random_primitive_ints::<u8>(EXAMPLE_SEED),
        repeat_n(&[][..], 20).collect_vec().as_slice(),
        &[(&[], 1000000)],
        (&[], None),
    );
    random_ordered_unique_vecs_fixed_length_helper(
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
    random_ordered_unique_vecs_fixed_length_helper(
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
            (&[57, 142, 207], 7),
            (&[32, 68, 169], 6),
            (&[36, 70, 195], 6),
            (&[125, 168, 194], 6),
            (&[0, 97, 205], 5),
            (&[2, 33, 227], 5),
            (&[5, 46, 239], 5),
            (&[9, 68, 189], 5),
            (&[9, 78, 240], 5),
            (&[1, 110, 203], 5),
        ],
        (&[52, 133, 241], Some(&[52, 133, 242])),
    );
    random_ordered_unique_vecs_fixed_length_helper(
        2,
        random_ordered_unique_vecs_fixed_length(2, random_primitive_ints::<u8>(EXAMPLE_SEED)),
        &[
            &[vec![69, 108], vec![113, 239]],
            &[vec![161, 168], vec![210, 228]],
            &[vec![32, 87], vec![83, 110]],
            &[vec![34, 188], vec![89, 238]],
            &[vec![93, 200], vec![115, 149]],
            &[vec![149, 189], vec![201, 217]],
            &[vec![31, 72], vec![117, 146]],
            &[vec![33, 174], vec![151, 169]],
            &[vec![7, 38], vec![81, 144]],
            &[vec![72, 127], vec![113, 128]],
            &[vec![46, 119], vec![107, 233]],
            &[vec![12, 18], vec![164, 243]],
            &[vec![59, 247], vec![114, 174]],
            &[vec![39, 174], vec![160, 184]],
            &[vec![37, 104], vec![100, 252]],
            &[vec![69, 107], vec![122, 228]],
            &[vec![142, 179], vec![242, 248]],
            &[vec![61, 189], vec![233, 239]],
            &[vec![7, 192], vec![85, 235]],
            &[vec![90, 200], vec![178, 185]],
        ],
        &[
            (&[vec![0, 78], vec![34, 52]], 2),
            (&[vec![1, 58], vec![6, 112]], 2),
            (&[vec![1, 63], vec![8, 154]], 2),
            (&[vec![1, 97], vec![7, 250]], 2),
            (&[vec![2, 33], vec![40, 81]], 2),
            (&[vec![3, 160], vec![7, 29]], 2),
            (&[vec![3, 32], vec![12, 60]], 2),
            (&[vec![6, 130], vec![7, 20]], 2),
            (&[vec![6, 68], vec![7, 126]], 2),
            (&[vec![6, 77], vec![36, 54]], 2),
        ],
        (
            &[vec![40, 193], vec![94, 142]],
            Some(&[vec![40, 193], vec![97, 243]]),
        ),
    );
}
