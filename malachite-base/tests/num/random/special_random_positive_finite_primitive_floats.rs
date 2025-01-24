// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::float::NiceFloat;
use malachite_base::num::random::special_random_positive_finite_primitive_floats;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::num::random::special_random_primitive_floats_helper_helper;
use malachite_base::test_util::stats::moments::{CheckedToF64, MomentStats};
use std::panic::catch_unwind;

fn special_random_positive_finite_primitive_floats_helper<T: CheckedToF64 + PrimitiveFloat>(
    mean_exponent_numerator: u64,
    mean_exponent_denominator: u64,
    mean_precision_numerator: u64,
    mean_precision_denominator: u64,
    expected_values: &[T],
    expected_common_values: &[(T, usize)],
    expected_median: (T, Option<T>),
    expected_moment_stats: MomentStats,
) {
    special_random_primitive_floats_helper_helper(
        special_random_positive_finite_primitive_floats::<T>(
            EXAMPLE_SEED,
            mean_exponent_numerator,
            mean_exponent_denominator,
            mean_precision_numerator,
            mean_precision_denominator,
        ),
        expected_values,
        expected_common_values,
        expected_median,
        expected_moment_stats,
    );
}

#[test]
fn test_special_random_positive_finite_primitive_floats() {
    // f32, mean abs of exponent = 1/64, mean precision = 65/64
    let values = &[
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 0.5, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.5, 1.0,
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    ];
    let common_values = &[
        (1.0, 954991),
        (1.5, 14672),
        (2.0, 14586),
        (0.5, 14578),
        (3.0, 243),
        (0.25, 228),
        (4.0, 226),
        (0.75, 207),
        (1.25, 127),
        (1.75, 118),
        (6.0, 6),
        (0.125, 4),
        (2.5, 3),
        (8.0, 3),
        (0.625, 3),
        (0.375, 2),
        (0.875, 2),
        (3.5, 1),
    ];
    let sample_median = (1.0, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1.0157463749999305),
        standard_deviation: NiceFloat(0.15870215416222863),
        skewness: NiceFloat(5.805128395366302),
        excess_kurtosis: NiceFloat(73.03005686221248),
    };
    special_random_positive_finite_primitive_floats_helper::<f32>(
        1,
        64,
        65,
        64,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f32, mean abs of exponent = 1, mean precision = 2
    let values = &[
        1.0, 1.5, 3.125, 1.0, 1.0, 2.0, 2.0, 1.0, 3.0, 1.0, 2.0, 3.0, 4.0, 1.5, 1.625, 2.0, 0.125,
        1.0, 0.375, 1.5, 6.375, 4.0, 6.875, 0.5, 0.234375, 1.0, 0.2421875, 6.0, 0.75, 6.0, 6.0,
        2.0, 0.21875, 2.0, 0.875, 0.875, 6.0, 16.0, 27.0, 2.25, 1.5, 1.5, 8.75, 4.0, 0.25, 1.5,
        0.375, 0.375, 1.0, 0.09375,
    ];
    let common_values = &[
        (1.0, 166355),
        (2.0, 83686),
        (0.5, 83270),
        (1.5, 82925),
        (3.0, 41733),
        (0.75, 41659),
        (4.0, 41550),
        (0.25, 41388),
        (1.75, 21006),
        (6.0, 20858),
        (0.125, 20779),
        (8.0, 20769),
        (0.375, 20764),
        (1.25, 20753),
        (12.0, 10512),
        (2.5, 10508),
        (0.0625, 10447),
        (0.625, 10414),
        (0.1875, 10394),
        (16.0, 10354),
    ];
    let sample_median = (1.03125, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(7.654060290573151),
        standard_deviation: NiceFloat(374.2183970257817),
        skewness: NiceFloat(257.0774178486101),
        excess_kurtosis: NiceFloat(79059.24924350459),
    };
    special_random_positive_finite_primitive_floats_helper::<f32>(
        1,
        1,
        2,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f32, mean abs of exponent = 10, mean precision = 10
    let values = &[
        0.80126953,
        0.0000013709068,
        0.015609741,
        0.98552704,
        65536.0,
        0.008257866,
        0.017333984,
        2.25,
        7.7089844,
        0.00004425831,
        0.40625,
        24576.0,
        37249.0,
        1.1991882,
        32.085938,
        0.4375,
        0.0012359619,
        1536.0,
        0.22912993,
        0.0015716553,
        1.6662057e-8,
        0.044523954,
        5694464.0,
        0.125,
        180.0,
        5.625,
        1572864.0,
        1.9092113e-7,
        0.28466797,
        0.0068359375,
        56737790.0,
        4813.375,
        20.954966,
        4.0,
        7.3125,
        3.6040926,
        0.000007293769,
        0.018554688,
        0.00009602308,
        0.000038146973,
        0.00022888184,
        36.324017,
        0.0068359375,
        0.008168057,
        20.0,
        21.398438,
        0.21679688,
        176.0,
        0.11355591,
        6144.0,
    ];
    let common_values = &[
        (1.0, 5117),
        (1.5, 4684),
        (2.0, 4643),
        (0.5, 4592),
        (3.0, 4327),
        (0.75, 4245),
        (0.25, 4231),
        (4.0, 4186),
        (8.0, 3995),
        (0.375, 3923),
        (6.0, 3869),
        (0.125, 3864),
        (0.0625, 3534),
        (16.0, 3502),
        (0.1875, 3489),
        (12.0, 3418),
        (24.0, 3293),
        (32.0, 3244),
        (0.09375, 3209),
        (0.03125, 3152),
    ];
    let sample_median = (1.4882812, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(7.110316815188854e31),
        standard_deviation: NiceFloat(5.291057687231236e34),
        skewness: NiceFloat(817.3228282694379),
        excess_kurtosis: NiceFloat(702102.631759681),
    };
    special_random_positive_finite_primitive_floats_helper::<f32>(
        10,
        1,
        10,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f64, mean abs of exponent = 1/64, mean precision = 65/64
    let values = &[
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 0.5, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.5, 1.0,
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    ];
    let common_values = &[
        (1.0, 954991),
        (1.5, 14672),
        (2.0, 14586),
        (0.5, 14578),
        (3.0, 243),
        (0.25, 228),
        (4.0, 226),
        (0.75, 207),
        (1.25, 127),
        (1.75, 118),
        (6.0, 6),
        (0.125, 4),
        (2.5, 3),
        (8.0, 3),
        (0.625, 3),
        (0.375, 2),
        (0.875, 2),
        (3.5, 1),
    ];
    let sample_median = (1.0, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1.0157463749999305),
        standard_deviation: NiceFloat(0.15870215416222863),
        skewness: NiceFloat(5.805128395366302),
        excess_kurtosis: NiceFloat(73.03005686221248),
    };
    special_random_positive_finite_primitive_floats_helper::<f64>(
        1,
        64,
        65,
        64,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f64, mean abs of exponent = 1, mean precision = 2
    let values = &[
        1.0, 1.5, 3.125, 1.0, 1.0, 2.0, 2.0, 1.0, 3.0, 1.0, 2.0, 3.0, 4.0, 1.5, 1.625, 2.0, 0.125,
        1.0, 0.375, 1.5, 6.375, 4.0, 6.875, 0.5, 0.234375, 1.0, 0.2421875, 6.0, 0.75, 6.0, 6.0,
        2.0, 0.21875, 2.0, 0.875, 0.875, 6.0, 16.0, 27.0, 2.25, 1.5, 1.5, 8.75, 4.0, 0.25, 1.5,
        0.375, 0.375, 1.0, 0.09375,
    ];
    let common_values = &[
        (1.0, 166355),
        (2.0, 83686),
        (0.5, 83270),
        (1.5, 82925),
        (3.0, 41733),
        (0.75, 41659),
        (4.0, 41550),
        (0.25, 41388),
        (1.75, 21006),
        (6.0, 20858),
        (0.125, 20779),
        (8.0, 20769),
        (0.375, 20764),
        (1.25, 20753),
        (12.0, 10512),
        (2.5, 10508),
        (0.0625, 10447),
        (0.625, 10414),
        (0.1875, 10394),
        (16.0, 10354),
    ];
    let sample_median = (1.03125, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(7.654060290573151),
        standard_deviation: NiceFloat(374.2183970257817),
        skewness: NiceFloat(257.0774178486101),
        excess_kurtosis: NiceFloat(79059.24924350459),
    };
    special_random_positive_finite_primitive_floats_helper::<f64>(
        1,
        1,
        2,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f64, mean abs of exponent = 10, mean precision = 10
    let values = &[
        0.80126953125,
        1.3709068298339844e-6,
        0.0156097412109375,
        0.9855270385742188,
        86113.82421875,
        0.012808799743652344,
        0.028076171875,
        2.75,
        7.009765625,
        0.000045569613575935364,
        0.34375,
        24576.0,
        33101.0,
        1.066680908203125,
        53.0390625,
        0.3125,
        0.0018768310546875,
        1536.0,
        0.15994291007518768,
        0.0014495849609375,
        2.5660824576334562e-8,
        0.04374957084655762,
        6583296.0,
        0.125,
        156.0,
        7.125,
        1572864.0,
        1.525198978780118e-7,
        0.32763671875,
        0.0048828125,
        47710208.0,
        6821.875,
        19.824071884155273,
        4.0,
        4.6875,
        2.3686094284057617,
        6.33427407592535e-6,
        0.0185546875,
        0.00007218122482299805,
        0.00003814697265625,
        0.0001373291015625,
        36.43232345581055,
        0.0048828125,
        0.011424465501870706,
        28.0,
        18.0546875,
        0.2207733978284523,
        144.0,
        0.083648681640625,
        4360.2293701171875,
    ];
    let common_values = &[
        (1.0, 4779),
        (1.5, 4356),
        (2.0, 4315),
        (0.5, 4233),
        (3.0, 4023),
        (0.25, 3931),
        (0.75, 3911),
        (4.0, 3866),
        (8.0, 3705),
        (0.375, 3612),
        (6.0, 3588),
        (0.125, 3538),
        (0.0625, 3271),
        (16.0, 3264),
        (0.1875, 3199),
        (12.0, 3152),
        (24.0, 3033),
        (32.0, 2974),
        (0.09375, 2963),
        (0.03125, 2937),
    ];
    let sample_median = (1.484375, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(5.352183416672257e38),
        standard_deviation: NiceFloat(5.3521788476440196e41),
        skewness: NiceFloat(999.9984999986674),
        excess_kurtosis: NiceFloat(999995.0000000936),
    };
    special_random_positive_finite_primitive_floats_helper::<f64>(
        10,
        1,
        10,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
}

fn special_random_positive_finite_primitive_floats_fail_helper<T: PrimitiveFloat>() {
    assert_panic!(special_random_positive_finite_primitive_floats::<T>(
        EXAMPLE_SEED,
        0,
        1,
        10,
        1
    ));
    assert_panic!(special_random_positive_finite_primitive_floats::<T>(
        EXAMPLE_SEED,
        1,
        0,
        10,
        1
    ));
    assert_panic!(special_random_positive_finite_primitive_floats::<T>(
        EXAMPLE_SEED,
        10,
        1,
        1,
        1
    ));
    assert_panic!(special_random_positive_finite_primitive_floats::<T>(
        EXAMPLE_SEED,
        10,
        1,
        1,
        0
    ));
}

#[test]
fn special_random_positive_finite_primitive_floats_fail() {
    apply_fn_to_primitive_floats!(special_random_positive_finite_primitive_floats_fail_helper);
}
