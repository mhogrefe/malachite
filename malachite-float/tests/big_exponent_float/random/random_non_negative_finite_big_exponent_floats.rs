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
    expected_values_hex: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_common_values_hex: &[(&str, usize)],
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
        expected_values_hex,
        expected_common_values,
        expected_common_values_hex,
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
            "0.25", "1.0", "12.0", "0.0", "2.0", "1.0", "0.50", "1.0", "1.5", "1.9072", "0.50",
            "0.50", "8.0", "0.0", "0.25", "1.0", "0.50", "0.0", "1.5", "1.0",
        ],
        &[
            "0x0.4", "0x1.0", "0xc.0", "0x0.0", "0x2.0", "0x1.0", "0x0.8", "0x1.0", "0x1.8",
            "0x1.e84", "0x0.8", "0x0.8", "0x8.0", "0x0.0", "0x0.4", "0x1.0", "0x0.8", "0x0.0",
            "0x1.8", "0x1.0",
        ],
        &[
            ("1.0", 200066),
            ("0.0", 100224),
            ("0.50", 99941),
            ("2.0", 99912),
            ("1.5", 49994),
            ("0.75", 25261),
            ("8.0", 25083),
            ("3.0", 24981),
            ("4.0", 24951),
            ("0.25", 24793),
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
            "0.016",
            "0.000549",
            "2.723e-305",
            "0.0",
            "1.4e11",
            "2.0",
            "1.3e2",
            "0.5820",
            "4.9e-147644613",
            "2.0",
            "1.5",
            "0.25",
            "0.000916",
            "0.0",
            "0.055",
            "7.0747e40",
            "0.062",
            "0.0",
            "48.0",
            "5.2e-154",
        ],
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
            ("0.0", 100224),
            ("1.0", 40019),
            ("0.50", 32219),
            ("2.0", 32098),
            ("1.5", 14975),
            ("0.12", 12841),
            ("0.25", 12815),
            ("8.0", 12809),
            ("4.0", 12668),
            ("3.0", 12020),
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
            "0.0",
            "0.0",
            "0.0",
            "2.56703535624353745445838131957693624532266956980757840240303884313649283936058928958423859480573702e-81991452301",
            "0.0",
            "5.3e-110",
            "0.0",
            "0.0",
            "0.0",
            "1.6013125860e-35",
            "2.33766899904786363655655276e1394695331",
            "19.065662855243196142595688945840543083",
            "0.0",
            "0.0",
            "1.977589338e502044",
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
            ("0.0", 499546),
            ("1.0", 470),
            ("0.50", 443),
            ("2.0", 429),
            ("1.5", 229),
            ("0.25", 219),
            ("8.0", 209),
            ("0.12", 206),
            ("4.0", 204),
            ("3.0", 200),
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
