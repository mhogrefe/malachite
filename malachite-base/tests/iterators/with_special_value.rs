// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::hash::Hash;
use itertools::Itertools;
use malachite_base::iterators::with_special_value;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::options::random::random_options;
use malachite_base::random::{Seed, EXAMPLE_SEED};
use malachite_base::test_util::stats::common_values_map::common_values_map_debug;
use malachite_base::test_util::stats::median;
use std::fmt::Debug;

fn with_special_value_helper<I: Clone + Iterator>(
    special_value: I::Item,
    p_special_numerator: u64,
    p_special_denominator: u64,
    xs_gen: &dyn Fn(Seed) -> I,
    expected_values: &[I::Item],
    expected_common_values: &[(I::Item, usize)],
    expected_median: (I::Item, Option<I::Item>),
) where
    I::Item: Clone + Debug + Eq + Hash + Ord,
{
    let xs = with_special_value(
        EXAMPLE_SEED,
        special_value,
        p_special_numerator,
        p_special_denominator,
        xs_gen,
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
fn test_with_special_value() {
    // special_value = 0, p = 1/2
    with_special_value_helper(
        0,
        1,
        2,
        &random_primitive_ints::<u8>,
        &[0, 0, 0, 85, 0, 11, 0, 0, 0, 136, 200, 235, 0, 0, 134, 0, 0, 0, 0, 0],
        &[
            (0, 501536),
            (81, 2080),
            (208, 2071),
            (35, 2070),
            (211, 2051),
            (112, 2043),
            (162, 2043),
            (143, 2041),
            (220, 2040),
            (73, 2038),
        ],
        (0, None),
    );
    // special_value = 0, p = 50/51
    with_special_value_helper(
        0,
        50,
        51,
        &random_primitive_ints::<u8>,
        &[0; 20],
        &[
            (0, 980537),
            (18, 101),
            (25, 99),
            (116, 97),
            (226, 97),
            (237, 97),
            (23, 95),
            (185, 95),
            (30, 94),
            (73, 94),
        ],
        (0, None),
    );
    // p = special_value = 0, 1/51
    with_special_value_helper(
        0,
        1,
        51,
        &random_primitive_ints::<u8>,
        &[
            85, 11, 136, 200, 235, 134, 203, 223, 38, 235, 217, 177, 162, 32, 166, 234, 30, 218,
            90, 106,
        ],
        &[
            (0, 23543),
            (58, 4029),
            (81, 4001),
            (194, 3979),
            (66, 3971),
            (64, 3969),
            (143, 3965),
            (4, 3962),
            (196, 3951),
            (208, 3939),
        ],
        (125, None),
    );
    // special_value = 0, p = 1/11
    with_special_value_helper(
        Some(0),
        1,
        11,
        &|seed| random_options(seed, 1, 11, &random_primitive_ints::<u8>),
        &[
            Some(229),
            Some(58),
            Some(126),
            Some(0),
            Some(192),
            Some(140),
            Some(235),
            Some(50),
            Some(162),
            Some(5),
            Some(14),
            Some(107),
            Some(218),
            Some(0),
            Some(96),
            Some(86),
            Some(51),
            Some(240),
            Some(0),
            Some(186),
        ],
        &[
            (Some(0), 94315),
            (None, 82945),
            (Some(186), 3381),
            (Some(193), 3376),
            (Some(55), 3364),
            (Some(83), 3364),
            (Some(245), 3360),
            (Some(148), 3352),
            (Some(143), 3345),
            (Some(136), 3339),
        ],
        (Some(101), None),
    );
}

#[test]
#[should_panic]
fn with_special_value_fail_1() {
    with_special_value(EXAMPLE_SEED, 0, 1, 0, &random_primitive_ints::<u8>);
}

#[test]
#[should_panic]
fn with_special_value_fail_2() {
    with_special_value(EXAMPLE_SEED, 0, 2, 1, &random_primitive_ints::<u8>);
}
