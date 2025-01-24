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
use malachite_q::random::random_non_negative_rationals;
use malachite_q::test_util::random::random_rationals_helper_helper;

fn random_non_negative_rationals_helper(
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_sample_median: (&str, Option<&str>),
    expected_sample_moment_stats: MomentStats,
) {
    random_rationals_helper_helper(
        random_non_negative_rationals(EXAMPLE_SEED, mean_bits_numerator, mean_bits_denominator),
        expected_values,
        expected_common_values,
        expected_sample_median,
        expected_sample_moment_stats,
    );
}

#[test]
fn test_random_non_negative_rationals() {
    // mean bits = 65/64
    let values = &[
        "0", "0", "0", "2", "0", "0", "1", "0", "0", "0", "0", "0", "0", "0", "28", "1", "4", "1",
        "1", "0",
    ];
    let common_values = &[
        ("0", 496048),
        ("1", 247640),
        ("3", 62242),
        ("2", 61902),
        ("4", 15714),
        ("7", 15554),
        ("6", 15541),
        ("5", 15493),
        ("8", 4052),
        ("11", 4013),
    ];
    let sample_median = ("2/3", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(7.803726415872939),
        standard_deviation: NiceFloat(857.0035840628179),
        skewness: NiceFloat(473.2359523744428),
        excess_kurtosis: NiceFloat(254290.48263912715),
    };
    random_non_negative_rationals_helper(
        65,
        64,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    // mean bits = 2
    let values = &[
        "1", "5/3", "0", "1", "0", "1/2", "356", "0", "0", "3/2", "3/5", "14/3", "0", "1/3",
        "19/3", "1/2", "0", "1", "0", "10",
    ];
    let common_values = &[
        ("0", 333130),
        ("1", 133186),
        ("2", 44272),
        ("3", 41733),
        ("1/2", 33380),
        ("1/3", 31211),
        ("4", 14811),
        ("5", 14622),
        ("6", 13959),
        ("7", 13895),
    ];
    let sample_median = ("2/3", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(13891.174382080539),
        standard_deviation: NiceFloat(5451707.141191459),
        skewness: NiceFloat(576.0667309545814),
        excess_kurtosis: NiceFloat(353958.67367137416),
    };
    random_non_negative_rationals_helper(
        2,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    // mean bits = 32
    let values = &[
        "7301/34",
        "4183103/1234731190583",
        "54812347098686/6195807891591254727",
        "812739/17841539017",
        "665/908",
        "677/1138982845180",
        "166/22491855393807861245619791028129",
        "270142/5",
        "52040856788711439301087669967/15975369961878544862054",
        "5718607/1953563256716085077",
        "8834633494449605/147372515680891813385292082245912643739605046366",
        "14860658876333535410753934016237/38209564041",
        "256/1033317698721",
        "1675/34808324932084086743491848009",
        "49",
        "42/5",
        "87750175104578/19615",
        "1/4767944",
        "137819495256811446350/41779",
        "2/187",
    ];
    let common_values = &[
        ("0", 30369),
        ("1", 1800),
        ("1/2", 822),
        ("2", 818),
        ("1/3", 705),
        ("3", 699),
        ("5", 399),
        ("1/4", 390),
        ("4", 382),
        ("1/5", 380),
    ];
    let sample_median = ("84579/122161", Some("1013/1463"));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(2.0180317983547227e148),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_non_negative_rationals_helper(
        32,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    // mean bits = 64
    let values = &[
        "1428130618501/11392923974388402817057849552586132522617914732498530",
        "3383508417938165445131453/56779550950694809089378809702538209946934076252940714133449",
        "602900875601911171470306076355/119191771",
        "3/14013585568406836752167657664673",
        "760776403/6462405519227986816335721703034929571679921",
        "3453088342103851715673829426753969982/25626510185",
        "1747398675/3172739",
        "8948691991346583905040602549520967352911/18",
        "16038312634753050980603803559756/9438855467532928850187287",
        "155434788890251/4034446723",
        "950902359766673/235910534939055966292926793",
        "294004238713694270841854/1596165279",
        "1030393/85299778977201964065475016444620",
        "124218250251176079819064/503926103984580328155607497147",
        "277206127786809155854294/47228889692473",
        "3673/301956358739051815786302694193",
        "166239031838/39",
        "3309620973011864735684788/31306944615",
        "138546001637/6539404996772746726586649886838863596921111",
        "417/14077532426874196091229260728580",
    ];
    let common_values = &[
        ("0", 15382),
        ("1", 433),
        ("2", 245),
        ("1/2", 232),
        ("1/3", 213),
        ("3", 198),
        ("4", 118),
        ("5", 114),
        ("2/3", 114),
        ("1/5", 111),
    ];
    let sample_median = ("25421/36471", Some("486/697"));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(4.459352389355579e272),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_non_negative_rationals_helper(
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
fn random_non_negative_rationals_fail_1() {
    random_non_negative_rationals(EXAMPLE_SEED, 1, 0);
}

#[test]
#[should_panic]
fn random_non_negative_rationals_fail_2() {
    random_non_negative_rationals(EXAMPLE_SEED, 2, 3);
}
