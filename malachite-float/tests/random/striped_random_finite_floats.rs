// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::float::NiceFloat;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::moments::MomentStats;
use malachite_float::random::striped_random_finite_floats;
use malachite_float::test_util::random::random_floats_helper_helper;

fn striped_random_finite_floats_helper(
    mean_sci_exponent_abs_numerator: u64,
    mean_sci_exponent_abs_denominator: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
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
        striped_random_finite_floats(
            EXAMPLE_SEED,
            mean_sci_exponent_abs_numerator,
            mean_sci_exponent_abs_denominator,
            mean_stripe_numerator,
            mean_stripe_denominator,
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
    )
}

#[test]
fn test_striped_random_finite_floats() {
    // mean |sci_exponent| 1, mean precision 2, special probability 1/10
    let values = &[
        "-3.0", "-1.2", "-2.0", "1.12", "-1.5", "1.0", "-2.0", "-0.25", "-1.0", "0.75", "0.0",
        "0.2617", "-1.2", "-0.594", "8.0", "-6.0", "-2.0", "-0.25", "-0.50", "-0.50",
    ];
    let values_hex = &[
        "-0x3.0#3",
        "-0x1.4#3",
        "-0x2.0#2",
        "0x1.2#5",
        "-0x1.8#2",
        "0x1.0#3",
        "-0x2.0#1",
        "-0x0.4#3",
        "-0x1.0#1",
        "0x0.c#2",
        "0x0.0",
        "0x0.430#9",
        "-0x1.4#3",
        "-0x0.98#5",
        "0x8.0#1",
        "-0x6.0#2",
        "-0x2.0#2",
        "-0x0.4#1",
        "-0x0.8#2",
        "-0x0.8#2",
    ];
    let common_values = &[
        ("1.0", 75012),
        ("-1.0", 74560),
        ("0.0", 49991),
        ("-0.0", 49908),
        ("0.50", 37822),
        ("2.0", 37501),
        ("-0.50", 37342),
        ("-2.0", 37246),
        ("1.5", 18939),
        ("-0.25", 18908),
    ];
    let common_values_hex = &[
        ("0x1.0#1", 75012),
        ("-0x1.0#1", 74560),
        ("0x0.0", 49991),
        ("-0x0.0", 49908),
        ("0x0.8#1", 37822),
        ("0x2.0#1", 37501),
        ("-0x0.8#1", 37342),
        ("-0x2.0#1", 37246),
        ("0x1.8#2", 18939),
        ("-0x0.4#1", 18908),
    ];
    let sample_median = ("0.0", None);
    let sample_median_hex = ("0x0.0", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.15755047615569168),
        standard_deviation: NiceFloat(1365.3366507063222),
        skewness: NiceFloat(296.763954633545),
        excess_kurtosis: NiceFloat(413698.88164587686),
    };
    striped_random_finite_floats_helper(
        1,
        1,
        8,
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

    // mean |sci_exponent| 1, mean precision 64, special probability 1/10
    let values = &[
        "-2.000000000007275957607407162325286656085306869873",
        "-1.999999999999999986",
        "-2.00",
        "1.99999955296516418457031249999999999999990816",
        "-1.99224853504",
        "1.7500000",
        "-2.000000000000000000",
        "-0.499999999534338712692260742",
        "-1.98437549173831934146150827576583644692",
        "0.999999985",
        "0.0",
        "0.250000058673549572715866",
        "-1.99999999906867742711924491",
        "-0.5000001192092895447097045991913682",
        "15.999985",
        "-7.50",
        "-3.99999999999994316",
        "-0.499999999999999999986447472844",
        "-0.50000",
        "-0.531188964844",
    ];
    let values_hex = &[
        "-0x2.0000000007ffffffe00000000003dfffffffff0#158",
        "-0x1.ffffffffffffff#57",
        "-0x2.0#5",
        "0x1.fffff87ffffffffffffffffffffffffff800#145",
        "-0x1.fe03ffff8#34",
        "0x1.c00000#22",
        "-0x2.00000000000000#57",
        "-0x0.7ffffffe00000000000000#84",
        "-0x1.fc00083ffffffc000003f0000007ffc#123",
        "0x0.ffffffc#26",
        "0x0.0",
        "0x0.400000fc003fffe0000#75",
        "-0x1.fffffffc0000001ffffff#85",
        "-0x0.800001ffffffff8fffefffffffe#107",
        "0xf.ffff0#21",
        "-0x7.8#4",
        "-0x3.fffffffffff000#55",
        "-0x0.7fffffffffffffffc0000000#94",
        "-0x0.8000#13",
        "-0x0.87fc00000#36",
    ];
    let common_values = &[
        ("0.0", 49991),
        ("-0.0", 49908),
        ("1.0", 2360),
        ("-1.0", 2302),
        ("-1.0", 1217),
        ("2.0", 1209),
        ("1.0", 1193),
        ("-0.50", 1177),
        ("-2.0", 1158),
        ("0.50", 1158),
    ];
    let common_values_hex = &[
        ("0x0.0", 49991),
        ("-0x0.0", 49908),
        ("0x1.0#1", 2360),
        ("-0x1.0#1", 2302),
        ("-0x1.0#2", 1217),
        ("0x2.0#1", 1209),
        ("0x1.0#2", 1193),
        ("-0x0.8#1", 1177),
        ("-0x2.0#1", 1158),
        ("0x0.8#1", 1158),
    ];
    let sample_median = ("0.0", None);
    let sample_median_hex = ("0x0.0", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.10376383630935192),
        standard_deviation: NiceFloat(1990.1410792105194),
        skewness: NiceFloat(339.3581426358755),
        excess_kurtosis: NiceFloat(463204.1189100673),
    };
    striped_random_finite_floats_helper(
        1,
        1,
        32,
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

    // mean |sci_exponent| 64, mean precision 2, special probability 1/10
    let values = &[
        "-5.3e13", "-2.8e-16", "-0.50", "3.02e8", "-1.4e-6", "0.25", "-1.5e-11", "-5.6e-17",
        "-3.4e10", "1.7e-13", "0.0", "3.090e20", "-1.6e-29", "-4.70e28", "1.5e-8", "-2.5e-21",
        "-8.6e9", "-1.1e43", "-0.12", "-0.12",
    ];
    let values_hex = &[
        "-0x3.0E+11#3",
        "-0x1.4E-13#3",
        "-0x0.8#2",
        "0x1.2E+7#5",
        "-0x0.000018#2",
        "0x0.4#3",
        "-0x1.0E-9#1",
        "-0x4.0E-14#3",
        "-0x8.0E+8#1",
        "0x3.0E-11#2",
        "0x0.0",
        "0x1.0cE+17#9",
        "-0x1.4E-24#3",
        "-0x9.8E+23#5",
        "0x4.0E-7#1",
        "-0xc.0E-18#2",
        "-0x2.0E+8#2",
        "-0x8.0E+35#1",
        "-0x0.2#2",
        "-0x0.2#2",
    ];
    let common_values = &[
        ("0.0", 49991),
        ("-0.0", 49908),
        ("1.0", 1791),
        ("0.50", 1788),
        ("-1.0", 1782),
        ("-4.0", 1778),
        ("-0.50", 1727),
        ("2.0", 1695),
        ("4.0", 1690),
        ("16.0", 1689),
    ];
    let common_values_hex = &[
        ("0x0.0", 49991),
        ("-0x0.0", 49908),
        ("0x1.0#1", 1791),
        ("0x0.8#1", 1788),
        ("-0x1.0#1", 1782),
        ("-0x4.0#1", 1778),
        ("-0x0.8#1", 1727),
        ("0x2.0#1", 1695),
        ("0x4.0#1", 1690),
        ("0x1.0E+1#1", 1689),
    ];
    let sample_median = ("0.0", None);
    let sample_median_hex = ("0x0.0", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1.789929787900277e243),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    striped_random_finite_floats_helper(
        64,
        1,
        8,
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

    // mean |sci_exponent| 64, mean precision 64, special probability 1/10
    let values = &[
        "-35184372088959.99999988079071044923187901068243857",
        "-4.440892098500626131e-16",
        "-0.500",
        "536870791.999999999999999999999999999999975348",
        "-1.89995625977e-6",
        "0.43750000",
        "-1.455191522836685181e-11",
        "-1.11022302359118077473234707e-16",
        "-68182622719.9999980926514790979809351489",
        "2.27373672e-13",
        "0.0",
        "295147974448853809152.000",
        "-2.52435489553174342868477269e-29",
        "-39614090701865134055025016831.99951",
        "2.9802294e-8",
        "-3.18e-21",
        "-17179869183.9997559",
        "-2.23007451985306231409312553628e43",
        "-0.12500",
        "-0.132797241211",
    ];
    let values_hex = &[
        "-0x20000000007f.fffffe00000000003dfffffffff0#158",
        "-0x1.ffffffffffffffE-13#57",
        "-0x0.80#5",
        "0x1fffff87.ffffffffffffffffffffffffff800#145",
        "-0x0.00001fe03ffff8#34",
        "0x0.700000#22",
        "-0x1.00000000000000E-9#57",
        "-0x7.ffffffe00000000000000E-14#84",
        "-0xfe00041ff.ffffe000001f8000003ffe#123",
        "0x3.ffffffE-11#26",
        "0x0.0",
        "0x1000003f000ffff800.00#75",
        "-0x1.fffffffc0000001ffffffE-24#85",
        "-0x800001ffffffff8fffefffff.ffe#107",
        "0x7.ffff8E-7#21",
        "-0xf.0E-18#4",
        "-0x3ffffffff.fff000#55",
        "-0xf.fffffffffffffff80000000E+35#94",
        "-0x0.2000#13",
        "-0x0.21ff000000#36",
    ];
    let common_values = &[
        ("0.0", 49991),
        ("-0.0", 49908),
        ("-16.0", 77),
        ("2.0", 66),
        ("-2.0", 61),
        ("-8.0", 61),
        ("-32.0", 59),
        ("-0.25", 58),
        ("0.50", 57),
        ("4.1e3", 57),
    ];
    let common_values_hex = &[
        ("0x0.0", 49991),
        ("-0x0.0", 49908),
        ("-0x1.0E+1#1", 77),
        ("0x2.0#1", 66),
        ("-0x2.0#1", 61),
        ("-0x8.0#1", 61),
        ("-0x2.0E+1#1", 59),
        ("-0x0.4#1", 58),
        ("0x0.8#1", 57),
        ("0x1.0E+3#1", 57),
    ];
    let sample_median = ("0.0", None);
    let sample_median_hex = ("0x0.0", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1.7908037774743073e243),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    striped_random_finite_floats_helper(
        64,
        1,
        32,
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
fn striped_random_finite_floats_fail_1() {
    striped_random_finite_floats(EXAMPLE_SEED, 1, 0, 2, 1, 2, 1, 1, 10);
}

#[test]
#[should_panic]
fn striped_random_finite_floats_fail_2() {
    striped_random_finite_floats(EXAMPLE_SEED, 0, 1, 2, 1, 2, 1, 1, 10);
}

#[test]
#[should_panic]
fn striped_random_finite_floats_fail_3() {
    striped_random_finite_floats(EXAMPLE_SEED, 1, 1, 2, 1, 1, 0, 1, 10);
}

#[test]
#[should_panic]
fn striped_random_finite_floats_fail_4() {
    striped_random_finite_floats(EXAMPLE_SEED, 1, 1, 2, 1, 1, 1, 1, 10);
}

#[test]
#[should_panic]
fn striped_random_finite_floats_fail_5() {
    striped_random_finite_floats(EXAMPLE_SEED, 1, 1, 2, 3, 2, 1, 1, 10);
}

#[test]
#[should_panic]
fn striped_random_finite_floats_fail_6() {
    striped_random_finite_floats(EXAMPLE_SEED, 1, 1, 1, 0, 2, 1, 1, 10);
}

#[test]
#[should_panic]
fn striped_random_finite_floats_fail_7() {
    striped_random_finite_floats(EXAMPLE_SEED, 1, 1, 2, 1, 2, 1, 1, 0);
}
