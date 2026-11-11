// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::random::EXAMPLE_SEED;
use malachite_float::big_exponent_float::random::*;
use malachite_float::test_util::big_exponent_float::random_big_exponent_floats_helper_helper;

#[allow(clippy::too_many_arguments)]
fn striped_random_non_negative_finite_big_exponent_floats_helper(
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
        striped_random_non_negative_finite_big_exponent_floats(
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
fn test_striped_random_non_negative_finite_big_exponent_floats() {
    // mean exponent bits = 1, mean stripe = 2, mean precision = 2, P(0) = 1/10
    striped_random_non_negative_finite_big_exponent_floats_helper(
        1,
        1,
        2,
        1,
        2,
        1,
        1,
        10,
        &[
            "0x0.4", "0x1.0", "0x6.0", "0x0.0", "0x2.0", "0x1.4", "0x0.8", "0x1.0", "0x1.0",
            "0x1.6fc", "0x0.8", "0x0.8", "0x8.0", "0x0.0", "0x0.2", "0x1.8", "0x0.8", "0x0.0",
            "0x1.0", "0x1.0",
        ],
        &[
            ("0x1.0", 200046),
            ("0x0.0", 100224),
            ("0x0.8", 100076),
            ("0x2.0", 99876),
            ("0x1.8", 49693),
            ("0x4.0", 25033),
            ("0x3.0", 25013),
            ("0x0.4", 25011),
            ("0x0.c", 24991),
            ("0x8.0", 24989),
        ],
    );
    // mean exponent bits = 4, mean stripe = 4, mean precision = 4, P(0) = 1/10
    striped_random_non_negative_finite_big_exponent_floats_helper(
        4,
        1,
        4,
        1,
        4,
        1,
        1,
        10,
        &[
            "0x0.04",
            "0x0.01",
            "0x1.8eE-184",
            "0x0.0",
            "0x8.0E+15",
            "0x3.8",
            "0xf.0E+1",
            "0x0.ff",
            "0x8.0E-109025478",
            "0x2.0",
            "0x1.0",
            "0x0.4",
            "0x0.01e",
            "0x0.0",
            "0x0.07",
            "0x2.7c1E+33",
            "0x0.08",
            "0x0.0",
            "0x1.0E+1",
            "0x3.cE-96",
        ],
        &[
            ("0x0.0", 100224),
            ("0x1.0", 46334),
            ("0x0.8", 37298),
            ("0x2.0", 37267),
            ("0x0.4", 14991),
            ("0x0.2", 14943),
            ("0x8.0", 14901),
            ("0x4.0", 14763),
            ("0x1.8", 13539),
            ("0x0.c", 10632),
        ],
    );
    // mean exponent bits = 16, mean stripe = 32, mean precision = 64, P(0) = 1/2
    striped_random_non_negative_finite_big_exponent_floats_helper(
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
            "0x1.ffffc000000000000fff8000000000fffffff0007fc00000000000007ffe0007ffffffff1feE-68\
            717379712",
            "0x0.0",
            "0x1.0E-64",
            "0x0.0",
            "0x0.0",
            "0x0.0",
            "0x1.ffff8002E-16",
            "0xf.ffff0000ffff3f00001fcE+1342177279",
            "0x1f.fe00000000001fffffffffff00ffff",
            "0x0.0",
            "0x0.0",
            "0xf.fffffbE+524287",
            "0x0.0",
            "0x0.0",
            "0x0.0",
            "0x0.0",
            "0x0.0",
        ],
        &[
            ("0x0.0", 499546),
            ("0x1.0", 2764),
            ("0x0.8", 2671),
            ("0x2.0", 2557),
            ("0x4.0", 1222),
            ("0x0.2", 1210),
            ("0x8.0", 1208),
            ("0x0.4", 1193),
            ("0x0.02", 1165),
            ("0x8.0E+1", 1132),
        ],
    );
}

#[test]
#[should_panic]
fn striped_random_non_negative_finite_big_exponent_floats_fail_1() {
    striped_random_non_negative_finite_big_exponent_floats(EXAMPLE_SEED, 1, 1, 2, 1, 0, 1, 1, 10);
}

#[test]
#[should_panic]
fn striped_random_non_negative_finite_big_exponent_floats_fail_2() {
    striped_random_non_negative_finite_big_exponent_floats(EXAMPLE_SEED, 1, 1, 2, 1, 1, 0, 1, 10);
}

#[test]
#[should_panic]
fn striped_random_non_negative_finite_big_exponent_floats_fail_3() {
    striped_random_non_negative_finite_big_exponent_floats(EXAMPLE_SEED, 1, 0, 2, 1, 2, 1, 1, 10);
}

#[test]
#[should_panic]
fn striped_random_non_negative_finite_big_exponent_floats_fail_4() {
    striped_random_non_negative_finite_big_exponent_floats(EXAMPLE_SEED, 1, 1, 1, 1, 2, 1, 1, 10);
}

#[test]
#[should_panic]
fn striped_random_non_negative_finite_big_exponent_floats_fail_5() {
    striped_random_non_negative_finite_big_exponent_floats(EXAMPLE_SEED, 1, 1, 2, 1, 2, 1, 11, 10);
}
