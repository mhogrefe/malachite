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
    expected_common_values: &[(&str, usize)],
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
        expected_common_values,
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
