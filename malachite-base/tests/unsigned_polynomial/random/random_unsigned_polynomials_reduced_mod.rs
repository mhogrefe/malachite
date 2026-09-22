// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::unsigned_polynomial::random::*;

fn random_unsigned_polynomials_reduced_mod_helper(
    m: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        random_unsigned_polynomials_reduced_mod::<u64>(
            EXAMPLE_SEED,
            m,
            mean_length_numerator,
            mean_length_denominator
        )
        .take(20)
        .map(|p| p.to_string())
        .collect_vec(),
        expected_values
    );
}

#[test]
fn test_random_unsigned_polynomials_reduced_mod() {
    // modulo 3
    random_unsigned_polynomials_reduced_mod_helper(
        3,
        2,
        1,
        &[
            "x^5+2*x^4+x^3+x^2+x+1",
            "2",
            "x^7+2*x^5+2*x^3",
            "1",
            "x^13+x^12+2*x^11+x^10+2*x^8+2*x^7+x^5+2*x^4+2*x^3+2*x^2+2",
            "0",
            "2*x^4+2*x^3+2*x^2+2",
            "2*x^3+x^2+2*x+1",
            "1",
            "0",
            "x^5+2*x^3+2*x^2+1",
            "x+2",
            "0",
            "0",
            "1",
            "2*x^2+2",
            "0",
            "1",
            "0",
            "2",
        ],
    );
    // modulo 10
    random_unsigned_polynomials_reduced_mod_helper(
        10,
        2,
        1,
        &[
            "3*x^5+8*x^4+8*x^3+5*x+5",
            "7",
            "9*x^7+x^6+9*x^5+2*x^4+6*x^3+8*x^2+6*x+8",
            "7",
            "9*x^13+5*x^12+9*x^11+7*x^10+8*x^9+9*x^7+6*x^6+5*x^5+x^4+6*x^3+2*x^2+2",
            "0",
            "3*x^4+5*x^3+4*x^2+6*x+1",
            "5*x^3+8*x^2+2*x+7",
            "2",
            "0",
            "3*x^5+2*x^4+3*x^3+2*x+9",
            "8*x+6",
            "0",
            "0",
            "6",
            "5*x^2+9*x+1",
            "0",
            "9",
            "0",
            "9",
        ],
    );
    // modulo the largest u64
    random_unsigned_polynomials_reduced_mod_helper(
        u64::MAX,
        2,
        1,
        &[
            "6282517168718784611*x^5+14328508029084493994*x^4+12663883950309859797*x^3+109383551\
            29926736414*x^2+16908237734149745446*x+16126131237969988437",
            "3854918945212287109",
            "3848495687584076942*x^7+4929296619887363376*x^6+18084098515246349065*x^5+5364743571\
            823285937*x^4+12452306358869796714*x^3+6855165495190718789*x^2+5274967849189775789*x\
            +16030916309388628338",
            "8242875068444962380",
            "33570146165392013*x^13+4042518734391281966*x^12+5799718956463847353*x^11+7321335884\
            194326556*x^10+7302066164643325347*x^9+13278249034234833257*x^8+15816946663310690555\
            *x^7+8896218915076694385*x^6+11159618536114539543*x^5+12011232285882986260*x^4+85348\
            87809013916308*x^3+5695140100314877469*x^2+4001260060885588185*x+8082601913180739774",
            "0",
            "1581093541351523808*x^4+12831608679968618560*x^3+2019075391003709918*x^2+1250225901\
            3159897071*x+8732207380342292346",
            "17580674005203639831*x^3+14223787268378284260*x^2+8316369894096577775*x+29093220421\
            36193251",
            "985495283534891315",
            "0",
            "5262163828948177754*x^5+1085073003158599774*x^4+1372385597973877686*x^3+13301370948\
            712062037*x^2+17182188607829803481*x+1655151457071799127",
            "16395975300252768012*x+8724967809846298799",
            "0",
            "0",
            "2644000139732919053",
            "16243027292956976550*x^2+14672638928462894619*x+8979006534907634478",
            "0",
            "10614542942872277191",
            "0",
            "3884538521225379396",
        ],
    );
}

#[test]
#[should_panic]
fn random_unsigned_polynomials_reduced_mod_fail_1() {
    let _ = random_unsigned_polynomials_reduced_mod::<u64>(EXAMPLE_SEED, 0, 2, 1);
}

#[test]
#[should_panic]
fn random_unsigned_polynomials_reduced_mod_fail_2() {
    let _ = random_unsigned_polynomials_reduced_mod::<u64>(EXAMPLE_SEED, 1, 2, 1);
}
