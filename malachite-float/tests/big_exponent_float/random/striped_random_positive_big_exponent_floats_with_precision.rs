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

fn striped_random_positive_big_exponent_floats_with_precision_helper(
    mean_exponent_bits_numerator: u64,
    mean_exponent_bits_denominator: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    prec: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
) {
    let xs = striped_random_positive_big_exponent_floats_with_precision(
        EXAMPLE_SEED,
        mean_exponent_bits_numerator,
        mean_exponent_bits_denominator,
        mean_stripe_numerator,
        mean_stripe_denominator,
        prec,
    );
    for x in xs.clone().take(10000) {
        assert_eq!(x.get_prec(), Some(prec));
        assert!(x.is_sign_positive());
    }
    random_big_exponent_floats_helper_helper(xs, expected_values, expected_common_values);
}

#[test]
fn test_striped_random_positive_big_exponent_floats_with_precision() {
    // mean exponent bits = 1, mean stripe = 2, precision = 1
    striped_random_positive_big_exponent_floats_with_precision_helper(
        1,
        1,
        2,
        1,
        1,
        &[
            "0x1.0",
            "0x2.0",
            "0x4.0",
            "0x1.0",
            "0x1.0",
            "0x0.004",
            "0x8.0",
            "0x1.0",
            "0x1.0",
            "0x1.0",
            "0x1.0",
            "0x1.0",
            "0x1.0",
            "0x2.0",
            "0x1.0E+16",
            "0x8.0E-7",
            "0x8.0E+1",
            "0x2.0E+1",
            "0x0.8",
            "0x0.8",
        ],
        &[
            ("0x1.0", 333732),
            ("0x0.8", 166936),
            ("0x2.0", 166566),
            ("0x0.4", 41637),
            ("0x0.2", 41580),
            ("0x4.0", 41488),
            ("0x8.0", 41208),
            ("0x2.0E+1", 10493),
            ("0x0.02", 10441),
            ("0x4.0E+1", 10427),
        ],
    );
    // mean exponent bits = 4, mean stripe = 4, precision = 10
    striped_random_positive_big_exponent_floats_with_precision_helper(
        4,
        1,
        4,
        1,
        10,
        &[
            "0x0.c7c",
            "0x0.402",
            "0x1.df8E-8",
            "0x2.1fE-6",
            "0x1.708",
            "0x0.3f9",
            "0x1.008E+16",
            "0xf.e4",
            "0x0.0003bf",
            "0xe7.c",
            "0xe.04E+16",
            "0xe.04E+15",
            "0x0.fc4",
            "0x4.02",
            "0x3.01",
            "0x2.c1",
            "0x0.303",
            "0x1.f08E+8",
            "0x0.01018",
            "0x5.06E+7",
        ],
        &[
            ("0x1.ff8", 7549),
            ("0x1.008", 7336),
            ("0x3.ff", 6089),
            ("0x0.ffc", 6023),
            ("0x2.01", 5951),
            ("0x0.804", 5908),
            ("0x1.0f8", 2535),
            ("0x1.7f8", 2533),
            ("0x1.f88", 2531),
            ("0x1.1f8", 2515),
        ],
    );
    // mean exponent bits = 16, mean stripe = 32, precision = 100
    striped_random_positive_big_exponent_floats_with_precision_helper(
        16,
        1,
        32,
        1,
        100,
        &[
            "0x0.1ffffffe000000007000000002",
            "0x1.00000000000003ffffffffffeE-8",
            "0x1.fffffff8000000003fffffffeE-64",
            "0x8.fffffff00000000300000001E+4095",
            "0x8.000000000007ffffff000007E+127",
            "0x1.ffc0000fffff8000000007ffeE+256",
            "0x2.1fff800000000000000fffffcE-4294967296",
            "0x3.f000001fffe00000000000004E-33554432",
            "0x0.fc00000003ffffffffeffffff",
            "0x3.ffffff800000000000007fffcE-8",
            "0x3.fffffffffc0003ffffffe0ffcE-7864321",
            "0xf.fffff00fffffffffffffffffE+8191",
            "0x1.fffffffffff8001fffffffffeE-75557863725922913353664",
            "0x2.00007c00000000000000003fc",
            "0xf.ffffff80007fffffffffffffE+1048575",
            "0x0.00020000fe0000003f007ff000004",
            "0x1.ffffffffc003f000003ffe7feE+67092480",
            "0x1.fffffffe003e07ffff8000002E+67108864",
            "0x1.0001ffffffffffffffffffffeE+9007199254740736",
            "0x2.00003f007c0006000000001fcE-256",
        ],
        &[
            ("0x1.0000000000000000000000002", 720),
            ("0x1.ffffffffffffffffffffffffe", 720),
            ("0x0.fffffffffffffffffffffffff", 690),
            ("0x2.0000000000000000000000004", 678),
            ("0x3.ffffffffffffffffffffffffc", 645),
            ("0x0.8000000000000000000000001", 624),
            ("0x0.3ffffffffffffffffffffffffc", 324),
            ("0x0.20000000000000000000000004", 321),
            ("0xf.ffffffffffffffffffffffff", 315),
            ("0x7.ffffffffffffffffffffffff8", 313),
        ],
    );
}

#[test]
#[should_panic]
fn striped_random_positive_big_exponent_floats_with_precision_fail_1() {
    striped_random_positive_big_exponent_floats_with_precision(EXAMPLE_SEED, 1, 1, 2, 1, 0);
}

#[test]
#[should_panic]
fn striped_random_positive_big_exponent_floats_with_precision_fail_2() {
    striped_random_positive_big_exponent_floats_with_precision(EXAMPLE_SEED, 1, 0, 2, 1, 10);
}

#[test]
#[should_panic]
fn striped_random_positive_big_exponent_floats_with_precision_fail_3() {
    striped_random_positive_big_exponent_floats_with_precision(EXAMPLE_SEED, 1, 1, 1, 1, 10);
}
