// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::unsigned_matrix::random::random_unsigned_matrices_with_rank_mod;

fn random_unsigned_matrices_with_rank_mod_helper(
    height: u64,
    width: u64,
    rank: u64,
    m: u8,
    expected_values: &[&str],
) {
    assert_eq!(
        random_unsigned_matrices_with_rank_mod(EXAMPLE_SEED, height, width, rank, m)
            .map(|m| m.to_string())
            .take(20)
            .collect_vec(),
        expected_values
    );
}

#[test]
fn test_random_unsigned_matrices_with_rank_mod() {
    // 3-by-4, rank 2, modulo 7
    random_unsigned_matrices_with_rank_mod_helper(
        3,
        4,
        2,
        7,
        &[
            "[2, 0, 0, 0; 0, 0, 0, 0; 0, 0, 6, 0]",
            "[0, 0, 0, 1; 2, 0, 0, 0; 0, 0, 0, 0]",
            "[0, 0, 0, 0; 6, 0, 0, 0; 0, 0, 0, 3]",
            "[0, 0, 6, 0; 0, 0, 0, 0; 0, 0, 0, 3]",
            "[0, 0, 0, 0; 0, 3, 0, 0; 0, 0, 0, 2]",
            "[0, 0, 0, 0; 0, 0, 3, 0; 5, 0, 0, 0]",
            "[0, 0, 0, 0; 0, 0, 0, 5; 0, 3, 0, 0]",
            "[0, 0, 0, 0; 0, 0, 0, 6; 0, 6, 0, 0]",
            "[0, 0, 0, 0; 0, 6, 0, 0; 0, 0, 4, 0]",
            "[4, 0, 0, 0; 0, 0, 0, 0; 0, 0, 6, 0]",
            "[0, 0, 0, 5; 2, 0, 0, 0; 0, 0, 0, 0]",
            "[0, 0, 0, 2; 0, 0, 0, 0; 6, 0, 0, 0]",
            "[0, 1, 0, 0; 2, 0, 0, 0; 0, 0, 0, 0]",
            "[0, 0, 5, 0; 0, 0, 0, 0; 6, 0, 0, 0]",
            "[0, 0, 0, 1; 0, 6, 0, 0; 0, 0, 0, 0]",
            "[0, 0, 0, 4; 0, 1, 0, 0; 0, 0, 0, 0]",
            "[0, 0, 0, 0; 0, 0, 0, 1; 0, 0, 6, 0]",
            "[0, 0, 0, 3; 0, 0, 0, 0; 0, 3, 0, 0]",
            "[0, 0, 0, 0; 0, 0, 2, 0; 0, 1, 0, 0]",
            "[0, 0, 3, 0; 4, 0, 0, 0; 0, 0, 0, 0]",
        ],
    );
    // 2-by-2, rank 2, modulo 2
    random_unsigned_matrices_with_rank_mod_helper(
        2,
        2,
        2,
        2,
        &[
            "[0, 1; 1, 0]",
            "[1, 0; 0, 1]",
            "[1, 0; 0, 1]",
            "[0, 1; 1, 0]",
            "[1, 0; 0, 1]",
            "[1, 0; 0, 1]",
            "[0, 1; 1, 0]",
            "[1, 0; 0, 1]",
            "[0, 1; 1, 0]",
            "[0, 1; 1, 0]",
            "[1, 0; 0, 1]",
            "[0, 1; 1, 0]",
            "[1, 0; 0, 1]",
            "[1, 0; 0, 1]",
            "[0, 1; 1, 0]",
            "[0, 1; 1, 0]",
            "[1, 0; 0, 1]",
            "[0, 1; 1, 0]",
            "[0, 1; 1, 0]",
            "[1, 0; 0, 1]",
        ],
    );
    // 4-by-4, rank 0, modulo 1
    random_unsigned_matrices_with_rank_mod_helper(
        4,
        4,
        0,
        1,
        &[
            "[0, 0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0]",
            "[0, 0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0]",
            "[0, 0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0]",
            "[0, 0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0]",
            "[0, 0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0]",
            "[0, 0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0]",
            "[0, 0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0]",
            "[0, 0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0]",
            "[0, 0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0]",
            "[0, 0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0]",
            "[0, 0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0]",
            "[0, 0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0]",
            "[0, 0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0]",
            "[0, 0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0]",
            "[0, 0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0]",
            "[0, 0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0]",
            "[0, 0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0]",
            "[0, 0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0]",
            "[0, 0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0]",
            "[0, 0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0]",
        ],
    );
    // 3-by-3, rank 3, modulo 255
    random_unsigned_matrices_with_rank_mod_helper(
        3,
        3,
        3,
        255,
        &[
            "[0, 70, 0; 0, 0, 240; 114, 0, 0]",
            "[0, 146, 0; 0, 0, 76; 164, 0, 0]",
            "[123, 0, 0; 0, 6, 0; 0, 0, 227]",
            "[0, 21, 0; 176, 0, 0; 0, 0, 73]",
            "[0, 239, 0; 0, 0, 201; 94, 0, 0]",
            "[0, 0, 207; 0, 246, 0; 87, 0, 0]",
            "[0, 0, 93; 40, 0, 0; 0, 158, 0]",
            "[73, 0, 0; 0, 0, 152; 0, 170, 0]",
            "[153, 0, 0; 0, 0, 29; 0, 135, 0]",
            "[0, 6, 0; 245, 0, 0; 0, 0, 138]",
            "[0, 0, 8; 0, 153, 0; 191, 0, 0]",
            "[132, 0, 0; 0, 0, 30; 0, 204, 0]",
            "[165, 0, 0; 0, 0, 244; 0, 115, 0]",
            "[0, 160, 0; 239, 0, 0; 0, 0, 221]",
            "[0, 131, 0; 0, 0, 163; 227, 0, 0]",
            "[67, 0, 0; 0, 0, 199; 0, 80, 0]",
            "[0, 146, 0; 91, 0, 0; 0, 0, 223]",
            "[0, 228, 0; 173, 0, 0; 0, 0, 124]",
            "[0, 212, 0; 95, 0, 0; 0, 0, 188]",
            "[0, 241, 0; 2, 0, 0; 0, 0, 179]",
        ],
    );
}

#[test]
#[should_panic]
fn random_unsigned_matrices_with_rank_mod_fail_1() {
    let _ = random_unsigned_matrices_with_rank_mod(EXAMPLE_SEED, 2, 3, 3, 7u8);
}

#[test]
#[should_panic]
fn random_unsigned_matrices_with_rank_mod_fail_2() {
    let _ = random_unsigned_matrices_with_rank_mod(EXAMPLE_SEED, 2, 3, 0, 0u8);
}

#[test]
#[should_panic]
fn random_unsigned_matrices_with_rank_mod_fail_3() {
    let _ = random_unsigned_matrices_with_rank_mod(EXAMPLE_SEED, 2, 3, 1, 1u8);
}
