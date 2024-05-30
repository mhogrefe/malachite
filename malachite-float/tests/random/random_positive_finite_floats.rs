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
use malachite_float::random::random_positive_finite_floats;
use malachite_float::test_util::random::random_floats_helper_helper;

fn random_positive_finite_floats_helper(
    mean_sci_exponent_abs_numerator: u64,
    mean_sci_exponent_abs_denominator: u64,
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
        random_positive_finite_floats(
            EXAMPLE_SEED,
            mean_sci_exponent_abs_numerator,
            mean_sci_exponent_abs_denominator,
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
fn test_random_positive_finite_floats() {
    // mean |sci_exponent| 1, mean precision 2
    let values = &[
        "1.5", "1.0", "2.0", "1.0", "1.0", "2.0", "2.0", "1.2", "2.0", "1.0", "2.0", "2.0", "4.0",
        "1.0", "1.8", "2.5", "0.1", "1.0", "0.2", "1.2",
    ];
    let values_hex = &[
        "0x1.8#2", "0x1.0#1", "0x2.0#2", "0x1.0#2", "0x1.0#1", "0x2.0#1", "0x2.0#1", "0x1.4#3",
        "0x2.0#1", "0x1.0#1", "0x2.0#1", "0x2.0#1", "0x4.0#1", "0x1.0#1", "0x1.c#3", "0x2.8#4",
        "0x0.2#1", "0x1.0#1", "0x0.4#1", "0x1.4#3",
    ];
    let common_values = &[
        ("1.0", 166114),
        ("0.5", 83464),
        ("2.0", 83434),
        ("1.5", 42025),
        ("0.2", 41531),
        ("1.0", 41506),
        ("4.0", 41483),
        ("3.0", 21005),
        ("2.0", 20892),
        ("0.1", 20888),
    ];
    let common_values_hex = &[
        ("0x1.0#1", 166114),
        ("0x0.8#1", 83464),
        ("0x2.0#1", 83434),
        ("0x1.8#2", 42025),
        ("0x0.4#1", 41531),
        ("0x1.0#2", 41506),
        ("0x4.0#1", 41483),
        ("0x3.0#2", 21005),
        ("0x2.0#2", 20892),
        ("0x0.2#1", 20888),
    ];
    let sample_median = ("1.0", None);
    let sample_median_hex = ("0x1.0#2", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(7.292847587427611),
        standard_deviation: NiceFloat(416.97010060005687),
        skewness: NiceFloat(334.72226371600976),
        excess_kurtosis: NiceFloat(139648.08254659182),
    };
    random_positive_finite_floats_helper(
        1,
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
        "1.3850071337780625300836",
        "3.21176241813",
        "1.981013494834061409257349",
        "1.665272",
        "3.0477509378551710980549069671536911164880571509",
        "3.86646429082638580413691355062141611967080168780483321308586",
        "1.1383718",
        "3.005438259332186398250782844",
        "1.31402772840010613",
        "2.2954821356993",
        "3.341070237",
        "6.42834377982969296",
        "1.055708800484474",
        "1.09154149257425903017951075071293557661092806",
        "2.750421277483313",
        "0.2219335998",
        "1.1563594283786540769774526007723931560664",
        "0.39596234107597",
        "1.3496675",
    ];
    let values_hex = &[
        "0x1.c#3",
        "0x1.628fd3d84db091d430#71",
        "0x3.36360fd46#38",
        "0x1.fb23b34d3f38af725038#79",
        "0x1.aa4f4#19",
        "0x3.0c3967cc70f8444c90df6c12283a15fa4637da#153",
        "0x3.ddd09a90404c02ee3c715fa17bf84c143354d47adb8aff3c0#195",
        "0x1.236c56#24",
        "0x3.016466d9fa99ac91c2dfb4#89",
        "0x1.50641f07840015#57",
        "0x2.4ba4b79d6178#48",
        "0x3.57506108#31",
        "0x6.6da7f01dd04530#57",
        "0x1.0e42ee942e22#48",
        "0x1.176f4364d55e3feb51571e25ef39e528b86a#145",
        "0x2.c01b9bdd036c4#54",
        "0x0.38d0a3f10#31",
        "0x1.28072be74ebd242348b5852a9e4a3c62e#132",
        "0x0.655dc9b95e7#43",
        "0x1.5983d0#24",
    ];
    let common_values = &[
        ("1.0", 5069),
        ("1.0", 2626),
        ("0.5", 2623),
        ("2.0", 2623),
        ("1.5", 2488),
        ("2.0", 1309),
        ("0.2", 1301),
        ("4.0", 1299),
        ("1.2", 1293),
        ("0.8", 1283),
    ];
    let common_values_hex = &[
        ("0x1.0#1", 5069),
        ("0x1.0#2", 2626),
        ("0x0.8#1", 2623),
        ("0x2.0#1", 2623),
        ("0x1.8#2", 2488),
        ("0x2.0#2", 1309),
        ("0x0.4#1", 1301),
        ("0x4.0#1", 1299),
        ("0x1.4#3", 1293),
        ("0x0.c#2", 1283),
    ];
    let sample_median = (
        "1.492046225515012964994532213857812720544842101177086840260174",
        Some("1.492048772960956"),
    );
    let sample_median_hex = (
        "0x1.7df6bdceb50c76e2fdee90d7296fa9b4a2cc3ae315ce8cf010#199",
        Some("0x1.7df6e88be77c#49"),
    );
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(9.138627718915657),
        standard_deviation: NiceFloat(468.606797626186),
        skewness: NiceFloat(309.74563573271297),
        excess_kurtosis: NiceFloat(127224.11304735676),
    };
    random_positive_finite_floats_helper(
        1,
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
        "3.0e-37", "8.0e-34", "4.0e-6", "3.0e-45", "2.0e15", "5.0e-10", "2.0e4", "0.08", "0.0001",
        "1.0e14", "2.0e34", "9.0e-13", "0.0001", "9.0e9", "2.3e33", "0.16", "0.00003", "5.0e18",
        "0.06", "7.0e8",
    ];
    let values_hex = &[
        "0x6.0E-31#2",
        "0x4.0E-28#1",
        "0x0.00004#2",
        "0x1.0E-37#2",
        "0x8.0E+12#1",
        "0x2.0E-8#1",
        "0x4.0E+3#1",
        "0x0.14#3",
        "0x0.0008#1",
        "0x8.0E+11#1",
        "0x4.0E+28#1",
        "0x1.0E-10#1",
        "0x0.0008#1",
        "0x2.0E+8#1",
        "0x7.0E+27#3",
        "0x0.28#4",
        "0x0.0002#1",
        "0x4.0E+15#1",
        "0x0.1#1",
        "0x2.8E+7#3",
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
        mean: NiceFloat(1.9680504915704222e255),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_positive_finite_floats_helper(
        64,
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
        "1.066970684853243558372e-33",
        "6.12595065713e-6",
        "5.55198233623541837705276e-45",
        "3.74986e15",
        "7.096098125575043023691761696603361322749539164e-10",
        "31674.0754704497525074895958066906408523432074264971936815994",
        "0.07114824",
        "0.0001834373937580680174713612576",
        "184932962124288.041",
        "2.38376493640286e34",
        "1.519342839e-12",
        "0.000196177483515310454",
        "9068469544.36041",
        "1416901865713435510498832543218638.47378814837",
        "0.17190132984270706",
        "0.00005418300776",
        "5332766608130525448.821004944540229640534",
        "0.09899058526899",
        "7.2459725e8",
    ];
    let values_hex = &[
        "0x7.0E-31#3",
        "0x5.8a3f4f6136c24750cE-28#71",
        "0x0.000066c6c1fa8c#38",
        "0x1.fb23b34d3f38af725038E-37#79",
        "0xd.527aE+12#19",
        "0x3.0c3967cc70f8444c90df6c12283a15fa4637daE-8#153",
        "0x7bba.13520809805dc78e2bf42f7f0982866a9a8f5b715fe78#195",
        "0x0.1236c56#24",
        "0x0.000c05919b67ea66b2470b7ed0#89",
        "0xa8320f83c200.0a8#57",
        "0x4.97496f3ac2f0E+28#48",
        "0x1.aba83084E-10#31",
        "0x0.000cdb4fe03ba08a60#57",
        "0x21c85dd28.5c44#48",
        "0x45dbd0d935578ffad455c7897bce.794a2e1a8#145",
        "0x0.2c01b9bdd036c4#54",
        "0x0.00038d0a3f10#31",
        "0x4a01caf9d3af4908.d22d614aa7928f18b8#132",
        "0x0.1957726e579c#43",
        "0x2.b307a0E+7#24",
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
    let sample_median = (
        "1.45263",
        Some("1.4527665236460671657422963814630798553398772791"),
    );
    let sample_median_hex = (
        "0x1.73df8#18",
        Some("0x1.73e881c3c8916f41f0bcfa9b4958a2e850b2da0#154"),
    );
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1.9911135832684696e255),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_positive_finite_floats_helper(
        64,
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
fn random_positive_finite_floats_fail_1() {
    random_positive_finite_floats(EXAMPLE_SEED, 1, 0, 2, 1);
}

#[test]
#[should_panic]
fn random_positive_finite_floats_fail_2() {
    random_positive_finite_floats(EXAMPLE_SEED, 0, 1, 2, 1);
}

#[test]
#[should_panic]
fn random_positive_finite_floats_fail_3() {
    random_positive_finite_floats(EXAMPLE_SEED, 1, 1, 1, 0);
}

#[test]
#[should_panic]
fn random_positive_finite_floats_fail_4() {
    random_positive_finite_floats(EXAMPLE_SEED, 1, 1, 1, 1);
}
