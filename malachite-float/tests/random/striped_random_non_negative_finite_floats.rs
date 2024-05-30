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
use malachite_float::random::striped_random_non_negative_finite_floats;
use malachite_float::test_util::random::random_floats_helper_helper;

fn striped_random_non_negative_finite_floats_helper(
    mean_sci_exponent_abs_numerator: u64,
    mean_sci_exponent_abs_denominator: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_precision_numerator: u64,
    mean_precision_denominator: u64,
    zero_p_numerator: u64,
    zero_p_denominator: u64,
    expected_values: &[&str],
    expected_values_hex: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_common_values_hex: &[(&str, usize)],
    expected_median: (&str, Option<&str>),
    expected_median_hex: (&str, Option<&str>),
    expected_moment_stats: MomentStats,
) {
    random_floats_helper_helper(
        striped_random_non_negative_finite_floats(
            EXAMPLE_SEED,
            mean_sci_exponent_abs_numerator,
            mean_sci_exponent_abs_denominator,
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_precision_numerator,
            mean_precision_denominator,
            zero_p_numerator,
            zero_p_denominator,
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
fn test_striped_random_non_negative_finite_floats() {
    // mean |sci_exponent| 1, mean precision 2
    let values = &[
        "8.0", "1.0", "0.8", "0.0", "4.0", "3.5", "1.0", "2.0", "0.5", "0.5", "2.0", "3.0e1",
        "0.5", "0.0", "1.0", "3.0", "0.5", "0.0", "1.0", "0.1",
    ];
    let values_hex = &[
        "0x8.0#1",
        "0x1.0#1",
        "0x0.c#2",
        "0x0.0",
        "0x4.0#1",
        "0x3.8#3",
        "0x1.0#1",
        "0x2.0#1",
        "0x0.8#2",
        "0x0.800#12",
        "0x2.0#1",
        "0x2.0E+1#1",
        "0x0.8#1",
        "0x0.0",
        "0x1.0#1",
        "0x3.0#2",
        "0x0.8#1",
        "0x0.0",
        "0x1.0#2",
        "0x0.2#1",
    ];
    let common_values = &[
        ("1.0", 149586),
        ("0.0", 100224),
        ("2.0", 74929),
        ("0.5", 74827),
        ("1.0", 38123),
        ("4.0", 37612),
        ("0.2", 37602),
        ("1.5", 37379),
        ("0.5", 18928),
        ("0.8", 18840),
    ];
    let common_values_hex = &[
        ("0x1.0#1", 149586),
        ("0x0.0", 100224),
        ("0x2.0#1", 74929),
        ("0x0.8#1", 74827),
        ("0x1.0#2", 38123),
        ("0x4.0#1", 37612),
        ("0x0.4#1", 37602),
        ("0x1.8#2", 37379),
        ("0x0.8#2", 18928),
        ("0x0.c#2", 18840),
    ];
    let sample_median = ("1.0", None);
    let sample_median_hex = ("0x1.0#1", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(8.756520872349506),
        standard_deviation: NiceFloat(1567.2504048767603),
        skewness: NiceFloat(588.0046776041995),
        excess_kurtosis: NiceFloat(368204.8297165849),
    };
    striped_random_non_negative_finite_floats_helper(
        1,
        1,
        8,
        1,
        2,
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

    // mean |sci_exponent| 1, mean precision 64
    let values = &[
        "15.99996948242187500002710174558968740475801788817904844007080242775028492173761449748527\
        50344620578",
        "1.0",
        "0.9999961855",
        "0.0",
        "7.9999923707218827326204245",
        "3.99902343750000005551115123125625546225",
        "1.999999963",
        "3.9687500000000568434188608076",
        "0.99999999999999645",
        "0.5",
        "3.5",
        "63.9375",
        "0.5019531249",
        "0.0",
        "1.5",
        "3.8",
        "0.5",
        "0.0",
        "1.0",
        "0.23",
    ];
    let values_hex = &[
        "0xf.fffe0000000000007ffc0000000007ffffff8003fe00000000000003fff0003ffffffff8ff000000#324",
        "0x1.0#1",
        "0x0.ffffc001#32",
        "0x0.0",
        "0x7.ffff80007fff9f80000fe#86",
        "0x3.ffc00000000003ffffffffffe01fffe#125",
        "0x1.ffffff6#28",
        "0x3.f8000000000fffffffffffe#93",
        "0x0.ffffffffffff00#55",
        "0x0.800#12",
        "0x3.8000#16",
        "0x3f.f00000000000000#65",
        "0x0.807fffff8#33",
        "0x0.0",
        "0x1.8#2",
        "0x3.c#4",
        "0x0.8000000000#37",
        "0x0.0",
        "0x1.00#9",
        "0x0.3c#4",
    ];
    let common_values = &[
        ("0.0", 100224),
        ("1.0", 4643),
        ("2.0", 2373),
        ("0.5", 2353),
        ("1.5", 2346),
        ("1.0", 2275),
        ("1.0", 2183),
        ("1.8", 2131),
        ("1.0", 2125),
        ("1.9", 2082),
    ];
    let common_values_hex = &[
        ("0x0.0", 100224),
        ("0x1.0#1", 4643),
        ("0x2.0#1", 2373),
        ("0x0.8#1", 2353),
        ("0x1.8#2", 2346),
        ("0x1.0#2", 2275),
        ("0x1.0#3", 2183),
        ("0x1.c#3", 2131),
        ("0x1.0#4", 2125),
        ("0x1.e#4", 2082),
    ];
    let sample_median = (
        "1.000007629394531249993647252693644355720408559059500172572",
        Some("1.000007629394531249997"),
    );
    let sample_median_hex = (
        "0x1.00007fffffffffffe1fffff000000000000000000000000#189",
        Some("0x1.00007ffffffffffff#69"),
    );
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(11.589283909473794),
        standard_deviation: NiceFloat(2398.6644729176514),
        skewness: NiceFloat(671.6561608456597),
        excess_kurtosis: NiceFloat(499631.39656685427),
    };
    striped_random_non_negative_finite_floats_helper(
        1,
        1,
        32,
        1,
        64,
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

    // mean |sci_exponent| 64, mean precision 2
    let values = &[
        "1.0e9", "2.0e-6", "24.0", "0.0", "5.0e-7", "0.11", "8.0e22", "7.0e19", "0.03", "3.355e7",
        "2.0e38", "7.0e-9", "2.0e15", "0.0", "3.0e11", "2.0e-8", "6.0e14", "0.0", "0.03",
        "2.0e-34",
    ];
    let values_hex = &[
        "0x4.0E+7#1",
        "0x0.00002#1",
        "0x18.0#2",
        "0x0.0",
        "0x8.0E-6#1",
        "0x0.1c#3",
        "0x1.0E+19#1",
        "0x4.0E+16#1",
        "0x0.08#2",
        "0x2.000E+6#12",
        "0x8.0E+31#1",
        "0x2.0E-7#1",
        "0x8.0E+12#1",
        "0x0.0",
        "0x4.0E+9#1",
        "0x6.0E-7#2",
        "0x2.0E+12#1",
        "0x0.0",
        "0x0.08#2",
        "0x1.0E-28#1",
    ];
    let common_values = &[
        ("0.0", 100224),
        ("1.0", 3531),
        ("2.0", 3503),
        ("4.0", 3399),
        ("0.5", 3381),
        ("0.1", 3375),
        ("0.2", 3369),
        ("0.06", 3330),
        ("8.0", 3283),
        ("2.0e1", 3250),
    ];
    let common_values_hex = &[
        ("0x0.0", 100224),
        ("0x1.0#1", 3531),
        ("0x2.0#1", 3503),
        ("0x4.0#1", 3399),
        ("0x0.8#1", 3381),
        ("0x0.2#1", 3375),
        ("0x0.4#1", 3369),
        ("0x0.1#1", 3330),
        ("0x8.0#1", 3283),
        ("0x1.0E+1#1", 3250),
    ];
    let sample_median = ("0.007", None);
    let sample_median_hex = ("0x0.01c#3", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(2.2374143686309695e242),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    striped_random_non_negative_finite_floats_helper(
        64,
        1,
        8,
        1,
        2,
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

    // mean |sci_exponent| 64, mean precision 64
    let values = &[
        "2147479552.000000000003637534717881863696837550734749938828247260989527393548280429992335\
        3285806191",
        "2.0e-6",
        "31.999877937",
        "0.0",
        "9.5367340692542585523371988e-7",
        "0.124969482421875001734723475976757983195",
        "1.511157246e23",
        "146421031085071663103.99999999",
        "0.062499999999999778",
        "3.355e7",
        "2.97747e38",
        "1.48866092786192893982e-8",
        "2.2605959062e15",
        "0.0",
        "4.0e11",
        "2.8e-8",
        "5.6294995342e14",
        "0.0",
        "0.0312",
        "3.6e-34",
    ];
    let values_hex = &[
        "0x7ffff000.0000000003ffe0000000003ffffffc001ff00000000000001fff8001ffffffffc7f8000000\
        #324",
        "0x0.00002#1",
        "0x1f.fff8002#32",
        "0x0.0",
        "0xf.ffff0000ffff3f00001fcE-6#86",
        "0x0.1ffe00000000001fffffffffff00ffff#125",
        "0x1.ffffff6E+19#28",
        "0x7f0000000001fffff.ffffffc#93",
        "0x0.0ffffffffffff00#55",
        "0x2.000E+6#12",
        "0xe.000E+31#16",
        "0x3.ff00000000000000E-7#65",
        "0x8.07fffff8E+12#33",
        "0x0.0",
        "0x6.0E+9#2",
        "0x7.8E-7#4",
        "0x2.000000000E+12#37",
        "0x0.0",
        "0x0.0800#9",
        "0x1.eE-28#4",
    ];
    let common_values = &[
        ("0.0", 100224),
        ("8.0", 129),
        ("4.0", 120),
        ("2.0", 107),
        ("0.06", 107),
        ("1.0e3", 106),
        ("3.0e1", 105),
        ("4.0e3", 104),
        ("0.5", 102),
        ("1.0e2", 102),
    ];
    let common_values_hex = &[
        ("0x0.0", 100224),
        ("0x8.0#1", 129),
        ("0x4.0#1", 120),
        ("0x2.0#1", 107),
        ("0x0.1#1", 107),
        ("0x4.0E+2#1", 106),
        ("0x2.0E+1#1", 105),
        ("0x1.0E+3#1", 104),
        ("0x0.8#1", 102),
        ("0x8.0E+1#1", 102),
    ];
    let sample_median = (
        "0.007812499999971581759",
        Some("0.007812499999971581759170316330143455009"),
    );
    let sample_median_hex = (
        "0x0.01fffffffff8003ffc#63",
        Some("0x0.01fffffffff8003ffc00f800000000ff#121"),
    );
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(2.238472715881828e242),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    striped_random_non_negative_finite_floats_helper(
        64,
        1,
        32,
        1,
        64,
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
}

#[test]
#[should_panic]
fn striped_random_non_negative_finite_floats_fail_1() {
    striped_random_non_negative_finite_floats(EXAMPLE_SEED, 1, 0, 2, 1, 2, 1, 1, 10);
}

#[test]
#[should_panic]
fn striped_random_non_negative_finite_floats_fail_2() {
    striped_random_non_negative_finite_floats(EXAMPLE_SEED, 0, 1, 2, 1, 2, 1, 1, 10);
}

#[test]
#[should_panic]
fn striped_random_non_negative_finite_floats_fail_3() {
    striped_random_non_negative_finite_floats(EXAMPLE_SEED, 1, 1, 2, 1, 1, 0, 1, 10);
}

#[test]
#[should_panic]
fn striped_random_non_negative_finite_floats_fail_4() {
    striped_random_non_negative_finite_floats(EXAMPLE_SEED, 1, 1, 2, 1, 1, 1, 1, 10);
}

#[test]
#[should_panic]
fn striped_random_non_negative_finite_floats_fail_5() {
    striped_random_non_negative_finite_floats(EXAMPLE_SEED, 1, 1, 2, 3, 2, 1, 1, 10);
}

#[test]
#[should_panic]
fn striped_random_non_negative_finite_floats_fail_6() {
    striped_random_non_negative_finite_floats(EXAMPLE_SEED, 1, 1, 1, 0, 2, 1, 1, 10);
}

#[test]
#[should_panic]
fn striped_random_non_negative_finite_floats_fail_7() {
    striped_random_non_negative_finite_floats(EXAMPLE_SEED, 1, 1, 2, 1, 2, 1, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_non_negative_finite_floats_fail_8() {
    striped_random_non_negative_finite_floats(EXAMPLE_SEED, 1, 1, 2, 1, 2, 1, 1, 0);
}
