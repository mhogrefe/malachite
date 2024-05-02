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
use malachite_base::vecs::random::random_unique_vecs;
use std::fmt::Debug;

fn random_unique_vecs_helper<T: Clone + Debug + Eq + Hash + Ord, I: Clone + Iterator<Item = T>>(
    xs_gen: &dyn Fn(Seed) -> I,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[&[T]],
    expected_common_values: &[(&[T], usize)],
    expected_median: (&[T], Option<&[T]>),
) {
    random_vecs_helper_helper(
        random_unique_vecs(
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
fn test_random_unique_vecs() {
    random_unique_vecs_helper(
        &random_primitive_ints::<u8>,
        4,
        1,
        &[
            &[],
            &[85, 11, 136, 200, 235, 134, 203, 223, 38, 217, 177, 162, 32, 166],
            &[234, 30, 218, 90],
            &[106, 9, 216, 204],
            &[151],
            &[],
            &[213, 97, 253, 78, 91],
            &[39, 191],
            &[175, 170, 232, 233],
            &[],
            &[2, 35, 22, 217, 198, 114],
            &[],
            &[],
            &[17, 32, 173, 114, 65, 121, 222, 25, 144, 148, 79],
            &[115, 52, 73, 69, 137, 91, 153, 178],
            &[],
            &[112, 34, 95],
            &[],
            &[106, 167, 197, 130, 168],
            &[122, 207, 172, 177, 86, 150, 221, 218, 101],
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
        (&[96], None),
    );
    random_unique_vecs_helper(
        &|seed| geometric_random_unsigneds::<u32>(seed, 32, 1),
        4,
        1,
        &[
            &[],
            &[1, 14, 42, 12, 141, 19, 9, 68, 16, 79, 21, 17, 41, 124],
            &[10, 1, 99, 0],
            &[77, 36, 2, 12],
            &[1],
            &[],
            &[103, 9, 19, 1, 5],
            &[7, 6],
            &[51, 159, 15, 18],
            &[],
            &[52, 75, 40, 64, 2, 26],
            &[],
            &[],
            &[67, 34, 51, 30, 31, 49, 43, 7, 5, 4, 3],
            &[14, 47, 24, 16, 52, 29, 1, 41],
            &[],
            &[13, 11, 62],
            &[],
            &[47, 3, 109, 42, 14],
            &[37, 86, 25, 96, 41, 13, 16, 42, 5],
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
        (&[15, 3], None),
    );
    random_unique_vecs_helper(
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
            (&[][..], 800023),
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
    random_unique_vecs_helper(
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
            &['q', 'á', 'g', 'Á', 'À', 'È', '»', 'ì', '³', 'ą', 'â', 'Ã', 'ñ', 'Ā'],
            &['Ã', 'ª', 'ä', '´'],
            &['½', 'Á', 'Ï', 'ý'],
            &['j'],
            &[],
            &['ï', 'Ñ', 'u', 'Â', '½'],
            &['õ', 'x'],
            &['Â', 'ù', '¡', 'Ċ'],
            &[],
            &['¬', 'b', 'Ñ', 's', 'Â', 'r'],
            &[],
            &[],
            &['n', '¿', 'Þ', 'ô', 'Ø', 'º', 'ü', 't', '¬', 'j', 'Á'],
            &['±', 'è', 'k', 'æ', 'b', 'Î', 'ā', 'Ü'],
            &[],
            &['¹', '«', 'Î'],
            &[],
            &['~', '´', 'Ý', 'â', '¯'],
            &['é', 'æ', 'Þ', 'ö', 'g', 'Î', 'Ç', 'Ü', '¼'],
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
            &['¶', 'Ă', 'ą', '®', 'Á', 'í', '¬', '¾', '¸', 'Ã', '}', 'ù', 'ý', '½', 'a'],
            Some(&['¶', 'Ă', 'ć']),
        ),
    );
}

#[test]
#[should_panic]
fn random_unique_vecs_fail_1() {
    random_unique_vecs(EXAMPLE_SEED, &random_primitive_ints::<u32>, 0, 1);
}

#[test]
#[should_panic]
fn random_unique_vecs_fail_2() {
    random_unique_vecs(EXAMPLE_SEED, &random_primitive_ints::<u32>, 1, 0);
}

#[test]
#[should_panic]
fn random_unique_vecs_fail_3() {
    random_unique_vecs(
        EXAMPLE_SEED,
        &random_primitive_ints::<u32>,
        u64::MAX,
        u64::MAX - 1,
    );
}
