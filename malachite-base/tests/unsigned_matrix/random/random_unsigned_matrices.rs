// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::unsigned_matrix::random::random_unsigned_matrices;

fn random_unsigned_matrices_helper(
    mean_dimension_numerator: u64,
    mean_dimension_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        (random_unsigned_matrices::<u8>(
            EXAMPLE_SEED,
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
fn test_random_unsigned_matrices() {
    // mean dimension = 1/1
    random_unsigned_matrices_helper(
        1,
        1,
        &[
            "[]_{0,0}",
            "[]_{0,1}",
            "[]_{0,2}",
            "[]_{0,0}",
            "[164, 76, 94, 126, 28, 110]",
            "[175, 17, 11, 238, 65]",
            "[62]",
            "[225, 188; 65, 75]",
            "[96, 154; 151, 17; 31, 199; 37, 162]",
            "[]_{4,0}",
            "[]_{1,0}",
            "[]_{1,0}",
            "[]_{0,0}",
            "[235, 226]",
            "[]_{2,0}",
            "[]_{3,0}",
            "[74; 140; 91]",
            "[]_{2,0}",
            "[]_{0,1}",
            "[207; 254]",
        ],
    );
    // mean dimension = 2/1
    random_unsigned_matrices_helper(
        2,
        1,
        &[
            "[]_{0,1}",
            "[]_{0,11}",
            "[]_{2,0}",
            "[164, 76; 94, 126; 28, 110; 175, 17]",
            "[]_{6,0}",
            "[]_{0,2}",
            "[]_{1,0}",
            "[]_{0,3}",
            "[]_{6,0}",
            "[11]",
            "[238, 65, 62, 225; 188, 65, 75, 96]",
            "[]_{0,1}",
            "[]_{0,3}",
            "[]_{5,0}",
            "[]_{4,0}",
            "[]_{0,6}",
            "[154, 151]",
            "[]_{0,1}",
            "[]_{0,0}",
            "[]_{0,0}",
        ],
    );
    // mean dimension = 3/2
    random_unsigned_matrices_helper(
        3,
        2,
        &[
            "[]_{0,0}",
            "[]_{6,0}",
            "[164, 76; 94, 126]",
            "[]_{1,0}",
            "[]_{3,0}",
            "[28, 110, 175, 17, 11, 238; 65, 62, 225, 188, 65, 75; 96, 154, 151, 17, 31, 199; \
            37, 162, 235, 226, 74, 140; 91, 207, 254, 63, 63, 190]",
            "[]_{0,1}",
            "[7]",
            "[]_{0,3}",
            "[128, 4, 115; 222, 72, 91; 7, 106, 237; 154, 205, 173]",
            "[]_{3,0}",
            "[]_{1,0}",
            "[]_{0,5}",
            "[85; 237; 254; 76]",
            "[]_{0,6}",
            "[]_{0,0}",
            "[245]",
            "[]_{0,2}",
            "[173; 1]",
            "[]_{1,0}",
        ],
    );
}

#[test]
#[should_panic]
fn random_unsigned_matrices_fail_1() {
    // mean_dimension_denominator is zero
    let _ = random_unsigned_matrices::<u8>(EXAMPLE_SEED, 1, 0);
}

#[test]
#[should_panic]
fn random_unsigned_matrices_fail_2() {
    // mean_dimension_numerator is zero
    let _ = random_unsigned_matrices::<u8>(EXAMPLE_SEED, 0, 1);
}
