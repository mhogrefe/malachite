// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::float::NiceFloat;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::moments::MomentStats;
use malachite_float::random::random_positive_floats_with_precision;
use malachite_float::test_util::random::{
    random_floats_helper_helper, random_floats_helper_helper_no_common_values,
};

fn random_positive_floats_with_precision_helper(
    mean_sci_exponent_abs_numerator: u64,
    mean_sci_exponent_abs_denominator: u64,
    precision: u64,
    expected_values: &[&str],
    expected_values_hex: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_common_values_hex: &[(&str, usize)],
    expected_median: (&str, Option<&str>),
    expected_median_hex: (&str, Option<&str>),
    expected_moment_stats: MomentStats,
) {
    random_floats_helper_helper(
        random_positive_floats_with_precision(
            EXAMPLE_SEED,
            mean_sci_exponent_abs_numerator,
            mean_sci_exponent_abs_denominator,
            precision,
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

fn random_positive_floats_with_precision_helper_no_common_values(
    mean_sci_exponent_abs_numerator: u64,
    mean_sci_exponent_abs_denominator: u64,
    precision: u64,
    expected_values: &[&str],
    expected_values_hex: &[&str],
    expected_median: (&str, Option<&str>),
    expected_median_hex: (&str, Option<&str>),
    expected_moment_stats: MomentStats,
) {
    random_floats_helper_helper_no_common_values(
        random_positive_floats_with_precision(
            EXAMPLE_SEED,
            mean_sci_exponent_abs_numerator,
            mean_sci_exponent_abs_denominator,
            precision,
        ),
        expected_values,
        expected_values_hex,
        expected_median,
        expected_median_hex,
        expected_moment_stats,
    )
}

#[test]
fn test_random_positive_floats_with_precision() {
    // mean |sci_exponent| 1, precision 1
    let values = &[
        "1.0", "1.0", "2.0", "1.0", "1.0", "2.0", "2.0", "1.0", "2.0", "1.0", "2.0", "2.0", "4.0",
        "1.0", "1.0", "2.0", "0.12", "1.0", "0.25", "1.0",
    ];
    let values_hex = &[
        "0x1.0#1", "0x1.0#1", "0x2.0#1", "0x1.0#1", "0x1.0#1", "0x2.0#1", "0x2.0#1", "0x1.0#1",
        "0x2.0#1", "0x1.0#1", "0x2.0#1", "0x2.0#1", "0x4.0#1", "0x1.0#1", "0x1.0#1", "0x2.0#1",
        "0x0.2#1", "0x1.0#1", "0x0.4#1", "0x1.0#1",
    ];
    let common_values = &[
        ("1.0", 333085),
        ("2.0", 167041),
        ("0.50", 166671),
        ("0.25", 83200),
        ("4.0", 83177),
        ("8.0", 41643),
        ("0.12", 41551),
        ("0.062", 20981),
        ("16.0", 20919),
        ("32.0", 10411),
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
    random_positive_floats_with_precision_helper(
        1,
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
        "1.9180", "1.9805", "3.3047", "1.4199", "1.5566", "3.0234", "2.5000", "1.5410", "3.6211",
        "1.8340", "3.1016", "2.6172", "7.1484", "1.4297", "1.9434", "3.7266", "0.21240", "1.8691",
        "0.27197", "1.1719",
    ];
    let values_hex = &[
        "0x1.eb0#10",
        "0x1.fb0#10",
        "0x3.4e#10",
        "0x1.6b8#10",
        "0x1.8e8#10",
        "0x3.06#10",
        "0x2.80#10",
        "0x1.8a8#10",
        "0x3.9f#10",
        "0x1.d58#10",
        "0x3.1a#10",
        "0x2.9e#10",
        "0x7.26#10",
        "0x1.6e0#10",
        "0x1.f18#10",
        "0x3.ba#10",
        "0x0.366#10",
        "0x1.de8#10",
        "0x0.45a#10",
        "0x1.2c0#10",
    ];
    let common_values = &[
        ("1.3809", 721),
        ("1.8691", 717),
        ("1.0840", 711),
        ("1.0332", 710),
        ("1.6348", 707),
        ("1.0586", 706),
        ("1.4688", 706),
        ("1.2520", 705),
        ("1.2051", 702),
        ("1.0469", 700),
    ];
    let common_values_hex = &[
        ("0x1.618#10", 721),
        ("0x1.de8#10", 717),
        ("0x1.158#10", 711),
        ("0x1.088#10", 710),
        ("0x1.a28#10", 707),
        ("0x1.0f0#10", 706),
        ("0x1.780#10", 706),
        ("0x1.408#10", 705),
        ("0x1.348#10", 702),
        ("0x1.0c0#10", 700),
    ];
    let sample_median = ("1.5000", None);
    let sample_median_hex = ("0x1.800#10", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(9.056696785111221),
        standard_deviation: NiceFloat(431.29029073720193),
        skewness: NiceFloat(278.5706756279952),
        excess_kurtosis: NiceFloat(97797.38656215608),
    };
    random_positive_floats_with_precision_helper(
        1,
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
        "1.74019940281767964796",
        "1.01924492151562612221",
        "3.11303076194048707363",
        "1.04714491729122524232",
        "1.38406408776653625273",
        "3.04606687550705462894",
        "3.47652620627111855200",
        "1.81189540295981596724",
        "3.72897715778079017596",
        "1.93367535183550314415",
        "2.15730700697094896498",
        "3.44988718503839745340",
        "6.94120406140530561137",
        "1.79431526996442727460",
        "1.24403945136712512328",
        "2.61378097120535489309",
        "0.239866256427144935304",
        "1.66297730225451555858",
        "0.255332140257940030768",
        "1.90069088054110908303",
    ];
    let values_hex = &[
        "0x1.bd7db5439ee3dbac#64",
        "0x1.04ed3c34861143ec#64",
        "0x3.1cef9581f9f24d38#64",
        "0x1.0c11b075f03d6dae#64",
        "0x1.62520628867d223a#64",
        "0x3.0bcb09ebbb50e418#64",
        "0x3.79fd9f179f145a00#64",
        "0x1.cfd8608b7c32de2a#64",
        "0x3.ba9e3f3c33141e7c#64",
        "0x1.ef05590d36fbcb56#64",
        "0x2.284545a25f32dc68#64",
        "0x3.732bce7aa1218278#64",
        "0x6.f0f2bfd699213c98#64",
        "0x1.cb583edb35eb99b8#64",
        "0x1.3e795e968e3acfc6#64",
        "0x2.9d20bfee3bef16e8#64",
        "0x0.3d67dffec4bedc598#64",
        "0x1.a9b8e1672c674f7a#64",
        "0x0.415d727806899f168#64",
        "0x1.e693ad73bac0ecb0#64",
    ];
    let sample_median = ("1.50214017002244585479", Some("1.50214119661822423086"));
    let sample_median_hex = ("0x1.808c42184118ca86#64", Some("0x1.808c5351731799cc#64"));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(9.06512816951906),
        standard_deviation: NiceFloat(439.97218971420835),
        skewness: NiceFloat(280.6616264514779),
        excess_kurtosis: NiceFloat(99433.37467826756),
    };
    random_positive_floats_with_precision_helper_no_common_values(
        1,
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
        "1.9e-37", "7.7e-34", "3.8e-6", "2.8e-45", "2.3e15", "4.7e-10", "1.6e4", "0.062",
        "0.00012", "1.4e14", "2.1e34", "9.1e-13", "0.00012", "8.6e9", "1.3e33", "0.12", "0.000031",
        "4.6e18", "0.062", "5.4e8",
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
        ("0.25", 7624),
        ("0.50", 7597),
        ("2.0", 7563),
        ("4.0", 7402),
        ("8.0", 7362),
        ("0.062", 7323),
        ("0.12", 7239),
        ("32.0", 7225),
        ("16.0", 7161),
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
    random_positive_floats_with_precision_helper(
        64,
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

    // mean |sci_exponent| 64, precision 10
    let values = &[
        "3.6073e-37",
        "1.5257e-33",
        "6.3032e-6",
        "3.9795e-45",
        "3.5052e15",
        "7.0395e-10",
        "20480.0",
        "0.096313",
        "0.00022101",
        "2.5811e14",
        "3.2208e34",
        "1.1902e-12",
        "0.00021815",
        "1.2281e10",
        "2.5226e33",
        "0.23291",
        "0.000051856",
        "8.6199e18",
        "0.067993",
        "6.2915e8",
    ];
    let values_hex = &[
        "0x7.acE-31#10",
        "0x7.ecE-28#10",
        "0x0.000069c#10",
        "0x1.6b8E-37#10",
        "0xc.74E+12#10",
        "0x3.06E-8#10",
        "0x5.00E+3#10",
        "0x0.18a8#10",
        "0x0.000e7c#10",
        "0xe.acE+11#10",
        "0x6.34E+28#10",
        "0x1.4f0E-10#10",
        "0x0.000e4c#10",
        "0x2.dcE+8#10",
        "0x7.c6E+27#10",
        "0x0.3ba#10",
        "0x0.000366#10",
        "0x7.7aE+15#10",
        "0x0.1168#10",
        "0x2.58E+7#10",
    ];
    let common_values = &[
        ("0.0010662", 31),
        ("1.3496", 29),
        ("1.6875", 29),
        ("0.44287", 29),
        ("0.81445", 29),
        ("2.8828", 28),
        ("496.00", 28),
        ("0.56934", 28),
        ("0.021912", 28),
        ("0.029541", 28),
    ];
    let common_values_hex = &[
        ("0x0.0045e#10", 31),
        ("0x1.598#10", 29),
        ("0x1.b00#10", 29),
        ("0x0.716#10", 29),
        ("0x0.d08#10", 29),
        ("0x2.e2#10", 28),
        ("0x1f0.0#10", 28),
        ("0x0.91c#10", 28),
        ("0x0.059c#10", 28),
        ("0x0.0790#10", 28),
    ];
    let sample_median = ("1.4707", None);
    let sample_median_hex = ("0x1.788#10", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(3.163487411254837e255),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_positive_floats_with_precision_helper(
        64,
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
        "3.27295130770640117855e-37",
        "7.85197726004624955735e-34",
        "5.93763496769044317938e-6",
        "2.93472512905146303523e-45",
        "3116635254961129.06958",
        "7.09217711237039097803e-10",
        "28479.7026817730031780",
        "0.113243462684988497952",
        "0.000227598703477831431638",
        "272140612311933.897141",
        "2.24027567903808595974e34",
        "1.56882705825337209417e-12",
        "0.000211828737225503711284",
        "15413050806.4212524556",
        "1.61485553380672856292e33",
        "0.163361310700334680818",
        "0.0000585610977605334314707",
        "7669129173769245662.50",
        "0.0638330350644850076921",
        "1020425646.46618828690",
    ];
    let values_hex = &[
        "0x6.f5f6d50e7b8f6eb0E-31#64",
        "0x4.13b4f0d218450fb0E-28#64",
        "0x0.0000639df2b03f3e49a70#64",
        "0x1.0c11b075f03d6daeE-37#64",
        "0xb1290314433e9.11d#64",
        "0x3.0bcb09ebbb50e418E-8#64",
        "0x6f3f.b3e2f3e28b400#64",
        "0x0.1cfd8608b7c32de2a#64",
        "0x0.000eea78fcf0cc5079f#64",
        "0xf782ac869b7d.e5ab#64",
        "0x4.508a8b44be65b8d0E+28#64",
        "0x1.b995e73d5090c13cE-10#64",
        "0x0.000de1e57fad3242793#64",
        "0x396b07db6.6bd73370#64",
        "0x4.f9e57a5a38eb3f18E+27#64",
        "0x0.29d20bfee3bef16e8#64",
        "0x0.0003d67dffec4bedc598#64",
        "0x6a6e3859cb19d3de.8#64",
        "0x0.10575c9e01a267c5a#64",
        "0x3cd275ae.77581d960#64",
    ];
    let sample_median = ("1.47440483307566421183", Some("1.47455000333761709127"));
    let sample_median_hex = ("0x1.7972985b1fd33b34#64", Some("0x1.797c1be8a6d97f72#64"));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(3.1785863803225345e255),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_positive_floats_with_precision_helper_no_common_values(
        64,
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
fn random_positive_floats_with_precision_fail_1() {
    random_positive_floats_with_precision(EXAMPLE_SEED, 1, 0, 2);
}

#[test]
#[should_panic]
fn random_positive_floats_with_precision_fail_2() {
    random_positive_floats_with_precision(EXAMPLE_SEED, 0, 1, 2);
}

#[test]
#[should_panic]
fn random_positive_floats_with_precision_fail_3() {
    random_positive_floats_with_precision(EXAMPLE_SEED, 1, 1, 0);
}
