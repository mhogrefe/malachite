// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::unsigned_matrix::random::striped_random_unsigned_matrices;

fn striped_random_unsigned_matrices_helper(
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_dimension_numerator: u64,
    mean_dimension_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        (striped_random_unsigned_matrices::<u8>(
            EXAMPLE_SEED,
            mean_stripe_numerator,
            mean_stripe_denominator,
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
fn test_striped_random_unsigned_matrices() {
    // mean stripe = 2/1, mean dimension = 1/1
    striped_random_unsigned_matrices_helper(
        2,
        1,
        1,
        1,
        &[
            "[]_{0,0}",
            "[]_{0,1}",
            "[]_{0,2}",
            "[]_{0,0}",
            "[94, 192, 188, 225, 230, 156]",
            "[0, 27, 169, 223, 37]",
            "[27]",
            "[213, 48; 57, 244]",
            "[250, 48; 120, 2; 182, 190; 207, 223]",
            "[]_{4,0}",
            "[]_{1,0}",
            "[]_{1,0}",
            "[]_{0,0}",
            "[211, 202]",
            "[]_{2,0}",
            "[]_{3,0}",
            "[45; 22; 8]",
            "[]_{2,0}",
            "[]_{0,1}",
            "[58; 167]",
        ],
    );
    // mean stripe = 4/1, mean dimension = 2/1
    striped_random_unsigned_matrices_helper(
        4,
        1,
        2,
        1,
        &[
            "[]_{0,1}",
            "[]_{0,11}",
            "[]_{2,0}",
            "[127, 128; 240, 255; 184, 224; 32, 3]",
            "[]_{6,0}",
            "[]_{0,2}",
            "[]_{1,0}",
            "[]_{0,3}",
            "[]_{6,0}",
            "[224]",
            "[254, 112, 7, 194; 60, 1, 128, 128]",
            "[]_{0,1}",
            "[]_{0,3}",
            "[]_{5,0}",
            "[]_{4,0}",
            "[]_{0,6}",
            "[15, 28]",
            "[]_{0,1}",
            "[]_{0,0}",
            "[]_{0,0}",
        ],
    );
    // mean stripe = 16/1, mean dimension = 2/1
    striped_random_unsigned_matrices_helper(
        16,
        1,
        2,
        1,
        &[
            "[]_{0,1}",
            "[]_{0,11}",
            "[]_{2,0}",
            "[0, 255; 128, 255; 255, 255; 0, 15]",
            "[]_{6,0}",
            "[]_{0,2}",
            "[]_{1,0}",
            "[]_{0,3}",
            "[]_{6,0}",
            "[255]",
            "[255, 126, 3, 255; 0, 0, 255, 255]",
            "[]_{0,1}",
            "[]_{0,3}",
            "[]_{5,0}",
            "[]_{4,0}",
            "[]_{0,6}",
            "[0, 0]",
            "[]_{0,1}",
            "[]_{0,0}",
            "[]_{0,0}",
        ],
    );
}

#[test]
#[should_panic]
fn striped_random_unsigned_matrices_fail_1() {
    // mean_stripe_denominator is zero
    let _ = striped_random_unsigned_matrices::<u8>(EXAMPLE_SEED, 1, 0, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_unsigned_matrices_fail_2() {
    // mean_stripe_numerator < mean_stripe_denominator
    let _ = striped_random_unsigned_matrices::<u8>(EXAMPLE_SEED, 1, 2, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_unsigned_matrices_fail_3() {
    // mean_dimension_denominator is zero
    let _ = striped_random_unsigned_matrices::<u8>(EXAMPLE_SEED, 4, 1, 2, 0);
}
