// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::unsigned_matrix::random::random_unsigned_square_matrices;

fn random_unsigned_square_matrices_helper(
    mean_size_numerator: u64,
    mean_size_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        (random_unsigned_square_matrices::<u8>(
            EXAMPLE_SEED,
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
fn test_random_unsigned_square_matrices() {
    // mean size = 1/2
    random_unsigned_square_matrices_helper(
        1,
        2,
        &[
            "[]_{0,0}",
            "[164]",
            "[]_{0,0}",
            "[76, 94; 126, 28]",
            "[]_{0,0}",
            "[]_{0,0}",
            "[]_{0,0}",
            "[]_{0,0}",
            "[110]",
            "[175]",
            "[]_{0,0}",
            "[]_{0,0}",
            "[17]",
            "[]_{0,0}",
            "[]_{0,0}",
            "[11]",
            "[238]",
            "[]_{0,0}",
            "[]_{0,0}",
            "[]_{0,0}",
        ],
    );
    // mean size = 1/1
    random_unsigned_square_matrices_helper(
        1,
        1,
        &[
            "[]_{0,0}",
            "[]_{0,0}",
            "[]_{0,0}",
            "[164]",
            "[]_{0,0}",
            "[76, 94; 126, 28]",
            "[]_{0,0}",
            "[]_{0,0}",
            "[110]",
            "[175, 17, 11, 238, 65, 62; 225, 188, 65, 75, 96, 154; 151, 17, 31, 199, 37, 162; 235, \
            226, 74, 140, 91, 207; 254, 63, 63, 190, 7, 128; 4, 115, 222, 72, 91, 7]",
            "[106]",
            "[237, 154, 205, 173, 85; 237, 254, 76, 245, 173; 1, 14, 168, 53, 140; 232, 61, 210, \
            126, 127; 218, 157, 11, 247, 194]",
            "[255]",
            "[245]",
            "[188, 156; 73, 18]",
            "[179, 9; 229, 126]",
            "[191, 155, 237, 131; 59, 117, 71, 66; 156, 15, 21, 80; 6, 206, 37, 200]",
            "[73, 173; 15, 88]",
            "[19, 155, 123, 110; 155, 156, 193, 28; 73, 75, 115, 245; 96, 55, 238, 94]",
            "[]_{0,0}",
        ],
    );
    // mean size = 3/2
    random_unsigned_square_matrices_helper(
        3,
        2,
        &[
            "[]_{0,0}",
            "[]_{0,0}",
            "[164, 76, 94, 126, 28, 110; 175, 17, 11, 238, 65, 62; 225, 188, 65, 75, 96, 154; 151, \
            17, 31, 199, 37, 162; 235, 226, 74, 140, 91, 207; 254, 63, 63, 190, 7, 128]",
            "[]_{0,0}",
            "[4, 115; 222, 72]",
            "[91, 7; 106, 237]",
            "[154]",
            "[]_{0,0}",
            "[205, 173, 85; 237, 254, 76; 245, 173, 1]",
            "[]_{0,0}",
            "[14, 168, 53, 140, 232; 61, 210, 126, 127, 218; 157, 11, 247, 194, 255; 245, 188, \
            156, 73, 18; 179, 9, 229, 126, 191]",
            "[155, 237, 131, 59, 117, 71; 66, 156, 15, 21, 80, 6; 206, 37, 200, 73, 173, 15; 88, \
            19, 155, 123, 110, 155; 156, 193, 28, 73, 75, 115; 245, 96, 55, 238, 94, 208]",
            "[]_{0,0}",
            "[32]",
            "[180]",
            "[127]",
            "[]_{0,0}",
            "[94, 198, 216; 198, 46, 156; 106, 234, 186]",
            "[231, 60, 27, 191; 100, 14, 158, 176; 238, 204, 121, 18; 86, 174, 188, 164]",
            "[217, 116, 165; 95, 251, 86; 61, 169, 71]",
        ],
    );
}

#[test]
#[should_panic]
fn random_unsigned_square_matrices_fail_1() {
    // mean_size_denominator is zero
    let _ = random_unsigned_square_matrices::<u8>(EXAMPLE_SEED, 1, 0);
}

#[test]
#[should_panic]
fn random_unsigned_square_matrices_fail_2() {
    // mean_size_numerator is zero
    let _ = random_unsigned_square_matrices::<u8>(EXAMPLE_SEED, 0, 1);
}
