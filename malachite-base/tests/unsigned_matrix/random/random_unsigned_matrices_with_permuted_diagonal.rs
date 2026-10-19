// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::unsigned_matrix::random::random_unsigned_matrices_with_permuted_diagonal;

fn random_unsigned_matrices_with_permuted_diagonal_helper(
    height: u64,
    width: u64,
    diagonal: &[u8],
    expected_values: &[&str],
) {
    assert_eq!(
        random_unsigned_matrices_with_permuted_diagonal(
            EXAMPLE_SEED,
            height,
            width,
            diagonal.to_vec()
        )
        .map(|(m, odd)| format!("({m}, {odd})"))
        .take(20)
        .collect_vec(),
        expected_values
    );
}

#[test]
fn test_random_unsigned_matrices_with_permuted_diagonal() {
    // 3-by-4, diagonal (2, 3, 5)
    random_unsigned_matrices_with_permuted_diagonal_helper(
        3,
        4,
        &[2, 3, 5],
        &[
            "([0, 3, 0, 0; 0, 0, 0, 5; 2, 0, 0, 0], true)",
            "([0, 2, 0, 0; 3, 0, 0, 0; 0, 0, 5, 0], true)",
            "([0, 0, 5, 0; 0, 3, 0, 0; 0, 0, 0, 2], false)",
            "([0, 2, 0, 0; 0, 0, 0, 3; 0, 0, 5, 0], false)",
            "([0, 0, 2, 0; 3, 0, 0, 0; 0, 0, 0, 5], true)",
            "([0, 0, 2, 0; 5, 0, 0, 0; 0, 0, 0, 3], true)",
            "([0, 0, 3, 0; 0, 2, 0, 0; 5, 0, 0, 0], true)",
            "([3, 0, 0, 0; 0, 2, 0, 0; 0, 0, 5, 0], false)",
            "([0, 3, 0, 0; 0, 0, 0, 5; 0, 0, 2, 0], false)",
            "([0, 5, 0, 0; 2, 0, 0, 0; 0, 0, 3, 0], true)",
            "([0, 0, 0, 2; 0, 0, 5, 0; 3, 0, 0, 0], true)",
            "([0, 0, 0, 3; 0, 5, 0, 0; 2, 0, 0, 0], false)",
            "([0, 0, 0, 3; 0, 0, 2, 0; 0, 5, 0, 0], false)",
            "([3, 0, 0, 0; 0, 0, 5, 0; 0, 2, 0, 0], true)",
            "([2, 0, 0, 0; 0, 0, 0, 3; 0, 0, 5, 0], true)",
            "([0, 0, 2, 0; 0, 0, 0, 5; 0, 3, 0, 0], true)",
            "([0, 0, 5, 0; 3, 0, 0, 0; 0, 2, 0, 0], false)",
            "([3, 0, 0, 0; 0, 5, 0, 0; 0, 0, 2, 0], false)",
            "([2, 0, 0, 0; 0, 5, 0, 0; 0, 0, 0, 3], true)",
            "([0, 0, 0, 2; 5, 0, 0, 0; 0, 3, 0, 0], true)",
        ],
    );
    // 2-by-2, diagonal (7, 1)
    random_unsigned_matrices_with_permuted_diagonal_helper(
        2,
        2,
        &[7, 1],
        &[
            "([0, 7; 1, 0], true)",
            "([1, 0; 0, 7], false)",
            "([7, 0; 0, 1], false)",
            "([0, 7; 1, 0], true)",
            "([7, 0; 0, 1], false)",
            "([7, 0; 0, 1], false)",
            "([0, 1; 7, 0], true)",
            "([7, 0; 0, 1], false)",
            "([0, 7; 1, 0], true)",
            "([0, 7; 1, 0], true)",
            "([1, 0; 0, 7], false)",
            "([0, 7; 1, 0], true)",
            "([1, 0; 0, 7], false)",
            "([7, 0; 0, 1], false)",
            "([0, 1; 7, 0], true)",
            "([0, 7; 1, 0], true)",
            "([1, 0; 0, 7], false)",
            "([0, 7; 1, 0], true)",
            "([0, 1; 7, 0], true)",
            "([7, 0; 0, 1], false)",
        ],
    );
    // 4-by-2, empty diagonal
    random_unsigned_matrices_with_permuted_diagonal_helper(
        4,
        2,
        &[],
        &[
            "([0, 0; 0, 0; 0, 0; 0, 0], false)",
            "([0, 0; 0, 0; 0, 0; 0, 0], true)",
            "([0, 0; 0, 0; 0, 0; 0, 0], false)",
            "([0, 0; 0, 0; 0, 0; 0, 0], false)",
            "([0, 0; 0, 0; 0, 0; 0, 0], false)",
            "([0, 0; 0, 0; 0, 0; 0, 0], true)",
            "([0, 0; 0, 0; 0, 0; 0, 0], true)",
            "([0, 0; 0, 0; 0, 0; 0, 0], false)",
            "([0, 0; 0, 0; 0, 0; 0, 0], false)",
            "([0, 0; 0, 0; 0, 0; 0, 0], false)",
            "([0, 0; 0, 0; 0, 0; 0, 0], false)",
            "([0, 0; 0, 0; 0, 0; 0, 0], false)",
            "([0, 0; 0, 0; 0, 0; 0, 0], true)",
            "([0, 0; 0, 0; 0, 0; 0, 0], true)",
            "([0, 0; 0, 0; 0, 0; 0, 0], false)",
            "([0, 0; 0, 0; 0, 0; 0, 0], false)",
            "([0, 0; 0, 0; 0, 0; 0, 0], true)",
            "([0, 0; 0, 0; 0, 0; 0, 0], true)",
            "([0, 0; 0, 0; 0, 0; 0, 0], false)",
            "([0, 0; 0, 0; 0, 0; 0, 0], true)",
        ],
    );
    // 1-by-3, diagonal (10)
    random_unsigned_matrices_with_permuted_diagonal_helper(
        1,
        3,
        &[10],
        &[
            "([0, 0, 10], false)",
            "([10, 0, 0], false)",
            "([10, 0, 0], true)",
            "([10, 0, 0], true)",
            "([10, 0, 0], true)",
            "([0, 10, 0], false)",
            "([0, 0, 10], false)",
            "([10, 0, 0], false)",
            "([10, 0, 0], false)",
            "([0, 10, 0], false)",
            "([0, 0, 10], false)",
            "([10, 0, 0], true)",
            "([0, 0, 10], true)",
            "([10, 0, 0], false)",
            "([0, 10, 0], false)",
            "([10, 0, 0], true)",
            "([10, 0, 0], false)",
            "([0, 10, 0], false)",
            "([0, 10, 0], true)",
            "([0, 10, 0], true)",
        ],
    );
}

#[test]
#[should_panic]
fn random_unsigned_matrices_with_permuted_diagonal_fail_1() {
    let _ = random_unsigned_matrices_with_permuted_diagonal(EXAMPLE_SEED, 2, 3, vec![1u8; 3]);
}
