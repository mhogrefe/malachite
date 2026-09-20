// Copyright © 2026 Mikhail Hogrefe
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
use malachite_base::num::random::{random_primitive_ints, random_unsigned_inclusive_range};
use malachite_base::random::{EXAMPLE_SEED, Seed};
use malachite_base::test_util::vecs::random::random_vecs_helper_helper;
use malachite_base::vecs::random::random_ordered_vecs;
use std::fmt::Debug;

fn random_ordered_vecs_helper<I: Clone + Iterator>(
    xs_gen: &dyn Fn(Seed) -> I,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[&[I::Item]],
    expected_common_values: &[(&[I::Item], usize)],
    expected_median: (&[I::Item], Option<&[I::Item]>),
) where
    I::Item: Clone + Debug + Eq + Hash + Ord,
{
    random_vecs_helper_helper(
        random_ordered_vecs(
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
fn test_random_ordered_vecs() {
    random_ordered_vecs_helper(
        &random_primitive_ints::<u8>,
        4,
        1,
        &[
            &[],
            &[11, 32, 38, 85, 134, 136, 162, 177, 200, 203, 217, 223, 235, 235],
            &[30, 166, 218, 234],
            &[9, 90, 106, 216],
            &[204],
            &[],
            &[78, 97, 151, 213, 253],
            &[39, 91],
            &[170, 175, 191, 232],
            &[],
            &[2, 22, 35, 198, 217, 233],
            &[],
            &[],
            &[17, 25, 32, 65, 114, 114, 121, 144, 173, 173, 222],
            &[52, 69, 73, 79, 91, 115, 137, 148],
            &[],
            &[112, 153, 178],
            &[],
            &[34, 95, 106, 167, 197],
            &[86, 122, 130, 150, 168, 172, 177, 207, 221],
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
        (
            &[27, 52, 108, 156, 187, 196, 197, 231],
            Some(&[27, 52, 109, 208, 221, 243]),
        ),
    );
    random_ordered_vecs_helper(
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
            (&[0], 4842),
            (&[1], 4831),
            (&[2], 4623),
            (&[3], 4460),
            (&[4], 4197),
            (&[5], 4141),
            (&[6], 4106),
            (&[7], 3872),
            (&[8], 3839),
        ],
        (
            &[3, 12, 20, 23, 24, 68, 70, 99],
            Some(&[3, 12, 20, 23, 26, 113]),
        ),
    );
    random_ordered_vecs_helper(
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
    random_ordered_vecs_helper(
        &|seed| random_unsigned_inclusive_range::<u8>(seed, 1, 3),
        4,
        1,
        &[
            &[],
            &[1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3],
            &[2, 3, 3, 3],
            &[1, 1, 3, 3],
            &[2],
            &[],
            &[1, 2, 3, 3, 3],
            &[2, 3],
            &[1, 2, 2, 3],
            &[],
            &[1, 1, 3, 3, 3, 3],
            &[],
            &[],
            &[1, 1, 2, 3, 3, 3, 3, 3, 3, 3, 3],
            &[1, 2, 2, 2, 3, 3, 3, 3],
            &[],
            &[2, 3, 3],
            &[],
            &[1, 2, 2, 3, 3],
            &[1, 1, 1, 1, 2, 2, 2, 3, 3],
        ],
        &[
            (&[], 199913),
            (&[3], 53448),
            (&[1], 53374),
            (&[2], 53351),
            (&[1, 2], 28636),
            (&[2, 3], 28371),
            (&[1, 3], 28328),
            (&[1, 2, 3], 22864),
            (&[1, 1], 14370),
            (&[3, 3], 14252),
        ],
        (&[1, 1, 2, 2, 2, 3, 3], None),
    );
    random_ordered_vecs_helper(
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
            &['g', 'g', 'q', '³', '»', 'À', 'Á', 'Ã', 'È', 'È', 'á', 'â', 'ì', 'ą'],
            &['ª', 'Ã', 'ñ', 'Ā'],
            &['´', '½', 'Á', 'ä'],
            &['Ï'],
            &[],
            &['j', 'u', 'Ñ', 'ï', 'ý'],
            &['½', 'Â'],
            &['x', 'Â', 'õ', 'ù'],
            &[],
            &['b', '¡', '¬', '¬', 'Ñ', 'Ċ'],
            &[],
            &[],
            &['n', 'r', 's', 't', 'º', '¿', 'Â', 'Ø', 'Þ', 'ô', 'ü'],
            &['b', 'j', 'k', '¬', '±', 'Á', 'æ', 'è'],
            &[],
            &['Î', 'Ü', 'ā'],
            &[],
            &['~', '«', '´', '¹', 'Î'],
            &['g', '¯', 'Î', 'Ý', 'Þ', 'â', 'æ', 'é', 'ö'],
        ],
        &[
            (&[], 199913),
            (&['j'], 1293),
            (&['Ĉ'], 1288),
            (&['Þ'], 1259),
            (&['p'], 1250),
            (&['¢'], 1245),
            (&['ê'], 1242),
            (&['¨'], 1240),
            (&['ć'], 1240),
            (&['ý'], 1236),
        ],
        (
            &['o', '¤', '¥', '¬', '¶', '¸', 'Í', 'Ö', 'ą', 'Ċ'],
            Some(&['o', '¤', '¥', '®', 'Å', 'È', 'Ü', 'Ý', 'æ', 'Ĉ']),
        ),
    );
}
