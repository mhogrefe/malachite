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

fn striped_random_natural_polynomials_min_degree_helper(
    min_degree: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        striped_random_natural_polynomials_min_degree(
            EXAMPLE_SEED,
            min_degree,
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
fn test_striped_random_natural_polynomials_min_degree() {
    // min degree 0, mean stripe = 16, mean bits = 2, mean length = 2/1
    striped_random_natural_polynomials_min_degree_helper(
        0,
        16,
        1,
        2,
        1,
        2,
        1,
        &[
            "x+4",
            "2*x^2+x",
            "2*x^2+16*x+11",
            "3*x^2+3*x+8",
            "x",
            "x^2+15*x+7",
            "1",
            "1",
            "9*x",
            "1",
            "1",
            "1",
            "2*x",
            "x^4+x^3+3*x+1",
            "15",
            "3*x^2",
            "1",
            "x^5+2*x^4+3*x",
            "15*x+32",
            "3*x",
        ],
    );
    // min degree 2, mean stripe = 16, mean bits = 2, mean length = 4/1
    striped_random_natural_polynomials_min_degree_helper(
        2,
        16,
        1,
        2,
        1,
        4,
        1,
        &[
            "x^3+x^2+4",
            "2*x^4+3*x^3+8*x^2+16*x+11",
            "2*x^4+15*x^2+7*x",
            "3*x^4+3*x^2+x",
            "x^3+1",
            "x^4+3*x",
            "x^2+32*x+2",
            "x^2+3*x",
            "9*x^3+2",
            "x^2+15",
            "x^2",
            "x^2+x",
            "2*x^3+4*x+1",
            "x^6+31*x^5+15*x^3+x^2+x+3",
            "15*x^2+63*x+1",
            "3*x^4+7*x^3+126",
            "x^2+12*x",
            "x^7+x^5+5*x^3+2*x+1",
            "15*x^3+64*x^2+x",
            "3*x^3",
        ],
    );
    // min degree 3, mean stripe = 4, mean bits = 4, mean length = 5/1
    striped_random_natural_polynomials_min_degree_helper(
        3,
        4,
        1,
        4,
        1,
        5,
        1,
        &[
            "15*x^4+x^3+12380*x^2+2",
            "x^5+15*x^4+15*x^3+x+4",
            "6*x^5+x^3+x+3",
            "x^5+14*x^4+x^3+5*x^2+x",
            "20*x^4+2*x^3+7*x^2+1984*x+1",
            "20*x^5+257*x^4+87*x^2+2*x+47",
            "3*x^3+1921*x^2+7*x+8",
            "7*x^3+3",
            "3071*x^4+892*x^3+511*x^2+x+3",
            "9*x^3+x^2+1",
            "2*x^3+15631*x^2+38*x+7",
            "x^3+4*x",
            "14*x^4+x^3+10*x^2+6*x",
            "8*x^7+251*x^6+2*x^5+1568*x^4+1638*x+22",
            "7*x^3+16*x^2+19",
            "15*x^5+7*x^3+259*x^2+x+94",
            "4*x^3+7*x^2+14",
            "3*x^8+639*x^7+x^6+8*x^5+16*x^4+x^3+30*x^2+8687",
            "463*x^4+88*x^3+x^2+383*x+1",
            "1799*x^4+x^3+15*x^2+7*x",
        ],
    );
}

#[test]
#[should_panic]
fn striped_random_natural_polynomials_min_degree_fail_1() {
    let _ = striped_random_natural_polynomials_min_degree(EXAMPLE_SEED, 2, 16, 1, 2, 1, 2, 1);
}
