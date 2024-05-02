// Copyright © 2024 Mikhail Hogrefe
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
        "1.918", "1.98", "3.305", "1.42", "1.557", "3.023", "2.5", "1.541", "3.621", "1.834",
        "3.102", "2.617", "7.15", "1.43", "1.943", "3.727", "0.2124", "1.869", "0.272", "1.172",
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
        ("1.381", 721),
        ("1.869", 717),
        ("1.084", 711),
        ("1.033", 710),
        ("1.635", 707),
        ("1.059", 706),
        ("1.469", 706),
        ("1.252", 705),
        ("1.205", 702),
        ("1.047", 700),
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
    let sample_median = ("1.5", None);
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
        "1.740199402817679648",
        "1.0192449215156261222",
        "3.1130307619404870736",
        "1.0471449172912252423",
        "1.3840640877665362527",
        "3.0460668755070546289",
        "3.476526206271118552",
        "1.8118954029598159672",
        "3.728977157780790176",
        "1.9336753518355031441",
        "2.157307006970948965",
        "3.4498871850383974534",
        "6.9412040614053056114",
        "1.7943152699644272746",
        "1.2440394513671251233",
        "2.6137809712053548931",
        "0.2398662564271449353",
        "1.6629773022545155586",
        "0.25533214025794003077",
        "1.900690880541109083",
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
    let sample_median = ("1.5021401700224458548", Some("1.5021411966182242309"));
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
        "3.607e-37",
        "1.526e-33",
        "6.303e-6",
        "3.979e-45",
        "3.505e15",
        "7.04e-10",
        "2.048e4",
        "0.0963",
        "0.000221",
        "2.581e14",
        "3.221e34",
        "1.19e-12",
        "0.0002182",
        "1.228e10",
        "2.523e33",
        "0.2329",
        "0.00005186",
        "8.62e18",
        "0.068",
        "6.29e8",
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
        ("0.001066", 31),
        ("1.35", 29),
        ("0.814", 29),
        ("1.688", 29),
        ("0.4429", 29),
        ("0.569", 28),
        ("2.883", 28),
        ("496.0", 28),
        ("0.02191", 28),
        ("0.02954", 28),
    ];
    let common_values_hex = &[
        ("0x0.0045e#10", 31),
        ("0x1.598#10", 29),
        ("0x0.d08#10", 29),
        ("0x1.b00#10", 29),
        ("0x0.716#10", 29),
        ("0x0.91c#10", 28),
        ("0x2.e2#10", 28),
        ("0x1f0.0#10", 28),
        ("0x0.059c#10", 28),
        ("0x0.0790#10", 28),
    ];
    let sample_median = ("1.471", None);
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
        "3.2729513077064011786e-37",
        "7.8519772600462495573e-34",
        "5.9376349676904431794e-6",
        "2.9347251290514630352e-45",
        "3116635254961129.0696",
        "7.092177112370390978e-10",
        "28479.702681773003178",
        "0.113243462684988497952",
        "0.00022759870347783143164",
        "272140612311933.89714",
        "2.2402756790380859597e34",
        "1.5688270582533720942e-12",
        "0.00021182873722550371128",
        "15413050806.4212524556",
        "1.6148555338067285629e33",
        "0.16336131070033468082",
        "0.000058561097760533431471",
        "7669129173769245662.5",
        "0.063833035064485007692",
        "1020425646.4661882869",
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
    let sample_median = ("1.4744048330756642118", Some("1.4745500033376170913"));
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
