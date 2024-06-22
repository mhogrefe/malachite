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
use malachite_q::random::random_rational_range_to_infinity;
use malachite_q::test_util::random::random_rationals_helper_helper;
use malachite_q::Rational;
use std::str::FromStr;

fn random_rational_range_to_infinity_helper(
    a: &str,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_sample_median: (&str, Option<&str>),
    expected_sample_moment_stats: MomentStats,
) {
    random_rationals_helper_helper(
        random_rational_range_to_infinity(
            EXAMPLE_SEED,
            Rational::from_str(a).unwrap(),
            mean_bits_numerator,
            mean_bits_denominator,
        ),
        expected_values,
        expected_common_values,
        expected_sample_median,
        expected_sample_moment_stats,
    );
}

#[test]
fn test_random_rational_range_to_infinity() {
    let values = &[
        "438/953",
        "4248",
        "6259791/632",
        "99/2",
        "34/87",
        "5/3",
        "3",
        "233500344/25",
        "45/2168",
        "2/3",
        "1",
        "194403055/2031",
        "1799/72",
        "162713/14",
        "1/31",
        "25413015/7691773",
        "243/3260",
        "2/1895",
        "261/252386",
        "3997/36893154",
    ];
    let common_values = &[
        ("0", 8410),
        ("1", 7679),
        ("1/2", 6403),
        ("1/3", 5093),
        ("1/6", 4068),
        ("3", 3591),
        ("2", 3472),
        ("3/2", 2895),
        ("1/4", 2642),
        ("2/3", 2350),
    ];
    let sample_median = ("56059/1740", Some("6303352/195639"));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(3.839214579009629e215),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_rational_range_to_infinity_helper(
        "0",
        10,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    let values = &[
        "121396406/953",
        "152",
        "99407/632",
        "99/2",
        "436/87",
        "11/3",
        "451",
        "97/25",
        "764903/2168",
        "14253863309/3",
        "6",
        "19165/2031",
        "3847/72",
        "7065/14",
        "360014815/31",
        "20728497559/7691773",
        "13813/3260",
        "221536/1895",
        "1651717/252386",
        "828014364376645/36893154",
    ];
    let common_values = &[
        ("7/2", 8108),
        ("5", 2819),
        ("6", 2789),
        ("7", 2788),
        ("4", 2727),
        ("11/3", 1331),
        ("11", 1302),
        ("14/3", 1302),
        ("15", 1299),
        ("15/4", 1299),
    ];
    let sample_median = ("5632495/34508", Some("12647038/77483"));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1.5244840655472015e31),
        standard_deviation: NiceFloat(1.4418360146531462e34),
        skewness: NiceFloat(995.0794447771476),
        excess_kurtosis: NiceFloat(993187.6643660564),
    };
    random_rational_range_to_infinity_helper(
        "245850922/78256779",
        10,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    let values = &[
        "3983598880556866742/953",
        "-1",
        "48202831/632",
        "99/2",
        "10/87",
        "-8/3",
        "15043",
        "7793790546/25",
        "-7/2168",
        "191126/3",
        "2",
        "1110539913949/2031",
        "416495/72",
        "125991833/14",
        "3039/31",
        "20728497559/7691773",
        "7/3260",
        "16/1895",
        "-1/252386",
        "-149405/36893154",
    ];
    let common_values = &[
        ("0", 7260),
        ("-1", 6845),
        ("1", 6673),
        ("-1/2", 5123),
        ("1/2", 5089),
        ("-1/3", 4054),
        ("1/3", 4037),
        ("2", 3132),
        ("3", 3086),
        ("-3", 3014),
    ];
    let sample_median = ("120/143", Some("167/199"));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1.8672643518545765e150),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_rational_range_to_infinity_helper(
        "-245850922/78256779",
        10,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
}

#[test]
#[should_panic]
fn random_rational_range_to_infinity_fail_1() {
    random_rational_range_to_infinity(EXAMPLE_SEED, Rational::from_unsigneds(1u32, 3), 10, 0);
}

#[test]
#[should_panic]
fn random_rational_range_to_infinity_fail_2() {
    random_rational_range_to_infinity(EXAMPLE_SEED, Rational::from_unsigneds(1u32, 3), 0, 0);
}

#[test]
#[should_panic]
fn random_rational_range_to_infinity_fail_3() {
    random_rational_range_to_infinity(EXAMPLE_SEED, Rational::from_unsigneds(1u32, 3), 2, 3);
}
