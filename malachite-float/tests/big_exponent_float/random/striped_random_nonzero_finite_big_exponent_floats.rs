// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::random::EXAMPLE_SEED;
use malachite_float::big_exponent_float::random::striped_random_nonzero_finite_big_exponent_floats;
use malachite_float::test_util::big_exponent_float::random_big_exponent_floats_helper_helper;

#[allow(clippy::too_many_arguments)]
fn striped_random_nonzero_finite_big_exponent_floats_helper(
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
        striped_random_nonzero_finite_big_exponent_floats(
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
fn test_striped_random_nonzero_finite_big_exponent_floats() {
    // mean exponent bits = 1, mean stripe = 2, mean precision = 2
    striped_random_nonzero_finite_big_exponent_floats_helper(
        1,
        1,
        2,
        1,
        2,
        1,
        &[
            "-0x0.4",
            "-0x1.0",
            "-0x6.0",
            "0x2.0",
            "-0x1.4",
            "0x0.8",
            "-0x1.0",
            "-0x1.0",
            "-0x1.6fc",
            "0x0.8",
            "0x0.8",
            "0x8.0",
            "-0x0.2",
            "-0x1.8",
            "0x0.8",
            "-0x1.0",
            "-0x1.0",
            "-0x2.0",
            "-0x5.0",
            "-0x1.cE+2",
        ],
        &[
            ("0x1.0", 111352),
            ("-0x1.0", 110909),
            ("0x2.0", 55924),
            ("-0x0.8", 55632),
            ("0x0.8", 55538),
            ("-0x2.0", 55240),
            ("0x1.8", 27628),
            ("-0x1.8", 27554),
            ("0x4.0", 14109),
            ("-0x3.0", 13973),
        ],
    );
    // mean exponent bits = 4, mean stripe = 4, mean precision = 4
    striped_random_nonzero_finite_big_exponent_floats_helper(
        4,
        1,
        4,
        1,
        4,
        1,
        &[
            "-0x0.04",
            "-0x0.01",
            "-0x1.8eE-184",
            "0x8.0E+15",
            "-0x3.8",
            "0xf.0E+1",
            "-0x0.ff",
            "-0x8.0E-109025478",
            "-0x2.0",
            "0x1.0",
            "0x0.4",
            "0x0.01e",
            "-0x0.07",
            "-0x2.7c1E+33",
            "0x0.08",
            "-0x1.0E+1",
            "-0x3.cE-96",
            "-0x0.8",
            "-0x1.eE-63",
            "-0x0.c",
        ],
        &[
            ("0x1.0", 25880),
            ("-0x1.0", 25612),
            ("0x0.8", 20837),
            ("-0x2.0", 20750),
            ("-0x0.8", 20656),
            ("0x2.0", 20630),
            ("-0x0.2", 8408),
            ("-0x8.0", 8341),
            ("0x0.4", 8312),
            ("0x0.2", 8293),
        ],
    );
    // mean exponent bits = 16, mean stripe = 32, mean precision = 64
    striped_random_nonzero_finite_big_exponent_floats_helper(
        16,
        1,
        32,
        1,
        64,
        1,
        &[
            "-0x1.ffffc000000000000fff8000000000fffffff0007fc00000000000007ffe0007ffffffff1feE-6\
            8717379712",
            "-0x1.0E-64",
            "-0x1.ffff8002E-16",
            "0xf.ffff0000ffff3f00001fcE+1342177279",
            "-0x1f.fe00000000001fffffffffff00ffff",
            "0xf.fffffbE+524287",
            "-0xf.e0000000003fffffffffff8E+63",
            "-0xf.fffffffffffE+67108863",
            "-0x2.0E-8192",
            "0x1.cE-34909494181888",
            "0x1.ff8E+67108864",
            "0x1.00ffffffE-128",
            "-0x3.0E-16",
            "-0x3.cE-8",
            "0x1.0E+34359738240",
            "-0x2.0E-2048",
            "-0x0.f",
            "-0x1.9ff3ffffffffffffffffE-28672",
            "-0xf.f003fffE+4722367045819598110719",
            "-0x1.fff8E-8589926400",
        ],
        &[
            ("-0x1.0", 2785),
            ("0x1.0", 2690),
            ("0x0.8", 2644),
            ("-0x0.8", 2572),
            ("0x2.0", 2564),
            ("-0x2.0", 2563),
            ("0x0.2", 1262),
            ("0x4.0", 1250),
            ("-0x4.0", 1242),
            ("-0x8.0", 1235),
        ],
    );
}

#[test]
#[should_panic]
fn striped_random_nonzero_finite_big_exponent_floats_fail_1() {
    striped_random_nonzero_finite_big_exponent_floats(EXAMPLE_SEED, 1, 1, 2, 1, 0, 1);
}

#[test]
#[should_panic]
fn striped_random_nonzero_finite_big_exponent_floats_fail_2() {
    striped_random_nonzero_finite_big_exponent_floats(EXAMPLE_SEED, 1, 1, 2, 1, 1, 0);
}

#[test]
#[should_panic]
fn striped_random_nonzero_finite_big_exponent_floats_fail_3() {
    striped_random_nonzero_finite_big_exponent_floats(EXAMPLE_SEED, 1, 0, 2, 1, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_nonzero_finite_big_exponent_floats_fail_4() {
    striped_random_nonzero_finite_big_exponent_floats(EXAMPLE_SEED, 1, 1, 1, 1, 2, 1);
}
