// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::unsigned_matrix::random::striped_random_unsigned_matrices_with_dimensions;

fn striped_random_unsigned_matrices_with_dimensions_helper(
    height: u64,
    width: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        (striped_random_unsigned_matrices_with_dimensions::<u8>(
            EXAMPLE_SEED,
            height,
            width,
            mean_stripe_numerator,
            mean_stripe_denominator
        ))
        .take(20)
        .map(|m| m.to_string())
        .collect_vec(),
        expected_values
    );
}

#[test]
fn test_striped_random_unsigned_matrices_with_dimensions() {
    // 1-by-0, mean stripe = 4/1
    striped_random_unsigned_matrices_with_dimensions_helper(
        1,
        0,
        4,
        1,
        &[
            "[]_{1,0}", "[]_{1,0}", "[]_{1,0}", "[]_{1,0}", "[]_{1,0}", "[]_{1,0}", "[]_{1,0}",
            "[]_{1,0}", "[]_{1,0}", "[]_{1,0}", "[]_{1,0}", "[]_{1,0}", "[]_{1,0}", "[]_{1,0}",
            "[]_{1,0}", "[]_{1,0}", "[]_{1,0}", "[]_{1,0}", "[]_{1,0}", "[]_{1,0}",
        ],
    );
    // 2-by-2, mean stripe = 2/1
    striped_random_unsigned_matrices_with_dimensions_helper(
        2,
        2,
        2,
        1,
        &[
            "[51, 114; 86, 180]",
            "[65, 195; 40, 55]",
            "[3, 196; 185, 251]",
            "[61, 115; 166, 82]",
            "[118, 51; 31, 83]",
            "[198, 228; 99, 74]",
            "[163, 34; 3, 185]",
            "[194, 70; 1, 246]",
            "[28, 132; 191, 63]",
            "[153, 50; 159, 79]",
            "[149, 139; 163, 91]",
            "[199, 163; 67, 165]",
            "[169, 156; 155, 26]",
            "[178, 251; 240, 140]",
            "[34, 169; 147, 64]",
            "[78, 133; 196, 182]",
            "[52, 109; 120, 155]",
            "[38, 114; 66, 45]",
            "[252, 179; 194, 57]",
            "[70, 173; 102, 193]",
        ],
    );
    // 2-by-3, mean stripe = 16/1
    striped_random_unsigned_matrices_with_dimensions_helper(
        2,
        3,
        16,
        1,
        &[
            "[15, 0, 0; 252, 0, 254]",
            "[0, 0, 0; 255, 192, 255]",
            "[127, 0, 224; 0, 0, 15]",
            "[0, 0, 255; 255, 0, 127]",
            "[255, 63, 0; 255, 192, 15]",
            "[15, 255, 0; 254, 255, 1]",
            "[255, 3, 224; 0, 255, 255]",
            "[255, 1, 248; 255, 0, 255]",
            "[255, 255, 255; 0, 255, 192]",
            "[248, 224, 0; 255, 252, 0]",
            "[0, 192, 255; 248, 127, 7]",
            "[0, 255, 0; 0, 0, 63]",
            "[240, 255, 255; 0, 7, 255]",
            "[0, 255, 0; 254, 0, 0]",
            "[48, 248, 255; 0, 255, 0]",
            "[128, 254, 0; 0, 0, 0]",
            "[127, 8, 127; 255, 255, 255]",
            "[127, 255, 252; 252, 255, 0]",
            "[249, 192, 0; 127, 1, 255]",
            "[1, 255, 255; 224, 255, 255]",
        ],
    );
}

#[test]
#[should_panic]
fn striped_random_unsigned_matrices_with_dimensions_fail_1() {
    // mean_stripe_denominator is zero
    let _ = striped_random_unsigned_matrices_with_dimensions::<u8>(EXAMPLE_SEED, 2, 2, 1, 0);
}

#[test]
#[should_panic]
fn striped_random_unsigned_matrices_with_dimensions_fail_2() {
    // mean_stripe_numerator < mean_stripe_denominator
    let _ = striped_random_unsigned_matrices_with_dimensions::<u8>(EXAMPLE_SEED, 2, 2, 1, 2);
}

#[test]
#[should_panic]
fn striped_random_unsigned_matrices_with_dimensions_fail_3() {
    // height * width overflows
    let _ = striped_random_unsigned_matrices_with_dimensions::<u8>(EXAMPLE_SEED, u64::MAX, 2, 4, 1);
}
