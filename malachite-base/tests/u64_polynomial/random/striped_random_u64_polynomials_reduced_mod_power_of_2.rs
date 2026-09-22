// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::u64_polynomial::random::*;

fn striped_random_u64_polynomials_reduced_mod_power_of_2_helper(
    pow: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        striped_random_u64_polynomials_reduced_mod_power_of_2(
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
fn test_striped_random_u64_polynomials_reduced_mod_power_of_2() {
    // modulo 256, mean stripe = 8
    striped_random_u64_polynomials_reduced_mod_power_of_2_helper(
        8,
        8,
        1,
        2,
        1,
        &[
            "248*x^5+7*x^4+31*x^3+248*x^2+220*x+255",
            "224",
            "111*x^7+112*x^4+255*x^3+255*x^2+x+3",
            "241",
            "124*x^13+159*x^12+127*x^11+239*x^10+63*x^9+252*x^8+216*x^7+135*x^6+32*x^5+3*x^4+255\
            *x^3+x^2",
            "0",
            "61*x^4+254*x^3+60*x^2+7*x+159",
            "255*x^3+127*x^2+8",
            "231",
            "0",
            "7*x^5+128*x^4+64*x+248",
            "176*x+31",
            "0",
            "0",
            "1",
            "252*x^2+220",
            "0",
            "255",
            "0",
            "8",
        ],
    );
    // modulo 2^32, mean stripe = 16
    striped_random_u64_polynomials_reduced_mod_power_of_2_helper(
        32,
        16,
        1,
        2,
        1,
        &[
            "63*x^5+507510783*x^4+6299520*x^3+4294967288*x^2+4294967295*x+4261425278",
            "1610678208",
            "3158016*x^7+491520*x^6+575*x^5+15*x^4+4026531903*x^3+4294959104*x^2+65536*x+2147475\
            456",
            "4227923967",
            "4294451199*x^13+4294967294*x^12+3221225472*x^10+2147483616*x^9+3355443199*x^8+42949\
            67295*x^7+4294963200*x^6+1073742847*x^5+8388352*x^4+4293922688*x^3+8388607*x^2+20132\
            8639*x+134103039",
            "0",
            "8190*x^4+4294967295*x^3+251660047*x^2+1572864*x+4278198271",
            "16376*x^3+67108892*x^2+536870911*x+8388577",
            "4294963200",
            "0",
            "511*x^5+4294901767*x^4+33554431*x^3+268434432*x^2+2139225857*x+4291813377",
            "786432*x+65532",
            "0",
            "0",
            "2047",
            "4294966303*x^2+63*x+3758112768",
            "0",
            "4059045887",
            "0",
            "1069547520",
        ],
    );
    // modulo 2^64, mean stripe = 16
    striped_random_u64_polynomials_reduced_mod_power_of_2_helper(
        64,
        16,
        1,
        2,
        1,
        &[
            "271656550527*x^5+4398046510592*x^4+2251799813816318*x^3+9727775212300075008*x^2+184\
            46744005015272960*x+18302682203357708288",
            "27127151148662784",
            "8866461766451184*x^7+4398046446591*x^5+9007199254740543*x^4+9223652962075148288*x^3\
            +13835058055549583359*x^2+31525223161659391*x+79164805742588",
            "18446181398633971712",
            "18446708889362628608*x^13+18446744073709551391*x^12+35064079451135*x^11+18446744069\
            414519583*x^10+288230341791973435*x^9+17293892663002529791*x^8+18446744073642442782*\
            x^7+18437736908812582907*x^6+576451956210401287*x^5+2305560451871916032*x^4+18446739\
            675661205407*x^3+274876915712*x^2+4123198226431*x+35047000244217",
            "0",
            "281474976647160*x^4+17302759399613267968*x^3+288019269919178751*x^2+127*x+184444928\
            23391618040",
            "8937393323614666736*x^3+36027697507338224*x^2+536347015*x+283726879603687423",
            "18446741977799064576",
            "0",
            "288221580058689536*x^5+18446743523953733632*x^4+34359737375*x^3+562812518662112*x^2\
            +8935159250774657023*x+18446744073692775420",
            "281466386776064*x+4609434252973438976",
            "0",
            "0",
            "2305842750458691583",
            "18446743940565565439*x^2+1121518973108216*x+18442241023837990911",
            "0",
            "17294948469009285120",
            "0",
            "216172919535960064",
        ],
    );
}

#[test]
#[should_panic]
fn striped_random_u64_polynomials_reduced_mod_power_of_2_fail_1() {
    let _ = striped_random_u64_polynomials_reduced_mod_power_of_2(EXAMPLE_SEED, 0, 8, 1, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_u64_polynomials_reduced_mod_power_of_2_fail_2() {
    let _ = striped_random_u64_polynomials_reduced_mod_power_of_2(EXAMPLE_SEED, 65, 8, 1, 2, 1);
}
