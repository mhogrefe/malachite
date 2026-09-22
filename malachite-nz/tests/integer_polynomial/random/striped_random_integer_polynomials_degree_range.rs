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

fn striped_random_integer_polynomials_degree_range_helper(
    a: u64,
    b: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        striped_random_integer_polynomials_degree_range(
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
fn test_striped_random_integer_polynomials_degree_range() {
    // degrees in [0, 2), mean stripe = 16, mean bits = 2
    striped_random_integer_polynomials_degree_range_helper(
        0,
        2,
        16,
        1,
        2,
        1,
        &[
            "x", "-48", "x+2", "x+2", "2", "x+3", "-3*x+14", "-1", "-x-4", "-3*x+1", "-1", "4*x-3",
            "-2", "-4*x", "-3*x+31", "-1", "-1", "-1", "x+1", "-1",
        ],
    );
    // degrees in [1, 4), mean stripe = 16, mean bits = 2
    striped_random_integer_polynomials_degree_range_helper(
        1,
        4,
        16,
        1,
        2,
        1,
        &[
            "x^2+2*x",
            "-48*x^3+14*x^2+3*x+2",
            "x^2+x-4",
            "x^3+31*x^2-3",
            "2*x^3-8*x^2-1023*x+1",
            "x^2+3",
            "-3*x-64",
            "-x^2-7*x+8",
            "-x",
            "-3*x^3-4*x^2+x+1",
            "-x^3-4*x^2-3*x",
            "4*x^2-3",
            "-2*x^3+x^2-3584*x-31",
            "-4*x^2-x+61",
            "-3*x^3-x^2+1792*x+1",
            "-x^3-511*x^2+4*x",
            "-x^3+x^2+x-2",
            "-x-1",
            "x^3+2*x^2-x-2",
            "-x+4",
        ],
    );
    // degrees in [2, 5), mean stripe = 4, mean bits = 4
    striped_random_integer_polynomials_degree_range_helper(
        2,
        5,
        4,
        1,
        4,
        1,
        &[
            "15*x^3+x^2-x-440",
            "-2*x^4-x^3+999*x^2+638*x+16",
            "2*x^3+485*x^2-15903*x+79",
            "75*x^4-4*x^2-99*x+32",
            "x^4+4*x^3-x-248",
            "x^3+15*x^2+8*x-1",
            "-5*x^2-40696",
            "-33*x^3+26*x^2-575*x-1",
            "-3*x^2",
            "-15*x^4+179*x^3-149*x^2+3*x-7",
            "-x^4-55*x^3+x^2-375*x+486",
            "8*x^3+x^2+x-20448",
            "-x^4-x^3+126*x^2-x-19",
            "-14*x^3-36*x-2032",
            "-7*x^4+1024*x^3+4*x^2-4220*x",
            "-4099*x^4-12*x^3+2*x^2+225*x+511",
            "-24*x^4+2*x^3+8319*x^2+5*x-23544",
            "-3*x^2+x-1",
            "51*x^4+31744*x^3-27*x+16",
            "-9*x^2-193*x+7804",
        ],
    );
}

#[test]
#[should_panic]
fn striped_random_integer_polynomials_degree_range_fail_1() {
    let _ = striped_random_integer_polynomials_degree_range(EXAMPLE_SEED, 3, 3, 16, 1, 2, 1);
}
