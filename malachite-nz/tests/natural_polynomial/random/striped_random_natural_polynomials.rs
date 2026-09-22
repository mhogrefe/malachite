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

fn striped_random_natural_polynomials_helper(
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        striped_random_natural_polynomials(
            EXAMPLE_SEED,
            mean_stripe_numerator,
            mean_stripe_denominator,
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
fn test_striped_random_natural_polynomials() {
    // mean stripe = 4, mean bits = 2, mean length = 2/1
    striped_random_natural_polynomials_helper(
        4,
        1,
        2,
        1,
        2,
        1,
        &[
            "x^5+26*x^4+9*x^3+x^2+4",
            "2",
            "2*x^7+15*x^4+7*x^3+3*x+8",
            "3",
            "x^13+56*x^11+2*x^10+3*x^7+x^3+3*x+1",
            "0",
            "x^4+2*x+3",
            "x^3+15",
            "1",
            "0",
            "9*x^5+4*x^4+x^3+x^2",
            "x",
            "0",
            "0",
            "1",
            "x^2+x+3",
            "0",
            "2",
            "0",
            "1",
        ],
    );
    // mean stripe = 16, mean bits = 2, mean length = 2/1
    striped_random_natural_polynomials_helper(
        16,
        1,
        2,
        1,
        2,
        1,
        &[
            "x^5+16*x^4+11*x^3+x^2+4",
            "2",
            "2*x^7+15*x^4+7*x^3+3*x+8",
            "3",
            "x^13+32*x^11+2*x^10+3*x^7+x^3+3*x+1",
            "0",
            "x^4+2*x+3",
            "x^3+15",
            "1",
            "0",
            "9*x^5+4*x^4+x^3+x^2",
            "x",
            "0",
            "0",
            "1",
            "x^2+x+3",
            "0",
            "2",
            "0",
            "1",
        ],
    );
    // mean stripe = 16, mean bits = 4, mean length = 3/1
    striped_random_natural_polynomials_helper(
        16,
        1,
        4,
        1,
        3,
        1,
        &[
            "15*x^7+x^5+4*x^4+x^3+16376*x^2+2",
            "1",
            "6*x^10+7*x^9+x^8+x^5+x^3+3*x^2+15*x+15",
            "1",
            "30*x^17+3*x^14+1439*x^13+7*x^12+8*x^11+327*x^10+127*x^8+2*x^7+63*x^6+2*x^5+7*x^4+10\
            24*x^3+x^2+14*x+1",
            "0",
            "16*x^6+x^4+768*x^3+511*x^2+x+3",
            "3*x^5+16383*x^3+32*x^2+7*x+1",
            "7",
            "4095",
            "15*x^7+16*x^6+x^5+8*x^4+4*x^3+4",
            "2*x+1536",
            "0",
            "0",
            "1",
            "8*x^3+1024*x^2",
            "8*x+2",
            "7",
            "0",
            "15*x+255",
        ],
    );
}

#[test]
#[should_panic]
fn striped_random_natural_polynomials_fail_1() {
    let _ = striped_random_natural_polynomials(EXAMPLE_SEED, 1, 2, 2, 1, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_natural_polynomials_fail_2() {
    let _ = striped_random_natural_polynomials(EXAMPLE_SEED, 4, 1, 1, 0, 2, 1);
}
