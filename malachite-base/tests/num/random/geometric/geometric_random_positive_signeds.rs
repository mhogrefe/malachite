// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::float::NiceFloat;
use malachite_base::num::random::geometric::geometric_random_positive_signeds;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::moments::{
    truncated_geometric_dist_assertions, CheckedToF64, MomentStats,
};
use std::panic::catch_unwind;

fn geometric_random_positive_signeds_helper<T: CheckedToF64 + PrimitiveSigned>(
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
        geometric_random_positive_signeds::<T>(EXAMPLE_SEED, um_numerator, um_denominator),
        T::ONE,
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
fn test_geometric_random_positive_signeds() {
    // i64, um = 65 / 64
    let values = &[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2];
    let common_values = &[(1, 984537), (2, 15210), (3, 253)];
    let pop_median = (1, None);
    let sample_median = (1, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(1.015625),
        standard_deviation: NiceFloat(0.12597277731716458),
        skewness: NiceFloat(8.186292482887549),
        excess_kurtosis: NiceFloat(69.01538461538773),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1.0157160000000027),
        standard_deviation: NiceFloat(0.12639233884624257),
        skewness: NiceFloat(8.160460378454992),
        excess_kurtosis: NiceFloat(68.31033619043166),
    };
    geometric_random_positive_signeds_helper::<i64>(
        65,
        64,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // i64, um = 12345/10000
    let values = &[1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 2, 2, 1, 1, 1, 1];
    let common_values = &[
        (1, 809395),
        (2, 154707),
        (3, 29037),
        (4, 5577),
        (5, 1053),
        (6, 185),
        (7, 40),
        (8, 5),
        (9, 1),
    ];
    let pop_median = (1, None);
    let sample_median = (1, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(1.2345),
        standard_deviation: NiceFloat(0.538042981554448),
        skewness: NiceFloat(2.730265146765687),
        excess_kurtosis: NiceFloat(9.45434777164341),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1.2349320000000084),
        standard_deviation: NiceFloat(0.5376590410783139),
        skewness: NiceFloat(2.716807176148366),
        excess_kurtosis: NiceFloat(9.308727522629948),
    };
    geometric_random_positive_signeds_helper::<i64>(
        12345,
        10000,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // i64, um = 2
    let values = &[2, 1, 1, 4, 5, 5, 2, 1, 1, 2, 1, 1, 3, 3, 1, 1, 2, 1, 4, 2];
    let common_values = &[
        (1, 500085),
        (2, 249510),
        (3, 125328),
        (4, 62428),
        (5, 31280),
        (6, 15676),
        (7, 7853),
        (8, 3994),
        (9, 1932),
        (10, 942),
    ];
    let pop_median = (1, Some(2));
    let sample_median = (1, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(2.0),
        standard_deviation: NiceFloat(std::f64::consts::SQRT_2),
        skewness: NiceFloat(2.1213203435596424),
        excess_kurtosis: NiceFloat(6.5),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(2.0006159999998947),
        standard_deviation: NiceFloat(1.414547850547892),
        skewness: NiceFloat(2.114056944012543),
        excess_kurtosis: NiceFloat(6.4341815215340645),
    };
    geometric_random_positive_signeds_helper::<i64>(
        2,
        1,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // i8, um = 64
    let values = &[53, 56, 71, 20, 113, 87, 55, 44, 4, 17, 44, 36, 5, 25, 55, 9, 114, 24, 15, 76];
    let common_values = &[
        (1, 18322),
        (2, 17831),
        (3, 17675),
        (4, 17346),
        (5, 17077),
        (6, 16664),
        (7, 16584),
        (8, 16201),
        (9, 15917),
        (10, 15785),
    ];
    let pop_median = (36, None);
    let sample_median = (36, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(44.12320959250907),
        standard_deviation: NiceFloat(33.35507189836844),
        skewness: NiceFloat(0.6800692970056713),
        excess_kurtosis: NiceFloat(-0.5495345339552125),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(44.11083399999909),
        standard_deviation: NiceFloat(33.35932545284804),
        skewness: NiceFloat(0.6778762972570164),
        excess_kurtosis: NiceFloat(-0.5537033314075828),
    };
    geometric_random_positive_signeds_helper::<i8>(
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
        &[29, 119, 12, 28, 113, 91, 84, 117, 46, 36, 71, 83, 43, 50, 127, 69, 5, 124, 10, 95];
    let common_values = &[
        (3, 8464),
        (11, 8429),
        (9, 8411),
        (26, 8407),
        (6, 8397),
        (12, 8387),
        (2, 8383),
        (8, 8363),
        (1, 8326),
        (14, 8313),
    ];
    let pop_median = (62, None);
    let sample_median = (62, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(62.65568926722266),
        standard_deviation: NiceFloat(36.64581386503865),
        skewness: NiceFloat(0.04401709622102498),
        excess_kurtosis: NiceFloat(-1.1974732777225838),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(62.68774500000057),
        standard_deviation: NiceFloat(36.63525108357442),
        skewness: NiceFloat(0.041814478066591894),
        excess_kurtosis: NiceFloat(-1.1973932602500912),
    };
    geometric_random_positive_signeds_helper::<i8>(
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

fn geometric_random_positive_signeds_fail_helper<T: PrimitiveSigned>() {
    assert_panic!(geometric_random_positive_signeds::<T>(EXAMPLE_SEED, 1, 0));
    assert_panic!(geometric_random_positive_signeds::<T>(EXAMPLE_SEED, 2, 3));
}

#[test]
fn geometric_random_positive_signeds_fail() {
    apply_fn_to_signeds!(geometric_random_positive_signeds_fail_helper);
}
