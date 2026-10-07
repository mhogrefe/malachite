// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::natural_vector::random::striped_random_natural_vectors_with_dimension;

fn striped_random_natural_vectors_with_dimension_helper(
    dimension: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        striped_random_natural_vectors_with_dimension(
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
fn test_striped_random_natural_vectors_with_dimension() {
    // dimension 0: only the 0-dimensional vector
    striped_random_natural_vectors_with_dimension_helper(
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
    striped_random_natural_vectors_with_dimension_helper(
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
    striped_random_natural_vectors_with_dimension_helper(
        3,
        64,
        1,
        8,
        1,
        &[
            "(1, 4, 128)",
            "(268435456, 4, 3)",
            "(16, 511, 128)",
            "(134217727, 1, 16)",
            "(0, 1, 262144)",
            "(7, 0, 4)",
            "(16383, 4503599627370496, 1)",
            "(3, 511, 4096)",
            "(72057594037927936, 1023, 512)",
            "(33554432, 3, 1)",
            "(0, 512, 0)",
            "(2, 1073741824, 32768)",
            "(4, 16, 2)",
            "(1048575, 64, 0)",
            "(1048703, 127, 12)",
            "(9, 0, 8)",
            "(0, 9, 7)",
            "(262140, 2, 16384)",
            "(64, 2044, 0)",
            "(4294967294, 62914560, 4095)",
        ],
    );
}

#[test]
#[should_panic]
fn striped_random_natural_vectors_with_dimension_fail_1() {
    // mean_stripe_denominator is zero
    let _ = striped_random_natural_vectors_with_dimension(EXAMPLE_SEED, 3, 1, 0, 4, 1);
}

#[test]
#[should_panic]
fn striped_random_natural_vectors_with_dimension_fail_2() {
    // mean_stripe_numerator < mean_stripe_denominator
    let _ = striped_random_natural_vectors_with_dimension(EXAMPLE_SEED, 3, 1, 2, 4, 1);
}

#[test]
#[should_panic]
fn striped_random_natural_vectors_with_dimension_fail_3() {
    // mean_bits_denominator is zero
    let _ = striped_random_natural_vectors_with_dimension(EXAMPLE_SEED, 3, 16, 1, 1, 0);
}
