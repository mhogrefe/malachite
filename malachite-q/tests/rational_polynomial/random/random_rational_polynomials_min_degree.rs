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

fn random_rational_polynomials_min_degree_helper(
    min_degree: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        random_rational_polynomials_min_degree(
            EXAMPLE_SEED,
            min_degree,
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
fn test_random_rational_polynomials_min_degree() {
    // min degree 0, mean bits = 65/64, mean length = 2/1
    random_rational_polynomials_min_degree_helper(
        0,
        65,
        64,
        2,
        1,
        &[
            "-x",
            "x^2-x",
            "-x^2-x",
            "x^2-3*x",
            "-x-3",
            "-x^2+25",
            "-1",
            "-1",
            "x",
            "1",
            "-1",
            "1",
            "-x",
            "-x^4+1/2*x^3+x^2",
            "1",
            "-x^2+4",
            "1",
            "-x^5-x^4-x^3-3*x^2+4",
            "x+1",
            "-x",
        ],
    );
    // min degree 2, mean bits = 2, mean length = 4/1
    random_rational_polynomials_min_degree_helper(
        2,
        2,
        1,
        4,
        1,
        &[
            "-1/8*x^3-6*x^2",
            "x^4+2",
            "-x^4+x^3-1/6*x^2-5/18",
            "4533*x^4+x^2",
            "-1/6*x^3+29/2*x",
            "-2*x^4-x^3-8*x^2+1",
            "-x^2+1/2*x-1/3",
            "-1/2*x^2+2/7*x+5/2",
            "1/2*x^3+26*x^2+2/3*x-5",
            "7/20*x^2",
            "-x^2-32371/12*x-2/3",
            "x^2-6*x+5",
            "-13*x^3-x-59",
            "-3*x^6-8*x^5-3/26*x^4-73*x^3+x^2+3/2*x+209/3",
            "x^2+7/2*x+21/2",
            "-3*x^4-1/2*x-25/3",
            "1/4*x^2+14*x+1/6",
            "-2*x^7+5/27*x^6-1933*x^4-3*x^3+x^2-3*x-56",
            "x^3-14/3*x^2+969*x+26/3",
            "-3*x^3-4*x^2+1/3*x+181/3",
        ],
    );
    // min degree 3, mean bits = 4, mean length = 5/1
    random_rational_polynomials_min_degree_helper(
        3,
        4,
        1,
        5,
        1,
        &[
            "-7*x^4-1/55*x^2+x",
            "157/5*x^5+265/42*x^4-1/6*x^3-3/194*x^2+7/118*x-5/166",
            "-1/2*x^5+53/15*x^4+x-1/6",
            "53*x^5-196*x^3+714121/22*x^2+5/4*x+1/14",
            "-10*x^4-8*x^3-10*x^2-3372766/3*x-1",
            "-35*x^5+36*x^4-6*x^3+3699/7*x^2",
            "-4*x^3-47/31566*x+59/2",
            "-934/31*x^3+13/6*x^2-107/76*x-1",
            "1/2*x^4-791*x^3-1/2",
            "7*x^3+117/31*x^2+4/83*x",
            "-164/17*x^3-x^2-41/2*x-9",
            "x^3-1/7*x^2+1/32*x+43/110",
            "-21/2*x^4+3/10*x^3+7/2*x^2+5/124*x-39",
            "-3*x^7+x^6-1392/7*x^5-21/4*x^4+14/99*x^3-x^2-1/6*x+5/1801",
            "3/2*x^3+13/4*x^2+35*x+1/27",
            "-5/51*x^5+4584477*x^4-6/5*x^3-222",
            "7/496*x^3-12/13*x^2-284/45*x-3",
            "-2/3*x^8-3628*x^7-168*x^6+1163/167*x^5-1/2*x^4+64/3*x^3+11/3*x^2+2375805",
            "117/2*x^4-2/5*x^3+1/3*x",
            "-167/294*x^4-3*x^3-241109/29*x^2-7*x+11/13522",
        ],
    );
}

#[test]
#[should_panic]
fn random_rational_polynomials_min_degree_fail_1() {
    let _ = random_rational_polynomials_min_degree(EXAMPLE_SEED, 0, 1, 0, 2, 1);
}

#[test]
#[should_panic]
fn random_rational_polynomials_min_degree_fail_2() {
    let _ = random_rational_polynomials_min_degree(EXAMPLE_SEED, 2, 2, 1, 2, 1);
}
