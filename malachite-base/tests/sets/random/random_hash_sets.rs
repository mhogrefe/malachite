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
use malachite_base::sets::random::random_hash_sets;
use std::collections::HashSet;
use std::fmt::Debug;

fn random_hash_sets_helper<T: Clone + Debug + Eq + Hash + Ord, I: Clone + Iterator<Item = T>>(
    xs_gen: &dyn Fn(Seed) -> I,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[HashSet<T>],
) {
    let xs = random_hash_sets(
        EXAMPLE_SEED,
        xs_gen,
        mean_length_numerator,
        mean_length_denominator,
    );
    let values = xs.take(20).collect_vec();
    assert_eq!(values.as_slice(), expected_values);
}

#[test]
fn test_random_hash_sets() {
    random_hash_sets_helper(
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
    random_hash_sets_helper(
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
    random_hash_sets_helper(
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
    random_hash_sets_helper(
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
}

#[test]
#[should_panic]
fn random_hash_sets_fail_1() {
    random_hash_sets(EXAMPLE_SEED, &random_primitive_ints::<u32>, 0, 1);
}

#[test]
#[should_panic]
fn random_hash_sets_fail_2() {
    random_hash_sets(EXAMPLE_SEED, &random_primitive_ints::<u32>, 1, 0);
}

#[test]
#[should_panic]
fn random_hash_sets_fail_3() {
    random_hash_sets(
        EXAMPLE_SEED,
        &random_primitive_ints::<u32>,
        u64::MAX,
        u64::MAX - 1,
    );
}
