// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::random::special_random_positive_primitive_floats;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::num::random::special_random_primitive_floats_helper_helper;
use malachite_base::test_util::stats::moments::{CheckedToF64, MomentStats, NAN_MOMENT_STATS};
use std::panic::catch_unwind;

fn special_random_positive_primitive_floats_helper<T: CheckedToF64 + PrimitiveFloat>(
    mean_exponent_numerator: u64,
    mean_exponent_denominator: u64,
    mean_precision_numerator: u64,
    mean_precision_denominator: u64,
    mean_special_p_numerator: u64,
    mean_special_p_denominator: u64,
    expected_values: &[T],
    expected_common_values: &[(T, usize)],
    expected_median: (T, Option<T>),
    expected_moment_stats: MomentStats,
) {
    special_random_primitive_floats_helper_helper(
        special_random_positive_primitive_floats::<T>(
            EXAMPLE_SEED,
            mean_exponent_numerator,
            mean_exponent_denominator,
            mean_precision_numerator,
            mean_precision_denominator,
            mean_special_p_numerator,
            mean_special_p_denominator,
        ),
        expected_values,
        expected_common_values,
        expected_median,
        expected_moment_stats,
    );
}

#[test]
fn test_special_random_positive_primitive_floats() {
    // f32, mean abs of exponent = 1/64, mean precision = 65/64, mean special P = 1/4
    let values = &[
        f32::INFINITY,
        1.5,
        1.0,
        f32::INFINITY,
        1.0,
        1.0,
        f32::INFINITY,
        1.0,
        f32::INFINITY,
        f32::INFINITY,
        1.0,
        f32::INFINITY,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        0.5,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        f32::INFINITY,
        1.0,
        f32::INFINITY,
        f32::INFINITY,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        f32::INFINITY,
        1.0,
        1.0,
        0.5,
        1.0,
        f32::INFINITY,
        f32::INFINITY,
        f32::INFINITY,
        1.0,
    ];
    let common_values = &[
        (1.0, 716268),
        (f32::INFINITY, 250209),
        (2.0, 10938),
        (0.5, 10888),
        (1.5, 10871),
        (4.0, 172),
        (0.25, 165),
        (3.0, 161),
        (0.75, 155),
        (1.25, 75),
        (1.75, 74),
        (0.375, 6),
        (3.5, 4),
        (0.125, 4),
        (8.0, 3),
        (0.875, 2),
        (1.875, 2),
        (6.0, 1),
        (0.625, 1),
        (1.125, 1),
    ];
    let sample_median = (1.0, None);
    special_random_positive_primitive_floats_helper::<f32>(
        1,
        64,
        65,
        64,
        1,
        4,
        values,
        common_values,
        sample_median,
        NAN_MOMENT_STATS,
    );

    // f32, mean abs of exponent = 1, mean precision = 2, mean special P = 1/10
    let values = &[
        1.0,
        1.0,
        3.0,
        f32::INFINITY,
        1.0,
        1.0,
        2.0,
        2.0,
        1.0,
        3.0,
        1.0,
        3.0,
        3.0,
        f32::INFINITY,
        6.0,
        1.125,
        1.25,
        f32::INFINITY,
        3.0,
        0.125,
        1.0,
        0.25,
        1.75,
        4.0,
        4.0,
        4.0,
        0.75,
        0.125,
        1.0,
        f32::INFINITY,
        0.125,
        6.0,
        f32::INFINITY,
        0.5,
        4.0,
        6.5,
        2.0,
        0.21875,
        2.0,
        0.5,
        0.5,
        4.0,
        16.0,
        16.0,
        3.0,
        1.0,
        f32::INFINITY,
        1.0,
        10.0,
        6.0,
    ];
    let common_values = &[
        (1.0, 151036),
        (f32::INFINITY, 100224),
        (0.5, 75239),
        (2.0, 74939),
        (1.5, 74396),
        (0.25, 37669),
        (0.75, 37538),
        (3.0, 37523),
        (4.0, 37411),
        (0.375, 18737),
        (6.0, 18708),
        (0.125, 18698),
        (1.25, 18586),
        (8.0, 18570),
        (1.75, 18339),
        (3.5, 9676),
        (0.0625, 9668),
        (16.0, 9474),
        (0.875, 9416),
        (12.0, 9376),
    ];
    let sample_median = (1.5, None);
    special_random_positive_primitive_floats_helper::<f32>(
        1,
        1,
        2,
        1,
        1,
        10,
        values,
        common_values,
        sample_median,
        NAN_MOMENT_STATS,
    );

    // f32, mean abs of exponent = 10, mean precision = 10, mean special P = 1/100
    let values = &[
        0.6328125,
        9.536743e-7,
        0.013671875,
        0.6875,
        70208.0,
        0.01550293,
        0.028625488,
        3.3095703,
        5.775879,
        0.000034958124,
        0.4375,
        31678.0,
        49152.0,
        1.0,
        49.885254,
        0.40625,
        0.0015869141,
        1889.5625,
        0.14140439,
        0.001449585,
        1.4901161e-8,
        0.03125,
        5750784.0,
        0.17578125,
        248.0,
        5.4375,
        1892352.0,
        1.5280966e-7,
        0.2826419,
        0.0057373047,
        51642370.0,
        6384.0,
        27.875542,
        6.152041,
        6.0,
        2.796875,
        0.0000057816505,
        0.029174805,
        0.00011384487,
        0.000039815903,
        0.00012207031,
        48.0,
        0.00390625,
        0.01171875,
        20.0,
        21.625,
        0.171875,
        197.0,
        0.11743164,
        5532.0,
    ];
    let common_values = &[
        (f32::INFINITY, 9989),
        (1.0, 5059),
        (2.0, 4689),
        (0.5, 4622),
        (1.5, 4576),
        (4.0, 4280),
        (3.0, 4153),
        (0.25, 4130),
        (0.75, 4105),
        (8.0, 3912),
        (6.0, 3841),
        (0.125, 3772),
        (0.375, 3663),
        (16.0, 3536),
        (0.1875, 3530),
        (0.0625, 3441),
        (12.0, 3433),
        (0.09375, 3185),
        (0.03125, 3176),
        (32.0, 3094),
    ];
    let sample_median = (1.5, None);
    special_random_positive_primitive_floats_helper::<f32>(
        10,
        1,
        10,
        1,
        1,
        100,
        values,
        common_values,
        sample_median,
        NAN_MOMENT_STATS,
    );

    // f64, mean abs of exponent = 1/64, mean precision = 65/64, mean special P = 1/4
    let values = &[
        f64::INFINITY,
        1.5,
        1.0,
        f64::INFINITY,
        1.0,
        1.0,
        f64::INFINITY,
        1.0,
        f64::INFINITY,
        f64::INFINITY,
        1.0,
        f64::INFINITY,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        0.5,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        f64::INFINITY,
        1.0,
        f64::INFINITY,
        f64::INFINITY,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        f64::INFINITY,
        1.0,
        1.0,
        0.5,
        1.0,
        f64::INFINITY,
        f64::INFINITY,
        f64::INFINITY,
        1.0,
    ];
    let common_values = &[
        (1.0, 716268),
        (f64::INFINITY, 250209),
        (2.0, 10938),
        (0.5, 10888),
        (1.5, 10871),
        (4.0, 172),
        (0.25, 165),
        (3.0, 161),
        (0.75, 155),
        (1.25, 75),
        (1.75, 74),
        (0.375, 6),
        (3.5, 4),
        (0.125, 4),
        (8.0, 3),
        (0.875, 2),
        (1.875, 2),
        (6.0, 1),
        (0.625, 1),
        (1.125, 1),
    ];
    let sample_median = (1.0, None);
    special_random_positive_primitive_floats_helper::<f64>(
        1,
        64,
        65,
        64,
        1,
        4,
        values,
        common_values,
        sample_median,
        NAN_MOMENT_STATS,
    );

    // f64, mean abs of exponent = 1, mean precision = 2, mean special P = 1/10
    let values = &[
        1.0,
        1.0,
        3.0,
        f64::INFINITY,
        1.0,
        1.0,
        2.0,
        2.0,
        1.0,
        3.0,
        1.0,
        3.0,
        3.0,
        f64::INFINITY,
        6.0,
        1.125,
        1.25,
        f64::INFINITY,
        3.0,
        0.125,
        1.0,
        0.25,
        1.75,
        4.0,
        4.0,
        4.0,
        0.75,
        0.125,
        1.0,
        f64::INFINITY,
        0.125,
        6.0,
        f64::INFINITY,
        0.5,
        4.0,
        6.5,
        2.0,
        0.21875,
        2.0,
        0.5,
        0.5,
        4.0,
        16.0,
        16.0,
        3.0,
        1.0,
        f64::INFINITY,
        1.0,
        10.0,
        6.0,
    ];
    let common_values = &[
        (1.0, 151036),
        (f64::INFINITY, 100224),
        (0.5, 75239),
        (2.0, 74939),
        (1.5, 74396),
        (0.25, 37669),
        (0.75, 37538),
        (3.0, 37523),
        (4.0, 37411),
        (0.375, 18737),
        (6.0, 18708),
        (0.125, 18698),
        (1.25, 18586),
        (8.0, 18570),
        (1.75, 18339),
        (3.5, 9676),
        (0.0625, 9668),
        (16.0, 9474),
        (0.875, 9416),
        (12.0, 9376),
    ];
    let sample_median = (1.5, None);
    special_random_positive_primitive_floats_helper::<f64>(
        1,
        1,
        2,
        1,
        1,
        10,
        values,
        common_values,
        sample_median,
        NAN_MOMENT_STATS,
    );

    // f64, mean abs of exponent = 10, mean precision = 10, mean special P = 1/100
    let values = &[
        0.6328125,
        9.5367431640625e-7,
        0.013671875,
        0.6875,
        70208.0,
        0.0155029296875,
        0.02862548828125,
        3.3095703125,
        5.77587890625,
        0.0000349581241607666,
        0.4375,
        31678.0,
        49152.0,
        1.0,
        49.88525390625,
        0.40625,
        0.0015869140625,
        1889.5625,
        0.141404390335083,
        0.0014495849609375,
        1.4901161193847656e-8,
        0.03125,
        5750784.0,
        0.17578125,
        248.0,
        5.4375,
        1892352.0,
        1.528096618130803e-7,
        0.2826418876647949,
        0.0057373046875,
        51642368.0,
        6384.0,
        27.87554168701172,
        6.152040958404541,
        6.0,
        2.796875,
        5.781650543212891e-6,
        0.0291748046875,
        0.0001138448715209961,
        0.00003981590270996094,
        0.0001220703125,
        48.0,
        0.00390625,
        0.01171875,
        20.0,
        21.625,
        0.171875,
        205.175663292408,
        0.118408203125,
        6436.0,
    ];
    let common_values = &[
        (f64::INFINITY, 9989),
        (1.0, 4750),
        (2.0, 4357),
        (0.5, 4257),
        (1.5, 4256),
        (4.0, 3955),
        (3.0, 3844),
        (0.25, 3812),
        (0.75, 3799),
        (8.0, 3628),
        (6.0, 3535),
        (0.125, 3447),
        (0.375, 3398),
        (16.0, 3257),
        (0.1875, 3234),
        (12.0, 3202),
        (0.0625, 3182),
        (0.09375, 2957),
        (0.03125, 2919),
        (32.0, 2805),
    ];
    let sample_median = (1.5162887573242188, Some(1.516387939453125));
    special_random_positive_primitive_floats_helper::<f64>(
        10,
        1,
        10,
        1,
        1,
        100,
        values,
        common_values,
        sample_median,
        NAN_MOMENT_STATS,
    );
}

