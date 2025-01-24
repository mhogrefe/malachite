// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::random::striped::get_striped_bool_vec::bool_slice_to_string;
use itertools::Itertools;
use malachite_base::num::random::striped::striped_random_bool_vecs_length_range;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::common_values_map::common_values_map_debug;
use malachite_base::test_util::stats::median;

fn striped_random_bool_vecs_length_range_helper(
    a: u64,
    b: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_median: (&str, Option<&str>),
) {
    let xs = striped_random_bool_vecs_length_range(
        EXAMPLE_SEED,
        a,
        b,
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
fn test_striped_random_bool_vecs_length_range() {
    striped_random_bool_vecs_length_range_helper(
        3,
        10,
        2,
        1,
        &[
            "00110011",
            "01110011",
            "00100110",
            "00010111",
            "001010000",
            "110",
            "0100",
            "1110011",
            "011001011",
            "111000",
            "111110010",
            "0001001",
            "00011110",
            "00000",
            "1110101",
            "0011001",
            "11111",
            "01000010",
            "01001110",
            "10111",
        ],
        &[
            ("001", 18041),
            ("110", 18008),
            ("111", 17919),
            ("100", 17906),
            ("011", 17905),
            ("000", 17819),
            ("010", 17792),
            ("101", 17782),
            ("0111", 9023),
            ("1011", 9023),
        ],
        ("100", None),
    );
    striped_random_bool_vecs_length_range_helper(
        3,
        10,
        10,
        1,
        &[
            "00000000",
            "00000000",
            "00000111",
            "01111111",
            "000001111",
            "111",
            "0001",
            "1111110",
            "000000000",
            "111000",
            "111111111",
            "0000000",
            "00000000",
            "00000",
            "1110000",
            "0000000",
            "11111",
            "00000000",
            "01111110",
            "11111",
        ],
        &[
            ("000", 58009),
            ("111", 57974),
            ("1111", 52110),
            ("0000", 51905),
            ("00000", 47047),
            ("11111", 46660),
            ("111111", 42411),
            ("000000", 41880),
            ("0000000", 38018),
            ("1111111", 37708),
        ],
        ("100", None),
    );
    striped_random_bool_vecs_length_range_helper(
        3,
        10,
        11,
        10,
        &[
            "01011010",
            "01010101",
            "00101011",
            "01010101",
            "001010010",
            "101",
            "0010",
            "1010101",
            "010101010",
            "101010",
            "100101011",
            "0101010",
            "01010101",
            "01001",
            "1101011",
            "0101010",
            "10101",
            "01010101",
            "01010101",
            "10101",
        ],
        &[
            ("010", 59241),
            ("101", 59092),
            ("1010", 53578),
            ("0101", 53360),
            ("01010", 48685),
            ("10101", 48527),
            ("010101", 44471),
            ("101010", 44350),
            ("0101010", 40536),
            ("1010101", 40115),
        ],
        ("100", None),
    );
}

#[test]
#[should_panic]
fn striped_random_bool_vecs_length_range_fail_1() {
    striped_random_bool_vecs_length_range(EXAMPLE_SEED, 3, 10, 1, 0);
}

#[test]
#[should_panic]
fn striped_random_bool_vecs_length_range_fail_2() {
    striped_random_bool_vecs_length_range(EXAMPLE_SEED, 3, 10, 2, 3);
}

#[test]
#[should_panic]
fn striped_random_bool_vecs_length_range_fail_3() {
    striped_random_bool_vecs_length_range(EXAMPLE_SEED, 1, 1, 4, 1);
}
