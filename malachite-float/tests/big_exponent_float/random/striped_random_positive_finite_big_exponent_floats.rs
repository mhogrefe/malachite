// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::random::EXAMPLE_SEED;
use malachite_float::big_exponent_float::random::striped_random_positive_finite_big_exponent_floats;
use malachite_float::test_util::big_exponent_float::random_big_exponent_floats_helper_helper;

#[allow(clippy::too_many_arguments)]
fn striped_random_positive_finite_big_exponent_floats_helper(
    mean_exponent_bits_numerator: u64,
    mean_exponent_bits_denominator: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_precision_numerator: u64,
    mean_precision_denominator: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
) {
    random_big_exponent_floats_helper_helper(
        striped_random_positive_finite_big_exponent_floats(
            EXAMPLE_SEED,
            mean_exponent_bits_numerator,
            mean_exponent_bits_denominator,
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_precision_numerator,
            mean_precision_denominator,
        ),
        expected_values,
        expected_common_values,
    );
}

#[test]
fn test_striped_random_positive_finite_big_exponent_floats() {
    // mean exponent bits = 1, mean stripe = 2, mean precision = 2
    striped_random_positive_finite_big_exponent_floats_helper(
        1,
        1,
        2,
        1,
        2,
        1,
        &[
            "0x1.8",
            "0x2.0",
            "0x4.0",
            "0x1.0",
            "0x1.0",
            "0x0.004",
            "0x8.0",
            "0x1.c",
            "0x1.0",
            "0x1.0",
            "0x1.0",
            "0x1.0",
            "0x1.0",
            "0x2.0",
            "0x1.8E+16",
            "0x8.0E-7",
            "0x8.0E+1",
            "0x2.0E+1",
            "0x0.8",
            "0x0.c",
        ],
        &[
            ("0x1.0", 222084),
            ("0x0.8", 111419),
            ("0x2.0", 110761),
            ("0x1.8", 55952),
            ("0x3.0", 27990),
            ("0x0.2", 27761),
            ("0x0.c", 27735),
            ("0x0.4", 27665),
            ("0x4.0", 27620),
            ("0x8.0", 27533),
        ],
    );
    // mean exponent bits = 4, mean stripe = 4, mean precision = 4
    striped_random_positive_finite_big_exponent_floats_helper(
        4,
        1,
        4,
        1,
        4,
        1,
        &[
            "0x0.9",
            "0x0.4",
            "0x1.0E-8",
            "0x2.0E-6",
            "0x1.0",
            "0x0.3e",
            "0x1.ff8E+16",
            "0x8.0",
            "0x0.0002",
            "0x8.0E+1",
            "0x8.0E+16",
            "0x8.0E+15",
            "0x0.8",
            "0x5.0",
            "0x2.0",
            "0x2.0",
            "0x0.218",
            "0x1.eE+8",
            "0x0.012",
            "0x4.33cE+7",
        ],
        &[
            ("0x1.0", 51683),
            ("0x2.0", 41404),
            ("0x0.8", 41218),
            ("0x4.0", 16552),
            ("0x8.0", 16546),
            ("0x0.2", 16481),
            ("0x0.4", 16424),
            ("0x1.8", 14988),
            ("0x3.0", 12059),
            ("0x0.c", 11686),
        ],
    );
    // mean exponent bits = 16, mean stripe = 32, mean precision = 64
    striped_random_positive_finite_big_exponent_floats_helper(
        16,
        1,
        32,
        1,
        64,
        1,
        &[
            "0x0.1c",
            "0x1.ffff7e00003ffffffcE-8",
            "0x1.0E-64",
            "0x8.0003fffffe0001ffffcE+4095",
            "0x8.0E+127",
            "0x1.ffffffffffffff8000000fffffffffffc003ffE+256",
            "0x2.001fffffffffffffe000007fffffff0000000000000001ff8E-4294967296",
            "0x2.0007fcE-33554432",
            "0x0.ff0040fffe0fffffffffc",
            "0x3.fcE-8",
            "0x3.ff000007fffcE-7864321",
            "0xf.f8E+8191",
            "0x1.0E-75557863725922913353664",
            "0x3.ff",
            "0x8.00000c00000000007ffffffffffffcE+1048575",
            "0x0.0003fffffc",
            "0x1.ffff803cE+67092480",
            "0x1.000000000000003fffff800001ffffff8E+67108864",
            "0x1.ffffffffcE+9007199254740736",
            "0x3.f878E-256",
        ],
        &[
            ("0x1.0", 5557),
            ("0x0.8", 5162),
            ("0x2.0", 5145),
            ("0x0.4", 2525),
            ("0x4.0", 2469),
            ("0x8.0", 2448),
            ("0x0.2", 2398),
            ("0x0.1", 2252),
            ("0x1.0E+1", 2211),
            ("0x8.0E+1", 2197),
        ],
    );
}

#[test]
#[should_panic]
fn striped_random_positive_finite_big_exponent_floats_fail_1() {
    striped_random_positive_finite_big_exponent_floats(EXAMPLE_SEED, 1, 1, 2, 1, 0, 1);
}

#[test]
#[should_panic]
fn striped_random_positive_finite_big_exponent_floats_fail_2() {
    striped_random_positive_finite_big_exponent_floats(EXAMPLE_SEED, 1, 1, 2, 1, 1, 0);
}

#[test]
#[should_panic]
fn striped_random_positive_finite_big_exponent_floats_fail_3() {
    striped_random_positive_finite_big_exponent_floats(EXAMPLE_SEED, 1, 0, 2, 1, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_positive_finite_big_exponent_floats_fail_4() {
    striped_random_positive_finite_big_exponent_floats(EXAMPLE_SEED, 1, 1, 1, 1, 2, 1);
}
