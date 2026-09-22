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

fn striped_random_rational_polynomials_degree_inclusive_range_helper(
    a: u64,
    b: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        striped_random_rational_polynomials_degree_inclusive_range(
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
fn test_striped_random_rational_polynomials_degree_inclusive_range() {
    // degrees in [0, 1], mean stripe = 16, mean bits = 2
    striped_random_rational_polynomials_degree_inclusive_range_helper(
        0,
        1,
        16,
        1,
        2,
        1,
        &[
            "-1/15*x", "1", "-x", "7807*x-4", "-1/4", "-3*x+3", "-x", "-1/3", "1/2*x", "4/31*x",
            "-1", "x-7/16", "-15", "-3*x", "x-1/7", "-6", "1/4", "-3", "x+1", "-3",
        ],
    );
    // degrees in [1, 3], mean stripe = 16, mean bits = 2
    striped_random_rational_polynomials_degree_inclusive_range_helper(
        1,
        3,
        16,
        1,
        2,
        1,
        &[
            "-1/15*x^2",
            "x^3+3*x-4",
            "-x^2",
            "7807*x^3-1/7*x^2-7/16",
            "-1/4*x^3+1",
            "-3*x^2+1",
            "-x",
            "-1/3*x^2+31/2",
            "1/2*x+1",
            "4/31*x^3-x^2-15*x",
            "-x^3+7/3*x^2+1/3*x-1/2",
            "x^2-7*x+1/2",
            "-15*x^3+31*x+1",
            "-3*x^2-2/3*x",
            "x^3-48/7*x^2+15/2*x-31759/8",
            "-6*x^3-x-63",
            "1/4*x^3+x^2+x+255/2",
            "-3*x-127",
            "x^3+14*x^2-8*x-3/16",
            "-3*x+4/3",
        ],
    );
    // degrees in [2, 4], mean stripe = 4, mean bits = 4
    striped_random_rational_polynomials_degree_inclusive_range_helper(
        2,
        4,
        4,
        1,
        4,
        1,
        &[
            "-4*x^3-3/95*x^2+x",
            "85/2*x^4-13/775*x^3+7/88*x^2-7/128*x",
            "-1/3*x^3-1/5*x^2+1535/239*x-1/4",
            "63*x^4+9/2*x^3+1",
            "-11*x^4-130*x^3+925547/16*x^2+x+1/8",
            "-29*x^3-3147902/3*x^2-11/12*x",
            "-3*x^2-8*x-15",
            "-1247/37*x^3+2999/4*x^2",
            "3/4*x^2+44*x-121/28",
            "4*x^4-x^3-44/28679*x+35/2",
            "-380/49*x^4-1/3*x^2+19/8*x-11/20",
            "x^3-1023*x",
            "-17/2*x^4-55/2*x^3-12*x^2+4*x+1/16",
            "-3*x^3+3/79*x^2+16/33*x-2/3",
            "x^4+7/3*x^3+7/75*x^2-32*x-1/4",
            "-1/12*x^4-2/3*x^3-1/7*x^2+7/1075*x+2/11",
            "2/141*x^4+x^3-1286/5*x^2-31/5*x+3/19",
            "-x^2+104/3*x+1/23",
            "65/3*x^4-191*x+2",
            "-21/41*x^2+4440550*x-4/7",
        ],
    );
}

#[test]
#[should_panic]
fn striped_random_rational_polynomials_degree_inclusive_range_fail_1() {
    let _ =
        striped_random_rational_polynomials_degree_inclusive_range(EXAMPLE_SEED, 3, 2, 16, 1, 2, 1);
}
