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

fn random_natural_polynomials_min_degree_helper(
    min_degree: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        random_natural_polynomials_min_degree(
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
fn test_random_natural_polynomials_min_degree() {
    // min degree 0, mean bits = 65/64, mean length = 2/1
    random_natural_polynomials_min_degree_helper(
        0,
        65,
        64,
        2,
        1,
        &[
            "x+5",
            "x^2+x+1",
            "x^2+1",
            "x^2+63",
            "x+1",
            "x^2+13*x",
            "1",
            "1",
            "x",
            "1",
            "1",
            "1",
            "x+6",
            "x^4+x+5",
            "1",
            "x^2+3*x",
            "1",
            "x^5+5*x+55",
            "x",
            "x+11",
        ],
    );
    // min degree 2, mean bits = 2, mean length = 4/1
    random_natural_polynomials_min_degree_helper(
        2,
        2,
        1,
        4,
        1,
        &[
            "x^3+x^2+5",
            "2*x^4+3*x^3+15*x^2+19*x+13",
            "2*x^4+10*x^2+5*x",
            "2*x^4+2*x^2+x",
            "x^3+1",
            "x^4+3*x",
            "x^2+35*x+3",
            "x^2+2*x",
            "15*x^3+3",
            "x^2+12",
            "x^2",
            "x^2+x",
            "3*x^3+4*x+1",
            "x^6+18*x^5+10*x^3+x^2+x+3",
            "15*x^2+62*x+1",
            "3*x^4+4*x^3+79",
            "x^2+15*x",
            "x^7+x^5+6*x^3+3*x+1",
            "12*x^3+96*x^2+x",
            "3*x^3",
        ],
    );
    // min degree 3, mean bits = 4, mean length = 5/1
    random_natural_polynomials_min_degree_helper(
        3,
        4,
        1,
        5,
        1,
        &[
            "14*x^4+x^3+13235*x^2+3",
            "x^5+13*x^4+15*x^3+x+7",
            "4*x^5+x^3+x+2",
            "x^5+11*x^4+x^3+7*x^2+x",
            "30*x^4+2*x^3+4*x^2+1523*x+1",
            "19*x^5+391*x^4+117*x^2+2*x+35",
            "2*x^3+1394*x^2+6*x+12",
            "6*x^3+2",
            "3911*x^4+1023*x^3+280*x^2+x+2",
            "14*x^3+x^2+1",
            "2*x^3+8210*x^2+52*x+6",
            "x^3+4*x",
            "15*x^4+x^3+12*x^2+4*x",
            "8*x^7+203*x^6+2*x^5+1866*x^4+1947*x+23",
            "7*x^3+29*x^2+26",
            "13*x^5+7*x^3+330*x^2+x+68",
            "6*x^3+6*x^2+11",
            "3*x^8+763*x^7+x^6+15*x^5+21*x^4+x^3+25*x^2+16074",
            "276*x^4+91*x^3+x^2+442*x+1",
            "1765*x^4+x^3+14*x^2+6*x",
        ],
    );
}

#[test]
#[should_panic]
fn random_natural_polynomials_min_degree_fail_1() {
    let _ = random_natural_polynomials_min_degree(EXAMPLE_SEED, 0, 1, 0, 2, 1);
}

#[test]
#[should_panic]
fn random_natural_polynomials_min_degree_fail_2() {
    let _ = random_natural_polynomials_min_degree(EXAMPLE_SEED, 2, 2, 1, 2, 1);
}
