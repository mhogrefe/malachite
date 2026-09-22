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

fn striped_random_rational_polynomials_min_degree_helper(
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
        striped_random_rational_polynomials_min_degree(
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
fn test_striped_random_rational_polynomials_min_degree() {
    // min degree 0, mean stripe = 16, mean bits = 2, mean length = 2/1
    striped_random_rational_polynomials_min_degree_helper(
        0,
        16,
        1,
        2,
        1,
        2,
        1,
        &[
            "-1/15*x",
            "x^2-4*x",
            "-x^2+3",
            "7807*x^2",
            "-1/4*x-7/16",
            "-3*x^2-1/7*x",
            "-1",
            "-1/3",
            "1/2*x+1",
            "4/31",
            "-1",
            "1",
            "-15*x",
            "-3*x^4+x",
            "1",
            "-6*x^2+31/2",
            "1/4",
            "-3*x^5-1/2*x^4-x^3-15*x^2+1",
            "x+1/3",
            "-3*x+7/3",
        ],
    );
    // min degree 2, mean stripe = 16, mean bits = 2, mean length = 4/1
    striped_random_rational_polynomials_min_degree_helper(
        2,
        16,
        1,
        2,
        1,
        4,
        1,
        &[
            "-1/15*x^3-4*x^2",
            "x^4+3",
            "-x^4+x^3-1/7*x^2-7/16",
            "7807*x^4+x^2",
            "-1/4*x^3+31/2*x",
            "-3*x^4-x^3-15*x^2+1",
            "-x^2+1/3*x-1/2",
            "-1/3*x^2+1/2*x+7/3",
            "1/2*x^3+31*x^2+x-7",
            "4/31*x^2",
            "-x^2-31759/8*x-2/3",
            "x^2-48/7*x+15/2",
            "-15*x^3-x-63",
            "-3*x^6-8*x^5-3/16*x^4-127*x^3+x^2+x+255/2",
            "x^2+4/3*x+14",
            "-6*x^4-1/2*x-31/2",
            "1/4*x^2+15*x+1/7",
            "-3*x^7+5/31*x^6-1792*x^4-3*x^3+x^2-2*x-32",
            "x^3-15/2*x^2+512*x+31/3",
            "-3*x^3-4*x^2+1/2*x+131/3",
        ],
    );
    // min degree 3, mean stripe = 4, mean bits = 4, mean length = 5/1
    striped_random_rational_polynomials_min_degree_helper(
        3,
        4,
        1,
        4,
        1,
        5,
        1,
        &[
            "-4*x^4-3/95*x^2+x",
            "85/2*x^5+1535/239*x^4-1/4*x^3-13/775*x^2+7/88*x-7/128",
            "-1/3*x^5+9/2*x^4+x-1/5",
            "63*x^5-130*x^3+925547/16*x^2+x+1/8",
            "-11*x^4-8*x^3-15*x^2-3147902/3*x-11/12",
            "-29*x^5+44*x^4-121/28*x^3+2999/4*x^2",
            "-3*x^3-44/28679*x+35/2",
            "-1247/37*x^3+19/8*x^2-11/20*x-1",
            "3/4*x^4-1023*x^3-1/3",
            "4*x^3+4*x^2+1/16*x",
            "-380/49*x^3-2/3*x^2-55/2*x-12",
            "x^3-1/4*x^2+3/79*x+16/33",
            "-17/2*x^4+2/11*x^3+7/3*x^2+7/75*x-32",
            "-3*x^7+x^6-1286/5*x^5-31/5*x^4+3/19*x^3-2/3*x^2-1/7*x+7/1075",
            "x^3+2*x^2+104/3*x+1/23",
            "-1/12*x^5+4440550*x^4-4/7*x^3-191",
            "2/141*x^3-31/23*x^2-82/11*x-2",
            "-x^8-2074*x^7-74*x^6+514/69*x^5-1/2*x^4+64/3*x^3+5*x^2+3161245",
            "65/3*x^4-2/7*x^3+1/2*x",
            "-21/41*x^4-2*x^3-137239/16*x^2-4*x+15/12349",
        ],
    );
}

#[test]
#[should_panic]
fn striped_random_rational_polynomials_min_degree_fail_1() {
    let _ = striped_random_rational_polynomials_min_degree(EXAMPLE_SEED, 2, 16, 1, 2, 1, 2, 1);
}
