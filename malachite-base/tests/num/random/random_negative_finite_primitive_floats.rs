// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::float::NiceFloat;
use malachite_base::num::random::random_negative_finite_primitive_floats;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::num::random::random_primitive_floats_helper_helper;
use malachite_base::test_util::stats::moments::{CheckedToF64, MomentStats};

fn random_negative_finite_primitive_floats_helper<T: CheckedToF64 + PrimitiveFloat>(
    expected_values: &[T],
    expected_common_values: &[(T, usize)],
    expected_median: (T, Option<T>),
    expected_moment_stats: MomentStats,
) {
    random_primitive_floats_helper_helper(
        random_negative_finite_primitive_floats::<T>(EXAMPLE_SEED),
        expected_values,
        expected_common_values,
        expected_median,
        expected_moment_stats,
    );
}

#[test]
fn test_random_negative_finite_primitive_floats() {
    // f32
    let values = &[
        -2.3484663e-27,
        -0.010641626,
        -5.8060583e-9,
        -2.8182442e-31,
        -10462.532,
        -821.12994,
        -6.303163e33,
        -9.50376e-15,
        -4.9561126e-11,
        -8.565163e-22,
        -6667.249,
        -8.632876e18,
        -1222.524,
        -2.8259287e25,
        -8.408057e32,
        -35.37676,
        -5.3312457e-35,
        -1.1431576e-12,
        -5.8021113e-15,
        -4.2578778e-14,
    ];
    let common_values = &[
        (-2.915314, 2),
        (-0.7262628, 2),
        (-1.3261845, 2),
        (-10.318969, 2),
        (-12496.082, 2),
        (-15.058022, 2),
        (-2.5221732, 2),
        (-20.562862, 2),
        (-203818.36, 2),
        (-26.077845, 2),
    ];
    let sample_median = (-1.6313586, Some(-1.6313426));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-1.9935686121103762e36),
        standard_deviation: NiceFloat(1.8605029843851628e37),
        skewness: NiceFloat(-12.384333043431482),
        excess_kurtosis: NiceFloat(170.5284644692483),
    };
    random_negative_finite_primitive_floats_helper::<f32>(
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f64
    let values = &[
        -1.4470055025606618e146,
        -6.2410386896936294e-139,
        -3.235416846271215e118,
        -2.8486059576132576e243,
        -1.3007593490666564e274,
        -1.267802986597089e-253,
        -6.298463311550517e114,
        -2.71148919055391e49,
        -11845642624.881702,
        -6.049786340671396e-245,
        -9.268165817801774e-308,
        -7.204960592983619e278,
        -3.152983742246857e-31,
        -2.9080351183768945e-22,
        -5.397630931761668e186,
        -8.140466543599511e145,
        -1.5709219762031943e-140,
        -1.0805614483727634e203,
        -2.6257467421064505e300,
        -3.922358679765479e20,
    ];
    let common_values = &[
        (-4081294.15273, 1),
        (-75856946.7859, 1),
        (-0.785631131986, 1),
        (-10.70965140303, 1),
        (-306424.6626443, 1),
        (-314701.8184209, 1),
        (-449320.3879786, 1),
        (-590754505.6957, 1),
        (-642882447.5536, 1),
        (-88637.71469077, 1),
    ];
    let sample_median = (-1.5131202036927986, Some(-1.511917932803396));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-1.3244537383462707e305),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_negative_finite_primitive_floats_helper::<f64>(
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
}
