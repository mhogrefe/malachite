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

fn random_integer_polynomials_degree_range_helper(
    a: u64,
    b: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        random_integer_polynomials_degree_range(
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
fn test_random_integer_polynomials_degree_range() {
    // degrees in [0, 2), mean bits = 65/64
    random_integer_polynomials_degree_range_helper(
        0,
        2,
        65,
        64,
        &[
            "x-3", "-1", "2*x-1", "x+5", "1", "x+3", "-x+1", "-1", "-x", "-x", "-1", "x", "-1",
            "-x+3", "-x-3", "-1", "-1", "-1", "x+1", "-1",
        ],
    );
    // degrees in [1, 4), mean bits = 2
    random_integer_polynomials_degree_range_helper(
        1,
        4,
        2,
        1,
        &[
            "x^2+3*x",
            "-38*x^3+11*x^2+3*x+3",
            "x^2+x-7",
            "x^3+18*x^2-3",
            "2*x^3-11*x^2-846*x+1",
            "x^2+3",
            "-2*x-81",
            "-x^2-6*x+11",
            "-x",
            "-2*x^3-6*x^2+x+1",
            "-x^3-4*x^2-3*x",
            "4*x^2-3",
            "-3*x^3+x^2-4012*x-23",
            "-4*x^2-x+50",
            "-3*x^3-x^2+1487*x+1",
            "-x^3-484*x^2+7*x",
            "-x^3+x^2+x-3",
            "-x-1",
            "x^3+2*x^2-x-2",
            "-x+7",
        ],
    );
    // degrees in [2, 5), mean bits = 4
    random_integer_polynomials_degree_range_helper(
        2,
        5,
        4,
        1,
        &[
            "14*x^3+x^2-x-497",
            "-2*x^4-x^3+799*x^2+799*x+19",
            "2*x^3+334*x^2-10721*x+66",
            "122*x^4-5*x^2-119*x+59",
            "x^4+7*x^3-x-131",
            "x^3+11*x^2+10*x-1",
            "-6*x^2-37408",
            "-34*x^3+28*x^2-903*x-1",
            "-3*x^2",
            "-14*x^4+190*x^3-132*x^2+2*x-6",
            "-x^4-36*x^3+x^2-280*x+463",
            "8*x^3+x^2+x-19203",
            "-x^4-x^3+88*x^2-x-18",
            "-8*x^3-39*x-1107",
            "-7*x^4+1142*x^3+6*x^2-8091*x",
            "-7853*x^4-12*x^3+3*x^2+218*x+459",
            "-26*x^4+3*x^3+8567*x^2+6*x-21747",
            "-3*x^2+x-1",
            "52*x^4+22821*x^3-27*x+25",
            "-13*x^2-193*x+6639",
        ],
    );
}

#[test]
#[should_panic]
fn random_integer_polynomials_degree_range_fail_1() {
    let _ = random_integer_polynomials_degree_range(EXAMPLE_SEED, 3, 3, 2, 1);
}
