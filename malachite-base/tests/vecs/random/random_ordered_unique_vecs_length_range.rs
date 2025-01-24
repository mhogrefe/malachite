// Copyright © 2025 Mikhail Hogrefe
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
use malachite_base::random::{Seed, EXAMPLE_SEED};
use malachite_base::test_util::vecs::random::random_vecs_helper_helper;
use malachite_base::vecs::random::random_ordered_unique_vecs_length_range;
use std::fmt::Debug;

fn random_ordered_unique_vecs_length_range_helper<
    T: Clone + Debug + Eq + Hash + Ord,
    I: Clone + Iterator<Item = T>,
>(
    a: u64,
    b: u64,
    xs_gen: &dyn Fn(Seed) -> I,
    expected_values: &[&[T]],
    expected_common_values: &[(&[T], usize)],
    expected_median: (&[T], Option<&[T]>),
) {
    random_vecs_helper_helper(
        random_ordered_unique_vecs_length_range(EXAMPLE_SEED, a, b, xs_gen),
        expected_values,
        expected_common_values,
        expected_median,
    );
}

#[test]
fn test_random_ordered_unique_vecs_length_range() {
    random_ordered_unique_vecs_length_range_helper(
        2,
        4,
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
            (&[106, 108], 34),
            (&[224, 237], 34),
            (&[51, 132], 32),
            (&[82, 117], 32),
            (&[72, 108], 31),
            (&[142, 194], 31),
            (&[0, 34], 30),
            (&[12, 208], 30),
            (&[15, 141], 30),
            (&[30, 248], 30),
        ],
        (&[62, 131, 203], Some(&[62, 131, 205])),
    );
    random_ordered_unique_vecs_length_range_helper(
        2,
        4,
        &|seed| geometric_random_unsigneds::<u32>(seed, 2, 1),
        &[
            &[0, 1, 5],
            &[1, 4],
            &[2, 4, 6],
            &[0, 1, 2],
            &[9, 13],
            &[0, 2, 7],
            &[4, 6, 7],
            &[0, 6],
            &[0, 1, 3],
            &[1, 2, 5],
            &[0, 1],
            &[0, 1, 4],
            &[0, 2],
            &[0, 2, 12],
            &[1, 2, 3],
            &[3, 9],
            &[0, 1],
            &[1, 2],
            &[0, 1, 11],
            &[1, 6],
        ],
        &[
            (&[0, 1], 103032),
            (&[0, 1, 2], 84142),
            (&[0, 2], 66185),
            (&[0, 1, 3], 52638),
            (&[0, 3], 42990),
            (&[1, 2], 40380),
            (&[0, 1, 4], 33815),
            (&[0, 2, 3], 31257),
            (&[0, 4], 28088),
            (&[1, 3], 26214),
        ],
        (&[0, 3], None),
    );
    random_ordered_unique_vecs_length_range_helper(
        2,
        4,
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
            &['a', 'j', 'n'],
            &['w', 'z'],
            &['b', 'l', 'w'],
            &['l', 'u'],
            &['e', 'l', 'n'],
            &['k', 'u', 'v'],
            &['c', 'h'],
            &['i', 'y'],
            &['m', 'r'],
            &['m', 's', 'y'],
            &['e', 'l'],
        ],
        &[
            (&['l', 'x'], 1640),
            (&['o', 't'], 1636),
            (&['b', 'p'], 1630),
            (&['m', 'v'], 1623),
            (&['h', 'u'], 1621),
            (&['a', 'x'], 1614),
            (&['d', 'f'], 1613),
            (&['e', 'r'], 1613),
            (&['o', 'p'], 1612),
            (&['c', 'i'], 1611),
        ],
        (&['g', 'j'], None),
    );
}

#[test]
#[should_panic]
fn random_ordered_unique_vecs_length_range_fail() {
    random_ordered_unique_vecs_length_range(EXAMPLE_SEED, 2, 2, &random_primitive_ints::<u32>);
}
