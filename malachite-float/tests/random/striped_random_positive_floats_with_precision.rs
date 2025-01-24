// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::float::NiceFloat;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::moments::MomentStats;
use malachite_float::random::striped_random_positive_floats_with_precision;
use malachite_float::test_util::random::{
    random_floats_helper_helper, random_floats_helper_helper_no_common_values,
};

fn striped_random_positive_floats_with_precision_helper(
    mean_sci_exponent_abs_numerator: u64,
    mean_sci_exponent_abs_denominator: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    prec: u64,
    expected_values: &[&str],
    expected_values_hex: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_common_values_hex: &[(&str, usize)],
    expected_median: (&str, Option<&str>),
    expected_median_hex: (&str, Option<&str>),
    expected_moment_stats: MomentStats,
) {
    random_floats_helper_helper(
        striped_random_positive_floats_with_precision(
            EXAMPLE_SEED,
            mean_sci_exponent_abs_numerator,
            mean_sci_exponent_abs_denominator,
            mean_stripe_numerator,
            mean_stripe_denominator,
            prec,
        ),
        expected_values,
        expected_values_hex,
        expected_common_values,
        expected_common_values_hex,
        expected_median,
        expected_median_hex,
        expected_moment_stats,
    )
}

fn striped_random_positive_floats_with_precision_helper_no_common_values(
    mean_sci_exponent_abs_numerator: u64,
    mean_sci_exponent_abs_denominator: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    prec: u64,
    expected_values: &[&str],
    expected_values_hex: &[&str],
    expected_median: (&str, Option<&str>),
    expected_median_hex: (&str, Option<&str>),
    expected_moment_stats: MomentStats,
) {
    random_floats_helper_helper_no_common_values(
        striped_random_positive_floats_with_precision(
            EXAMPLE_SEED,
            mean_sci_exponent_abs_numerator,
            mean_sci_exponent_abs_denominator,
            mean_stripe_numerator,
            mean_stripe_denominator,
            prec,
        ),
        expected_values,
        expected_values_hex,
        expected_median,
        expected_median_hex,
        expected_moment_stats,
    )
}

