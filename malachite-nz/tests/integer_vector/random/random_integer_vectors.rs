// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::integer_vector::random::random_integer_vectors;

fn random_integer_vectors_helper(
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        random_integer_vectors(
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
fn test_random_integer_vectors() {
    // mean bits = 1, mean dimension = 1
    random_integer_vectors_helper(
        1,
        1,
        1,
        1,
        &[
            "(0)",
            "(1, 7)",
            "(-3, 0)",
            "(7, -7)",
            "(-7967)",
            "(-1, 0)",
            "()",
            "()",
            "(0)",
            "()",
            "()",
            "()",
            "(1)",
            "(-1, 1, 1, -1)",
            "()",
            "(-5, -1)",
            "()",
            "(0, 0, 0, 1, 0)",
            "(0)",
            "(11)",
        ],
    );
    // mean bits = 2, mean dimension = 2
    random_integer_vectors_helper(
        2,
        1,
        2,
        1,
        &[
            "(0, 3, 3, 3, 11, -7)",
            "(1)",
            "(-3, 0, 18, 1, -846, -11, 3, 0)",
            "(-81)",
            "(11, -6, 0, 1, 1, -6, 0, -3, -4, -3, 0, -23, -4012, 1)",
            "()",
            "(50, -1, 1, 1487, -1)",
            "(0, 7, -484, -3)",
            "(1)",
            "()",
            "(1, -1, -2, -1, 2, 7)",
            "(0, 0)",
            "()",
            "()",
            "(-1)",
            "(0, 91, 2)",
            "()",
            "(14)",
            "()",
            "(3)",
        ],
    );
    // mean bits = 4, mean dimension = 4
    random_integer_vectors_helper(
        4,
        1,
        4,
        1,
        &[
            "()",
            "(-497, -1, 1, 19, 799, 799, -1, 66, -10721, 334, 59, -119, -5, 0)",
            "(-131, -1, 0, 7)",
            "(-1, 10, 11, -37408)",
            "(0)",
            "()",
            "(-1, -903, 28, 0, 0)",
            "(-6, 2)",
            "(-132, 190, 463, -280)",
            "()",
            "(1, -36, -19203, 1, 1, -18)",
            "()",
            "()",
            "(-1, 88, -1, -1107, -39, 0, 0, -8091, 6, 1142, 459)",
            "(218, 3, -12, -21747, 6, 8567, 3, -1)",
            "()",
            "(1, 25, -27)",
            "()",
            "(0, 22821, 6639, -193, 0)",
            "(-507, -779, 58, 2, -27, -6, 6, -1, 1)",
        ],
    );
}

#[test]
#[should_panic]
fn random_integer_vectors_fail_1() {
    // mean_bits_denominator is zero
    let _ = random_integer_vectors(EXAMPLE_SEED, 1, 0, 2, 1);
}

#[test]
#[should_panic]
fn random_integer_vectors_fail_2() {
    // mean_bits_numerator is zero
    let _ = random_integer_vectors(EXAMPLE_SEED, 0, 1, 2, 1);
}

#[test]
#[should_panic]
fn random_integer_vectors_fail_3() {
    // mean_length_denominator is zero
    let _ = random_integer_vectors(EXAMPLE_SEED, 4, 1, 2, 0);
}

#[test]
#[should_panic]
fn random_integer_vectors_fail_4() {
    // mean_length_numerator is zero
    let _ = random_integer_vectors(EXAMPLE_SEED, 4, 1, 0, 1);
}
