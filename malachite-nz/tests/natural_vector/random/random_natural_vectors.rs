// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::natural_vector::random::random_natural_vectors;

fn random_natural_vectors_helper(
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        random_natural_vectors(
            EXAMPLE_SEED,
            mean_bits_numerator,
            mean_bits_denominator,
            mean_length_numerator,
            mean_length_denominator
        )
        .take(20)
        .map(|v| v.to_string())
        .collect_vec(),
        expected_values
    );
}

#[test]
fn test_random_natural_vectors() {
    // mean bits = 1, mean dimension = 1
    random_natural_vectors_helper(
        1,
        1,
        1,
        1,
        &[
            "(1)",
            "(0, 11)",
            "(0, 0)",
            "(0, 0)",
            "(5)",
            "(0, 0)",
            "()",
            "()",
            "(0)",
            "()",
            "()",
            "()",
            "(3)",
            "(3, 15, 3, 0)",
            "()",
            "(1, 1)",
            "()",
            "(1, 0, 7, 1, 0)",
            "(1)",
            "(11)",
        ],
    );
    // mean bits = 2, mean dimension = 2
    random_natural_vectors_helper(
        2,
        1,
        2,
        1,
        &[
            "(5, 0, 1, 13, 19, 15)",
            "(3)",
            "(0, 5, 10, 0, 0, 1, 2, 0)",
            "(1)",
            "(0, 0, 0, 3, 0, 0, 3, 35, 0, 2, 3, 0, 0, 12)",
            "()",
            "(0, 0, 0, 0, 1)",
            "(1, 4, 0, 3)",
            "(1)",
            "()",
            "(1, 10, 0, 18, 1, 62)",
            "(79, 0)",
            "()",
            "()",
            "(0)",
            "(4, 0, 15)",
            "()",
            "(1)",
            "()",
            "(3)",
        ],
    );
    // mean bits = 4, mean dimension = 4
    random_natural_vectors_helper(
        4,
        1,
        4,
        1,
        &[
            "()",
            "(3, 0, 13235, 1, 7, 1, 0, 15, 13, 2, 1, 0, 1, 0)",
            "(0, 1, 7, 1)",
            "(11, 1, 1523, 4)",
            "(2)",
            "()",
            "(35, 2, 117, 0, 391)",
            "(12, 6)",
            "(1394, 2, 0, 0)",
            "()",
            "(2, 1, 280, 1023, 1, 0)",
            "()",
            "()",
            "(1, 6, 52, 8210, 0, 4, 0, 0, 4, 12, 1)",
            "(23, 1947, 0, 0, 1866, 2, 203, 26)",
            "()",
            "(0, 29, 68)",
            "()",
            "(1, 330, 7, 0, 11)",
            "(0, 6, 16074, 0, 25, 1, 21, 15, 1)",
        ],
    );
}

#[test]
#[should_panic]
fn random_natural_vectors_fail_1() {
    // mean_bits_denominator is zero
    let _ = random_natural_vectors(EXAMPLE_SEED, 1, 0, 2, 1);
}

#[test]
#[should_panic]
fn random_natural_vectors_fail_2() {
    // mean_bits_numerator is zero
    let _ = random_natural_vectors(EXAMPLE_SEED, 0, 1, 2, 1);
}

#[test]
#[should_panic]
fn random_natural_vectors_fail_3() {
    // mean_length_denominator is zero
    let _ = random_natural_vectors(EXAMPLE_SEED, 4, 1, 2, 0);
}

#[test]
#[should_panic]
fn random_natural_vectors_fail_4() {
    // mean_length_numerator is zero
    let _ = random_natural_vectors(EXAMPLE_SEED, 4, 1, 0, 1);
}
