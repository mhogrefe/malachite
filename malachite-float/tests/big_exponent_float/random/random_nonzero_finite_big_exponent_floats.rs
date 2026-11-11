// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::random::EXAMPLE_SEED;
use malachite_float::big_exponent_float::random::random_nonzero_finite_big_exponent_floats;
use malachite_float::test_util::big_exponent_float::random_big_exponent_floats_helper_helper;

#[allow(clippy::too_many_arguments)]
fn random_nonzero_finite_big_exponent_floats_helper(
    mean_exponent_bits_numerator: u64,
    mean_exponent_bits_denominator: u64,
    mean_precision_numerator: u64,
    mean_precision_denominator: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
) {
    random_big_exponent_floats_helper_helper(
        random_nonzero_finite_big_exponent_floats(
            EXAMPLE_SEED,
            mean_exponent_bits_numerator,
            mean_exponent_bits_denominator,
            mean_precision_numerator,
            mean_precision_denominator,
        ),
        expected_values,
        expected_common_values,
    );
}

#[test]
fn test_random_nonzero_finite_big_exponent_floats() {
    // mean exponent bits = 1, mean precision = 2
    random_nonzero_finite_big_exponent_floats_helper(
        1,
        1,
        2,
        1,
        &[
            "-0x0.4",
            "-0x1.0",
            "-0xc.0",
            "0x2.0",
            "-0x1.0",
            "0x0.8",
            "-0x1.0",
            "-0x1.8",
            "-0x1.e84",
            "0x0.8",
            "0x0.8",
            "0x8.0",
            "-0x0.4",
            "-0x1.0",
            "0x0.8",
            "-0x1.8",
            "-0x1.0",
            "-0x2.0",
            "-0xc.0",
            "-0x2.0E+3",
        ],
        &[
            ("0x1.0", 111282),
            ("-0x1.0", 110907),
            ("0x2.0", 55898),
            ("-0x0.8", 55773),
            ("-0x2.0", 55358),
            ("0x0.8", 55219),
            ("0x1.8", 27845),
            ("-0x1.8", 27726),
            ("0x0.c", 14169),
            ("0x8.0", 14133),
        ],
    );
    // mean exponent bits = 4, mean precision = 4
    random_nonzero_finite_big_exponent_floats_helper(
        4,
        1,
        4,
        1,
        &[
            "-0x0.04",
            "-0x0.0024",
            "-0x1.32E-253",
            "0x2.0E+9",
            "-0x2.0",
            "0x8.0E+1",
            "-0x0.95",
            "-0x5.0E-122616197",
            "-0x2.0",
            "0x1.8",
            "0x0.4",
            "0x0.003c",
            "-0x0.0e",
            "-0xc.fe8E+33",
            "0x0.1",
            "-0x3.0E+1",
            "-0x7.0E-128",
            "-0x0.8",
            "-0xe.0E-60",
            "-0x0.8",
        ],
        &[
            ("0x1.0", 22316),
            ("-0x1.0", 22097),
            ("0x0.8", 17945),
            ("0x2.0", 17840),
            ("-0x0.8", 17829),
            ("-0x2.0", 17770),
            ("0x1.8", 8366),
            ("-0x1.8", 8302),
            ("-0x0.4", 7173),
            ("0x0.2", 7165),
        ],
    );
    // mean exponent bits = 16, mean precision = 64
    random_nonzero_finite_big_exponent_floats_helper(
        16,
        1,
        64,
        1,
        &[
            "-0x4.24f15fca75f9865c038bd55844fff40d21205eaa8642524cd23a9f1837937e9c89911612c6c481\
            9d8E-68092427235",
            "-0x2.0E-91",
            "-0x1.548fbffaE-29",
            "0x2.1e04e45037aa8304b9742E+1158269401",
            "-0x13.10cf47e7d4ddcf7a0d6eca9afb143",
            "0x8.4b7a03E+416938",
            "-0x2.671785a18c13ffbbe8b27f4E+49",
            "-0x4.369ba55e29d85E+55507332",
            "-0x1.12cE-6482",
            "0x1.b5fcE-28081789395313",
            "0xf.6cde7f20e755ec7E+130181906",
            "0xd.56b73d18E-236",
            "-0x2.0E-10",
            "-0x1.4E-6",
            "0x2.a1d1ece7aE+34345825487",
            "-0x4.fcE-1280",
            "-0x0.9",
            "-0x9.f73d47eec00b12e51e0814E-27676",
            "-0x2.894441dcE+7003186721546503926808",
            "-0x8.024E-4537743746",
        ],
        &[
            ("0x1.0", 447),
            ("-0x1.0", 441),
            ("0x2.0", 438),
            ("-0x0.8", 432),
            ("-0x2.0", 431),
            ("0x0.8", 429),
            ("-0x1.8", 236),
            ("-0x0.4", 226),
            ("0x0.2", 219),
            ("-0x3.0", 218),
        ],
    );
}

#[test]
#[should_panic]
fn random_nonzero_finite_big_exponent_floats_fail_1() {
    random_nonzero_finite_big_exponent_floats(EXAMPLE_SEED, 1, 1, 0, 1);
}

#[test]
#[should_panic]
fn random_nonzero_finite_big_exponent_floats_fail_2() {
    random_nonzero_finite_big_exponent_floats(EXAMPLE_SEED, 1, 1, 1, 0);
}

#[test]
#[should_panic]
fn random_nonzero_finite_big_exponent_floats_fail_3() {
    random_nonzero_finite_big_exponent_floats(EXAMPLE_SEED, 1, 0, 2, 1);
}
