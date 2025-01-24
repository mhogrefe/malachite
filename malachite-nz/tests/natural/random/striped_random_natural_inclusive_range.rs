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
use malachite_nz::natural::random::striped_random_natural_inclusive_range;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::natural::random::random_naturals_helper_helper;
use std::str::FromStr;

fn striped_random_natural_inclusive_range_helper(
    a: &str,
    b: &str,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_sample_median: (&str, Option<&str>),
    expected_sample_moment_stats: MomentStats,
) {
    random_naturals_helper_helper(
        striped_random_natural_inclusive_range(
            EXAMPLE_SEED,
            Natural::from_str(a).unwrap(),
            Natural::from_str(b).unwrap(),
            mean_stripe_numerator,
            mean_stripe_denominator,
        ),
        expected_values,
        expected_common_values,
        expected_sample_median,
        expected_sample_moment_stats,
    );
}

#[test]
fn test_striped_random_natural_inclusive_range() {
    let values = &["0"; 20];
    let common_values = &[("0", 1000000)];
    let sample_median = ("0", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.0),
        standard_deviation: NiceFloat(0.0),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    striped_random_natural_inclusive_range_helper(
        "0",
        "0",
        4,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    let values = &[
        "1990", "1991", "1991", "2006", "1996", "1991", "2020", "1991", "1990", "2014", "1990",
        "2020", "1991", "1990", "2020", "1991", "2015", "2020", "2016", "2016",
    ];
    let common_values = &[
        ("1990", 141061),
        ("1991", 140282),
        ("2016", 140025),
        ("2021", 125104),
        ("2020", 124770),
        ("2017", 47126),
        ("2019", 46880),
        ("1999", 39864),
        ("2015", 39502),
        ("2018", 15866),
    ];
    let sample_median = ("2015", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(2007.63283599996),
        standard_deviation: NiceFloat(12.714969927906306),
        skewness: NiceFloat(-0.39110989081904446),
        excess_kurtosis: NiceFloat(-1.6497443674417989),
    };
    striped_random_natural_inclusive_range_helper(
        "1990",
        "2021",
        4,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    let values = &[
        "1000425", "1036272", "1007600", "1999887", "1018367", "1000191", "1048387", "1049087",
        "1007631", "1015792", "1971832", "1046770", "1023876", "1966085", "1838648", "1017728",
        "1046662", "1998848", "1613817", "1982463",
    ];
    let common_values = &[
        ("2000000", 26405),
        ("1999872", 3441),
        ("1048575", 3413),
        ("1015807", 3052),
        ("1998848", 2966),
        ("1000447", 2758),
        ("1000063", 2381),
        ("1000000", 2380),
        ("1966080", 1441),
        ("1999935", 1251),
    ];
    let sample_median = ("1048576", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1371348.0426910813),
        standard_deviation: NiceFloat(417874.3987798391),
        skewness: NiceFloat(0.5348808065018619),
        excess_kurtosis: NiceFloat(-1.5362497902865004),
    };
    striped_random_natural_inclusive_range_helper(
        "1000000",
        "2000000",
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
fn striped_random_natural_inclusive_range_fail_1() {
    striped_random_natural_inclusive_range(
        EXAMPLE_SEED,
        Natural::from(10u32),
        Natural::from(100u32),
        1,
        0,
    );
}

#[test]
#[should_panic]
fn striped_random_natural_inclusive_range_fail_2() {
    striped_random_natural_inclusive_range(
        EXAMPLE_SEED,
        Natural::from(10u32),
        Natural::from(100u32),
        1,
        1,
    );
}

#[test]
#[should_panic]
fn striped_random_natural_inclusive_range_fail_3() {
    striped_random_natural_inclusive_range(
        EXAMPLE_SEED,
        Natural::from(10u32),
        Natural::from(9u32),
        10,
        1,
    );
}
