// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::extra_variadic::random_ordered_unique_triples;
use core::hash::Hash;
use itertools::Itertools;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::common_values_map::common_values_map_debug;
use malachite_base::test_util::stats::median;
use malachite_base::tuples::random::random_ordered_unique_pairs;
use std::fmt::Debug;

#[allow(clippy::type_complexity)]
fn random_ordered_unique_pairs_helper<I: Clone + Iterator>(
    xs: I,
    expected_values: &[(I::Item, I::Item)],
    expected_common_values: &[((I::Item, I::Item), usize)],
    expected_median: ((I::Item, I::Item), Option<(I::Item, I::Item)>),
) where
    I::Item: Clone + Debug + Eq + Hash + Ord,
{
    let xs = random_ordered_unique_pairs(xs);
    let values = xs.clone().take(20).collect_vec();
    let common_values = common_values_map_debug(1000000, 10, xs.clone());
    let median = median(xs.take(1000000));
    assert_eq!(
        (values.as_slice(), common_values.as_slice(), median),
        (expected_values, expected_common_values, expected_median)
    );
}

#[allow(clippy::type_complexity)]
fn random_ordered_unique_triples_helper<I: Clone + Iterator>(
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
    let xs = random_ordered_unique_triples(xs);
    let values = xs.clone().take(20).collect_vec();
    let common_values = common_values_map_debug(1000000, 10, xs.clone());
    let median = median(xs.take(1000000));
    assert_eq!(
        (values.as_slice(), common_values.as_slice(), median),
        (expected_values, expected_common_values, expected_median)
    );
}

#[test]
fn test_random_ordered_unique_tuples() {
    random_ordered_unique_triples_helper(
        random_primitive_ints::<u8>(EXAMPLE_SEED),
        &[
            (69, 113, 239),
            (108, 210, 228),
            (87, 161, 168),
            (32, 83, 110),
            (34, 89, 188),
            (93, 200, 238),
            (115, 149, 189),
            (149, 201, 217),
            (31, 117, 146),
            (72, 151, 169),
            (7, 33, 174),
            (38, 81, 144),
            (72, 113, 127),
            (107, 128, 233),
            (12, 46, 119),
            (18, 164, 243),
            (59, 114, 174),
            (39, 174, 247),
            (104, 160, 184),
            (37, 100, 252),
        ],
        &[
            ((57, 142, 207), 7),
            ((32, 68, 169), 6),
            ((36, 70, 195), 6),
            ((125, 168, 194), 6),
            ((0, 97, 205), 5),
            ((2, 33, 227), 5),
            ((5, 46, 239), 5),
            ((9, 68, 189), 5),
            ((9, 78, 240), 5),
            ((1, 110, 203), 5),
        ],
        ((52, 133, 241), Some((52, 133, 242))),
    );
    random_ordered_unique_pairs_helper(
        random_ordered_unique_pairs(random_primitive_ints::<u8>(EXAMPLE_SEED)),
        &[
            ((69, 108), (113, 239)),
            ((161, 168), (210, 228)),
            ((32, 87), (83, 110)),
            ((34, 188), (89, 238)),
            ((93, 200), (115, 149)),
            ((149, 189), (201, 217)),
            ((31, 72), (117, 146)),
            ((33, 174), (151, 169)),
            ((7, 38), (81, 144)),
            ((72, 127), (113, 128)),
            ((46, 119), (107, 233)),
            ((12, 18), (164, 243)),
            ((59, 247), (114, 174)),
            ((39, 174), (160, 184)),
            ((37, 104), (100, 252)),
            ((69, 107), (122, 228)),
            ((142, 179), (242, 248)),
            ((61, 189), (233, 239)),
            ((7, 192), (85, 235)),
            ((90, 200), (178, 185)),
        ],
        &[
            (((0, 78), (34, 52)), 2),
            (((1, 58), (6, 112)), 2),
            (((1, 63), (8, 154)), 2),
            (((1, 97), (7, 250)), 2),
            (((2, 33), (40, 81)), 2),
            (((3, 160), (7, 29)), 2),
            (((3, 32), (12, 60)), 2),
            (((6, 130), (7, 20)), 2),
            (((6, 68), (7, 126)), 2),
            (((6, 77), (36, 54)), 2),
        ],
        (((40, 193), (94, 142)), Some(((40, 193), (97, 243)))),
    );
}
