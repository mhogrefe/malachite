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

fn random_positive_big_exponent_floats_with_precision_helper(
    mean_exponent_bits_numerator: u64,
    mean_exponent_bits_denominator: u64,
    prec: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
) {
    let xs = random_positive_big_exponent_floats_with_precision(
        EXAMPLE_SEED,
        mean_exponent_bits_numerator,
        mean_exponent_bits_denominator,
        prec,
    );
    for x in xs.clone().take(10000) {
        assert_eq!(x.get_prec(), Some(prec));
        assert!(x.is_sign_positive());
    }
    random_big_exponent_floats_helper_helper(xs, expected_values, expected_common_values);
}

#[test]
fn test_random_positive_big_exponent_floats_with_precision() {
    // mean exponent bits = 1, precision = 1
    random_positive_big_exponent_floats_with_precision_helper(
        1,
        1,
        1,
        &[
            "0x1.0",
            "0x2.0",
            "0x8.0",
            "0x1.0",
            "0x1.0",
            "0x0.0008",
            "0x4.0",
            "0x1.0",
            "0x1.0",
            "0x1.0",
            "0x1.0",
            "0x1.0",
            "0x1.0",
            "0x2.0",
            "0x4.0E+27",
            "0x8.0E-6",
            "0x2.0E+1",
            "0x8.0E+1",
            "0x0.8",
            "0x0.8",
        ],
        &[
            ("0x1.0", 333732),
            ("0x0.8", 166936),
            ("0x2.0", 166566),
            ("0x0.2", 41707),
            ("0x8.0", 41569),
            ("0x0.4", 41510),
            ("0x4.0", 41127),
            ("0x8.0E+1", 10447),
            ("0x0.08", 10440),
            ("0x1.0E+1", 10421),
        ],
    );
    // mean exponent bits = 4, precision = 10
    random_positive_big_exponent_floats_with_precision_helper(
        4,
        1,
        10,
        &[
            "0x0.f5c",
            "0x0.3f7",
            "0xd.3cE-16",
            "0x5.aeE-7",
            "0x1.8e8",
            "0x0.60e",
            "0x5.02E+27",
            "0xc.54",
            "0x0.000e7c",
            "0xea.c",
            "0xc.6cE+18",
            "0x2.9fE+12",
            "0x0.e4c",
            "0xb.74",
            "0x3.e3",
            "0x3.bb",
            "0x0.6ce",
            "0xe.f4E+10",
            "0x0.00022d",
            "0x2.59E+6",
        ],
        &[
            ("0x1.528", 494),
            ("0x1.6d8", 491),
            ("0x1.878", 484),
            ("0x1.ca8", 482),
            ("0x1.bf8", 481),
            ("0x1.668", 477),
            ("0x1.b18", 477),
            ("0x1.ba8", 475),
            ("0x1.0f8", 474),
            ("0x1.808", 474),
        ],
    );
    // mean exponent bits = 16, precision = 100
    random_positive_big_exponent_floats_with_precision_helper(
        16,
        1,
        100,
        &[
            "0x0.0290c2287db7afb6a873dc7b75c",
            "0xa.f81eb6d7c73be5607e7c934fE-13",
            "0xa.eed43906b1290314433e911dE-80",
            "0x6.df0cb78aef3fb3e2f3e28b408E+2370",
            "0x1.d36fbcb56dd4f1f9e198a0f3eE+100",
            "0x7.5424304f4508a8b44be65b8d8E+443",
            "0xd.9af5ccdc5e1e57fad3242793E-2446867802",
            "0xb.8efbc5ba1f3caf4b471d67e3E-20407738",
            "0x0.b9633a7bdf59f7ffb12fb7167",
            "0x2.77581d9620aeb93c0344cf8b4E-7",
            "0x8.088185366f51d7dbc9c73cadE-4588845",
            "0x6.be5230d917d7be141b4990108E+7688",
            "0x2.b3ec01d502f43e0add6ebd01cE-101292979269316240387212",
            "0x2.96202cd2556b64f0377d2010c",
            "0x1.c0bb37961d18237d3e8233116E+716346",
            "0x0.00269a9cf90387e19cca2a7669a4",
            "0xa.9c77a2f311f698b4f7f92fddE+65001713",
            "0x3.58e9be942f633cf45c44dd594E+103924662",
            "0xf.50d9baff3ed9313e23ee29bfE+6299228510842475",
            "0x1.1437d8def0930e1b53f4991caE-140",
        ],
        &[
            ("0x8.005fa58db32836780c8bbeb1", 1),
            ("0x8.006336c5d6bbfe75ad65e451", 1),
            ("0x8.006b076704427f656c383553", 1),
            ("0x8.0070f0713d2e9493a77153b5", 1),
            ("0x8.008f92022ad471c3759a0775", 1),
            ("0x8.00965c1b83634b1b76e13b15", 1),
            ("0x8.00aa3464a9483fbfff311009", 1),
            ("0x8.00b29b5f94d7acd60a3c00d9", 1),
            ("0x8.00be0bac9509136d36e3ffe3", 1),
            ("0x8.00dbfda40af8232a788405a5", 1),
        ],
    );
}

#[test]
#[should_panic]
fn random_positive_big_exponent_floats_with_precision_fail_1() {
    random_positive_big_exponent_floats_with_precision(EXAMPLE_SEED, 0, 1, 0);
}

#[test]
#[should_panic]
fn random_positive_big_exponent_floats_with_precision_fail_2() {
    random_positive_big_exponent_floats_with_precision(EXAMPLE_SEED, 1, 0, 10);
}
