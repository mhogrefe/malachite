// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::random::EXAMPLE_SEED;
use malachite_float::big_exponent_float::random::random_big_exponent_floats;
use malachite_float::test_util::big_exponent_float::random_big_exponent_floats_helper_helper;

#[allow(clippy::too_many_arguments)]
fn random_big_exponent_floats_helper(
    mean_exponent_bits_numerator: u64,
    mean_exponent_bits_denominator: u64,
    mean_precision_numerator: u64,
    mean_precision_denominator: u64,
    mean_special_p_numerator: u64,
    mean_special_p_denominator: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
) {
    random_big_exponent_floats_helper_helper(
        random_big_exponent_floats(
            EXAMPLE_SEED,
            mean_exponent_bits_numerator,
            mean_exponent_bits_denominator,
            mean_precision_numerator,
            mean_precision_denominator,
            mean_special_p_numerator,
            mean_special_p_denominator,
        ),
        expected_values,
        expected_common_values,
    );
}

#[test]
fn test_random_big_exponent_floats() {
    // mean exponent bits = 1, mean precision = 2, P(special) = 1/10
    random_big_exponent_floats_helper(
        1,
        1,
        2,
        1,
        1,
        10,
        &[
            "-0x1.0",
            "0x2.0",
            "0x1.0",
            "NaN",
            "0x1.0",
            "0x0.8",
            "0x1.8",
            "-0x8.0E+2",
            "0x0.2",
            "0x2.0",
            "0x2.0",
            "0x0.0",
            "-0x1.0",
            "Infinity",
            "-0x1.0",
            "0x1.0",
            "-0x1.8",
            "NaN",
            "0x2.0",
            "-0x1.0",
        ],
        &[
            ("-0x1.0", 90637),
            ("0x1.0", 89936),
            ("0x0.0", 89797),
            ("-0x0.8", 45148),
            ("0x0.8", 45029),
            ("0x2.0", 44931),
            ("-0x2.0", 44874),
            ("Infinity", 33640),
            ("NaN", 33393),
            ("-Infinity", 33191),
        ],
    );
    // mean exponent bits = 4, mean precision = 4, P(special) = 1/10
    random_big_exponent_floats_helper(
        4,
        1,
        4,
        1,
        1,
        10,
        &[
            "-0x2.0",
            "0x3.0",
            "0x8.0E+2",
            "NaN",
            "0x3.0E+10",
            "0x4.0E+2",
            "0x1.1ec",
            "-0x1.aE-6",
            "0x3.8",
            "0x0.8",
            "0x2.0E+6",
            "0x0.0",
            "-0x0.00006",
            "Infinity",
            "-0x0.56",
            "0x8.0E+3",
            "-0xe.0",
            "NaN",
            "0x0.4",
            "-0x4.0E+224",
        ],
        &[
            ("0x0.0", 89797),
            ("Infinity", 33640),
            ("NaN", 33393),
            ("-Infinity", 33191),
            ("-0x1.0", 18056),
            ("0x1.0", 17980),
            ("0x0.8", 14405),
            ("-0x0.8", 14361),
            ("0x2.0", 14349),
            ("-0x2.0", 14320),
        ],
    );
    // mean exponent bits = 16, mean precision = 64, P(special) = 1/2
    random_big_exponent_floats_helper(
        16,
        1,
        64,
        1,
        1,
        2,
        &[
            "NaN",
            "Infinity",
            "NaN",
            "-0x3.359a",
            "Infinity",
            "0x2.f382fe0c74E-77",
            "-Infinity",
            "NaN",
            "Infinity",
            "0x8.fcbaf75de590d1977062d7cE+429174",
            "0x0.0",
            "0x0.0",
            "Infinity",
            "Infinity",
            "0x0.0",
            "NaN",
            "Infinity",
            "Infinity",
            "Infinity",
            "Infinity",
        ],
        &[
            ("0x0.0", 249619),
            ("NaN", 166801),
            ("Infinity", 166731),
            ("-Infinity", 166014),
            ("0x0.8", 120),
            ("0x2.0", 118),
            ("0x1.0", 111),
            ("-0x2.0", 111),
            ("-0x1.0", 107),
            ("-0x0.8", 99),
        ],
    );
}

#[test]
#[should_panic]
fn random_big_exponent_floats_fail_1() {
    random_big_exponent_floats(EXAMPLE_SEED, 1, 1, 0, 1, 1, 10);
}

#[test]
#[should_panic]
fn random_big_exponent_floats_fail_2() {
    random_big_exponent_floats(EXAMPLE_SEED, 1, 1, 1, 0, 1, 10);
}

#[test]
#[should_panic]
fn random_big_exponent_floats_fail_3() {
    random_big_exponent_floats(EXAMPLE_SEED, 1, 0, 2, 1, 1, 10);
}

#[test]
#[should_panic]
fn random_big_exponent_floats_fail_4() {
    random_big_exponent_floats(EXAMPLE_SEED, 1, 1, 2, 1, 11, 10);
}
