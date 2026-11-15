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
    expected_values_hex: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_common_values_hex: &[(&str, usize)],
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
    random_big_exponent_floats_helper_helper(
        xs,
        expected_values,
        expected_values_hex,
        expected_common_values,
        expected_common_values_hex,
    );
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
            "1.0", "2.0", "4.0", "1.0", "1.0", "0.00098", "8.0", "1.0", "1.0", "1.0", "1.0", "1.0",
            "1.0", "2.0", "1.8e19", "3.0e-8", "1.3e2", "32.0", "0.50", "0.50",
        ],
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
            ("1.0", 333732),
            ("0.50", 166936),
            ("2.0", 166566),
            ("0.25", 41637),
            ("0.12", 41580),
            ("4.0", 41488),
            ("8.0", 41208),
            ("32.0", 10493),
            ("0.0078", 10441),
            ("64.0", 10427),
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
            "0.78027",
            "0.25049",
            "4.3610e-10",
            "1.2643e-7",
            "1.4395",
            "0.24829",
            "1.8483e19",
            "15.891",
            "0.000057161",
            "231.75",
            "2.5854e20",
            "1.6159e19",
            "0.98535",
            "4.0078",
            "3.0039",
            "2.7539",
            "0.18823",
            "8.3299e9",
            "0.0039291",
            "1.3485e9",
        ],
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
            ("1.9980", 7549),
            ("1.0020", 7336),
            ("3.9961", 6089),
            ("0.99902", 6023),
            ("2.0039", 5951),
            ("0.50098", 5908),
            ("1.0605", 2535),
            ("1.4980", 2533),
            ("1.9707", 2531),
            ("1.1230", 2515),
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
            "0.12499999953433871271597766471072",
            "2.3283064365386964183094707114069e-10",
            "1.7272337094102771783802155265270e-77",
            "6.6922396586143691803352744459638e4931",
            "6.7039039649713223668381527667631e153",
            "3.5936307167195434366496440288201e308",
            "2.2912539186358427683311964706409e-5171655946",
            "3.2911763999366520912172202894665e-40403562",
            "0.98437500000363797880707847806107",
            "9.3132256767658461171777171864806e-10",
            "3.4350998566080552476553257366251e-9469586",
            "1.4154609470064655430840280726490e9864",
            "1.8253888610284194330620239159115e-90980733559177026567096",
            "2.0000073909759521484375000008046",
            "2.0650635359894660317059781887247e1262611",
            "0.000030517809136654263664954738033085",
            "1.4288050479115614196581904805846e80787396",
            "2.8626536776242976812452778615845e80807124",
            "4.4084177451738462596149808082555e10845748610396873",
            "1.1125390181559419487500439654662e-308",
        ],
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
            ("1.0000000000000000000000000000016", 720),
            ("1.9999999999999999999999999999984", 720),
            ("0.99999999999999999999999999999921", 690),
            ("2.0000000000000000000000000000032", 678),
            ("3.9999999999999999999999999999968", 645),
            ("0.50000000000000000000000000000079", 624),
            ("0.24999999999999999999999999999980", 324),
            ("0.12500000000000000000000000000020", 321),
            ("15.999999999999999999999999999987", 315),
            ("7.9999999999999999999999999999937", 313),
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
