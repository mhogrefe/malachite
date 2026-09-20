// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::hash::Hash;
use malachite_base::chars::random::random_char_inclusive_range;
use malachite_base::num::random::geometric::geometric_random_unsigneds;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::{EXAMPLE_SEED, Seed};
use malachite_base::test_util::vecs::random::random_vecs_helper_helper;
use malachite_base::vecs::random::random_ordered_vecs_length_inclusive_range;
use std::fmt::Debug;

fn random_ordered_vecs_length_inclusive_range_helper<I: Clone + Iterator>(
    a: u64,
    b: u64,
    xs_gen: &dyn Fn(Seed) -> I,
    expected_values: &[&[I::Item]],
    expected_common_values: &[(&[I::Item], usize)],
    expected_median: (&[I::Item], Option<&[I::Item]>),
) where
    I::Item: Clone + Debug + Eq + Hash + Ord,
{
    random_vecs_helper_helper(
        random_ordered_vecs_length_inclusive_range(EXAMPLE_SEED, a, b, xs_gen),
        expected_values,
        expected_common_values,
        expected_median,
    );
}

#[test]
fn test_random_ordered_vecs_length_inclusive_range() {
    random_ordered_vecs_length_inclusive_range_helper(
        2,
        3,
        &random_primitive_ints::<u8>,
        &[
            &[11, 85, 136],
            &[200, 235],
            &[134, 203, 223],
            &[38, 217, 235],
            &[162, 177],
            &[32, 166, 234],
            &[30, 90, 218],
            &[9, 106],
            &[151, 204, 216],
            &[97, 213, 253],
            &[78, 91],
            &[39, 175, 191],
            &[170, 232],
            &[2, 35, 233],
            &[22, 198, 217],
            &[17, 114],
            &[32, 173],
            &[65, 114],
            &[121, 173, 222],
            &[25, 144],
        ],
        &[
            (&[26, 255], 32),
            (&[192, 234], 32),
            (&[65, 200], 31),
            (&[34, 253], 30),
            (&[39, 134], 30),
            (&[213, 224], 30),
            (&[3, 14], 29),
            (&[7, 85], 29),
            (&[57, 60], 29),
            (&[87, 96], 29),
        ],
        (&[62, 162], None),
    );
    random_ordered_vecs_length_inclusive_range_helper(
        2,
        3,
        &|seed| geometric_random_unsigneds::<u32>(seed, 2, 1),
        &[
            &[0, 0, 5],
            &[1, 1],
            &[1, 2, 4],
            &[2, 4, 6],
            &[0, 2],
            &[1, 9, 13],
            &[0, 0, 2],
            &[0, 7],
            &[4, 6, 7],
            &[0, 0, 6],
            &[0, 1],
            &[1, 3, 5],
            &[1, 2],
            &[0, 0, 1],
            &[0, 2, 4],
            &[0, 12],
            &[0, 2],
            &[1, 3],
            &[1, 1, 2],
            &[3, 3],
        ],
        &[
            (&[0, 1], 74285),
            (&[0, 0], 55357),
            (&[0, 2], 49556),
            (&[0, 0, 1], 37104),
            (&[0, 3], 32930),
            (&[1, 2], 32897),
            (&[0, 1, 2], 32862),
            (&[0, 0, 2], 24955),
            (&[0, 1, 1], 24882),
            (&[1, 1], 24686),
        ],
        (&[0, 3], None),
    );
    random_ordered_vecs_length_inclusive_range_helper(
        2,
        3,
        &|seed| random_char_inclusive_range(seed, 'a', 'z'),
        &[
            &['c', 'q', 'v'],
            &['e', 'i'],
            &['g', 'p', 's'],
            &['m', 'n', 't'],
            &['o', 'z'],
            &['f', 'k', 'm'],
            &['q', 'u', 'y'],
            &['k', 'x'],
            &['h', 'n', 'u'],
            &['j', 'n', 'n'],
            &['a', 'j'],
            &['l', 'w', 'z'],
            &['b', 'w'],
            &['l', 'n', 'u'],
            &['e', 'l', 'v'],
            &['k', 'u'],
            &['c', 'h'],
            &['i', 'y'],
            &['m', 'm', 'r'],
            &['s', 'y'],
        ],
        &[
            (&['k', 'q'], 1590),
            (&['h', 'p'], 1583),
            (&['f', 'i'], 1567),
            (&['f', 's'], 1564),
            (&['x', 'y'], 1564),
            (&['j', 's'], 1559),
            (&['g', 'p'], 1557),
            (&['l', 'r'], 1556),
            (&['b', 'c'], 1555),
            (&['b', 'i'], 1554),
        ],
        (&['g', 'l', 't'], None),
    );
}
