// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_q::rational_vector::random::random_rational_vectors;

fn random_rational_vectors_helper(
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        random_rational_vectors(
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
fn test_random_rational_vectors() {
    // mean bits = 2, mean dimension = 1
    random_rational_vectors_helper(
        2,
        1,
        1,
        1,
        &[
            "(0)",
            "(0, -6)",
            "(2, 0)",
            "(0, 0)",
            "(-5/18)",
            "(0, -1/6)",
            "()",
            "()",
            "(1)",
            "()",
            "()",
            "()",
            "(0)",
            "(0, 1, 0, 0)",
            "()",
            "(29/2, 0)",
            "()",
            "(1, 0, -8, -1, -1/3)",
            "(1/2)",
            "(5/2)",
        ],
    );
    // mean bits = 3, mean dimension = 2
    random_rational_vectors_helper(
        3,
        1,
        2,
        1,
        &[
            "(0, 0, -3/23, 2, 0, 0)",
            "(0)",
            "(-5/6, 1/82, -5/6, 1, -3, 0, 1/7, 0)",
            "(1/2)",
            "(53/6, 0, 1, 0, -181/3, -1, -4, 1/30, 7/2, 2, -8/7, 7, 89, 0)",
            "()",
            "(0, -2/3, -216059/12, 5/2, -337)",
            "(-491, -7/3, 0, 2953/3)",
            "(7/38)",
            "()",
            "(2/7, -245/3, -43/2, -9/2, 13/3, 3/16)",
            "(-22/7, -2)",
            "()",
            "()",
            "(0)",
            "(0, 1/6, 9/265)",
            "()",
            "(-157/2)",
            "()",
            "(-1)",
        ],
    );
    // mean bits = 4, mean dimension = 4
    random_rational_vectors_helper(
        4,
        1,
        4,
        1,
        &[
            "()",
            "(0, 1, -1/55, 0, -5/166, 7/118, -3/194, -1/6, 265/42, -1/6, 1, 0, 0, 53/15)",
            "(1/14, 5/4, 714121/22, -196)",
            "(0, -1, -3372766/3, -10)",
            "(-8)",
            "()",
            "(0, 0, 3699/7, -6, 36)",
            "(59/2, -47/31566)",
            "(0, -1, -107/76, 13/6)",
            "()",
            "(-1/2, 0, 0, -791, 0, 4/83)",
            "()",
            "()",
            "(117/31, -9, -41/2, -1, 43/110, 1/32, -1/7, -39, 5/124, 7/2, 3/10)",
            "(5/1801, -1/6, -1, 14/99, -21/4, -1392/7, 1, 1/27)",
            "()",
            "(35, 13/4, -222)",
            "()",
            "(0, 0, -6/5, 4584477, -3)",
            "(-284/45, -12/13, 2375805, 0, 11/3, 64/3, -1/2, 1163/167, -168)",
        ],
    );
}

#[test]
#[should_panic]
fn random_rational_vectors_fail_1() {
    // mean_bits_denominator is zero
    let _ = random_rational_vectors(EXAMPLE_SEED, 1, 0, 2, 1);
}

#[test]
#[should_panic]
fn random_rational_vectors_fail_2() {
    // mean_bits_numerator is zero
    let _ = random_rational_vectors(EXAMPLE_SEED, 0, 1, 2, 1);
}

#[test]
#[should_panic]
fn random_rational_vectors_fail_3() {
    // mean_bits_numerator <= mean_bits_denominator
    let _ = random_rational_vectors(EXAMPLE_SEED, 1, 1, 2, 1);
}

#[test]
#[should_panic]
fn random_rational_vectors_fail_4() {
    // mean_length_denominator is zero
    let _ = random_rational_vectors(EXAMPLE_SEED, 4, 1, 2, 0);
}

#[test]
#[should_panic]
fn random_rational_vectors_fail_5() {
    // mean_length_numerator is zero
    let _ = random_rational_vectors(EXAMPLE_SEED, 4, 1, 0, 1);
}
