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

fn random_natural_polynomials_with_degree_helper(
    degree: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        random_natural_polynomials_with_degree(
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
fn test_random_natural_polynomials_with_degree() {
    // degree 0, the nonzero constants, mean bits = 65/64
    random_natural_polynomials_with_degree_helper(
        0,
        65,
        64,
        &[
            "1", "1", "1", "1", "1", "1", "1", "1", "1", "1", "1", "1", "1", "1", "1", "1", "1",
            "1", "1", "1",
        ],
    );
    // degree 1, mean bits = 2
    random_natural_polynomials_with_degree_helper(
        1,
        2,
        1,
        &[
            "x+5", "2*x", "2*x+1", "2*x+13", "x+19", "x+15", "x+3", "x", "15*x+5", "x+10", "x",
            "x", "3*x+1", "x+2", "15*x", "3*x+1", "x", "x", "12*x", "3*x+3",
        ],
    );
    // degree 3, mean bits = 4
    random_natural_polynomials_with_degree_helper(
        3,
        4,
        1,
        &[
            "14*x^3+13235*x^2+3",
            "x^3+x^2+7*x+1",
            "4*x^3+13*x^2+15*x",
            "x^3+x+2",
            "30*x^3+1",
            "19*x^3+x^2+7*x+1",
            "2*x^3+1523*x^2+x+11",
            "6*x^3+35*x^2+2*x+4",
            "3911*x^3+117*x+2",
            "14*x^3+6*x^2+12*x+391",
            "2*x^3+2*x+1394",
            "x^3+x^2+2*x",
            "15*x^3+x^2+1023*x+280",
            "8*x^3+6*x^2+x",
            "7*x^3+8210*x+52",
            "13*x^3+4",
            "6*x^3+x^2+12*x+4",
            "3*x^3+1947*x+23",
            "276*x^3+2*x^2+1866*x",
            "1765*x^3+26*x+203",
        ],
    );
}

#[test]
#[should_panic]
fn random_natural_polynomials_with_degree_fail_1() {
    let _ = random_natural_polynomials_with_degree(EXAMPLE_SEED, 0, 1, 0);
}
