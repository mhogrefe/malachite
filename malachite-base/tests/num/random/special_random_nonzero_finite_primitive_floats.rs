use itertools::Itertools;
use malachite_base::num::float::nice_float::NiceFloat;
use malachite_base::num::float::PrimitiveFloat;
use malachite_base::num::random::special_random_nonzero_finite_primitive_floats;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base_test_util::stats::common_values_map::common_values_map;
use malachite_base_test_util::stats::median;
use malachite_base_test_util::stats::moments::{moment_stats, CheckedToF64, MomentStats};
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
    let xs = special_random_nonzero_finite_primitive_floats::<T>(
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
fn test_special_random_nonzero_finite_primitive_floats() {
    // f32, mean abs of exponent = 1/64, mean precision = 65/64
    let values = &[
        -1.0, -1.0, -1.0, 1.0, -1.0, 1.0, -1.0, -1.0, -1.0, 1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0,
        -1.0, -1.0, -1.0, -0.5, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0,
        -1.0, 1.0, 0.5, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0,
        1.0,
    ];
    let common_values = &[
        (1.0, 478039),
        (-1.0, 477030),
        (-2.0, 7426),
        (1.5, 7378),
        (0.5, 7290),
        (-0.5, 7277),
        (-1.5, 7246),
        (2.0, 7189),
        (0.25, 121),
        (-3.0, 120),
        (0.75, 119),
        (-4.0, 114),
        (4.0, 113),
        (-0.25, 105),
        (-0.75, 101),
        (3.0, 92),
        (-1.25, 55),
        (-1.75, 55),
        (1.75, 52),
        (1.25, 48),
    ];
    let sample_median = (0.5, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.0006355000000000012),
        standard_deviation: NiceFloat(1.0279559950860144),
        skewness: NiceFloat(-0.004345838377460419),
        excess_kurtosis: NiceFloat(-1.781181897251214),
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
        -1.25, -1.125, -3.5, 1.0, -1.0, 2.0, -2.5, -1.5, -2.0, 1.0, 3.0, 2.0, -5.109375, -1.25,
        1.5, -3.5, -0.1875, -1.0, -0.25, -1.5, 6.0, 6.0, -6.0, -0.5, 0.125, -1.0, -0.1875, 4.0,
        0.625, -5.0, -7.25, 3.5, -0.125, 2.0, 0.5, -0.875, 7.0, -16.0, 16.0, -3.0, 1.0, 1.0, 8.0,
        -4.0, 0.375, 1.328125, -0.25, 0.375, 1.75, 0.0625,
    ];
    let common_values = &[
        (1.0, 83626),
        (-1.0, 82821),
        (-0.5, 41858),
        (1.5, 41785),
        (2.0, 41755),
        (-2.0, 41721),
        (0.5, 41621),
        (-1.5, 41478),
        (3.0, 20939),
        (-0.25, 20837),
        (0.75, 20825),
        (4.0, 20807),
        (-3.0, 20797),
        (-0.75, 20769),
        (0.25, 20757),
        (-4.0, 20702),
        (-6.0, 10611),
        (1.25, 10533),
        (8.0, 10508),
        (-0.125, 10484),
    ];
    let sample_median = (0.001953125, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.3223238367137598),
        standard_deviation: NiceFloat(394.3856248117776),
        skewness: NiceFloat(-240.71896098861947),
        excess_kurtosis: NiceFloat(167164.9063327764),
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
        -0.5688782,
        -0.00000166893,
        -0.015167236,
        0.75,
        -92864.0,
        0.012998581,
        -0.02949524,
        -3.28125,
        -7.5703125,
        0.00005722046,
        0.33540344,
        18432.0,
        -61440.0,
        -1.4375,
        50.0,
        -0.4140625,
        -0.001739502,
        -1388.125,
        -0.15625,
        -0.001373291,
        2.7008355e-8,
        0.03930664,
        -7995392.0,
        -0.23046875,
        203.5,
        -7.0,
        -1048576.0,
        1.1920929e-7,
        0.4765625,
        -0.0065612793,
        -46137344.0,
        4096.0,
        -18.8125,
        6.0,
        6.5664062,
        -2.1781006,
        0.0000038146973,
        -0.022613525,
        0.000061035156,
        -0.0000440937,
        0.00015640259,
        33.0,
        0.0043945312,
        -0.013641357,
        30.759766,
        26.0,
        -0.19921875,
        230.125,
        0.078125,
        4352.0,
    ];
    let common_values = &[
        (-1.0, 2622),
        (1.0, 2544),
        (-0.5, 2405),
        (-2.0, 2399),
        (0.5, 2346),
        (1.5, 2333),
        (-1.5, 2299),
        (2.0, 2257),
        (-3.0, 2158),
        (0.25, 2132),
        (-4.0, 2099),
        (3.0, 2097),
        (0.75, 2090),
        (4.0, 2085),
        (-0.25, 2079),
        (-0.75, 2076),
        (-0.125, 2016),
        (0.375, 2003),
        (0.125, 1979),
        (6.0, 1932),
    ];
    let sample_median = (1.7511207e-20, Some(1.8211208e-20));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-7.696078660472416e30),
        standard_deviation: NiceFloat(6.315739402347176e34),
        skewness: NiceFloat(-157.2259035396599),
        excess_kurtosis: NiceFloat(510735.9759278179),
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
        -1.0, -1.0, -1.0, 1.0, -1.0, 1.0, -1.0, -1.0, -1.0, 1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0,
        -1.0, -1.0, -1.0, -0.5, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0,
        -1.0, 1.0, 0.5, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0,
        1.0,
    ];
    let common_values = &[
        (1.0, 478039),
        (-1.0, 477030),
        (-2.0, 7426),
        (1.5, 7378),
        (0.5, 7290),
        (-0.5, 7277),
        (-1.5, 7246),
        (2.0, 7189),
        (0.25, 121),
        (-3.0, 120),
        (0.75, 119),
        (-4.0, 114),
        (4.0, 113),
        (-0.25, 105),
        (-0.75, 101),
        (3.0, 92),
        (-1.25, 55),
        (-1.75, 55),
        (1.75, 52),
        (1.25, 48),
    ];
    let sample_median = (0.5, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.0006355000000000012),
        standard_deviation: NiceFloat(1.0279559950860144),
        skewness: NiceFloat(-0.004345838377460419),
        excess_kurtosis: NiceFloat(-1.781181897251214),
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
        -1.25, -1.125, -3.5, 1.0, -1.0, 2.0, -2.5, -1.5, -2.0, 1.0, 3.0, 2.0, -5.109375, -1.25,
        1.5, -3.5, -0.1875, -1.0, -0.25, -1.5, 6.0, 6.0, -6.0, -0.5, 0.125, -1.0, -0.1875, 4.0,
        0.625, -5.0, -7.25, 3.5, -0.125, 2.0, 0.5, -0.875, 7.0, -16.0, 16.0, -3.0, 1.0, 1.0, 8.0,
        -4.0, 0.375, 1.328125, -0.25, 0.375, 1.75, 0.0625,
    ];
    let common_values = &[
        (1.0, 83626),
        (-1.0, 82821),
        (-0.5, 41858),
        (1.5, 41785),
        (2.0, 41754),
        (-2.0, 41721),
        (0.5, 41621),
        (-1.5, 41478),
        (3.0, 20939),
        (-0.25, 20837),
        (0.75, 20825),
        (4.0, 20807),
        (-3.0, 20797),
        (-0.75, 20769),
        (0.25, 20757),
        (-4.0, 20702),
        (-6.0, 10611),
        (8.0, 10508),
        (-0.125, 10484),
        (1.75, 10482),
    ];
    let sample_median = (0.001953125, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.3063337925478731),
        standard_deviation: NiceFloat(394.4059226322049),
        skewness: NiceFloat(-240.54930024583095),
        excess_kurtosis: NiceFloat(167137.45677843541),
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
        -0.568878173828125,
        -1.6689300537109375e-6,
        -0.015167236328125,
        0.75,
        -92864.0,
        0.012998580932617188,
        -0.02291788616003032,
        -2.762363215908408,
        -6.1796875,
        0.000041961669921875,
        0.4728546142578125,
        22528.0,
        -61440.0,
        -1.4375,
        38.0,
        -0.4296875,
        -0.0013771892390650464,
        -1069.625,
        -0.21875,
        -0.001739501953125,
        1.7695128917694092e-8,
        0.055419921875,
        -5111808.0,
        -0.23828125,
        136.5,
        -5.0,
        -1048576.0,
        1.1920928955078125e-7,
        0.3828125,
        -0.006805419921875,
        -54525952.0,
        4096.0,
        -22.0625,
        6.0,
        6.43359375,
        -2.0997314453125,
        3.814697265625e-6,
        -0.029205322265625,
        0.00006103515625,
        -0.00003288569860160351,
        0.000209808349609375,
        37.0,
        0.00537109375,
        -0.008087158203125,
        21.623046875,
        26.0,
        -0.17578125,
        242.625,
        0.078125,
        5376.0,
    ];
    let common_values = &[
        (-1.0, 2432),
        (1.0, 2389),
        (-0.5, 2186),
        (0.5, 2185),
        (1.5, 2164),
        (2.0, 2159),
        (-1.5, 2159),
        (-2.0, 2142),
        (-3.0, 2001),
        (4.0, 1958),
        (0.25, 1958),
        (3.0, 1943),
        (0.75, 1939),
        (-0.25, 1902),
        (-4.0, 1890),
        (-0.75, 1888),
        (-0.375, 1847),
        (0.125, 1845),
        (8.0, 1830),
        (-0.125, 1827),
    ];
    let sample_median = (2.5199230180815435e-20, Some(2.541098841762901e-20));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(4.460154236256426e38),
        standard_deviation: NiceFloat(4.460149039703899e41),
        skewness: NiceFloat(999.9984999973713),
        excess_kurtosis: NiceFloat(999994.9999983368),
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
