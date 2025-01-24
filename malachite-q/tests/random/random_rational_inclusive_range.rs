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
use malachite_q::random::random_rational_inclusive_range;
use malachite_q::test_util::random::random_rationals_helper_helper;
use malachite_q::Rational;
use std::str::FromStr;

fn random_rational_inclusive_range_helper(
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
        random_rational_inclusive_range(
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
fn test_random_rational_inclusive_range() {
    let values = &[
        "1/3", "1/3", "1/3", "1/3", "1/3", "1/3", "1/3", "1/3", "1/3", "1/3", "1/3", "1/3", "1/3",
        "1/3", "1/3", "1/3", "1/3", "1/3", "1/3", "1/3",
    ];
    let common_values = &[("1/3", 1000000)];
    let sample_median = ("1/3", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.3333333333333333),
        standard_deviation: NiceFloat(0.0),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_rational_inclusive_range_helper(
        "1/3",
        "1/3",
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
        "0", "1/2", "0", "1", "0", "0", "1", "0", "1/2", "1", "1/2", "1", "1/3", "0", "1/2", "1/2",
        "1/3", "1", "1/6", "1",
    ];
    let common_values = &[
        ("0", 262055),
        ("1/2", 249730),
        ("1", 237742),
        ("1/3", 86236),
        ("1/4", 42606),
        ("2/3", 39168),
        ("3/4", 19633),
        ("1/5", 13421),
        ("1/6", 12245),
        ("3/5", 6206),
    ];
    let sample_median = ("1/2", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.46651895490175377),
        standard_deviation: NiceFloat(0.3690116747518855),
        skewness: NiceFloat(0.18424550073715937),
        excess_kurtosis: NiceFloat(-1.2512329853066777),
    };
    random_rational_inclusive_range_helper(
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
        "1/2", "1/6", "0", "11/15", "1/9", "1/24", "0", "1/6", "1", "7/10", "14/15", "4/29", "0",
        "3/13", "1/3", "10/29", "17/18", "0", "1", "1/8",
    ];
    let common_values = &[
        ("1/2", 82681),
        ("1/3", 51495),
        ("0", 47408),
        ("1/4", 46547),
        ("1/6", 44587),
        ("1", 43387),
        ("1/5", 26408),
        ("1/8", 24875),
        ("2/3", 23502),
        ("3/4", 21468),
    ];
    let sample_median = ("4/15", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.3574859107909007),
        standard_deviation: NiceFloat(0.29213805168114404),
        skewness: NiceFloat(0.7340743758337255),
        excess_kurtosis: NiceFloat(-0.6079435842676566),
    };
    random_rational_inclusive_range_helper(
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
        "1/2", "1/3", "1/2", "1/2", "1/2", "1/2", "1/2", "1/2", "1/3", "1/2", "1/3", "1/2", "2/5",
        "1/2", "1/3", "1/3", "2/5", "1/2", "4/9", "1/2",
    ];
    let common_values = &[
        ("1/2", 499797),
        ("1/3", 249730),
        ("2/5", 125404),
        ("3/7", 62239),
        ("3/8", 31351),
        ("4/9", 15620),
        ("5/11", 4024),
        ("4/11", 3989),
        ("5/12", 3856),
        ("6/13", 1003),
    ];
    let sample_median = ("7/15", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.43521234958930916),
        standard_deviation: NiceFloat(0.0706863774046333),
        skewness: NiceFloat(-0.40352004419103704),
        excess_kurtosis: NiceFloat(-1.5393678543556049),
    };
    random_rational_inclusive_range_helper(
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
        "1/3", "4/9", "1/2", "9/19", "5/13", "13/28", "1/2", "4/9", "1/2", "5/14", "9/19", "14/33",
        "1/2", "8/17", "2/5", "14/33", "9/22", "1/2", "1/2", "5/12",
    ];
    let common_values = &[
        ("1/2", 90795),
        ("1/3", 82681),
        ("2/5", 74997),
        ("3/7", 68015),
        ("3/8", 61965),
        ("4/9", 56891),
        ("5/12", 46440),
        ("5/14", 38639),
        ("7/15", 35083),
        ("7/16", 31926),
    ];
    let sample_median = ("5/12", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.4146806356838901),
        standard_deviation: NiceFloat(0.04970657076728507),
        skewness: NiceFloat(0.05385178173988354),
        excess_kurtosis: NiceFloat(-1.0272575619229356),
    };
    random_rational_inclusive_range_helper(
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
    random_rational_inclusive_range_helper(
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
    random_rational_inclusive_range_helper(
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
}

#[test]
#[should_panic]
fn random_rational_inclusive_range_fail_1() {
    random_rational_inclusive_range(
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
fn random_rational_inclusive_range_fail_2() {
    random_rational_inclusive_range(
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
fn random_rational_inclusive_range_fail_3() {
    random_rational_inclusive_range(
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
fn random_rational_inclusive_range_fail_4() {
    random_rational_inclusive_range(
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
fn random_rational_inclusive_range_fail_5() {
    random_rational_inclusive_range(
        EXAMPLE_SEED,
        Rational::from_unsigneds(1u32, 2),
        Rational::from_unsigneds(1u32, 3),
        2,
        3,
        1,
        1,
    );
}
