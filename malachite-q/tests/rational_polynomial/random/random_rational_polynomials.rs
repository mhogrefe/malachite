// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_q::rational_polynomial::random::*;

fn random_rational_polynomials_helper(
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        random_rational_polynomials(
            EXAMPLE_SEED,
            mean_bits_numerator,
            mean_bits_denominator,
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
fn test_random_rational_polynomials() {
    // mean bits = 65/64, mean length = 2/1
    random_rational_polynomials_helper(
        65,
        64,
        2,
        1,
        &[
            "-x^5-x^4-x^2",
            "1",
            "-x^7+25*x^3-3*x^2-3*x",
            "1",
            "-x^13+x^11-x^10-x^9-3*x^8+4*x^6+4*x^4+1/2*x^3+x^2",
            "0",
            "-x^4-x+2",
            "-x^3+2*x-1",
            "-1",
            "0",
            "x^5",
            "x-4",
            "0",
            "0",
            "-1",
            "x^2+x+1",
            "0",
            "-1",
            "0",
            "-1",
        ],
    );
    // mean bits = 2, mean length = 2/1
    random_rational_polynomials_helper(
        2,
        1,
        2,
        1,
        &[
            "-1/8*x^5+2*x^3-6*x^2",
            "1",
            "-x^7+x^5-1/6*x^4-5/18*x^2",
            "4533",
            "-1/6*x^13+5/2*x^12+1/2*x^11-1/3*x^10-x^9-8*x^8+x^6+29/2*x^4+x",
            "0",
            "-2*x^4+26*x^3+2/3*x^2-5*x+2/7",
            "-x^3-2/3*x^2",
            "-1/2",
            "0",
            "1/2*x^5-x^4-59*x^3-6*x^2+5*x-32371/12",
            "7/20*x",
            "0",
            "0",
            "-1",
            "x^2+3/2*x+209/3",
            "0",
            "-13",
            "0",
            "-3",
        ],
    );
    // mean bits = 4, mean length = 3/1
    random_rational_polynomials_helper(
        4,
        1,
        3,
        1,
        &[
            "-7*x^7-3/194*x^6+7/118*x^5-5/166*x^4-1/55*x^2+x",
            "157/5",
            "-1/2*x^10+714121/22*x^9+5/4*x^8+1/14*x^7+53/15*x^6+x^3-1/6*x^2+265/42*x-1/6",
            "53",
            "-10*x^17+13/6*x^16-107/76*x^15-x^14-47/31566*x^12+59/2*x^11+36*x^10-6*x^9+3699/7*x^\
            8-8*x^5-10*x^4-3372766/3*x^3-x^2-196",
            "0",
            "-35*x^6+4/83*x^5-791*x^3-1/2",
            "-4*x^5+43/110*x^4-x^3-41/2*x^2-9*x+117/31",
            "-934/31",
            "1/2",
            "7*x^7+5/1801*x^6+3/10*x^5+7/2*x^4+5/124*x^3-39*x^2-1/7*x+1/32",
            "-164/17*x-1/6",
            "0",
            "0",
            "1",
            "-21/2*x^3-21/4*x^2+14/99*x-1",
            "-3*x-1392/7",
            "3/2",
            "0",
            "-5/51*x+1",
        ],
    );
}

#[test]
#[should_panic]
fn random_rational_polynomials_fail_1() {
    let _ = random_rational_polynomials(EXAMPLE_SEED, 1, 0, 2, 1);
}

#[test]
#[should_panic]
fn random_rational_polynomials_fail_2() {
    let _ = random_rational_polynomials(EXAMPLE_SEED, 2, 1, 1, 0);
}
