// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_q::rational_polynomial::random::*;

fn striped_random_rational_polynomials_with_degree_helper(
    degree: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        striped_random_rational_polynomials_with_degree(
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
fn test_striped_random_rational_polynomials_with_degree() {
    // degree 0, mean stripe = 16, mean bits = 2
    striped_random_rational_polynomials_with_degree_helper(
        0,
        16,
        1,
        2,
        1,
        &[
            "-1/15", "1", "-1", "7807", "-1/4", "-3", "-1", "-1/3", "1/2", "4/31", "-1", "1",
            "-15", "-3", "1", "-6", "1/4", "-3", "1", "-3",
        ],
    );
    // degree 1, mean stripe = 16, mean bits = 2
    striped_random_rational_polynomials_with_degree_helper(
        1,
        16,
        1,
        2,
        1,
        &[
            "-1/15*x",
            "x",
            "-x-4",
            "7807*x+3",
            "-1/4*x",
            "-3*x",
            "-x",
            "-1/3*x-7/16",
            "1/2*x",
            "4/31*x-1/7",
            "-x+1",
            "x",
            "-15*x",
            "-3*x+1",
            "x",
            "-6*x",
            "1/4*x+31/2",
            "-3*x",
            "x+1",
            "-3*x",
        ],
    );
    // degree 3, mean stripe = 4, mean bits = 4
    striped_random_rational_polynomials_with_degree_helper(
        3,
        4,
        1,
        4,
        1,
        &[
            "-4*x^3-3/95*x^2+x",
            "85/2*x^3+7/88*x^2-7/128*x",
            "-1/3*x^3+1535/239*x^2-1/4*x-13/775",
            "63*x^3+x-1/5",
            "-11*x^3+1/8*x^2+9/2*x",
            "-29*x^3-130*x^2+925547/16*x+1",
            "-3*x^3-3147902/3*x^2-11/12*x",
            "-1247/37*x^3-8*x-15",
            "3/4*x^3-121/28*x^2+2999/4*x",
            "4*x^3-44/28679*x^2+35/2*x+44",
            "-380/49*x^3-11/20*x^2-x",
            "x^3-1/3*x+19/8",
            "-17/2*x^3-1023*x",
            "-3*x^3-12*x^2+4*x+1/16",
            "x^3+16/33*x^2-2/3*x-55/2",
            "-1/12*x^3-32*x^2-1/4*x+3/79",
            "2/141*x^3+2/11*x^2+7/3*x+7/75",
            "-x^3-2/3*x^2-1/7*x+7/1075",
            "65/3*x^3-1286/5*x^2-31/5*x+3/19",
            "-21/41*x^3+104/3*x^2+1/23*x+1",
        ],
    );
}

#[test]
#[should_panic]
fn striped_random_rational_polynomials_with_degree_fail_1() {
    let _ = striped_random_rational_polynomials_with_degree(EXAMPLE_SEED, 0, 1, 2, 2, 1);
}
