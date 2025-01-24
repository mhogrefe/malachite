// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::float::NiceFloat;
use malachite_base::num::random::random_positive_primitive_floats;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::num::random::random_primitive_floats_helper_helper;
use malachite_base::test_util::stats::moments::{CheckedToF64, MomentStats};

fn random_positive_primitive_floats_helper<T: CheckedToF64 + PrimitiveFloat>(
    expected_values: &[T],
    expected_common_values: &[(T, usize)],
    expected_median: (T, Option<T>),
    expected_moment_stats: MomentStats,
) {
    random_primitive_floats_helper_helper(
        random_positive_primitive_floats::<T>(EXAMPLE_SEED),
        expected_values,
        expected_common_values,
        expected_median,
        expected_moment_stats,
    );
}

#[test]
fn test_random_positive_primitive_floats() {
    // f32
    let values = &[
        9.5715654e26,
        209.6476,
        386935780.0,
        7.965817e30,
        0.00021030706,
        0.0027270128,
        3.4398167e-34,
        2.3397111e14,
        44567765000.0,
        2.3479653e21,
        0.00033502287,
        2.4460542e-19,
        0.0017637977,
        7.956594e-26,
        2.62625e-33,
        0.059202384,
        3.9310746e34,
        1916542100000.0,
        3.847343e14,
        5.2843143e13,
    ];
    let common_values = &[
        (59.25553, 2),
        (68.23402, 2),
        (75936.53, 2),
        (9.581732, 2),
        (0.7711715, 2),
        (0.8694567, 2),
        (1.6738155, 2),
        (1168060.5, 2),
        (12.359466, 2),
        (146.89862, 2),
    ];
    let sample_median = (1.3686414, Some(1.3686574));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(2.0305910210122042e36),
        standard_deviation: NiceFloat(1.8908940416475047e37),
        skewness: NiceFloat(12.30495721768228),
        excess_kurtosis: NiceFloat(168.0100163146302),
    };
    random_positive_primitive_floats_helper::<f32>(
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f64
    let values = &[
        1.553103320279171e-146,
        3.399767269777243e138,
        6.920999672956352e-119,
        7.798612728071752e-244,
        1.7297528302435229e-274,
        1.6730698009459568e253,
        3.494622700146595e-115,
        7.870553878200458e-50,
        1.88707572126009e-10,
        3.680784346523073e244,
        2.2006770848831818e307,
        3.0495236892779565e-279,
        7.105167957384262e30,
        7.68196442827874e21,
        4.043761560817649e-187,
        2.743276189070554e-146,
        1.4322725127932769e140,
        2.0686074901798122e-203,
        7.761683804525804e-301,
        5.6617484237282776e-21,
    ];
    let common_values = &[
        (66730.2435683, 1),
        (2.857475472056, 1),
        (2042187757.012, 1),
        (206.7039373431, 1),
        (2621701.092576, 1),
        (27.03018555735, 1),
        (3.025032765206, 1),
        (454348736.2729, 1),
        (5683730270.934, 1),
        (79306986.55539, 1),
    ];
    let sample_median = (1.4868797963072014, Some(1.488082067196604));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1.3252967815615318e305),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_positive_primitive_floats_helper::<f64>(
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
}
