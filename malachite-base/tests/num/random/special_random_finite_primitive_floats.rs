// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::float::NiceFloat;
use malachite_base::num::random::special_random_finite_primitive_floats;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::num::random::special_random_primitive_floats_helper_helper;
use malachite_base::test_util::stats::moments::{CheckedToF64, MomentStats};
use std::panic::catch_unwind;

fn special_random_finite_primitive_floats_helper<T: CheckedToF64 + PrimitiveFloat>(
    mean_exponent_numerator: u64,
    mean_exponent_denominator: u64,
    mean_precision_numerator: u64,
    mean_precision_denominator: u64,
    mean_zero_p_numerator: u64,
    mean_zero_p_denominator: u64,
    expected_values: &[T],
    expected_common_values: &[(T, usize)],
    expected_median: (T, Option<T>),
    expected_moment_stats: MomentStats,
) {
    special_random_primitive_floats_helper_helper(
        special_random_finite_primitive_floats::<T>(
            EXAMPLE_SEED,
            mean_exponent_numerator,
            mean_exponent_denominator,
            mean_precision_numerator,
            mean_precision_denominator,
            mean_zero_p_numerator,
            mean_zero_p_denominator,
        ),
        expected_values,
        expected_common_values,
        expected_median,
        expected_moment_stats,
    );
}

