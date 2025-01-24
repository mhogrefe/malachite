// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::f64;
use malachite_base::num::float::NiceFloat;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::moments::MomentStats;
use malachite_q::random::striped_random_rational_range_to_negative_infinity;
use malachite_q::test_util::random::random_rationals_helper_helper;
use malachite_q::Rational;
use std::str::FromStr;

fn striped_random_rational_range_to_negative_infinity_helper(
    a: &str,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_sample_median: (&str, Option<&str>),
    expected_sample_moment_stats: MomentStats,
) {
    random_rationals_helper_helper(
        striped_random_rational_range_to_negative_infinity(
            EXAMPLE_SEED,
            Rational::from_str(a).unwrap(),
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_bits_numerator,
            mean_bits_denominator,
        ),
        expected_values,
        expected_common_values,
        expected_sample_median,
        expected_sample_moment_stats,
    );
}

#[test]
fn test_striped_random_rational_range_to_negative_infinity() {
    let values = &[
        "-271/512",
        "-6171",
        "-566935683071/512",
        "-76/3",
        "-15/64",
        "-255/2",
        "-4",
        "-48127/16",
        "-11/2048",
        "-3/2",
        "-515",
        "-17242800127/1024",
        "-1048703/124",
        "-234881024/15",
        "-128/31",
        "-8796218851359/8388544",
        "-533112815615/2112",
        "-2/2047",
        "-56/163839",
        "-35840/67108639",
    ];
    let common_values = &[
        ("0", 8352),
        ("-1", 7761),
        ("-1/2", 6447),
        ("-1/3", 4862),
        ("-1/4", 4784),
        ("-1/8", 3643),
        ("-2", 3520),
        ("-3", 3478),
        ("-1/15", 3148),
        ("-1/7", 3059),
    ];
    let sample_median = ("-32", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-4.409358732383749e152),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    striped_random_rational_range_to_negative_infinity_helper(
        "0",
        10,
        1,
        10,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    let values = &[
        "-4503599627887631/512",
        "-135235075",
        "-262115/512",
        "-4198396/3",
        "-7/64",
        "1/2",
        "-37588312048",
        "1/16",
        "-11/2048",
        "-3/2",
        "-1",
        "-63/1024",
        "259/124",
        "-8191/15",
        "-8796093022177/31",
        "-4095/8388544",
        "-1/2112",
        "-8/2047",
        "-67108867/163839",
        "2/67108639",
    ];
    let common_values = &[
        ("0", 7327),
        ("-1", 6761),
        ("1", 6713),
        ("1/2", 5344),
        ("-1/2", 5234),
        ("-1/3", 3833),
        ("1/3", 3826),
        ("1/4", 3810),
        ("-1/4", 3726),
        ("3", 3159),
    ];
    let sample_median = ("-1", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-5.615673330944999e205),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    striped_random_rational_range_to_negative_infinity_helper(
        "245850922/78256779",
        10,
        1,
        10,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    let values = &[
        "-67625999/512",
        "-255",
        "-65535/512",
        "-64/3",
        "-15359/64",
        "-127/2",
        "-4",
        "-51/16",
        "-6447/2048",
        "-12884903935/2",
        "-16",
        "-507907/1024",
        "-481238123007/124",
        "-240652386176/15",
        "-259/31",
        "-8796092760095/8388544",
        "-137438953471/2112",
        "-8127456/2047",
        "-4195072/163839",
        "-34359685144/67108639",
    ];
    let common_values = &[
        ("-7/2", 8188),
        ("-4", 5039),
        ("-7", 5016),
        ("-8", 4043),
        ("-15", 3931),
        ("-13/4", 3381),
        ("-16", 3288),
        ("-15/4", 3136),
        ("-31", 3076),
        ("-15/2", 2959),
    ];
    let sample_median = ("-2175/16", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-7.744407957000211e34),
        standard_deviation: NiceFloat(7.744259426671135e37),
        skewness: NiceFloat(-999.9984995429722),
        excess_kurtosis: NiceFloat(999994.9993924203),
    };
    striped_random_rational_range_to_negative_infinity_helper(
        "-245850922/78256779",
        10,
        1,
        10,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    let values = &[
        "-425/682",
        "-5417",
        "-915560901293/682",
        "-106/3",
        "-13/106",
        "-213/2",
        "-6",
        "-43605/26",
        "-1/2730",
        "-53/2",
        "-30042",
        "-2022571682517/1354",
        "-3413/122",
        "-29320310074197/13",
        "-10/27",
        "-1365/6728362",
        "-421/3402",
        "-218/1365",
        "-10/187029",
        "-123120293/47535445",
    ];
    let common_values = &[
        ("0", 8397),
        ("-1", 7518),
        ("-1/6", 7132),
        ("-1/2", 6454),
        ("-1/3", 5083),
        ("-1/10", 4786),
        ("-2", 3565),
        ("-3", 3511),
        ("-1/42", 3475),
        ("-1/5", 3333),
    ];
    let sample_median = ("-168/5", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-1.94013873735947e225),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    striped_random_rational_range_to_negative_infinity_helper(
        "0",
        11,
        10,
        10,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    let values = &[
        "-3735132622739113/682",
        "-223696213",
        "-218453/682",
        "-5416618/3",
        "-5/106",
        "1/2",
        "-57355358890",
        "-25/26",
        "-5960819038131541/2730",
        "-27317/2",
        "-1453",
        "853/1354",
        "-223696213/122",
        "-42/13",
        "1/27",
        "-53/6728362",
        "-43/3402",
        "-693673/1365",
        "-2863311530/187029",
        "21802/47535445",
    ];
    let common_values = &[
        ("0", 7227),
        ("1", 6794),
        ("-1", 6488),
        ("1/2", 5198),
        ("1/6", 5142),
        ("-1/2", 5003),
        ("-1/6", 4997),
        ("-1/3", 4098),
        ("1/3", 3964),
        ("1/10", 3535),
    ];
    let sample_median = ("-5/6", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-4.8115651702518996e170),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    striped_random_rational_range_to_negative_infinity_helper(
        "245850922/78256779",
        11,
        10,
        10,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    let values = &[
        "-86693545/682",
        "-213",
        "-89461/682",
        "-86/3",
        "-13485/106",
        "-85/2",
        "-5",
        "-85/26",
        "-58640620148053/2730",
        "-85/2",
        "-1791306",
        "-3515733/1354",
        "-5461/122",
        "-5546/13",
        "-425022890/27",
        "-28644288853/6728362",
        "-11605/3402",
        "-218278/1365",
        "-14316545365/187029",
        "-447042218/47535445",
    ];
    let common_values = &[
        ("-7/2", 8241),
        ("-5", 5118),
        ("-6", 5008),
        ("-10", 4145),
        ("-13", 4037),
        ("-53/6", 3903),
        ("-21", 3250),
        ("-26", 3189),
        ("-13/3", 3159),
        ("-85/6", 3149),
    ];
    let sample_median = ("-170", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-3.679955500231244e31),
        standard_deviation: NiceFloat(3.5472081267359855e34),
        skewness: NiceFloat(-997.8746442700499),
        excess_kurtosis: NiceFloat(997092.4918116309),
    };
    striped_random_rational_range_to_negative_infinity_helper(
        "-245850922/78256779",
        11,
        10,
        10,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
}

#[test]
#[should_panic]
fn striped_random_rational_range_to_negative_infinity_fail_1() {
    striped_random_rational_range_to_negative_infinity(
        EXAMPLE_SEED,
        Rational::from_unsigneds(1u32, 3),
        10,
        1,
        10,
        0,
    );
}

#[test]
#[should_panic]
fn striped_random_rational_range_to_negative_infinity_fail_2() {
    striped_random_rational_range_to_negative_infinity(
        EXAMPLE_SEED,
        Rational::from_unsigneds(1u32, 3),
        10,
        1,
        0,
        0,
    );
}

#[test]
#[should_panic]
fn striped_random_rational_range_to_negative_infinity_fail_3() {
    striped_random_rational_range_to_negative_infinity(
        EXAMPLE_SEED,
        Rational::from_unsigneds(1u32, 3),
        10,
        1,
        2,
        3,
    );
}

#[test]
#[should_panic]
fn striped_random_rational_range_to_negative_infinity_fail_4() {
    striped_random_rational_range_to_negative_infinity(
        EXAMPLE_SEED,
        Rational::from_unsigneds(1u32, 3),
        1,
        0,
        10,
        1,
    );
}

#[test]
#[should_panic]
fn striped_random_rational_range_to_negative_infinity_fail_5() {
    striped_random_rational_range_to_negative_infinity(
        EXAMPLE_SEED,
        Rational::from_unsigneds(1u32, 3),
        2,
        3,
        10,
        1,
    );
}
