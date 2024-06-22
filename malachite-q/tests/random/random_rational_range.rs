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
use malachite_q::random::random_rational_range;
use malachite_q::test_util::random::random_rationals_helper_helper;
use malachite_q::Rational;
use std::str::FromStr;

fn random_rational_range_helper(
    a: &str,
    b: &str,
    mean_numerator_bits_numerator: u64,
    mean_numerator_bits_denominator: u64,
    mean_denominator_index_numerator: u64,
    mean_denominator_index_denominator: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_sample_median: (&str, Option<&str>),
    expected_sample_moment_stats: MomentStats,
) {
    random_rationals_helper_helper(
        random_rational_range(
            EXAMPLE_SEED,
            Rational::from_str(a).unwrap(),
            Rational::from_str(b).unwrap(),
            mean_numerator_bits_numerator,
            mean_numerator_bits_denominator,
            mean_denominator_index_numerator,
            mean_denominator_index_denominator,
        ),
        expected_values,
        expected_common_values,
        expected_sample_median,
        expected_sample_moment_stats,
    );
}

#[test]
fn test_random_rational_range() {
    let values = &[
        "0", "1/2", "0", "0", "0", "0", "0", "0", "1/2", "0", "1/2", "0", "1/3", "0", "1/2", "1/2",
        "1/3", "0", "1/6", "0",
    ];
    let common_values = &[
        ("0", 499797),
        ("1/2", 249730),
        ("1/3", 86236),
        ("1/4", 42606),
        ("2/3", 39168),
        ("3/4", 19633),
        ("1/5", 13421),
        ("1/6", 12245),
        ("3/5", 6206),
        ("2/5", 6141),
    ];
    let sample_median = ("1/12", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.22877695490175154),
        standard_deviation: NiceFloat(0.2524452096013498),
        skewness: NiceFloat(0.4918767315871824),
        excess_kurtosis: NiceFloat(-1.2397727721157625),
    };
    random_rational_range_helper(
        "0",
        "1",
        10,
        1,
        1,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    let values = &[
        "1/2", "1/6", "0", "11/15", "1/9", "1/24", "0", "1/6", "0", "7/10", "14/15", "4/29", "0",
        "3/13", "1/3", "10/29", "17/18", "0", "0", "1/8",
    ];
    let common_values = &[
        ("0", 90795),
        ("1/2", 82681),
        ("1/3", 51495),
        ("1/4", 46547),
        ("1/6", 44587),
        ("1/5", 26408),
        ("1/8", 24875),
        ("2/3", 23502),
        ("3/4", 21468),
        ("1/12", 20489),
    ];
    let sample_median = ("1/4", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.3140989107909008),
        standard_deviation: NiceFloat(0.26663772734754415),
        skewness: NiceFloat(0.7462991500757579),
        excess_kurtosis: NiceFloat(-0.5495263431182642),
    };
    random_rational_range_helper(
        "0",
        "1",
        10,
        1,
        10,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    let values = &[
        "1/3", "1/3", "1/3", "2/5", "1/3", "1/3", "2/5", "4/9", "1/3", "3/7", "2/5", "1/3", "2/5",
        "1/3", "1/3", "5/13", "1/3", "1/3", "2/5", "1/3",
    ];
    let common_values = &[
        ("1/3", 499619),
        ("2/5", 250775),
        ("3/7", 125038),
        ("3/8", 62105),
        ("4/9", 31072),
        ("5/11", 7939),
        ("4/11", 7842),
        ("5/12", 7736),
        ("6/13", 1984),
        ("5/14", 1928),
    ];
    let sample_median = ("5/14", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.3704726750344205),
        standard_deviation: NiceFloat(0.04006148834460584),
        skewness: NiceFloat(0.4071245940784675),
        excess_kurtosis: NiceFloat(-1.377512459092156),
    };
    random_rational_range_helper(
        "1/3",
        "1/2",
        10,
        1,
        1,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    let values = &[
        "1/3", "4/9", "9/19", "5/13", "13/28", "4/9", "5/14", "9/19", "14/33", "8/17", "2/5",
        "14/33", "9/22", "5/12", "3/8", "5/12", "4/9", "14/33", "3/7", "9/23",
    ];
    let common_values = &[
        ("1/3", 90947),
        ("2/5", 82548),
        ("3/7", 74875),
        ("3/8", 68126),
        ("4/9", 62503),
        ("5/12", 51079),
        ("5/14", 42543),
        ("7/15", 38588),
        ("7/16", 35069),
        ("7/18", 28933),
    ];
    let sample_median = ("11/27", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.4061491128373723),
        standard_deviation: NiceFloat(0.04379766958185411),
        skewness: NiceFloat(-0.06037812600265093),
        excess_kurtosis: NiceFloat(-1.1137203231085004),
    };
    random_rational_range_helper(
        "1/3",
        "1/2",
        10,
        1,
        10,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    let values = &[
        "3", "11/4", "3", "3", "3", "3", "3", "3", "11/4", "3", "11/4", "3", "14/5", "3", "11/4",
        "11/4", "14/5", "3", "25/8", "3",
    ];
    let common_values = &[
        ("3", 499797),
        ("11/4", 249730),
        ("14/5", 125404),
        ("17/6", 62239),
        ("20/7", 31351),
        ("23/8", 7872),
        ("25/8", 7748),
        ("25/9", 2675),
        ("28/9", 2671),
        ("26/9", 2667),
    ];
    let sample_median = ("3", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(2.8966573835412754),
        standard_deviation: NiceFloat(0.11355671930910265),
        skewness: NiceFloat(-0.15916553020077587),
        excess_kurtosis: NiceFloat(-1.693978851875664),
    };
    random_rational_range_helper(
        "268876667/98914198",
        "245850922/78256779",
        10,
        1,
        1,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    let values = &[
        "11/4", "25/8", "3", "52/17", "30/11", "79/26", "3", "25/8", "3", "35/12", "50/17",
        "92/31", "3", "46/15", "14/5", "91/31", "57/20", "3", "3", "29/10",
    ];
    let common_values = &[
        ("3", 90795),
        ("11/4", 82681),
        ("14/5", 74997),
        ("17/6", 68015),
        ("20/7", 61965),
        ("25/8", 28541),
        ("23/8", 28350),
        ("29/10", 23236),
        ("31/10", 23204),
        ("37/12", 19335),
    ];
    let sample_median = ("23/8", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(2.9061506622897797),
        standard_deviation: NiceFloat(0.12219692502115352),
        skewness: NiceFloat(0.3585054332079069),
        excess_kurtosis: NiceFloat(-1.1302487991867385),
    };
    random_rational_range_helper(
        "268876667/98914198",
        "245850922/78256779",
        10,
        1,
        10,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    let values = &[
        "1/3", "4/9", "7/19", "5/13", "13/28", "4/9", "5/14", "7/19", "14/33", "8/17", "2/5",
        "16/33", "9/22", "5/12", "3/8", "5/12", "4/9", "14/33", "3/7", "9/23",
    ];
    let common_values = &[
        ("1/3", 90947),
        ("2/5", 82548),
        ("3/7", 74875),
        ("3/8", 68126),
        ("4/9", 62503),
        ("5/12", 51079),
        ("5/14", 42543),
        ("7/15", 38588),
        ("7/16", 35069),
        ("7/18", 28933),
    ];
    let sample_median = ("13/32", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.4056622022625132),
        standard_deviation: NiceFloat(0.04370473197429072),
        skewness: NiceFloat(-0.04788831075607498),
        excess_kurtosis: NiceFloat(-1.1147841677775145),
    };
    random_rational_range_helper(
        "1/3",
        "1/2",
        3,
        1,
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
fn random_rational_range_fail_1() {
    random_rational_range(
        EXAMPLE_SEED,
        Rational::from_unsigneds(1u32, 3),
        Rational::from_unsigneds(2u32, 3),
        10,
        0,
        1,
        1,
    );
}

#[test]
#[should_panic]
fn random_rational_range_fail_2() {
    random_rational_range(
        EXAMPLE_SEED,
        Rational::from_unsigneds(1u32, 3),
        Rational::from_unsigneds(2u32, 3),
        0,
        0,
        1,
        1,
    );
}

#[test]
#[should_panic]
fn random_rational_range_fail_3() {
    random_rational_range(
        EXAMPLE_SEED,
        Rational::from_unsigneds(1u32, 3),
        Rational::from_unsigneds(2u32, 3),
        10,
        1,
        1,
        0,
    );
}

#[test]
#[should_panic]
fn random_rational_range_fail_4() {
    random_rational_range(
        EXAMPLE_SEED,
        Rational::from_unsigneds(1u32, 3),
        Rational::from_unsigneds(2u32, 3),
        10,
        1,
        0,
        0,
    );
}

#[test]
#[should_panic]
fn random_rational_range_fail_5() {
    random_rational_range(
        EXAMPLE_SEED,
        Rational::from_unsigneds(1u32, 3),
        Rational::from_unsigneds(1u32, 3),
        2,
        3,
        1,
        1,
    );
}

#[test]
#[should_panic]
fn random_rational_range_fail_6() {
    random_rational_range(
        EXAMPLE_SEED,
        Rational::from_unsigneds(1u32, 2),
        Rational::from_unsigneds(1u32, 3),
        2,
        3,
        1,
        1,
    );
}
