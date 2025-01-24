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
use malachite_base::tuples::random::random_units;
use malachite_base::vecs::random::random_vecs;
use std::fmt::Debug;

fn random_vecs_helper<T: Clone + Debug + Eq + Hash + Ord, I: Clone + Iterator<Item = T>>(
    xs_gen: &dyn Fn(Seed) -> I,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[&[T]],
    expected_common_values: &[(&[T], usize)],
    expected_median: (&[T], Option<&[T]>),
) {
    random_vecs_helper_helper(
        random_vecs(
            EXAMPLE_SEED,
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
fn test_random_vecs() {
    random_vecs_helper(
        &|_| random_units(),
        4,
        1,
        &[
            &[],
            &[(), (), (), (), (), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), ()],
            &[(), (), (), ()],
            &[()],
            &[],
            &[(), (), (), (), ()],
            &[(), ()],
            &[(), (), (), ()],
            &[],
            &[(), (), (), (), (), ()],
            &[],
            &[],
            &[(), (), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), ()],
            &[],
            &[(), (), ()],
            &[],
            &[(), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), ()],
        ],
        &[
            (&[], 199913),
            (&[()], 160173),
            (&[(), ()], 128173),
            (&[(), (), ()], 102460),
            (&[(), (), (), ()], 81463),
            (&[(), (), (), (), ()], 65695),
            (&[(), (), (), (), (), ()], 52495),
            (&[(), (), (), (), (), (), ()], 41943),
            (&[(), (), (), (), (), (), (), ()], 33396),
            (&[(), (), (), (), (), (), (), (), ()], 27035),
        ],
        (&[(), (), ()], None),
    );
    random_vecs_helper(
        &random_primitive_ints::<u8>,
        4,
        1,
        &[
            &[],
            &[85, 11, 136, 200, 235, 134, 203, 223, 38, 235, 217, 177, 162, 32],
            &[166, 234, 30, 218],
            &[90, 106, 9, 216],
            &[204],
            &[],
            &[151, 213, 97, 253, 78],
            &[91, 39],
            &[191, 175, 170, 232],
            &[],
            &[233, 2, 35, 22, 217, 198],
            &[],
            &[],
            &[114, 17, 32, 173, 114, 65, 121, 222, 173, 25, 144],
            &[148, 79, 115, 52, 73, 69, 137, 91],
            &[],
            &[153, 178, 112],
            &[],
            &[34, 95, 106, 167, 197],
            &[130, 168, 122, 207, 172, 177, 86, 150, 221],
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
        (&[96], None),
    );
    random_vecs_helper(
        &|seed| geometric_random_unsigneds::<u32>(seed, 2, 1),
        4,
        1,
        &[
            &[],
            &[5, 0, 0, 1, 1, 1, 4, 2, 4, 6, 2, 2, 0, 1],
            &[9, 13, 0, 0],
            &[2, 0, 7, 4],
            &[6],
            &[],
            &[7, 6, 0, 0, 0],
            &[1, 3],
            &[5, 1, 2, 1],
            &[],
            &[0, 0, 1, 4, 2, 0],
            &[],
            &[],
            &[12, 0, 0, 2, 3, 1, 1, 1, 2, 3, 3],
            &[9, 1, 0, 2, 1, 11, 1, 0],
            &[],
            &[1, 6, 0],
            &[],
            &[3, 18, 3, 3, 0],
            &[5, 1, 2, 5, 0, 0, 2, 3, 1],
        ],
        &[
            (&[], 199913),
            (&[0], 53462),
            (&[1], 35352),
            (&[2], 23512),
            (&[3], 16118),
            (&[0, 0], 14371),
            (&[4], 10594),
            (&[0, 1], 9566),
            (&[1, 0], 9409),
            (&[5], 7157),
        ],
        (&[1], None),
    );
    random_vecs_helper(
        &random_primitive_ints::<u8>,
        1,
        4,
        &[
            &[],
            &[],
            &[85],
            &[11],
            &[136, 200],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[235, 134],
            &[203],
            &[],
            &[223, 38],
            &[],
            &[],
            &[],
            &[],
        ],
        &[
            (&[], 800023),
            (&[8], 704),
            (&[162], 691),
            (&[81], 690),
            (&[211], 690),
            (&[108], 688),
            (&[235], 688),
            (&[35], 687),
            (&[65], 682),
            (&[208], 679),
        ],
        (&[], None),
    );
    random_vecs_helper(
        &|seed| random_char_inclusive_range(seed, 'a', 'z'),
        4,
        1,
        &[
            &[],
            &['v', 'c', 'q', 'i', 'e', 'p', 'g', 's', 'n', 't', 'm', 'z', 'o', 'm'],
            &['f', 'k', 'q', 'y'],
            &['u', 'k', 'x', 'h'],
            &['u'],
            &[],
            &['n', 'n', 'j', 'n', 'j'],
            &['a', 'w'],
            &['z', 'l', 'w', 'b'],
            &[],
            &['l', 'u', 'n', 'e', 'l', 'v'],
            &[],
            &[],
            &['k', 'u', 'h', 'c', 'y', 'i', 'm', 'r', 'm', 'y', 's'],
            &['l', 'e', 'a', 's', 'w', 'k', 'o', 'b'],
            &[],
            &['k', 'w', 'g'],
            &[],
            &['d', 'q', 'e', 'f', 'u'],
            &['z', 'r', 'g', 'j', 'k', 'r', 's', 'y', 'n'],
        ],
        &[
            (&[], 199913),
            (&['o'], 6313),
            (&['y'], 6262),
            (&['q'], 6261),
            (&['j'], 6245),
            (&['p'], 6244),
            (&['g'], 6219),
            (&['x'], 6215),
            (&['e'], 6200),
            (&['t'], 6188),
        ],
        (&['j', 's', 'z'], None),
    );
}

#[test]
#[should_panic]
fn random_vecs_fail_1() {
    random_vecs(EXAMPLE_SEED, &random_primitive_ints::<u32>, 0, 1);
}

#[test]
#[should_panic]
fn random_vecs_fail_2() {
    random_vecs(EXAMPLE_SEED, &random_primitive_ints::<u32>, 1, 0);
}

#[test]
#[should_panic]
fn random_vecs_fail_3() {
    random_vecs(
        EXAMPLE_SEED,
        &random_primitive_ints::<u32>,
        u64::MAX,
        u64::MAX - 1,
    );
}
