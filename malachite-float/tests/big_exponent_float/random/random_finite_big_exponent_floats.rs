// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::random::EXAMPLE_SEED;
use malachite_float::big_exponent_float::random::random_finite_big_exponent_floats;
use malachite_float::test_util::big_exponent_float::random_big_exponent_floats_helper_helper;

#[allow(clippy::too_many_arguments)]
fn random_finite_big_exponent_floats_helper(
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
        random_finite_big_exponent_floats(
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
fn test_random_finite_big_exponent_floats() {
    // mean exponent bits = 1, mean precision = 2, P(0) = 1/10
    random_finite_big_exponent_floats_helper(
        1,
        1,
        2,
        1,
        1,
        10,
        &[
            "0.75", "0.00012", "0.12", "0.0", "-1.12", "-0.50", "-1.5", "-1.0", "0.62", "0.50",
            "-0.75", "-0.6758", "-5.0", "0.0", "-0.594", "-0.0078", "-1.5", "0.0", "48.0", "-0.50",
        ],
        &[
            "0x0.c", "0x0.0008", "0x0.2", "0x0.0", "-0x1.2", "-0x0.8", "-0x1.8", "-0x1.0", "0x0.a",
            "0x0.8", "-0x0.c", "-0x0.ad", "-0x5.0", "0x0.0", "-0x0.98", "-0x0.02", "-0x1.8",
            "0x0.0", "0x3.0E+1", "-0x0.8",
        ],
        &[
            ("0.0", 100224),
            ("1.0", 100168),
            ("-1.0", 99470),
            ("2.0", 50226),
            ("-2.0", 50159),
            ("-0.50", 49959),
            ("0.50", 49868),
            ("1.5", 25116),
            ("-1.5", 24791),
            ("0.25", 12662),
        ],
        &[
            ("0x0.0", 100224),
            ("0x1.0", 100168),
            ("-0x1.0", 99470),
            ("0x2.0", 50226),
            ("-0x2.0", 50159),
            ("-0x0.8", 49959),
            ("0x0.8", 49868),
            ("0x1.8", 25116),
            ("-0x1.8", 24791),
            ("0x0.4", 12662),
        ],
    );
    // mean exponent bits = 4, mean precision = 4, P(0) = 1/10
    random_finite_big_exponent_floats_helper(
        4,
        1,
        4,
        1,
        1,
        10,
        &[
            "0.015312506",
            "8.0",
            "8.0",
            "0.0",
            "-2.5",
            "-1.70e-28",
            "-0.00005984",
            "-3.6720e353",
            "0.38",
            "2.49e-13",
            "-3.55e133",
            "-0.01331",
            "-1.2e-11195",
            "0.0",
            "-0.04138",
            "-0.0039",
            "-0.88",
            "0.0",
            "0.0144",
            "-0.25",
        ],
        &[
            "0x0.03eb8538",
            "0x8.0",
            "0x8.0",
            "0x0.0",
            "-0x2.8",
            "-0xd.8E-24",
            "-0x0.0003ec",
            "-0x5.b98E+293",
            "0x0.6",
            "0x4.6E-11",
            "-0xc.8E+110",
            "-0x0.0368",
            "-0xa.0E-9298",
            "0x0.0",
            "-0x0.0a98",
            "-0x0.01",
            "-0x0.e",
            "0x0.0",
            "0x0.03b",
            "-0x0.4",
        ],
        &[
            ("0.0", 100224),
            ("1.0", 19969),
            ("-1.0", 19801),
            ("-2.0", 16306),
            ("0.50", 16111),
            ("-0.50", 15982),
            ("2.0", 15895),
            ("1.5", 7586),
            ("-1.5", 7393),
            ("-4.0", 6473),
        ],
        &[
            ("0x0.0", 100224),
            ("0x1.0", 19969),
            ("-0x1.0", 19801),
            ("-0x2.0", 16306),
            ("0x0.8", 16111),
            ("-0x0.8", 15982),
            ("0x2.0", 15895),
            ("0x1.8", 7586),
            ("-0x1.8", 7393),
            ("-0x4.0", 6473),
        ],
    );
    // mean exponent bits = 16, mean precision = 64, P(0) = 1/2
    random_finite_big_exponent_floats_helper(
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
            "3.635466064664447487021950059587753040502154212371e-306",
            "0.0",
            "2.59721294953017214e42040492545",
            "0.0",
            "0.0",
            "0.0",
            "11.0",
            "-3579779918061.33894337508670218296688817260237",
            "-1.7808474983e-28",
            "0.0",
            "0.0",
            "-2.0760188e-8070",
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
            "0x2.8d8b94c28e52f82d1e50104590476568ffae14eE-254",
            "0x0.0",
            "0x2.dbbbac5bdf8b04E+34913873327",
            "0x0.0",
            "0x0.0",
            "0x0.0",
            "0xb.0",
            "-0x3417b82a8ed.56c4fe373178a356492ec07bec",
            "-0xe.1bfce318E-24",
            "0x0.0",
            "0x0.0",
            "-0x2.22819E-6702",
            "0x0.0",
            "0x0.0",
            "0x0.0",
            "0x0.0",
            "0x0.0",
        ],
        &[
            ("0.0", 499546),
            ("-1.0", 246),
            ("-0.50", 238),
            ("0.50", 237),
            ("1.0", 213),
            ("2.0", 210),
            ("-2.0", 193),
            ("8.0", 118),
            ("-3.0", 109),
            ("0.75", 107),
        ],
        &[
            ("0x0.0", 499546),
            ("-0x1.0", 246),
            ("-0x0.8", 238),
            ("0x0.8", 237),
            ("0x1.0", 213),
            ("0x2.0", 210),
            ("-0x2.0", 193),
            ("0x8.0", 118),
            ("-0x3.0", 109),
            ("0x0.c", 107),
        ],
    );
}

#[test]
#[should_panic]
fn random_finite_big_exponent_floats_fail_1() {
    random_finite_big_exponent_floats(EXAMPLE_SEED, 1, 1, 0, 1, 1, 10);
}

#[test]
#[should_panic]
fn random_finite_big_exponent_floats_fail_2() {
    random_finite_big_exponent_floats(EXAMPLE_SEED, 1, 1, 1, 0, 1, 10);
}

#[test]
#[should_panic]
fn random_finite_big_exponent_floats_fail_3() {
    random_finite_big_exponent_floats(EXAMPLE_SEED, 1, 0, 2, 1, 1, 10);
}

#[test]
#[should_panic]
fn random_finite_big_exponent_floats_fail_4() {
    random_finite_big_exponent_floats(EXAMPLE_SEED, 1, 1, 2, 1, 11, 10);
}
