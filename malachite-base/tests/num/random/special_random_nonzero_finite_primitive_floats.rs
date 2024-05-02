// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::float::NiceFloat;
use malachite_base::num::random::special_random_nonzero_finite_primitive_floats;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::num::random::special_random_primitive_floats_helper_helper;
use malachite_base::test_util::stats::moments::{CheckedToF64, MomentStats};
use std::panic::catch_unwind;

fn special_random_nonzero_finite_primitive_floats_helper<T: CheckedToF64 + PrimitiveFloat>(
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
        special_random_nonzero_finite_primitive_floats::<T>(
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
fn test_special_random_nonzero_finite_primitive_floats() {
    // f32, mean abs of exponent = 1/64, mean precision = 65/64
    let values = &[
        -1.5, -1.0, -1.0, 1.0, -1.0, 1.0, -1.0, -1.0, -1.0, 1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0,
        -1.0, -1.0, -1.0, -0.5, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0,
        -1.0, 1.0, 0.5, -1.0, 1.0, -1.5, 1.0, -1.0, 1.0, 1.0, 1.0, -1.0, 1.5, 1.0, -1.0, 1.0, 1.0,
        1.0,
    ];
    let common_values = &[
        (1.0, 478216),
        (-1.0, 476955),
        (-2.0, 7440),
        (-1.5, 7332),
        (0.5, 7303),
        (-0.5, 7275),
        (1.5, 7189),
        (2.0, 7170),
        (0.25, 120),
        (4.0, 116),
        (-4.0, 114),
        (3.0, 111),
        (-3.0, 107),
        (0.75, 105),
        (-0.25, 103),
        (-0.75, 103),
        (-1.25, 62),
        (1.75, 61),
        (1.25, 50),
        (-1.75, 39),
    ];
    let sample_median = (0.5, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.000541124999999992),
        standard_deviation: NiceFloat(1.0278732889234226),
        skewness: NiceFloat(-0.0043076036065952585),
        excess_kurtosis: NiceFloat(-1.7844456265500606),
    };
    special_random_nonzero_finite_primitive_floats_helper::<f32>(
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
        -1.0, -1.0, -3.0, 1.0, -1.0, 2.0, -2.0, -1.0, -3.0, 1.0, 3.0, 3.0, -6.0, -1.125, 1.25,
        -3.0, -0.125, -1.0, -0.25, -1.75, 4.0, 4.0, -4.0, -0.75, 0.125, -1.0, -0.125, 6.0, 0.5,
        -4.0, -6.5, 2.0, -0.21875, 2.0, 0.5, -0.5, 4.0, -16.0, 16.0, -3.0, 1.0, 1.0, 10.0, -6.0,
        0.25, 1.0, -0.390625, 0.375, 1.5, 0.09375,
    ];
    let common_values = &[
        (1.0, 84345),
        (-1.0, 83427),
        (-0.5, 42050),
        (2.0, 41730),
        (-2.0, 41718),
        (0.5, 41631),
        (1.5, 41446),
        (-1.5, 41312),
        (3.0, 20932),
        (-0.25, 20908),
        (-3.0, 20854),
        (-0.75, 20853),
        (-4.0, 20831),
        (0.25, 20819),
        (0.75, 20775),
        (4.0, 20696),
        (0.125, 10518),
        (0.375, 10492),
        (-6.0, 10471),
        (8.0, 10359),
    ];
    let sample_median = (0.001953125, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.14825837830007416),
        standard_deviation: NiceFloat(371.086277437807),
        skewness: NiceFloat(-124.2604097831067),
        excess_kurtosis: NiceFloat(123898.81502837151),
    };
    special_random_nonzero_finite_primitive_floats_helper::<f32>(
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
        -0.6328125,
        -9.536743e-7,
        -0.013671875,
        0.6875,
        -70208.0,
        0.01550293,
        -0.028625488,
        -3.3095703,
        -5.775879,
        0.000034958124,
        0.4375,
        31678.0,
        -49152.0,
        -1.0,
        49.885254,
        -0.40625,
        -0.0015869141,
        -1889.5625,
        -0.14140439,
        -0.001449585,
        1.4901161e-8,
        0.03125,
        -5750784.0,
        -0.17578125,
        248.0,
        -5.4375,
        -1892352.0,
        1.5280966e-7,
        0.2826419,
        -0.0057373047,
        -51642370.0,
        6384.0,
        -27.875542,
        6.152041,
        6.0,
        -2.796875,
        0.0000057816505,
        -0.029174805,
        0.00011384487,
        -0.000039815903,
        0.00012207031,
        48.0,
        0.00390625,
        -0.01171875,
        20.0,
        21.625,
        -0.171875,
        197.0,
        0.11743164,
        5532.0,
    ];
    let common_values = &[
        (-1.0, 2552),
        (1.0, 2548),
        (2.0, 2409),
        (-0.5, 2367),
        (-2.0, 2335),
        (1.5, 2326),
        (0.5, 2308),
        (-1.5, 2290),
        (-4.0, 2193),
        (4.0, 2130),
        (-3.0, 2106),
        (3.0, 2096),
        (0.25, 2092),
        (-0.25, 2085),
        (-0.75, 2084),
        (0.75, 2065),
        (8.0, 2008),
        (6.0, 1972),
        (-8.0, 1945),
        (-0.125, 1922),
    ];
    let sample_median = (1.6940659e-20, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-3.7799889499291132e31),
        standard_deviation: NiceFloat(6.206676176137639e34),
        skewness: NiceFloat(-784.8389177089807),
        excess_kurtosis: NiceFloat(789333.7989913624),
    };
    special_random_nonzero_finite_primitive_floats_helper::<f32>(
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
        -1.5, -1.0, -1.0, 1.0, -1.0, 1.0, -1.0, -1.0, -1.0, 1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0,
        -1.0, -1.0, -1.0, -0.5, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0,
        -1.0, 1.0, 0.5, -1.0, 1.0, -1.5, 1.0, -1.0, 1.0, 1.0, 1.0, -1.0, 1.5, 1.0, -1.0, 1.0, 1.0,
        1.0,
    ];
    let common_values = &[
        (1.0, 478216),
        (-1.0, 476955),
        (-2.0, 7440),
        (-1.5, 7332),
        (0.5, 7303),
        (-0.5, 7275),
        (1.5, 7189),
        (2.0, 7170),
        (0.25, 120),
        (4.0, 116),
        (-4.0, 114),
        (3.0, 111),
        (-3.0, 107),
        (0.75, 105),
        (-0.25, 103),
        (-0.75, 103),
        (-1.25, 62),
        (1.75, 61),
        (1.25, 50),
        (-1.75, 39),
    ];
    let sample_median = (0.5, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.000541124999999992),
        standard_deviation: NiceFloat(1.0278732889234226),
        skewness: NiceFloat(-0.0043076036065952585),
        excess_kurtosis: NiceFloat(-1.7844456265500606),
    };
    special_random_nonzero_finite_primitive_floats_helper::<f64>(
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
        -1.0, -1.0, -3.0, 1.0, -1.0, 2.0, -2.0, -1.0, -3.0, 1.0, 3.0, 3.0, -6.0, -1.125, 1.25,
        -3.0, -0.125, -1.0, -0.25, -1.75, 4.0, 4.0, -4.0, -0.75, 0.125, -1.0, -0.125, 6.0, 0.5,
        -4.0, -6.5, 2.0, -0.21875, 2.0, 0.5, -0.5, 4.0, -16.0, 16.0, -3.0, 1.0, 1.0, 10.0, -6.0,
        0.25, 1.0, -0.390625, 0.375, 1.5, 0.09375,
    ];
    let common_values = &[
        (1.0, 84345),
        (-1.0, 83427),
        (-0.5, 42050),
        (2.0, 41730),
        (-2.0, 41718),
        (0.5, 41631),
        (1.5, 41446),
        (-1.5, 41312),
        (3.0, 20932),
        (-0.25, 20908),
        (-3.0, 20854),
        (-0.75, 20853),
        (-4.0, 20831),
        (0.25, 20819),
        (0.75, 20775),
        (4.0, 20696),
        (0.125, 10518),
        (0.375, 10492),
        (-6.0, 10471),
        (8.0, 10359),
    ];
    let sample_median = (0.001953125, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.14825837830007416),
        standard_deviation: NiceFloat(371.086277437807),
        skewness: NiceFloat(-124.2604097831067),
        excess_kurtosis: NiceFloat(123898.81502837151),
    };
    special_random_nonzero_finite_primitive_floats_helper::<f64>(
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
        -0.6328125,
        -9.5367431640625e-7,
        -0.013671875,
        0.6875,
        -70208.0,
        0.0155029296875,
        -0.02862548828125,
        -3.3095703125,
        -5.77587890625,
        0.0000349581241607666,
        0.4375,
        31678.0,
        -49152.0,
        -1.0,
        49.88525390625,
        -0.40625,
        -0.0015869140625,
        -1889.5625,
        -0.141404390335083,
        -0.0014495849609375,
        1.4901161193847656e-8,
        0.03125,
        -5750784.0,
        -0.17578125,
        248.0,
        -5.4375,
        -1892352.0,
        1.528096618130803e-7,
        0.2826418876647949,
        -0.0057373046875,
        -51642368.0,
        6384.0,
        -27.87554168701172,
        6.152040958404541,
        6.0,
        -2.796875,
        5.781650543212891e-6,
        -0.0291748046875,
        0.0001138448715209961,
        -0.00003981590270996094,
        0.0001220703125,
        48.0,
        0.00390625,
        -0.01171875,
        20.0,
        21.625,
        -0.171875,
        205.175663292408,
        0.118408203125,
        6436.0,
    ];
    let common_values = &[
        (-1.0, 2444),
        (1.0, 2346),
        (2.0, 2250),
        (1.5, 2165),
        (-0.5, 2156),
        (-2.0, 2155),
        (0.5, 2150),
        (-1.5, 2135),
        (4.0, 2036),
        (-4.0, 1965),
        (0.75, 1957),
        (-0.25, 1953),
        (3.0, 1950),
        (-3.0, 1943),
        (0.25, 1899),
        (-0.75, 1878),
        (8.0, 1866),
        (6.0, 1819),
        (-8.0, 1804),
        (0.125, 1779),
    ];
    let sample_median = (2.3597808514912774e-20, Some(2.371692252312041e-20));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(5.965455610813376e38),
        standard_deviation: NiceFloat(5.9654493406037485e41),
        skewness: NiceFloat(999.998499997817),
        excess_kurtosis: NiceFloat(999994.9999989544),
    };
    special_random_nonzero_finite_primitive_floats_helper::<f64>(
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

fn special_random_nonzero_finite_primitive_floats_fail_helper<T: PrimitiveFloat>() {
    assert_panic!(special_random_nonzero_finite_primitive_floats::<T>(
        EXAMPLE_SEED,
        0,
        1,
        10,
        1
    ));
    assert_panic!(special_random_nonzero_finite_primitive_floats::<T>(
        EXAMPLE_SEED,
        1,
        0,
        10,
        1
    ));
    assert_panic!(special_random_nonzero_finite_primitive_floats::<T>(
        EXAMPLE_SEED,
        10,
        1,
        1,
        1
    ));
    assert_panic!(special_random_nonzero_finite_primitive_floats::<T>(
        EXAMPLE_SEED,
        10,
        1,
        1,
        0
    ));
}

#[test]
fn special_random_nonzero_finite_primitive_floats_fail() {
    apply_fn_to_primitive_floats!(special_random_nonzero_finite_primitive_floats_fail_helper);
}
