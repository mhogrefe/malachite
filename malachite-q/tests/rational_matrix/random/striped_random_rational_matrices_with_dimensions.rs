// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_q::rational_matrix::random::striped_random_rational_matrices_with_dimensions;

fn striped_random_rational_matrices_with_dimensions_helper(
    height: u64,
    width: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        (striped_random_rational_matrices_with_dimensions(
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
fn test_striped_random_rational_matrices_with_dimensions() {
    // 1-by-0, mean stripe = 4/1, mean bits = 4/1
    striped_random_rational_matrices_with_dimensions_helper(
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
    striped_random_rational_matrices_with_dimensions_helper(
        2,
        2,
        2,
        1,
        2,
        1,
        &[
            "[-2/3, -4/3; 0, 1]",
            "[0, 1/2; 504, 0]",
            "[0, 1; 5/12, -5]",
            "[0, -1/3; -29/3, -1/3]",
            "[0, -1; 0, -15]",
            "[-6, 0; 0, 0]",
            "[0, -2; -1/10, 1/5]",
            "[-1, 6; 0, 0]",
            "[3, 1/2; 0, -291]",
            "[0, 62/5; 2, -1]",
            "[0, 1; -7/12, 0]",
            "[-8, 9; 0, 1]",
            "[0, -73; -3/2, 3/2]",
            "[-27/19, -1; 32, 11/2]",
            "[0, 0; 0, 1]",
            "[0, -12; -3/10, -1]",
            "[5/3, 1/10; 1, 0]",
            "[-1, 7; -2/3, 4/5]",
            "[1, 25/2; 1, -3]",
            "[-2/3, 0; 2/23, -1/222]",
        ],
    );
    // 2-by-3, mean stripe = 16/1, mean bits = 4/1
    striped_random_rational_matrices_with_dimensions_helper(
        2,
        3,
        16,
        1,
        4,
        1,
        &[
            "[0, -1/3, 1; 1/64, 0, 2/7]",
            "[1/16, 16/3, 1/64; 31/32, 0, -127/23]",
            "[1, -2321847, 0; -7/1023, -769/64, -8/7]",
            "[0, 0, 0; 32, 0, -1]",
            "[-2, -536903679, -2; 7/127, -128/31, 16]",
            "[15/4096, -512, 512/511; 31/4, -17/511, 0]",
            "[0, 16/7, 1/3; -3/7, 7/3, 5]",
            "[-3/4, -7/16, -31/112; 2, 3/62, 3/128]",
            "[-64, -1/32, 0; 0, -11, -127/103]",
            "[0, 63, -9/256; -1/3, 0, 15]",
            "[1/4, -2, -31; -2, 0, 0]",
            "[0, -8, -1/16; 2/255, -32/31, 0]",
            "[9/32, 1/16, 1/4; -7/3, -2, 8]",
            "[7/4097, -15/2, -39/8; -16, 0, 8]",
            "[2/7, -8191, 0; -128/3, 15, 32]",
            "[-255, 256, 1; 256, -127/16, -393247/4]",
            "[0, 0, -4; 0, 3, 1/8]",
            "[124, -15/8, 127/5; 52531/3, -2147222527, -1/15]",
            "[-1/7, 511/8, 0; -1/4, 0, 5/21]",
            "[0, -64/23, 0; -1, 8, -32/9]",
        ],
    );
}

#[test]
#[should_panic]
fn striped_random_rational_matrices_with_dimensions_fail_1() {
    // mean_stripe_denominator is zero
    let _ = striped_random_rational_matrices_with_dimensions(EXAMPLE_SEED, 2, 2, 1, 0, 4, 1);
}

#[test]
#[should_panic]
fn striped_random_rational_matrices_with_dimensions_fail_2() {
    // mean_stripe_numerator < mean_stripe_denominator
    let _ = striped_random_rational_matrices_with_dimensions(EXAMPLE_SEED, 2, 2, 1, 2, 4, 1);
}

#[test]
#[should_panic]
fn striped_random_rational_matrices_with_dimensions_fail_3() {
    // mean_bits_numerator is zero
    let _ = striped_random_rational_matrices_with_dimensions(EXAMPLE_SEED, 2, 2, 4, 1, 0, 1);
}

#[test]
#[should_panic]
fn striped_random_rational_matrices_with_dimensions_fail_4() {
    // height * width overflows
    let _ = striped_random_rational_matrices_with_dimensions(EXAMPLE_SEED, u64::MAX, 2, 4, 1, 4, 1);
}
