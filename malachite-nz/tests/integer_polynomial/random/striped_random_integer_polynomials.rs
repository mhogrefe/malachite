// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::integer_polynomial::random::*;

fn striped_random_integer_polynomials_helper(
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        striped_random_integer_polynomials(
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
fn test_striped_random_integer_polynomials() {
    // mean stripe = 4, mean bits = 2, mean length = 2/1
    striped_random_integer_polynomials_helper(
        4,
        1,
        2,
        1,
        2,
        1,
        &[
            "x^5+8*x^4+3*x^3+2*x^2+2*x",
            "-48",
            "x^7-575*x^6+x^5+31*x^4-3*x^2+x-6",
            "1",
            "2*x^13-6*x^12-3*x^11-4*x^9+x^8+x^7-7*x^5+12*x^4-64*x^3+3*x-12",
            "0",
            "x^4-2152*x^3-19*x^2-3",
            "-3*x^3-x^2+63*x+1",
            "-1",
            "0",
            "-x^5+4*x^4-x^2+1038*x+1",
            "-3*x-511",
            "0",
            "0",
            "-1",
            "6*x^2+x-2",
            "0",
            "-2",
            "0",
            "-6",
        ],
    );
    // mean stripe = 16, mean bits = 2, mean length = 2/1
    striped_random_integer_polynomials_helper(
        16,
        1,
        2,
        1,
        2,
        1,
        &[
            "x^5+14*x^4+3*x^3+2*x^2+2*x",
            "-48",
            "x^7-1023*x^6+x^5+31*x^4-3*x^2+x-4",
            "1",
            "2*x^13-4*x^12-3*x^11-4*x^9+x^8+x^7-7*x^5+8*x^4-64*x^3+3*x-8",
            "0",
            "x^4-3584*x^3-31*x^2-3",
            "-3*x^3-x^2+61*x+1",
            "-1",
            "0",
            "-x^5+4*x^4-x^2+1792*x+1",
            "-3*x-511",
            "0",
            "0",
            "-1",
            "4*x^2+x-2",
            "0",
            "-2",
            "0",
            "-4",
        ],
    );
    // mean stripe = 16, mean bits = 4, mean length = 3/1
    striped_random_integer_polynomials_helper(
        16,
        1,
        4,
        1,
        3,
        1,
        &[
            "15*x^7-x^6+543*x^5+512*x^4+16*x^3+x^2-x-496",
            "-2",
            "2*x^10-x^8-230*x^7-4*x^5-127*x^4+36*x^3+383*x^2-10239*x+127",
            "65",
            "x^17-511*x^16+480*x^15+255*x^14-191*x^13+3*x^12-7*x^11+16*x^8-1023*x^7-x^6-65024*x^\
            4+15*x^3+8*x^2-x+4",
            "0",
            "x^6-31*x^5+x^4+x^3-16384*x^2-63*x+1",
            "-7*x^5-56*x^4-1024*x^3-x^2+96*x-1",
            "-63",
            "-3",
            "-15*x^7+255*x^6+257*x^5+1024*x^4+4*x^3-4110*x^2",
            "-x+2",
            "0",
            "0",
            "8",
            "-x^3+7*x^2-32256*x-8",
            "-8*x+9215",
            "-7",
            "0",
            "-4111*x+2",
        ],
    );
}

#[test]
#[should_panic]
fn striped_random_integer_polynomials_fail_1() {
    let _ = striped_random_integer_polynomials(EXAMPLE_SEED, 1, 2, 2, 1, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_integer_polynomials_fail_2() {
    let _ = striped_random_integer_polynomials(EXAMPLE_SEED, 4, 1, 1, 0, 2, 1);
}
