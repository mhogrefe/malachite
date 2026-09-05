// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_q::gaussian_rational::random::striped_random_real_gaussian_rationals;

fn striped_random_real_gaussian_rationals_helper(
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        striped_random_real_gaussian_rationals(
            EXAMPLE_SEED,
            mean_stripe_numerator,
            mean_stripe_denominator,
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
fn test_striped_random_real_gaussian_rationals() {
    // mean stripe = 2, mean bits = 2
    striped_random_real_gaussian_rationals_helper(
        2,
        1,
        2,
        1,
        &[
            "-2/3", "-4/3", "0", "1", "0", "1/2", "504", "0", "0", "1", "5/12", "-5", "0", "-1/3",
            "-29/3", "-1/3", "0", "-1", "0", "-15",
        ],
    );
    // mean stripe = 4, mean bits = 32
    striped_random_real_gaussian_rationals_helper(
        4,
        1,
        32,
        1,
        &[
            "-15872/95",
            "-14682056/4342731251687",
            "16372281702638/1697861442501472511",
            "484863/16246636744",
            "-2008/1547",
            "286/610153791503",
            "13/3372837971958169691128569110528",
            "520127/7",
            "78060415730245188559963618547/9738228119607812812920",
            "8380927/1153053729185595520",
            "6757598196252915/181084673206721801817020549976667446018805070944",
            "-30347021168215199251209929556543/137389662719",
            "505/1098166317119",
            "-2032/39594738999445089176631377679",
            "-48",
            "-93/11",
            "-7688386118535/1264",
            "-1/8372751",
            "-572068616900777659964/196531",
            "-3/221",
        ],
    );
    // mean stripe = 16, mean bits = 32
    striped_random_real_gaussian_rationals_helper(
        16,
        1,
        32,
        1,
        &[
            "-8192/127",
            "-16776704/4396972769407",
            "8796093005951/648518346332962816",
            "87381/2863267840",
            "-1024/2043",
            "51/58408828928",
            "85/13521606402434254795714066382848",
            "270335/7",
            "59421159664630116152453890047/9444741445172838006656",
            "6291455/1154891846623166464",
            "4503599631564799/114177029184456441820717001177155938271778439152",
            "-40247906632508999881205124923399/137438953471",
            "73/154619122249",
            "-1024/39611663922002864317824761855",
            "-32",
            "-127/9",
            "-2199023247360/287",
            "-1/8257539",
            "-590156181179127562240/131199",
            "-1/85",
        ],
    );
    // mean stripe = 32, mean bits = 64
    striped_random_real_gaussian_rationals_helper(
        32,
        1,
        64,
        1,
        &[
            "-1464583847936/7981747608676504359847391117664870922673555168908629",
            "-2422574005712127994617856/10004130909477531191275152378621376563629406242447645946675\
            1",
            "9671406556916483641901054/2047",
            "1/10141204801678261259383949230080",
            "-1/10384593719487506031596923529461760",
            "166153499473114484112975882535075839/1073741824",
            "1073758207/2097152",
            "10889035740836205568492768571262465220607/31",
            "16225927683142697268042315648307/15474248646392859802468352",
            "211174952009727/4294836224",
            "1125625028999183/309485009533116616750923776",
            "-160551237036734989468671/2146697215",
            "4325375/324527219843164634252394901798911",
            "-5666839779310716881032/42255019850195730860877091089",
            "-201487684640834221069648/46912675075413",
            "-1365/52818778157753880297518486869",
            "-17179869184/7",
            "-2420212822470693171986431/34359738367",
            "-274877382656/11150372599265311570767859136324172163055871",
            "-181/10141204802612896292451899146325",
        ],
    );
}

#[test]
fn striped_random_real_gaussian_rationals_axis() {
    assert!(
        striped_random_real_gaussian_rationals(EXAMPLE_SEED, 16, 1, 32, 1)
            .take(100)
            .all(|x| x.imaginary == 0u32)
    );
}

#[test]
#[should_panic]
fn striped_random_real_gaussian_rationals_fail_1() {
    let _ = striped_random_real_gaussian_rationals(EXAMPLE_SEED, 1, 2, 32, 1);
}

#[test]
#[should_panic]
fn striped_random_real_gaussian_rationals_fail_2() {
    let _ = striped_random_real_gaussian_rationals(EXAMPLE_SEED, 2, 1, 1, 0);
}