#[test]
fn test_special_random_finite_primitive_floats() {
    // f32, mean abs of exponent = 1/64, mean precision = 65/64, mean zero P = 1/4
    let values = &[
        0.0, 1.0, 1.0, -0.0, 1.0, -1.0, 0.0, -1.0, 0.0, -0.0, -1.0, -0.0, -1.0, 1.0, 1.0, -1.0,
        -1.0, -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 0.5, 1.0, -1.0, 1.0, -1.0, 1.0, 1.0,
        0.0, -1.5, -0.0, 0.0, -1.0, 1.0, -1.0, -1.0, -1.0, 0.0, 1.0, -1.0, -0.5, -1.0, -0.0, 0.0,
        0.0, 1.0,
    ];
    let common_values = &[
        (1.0, 358244),
        (-1.0, 357926),
        (0.0, 125637),
        (-0.0, 124572),
        (2.0, 5538),
        (1.5, 5500),
        (0.5, 5497),
        (-1.5, 5454),
        (-2.0, 5379),
        (-0.5, 5357),
        (0.75, 102),
        (3.0, 98),
        (-4.0, 95),
        (-0.25, 91),
        (-0.75, 87),
        (-3.0, 86),
        (0.25, 79),
        (4.0, 75),
        (-1.25, 48),
        (1.75, 44),
    ];
    let sample_median = (0.0, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.0007401249999999665),
        standard_deviation: NiceFloat(0.8901654924701277),
        skewness: NiceFloat(-0.00169030138949471),
        excess_kurtosis: NiceFloat(-1.371049776159086),
    };
    special_random_finite_primitive_floats_helper::<f32>(
        1,
        64,
        65,
        64,
        1,
        4,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f32, mean abs of exponent = 1, mean precision = 2, mean zero P = 1/10
    let values = &[
        1.0, 1.25, 3.0, 0.0, -1.0, -1.0, -2.0, -3.5, 1.0, 2.0, -1.5, -2.5, -2.0, -0.0, -6.5, -1.0,
        -1.0, 0.0, 3.0, -0.21875, -1.0, 0.25, 1.5, 5.25, -4.0, 7.0, -0.5, 0.1875, 1.25, 0.0,
        -0.1875, -7.5, -0.0, 0.75, -7.0, -6.0, -3.0, 0.234375, -2.0, -0.875, -0.75, 6.0, -24.0,
        24.0, -2.0, 1.5, -0.0, -1.25, 14.0, 5.0,
    ];
    let common_values = &[
        (1.0, 74789),
        (-1.0, 74702),
        (0.0, 50351),
        (-0.0, 49873),
        (1.5, 38119),
        (-0.5, 37713),
        (2.0, 37640),
        (-1.5, 37613),
        (-2.0, 37333),
        (0.5, 37027),
        (0.75, 19050),
        (4.0, 18892),
        (0.25, 18875),
        (-3.0, 18866),
        (3.0, 18821),
        (-0.75, 18725),
        (-4.0, 18663),
        (-0.25, 18537),
        (0.125, 9445),
        (-0.375, 9395),
    ];
    let sample_median = (0.0, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.8271329178933905),
        standard_deviation: NiceFloat(427.372166726293),
        skewness: NiceFloat(-141.119016305626),
        excess_kurtosis: NiceFloat(144205.19930780405),
    };
    special_random_finite_primitive_floats_helper::<f32>(
        1,
        1,
        2,
        1,
        1,
        10,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f32, mean abs of exponent = 10, mean precision = 10, mean zero P = 1/100
    let values = &[
        0.65625,
        0.0000014255784,
        0.013183594,
        -0.8125,
        -74240.0,
        -0.0078125,
        -0.03060913,
        3.331552,
        4.75,
        -0.000038146973,
        -0.3125,
        -27136.0,
        -59392.0,
        -1.75,
        -41.1875,
        0.30940247,
        -0.0009765625,
        -1536.0,
        0.2109375,
        0.0014648438,
        2.1129381e-8,
        -0.037109375,
        5242880.0,
        -0.21386719,
        134.21094,
        4.184082,
        -1561370.0,
        -2.1420419e-7,
        0.38085938,
        -0.007003784,
        -37748736.0,
        -6448.0,
        28.25,
        -6.703125,
        -4.483364,
        -3.1757812,
        0.000003915804,
        -0.020751953,
        0.00011110306,
        -0.000053405256,
        0.00019985437,
        -35.40625,
        0.005859375,
        0.0078125,
        28.25,
        30.0,
        -0.20776367,
        -144.0,
        -0.109375,
        -6144.0,
    ];
    let common_values = &[
        (0.0, 5098),
        (-0.0, 4891),
        (1.0, 2559),
        (-1.0, 2528),
        (0.5, 2362),
        (-2.0, 2312),
        (-1.5, 2306),
        (2.0, 2304),
        (1.5, 2275),
        (-0.5, 2243),
        (-3.0, 2204),
        (-4.0, 2163),
        (-0.25, 2129),
        (0.75, 2103),
        (3.0, 2081),
        (0.25, 2070),
        (-0.75, 2047),
        (4.0, 2038),
        (-6.0, 1943),
        (-8.0, 1918),
    ];
    let sample_median = (0.0, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-8.736310580536276e31),
        standard_deviation: NiceFloat(6.494074857946111e34),
        skewness: NiceFloat(-779.0012319222365),
        excess_kurtosis: NiceFloat(633402.0042901832),
    };
    special_random_finite_primitive_floats_helper::<f32>(
        10,
        1,
        10,
        1,
        1,
        100,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f64, mean abs of exponent = 1/64, mean precision = 65/64, mean zero P = 1/4
    let values = &[
        0.0, 1.0, 1.0, -0.0, 1.0, -1.0, 0.0, -1.0, 0.0, -0.0, -1.0, -0.0, -1.0, 1.0, 1.0, -1.0,
        -1.0, -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 0.5, 1.0, -1.0, 1.0, -1.0, 1.0, 1.0,
        0.0, -1.5, -0.0, 0.0, -1.0, 1.0, -1.0, -1.0, -1.0, 0.0, 1.0, -1.0, -0.5, -1.0, -0.0, 0.0,
        0.0, 1.0,
    ];
    let common_values = &[
        (1.0, 358244),
        (-1.0, 357926),
        (0.0, 125637),
        (-0.0, 124572),
        (2.0, 5538),
        (1.5, 5500),
        (0.5, 5497),
        (-1.5, 5454),
        (-2.0, 5379),
        (-0.5, 5357),
        (0.75, 102),
        (3.0, 98),
        (-4.0, 95),
        (-0.25, 91),
        (-0.75, 87),
        (-3.0, 86),
        (0.25, 79),
        (4.0, 75),
        (-1.25, 48),
        (1.75, 44),
    ];
    let sample_median = (0.0, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.0007401249999999665),
        standard_deviation: NiceFloat(0.8901654924701277),
        skewness: NiceFloat(-0.00169030138949471),
        excess_kurtosis: NiceFloat(-1.371049776159086),
    };
    special_random_finite_primitive_floats_helper::<f64>(
        1,
        64,
        65,
        64,
        1,
        4,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f64, mean abs of exponent = 1, mean precision = 2, mean zero P = 1/10
    let values = &[
        1.0, 1.25, 3.0, 0.0, -1.0, -1.0, -2.0, -3.5, 1.0, 2.0, -1.5, -2.5, -2.0, -0.0, -6.5, -1.0,
        -1.0, 0.0, 3.0, -0.21875, -1.0, 0.25, 1.5, 5.25, -4.0, 7.0, -0.5, 0.1875, 1.25, 0.0,
        -0.1875, -7.5, -0.0, 0.75, -7.0, -6.0, -3.0, 0.234375, -2.0, -0.875, -0.75, 6.0, -24.0,
        24.0, -2.0, 1.5, -0.0, -1.25, 14.0, 5.0,
    ];
    let common_values = &[
        (1.0, 74789),
        (-1.0, 74702),
        (0.0, 50351),
        (-0.0, 49873),
        (1.5, 38119),
        (-0.5, 37713),
        (2.0, 37640),
        (-1.5, 37613),
        (-2.0, 37333),
        (0.5, 37027),
        (0.75, 19050),
        (4.0, 18892),
        (0.25, 18875),
        (-3.0, 18866),
        (3.0, 18821),
        (-0.75, 18725),
        (-4.0, 18663),
        (-0.25, 18537),
        (0.125, 9445),
        (-0.375, 9395),
    ];
    let sample_median = (0.0, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.8271329178933905),
        standard_deviation: NiceFloat(427.372166726293),
        skewness: NiceFloat(-141.119016305626),
        excess_kurtosis: NiceFloat(144205.19930780405),
    };
    special_random_finite_primitive_floats_helper::<f64>(
        1,
        1,
        2,
        1,
        1,
        10,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f64, mean abs of exponent = 10, mean precision = 10, mean zero P = 1/100
    let values = &[
        0.7709910366684198,
        1.2504315236583352e-6,
        0.00830078125,
        -0.8125,
        -85504.0,
        -0.0078125,
        -0.018890380859375,
        2.5721821784973145,
        5.75,
        -0.00003814697265625,
        -0.4375,
        -24064.0,
        -43008.0,
        -1.75,
        -54.6875,
        0.4641265869140625,
        -0.0014760522753931582,
        -1536.0,
        0.1484375,
        0.00146484375,
        1.9383151084184647e-8,
        -0.060546875,
        7340032.0,
        -0.1982421875,
        203.0546875,
        4.57177734375,
        -1555162.0,
        -2.0675361156463623e-7,
        0.279296875,
        -0.0045928955078125,
        -46137344.0,
        -5712.0,
        17.75,
        -5.265625,
        -7.966220855712891,
        -2.99609375,
        5.397188942879438e-6,
        -0.017333984375,
        0.00011491775512695312,
        -0.00005845972555107437,
        0.00020831823348999023,
        -46.78125,
        0.005859375,
        0.0078125,
        27.25,
        30.0,
        -0.175537109375,
        -208.0,
        -0.109375,
        -6144.0,
    ];
    let common_values = &[
        (0.0, 5098),
        (-0.0, 4891),
        (1.0, 2396),
        (-1.0, 2336),
        (-2.0, 2200),
        (-1.5, 2169),
        (0.5, 2116),
        (2.0, 2108),
        (-0.5, 2101),
        (1.5, 2085),
        (-3.0, 2000),
        (4.0, 1993),
        (3.0, 1969),
        (-0.25, 1955),
        (0.75, 1946),
        (0.25, 1917),
        (-4.0, 1882),
        (-0.75, 1863),
        (8.0, 1826),
        (-6.0, 1782),
    ];
    let sample_median = (0.0, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-7.053229374417263e38),
        standard_deviation: NiceFloat(7.053232003373143e41),
        skewness: NiceFloat(-999.9984999989927),
        excess_kurtosis: NiceFloat(999995.0000005187),
    };
    special_random_finite_primitive_floats_helper::<f64>(
        10,
        1,
        10,
        1,
        1,
        100,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
}

fn special_random_finite_primitive_floats_fail_helper<T: PrimitiveFloat>() {
    assert_panic!(special_random_finite_primitive_floats::<T>(
        EXAMPLE_SEED,
        0,
        1,
        10,
        1,
        1,
        10
    ));
    assert_panic!(special_random_finite_primitive_floats::<T>(
        EXAMPLE_SEED,
        1,
        0,
        10,
        1,
        1,
        10
    ));
    assert_panic!(special_random_finite_primitive_floats::<T>(
        EXAMPLE_SEED,
        10,
        1,
        1,
        1,
        1,
        10
    ));
    assert_panic!(special_random_finite_primitive_floats::<T>(
        EXAMPLE_SEED,
        10,
        1,
        1,
        0,
        1,
        10
    ));
    assert_panic!(special_random_finite_primitive_floats::<T>(
        EXAMPLE_SEED,
        10,
        1,
        10,
        1,
        1,
        0
    ));
    assert_panic!(special_random_finite_primitive_floats::<T>(
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
fn special_random_finite_primitive_floats_fail() {
    apply_fn_to_primitive_floats!(special_random_finite_primitive_floats_fail_helper);
}
