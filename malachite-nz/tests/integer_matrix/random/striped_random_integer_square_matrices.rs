// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::integer_matrix::random::striped_random_integer_square_matrices;

fn striped_random_integer_square_matrices_helper(
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_size_numerator: u64,
    mean_size_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        (striped_random_integer_square_matrices(
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
fn test_striped_random_integer_square_matrices() {
    // mean stripe = 2/1, mean bits = 2/1, mean size = 1/1
    striped_random_integer_square_matrices_helper(
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
            "[-3, -56; -1, 0]",
            "[]_{0,0}",
            "[]_{0,0}",
            "[-2]",
            "[1, 5, -2, -3688, 0, 0; -1, 1, 7, -21, 1881, -9; 1, 4, 9, -6, 3, 0; 0, -1, -114, -4, \
            -2, 104; -1, -3, -50, 12, 14, 1; -1, -1, 1, -1, 1, 15]",
            "[0]",
            "[1, 0, 4, -1, 4; 38, -2, 50, 47, -2; 3, 0, -17, -1, 1; -16, 0, 1, 3, -1; 0, -1, 1, 0, \
            -1]",
            "[5]",
            "[25]",
            "[-1, 12; -5, 18]",
            "[18, -218; 2, 0]",
            "[3, -11, 10, 3; 0, -3, -1, -1; -1, 6, -7, 0; 1, 0, -1, 0]",
            "[-3, -7; 9, 0]",
            "[2, 15, 1, -3; -26, -2, 12, 0; 3, -3, -4, -28; -7, -3, 3, 0]",
            "[]_{0,0}",
        ],
    );
    // mean stripe = 16/1, mean bits = 4/1, mean size = 1/1
    striped_random_integer_square_matrices_helper(
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
            "[0]",
            "[]_{0,0}",
            "[0, -1; 3, -1]",
            "[]_{0,0}",
            "[]_{0,0}",
            "[-7]",
            "[0, 48, 1793, 4095, -1, -61440; 512, -7, -1, -31, -15, 3; 2, -128, 1023, -4, 1, -1; \
            46, -8, -2048, 2, -3, 0; 0, 31, -4, -32, 1, 0; 2, 512, 32, -1, -47, 63]",
            "[-71]",
            "[256, -4, 1, 8248, 64; -2, 1, -1, -1, 7; -1, 8, 64, -8, 1; 7, -4, 65540, 7, 1; 8207, \
            511, 0, -131072, -6]",
            "[31]",
            "[-8]",
            "[0, 1; 512, -2]",
            "[3, 3; 0, -2]",
            "[1031, -255, 3, 79; -3, -4, -15, 2048; -3, 1, 1, -3; 1036, -511, 7, -31]",
            "[-28, 0; 16, 1]",
            "[0, 0, -31759, -257; 1, 0, -16, 31; -263, 3, 0, -131072; -2, -3, -32, -256]",
            "[]_{0,0}",
        ],
    );
    // mean stripe = 4/1, mean bits = 8/1, mean size = 3/2
    striped_random_integer_square_matrices_helper(
        4,
        1,
        8,
        1,
        3,
        2,
        &[
            "[]_{0,0}",
            "[]_{0,0}",
            "[65, -67, -125986, -247, -57647126777882936192, 292867; -1, -65500, -16000, -16, 1, \
            71; 1, -33, -71151, 2, -7932, -1; -3704060, -257, 3, 28, -1840, 4; -248, 63, 2017, 48, \
            -4, -30; 68704779264, 0, -56, -71614, -1, 7]",
            "[]_{0,0}",
            "[-9, -961; -224, 4]",
            "[7, 4; -36958456, -754931358]",
            "[4]",
            "[]_{0,0}",
            "[-31, 2, -1; 255, 4, 0; 33046200, -1, -79]",
            "[]_{0,0}",
            "[31, -14, 32, -1, 7; 131, -2079, 1, 1, -19; 48, 188, 6336, -130048, -4095; -15665, \
            -1, 3, -15, -207; -38764551, 203, 2, 35784115681283, 8]",
            "[-3, 539, 1009, -1807, -2, -27; 132093, -47, 1149936, 43022770, -112, -7; 111, 2, \
            254, 1151, -18141910398979, 1; -14026211528, 1, 18431, -1, 0, 1; 0, 58, 1849287, 1, \
            -2, 512; 48, 32, -7, 0, -2, -34]",
            "[]_{0,0}",
            "[0]",
            "[0]",
            "[1]",
            "[]_{0,0}",
            "[2048, 7, -15; 94, 8, -8; 519680, 7868159, -4]",
            "[-35, -15, 0, -30783; 0, -31, -2, 1536; -15747, -3934, 0, -127; -4, -1, -31, 65]",
            "[-31, -127168, -63; -36800, 9180, -32; 343536843096288, 515, 8562]",
        ],
    );
}

#[test]
#[should_panic]
fn striped_random_integer_square_matrices_fail_1() {
    // mean_stripe_denominator is zero
    let _ = striped_random_integer_square_matrices(EXAMPLE_SEED, 1, 0, 4, 1, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_integer_square_matrices_fail_2() {
    // mean_stripe_numerator < mean_stripe_denominator
    let _ = striped_random_integer_square_matrices(EXAMPLE_SEED, 1, 2, 4, 1, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_integer_square_matrices_fail_3() {
    // mean_bits_numerator is zero
    let _ = striped_random_integer_square_matrices(EXAMPLE_SEED, 4, 1, 0, 1, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_integer_square_matrices_fail_4() {
    // mean_size_denominator is zero
    let _ = striped_random_integer_square_matrices(EXAMPLE_SEED, 4, 1, 4, 1, 2, 0);
}
