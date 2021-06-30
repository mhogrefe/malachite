use itertools::Itertools;
use malachite_base::num::float::NiceFloat;
use malachite_base::num::float::PrimitiveFloat;
use malachite_base::num::random::random_finite_primitive_floats;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base_test_util::stats::common_values_map::common_values_map;
use malachite_base_test_util::stats::median;
use malachite_base_test_util::stats::moments::{moment_stats, CheckedToF64, MomentStats};

fn random_finite_primitive_floats_helper<T: CheckedToF64 + PrimitiveFloat>(
    expected_values: &[T],
    expected_common_values: &[(T, usize)],
    expected_median: (T, Option<T>),
    expected_moment_stats: MomentStats,
) {
    let xs = random_finite_primitive_floats::<T>(EXAMPLE_SEED);
    let actual_values = xs.clone().take(20).map(NiceFloat).collect_vec();
    let actual_common_values = common_values_map(1000000, 10, xs.clone().map(NiceFloat));
    let actual_median = median(xs.clone().map(NiceFloat).take(1000000));
    let actual_moment_stats = moment_stats(xs.take(1000000));
    let (lo, hi) = expected_median;
    assert_eq!(
        (
            actual_values,
            actual_common_values.as_slice(),
            actual_median,
            actual_moment_stats
        ),
        (
            expected_values.iter().cloned().map(NiceFloat).collect_vec(),
            expected_common_values
                .iter()
                .map(|&(x, freq)| (NiceFloat(x), freq))
                .collect_vec()
                .as_slice(),
            (NiceFloat(lo), hi.map(NiceFloat)),
            expected_moment_stats
        )
    );
}

#[test]
fn test_random_finite_primitive_floats() {
    // f32
    let values = &[
        -2.3484663e-27,
        2.287989e-18,
        -2.0729893e-12,
        3.360012e28,
        -9.021723e-32,
        3564911.2,
        -0.0000133769445,
        -1.8855448e18,
        8.2494555e-29,
        2.2178014e-38,
        -6.3067724e-34,
        5.1996016e31,
        7.613263e33,
        0.00015323666,
        9.476819e36,
        -0.0005665586,
        8.8733265e-30,
        0.09273135,
        -7.7748304e33,
        4.3156234e-8,
    ];
    let common_values = &[
        (-66476.9, 2),
        (34.61204, 2),
        (73439.86, 2),
        (780.0361, 2),
        (-66297.14, 2),
        (0.2084277, 2),
        (13200.072, 2),
        (224535.31, 2),
        (3306.3638, 2),
        (5.7126203, 2),
    ];
    let sample_median = (4.601795e-39, Some(4.606578e-39));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-1.871800306879315e34),
        standard_deviation: NiceFloat(1.8597574252028948e37),
        skewness: NiceFloat(-0.04588142291892213),
        excess_kurtosis: NiceFloat(174.30920632007357),
    };
    random_finite_primitive_floats_helper::<f32>(
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f64
    let values = &[
        3.1062066405583414e-146,
        7.268713316268922e223,
        1.1685126708702853e48,
        -1.0824685183946233e146,
        3.1146051606615834e-306,
        2.2453015573637678e249,
        1.2548860979388688e-35,
        -8.287939157477945e-27,
        2.1255041535787168e-13,
        4.815129234795049e-64,
        1.3850402674408149e-17,
        -1.2535717707589619e207,
        -1.4941028004491903e142,
        4.3666119614549075e-51,
        -7.063699168119983e17,
        -7.062565582436956e90,
        1.1662950933663384e-221,
        2.1976577668343595e-97,
        -2.821294426687019e-137,
        1.225091633874841e-222,
    ];
    let common_values = &[
        (3.637321705391, 1),
        (30.80883877248, 1),
        (915366460504.2, 1),
        (9256888.416622, 1),
        (-737164661.2491, 1),
        (-81413.95043198, 1),
        (-994938.9166069, 1),
        (1.4299272196643, 1),
        (12812252.479435, 1),
        (1598038.5592174, 1),
    ];
    let sample_median = (2.7723065921722726e-308, Some(2.782073119497922e-308));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(7.922018643581054e303),
        standard_deviation: NiceFloat(f64::POSITIVE_INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_finite_primitive_floats_helper::<f64>(
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
}
