// Copyright © 2024 Mikhail Hogrefe
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
use malachite_base::sets::random::random_b_tree_sets_min_length;
use malachite_base::test_util::stats::common_values_map::common_values_map_debug;
use malachite_base::test_util::stats::median;
use std::collections::BTreeSet;
use std::fmt::Debug;

fn random_b_tree_sets_min_length_helper<
    T: Clone + Debug + Eq + Hash + Ord,
    I: Clone + Iterator<Item = T>,
>(
    min_length: u64,
    xs_gen: &dyn Fn(Seed) -> I,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[BTreeSet<T>],
    expected_common_values: &[(BTreeSet<T>, usize)],
    expected_median: (BTreeSet<T>, Option<BTreeSet<T>>),
) {
    let xs = random_b_tree_sets_min_length(
        EXAMPLE_SEED,
        min_length,
        xs_gen,
        mean_length_numerator,
        mean_length_denominator,
    );
    let values = xs.clone().take(20).collect_vec();
    let common_values = common_values_map_debug(1000000, 10, xs.clone());
    let median = median(xs.take(1000000));
    assert_eq!(
        (values.as_slice(), common_values.as_slice(), median),
        (expected_values, expected_common_values, expected_median)
    );
}

#[test]
fn test_random_b_tree_sets_min_length() {
    random_b_tree_sets_min_length_helper(
        0,
        &random_primitive_ints::<u8>,
        4,
        1,
        &[
            btreeset! {},
            btreeset! {11, 32, 38, 85, 134, 136, 162, 166, 177, 200, 203, 217, 223, 235},
            btreeset! {30, 90, 218, 234},
            btreeset! {9, 106, 204, 216},
            btreeset! {151},
            btreeset! {},
            btreeset! {78, 91, 97, 213, 253},
            btreeset! {39, 191},
            btreeset! {170, 175, 232, 233},
            btreeset! {},
            btreeset! {2, 22, 35, 114, 198, 217},
            btreeset! {},
            btreeset! {},
            btreeset! {17, 25, 32, 65, 79, 114, 121, 144, 148, 173, 222},
            btreeset! {52, 69, 73, 91, 115, 137, 153, 178},
            btreeset! {},
            btreeset! {34, 95, 112},
            btreeset! {},
            btreeset! {106, 130, 167, 168, 197},
            btreeset! {86, 101, 122, 150, 172, 177, 207, 218, 221},
        ],
        &[
            (btreeset! {}, 199913),
            (btreeset! {7}, 705),
            (btreeset! {25}, 689),
            (btreeset! {184}, 681),
            (btreeset! {213}, 681),
            (btreeset! {255}, 676),
            (btreeset! {215}, 675),
            (btreeset! {54}, 673),
            (btreeset! {122}, 672),
            (btreeset! {207}, 672),
        ],
        (
            btreeset! {27, 31, 211, 238},
            Some(btreeset! {27, 31, 247, 251}),
        ),
    );
    random_b_tree_sets_min_length_helper(
        3,
        &random_primitive_ints::<u8>,
        7,
        1,
        &[
            btreeset! {11, 85, 136},
            btreeset! {
                9, 30, 32, 38, 90, 106, 134, 162, 166, 177, 200, 203, 217, 218, 223, 234, 235
            },
            btreeset! {78, 97, 151, 204, 213, 216, 253},
            btreeset! {39, 91, 170, 175, 191, 232, 233},
            btreeset! {2, 22, 35, 217},
            btreeset! {17, 114, 198},
            btreeset! {25, 32, 65, 114, 121, 144, 173, 222},
            btreeset! {52, 73, 79, 115, 148},
            btreeset! {34, 69, 91, 112, 137, 153, 178},
            btreeset! {95, 106, 167},
            btreeset! {86, 122, 130, 150, 168, 172, 177, 197, 207},
            btreeset! {101, 218, 221},
            btreeset! {9, 74, 115},
            btreeset! {40, 48, 52, 97, 104, 109, 123, 133, 159, 196, 201, 235, 247, 250},
            btreeset! {7, 11, 24, 43, 68, 103, 112, 157, 190, 216, 217},
            btreeset! {84, 135, 211},
            btreeset! {29, 55, 65, 89, 191, 206},
            btreeset! {9, 51, 79},
            btreeset! {3, 20, 22, 34, 62, 114, 118, 148},
            btreeset! {23, 32, 47, 50, 120, 166, 176, 177, 194, 204, 238, 248},
        ],
        &[
            (btreeset! {5, 128, 142}, 4),
            (btreeset! {137, 145, 160}, 4),
            (btreeset! {2, 4, 52}, 3),
            (btreeset! {1, 5, 192}, 3),
            (btreeset! {12, 41, 58}, 3),
            (btreeset! {2, 95, 171}, 3),
            (btreeset! {20, 86, 94}, 3),
            (btreeset! {21, 43, 50}, 3),
            (btreeset! {3, 81, 122}, 3),
            (btreeset! {31, 54, 79}, 3),
        ],
        (
            btreeset! {26, 138, 167},
            Some(btreeset! {26, 138, 167, 173, 211}),
        ),
    );
    random_b_tree_sets_min_length_helper(
        0,
        &|seed| geometric_random_unsigneds::<u32>(seed, 32, 1),
        4,
        1,
        &[
            btreeset! {},
            btreeset! {1, 9, 12, 14, 16, 17, 19, 21, 41, 42, 68, 79, 124, 141},
            btreeset! {0, 1, 10, 99},
            btreeset! {2, 12, 36, 77},
            btreeset! {1},
            btreeset! {},
            btreeset! {1, 5, 9, 19, 103},
            btreeset! {6, 7},
            btreeset! {15, 18, 51, 159},
            btreeset! {},
            btreeset! {2, 26, 40, 52, 64, 75},
            btreeset! {},
            btreeset! {},
            btreeset! {3, 4, 5, 7, 30, 31, 34, 43, 49, 51, 67},
            btreeset! {1, 14, 16, 24, 29, 41, 47, 52},
            btreeset! {},
            btreeset! {11, 13, 62},
            btreeset! {},
            btreeset! {3, 14, 42, 47, 109},
            btreeset! {5, 13, 16, 25, 37, 41, 42, 86, 96},
        ],
        &[
            (btreeset! {}, 199913),
            (btreeset! {0}, 4861),
            (btreeset! {1}, 4593),
            (btreeset! {2}, 4498),
            (btreeset! {3}, 4405),
            (btreeset! {4}, 4330),
            (btreeset! {5}, 4078),
            (btreeset! {6}, 4050),
            (btreeset! {7}, 3858),
            (btreeset! {8}, 3848),
        ],
        (
            btreeset! {3, 9, 14, 22, 36, 56, 107},
            Some(btreeset! {3, 9, 14, 22, 42, 54, 73, 150}),
        ),
    );
    random_b_tree_sets_min_length_helper(
        3,
        &|seed| geometric_random_unsigneds::<u32>(seed, 32, 1),
        7,
        1,
        &[
            btreeset! {1, 14, 42},
            btreeset! {0, 1, 9, 10, 12, 16, 17, 19, 21, 36, 41, 68, 77, 79, 99, 124, 141},
            btreeset! {1, 2, 5, 9, 12, 19, 103},
            btreeset! {6, 7, 15, 18, 51, 52, 159},
            btreeset! {2, 40, 64, 75},
            btreeset! {26, 34, 67},
            btreeset! {4, 5, 7, 30, 31, 43, 49, 51},
            btreeset! {3, 14, 16, 24, 47},
            btreeset! {1, 11, 13, 29, 41, 52, 62},
            btreeset! {3, 47, 109},
            btreeset! {13, 14, 16, 25, 37, 41, 42, 86, 96},
            btreeset! {5, 20, 42},
            btreeset! {2, 74, 82},
            btreeset! {3, 6, 7, 11, 17, 20, 36, 45, 56, 66, 76, 80, 89, 127},
            btreeset! {1, 6, 10, 13, 19, 23, 25, 32, 41, 43, 97},
            btreeset! {7, 41, 134},
            btreeset! {9, 10, 25, 26, 47, 105},
            btreeset! {68, 94, 109},
            btreeset! {1, 3, 9, 13, 28, 43, 44, 84},
            btreeset! {0, 4, 5, 6, 7, 13, 31, 32, 37, 42, 50, 75},
        ],
        &[
            (btreeset! {0, 2, 5}, 42),
            (btreeset! {0, 1, 8}, 39),
            (btreeset! {0, 3, 4}, 38),
            (btreeset! {1, 3, 9}, 38),
            (btreeset! {0, 1, 7}, 35),
            (btreeset! {0, 2, 8}, 34),
            (btreeset! {1, 2, 12}, 34),
            (btreeset! {0, 1, 2}, 33),
            (btreeset! {1, 2, 3}, 33),
            (btreeset! {1, 3, 4}, 33),
        ],
        (
            btreeset! {3, 8, 14, 19, 25, 36, 52, 64, 71},
            Some(btreeset! {3, 8, 14, 19, 25, 38, 58, 61}),
        ),
    );
    random_b_tree_sets_min_length_helper(
        0,
        &random_primitive_ints::<u8>,
        1,
        4,
        &[
            btreeset! {},
            btreeset! {},
            btreeset! {85},
            btreeset! {11},
            btreeset! {136, 200},
            btreeset! {},
            btreeset! {},
            btreeset! {},
            btreeset! {},
            btreeset! {},
            btreeset! {},
            btreeset! {},
            btreeset! {134, 235},
            btreeset! {203},
            btreeset! {},
            btreeset! {38, 223},
            btreeset! {},
            btreeset! {},
            btreeset! {},
            btreeset! {},
        ],
        &[
            (btreeset! {}, 800023),
            (btreeset! {162}, 692),
            (btreeset! {235}, 690),
            (btreeset! {90}, 688),
            (btreeset! {65}, 687),
            (btreeset! {249}, 686),
            (btreeset! {175}, 684),
            (btreeset! {108}, 683),
            (btreeset! {211}, 682),
            (btreeset! {237}, 680),
        ],
        (btreeset! {}, None),
    );
    random_b_tree_sets_min_length_helper(
        3,
        &random_primitive_ints::<u8>,
        13,
        4,
        &[
            btreeset! {11, 85, 136},
            btreeset! {134, 200, 235},
            btreeset! {38, 203, 223, 235},
            btreeset! {32, 162, 177, 217},
            btreeset! {30, 90, 166, 218, 234},
            btreeset! {9, 106, 216},
            btreeset! {151, 204, 213},
            btreeset! {78, 97, 253},
            btreeset! {39, 91, 191},
            btreeset! {170, 175, 232},
            btreeset! {2, 35, 233},
            btreeset! {22, 198, 217},
            btreeset! {17, 32, 65, 114, 173},
            btreeset! {25, 121, 173, 222},
            btreeset! {79, 144, 148},
            btreeset! {52, 69, 73, 115, 137},
            btreeset! {91, 153, 178},
            btreeset! {34, 95, 112},
            btreeset! {106, 167, 197},
            btreeset! {122, 130, 168},
        ],
        &[
            (btreeset! {10, 87, 204}, 6),
            (btreeset! {15, 40, 115}, 6),
            (btreeset! {108, 193, 199}, 6),
            (btreeset! {1, 22, 70}, 5),
            (btreeset! {1, 8, 212}, 5),
            (btreeset! {2, 40, 169}, 5),
            (btreeset! {2, 58, 211}, 5),
            (btreeset! {3, 29, 186}, 5),
            (btreeset! {3, 97, 112}, 5),
            (btreeset! {11, 66, 140}, 5),
        ],
        (btreeset! {49, 78, 193}, Some(btreeset! {49, 78, 193, 215})),
    );
    random_b_tree_sets_min_length_helper(
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
            btreeset! {},
            btreeset! {'g', 'q', '³', '»', 'À', 'Á', 'Ã', 'È', 'á', 'â', 'ì', 'ñ', 'Ā', 'ą'},
            btreeset! {'ª', '´', 'Ã', 'ä'},
            btreeset! {'½', 'Á', 'Ï', 'ý'},
            btreeset! {'j'},
            btreeset! {},
            btreeset! {'u', '½', 'Â', 'Ñ', 'ï'},
            btreeset! {'x', 'õ'},
            btreeset! {'¡', 'Â', 'ù', 'Ċ'},
            btreeset! {},
            btreeset! {'b', 'r', 's', '¬', 'Â', 'Ñ'},
            btreeset! {},
            btreeset! {},
            btreeset! {'j', 'n', 't', '¬', 'º', '¿', 'Á', 'Ø', 'Þ', 'ô', 'ü'},
            btreeset! {'b', 'k', '±', 'Î', 'Ü', 'æ', 'è', 'ā'},
            btreeset! {},
            btreeset! {'«', '¹', 'Î'},
            btreeset! {},
            btreeset! {'~', '¯', '´', 'Ý', 'â'},
            btreeset! {'g', '¼', 'Ç', 'Î', 'Ü', 'Þ', 'æ', 'é', 'ö'},
        ],
        &[
            (btreeset! {}, 199913),
            (btreeset! {'Ó'}, 1270),
            (btreeset! {'Â'}, 1249),
            (btreeset! {'§'}, 1244),
            (btreeset! {'¿'}, 1243),
            (btreeset! {'õ'}, 1241),
            (btreeset! {'ĉ'}, 1234),
            (btreeset! {'¤'}, 1232),
            (btreeset! {'¼'}, 1232),
            (btreeset! {'Ì'}, 1229),
        ],
        (
            btreeset! {'o', 'v', '¢', '±', 'Ä', 'Ć'},
            Some(btreeset! {'o', 'v', '¢', '³', 'ã'}),
        ),
    );
    random_b_tree_sets_min_length_helper(
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
            btreeset! {'g', 'q', 'á'},
            btreeset! {
                'g', 'ª', '³', '´', '»', '½', 'À', 'Á', 'Ã', 'È', 'Ï', 'â', 'ä', 'ì', 'ñ', 'Ā', 'ą',
            },
            btreeset! {'j', 'u', '½', 'Â', 'Ñ', 'ï', 'ý'},
            btreeset! {'x', '¡', '¬', 'Â', 'õ', 'ù', 'Ċ'},
            btreeset! {'b', 's', '¬', 'Ñ'},
            btreeset! {'n', 'r', 'Â'},
            btreeset! {'t', '¬', 'º', '¿', 'Ø', 'Þ', 'ô', 'ü'},
            btreeset! {'j', 'k', '±', 'Á', 'è'},
            btreeset! {'b', '«', '¹', 'Î', 'Ü', 'æ', 'ā'},
            btreeset! {'~', '´', 'Î'},
            btreeset! {'g', '¯', 'Î', 'Ý', 'Þ', 'â', 'æ', 'é', 'ö'},
            btreeset! {'¼', 'Ç', 'Ü'},
            btreeset! {'¡', '§', 'Ì'},
            btreeset! {'d', 'm', 'z', '{', '¨', '®', '±', '¼', 'Ë', 'Ü', 'ê', 'ì', 'ý', 'þ'},
            btreeset! {'x', 'ª', '½', 'À', 'Õ', 'ì', 'ï', 'û', 'ă', 'Ą', 'ċ'},
            btreeset! {'¢', '«', 'Ć'},
            btreeset! {'{', '¢', '½', 'È', 'ä', 'ÿ'},
            btreeset! {'Ë', 'Õ', 'ê'},
            btreeset! {'p', '¨', '°', 'º', 'Å', 'Ó', '×', 'ü'},
            btreeset! {'d', 'k', 'o', 'v', '¥', '±', 'Ä', 'È', 'Ê', 'ß', 'æ', 'Ć'},
        ],
        &[
            (btreeset! {'m', 'u', 'w'}, 6),
            (btreeset! {'b', 'n', 'Ã'}, 6),
            (btreeset! {'g', '®', 'Ý'}, 6),
            (btreeset! {'x', 'Ä', 'î'}, 6),
            (btreeset! {'º', 'Ú', '÷'}, 6),
            (btreeset! {'a', 'w', 'ø'}, 5),
            (btreeset! {'c', 'e', 'Þ'}, 5),
            (btreeset! {'d', 't', 'Ã'}, 5),
            (btreeset! {'m', 'r', 'È'}, 5),
            (btreeset! {'w', '{', '³'}, 5),
        ],
        (
            btreeset! {'o', 's', '×', 'Ý', 'Þ', 'ß', 'î', 'ù'},
            Some(btreeset! {'o', 's', '×', 'à', 'ã', 'ò', 'ċ'}),
        ),
    );
}

#[test]
#[should_panic]
fn random_b_tree_sets_min_length_fail_1() {
    random_b_tree_sets_min_length(EXAMPLE_SEED, 3, &random_primitive_ints::<u32>, 3, 1);
}

#[test]
#[should_panic]
fn random_b_tree_sets_min_length_fail_2() {
    random_b_tree_sets_min_length(EXAMPLE_SEED, 1, &random_primitive_ints::<u32>, 1, 0);
}

#[test]
#[should_panic]
fn random_b_tree_sets_min_length_fail_3() {
    random_b_tree_sets_min_length(
        EXAMPLE_SEED,
        0,
        &random_primitive_ints::<u32>,
        u64::MAX,
        u64::MAX - 1,
    );
}
