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

fn striped_random_natural_polynomials_reduced_mod_power_of_2_helper(
    pow: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        striped_random_natural_polynomials_reduced_mod_power_of_2(
            EXAMPLE_SEED,
            pow,
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
fn test_striped_random_natural_polynomials_reduced_mod_power_of_2() {
    // modulo 256, mean stripe = 8
    striped_random_natural_polynomials_reduced_mod_power_of_2_helper(
        8,
        8,
        1,
        2,
        1,
        &[
            "31*x^5+224*x^4+248*x^3+31*x^2+59*x+255",
            "7",
            "246*x^7+14*x^4+255*x^3+255*x^2+128*x+192",
            "143",
            "62*x^13+249*x^12+254*x^11+247*x^10+252*x^9+63*x^8+27*x^7+225*x^6+4*x^5+192*x^4+255*\
            x^3+128*x^2",
            "0",
            "188*x^4+127*x^3+60*x^2+224*x+249",
            "255*x^3+254*x^2+16",
            "231",
            "0",
            "224*x^5+x^4+2*x+31",
            "13*x+248",
            "0",
            "0",
            "128",
            "63*x^2+59",
            "0",
            "255",
            "0",
            "16",
        ],
    );
    // modulo 2^16, mean stripe = 16
    striped_random_natural_polynomials_reduced_mod_power_of_2_helper(
        16,
        16,
        1,
        2,
        1,
        &[
            "63488*x^5+x^2+999*x+127",
            "12",
            "61443*x^7+65024*x^6+65472*x^5+14463*x^3+57471*x^2+53246*x",
            "53247",
            "96*x^13+65535*x^12+7680*x^11+59*x^10+127*x^8+65535*x^7+15*x^6+61440*x^4+2047*x^3+64\
            *x",
            "0",
            "65024*x^4+15*x^3+12*x^2+28675",
            "15*x^3+61440*x^2+65280",
            "252",
            "0",
            "511*x^5+511*x^4+8*x^3+64512*x^2+64512*x+64543",
            "63*x",
            "0",
            "0",
            "65024",
            "65504*x^2+255",
            "0",
            "4095",
            "0",
            "1",
        ],
    );
    // modulo 2^32, mean stripe = 16, mean length = 3/1
    striped_random_natural_polynomials_reduced_mod_power_of_2_helper(
        32,
        16,
        1,
        3,
        1,
        &[
            "4227858432*x^7+32768*x^6+524286*x^5+4294966392*x^4+33031680*x^3+536870911*x^2+42949\
            67295*x+2114715775",
            "67043334",
            "789504*x^10+16776704*x^9+32509951*x^8+4294966784*x^7+4292870192*x^6+4294737888*x^5+\
            122880*x^4+4232052736*x^3+4026531840*x^2+4227858447*x+524287",
            "4294901823",
            "4294451199*x^17+2164228606*x^16+2148004863*x^15+939524128*x^14+4294967288*x^13+2281\
            700864*x^12+4294967295*x^11+4041212144*x^10+6144*x^9+4294443263*x^8+2147483647*x^7+3\
            *x^5+134217726*x^4+4294967267*x^3+4294967295*x^2+1048575*x+4290772994",
            "0",
            "2146959360*x^6+4227858432*x^5+131079*x^4+1073676288*x^3+3758161919*x^2+4294967168*x\
            +4194288",
            "536608768*x^5+1179647*x^4+524287*x^3+134217727*x^2+4026564480*x+15794175",
            "1048575",
            "4286578688",
            "12288*x^7+3221749632*x^6+1835008*x^5+4294934528*x^3+2143811584*x^2+474880*x+4290772\
            991",
            "4292870144*x+4278312975",
            "0",
            "0",
            "4164943871",
            "4294446991*x^3+4294955004*x^2+2097151*x+4294967295",
            "1020*x+4294967295",
            "1073741808",
            "0",
            "3221258215*x+32513",
        ],
    );
}

#[test]
#[should_panic]
fn striped_random_natural_polynomials_reduced_mod_power_of_2_fail_1() {
    let _ = striped_random_natural_polynomials_reduced_mod_power_of_2(EXAMPLE_SEED, 0, 8, 1, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_natural_polynomials_reduced_mod_power_of_2_fail_2() {
    let _ = striped_random_natural_polynomials_reduced_mod_power_of_2(EXAMPLE_SEED, 8, 1, 2, 2, 1);
}
