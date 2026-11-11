// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::random::EXAMPLE_SEED;
use malachite_float::big_exponent_float::random::random_negative_finite_big_exponent_floats;
use malachite_float::test_util::big_exponent_float::random_big_exponent_floats_helper_helper;

#[allow(clippy::too_many_arguments)]
fn random_negative_finite_big_exponent_floats_helper(
    mean_exponent_bits_numerator: u64,
    mean_exponent_bits_denominator: u64,
    mean_precision_numerator: u64,
    mean_precision_denominator: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
) {
    random_big_exponent_floats_helper_helper(
        random_negative_finite_big_exponent_floats(
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
fn test_random_negative_finite_big_exponent_floats() {
    // mean exponent bits = 1, mean precision = 2
    random_negative_finite_big_exponent_floats_helper(
        1,
        1,
        2,
        1,
        &[
            "-0x1.8",
            "-0x2.0",
            "-0x8.0",
            "-0x1.0",
            "-0x1.0",
            "-0x0.0008",
            "-0x4.0",
            "-0x1.4",
            "-0x1.0",
            "-0x1.0",
            "-0x1.0",
            "-0x1.0",
            "-0x1.0",
            "-0x2.0",
            "-0x7.0E+27",
            "-0xa.0E-6",
            "-0x2.0E+1",
            "-0x8.0E+1",
            "-0x0.8",
            "-0x0.a",
        ],
        &[
            ("-0x1.0", 221963),
            ("-0x0.8", 111272),
            ("-0x2.0", 110855),
            ("-0x1.8", 55874),
            ("-0x0.c", 27904),
            ("-0x0.4", 27826),
            ("-0x8.0", 27793),
            ("-0x3.0", 27689),
            ("-0x0.2", 27671),
            ("-0x4.0", 27324),
        ],
    );
    // mean exponent bits = 4, mean precision = 4
    random_negative_finite_big_exponent_floats_helper(
        4,
        1,
        4,
        1,
        &[
            "-0x0.b",
            "-0x0.2",
            "-0x8.0E-16",
            "-0x4.0E-7",
            "-0x1.0",
            "-0x0.6",
            "-0x6.7aE+27",
            "-0x8.0",
            "-0x0.0008",
            "-0xc.0E+1",
            "-0x8.0E+18",
            "-0x2.0E+12",
            "-0x0.8",
            "-0x9.8",
            "-0x2.0",
            "-0x2.d",
            "-0x0.73",
            "-0xa.8E+10",
            "-0x0.0003c",
            "-0x2.c21E+6",
        ],
        &[
            ("-0x1.0", 44438),
            ("-0x2.0", 35835),
            ("-0x0.8", 35411),
            ("-0x1.8", 16905),
            ("-0x0.4", 14435),
            ("-0x8.0", 14307),
            ("-0x4.0", 14114),
            ("-0x0.2", 13986),
            ("-0x3.0", 13300),
            ("-0x0.c", 13244),
        ],
    );
    // mean exponent bits = 16, mean precision = 64
    random_negative_finite_big_exponent_floats_helper(
        16,
        1,
        64,
        1,
        &[
            "-0x0.038",
            "-0xb.147e9ec26d848ea18E-13",
            "-0xc.d8d83f518E-80",
            "-0x7.ec8ecd34fce2bdc940eE+2370",
            "-0x1.aa4f4E+100",
            "-0x6.1872cf98e1f0889921bed82450742bf48c6fb4E+443",
            "-0xf.77426a4101300bb8f1c57e85efe13050cd5351eb6e2bfcfE-2446867802",
            "-0x9.1b62bE-20407738",
            "-0x0.c05919b67ea66b2470b7ed",
            "-0x2.a0c83e0f08002aE-7",
            "-0x9.2e92de7585eE-4588845",
            "-0x6.aea0c21E+7688",
            "-0x3.36d3f80ee82298E-101292979269316240387212",
            "-0x2.1c85dd285c44",
            "-0x1.176f4364d55e3feb51571e25ef39e528b86aE+716346",
            "-0x0.002c01b9bdd036c4",
            "-0xe.3428fc4E+65001713",
            "-0x2.500e57ce9d7a4846916b0a553c9478c5cE+103924662",
            "-0xc.abb9372bceE+6299228510842475",
            "-0x1.5983dE-140",
        ],
        &[
            ("-0x1.0", 966),
            ("-0x0.8", 889),
            ("-0x2.0", 869),
            ("-0x1.8", 447),
            ("-0x0.c", 434),
            ("-0x8.0", 430),
            ("-0x4.0", 422),
            ("-0x0.4", 416),
            ("-0x3.0", 406),
            ("-0x0.2", 385),
        ],
    );
}

#[test]
#[should_panic]
fn random_negative_finite_big_exponent_floats_fail_1() {
    random_negative_finite_big_exponent_floats(EXAMPLE_SEED, 1, 1, 0, 1);
}

#[test]
#[should_panic]
fn random_negative_finite_big_exponent_floats_fail_2() {
    random_negative_finite_big_exponent_floats(EXAMPLE_SEED, 1, 1, 1, 0);
}

#[test]
#[should_panic]
fn random_negative_finite_big_exponent_floats_fail_3() {
    random_negative_finite_big_exponent_floats(EXAMPLE_SEED, 1, 0, 2, 1);
}
