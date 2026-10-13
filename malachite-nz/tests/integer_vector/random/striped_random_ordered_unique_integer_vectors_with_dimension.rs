// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::integer_vector::random::*;

#[track_caller]
fn striped_random_ordered_unique_integer_vectors_with_dimension_helper(
    dimension: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        striped_random_ordered_unique_integer_vectors_with_dimension(
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
fn test_striped_random_ordered_unique_integer_vectors_with_dimension() {
    // dimension 0: only the 0-dimensional vector
    striped_random_ordered_unique_integer_vectors_with_dimension_helper(
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
    striped_random_ordered_unique_integer_vectors_with_dimension_helper(
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
    striped_random_ordered_unique_integer_vectors_with_dimension_helper(
        3,
        64,
        1,
        8,
        1,
        &[
            "(2, 32, 2048)",
            "(-281474976710656, -8192, -255)",
            "(-4194303, -32, 4)",
            "(7, 31, 524288)",
            "(-128, -2, 0)",
            "(-68719476736, -2097151, -31)",
            "(-4, -3, 256)",
            "(-4096, -32, 2047)",
            "(8, 16, 524287)",
            "(3, 15, 16384)",
            "(-16, -2, 524288)",
            "(-2, 1, 16)",
            "(-64, 0, 2047)",
            "(-7, -3, 0)",
            "(-1, 0, 128)",
            "(-3, 127, 1048576)",
            "(0, 1, 256)",
            "(-1536, -2, 16)",
            "(-98303, 4, 8)",
            "(-524287, 139137, 196607)",
        ],
    );
    // dimension 5, mean stripe = 2, mean bits = 3/2: many repeats are redrawn
    striped_random_ordered_unique_integer_vectors_with_dimension_helper(
        5,
        2,
        1,
        3,
        2,
        &[
            "(-28, -1, 0, 1, 11)",
            "(-24, -3, -1, 0, 1)",
            "(-1, 0, 3, 14, 230)",
            "(-5, -1, 0, 1, 2)",
            "(-13, -8, -1, 0, 2)",
            "(-1, 0, 1, 2, 3)",
            "(-2, -1, 0, 1, 1658)",
            "(-5, 0, 1, 3, 6)",
            "(-573, -15, -1, 0, 1)",
            "(-1, 1, 2, 6, 18)",
            "(-2, -1, 0, 1, 3)",
            "(-4, -2, 1, 2, 11)",
            "(-1, 0, 1, 18, 48)",
            "(-4, -1, 0, 1, 3)",
            "(-20, -1, 1, 4, 6)",
            "(-23, -2, -1, 0, 1)",
            "(-5, -3, -1, 0, 3)",
            "(-2, -1, 0, 6, 14)",
            "(-97, -2, 1, 3, 25)",
            "(-49, -10, -1, 0, 1)",
        ],
    );
}

#[test]
#[should_panic]
fn striped_random_ordered_unique_integer_vectors_with_dimension_fail_1() {
    // mean_stripe_denominator is zero
    let _ =
        striped_random_ordered_unique_integer_vectors_with_dimension(EXAMPLE_SEED, 3, 1, 0, 4, 1);
}

#[test]
#[should_panic]
fn striped_random_ordered_unique_integer_vectors_with_dimension_fail_2() {
    // mean_stripe_numerator < mean_stripe_denominator
    let _ =
        striped_random_ordered_unique_integer_vectors_with_dimension(EXAMPLE_SEED, 3, 1, 2, 4, 1);
}

#[test]
#[should_panic]
fn striped_random_ordered_unique_integer_vectors_with_dimension_fail_3() {
    // mean_bits_denominator is zero
    let _ =
        striped_random_ordered_unique_integer_vectors_with_dimension(EXAMPLE_SEED, 3, 16, 1, 1, 0);
}
