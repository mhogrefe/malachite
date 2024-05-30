// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::float::NiceFloat;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::moments::MomentStats;
use malachite_float::random::striped_random_nonzero_finite_floats;
use malachite_float::test_util::random::random_floats_helper_helper;

fn striped_random_nonzero_finite_floats_helper(
    mean_sci_exponent_abs_numerator: u64,
    mean_sci_exponent_abs_denominator: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_precision_numerator: u64,
    mean_precision_denominator: u64,
    expected_values: &[&str],
    expected_values_hex: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_common_values_hex: &[(&str, usize)],
    expected_median: (&str, Option<&str>),
    expected_median_hex: (&str, Option<&str>),
    expected_moment_stats: MomentStats,
) {
    random_floats_helper_helper(
        striped_random_nonzero_finite_floats(
            EXAMPLE_SEED,
            mean_sci_exponent_abs_numerator,
            mean_sci_exponent_abs_denominator,
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_precision_numerator,
            mean_precision_denominator,
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
fn test_striped_random_nonzero_finite_floats() {
    // mean |sci_exponent| 1, mean precision 2
    let values = &[
        "-8.0", "-1.0", "-0.8", "4.0", "-3.5", "1.0", "-2.0", "-0.5", "-0.5", "2.0", "3.0e1",
        "0.5", "-1.0", "-3.0", "0.5", "-1.0", "-0.1", "-1.0", "-0.055", "-1.8",
    ];
    let values_hex = &[
        "-0x8.0#1",
        "-0x1.0#1",
        "-0x0.c#2",
        "0x4.0#1",
        "-0x3.8#3",
        "0x1.0#1",
        "-0x2.0#1",
        "-0x0.8#2",
        "-0x0.800#12",
        "0x2.0#1",
        "0x2.0E+1#1",
        "0x0.8#1",
        "-0x1.0#1",
        "-0x3.0#2",
        "0x0.8#1",
        "-0x1.0#2",
        "-0x0.2#1",
        "-0x1.0#1",
        "-0x0.0e#3",
        "-0x1.c#3",
    ];
    let common_values = &[
        ("1.0", 83167),
        ("-1.0", 83060),
        ("2.0", 41731),
        ("-2.0", 41688),
        ("0.5", 41643),
        ("-0.5", 41534),
        ("1.0", 21186),
        ("-4.0", 21185),
        ("-1.0", 21101),
        ("0.2", 21077),
    ];
    let common_values_hex = &[
        ("0x1.0#1", 83167),
        ("-0x1.0#1", 83060),
        ("0x2.0#1", 41731),
        ("-0x2.0#1", 41688),
        ("0x0.8#1", 41643),
        ("-0x0.8#1", 41534),
        ("0x1.0#2", 21186),
        ("-0x4.0#1", 21185),
        ("-0x1.0#2", 21101),
        ("0x0.4#1", 21077),
    ];
    let sample_median = ("0.002", None);
    let sample_median_hex = ("0x0.008#1", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-1.1181422910011878),
        standard_deviation: NiceFloat(1654.3094882610405),
        skewness: NiceFloat(-398.68071157281685),
        excess_kurtosis: NiceFloat(306701.41416153224),
    };
    striped_random_nonzero_finite_floats_helper(
        1,
        1,
        8,
        1,
        2,
        1,
        values,
        values_hex,
        common_values,
        common_values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );

    // mean |sci_exponent| 1, mean precision 64
    let values = &[
        "-15.9999694824218750000271017455896874047580178881790484400708024277502849217376144974852\
        750344620578",
        "-1.0",
        "-0.9999961855",
        "7.9999923707218827326204245",
        "-3.99902343750000005551115123125625546225",
        "1.999999963",
        "-3.9687500000000568434188608076",
        "-0.99999999999999645",
        "-0.5",
        "3.5",
        "63.9375",
        "0.5019531249",
        "-1.5",
        "-3.8",
        "0.5",
        "-1.0",
        "-0.23",
        "-1.624816894531249999999999173",
        "-0.06225609778",
        "-1.9999",
    ];
    let values_hex = &[
        "-0xf.fffe0000000000007ffc0000000007ffffff8003fe00000000000003fff0003ffffffff8ff000000\
        #324",
        "-0x1.0#1",
        "-0x0.ffffc001#32",
        "0x7.ffff80007fff9f80000fe#86",
        "-0x3.ffc00000000003ffffffffffe01fffe#125",
        "0x1.ffffff6#28",
        "-0x3.f8000000000fffffffffffe#93",
        "-0x0.ffffffffffff00#55",
        "-0x0.800#12",
        "0x3.8000#16",
        "0x3f.f00000000000000#65",
        "0x0.807fffff8#33",
        "-0x1.8#2",
        "-0x3.c#4",
        "0x0.8000000000#37",
        "-0x1.00#9",
        "-0x0.3c#4",
        "-0x1.9ff3ffffffffffffffff000#90",
        "-0x0.0ff003fff#32",
        "-0x1.fff8#14",
    ];
    let common_values = &[
        ("-1.0", 2594),
        ("1.0", 2582),
        ("2.0", 1322),
        ("0.5", 1316),
        ("-2.0", 1310),
        ("-1.5", 1300),
        ("1.0", 1293),
        ("-0.5", 1282),
        ("1.5", 1281),
        ("-1.0", 1251),
    ];
    let common_values_hex = &[
        ("-0x1.0#1", 2594),
        ("0x1.0#1", 2582),
        ("0x2.0#1", 1322),
        ("0x0.8#1", 1316),
        ("-0x2.0#1", 1310),
        ("-0x1.8#2", 1300),
        ("0x1.0#2", 1293),
        ("-0x0.8#1", 1282),
        ("0x1.8#2", 1281),
        ("-0x1.0#2", 1251),
    ];
    let sample_median = (
        "0.0019826889",
        Some("0.00198341719988093245774507522583003"),
    );
    let sample_median_hex = (
        "0x0.0081f0000#26",
        Some("0x0.0081fc3801ffffffffffffffffffc#106"),
    );
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-1.535338105008931),
        standard_deviation: NiceFloat(2619.656937757806),
        skewness: NiceFloat(-395.5135549279457),
        excess_kurtosis: NiceFloat(376873.9371275132),
    };
    striped_random_nonzero_finite_floats_helper(
        1,
        1,
        32,
        1,
        64,
        1,
        values,
        values_hex,
        common_values,
        common_values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );

    // mean |sci_exponent| 64, mean precision 2
    let values = &[
        "-1.0e9", "-2.0e-6", "-24.0", "5.0e-7", "-0.11", "8.0e22", "-7.0e19", "-0.03", "-3.355e7",
        "2.0e38", "7.0e-9", "2.0e15", "-3.0e11", "-2.0e-8", "6.0e14", "-0.03", "-2.0e-34",
        "-6.0e14", "-4.4e30", "-2.7e57",
    ];
    let values_hex = &[
        "-0x4.0E+7#1",
        "-0x0.00002#1",
        "-0x18.0#2",
        "0x8.0E-6#1",
        "-0x0.1c#3",
        "0x1.0E+19#1",
        "-0x4.0E+16#1",
        "-0x0.08#2",
        "-0x2.000E+6#12",
        "0x8.0E+31#1",
        "0x2.0E-7#1",
        "0x8.0E+12#1",
        "-0x4.0E+9#1",
        "-0x6.0E-7#2",
        "0x2.0E+12#1",
        "-0x0.08#2",
        "-0x1.0E-28#1",
        "-0x2.0E+12#1",
        "-0x3.8E+25#3",
        "-0x7.0E+47#3",
    ];
    let common_values = &[
        ("1.0", 2018),
        ("-2.0", 2004),
        ("-1.0", 1938),
        ("4.0", 1933),
        ("0.5", 1901),
        ("0.1", 1900),
        ("-0.2", 1883),
        ("-4.0", 1875),
        ("0.2", 1874),
        ("2.0", 1873),
    ];
    let common_values_hex = &[
        ("0x1.0#1", 2018),
        ("-0x2.0#1", 2004),
        ("-0x1.0#1", 1938),
        ("0x4.0#1", 1933),
        ("0x0.8#1", 1901),
        ("0x0.2#1", 1900),
        ("-0x0.4#1", 1883),
        ("-0x4.0#1", 1875),
        ("0x0.4#1", 1874),
        ("0x2.0#1", 1873),
    ];
    let sample_median = ("1.0e-122", None);
    let sample_median_hex = ("0x8.0E-102#2", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-2.237414368630952e242),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    striped_random_nonzero_finite_floats_helper(
        64,
        1,
        8,
        1,
        2,
        1,
        values,
        values_hex,
        common_values,
        common_values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );

    // mean |sci_exponent| 64, mean precision 64
    let values = &[
        "-2147479552.00000000000363753471788186369683755073474993882824726098952739354828042999233\
        53285806191",
        "-2.0e-6",
        "-31.999877937",
        "9.5367340692542585523371988e-7",
        "-0.124969482421875001734723475976757983195",
        "1.511157246e23",
        "-146421031085071663103.99999999",
        "-0.062499999999999778",
        "-3.355e7",
        "2.97747e38",
        "1.48866092786192893982e-8",
        "2.2605959062e15",
        "-4.0e11",
        "-2.8e-8",
        "5.6294995342e14",
        "-0.0312",
        "-3.6e-34",
        "-914690595094527.999999999534",
        "-5.050814702e30",
        "-3.1384e57",
    ];
    let values_hex = &[
        "-0x7ffff000.0000000003ffe0000000003ffffffc001ff00000000000001fff8001ffffffffc7f8000000\
        #324",
        "-0x0.00002#1",
        "-0x1f.fff8002#32",
        "0xf.ffff0000ffff3f00001fcE-6#86",
        "-0x0.1ffe00000000001fffffffffff00ffff#125",
        "0x1.ffffff6E+19#28",
        "-0x7f0000000001fffff.ffffffc#93",
        "-0x0.0ffffffffffff00#55",
        "-0x2.000E+6#12",
        "0xe.000E+31#16",
        "0x3.ff00000000000000E-7#65",
        "0x8.07fffff8E+12#33",
        "-0x6.0E+9#2",
        "-0x7.8E-7#4",
        "0x2.000000000E+12#37",
        "-0x0.0800#9",
        "-0x1.eE-28#4",
        "-0x33fe7ffffffff.fffffffe00#90",
        "-0x3.fc00fffcE+25#32",
        "-0x7.ffeE+47#14",
    ];
    let common_values = &[
        ("-8.0", 73),
        ("1.0e3", 70),
        ("4.0", 69),
        ("2.0", 68),
        ("8.0", 68),
        ("-1.0e2", 67),
        ("-0.02", 65),
        ("-0.5", 64),
        ("2.0e1", 64),
        ("-4.0", 62),
    ];
    let common_values_hex = &[
        ("-0x8.0#1", 73),
        ("0x4.0E+2#1", 70),
        ("0x4.0#1", 69),
        ("0x2.0#1", 68),
        ("0x8.0#1", 68),
        ("-0x8.0E+1#1", 67),
        ("-0x0.04#1", 65),
        ("-0x0.8#1", 64),
        ("0x1.0E+1#1", 64),
        ("-0x4.0#1", 62),
    ];
    let sample_median = (
        "1.21019420636902759627106091036435e-122",
        Some("1.81527747811658e-122"),
    );
    let sample_median_hex = (
        "0x8.0003fffffffc00000000000000E-102#105",
        Some("0xc.000001fff00E-102#45"),
    );
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-2.2384727158818326e242),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    striped_random_nonzero_finite_floats_helper(
        64,
        1,
        32,
        1,
        64,
        1,
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
fn striped_random_nonzero_finite_floats_fail_1() {
    striped_random_nonzero_finite_floats(EXAMPLE_SEED, 1, 0, 2, 1, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_nonzero_finite_floats_fail_2() {
    striped_random_nonzero_finite_floats(EXAMPLE_SEED, 0, 1, 2, 1, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_nonzero_finite_floats_fail_3() {
    striped_random_nonzero_finite_floats(EXAMPLE_SEED, 1, 1, 2, 1, 1, 0);
}

#[test]
#[should_panic]
fn striped_random_nonzero_finite_floats_fail_4() {
    striped_random_nonzero_finite_floats(EXAMPLE_SEED, 1, 1, 2, 1, 1, 1);
}

#[test]
#[should_panic]
fn striped_random_nonzero_finite_floats_fail_5() {
    striped_random_nonzero_finite_floats(EXAMPLE_SEED, 1, 1, 2, 3, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_nonzero_finite_floats_fail_6() {
    striped_random_nonzero_finite_floats(EXAMPLE_SEED, 1, 1, 1, 0, 2, 1);
}
