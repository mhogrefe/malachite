use itertools::Itertools;
use malachite_base::num::float::nice_float::NiceFloat;
use malachite_base::num::float::PrimitiveFloat;
use malachite_base::num::random::special_random_finite_primitive_floats;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base_test_util::stats::common_values_map::common_values_map;
use malachite_base_test_util::stats::median;
use malachite_base_test_util::stats::moments::{moment_stats, CheckedToF64, MomentStats};
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
    let xs = special_random_finite_primitive_floats::<T>(
        EXAMPLE_SEED,
        mean_exponent_numerator,
        mean_exponent_denominator,
        mean_precision_numerator,
        mean_precision_denominator,
        mean_zero_p_numerator,
        mean_zero_p_denominator,
    );
    let actual_values = xs.clone().take(50).map(NiceFloat).collect_vec();
    let actual_common_values = common_values_map(1000000, 20, xs.clone().map(NiceFloat));
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
fn test_special_random_finite_primitive_floats() {
    // f32, mean abs of exponent = 1/64, mean precision = 65/64, mean zero P = 1/4
    let values = &[
        0.0, 1.0, 1.0, -0.0, 1.0, -1.0, 0.0, -1.0, 0.0, -0.0, -1.0, -0.0, -1.0, 1.0, 1.0, -1.0,
        -1.0, -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 0.5, 1.0, -1.0, 1.0, -1.0, 1.0, 1.0,
        0.0, -1.0, -0.0, 0.0, -1.0, 1.0, -1.0, -1.0, -1.0, 0.0, 1.0, -1.0, -0.5, -1.0, -0.0, 0.0,
        0.0, 1.0,
    ];
    let common_values = &[
        (1.0, 358273),
        (-1.0, 357906),
        (0.0, 125637),
        (-0.0, 124572),
        (2.0, 5548),
        (0.5, 5531),
        (1.5, 5489),
        (-1.5, 5469),
        (-2.0, 5383),
        (-0.5, 5355),
        (-4.0, 94),
        (-0.75, 90),
        (-0.25, 89),
        (3.0, 86),
        (-3.0, 80),
        (0.25, 79),
        (4.0, 75),
        (0.75, 69),
        (-1.25, 45),
        (-1.75, 43),
    ];
    let sample_median = (0.0, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.0006923750000000453),
        standard_deviation: NiceFloat(0.8901311615854103),
        skewness: NiceFloat(-0.0023487399300697893),
        excess_kurtosis: NiceFloat(-1.3689844952614472),
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
        1.25, 1.375, 2.5, 0.0, -1.0, -1.0, -2.0, -3.5, 1.5, 2.0, -1.0, -3.0, -2.0, -0.0, -6.671875,
        -1.75, -1.5, 0.0, 3.5, -0.1875, -1.0, 0.25, 1.5, 6.0, -6.0, 6.0, -0.5, 0.125, 1.0, 0.0,
        -0.1875, -4.0, -0.0, 0.875, -7.0, -6.75, -2.5, 0.125, -2.0, -0.5, -0.875, 5.0, -16.0, 16.0,
        -3.0, 1.0, -0.0, -1.0, 8.0, 4.0,
    ];
    let common_values = &[
        (1.0, 74871),
        (-1.0, 74815),
        (0.0, 50351),
        (-0.0, 49873),
        (-0.5, 37712),
        (2.0, 37667),
        (1.5, 37661),
        (-1.5, 37408),
        (0.5, 37381),
        (-2.0, 37313),
        (3.0, 18789),
        (-0.75, 18765),
        (-3.0, 18750),
        (0.75, 18729),
        (-0.25, 18729),
        (-4.0, 18710),
        (4.0, 18693),
        (0.25, 18660),
        (6.0, 9572),
        (-1.25, 9495),
    ];
    let sample_median = (0.0, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.7730407924471795),
        standard_deviation: NiceFloat(393.0313854478049),
        skewness: NiceFloat(-313.150459800242),
        excess_kurtosis: NiceFloat(209686.8486734463),
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
        0.9171448,
        0.00000166893,
        0.013153076,
        -0.75,
        -102720.0,
        -0.012674332,
        -0.022476196,
        2.65625,
        4.2109375,
        -0.000049591064,
        -0.3943634,
        -18432.0,
        -45056.0,
        -1.4375,
        -58.0,
        0.3046875,
        -0.0018005371,
        -1302.625,
        0.21875,
        0.0018005371,
        2.7008355e-8,
        -0.04321289,
        4325376.0,
        -0.23828125,
        133.5,
        5.0,
        -1048576.0,
        -1.1920929e-7,
        0.3046875,
        -0.0062561035,
        -54525950.0,
        -4096.0,
        21.9375,
        -6.0,
        -6.1132812,
        -3.1497803,
        0.0000038146973,
        -0.027618408,
        0.000061035156,
        -0.00005314802,
        0.00020980835,
        -47.0,
        0.0043945312,
        0.009185791,
        22.78711,
        18.0,
        -0.19140625,
        -232.625,
        -0.078125,
        -6912.0,
    ];
    let common_values = &[
        (0.0, 5098),
        (-0.0, 4891),
        (1.0, 2570),
        (-1.0, 2538),
        (-2.0, 2404),
        (-1.5, 2369),
        (0.5, 2354),
        (-0.5, 2343),
        (2.0, 2213),
        (1.5, 2212),
        (3.0, 2116),
        (0.25, 2102),
        (-3.0, 2100),
        (4.0, 2095),
        (0.75, 2067),
        (-0.75, 2059),
        (-4.0, 2058),
        (-0.25, 2056),
        (-0.375, 1985),
        (-0.125, 1981),
    ];
    let sample_median = (0.0, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-1.0686635590554205e32),
        standard_deviation: NiceFloat(7.914600976576396e34),
        skewness: NiceFloat(-781.6012831450423),
        excess_kurtosis: NiceFloat(637958.481926156),
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
        0.0, -1.0, -0.0, 0.0, -1.0, 1.0, -1.0, -1.0, -1.0, 0.0, 1.0, -1.0, -0.5, -1.0, -0.0, 0.0,
        0.0, 1.0,
    ];
    let common_values = &[
        (1.0, 358273),
        (-1.0, 357906),
        (0.0, 125637),
        (-0.0, 124572),
        (2.0, 5548),
        (0.5, 5531),
        (1.5, 5489),
        (-1.5, 5469),
        (-2.0, 5383),
        (-0.5, 5355),
        (-4.0, 94),
        (-0.75, 90),
        (-0.25, 89),
        (3.0, 86),
        (-3.0, 80),
        (0.25, 79),
        (4.0, 75),
        (0.75, 69),
        (-1.25, 45),
        (-1.75, 43),
    ];
    let sample_median = (0.0, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.0006923750000000453),
        standard_deviation: NiceFloat(0.8901311615854103),
        skewness: NiceFloat(-0.0023487399300697893),
        excess_kurtosis: NiceFloat(-1.3689844952614472),
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
        1.25, 1.375, 2.5, 0.0, -1.0, -1.0, -2.0, -3.5, 1.5, 2.0, -1.0, -3.0, -2.0, -0.0, -6.671875,
        -1.75, -1.5, 0.0, 3.5, -0.1875, -1.0, 0.25, 1.5, 6.0, -6.0, 6.0, -0.5, 0.125, 1.0, 0.0,
        -0.1875, -4.0, -0.0, 0.875, -7.0, -6.75, -2.5, 0.125, -2.0, -0.5, -0.875, 5.0, -16.0, 16.0,
        -3.0, 1.0, -0.0, -1.0, 8.0, 4.0,
    ];
    let common_values = &[
        (1.0, 74871),
        (-1.0, 74815),
        (0.0, 50351),
        (-0.0, 49873),
        (-0.5, 37712),
        (2.0, 37667),
        (1.5, 37661),
        (-1.5, 37408),
        (0.5, 37381),
        (-2.0, 37312),
        (3.0, 18789),
        (-0.75, 18765),
        (-3.0, 18750),
        (0.75, 18729),
        (-0.25, 18729),
        (-4.0, 18710),
        (4.0, 18693),
        (0.25, 18660),
        (6.0, 9572),
        (-1.75, 9459),
    ];
    let sample_median = (0.0, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.7526241571662066),
        standard_deviation: NiceFloat(392.9632667945442),
        skewness: NiceFloat(-313.02203622107453),
        excess_kurtosis: NiceFloat(209792.68944511082),
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
        0.917144775390625,
        1.6689300537109375e-6,
        0.013153076171875,
        -0.75,
        -102720.0,
        -0.012674331665039062,
        -0.019363800472092407,
        3.4191773291677237,
        5.6328125,
        -0.000057220458984375,
        -0.2560577392578125,
        -26624.0,
        -61440.0,
        -1.6875,
        -34.0,
        0.4453125,
        -0.0013496266637957888,
        -1318.375,
        0.15625,
        0.001068115234375,
        2.1420419216156006e-8,
        -0.052490234375,
        7208960.0,
        -0.18359375,
        247.5,
        7.0,
        -1048576.0,
        -1.1920928955078125e-7,
        0.2578125,
        -0.004608154296875,
        -46137344.0,
        -4096.0,
        22.3125,
        -6.0,
        -6.02734375,
        -3.3521728515625,
        3.814697265625e-6,
        -0.030975341796875,
        0.00006103515625,
        -0.00003659701906144619,
        0.000202178955078125,
        -45.0,
        0.00732421875,
        0.011749267578125,
        21.310546875,
        26.0,
        -0.19921875,
        -190.625,
        -0.078125,
        -6912.0,
    ];
    let common_values = &[
        (0.0, 5098),
        (-0.0, 4891),
        (-1.0, 2438),
        (1.0, 2334),
        (0.5, 2199),
        (2.0, 2180),
        (1.5, 2158),
        (-0.5, 2128),
        (-1.5, 2117),
        (-2.0, 2081),
        (-3.0, 1982),
        (-4.0, 1936),
        (3.0, 1931),
        (0.25, 1909),
        (-0.25, 1907),
        (0.75, 1901),
        (4.0, 1880),
        (-0.75, 1879),
        (0.125, 1874),
        (-0.375, 1826),
    ];
    let sample_median = (0.0, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-6.244204733961268e38),
        standard_deviation: NiceFloat(6.244208655594658e41),
        skewness: NiceFloat(-999.9984999983285),
        excess_kurtosis: NiceFloat(999994.9999996312),
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
