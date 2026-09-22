// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::natural_polynomial::random::*;

fn random_natural_polynomials_reduced_mod_power_of_2_helper(
    pow: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        random_natural_polynomials_reduced_mod_power_of_2(
            EXAMPLE_SEED,
            pow,
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
fn test_random_natural_polynomials_reduced_mod_power_of_2() {
    // modulo 2, the polynomials over GF(2)
    random_natural_polynomials_reduced_mod_power_of_2_helper(
        1,
        2,
        1,
        &[
            "x^5+x^3+1",
            "1",
            "x^7+x^5+x^4+x^2+x",
            "1",
            "x^13+x^11+x^9+x^8+x^7+x^6+x^5+x^2+x",
            "0",
            "x^4+x",
            "x^3+x+1",
            "1",
            "0",
            "x^5+x^2+x+1",
            "x+1",
            "0",
            "0",
            "1",
            "x^2+x",
            "0",
            "1",
            "0",
            "1",
        ],
    );
    // modulo 16
    random_natural_polynomials_reduced_mod_power_of_2_helper(
        4,
        2,
        1,
        &[
            "2*x^5+10*x^4+5*x^3+14*x^2+6*x+5",
            "4",
            "13*x^7+9*x^5+x^4+10*x^3+5*x^2+13*x+2",
            "11",
            "12*x^13+14*x^12+9*x^11+12*x^10+3*x^9+9*x^8+11*x^7+x^6+7*x^5+4*x^4+4*x^3+13*x^2+9*x+\
            14",
            "0",
            "15*x^4+14*x^2+15*x+10",
            "6*x^3+4*x^2+15*x+3",
            "2",
            "0",
            "9*x^5+14*x^4+6*x^3+5*x^2+9*x+7",
            "11*x+15",
            "0",
            "0",
            "12",
            "5*x^2+11*x+14",
            "0",
            "6",
            "0",
            "3",
        ],
    );
    // modulo 256, mean length = 3/1
    random_natural_polynomials_reduced_mod_power_of_2_helper(
        8,
        3,
        1,
        &[
            "98*x^7+173*x^6+114*x^5+170*x^4+213*x^3+30*x^2+38*x+85",
            "132",
            "141*x^10+20*x^9+148*x^8+29*x^7+217*x^6+190*x^5+48*x^4+9*x^3+177*x^2+106*x+69",
            "75",
            "140*x^17+217*x^16+87*x^15+228*x^14+239*x^13+227*x^12+64*x^11+222*x^10+239*x^9+122*x\
            ^8+46*x^7+185*x^6+28*x^5+163*x^4+105*x^3+251*x^2+113*x+23",
            "0",
            "223*x^6+27*x^5+46*x^4+175*x^3+94*x^2+182*x+85",
            "22*x^5+203*x^4+31*x^3+16*x^2+115*x+227",
            "50",
            "89",
            "11*x^7+184*x^6+183*x^5+73*x^4+61*x^3+167*x^2+133*x+76",
            "12*x+220",
            "0",
            "0",
            "165",
            "198*x^3+145*x^2+177*x+225",
            "67*x+58",
            "48",
            "0",
            "47*x+63",
        ],
    );
}

#[test]
#[should_panic]
fn random_natural_polynomials_reduced_mod_power_of_2_fail_1() {
    let _ = random_natural_polynomials_reduced_mod_power_of_2(EXAMPLE_SEED, 0, 2, 1);
}

#[test]
#[should_panic]
fn random_natural_polynomials_reduced_mod_power_of_2_fail_2() {
    let _ = random_natural_polynomials_reduced_mod_power_of_2(EXAMPLE_SEED, 4, 1, 0);
}
