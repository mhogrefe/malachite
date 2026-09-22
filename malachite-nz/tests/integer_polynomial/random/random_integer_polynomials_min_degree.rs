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

fn random_integer_polynomials_min_degree_helper(
    min_degree: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        random_integer_polynomials_min_degree(
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
fn test_random_integer_polynomials_min_degree() {
    // min degree 0, mean bits = 65/64, mean length = 2/1
    random_integer_polynomials_min_degree_helper(
        0,
        65,
        64,
        2,
        1,
        &[
            "x-3",
            "-x^2+5*x-1",
            "2*x^2+x+3",
            "x^2",
            "x",
            "x^2-3*x+3",
            "-1",
            "-1",
            "-x+1",
            "-1",
            "-1",
            "1",
            "-x+1",
            "-x^4-3*x^3+78*x",
            "-1",
            "-x^2+x",
            "-1",
            "-x^5-2*x^4-3*x^3-2*x^2-x+3",
            "x",
            "-x",
        ],
    );
    // min degree 2, mean bits = 2, mean length = 4/1
    random_integer_polynomials_min_degree_helper(
        2,
        2,
        1,
        4,
        1,
        &[
            "x^3+3*x^2+3*x",
            "-38*x^4+x^3-7*x^2+11*x+3",
            "x^4+x^3+18*x^2-3",
            "x^4+3*x^2-11*x-846",
            "2*x^3-6*x^2+11*x-81",
            "x^4-6*x^3+x^2+x",
            "-2*x^2-3*x",
            "-x^2-3*x-4",
            "-x^3-4012*x^2-23*x",
            "-2*x^2+50*x+1",
            "-x^2+x-1",
            "4*x^2-x+1487",
            "-3*x^3-484*x^2+7*x",
            "-4*x^6-x^5-2*x^4-x^3+x^2+x-3",
            "-3*x^2+7*x+2",
            "-x^4-x^2",
            "-x^2+2*x+91",
            "-x^7-x^5+4*x^4-3*x^3-6*x^2+3*x+14",
            "x^3-55*x^2+2",
            "-x^3-6*x^2-10*x+1",
        ],
    );
    // min degree 3, mean bits = 4, mean length = 5/1
    random_integer_polynomials_min_degree_helper(
        3,
        4,
        1,
        5,
        1,
        &[
            "14*x^4+19*x^3+x^2-x-497",
            "-2*x^5-10721*x^4+66*x^3-x^2+799*x+799",
            "2*x^5-5*x^3-119*x^2+59*x+334",
            "122*x^5-x^4+7*x^3-x-131",
            "x^4-37408*x^2+11*x+10",
            "x^5+28*x^2-903*x-1",
            "-6*x^3-132*x^2+2*x-6",
            "-34*x^3-280*x^2+463*x+190",
            "-3*x^4+x^3-19203*x^2-36*x+1",
            "-14*x^3-x^2-18*x+1",
            "-x^3-1107*x^2-x+88",
            "8*x^3-39",
            "-x^4+459*x^3+1142*x^2+6*x-8091",
            "-8*x^7+3*x^6+8567*x^5+6*x^4-21747*x^3-12*x^2+3*x+218",
            "-7*x^3+25*x^2+x-1",
            "-7853*x^5-193*x^4+6639*x^3+22821*x^2-27",
            "-26*x^3-779*x^2-507*x",
            "-3*x^8-44*x^7+x^6-x^5+6*x^4-6*x^3-27*x^2+2*x+58",
            "52*x^4-15*x^2+117*x-14",
            "-13*x^4+4*x^3-7*x^2-3*x",
        ],
    );
}

#[test]
#[should_panic]
fn random_integer_polynomials_min_degree_fail_1() {
    let _ = random_integer_polynomials_min_degree(EXAMPLE_SEED, 0, 1, 0, 2, 1);
}

#[test]
#[should_panic]
fn random_integer_polynomials_min_degree_fail_2() {
    let _ = random_integer_polynomials_min_degree(EXAMPLE_SEED, 2, 2, 1, 2, 1);
}
