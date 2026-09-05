// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_q::gaussian_rational::random::random_imaginary_gaussian_rationals;

fn random_imaginary_gaussian_rationals_helper(
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        random_imaginary_gaussian_rationals(
            EXAMPLE_SEED,
            mean_bits_numerator,
            mean_bits_denominator
        )
        .take(20)
        .map(|x| x.to_string())
        .collect_vec(),
        expected_values
    );
}

#[test]
fn test_random_imaginary_gaussian_rationals() {
    // mean bits = 65/64
    random_imaginary_gaussian_rationals_helper(
        65,
        64,
        &[
            "0", "0", "0", "2i", "0", "0", "i", "0", "0", "0", "0", "0", "0", "0", "-28i", "-i",
            "-4i", "-i", "-i", "0",
        ],
    );
    // mean bits = 2
    random_imaginary_gaussian_rationals_helper(
        2,
        1,
        &[
            "-i", "-5i/3", "0", "i", "0", "i/2", "356i", "0", "0", "3i/2", "3i/5", "-14i/3", "0",
            "-i/3", "-19i/3", "-i/2", "0", "-i", "0", "-10i",
        ],
    );
    // mean bits = 32
    random_imaginary_gaussian_rationals_helper(
        32,
        1,
        &[
            "-7301i/34",
            "-4183103i/1234731190583",
            "54812347098686i/6195807891591254727",
            "812739i/17841539017",
            "-665i/908",
            "677i/1138982845180",
            "166i/22491855393807861245619791028129",
            "270142i/5",
            "52040856788711439301087669967i/15975369961878544862054",
            "5718607i/1953563256716085077",
            "8834633494449605i/147372515680891813385292082245912643739605046366",
            "-14860658876333535410753934016237i/38209564041",
            "256i/1033317698721",
            "-1675i/34808324932084086743491848009",
            "-49i",
            "-42i/5",
            "-87750175104578i/19615",
            "-i/4767944",
            "-137819495256811446350i/41779",
            "-2i/187",
        ],
    );
    // mean bits = 64
    random_imaginary_gaussian_rationals_helper(
        64,
        1,
        &[
            "-1428130618501i/11392923974388402817057849552586132522617914732498530",
            "-3383508417938165445131453i/5677955095069480908937880970253820994693407625294071413344\
            9",
            "602900875601911171470306076355i/119191771",
            "3i/14013585568406836752167657664673",
            "-760776403i/6462405519227986816335721703034929571679921",
            "3453088342103851715673829426753969982i/25626510185",
            "1747398675i/3172739",
            "8948691991346583905040602549520967352911i/18",
            "16038312634753050980603803559756i/9438855467532928850187287",
            "155434788890251i/4034446723",
            "950902359766673i/235910534939055966292926793",
            "-294004238713694270841854i/1596165279",
            "1030393i/85299778977201964065475016444620",
            "-124218250251176079819064i/503926103984580328155607497147",
            "-277206127786809155854294i/47228889692473",
            "-3673i/301956358739051815786302694193",
            "-166239031838i/39",
            "-3309620973011864735684788i/31306944615",
            "-138546001637i/6539404996772746726586649886838863596921111",
            "-417i/14077532426874196091229260728580",
        ],
    );
}

#[test]
fn random_imaginary_gaussian_rationals_axis() {
    assert!(
        random_imaginary_gaussian_rationals(EXAMPLE_SEED, 32, 1)
            .take(100)
            .all(|x| x.real == 0u32)
    );
}

#[test]
#[should_panic]
fn random_imaginary_gaussian_rationals_fail_1() {
    let _ = random_imaginary_gaussian_rationals(EXAMPLE_SEED, 1, 0);
}

#[test]
#[should_panic]
fn random_imaginary_gaussian_rationals_fail_2() {
    let _ = random_imaginary_gaussian_rationals(EXAMPLE_SEED, u64::MAX, 1);
}
