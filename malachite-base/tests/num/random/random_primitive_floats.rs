// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::float::NiceFloat;
use malachite_base::num::random::random_primitive_floats;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::num::random::random_primitive_floats_helper_helper;
use malachite_base::test_util::stats::moments::{CheckedToF64, MomentStats};

fn random_primitive_floats_helper<T: CheckedToF64 + PrimitiveFloat>(
    expected_values: &[T],
    expected_common_values: &[(T, usize)],
    expected_median: (T, Option<T>),
    expected_moment_stats: MomentStats,
) {
    random_primitive_floats_helper_helper(
        random_primitive_floats::<T>(EXAMPLE_SEED),
        expected_values,
        expected_common_values,
        expected_median,
        expected_moment_stats,
    );
}

#[test]
fn test_random_primitive_floats() {
    // f32
    let values = &[
        -2.3484665e-27,
        2.2879888e-18,
        -2.0729896e-12,
        3.3600117e28,
        -9.0217234e-32,
        3564911.0,
        -0.000013376945,
        -1.885545e18,
        8.249455e-29,
        2.2178013e-38,
        -6.306773e-34,
        5.199601e31,
        7.6132625e33,
        0.00015323664,
        9.4768183e36,
        -0.0005665587,
        8.873326e-30,
        0.09273134,
        -7.774831e33,
        4.315623e-8,
    ];
    let common_values = &[
        (5.71262, 2),
        (780.036, 2),
        (224535.3, 2),
        (58.67172, 2),
        (73439.85, 2),
        (-58.01006, 2),
        (-66297.15, 2),
        (-66476.91, 2),
        (13200.071, 2),
        (3306.3635, 2),
    ];
    let sample_median = (4.601794e-39, Some(4.606577e-39));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-1.871815997376882e34),
        standard_deviation: NiceFloat(1.8597574260800838e37),
        skewness: NiceFloat(-0.04588420234596291),
        excess_kurtosis: NiceFloat(174.30920609573673),
    };
    random_primitive_floats_helper::<f32>(
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f64
    let values = &[
        3.106206640558341e-146,
        7.268713316268921e223,
        1.1685126708702852e48,
        -1.0824685183946236e146,
        3.114605160661583e-306,
        2.2453015573637674e249,
        1.2548860979388685e-35,
        -8.287939157477947e-27,
        2.1255041535787165e-13,
        4.815129234795048e-64,
        1.3850402674408148e-17,
        -1.253571770758962e207,
        -1.4941028004491906e142,
        4.366611961454907e-51,
        -7.063699168119985e17,
        -7.062565582436957e90,
        1.1662950933663382e-221,
        2.1976577668343592e-97,
        -2.8212944266870196e-137,
        1.2250916338748408e-222,
    ];
    let common_values = &[
        (-9967188.16722, 1),
        (1808.830612999, 1),
        (32578528203.69, 1),
        (5643444.695113, 1),
        (812845035127.8, 1),
        (-13741970740.45, 1),
        (-1434325.082519, 1),
        (-33781527.93352, 1),
        (-374012916597.5, 1),
        (-46629353341.91, 1),
    ];
    let sample_median = (2.772306592172272e-308, Some(2.7820731194979217e-308));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(7.922018643581038e303),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_primitive_floats_helper::<f64>(
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
}
