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
use malachite_nz::integer::random::random_negative_integers;
use malachite_nz::test_util::integer::random::random_integers_helper_helper;

fn random_negative_integers_helper(
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_sample_median: (&str, Option<&str>),
    expected_sample_moment_stats: MomentStats,
) {
    random_integers_helper_helper(
        random_negative_integers(EXAMPLE_SEED, mean_bits_numerator, mean_bits_denominator),
        expected_values,
        expected_common_values,
        expected_sample_median,
        expected_sample_moment_stats,
    );
}

#[test]
fn test_random_negative_integers() {
    // mean bits = 65/64
    let values = &["-1"; 20];
    let common_values = &[
        ("-1", 984681),
        ("-3", 7622),
        ("-2", 7455),
        ("-5", 73),
        ("-6", 66),
        ("-7", 54),
        ("-4", 44),
        ("-8", 2),
        ("-10", 2),
        ("-14", 1),
    ];
    let sample_median = ("-1", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-1.023822000000005),
        standard_deviation: NiceFloat(0.20727410662829246),
        skewness: NiceFloat(-10.72004433095801),
        excess_kurtosis: NiceFloat(159.60627558337237),
    };
    random_negative_integers_helper(
        65,
        64,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    // mean bits = 2
    let values = &[
        "-1", "-24", "-1", "-30", "-6", "-12", "-2", "-1", "-1", "-1", "-1", "-1", "-2", "-2",
        "-1", "-1", "-3", "-3", "-1", "-1",
    ];
    let common_values = &[
        ("-1", 500248),
        ("-3", 124972),
        ("-2", 124519),
        ("-7", 31554),
        ("-5", 31346),
        ("-6", 31198),
        ("-4", 31043),
        ("-12", 8033),
        ("-11", 7959),
        ("-10", 7935),
    ];
    let sample_median = ("-1", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-15.68562000000007),
        standard_deviation: NiceFloat(2088.3045530403606),
        skewness: NiceFloat(-877.2889258611025),
        excess_kurtosis: NiceFloat(832799.3689336807),
    };
    random_negative_integers_helper(
        2,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    // mean bits = 32
    let values = &[
        "-22",
        "-4",
        "-178",
        "-55845661150",
        "-93254818",
        "-7577967529619388",
        "-8",
        "-11316951483471",
        "-11",
        "-1005760138411689342464923704482",
        "-948931",
        "-42716754",
        "-81013760999253680590984897748479904878392",
        "-23",
        "-5",
        "-488225822927510",
        "-1558028859598",
        "-29",
        "-200127331174844881647",
        "-4058622214797175252",
    ];
    let common_values = &[
        ("-1", 31094),
        ("-2", 15260),
        ("-3", 15185),
        ("-4", 7586),
        ("-5", 7376),
        ("-7", 7346),
        ("-6", 7258),
        ("-10", 3631),
        ("-14", 3607),
        ("-11", 3605),
    ];
    let sample_median = ("-3799067", Some("-3799061"));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-2.312362311300544e130),
        standard_deviation: NiceFloat(2.3122865276852406e133),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_negative_integers_helper(
        32,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    // mean bits = 64
    let values = &[
        "-1030304779202860497815440824491190",
        "-886085025458",
        "-207326",
        "-83590267817164982586207812646050",
        "-142592182196136038718074156629812683467448",
        "-486577913627642327503939268330036386",
        "-5557920650918595",
        "-82",
        "-3896",
        "-259694111319673990840",
        "-38511521798151392412656616617957654586378660839",
        "-637134",
        "-2330568192653124764618470467652346596061",
        "-2516",
        "-512663303",
        "-39317568409",
        "-18536901993439",
        "-4959577657266999117207",
        "-628",
        "-42485719907732979",
    ];
    let common_values = &[
        ("-1", 15720),
        ("-2", 7718),
        ("-3", 7584),
        ("-6", 3790),
        ("-4", 3739),
        ("-7", 3704),
        ("-5", 3673),
        ("-9", 1918),
        ("-11", 1916),
        ("-10", 1904),
    ];
    let sample_median = ("-18438360920148", Some("-18436851140261"));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-5.519478531998525e283),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_negative_integers_helper(
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
fn random_negative_integers_fail_1() {
    random_negative_integers(EXAMPLE_SEED, 1, 0);
}

#[test]
#[should_panic]
fn random_negative_integers_fail_2() {
    random_negative_integers(EXAMPLE_SEED, 2, 3);
}
