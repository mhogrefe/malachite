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

fn random_integer_polynomials_helper(
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        random_integer_polynomials(
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
fn test_random_integer_polynomials() {
    // mean bits = 65/64, mean length = 2/1
    random_integer_polynomials_helper(
        65,
        64,
        2,
        1,
        &[
            "x^5+x^4+3*x^3+5*x^2-x-3",
            "-1",
            "2*x^7+x^6+x^5-3*x^4+3*x^3",
            "1",
            "x^13-2*x^10-3*x^9-2*x^8-x^7+3*x^6+x^5-3*x^3+78*x",
            "0",
            "x^4-x^3-4*x^2+x-1",
            "-x^3+x^2+3",
            "-1",
            "0",
            "-x^5+4*x^3+x^2+2",
            "-x",
            "0",
            "0",
            "-1",
            "x^2+x",
            "0",
            "-1",
            "0",
            "-1",
        ],
    );
    // mean bits = 2, mean length = 2/1
    random_integer_polynomials_helper(
        2,
        1,
        2,
        1,
        &[
            "x^5+11*x^4+3*x^3+3*x^2+3*x",
            "-38",
            "x^7-846*x^6+x^5+18*x^4-3*x^2+x-7",
            "1",
            "2*x^13-4*x^12-3*x^11-6*x^9+x^8+x^7-6*x^5+11*x^4-81*x^3+3*x-11",
            "0",
            "x^4-4012*x^3-23*x^2-3",
            "-2*x^3-x^2+50*x+1",
            "-1",
            "0",
            "-x^5+7*x^4-x^2+1487*x+1",
            "-2*x-484",
            "0",
            "0",
            "-1",
            "4*x^2+x-3",
            "0",
            "-3",
            "0",
            "-4",
        ],
    );
    // mean bits = 4, mean length = 3/1
    random_integer_polynomials_helper(
        4,
        1,
        3,
        1,
        &[
            "14*x^7-x^6+799*x^5+799*x^4+19*x^3+x^2-x-497",
            "-2",
            "2*x^10-x^8-131*x^7-5*x^5-119*x^4+59*x^3+334*x^2-10721*x+66",
            "122",
            "x^17-280*x^16+463*x^15+190*x^14-132*x^13+2*x^12-6*x^11+28*x^8-903*x^7-x^6-37408*x^4\
            +11*x^3+10*x^2-x+7",
            "0",
            "x^6-18*x^5+x^4+x^3-19203*x^2-36*x+1",
            "-6*x^5-39*x^4-1107*x^3-x^2+88*x-1",
            "-34",
            "-3",
            "-14*x^7+218*x^6+459*x^5+1142*x^4+6*x^3-8091*x^2",
            "-x+3",
            "0",
            "0",
            "8",
            "-x^3+6*x^2-21747*x-12",
            "-8*x+8567",
            "-7",
            "0",
            "-7853*x+3",
        ],
    );
}

#[test]
#[should_panic]
fn random_integer_polynomials_fail_1() {
    let _ = random_integer_polynomials(EXAMPLE_SEED, 1, 0, 2, 1);
}

#[test]
#[should_panic]
fn random_integer_polynomials_fail_2() {
    let _ = random_integer_polynomials(EXAMPLE_SEED, 2, 1, 1, 0);
}
