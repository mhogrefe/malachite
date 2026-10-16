// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::unsigned_matrix::random::striped_random_unsigned_square_matrices;

fn striped_random_unsigned_square_matrices_helper(
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_size_numerator: u64,
    mean_size_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        (striped_random_unsigned_square_matrices::<u8>(
            EXAMPLE_SEED,
            mean_stripe_numerator,
            mean_stripe_denominator,
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
fn test_striped_random_unsigned_square_matrices() {
    // mean stripe = 2/1, mean size = 1/1
    striped_random_unsigned_square_matrices_helper(
        2,
        1,
        1,
        1,
        &[
            "[]_{0,0}",
            "[]_{0,0}",
            "[]_{0,0}",
            "[94]",
            "[]_{0,0}",
            "[192, 188; 225, 230]",
            "[]_{0,0}",
            "[]_{0,0}",
            "[156]",
            "[0, 27, 169, 223, 37, 27; 213, 48, 57, 244, 250, 48; 120, 2, 182, 190, 207, 223; 211, \
            202, 45, 22, 8, 58; 167, 24, 179, 134, 54, 134; 4, 11, 72, 143, 86, 25]",
            "[15]",
            "[234, 42, 46, 210, 94; 23, 11, 58, 78, 96; 194, 56, 22, 178, 6; 64, 18, 11, 230, 237; \
            5, 192, 146, 50, 108]",
            "[166]",
            "[211]",
            "[72, 59; 122, 20]",
            "[103, 5; 101, 231]",
            "[148, 200, 5, 144; 65, 153, 47, 87; 105, 95, 109, 23; 71, 243, 220, 252]",
            "[15, 156; 8, 166]",
            "[105, 197, 88, 195; 97, 241, 228, 49; 228, 176, 127, 1; 247, 110, 16, 61]",
            "[]_{0,0}",
        ],
    );
    // mean stripe = 4/1, mean size = 1/1
    striped_random_unsigned_square_matrices_helper(
        4,
        1,
        1,
        1,
        &[
            "[]_{0,0}",
            "[]_{0,0}",
            "[]_{0,0}",
            "[127]",
            "[]_{0,0}",
            "[128, 240; 255, 184]",
            "[]_{0,0}",
            "[]_{0,0}",
            "[224]",
            "[32, 3, 224, 254, 112, 7; 194, 60, 1, 128, 128, 15; 28, 127, 191, 253, 216, 200; 195, \
            239, 1, 3, 127, 126; 224, 62, 254, 240, 120, 128; 28, 1, 31, 220, 16, 112]",
            "[59]",
            "[200, 60, 127, 255, 0; 7, 62, 127, 0, 31; 224, 0, 0, 193, 12; 0, 5, 0, 128, 254; 28, \
            255, 198, 0, 28]",
            "[133]",
            "[255]",
            "[111, 120; 28, 0]",
            "[112, 3; 60, 240]",
            "[196, 240, 6, 255; 63, 255, 0, 1; 5, 63, 34, 120; 30, 248, 145, 255]",
            "[3, 240; 63, 192]",
            "[0, 254, 64, 255; 0, 128, 244, 116; 192, 255, 127, 115; 247, 3, 127, 126]",
            "[]_{0,0}",
        ],
    );
    // mean stripe = 16/1, mean size = 3/2
    striped_random_unsigned_square_matrices_helper(
        16,
        1,
        3,
        2,
        &[
            "[]_{0,0}",
            "[]_{0,0}",
            "[0, 255, 128, 255, 255, 255; 0, 15, 255, 255, 126, 3; 255, 0, 0, 255, 255, 0; 0, 7, \
            192, 227, 255, 255; 255, 255, 0, 0, 0, 1; 255, 0, 255, 255, 31, 192]",
            "[]_{0,0}",
            "[0, 0; 0, 224]",
            "[0, 0; 0, 224]",
            "[15]",
            "[]_{0,0}",
            "[0, 192, 0; 0, 0, 127; 0, 28, 255]",
            "[]_{0,0}",
            "[0, 0, 255, 15, 0; 0, 1, 255, 255, 126; 255, 255, 0, 48, 129; 248, 0, 63, 7, 127; 0, \
            0, 0, 255, 255]",
            "[240, 15, 240, 31, 255, 0; 0, 0, 0, 0, 0, 0; 249, 255, 255, 0, 255, 3; 192, 3, 255, \
            31, 240, 0; 192, 255, 0, 231, 193, 0; 112, 239, 0, 16, 0, 255]",
            "[]_{0,0}",
            "[0]",
            "[255]",
            "[0]",
            "[]_{0,0}",
            "[255, 15, 12; 0, 0, 15; 0, 31, 255]",
            "[240, 252, 0, 255; 128, 254, 254, 0; 15, 252, 0, 0; 255, 0, 0, 63]",
            "[255, 255, 255; 240, 255, 255; 1, 32, 0]",
        ],
    );
}

#[test]
#[should_panic]
fn striped_random_unsigned_square_matrices_fail_1() {
    // mean_stripe_denominator is zero
    let _ = striped_random_unsigned_square_matrices::<u8>(EXAMPLE_SEED, 1, 0, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_unsigned_square_matrices_fail_2() {
    // mean_stripe_numerator < mean_stripe_denominator
    let _ = striped_random_unsigned_square_matrices::<u8>(EXAMPLE_SEED, 1, 2, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_unsigned_square_matrices_fail_3() {
    // mean_size_denominator is zero
    let _ = striped_random_unsigned_square_matrices::<u8>(EXAMPLE_SEED, 4, 1, 2, 0);
}
