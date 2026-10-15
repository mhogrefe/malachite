// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::natural_matrix::random::striped_random_natural_matrices_with_dimensions;

fn striped_random_natural_matrices_with_dimensions_helper(
    height: usize,
    width: usize,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        (striped_random_natural_matrices_with_dimensions(
            EXAMPLE_SEED,
            height,
            width,
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_bits_numerator,
            mean_bits_denominator
        ))
        .take(20)
        .map(|m| m.to_string())
        .collect_vec(),
        expected_values
    );
}

#[test]
fn test_striped_random_natural_matrices_with_dimensions() {
    // 1-by-0, mean stripe = 4/1, mean bits = 4/1
    striped_random_natural_matrices_with_dimensions_helper(
        1,
        0,
        4,
        1,
        4,
        1,
        &[
            "[]_{1,0}", "[]_{1,0}", "[]_{1,0}", "[]_{1,0}", "[]_{1,0}", "[]_{1,0}", "[]_{1,0}",
            "[]_{1,0}", "[]_{1,0}", "[]_{1,0}", "[]_{1,0}", "[]_{1,0}", "[]_{1,0}", "[]_{1,0}",
            "[]_{1,0}", "[]_{1,0}", "[]_{1,0}", "[]_{1,0}", "[]_{1,0}", "[]_{1,0}",
        ],
    );
    // 2-by-2, mean stripe = 2/1, mean bits = 2/1
    striped_random_natural_matrices_with_dimensions_helper(
        2,
        2,
        2,
        1,
        2,
        1,
        &[
            "[44, 0; 0, 2]",
            "[2, 0; 2, 24]",
            "[5, 1; 1, 0]",
            "[2, 3; 9, 2]",
            "[1, 1; 1, 30]",
            "[5, 2; 1, 1]",
            "[15, 6; 0, 6]",
            "[111, 1; 48, 0]",
            "[0, 0; 0, 1]",
            "[0, 89; 0, 58]",
            "[12, 1; 0, 0]",
            "[0, 32; 1, 54]",
            "[0, 1; 0, 7]",
            "[2, 7; 3975, 2]",
            "[0, 3; 0, 2]",
            "[0, 1; 37, 1]",
            "[1, 44; 272, 2]",
            "[0, 10; 4, 1]",
            "[15, 125; 3, 0]",
            "[0, 31; 0, 0]",
        ],
    );
    // 2-by-3, mean stripe = 16/1, mean bits = 4/1
    striped_random_natural_matrices_with_dimensions_helper(
        2,
        3,
        16,
        1,
        4,
        1,
        &[
            "[8, 8, 8; 1, 1, 0]",
            "[1, 128, 15; 64, 1, 3]",
            "[2, 4, 1; 7, 376, 3]",
            "[8, 1, 1; 2047, 1920, 3840]",
            "[3, 1984, 0; 4, 0, 7]",
            "[1, 2, 0; 6144, 2, 16384]",
            "[0, 1, 0; 0, 4, 1]",
            "[255, 2, 0; 7, 0, 0]",
            "[1, 4, 511; 4, 127, 3]",
            "[62, 32, 16; 512, 8, 2]",
            "[64, 3, 7; 15, 15, 255]",
            "[3, 0, 64; 0, 49, 2]",
            "[2, 128, 0; 1023, 0, 8]",
            "[0, 4092, 8; 256, 1, 0]",
            "[512, 4, 9215; 264190, 7, 31]",
            "[1, 31, 15; 0, 1, 7]",
            "[0, 2, 63; 512, 0, 3]",
            "[518016, 1, 2; 1, 1535, 15]",
            "[3, 128, 0; 256, 16, 2]",
            "[31, 65536, 262143; 0, 4, 31]",
        ],
    );
}

#[test]
#[should_panic]
fn striped_random_natural_matrices_with_dimensions_fail_1() {
    // mean_stripe_denominator is zero
    let _ = striped_random_natural_matrices_with_dimensions(EXAMPLE_SEED, 2, 2, 1, 0, 4, 1);
}

#[test]
#[should_panic]
fn striped_random_natural_matrices_with_dimensions_fail_2() {
    // mean_stripe_numerator < mean_stripe_denominator
    let _ = striped_random_natural_matrices_with_dimensions(EXAMPLE_SEED, 2, 2, 1, 2, 4, 1);
}

#[test]
#[should_panic]
fn striped_random_natural_matrices_with_dimensions_fail_3() {
    // mean_bits_numerator is zero
    let _ = striped_random_natural_matrices_with_dimensions(EXAMPLE_SEED, 2, 2, 4, 1, 0, 1);
}

#[test]
#[should_panic]
fn striped_random_natural_matrices_with_dimensions_fail_4() {
    // height * width overflows
    let _ =
        striped_random_natural_matrices_with_dimensions(EXAMPLE_SEED, usize::MAX, 2, 4, 1, 4, 1);
}
