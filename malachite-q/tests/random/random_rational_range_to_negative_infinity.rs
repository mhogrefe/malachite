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
use malachite_q::random::random_rational_range_to_negative_infinity;
use malachite_q::test_util::random::random_rationals_helper_helper;
use malachite_q::Rational;
use std::str::FromStr;

fn random_rational_range_to_negative_infinity_helper(
    a: &str,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_sample_median: (&str, Option<&str>),
    expected_sample_moment_stats: MomentStats,
) {
    random_rationals_helper_helper(
        random_rational_range_to_negative_infinity(
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
fn test_random_rational_range_to_negative_infinity() {
    let values = &[
        "-438/953",
        "-4248",
        "-6259791/632",
        "-99/2",
        "-34/87",
        "-5/3",
        "-3",
        "-233500344/25",
        "-45/2168",
        "-2/3",
        "-1",
        "-194403055/2031",
        "-1799/72",
        "-162713/14",
        "-1/31",
        "-25413015/7691773",
        "-243/3260",
        "-2/1895",
        "-261/252386",
        "-3997/36893154",
    ];
    let common_values = &[
        ("0", 8410),
        ("-1", 7679),
        ("-1/2", 6403),
        ("-1/3", 5093),
        ("-1/6", 4068),
        ("-3", 3591),
        ("-2", 3472),
        ("-3/2", 2895),
        ("-1/4", 2642),
        ("-2/3", 2350),
    ];
    let sample_median = ("-6303352/195639", Some("-56059/1740"));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-3.839214579009629e215),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_rational_range_to_negative_infinity_helper(
        "0",
        10,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    let values = &[
        "-2416809961348278/953",
        "-239755416",
        "-786400335/632",
        "-1187/2",
        "-1744034/87",
        "-31427/3",
        "1",
        "-1848/25",
        "-231/2168",
        "-662/3",
        "-30",
        "-31/2031",
        "-23/72",
        "-409/14",
        "1/31",
        "-389574262244759/7691773",
        "-3367126941/3260",
        "-12414/1895",
        "21/252386",
        "-254071517765467099/36893154",
    ];
    let common_values = &[
        ("0", 7287),
        ("-1", 6777),
        ("1", 6695),
        ("1/2", 5313),
        ("-1/2", 5087),
        ("-1/3", 3983),
        ("1/3", 3973),
        ("-2", 3145),
        ("2", 3070),
        ("-3", 3057),
    ];
    let sample_median = ("-5/6", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-2.0896126103851687e162),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_rational_range_to_negative_infinity_helper(
        "245850922/78256779",
        10,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    let values = &[
        "-121396406/953",
        "-152",
        "-99407/632",
        "-99/2",
        "-349/87",
        "-14/3",
        "-451",
        "-109/25",
        "-764903/2168",
        "-14253863309/3",
        "-5",
        "-19165/2031",
        "-3847/72",
        "-7065/14",
        "-360014815/31",
        "-20728497559/7691773",
        "-29701/3260",
        "-10141/1895",
        "-631864475205/252386",
        "-34535611967/36893154",
    ];
    let common_values = &[
        ("-7/2", 8239),
        ("-4", 2880),
        ("-5", 2814),
        ("-6", 2784),
        ("-7", 2757),
        ("-13/4", 1348),
        ("-10/3", 1319),
        ("-11", 1297),
        ("-12", 1287),
        ("-9", 1283),
    ];
    let sample_median = ("-399951/2456", Some("-158474076705/973165216"));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-5.436272253581606e29),
        standard_deviation: NiceFloat(3.412894005444307e32),
        skewness: NiceFloat(-733.9934078667648),
        excess_kurtosis: NiceFloat(587074.6340754497),
    };
    random_rational_range_to_negative_infinity_helper(
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
fn random_rational_range_to_negative_infinity_fail_1() {
    random_rational_range_to_negative_infinity(
        EXAMPLE_SEED,
        Rational::from_unsigneds(1u32, 3),
        10,
        0,
    );
}

#[test]
#[should_panic]
fn random_rational_range_to_negative_infinity_fail_2() {
    random_rational_range_to_negative_infinity(
        EXAMPLE_SEED,
        Rational::from_unsigneds(1u32, 3),
        0,
        0,
    );
}

#[test]
#[should_panic]
fn random_rational_range_to_negative_infinity_fail_3() {
    random_rational_range_to_negative_infinity(EXAMPLE_SEED, Rational::from_signeds(-1, 3), 2, 3);
}
