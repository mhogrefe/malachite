// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::float::NiceFloat;
use malachite_base::num::random::geometric::geometric_random_signeds;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::moments::{
    double_truncated_geometric_dist_assertions, CheckedToF64, MomentStats,
};
use std::panic::catch_unwind;

fn geometric_random_signeds_helper<T: CheckedToF64 + PrimitiveSigned>(
    um_numerator: u64,
    um_denominator: u64,
    expected_values: &[T],
    expected_common_values: &[(T, usize)],
    expected_natural_mean: NiceFloat<f64>,
    expected_pop_median: (T, Option<T>),
    expected_sample_median: (T, Option<T>),
    expected_pop_moment_stats: MomentStats,
    expected_sample_moment_stats: MomentStats,
) {
    double_truncated_geometric_dist_assertions(
        geometric_random_signeds::<T>(EXAMPLE_SEED, um_numerator, um_denominator),
        T::MIN,
        T::MAX,
        um_numerator,
        um_denominator,
        expected_values,
        expected_common_values,
        expected_natural_mean,
        expected_pop_median,
        expected_sample_median,
        expected_pop_moment_stats,
        expected_sample_moment_stats,
    );
}

#[test]
fn test_geometric_random_signeds() {
    // i64, um = 1 / 64
    let values = &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let common_values =
        &[(0, 969281), (-1, 15234), (1, 14983), (-2, 267), (2, 230), (3, 3), (-3, 2)];
    let natural_mean = NiceFloat(0.01570299999999922);
    let pop_median = (0, None);
    let sample_median = (0, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(-0.0),
        standard_deviation: NiceFloat(0.17815241017173997),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(34.507692307692416),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.0003219999999999915),
        standard_deviation: NiceFloat(0.17958265107166738),
        skewness: NiceFloat(-0.08440731496142939),
        excess_kurtosis: NiceFloat(34.08776956313593),
    };
    geometric_random_signeds_helper::<i64>(
        1,
        64,
        values,
        common_values,
        natural_mean,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // i64, um = 12345/10000
    let values = &[-1, -1, 0, -3, 1, 0, 0, -4, 0, -6, 1, 4, 0, 1, -2, 4, -3, -3, 1, -3];
    let common_values = &[
        (0, 288130),
        (1, 159322),
        (-1, 158541),
        (-2, 88726),
        (2, 88078),
        (3, 48713),
        (-3, 48584),
        (-4, 26933),
        (4, 26804),
        (-5, 14960),
    ];
    let natural_mean = NiceFloat(1.233504000000006);
    let pop_median = (0, None);
    let sample_median = (0, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(-0.0),
        standard_deviation: NiceFloat(2.3488253447202068),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(3.181258570698229),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.0021609999999999915),
        standard_deviation: NiceFloat(2.3469592749549535),
        skewness: NiceFloat(-0.007388197841520547),
        excess_kurtosis: NiceFloat(3.1771115107088255),
    };
    geometric_random_signeds_helper::<i64>(
        12345,
        10000,
        values,
        common_values,
        natural_mean,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // i64, um = 2
    let values = &[-5, 0, 0, 1, -1, -1, -4, 2, 4, 6, -2, -2, 0, -1, -9, -13, 0, -2, 0, -7];
    let common_values = &[
        (0, 200052),
        (-1, 133433),
        (1, 133168),
        (2, 89079),
        (-2, 88845),
        (3, 59306),
        (-3, 59229),
        (-4, 39537),
        (4, 39457),
        (-5, 26287),
    ];
    let natural_mean = NiceFloat(1.9983939999999663);
    let pop_median = (0, None);
    let sample_median = (0, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(-0.0),
        standard_deviation: NiceFloat(3.4641016151377553),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(3.0833333333333304),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.004637999999999956),
        standard_deviation: NiceFloat(3.462957476064434),
        skewness: NiceFloat(-0.0014668264694796887),
        excess_kurtosis: NiceFloat(3.1056798270038826),
    };
    geometric_random_signeds_helper::<i64>(
        2,
        1,
        values,
        common_values,
        natural_mean,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // i8, um = 64
    let values = &[
        -58, -18, 80, -31, -85, -74, 114, 29, 46, -28, -5, -54, -73, -67, 96, 74, -67, -20, 93, -95,
    ];
    let common_values = &[
        (0, 8916),
        (1, 8897),
        (-1, 8830),
        (3, 8811),
        (-2, 8800),
        (2, 8757),
        (-4, 8491),
        (4, 8459),
        (-3, 8413),
        (-5, 8336),
    ];
    let natural_mean = NiceFloat(43.609158999998506);
    let pop_median = (0, None);
    let sample_median = (0, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(-0.15811030451314936),
        standard_deviation: NiceFloat(55.48244935032745),
        skewness: NiceFloat(-0.006618243685084806),
        excess_kurtosis: NiceFloat(-0.3947891766587368),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.09556800000000087),
        standard_deviation: NiceFloat(55.48675459546383),
        skewness: NiceFloat(-0.007283472641276268),
        excess_kurtosis: NiceFloat(-0.3961004797032963),
    };
    geometric_random_signeds_helper::<i8>(
        64,
        1,
        values,
        common_values,
        natural_mean,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // i8, um = 1000
    let values =
        &[25, -53, 39, 32, 26, 13, -94, 45, 125, 119, 11, 15, 66, 68, 60, -71, -54, 83, 76, -33];
    let common_values = &[
        (-2, 4277),
        (1, 4263),
        (9, 4217),
        (-1, 4213),
        (5, 4207),
        (-4, 4207),
        (0, 4206),
        (-6, 4200),
        (19, 4199),
        (8, 4179),
    ];
    let natural_mean = NiceFloat(62.13860000000173);
    let pop_median = (0, None);
    let sample_median = (0, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(-0.4686977489785049),
        standard_deviation: NiceFloat(72.71434769926142),
        skewness: NiceFloat(-0.0006359350720949359),
        excess_kurtosis: NiceFloat(-1.1608263148064384),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.446604999999992),
        standard_deviation: NiceFloat(72.6977982881445),
        skewness: NiceFloat(-0.0006088230606281416),
        excess_kurtosis: NiceFloat(-1.1593850084998423),
    };
    geometric_random_signeds_helper::<i8>(
        1000,
        1,
        values,
        common_values,
        natural_mean,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );
}

fn geometric_random_signeds_fail_helper<T: PrimitiveSigned>() {
    assert_panic!(geometric_random_signeds::<T>(EXAMPLE_SEED, 0, 1));
    assert_panic!(geometric_random_signeds::<T>(EXAMPLE_SEED, 1, 0));
    assert_panic!(geometric_random_signeds::<T>(
        EXAMPLE_SEED,
        u64::MAX,
        u64::MAX - 1
    ));
}

#[test]
fn geometric_random_signeds_fail() {
    apply_fn_to_signeds!(geometric_random_signeds_fail_helper);
}
