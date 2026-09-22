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

fn striped_random_natural_polynomials_with_degree_helper(
    degree: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        striped_random_natural_polynomials_with_degree(
            EXAMPLE_SEED,
            degree,
            mean_stripe_numerator,
            mean_stripe_denominator,
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
fn test_striped_random_natural_polynomials_with_degree() {
    // degree 0, mean stripe = 16, mean bits = 2
    striped_random_natural_polynomials_with_degree_helper(
        0,
        16,
        1,
        2,
        1,
        &[
            "1", "2", "2", "3", "1", "1", "1", "1", "9", "1", "1", "1", "2", "1", "15", "3", "1",
            "1", "15", "3",
        ],
    );
    // degree 1, mean stripe = 16, mean bits = 2
    striped_random_natural_polynomials_with_degree_helper(
        1,
        16,
        1,
        2,
        1,
        &[
            "x+4", "2*x", "2*x+1", "3*x+11", "x+16", "x+8", "x+3", "x", "9*x+7", "x+15", "x", "x",
            "2*x+1", "x+3", "15*x", "3*x+1", "x", "x", "15*x", "3*x+3",
        ],
    );
    // degree 3, mean stripe = 4, mean bits = 4
    striped_random_natural_polynomials_with_degree_helper(
        3,
        4,
        1,
        4,
        1,
        &[
            "15*x^3+12380*x^2+2",
            "x^3+x^2+4*x+1",
            "6*x^3+15*x^2+15*x",
            "x^3+x+3",
            "20*x^3+1",
            "20*x^3+x^2+5*x+1",
            "3*x^3+1984*x^2+x+14",
            "7*x^3+47*x^2+2*x+7",
            "3071*x^3+87*x+2",
            "9*x^3+7*x^2+8*x+257",
            "2*x^3+3*x+1921",
            "x^3+x^2+3*x",
            "14*x^3+x^2+892*x+511",
            "8*x^3+7*x^2+x",
            "7*x^3+15631*x+38",
            "15*x^3+4",
            "4*x^3+x^2+10*x+6",
            "3*x^3+1638*x+22",
            "463*x^3+2*x^2+1568*x",
            "1799*x^3+19*x+251",
        ],
    );
}

#[test]
#[should_panic]
fn striped_random_natural_polynomials_with_degree_fail_1() {
    let _ = striped_random_natural_polynomials_with_degree(EXAMPLE_SEED, 0, 1, 2, 2, 1);
}
