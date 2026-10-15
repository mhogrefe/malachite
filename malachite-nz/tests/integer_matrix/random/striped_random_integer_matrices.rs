// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::integer_matrix::random::striped_random_integer_matrices;

fn striped_random_integer_matrices_helper(
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_dimension_numerator: u64,
    mean_dimension_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        (striped_random_integer_matrices(
            EXAMPLE_SEED,
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_bits_numerator,
            mean_bits_denominator,
            mean_dimension_numerator,
            mean_dimension_denominator
        ))
        .take(20)
        .map(|m| m.to_string())
        .collect_vec(),
        expected_values
    );
}

#[test]
fn test_striped_random_integer_matrices() {
    // mean stripe = 2/1, mean bits = 2/1, mean dimension = 1/1
    striped_random_integer_matrices_helper(
        2,
        1,
        2,
        1,
        1,
        1,
        &[
            "[]_{0,0}",
            "[]_{0,1}",
            "[]_{0,2}",
            "[]_{0,0}",
            "[3, -3, -56, -1, 0, -2]",
            "[1, 5, -2, -3688, 0]",
            "[0]",
            "[-1, 1; 7, -21]",
            "[1881, -9; 1, 4; 9, -6; 3, 0]",
            "[]_{4,0}",
            "[]_{1,0}",
            "[]_{1,0}",
            "[]_{0,0}",
            "[0, -1]",
            "[]_{2,0}",
            "[]_{3,0}",
            "[-114; -4; -2]",
            "[]_{2,0}",
            "[]_{0,1}",
            "[104; -1]",
        ],
    );
    // mean stripe = 16/1, mean bits = 4/1, mean dimension = 2/1
    striped_random_integer_matrices_helper(
        16,
        1,
        4,
        1,
        2,
        1,
        &[
            "[]_{0,1}",
            "[]_{0,11}",
            "[]_{2,0}",
            "[0, 0; -1, 3; -1, -7; 0, 48]",
            "[]_{6,0}",
            "[]_{0,2}",
            "[]_{1,0}",
            "[]_{0,3}",
            "[]_{6,0}",
            "[1793]",
            "[4095, -1, -61440, 512; -7, -1, -31, -15]",
            "[]_{0,1}",
            "[]_{0,3}",
            "[]_{5,0}",
            "[]_{4,0}",
            "[]_{0,6}",
            "[3, 2]",
            "[]_{0,1}",
            "[]_{0,0}",
            "[]_{0,0}",
        ],
    );
    // mean stripe = 4/1, mean bits = 8/1, mean dimension = 2/1
    striped_random_integer_matrices_helper(
        4,
        1,
        8,
        1,
        2,
        1,
        &[
            "[]_{0,1}",
            "[]_{0,11}",
            "[]_{2,0}",
            "[65, -67; -125986, -247; -57647126777882936192, 292867; -1, -65500]",
            "[]_{6,0}",
            "[]_{0,2}",
            "[]_{1,0}",
            "[]_{0,3}",
            "[]_{6,0}",
            "[-16000]",
            "[-16, 1, 71, 1; -33, -71151, 2, -7932]",
            "[]_{0,1}",
            "[]_{0,3}",
            "[]_{5,0}",
            "[]_{4,0}",
            "[]_{0,6}",
            "[-1, -3704060]",
            "[]_{0,1}",
            "[]_{0,0}",
            "[]_{0,0}",
        ],
    );
}

#[test]
#[should_panic]
fn striped_random_integer_matrices_fail_1() {
    // mean_stripe_denominator is zero
    let _ = striped_random_integer_matrices(EXAMPLE_SEED, 1, 0, 4, 1, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_integer_matrices_fail_2() {
    // mean_stripe_numerator < mean_stripe_denominator
    let _ = striped_random_integer_matrices(EXAMPLE_SEED, 1, 2, 4, 1, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_integer_matrices_fail_3() {
    // mean_bits_numerator is zero
    let _ = striped_random_integer_matrices(EXAMPLE_SEED, 4, 1, 0, 1, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_integer_matrices_fail_4() {
    // mean_dimension_denominator is zero
    let _ = striped_random_integer_matrices(EXAMPLE_SEED, 4, 1, 4, 1, 2, 0);
}
