// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::extra_variadic::random_triples_from_single;
use core::hash::Hash;
use itertools::Itertools;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::common_values_map::common_values_map_debug;
use malachite_base::test_util::stats::median;
use malachite_base::tuples::random::random_pairs_from_single;
use std::fmt::Debug;

#[allow(clippy::type_complexity)]
fn random_pairs_from_single_helper<I: Clone + Iterator>(
    xs: I,
    expected_values: &[(I::Item, I::Item)],
    expected_common_values: &[((I::Item, I::Item), usize)],
    expected_median: ((I::Item, I::Item), Option<(I::Item, I::Item)>),
) where
    I::Item: Clone + Debug + Eq + Hash + Ord,
{
    let xs = random_pairs_from_single(xs);
    let values = xs.clone().take(20).collect_vec();
    let common_values = common_values_map_debug(1000000, 10, xs.clone());
    let median = median(xs.take(1000000));
    assert_eq!(
        (values.as_slice(), common_values.as_slice(), median),
        (expected_values, expected_common_values, expected_median)
    );
}

#[allow(clippy::type_complexity)]
fn random_triples_from_single_helper<I: Clone + Iterator>(
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
    let xs = random_triples_from_single(xs);
    let values = xs.clone().take(20).collect_vec();
    let common_values = common_values_map_debug(1000000, 10, xs.clone());
    let median = median(xs.take(1000000));
    assert_eq!(
        (values.as_slice(), common_values.as_slice(), median),
        (expected_values, expected_common_values, expected_median)
    );
}

#[test]
fn test_random_tuples_from_single() {
    random_triples_from_single_helper(
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
            ((222, 60, 79), 4),
            ((26, 110, 13), 4),
            ((41, 254, 55), 4),
            ((109, 134, 76), 4),
            ((165, 174, 73), 4),
            ((236, 57, 174), 4),
            ((73, 168, 192), 4),
            ((89, 197, 244), 4),
            ((91, 170, 115), 4),
            ((142, 168, 231), 4),
        ],
        ((127, 253, 76), Some((127, 253, 86))),
    );
    random_pairs_from_single_helper(
        random_pairs_from_single(random_primitive_ints::<u8>(EXAMPLE_SEED)),
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
            (((28, 96), (0, 11)), 2),
            (((2, 43), (64, 233)), 2),
            (((20, 33), (14, 10)), 2),
            (((223, 84), (7, 22)), 2),
            (((43, 33), (131, 6)), 2),
            (((6, 233), (45, 89)), 2),
            (((65, 26), (6, 146)), 2),
            (((71, 80), (68, 88)), 2),
            (((9, 85), (186, 55)), 2),
            (((96, 254), (9, 37)), 2),
        ],
        (((127, 243), (125, 130)), Some(((127, 243), (134, 100)))),
    );
}
