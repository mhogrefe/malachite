// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::hash::Hash;
use itertools::Itertools;
use malachite_base::chars::exhaustive::exhaustive_chars;
use malachite_base::chars::random::graphic_weighted_random_char_inclusive_range;
use malachite_base::num::random::geometric::geometric_random_unsigneds;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::{Seed, EXAMPLE_SEED};
use malachite_base::sets::random::random_hash_sets_min_length;
use std::collections::HashSet;
use std::fmt::Debug;

fn random_hash_sets_min_length_helper<
    T: Clone + Debug + Eq + Hash + Ord,
    I: Clone + Iterator<Item = T>,
>(
    min_length: u64,
    xs_gen: &dyn Fn(Seed) -> I,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[HashSet<T>],
) {
    let xs = random_hash_sets_min_length(
        EXAMPLE_SEED,
        min_length,
        xs_gen,
        mean_length_numerator,
        mean_length_denominator,
    );
    let values = xs.take(20).collect_vec();
    assert_eq!(values.as_slice(), expected_values);
}

#[test]
fn test_random_hash_sets_min_length() {
    random_hash_sets_min_length_helper(
        0,
        &random_primitive_ints::<u8>,
        4,
        1,
        &[
            hashset! {},
            hashset! {11, 32, 38, 85, 134, 136, 162, 166, 177, 200, 203, 217, 223, 235},
            hashset! {30, 90, 218, 234},
            hashset! {9, 106, 204, 216},
            hashset! {151},
            hashset! {},
            hashset! {78, 91, 97, 213, 253},
            hashset! {39, 191},
            hashset! {170, 175, 232, 233},
            hashset! {},
            hashset! {2, 22, 35, 114, 198, 217},
            hashset! {},
            hashset! {},
            hashset! {17, 25, 32, 65, 79, 114, 121, 144, 148, 173, 222},
            hashset! {52, 69, 73, 91, 115, 137, 153, 178},
            hashset! {},
            hashset! {34, 95, 112},
            hashset! {},
            hashset! {106, 130, 167, 168, 197},
            hashset! {86, 101, 122, 150, 172, 177, 207, 218, 221},
        ],
    );
    random_hash_sets_min_length_helper(
        3,
        &random_primitive_ints::<u8>,
        7,
        1,
        &[
            hashset! {11, 85, 136},
            hashset! {
                9, 30, 32, 38, 90, 106, 134, 162, 166, 177, 200, 203, 217, 218, 223, 234, 235
            },
            hashset! {78, 97, 151, 204, 213, 216, 253},
            hashset! {39, 91, 170, 175, 191, 232, 233},
            hashset! {2, 22, 35, 217},
            hashset! {17, 114, 198},
            hashset! {25, 32, 65, 114, 121, 144, 173, 222},
            hashset! {52, 73, 79, 115, 148},
            hashset! {34, 69, 91, 112, 137, 153, 178},
            hashset! {95, 106, 167},
            hashset! {86, 122, 130, 150, 168, 172, 177, 197, 207},
            hashset! {101, 218, 221},
            hashset! {9, 74, 115},
            hashset! {40, 48, 52, 97, 104, 109, 123, 133, 159, 196, 201, 235, 247, 250},
            hashset! {7, 11, 24, 43, 68, 103, 112, 157, 190, 216, 217},
            hashset! {84, 135, 211},
            hashset! {29, 55, 65, 89, 191, 206},
            hashset! {9, 51, 79},
            hashset! {3, 20, 22, 34, 62, 114, 118, 148},
            hashset! {23, 32, 47, 50, 120, 166, 176, 177, 194, 204, 238, 248},
        ],
    );
    random_hash_sets_min_length_helper(
        0,
        &|seed| geometric_random_unsigneds::<u32>(seed, 32, 1),
        4,
        1,
        &[
            hashset! {},
            hashset! {1, 9, 12, 14, 16, 17, 19, 21, 41, 42, 68, 79, 124, 141},
            hashset! {0, 1, 10, 99},
            hashset! {2, 12, 36, 77},
            hashset! {1},
            hashset! {},
            hashset! {1, 5, 9, 19, 103},
            hashset! {6, 7},
            hashset! {15, 18, 51, 159},
            hashset! {},
            hashset! {2, 26, 40, 52, 64, 75},
            hashset! {},
            hashset! {},
            hashset! {3, 4, 5, 7, 30, 31, 34, 43, 49, 51, 67},
            hashset! {1, 14, 16, 24, 29, 41, 47, 52},
            hashset! {},
            hashset! {11, 13, 62},
            hashset! {},
            hashset! {3, 14, 42, 47, 109},
            hashset! {5, 13, 16, 25, 37, 41, 42, 86, 96},
        ],
    );
    random_hash_sets_min_length_helper(
        3,
        &|seed| geometric_random_unsigneds::<u32>(seed, 32, 1),
        7,
        1,
        &[
            hashset! {1, 14, 42},
            hashset! {0, 1, 9, 10, 12, 16, 17, 19, 21, 36, 41, 68, 77, 79, 99, 124, 141},
            hashset! {1, 2, 5, 9, 12, 19, 103},
            hashset! {6, 7, 15, 18, 51, 52, 159},
            hashset! {2, 40, 64, 75},
            hashset! {26, 34, 67},
            hashset! {4, 5, 7, 30, 31, 43, 49, 51},
            hashset! {3, 14, 16, 24, 47},
            hashset! {1, 11, 13, 29, 41, 52, 62},
            hashset! {3, 47, 109},
            hashset! {13, 14, 16, 25, 37, 41, 42, 86, 96},
            hashset! {5, 20, 42},
            hashset! {2, 74, 82},
            hashset! {3, 6, 7, 11, 17, 20, 36, 45, 56, 66, 76, 80, 89, 127},
            hashset! {1, 6, 10, 13, 19, 23, 25, 32, 41, 43, 97},
            hashset! {7, 41, 134},
            hashset! {9, 10, 25, 26, 47, 105},
            hashset! {68, 94, 109},
            hashset! {1, 3, 9, 13, 28, 43, 44, 84},
            hashset! {0, 4, 5, 6, 7, 13, 31, 32, 37, 42, 50, 75},
        ],
    );
    random_hash_sets_min_length_helper(
        0,
        &random_primitive_ints::<u8>,
        1,
        4,
        &[
            hashset! {},
            hashset! {},
            hashset! {85},
            hashset! {11},
            hashset! {136, 200},
            hashset! {},
            hashset! {},
            hashset! {},
            hashset! {},
            hashset! {},
            hashset! {},
            hashset! {},
            hashset! {134, 235},
            hashset! {203},
            hashset! {},
            hashset! {38, 223},
            hashset! {},
            hashset! {},
            hashset! {},
            hashset! {},
        ],
    );
    random_hash_sets_min_length_helper(
        3,
        &random_primitive_ints::<u8>,
        13,
        4,
        &[
            hashset! {11, 85, 136},
            hashset! {134, 200, 235},
            hashset! {38, 203, 223, 235},
            hashset! {32, 162, 177, 217},
            hashset! {30, 90, 166, 218, 234},
            hashset! {9, 106, 216},
            hashset! {151, 204, 213},
            hashset! {78, 97, 253},
            hashset! {39, 91, 191},
            hashset! {170, 175, 232},
            hashset! {2, 35, 233},
            hashset! {22, 198, 217},
            hashset! {17, 32, 65, 114, 173},
            hashset! {25, 121, 173, 222},
            hashset! {79, 144, 148},
            hashset! {52, 69, 73, 115, 137},
            hashset! {91, 153, 178},
            hashset! {34, 95, 112},
            hashset! {106, 167, 197},
            hashset! {122, 130, 168},
        ],
    );
    random_hash_sets_min_length_helper(
        0,
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
            hashset! {},
            hashset! {'g', 'q', '³', '»', 'À', 'Á', 'Ã', 'È', 'á', 'â', 'ì', 'ñ', 'Ā', 'ą'},
            hashset! {'ª', '´', 'Ã', 'ä'},
            hashset! {'½', 'Á', 'Ï', 'ý'},
            hashset! {'j'},
            hashset! {},
            hashset! {'u', '½', 'Â', 'Ñ', 'ï'},
            hashset! {'x', 'õ'},
            hashset! {'¡', 'Â', 'ù', 'Ċ'},
            hashset! {},
            hashset! {'b', 'r', 's', '¬', 'Â', 'Ñ'},
            hashset! {},
            hashset! {},
            hashset! {'j', 'n', 't', '¬', 'º', '¿', 'Á', 'Ø', 'Þ', 'ô', 'ü'},
            hashset! {'b', 'k', '±', 'Î', 'Ü', 'æ', 'è', 'ā'},
            hashset! {},
            hashset! {'«', '¹', 'Î'},
            hashset! {},
            hashset! {'~', '¯', '´', 'Ý', 'â'},
            hashset! {'g', '¼', 'Ç', 'Î', 'Ü', 'Þ', 'æ', 'é', 'ö'},
        ],
    );
    random_hash_sets_min_length_helper(
        3,
        &|seed| {
            graphic_weighted_random_char_inclusive_range(
                seed,
                'a',
                exhaustive_chars().nth(200).unwrap(),
                1,
                1,
            )
        },
        7,
        1,
        &[
            hashset! {'g', 'q', 'á'},
            hashset! {
                'g', 'ª', '³', '´', '»', '½', 'À', 'Á', 'Ã', 'È', 'Ï', 'â', 'ä', 'ì', 'ñ', 'Ā', 'ą',
            },
            hashset! {'j', 'u', '½', 'Â', 'Ñ', 'ï', 'ý'},
            hashset! {'x', '¡', '¬', 'Â', 'õ', 'ù', 'Ċ'},
            hashset! {'b', 's', '¬', 'Ñ'},
            hashset! {'n', 'r', 'Â'},
            hashset! {'t', '¬', 'º', '¿', 'Ø', 'Þ', 'ô', 'ü'},
            hashset! {'j', 'k', '±', 'Á', 'è'},
            hashset! {'b', '«', '¹', 'Î', 'Ü', 'æ', 'ā'},
            hashset! {'~', '´', 'Î'},
            hashset! {'g', '¯', 'Î', 'Ý', 'Þ', 'â', 'æ', 'é', 'ö'},
            hashset! {'¼', 'Ç', 'Ü'},
            hashset! {'¡', '§', 'Ì'},
            hashset! {'d', 'm', 'z', '{', '¨', '®', '±', '¼', 'Ë', 'Ü', 'ê', 'ì', 'ý', 'þ'},
            hashset! {'x', 'ª', '½', 'À', 'Õ', 'ì', 'ï', 'û', 'ă', 'Ą', 'ċ'},
            hashset! {'¢', '«', 'Ć'},
            hashset! {'{', '¢', '½', 'È', 'ä', 'ÿ'},
            hashset! {'Ë', 'Õ', 'ê'},
            hashset! {'p', '¨', '°', 'º', 'Å', 'Ó', '×', 'ü'},
            hashset! {'d', 'k', 'o', 'v', '¥', '±', 'Ä', 'È', 'Ê', 'ß', 'æ', 'Ć'},
        ],
    );
}

#[test]
#[should_panic]
fn random_hash_sets_min_length_fail_1() {
    random_hash_sets_min_length(EXAMPLE_SEED, 3, &random_primitive_ints::<u32>, 3, 1);
}

#[test]
#[should_panic]
fn random_hash_sets_min_length_fail_2() {
    random_hash_sets_min_length(EXAMPLE_SEED, 1, &random_primitive_ints::<u32>, 1, 0);
}

#[test]
#[should_panic]
fn random_hash_sets_min_length_fail_3() {
    random_hash_sets_min_length(
        EXAMPLE_SEED,
        0,
        &random_primitive_ints::<u32>,
        u64::MAX,
        u64::MAX - 1,
    );
}
