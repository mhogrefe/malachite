// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::gaussian_integer::random::random_gaussian_integers;

fn random_gaussian_integers_helper(
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        random_gaussian_integers(EXAMPLE_SEED, mean_bits_numerator, mean_bits_denominator)
            .take(20)
            .map(|x| x.to_string())
            .collect_vec(),
        expected_values
    );
}

#[test]
fn test_random_gaussian_integers() {
    // mean bits = 65/64
    random_gaussian_integers_helper(
        65,
        64,
        &[
            "2i", "-i", "-6i", "98-2i", "i", "1", "3-15i", "-1-18i", "-67", "1+i", "0", "i", "-6",
            "-5-i", "1-2i", "5i", "-1+7i", "1+4i", "1-i", "1-7i",
        ],
    );
    // mean bits = 2
    random_gaussian_integers_helper(
        2,
        1,
        &[
            "i", "18-24i", "-18-6i", "2+4i", "-3", "-1-i", "-35-9i", "-8-6i", "0", "6+i", "-13+i",
            "-14+i", "61", "-10+7i", "-15+4i", "-3i", "15-7i", "12-i", "4-3i", "6i",
        ],
    );
    // mean bits = 32
    random_gaussian_integers_helper(
        32,
        1,
        &[
            "69403499476962893258904+89270i",
            "-1848070042786+62i",
            "-696-64671510460i",
            "-79",
            "7330+70819i",
            "-424643+215441i",
            "-84146163512-11858i",
            "1518-7212822200i",
            "-909+23i",
            "-46-60054i",
            "-1948916062731748141639604402281199-93i",
            "42901255+878950536358356i",
            "991-9i",
            "-486797940-2300029688331914572973463i",
            "705728868192395944288+3i",
            "4682236801174748365421411862611191777456+6i",
            "-267428894934628+17582240409145470i",
            "14045495935318169878445637+870050206i",
            "2714-554364381654822i",
            "81670732763+10326940i",
        ],
    );
    // mean bits = 64
    random_gaussian_integers_helper(
        64,
        1,
        &[
            "204354108892664954266560767940941860034994328+15542i",
            "-323516+5282i",
            "-248570628312176883893327-400812728i",
            "-63523217+5606382754i",
            "25408382788335305673841323624499957642146385720-15024295498724618356672330435i",
            "33157733495351097449766897571769262785295460456592996025656609489115364170390153697558\
            40712936487655650300919339856269+70153184455655i",
            "-5826316-2179070834703641056854463566957970466590674233219693760530182904389383i",
            "-1-8647284i",
            "18608+43088412843029635753589496830104451113312i",
            "-114916707179919722397-3946823889925i",
            "-4799249989-88356322562174711170739098356352612i",
            "28454-i",
            "-4+112788265626i",
            "-497276407+11i",
            "93324750556576901370330009555772605539111-14322723569422882090528539954636308243980395\
            487335966329356020413192i",
            "4+327348906037314254109657813958311812903197063826171634241465278646167051319i",
            "130408+1223552652497638366350316543747204668309140974055223336140842279402989540052826\
            45i",
            "319066633-146044354552115025764i",
            "-75705+21384i",
            "4479800322160182569+89i",
        ],
    );
}

#[test]
#[should_panic]
fn random_gaussian_integers_fail_1() {
    let _ = random_gaussian_integers(EXAMPLE_SEED, 1, 0);
}

#[test]
#[should_panic]
fn random_gaussian_integers_fail_2() {
    let _ = random_gaussian_integers(EXAMPLE_SEED, u64::MAX, 1);
}
