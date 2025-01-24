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
use malachite_float::random::random_nonzero_finite_floats;
use malachite_float::test_util::random::random_floats_helper_helper;

fn random_nonzero_finite_floats_helper(
    mean_sci_exponent_abs_numerator: u64,
    mean_sci_exponent_abs_denominator: u64,
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
        random_nonzero_finite_floats(
            EXAMPLE_SEED,
            mean_sci_exponent_abs_numerator,
            mean_sci_exponent_abs_denominator,
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
fn test_random_nonzero_finite_floats() {
    // mean |sci_exponent| 1, mean precision 2
    let values = &[
        "-8.0", "-1.0", "-0.8", "4.0", "-2.0", "1.0", "-2.0", "-0.8", "-0.9536", "2.0", "3.0e1",
        "0.5", "-1.0", "-2.0", "0.5", "-1.5", "-0.1", "-1.0", "-0.047", "-1.0",
    ];
    let values_hex = &[
        "-0x8.0#1",
        "-0x1.0#1",
        "-0x0.c#2",
        "0x4.0#1",
        "-0x2.0#3",
        "0x1.0#1",
        "-0x2.0#1",
        "-0x0.c#2",
        "-0x0.f42#12",
        "0x2.0#1",
        "0x2.0E+1#1",
        "0x0.8#1",
        "-0x1.0#1",
        "-0x2.0#2",
        "0x0.8#1",
        "-0x1.8#2",
        "-0x0.2#1",
        "-0x1.0#1",
        "-0x0.0c#3",
        "-0x1.0#3",
    ];
    let common_values = &[
        ("1.0", 83167),
        ("-1.0", 83060),
        ("2.0", 41731),
        ("-2.0", 41688),
        ("0.5", 41643),
        ("-0.5", 41534),
        ("-4.0", 21185),
        ("1.0", 21166),
        ("0.2", 21077),
        ("-1.0", 20914),
    ];
    let common_values_hex = &[
        ("0x1.0#1", 83167),
        ("-0x1.0#1", 83060),
        ("0x2.0#1", 41731),
        ("-0x2.0#1", 41688),
        ("0x0.8#1", 41643),
        ("-0x0.8#1", 41534),
        ("-0x4.0#1", 21185),
        ("0x1.0#2", 21166),
        ("0x0.4#1", 21077),
        ("-0x1.0#2", 20914),
    ];
    let sample_median = ("0.002", None);
    let sample_median_hex = ("0x0.008#1", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-1.137441392805658),
        standard_deviation: NiceFloat(1753.4002861490633),
        skewness: NiceFloat(-540.308156545252),
        excess_kurtosis: NiceFloat(486100.3517515351),
    };
    random_nonzero_finite_floats_helper(
        1,
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
        "-8.28861615548880710123228230074063493724223027547309594725606129258466731212769395352521\
        9033439102",
        "-1.0",
        "-0.6651592248",
        "4.2345242873846765390825841",
        "-2.38320785690539951782446111823006788535",
        "1.036853813",
        "-2.4027026671793265640566992824",
        "-0.52666405862440172",
        "-0.5366",
        "3.42175",
        "61.701080114490962438",
        "0.8336708437",
        "-1.0",
        "-2.5",
        "0.658027364385",
        "-1.246",
        "-0.14",
        "-1.245722352956368383395858819",
        "-0.03962809021",
        "-1.0011",
    ];
    let values_hex = &[
        "-0x8.49e2bf94ebf30cb80717aab089ffe81a4240bd550c84a499a4753e306f26fd3913222c258d89033b\
        #324",
        "-0x1.0#1",
        "-0x0.aa47dffd#32",
        "0x4.3c09c8a06f55060972e84#86",
        "-0x2.6219e8fcfa9bb9ef41add9535f62860#125",
        "0x1.096f406#28",
        "-0x2.671785a18c13ffbbe8b27f4#93",
        "-0x0.86d374abc53b0a#55",
        "-0x0.896#12",
        "0x3.6bf8#16",
        "0x3d.b379fc839d57b1c#65",
        "0x0.d56b73d18#33",
        "-0x1.0#2",
        "-0x2.8#4",
        "0x0.a8747b39e8#37",
        "-0x1.3f#9",
        "-0x0.24#4",
        "-0x1.3ee7a8fdd801625ca3c1028#90",
        "-0x0.0a2511077#32",
        "-0x1.0048#14",
    ];
    let common_values = &[
        ("-1.0", 2594),
        ("1.0", 2582),
        ("2.0", 1322),
        ("0.5", 1316),
        ("-2.0", 1310),
        ("-1.5", 1295),
        ("1.5", 1288),
        ("1.0", 1286),
        ("-0.5", 1282),
        ("-1.0", 1256),
    ];
    let common_values_hex = &[
        ("-0x1.0#1", 2594),
        ("0x1.0#1", 2582),
        ("0x2.0#1", 1322),
        ("0x0.8#1", 1316),
        ("-0x2.0#1", 1310),
        ("-0x1.8#2", 1295),
        ("0x1.8#2", 1288),
        ("0x1.0#2", 1286),
        ("-0x0.8#1", 1282),
        ("-0x1.0#2", 1256),
    ];
    let sample_median = (
        "0.0027890310577504986877",
        Some("0.0027891362634973129863915401"),
    );
    let sample_median_hex = (
        "0x0.00b6c82d2e399e2de0#63",
        Some("0x0.00b6c9f1092ac094b9e5264#84"),
    );
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.6942588764777254),
        standard_deviation: NiceFloat(1892.6524949864393),
        skewness: NiceFloat(-231.427091034788),
        excess_kurtosis: NiceFloat(286498.2528287583),
    };
    random_nonzero_finite_floats_helper(
        1,
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
        "-1.0e9", "-2.0e-6", "-24.0", "5.0e-7", "-0.06", "8.0e22", "-7.0e19", "-0.05", "-6.4e7",
        "2.0e38", "7.0e-9", "2.0e15", "-3.0e11", "-1.0e-8", "6.0e14", "-0.05", "-2.0e-34",
        "-6.0e14", "-3.8e30", "-1.6e57",
    ];
    let values_hex = &[
        "-0x4.0E+7#1",
        "-0x0.00002#1",
        "-0x18.0#2",
        "0x8.0E-6#1",
        "-0x0.10#3",
        "0x1.0E+19#1",
        "-0x4.0E+16#1",
        "-0x0.0c#2",
        "-0x3.d08E+6#12",
        "0x8.0E+31#1",
        "0x2.0E-7#1",
        "0x8.0E+12#1",
        "-0x4.0E+9#1",
        "-0x4.0E-7#2",
        "0x2.0E+12#1",
        "-0x0.0c#2",
        "-0x1.0E-28#1",
        "-0x2.0E+12#1",
        "-0x3.0E+25#3",
        "-0x4.0E+47#3",
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
    let sample_median = ("1.0e-122", Some("1.0e-122"));
    let sample_median_hex = ("0x8.0E-102#2", Some("0x8.0E-102#3"));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-2.2374143686309846e242),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_nonzero_finite_floats_helper(
        64,
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
        "-1112479228.65380241855766293066002073855407473322681306316671638091945729426964592832149\
        24893705523",
        "-2.0e-6",
        "-21.285095192",
        "5.0479463188465553988010694e-7",
        "-0.074475245528293734932014409944689621417",
        "7.83424591e22",
        "-88644082373352750967.8179473",
        "-0.032916503664025107",
        "-3.601e7",
        "2.91091e38",
        "1.43659021971959067597e-8",
        "3.7545197012e15",
        "-3.0e11",
        "-1.9e-8",
        "7.4087294826e14",
        "-0.0389",
        "-2.2e-34",
        "-701279340572674.768452138179",
        "-3.215012629e30",
        "-1.571e57",
    ];
    let values_hex = &[
        "-0x424f15fc.a75f9865c038bd55844fff40d21205eaa8642524cd23a9f1837937e9c89911612c6c4819d8\
        #324",
        "-0x0.00002#1",
        "-0x15.48fbffa#32",
        "0x8.78139140deaa0c12e5d08E-6#86",
        "-0x0.1310cf47e7d4ddcf7a0d6eca9afb1430#125",
        "0x1.096f406E+19#28",
        "-0x4ce2f0b431827ff77.d164fe8#93",
        "-0x0.086d374abc53b0a#55",
        "-0x2.258E+6#12",
        "0xd.afeE+31#16",
        "0x3.db379fc839d57b1cE-7#65",
        "0xd.56b73d18E+12#33",
        "-0x4.0E+9#2",
        "-0x5.0E-7#4",
        "0x2.a1d1ece7aE+12#37",
        "-0x0.09f8#9",
        "-0x1.2E-28#4",
        "-0x27dcf51fbb002.c4b9478205#90",
        "-0x2.894441dcE+25#32",
        "-0x4.012E+47#14",
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
        "2.22082266973856198260258239259022e-122",
        Some("2.35472681582314e-122"),
    );
    let sample_median_hex = (
        "0xe.ae4e347efb6723ba8a9c199b48E-102#105",
        Some("0xf.90e97334e78E-102#45"),
    );
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-3.6946902524964396e242),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_nonzero_finite_floats_helper(
        64,
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
fn random_nonzero_finite_floats_fail_1() {
    random_nonzero_finite_floats(EXAMPLE_SEED, 1, 0, 2, 1);
}

#[test]
#[should_panic]
fn random_nonzero_finite_floats_fail_2() {
    random_nonzero_finite_floats(EXAMPLE_SEED, 0, 1, 2, 1);
}

#[test]
#[should_panic]
fn random_nonzero_finite_floats_fail_3() {
    random_nonzero_finite_floats(EXAMPLE_SEED, 1, 1, 1, 0);
}

#[test]
#[should_panic]
fn random_nonzero_finite_floats_fail_4() {
    random_nonzero_finite_floats(EXAMPLE_SEED, 1, 1, 1, 1);
}
