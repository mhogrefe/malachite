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
use malachite_float::random::striped_random_floats;
use malachite_float::test_util::random::random_floats_helper_helper;

fn striped_random_floats_helper(
    mean_sci_exponent_abs_numerator: u64,
    mean_sci_exponent_abs_denominator: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_precision_numerator: u64,
    mean_precision_denominator: u64,
    mean_special_p_numerator: u64,
    mean_special_p_denominator: u64,
    expected_values: &[&str],
    expected_values_hex: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_common_values_hex: &[(&str, usize)],
    expected_median: (&str, Option<&str>),
    expected_median_hex: (&str, Option<&str>),
    expected_moment_stats: MomentStats,
) {
    random_floats_helper_helper(
        striped_random_floats(
            EXAMPLE_SEED,
            mean_sci_exponent_abs_numerator,
            mean_sci_exponent_abs_denominator,
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_precision_numerator,
            mean_precision_denominator,
            mean_special_p_numerator,
            mean_special_p_denominator,
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

#[test]
fn test_striped_random_floats() {
    // mean |sci_exponent| 1, mean precision 2, special probability 1/10
    let values = &[
        "1.0", "0.2", "0.0", "NaN", "-0.2", "-1.0", "-1.0", "-1.5", "2.0", "0.2", "-1.0", "-0.2",
        "-2.0", "Infinity", "-0.5", "-1.5", "-0.2", "NaN", "0.5", "-0.0",
    ];
    let values_hex = &[
        "0x1.0#1", "0x0.4#1", "0x0.0", "NaN", "-0x0.4#1", "-0x1.0#2", "-0x1.0#1", "-0x1.8#2",
        "0x2.0#1", "0x0.4#1", "-0x1.0#3", "-0x0.4#2", "-0x2.0#1", "Infinity", "-0x0.8#1",
        "-0x1.8#2", "-0x0.4#2", "NaN", "0x0.8#1", "-0x0.0",
    ];
    let common_values = &[
        ("-1.0", 67626),
        ("1.0", 67293),
        ("0.0", 45033),
        ("-0.0", 44760),
        ("-2.0", 33918),
        ("2.0", 33760),
        ("0.5", 33658),
        ("Infinity", 33640),
        ("-0.5", 33629),
        ("NaN", 33393),
    ];
    let common_values_hex = &[
        ("-0x1.0#1", 67626),
        ("0x1.0#1", 67293),
        ("0x0.0", 45033),
        ("-0x0.0", 44760),
        ("-0x2.0#1", 33918),
        ("0x2.0#1", 33760),
        ("0x0.8#1", 33658),
        ("Infinity", 33640),
        ("-0x0.8#1", 33629),
        ("NaN", 33393),
    ];
    let sample_median = ("NaN", None);
    let sample_median_hex = ("NaN", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(f64::NAN),
        standard_deviation: NiceFloat(f64::NAN),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    striped_random_floats_helper(
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

    // mean |sci_exponent| 1, mean precision 64, special probability 1/10
    let values = &[
        "1.99999",
        "0.4999999850988",
        "0.0",
        "NaN",
        "-0.25000000005820766091346740721",
        "-1.0",
        "-1.99999999999999999999978989612421431724891016",
        "-1.0000000000145519117589199127777235073172084570819473318104658",
        "3.97",
        "0.250057220575342853408074206771059911",
        "-1.0000305175708774606",
        "-0.49988",
        "-2.999969497323036190378209298059007584907370931259",
        "Infinity",
        "-0.9999999999999996",
        "-1.5",
        "-0.25",
        "NaN",
        "0.531249999999943",
        "-0.0",
    ];
    let values_hex = &[
        "0x1.ffff8#18",
        "0x0.7fffffc0000#40",
        "0x0.0",
        "NaN",
        "-0x0.400000003fffffffffffffff#95",
        "-0x1.00000000#30",
        "-0x1.ffffffffffffffffff01fffffc0001ffffff8#146",
        "-0x1.000000000fffffc0000003ffffffffffffffffffffff80003ff#205",
        "0x3.f8#7",
        "0x0.4003c0007ff0000000003dfffffffe#118",
        "-0x1.0001fffff807ffc0#62",
        "-0x0.7ff8#14",
        "-0x2.fffe003fffffffc000000000ffffff000001ffff#162",
        "Infinity",
        "-0x0.ffffffffffffe#51",
        "-0x1.8#2",
        "-0x0.400000#20",
        "NaN",
        "0x0.87fffffffff0#48",
        "-0x0.0",
    ];
    let common_values = &[
        ("0.0", 45033),
        ("-0.0", 44760),
        ("Infinity", 33640),
        ("NaN", 33393),
        ("-Infinity", 33191),
        ("1.0", 2133),
        ("-1.0", 2122),
        ("-2.0", 1103),
        ("0.5", 1094),
        ("2.0", 1075),
    ];
    let common_values_hex = &[
        ("0x0.0", 45033),
        ("-0x0.0", 44760),
        ("Infinity", 33640),
        ("NaN", 33393),
        ("-Infinity", 33191),
        ("0x1.0#1", 2133),
        ("-0x1.0#1", 2122),
        ("-0x2.0#1", 1103),
        ("0x0.8#1", 1094),
        ("0x2.0#1", 1075),
    ];
    let sample_median = ("NaN", None);
    let sample_median_hex = ("NaN", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(f64::NAN),
        standard_deviation: NiceFloat(f64::NAN),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    striped_random_floats_helper(
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

    // mean |sci_exponent| 64, mean precision 2, special probability 1/10
    let values = &[
        "5.0e11", "7.0e122", "0.0", "NaN", "-2.0e25", "-2.0e21", "-2.0e-9", "-4.0e33", "4.0e-9",
        "5.0e2", "-4.0e-16", "-7.0e-18", "-2.0e-19", "Infinity", "-2.0e50", "-1.6e12", "-2.0e-28",
        "NaN", "0.02", "-0.0",
    ];
    let values_hex = &[
        "0x8.0E+9#1",
        "0x1.0E+102#1",
        "0x0.0",
        "NaN",
        "-0x1.0E+21#1",
        "-0x8.0E+17#2",
        "-0x8.0E-8#1",
        "-0xc.0E+27#2",
        "0x1.0E-7#1",
        "0x2.0E+2#1",
        "-0x2.0E-13#3",
        "-0x8.0E-15#2",
        "-0x4.0E-16#1",
        "Infinity",
        "-0x8.0E+41#1",
        "-0x1.8E+10#2",
        "-0x1.0E-23#2",
        "NaN",
        "0x0.04#1",
        "-0x0.0",
    ];
    let common_values = &[
        ("0.0", 45033),
        ("-0.0", 44760),
        ("Infinity", 33640),
        ("NaN", 33393),
        ("-Infinity", 33191),
        ("0.5", 1583),
        ("-1.0", 1560),
        ("2.0", 1542),
        ("-2.0", 1523),
        ("1.0", 1518),
    ];
    let common_values_hex = &[
        ("0x0.0", 45033),
        ("-0x0.0", 44760),
        ("Infinity", 33640),
        ("NaN", 33393),
        ("-Infinity", 33191),
        ("0x0.8#1", 1583),
        ("-0x1.0#1", 1560),
        ("0x2.0#1", 1542),
        ("-0x2.0#1", 1523),
        ("0x1.0#1", 1518),
    ];
    let sample_median = ("NaN", None);
    let sample_median_hex = ("NaN", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(f64::NAN),
        standard_deviation: NiceFloat(f64::NAN),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    striped_random_floats_helper(
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

    // mean |sci_exponent| 64, mean precision 64, special probability 1/10
    let values = &[
        "1.099507e12",
        "1.322111898178e123",
        "0.0",
        "NaN",
        "-19342813118337666422669311.999",
        "-2.361183241e21",
        "-3.72529029846191406249960865103493317449262395e-9",
        "-2596148429305192737121006105133055.9999999999999999999864475761",
        "7.4e-9",
        "512.117187738302163779735975467130698",
        "-4.441027623740001309e-16",
        "-1.38744e-17",
        "-3.252573446387882516080103398476707881754715262554e-19",
        "Infinity",
        "-3.74144419156711e50",
        "-1.6e12",
        "-2.019484e-28",
        "NaN",
        "0.0166015624999982",
        "-0.0",
    ];
    let values_hex = &[
        "0xf.fffcE+9#18",
        "0x1.ffffff0000E+102#40",
        "0x0.0",
        "NaN",
        "-0x100000000fffffffffffff.ffc#95",
        "-0x8.0000000E+17#30",
        "-0xf.fffffffffffffffff80fffffe0000ffffffcE-8#146",
        "-0x8000000007ffffe0000001ffffff.ffffffffffffffffc0001ff8#205",
        "0x1.fcE-7#7",
        "0x200.1e0003ff8000000001effffffff#118",
        "-0x2.0003fffff00fff8E-13#62",
        "-0xf.ff0E-15#14",
        "-0x5.fffc007fffffff8000000001fffffe000003fffeE-16#162",
        "Infinity",
        "-0xf.fffffffffffeE+41#51",
        "-0x1.8E+10#2",
        "-0x1.00000E-23#20",
        "NaN",
        "0x0.043fffffffff80#48",
        "-0x0.0",
    ];
    let common_values = &[
        ("0.0", 45033),
        ("-0.0", 44760),
        ("Infinity", 33640),
        ("NaN", 33393),
        ("-Infinity", 33191),
        ("0.00003", 60),
        ("-0.5", 57),
        ("-6.0e1", 56),
        ("-5.0e2", 55),
        ("8.0", 54),
    ];
    let common_values_hex = &[
        ("0x0.0", 45033),
        ("-0x0.0", 44760),
        ("Infinity", 33640),
        ("NaN", 33393),
        ("-Infinity", 33191),
        ("0x0.0002#1", 60),
        ("-0x0.8#1", 57),
        ("-0x4.0E+1#1", 56),
        ("-0x2.0E+2#1", 55),
        ("0x8.0#1", 54),
    ];
    let sample_median = ("NaN", None);
    let sample_median_hex = ("NaN", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(f64::NAN),
        standard_deviation: NiceFloat(f64::NAN),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    striped_random_floats_helper(
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
fn striped_random_floats_fail_1() {
    striped_random_floats(EXAMPLE_SEED, 1, 0, 2, 1, 2, 1, 1, 10);
}

#[test]
#[should_panic]
fn striped_random_floats_fail_2() {
    striped_random_floats(EXAMPLE_SEED, 0, 1, 2, 1, 2, 1, 1, 10);
}

#[test]
#[should_panic]
fn striped_random_floats_fail_3() {
    striped_random_floats(EXAMPLE_SEED, 1, 1, 2, 1, 1, 0, 1, 10);
}

#[test]
#[should_panic]
fn striped_random_floats_fail_4() {
    striped_random_floats(EXAMPLE_SEED, 1, 1, 2, 1, 1, 1, 1, 10);
}

#[test]
#[should_panic]
fn striped_random_floats_fail_5() {
    striped_random_floats(EXAMPLE_SEED, 1, 1, 2, 3, 2, 1, 1, 10);
}

#[test]
#[should_panic]
fn striped_random_floats_fail_6() {
    striped_random_floats(EXAMPLE_SEED, 1, 1, 1, 0, 2, 1, 1, 10);
}

#[test]
#[should_panic]
fn striped_random_floats_fail_7() {
    striped_random_floats(EXAMPLE_SEED, 1, 1, 2, 1, 2, 1, 1, 0);
}
