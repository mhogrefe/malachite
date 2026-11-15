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
    expected_values_hex: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_common_values_hex: &[(&str, usize)],
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
        expected_values_hex,
        expected_common_values,
        expected_common_values_hex,
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
            "0.25", "1.0", "6.0", "0.0", "2.0", "1.2", "0.50", "1.0", "1.0", "1.4365", "0.50",
            "0.50", "8.0", "0.0", "0.12", "1.5", "0.50", "0.0", "1.0", "1.0",
        ],
        &[
            "0x0.4", "0x1.0", "0x6.0", "0x0.0", "0x2.0", "0x1.4", "0x0.8", "0x1.0", "0x1.0",
            "0x1.6fc", "0x0.8", "0x0.8", "0x8.0", "0x0.0", "0x0.2", "0x1.8", "0x0.8", "0x0.0",
            "0x1.0", "0x1.0",
        ],
        &[
            ("1.0", 200046),
            ("0.0", 100224),
            ("0.50", 100076),
            ("2.0", 99876),
            ("1.5", 49693),
            ("4.0", 25033),
            ("3.0", 25013),
            ("0.25", 25011),
            ("0.75", 24991),
            ("8.0", 24989),
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
            "0.016",
            "0.0039",
            "4.301e-222",
            "0.0",
            "9.2e18",
            "3.5",
            "240.0",
            "0.9961",
            "1.7e-131279756",
            "2.0",
            "1.0",
            "0.25",
            "0.00732",
            "0.0",
            "0.027",
            "1.35276e40",
            "0.031",
            "0.0",
            "16.0",
            "9.52e-116",
        ],
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
            ("0.0", 100224),
            ("1.0", 46334),
            ("0.50", 37298),
            ("2.0", 37267),
            ("0.25", 14991),
            ("0.12", 14943),
            ("8.0", 14901),
            ("4.0", 14763),
            ("1.5", 13539),
            ("0.75", 10632),
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
            "0.0",
            "0.0",
            "0.0",
            "2.1232041519637758527835602377410497335936532709596052940737115303832552351072956983355555408e-82743970067",
            "0.0",
            "8.6e-78",
            "0.0",
            "0.0",
            "0.0",
            "1.0841980368e-19",
            "1.30249445797802352701721563e1616142483",
            "31.9921875000000004440892098500500436980",
            "0.0",
            "0.0",
            "4.544296935e631305",
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
            ("0.0", 499546),
            ("1.0", 2764),
            ("0.50", 2671),
            ("2.0", 2557),
            ("4.0", 1222),
            ("0.12", 1210),
            ("8.0", 1208),
            ("0.25", 1193),
            ("0.0078", 1165),
            ("1.3e2", 1132),
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
