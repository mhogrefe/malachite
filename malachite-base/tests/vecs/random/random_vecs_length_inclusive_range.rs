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
use malachite_base::tuples::random::random_units;
use malachite_base::vecs::random::random_vecs_length_inclusive_range;
use std::fmt::Debug;

fn random_vecs_length_inclusive_range_helper<
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
        random_vecs_length_inclusive_range(EXAMPLE_SEED, a, b, xs_gen),
        expected_values,
        expected_common_values,
        expected_median,
    );
}

#[test]
fn test_random_vecs_length_inclusive_range() {
    random_vecs_length_inclusive_range_helper(
        2,
        3,
        &|_| random_units(),
        &[
            &[(), (), ()],
            &[(), ()],
            &[(), (), ()],
            &[(), (), ()],
            &[(), ()],
            &[(), (), ()],
            &[(), (), ()],
            &[(), ()],
            &[(), (), ()],
            &[(), (), ()],
            &[(), ()],
            &[(), (), ()],
            &[(), ()],
            &[(), (), ()],
            &[(), (), ()],
            &[(), ()],
            &[(), ()],
            &[(), ()],
            &[(), (), ()],
            &[(), ()],
        ],
        &[(&[(), (), ()], 500363), (&[(), ()], 499637)],
        (&[(), (), ()], None),
    );
    random_vecs_length_inclusive_range_helper(
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
            (&[234, 192], 23),
            (&[0, 40], 19),
            (&[68, 88], 19),
            (&[188, 21], 19),
            (&[215, 22], 19),
            (&[221, 92], 19),
            (&[255, 26], 19),
            (&[34, 253], 19),
            (&[61, 159], 19),
            (&[155, 140], 19),
        ],
        (&[128, 5, 208], Some(&[128, 5, 239])),
    );
    random_vecs_length_inclusive_range_helper(
        2,
        3,
        &|seed| geometric_random_unsigneds::<u32>(seed, 2, 1),
        &[
            &[5, 0, 0],
            &[1, 1],
            &[1, 4, 2],
            &[4, 6, 2],
            &[2, 0],
            &[1, 9, 13],
            &[0, 0, 2],
            &[0, 7],
            &[4, 6, 7],
            &[6, 0, 0],
            &[0, 1],
            &[3, 5, 1],
            &[2, 1],
            &[0, 0, 1],
            &[4, 2, 0],
            &[12, 0],
            &[0, 2],
            &[3, 1],
            &[1, 1, 2],
            &[3, 3],
        ],
        &[
            (&[0, 0], 55357),
            (&[0, 1], 37179),
            (&[1, 0], 37106),
            (&[0, 2], 24784),
            (&[2, 0], 24772),
            (&[1, 1], 24686),
            (&[0, 0, 0], 18703),
            (&[3, 0], 16656),
            (&[2, 1], 16622),
            (&[1, 2], 16275),
        ],
        (&[1, 3], None),
    );
    random_vecs_length_inclusive_range_helper(
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
            &['n', 'j', 'n'],
            &['j', 'a'],
            &['w', 'z', 'l'],
            &['w', 'b'],
            &['l', 'u', 'n'],
            &['e', 'l', 'v'],
            &['k', 'u'],
            &['h', 'c'],
            &['y', 'i'],
            &['m', 'r', 'm'],
            &['y', 's'],
        ],
        &[
            (&['w', 'o'], 822),
            (&['f', 's'], 814),
            (&['w', 'u'], 810),
            (&['g', 'c'], 806),
            (&['w', 'f'], 806),
            (&['m', 'z'], 805),
            (&['q', 'k'], 805),
            (&['i', 'b'], 802),
            (&['u', 'k'], 800),
            (&['h', 'p'], 798),
        ],
        (&['m', 'z', 'w'], None),
    );
}

#[test]
#[should_panic]
fn random_vecs_length_inclusive_range_fail() {
    random_vecs_length_inclusive_range(EXAMPLE_SEED, 2, 1, &random_primitive_ints::<u32>);
}
