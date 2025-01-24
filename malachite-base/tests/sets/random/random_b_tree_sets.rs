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
use malachite_base::sets::random::random_b_tree_sets;
use malachite_base::test_util::stats::common_values_map::common_values_map_debug;
use malachite_base::test_util::stats::median;
use std::collections::BTreeSet;
use std::fmt::Debug;

fn random_b_tree_sets_helper<T: Clone + Debug + Eq + Hash + Ord, I: Clone + Iterator<Item = T>>(
    xs_gen: &dyn Fn(Seed) -> I,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[BTreeSet<T>],
    expected_common_values: &[(BTreeSet<T>, usize)],
    expected_median: (BTreeSet<T>, Option<BTreeSet<T>>),
) {
    let xs = random_b_tree_sets(
        EXAMPLE_SEED,
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
fn test_random_b_tree_sets() {
    random_b_tree_sets_helper(
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
    random_b_tree_sets_helper(
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
    random_b_tree_sets_helper(
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
    random_b_tree_sets_helper(
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
}

#[test]
#[should_panic]
fn random_b_tree_sets_fail_1() {
    random_b_tree_sets(EXAMPLE_SEED, &random_primitive_ints::<u32>, 0, 1);
}

#[test]
#[should_panic]
fn random_b_tree_sets_fail_2() {
    random_b_tree_sets(EXAMPLE_SEED, &random_primitive_ints::<u32>, 1, 0);
}

#[test]
#[should_panic]
fn random_b_tree_sets_fail_3() {
    random_b_tree_sets(
        EXAMPLE_SEED,
        &random_primitive_ints::<u32>,
        u64::MAX,
        u64::MAX - 1,
    );
}
