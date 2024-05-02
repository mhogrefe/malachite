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
use malachite_q::random::random_rationals;
use malachite_q::test_util::random::random_rationals_helper_helper;

fn random_rationals_helper(
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_sample_median: (&str, Option<&str>),
    expected_sample_moment_stats: MomentStats,
) {
    random_rationals_helper_helper(
        random_rationals(EXAMPLE_SEED, mean_bits_numerator, mean_bits_denominator),
        expected_values,
        expected_common_values,
        expected_sample_median,
        expected_sample_moment_stats,
    );
}

#[test]
fn test_random_rationals() {
    // mean bits = 65/64
    let values = &[
        "0", "0", "0", "2", "0", "0", "1", "0", "0", "0", "0", "0", "0", "0", "-28", "-1", "-4",
        "-1", "-1", "0",
    ];
    let common_values = &[
        ("0", 496048),
        ("1", 123950),
        ("-1", 123690),
        ("3", 31184),
        ("-2", 31130),
        ("-3", 31058),
        ("2", 30772),
        ("-4", 7921),
        ("-7", 7828),
        ("-6", 7799),
    ];
    let sample_median = ("0", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.5817718650793547),
        standard_deviation: NiceFloat(857.0389155951154),
        skewness: NiceFloat(-315.8438705097454),
        excess_kurtosis: NiceFloat(254264.92899445235),
    };
    random_rationals_helper(
        65,
        64,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    // mean bits = 2
    let values = &[
        "-1", "-5/3", "0", "1", "0", "1/2", "356", "0", "0", "3/2", "3/5", "-14/3", "0", "-1/3",
        "-19/3", "-1/2", "0", "-1", "0", "-10",
    ];
    let common_values = &[
        ("0", 333130),
        ("1", 66630),
        ("-1", 66556),
        ("2", 22206),
        ("-2", 22066),
        ("-3", 20899),
        ("3", 20834),
        ("1/2", 16750),
        ("-1/2", 16630),
        ("1/3", 15616),
    ];
    let sample_median = ("0", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-3875.4043163058423),
        standard_deviation: NiceFloat(5451723.46138941),
        skewness: NiceFloat(-481.96150934322344),
        excess_kurtosis: NiceFloat(353958.9361419337),
    };
    random_rationals_helper(
        2,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    // mean bits = 32
    let values = &[
        "-7301/34",
        "-4183103/1234731190583",
        "54812347098686/6195807891591254727",
        "812739/17841539017",
        "-665/908",
        "677/1138982845180",
        "166/22491855393807861245619791028129",
        "270142/5",
        "52040856788711439301087669967/15975369961878544862054",
        "5718607/1953563256716085077",
        "8834633494449605/147372515680891813385292082245912643739605046366",
        "-14860658876333535410753934016237/38209564041",
        "256/1033317698721",
        "-1675/34808324932084086743491848009",
        "-49",
        "-42/5",
        "-87750175104578/19615",
        "-1/4767944",
        "-137819495256811446350/41779",
        "-2/187",
    ];
    let common_values = &[
        ("0", 30369),
        ("-1", 929),
        ("1", 871),
        ("1/2", 423),
        ("2", 410),
        ("-2", 408),
        ("-1/2", 399),
        ("3", 372),
        ("1/3", 368),
        ("-1/3", 337),
    ];
    let sample_median = ("0", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(2.0180317983547227e148),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_rationals_helper(
        32,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    // mean bits = 64
    let values = &[
        "-1428130618501/11392923974388402817057849552586132522617914732498530",
        "-3383508417938165445131453/56779550950694809089378809702538209946934076252940714133449",
        "602900875601911171470306076355/119191771",
        "3/14013585568406836752167657664673",
        "-760776403/6462405519227986816335721703034929571679921",
        "3453088342103851715673829426753969982/25626510185",
        "1747398675/3172739",
        "8948691991346583905040602549520967352911/18",
        "16038312634753050980603803559756/9438855467532928850187287",
        "155434788890251/4034446723",
        "950902359766673/235910534939055966292926793",
        "-294004238713694270841854/1596165279",
        "1030393/85299778977201964065475016444620",
        "-124218250251176079819064/503926103984580328155607497147",
        "-277206127786809155854294/47228889692473",
        "-3673/301956358739051815786302694193",
        "-166239031838/39",
        "-3309620973011864735684788/31306944615",
        "-138546001637/6539404996772746726586649886838863596921111",
        "-417/14077532426874196091229260728580",
    ];
    let common_values = &[
        ("0", 15382),
        ("-1", 217),
        ("1", 216),
        ("-2", 126),
        ("1/2", 121),
        ("2", 119),
        ("-1/3", 117),
        ("-1/2", 111),
        ("-3", 100),
        ("3", 98),
    ];
    let sample_median = ("0", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-4.459352389355579e272),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_rationals_helper(
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
fn random_rationals_fail_1() {
    random_rationals(EXAMPLE_SEED, 1, 0);
}

#[test]
#[should_panic]
fn random_rationals_fail_2() {
    random_rationals(EXAMPLE_SEED, 2, 3);
}
