// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::natural_vector::random::*;

#[track_caller]
fn striped_random_ordered_unique_natural_vectors_with_dimension_helper(
    dimension: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        striped_random_ordered_unique_natural_vectors_with_dimension(
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
fn test_striped_random_ordered_unique_natural_vectors_with_dimension() {
    // dimension 0: only the 0-dimensional vector
    striped_random_ordered_unique_natural_vectors_with_dimension_helper(
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
    striped_random_ordered_unique_natural_vectors_with_dimension_helper(
        1,
        8,
        1,
        4,
        1,
        &[
            "(8)", "(8)", "(8)", "(1)", "(1)", "(0)", "(1)", "(252)", "(11)", "(64)", "(1)", "(3)",
            "(2)", "(4)", "(1)", "(5)", "(256)", "(3)", "(8)", "(1)",
        ],
    );
    // dimension 3, mean stripe = 64, mean bits = 8
    striped_random_ordered_unique_natural_vectors_with_dimension_helper(
        3,
        64,
        1,
        8,
        1,
        &[
            "(1, 4, 128)",
            "(3, 4, 268435456)",
            "(16, 128, 511)",
            "(1, 16, 134217727)",
            "(0, 1, 262144)",
            "(0, 4, 7)",
            "(1, 16383, 4503599627370496)",
            "(3, 511, 4096)",
            "(512, 1023, 72057594037927936)",
            "(1, 3, 33554432)",
            "(0, 2, 512)",
            "(4, 32768, 1073741824)",
            "(2, 16, 1048575)",
            "(0, 64, 1048703)",
            "(9, 12, 127)",
            "(0, 8, 9)",
            "(2, 7, 262140)",
            "(64, 2044, 16384)",
            "(0, 62914560, 4294967294)",
            "(4095, 1310719, 4294983679)",
        ],
    );
    // dimension 5, mean stripe = 2, mean bits = 3/2: many repeats are redrawn
    striped_random_ordered_unique_natural_vectors_with_dimension_helper(
        5,
        2,
        1,
        3,
        2,
        &[
            "(0, 1, 2, 12, 14)",
            "(0, 1, 2, 3, 30)",
            "(0, 1, 2, 7, 9)",
            "(0, 1, 2, 14, 69)",
            "(0, 1, 4, 12, 26)",
            "(0, 2, 3, 14, 1433)",
            "(0, 1, 3, 47, 60)",
            "(0, 1, 6, 34, 46)",
            "(0, 1, 7, 9, 117)",
            "(0, 1, 2, 14, 16)",
            "(0, 2, 6, 10, 55)",
            "(0, 2, 3, 6, 58)",
            "(0, 3, 7, 13, 182)",
            "(0, 2, 3, 7, 118)",
            "(0, 1, 2, 7, 26)",
            "(0, 1, 7, 11, 13)",
            "(0, 1, 2, 11, 36)",
            "(0, 1, 3, 15, 36)",
            "(0, 1, 22, 54, 123)",
            "(0, 1, 2, 3, 358)",
        ],
    );
}

#[test]
#[should_panic]
fn striped_random_ordered_unique_natural_vectors_with_dimension_fail_1() {
    // mean_stripe_denominator is zero
    let _ =
        striped_random_ordered_unique_natural_vectors_with_dimension(EXAMPLE_SEED, 3, 1, 0, 4, 1);
}

#[test]
#[should_panic]
fn striped_random_ordered_unique_natural_vectors_with_dimension_fail_2() {
    // mean_stripe_numerator < mean_stripe_denominator
    let _ =
        striped_random_ordered_unique_natural_vectors_with_dimension(EXAMPLE_SEED, 3, 1, 2, 4, 1);
}

#[test]
#[should_panic]
fn striped_random_ordered_unique_natural_vectors_with_dimension_fail_3() {
    // mean_bits_denominator is zero
    let _ =
        striped_random_ordered_unique_natural_vectors_with_dimension(EXAMPLE_SEED, 3, 16, 1, 1, 0);
}
