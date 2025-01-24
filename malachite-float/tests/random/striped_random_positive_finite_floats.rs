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
use malachite_float::random::striped_random_positive_finite_floats;
use malachite_float::test_util::random::random_floats_helper_helper;

fn striped_random_positive_finite_floats_helper(
    mean_sci_exponent_abs_numerator: u64,
    mean_sci_exponent_abs_denominator: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_precision_numerator: u64,
    mean_precision_denominator: u64,
    expected_values: &[&str],
    expected_values_hex: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_common_values_hex: &[(&str, usize)],
    expected_median: (&str, Option<&str>),
    expected_median_hex: (&str, Option<&str>),
    expected_moment_stats: MomentStats,
) {
    random_floats_helper_helper(
        striped_random_positive_finite_floats(
            EXAMPLE_SEED,
            mean_sci_exponent_abs_numerator,
            mean_sci_exponent_abs_denominator,
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_precision_numerator,
            mean_precision_denominator,
        ),
        expected_values,
        expected_values_hex,
        expected_common_values,
        expected_common_values_hex,
        expected_median,
        expected_median_hex,
        expected_moment_stats,
    );
}

#[test]
fn test_striped_random_positive_finite_floats() {
    // mean |sci_exponent| 1, mean precision 2
    let values = &[
        "1.5", "1.0", "2.0", "1.0", "1.0", "2.0", "2.0", "1.8", "2.0", "1.0", "2.0", "2.0", "4.0",
        "1.0", "1.0", "2.0", "0.1", "1.0", "0.2", "1.0",
    ];
    let values_hex = &[
        "0x1.8#2", "0x1.0#1", "0x2.0#2", "0x1.0#2", "0x1.0#1", "0x2.0#1", "0x2.0#1", "0x1.c#3",
        "0x2.0#1", "0x1.0#1", "0x2.0#1", "0x2.0#1", "0x4.0#1", "0x1.0#1", "0x1.0#3", "0x2.0#4",
        "0x0.2#1", "0x1.0#1", "0x0.4#1", "0x1.0#3",
    ];
    let common_values = &[
        ("1.0", 166114),
        ("0.5", 83464),
        ("2.0", 83434),
        ("1.0", 42010),
        ("0.2", 41531),
        ("1.5", 41521),
        ("4.0", 41483),
        ("3.0", 21125),
        ("0.1", 20888),
        ("8.0", 20800),
    ];
    let common_values_hex = &[
        ("0x1.0#1", 166114),
        ("0x0.8#1", 83464),
        ("0x2.0#1", 83434),
        ("0x1.0#2", 42010),
        ("0x0.4#1", 41531),
        ("0x1.8#2", 41521),
        ("0x4.0#1", 41483),
        ("0x3.0#2", 21125),
        ("0x0.2#1", 20888),
        ("0x8.0#1", 20800),
    ];
    let sample_median = ("1.0", None);
    let sample_median_hex = ("0x1.0#2", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(7.102812885245132),
        standard_deviation: NiceFloat(351.5541728670195),
        skewness: NiceFloat(274.1565025440714),
        excess_kurtosis: NiceFloat(92676.98806519402),
    };
    striped_random_positive_finite_floats_helper(
        1,
        1,
        8,
        1,
        2,
        1,
        values,
        values_hex,
        common_values,
        common_values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );

    // mean |sci_exponent| 1, mean precision 64
    let values = &[
        "1.8",
        "1.999992251396406572893",
        "2.0",
        "1.000007629394303879793997",
        "1.0",
        "3.9999999999999999861222122955831198138315944982",
        "2.00048828124999999999322373803755273042776921295151896098423",
        "1.0000609",
        "3.98439049720059301762375898",
        "1.9921875",
        "3.99609375186263",
        "3.9921875",
        "4.0",
        "1.998046875",
        "1.0000000894069671630893256317890172009801151",
        "3.9999997615814209",
        "0.2499990481",
        "1.0000000000000000034694465383634065199217",
        "0.49999999998545",
        "1.9852905",
    ];
    let values_hex = &[
        "0x1.c#3",
        "0x1.ffff7e00003ffffffc#71",
        "0x2.000000000#38",
        "0x1.00007fffffc0003ffff8#79",
        "0x1.00000#19",
        "0x3.ffffffffffffff0000001fffffffffff8007fe#153",
        "0x2.001fffffffffffffe000007fffffff0000000000000001ff8#195",
        "0x1.0003fe#24",
        "0x3.fc0103fff83fffffffff00#89",
        "0x1.fe000000000000#57",
        "0x3.ff000007fffc#48",
        "0x3.fe000000#31",
        "0x4.00000000000000#57",
        "0x1.ff8000000000#48",
        "0x1.00000180000000000fffffffffffff800000#145",
        "0x3.fffffc0000000#54",
        "0x0.3ffff0078#31",
        "0x1.000000000000003fffff800001ffffff8#132",
        "0x0.7ffffffff00#43",
        "0x1.fc3c00#24",
    ];
    let common_values = &[
        ("1.0", 5069),
        ("0.5", 2623),
        ("2.0", 2623),
        ("1.0", 2567),
        ("1.5", 2547),
        ("1.8", 2419),
        ("1.0", 2408),
        ("1.9", 2381),
        ("1.0", 2336),
        ("1.0", 2240),
    ];
    let common_values_hex = &[
        ("0x1.0#1", 5069),
        ("0x0.8#1", 2623),
        ("0x2.0#1", 2623),
        ("0x1.0#2", 2567),
        ("0x1.8#2", 2547),
        ("0x1.c#3", 2419),
        ("0x1.0#3", 2408),
        ("0x1.e#4", 2381),
        ("0x1.0#4", 2336),
        ("0x1.00#6", 2240),
    ];
    let sample_median = ("1.4999997616", Some("1.4999997616"));
    let sample_median_hex = ("0x1.7ffffc00#32", Some("0x1.7ffffc00#33"));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(9.303344556597335),
        standard_deviation: NiceFloat(516.4696271945669),
        skewness: NiceFloat(332.951705809113),
        excess_kurtosis: NiceFloat(144341.98602578667),
    };
    striped_random_positive_finite_floats_helper(
        1,
        1,
        32,
        1,
        64,
        1,
        values,
        values_hex,
        common_values,
        common_values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );

    // mean |sci_exponent| 64, mean precision 2
    let values = &[
        "3.0e-37", "8.0e-34", "4.0e-6", "3.0e-45", "2.0e15", "5.0e-10", "2.0e4", "0.11", "0.0001",
        "1.0e14", "2.0e34", "9.0e-13", "0.0001", "9.0e9", "1.3e33", "0.12", "0.00003", "5.0e18",
        "0.06", "5.0e8",
    ];
    let values_hex = &[
        "0x6.0E-31#2",
        "0x4.0E-28#1",
        "0x0.00004#2",
        "0x1.0E-37#2",
        "0x8.0E+12#1",
        "0x2.0E-8#1",
        "0x4.0E+3#1",
        "0x0.1c#3",
        "0x0.0008#1",
        "0x8.0E+11#1",
        "0x4.0E+28#1",
        "0x1.0E-10#1",
        "0x0.0008#1",
        "0x2.0E+8#1",
        "0x4.0E+27#3",
        "0x0.20#4",
        "0x0.0002#1",
        "0x4.0E+15#1",
        "0x0.1#1",
        "0x2.0E+7#3",
    ];
    let common_values = &[
        ("1.0", 3831),
        ("0.2", 3830),
        ("0.5", 3826),
        ("2.0", 3753),
        ("4.0", 3716),
        ("8.0", 3653),
        ("0.1", 3639),
        ("0.06", 3637),
        ("3.0e1", 3632),
        ("2.0e1", 3617),
    ];
    let common_values_hex = &[
        ("0x1.0#1", 3831),
        ("0x0.4#1", 3830),
        ("0x0.8#1", 3826),
        ("0x2.0#1", 3753),
        ("0x4.0#1", 3716),
        ("0x8.0#1", 3653),
        ("0x0.2#1", 3639),
        ("0x0.1#1", 3637),
        ("0x2.0E+1#1", 3632),
        ("0x1.0E+1#1", 3617),
    ];
    let sample_median = ("1.0", None);
    let sample_median_hex = ("0x1.0#1", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(2.952075737355551e255),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    striped_random_positive_finite_floats_helper(
        64,
        1,
        8,
        1,
        2,
        1,
        values,
        values_hex,
        common_values,
        common_values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );

    // mean |sci_exponent| 64, mean precision 64
    let values = &[
        "3.3e-37",
        "1.5407379862027135753234e-33",
        "3.81469726562e-6",
        "2.80261831076667765237162e-45",
        "2.2518e15",
        "9.3132257461547851239382575628885994987366592936e-10",
        "16387.9999999999999999444888620036319676642853924988433283828",
        "0.062503807",
        "0.000243187896557653382423325133",
        "280375465082880.0",
        "4.1497810068414e34",
        "1.81543669e-12",
        "0.0001220703125",
        "17163091968.0",
        "1298074330690585590141422900609023.99951171875",
        "0.24999998509883881",
        "0.00006103492385",
        "4611686018427387919.999998092651821934844",
        "0.12499999999636",
        "1.06584474e9",
    ];
    let values_hex = &[
        "0x7.0E-31#3",
        "0x7.fffdf80000fffffffE-28#71",
        "0x0.00004000000000#38",
        "0x1.00007fffffc0003ffff8E-37#79",
        "0x8.0000E+12#19",
        "0x3.ffffffffffffff0000001fffffffffff8007feE-8#153",
        "0x4003.fffffffffffffc00000fffffffe0000000000000003ff#195",
        "0x0.10003fe#24",
        "0x0.000ff0040fffe0fffffffffc00#89",
        "0xff0000000000.000#57",
        "0x7.fe00000ffff8E+28#48",
        "0x1.ff000000E-10#31",
        "0x0.000800000000000000#57",
        "0x3ff000000.0000#48",
        "0x400000600000000003ffffffffff.ffe000000#145",
        "0x0.3fffffc0000000#54",
        "0x0.0003ffff0078#31",
        "0x400000000000000f.ffffe000007fffffe0#132",
        "0x0.1ffffffffc00#43",
        "0x3.f87800E+7#24",
    ];
    let common_values = &[
        ("2.0", 130),
        ("2.0e1", 129),
        ("0.2", 122),
        ("3.0e1", 117),
        ("0.004", 116),
        ("1.0e5", 114),
        ("0.02", 112),
        ("0.00002", 112),
        ("0.1", 109),
        ("8.0", 108),
    ];
    let common_values_hex = &[
        ("0x2.0#1", 130),
        ("0x1.0E+1#1", 129),
        ("0x0.4#1", 122),
        ("0x2.0E+1#1", 117),
        ("0x0.01#1", 116),
        ("0x2.0E+4#1", 114),
        ("0x0.04#1", 112),
        ("0x0.0001#1", 112),
        ("0x0.2#1", 109),
        ("0x8.0#1", 108),
    ];
    let sample_median = ("1.12499988079071", Some("1.1249998807907122255"));
    let sample_median_hex = ("0x1.1ffffe000000#49", Some("0x1.1ffffe0000007ffe#64"));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(3.8054101301847865e255),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    striped_random_positive_finite_floats_helper(
        64,
        1,
        32,
        1,
        64,
        1,
        values,
        values_hex,
        common_values,
        common_values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );
}

#[test]
#[should_panic]
fn striped_random_positive_finite_floats_fail_1() {
    striped_random_positive_finite_floats(EXAMPLE_SEED, 1, 0, 2, 1, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_positive_finite_floats_fail_2() {
    striped_random_positive_finite_floats(EXAMPLE_SEED, 0, 1, 2, 1, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_positive_finite_floats_fail_3() {
    striped_random_positive_finite_floats(EXAMPLE_SEED, 1, 1, 2, 1, 1, 0);
}

#[test]
#[should_panic]
fn striped_random_positive_finite_floats_fail_4() {
    striped_random_positive_finite_floats(EXAMPLE_SEED, 1, 1, 2, 1, 1, 1);
}

#[test]
#[should_panic]
fn striped_random_positive_finite_floats_fail_5() {
    striped_random_positive_finite_floats(EXAMPLE_SEED, 1, 1, 2, 3, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_positive_finite_floats_fail_6() {
    striped_random_positive_finite_floats(EXAMPLE_SEED, 1, 1, 1, 0, 2, 1);
}
