// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::random::striped::get_striped_bool_vec::bool_slice_to_string;
use itertools::Itertools;
use malachite_base::num::random::striped::striped_random_bool_vecs;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::common_values_map::common_values_map_debug;
use malachite_base::test_util::stats::median;

fn striped_random_bool_vecs_helper(
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_median: (&str, Option<&str>),
) {
    let xs = striped_random_bool_vecs(
        EXAMPLE_SEED,
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
fn test_striped_random_bool_vecs() {
    striped_random_bool_vecs_helper(
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
    striped_random_bool_vecs_helper(
        10,
        1,
        4,
        1,
        &[
            "",
            "00000000000000",
            "0000",
            "0001",
            "0",
            "",
            "00011",
            "11",
            "0000",
            "",
            "111111",
            "",
            "",
            "01111111100",
            "11111000",
            "",
            "111",
            "",
            "00000",
            "000111111",
        ],
        &[
            ("", 199913),
            ("1", 80247),
            ("0", 79926),
            ("11", 57775),
            ("00", 57583),
            ("000", 41616),
            ("111", 41544),
            ("0000", 29747),
            ("1111", 29589),
            ("11111", 21524),
        ],
        ("00000001111", None),
    );
    striped_random_bool_vecs_helper(
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
    striped_random_bool_vecs_helper(
        2,
        1,
        10,
        1,
        &[
            "0011001100011000100110001011",
            "000101000001011110",
            "0011001101000011111",
            "000",
            "011011101100",
            "1",
            "00111100000001010011001111101",
            "11110",
            "01",
            "10110001011",
            "111100001101100101000100011011",
            "01010",
            "0111",
            "00110101",
            "11111011001",
            "0011",
            "110111111",
            "000101101",
            "0110111011110110011001000",
            "1",
        ],
        &[
            ("", 90709),
            ("0", 41449),
            ("1", 41338),
            ("00", 18773),
            ("10", 18748),
            ("11", 18731),
            ("01", 18531),
            ("001", 8685),
            ("111", 8639),
            ("010", 8624),
        ],
        ("011011101010", None),
    );
    striped_random_bool_vecs_helper(
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
    striped_random_bool_vecs_helper(
        11,
        10,
        10,
        1,
        &[
            "0101101010101011010100101010",
            "011010110101101010",
            "0101010101010101011",
            "010",
            "010110101010",
            "1",
            "01010101001101011010101010101",
            "10101",
            "01",
            "10101010101",
            "101101010110101001010101001010",
            "01010",
            "0101",
            "01010101",
            "10101101010",
            "0101",
            "101011010",
            "010010101",
            "0101010101010101010101011",
            "1",
        ],
        &[
            ("", 90709),
            ("0", 41449),
            ("1", 41338),
            ("10", 34030),
            ("01", 33878),
            ("010", 28388),
            ("101", 28324),
            ("1010", 23483),
            ("0101", 23162),
            ("10101", 19518),
        ],
        ("01011010", None),
    );
    striped_random_bool_vecs_helper(
        2,
        1,
        1,
        4,
        &["", "", "0", "0", "00", "", "", "", "", "", "", "", "01", "0", "", "11", "", "", "", ""],
        &[
            ("", 800023),
            ("1", 80189),
            ("0", 79739),
            ("00", 8218),
            ("01", 8023),
            ("11", 7972),
            ("10", 7920),
            ("001", 819),
            ("110", 812),
            ("101", 810),
        ],
        ("", None),
    );
    striped_random_bool_vecs_helper(
        10,
        1,
        1,
        4,
        &["", "", "0", "0", "00", "", "", "", "", "", "", "", "00", "0", "", "11", "", "", "", ""],
        &[
            ("", 800023),
            ("1", 80189),
            ("0", 79739),
            ("00", 14590),
            ("11", 14351),
            ("000", 2591),
            ("111", 2530),
            ("01", 1651),
            ("10", 1541),
            ("1111", 460),
        ],
        ("", None),
    );
    striped_random_bool_vecs_helper(
        11,
        10,
        1,
        4,
        &["", "", "0", "0", "01", "", "", "", "", "", "", "", "01", "0", "", "10", "", "", "", ""],
        &[
            ("", 800023),
            ("1", 80189),
            ("0", 79739),
            ("01", 14758),
            ("10", 14493),
            ("101", 2640),
            ("010", 2614),
            ("00", 1483),
            ("11", 1399),
            ("0101", 467),
        ],
        ("", None),
    );
}

#[test]
#[should_panic]
fn striped_random_bool_vecs_fail_1() {
    striped_random_bool_vecs(EXAMPLE_SEED, 1, 0, 4, 1);
}

#[test]
#[should_panic]
fn striped_random_bool_vecs_fail_2() {
    striped_random_bool_vecs(EXAMPLE_SEED, 2, 3, 4, 1);
}

#[test]
#[should_panic]
fn striped_random_bool_vecs_fail_3() {
    striped_random_bool_vecs(EXAMPLE_SEED, 4, 1, 0, 1);
}

#[test]
#[should_panic]
fn striped_random_bool_vecs_fail_4() {
    striped_random_bool_vecs(EXAMPLE_SEED, 4, 1, 1, 0);
}

#[test]
#[should_panic]
fn striped_random_bool_vecs_fail_5() {
    striped_random_bool_vecs(EXAMPLE_SEED, 4, 1, u64::MAX, u64::MAX - 1);
}
