// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_q::rational_vector::random::striped_random_rational_vectors_with_dimension;

fn striped_random_rational_vectors_with_dimension_helper(
    dimension: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        striped_random_rational_vectors_with_dimension(
            EXAMPLE_SEED,
            dimension,
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_bits_numerator,
            mean_bits_denominator
        )
        .take(20)
        .map(|v| v.to_string())
        .collect_vec(),
        expected_values
    );
}

#[test]
fn test_striped_random_rational_vectors_with_dimension() {
    // dimension 0: only the 0-dimensional vector
    striped_random_rational_vectors_with_dimension_helper(
        0,
        2,
        1,
        2,
        1,
        &[
            "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()",
            "()", "()", "()", "()", "()", "()",
        ],
    );
    // dimension 1, mean stripe = 8, mean bits = 4
    striped_random_rational_vectors_with_dimension_helper(
        1,
        8,
        1,
        4,
        1,
        &[
            "(0)",
            "(-1/3)",
            "(1)",
            "(1/64)",
            "(0)",
            "(1/2)",
            "(1/28)",
            "(16/3)",
            "(1/64)",
            "(19/62)",
            "(0)",
            "(-127/31)",
            "(1)",
            "(-8650499/7)",
            "(0)",
            "(-7/1017)",
            "(-903/64)",
            "(-8/7)",
            "(0)",
            "(0)",
        ],
    );
    // dimension 3, mean stripe = 64, mean bits = 8
    striped_random_rational_vectors_with_dimension_helper(
        3,
        64,
        1,
        8,
        1,
        &[
            "(-8/3, -32/3, 16)",
            "(1/32, 0, 1/65536)",
            "(2, 4681, 0)",
            "(511/1024, 65535/64, -268697599/255)",
            "(524287/15, 0, -1/31)",
            "(-7, -1/256, -262144/31)",
            "(0, -263/8191, -2048)",
            "(0, -2047/256, 0)",
            "(0, -16, -31/4096)",
            "(2048/15, -268435456/7, 1/4)",
            "(0, -2/5, 4/2047)",
            "(3/8, -1/31, -262144)",
            "(-63/16, 3/7, 1057)",
            "(-4311810305/65793, 127/9, 0)",
            "(-32767/64, -15/128, -16)",
            "(0, 31/16, 23/8)",
            "(-262144, -1023/2044, -63/4)",
            "(8195/8, -575/191, -511/65535)",
            "(67108863/2, 4095/36893488147419103232, -4)",
            "(-1/4194303, -31/7, 32)",
        ],
    );
}

#[test]
#[should_panic]
fn striped_random_rational_vectors_with_dimension_fail_1() {
    // mean_stripe_denominator is zero
    let _ = striped_random_rational_vectors_with_dimension(EXAMPLE_SEED, 3, 1, 0, 4, 1);
}

#[test]
#[should_panic]
fn striped_random_rational_vectors_with_dimension_fail_2() {
    // mean_stripe_numerator < mean_stripe_denominator
    let _ = striped_random_rational_vectors_with_dimension(EXAMPLE_SEED, 3, 1, 2, 4, 1);
}

#[test]
#[should_panic]
fn striped_random_rational_vectors_with_dimension_fail_3() {
    // mean_bits_denominator is zero
    let _ = striped_random_rational_vectors_with_dimension(EXAMPLE_SEED, 3, 16, 1, 1, 0);
}

#[test]
#[should_panic]
fn striped_random_rational_vectors_with_dimension_fail_4() {
    // mean_bits_numerator <= mean_bits_denominator
    let _ = striped_random_rational_vectors_with_dimension(EXAMPLE_SEED, 3, 16, 1, 1, 1);
}
