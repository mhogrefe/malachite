// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::random::EXAMPLE_SEED;
use malachite_float::big_exponent_float::random::striped_random_big_exponent_floats;
use malachite_float::test_util::big_exponent_float::random_big_exponent_floats_helper_helper;

#[allow(clippy::too_many_arguments)]
fn striped_random_big_exponent_floats_helper(
    mean_exponent_bits_numerator: u64,
    mean_exponent_bits_denominator: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_precision_numerator: u64,
    mean_precision_denominator: u64,
    mean_special_p_numerator: u64,
    mean_special_p_denominator: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
) {
    random_big_exponent_floats_helper_helper(
        striped_random_big_exponent_floats(
            EXAMPLE_SEED,
            mean_exponent_bits_numerator,
            mean_exponent_bits_denominator,
            mean_stripe_numerator,
            mean_stripe_denominator,
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
fn test_striped_random_big_exponent_floats() {
    // mean exponent bits = 1, mean stripe = 2, mean precision = 2, P(special) = 1/10
    striped_random_big_exponent_floats_helper(
        1,
        1,
        2,
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
            "-0x8.0E+3",
            "0x0.2",
            "0x2.0",
            "0x2.0",
            "0x0.0",
            "-0x1.0",
            "Infinity",
            "-0x1.0",
            "0x1.8",
            "-0x1.0",
            "NaN",
            "0x2.0",
            "-0x1.0",
        ],
        &[
            ("-0x1.0", 90669),
            ("0x1.0", 89855),
            ("0x0.0", 89797),
            ("-0x0.8", 45208),
            ("0x2.0", 44941),
            ("0x0.8", 44893),
            ("-0x2.0", 44758),
            ("Infinity", 33640),
            ("NaN", 33393),
            ("-Infinity", 33191),
        ],
    );
    // mean exponent bits = 4, mean stripe = 4, mean precision = 4, P(special) = 1/10
    striped_random_big_exponent_floats_helper(
        4,
        1,
        4,
        1,
        4,
        1,
        1,
        10,
        &[
            "-0x2.0",
            "0x2.0",
            "0xc.0E+2",
            "NaN",
            "0x8.0E+15",
            "0xe.0E+2",
            "0x1.f8c",
            "-0x3.cE-8",
            "0x3.8",
            "0x0.8",
            "0x8.0E+7",
            "0x0.0",
            "-0x5.0E-8",
            "Infinity",
            "-0x0.21",
            "0x1.4E+2",
            "-0x4.0",
            "NaN",
            "0x0.4",
            "-0x1.0E+191",
        ],
        &[
            ("0x0.0", 89797),
            ("Infinity", 33640),
            ("NaN", 33393),
            ("-Infinity", 33191),
            ("-0x1.0", 20995),
            ("0x1.0", 20769),
            ("0x0.8", 16781),
            ("-0x0.8", 16645),
            ("-0x2.0", 16611),
            ("0x2.0", 16567),
        ],
    );
    // mean exponent bits = 16, mean stripe = 32, mean precision = 64, P(special) = 1/2
    striped_random_big_exponent_floats_helper(
        16,
        1,
        32,
        1,
        64,
        1,
        1,
        2,
        &[
            "NaN",
            "Infinity",
            "NaN",
            "-0x3.ffff",
            "Infinity",
            "0x3.fffffeE-128",
            "-Infinity",
            "NaN",
            "Infinity",
            "0x8.00000007ffffffffffffffeE+524287",
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
            ("0x2.0", 690),
            ("-0x1.0", 686),
            ("0x1.0", 677),
            ("-0x2.0", 670),
            ("0x0.8", 639),
            ("-0x0.8", 602),
        ],
    );
}

#[test]
#[should_panic]
fn striped_random_big_exponent_floats_fail_1() {
    striped_random_big_exponent_floats(EXAMPLE_SEED, 1, 1, 2, 1, 0, 1, 1, 10);
}

#[test]
#[should_panic]
fn striped_random_big_exponent_floats_fail_2() {
    striped_random_big_exponent_floats(EXAMPLE_SEED, 1, 1, 2, 1, 1, 0, 1, 10);
}

#[test]
#[should_panic]
fn striped_random_big_exponent_floats_fail_3() {
    striped_random_big_exponent_floats(EXAMPLE_SEED, 1, 0, 2, 1, 2, 1, 1, 10);
}

#[test]
#[should_panic]
fn striped_random_big_exponent_floats_fail_4() {
    striped_random_big_exponent_floats(EXAMPLE_SEED, 1, 1, 1, 1, 2, 1, 1, 10);
}

#[test]
#[should_panic]
fn striped_random_big_exponent_floats_fail_5() {
    striped_random_big_exponent_floats(EXAMPLE_SEED, 1, 1, 2, 1, 2, 1, 11, 10);
}
