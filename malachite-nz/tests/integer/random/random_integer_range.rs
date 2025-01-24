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
use malachite_nz::integer::random::random_integer_range;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::integer::random::random_integers_helper_helper;
use std::str::FromStr;

fn random_integer_range_helper(
    a: &str,
    b: &str,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_sample_median: (&str, Option<&str>),
    expected_sample_moment_stats: MomentStats,
) {
    random_integers_helper_helper(
        random_integer_range(
            EXAMPLE_SEED,
            Integer::from_str(a).unwrap(),
            Integer::from_str(b).unwrap(),
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
fn test_random_integer_range() {
    let values = &["0"; 20];
    let common_values = &[("0", 1000000)];
    let sample_median = ("0", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.0),
        standard_deviation: NiceFloat(0.0),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_integer_range_helper(
        "0",
        "1",
        1,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    let values = &[
        "1", "0", "1", "-1", "-1", "-4", "-2", "-2", "-1", "-1", "0", "1", "2", "-4", "0", "1",
        "0", "0", "1", "0",
    ];
    let common_values = &[
        ("0", 284116),
        ("1", 189679),
        ("-1", 189332),
        ("-4", 84500),
        ("3", 63397),
        ("2", 63173),
        ("-3", 62961),
        ("-2", 62842),
    ];
    let sample_median = ("0", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.3356830000000005),
        standard_deviation: NiceFloat(1.8054398863225456),
        skewness: NiceFloat(-0.35221934475763134),
        excess_kurtosis: NiceFloat(-0.2458978296075136),
    };
    random_integer_range_helper(
        "-4",
        "4",
        2,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    let values = &[
        "1023", "1022", "1023", "1023", "1023", "1022", "1023", "1024", "1024", "1025", "1025",
        "1022", "1023", "1023", "1023", "1022", "1025", "1024", "1024", "1024",
    ];
    let common_values = &[("1023", 300404), ("1022", 299811), ("1025", 200144), ("1024", 199641)];
    let sample_median = ("1023", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1023.3001179999817),
        standard_deviation: NiceFloat(1.0999810889439465),
        skewness: NiceFloat(0.2889412070926685),
        excess_kurtosis: NiceFloat(-1.2389995110068848),
    };
    random_integer_range_helper(
        "1022",
        "1026",
        12,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    let values = &[
        "-1023", "-1023", "-1023", "-1023", "-1023", "-1023", "-1023", "-1025", "-1024", "-1025",
        "-1024", "-1023", "-1023", "-1023", "-1023", "-1023", "-1025", "-1026", "-1024", "-1024",
    ];
    let common_values =
        &[("-1023", 600215), ("-1024", 133294), ("-1026", 133261), ("-1025", 133230)];
    let sample_median = ("-1023", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-1023.7995370000281),
        standard_deviation: NiceFloat(1.1073864781257785),
        skewness: NiceFloat(-0.990171399672332),
        excess_kurtosis: NiceFloat(-0.5751529612720772),
    };
    random_integer_range_helper(
        "-1026",
        "-1022",
        12,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    let values = &[
        "2", "152", "1", "0", "-62", "5282", "0", "28", "-4", "-79", "-11", "2", "1", "-1", "82",
        "-1", "696", "-6", "-39", "1421",
    ];
    let common_values = &[
        ("0", 118542),
        ("1", 95287),
        ("-1", 95269),
        ("-3", 38248),
        ("3", 38202),
        ("2", 38150),
        ("-2", 38078),
        ("-4", 15429),
        ("7", 15423),
        ("-5", 15382),
    ];
    let sample_median = ("0", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(130.2935679999988),
        standard_deviation: NiceFloat(901.4375229872913),
        skewness: NiceFloat(7.858028656993447),
        excess_kurtosis: NiceFloat(67.9560213744922),
    };
    random_integer_range_helper(
        "-1000",
        "10000",
        4,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
}

#[test]
#[should_panic]
fn random_integer_range_fail_1() {
    random_integer_range(EXAMPLE_SEED, Integer::from(-100), Integer::from(-10), 1, 0);
}

#[test]
#[should_panic]
fn random_integer_range_fail_2() {
    random_integer_range(EXAMPLE_SEED, Integer::from(-100), Integer::from(-10), 4, 1);
}

#[test]
#[should_panic]
fn random_integer_range_fail_3() {
    random_integer_range(EXAMPLE_SEED, Integer::from(-9), Integer::from(-10), 10, 1);
}

#[test]
#[should_panic]
fn random_integer_range_fail_4() {
    random_integer_range(EXAMPLE_SEED, Integer::from(10), Integer::from(10), 10, 1);
}
