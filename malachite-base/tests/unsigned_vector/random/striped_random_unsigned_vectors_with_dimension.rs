// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::unsigned_vector::random::striped_random_unsigned_vectors_with_dimension;

fn striped_random_unsigned_vectors_with_dimension_helper(
    dimension: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        striped_random_unsigned_vectors_with_dimension::<u8>(
            EXAMPLE_SEED,
            dimension,
            mean_stripe_numerator,
            mean_stripe_denominator,
        )
        .take(20)
        .map(|v| v.to_string())
        .collect_vec(),
        expected_values
    );
}

#[test]
fn test_striped_random_unsigned_vectors_with_dimension() {
    // dimension 0: only the 0-dimensional vector
    striped_random_unsigned_vectors_with_dimension_helper(
        0,
        4,
        1,
        &[
            "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()",
            "()", "()", "()", "()", "()", "()",
        ],
    );
    // dimension 1, mean stripe length = 4
    striped_random_unsigned_vectors_with_dimension_helper(
        1,
        4,
        1,
        &[
            "(1)", "(76)", "(127)", "(195)", "(0)", "(128)", "(15)", "(118)", "(0)", "(248)",
            "(255)", "(253)", "(121)", "(0)", "(240)", "(3)", "(0)", "(127)", "(1)", "(0)",
        ],
    );
    // dimension 3, mean stripe length = 4
    striped_random_unsigned_vectors_with_dimension_helper(
        3,
        4,
        1,
        &[
            "(1, 76, 127)",
            "(195, 0, 128)",
            "(15, 118, 0)",
            "(248, 255, 253)",
            "(121, 0, 240)",
            "(3, 0, 127)",
            "(1, 0, 211)",
            "(135, 7, 121)",
            "(176, 30, 127)",
            "(255, 244, 63)",
            "(113, 227, 30)",
            "(255, 241, 121)",
            "(255, 0, 137)",
            "(0, 231, 224)",
            "(255, 0, 252)",
            "(240, 64, 241)",
            "(224, 255, 236)",
            "(63, 152, 156)",
            "(231, 240, 23)",
            "(255, 194, 127)",
        ],
    );
    // dimension 3, mean stripe length = 16
    striped_random_unsigned_vectors_with_dimension_helper(
        3,
        16,
        1,
        &[
            "(15, 0, 0)",
            "(252, 0, 254)",
            "(0, 0, 0)",
            "(255, 192, 255)",
            "(127, 0, 224)",
            "(0, 0, 15)",
            "(0, 0, 255)",
            "(255, 0, 127)",
            "(255, 63, 0)",
            "(255, 192, 15)",
            "(15, 255, 0)",
            "(254, 255, 1)",
            "(255, 3, 224)",
            "(0, 255, 255)",
            "(255, 1, 248)",
            "(255, 0, 255)",
            "(255, 255, 255)",
            "(0, 255, 192)",
            "(248, 224, 0)",
            "(255, 252, 0)",
        ],
    );
}

#[test]
#[should_panic]
fn striped_random_unsigned_vectors_with_dimension_fail_1() {
    // mean_stripe_denominator is zero
    let _ = striped_random_unsigned_vectors_with_dimension::<u8>(EXAMPLE_SEED, 3, 4, 0);
}

#[test]
#[should_panic]
fn striped_random_unsigned_vectors_with_dimension_fail_2() {
    // mean_stripe_numerator < mean_stripe_denominator
    let _ = striped_random_unsigned_vectors_with_dimension::<u8>(EXAMPLE_SEED, 3, 1, 2);
}
