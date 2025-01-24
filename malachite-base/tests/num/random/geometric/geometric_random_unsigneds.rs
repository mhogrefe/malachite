// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::float::NiceFloat;
use malachite_base::num::random::geometric::geometric_random_unsigneds;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::moments::{
    truncated_geometric_dist_assertions, CheckedToF64, MomentStats,
};
use std::panic::catch_unwind;

fn geometric_random_unsigneds_helper<T: CheckedToF64 + PrimitiveUnsigned>(
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
        geometric_random_unsigneds::<T>(EXAMPLE_SEED, um_numerator, um_denominator),
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
fn test_geometric_random_unsigneds() {
    // u64, um = 1 / 64
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
    geometric_random_unsigneds_helper::<u64>(
        1,
        64,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // u64, um = 1
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
    geometric_random_unsigneds_helper::<u64>(
        1,
        1,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // u64, um = 12345/10000
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
    geometric_random_unsigneds_helper::<u64>(
        12345,
        10000,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // u8, um = 64
    let values =
        &[181, 90, 125, 70, 146, 41, 109, 5, 224, 93, 33, 18, 89, 6, 17, 1, 129, 2, 123, 134];
    let common_values = &[
        (0, 15734),
        (1, 15591),
        (2, 15469),
        (3, 14917),
        (4, 14672),
        (5, 14340),
        (6, 14329),
        (7, 14274),
        (8, 13841),
        (9, 13586),
    ];
    let pop_median = (43, None);
    let sample_median = (43, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(59.070796734994865),
        standard_deviation: NiceFloat(53.608086324088696),
        skewness: NiceFloat(1.2637908431790819),
        excess_kurtosis: NiceFloat(1.1650093560615717),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(59.03148100000193),
        standard_deviation: NiceFloat(53.60145056863821),
        skewness: NiceFloat(1.2649824077121026),
        excess_kurtosis: NiceFloat(1.1705093182268955),
    };
    geometric_random_unsigneds_helper::<u8>(
        64,
        1,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // u8, um = 1000
    let values = &[
        24, 115, 248, 25, 104, 215, 208, 240, 37, 35, 196, 204, 40, 37, 245, 193, 252, 119, 0, 219,
    ];
    let common_values = &[
        (18, 4482),
        (11, 4469),
        (6, 4431),
        (25, 4426),
        (0, 4424),
        (3, 4415),
        (5, 4400),
        (8, 4400),
        (20, 4396),
        (10, 4386),
    ];
    let pop_median = (119, None);
    let sample_median = (119, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(122.04742583121647),
        standard_deviation: NiceFloat(73.7795264500733),
        skewness: NiceFloat(0.08861336552406392),
        excess_kurtosis: NiceFloat(-1.1891899016227028),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(122.12244000000315),
        standard_deviation: NiceFloat(73.78371977939058),
        skewness: NiceFloat(0.08799741904573245),
        excess_kurtosis: NiceFloat(-1.1900813344800518),
    };
    geometric_random_unsigneds_helper::<u8>(
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

fn geometric_random_unsigneds_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(geometric_random_unsigneds::<T>(EXAMPLE_SEED, 0, 1));
    assert_panic!(geometric_random_unsigneds::<T>(EXAMPLE_SEED, 1, 0));
    assert_panic!(geometric_random_unsigneds::<T>(
        EXAMPLE_SEED,
        u64::MAX,
        u64::MAX - 1
    ));
}

#[test]
fn geometric_random_unsigneds_fail() {
    apply_fn_to_unsigneds!(geometric_random_unsigneds_fail_helper);
}
