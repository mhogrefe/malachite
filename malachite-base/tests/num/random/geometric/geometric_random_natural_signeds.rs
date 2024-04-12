// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::float::NiceFloat;
use malachite_base::num::random::geometric::geometric_random_natural_signeds;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::moments::{
    truncated_geometric_dist_assertions, CheckedToF64, MomentStats,
};
use std::panic::catch_unwind;

fn geometric_random_natural_signeds_helper<T: CheckedToF64 + PrimitiveSigned>(
    um_numerator: u64,
    um_denominator: u64,
    expected_values: &[T],
    expected_common_values: &[(T, usize)],
    expected_pop_median: (T, Option<T>),
    expected_sample_median: (T, Option<T>),
    expected_pop_moment_stats: MomentStats,
    expected_sample_moment_stats: MomentStats,
) {
    truncated_geometric_dist_assertions(
        geometric_random_natural_signeds::<T>(EXAMPLE_SEED, um_numerator, um_denominator),
        T::ZERO,
        T::MAX,
        um_numerator,
        um_denominator,
        expected_values,
        expected_common_values,
        expected_pop_median,
        expected_sample_median,
        expected_pop_moment_stats,
        expected_sample_moment_stats,
    );
}

#[test]
fn test_geometric_random_natural_signeds() {
    // i64, um = 1 / 64
    let values = &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
    let common_values = &[(0, 984537), (1, 15210), (2, 253)];
    let pop_median = (0, None);
    let sample_median = (0, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(0.015624999999999944),
        standard_deviation: NiceFloat(0.12597277731716458),
        skewness: NiceFloat(8.186292482887549),
        excess_kurtosis: NiceFloat(69.01538461538773),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.015716000000000004),
        standard_deviation: NiceFloat(0.1263923388462427),
        skewness: NiceFloat(8.160460378454996),
        excess_kurtosis: NiceFloat(68.31033619043119),
    };
    geometric_random_natural_signeds_helper::<i64>(
        1,
        64,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // i64, um = 1
    let values = &[1, 0, 0, 3, 4, 4, 1, 0, 0, 1, 0, 0, 2, 2, 0, 0, 1, 0, 3, 1];
    let common_values = &[
        (0, 500085),
        (1, 249510),
        (2, 125328),
        (3, 62428),
        (4, 31280),
        (5, 15676),
        (6, 7853),
        (7, 3994),
        (8, 1932),
        (9, 942),
    ];
    let pop_median = (0, Some(1));
    let sample_median = (0, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(1.0),
        standard_deviation: NiceFloat(std::f64::consts::SQRT_2),
        skewness: NiceFloat(2.1213203435596424),
        excess_kurtosis: NiceFloat(6.5),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1.0006159999999573),
        standard_deviation: NiceFloat(1.414547850547892),
        skewness: NiceFloat(2.1140569440125403),
        excess_kurtosis: NiceFloat(6.4341815215340805),
    };
    geometric_random_natural_signeds_helper::<i64>(
        1,
        1,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // i64, um = 12345/10000
    let values = &[1, 3, 8, 2, 2, 0, 1, 1, 0, 0, 0, 0, 1, 1, 7, 0, 0, 3, 0, 0];
    let common_values = &[
        (0, 446911),
        (1, 246858),
        (2, 137217),
        (3, 75488),
        (4, 41605),
        (5, 23149),
        (6, 12981),
        (7, 7024),
        (8, 3848),
        (9, 2206),
    ];
    let pop_median = (1, None);
    let sample_median = (1, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(1.2344999999999997),
        standard_deviation: NiceFloat(1.6608703290744884),
        skewness: NiceFloat(2.088663960860256),
        excess_kurtosis: NiceFloat(6.362517141396456),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1.2380129999999983),
        standard_deviation: NiceFloat(1.6649767517832197),
        skewness: NiceFloat(2.0942793135700466),
        excess_kurtosis: NiceFloat(6.469904862333731),
    };
    geometric_random_natural_signeds_helper::<i64>(
        12345,
        10000,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // i8, um = 64
    let values = &[53, 90, 125, 70, 18, 41, 109, 5, 96, 93, 33, 18, 89, 6, 17, 1, 1, 2, 123, 6];
    let common_values = &[
        (0, 17832),
        (1, 17626),
        (2, 17520),
        (3, 16976),
        (4, 16636),
        (5, 16395),
        (6, 16282),
        (7, 16231),
        (8, 15823),
        (9, 15338),
    ];
    let pop_median = (36, None);
    let sample_median = (36, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(43.60377071780361),
        standard_deviation: NiceFloat(33.66417209656191),
        skewness: NiceFloat(0.6750025251723596),
        excess_kurtosis: NiceFloat(-0.5593715461161066),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(43.58636099999891),
        standard_deviation: NiceFloat(33.66410490492275),
        skewness: NiceFloat(0.6756300365723926),
        excess_kurtosis: NiceFloat(-0.5577628868956519),
    };
    geometric_random_natural_signeds_helper::<i8>(
        64,
        1,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // i8, um = 1000
    let values =
        &[24, 115, 120, 25, 104, 87, 80, 112, 37, 35, 68, 76, 40, 37, 117, 65, 124, 119, 0, 91];
    let common_values = &[
        (0, 8435),
        (4, 8343),
        (1, 8333),
        (5, 8306),
        (18, 8302),
        (8, 8298),
        (11, 8272),
        (2, 8267),
        (7, 8259),
        (3, 8248),
    ];
    let pop_median = (61, None);
    let sample_median = (61, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(62.13580429363961),
        standard_deviation: NiceFloat(36.93417606567501),
        skewness: NiceFloat(0.044319234592297266),
        excess_kurtosis: NiceFloat(-1.197434568717292),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(62.1484240000004),
        standard_deviation: NiceFloat(36.919219186555864),
        skewness: NiceFloat(0.04244530182494719),
        excess_kurtosis: NiceFloat(-1.196858045789015),
    };
    geometric_random_natural_signeds_helper::<i8>(
        1000,
        1,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );
}

fn geometric_random_natural_signeds_fail_helper<T: PrimitiveSigned>() {
    assert_panic!(geometric_random_natural_signeds::<T>(EXAMPLE_SEED, 0, 1));
    assert_panic!(geometric_random_natural_signeds::<T>(EXAMPLE_SEED, 1, 0));
    assert_panic!(geometric_random_natural_signeds::<T>(
        EXAMPLE_SEED,
        u64::MAX,
        u64::MAX - 1
    ));
}

#[test]
fn geometric_random_natural_signeds_fail() {
    apply_fn_to_signeds!(geometric_random_natural_signeds_fail_helper);
}
