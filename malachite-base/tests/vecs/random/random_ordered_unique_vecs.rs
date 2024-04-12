// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::hash::Hash;
use malachite_base::chars::exhaustive::exhaustive_chars;
use malachite_base::chars::random::graphic_weighted_random_char_inclusive_range;
use malachite_base::num::random::geometric::geometric_random_unsigneds;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::{Seed, EXAMPLE_SEED};
use malachite_base::test_util::vecs::random::random_vecs_helper_helper;
use malachite_base::vecs::random::random_ordered_unique_vecs;
use std::fmt::Debug;

fn random_ordered_unique_vecs_helper<
    T: Clone + Debug + Eq + Hash + Ord,
    I: Clone + Iterator<Item = T>,
>(
    xs_gen: &dyn Fn(Seed) -> I,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[&[T]],
    expected_common_values: &[(&[T], usize)],
    expected_median: (&[T], Option<&[T]>),
) {
    random_vecs_helper_helper(
        random_ordered_unique_vecs(
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
fn test_random_ordered_unique_vecs() {
    random_ordered_unique_vecs_helper(
        &random_primitive_ints::<u8>,
        4,
        1,
        &[
            &[][..],
            &[11, 32, 38, 85, 134, 136, 162, 166, 177, 200, 203, 217, 223, 235],
            &[30, 90, 218, 234],
            &[9, 106, 204, 216],
            &[151],
            &[],
            &[78, 91, 97, 213, 253],
            &[39, 191],
            &[170, 175, 232, 233],
            &[],
            &[2, 22, 35, 114, 198, 217],
            &[],
            &[],
            &[17, 25, 32, 65, 79, 114, 121, 144, 148, 173, 222],
            &[52, 69, 73, 91, 115, 137, 153, 178],
            &[],
            &[34, 95, 112],
            &[],
            &[106, 130, 167, 168, 197],
            &[86, 101, 122, 150, 172, 177, 207, 218, 221],
        ],
        &[
            (&[], 199913),
            (&[7], 705),
            (&[25], 689),
            (&[184], 681),
            (&[213], 681),
            (&[255], 676),
            (&[215], 675),
            (&[54], 673),
            (&[122], 672),
            (&[207], 672),
        ],
        (&[27, 31, 211, 238], Some(&[27, 31, 247, 251])),
    );
    random_ordered_unique_vecs_helper(
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
            (&[0], 4861),
            (&[1], 4593),
            (&[2], 4498),
            (&[3], 4405),
            (&[4], 4330),
            (&[5], 4078),
            (&[6], 4050),
            (&[7], 3858),
            (&[8], 3848),
        ],
        (
            &[3, 9, 14, 22, 36, 56, 107],
            Some(&[3, 9, 14, 22, 42, 54, 73, 150]),
        ),
    );
    random_ordered_unique_vecs_helper(
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
            &[134, 235],
            &[203],
            &[],
            &[38, 223],
            &[],
            &[],
            &[],
            &[],
        ],
        &[
            (&[], 800023),
            (&[162], 692),
            (&[235], 690),
            (&[90], 688),
            (&[65], 687),
            (&[249], 686),
            (&[175], 684),
            (&[108], 683),
            (&[211], 682),
            (&[237], 680),
        ],
        (&[], None),
    );
    random_ordered_unique_vecs_helper(
        &|seed| {
            graphic_weighted_random_char_inclusive_range(
                seed,
                'a',
                exhaustive_chars().nth(200).unwrap(),
                1,
                1,
            )
        },
        4,
        1,
        &[
            &[],
            &['g', 'q', '³', '»', 'À', 'Á', 'Ã', 'È', 'á', 'â', 'ì', 'ñ', 'Ā', 'ą'],
            &['ª', '´', 'Ã', 'ä'],
            &['½', 'Á', 'Ï', 'ý'],
            &['j'],
            &[],
            &['u', '½', 'Â', 'Ñ', 'ï'],
            &['x', 'õ'],
            &['¡', 'Â', 'ù', 'Ċ'],
            &[],
            &['b', 'r', 's', '¬', 'Â', 'Ñ'],
            &[],
            &[],
            &['j', 'n', 't', '¬', 'º', '¿', 'Á', 'Ø', 'Þ', 'ô', 'ü'],
            &['b', 'k', '±', 'Î', 'Ü', 'æ', 'è', 'ā'],
            &[],
            &['«', '¹', 'Î'],
            &[],
            &['~', '¯', '´', 'Ý', 'â'],
            &['g', '¼', 'Ç', 'Î', 'Ü', 'Þ', 'æ', 'é', 'ö'],
        ],
        &[
            (&[], 199913),
            (&['Ó'], 1270),
            (&['Â'], 1249),
            (&['§'], 1244),
            (&['¿'], 1243),
            (&['õ'], 1241),
            (&['ĉ'], 1234),
            (&['¤'], 1232),
            (&['¼'], 1232),
            (&['Ì'], 1229),
        ],
        (
            &['o', 'v', '¢', '±', 'Ä', 'Ć'],
            Some(&['o', 'v', '¢', '³', 'ã']),
        ),
    );
}

#[test]
#[should_panic]
fn random_ordered_unique_vecs_fail_1() {
    random_ordered_unique_vecs(EXAMPLE_SEED, &random_primitive_ints::<u32>, 0, 1);
}

#[test]
#[should_panic]
fn random_ordered_unique_vecs_fail_2() {
    random_ordered_unique_vecs(EXAMPLE_SEED, &random_primitive_ints::<u32>, 1, 0);
}

#[test]
#[should_panic]
fn random_ordered_unique_vecs_fail_3() {
    random_ordered_unique_vecs(
        EXAMPLE_SEED,
        &random_primitive_ints::<u32>,
        u64::MAX,
        u64::MAX - 1,
    );
}
