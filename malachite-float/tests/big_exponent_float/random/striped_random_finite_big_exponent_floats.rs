// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::random::EXAMPLE_SEED;
use malachite_float::big_exponent_float::random::striped_random_finite_big_exponent_floats;
use malachite_float::test_util::big_exponent_float::random_big_exponent_floats_helper_helper;

#[allow(clippy::too_many_arguments)]
fn striped_random_finite_big_exponent_floats_helper(
    mean_exponent_bits_numerator: u64,
    mean_exponent_bits_denominator: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_precision_numerator: u64,
    mean_precision_denominator: u64,
    zero_p_numerator: u64,
    zero_p_denominator: u64,
    expected_values: &[&str],
    expected_values_hex: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_common_values_hex: &[(&str, usize)],
) {
    random_big_exponent_floats_helper_helper(
        striped_random_finite_big_exponent_floats(
            EXAMPLE_SEED,
            mean_exponent_bits_numerator,
            mean_exponent_bits_denominator,
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_precision_numerator,
            mean_precision_denominator,
            zero_p_numerator,
            zero_p_denominator,
        ),
        expected_values,
        expected_values_hex,
        expected_common_values,
        expected_common_values_hex,
    );
}

#[test]
fn test_striped_random_finite_big_exponent_floats() {
    // mean exponent bits = 1, mean stripe = 2, mean precision = 2, P(0) = 1/10
    striped_random_finite_big_exponent_floats_helper(
        1,
        1,
        2,
        1,
        2,
        1,
        1,
        10,
        &[
            "0.75", "0.000038", "0.12", "0.0", "-1.2", "-0.75", "-1.0", "-1.0", "0.75", "0.50",
            "-0.75", "-0.5820", "-7.0", "0.0", "-0.906", "-0.016", "-1.5", "0.0", "32.0", "-0.50",
        ],
        &[
            "0x0.c",
            "0x0.00028",
            "0x0.2",
            "0x0.0",
            "-0x1.4",
            "-0x0.c",
            "-0x1.0",
            "-0x1.0",
            "0x0.c",
            "0x0.8",
            "-0x0.c",
            "-0x0.95",
            "-0x7.0",
            "0x0.0",
            "-0x0.e8",
            "-0x0.04",
            "-0x1.8",
            "0x0.0",
            "0x2.0E+1",
            "-0x0.8",
        ],
        &[
            ("0.0", 100224),
            ("1.0", 99815),
            ("-1.0", 99399),
            ("2.0", 50393),
            ("-2.0", 50084),
            ("-0.50", 49899),
            ("0.50", 49754),
            ("1.5", 25241),
            ("-1.5", 25100),
            ("0.25", 12695),
        ],
        &[
            ("0x0.0", 100224),
            ("0x1.0", 99815),
            ("-0x1.0", 99399),
            ("0x2.0", 50393),
            ("-0x2.0", 50084),
            ("-0x0.8", 49899),
            ("0x0.8", 49754),
            ("0x1.8", 25241),
            ("-0x1.8", 25100),
            ("0x0.4", 12695),
        ],
    );
    // mean exponent bits = 4, mean stripe = 4, mean precision = 4, P(0) = 1/10
    striped_random_finite_big_exponent_floats_helper(
        4,
        1,
        4,
        1,
        4,
        1,
        1,
        10,
        &[
            "0.014641507",
            "14.0",
            "8.0",
            "0.0",
            "-2.0",
            "-6.99e-21",
            "-0.00623",
            "-8.9281e611",
            "0.12",
            "1.102e-16",
            "-6.05e144",
            "-0.0918",
            "-8.3e-15731",
            "0.0",
            "-0.01559",
            "-0.0039",
            "-0.88",
            "0.0",
            "0.0151",
            "-0.25",
        ],
        &[
            "0x0.03bf8bb8",
            "0xe.0",
            "0x8.0",
            "0x0.0",
            "-0x2.0",
            "-0x2.1E-17",
            "-0x0.0198",
            "-0x1.cf8E+508",
            "0x0.2",
            "0x7.fE-14",
            "-0x1.fE+120",
            "-0x0.178",
            "-0x3.8E-13064",
            "0x0.0",
            "-0x0.03fe",
            "-0x0.01",
            "-0x0.e",
            "0x0.0",
            "0x0.03e",
            "-0x0.4",
        ],
        &[
            ("0.0", 100224),
            ("1.0", 23410),
            ("-1.0", 22912),
            ("-2.0", 18898),
            ("2.0", 18701),
            ("0.50", 18689),
            ("-0.50", 18487),
            ("4.0", 7517),
            ("-4.0", 7513),
            ("-0.12", 7478),
        ],
        &[
            ("0x0.0", 100224),
            ("0x1.0", 23410),
            ("-0x1.0", 22912),
            ("-0x2.0", 18898),
            ("0x2.0", 18701),
            ("0x0.8", 18689),
            ("-0x0.8", 18487),
            ("0x4.0", 7517),
            ("-0x4.0", 7513),
            ("-0x0.2", 7478),
        ],
    );
    // mean exponent bits = 16, mean stripe = 32, mean precision = 64, P(0) = 1/2
    striped_random_finite_big_exponent_floats_helper(
        16,
        1,
        32,
        1,
        64,
        1,
        1,
        2,
        &[
            "0.0",
            "0.0",
            "0.0",
            "1.11253692925764807731207842861963719311292187707e-308",
            "0.0",
            "2.996032157743878448e82746495135",
            "0.0",
            "0.0",
            "0.0",
            "8.0",
            "-18446739950540947455.9999999999999999999992",
            "-1.17093844919e-38",
            "0.0",
            "0.0",
            "-8.1e-9860",
            "0.0",
            "0.0",
            "0.0",
            "0.0",
            "0.0",
        ],
        &[
            "0x0.0",
            "0x0.0",
            "0x0.0",
            "0x2.0000000007ffffffe00000000003dfffffffffE-256",
            "0x0.0",
            "0xf.fffffffffffff8E+68719476735",
            "0x0.0",
            "0x0.0",
            "0x0.0",
            "0x8.0",
            "-0xfffffc3fffffffff.fffffffffffffffffc",
            "-0x3.fc07ffffE-32",
            "0x0.0",
            "0x0.0",
            "-0x1.cE-8188",
            "0x0.0",
            "0x0.0",
            "0x0.0",
            "0x0.0",
            "0x0.0",
        ],
        &[
            ("0.0", 499546),
            ("-1.0", 1394),
            ("1.0", 1334),
            ("-0.50", 1310),
            ("0.50", 1282),
            ("-2.0", 1260),
            ("2.0", 1255),
            ("4.0", 640),
            ("-4.0", 638),
            ("8.0", 631),
        ],
        &[
            ("0x0.0", 499546),
            ("-0x1.0", 1394),
            ("0x1.0", 1334),
            ("-0x0.8", 1310),
            ("0x0.8", 1282),
            ("-0x2.0", 1260),
            ("0x2.0", 1255),
            ("0x4.0", 640),
            ("-0x4.0", 638),
            ("0x8.0", 631),
        ],
    );
}

#[test]
#[should_panic]
fn striped_random_finite_big_exponent_floats_fail_1() {
    striped_random_finite_big_exponent_floats(EXAMPLE_SEED, 1, 1, 2, 1, 0, 1, 1, 10);
}

#[test]
#[should_panic]
fn striped_random_finite_big_exponent_floats_fail_2() {
    striped_random_finite_big_exponent_floats(EXAMPLE_SEED, 1, 1, 2, 1, 1, 0, 1, 10);
}

#[test]
#[should_panic]
fn striped_random_finite_big_exponent_floats_fail_3() {
    striped_random_finite_big_exponent_floats(EXAMPLE_SEED, 1, 0, 2, 1, 2, 1, 1, 10);
}

#[test]
#[should_panic]
fn striped_random_finite_big_exponent_floats_fail_4() {
    striped_random_finite_big_exponent_floats(EXAMPLE_SEED, 1, 1, 1, 1, 2, 1, 1, 10);
}

#[test]
#[should_panic]
fn striped_random_finite_big_exponent_floats_fail_5() {
    striped_random_finite_big_exponent_floats(EXAMPLE_SEED, 1, 1, 2, 1, 2, 1, 11, 10);
}
