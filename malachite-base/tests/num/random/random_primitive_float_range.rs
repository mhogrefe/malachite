// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::NegativeInfinity;
use malachite_base::num::float::NiceFloat;
use malachite_base::num::random::random_primitive_float_range;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::num::random::random_primitive_floats_helper_helper;
use malachite_base::test_util::stats::moments::{CheckedToF64, MomentStats};
use std::panic::catch_unwind;

fn random_primitive_float_range_helper<T: CheckedToF64 + PrimitiveFloat>(
    a: T,
    b: T,
    expected_values: &[T],
    expected_common_values: &[(T, usize)],
    expected_median: (T, Option<T>),
    expected_moment_stats: MomentStats,
) {
    random_primitive_floats_helper_helper(
        random_primitive_float_range::<T>(EXAMPLE_SEED, a, b),
        expected_values,
        expected_common_values,
        expected_median,
        expected_moment_stats,
    );
}

#[test]
fn test_random_primitive_float_range() {
    // f32, a = 1.0, b = 2.0
    let values = &[
        1.5463697, 1.2951918, 1.7384838, 1.2143862, 1.1419607, 1.0917295, 1.7257521, 1.849941,
        1.1442195, 1.363777, 1.052571, 1.0717841, 1.9104315, 1.3754328, 1.590667, 1.0705026,
        1.8980603, 1.8630176, 1.0212592, 1.3380667,
    ];
    let common_values = &[
        (1.9376882, 5),
        (1.012385, 4),
        (1.439915, 4),
        (1.709473, 4),
        (1.754993, 4),
        (1.944844, 4),
        (1.971242, 4),
        (1.978845, 4),
        (1.0289025, 4),
        (1.0466498, 4),
    ];
    let sample_median = (1.499921, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1.49979057649457),
        standard_deviation: NiceFloat(0.2887387766808365),
        skewness: NiceFloat(0.0002622267624830283),
        excess_kurtosis: NiceFloat(-1.1997935828388204),
    };
    random_primitive_float_range_helper::<f32>(
        1.0,
        2.0,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f32, a = -0.1, b = 0.1
    let values = &[
        5.664681e-11,
        1.2492925e-35,
        2.3242339e-29,
        4.699183e-7,
        -2.8244436e-36,
        -2.264039e-37,
        -0.0000017299129,
        1.40616e-23,
        2.7418007e-27,
        1.5418819e-16,
        -1.8473076e-36,
        -2.4935917e-21,
        -3.373897e-37,
        -7.5386525e-15,
        -2.2595721e-7,
        -8.293393e-39,
        0.0025248893,
        1.1819218e-25,
        2.3384073e-23,
        3.1464167e-24,
    ];
    let common_values = &[
        (0.02590246, 2),
        (-0.09233444, 2),
        (0.001610253, 2),
        (0.010553952, 2),
        (0.020663222, 2),
        (0.031000609, 2),
        (1.30495e-38, 2),
        (1.409154e-8, 2),
        (2.599722e-7, 2),
        (3.67508e-29, 2),
    ];
    let sample_median = (-1.472737e-39, Some(-1.471169e-39));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-3.190292459186594e-6),
        standard_deviation: NiceFloat(0.007506907081695582),
        skewness: NiceFloat(-0.02559343794273501),
        excess_kurtosis: NiceFloat(84.97988219106435),
    };
    random_primitive_float_range_helper::<f32>(
        -0.1,
        0.1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f32, a = e, b = π
    let values = &[
        2.811021, 3.0798163, 2.8497639, 2.9021935, 3.0803769, 3.0796993, 3.088304, 2.872187,
        2.8092258, 2.7708528, 3.0054183, 2.7851858, 2.745991, 2.9290476, 2.913056, 2.899723,
        2.9672115, 2.875196, 3.01054, 3.0299006,
    ];
    let common_values = &[
        (2.7395, 7),
        (2.7335808, 7),
        (2.8363338, 7),
        (3.0879333, 7),
        (2.760186, 6),
        (2.799341, 6),
        (2.933202, 6),
        (2.978166, 6),
        (3.012332, 6),
        (3.034496, 6),
    ];
    let sample_median = (2.9301434, Some(2.930144));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(2.9300705904196347),
        standard_deviation: NiceFloat(0.12218018336191779),
        skewness: NiceFloat(-0.0024072138827345158),
        excess_kurtosis: NiceFloat(-1.1980037439170255),
    };
    random_primitive_float_range_helper::<f32>(
        core::f32::consts::E,
        core::f32::consts::PI,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f32, a = 100.0, b = 101.0
    let values = &[
        100.96766, 100.10573, 100.102974, 100.47697, 100.441444, 100.94259, 100.696365, 100.36691,
        100.79254, 100.435005, 100.23124, 100.153755, 100.25385, 100.64986, 100.26314, 100.148544,
        100.28187, 100.3743, 100.18771, 100.901344,
    ];
    let common_values = &[
        (100.15877, 24),
        (100.081535, 22),
        (100.26679, 21),
        (100.56587, 21),
        (100.894196, 21),
        (100.3593, 20),
        (100.4054, 20),
        (100.30979, 20),
        (100.45853, 20),
        (100.49529, 20),
    ];
    let sample_median = (100.50088, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(100.50054111846543),
        standard_deviation: NiceFloat(0.2888116297082562),
        skewness: NiceFloat(-0.003221278138738849),
        excess_kurtosis: NiceFloat(-1.2016989304148467),
    };
    random_primitive_float_range_helper::<f32>(
        100.0,
        101.0,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f32, a = 1.0e38, b = Infinity
    let values = &[
        1.4647999e38,
        3.1018272e38,
        1.582411e38,
        1.5544886e38,
        1.5924082e38,
        2.9619212e38,
        2.8168304e38,
        2.9816339e38,
        1.2098325e38,
        2.5528384e38,
        1.0473973e38,
        2.2168899e38,
        1.8072246e38,
        1.732986e38,
        1.0828477e38,
        1.3966511e38,
        2.61352e38,
        1.6959917e38,
        1.727243e38,
        2.8140436e38,
    ];
    let common_values = &[
        (1.223221e38, 4),
        (1.372136e38, 4),
        (1.0892582e38, 4),
        (1.4897022e38, 4),
        (1.5085965e38, 4),
        (1.5266252e38, 4),
        (1.8360457e38, 4),
        (2.5784374e38, 4),
        (2.6144523e38, 4),
        (2.7852527e38, 4),
    ];
    let sample_median = (1.8507265e38, Some(1.8507311e38));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(2.0095713198371904e38),
        standard_deviation: NiceFloat(7.129528670871142e37),
        skewness: NiceFloat(0.37808793164351623),
        excess_kurtosis: NiceFloat(-1.168840184381319),
    };
    random_primitive_float_range_helper::<f32>(
        1.0e38,
        f32::INFINITY,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f32, a = -f32::MIN_POSITIVE_SUBNORMAL, b = f32::MIN_POSITIVE_SUBNORMAL
    let values = &[
        -0.0, -1.0e-45, -0.0, 0.0, -0.0, -0.0, -1.0e-45, -0.0, -1.0e-45, 0.0, -0.0, -1.0e-45, -0.0,
        0.0, 0.0, -1.0e-45, -0.0, -1.0e-45, 0.0, 0.0,
    ];
    let common_values = &[(-0.0, 333784), (0.0, 333516), (-1.0e-45, 332700)];
    let sample_median = (-0.0, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-4.662119990808644e-46),
        standard_deviation: NiceFloat(6.602643154251322e-46),
        skewness: NiceFloat(-0.7101318209186737),
        excess_kurtosis: NiceFloat(-1.4957127969187527),
    };
    random_primitive_float_range_helper::<f32>(
        -f32::MIN_POSITIVE_SUBNORMAL,
        f32::MIN_POSITIVE_SUBNORMAL,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f32, a = -0.0, b = f32::MIN_POSITIVE_SUBNORMAL
    let values = &[
        0.0, -0.0, -0.0, -0.0, 0.0, 0.0, 0.0, -0.0, 0.0, 0.0, 0.0, 0.0, -0.0, 0.0, 0.0, 0.0, 0.0,
        -0.0, 0.0, -0.0,
    ];
    let common_values = &[(0.0, 500473), (-0.0, 499527)];
    let sample_median = (0.0, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.0),
        standard_deviation: NiceFloat(0.0),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_primitive_float_range_helper::<f32>(
        -0.0,
        f32::MIN_POSITIVE_SUBNORMAL,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f32, a = 0.0, b = f32::MIN_POSITIVE_SUBNORMAL
    let values = &[0.0; 20];
    let common_values = &[(0.0, 1000000)];
    let sample_median = (0.0, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.0),
        standard_deviation: NiceFloat(0.0),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_primitive_float_range_helper::<f32>(
        0.0,
        f32::MIN_POSITIVE_SUBNORMAL,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f32, a = -f32::MIN_POSITIVE_SUBNORMAL, b = -0.0
    let values = &[-1.0e-45; 20];
    let common_values = &[(-1.0e-45, 1000000)];
    let sample_median = (-1.0e-45, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-1.401298464324817e-45),
        standard_deviation: NiceFloat(0.0),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_primitive_float_range_helper::<f32>(
        -f32::MIN_POSITIVE_SUBNORMAL,
        -0.0,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f32, a = -f32::MIN_POSITIVE_SUBNORMAL, b = 0.0
    let values = &[
        -0.0, -1.0e-45, -1.0e-45, -1.0e-45, -0.0, -0.0, -0.0, -1.0e-45, -0.0, -0.0, -0.0, -0.0,
        -1.0e-45, -0.0, -0.0, -0.0, -0.0, -1.0e-45, -0.0, -1.0e-45,
    ];
    let common_values = &[(-0.0, 500473), (-1.0e-45, 499527)];
    let sample_median = (-0.0, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-6.9998641798878095e-46),
        standard_deviation: NiceFloat(7.006492689759787e-46),
        skewness: NiceFloat(-0.0018920008465908337),
        excess_kurtosis: NiceFloat(-1.9999964203328955),
    };
    random_primitive_float_range_helper::<f32>(
        -f32::MIN_POSITIVE_SUBNORMAL,
        0.0,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f32, a = -Infinity, b = Infinity
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
    random_primitive_float_range_helper::<f32>(
        f32::NEGATIVE_INFINITY,
        f32::INFINITY,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f32, a = -0.0, b = 0.0
    let values = &[-0.0; 20];
    let common_values = &[(-0.0, 1000000)];
    let sample_median = (-0.0, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.0),
        standard_deviation: NiceFloat(0.0),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_primitive_float_range_helper::<f32>(
        -0.0,
        0.0,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f64, a = 1.0, b = 2.0
    let values = &[
        1.5514873723431857,
        1.7356480435333936,
        1.2240680379087014,
        1.5721098095143498,
        1.445723211731554,
        1.443348441346778,
        1.128043347677334,
        1.9657544165271619,
        1.259133073045527,
        1.9463717627559034,
        1.827615676661706,
        1.3546147198266414,
        1.3547277462886724,
        1.6644379935168552,
        1.7300004987549573,
        1.1347106338290753,
        1.6337434960012935,
        1.9398684976828995,
        1.5480087631774717,
        1.5114010060819247,
    ];
    let common_values = &[
        (1.3443697926, 1),
        (1.3820769412, 1),
        (1.4136496448, 1),
        (1.05230401048, 1),
        (1.06345642396, 1),
        (1.08636222403, 1),
        (1.08890959097, 1),
        (1.10364420294, 1),
        (1.17100333598, 1),
        (1.21003284406, 1),
    ];
    let sample_median = (1.4997587655631748, Some(1.4997590736389839));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1.5002317198585347),
        standard_deviation: NiceFloat(0.2886284765385832),
        skewness: NiceFloat(0.0005691088300059665),
        excess_kurtosis: NiceFloat(-1.1997562526471726),
    };
    random_primitive_float_range_helper::<f64>(
        1.0,
        2.0,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f64, a = -0.1, b = 0.1
    let values = &[
        -7.283095678343042e-164,
        4.085787276271492e-169,
        -1.6885972585325658e-191,
        -1.5059586906723643e-66,
        -6.637230143944272e-36,
        2.0111059084569595e-54,
        -3.2171834547379634e-195,
        -1.4304898186595632e-260,
        -5.910214544689135e-300,
        4.248352948466203e-63,
        -3.6882240870537675e-31,
        8.12900376877632e-277,
        8.630695763640745e-286,
        -2.7842211494385523e-123,
        -4.271131813514248e-164,
        1.613930919542087e-167,
        -5.39182068994581e-107,
        -1.4532461060667818e-9,
        -1.9793582955127234e-289,
        5.420373932282823e-196,
    ];
    let common_values = &[
        (5.62015686679e-6, 1),
        (-0.09576016351376, 1),
        (-3.9141428595e-60, 1),
        (-4.5355157777e-28, 1),
        (0.008342058495796, 1),
        (0.012335893098144, 1),
        (0.014079819535342, 1),
        (0.014718940078426, 1),
        (0.031741597598458, 1),
        (0.033991243007763, 1),
    ];
    let sample_median = (1.566509212534917e-309, Some(1.56863192120459e-309));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(4.4695816858634463e-7),
        standard_deviation: NiceFloat(0.002635102953882735),
        skewness: NiceFloat(0.27772415900587566),
        excess_kurtosis: NiceFloat(707.152044677798),
    };
    random_primitive_float_range_helper::<f64>(
        -0.1,
        0.1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f64, a = e, b = π
    let values = &[
        2.8212565731454164,
        3.103466176726195,
        2.888459041537496,
        2.94833744629582,
        2.9285662350147255,
        3.059002590500268,
        2.808432051804475,
        3.077033595571352,
        3.0898242789403123,
        3.093937352570613,
        2.7596383425151814,
        3.1049928702292573,
        2.7453107067232327,
        3.0779370799622736,
        2.9748071250720396,
        2.927927166467895,
        2.81511226878185,
        2.928920013122519,
        2.964625285981546,
        3.046598518604858,
    ];
    let common_values = &[
        (2.7683806707, 1),
        (2.8058681766, 1),
        (2.8522842725, 1),
        (2.8873246989, 1),
        (2.72492950364, 1),
        (2.73164898148, 1),
        (2.73476073924, 1),
        (2.73598990929, 1),
        (2.73653142351, 1),
        (2.74563905301, 1),
    ];
    let sample_median = (2.930132942011006, Some(2.9301336276615912));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(2.929964069913902),
        standard_deviation: NiceFloat(0.12226749948876238),
        skewness: NiceFloat(-0.0013881669668324012),
        excess_kurtosis: NiceFloat(-1.2003731669148405),
    };
    random_primitive_float_range_helper::<f64>(
        core::f64::consts::E,
        core::f64::consts::PI,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f64, a = 100.0, b = 101.0
    let values = &[
        100.29519182996388,
        100.21438631278083,
        100.09172953867444,
        100.84994110175992,
        100.36377705862755,
        100.07178414494646,
        100.37543295746225,
        100.07050270922983,
        100.86301766610865,
        100.33806669965496,
        100.35496099272225,
        100.93577122524063,
        100.00524419289253,
        100.29363379918549,
        100.98421354539467,
        100.68228296091216,
        100.93250012468873,
        100.1553701412652,
        100.95333990532461,
        100.2218641465098,
    ];
    let common_values = &[
        (100.10137554, 1),
        (100.34387327, 1),
        (100.223865218, 1),
        (100.237336607, 1),
        (100.241016737, 1),
        (100.358275298, 1),
        (100.490668361, 1),
        (100.563824325, 1),
        (100.567992111, 1),
        (100.619353436, 1),
    ];
    let sample_median = (100.49999381186375, Some(100.49999461609349));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(100.4998603968099),
        standard_deviation: NiceFloat(0.28878031747138194),
        skewness: NiceFloat(-0.00018856944159801264),
        excess_kurtosis: NiceFloat(-1.2006169795569301),
    };
    random_primitive_float_range_helper::<f64>(
        100.0,
        101.0,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f64, a = 1.0e38, b = Infinity
    let values = &[
        1.3219190533477493e200,
        3.652437632585123e180,
        2.0420353527516904e248,
        2.505458962964126e276,
        2.659899792371364e116,
        2.7125386559147274e90,
        9.536479965391043e185,
        9.567216720381635e239,
        5.16993041287954e245,
        4.939547529284952e179,
        3.1175116898205872e224,
        1.7555281884088452e42,
        5.429209768108731e84,
        1.0447670959436904e299,
        1.9580250342195754e105,
        8.848423533619703e204,
        3.4434065546244285e79,
        3.6093218170205304e216,
        8.464035133686624e293,
        1.22423660941592e120,
    ];
    let common_values = &[
        (2.141438721e116, 1),
        (8.7676954155e86, 1),
        (1.28439118539e55, 1),
        (1.79171075176e53, 1),
        (2.10333657725e74, 1),
        (2.3236426209e231, 1),
        (2.95823857742e58, 1),
        (3.1078914828e141, 1),
        (3.38975629714e61, 1),
        (4.28790184556e74, 1),
    ];
    let sample_median = (1.2523958970084127e173, Some(1.2542732495420994e173));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(2.939399538027295e305),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_primitive_float_range_helper::<f64>(
        1.0e38,
        f64::INFINITY,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f64, a = -f64::MIN_POSITIVE_SUBNORMAL, b = f64::MIN_POSITIVE_SUBNORMAL
    let values = &[
        -0.0, -5.0e-324, -0.0, 0.0, -0.0, -0.0, -5.0e-324, -0.0, -5.0e-324, 0.0, -0.0, -5.0e-324,
        -0.0, 0.0, 0.0, -5.0e-324, -0.0, -5.0e-324, 0.0, 0.0,
    ];
    let common_values = &[(-0.0, 333784), (0.0, 333516), (-5.0e-324, 332700)];
    let sample_median = (-0.0, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.0),
        standard_deviation: NiceFloat(0.0),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_primitive_float_range_helper::<f64>(
        -f64::MIN_POSITIVE_SUBNORMAL,
        f64::MIN_POSITIVE_SUBNORMAL,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f64, a = -0.0, b = f64::MIN_POSITIVE_SUBNORMAL
    let values = &[
        0.0, -0.0, -0.0, -0.0, 0.0, 0.0, 0.0, -0.0, 0.0, 0.0, 0.0, 0.0, -0.0, 0.0, 0.0, 0.0, 0.0,
        -0.0, 0.0, -0.0,
    ];
    let common_values = &[(0.0, 500473), (-0.0, 499527)];
    let sample_median = (0.0, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.0),
        standard_deviation: NiceFloat(0.0),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_primitive_float_range_helper::<f64>(
        -0.0,
        f64::MIN_POSITIVE_SUBNORMAL,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f64, a = 0.0, b = f64::MIN_POSITIVE_SUBNORMAL
    let values = &[0.0; 20];
    let common_values = &[(0.0, 1000000)];
    let sample_median = (0.0, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.0),
        standard_deviation: NiceFloat(0.0),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_primitive_float_range_helper::<f64>(
        0.0,
        f64::MIN_POSITIVE_SUBNORMAL,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f64, a = -f64::MIN_POSITIVE_SUBNORMAL, b = -0.0
    let values = &[-5.0e-324; 20];
    let common_values = &[(-5.0e-324, 1000000)];
    let sample_median = (-5.0e-324, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-5.0e-324),
        standard_deviation: NiceFloat(0.0),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_primitive_float_range_helper::<f64>(
        -f64::MIN_POSITIVE_SUBNORMAL,
        -0.0,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f64, a = -f64::MIN_POSITIVE_SUBNORMAL, b = 0.0
    let values = &[
        -0.0, -5.0e-324, -5.0e-324, -5.0e-324, -0.0, -0.0, -0.0, -5.0e-324, -0.0, -0.0, -0.0, -0.0,
        -5.0e-324, -0.0, -0.0, -0.0, -0.0, -5.0e-324, -0.0, -5.0e-324,
    ];
    let common_values = &[(-0.0, 500473), (-5.0e-324, 499527)];
    let sample_median = (-0.0, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.0),
        standard_deviation: NiceFloat(0.0),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_primitive_float_range_helper::<f64>(
        -f64::MIN_POSITIVE_SUBNORMAL,
        0.0,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f64, a = -Infinity, b = Infinity
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
    random_primitive_float_range_helper::<f64>(
        f64::NEGATIVE_INFINITY,
        f64::INFINITY,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f64, a = -0.0, b = 0.0
    let values = &[-0.0; 20];
    let common_values = &[(-0.0, 1000000)];
    let sample_median = (-0.0, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.0),
        standard_deviation: NiceFloat(0.0),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_primitive_float_range_helper::<f64>(
        -0.0,
        0.0,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
}

fn random_primitive_float_range_fail_helper<T: PrimitiveFloat>() {
    assert_panic!(random_primitive_float_range::<T>(
        EXAMPLE_SEED,
        T::ZERO,
        T::ZERO
    ));
    assert_panic!(random_primitive_float_range::<T>(
        EXAMPLE_SEED,
        T::ONE,
        T::ZERO
    ));
    assert_panic!(random_primitive_float_range::<T>(
        EXAMPLE_SEED,
        T::ONE,
        T::NAN
    ));
}

#[test]
fn random_primitive_float_range_fail() {
    apply_fn_to_primitive_floats!(random_primitive_float_range_fail_helper);
}
