// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::natural_matrix::random::striped_random_natural_square_matrices;

fn striped_random_natural_square_matrices_helper(
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_size_numerator: u64,
    mean_size_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        (striped_random_natural_square_matrices(
            EXAMPLE_SEED,
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_bits_numerator,
            mean_bits_denominator,
            mean_size_numerator,
            mean_size_denominator
        ))
        .take(20)
        .map(|m| m.to_string())
        .collect_vec(),
        expected_values
    );
}

#[test]
fn test_striped_random_natural_square_matrices() {
    // mean stripe = 2/1, mean bits = 2/1, mean size = 1/1
    striped_random_natural_square_matrices_helper(
        2,
        1,
        2,
        1,
        1,
        1,
        &[
            "[]_{0,0}",
            "[]_{0,0}",
            "[]_{0,0}",
            "[3]",
            "[]_{0,0}",
            "[0, 177; 0, 1]",
            "[]_{0,0}",
            "[]_{0,0}",
            "[0]",
            "[5, 0, 4, 1, 0, 3; 1, 1, 1, 1, 0, 0; 1, 1, 5, 9, 0, 2; 1, 0, 0, 321, 0, 18; 0, 0, 3, \
            3, 1, 0; 2, 6, 0, 1892, 0, 1]",
            "[0]",
            "[0, 5, 6, 0, 1; 2, 1, 12, 6, 1; 1, 1, 1, 4, 1; 23, 2, 0, 0, 0; 14, 2, 1, 0, 3]",
            "[0]",
            "[6]",
            "[0, 0; 11, 3]",
            "[1, 992; 1, 1]",
            "[121, 0, 6, 12; 0, 31, 25, 7; 0, 33, 52, 50; 0, 3, 2, 0]",
            "[2, 6; 0, 36]",
            "[0, 3, 5, 0; 0, 4, 1, 3; 3, 1, 1, 0; 0, 1, 0, 15]",
            "[]_{0,0}",
        ],
    );
    // mean stripe = 16/1, mean bits = 4/1, mean size = 1/1
    striped_random_natural_square_matrices_helper(
        16,
        1,
        4,
        1,
        1,
        1,
        &[
            "[]_{0,0}",
            "[]_{0,0}",
            "[]_{0,0}",
            "[3]",
            "[]_{0,0}",
            "[7999, 24; 7, 0]",
            "[]_{0,0}",
            "[]_{0,0}",
            "[2]",
            "[0, 1, 1, 0, 0, 4; 4, 0, 65536, 1, 0, 1; 3, 3, 31, 4, 256, 0; 127, 1, 7, 0, 1, 0; \
            128, 1024, 4, 524296, 5, 15; 4, 8192, 4, 64, 1, 2040]",
            "[2]",
            "[0, 1, 17, 0, 15; 4, 0, 526080, 1, 16; 8, 0, 0, 32, 64; 1, 2, 2097311, 127, 1; 4, 32, \
            15, 3, 16]",
            "[0]",
            "[2]",
            "[131071, 7; 1, 17]",
            "[0, 4; 0, 8]",
            "[0, 7, 8, 63488; 4, 1, 511, 799; 2, 15, 3, 15; 0, 15, 95, 7180]",
            "[1, 8; 0, 3]",
            "[7, 0, 0, 3; 0, 0, 31, 2; 67, 127, 319, 0; 56, 1, 8, 319]",
            "[]_{0,0}",
        ],
    );
    // mean stripe = 4/1, mean bits = 8/1, mean size = 3/2
    striped_random_natural_square_matrices_helper(
        4,
        1,
        8,
        1,
        3,
        2,
        &[
            "[]_{0,0}",
            "[]_{0,0}",
            "[3, 127, 62, 3, 46, 79; 3, 1, 4160, 0, 8, 0; 1, 2323511445475, 5, 8321966083, 0, 15; \
            0, 4, 2056, 3, 30, 0; 3, 1113623, 958720, 1, 2, 0; 248, 11, 4071, 151380834436, 4, \
            124]",
            "[]_{0,0}",
            "[132032, 20; 0, 2]",
            "[0, 62; 3, 20735]",
            "[3871]",
            "[]_{0,0}",
            "[16, 2, 0; 1, 16, 156; 0, 1, 72]",
            "[]_{0,0}",
            "[139, 1, 30759, 3, 8; 0, 254, 2, 2613745, 3; 30, 1, 3, 1871, 399399346197; 43, 224, \
            2040, 0, 19; 224, 2, 16, 4, 67]",
            "[0, 1, 278464, 99, 4223991, 0; 79, 255, 15, 960, 51, 6; 3, 1539, 3, 3, 8184, 1669895; \
            263, 47, 112, 35842272, 768, 16284739; 0, 479, 126, 56, 1023, 0; 4065, 29, 2886, 306, \
            0, 0]",
            "[]_{0,0}",
            "[11]",
            "[80]",
            "[3703170]",
            "[]_{0,0}",
            "[63488, 1025, 0; 3, 0, 128; 124, 32, 16]",
            "[1, 0, 843744, 2; 16, 331804, 127, 1087; 17282, 2290, 1, 32; 3, 4, 30751, 5]",
            "[3, 260571135, 512; 184, 3, 2; 3, 4, 127]",
        ],
    );
}

#[test]
#[should_panic]
fn striped_random_natural_square_matrices_fail_1() {
    // mean_stripe_denominator is zero
    let _ = striped_random_natural_square_matrices(EXAMPLE_SEED, 1, 0, 4, 1, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_natural_square_matrices_fail_2() {
    // mean_stripe_numerator < mean_stripe_denominator
    let _ = striped_random_natural_square_matrices(EXAMPLE_SEED, 1, 2, 4, 1, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_natural_square_matrices_fail_3() {
    // mean_bits_numerator is zero
    let _ = striped_random_natural_square_matrices(EXAMPLE_SEED, 4, 1, 0, 1, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_natural_square_matrices_fail_4() {
    // mean_size_denominator is zero
    let _ = striped_random_natural_square_matrices(EXAMPLE_SEED, 4, 1, 4, 1, 2, 0);
}
