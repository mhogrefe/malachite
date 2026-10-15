// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_q::rational_matrix::random::striped_random_rational_matrices;

fn striped_random_rational_matrices_helper(
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_dimension_numerator: u64,
    mean_dimension_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        (striped_random_rational_matrices(
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
fn test_striped_random_rational_matrices() {
    // mean stripe = 2/1, mean bits = 2/1, mean dimension = 1/1
    striped_random_rational_matrices_helper(
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
            "[0, -62/5, 1, 0, 0, -7/3]",
            "[4, 1, 1, 0, -1]",
            "[-9/14]",
            "[-1, 2; 0, -1]",
            "[-6, 8; -13/3, -1/6; 1, -1; 0, 0]",
            "[]_{4,0}",
            "[]_{1,0}",
            "[]_{1,0}",
            "[]_{0,0}",
            "[0, -3]",
            "[]_{2,0}",
            "[]_{3,0}",
            "[5/6; -13; -2]",
            "[]_{2,0}",
            "[]_{0,1}",
            "[-26; 0]",
        ],
    );
    // mean stripe = 16/1, mean bits = 4/1, mean dimension = 2/1
    striped_random_rational_matrices_helper(
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
            "[-1/8, 0; 575/14, -15; 1/4, -4; 63/31, 32]",
            "[]_{6,0}",
            "[]_{0,2}",
            "[]_{1,0}",
            "[]_{0,3}",
            "[]_{6,0}",
            "[1677671/3]",
            "[31/7, -1/2, -4, -2; 1/8, 1/3, -1, -31/7]",
            "[]_{0,1}",
            "[]_{0,3}",
            "[]_{5,0}",
            "[]_{4,0}",
            "[]_{0,6}",
            "[64, -3/2]",
            "[]_{0,1}",
            "[]_{0,0}",
            "[]_{0,0}",
        ],
    );
    // mean stripe = 4/1, mean bits = 8/1, mean dimension = 2/1
    striped_random_rational_matrices_helper(
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
            "[-1/7, -1272959/39; 65343/8, -15531526; 76, -39881/135; 88/9, 0]",
            "[]_{6,0}",
            "[]_{0,2}",
            "[]_{1,0}",
            "[]_{0,3}",
            "[]_{6,0}",
            "[71/11]",
            "[15/31, -67630979/8643, 0, -1920; 2132737536/7, 1/15619, -1/256, -63/31]",
            "[]_{0,1}",
            "[]_{0,3}",
            "[]_{5,0}",
            "[]_{4,0}",
            "[]_{0,6}",
            "[13/3, -236]",
            "[]_{0,1}",
            "[]_{0,0}",
            "[]_{0,0}",
        ],
    );
}

#[test]
#[should_panic]
fn striped_random_rational_matrices_fail_1() {
    // mean_stripe_denominator is zero
    let _ = striped_random_rational_matrices(EXAMPLE_SEED, 1, 0, 4, 1, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_rational_matrices_fail_2() {
    // mean_stripe_numerator < mean_stripe_denominator
    let _ = striped_random_rational_matrices(EXAMPLE_SEED, 1, 2, 4, 1, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_rational_matrices_fail_3() {
    // mean_bits_numerator is zero
    let _ = striped_random_rational_matrices(EXAMPLE_SEED, 4, 1, 0, 1, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_rational_matrices_fail_4() {
    // mean_dimension_denominator is zero
    let _ = striped_random_rational_matrices(EXAMPLE_SEED, 4, 1, 4, 1, 2, 0);
}
