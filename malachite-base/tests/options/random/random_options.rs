// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::hash::Hash;
use itertools::Itertools;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::options::random::random_options;
use malachite_base::random::{Seed, EXAMPLE_SEED};
use malachite_base::test_util::stats::common_values_map::common_values_map_debug;
use malachite_base::test_util::stats::median;
use std::fmt::Debug;

fn random_options_helper<I: Clone + Iterator>(
    p_none_numerator: u64,
    p_none_denominator: u64,
    xs_gen: &dyn Fn(Seed) -> I,
    expected_values: &[Option<I::Item>],
    expected_common_values: &[(Option<I::Item>, usize)],
    expected_median: (Option<I::Item>, Option<Option<I::Item>>),
) where
    I::Item: Clone + Debug + Eq + Hash + Ord,
{
    let xs = random_options(EXAMPLE_SEED, p_none_numerator, p_none_denominator, xs_gen);
    let values = xs.clone().take(20).collect_vec();
    let common_values = common_values_map_debug(1000000, 10, xs.clone());
    let median = median(xs.take(1000000));
    assert_eq!(
        (values.as_slice(), common_values.as_slice(), median),
        (expected_values, expected_common_values, expected_median)
    );
}

#[test]
fn test_random_options() {
    // p = 1/2
    random_options_helper(
        1,
        2,
        &random_primitive_ints::<u8>,
        &[
            Some(85),
            Some(11),
            Some(136),
            None,
            Some(200),
            None,
            Some(235),
            Some(134),
            Some(203),
            None,
            None,
            None,
            Some(223),
            Some(38),
            None,
            Some(235),
            Some(217),
            Some(177),
            Some(162),
            Some(32),
        ],
        &[
            (None, 500454),
            (Some(81), 2076),
            (Some(208), 2066),
            (Some(35), 2065),
            (Some(211), 2045),
            (Some(112), 2042),
            (Some(143), 2039),
            (Some(162), 2037),
            (Some(170), 2036),
            (Some(58), 2035),
        ],
        (None, None),
    );
    // p = 50/51
    random_options_helper(
        50,
        51,
        &random_primitive_ints::<u8>,
        &[None; 20],
        &[
            (None, 980283),
            (Some(18), 102),
            (Some(25), 99),
            (Some(237), 98),
            (Some(116), 97),
            (Some(226), 97),
            (Some(23), 95),
            (Some(185), 95),
            (Some(30), 94),
            (Some(73), 94),
        ],
        (None, None),
    );
    // p = 1/51
    random_options_helper(
        1,
        51,
        &random_primitive_ints::<u8>,
        &[
            Some(85),
            Some(11),
            Some(136),
            Some(200),
            Some(235),
            Some(134),
            Some(203),
            Some(223),
            Some(38),
            Some(235),
            Some(217),
            Some(177),
            Some(162),
            Some(32),
            Some(166),
            Some(234),
            Some(30),
            Some(218),
            Some(90),
            Some(106),
        ],
        &[
            (None, 19543),
            (Some(58), 4030),
            (Some(81), 4001),
            (Some(194), 3981),
            (Some(66), 3973),
            (Some(64), 3969),
            (Some(143), 3965),
            (Some(4), 3964),
            (Some(196), 3952),
            (Some(208), 3941),
        ],
        (Some(125), None),
    );
    // p = 1/11
    random_options_helper(
        1,
        11,
        &|seed| random_options(seed, 1, 11, &random_primitive_ints::<u8>),
        &[
            Some(Some(229)),
            Some(Some(58)),
            Some(Some(126)),
            Some(Some(192)),
            Some(Some(140)),
            Some(Some(235)),
            Some(Some(50)),
            Some(Some(162)),
            Some(Some(5)),
            Some(Some(14)),
            Some(Some(107)),
            Some(Some(218)),
            Some(Some(96)),
            Some(Some(86)),
            Some(Some(51)),
            None,
            Some(Some(240)),
            Some(Some(186)),
            Some(Some(180)),
            Some(Some(152)),
        ],
        &[
            (None, 90592),
            (Some(None), 83007),
            (Some(Some(186)), 3385),
            (Some(Some(193)), 3377),
            (Some(Some(83)), 3366),
            (Some(Some(55)), 3365),
            (Some(Some(245)), 3362),
            (Some(Some(148)), 3354),
            (Some(Some(143)), 3345),
            (Some(Some(136)), 3341),
        ],
        (Some(Some(101)), None),
    );
}

#[test]
#[should_panic]
fn random_options_fail_1() {
    random_options(EXAMPLE_SEED, 1, 0, &random_primitive_ints::<u8>);
}

#[test]
#[should_panic]
fn random_options_fail_2() {
    random_options(EXAMPLE_SEED, 2, 1, &random_primitive_ints::<u8>);
}
