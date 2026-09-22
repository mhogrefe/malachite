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

fn striped_random_integer_polynomials_min_degree_helper(
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
        striped_random_integer_polynomials_min_degree(
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
fn test_striped_random_integer_polynomials_min_degree() {
    // min degree 0, mean stripe = 16, mean bits = 2, mean length = 2/1
    striped_random_integer_polynomials_min_degree_helper(
        0,
        16,
        1,
        2,
        1,
        2,
        1,
        &[
            "x",
            "-48*x^2+2*x+2",
            "x^2+14*x+3",
            "x^2+x-4",
            "2*x-3",
            "x^2+31*x",
            "-3",
            "-1",
            "-x+1",
            "-3",
            "-1",
            "4",
            "-2*x-1023",
            "-4*x^4-64*x^3+3*x-8",
            "-3",
            "-x^2-7*x+8",
            "-1",
            "-x^5-4*x^3+x^2+x",
            "x-3",
            "-x-4",
        ],
    );
    // min degree 2, mean stripe = 16, mean bits = 2, mean length = 4/1
    striped_random_integer_polynomials_min_degree_helper(
        2,
        16,
        1,
        2,
        1,
        4,
        1,
        &[
            "x^3+2*x^2+2*x",
            "-48*x^4+x^3-4*x^2+14*x+3",
            "x^4+x^3+31*x^2-3",
            "x^4+3*x^2-8*x-1023",
            "2*x^3-7*x^2+8*x-64",
            "x^4-4*x^3+x^2+x",
            "-3*x^2-3*x",
            "-x^2-3*x-4",
            "-x^3-3584*x^2-31*x",
            "-3*x^2+61*x+1",
            "-x^2+x-1",
            "4*x^2-x+1792",
            "-2*x^3-511*x^2+4*x",
            "-4*x^6-x^5-2*x^4-x^3+x^2+x-2",
            "-3*x^2+4*x+2",
            "-x^4-x^2",
            "-x^2+2*x+64",
            "-x^7-x^5+4*x^4-2*x^3-7*x^2+3*x+8",
            "x^3-63*x^2+3",
            "-x^3-7*x^2-15*x+1",
        ],
    );
    // min degree 3, mean stripe = 4, mean bits = 4, mean length = 5/1
    striped_random_integer_polynomials_min_degree_helper(
        3,
        4,
        1,
        4,
        1,
        5,
        1,
        &[
            "15*x^4+16*x^3+x^2-x-440",
            "-2*x^5-15903*x^4+79*x^3-x^2+999*x+638",
            "2*x^5-4*x^3-99*x^2+32*x+485",
            "75*x^5-x^4+4*x^3-x-248",
            "x^4-40696*x^2+15*x+8",
            "x^5+26*x^2-575*x-1",
            "-5*x^3-149*x^2+3*x-7",
            "-33*x^3-375*x^2+486*x+179",
            "-3*x^4+x^3-20448*x^2-55*x+1",
            "-15*x^3-x^2-19*x+1",
            "-x^3-2032*x^2-x+126",
            "8*x^3-36",
            "-x^4+511*x^3+1024*x^2+4*x-4220",
            "-14*x^7+2*x^6+8319*x^5+5*x^4-23544*x^3-12*x^2+2*x+225",
            "-7*x^3+16*x^2+x-1",
            "-4099*x^5-193*x^4+7804*x^3+31744*x^2-27",
            "-24*x^3-1023*x^2-487*x",
            "-3*x^8-50*x^7+x^6-x^5+7*x^4-7*x^3-16*x^2+3*x+35",
            "51*x^4-14*x^2+104*x-8",
            "-9*x^4+7*x^3-4*x^2-2*x",
        ],
    );
}

#[test]
#[should_panic]
fn striped_random_integer_polynomials_min_degree_fail_1() {
    let _ = striped_random_integer_polynomials_min_degree(EXAMPLE_SEED, 2, 16, 1, 2, 1, 2, 1);
}
