// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::traits::Zero;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::random::*;

fn striped_random_natural_polynomials_reduced_mod_helper(
    m: &Natural,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        striped_random_natural_polynomials_reduced_mod(
            EXAMPLE_SEED,
            m,
            mean_stripe_numerator,
            mean_stripe_denominator,
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
fn test_striped_random_natural_polynomials_reduced_mod() {
    // modulo 1000, mean stripe = 8
    striped_random_natural_polynomials_reduced_mod_helper(
        &Natural::from(1000u32),
        8,
        1,
        2,
        1,
        &[
            "x^5+3*x^4+x^3+62*x^2+952*x+999",
            "1",
            "x^7+511*x^3+7*x^2+999*x+7",
            "775",
            "992*x^13+959*x^12+3*x^11+481*x^10+512*x^9+636*x^8+992*x^7+33*x^6+799*x^5+639*x^4+54\
            2*x^3+479*x^2+992*x+127",
            "0",
            "55*x^4+992*x^3+256*x^2+7*x+31",
            "995*x^3+988*x^2+7*x+7",
            "255",
            "0",
            "497*x^5+999*x^4+961*x^3+993*x^2+992*x+3",
            "991*x+994",
            "0",
            "0",
            "993",
            "254*x^2+7",
            "0",
            "992",
            "0",
            "39",
        ],
    );
    // modulo 1000, mean stripe = 16
    striped_random_natural_polynomials_reduced_mod_helper(
        &Natural::from(1000u32),
        16,
        1,
        2,
        1,
        &[
            "x^5+504*x^3+96*x^2+512*x+999",
            "1",
            "x^7+x^6+7*x^5+999*x",
            "771",
            "512*x^13+8*x^12+992*x^10+127*x^9+999*x^7+993*x^6+992*x^5+999*x^4+224*x^3+7*x^2+519*\
            x+255",
            "0",
            "31*x^4+992*x^3+992",
            "15*x^3+992*x^2+x+768",
            "992",
            "0",
            "12*x^5+63*x^2+999*x+999",
            "24*x",
            "0",
            "0",
            "999",
            "x^2+511",
            "0",
            "511",
            "0",
            "768",
        ],
    );
    // modulo 2^32, mean stripe = 16, mean length = 3/1
    striped_random_natural_polynomials_reduced_mod_helper(
        &Natural::power_of_2(32),
        16,
        1,
        3,
        1,
        &[
            "63*x^7+65536*x^6+2147475456*x^5+507510783*x^4+6299520*x^3+4294967288*x^2+4294967295\
            *x+4261425278",
            "1610678208",
            "3158016*x^10+8388352*x^9+4293922688*x^8+8388607*x^7+201328639*x^6+134103039*x^5+491\
            520*x^4+575*x^3+15*x^2+4026531903*x+4294959104",
            "4227923967",
            "4294451199*x^17+2139225857*x^16+4291813377*x^15+67108892*x^14+536870911*x^13+838857\
            7*x^12+4294967295*x^11+251660047*x^10+1572864*x^9+4278198271*x^8+4294967294*x^7+3221\
            225472*x^5+2147483616*x^4+3355443199*x^3+4294967295*x^2+4294963200*x+1073742847",
            "0",
            "8190*x^6+63*x^5+3758112768*x^4+65532*x^3+4294901767*x^2+33554431*x+268434432",
            "16376*x^5+4294936576*x^4+4294959104*x^3+4294967264*x^2+33423375*x+4294905600",
            "4294963200",
            "511",
            "786432*x^7+33546243*x^6+14336*x^5+131071*x^3+2089982*x^2+16572416*x+4294966783",
            "2047*x+4027023615",
            "0",
            "0",
            "4294966303",
            "4059045887*x^3+1072955391*x^2+4294965248*x+4294967295",
            "1069547520*x+4294967295",
            "268435452",
            "0",
            "3892183043*x+2164129792",
        ],
    );
}

#[test]
#[should_panic]
fn striped_random_natural_polynomials_reduced_mod_fail_1() {
    let _ =
        striped_random_natural_polynomials_reduced_mod(EXAMPLE_SEED, &Natural::ZERO, 8, 1, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_natural_polynomials_reduced_mod_fail_2() {
    let _ = striped_random_natural_polynomials_reduced_mod(
        EXAMPLE_SEED,
        &Natural::from(3u32),
        1,
        2,
        2,
        1,
    );
}
