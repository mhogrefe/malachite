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

fn striped_random_integer_polynomials_with_degree_helper(
    degree: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        striped_random_integer_polynomials_with_degree(
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
fn test_striped_random_integer_polynomials_with_degree() {
    // degree 0, mean stripe = 16, mean bits = 2
    striped_random_integer_polynomials_with_degree_helper(
        0,
        16,
        1,
        2,
        1,
        &[
            "1", "-48", "1", "1", "2", "1", "-3", "-1", "-1", "-3", "-1", "4", "-2", "-4", "-3",
            "-1", "-1", "-1", "1", "-1",
        ],
    );
    // degree 1, mean stripe = 16, mean bits = 2
    striped_random_integer_polynomials_with_degree_helper(
        1,
        16,
        1,
        2,
        1,
        &[
            "x", "-48*x+2", "x+2", "x+3", "2*x+14", "x-4", "-3*x+1", "-x-3", "-x", "-3*x+31",
            "-x+1", "4*x-1023", "-2*x-8", "-4*x+3", "-3*x", "-x-64", "-x+8", "-x-7", "x", "-x+1",
        ],
    );
    // degree 3, mean stripe = 4, mean bits = 4
    striped_random_integer_polynomials_with_degree_helper(
        3,
        4,
        1,
        4,
        1,
        &[
            "15*x^3+x^2-x-440",
            "-2*x^3+999*x^2+638*x+16",
            "2*x^3-15903*x^2+79*x-1",
            "75*x^3-99*x^2+32*x+485",
            "x^3-248*x^2-4",
            "x^3+4*x^2-1",
            "-5*x^3+15*x^2+8*x-1",
            "-33*x^3-x^2-40696",
            "-3*x^3+26*x-575",
            "-15*x^3+3*x^2-7*x",
            "-x^3+486*x^2+179*x-149",
            "8*x^3-55*x^2+x-375",
            "-x^3+x^2+x-20448",
            "-14*x^3+126*x^2-x-19",
            "-7*x^3-36*x^2-2032*x-1",
            "-4099*x^3-4220*x^2",
            "-24*x^3+511*x^2+1024*x+4",
            "-3*x^3-12*x^2+2*x+225",
            "51*x^3+8319*x^2+5*x-23544",
            "-9*x^3+x^2-x+2",
        ],
    );
}

#[test]
#[should_panic]
fn striped_random_integer_polynomials_with_degree_fail_1() {
    let _ = striped_random_integer_polynomials_with_degree(EXAMPLE_SEED, 0, 1, 2, 2, 1);
}
