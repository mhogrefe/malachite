// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_q::gaussian_rational::random::striped_random_imaginary_gaussian_rationals;

fn striped_random_imaginary_gaussian_rationals_helper(
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        striped_random_imaginary_gaussian_rationals(
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
fn test_striped_random_imaginary_gaussian_rationals() {
    // mean stripe = 2, mean bits = 2
    striped_random_imaginary_gaussian_rationals_helper(
        2,
        1,
        2,
        1,
        &[
            "-2i/3", "-4i/3", "0", "i", "0", "i/2", "504i", "0", "0", "i", "5i/12", "-5i", "0",
            "-i/3", "-29i/3", "-i/3", "0", "-i", "0", "-15i",
        ],
    );
    // mean stripe = 4, mean bits = 32
    striped_random_imaginary_gaussian_rationals_helper(
        4,
        1,
        32,
        1,
        &[
            "-15872i/95",
            "-14682056i/4342731251687",
            "16372281702638i/1697861442501472511",
            "484863i/16246636744",
            "-2008i/1547",
            "286i/610153791503",
            "13i/3372837971958169691128569110528",
            "520127i/7",
            "78060415730245188559963618547i/9738228119607812812920",
            "8380927i/1153053729185595520",
            "6757598196252915i/181084673206721801817020549976667446018805070944",
            "-30347021168215199251209929556543i/137389662719",
            "505i/1098166317119",
            "-2032i/39594738999445089176631377679",
            "-48i",
            "-93i/11",
            "-7688386118535i/1264",
            "-i/8372751",
            "-572068616900777659964i/196531",
            "-3i/221",
        ],
    );
    // mean stripe = 16, mean bits = 32
    striped_random_imaginary_gaussian_rationals_helper(
        16,
        1,
        32,
        1,
        &[
            "-8192i/127",
            "-16776704i/4396972769407",
            "8796093005951i/648518346332962816",
            "87381i/2863267840",
            "-1024i/2043",
            "51i/58408828928",
            "85i/13521606402434254795714066382848",
            "270335i/7",
            "59421159664630116152453890047i/9444741445172838006656",
            "6291455i/1154891846623166464",
            "4503599631564799i/114177029184456441820717001177155938271778439152",
            "-40247906632508999881205124923399i/137438953471",
            "73i/154619122249",
            "-1024i/39611663922002864317824761855",
            "-32i",
            "-127i/9",
            "-2199023247360i/287",
            "-i/8257539",
            "-590156181179127562240i/131199",
            "-i/85",
        ],
    );
    // mean stripe = 32, mean bits = 64
    striped_random_imaginary_gaussian_rationals_helper(
        32,
        1,
        64,
        1,
        &[
            "-1464583847936i/7981747608676504359847391117664870922673555168908629",
            "-2422574005712127994617856i/1000413090947753119127515237862137656362940624244764594667\
            51",
            "9671406556916483641901054i/2047",
            "i/10141204801678261259383949230080",
            "-i/10384593719487506031596923529461760",
            "166153499473114484112975882535075839i/1073741824",
            "1073758207i/2097152",
            "10889035740836205568492768571262465220607i/31",
            "16225927683142697268042315648307i/15474248646392859802468352",
            "211174952009727i/4294836224",
            "1125625028999183i/309485009533116616750923776",
            "-160551237036734989468671i/2146697215",
            "4325375i/324527219843164634252394901798911",
            "-5666839779310716881032i/42255019850195730860877091089",
            "-201487684640834221069648i/46912675075413",
            "-1365i/52818778157753880297518486869",
            "-17179869184i/7",
            "-2420212822470693171986431i/34359738367",
            "-274877382656i/11150372599265311570767859136324172163055871",
            "-181i/10141204802612896292451899146325",
        ],
    );
}

#[test]
fn striped_random_imaginary_gaussian_rationals_axis() {
    assert!(
        striped_random_imaginary_gaussian_rationals(EXAMPLE_SEED, 16, 1, 32, 1)
            .take(100)
            .all(|x| x.real == 0)
    );
}

#[test]
#[should_panic]
fn striped_random_imaginary_gaussian_rationals_fail_1() {
    let _ = striped_random_imaginary_gaussian_rationals(EXAMPLE_SEED, 1, 2, 32, 1);
}

#[test]
#[should_panic]
fn striped_random_imaginary_gaussian_rationals_fail_2() {
    let _ = striped_random_imaginary_gaussian_rationals(EXAMPLE_SEED, 2, 1, 1, 0);
}
