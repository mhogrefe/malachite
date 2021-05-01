use itertools::Itertools;
use malachite_base::num::float::nice_float::NiceFloat;
use malachite_base::num::float::PrimitiveFloat;
use malachite_base::num::random::special_random_negative_finite_primitive_floats;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base_test_util::stats::common_values_map::common_values_map;
use malachite_base_test_util::stats::median;
use malachite_base_test_util::stats::moments::{moment_stats, CheckedToF64, MomentStats};
use std::panic::catch_unwind;

fn special_random_negative_finite_primitive_floats_helper<T: CheckedToF64 + PrimitiveFloat>(
    mean_exponent_numerator: u64,
    mean_exponent_denominator: u64,
    mean_precision_numerator: u64,
    mean_precision_denominator: u64,
    expected_values: &[T],
    expected_common_values: &[(T, usize)],
    expected_median: (T, Option<T>),
    expected_moment_stats: MomentStats,
) {
    let xs = special_random_negative_finite_primitive_floats::<T>(
        EXAMPLE_SEED,
        mean_exponent_numerator,
        mean_exponent_denominator,
        mean_precision_numerator,
        mean_precision_denominator,
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
fn test_special_random_negative_finite_primitive_floats() {
    // f32, mean abs of exponent = 1/64, mean precision = 65/64
    let values = &[
        -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0,
        -1.0, -1.0, -1.0, -1.0, -0.5, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0,
        -1.0, -1.0, -1.0, -1.0, -0.5, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0,
        -1.0, -1.0, -1.0, -1.0, -1.0,
    ];
    let common_values = &[
        (-1.0, 955069),
        (-1.5, 14624),
        (-2.0, 14615),
        (-0.5, 14567),
        (-4.0, 227),
        (-0.25, 226),
        (-0.75, 220),
        (-3.0, 212),
        (-1.25, 108),
        (-1.75, 102),
        (-6.0, 4),
        (-0.125, 4),
        (-0.375, 4),
        (-2.5, 3),
        (-3.5, 3),
        (-8.0, 3),
        (-1.125, 3),
        (-0.625, 2),
        (-7.0, 1),
        (-0.875, 1),
    ];
    let sample_median = (-1.0, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-1.015681499999981),
        standard_deviation: NiceFloat(0.15835874031843308),
        skewness: NiceFloat(-5.793817851839372),
        excess_kurtosis: NiceFloat(73.29176537824256),
    };
    special_random_negative_finite_primitive_floats_helper::<f32>(
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
        -1.25, -1.625, -2.5, -1.0, -1.0, -2.0, -3.5, -1.5, -2.0, -1.0, -3.0, -2.0, -5.796875,
        -1.75, -1.5, -3.5, -0.1875, -1.0, -0.25, -1.5, -6.0, -6.0, -6.0, -0.5, -0.125, -1.0,
        -0.1875, -4.0, -0.875, -7.0, -7.75, -3.5, -0.125, -2.0, -0.5, -0.875, -7.0, -16.0, -16.0,
        -3.0, -1.0, -1.0, -8.0, -4.0, -0.375, -1.578125, -0.25, -0.375, -1.25, -0.0625,
    ];
    let common_values = &[
        (-1.0, 166447),
        (-0.5, 83479),
        (-2.0, 83476),
        (-1.5, 83263),
        (-3.0, 41736),
        (-0.25, 41594),
        (-0.75, 41594),
        (-4.0, 41509),
        (-6.0, 21006),
        (-0.125, 20907),
        (-1.75, 20901),
        (-1.25, 20846),
        (-8.0, 20803),
        (-0.375, 20766),
        (-16.0, 10498),
        (-0.875, 10446),
        (-3.5, 10396),
        (-0.1875, 10396),
        (-2.5, 10391),
        (-12.0, 10386),
    ];
    let sample_median = (-1.03125, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-7.570578568668992),
        standard_deviation: NiceFloat(393.04134139946035),
        skewness: NiceFloat(-353.5170834477369),
        excess_kurtosis: NiceFloat(169352.25990646527),
    };
    special_random_negative_finite_primitive_floats_helper::<f32>(
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
        -0.86257935,
        -0.00000166893,
        -0.015594482,
        -0.75,
        -70336.0,
        -0.012937546,
        -0.021591187,
        -3.03125,
        -7.4140625,
        -0.00004196167,
        -0.30628967,
        -22528.0,
        -36864.0,
        -1.0625,
        -46.0,
        -0.3203125,
        -0.0018615723,
        -1637.875,
        -0.15625,
        -0.0010681152,
        -1.5832484e-8,
        -0.033447266,
        -4587520.0,
        -0.13671875,
        -194.5,
        -5.0,
        -1048576.0,
        -1.1920929e-7,
        -0.3359375,
        -0.007659912,
        -37748736.0,
        -4096.0,
        -16.1875,
        -6.0,
        -4.9492188,
        -3.5924072,
        -0.0000038146973,
        -0.027740479,
        -0.000061035156,
        -0.00003661844,
        -0.0001487732,
        -47.0,
        -0.0043945312,
        -0.0113220215,
        -26.845703,
        -22.0,
        -0.17578125,
        -196.375,
        -0.109375,
        -5888.0,
    ];
    let common_values = &[
        (-1.0, 5166),
        (-0.5, 4751),
        (-2.0, 4656),
        (-1.5, 4632),
        (-3.0, 4255),
        (-0.25, 4211),
        (-4.0, 4184),
        (-0.75, 4166),
        (-0.125, 3995),
        (-0.375, 3925),
        (-6.0, 3843),
        (-8.0, 3831),
        (-0.0625, 3533),
        (-0.1875, 3514),
        (-12.0, 3481),
        (-16.0, 3479),
        (-32.0, 3265),
        (-0.09375, 3182),
        (-0.03125, 3154),
        (-24.0, 3046),
    ];
    let sample_median = (-1.484375, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-1.0050435633895413e32),
        standard_deviation: NiceFloat(7.154840344538984e34),
        skewness: NiceFloat(-737.2265636121816),
        excess_kurtosis: NiceFloat(556494.2306446731),
    };
    special_random_negative_finite_primitive_floats_helper::<f32>(
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
        -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0,
        -1.0, -1.0, -1.0, -1.0, -0.5, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0,
        -1.0, -1.0, -1.0, -1.0, -0.5, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0,
        -1.0, -1.0, -1.0, -1.0, -1.0,
    ];
    let common_values = &[
        (-1.0, 955069),
        (-1.5, 14624),
        (-2.0, 14615),
        (-0.5, 14567),
        (-4.0, 227),
        (-0.25, 226),
        (-0.75, 220),
        (-3.0, 212),
        (-1.25, 108),
        (-1.75, 102),
        (-6.0, 4),
        (-0.125, 4),
        (-0.375, 4),
        (-2.5, 3),
        (-3.5, 3),
        (-8.0, 3),
        (-1.125, 3),
        (-0.625, 2),
        (-7.0, 1),
        (-0.875, 1),
    ];
    let sample_median = (-1.0, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-1.015681499999981),
        standard_deviation: NiceFloat(0.15835874031843308),
        skewness: NiceFloat(-5.793817851839372),
        excess_kurtosis: NiceFloat(73.29176537824256),
    };
    special_random_negative_finite_primitive_floats_helper::<f64>(
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
        -1.25, -1.625, -2.5, -1.0, -1.0, -2.0, -3.5, -1.5, -2.0, -1.0, -3.0, -2.0, -5.796875,
        -1.75, -1.5, -3.5, -0.1875, -1.0, -0.25, -1.5, -6.0, -6.0, -6.0, -0.5, -0.125, -1.0,
        -0.1875, -4.0, -0.875, -7.0, -7.75, -3.5, -0.125, -2.0, -0.5, -0.875, -7.0, -16.0, -16.0,
        -3.0, -1.0, -1.0, -8.0, -4.0, -0.375, -1.578125, -0.25, -0.375, -1.25, -0.0625,
    ];
    let common_values = &[
        (-1.0, 166447),
        (-0.5, 83479),
        (-2.0, 83475),
        (-1.5, 83263),
        (-3.0, 41736),
        (-0.25, 41594),
        (-0.75, 41594),
        (-4.0, 41509),
        (-6.0, 21006),
        (-1.75, 20943),
        (-0.125, 20907),
        (-1.25, 20804),
        (-8.0, 20803),
        (-0.375, 20766),
        (-16.0, 10498),
        (-3.5, 10475),
        (-0.625, 10399),
        (-0.1875, 10395),
        (-12.0, 10386),
        (-0.875, 10386),
    ];
    let sample_median = (-1.03125, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-7.60099607639277),
        standard_deviation: NiceFloat(388.5128048991303),
        skewness: NiceFloat(-326.40887271620636),
        excess_kurtosis: NiceFloat(143378.9055688394),
    };
    special_random_negative_finite_primitive_floats_helper::<f64>(
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
        -0.862579345703125,
        -1.6689300537109375e-6,
        -0.015594482421875,
        -0.75,
        -70336.0,
        -0.012937545776367188,
        -0.02503855562008539,
        -2.3733139764517546,
        -5.3046875,
        -0.000034332275390625,
        -0.2666168212890625,
        -22528.0,
        -36864.0,
        -1.3125,
        -34.0,
        -0.3359375,
        -0.0015225456645566737,
        -1431.625,
        -0.21875,
        -0.001068115234375,
        -2.8870999813079834e-8,
        -0.042236328125,
        -5898240.0,
        -0.17578125,
        -230.5,
        -7.0,
        -1048576.0,
        -1.1920928955078125e-7,
        -0.3828125,
        -0.005645751953125,
        -37748736.0,
        -4096.0,
        -22.8125,
        -6.0,
        -5.60546875,
        -2.9417724609375,
        -3.814697265625e-6,
        -0.030548095703125,
        -0.00006103515625,
        -0.00004168064333498478,
        -0.000202178955078125,
        -53.0,
        -0.00537109375,
        -0.015045166015625,
        -23.646484375,
        -26.0,
        -0.14453125,
        -186.375,
        -0.109375,
        -6400.0,
    ];
    let common_values = &[
        (-1.0, 4821),
        (-0.5, 4371),
        (-1.5, 4323),
        (-2.0, 4301),
        (-3.0, 3944),
        (-0.25, 3860),
        (-4.0, 3848),
        (-0.75, 3827),
        (-0.125, 3672),
        (-0.375, 3657),
        (-8.0, 3583),
        (-6.0, 3540),
        (-0.0625, 3286),
        (-12.0, 3244),
        (-16.0, 3222),
        (-0.1875, 3193),
        (-32.0, 2984),
        (-0.03125, 2928),
        (-0.09375, 2904),
        (-24.0, 2842),
    ];
    let sample_median = (-1.48828125, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-4.460155000051196e38),
        standard_deviation: NiceFloat(4.4601490397031e41),
        skewness: NiceFloat(-999.9984999973876),
        excess_kurtosis: NiceFloat(999994.9999983647),
    };
    special_random_negative_finite_primitive_floats_helper::<f64>(
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

fn special_random_negative_finite_primitive_floats_fail_helper<T: PrimitiveFloat>() {
    assert_panic!(special_random_negative_finite_primitive_floats::<T>(
        EXAMPLE_SEED,
        0,
        1,
        10,
        1
    ));
    assert_panic!(special_random_negative_finite_primitive_floats::<T>(
        EXAMPLE_SEED,
        1,
        0,
        10,
        1
    ));
    assert_panic!(special_random_negative_finite_primitive_floats::<T>(
        EXAMPLE_SEED,
        10,
        1,
        1,
        1
    ));
    assert_panic!(special_random_negative_finite_primitive_floats::<T>(
        EXAMPLE_SEED,
        10,
        1,
        1,
        0
    ));
}

#[test]
fn special_random_negative_finite_primitive_floats_fail() {
    apply_fn_to_primitive_floats!(special_random_negative_finite_primitive_floats_fail_helper);
}
