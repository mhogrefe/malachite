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
use malachite_nz::integer::random::striped_random_nonzero_integers;
use malachite_nz::test_util::integer::random::random_integers_helper_helper;

fn striped_random_nonzero_integers_helper(
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_sample_median: (&str, Option<&str>),
    expected_sample_moment_stats: MomentStats,
) {
    random_integers_helper_helper(
        striped_random_nonzero_integers(
            EXAMPLE_SEED,
            mean_stripe_numerator,
            mean_stripe_denominator,
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
fn test_striped_random_nonzero_integers() {
    // mean bits = 65/64
    let values = &[
        "1", "1", "1", "-1", "-1", "-1", "1", "-1", "-1", "1", "1", "1", "-1", "-1", "-1", "-1",
        "1", "1", "-1", "-1",
    ];
    let common_values = &[
        ("1", 492842),
        ("-1", 491818),
        ("2", 3848),
        ("3", 3791),
        ("-3", 3753),
        ("-2", 3709),
        ("7", 50),
        ("4", 49),
        ("-4", 41),
        ("-7", 36),
    ];
    let sample_median = ("1", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.0015040000000000353),
        standard_deviation: NiceFloat(1.0443710206123127),
        skewness: NiceFloat(0.0008346150701385402),
        excess_kurtosis: NiceFloat(-1.2794877665824687),
    };
    striped_random_nonzero_integers_helper(
        4,
        1,
        65,
        64,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    // mean bits = 2
    let values = &[
        "4", "1", "4", "-8", "-1", "-1", "1", "-1", "-2", "7", "7", "6", "-1", "-1", "-3", "-14",
        "1", "4", "-8", "-1",
    ];
    let common_values = &[
        ("1", 249934),
        ("-1", 249480),
        ("3", 62605),
        ("-3", 62544),
        ("2", 62495),
        ("-2", 62282),
        ("4", 23545),
        ("-7", 23428),
        ("7", 23343),
        ("-4", 23304),
    ];
    let sample_median = ("1", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.9232179999999961),
        standard_deviation: NiceFloat(942.1853934867996),
        skewness: NiceFloat(-30.544317259845204),
        excess_kurtosis: NiceFloat(179726.72807613286),
    };
    striped_random_nonzero_integers_helper(
        4,
        1,
        2,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    // mean bits = 32
    let values = &[
        "4",
        "268435456",
        "84405977732342160290572740160760316144",
        "-133169152",
        "-131064",
        "-2251834173421823",
        "1577058304",
        "-126100789566374399",
        "-76",
        "270335",
        "33554431",
        "262144",
        "-4398046511104",
        "-20352",
        "-1023",
        "-1",
        "63",
        "72057589742960640",
        "-1",
        "-8388607",
    ];
    let common_values = &[
        ("1", 15709),
        ("-1", 15677),
        ("-3", 7646),
        ("-2", 7603),
        ("3", 7564),
        ("2", 7494),
        ("4", 6925),
        ("7", 6916),
        ("-7", 6903),
        ("-4", 6802),
    ];
    let sample_median = ("1", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-2.5217288846207e111),
        standard_deviation: NiceFloat(2.5217283396166554e114),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    striped_random_nonzero_integers_helper(
        16,
        1,
        32,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    // mean bits = 64
    let values = &[
        "67108864",
        "47890485651710580317658107747553101683567604294221824",
        "512",
        "-311675034947891977256960",
        "-309485009821345068724780928",
        "-1179647",
        "1",
        "-131071",
        "-17179869184",
        "1056767",
        "273820942303",
        "137438952504",
        "-536870912",
        "-36749372959343247360",
        "-4722366482869645217791",
        "-786432",
        "1023",
        "8388608",
        "-274911460352",
        "-24575",
    ];
    let common_values = &[
        ("1", 7878),
        ("-1", 7830),
        ("-3", 3940),
        ("-2", 3893),
        ("2", 3885),
        ("3", 3806),
        ("4", 3707),
        ("-7", 3694),
        ("-4", 3689),
        ("7", 3608),
    ];
    let sample_median = ("1", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-1.7976931348623025e302),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    striped_random_nonzero_integers_helper(
        32,
        1,
        64,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
}

#[test]
#[should_panic]
fn striped_random_nonzero_integers_fail_1() {
    striped_random_nonzero_integers(EXAMPLE_SEED, 1, 0, 4, 1);
}

#[test]
#[should_panic]
fn striped_random_nonzero_integers_fail_2() {
    striped_random_nonzero_integers(EXAMPLE_SEED, 2, 3, 4, 1);
}

#[test]
#[should_panic]
fn striped_random_nonzero_integers_fail_3() {
    striped_random_nonzero_integers(EXAMPLE_SEED, 4, 1, 1, 0);
}

#[test]
#[should_panic]
fn striped_random_nonzero_integers_fail_4() {
    striped_random_nonzero_integers(EXAMPLE_SEED, 4, 1, 2, 3);
}