fn special_random_positive_primitive_floats_fail_helper<T: PrimitiveFloat>() {
    assert_panic!(special_random_positive_primitive_floats::<T>(
        EXAMPLE_SEED,
        0,
        1,
        10,
        1,
        1,
        10
    ));
    assert_panic!(special_random_positive_primitive_floats::<T>(
        EXAMPLE_SEED,
        1,
        0,
        10,
        1,
        1,
        10
    ));
    assert_panic!(special_random_positive_primitive_floats::<T>(
        EXAMPLE_SEED,
        10,
        1,
        1,
        1,
        1,
        10
    ));
    assert_panic!(special_random_positive_primitive_floats::<T>(
        EXAMPLE_SEED,
        10,
        1,
        1,
        0,
        1,
        10
    ));
    assert_panic!(special_random_positive_primitive_floats::<T>(
        EXAMPLE_SEED,
        10,
        1,
        10,
        1,
        1,
        0
    ));
    assert_panic!(special_random_positive_primitive_floats::<T>(
        EXAMPLE_SEED,
        10,
        1,
        10,
        1,
        2,
        1
    ));
}

#[test]
fn special_random_positive_primitive_floats_fail() {
    apply_fn_to_primitive_floats!(special_random_positive_primitive_floats_fail_helper);
}
