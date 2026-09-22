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

fn striped_random_rational_polynomials_helper(
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        striped_random_rational_polynomials(
            EXAMPLE_SEED,
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
fn test_striped_random_rational_polynomials() {
    // mean stripe = 4, mean bits = 2, mean length = 2/1
    striped_random_rational_polynomials_helper(
        4,
        1,
        2,
        1,
        2,
        1,
        &[
            "-1/15*x^5+3*x^3-4*x^2",
            "1",
            "-x^7+x^5-1/7*x^4-7/16*x^2",
            "8063",
            "-1/4*x^13+7/3*x^12+1/3*x^11-1/2*x^10-x^9-15*x^8+x^6+29/2*x^4+x",
            "0",
            "-3*x^4+27*x^3+x^2-7*x+1/2",
            "-x^3-2/3*x^2",
            "-1/3",
            "0",
            "1/2*x^5-x^4-33*x^3-60/7*x^2+9/2*x-5659/4",
            "4/21*x",
            "0",
            "0",
            "-1",
            "x^2+x+233/2",
            "0",
            "-11",
            "0",
            "-3",
        ],
    );
    // mean stripe = 16, mean bits = 2, mean length = 2/1
    striped_random_rational_polynomials_helper(
        16,
        1,
        2,
        1,
        2,
        1,
        &[
            "-1/15*x^5+3*x^3-4*x^2",
            "1",
            "-x^7+x^5-1/7*x^4-7/16*x^2",
            "7807",
            "-1/4*x^13+7/3*x^12+1/3*x^11-1/2*x^10-x^9-15*x^8+x^6+31/2*x^4+x",
            "0",
            "-3*x^4+31*x^3+x^2-7*x+1/2",
            "-x^3-2/3*x^2",
            "-1/3",
            "0",
            "1/2*x^5-x^4-63*x^3-48/7*x^2+15/2*x-31759/8",
            "4/31*x",
            "0",
            "0",
            "-1",
            "x^2+x+255/2",
            "0",
            "-15",
            "0",
            "-3",
        ],
    );
    // mean stripe = 16, mean bits = 4, mean length = 3/1
    striped_random_rational_polynomials_helper(
        16,
        1,
        4,
        1,
        3,
        1,
        &[
            "-4*x^7-5/341*x^6+1/16*x^5-7/128*x^4-3/127*x^2+x",
            "85/2",
            "-1/3*x^10+1048323/28*x^9+x^8+1/14*x^7+4*x^6+x^3-1/7*x^2+363/85*x-1/4",
            "63",
            "-15*x^17+7/2*x^16-14/15*x^15-x^14-32/16639*x^12+63/2*x^11+62*x^10-79/16*x^9+4095/4*\
            x^8-8*x^5-15*x^4-2162672/3*x^3-15/8*x^2-128",
            "0",
            "-67/3*x^6+1/16*x^5-1023*x^3-1/3",
            "-5*x^5+15/16*x^4-2/3*x^3-33/2*x^2-8*x+4",
            "-1025/33",
            "3/4",
            "4*x^7+7/2047*x^6+2/15*x^5+7/3*x^4+7/127*x^3-32*x^2-1/4*x+3/95",
            "-256/63*x-1/7",
            "0",
            "0",
            "1",
            "-31/2*x^3-19/7*x^2+15/127*x-2/3",
            "-3*x-1024/7",
            "1",
            "0",
            "-1/14*x+1",
        ],
    );
}

#[test]
#[should_panic]
fn striped_random_rational_polynomials_fail_1() {
    let _ = striped_random_rational_polynomials(EXAMPLE_SEED, 1, 2, 2, 1, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_rational_polynomials_fail_2() {
    let _ = striped_random_rational_polynomials(EXAMPLE_SEED, 4, 1, 1, 0, 2, 1);
}
