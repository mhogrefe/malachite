// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::float::NiceFloat;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::moments::MomentStats;
use malachite_float::random::random_floats;
use malachite_float::test_util::random::random_floats_helper_helper;

fn random_floats_helper(
    mean_sci_exponent_abs_numerator: u64,
    mean_sci_exponent_abs_denominator: u64,
    mean_precision_numerator: u64,
    mean_precision_denominator: u64,
    mean_zero_p_numerator: u64,
    mean_zero_p_denominator: u64,
    expected_values: &[&str],
    expected_values_hex: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_common_values_hex: &[(&str, usize)],
    expected_median: (&str, Option<&str>),
    expected_median_hex: (&str, Option<&str>),
    expected_moment_stats: MomentStats,
) {
    random_floats_helper_helper(
        random_floats(
            EXAMPLE_SEED,
            mean_sci_exponent_abs_numerator,
            mean_sci_exponent_abs_denominator,
            mean_precision_numerator,
            mean_precision_denominator,
            mean_zero_p_numerator,
            mean_zero_p_denominator,
        ),
        expected_values,
        expected_values_hex,
        expected_common_values,
        expected_common_values_hex,
        expected_median,
        expected_median_hex,
        expected_moment_stats,
    );
}

#[test]
fn test_random_floats() {
    // mean |sci_exponent| 1, mean precision 2, zero probability 1/10
    let values = &[
        "1.0", "0.2", "0.0", "NaN", "-0.2", "-1.0", "-1.0", "-1.5", "2.0", "0.2", "-1.0", "-0.2",
        "-2.0", "Infinity", "-0.5", "-1.0", "-0.4", "NaN", "0.5", "-0.0",
    ];
    let values_hex = &[
        "0x1.0#1", "0x0.4#1", "0x0.0", "NaN", "-0x0.4#1", "-0x1.0#2", "-0x1.0#1", "-0x1.8#2",
        "0x2.0#1", "0x0.4#1", "-0x1.0#3", "-0x0.4#2", "-0x2.0#1", "Infinity", "-0x0.8#1",
        "-0x1.0#2", "-0x0.6#2", "NaN", "0x0.8#1", "-0x0.0",
    ];
    let common_values = &[
        ("-1.0", 67626),
        ("1.0", 67293),
        ("0.0", 45033),
        ("-0.0", 44760),
        ("-2.0", 33918),
        ("2.0", 33760),
        ("0.5", 33658),
        ("Infinity", 33640),
        ("-0.5", 33629),
        ("NaN", 33393),
    ];
    let common_values_hex = &[
        ("-0x1.0#1", 67626),
        ("0x1.0#1", 67293),
        ("0x0.0", 45033),
        ("-0x0.0", 44760),
        ("-0x2.0#1", 33918),
        ("0x2.0#1", 33760),
        ("0x0.8#1", 33658),
        ("Infinity", 33640),
        ("-0x0.8#1", 33629),
        ("NaN", 33393),
    ];
    let sample_median = ("NaN", None);
    let sample_median_hex = ("NaN", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(f64::NAN),
        standard_deviation: NiceFloat(f64::NAN),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_floats_helper(
        1,
        1,
        2,
        1,
        1,
        10,
        values,
        values_hex,
        common_values,
        common_values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );

    // mean |sci_exponent| 1, mean precision 64, zero probability 1/10
    let values = &[
        "1.60469",
        "0.3689021918822",
        "0.0",
        "NaN",
        "-0.28085087121796634898241172643",
        "-1.753651001",
        "-1.76635021435647835233096910201935186533898089",
        "-1.43252385721349903405196303338780402302813416810241256872117258",
        "2.0",
        "0.250602502656981020491543289450999271",
        "-1.8960717298288877935",
        "-0.25858",
        "-2.6358228847359354842130018065396119276102632191023",
        "Infinity",
        "-0.9541023510530255",
        "-1.5",
        "-0.464323",
        "NaN",
        "0.725643469742899",
        "-0.0",
    ];
    let values_hex = &[
        "0x1.9acd0#18",
        "0x0.5e705fc18e8#40",
        "0x0.0",
        "NaN",
        "-0x0.47e5d7baef2c868cbb8316be#95",
        "-0x1.c0ef45a0#30",
        "-0x1.c42f8713f193fa44085ebdae1b4419f73e3d8#146",
        "-0x1.6eb9e22d78c4bde3d56b1e7932a0b849250a5abf6d02e887830#205",
        "0x2.00#7",
        "0x0.40277c51351dd7528f4d46d26a8156#118",
        "-0x1.e564f4f67c376d50#62",
        "-0x0.4232#14",
        "-0x2.a2c549dffd3d8e02dee8e02a95b00cb3a36c841e#162",
        "Infinity",
        "-0x0.f4400d3acf388#51",
        "-0x1.8#2",
        "-0x0.76dde0#20",
        "NaN",
        "0x0.b9c3c53b1a0a#48",
        "-0x0.0",
    ];
    let common_values = &[
        ("0.0", 45033),
        ("-0.0", 44760),
        ("Infinity", 33640),
        ("NaN", 33393),
        ("-Infinity", 33191),
        ("1.0", 2133),
        ("-1.0", 2122),
        ("-2.0", 1103),
        ("0.5", 1094),
        ("1.5", 1091),
    ];
    let common_values_hex = &[
        ("0x0.0", 45033),
        ("-0x0.0", 44760),
        ("Infinity", 33640),
        ("NaN", 33393),
        ("-Infinity", 33191),
        ("0x1.0#1", 2133),
        ("-0x1.0#1", 2122),
        ("-0x2.0#1", 1103),
        ("0x0.8#1", 1094),
        ("0x1.8#2", 1091),
    ];
    let sample_median = ("NaN", None);
    let sample_median_hex = ("NaN", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(f64::NAN),
        standard_deviation: NiceFloat(f64::NAN),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_floats_helper(
        1,
        1,
        64,
        1,
        1,
        10,
        values,
        values_hex,
        common_values,
        common_values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );

    // mean |sci_exponent| 64, mean precision 2, zero probability 1/10
    let values = &[
        "5.0e11", "7.0e122", "0.0", "NaN", "-2.0e25", "-2.0e21", "-2.0e-9", "-4.0e33", "4.0e-9",
        "5.0e2", "-4.0e-16", "-7.0e-18", "-2.0e-19", "Infinity", "-2.0e50", "-1.0e12", "-3.0e-28",
        "NaN", "0.02", "-0.0",
    ];
    let values_hex = &[
        "0x8.0E+9#1",
        "0x1.0E+102#1",
        "0x0.0",
        "NaN",
        "-0x1.0E+21#1",
        "-0x8.0E+17#2",
        "-0x8.0E-8#1",
        "-0xc.0E+27#2",
        "0x1.0E-7#1",
        "0x2.0E+2#1",
        "-0x2.0E-13#3",
        "-0x8.0E-15#2",
        "-0x4.0E-16#1",
        "Infinity",
        "-0x8.0E+41#1",
        "-0x1.0E+10#2",
        "-0x1.8E-23#2",
        "NaN",
        "0x0.04#1",
        "-0x0.0",
    ];
    let common_values = &[
        ("0.0", 45033),
        ("-0.0", 44760),
        ("Infinity", 33640),
        ("NaN", 33393),
        ("-Infinity", 33191),
        ("0.5", 1583),
        ("-1.0", 1560),
        ("2.0", 1542),
        ("-2.0", 1523),
        ("1.0", 1518),
    ];
    let common_values_hex = &[
        ("0x0.0", 45033),
        ("-0x0.0", 44760),
        ("Infinity", 33640),
        ("NaN", 33393),
        ("-Infinity", 33191),
        ("0x0.8#1", 1583),
        ("-0x1.0#1", 1560),
        ("0x2.0#1", 1542),
        ("-0x2.0#1", 1523),
        ("0x1.0#1", 1518),
    ];
    let sample_median = ("NaN", None);
    let sample_median_hex = ("NaN", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(f64::NAN),
        standard_deviation: NiceFloat(f64::NAN),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_floats_helper(
        64,
        1,
        2,
        1,
        1,
        10,
        values,
        values_hex,
        common_values,
        common_values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );

    // mean |sci_exponent| 64, mean precision 64, zero probability 1/10
    let values = &[
        "8.82188e11",
        "9.75459983374e122",
        "0.0",
        "NaN",
        "-21729783659306408649613509.686",
        "-4.140691354e21",
        "-3.29008365861415556134836580980448399733562188e-9",
        "-3719044561792922503530448846362960.3599330496921301151502834994",
        "3.73e-9",
        "513.233925441497129966680656795646506",
        "-8.420249963187521806e-16",
        "-7.177e-18",
        "-2.8577648979177105962332201291018926848163080599637e-19",
        "Infinity",
        "-3.569720699507868e50",
        "-1.6e12",
        "-3.750772e-28",
        "NaN",
        "0.0226763584294656",
        "-0.0",
    ];
    let values_hex = &[
        "0xc.d668E+9#18",
        "0x1.79c17f063aE+102#40",
        "0x0.0",
        "NaN",
        "-0x11f975eebbcb21a32ee0c5.af8#95",
        "-0xe.077a2d0E+17#30",
        "-0xe.217c389f8c9fd22042f5ed70da20cfb9f1ecE-8#146",
        "-0xb75cf116bc625ef1eab58f3c9950.5c2492852d5fb6817443c180#205",
        "0x1.00E-7#7",
        "0x201.3be289a8eeba947a6a3693540ab#118",
        "-0x3.cac9e9ecf86edaaE-13#62",
        "-0x8.464E-15#14",
        "-0x5.458a93bffa7b1c05bdd1c0552b60196746d9083cE-16#162",
        "Infinity",
        "-0xf.4400d3acf388E+41#51",
        "-0x1.8E+10#2",
        "-0x1.db778E-23#20",
        "NaN",
        "0x0.05ce1e29d8d050#48",
        "-0x0.0",
    ];
    let common_values = &[
        ("0.0", 45033),
        ("-0.0", 44760),
        ("Infinity", 33640),
        ("NaN", 33393),
        ("-Infinity", 33191),
        ("0.00003", 60),
        ("-0.5", 57),
        ("-6.0e1", 56),
        ("-5.0e2", 55),
        ("8.0", 54),
    ];
    let common_values_hex = &[
        ("0x0.0", 45033),
        ("-0x0.0", 44760),
        ("Infinity", 33640),
        ("NaN", 33393),
        ("-Infinity", 33191),
        ("0x0.0002#1", 60),
        ("-0x0.8#1", 57),
        ("-0x4.0E+1#1", 56),
        ("-0x2.0E+2#1", 55),
        ("0x8.0#1", 54),
    ];
    let sample_median = ("NaN", None);
    let sample_median_hex = ("NaN", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(f64::NAN),
        standard_deviation: NiceFloat(f64::NAN),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_floats_helper(
        64,
        1,
        64,
        1,
        1,
        10,
        values,
        values_hex,
        common_values,
        common_values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );
}

#[test]
#[should_panic]
fn random_floats_fail_1() {
    random_floats(EXAMPLE_SEED, 1, 0, 2, 1, 1, 10);
}

#[test]
#[should_panic]
fn random_floats_fail_2() {
    random_floats(EXAMPLE_SEED, 0, 1, 2, 1, 1, 10);
}

#[test]
#[should_panic]
fn random_floats_fail_3() {
    random_floats(EXAMPLE_SEED, 1, 1, 1, 0, 1, 10);
}

#[test]
#[should_panic]
fn random_floats_fail_4() {
    random_floats(EXAMPLE_SEED, 1, 1, 1, 1, 1, 10);
}

#[test]
#[should_panic]
fn random_floats_fail_5() {
    random_floats(EXAMPLE_SEED, 1, 1, 2, 1, 1, 0);
}

#[test]
#[should_panic]
fn random_floats_fail_6() {
    random_floats(EXAMPLE_SEED, 1, 1, 2, 1, 2, 1);
}
