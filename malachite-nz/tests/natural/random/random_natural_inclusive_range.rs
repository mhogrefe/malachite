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
use malachite_nz::natural::random::random_natural_inclusive_range;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::natural::random::random_naturals_helper_helper;
use std::str::FromStr;

fn random_natural_inclusive_range_helper(
    a: &str,
    b: &str,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_sample_median: (&str, Option<&str>),
    expected_sample_moment_stats: MomentStats,
) {
    random_naturals_helper_helper(
        random_natural_inclusive_range(
            EXAMPLE_SEED,
            Natural::from_str(a).unwrap(),
            Natural::from_str(b).unwrap(),
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
fn test_random_natural_inclusive_range() {
    let values = &["0"; 20];
    let common_values = &[("0", 1000000)];
    let sample_median = ("0", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.0),
        standard_deviation: NiceFloat(0.0),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_natural_inclusive_range_helper(
        "0",
        "0",
        1,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    let values = &[
        "14", "8", "10", "1", "1", "0", "1", "0", "8", "66", "1", "3", "2", "5", "1", "6", "1",
        "2", "14", "1",
    ];
    let common_values = &[
        ("0", 240959),
        ("1", 191819),
        ("2", 76868),
        ("3", 76523),
        ("7", 30894),
        ("6", 30881),
        ("4", 30635),
        ("5", 30631),
        ("14", 12472),
        ("15", 12457),
    ];
    let sample_median = ("2", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(11.335956999999699),
        standard_deviation: NiceFloat(20.565945026967327),
        skewness: NiceFloat(2.486687831499969),
        excess_kurtosis: NiceFloat(5.6402145781222615),
    };
    random_natural_inclusive_range_helper(
        "0",
        "99",
        4,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    let values = &[
        "14", "1", "1", "4", "6", "1", "6", "66", "10", "2", "4", "1", "7", "7", "2", "17", "3",
        "2", "2", "97",
    ];
    let common_values = &[
        ("1", 288484),
        ("3", 108475),
        ("2", 108256),
        ("7", 40848),
        ("5", 40475),
        ("4", 40324),
        ("6", 40213),
        ("12", 15378),
        ("14", 15279),
        ("8", 15268),
    ];
    let sample_median = ("3", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(12.712097999999987),
        standard_deviation: NiceFloat(20.543779459614104),
        skewness: NiceFloat(2.4002783415844497),
        excess_kurtosis: NiceFloat(5.233152563653805),
    };
    random_natural_inclusive_range_helper(
        "1",
        "99",
        4,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    let values = &[
        "1987", "1993", "1907", "1984", "1927", "1946", "1993", "1922", "1986", "1901", "1907",
        "1929", "1925", "1956", "1997", "1938", "1970", "1906", "1955", "1929",
    ];
    let common_values = &[
        ("1945", 10146),
        ("1987", 10096),
        ("1991", 10094),
        ("1982", 10056),
        ("1900", 10042),
        ("1973", 10033),
        ("1959", 10029),
        ("1967", 10026),
        ("1974", 10024),
        ("1946", 10023),
    ];
    let sample_median = ("1950", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1949.98699899998),
        standard_deviation: NiceFloat(29.18007161489914),
        skewness: NiceFloat(0.000791345316435403),
        excess_kurtosis: NiceFloat(-1.2020606886458867),
    };
    random_natural_inclusive_range_helper(
        "1900",
        "2000",
        11,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    let values = &[
        "1206",
        "2200",
        "32434",
        "2526",
        "428799138",
        "12119996",
        "2744",
        "2065487",
        "5283",
        "105634",
        "63523217",
        "18250435",
        "42716754",
        "7992",
        "1720",
        "481774",
        "7143",
        "2445",
        "6806",
        "297908430",
    ];
    let common_values = &[
        ("1022", 3070),
        ("1006", 3057),
        ("1001", 3047),
        ("1014", 3040),
        ("1020", 3038),
        ("1003", 3015),
        ("1010", 3010),
        ("1012", 3008),
        ("1018", 3008),
        ("1008", 3000),
    ];
    let sample_median = ("148810", Some("148811"));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(48716724.95089779),
        standard_deviation: NiceFloat(150453248.85048282),
        skewness: NiceFloat(4.049374838772574),
        excess_kurtosis: NiceFloat(17.011313559369025),
    };
    random_natural_inclusive_range_helper(
        "1000",
        "999999999",
        32,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
}

#[test]
#[should_panic]
fn random_natural_inclusive_range_fail_1() {
    random_natural_inclusive_range(
        EXAMPLE_SEED,
        Natural::from(10u32),
        Natural::from(100u32),
        1,
        0,
    );
}

#[test]
#[should_panic]
fn random_natural_inclusive_range_fail_2() {
    random_natural_inclusive_range(
        EXAMPLE_SEED,
        Natural::from(10u32),
        Natural::from(100u32),
        4,
        1,
    );
}

#[test]
#[should_panic]
fn random_natural_inclusive_range_fail_3() {
    random_natural_inclusive_range(
        EXAMPLE_SEED,
        Natural::from(10u32),
        Natural::from(9u32),
        10,
        1,
    );
}
