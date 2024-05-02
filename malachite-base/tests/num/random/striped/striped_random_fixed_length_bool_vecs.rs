// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::random::striped::get_striped_bool_vec::bool_slice_to_string;
use itertools::Itertools;
use malachite_base::num::random::striped::striped_random_fixed_length_bool_vecs;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::common_values_map::common_values_map_debug;
use malachite_base::test_util::stats::median;

fn striped_random_fixed_length_bool_vecs_helper(
    len: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_median: (&str, Option<&str>),
) {
    let xs = striped_random_fixed_length_bool_vecs(
        EXAMPLE_SEED,
        len,
        mean_stripe_numerator,
        mean_stripe_denominator,
    );
    let values = xs
        .clone()
        .take(20)
        .map(|bs| bool_slice_to_string(&bs))
        .collect_vec();
    let common_values = common_values_map_debug(1000000, 10, xs.clone())
        .into_iter()
        .map(|(bs, freq)| (bool_slice_to_string(&bs), freq))
        .collect_vec();
    let (median_lo, median_hi) = median(xs.take(1000000));
    let median_lo = bool_slice_to_string(&median_lo);
    let median_hi = median_hi.map(|bs| bool_slice_to_string(&bs));
    assert_eq!(
        (
            values.iter().map(String::as_str).collect_vec().as_slice(),
            common_values
                .iter()
                .map(|(s, f)| (s.as_str(), *f))
                .collect_vec()
                .as_slice(),
            (median_lo.as_str(), median_hi.as_deref())
        ),
        (expected_values, expected_common_values, expected_median)
    );
}

#[test]
fn test_striped_random_fixed_length_bool_vecs() {
    striped_random_fixed_length_bool_vecs_helper(0, 10, 1, &[""; 20], &[("", 1000000)], ("", None));
    striped_random_fixed_length_bool_vecs_helper(
        1,
        10,
        1,
        &[
            "0", "0", "0", "0", "0", "1", "0", "1", "0", "1", "1", "0", "0", "0", "1", "0", "1",
            "0", "0", "1",
        ],
        &[("1", 500079), ("0", 499921)],
        ("1", None),
    );
    striped_random_fixed_length_bool_vecs_helper(
        2,
        10,
        1,
        &[
            "00", "00", "00", "00", "00", "11", "00", "11", "00", "11", "11", "00", "00", "00",
            "11", "00", "11", "00", "01", "11",
        ],
        &[("11", 449989), ("00", 449537), ("01", 50384), ("10", 50090)],
        ("10", None),
    );
    striped_random_fixed_length_bool_vecs_helper(
        5,
        10,
        1,
        &[
            "00000", "00000", "00000", "00000", "00011", "11000", "00000", "11111", "01111",
            "11111", "10000", "00011", "00000", "00000", "11000", "00000", "11111", "00000",
            "00000", "11111",
        ],
        &[
            ("11111", 328176),
            ("00000", 327532),
            ("00001", 36685),
            ("10000", 36616),
            ("00111", 36602),
            ("01111", 36495),
            ("11110", 36487),
            ("11000", 36446),
            ("00011", 36354),
            ("11100", 36250),
        ],
        ("10000", None),
    );
}

#[test]
#[should_panic]
fn striped_random_fixed_length_bool_vecs_fail_1() {
    striped_random_fixed_length_bool_vecs(EXAMPLE_SEED, 5, 1, 0);
}

#[test]
#[should_panic]
fn striped_random_fixed_length_bool_vecs_fail_2() {
    striped_random_fixed_length_bool_vecs(EXAMPLE_SEED, 5, 2, 3);
}