#[test]
fn test_striped_random_positive_floats_with_precision() {
    // mean |sci_exponent| 1, precision 1
    let values = &[
        "1.0", "1.0", "2.0", "1.0", "1.0", "2.0", "2.0", "1.0", "2.0", "1.0", "2.0", "2.0", "4.0",
        "1.0", "1.0", "2.0", "0.1", "1.0", "0.2", "1.0",
    ];
    let values_hex = &[
        "0x1.0#1", "0x1.0#1", "0x2.0#1", "0x1.0#1", "0x1.0#1", "0x2.0#1", "0x2.0#1", "0x1.0#1",
        "0x2.0#1", "0x1.0#1", "0x2.0#1", "0x2.0#1", "0x4.0#1", "0x1.0#1", "0x1.0#1", "0x2.0#1",
        "0x0.2#1", "0x1.0#1", "0x0.4#1", "0x1.0#1",
    ];
    let common_values = &[
        ("1.0", 333085),
        ("2.0", 167041),
        ("0.5", 166671),
        ("0.2", 83200),
        ("4.0", 83177),
        ("8.0", 41643),
        ("0.1", 41551),
        ("0.06", 20981),
        ("2.0e1", 20919),
        ("3.0e1", 10411),
    ];
    let common_values_hex = &[
        ("0x1.0#1", 333085),
        ("0x2.0#1", 167041),
        ("0x0.8#1", 166671),
        ("0x0.4#1", 83200),
        ("0x4.0#1", 83177),
        ("0x8.0#1", 41643),
        ("0x0.2#1", 41551),
        ("0x0.1#1", 20981),
        ("0x1.0E+1#1", 20919),
        ("0x2.0E+1#1", 10411),
    ];
    let sample_median = ("1.0", None);
    let sample_median_hex = ("0x1.0#1", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(6.111051015726221),
        standard_deviation: NiceFloat(305.2547556508084),
        skewness: NiceFloat(297.86495510456103),
        excess_kurtosis: NiceFloat(112470.88988361698),
    };
    striped_random_positive_floats_with_precision_helper(
        1,
        1,
        8,
        1,
        1,
        values,
        values_hex,
        common_values,
        common_values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );

    // mean |sci_exponent| 1, precision 10
    let values = &[
        "1.998", "1.0", "3.996", "1.123", "1.0", "3.996", "2.0", "1.971", "3.996", "1.998",
        "3.996", "3.996", "7.99", "1.0", "1.998", "2.0", "0.2498", "1.998", "0.2505", "1.0",
    ];
    let values_hex = &[
        "0x1.ff8#10",
        "0x1.000#10",
        "0x3.ff#10",
        "0x1.1f8#10",
        "0x1.000#10",
        "0x3.ff#10",
        "0x2.00#10",
        "0x1.f88#10",
        "0x3.ff#10",
        "0x1.ff8#10",
        "0x3.ff#10",
        "0x3.ff#10",
        "0x7.fe#10",
        "0x1.000#10",
        "0x1.ff8#10",
        "0x2.00#10",
        "0x0.3ff#10",
        "0x1.ff8#10",
        "0x0.402#10",
        "0x1.000#10",
    ];
    let common_values = &[
        ("1.998", 129180),
        ("1.0", 129087),
        ("2.0", 64918),
        ("0.999", 64782),
        ("3.996", 64781),
        ("0.5", 64602),
        ("7.99", 32474),
        ("0.4995", 32422),
        ("4.0", 32040),
        ("0.25", 32029),
    ];
    let common_values_hex = &[
        ("0x1.ff8#10", 129180),
        ("0x1.000#10", 129087),
        ("0x2.00#10", 64918),
        ("0x0.ffc#10", 64782),
        ("0x3.ff#10", 64781),
        ("0x0.800#10", 64602),
        ("0x7.fe#10", 32474),
        ("0x0.7fe#10", 32422),
        ("0x4.00#10", 32040),
        ("0x0.400#10", 32029),
    ];
    let sample_median = ("1.5", None);
    let sample_median_hex = ("0x1.800#10", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(9.349869877794564),
        standard_deviation: NiceFloat(520.151410587591),
        skewness: NiceFloat(337.57543940728004),
        excess_kurtosis: NiceFloat(145252.3259106671),
    };
    striped_random_positive_floats_with_precision_helper(
        1,
        1,
        32,
        1,
        10,
        values,
        values_hex,
        common_values,
        common_values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );

    // mean |sci_exponent| 1, precision 64
    let values = &[
        "1.9999999925494194034",
        "1.2499999999999999999",
        "3.9999999925494194031",
        "1.0000000000018189893",
        "1.0000610351562482236",
        "3.9999389648437526645",
        "2.0",
        "1.9843750001164152663",
        "3.8750004768369308294",
        "1.9999923706636728582",
        "3.9999999999999431566",
        "3.9960937504656541819",
        "7.9999999999998863132",
        "1.0000305175781248889",
        "1.9999999999990905053",
        "2.0000000001164153216",
        "0.24999999906869163623",
        "1.9990539550781179222",
        "0.25",
        "1.0000000000001136799",
    ];
    let values_hex = &[
        "0x1.ffffffe000000006#64",
        "0x1.3ffffffffffffffe#64",
        "0x3.ffffffe000000000#64",
        "0x1.0000000001fffffe#64",
        "0x1.0003ffffffff8000#64",
        "0x3.fffc00000000c000#64",
        "0x2.0000000000000000#64",
        "0x1.fc0000007ffffc00#64",
        "0x3.e00007ffffc00000#64",
        "0x1.ffff80003fff0000#64",
        "0x3.fffffffffff00000#64",
        "0x3.ff000001fffe0000#64",
        "0x7.ffffffffffe00000#64",
        "0x1.0001fffffffff7fe#64",
        "0x1.ffffffffff000000#64",
        "0x2.000000007ffffffc#64",
        "0x0.3ffffffc0003ffffc#64",
        "0x1.ffc1fffffffe01fe#64",
        "0x0.40000000000000000#64",
        "0x1.00000000001fff80#64",
    ];
    let sample_median = ("1.5", None);
    let sample_median_hex = ("0x1.8000000000000000#64", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(9.337530231284171),
        standard_deviation: NiceFloat(520.317614819243),
        skewness: NiceFloat(340.99301578218797),
        excess_kurtosis: NiceFloat(147660.4928189293),
    };
    striped_random_positive_floats_with_precision_helper_no_common_values(
        1,
        1,
        32,
        1,
        64,
        values,
        values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );

    // mean |sci_exponent| 64, precision 1
    let values = &[
        "2.0e-37", "8.0e-34", "4.0e-6", "3.0e-45", "2.0e15", "5.0e-10", "2.0e4", "0.06", "0.0001",
        "1.0e14", "2.0e34", "9.0e-13", "0.0001", "9.0e9", "1.0e33", "0.1", "0.00003", "5.0e18",
        "0.06", "5.0e8",
    ];
    let values_hex = &[
        "0x4.0E-31#1",
        "0x4.0E-28#1",
        "0x0.00004#1",
        "0x1.0E-37#1",
        "0x8.0E+12#1",
        "0x2.0E-8#1",
        "0x4.0E+3#1",
        "0x0.1#1",
        "0x0.0008#1",
        "0x8.0E+11#1",
        "0x4.0E+28#1",
        "0x1.0E-10#1",
        "0x0.0008#1",
        "0x2.0E+8#1",
        "0x4.0E+27#1",
        "0x0.2#1",
        "0x0.0002#1",
        "0x4.0E+15#1",
        "0x0.1#1",
        "0x2.0E+7#1",
    ];
    let common_values = &[
        ("1.0", 7698),
        ("0.2", 7624),
        ("0.5", 7597),
        ("2.0", 7563),
        ("4.0", 7402),
        ("8.0", 7362),
        ("0.06", 7323),
        ("0.1", 7239),
        ("3.0e1", 7225),
        ("2.0e1", 7161),
    ];
    let common_values_hex = &[
        ("0x1.0#1", 7698),
        ("0x0.4#1", 7624),
        ("0x0.8#1", 7597),
        ("0x2.0#1", 7563),
        ("0x4.0#1", 7402),
        ("0x8.0#1", 7362),
        ("0x0.1#1", 7323),
        ("0x0.2#1", 7239),
        ("0x2.0E+1#1", 7225),
        ("0x1.0E+1#1", 7161),
    ];
    let sample_median = ("1.0", None);
    let sample_median_hex = ("0x1.0#1", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1.9680504915704222e255),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    striped_random_positive_floats_with_precision_helper(
        64,
        1,
        8,
        1,
        1,
        values,
        values_hex,
        common_values,
        common_values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );

    // mean |sci_exponent| 64, precision 64
    let values = &[
        "3.758e-37",
        "7.7e-34",
        "7.62e-6",
        "3.147e-45",
        "2.252e15",
        "9.304e-10",
        "1.638e4",
        "0.1232",
        "0.0002439",
        "2.812e14",
        "4.15e34",
        "1.817e-12",
        "0.0002439",
        "8.59e9",
        "2.594e33",
        "0.125",
        "0.00006098",
        "9.214e18",
        "0.0626",
        "5.37e8",
    ];
    let values_hex = &[
        "0x7.feE-31#10",
        "0x4.00E-28#10",
        "0x0.00007fe#10",
        "0x1.1f8E-37#10",
        "0x8.00E+12#10",
        "0x3.ffE-8#10",
        "0x4.00E+3#10",
        "0x0.1f88#10",
        "0x0.000ffc#10",
        "0xf.fcE+11#10",
        "0x7.feE+28#10",
        "0x1.ff8E-10#10",
        "0x0.000ffc#10",
        "0x2.00E+8#10",
        "0x7.feE+27#10",
        "0x0.200#10",
        "0x0.0003ff#10",
        "0x7.feE+15#10",
        "0x0.1008#10",
        "0x2.00E+7#10",
    ];
    let common_values = &[
        ("1.0", 3037),
        ("1.998", 2989),
        ("3.996", 2983),
        ("0.5", 2957),
        ("0.25", 2940),
        ("0.4995", 2931),
        ("0.999", 2926),
        ("2.0", 2913),
        ("15.98", 2904),
        ("7.99", 2887),
    ];
    let common_values_hex = &[
        ("0x1.000#10", 3037),
        ("0x1.ff8#10", 2989),
        ("0x3.ff#10", 2983),
        ("0x0.800#10", 2957),
        ("0x0.400#10", 2940),
        ("0x0.7fe#10", 2931),
        ("0x0.ffc#10", 2926),
        ("0x2.00#10", 2913),
        ("0xf.fc#10", 2904),
        ("0x7.fe#10", 2887),
    ];
    let sample_median = ("1.127", Some("1.188"));
    let sample_median_hex = ("0x1.208#10", Some("0x1.300#10"));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(3.932257134524397e255),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    striped_random_positive_floats_with_precision_helper(
        64,
        1,
        32,
        1,
        10,
        values,
        values_hex,
        common_values,
        common_values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );

    // mean |sci_exponent| 64, precision 64
    let values = &[
        "3.7615819086183353829e-37",
        "9.6296497219361792644e-34",
        "7.6293945170391452848e-6",
        "2.8025969286547320357e-45",
        "2251937252638716.0",
        "9.3130836376076393401e-10",
        "16384.0",
        "0.124023437507275954145",
        "0.00023651125957256657895",
        "281473902977023.5",
        "4.1538374868278030732e34",
        "1.817213046918211231e-12",
        "0.00024414062499999653055",
        "8590196735.999999045",
        "2.5961484292662332226e33",
        "0.1250000000072759576",
        "0.000061035156022629794",
        "9219009174715727999.5",
        "0.0625",
        "536870912.00006103143",
    ];
    let values_hex = &[
        "0x7.ffffff8000000018E-31#64",
        "0x4.fffffffffffffff8E-28#64",
        "0x0.00007ffffffc000000000#64",
        "0x1.0000000001fffffeE-37#64",
        "0x8001ffffffffc.000#64",
        "0x3.fffc00000000c000E-8#64",
        "0x4000.0000000000000#64",
        "0x0.1fc0000007ffffc00#64",
        "0x0.000f80001fffff00000#64",
        "0xffffc0001fff.8000#64",
        "0x7.ffffffffffe00000E+28#64",
        "0x1.ff800000ffff0000E-10#64",
        "0x0.000fffffffffffc0000#64",
        "0x20003ffff.ffffeffc#64",
        "0x7.fffffffffc000000E+27#64",
        "0x0.2000000007ffffffc#64",
        "0x0.0003ffffffc0003ffffc#64",
        "0x7ff07fffffff807f.8#64",
        "0x0.10000000000000000#64",
        "0x20000000.0003fff00#64",
    ];
    let sample_median = ("1.2490234375", Some("1.2490234449505805968"));
    let sample_median_hex = ("0x1.3fc0000000000000#64", Some("0x1.3fc0001ffffffffe#64"));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(3.9361009831408444e255),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    striped_random_positive_floats_with_precision_helper_no_common_values(
        64,
        1,
        32,
        1,
        64,
        values,
        values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );
}

#[test]
#[should_panic]
fn striped_random_positive_floats_with_precision_fail_1() {
    striped_random_positive_floats_with_precision(EXAMPLE_SEED, 1, 0, 2, 1, 2);
}

#[test]
#[should_panic]
fn striped_random_positive_floats_with_precision_fail_2() {
    striped_random_positive_floats_with_precision(EXAMPLE_SEED, 0, 1, 2, 1, 2);
}

#[test]
#[should_panic]
fn striped_random_positive_floats_with_precision_fail_3() {
    striped_random_positive_floats_with_precision(EXAMPLE_SEED, 1, 1, 2, 1, 0);
}

#[test]
#[should_panic]
fn striped_random_positive_floats_with_precision_fail_4() {
    striped_random_positive_floats_with_precision(EXAMPLE_SEED, 1, 1, 2, 3, 2);
}

#[test]
#[should_panic]
fn striped_random_positive_floats_with_precision_fail_5() {
    striped_random_positive_floats_with_precision(EXAMPLE_SEED, 1, 1, 1, 0, 2);
}
