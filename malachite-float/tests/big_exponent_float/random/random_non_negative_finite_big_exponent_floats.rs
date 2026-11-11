// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::random::EXAMPLE_SEED;
use malachite_float::big_exponent_float::random::random_non_negative_finite_big_exponent_floats;
use malachite_float::test_util::big_exponent_float::random_big_exponent_floats_helper_helper;

#[allow(clippy::too_many_arguments)]
fn random_non_negative_finite_big_exponent_floats_helper(
    mean_exponent_bits_numerator: u64,
    mean_exponent_bits_denominator: u64,
    mean_precision_numerator: u64,
    mean_precision_denominator: u64,
    zero_p_numerator: u64,
    zero_p_denominator: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
) {
    random_big_exponent_floats_helper_helper(
        random_non_negative_finite_big_exponent_floats(
            EXAMPLE_SEED,
            mean_exponent_bits_numerator,
            mean_exponent_bits_denominator,
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
fn test_random_non_negative_finite_big_exponent_floats() {
    // mean exponent bits = 1, mean precision = 2, P(0) = 1/10
    random_non_negative_finite_big_exponent_floats_helper(
        1,
        1,
        2,
        1,
        1,
        10,
        &[
            "0x0.4", "0x1.0", "0xc.0", "0x0.0", "0x2.0", "0x1.0", "0x0.8", "0x1.0", "0x1.8",
            "0x1.e84", "0x0.8", "0x0.8", "0x8.0", "0x0.0", "0x0.4", "0x1.0", "0x0.8", "0x0.0",
            "0x1.8", "0x1.0",
        ],
        &[
            ("0x1.0", 200066),
            ("0x0.0", 100224),
            ("0x0.8", 99941),
            ("0x2.0", 99912),
            ("0x1.8", 49994),
            ("0x0.c", 25261),
            ("0x8.0", 25083),
            ("0x3.0", 24981),
            ("0x4.0", 24951),
            ("0x0.4", 24793),
        ],
    );
    // mean exponent bits = 4, mean precision = 4, P(0) = 1/10
    random_non_negative_finite_big_exponent_floats_helper(
        4,
        1,
        4,
        1,
        1,
        10,
        &[
            "0x0.04",
            "0x0.0024",
            "0x1.32E-253",
            "0x0.0",
            "0x2.0E+9",
            "0x2.0",
            "0x8.0E+1",
            "0x0.95",
            "0x5.0E-122616197",
            "0x2.0",
            "0x1.8",
            "0x0.4",
            "0x0.003c",
            "0x0.0",
            "0x0.0e",
            "0xc.fe8E+33",
            "0x0.1",
            "0x0.0",
            "0x3.0E+1",
            "0x7.0E-128",
        ],
        &[
            ("0x0.0", 100224),
            ("0x1.0", 40019),
            ("0x0.8", 32219),
            ("0x2.0", 32098),
            ("0x1.8", 14975),
            ("0x0.2", 12841),
            ("0x0.4", 12815),
            ("0x8.0", 12809),
            ("0x4.0", 12668),
            ("0x3.0", 12020),
        ],
    );
    // mean exponent bits = 16, mean precision = 64, P(0) = 1/2
    random_non_negative_finite_big_exponent_floats_helper(
        16,
        1,
        64,
        1,
        1,
        2,
        &[
            "0x0.0",
            "0x0.0",
            "0x0.0",
            "0x4.24f15fca75f9865c038bd55844fff40d21205eaa8642524cd23a9f1837937e9c89911612c6c4819\
            d8E-68092427235",
            "0x0.0",
            "0x2.0E-91",
            "0x0.0",
            "0x0.0",
            "0x0.0",
            "0x1.548fbffaE-29",
            "0x2.1e04e45037aa8304b9742E+1158269401",
            "0x13.10cf47e7d4ddcf7a0d6eca9afb143",
            "0x0.0",
            "0x0.0",
            "0x8.4b7a03E+416938",
            "0x0.0",
            "0x0.0",
            "0x0.0",
            "0x0.0",
            "0x0.0",
        ],
        &[
            ("0x0.0", 499546),
            ("0x1.0", 470),
            ("0x0.8", 443),
            ("0x2.0", 429),
            ("0x1.8", 229),
            ("0x0.4", 219),
            ("0x8.0", 209),
            ("0x0.2", 206),
            ("0x4.0", 204),
            ("0x3.0", 200),
        ],
    );
}

#[test]
#[should_panic]
fn random_non_negative_finite_big_exponent_floats_fail_1() {
    random_non_negative_finite_big_exponent_floats(EXAMPLE_SEED, 1, 1, 0, 1, 1, 10);
}

#[test]
#[should_panic]
fn random_non_negative_finite_big_exponent_floats_fail_2() {
    random_non_negative_finite_big_exponent_floats(EXAMPLE_SEED, 1, 1, 1, 0, 1, 10);
}

#[test]
#[should_panic]
fn random_non_negative_finite_big_exponent_floats_fail_3() {
    random_non_negative_finite_big_exponent_floats(EXAMPLE_SEED, 1, 0, 2, 1, 1, 10);
}

#[test]
#[should_panic]
fn random_non_negative_finite_big_exponent_floats_fail_4() {
    random_non_negative_finite_big_exponent_floats(EXAMPLE_SEED, 1, 1, 2, 1, 11, 10);
}
