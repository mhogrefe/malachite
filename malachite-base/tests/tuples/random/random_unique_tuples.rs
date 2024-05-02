// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::extra_variadic::random_unique_triples;
use core::hash::Hash;
use itertools::Itertools;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::common_values_map::common_values_map_debug;
use malachite_base::test_util::stats::median;
use malachite_base::tuples::random::random_unique_pairs;
use std::fmt::Debug;

#[allow(clippy::type_complexity)]
fn random_unique_pairs_helper<I: Clone + Iterator>(
    xs: I,
    expected_values: &[(I::Item, I::Item)],
    expected_common_values: &[((I::Item, I::Item), usize)],
    expected_median: ((I::Item, I::Item), Option<(I::Item, I::Item)>),
) where
    I::Item: Clone + Debug + Eq + Hash + Ord,
{
    let xs = random_unique_pairs(xs);
    let values = xs.clone().take(20).collect_vec();
    let common_values = common_values_map_debug(1000000, 10, xs.clone());
    let median = median(xs.take(1000000));
    assert_eq!(
        (values.as_slice(), common_values.as_slice(), median),
        (expected_values, expected_common_values, expected_median)
    );
}

#[allow(clippy::type_complexity)]
fn random_unique_triples_helper<I: Clone + Iterator>(
    xs: I,
    expected_values: &[(I::Item, I::Item, I::Item)],
    expected_common_values: &[((I::Item, I::Item, I::Item), usize)],
    expected_median: (
        (I::Item, I::Item, I::Item),
        Option<(I::Item, I::Item, I::Item)>,
    ),
) where
    I::Item: Clone + Debug + Eq + Hash + Ord,
{
    let xs = random_unique_triples(xs);
    let values = xs.clone().take(20).collect_vec();
    let common_values = common_values_map_debug(1000000, 10, xs.clone());
    let median = median(xs.take(1000000));
    assert_eq!(
        (values.as_slice(), common_values.as_slice(), median),
        (expected_values, expected_common_values, expected_median)
    );
}

#[test]
fn test_random_unique_tuples() {
    random_unique_triples_helper(
        random_primitive_ints::<u8>(EXAMPLE_SEED),
        &[
            (113, 239, 69),
            (108, 228, 210),
            (168, 161, 87),
            (32, 110, 83),
            (188, 34, 89),
            (238, 93, 200),
            (149, 115, 189),
            (149, 217, 201),
            (117, 146, 31),
            (72, 151, 169),
            (174, 33, 7),
            (38, 81, 144),
            (72, 127, 113),
            (128, 233, 107),
            (46, 119, 12),
            (18, 164, 243),
            (114, 174, 59),
            (247, 39, 174),
            (160, 184, 104),
            (37, 100, 252),
        ],
        &[
            ((205, 0, 97), 4),
            ((102, 18, 19), 4),
            ((105, 70, 13), 4),
            ((22, 45, 192), 4),
            ((87, 100, 26), 4),
            ((15, 107, 109), 4),
            ((134, 245, 157), 4),
            ((138, 164, 179), 4),
            ((219, 253, 196), 4),
            ((237, 197, 239), 4),
        ],
        ((128, 16, 107), Some((128, 16, 116))),
    );
    random_unique_pairs_helper(
        random_unique_pairs(random_primitive_ints::<u8>(EXAMPLE_SEED)),
        &[
            ((113, 239), (69, 108)),
            ((228, 210), (168, 161)),
            ((87, 32), (110, 83)),
            ((188, 34), (89, 238)),
            ((93, 200), (149, 115)),
            ((189, 149), (217, 201)),
            ((117, 146), (31, 72)),
            ((151, 169), (174, 33)),
            ((7, 38), (81, 144)),
            ((72, 127), (113, 128)),
            ((233, 107), (46, 119)),
            ((12, 18), (164, 243)),
            ((114, 174), (59, 247)),
            ((39, 174), (160, 184)),
            ((104, 37), (100, 252)),
            ((228, 122), (107, 69)),
            ((242, 248), (179, 142)),
            ((239, 233), (61, 189)),
            ((235, 85), (192, 7)),
            ((200, 90), (185, 178)),
        ],
        &[
            (((60, 12), (3, 32)), 2),
            (((0, 80), (88, 210)), 2),
            (((1, 3), (216, 183)), 2),
            (((159, 0), (69, 30)), 2),
            (((199, 6), (95, 79)), 2),
            (((2, 98), (221, 19)), 2),
            (((212, 65), (99, 2)), 2),
            (((3, 14), (61, 170)), 2),
            (((41, 155), (3, 72)), 2),
            (((47, 85), (69, 66)), 2),
        ],
        (((128, 41), (252, 44)), Some(((128, 42), (8, 241)))),
    );
}
