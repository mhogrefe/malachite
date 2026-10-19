// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::integer_matrix::random::{
    random_integer_matrices_from_elementary_operations, random_integer_matrices_with_determinant,
    random_integer_matrices_with_dimensions, random_integer_matrices_with_rank,
};

fn random_integer_matrices_from_elementary_operations_helper<I: Iterator<Item = IntegerMatrix>>(
    xs: I,
    count: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        random_integer_matrices_from_elementary_operations(EXAMPLE_SEED, xs, count)
            .map(|m| m.to_string())
            .take(20)
            .collect_vec(),
        expected_values
    );
}

#[test]
fn test_random_integer_matrices_from_elementary_operations() {
    // 3-by-3 rank 2, 10 operations
    random_integer_matrices_from_elementary_operations_helper(
        random_integer_matrices_with_rank(EXAMPLE_SEED.fork("rank"), 3, 3, 2, 2),
        10,
        &[
            "[-2, 0, 2; -7, -3, 10; -2, 0, 2]",
            "[-2, 0, -2; 3, 3, 0; -3, -3, 0]",
            "[-2, 11, 2; -2, 2, 2; -2, 5, 2]",
            "[0, 0, -2; -3, -3, -5; -6, -6, -4]",
            "[3, 0, 0; 0, -3, -6; 0, 0, 0]",
            "[3, 3, 0; 0, -3, 0; 0, 0, 0]",
            "[-2, -2, 0; -8, -5, 3; 0, 0, 0]",
            "[0, 0, 0; 0, 0, -2; 3, 0, 1]",
            "[2, -2, 0; 2, 1, 3; 0, 0, 0]",
            "[-2, -2, -2; 0, -3, 0; 2, 2, 2]",
            "[3, -3, 3; -9, 6, -9; -3, 3, -3]",
            "[-3, 0, 0; 2, -2, 0; 0, 0, 0]",
            "[3, 6, 0; 0, 0, 0; -3, -3, 0]",
            "[0, 0, -2; 0, -2, 0; 0, 0, 0]",
            "[6, -6, 3; 6, -6, 3; 3, 0, 0]",
            "[0, -3, 0; 0, 3, 0; 0, -3, -2]",
            "[-3, -3, 0; 0, -3, 0; 0, -3, 0]",
            "[2, -2, 0; -3, 0, 0; 1, 2, 0]",
            "[8, -4, -3; 0, 0, 0; 4, -2, 0]",
            "[3, 0, 3; -6, 0, -6; 1, 0, 3]",
        ],
    );
    // 2-by-2 determinant 6, 5 operations
    random_integer_matrices_from_elementary_operations_helper(
        random_integer_matrices_with_determinant(
            EXAMPLE_SEED.fork("determinant"),
            2,
            &Integer::from_str("6").unwrap(),
        ),
        5,
        &[
            "[2, 3; 2, 6]",
            "[2, 1; -2, 2]",
            "[6, 3; -10, -4]",
            "[-7, -1; 6, 0]",
            "[0, 6; -1, 0]",
            "[3, 3; 3, 5]",
            "[6, -6; 0, 1]",
            "[0, 6; -1, 1]",
            "[5, 11; -6, -12]",
            "[0, -3; 2, 2]",
            "[-2, -2; 3, 0]",
            "[1, -12; -1, 18]",
            "[0, 6; -1, 0]",
            "[-2, -3; 2, 0]",
            "[1, -1; 3, 3]",
            "[-1, -12; 1, 6]",
            "[1, -6; 0, 6]",
            "[2, 6; 1, 6]",
            "[-3, 0; -2, -2]",
            "[5, 3; -2, 0]",
        ],
    );
    // 2-by-3, no operations
    random_integer_matrices_from_elementary_operations_helper(
        random_integer_matrices_with_dimensions(EXAMPLE_SEED.fork("dimensions"), 2, 3, 2, 1),
        0,
        &[
            "[3, 3, -1; 2, 2, 0]",
            "[2, 0, -27; 0, -2, 0]",
            "[12, -1194, 0; 0, 71, 0]",
            "[6, 0, 1; 51, -1, 1]",
            "[-3, 26, -119; 9, -3, 104]",
            "[47, 17, -1; 0, -1, 36]",
            "[-8, 3233, 3; 1, 1, -3]",
            "[-6, 0, -20; 0, 1, -44]",
            "[-1, 1, -2; 15, -462, 0]",
            "[-31, -26, 1; 3, 0, 0]",
            "[3, -7, -38; -3, -6, 6]",
            "[0, -28, 1; 8, -1, -504]",
            "[-9, -1, 0; 0, 0, -5]",
            "[8, 2, 1; -65, 0, -4]",
            "[0, 0, 1; -4, -5, -2]",
            "[-93, -1, 8; 0, -17, 3]",
            "[-12, 1, 0; 3, -2, -3]",
            "[1, -16, 0; 0, -6, -2]",
            "[-47, 1, -193; -1, 0, 0]",
            "[0, 5, -3; -13, 10, -1]",
        ],
    );
    // 3-by-0, 4 operations
    random_integer_matrices_from_elementary_operations_helper(
        random_integer_matrices_with_dimensions(EXAMPLE_SEED.fork("dimensions"), 3, 0, 2, 1),
        4,
        &[
            "[]_{3,0}", "[]_{3,0}", "[]_{3,0}", "[]_{3,0}", "[]_{3,0}", "[]_{3,0}", "[]_{3,0}",
            "[]_{3,0}", "[]_{3,0}", "[]_{3,0}", "[]_{3,0}", "[]_{3,0}", "[]_{3,0}", "[]_{3,0}",
            "[]_{3,0}", "[]_{3,0}", "[]_{3,0}", "[]_{3,0}", "[]_{3,0}", "[]_{3,0}",
        ],
    );
}
