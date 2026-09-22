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

fn random_rational_polynomials_with_degree_helper(
    degree: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        random_rational_polynomials_with_degree(
            EXAMPLE_SEED,
            degree,
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
fn test_random_rational_polynomials_with_degree() {
    // degree 0, the nonzero constants, mean bits = 65/64
    random_rational_polynomials_with_degree_helper(
        0,
        65,
        64,
        &[
            "-1", "1", "-1", "1", "-1", "-1", "-1", "-1", "1", "1", "-1", "1", "-1", "-1", "1",
            "-1", "1", "-1", "1", "-1",
        ],
    );
    // degree 1, mean bits = 2
    random_rational_polynomials_with_degree_helper(
        1,
        2,
        1,
        &[
            "-1/8*x",
            "x",
            "-x-6",
            "4533*x+2",
            "-1/6*x",
            "-2*x",
            "-x",
            "-1/2*x-5/18",
            "1/2*x",
            "7/20*x-1/6",
            "-x+1",
            "x",
            "-13*x",
            "-3*x+1",
            "x",
            "-3*x",
            "1/4*x+29/2",
            "-2*x",
            "x+1",
            "-3*x",
        ],
    );
    // degree 3, mean bits = 4
    random_rational_polynomials_with_degree_helper(
        3,
        4,
        1,
        &[
            "-7*x^3-1/55*x^2+x",
            "157/5*x^3+7/118*x^2-5/166*x",
            "-1/2*x^3+265/42*x^2-1/6*x-3/194",
            "53*x^3+x-1/6",
            "-10*x^3+1/14*x^2+53/15*x",
            "-35*x^3-196*x^2+714121/22*x+5/4",
            "-4*x^3-3372766/3*x^2-x",
            "-934/31*x^3-8*x-10",
            "1/2*x^3-6*x^2+3699/7*x",
            "7*x^3-47/31566*x^2+59/2*x+36",
            "-164/17*x^3-107/76*x^2-x",
            "x^3-1/2*x+13/6",
            "-21/2*x^3-791*x",
            "-3*x^3-9*x^2+117/31*x+4/83",
            "3/2*x^3+43/110*x^2-x-41/2",
            "-5/51*x^3-39*x^2-1/7*x+1/32",
            "7/496*x^3+3/10*x^2+7/2*x+5/124",
            "-2/3*x^3-x^2-1/6*x+5/1801",
            "117/2*x^3-1392/7*x^2-21/4*x+14/99",
            "-167/294*x^3+35*x^2+1/27*x+1",
        ],
    );
}

#[test]
#[should_panic]
fn random_rational_polynomials_with_degree_fail_1() {
    let _ = random_rational_polynomials_with_degree(EXAMPLE_SEED, 0, 1, 0);
}
