// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::float::NiceFloat;
use malachite_base::num::random::geometric::geometric_random_nonzero_signeds;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::moments::{
    double_nonzero_truncated_geometric_dist_assertions, CheckedToF64, MomentStats,
};
use std::panic::catch_unwind;

fn geometric_random_nonzero_signeds_helper<T: CheckedToF64 + PrimitiveSigned>(
    um_numerator: u64,
    um_denominator: u64,
    expected_values: &[T],
    expected_common_values: &[(T, usize)],
    expected_abs_mean: NiceFloat<f64>,
    expected_pop_median: (T, Option<T>),
    expected_sample_median: (T, Option<T>),
    expected_pop_moment_stats: MomentStats,
    expected_sample_moment_stats: MomentStats,
) {
    double_nonzero_truncated_geometric_dist_assertions(
        geometric_random_nonzero_signeds::<T>(EXAMPLE_SEED, um_numerator, um_denominator),
        T::MIN,
        T::MAX,
        um_numerator,
        um_denominator,
        expected_values,
        expected_common_values,
        expected_abs_mean,
        expected_pop_median,
        expected_sample_median,
        expected_pop_moment_stats,
        expected_sample_moment_stats,
    );
}

#[test]
fn test_geometric_random_nonzero_signeds() {
    // i64, um = 65 / 64
    let values = &[-1, -1, -1, 1, -1, 1, -1, -1, -1, 1, 1, 1, -1, -1, 1, -1, -1, -1, -1, -1];
    let common_values =
        &[(1, 492630), (-1, 491792), (2, 7695), (-2, 7623), (-3, 130), (3, 128), (4, 1), (-4, 1)];
    let abs_mean = NiceFloat(1.0158400000000025);
    let pop_median = (-1, Some(1));
    let sample_median = (1, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(-0.0),
        standard_deviation: NiceFloat(1.0234076808633008),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.863403263403264),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.0009760000000000123),
        standard_deviation: NiceFloat(1.0237422016660624),
        skewness: NiceFloat(-0.0015925396363277624),
        excess_kurtosis: NiceFloat(-1.8611455716465144),
    };
    geometric_random_nonzero_signeds_helper::<i64>(
        65,
        64,
        values,
        common_values,
        abs_mean,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // i64, um = 12345/10000
    let values = &[-3, -1, -1, 2, -1, 1, -1, -1, -1, 1, 3, 2, -2, -1, 1, -1, -2, -1, -1, -1];
    let common_values = &[
        (1, 405592),
        (-1, 404788),
        (2, 76970),
        (-2, 76657),
        (-3, 14659),
        (3, 14550),
        (-4, 2798),
        (4, 2694),
        (5, 525),
        (-5, 504),
    ];
    let abs_mean = NiceFloat(1.2340139999999655);
    let pop_median = (-1, Some(1));
    let sample_median = (1, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(-0.0),
        standard_deviation: NiceFloat(1.3466553011071538),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-0.5329853284885049),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.0006879999999999908),
        standard_deviation: NiceFloat(1.3459834093804237),
        skewness: NiceFloat(-0.004555831860856982),
        excess_kurtosis: NiceFloat(-0.5314899892072216),
    };
    geometric_random_nonzero_signeds_helper::<i64>(
        12345,
        10000,
        values,
        common_values,
        abs_mean,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // i64, um = 2
    let values = &[-2, -2, -2, 2, -3, 2, -1, -1, -1, 1, 1, 1, -2, -1, 1, -2, -1, -1, -2, -1];
    let common_values = &[
        (1, 250123),
        (-1, 250114),
        (2, 125321),
        (-2, 124779),
        (3, 62655),
        (-3, 62429),
        (4, 31220),
        (-4, 30972),
        (-5, 15670),
        (5, 15610),
    ];
    let abs_mean = NiceFloat(1.9985919999999855);
    let pop_median = (-1, Some(1));
    let sample_median = (1, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(-0.0),
        standard_deviation: NiceFloat(2.449489742783178),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(1.166666666666667),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.0027099999999999395),
        standard_deviation: NiceFloat(2.4473305141166697),
        skewness: NiceFloat(0.003736776778462254),
        excess_kurtosis: NiceFloat(1.1584370930225818),
    };
    geometric_random_nonzero_signeds_helper::<i64>(
        2,
        1,
        values,
        common_values,
        abs_mean,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // i8, um = 64
    let values = &[
        -3, -45, -90, 30, -33, -38, -25, 15, 40, -20, 42, -34, -74, -110, -15, 12, -76, -2, -6, -76,
    ];
    let common_values = &[
        (1, 9067),
        (-1, 9026),
        (-2, 8948),
        (2, 8915),
        (-3, 8837),
        (3, 8719),
        (5, 8571),
        (4, 8530),
        (-4, 8430),
        (6, 8385),
    ];
    let abs_mean = NiceFloat(44.16314300000121);
    let pop_median = (-1, None);
    let sample_median = (-1, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(-0.15631902384873833),
        standard_deviation: NiceFloat(55.4586923219402),
        skewness: NiceFloat(-0.006558922726586753),
        excess_kurtosis: NiceFloat(-0.4026097866721505),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.15813500000000466),
        standard_deviation: NiceFloat(55.40978289585497),
        skewness: NiceFloat(-0.008649136036163145),
        excess_kurtosis: NiceFloat(-0.39765880957469824),
    };
    geometric_random_nonzero_signeds_helper::<i8>(
        64,
        1,
        values,
        common_values,
        abs_mean,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // i8, um = 1000
    let values =
        &[29, -58, 40, 58, 29, 21, -101, 50, 126, -3, 13, 22, 22, 69, 70, 62, -82, -67, 91, 87];
    let common_values = &[
        (3, 4314),
        (-1, 4293),
        (-4, 4243),
        (2, 4240),
        (10, 4233),
        (-12, 4223),
        (-26, 4220),
        (7, 4203),
        (-7, 4198),
        (-19, 4196),
    ];
    let abs_mean = NiceFloat(62.86584099999888);
    let pop_median = (-1, None);
    let sample_median = (-1, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(-0.4706257221678366),
        standard_deviation: NiceFloat(72.86493046408025),
        skewness: NiceFloat(-0.0005545932485799438),
        excess_kurtosis: NiceFloat(-1.1684399210707623),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.5576419999999735),
        standard_deviation: NiceFloat(72.84782263187428),
        skewness: NiceFloat(0.0014314361576379332),
        excess_kurtosis: NiceFloat(-1.1671979175710692),
    };
    geometric_random_nonzero_signeds_helper::<i8>(
        1000,
        1,
        values,
        common_values,
        abs_mean,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );
}

fn geometric_random_nonzero_signeds_fail_helper<T: PrimitiveSigned>() {
    assert_panic!(geometric_random_nonzero_signeds::<T>(EXAMPLE_SEED, 1, 0));
    assert_panic!(geometric_random_nonzero_signeds::<T>(EXAMPLE_SEED, 2, 3));
}

#[test]
fn geometric_random_nonzero_signeds_fail() {
    apply_fn_to_signeds!(geometric_random_nonzero_signeds_fail_helper);
}
