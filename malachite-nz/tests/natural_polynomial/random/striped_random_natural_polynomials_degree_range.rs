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

fn striped_random_natural_polynomials_degree_range_helper(
    a: u64,
    b: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        striped_random_natural_polynomials_degree_range(
            EXAMPLE_SEED,
            a,
            b,
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
fn test_striped_random_natural_polynomials_degree_range() {
    // degrees in [0, 2), mean stripe = 16, mean bits = 2
    striped_random_natural_polynomials_degree_range_helper(
        0,
        2,
        16,
        1,
        2,
        1,
        &[
            "x+4", "2", "2*x", "3*x+1", "1", "x+11", "x+16", "1", "9*x+8", "x+3", "1", "x", "2",
            "x+7", "15*x+15", "3", "1", "1", "15*x", "3",
        ],
    );
    // degrees in [1, 4), mean stripe = 16, mean bits = 2
    striped_random_natural_polynomials_degree_range_helper(
        1,
        4,
        16,
        1,
        2,
        1,
        &[
            "x^2+4",
            "2*x^3+16*x^2+11*x+1",
            "2*x^2+3*x+8",
            "3*x^3+15*x^2+7*x",
            "x^3+x^2",
            "x^2+3",
            "x+1",
            "x^2",
            "9*x",
            "x^3+3",
            "x^3+32*x+2",
            "x^2+2*x+3",
            "2*x^3+15*x^2",
            "x^2",
            "15*x^3+x^2",
            "3*x^3+4*x+1",
            "x^3+x^2+x+3",
            "x+15",
            "15*x^3+x^2+31*x",
            "3*x+63",
        ],
    );
    // degrees in [2, 5), mean stripe = 4, mean bits = 4
    striped_random_natural_polynomials_degree_range_helper(
        2,
        5,
        4,
        1,
        4,
        1,
        &[
            "15*x^3+12380*x^2+2",
            "x^4+x^2+4*x+1",
            "6*x^3+3*x^2+15*x+15",
            "x^4+x^2+1",
            "20*x^4+x^3+5*x^2+x",
            "20*x^3+1984*x^2+x+14",
            "3*x^2+2*x+7",
            "7*x^3+87*x^2+2*x+47",
            "3071*x^2+257*x",
            "9*x^4+3*x^3+1921*x^2+7*x+8",
            "2*x^4+x^3+3*x^2",
            "x^3+x^2+892*x+511",
            "14*x^4+38*x^3+7*x^2+x",
            "8*x^3+4*x^2+15631",
            "7*x^4+10*x^3+6*x^2",
            "15*x^4+1638*x^2+22*x+1",
            "4*x^4+251*x^3+2*x^2+1568*x",
            "3*x^2+19",
            "463*x^4+259*x^3+x^2+94*x+16",
            "1799*x^2+7",
        ],
    );
}

#[test]
#[should_panic]
fn striped_random_natural_polynomials_degree_range_fail_1() {
    let _ = striped_random_natural_polynomials_degree_range(EXAMPLE_SEED, 3, 3, 16, 1, 2, 1);
}
