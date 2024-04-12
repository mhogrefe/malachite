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
use malachite_nz::integer::random::striped_random_integer_range;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::integer::random::random_integers_helper_helper;
use std::str::FromStr;

fn striped_random_integer_range_helper(
    a: &str,
    b: &str,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_sample_median: (&str, Option<&str>),
    expected_sample_moment_stats: MomentStats,
) {
    random_integers_helper_helper(
        striped_random_integer_range(
            EXAMPLE_SEED,
            Integer::from_str(a).unwrap(),
            Integer::from_str(b).unwrap(),
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
fn test_striped_random_integer_range() {
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
    striped_random_integer_range_helper(
        "1990",
        "2022",
        4,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    let values = &[
        "-1990", "-1991", "-1991", "-2006", "-1996", "-1991", "-2020", "-1991", "-1990", "-2014",
        "-1990", "-2020", "-1991", "-1990", "-2020", "-1991", "-2015", "-2020", "-2016", "-2016",
    ];
    let common_values = &[
        ("-1990", 141061),
        ("-1991", 140282),
        ("-2016", 140025),
        ("-2021", 125104),
        ("-2020", 124770),
        ("-2017", 47126),
        ("-2019", 46880),
        ("-1999", 39864),
        ("-2015", 39502),
        ("-2018", 15866),
    ];
    let sample_median = ("-2015", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-2007.63283599996),
        standard_deviation: NiceFloat(12.714969927906306),
        skewness: NiceFloat(0.39110989081904446),
        excess_kurtosis: NiceFloat(-1.6497443674417989),
    };
    striped_random_integer_range_helper(
        "-2021",
        "-1989",
        4,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    let values = &[
        "-1792", "-1766", "1551", "111", "-1567", "16", "1985", "142", "1935", "992", "3", "-510",
        "1984", "-624", "-30", "-1542", "-1", "-2017", "-1927", "-7",
    ];
    let common_values = &[
        ("1990", 29612),
        ("1984", 22443),
        ("-1", 19100),
        ("-2016", 16609),
        ("-2021", 14942),
        ("-2020", 14868),
        ("0", 13951),
        ("1985", 7476),
        ("1987", 7411),
        ("1988", 7362),
    ];
    let sample_median = ("-1", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-2.049800999999855),
        standard_deviation: NiceFloat(1269.7724478413352),
        skewness: NiceFloat(-0.007894098306828256),
        excess_kurtosis: NiceFloat(-1.0358454536315198),
    };
    striped_random_integer_range_helper(
        "-2021",
        "1991",
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
fn striped_random_integer_range_fail_1() {
    striped_random_integer_range(
        EXAMPLE_SEED,
        Integer::from(10u32),
        Integer::from(100u32),
        1,
        0,
    );
}

#[test]
#[should_panic]
fn striped_random_integer_range_fail_2() {
    striped_random_integer_range(
        EXAMPLE_SEED,
        Integer::from(10u32),
        Integer::from(100u32),
        1,
        1,
    );
}

#[test]
#[should_panic]
fn striped_random_integer_range_fail_3() {
    striped_random_integer_range(
        EXAMPLE_SEED,
        Integer::from(10u32),
        Integer::from(9u32),
        10,
        1,
    );
}
