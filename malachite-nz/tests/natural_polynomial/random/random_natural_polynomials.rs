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

fn random_natural_polynomials_helper(
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        random_natural_polynomials(
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
fn test_random_natural_polynomials() {
    // mean bits = 65/64, mean length = 2/1
    random_natural_polynomials_helper(
        65,
        64,
        2,
        1,
        &[
            "x^5+x^3+x^2+x+5",
            "1",
            "x^7+6*x^6+13*x^4+x^2+63",
            "1",
            "x^13+11*x^12+5*x^7+55*x^6+3*x^5+x+5",
            "0",
            "x^4",
            "x^3+7*x+2",
            "1",
            "0",
            "x^5+6*x^4+2*x^2",
            "x",
            "0",
            "0",
            "1",
            "x^2+3*x",
            "0",
            "1",
            "0",
            "1",
        ],
    );
    // mean bits = 2, mean length = 2/1
    random_natural_polynomials_helper(
        2,
        1,
        2,
        1,
        &[
            "x^5+19*x^4+13*x^3+x^2+5",
            "2",
            "2*x^7+10*x^4+5*x^3+3*x+15",
            "2",
            "x^13+35*x^11+3*x^10+3*x^7+x^3+2*x+1",
            "0",
            "x^4+3*x+2",
            "x^3+12",
            "1",
            "0",
            "15*x^5+4*x^4+x^3+x^2",
            "x",
            "0",
            "0",
            "1",
            "x^2+x+3",
            "0",
            "3",
            "0",
            "1",
        ],
    );
    // mean bits = 4, mean length = 3/1
    random_natural_polynomials_helper(
        4,
        1,
        3,
        1,
        &[
            "14*x^7+x^5+7*x^4+x^3+13235*x^2+3",
            "1",
            "4*x^10+7*x^9+x^8+x^5+x^3+2*x^2+13*x+15",
            "1",
            "30*x^17+2*x^14+1394*x^13+6*x^12+12*x^11+391*x^10+117*x^8+2*x^7+35*x^6+2*x^5+4*x^4+1\
            523*x^3+x^2+11*x+1",
            "0",
            "19*x^6+x^4+1023*x^3+280*x^2+x+2",
            "2*x^5+8210*x^3+52*x^2+6*x+1",
            "6",
            "3911",
            "14*x^7+23*x^6+x^5+12*x^4+4*x^3+4",
            "2*x+1947",
            "0",
            "0",
            "1",
            "15*x^3+1866*x^2",
            "8*x+2",
            "7",
            "0",
            "13*x+203",
        ],
    );
}

#[test]
#[should_panic]
fn random_natural_polynomials_fail_1() {
    let _ = random_natural_polynomials(EXAMPLE_SEED, 1, 0, 2, 1);
}

#[test]
#[should_panic]
fn random_natural_polynomials_fail_2() {
    let _ = random_natural_polynomials(EXAMPLE_SEED, 2, 1, 1, 0);
}
