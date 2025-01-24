// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::random::striped::get_striped_bool_vec::bool_slice_to_string;
use itertools::Itertools;
use malachite_base::num::random::striped::striped_random_bool_vecs_min_length;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::common_values_map::common_values_map_debug;
use malachite_base::test_util::stats::median;

fn striped_random_bool_vecs_min_length_helper(
    min_length: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_median: (&str, Option<&str>),
) {
    let xs = striped_random_bool_vecs_min_length(
        EXAMPLE_SEED,
        min_length,
        mean_stripe_numerator,
        mean_stripe_denominator,
        mean_length_numerator,
        mean_length_denominator,
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
fn test_striped_random_bool_vecs_min_length() {
    striped_random_bool_vecs_min_length_helper(
        0,
        2,
        1,
        4,
        1,
        &[
            "",
            "00110011000110",
            "0001",
            "0110",
            "0",
            "",
            "00111",
            "10",
            "0100",
            "",
            "111010",
            "",
            "",
            "01111101000",
            "11001100",
            "",
            "100",
            "",
            "01011",
            "000111111",
        ],
        &[
            ("", 199913),
            ("1", 80247),
            ("0", 79926),
            ("10", 32241),
            ("00", 32110),
            ("11", 31988),
            ("01", 31834),
            ("001", 12991),
            ("111", 12989),
            ("010", 12807),
        ],
        ("0100011111011", None),
    );
    striped_random_bool_vecs_min_length_helper(
        3,
        10,
        1,
        4,
        1,
        &[
            "0000", "00000", "00000", "00000", "0000", "10001", "000", "111", "0000", "111", "110",
            "000", "0000", "0001111", "111", "01111", "111", "00000011", "0000", "1111",
        ],
        &[
            ("000", 202463),
            ("111", 202257),
            ("0000", 91181),
            ("1111", 91085),
            ("00000", 41060),
            ("11111", 41041),
            ("100", 22693),
            ("110", 22484),
            ("001", 22434),
            ("011", 22270),
        ],
        ("100", None),
    );
    striped_random_bool_vecs_min_length_helper(
        0,
        11,
        10,
        4,
        1,
        &[
            "",
            "01011010101010",
            "0110",
            "0101",
            "0",
            "",
            "01101",
            "10",
            "0101",
            "",
            "100101",
            "",
            "",
            "01101011010",
            "10101010",
            "",
            "101",
            "",
            "01010",
            "010101001",
        ],
        &[
            ("", 199913),
            ("1", 80247),
            ("0", 79926),
            ("10", 58377),
            ("01", 58287),
            ("010", 42404),
            ("101", 42353),
            ("1010", 30704),
            ("0101", 30613),
            ("10101", 22466),
        ],
        ("010101", None),
    );
    striped_random_bool_vecs_min_length_helper(
        3,
        2,
        1,
        10,
        1,
        &[
            "00110011",
            "01110011101100111010000101",
            "011111010000",
            "01100110010",
            "0111",
            "110",
            "000000011",
            "101110",
            "011000111100000",
            "111",
            "101011001100",
            "0000",
            "010",
            "000010100111010000011",
            "1110010011010111",
            "010",
            "11100100",
            "010",
            "010111100101",
            "10000010011001110111111110100",
        ],
        &[
            ("011", 15722),
            ("101", 15676),
            ("110", 15638),
            ("100", 15613),
            ("010", 15610),
            ("111", 15603),
            ("000", 15516),
            ("001", 15471),
            ("0110", 7099),
            ("1010", 6906),
        ],
        ("100", None),
    );
    striped_random_bool_vecs_min_length_helper(
        0,
        10,
        1,
        10,
        1,
        &[
            "0000000000000000000111000000",
            "000000111111110000",
            "0001111111111100000",
            "000",
            "000000000000",
            "1",
            "00000000000001111111111111111",
            "11111",
            "00",
            "10000001111",
            "111111111101111111111111000000",
            "00000",
            "0000",
            "00000000",
            "11000011111",
            "0001",
            "111111111",
            "000000000",
            "0000000001100111111111111",
            "1",
        ],
        &[
            ("", 90709),
            ("0", 41449),
            ("1", 41338),
            ("00", 33629),
            ("11", 33603),
            ("111", 27832),
            ("000", 27669),
            ("1111", 22666),
            ("0000", 22522),
            ("11111", 18708),
        ],
        (
            "00111111111110000000000011111000000001",
            Some("001111111111100000000000111110011111"),
        ),
    );
    striped_random_bool_vecs_min_length_helper(
        3,
        11,
        10,
        10,
        1,
        &[
            "01011010",
            "01010101101010010101011010",
            "011010110101",
            "01010101010",
            "0101",
            "101",
            "010110101",
            "100101",
            "010101010101001",
            "110",
            "101001010101",
            "0101",
            "010",
            "010101010101010101101",
            "1010110101001010",
            "010",
            "10110101",
            "010",
            "010101010101",
            "10101001010101010100101011010",
        ],
        &[
            ("101", 51809),
            ("010", 51385),
            ("0101", 41252),
            ("1010", 40909),
            ("01010", 33103),
            ("10101", 32685),
            ("101010", 25995),
            ("010101", 25896),
            ("0101010", 20699),
            ("1010101", 20622),
        ],
        ("100", None),
    );
}

#[test]
#[should_panic]
fn striped_random_bool_vecs_min_length_fail_1() {
    striped_random_bool_vecs_min_length(EXAMPLE_SEED, 3, 1, 0, 4, 1);
}

#[test]
#[should_panic]
fn striped_random_bool_vecs_min_length_fail_2() {
    striped_random_bool_vecs_min_length(EXAMPLE_SEED, 3, 2, 3, 4, 1);
}

#[test]
#[should_panic]
fn striped_random_bool_vecs_min_length_fail_3() {
    striped_random_bool_vecs_min_length(EXAMPLE_SEED, 3, 4, 1, 3, 1);
}

#[test]
#[should_panic]
fn striped_random_bool_vecs_min_length_fail_4() {
    striped_random_bool_vecs_min_length(EXAMPLE_SEED, 1, 4, 1, 1, 0);
}

#[test]
#[should_panic]
fn striped_random_bool_vecs_min_length_fail_5() {
    striped_random_bool_vecs_min_length(EXAMPLE_SEED, 0, 4, 1, u64::MAX, u64::MAX - 1);
}
