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

fn random_integer_polynomials_with_degree_helper(
    degree: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        random_integer_polynomials_with_degree(
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
fn test_random_integer_polynomials_with_degree() {
    // degree 0, the nonzero constants, mean bits = 65/64
    random_integer_polynomials_with_degree_helper(
        0,
        65,
        64,
        &[
            "1", "-1", "2", "1", "1", "1", "-1", "-1", "-1", "-1", "-1", "1", "-1", "-1", "-1",
            "-1", "-1", "-1", "1", "-1",
        ],
    );
    // degree 1, mean bits = 2
    random_integer_polynomials_with_degree_helper(
        1,
        2,
        1,
        &[
            "x", "-38*x+3", "x+3", "x+3", "2*x+11", "x-7", "-2*x+1", "-x-3", "-x", "-2*x+18",
            "-x+1", "4*x-846", "-3*x-11", "-4*x+3", "-3*x", "-x-81", "-x+11", "-x-6", "x", "-x+1",
        ],
    );
    // degree 3, mean bits = 4
    random_integer_polynomials_with_degree_helper(
        3,
        4,
        1,
        &[
            "14*x^3+x^2-x-497",
            "-2*x^3+799*x^2+799*x+19",
            "2*x^3-10721*x^2+66*x-1",
            "122*x^3-119*x^2+59*x+334",
            "x^3-131*x^2-5",
            "x^3+7*x^2-1",
            "-6*x^3+11*x^2+10*x-1",
            "-34*x^3-x^2-37408",
            "-3*x^3+28*x-903",
            "-14*x^3+2*x^2-6*x",
            "-x^3+463*x^2+190*x-132",
            "8*x^3-36*x^2+x-280",
            "-x^3+x^2+x-19203",
            "-8*x^3+88*x^2-x-18",
            "-7*x^3-39*x^2-1107*x-1",
            "-7853*x^3-8091*x^2",
            "-26*x^3+459*x^2+1142*x+6",
            "-3*x^3-12*x^2+3*x+218",
            "52*x^3+8567*x^2+6*x-21747",
            "-13*x^3+x^2-x+3",
        ],
    );
}

#[test]
#[should_panic]
fn random_integer_polynomials_with_degree_fail_1() {
    let _ = random_integer_polynomials_with_degree(EXAMPLE_SEED, 0, 1, 0);
}
