// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_q::gaussian_rational::random::random_real_gaussian_rationals;

fn random_real_gaussian_rationals_helper(
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        random_real_gaussian_rationals(EXAMPLE_SEED, mean_bits_numerator, mean_bits_denominator)
            .take(20)
            .map(|x| x.to_string())
            .collect_vec(),
        expected_values
    );
}

#[test]
fn test_random_real_gaussian_rationals() {
    // mean bits = 65/64
    random_real_gaussian_rationals_helper(
        65,
        64,
        &[
            "0", "0", "0", "2", "0", "0", "1", "0", "0", "0", "0", "0", "0", "0", "-28", "-1",
            "-4", "-1", "-1", "0",
        ],
    );
    // mean bits = 2
    random_real_gaussian_rationals_helper(
        2,
        1,
        &[
            "-1", "-5/3", "0", "1", "0", "1/2", "356", "0", "0", "3/2", "3/5", "-14/3", "0",
            "-1/3", "-19/3", "-1/2", "0", "-1", "0", "-10",
        ],
    );
    // mean bits = 32
    random_real_gaussian_rationals_helper(
        32,
        1,
        &[
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
        ],
    );
    // mean bits = 64
    random_real_gaussian_rationals_helper(
        64,
        1,
        &[
            "-1428130618501/11392923974388402817057849552586132522617914732498530",
            "-3383508417938165445131453/5677955095069480908937880970253820994693407625294071413344\
            9",
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
        ],
    );
}

#[test]
fn random_real_gaussian_rationals_axis() {
    assert!(
        random_real_gaussian_rationals(EXAMPLE_SEED, 32, 1)
            .take(100)
            .all(|x| x.imaginary == 0)
    );
}

#[test]
#[should_panic]
fn random_real_gaussian_rationals_fail_1() {
    let _ = random_real_gaussian_rationals(EXAMPLE_SEED, 1, 0);
}

#[test]
#[should_panic]
fn random_real_gaussian_rationals_fail_2() {
    let _ = random_real_gaussian_rationals(EXAMPLE_SEED, u64::MAX, 1);
}
