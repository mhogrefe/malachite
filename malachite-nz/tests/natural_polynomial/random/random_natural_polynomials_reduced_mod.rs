// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::basic::traits::One;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::random::*;

fn random_natural_polynomials_reduced_mod_helper(
    m: &Natural,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        random_natural_polynomials_reduced_mod(
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
fn test_random_natural_polynomials_reduced_mod() {
    // modulo 3
    random_natural_polynomials_reduced_mod_helper(
        &Natural::from(3u32),
        2,
        1,
        &[
            "2*x^5+2*x^4+x^3+2*x^2+2*x+1",
            "1",
            "2*x^7+x^5+x^4+2*x^3+x^2+x+2",
            "2",
            "x^13+2*x^11+2*x^10+2*x^9+x^8+x^6+x^5+x^2+x+2",
            "0",
            "x^4+2*x^3+x^2+x",
            "2*x^3+2*x+2",
            "1",
            "0",
            "x^5+x^3+x^2+x",
            "2*x",
            "0",
            "0",
            "1",
            "2*x^2+x+1",
            "0",
            "2",
            "0",
            "1",
        ],
    );
    // modulo 10
    random_natural_polynomials_reduced_mod_helper(
        &Natural::from(10u32),
        2,
        1,
        &[
            "2*x^5+5*x^4+2*x^3+5*x^2+6*x+5",
            "4",
            "6*x^7+7*x^6+4*x^5+4*x^4+9*x^3+9*x+1",
            "2",
            "9*x^13+3*x^12+3*x^11+6*x^10+5*x^9+9*x^8+7*x^7+4*x^6+3*x^5+9*x^3+3*x^2+9*x+1",
            "0",
            "5*x^4+9*x^3+7*x^2+5*x",
            "6*x^3+x^2+8*x+7",
            "3",
            "0",
            "7*x^5+7*x^4+x^3+2*x^2+x+1",
            "4*x+6",
            "0",
            "0",
            "9",
            "x^2+7*x+3",
            "0",
            "7",
            "0",
            "1",
        ],
    );
    // modulo 1000, mean length = 3/1
    random_natural_polynomials_reduced_mod_helper(
        &Natural::from(1000u32),
        3,
        1,
        &[
            "98*x^7+429*x^6+370*x^5+170*x^4+469*x^3+542*x^2+806*x+853",
            "132",
            "141*x^10+788*x^9+660*x^8+541*x^7+217*x^6+190*x^5+304*x^4+777*x^3+689*x^2+874*x+325",
            "587",
            "652*x^17+85*x^16+473*x^15+855*x^14+228*x^13+239*x^12+227*x^11+64*x^10+478*x^9+890*x\
            ^8+302*x^7+953*x^6+28*x^5+419*x^4+361*x^3+251*x^2+369*x+23",
            "0",
            "479*x^6+995*x^5+539*x^4+814*x^3+175*x^2+94*x+950",
            "534*x^5+588*x^4+715*x^3+543*x^2+272*x+627",
            "306",
            "857",
            "779*x^7+732*x^6+952*x^5+695*x^4+329*x^3+829*x^2+679*x+645",
            "780*x+993",
            "0",
            "0",
            "421",
            "198*x^3+570*x^2+913*x+945",
            "579*x+63",
            "304",
            "0",
            "559*x+42",
        ],
    );
}

#[test]
#[should_panic]
fn random_natural_polynomials_reduced_mod_fail_1() {
    let _ = random_natural_polynomials_reduced_mod(EXAMPLE_SEED, &Natural::ONE, 2, 1);
}

#[test]
#[should_panic]
fn random_natural_polynomials_reduced_mod_fail_2() {
    let _ = random_natural_polynomials_reduced_mod(EXAMPLE_SEED, &Natural::from(3u32), 1, 0);
}
