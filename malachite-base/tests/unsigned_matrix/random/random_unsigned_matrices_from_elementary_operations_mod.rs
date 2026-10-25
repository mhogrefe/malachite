// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::matrix::Matrix;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_base::unsigned_matrix::random::{
    random_unsigned_lower_triangular_matrices_mod,
    random_unsigned_matrices_from_elementary_operations_mod,
    random_unsigned_matrices_with_rank_mod,
};

fn random_unsigned_matrices_from_elementary_operations_mod_helper<
    I: Iterator<Item = UnsignedMatrix<u8>>,
>(
    xs: I,
    count: u64,
    m: u8,
    expected_values: &[&str],
) {
    assert_eq!(
        random_unsigned_matrices_from_elementary_operations_mod(EXAMPLE_SEED, xs, count, m)
            .map(|m| m.to_string())
            .take(20)
            .collect_vec(),
        expected_values
    );
}

#[test]
fn test_random_unsigned_matrices_from_elementary_operations_mod() {
    // 3-by-3 rank 2 modulo 7, 10 operations
    random_unsigned_matrices_from_elementary_operations_mod_helper(
        random_unsigned_matrices_with_rank_mod(EXAMPLE_SEED.fork("rank"), 3, 3, 2, 7u8),
        10,
        7,
        &[
            "[3, 3, 1; 6, 6, 2; 3, 3, 6]",
            "[5, 5, 0; 4, 0, 4; 0, 0, 0]",
            "[5, 4, 2; 0, 2, 0; 4, 5, 3]",
            "[1, 4, 4; 3, 6, 6; 3, 0, 0]",
            "[0, 0, 6; 0, 6, 5; 0, 0, 0]",
            "[1, 2, 2; 0, 0, 2; 1, 2, 0]",
            "[1, 1, 0; 5, 1, 0; 0, 0, 0]",
            "[0, 0, 1; 0, 0, 1; 2, 0, 5]",
            "[3, 4, 0; 5, 4, 0; 0, 0, 0]",
            "[0, 0, 0; 1, 3, 1; 0, 2, 0]",
            "[0, 5, 1; 0, 6, 5; 0, 2, 6]",
            "[1, 1, 0; 0, 0, 0; 1, 6, 0]",
            "[3, 4, 4; 0, 0, 0; 0, 5, 5]",
            "[5, 0, 3; 5, 3, 0; 0, 4, 3]",
            "[0, 0, 0; 6, 1, 0; 3, 0, 0]",
            "[0, 0, 3; 0, 2, 4; 0, 5, 3]",
            "[1, 3, 1; 1, 0, 1; 1, 0, 1]",
            "[5, 2, 1; 0, 0, 0; 5, 2, 6]",
            "[0, 6, 0; 6, 0, 0; 0, 0, 0]",
            "[5, 0, 5; 4, 0, 4; 6, 6, 5]",
        ],
    );
    // 2-by-2 unit lower triangular modulo 5, 5 operations
    random_unsigned_matrices_from_elementary_operations_mod_helper(
        random_unsigned_lower_triangular_matrices_mod(EXAMPLE_SEED.fork("lower"), 2, 2, true, 5u8),
        5,
        5,
        &[
            "[2, 4; 3, 4]",
            "[4, 0; 2, 4]",
            "[2, 1; 2, 4]",
            "[3, 4; 3, 1]",
            "[1, 0; 3, 1]",
            "[1, 1; 2, 3]",
            "[2, 2; 1, 4]",
            "[0, 1; 4, 3]",
            "[4, 3; 2, 3]",
            "[1, 1; 3, 4]",
            "[1, 0; 1, 1]",
            "[2, 1; 2, 4]",
            "[1, 0; 0, 1]",
            "[4, 4; 2, 1]",
            "[1, 0; 2, 1]",
            "[2, 3; 2, 1]",
            "[1, 4; 0, 1]",
            "[4, 1; 3, 1]",
            "[1, 0; 2, 1]",
            "[2, 4; 4, 1]",
        ],
    );
    // 2-by-3 rank 1 modulo 2, no operations
    random_unsigned_matrices_from_elementary_operations_mod_helper(
        random_unsigned_matrices_with_rank_mod(EXAMPLE_SEED.fork("rank"), 2, 3, 1, 2u8),
        0,
        2,
        &[
            "[0, 1, 0; 0, 0, 0]",
            "[0, 0, 0; 1, 0, 0]",
            "[0, 0, 0; 0, 0, 1]",
            "[0, 0, 0; 0, 1, 0]",
            "[0, 1, 0; 0, 0, 0]",
            "[0, 0, 0; 0, 0, 1]",
            "[0, 0, 1; 0, 0, 0]",
            "[0, 0, 1; 0, 0, 0]",
            "[0, 0, 0; 0, 0, 1]",
            "[0, 0, 1; 0, 0, 0]",
            "[0, 1, 0; 0, 0, 0]",
            "[1, 0, 0; 0, 0, 0]",
            "[0, 0, 0; 1, 0, 0]",
            "[0, 0, 0; 0, 1, 0]",
            "[0, 1, 0; 0, 0, 0]",
            "[0, 0, 0; 0, 1, 0]",
            "[0, 0, 0; 0, 0, 1]",
            "[0, 0, 1; 0, 0, 0]",
            "[0, 0, 0; 0, 1, 0]",
            "[0, 0, 0; 1, 0, 0]",
        ],
    );
}

#[test]
#[should_panic]
fn random_unsigned_matrices_from_elementary_operations_mod_fail_1() {
    let _ = random_unsigned_matrices_from_elementary_operations_mod(
        EXAMPLE_SEED,
        random_unsigned_matrices_with_rank_mod(EXAMPLE_SEED, 2, 2, 1, 7u8),
        1,
        0u8,
    );
}

#[test]
#[should_panic]
fn random_unsigned_matrices_from_elementary_operations_mod_fail_2() {
    // The elements are reduced modulo 7 but not modulo 3.
    let _ = random_unsigned_matrices_from_elementary_operations_mod(
        EXAMPLE_SEED,
        random_unsigned_matrices_with_rank_mod(EXAMPLE_SEED, 2, 2, 2, 7u8)
            .skip_while(|a| a.elements_ref().iter().all(|x| *x < 3)),
        1,
        3u8,
    )
    .next();
}
