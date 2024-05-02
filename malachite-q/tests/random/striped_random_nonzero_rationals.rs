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
use malachite_q::random::striped_random_nonzero_rationals;
use malachite_q::test_util::random::random_rationals_helper_helper;

#[allow(clippy::too_many_arguments)]
fn striped_random_nonzero_rationals_helper(
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
        striped_random_nonzero_rationals(
            EXAMPLE_SEED,
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
fn test_striped_random_nonzero_rationals() {
    // mean bits = 65/64
    let values = &[
        "-1", "-1", "1", "1", "-1", "1", "1", "1", "1", "1", "1", "-1", "1", "-1", "-1", "-1",
        "-1", "-1", "-1", "-1",
    ];
    let common_values = &[
        ("-1", 484931),
        ("1", 484658),
        ("3", 3817),
        ("1/2", 3814),
        ("-1/2", 3763),
        ("1/3", 3737),
        ("-2", 3697),
        ("-1/3", 3678),
        ("2", 3660),
        ("-3", 3636),
    ];
    let sample_median = ("-1/7", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.00023288528138526625),
        standard_deviation: NiceFloat(1.037925742275186),
        skewness: NiceFloat(0.0009392339727778673),
        excess_kurtosis: NiceFloat(-1.2291100464705937),
    };
    striped_random_nonzero_rationals_helper(
        4,
        1,
        65,
        64,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    // mean bits = 2
    let values = &[
        "-14", "-1/7", "6", "1/3", "-1", "1/5", "1", "4", "1/31", "4/3", "1/3", "-2", "2", "-1/15",
        "-2", "-1/68", "-37", "-23", "-8/15", "-1",
    ];
    let common_values = &[
        ("1", 143643),
        ("-1", 143385),
        ("-2", 36002),
        ("1/2", 35965),
        ("-1/2", 35954),
        ("2", 35814),
        ("-3", 33135),
        ("-1/3", 33023),
        ("1/3", 32961),
        ("3", 32694),
    ];
    let sample_median = ("-1/12767", Some("-1/15422"));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.06690886383057024),
        standard_deviation: NiceFloat(743.3873846358883),
        skewness: NiceFloat(-34.20312291886276),
        excess_kurtosis: NiceFloat(162779.3378205672),
    };
    striped_random_nonzero_rationals_helper(
        4,
        1,
        2,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    // mean bits = 32
    let values = &[
        "-68720000000/18006083452797439",
        "-2545165805/29",
        "549754781664/1236950581247",
        "1065353727/2047",
        "-2147745791/513",
        "16128/575",
        "8192/17000482516899619632318463",
        "18431/16778240",
        "1/31",
        "4096/526335",
        "128/1648730570755",
        "-235590160969/299520",
        "256/1015",
        "-134213640/1015807",
        "-2135296358076219148941708516212794237273505792/134217731",
        "-1099511504895/144117387099111422",
        "-18854877723802865238015/1180663678191190081543",
        "-8191/327168",
        "-1/127",
        "-79/4095",
    ];
    let common_values = &[
        ("1", 1790),
        ("-1", 1776),
        ("1/2", 915),
        ("2", 911),
        ("-1/2", 882),
        ("-2", 880),
        ("-4", 815),
        ("4", 804),
        ("1/4", 790),
        ("-1/4", 787),
    ];
    let sample_median = (
        "-32/1667110261681770706918410051940899342228396408049959135280966429665234272396880613105\
        8561778175",
        Some(
            "-589823/54681270984110515773306643211150111379599332867331476905258877712070223126025\
            4978705174476571541504",
        ),
    );
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-9.989614378172733e139),
        standard_deviation: NiceFloat(9.989614377590213e142),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    striped_random_nonzero_rationals_helper(
        16,
        1,
        32,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    // mean bits = 64
    let values = &[
        "-70988433612846563009577969778432/37778931862957161709535",
        "-18874367/34359740415",
        "128/1048575",
        "9215/576461851815051263",
        "-67043359/4194303",
        "557501356260338958336/32794196171264010809",
        "68719476735/36028797018963968",
        "2596148429269774428927230031691776",
        "298270729465465682551124201231/2275625072216288986599183",
        "2048/28735",
        "7818749353074/7",
        "-288230376185266175/9671406556917033397649408",
        "4611686147284795328/2361183241443412541439",
        "-1208925819333154214772480/12068402384318877481825725909459066626185514174187044863",
        "-292842132531367112448",
        "-27670116179283804159/5708990770823839524233181656729843468399049728",
        "-10384554102988398124892195902719999/16809983",
        "-1162146002543608594431/256",
        "-784637716922080678562056411478885332466638530662142836736/8191",
        "-35184372080639/11418025097790482669312781854182320779382783943",
    ];
    let common_values = &[
        ("-1", 771),
        ("1", 746),
        ("1/2", 400),
        ("-2", 393),
        ("2", 392),
        ("-1/4", 376),
        ("4", 374),
        ("1/4", 374),
        ("-1/2", 371),
        ("1/8", 364),
    ];
    let sample_median = (
        "-144115205255725055/979746761720636838570985193498093558267704500484488575872951292219126\
        535205986877548162797451588896054171174290755185947242500542102825307421372913188911655327\
        9240115862075993700091884987318332951625727",
        Some(
            "-64/870216902382548101082732198927734427236298632942811591373701982167193271662795024\
            74847721902845790124837140768618290655265839460108220779308034391550625260769672349354\
            97445986979146629119",
        ),
    );
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-3.8438486163830216e252),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    striped_random_nonzero_rationals_helper(
        32,
        1,
        64,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
}

#[test]
#[should_panic]
fn striped_random_nonzero_rationals_fail_1() {
    striped_random_nonzero_rationals(EXAMPLE_SEED, 1, 0, 4, 1);
}

#[test]
#[should_panic]
fn striped_random_nonzero_rationals_fail_2() {
    striped_random_nonzero_rationals(EXAMPLE_SEED, 2, 3, 4, 1);
}

#[test]
#[should_panic]
fn striped_random_nonzero_rationals_fail_3() {
    striped_random_nonzero_rationals(EXAMPLE_SEED, 4, 1, 1, 0);
}

#[test]
#[should_panic]
fn striped_random_nonzero_rationals_fail_4() {
    striped_random_nonzero_rationals(EXAMPLE_SEED, 4, 1, 2, 3);
}
