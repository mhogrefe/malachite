// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::hash::Hash;
use itertools::Itertools;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::options::random::random_somes;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::common_values_map::common_values_map_debug;
use malachite_base::test_util::stats::median;
use std::fmt::Debug;

fn random_somes_helper<I: Clone + Iterator>(
    xs: I,
    expected_values: &[Option<I::Item>],
    expected_common_values: &[(Option<I::Item>, usize)],
    expected_median: (Option<I::Item>, Option<Option<I::Item>>),
) where
    I::Item: Clone + Debug + Eq + Hash + Ord,
{
    let xs = random_somes(xs);
    let values = xs.clone().take(20).collect_vec();
    let common_values = common_values_map_debug(1000000, 10, xs.clone());
    let median = median(xs.take(1000000));
    assert_eq!(
        (values.as_slice(), common_values.as_slice(), median),
        (expected_values, expected_common_values, expected_median)
    );
}

#[test]
fn test_random_somes() {
    random_somes_helper(
        random_primitive_ints::<u8>(EXAMPLE_SEED),
        &[
            Some(113),
            Some(239),
            Some(69),
            Some(108),
            Some(228),
            Some(210),
            Some(168),
            Some(161),
            Some(87),
            Some(32),
            Some(110),
            Some(83),
            Some(188),
            Some(34),
            Some(89),
            Some(238),
            Some(93),
            Some(200),
            Some(149),
            Some(115),
        ],
        &[
            (Some(214), 4097),
            (Some(86), 4078),
            (Some(166), 4049),
            (Some(22), 4048),
            (Some(126), 4047),
            (Some(55), 4040),
            (Some(93), 4037),
            (Some(191), 4036),
            (Some(36), 4035),
            (Some(42), 4032),
        ],
        (Some(127), None),
    );
    random_somes_helper(
        random_somes(random_primitive_ints::<u8>(EXAMPLE_SEED)),
        &[
            Some(Some(113)),
            Some(Some(239)),
            Some(Some(69)),
            Some(Some(108)),
            Some(Some(228)),
            Some(Some(210)),
            Some(Some(168)),
            Some(Some(161)),
            Some(Some(87)),
            Some(Some(32)),
            Some(Some(110)),
            Some(Some(83)),
            Some(Some(188)),
            Some(Some(34)),
            Some(Some(89)),
            Some(Some(238)),
            Some(Some(93)),
            Some(Some(200)),
            Some(Some(149)),
            Some(Some(115)),
        ],
        &[
            (Some(Some(214)), 4097),
            (Some(Some(86)), 4078),
            (Some(Some(166)), 4049),
            (Some(Some(22)), 4048),
            (Some(Some(126)), 4047),
            (Some(Some(55)), 4040),
            (Some(Some(93)), 4037),
            (Some(Some(191)), 4036),
            (Some(Some(36)), 4035),
            (Some(Some(42)), 4032),
        ],
        (Some(Some(127)), None),
    );
}
