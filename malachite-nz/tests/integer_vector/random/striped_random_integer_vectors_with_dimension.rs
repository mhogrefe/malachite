// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::integer_vector::random::striped_random_integer_vectors_with_dimension;

fn striped_random_integer_vectors_with_dimension_helper(
    dimension: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        striped_random_integer_vectors_with_dimension(
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
fn test_striped_random_integer_vectors_with_dimension() {
    // dimension 0: only the 0-dimensional vector
    striped_random_integer_vectors_with_dimension_helper(
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
    striped_random_integer_vectors_with_dimension_helper(
        1,
        8,
        1,
        4,
        1,
        &[
            "(2)",
            "(128)",
            "(1)",
            "(0)",
            "(-56)",
            "(8160)",
            "(0)",
            "(31)",
            "(4296999934)",
            "(127)",
            "(-8)",
            "(-5119)",
            "(-1)",
            "(-8380670)",
            "(2032)",
            "(64)",
            "(-1)",
            "(-896)",
            "(-7)",
            "(-56)",
        ],
    );
    // dimension 3, mean stripe = 64, mean bits = 8
    striped_random_integer_vectors_with_dimension_helper(
        3,
        64,
        1,
        8,
        1,
        &[
            "(2048, 2, 32)",
            "(-281474976710656, -8192, -255)",
            "(4, -4194303, -32)",
            "(31, 7, 524288)",
            "(-2, 0, -128)",
            "(-2097151, -68719476736, -31)",
            "(-4, 256, -3)",
            "(2047, -4096, -32)",
            "(524287, 16, 8)",
            "(3, 15, 16384)",
            "(-2, 524288, -16)",
            "(16, -2, 1)",
            "(0, 2047, -64)",
            "(-3, -7, 0)",
            "(0, 128, -1)",
            "(1048576, 127, -3)",
            "(256, 0, 1)",
            "(-2, 16, -1536)",
            "(4, 8, -98303)",
            "(139137, -524287, 196607)",
        ],
    );
}

#[test]
#[should_panic]
fn striped_random_integer_vectors_with_dimension_fail_1() {
    // mean_stripe_denominator is zero
    let _ = striped_random_integer_vectors_with_dimension(EXAMPLE_SEED, 3, 1, 0, 4, 1);
}

#[test]
#[should_panic]
fn striped_random_integer_vectors_with_dimension_fail_2() {
    // mean_stripe_numerator < mean_stripe_denominator
    let _ = striped_random_integer_vectors_with_dimension(EXAMPLE_SEED, 3, 1, 2, 4, 1);
}

#[test]
#[should_panic]
fn striped_random_integer_vectors_with_dimension_fail_3() {
    // mean_bits_denominator is zero
    let _ = striped_random_integer_vectors_with_dimension(EXAMPLE_SEED, 3, 16, 1, 1, 0);
}
