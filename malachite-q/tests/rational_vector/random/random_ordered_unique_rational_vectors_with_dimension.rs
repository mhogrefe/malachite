// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_q::rational_vector::random::random_ordered_unique_rational_vectors_with_dimension;

#[track_caller]
fn random_ordered_unique_rational_vectors_with_dimension_helper(
    dimension: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        random_ordered_unique_rational_vectors_with_dimension(
            EXAMPLE_SEED,
            dimension,
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
fn test_random_ordered_unique_rational_vectors_with_dimension() {
    // dimension 0: only the 0-dimensional vector
    random_ordered_unique_rational_vectors_with_dimension_helper(
        0,
        2,
        1,
        &[
            "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()",
            "()", "()", "()", "()", "()", "()",
        ],
    );
    // dimension 1, mean bits = 4
    random_ordered_unique_rational_vectors_with_dimension_helper(
        1,
        4,
        1,
        &[
            "(0)",
            "(-1/3)",
            "(1)",
            "(4/323)",
            "(0)",
            "(7/12)",
            "(1/17)",
            "(7)",
            "(1/97)",
            "(15/19)",
            "(0)",
            "(-79/21)",
            "(7/6)",
            "(-9912911/5)",
            "(0)",
            "(-5/786)",
            "(-986/97)",
            "(-12/5)",
            "(0)",
            "(0)",
        ],
    );
    // dimension 3, mean bits = 8
    random_ordered_unique_rational_vectors_with_dimension_helper(
        3,
        8,
        1,
        &[
            "(-61/3, -5, 574/39)",
            "(0, 25/1752767, 7/201)",
            "(0, 16037/8097, 19878/7)",
            "(-405221395/213, 159/947, 6393/14)",
            "(-1/19, 0, 67961/2)",
            "(-320256/25, -6, -12/3233)",
            "(-16039/9, -79/885, 0)",
            "(-34/3, -341/76, 0)",
            "(-308546125/7, -11/2674, 2360/9)",
            "(-3/7, 0, 1/7)",
            "(-1/24, 25/4203, 2/9)",
            "(-1621206/5, -37/19, 2/7)",
            "(-348470487639/7824721, 103/9, 24317/28)",
            "(-27425/77, -8/185, 0)",
            "(-23, 0, 23/30)",
            "(-389694, -209/351, 19/13)",
            "(-61/4, -517/221, 1949/2)",
            "(-327/54638, 3499/38252526123374787586, 47065394/3)",
            "(-24/7, -33/10, -1/2210485)",
            "(0, 2085/109, 369/19)",
        ],
    );
    // dimension 5, mean bits = 3/2: small elements, so many repeats are redrawn
    random_ordered_unique_rational_vectors_with_dimension_helper(
        5,
        3,
        2,
        &[
            "(0, 1/17, 1, 5, 10)",
            "(-3, -1, -1/3, 0, 1/2)",
            "(-19, -7, -5, -1/3, 0)",
            "(-4, -1, -2/3, 0, 218)",
            "(-1, -1/3, 0, 1, 30)",
            "(-1, 0, 1/11, 1/2, 29)",
            "(-52, -5, -1, 0, 1)",
            "(-6, -3, -2, 0, 5)",
            "(-7, -3, -2, 0, 1)",
            "(-1, 0, 1/4, 1, 7/3)",
            "(-843, -21/2, -7/2, -1, 0)",
            "(-20, -19/2, -7, -2, 0)",
            "(-3, 0, 1/2, 3, 5)",
            "(-2, -1, 0, 1, 79)",
            "(-11, -11/8, -1, 0, 2)",
            "(-24, -9, -1/2, 0, 2)",
            "(-3, -1, 0, 1/6, 10)",
            "(-3, -1, -1/3, 0, 45)",
            "(-8, -1, 0, 1, 2)",
            "(-2, -1, 1/3, 1, 5)",
        ],
    );
}

#[test]
#[should_panic]
fn random_ordered_unique_rational_vectors_with_dimension_fail_1() {
    // mean_bits_denominator is zero
    let _ = random_ordered_unique_rational_vectors_with_dimension(EXAMPLE_SEED, 3, 1, 0);
}

#[test]
#[should_panic]
fn random_ordered_unique_rational_vectors_with_dimension_fail_2() {
    // mean_bits_numerator is zero
    let _ = random_ordered_unique_rational_vectors_with_dimension(EXAMPLE_SEED, 3, 0, 1);
}

#[test]
#[should_panic]
fn random_ordered_unique_rational_vectors_with_dimension_fail_3() {
    // mean_bits_numerator <= mean_bits_denominator
    let _ = random_ordered_unique_rational_vectors_with_dimension(EXAMPLE_SEED, 3, 1, 1);
}
