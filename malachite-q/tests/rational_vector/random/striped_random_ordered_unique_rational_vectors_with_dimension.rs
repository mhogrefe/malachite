// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_q::rational_vector::random::*;

#[track_caller]
fn striped_random_ordered_unique_rational_vectors_with_dimension_helper(
    dimension: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        striped_random_ordered_unique_rational_vectors_with_dimension(
            EXAMPLE_SEED,
            dimension,
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_bits_numerator,
            mean_bits_denominator
        )
        .take(20)
        .inspect(|v| assert!(v.elements.is_sorted_by(|x, y| x < y)))
        .map(|v| v.to_string())
        .collect_vec(),
        expected_values
    );
}

#[test]
fn test_striped_random_ordered_unique_rational_vectors_with_dimension() {
    // dimension 0: only the 0-dimensional vector
    striped_random_ordered_unique_rational_vectors_with_dimension_helper(
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
    striped_random_ordered_unique_rational_vectors_with_dimension_helper(
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
    striped_random_ordered_unique_rational_vectors_with_dimension_helper(
        3,
        64,
        1,
        8,
        1,
        &[
            "(-32/3, -8/3, 16)",
            "(0, 1/65536, 1/32)",
            "(0, 2, 4681)",
            "(-268697599/255, 511/1024, 65535/64)",
            "(-1/31, 0, 524287/15)",
            "(-262144/31, -7, -1/256)",
            "(-2048, -263/8191, 0)",
            "(-16, -2047/256, 0)",
            "(-268435456/7, -31/4096, 2048/15)",
            "(-2/5, 0, 1/4)",
            "(-1/31, 4/2047, 3/8)",
            "(-262144, -63/16, 3/7)",
            "(-4311810305/65793, 127/9, 1057)",
            "(-32767/64, -15/128, 0)",
            "(-16, 0, 31/16)",
            "(-262144, -1023/2044, 23/8)",
            "(-63/4, -575/191, 8195/8)",
            "(-511/65535, 4095/36893488147419103232, 67108863/2)",
            "(-31/7, -4, -1/4194303)",
            "(0, 16, 32)",
        ],
    );
    // dimension 5, mean stripe = 2, mean bits = 3/2: many repeats are redrawn
    striped_random_ordered_unique_rational_vectors_with_dimension_helper(
        5,
        2,
        1,
        3,
        2,
        &[
            "(0, 1/22, 1, 4, 8)",
            "(-5/3, -1, -1/3, 0, 1/3)",
            "(-31, -7, -1/3, 0, 203)",
            "(-4, -1, -1/3, 0, 17)",
            "(-1, 0, 1/13, 1/3, 1)",
            "(-6, -1, 0, 1, 20)",
            "(-37, -7, -6/7, 0, 5)",
            "(-5, -3, 0, 3/11, 1)",
            "(-807, -1, 0, 1, 5/2)",
            "(-23/3, -7, -5/2, -1, 0)",
            "(-24, -14, -3, -2, 0)",
            "(-2, 0, 2/7, 1/2, 6)",
            "(-3, -1, 0, 1, 2)",
            "(-9, -1, -9/11, 0, 116)",
            "(-16, -13, -1/3, 0, 2)",
            "(-2, -1, 0, 1/6, 8)",
            "(-3, -1, -1/3, 0, 38)",
            "(-10, -1, 0, 1, 3)",
            "(-2, -1, 1/3, 1, 5)",
            "(-3, -1, 0, 1, 6)",
        ],
    );
}

#[test]
#[should_panic]
fn striped_random_ordered_unique_rational_vectors_with_dimension_fail_1() {
    // mean_stripe_denominator is zero
    let _ =
        striped_random_ordered_unique_rational_vectors_with_dimension(EXAMPLE_SEED, 3, 1, 0, 4, 1);
}

#[test]
#[should_panic]
fn striped_random_ordered_unique_rational_vectors_with_dimension_fail_2() {
    // mean_stripe_numerator < mean_stripe_denominator
    let _ =
        striped_random_ordered_unique_rational_vectors_with_dimension(EXAMPLE_SEED, 3, 1, 2, 4, 1);
}

#[test]
#[should_panic]
fn striped_random_ordered_unique_rational_vectors_with_dimension_fail_3() {
    // mean_bits_denominator is zero
    let _ =
        striped_random_ordered_unique_rational_vectors_with_dimension(EXAMPLE_SEED, 3, 16, 1, 1, 0);
}

#[test]
#[should_panic]
fn striped_random_ordered_unique_rational_vectors_with_dimension_fail_4() {
    // mean_bits_numerator <= mean_bits_denominator
    let _ =
        striped_random_ordered_unique_rational_vectors_with_dimension(EXAMPLE_SEED, 3, 16, 1, 1, 1);
}
