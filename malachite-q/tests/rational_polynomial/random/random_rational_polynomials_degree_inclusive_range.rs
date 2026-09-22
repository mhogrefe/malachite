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

fn random_rational_polynomials_degree_inclusive_range_helper(
    a: u64,
    b: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        random_rational_polynomials_degree_inclusive_range(
            EXAMPLE_SEED,
            a,
            b,
            mean_bits_numerator,
            mean_bits_denominator
        )
        .take(20)
        .map(|p| p.to_string())
        .collect_vec(),
        expected_values
    );
}

#[test]
fn test_random_rational_polynomials_degree_inclusive_range() {
    // degrees in [0, 1], mean bits = 65/64
    random_rational_polynomials_degree_inclusive_range_helper(
        0,
        1,
        65,
        64,
        &[
            "-x", "1", "-x", "x-1", "-1", "-x", "-x-1", "-1", "x", "x-3", "-1", "x-3", "-1",
            "-x+25", "x", "-1", "1", "-1", "x", "-1",
        ],
    );
    // degrees in [1, 3], mean bits = 2
    random_rational_polynomials_degree_inclusive_range_helper(
        1,
        3,
        2,
        1,
        &[
            "-1/8*x^2",
            "x^3+2*x-6",
            "-x^2",
            "4533*x^3-1/6*x^2-5/18",
            "-1/6*x^3+1",
            "-2*x^2+1",
            "-x",
            "-1/2*x^2+29/2",
            "1/2*x+1",
            "7/20*x^3-x^2-8*x",
            "-x^3+5/2*x^2+1/2*x-1/3",
            "x^2-5*x+2/7",
            "-13*x^3+26*x+2/3",
            "-3*x^2-2/3*x",
            "x^3-6*x^2+5*x-32371/12",
            "-3*x^3-x-59",
            "1/4*x^3+x^2+3/2*x+209/3",
            "-2*x-73",
            "x^3+21/2*x^2-8*x-3/26",
            "-3*x+7/2",
        ],
    );
    // degrees in [2, 4], mean bits = 4
    random_rational_polynomials_degree_inclusive_range_helper(
        2,
        4,
        4,
        1,
        &[
            "-7*x^3-1/55*x^2+x",
            "157/5*x^4-3/194*x^3+7/118*x^2-5/166*x",
            "-1/2*x^3-1/6*x^2+265/42*x-1/6",
            "53*x^4+53/15*x^3+1",
            "-10*x^4-196*x^3+714121/22*x^2+5/4*x+1/14",
            "-35*x^3-3372766/3*x^2-x",
            "-4*x^2-8*x-10",
            "-934/31*x^3+3699/7*x^2",
            "1/2*x^2+36*x-6",
            "7*x^4-x^3-47/31566*x+59/2",
            "-164/17*x^4-1/2*x^2+13/6*x-107/76",
            "x^3-791*x",
            "-21/2*x^4-41/2*x^3-9*x^2+117/31*x+4/83",
            "-3*x^3+1/32*x^2+43/110*x-1",
            "3/2*x^4+7/2*x^3+5/124*x^2-39*x-1/7",
            "-5/51*x^4-x^3-1/6*x^2+5/1801*x+3/10",
            "7/496*x^4+x^3-1392/7*x^2-21/4*x+14/99",
            "-2/3*x^2+35*x+1/27",
            "117/2*x^4-222*x+13/4",
            "-167/294*x^2+4584477*x-6/5",
        ],
    );
}

#[test]
#[should_panic]
fn random_rational_polynomials_degree_inclusive_range_fail_1() {
    let _ = random_rational_polynomials_degree_inclusive_range(EXAMPLE_SEED, 3, 2, 2, 1);
}
