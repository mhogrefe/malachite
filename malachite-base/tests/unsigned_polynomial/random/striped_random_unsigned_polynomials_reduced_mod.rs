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

fn striped_random_unsigned_polynomials_reduced_mod_helper(
    m: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        striped_random_unsigned_polynomials_reduced_mod::<u64>(
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
fn test_striped_random_unsigned_polynomials_reduced_mod() {
    // modulo 1000, mean stripe = 8
    striped_random_unsigned_polynomials_reduced_mod_helper(
        1000,
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
    striped_random_unsigned_polynomials_reduced_mod_helper(
        1000,
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
    // modulo the largest u64, mean stripe = 16
    striped_random_unsigned_polynomials_reduced_mod_helper(
        u64::MAX,
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
fn striped_random_unsigned_polynomials_reduced_mod_fail_1() {
    let _ = striped_random_unsigned_polynomials_reduced_mod::<u64>(EXAMPLE_SEED, 1, 8, 1, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_unsigned_polynomials_reduced_mod_fail_2() {
    let _ = striped_random_unsigned_polynomials_reduced_mod::<u64>(EXAMPLE_SEED, 3, 1, 2, 2, 1);
}
