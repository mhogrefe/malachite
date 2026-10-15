// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::integer_matrix::random::striped_random_integer_matrices_with_dimensions;

fn striped_random_integer_matrices_with_dimensions_helper(
    height: usize,
    width: usize,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        (striped_random_integer_matrices_with_dimensions(
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
fn test_striped_random_integer_matrices_with_dimensions() {
    // 1-by-0, mean stripe = 4/1, mean bits = 4/1
    striped_random_integer_matrices_with_dimensions_helper(
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
    striped_random_integer_matrices_with_dimensions_helper(
        2,
        2,
        2,
        1,
        2,
        1,
        &[
            "[1, 0; -28, 28]",
            "[-4, -22; 7, 2]",
            "[0, -3; -1, -1]",
            "[-9, -40; -4, -12]",
            "[0, 0; 1, 6]",
            "[1, -8; 1, -11]",
            "[0, 33; 4, -12]",
            "[5, -10; -2, 0]",
            "[-7, 9; -1, 8]",
            "[-2, 6; 6, 0]",
            "[8, -2; 7, 1]",
            "[-3, 195; -8, 1]",
            "[-1, 0; 87, -115]",
            "[16, -1; 10, 1]",
            "[24, -50; 0, -1]",
            "[171531, 0; -1, 1]",
            "[3, -3; 1, -1]",
            "[1, 2; -1, 954]",
            "[3, 0; -1, 12]",
            "[-980, 0; -1, 41]",
        ],
    );
    // 2-by-3, mean stripe = 16/1, mean bits = 4/1
    striped_random_integer_matrices_with_dimensions_helper(
        2,
        3,
        16,
        1,
        4,
        1,
        &[
            "[2, 128, 1; 0, -32, 4096]",
            "[0, 31, 6442451424; 127, -8, -6145]",
            "[-1, -7864320, 1024; 64, -1, -512]",
            "[-7, -32, -1024; -135, 3, -1]",
            "[-1, 4095, 512; 6, 127, 19]",
            "[128, 2, 0; 256, -4, 14]",
            "[0, 1, -14; -7, 0, -3145728]",
            "[0, 1, 3; -4, 63, -4]",
            "[-3, 0, 1; 2, -2, 4]",
            "[-1, 0, 1; -8, 2, -15]",
            "[4456447, -5119, 1; 3, 0, 127]",
            "[-1, -3, -8; -4, -2, 7]",
            "[8, 2, 16; 0, 8, 1]",
            "[-4, 0, -16; 31, -16, 1]",
            "[195, 17, -255; -1, 2, 31]",
            "[4, 0, 0; 15, -8, 3]",
            "[1, 1, 1; 127, 1, 0]",
            "[0, -1, -31; 8, 8, -64]",
            "[8, -7, 8; 3, -2, -1023]",
            "[-15, 1, -3; 127, -2, -1]",
        ],
    );
}

#[test]
#[should_panic]
fn striped_random_integer_matrices_with_dimensions_fail_1() {
    // mean_stripe_denominator is zero
    let _ = striped_random_integer_matrices_with_dimensions(EXAMPLE_SEED, 2, 2, 1, 0, 4, 1);
}

#[test]
#[should_panic]
fn striped_random_integer_matrices_with_dimensions_fail_2() {
    // mean_stripe_numerator < mean_stripe_denominator
    let _ = striped_random_integer_matrices_with_dimensions(EXAMPLE_SEED, 2, 2, 1, 2, 4, 1);
}

#[test]
#[should_panic]
fn striped_random_integer_matrices_with_dimensions_fail_3() {
    // mean_bits_numerator is zero
    let _ = striped_random_integer_matrices_with_dimensions(EXAMPLE_SEED, 2, 2, 4, 1, 0, 1);
}

#[test]
#[should_panic]
fn striped_random_integer_matrices_with_dimensions_fail_4() {
    // height * width overflows
    let _ =
        striped_random_integer_matrices_with_dimensions(EXAMPLE_SEED, usize::MAX, 2, 4, 1, 4, 1);
}
