// Copyright © 2024 Mikhail Hogrefe
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
use malachite_base::vecs::random::random_unique_vecs_length_inclusive_range;
use std::fmt::Debug;

fn random_unique_vecs_length_inclusive_range_helper<
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
        random_unique_vecs_length_inclusive_range(EXAMPLE_SEED, a, b, xs_gen),
        expected_values,
        expected_common_values,
        expected_median,
    );
}

#[test]
fn test_random_unique_vecs_length_inclusive_range() {
    random_unique_vecs_length_inclusive_range_helper(
        2,
        3,
        &random_primitive_ints::<u8>,
        &[
            &[85, 11, 136],
            &[200, 235],
            &[134, 203, 223],
            &[38, 235, 217],
            &[177, 162],
            &[32, 166, 234],
            &[30, 218, 90],
            &[106, 9],
            &[216, 204, 151],
            &[213, 97, 253],
            &[78, 91],
            &[39, 191, 175],
            &[170, 232],
            &[233, 2, 35],
            &[22, 217, 198],
            &[114, 17],
            &[32, 173],
            &[114, 65],
            &[121, 222, 173],
            &[25, 144],
        ],
        &[
            (&[149, 194], 23),
            (&[237, 224], 23),
            (&[109, 76], 21),
            (&[187, 29], 21),
            (&[96, 105], 21),
            (&[233, 132], 21),
            (&[25, 96], 20),
            (&[92, 85], 20),
            (&[108, 72], 20),
            (&[128, 48], 20),
        ],
        (&[127, 247], None),
    );
    random_unique_vecs_length_inclusive_range_helper(
        2,
        3,
        &|seed| geometric_random_unsigneds::<u32>(seed, 2, 1),
        &[
            &[5, 0, 1],
            &[1, 4],
            &[2, 4, 6],
            &[2, 0, 1],
            &[9, 13],
            &[0, 2, 7],
            &[4, 6, 7],
            &[6, 0],
            &[0, 1, 3],
            &[5, 1, 2],
            &[1, 0],
            &[0, 1, 4],
            &[2, 0],
            &[12, 0, 2],
            &[3, 1, 2],
            &[3, 9],
            &[1, 0],
            &[2, 1],
            &[11, 1, 0],
            &[1, 6],
        ],
        &[
            (&[0, 1], 55434),
            (&[1, 0], 47598),
            (&[0, 2], 37211),
            (&[2, 0], 28974),
            (&[0, 3], 24737),
            (&[1, 2], 21227),
            (&[2, 1], 19153),
            (&[0, 1, 2], 18604),
            (&[3, 0], 18253),
            (&[0, 4], 16195),
        ],
        (&[1, 4], None),
    );
    random_unique_vecs_length_inclusive_range_helper(
        2,
        3,
        &|seed| random_char_inclusive_range(seed, 'a', 'z'),
        &[
            &['v', 'c', 'q'],
            &['i', 'e'],
            &['p', 'g', 's'],
            &['n', 't', 'm'],
            &['z', 'o'],
            &['m', 'f', 'k'],
            &['q', 'y', 'u'],
            &['k', 'x'],
            &['h', 'u', 'n'],
            &['n', 'j', 'a'],
            &['w', 'z'],
            &['l', 'w', 'b'],
            &['l', 'u'],
            &['n', 'e', 'l'],
            &['v', 'k', 'u'],
            &['h', 'c'],
            &['y', 'i'],
            &['m', 'r'],
            &['m', 'y', 's'],
            &['l', 'e'],
        ],
        &[
            (&['i', 'p'], 855),
            (&['o', 't'], 845),
            (&['c', 'i'], 842),
            (&['h', 'u'], 841),
            (&['x', 'l'], 841),
            (&['a', 'o'], 833),
            (&['g', 'h'], 833),
            (&['z', 'n'], 832),
            (&['j', 'n'], 831),
            (&['l', 'c'], 829),
        ],
        (&['m', 'z', 'l'], None),
    );
}

#[test]
#[should_panic]
fn random_unique_vecs_length_inclusive_range_fail() {
    random_unique_vecs_length_inclusive_range(EXAMPLE_SEED, 2, 1, &random_primitive_ints::<u32>);
}
