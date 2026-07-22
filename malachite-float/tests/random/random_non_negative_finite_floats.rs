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
use malachite_float::random::random_non_negative_finite_floats;
use malachite_float::test_util::random::random_floats_helper_helper;

fn random_non_negative_finite_floats_helper(
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
        random_non_negative_finite_floats(
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
fn test_random_non_negative_finite_floats() {
    // mean |sci_exponent| 1, mean precision 2, zero probability 1/10
    let values = &[
        "8.0", "1.0", "0.75", "0.0", "4.0", "2.0", "1.0", "2.0", "0.75", "0.95361", "2.0", "32.0",
        "0.50", "0.0", "1.0", "2.0", "0.50", "0.0", "1.5", "0.12",
    ];
    let values_hex = &[
        "0x8.0#1",
        "0x1.0#1",
        "0x0.c#2",
        "0x0.0",
        "0x4.0#1",
        "0x2.0#3",
        "0x1.0#1",
        "0x2.0#1",
        "0x0.c#2",
        "0x0.f42#12",
        "0x2.0#1",
        "0x2.0E+1#1",
        "0x0.8#1",
        "0x0.0",
        "0x1.0#1",
        "0x2.0#2",
        "0x0.8#1",
        "0x0.0",
        "0x1.8#2",
        "0x0.2#1",
    ];
    let common_values = &[
        ("1.0", 149586),
        ("0.0", 100224),
        ("2.0", 74929),
        ("0.50", 74827),
        ("1.0", 37902),
        ("4.0", 37612),
        ("0.25", 37602),
        ("1.5", 37600),
        ("0.50", 18934),
        ("0.75", 18834),
    ];
    let common_values_hex = &[
        ("0x1.0#1", 149586),
        ("0x0.0", 100224),
        ("0x2.0#1", 74929),
        ("0x0.8#1", 74827),
        ("0x1.0#2", 37902),
        ("0x4.0#1", 37612),
        ("0x0.4#1", 37602),
        ("0x1.8#2", 37600),
        ("0x0.8#2", 18934),
        ("0x0.c#2", 18834),
    ];
    let sample_median = ("1.0", None);
    let sample_median_hex = ("0x1.0#1", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(8.693371936934735),
        standard_deviation: NiceFloat(1671.7840625385452),
        skewness: NiceFloat(718.2460436055444),
        excess_kurtosis: NiceFloat(578520.0792964109),
    };
    random_non_negative_finite_floats_helper(
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
        "8.28861615548880710123228230074063493724223027547309594725606129258466731212769395352521903343910199",
        "1.0",
        "0.66515922477",
        "0.0",
        "4.23452428738467653908258408",
        "2.38320785690539951782446111823006788535",
        "1.036853813",
        "2.4027026671793265640566992824",
        "0.526664058624401715",
        "0.53662",
        "3.42175",
        "61.7010801144909624381",
        "0.83367084374",
        "0.0",
        "1.0",
        "2.50",
        "0.6580273643849",
        "0.0",
        "1.246",
        "0.141",
    ];
    let values_hex = &[
        "0x8.49e2bf94ebf30cb80717aab089ffe81a4240bd550c84a499a4753e306f26fd3913222c258d89033b#324",
        "0x1.0#1",
        "0x0.aa47dffd#32",
        "0x0.0",
        "0x4.3c09c8a06f55060972e84#86",
        "0x2.6219e8fcfa9bb9ef41add9535f62860#125",
        "0x1.096f406#28",
        "0x2.671785a18c13ffbbe8b27f4#93",
        "0x0.86d374abc53b0a#55",
        "0x0.896#12",
        "0x3.6bf8#16",
        "0x3d.b379fc839d57b1c#65",
        "0x0.d56b73d18#33",
        "0x0.0",
        "0x1.0#2",
        "0x2.8#4",
        "0x0.a8747b39e8#37",
        "0x0.0",
        "0x1.3f#9",
        "0x0.24#4",
    ];
    let common_values = &[
        ("0.0", 100224),
        ("1.0", 4643),
        ("2.0", 2373),
        ("0.50", 2353),
        ("1.5", 2327),
        ("1.0", 2294),
        ("0.50", 1219),
        ("4.0", 1204),
        ("2.0", 1195),
        ("0.75", 1191),
    ];
    let common_values_hex = &[
        ("0x0.0", 100224),
        ("0x1.0#1", 4643),
        ("0x2.0#1", 2373),
        ("0x0.8#1", 2353),
        ("0x1.8#2", 2327),
        ("0x1.0#2", 2294),
        ("0x0.8#2", 1219),
        ("0x4.0#1", 1204),
        ("0x2.0#2", 1195),
        ("0x0.c#2", 1191),
    ];
    let sample_median = (
        "1.312671781185719983785",
        Some("1.31268190290298901432879852014280461789620659172002407"),
    );
    let sample_median_hex = (
        "0x1.500b42029321dc7ec#67",
        Some("0x1.500bebd304f66efb8d7a481eef659ad171581e14dea8#175"),
    );
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(10.510498034617717),
        standard_deviation: NiceFloat(1688.742389206287),
        skewness: NiceFloat(594.3533044311691),
        excess_kurtosis: NiceFloat(387838.0226770939),
    };
    random_non_negative_finite_floats_helper(
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
        "1.1e9", "1.9e-6", "24.0", "0.0", "4.8e-7", "0.062", "7.6e22", "7.4e19", "0.047",
        "6.3996e7", "1.7e38", "7.5e-9", "2.3e15", "0.0", "2.7e11", "1.5e-8", "5.6e14", "0.0",
        "0.047", "1.9e-34",
    ];
    let values_hex = &[
        "0x4.0E+7#1",
        "0x0.00002#1",
        "0x18.0#2",
        "0x0.0",
        "0x8.0E-6#1",
        "0x0.10#3",
        "0x1.0E+19#1",
        "0x4.0E+16#1",
        "0x0.0c#2",
        "0x3.d08E+6#12",
        "0x8.0E+31#1",
        "0x2.0E-7#1",
        "0x8.0E+12#1",
        "0x0.0",
        "0x4.0E+9#1",
        "0x4.0E-7#2",
        "0x2.0E+12#1",
        "0x0.0",
        "0x0.0c#2",
        "0x1.0E-28#1",
    ];
    let common_values = &[
        ("0.0", 100224),
        ("1.0", 3531),
        ("2.0", 3503),
        ("4.0", 3399),
        ("0.50", 3381),
        ("0.12", 3375),
        ("0.25", 3369),
        ("0.062", 3330),
        ("8.0", 3283),
        ("16.0", 3250),
    ];
    let common_values_hex = &[
        ("0x0.0", 100224),
        ("0x1.0#1", 3531),
        ("0x2.0#1", 3503),
        ("0x4.0#1", 3399),
        ("0x0.8#1", 3381),
        ("0x0.2#1", 3375),
        ("0x0.4#1", 3369),
        ("0x0.1#1", 3330),
        ("0x8.0#1", 3283),
        ("0x1.0E+1#1", 3250),
    ];
    let sample_median = ("0.0059", Some("0.00586"));
    let sample_median_hex = ("0x0.018#3", Some("0x0.018#4"));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(2.237414368630999e242),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_non_negative_finite_floats_helper(
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
        "1112479228.65380241855766293066002073855407473322681306316671638091945729426964592832149248937055230",
        "1.9e-6",
        "21.285095192",
        "0.0",
        "5.04794631884655539880106935e-7",
        "0.0744752455282937349320144099446896214172",
        "7.834245908e22",
        "88644082373352750967.817947298",
        "0.0329165036640251072",
        "3.6012e7",
        "2.91091e38",
        "1.43659021971959067597e-8",
        "3.7545197012e15",
        "0.0",
        "2.7e11",
        "1.86e-8",
        "7.408729482609e14",
        "0.0",
        "0.03894",
        "2.17e-34",
    ];
    let values_hex = &[
        "0x424f15fc.a75f9865c038bd55844fff40d21205eaa8642524cd23a9f1837937e9c89911612c6c4819d8#324",
        "0x0.00002#1",
        "0x15.48fbffa#32",
        "0x0.0",
        "0x8.78139140deaa0c12e5d08E-6#86",
        "0x0.1310cf47e7d4ddcf7a0d6eca9afb1430#125",
        "0x1.096f406E+19#28",
        "0x4ce2f0b431827ff77.d164fe8#93",
        "0x0.086d374abc53b0a#55",
        "0x2.258E+6#12",
        "0xd.afeE+31#16",
        "0x3.db379fc839d57b1cE-7#65",
        "0xd.56b73d18E+12#33",
        "0x0.0",
        "0x4.0E+9#2",
        "0x5.0E-7#4",
        "0x2.a1d1ece7aE+12#37",
        "0x0.0",
        "0x0.09f8#9",
        "0x1.2E-28#4",
    ];
    let common_values = &[
        ("0.0", 100224),
        ("8.0", 129),
        ("4.0", 120),
        ("2.0", 107),
        ("0.062", 107),
        ("1.0e3", 106),
        ("32.0", 105),
        ("4.1e3", 104),
        ("0.50", 102),
        ("1.3e2", 102),
    ];
    let common_values_hex = &[
        ("0x0.0", 100224),
        ("0x8.0#1", 129),
        ("0x4.0#1", 120),
        ("0x2.0#1", 107),
        ("0x0.1#1", 107),
        ("0x4.0E+2#1", 106),
        ("0x2.0E+1#1", 105),
        ("0x1.0E+3#1", 104),
        ("0x0.8#1", 102),
        ("0x8.0E+1#1", 102),
    ];
    let sample_median = ("0.0074474812", Some("0.00744798329447"));
    let sample_median_hex = ("0x0.01e8140#21", Some("0x0.01e81c6cabc#35"));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(3.6946902524964283e242),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_non_negative_finite_floats_helper(
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
fn random_non_negative_finite_floats_fail_1() {
    random_non_negative_finite_floats(EXAMPLE_SEED, 1, 0, 2, 1, 1, 10);
}

#[test]
#[should_panic]
fn random_non_negative_finite_floats_fail_2() {
    random_non_negative_finite_floats(EXAMPLE_SEED, 0, 1, 2, 1, 1, 10);
}

#[test]
#[should_panic]
fn random_non_negative_finite_floats_fail_3() {
    random_non_negative_finite_floats(EXAMPLE_SEED, 1, 1, 1, 0, 1, 10);
}

#[test]
#[should_panic]
fn random_non_negative_finite_floats_fail_4() {
    random_non_negative_finite_floats(EXAMPLE_SEED, 1, 1, 1, 1, 1, 10);
}

#[test]
#[should_panic]
fn random_non_negative_finite_floats_fail_5() {
    random_non_negative_finite_floats(EXAMPLE_SEED, 1, 1, 2, 1, 1, 0);
}

#[test]
#[should_panic]
fn random_non_negative_finite_floats_fail_6() {
    random_non_negative_finite_floats(EXAMPLE_SEED, 1, 1, 2, 1, 2, 1);
}
