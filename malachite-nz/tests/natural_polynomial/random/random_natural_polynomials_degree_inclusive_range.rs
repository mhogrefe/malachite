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

fn random_natural_polynomials_degree_inclusive_range_helper(
    a: u64,
    b: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        random_natural_polynomials_degree_inclusive_range(
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
fn test_random_natural_polynomials_degree_inclusive_range() {
    // degrees in [0, 1], mean bits = 65/64
    random_natural_polynomials_degree_inclusive_range_helper(
        0,
        1,
        65,
        64,
        &[
            "x+5", "1", "x+1", "x+1", "1", "x+1", "x", "1", "x+63", "x", "1", "x+1", "1", "x",
            "x+13", "1", "1", "1", "x", "1",
        ],
    );
    // degrees in [1, 3], mean bits = 2
    random_natural_polynomials_degree_inclusive_range_helper(
        1,
        3,
        2,
        1,
        &[
            "x^2+5",
            "2*x^3+19*x^2+13*x+1",
            "2*x^2+3*x+15",
            "2*x^3+10*x^2+5*x",
            "x^3+x^2",
            "x^2+2",
            "x+1",
            "x^2",
            "15*x",
            "x^3+3",
            "x^3+35*x+3",
            "x^2+3*x+2",
            "3*x^3+12*x^2",
            "x^2",
            "15*x^3+x^2",
            "3*x^3+4*x+1",
            "x^3+x^2+x+3",
            "x+10",
            "12*x^3+x^2+18*x",
            "3*x+62",
        ],
    );
    // degrees in [2, 4], mean bits = 4
    random_natural_polynomials_degree_inclusive_range_helper(
        2,
        4,
        4,
        1,
        &[
            "14*x^3+13235*x^2+3",
            "x^4+x^2+7*x+1",
            "4*x^3+2*x^2+13*x+15",
            "x^4+x^2+1",
            "30*x^4+x^3+7*x^2+x",
            "19*x^3+1523*x^2+x+11",
            "2*x^2+2*x+4",
            "6*x^3+117*x^2+2*x+35",
            "3911*x^2+391*x",
            "14*x^4+2*x^3+1394*x^2+6*x+12",
            "2*x^4+x^3+2*x^2",
            "x^3+x^2+1023*x+280",
            "15*x^4+52*x^3+6*x^2+x",
            "8*x^3+4*x^2+8210",
            "7*x^4+12*x^3+4*x^2",
            "13*x^4+1947*x^2+23*x+1",
            "6*x^4+203*x^3+2*x^2+1866*x",
            "3*x^2+26",
            "276*x^4+330*x^3+x^2+68*x+29",
            "1765*x^2+7",
        ],
    );
}

#[test]
#[should_panic]
fn random_natural_polynomials_degree_inclusive_range_fail_1() {
    let _ = random_natural_polynomials_degree_inclusive_range(EXAMPLE_SEED, 3, 2, 2, 1);
}
