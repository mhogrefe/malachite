// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::NegativeInfinity;
use malachite_base::num::float::NiceFloat;
use malachite_base::num::random::random_primitive_float_inclusive_range;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::num::random::random_primitive_floats_helper_helper;
use malachite_base::test_util::stats::moments::{CheckedToF64, MomentStats};
use std::panic::catch_unwind;

fn random_primitive_float_inclusive_range_helper<T: CheckedToF64 + PrimitiveFloat>(
    a: T,
    b: T,
    expected_values: &[T],
    expected_common_values: &[(T, usize)],
    expected_median: (T, Option<T>),
    expected_moment_stats: MomentStats,
) {
    random_primitive_floats_helper_helper(
        random_primitive_float_inclusive_range::<T>(EXAMPLE_SEED, a, b),
        expected_values,
        expected_common_values,
        expected_median,
        expected_moment_stats,
    );
}

#[test]
fn test_random_primitive_float_inclusive_range() {
    // f32, a = 1.0, b = 2.0
    let values = &[
        1.5463697, 1.6846209, 1.6517982, 1.6963725, 1.246657, 1.0557153, 1.8866968, 1.8430634,
        1.0973871, 1.4662611, 1.8181343, 1.8396878, 1.7092294, 1.6617042, 1.7495066, 1.1922526,
        1.9193928, 1.1640857, 1.2869775, 1.7921972,
    ];
    let common_values = &[
        (1.262395, 4),
        (1.619045, 4),
        (1.942131, 4),
        (1.0062196, 4),
        (1.0075867, 4),
        (1.0664382, 4),
        (1.0932482, 4),
        (1.1049225, 4),
        (1.1625684, 4),
        (1.1672009, 4),
    ];
    let sample_median = (1.4996792, Some(1.4996796));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1.4997834476890244),
        standard_deviation: NiceFloat(0.2887567306490316),
        skewness: NiceFloat(0.001395347389592521),
        excess_kurtosis: NiceFloat(-1.2017290675577252),
    };
    random_primitive_float_inclusive_range_helper::<f32>(
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
    random_primitive_float_inclusive_range_helper::<f32>(
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
    random_primitive_float_inclusive_range_helper::<f32>(
        core::f32::consts::E,
        core::f32::consts::PI,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f32, a = 100.0, b = 101.0
    let values = &[
        100.96766, 100.552864, 100.684616, 100.34195, 100.725746, 100.24665, 100.79547, 100.21028,
        100.7486, 100.09738, 100.360596, 100.518265, 100.83968, 100.47318, 100.16781, 100.6617,
        100.48242, 100.192245, 100.84114, 100.095436,
    ];
    let common_values = &[
        (100.32666, 22),
        (100.33122, 22),
        (100.60651, 22),
        (100.29688, 21),
        (100.51455, 21),
        (100.75446, 21),
        (100.672554, 21),
        (100.863556, 21),
        (100.125, 20),
        (100.06987, 20),
    ];
    let sample_median = (100.50014, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(100.5000247275421),
        standard_deviation: NiceFloat(0.2887172501103727),
        skewness: NiceFloat(0.0005303743867281354),
        excess_kurtosis: NiceFloat(-1.2019547226159832),
    };
    random_primitive_float_inclusive_range_helper::<f32>(
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
    random_primitive_float_inclusive_range_helper::<f32>(
        1.0e38,
        f32::INFINITY,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f32, a = -f32::MIN_POSITIVE_SUBNORMAL, b = f32::MIN_POSITIVE_SUBNORMAL
    let values = &[
        -0.0, -1.0e-45, 1.0e-45, -0.0, 1.0e-45, 1.0e-45, 0.0, 1.0e-45, -0.0, -0.0, -1.0e-45, -0.0,
        -1.0e-45, 1.0e-45, 0.0, -0.0, -1.0e-45, -0.0, 0.0, 1.0e-45,
    ];
    let common_values = &[(-0.0, 250314), (1.0e-45, 250015), (0.0, 249955), (-1.0e-45, 249716)];
    let sample_median = (-0.0, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(4.18988240833055e-49),
        standard_deviation: NiceFloat(9.90601474026171e-46),
        skewness: NiceFloat(-0.00042250825585668007),
        excess_kurtosis: NiceFloat(-0.9989230633891024),
    };
    random_primitive_float_inclusive_range_helper::<f32>(
        -f32::MIN_POSITIVE_SUBNORMAL,
        f32::MIN_POSITIVE_SUBNORMAL,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f32, a = -0.0, b = f32::MIN_POSITIVE_SUBNORMAL
    let values = &[
        0.0, -0.0, 0.0, 1.0e-45, 0.0, 0.0, -0.0, 0.0, -0.0, 1.0e-45, 0.0, -0.0, 0.0, 1.0e-45,
        1.0e-45, -0.0, 0.0, -0.0, 1.0e-45, 1.0e-45,
    ];
    let common_values = &[(0.0, 333784), (1.0e-45, 333516), (-0.0, 332700)];
    let sample_median = (0.0, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(4.673554586277644e-46),
        standard_deviation: NiceFloat(6.606692048548519e-46),
        skewness: NiceFloat(0.7062350975867705),
        excess_kurtosis: NiceFloat(-1.5012319869365465),
    };
    random_primitive_float_inclusive_range_helper::<f32>(
        -0.0,
        f32::MIN_POSITIVE_SUBNORMAL,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f32, a = 0.0, b = 0.0
    let values = &[0.0; 20];
    let common_values = &[(0.0, 1000000)];
    let sample_median = (0.0, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.0),
        standard_deviation: NiceFloat(0.0),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_primitive_float_inclusive_range_helper::<f32>(
        0.0,
        0.0,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f32, a = -f32::MIN_POSITIVE_SUBNORMAL, b = -f32::MIN_POSITIVE_SUBNORMAL
    let values = &[-1.0e-45; 20];
    let common_values = &[(-1.0e-45, 1000000)];
    let sample_median = (-1.0e-45, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-1.401298464324817e-45),
        standard_deviation: NiceFloat(0.0),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_primitive_float_inclusive_range_helper::<f32>(
        -f32::MIN_POSITIVE_SUBNORMAL,
        -f32::MIN_POSITIVE_SUBNORMAL,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f32, a = -f32::MIN_POSITIVE_SUBNORMAL, b = 0.0
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
    random_primitive_float_inclusive_range_helper::<f32>(
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
    random_primitive_float_inclusive_range_helper::<f32>(
        f32::NEGATIVE_INFINITY,
        f32::INFINITY,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f32, a = -0.0, b = -0.0
    let values = &[-0.0; 20];
    let common_values = &[(-0.0, 1000000)];
    let sample_median = (-0.0, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.0),
        standard_deviation: NiceFloat(0.0),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_primitive_float_inclusive_range_helper::<f32>(
        -0.0,
        -0.0,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f64, a = 1.0, b = 2.0
    let values = &[
        1.5514873723431857,
        1.027857700733222,
        1.0138546387920868,
        1.3122324563791183,
        1.3405051829686652,
        1.035872192413254,
        1.1044047458079171,
        1.1492076100807012,
        1.6654435140088601,
        1.0970321240173933,
        1.7768534778894969,
        1.7596944738316886,
        1.4195592641558248,
        1.6870312410839399,
        1.6389477926482805,
        1.7596601473487807,
        1.5802838577448093,
        1.6118733984422406,
        1.6845629053029185,
        1.7184068862055195,
    ];
    let common_values = &[
        (1.403773233, 1),
        (1.4279826281, 1),
        (1.00013172472, 1),
        (1.12691284792, 1),
        (1.18903919511, 1),
        (1.19748472183, 1),
        (1.22623118845, 1),
        (1.28755110409, 1),
        (1.29488660814, 1),
        (1.29653599853, 1),
    ];
    let sample_median = (1.5009930608108493, Some(1.5009982675741425));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1.5004909167990677),
        standard_deviation: NiceFloat(0.288665614327161),
        skewness: NiceFloat(-0.0026087726173027),
        excess_kurtosis: NiceFloat(-1.199166529481062),
    };
    random_primitive_float_inclusive_range_helper::<f64>(
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
    random_primitive_float_inclusive_range_helper::<f64>(
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
    random_primitive_float_inclusive_range_helper::<f64>(
        core::f64::consts::E,
        core::f64::consts::PI,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f64, a = 100.0, b = 101.0
    let values = &[
        100.60719315639041,
        100.52293238466861,
        100.39773606616421,
        100.15849325452957,
        100.62446491275823,
        100.00756963969552,
        100.87724201202582,
        100.41761898323166,
        100.61748356221035,
        100.19203505270988,
        100.00337267247278,
        100.15603343751732,
        100.23002386168925,
        100.77135404787671,
        100.23327285882462,
        100.27432542366292,
        100.63224218858957,
        100.47879402977489,
        100.50368945395086,
        100.4540267175569,
    ];
    let common_values = &[
        (100.49174834, 1),
        (100.95678412, 1),
        (100.029622688, 1),
        (100.110761848, 1),
        (100.163351429, 1),
        (100.241016737, 1),
        (100.334091318, 1),
        (100.374320788, 1),
        (100.375142847, 1),
        (100.403842534, 1),
    ];
    let sample_median = (100.49938135068814, Some(100.49938267498312));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(100.49957670089151),
        standard_deviation: NiceFloat(0.28876054194108447),
        skewness: NiceFloat(0.00047268031818767473),
        excess_kurtosis: NiceFloat(-1.2008323763153763),
    };
    random_primitive_float_inclusive_range_helper::<f64>(
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
    random_primitive_float_inclusive_range_helper::<f64>(
        1.0e38,
        f64::INFINITY,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f64, a = -f64::MIN_POSITIVE_SUBNORMAL, b = f64::MIN_POSITIVE_SUBNORMAL
    let values = &[
        -0.0, -5.0e-324, 5.0e-324, -0.0, 5.0e-324, 5.0e-324, 0.0, 5.0e-324, -0.0, -0.0, -5.0e-324,
        -0.0, -5.0e-324, 5.0e-324, 0.0, -0.0, -5.0e-324, -0.0, 0.0, 5.0e-324,
    ];
    let common_values = &[(-0.0, 250314), (5.0e-324, 250015), (0.0, 249955), (-5.0e-324, 249716)];
    let sample_median = (-0.0, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.0),
        standard_deviation: NiceFloat(0.0),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_primitive_float_inclusive_range_helper::<f64>(
        -f64::MIN_POSITIVE_SUBNORMAL,
        f64::MIN_POSITIVE_SUBNORMAL,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f64, a = -0.0, b = f64::MIN_POSITIVE_SUBNORMAL
    let values = &[
        0.0, -0.0, 0.0, 5.0e-324, 0.0, 0.0, -0.0, 0.0, -0.0, 5.0e-324, 0.0, -0.0, 0.0, 5.0e-324,
        5.0e-324, -0.0, 0.0, -0.0, 5.0e-324, 5.0e-324,
    ];
    let common_values = &[(0.0, 333784), (5.0e-324, 333516), (-0.0, 332700)];
    let sample_median = (0.0, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.0),
        standard_deviation: NiceFloat(0.0),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_primitive_float_inclusive_range_helper::<f64>(
        -0.0,
        f64::MIN_POSITIVE_SUBNORMAL,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f64, a = 0.0, b = 0.0
    let values = &[0.0; 20];
    let common_values = &[(0.0, 1000000)];
    let sample_median = (0.0, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.0),
        standard_deviation: NiceFloat(0.0),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_primitive_float_inclusive_range_helper::<f64>(
        0.0,
        0.0,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f64, a = -f64::MIN_POSITIVE_SUBNORMAL, b = -f64::MIN_POSITIVE_SUBNORMAL
    let values = &[-5.0e-324; 20];
    let common_values = &[(-5.0e-324, 1000000)];
    let sample_median = (-5.0e-324, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-5.0e-324),
        standard_deviation: NiceFloat(0.0),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_primitive_float_inclusive_range_helper::<f64>(
        -f64::MIN_POSITIVE_SUBNORMAL,
        -f64::MIN_POSITIVE_SUBNORMAL,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f64, a = -f64::MIN_POSITIVE_SUBNORMAL, b = 0.0
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
    random_primitive_float_inclusive_range_helper::<f64>(
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
    random_primitive_float_inclusive_range_helper::<f64>(
        f64::NEGATIVE_INFINITY,
        f64::INFINITY,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // f64, a = -0.0, b = 0.0
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
    random_primitive_float_inclusive_range_helper::<f64>(
        -0.0,
        0.0,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
}

fn random_primitive_float_inclusive_range_fail_helper<T: PrimitiveFloat>() {
    assert_panic!(random_primitive_float_inclusive_range::<T>(
        EXAMPLE_SEED,
        T::ONE,
        T::ZERO
    ));
    assert_panic!(random_primitive_float_inclusive_range::<T>(
        EXAMPLE_SEED,
        T::ONE,
        T::NAN
    ));
}

#[test]
fn random_primitive_float_inclusive_range_fail() {
    apply_fn_to_primitive_floats!(random_primitive_float_inclusive_range_fail_helper);
}
