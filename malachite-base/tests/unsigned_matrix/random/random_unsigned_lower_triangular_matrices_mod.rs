// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::unsigned_matrix::random::random_unsigned_lower_triangular_matrices_mod;

fn random_unsigned_lower_triangular_matrices_mod_helper(
    height: u64,
    width: u64,
    unit: bool,
    m: u8,
    expected_values: &[&str],
) {
    assert_eq!(
        random_unsigned_lower_triangular_matrices_mod(EXAMPLE_SEED, height, width, unit, m)
            .map(|m| m.to_string())
            .take(20)
            .collect_vec(),
        expected_values
    );
}

#[test]
fn test_random_unsigned_lower_triangular_matrices_mod() {
    // 3-by-3, random diagonal, modulo 7
    random_unsigned_lower_triangular_matrices_mod_helper(
        3,
        3,
        false,
        7,
        &[
            "[2, 0, 0; 6, 6, 0; 6, 3, 2]",
            "[3, 0, 0; 4, 6, 0; 1, 2, 6]",
            "[5, 0, 0; 6, 1, 0; 5, 6, 1]",
            "[3, 0, 0; 5, 3, 0; 0, 4, 1]",
            "[5, 0, 0; 2, 5, 0; 2, 1, 3]",
            "[3, 0, 0; 6, 3, 0; 6, 5, 6]",
            "[1, 0, 0; 2, 6, 0; 2, 6, 2]",
            "[3, 0, 0; 5, 6, 0; 2, 6, 5]",
            "[6, 0, 0; 3, 3, 0; 6, 5, 2]",
            "[2, 0, 0; 1, 1, 0; 0, 1, 6]",
            "[4, 0, 0; 1, 4, 0; 2, 5, 6]",
            "[1, 0, 0; 2, 2, 0; 0, 6, 5]",
            "[5, 0, 0; 0, 6, 0; 0, 4, 5]",
            "[1, 0, 0; 1, 6, 0; 2, 4, 4]",
            "[1, 0, 0; 0, 5, 0; 6, 2, 4]",
            "[6, 0, 0; 4, 4, 0; 0, 3, 1]",
            "[3, 0, 0; 2, 1, 0; 2, 2, 5]",
            "[3, 0, 0; 6, 2, 0; 2, 6, 2]",
            "[4, 0, 0; 2, 5, 0; 3, 5, 1]",
            "[5, 0, 0; 2, 5, 0; 3, 1, 3]",
        ],
    );
    // 3-by-3, unit diagonal, modulo 7
    random_unsigned_lower_triangular_matrices_mod_helper(
        3,
        3,
        true,
        7,
        &[
            "[1, 0, 0; 1, 1, 0; 6, 5, 1]",
            "[1, 0, 0; 6, 1, 0; 3, 1, 1]",
            "[1, 0, 0; 2, 1, 0; 4, 5, 1]",
            "[1, 0, 0; 1, 1, 0; 2, 6, 1]",
            "[1, 0, 0; 5, 1, 0; 4, 6, 1]",
            "[1, 0, 0; 0, 1, 0; 5, 6, 1]",
            "[1, 0, 0; 0, 1, 0; 2, 5, 1]",
            "[1, 0, 0; 2, 1, 0; 0, 4, 1]",
            "[1, 0, 0; 0, 1, 0; 6, 6, 1]",
            "[1, 0, 0; 4, 1, 0; 2, 4, 1]",
            "[1, 0, 0; 2, 1, 0; 1, 2, 1]",
            "[1, 0, 0; 2, 1, 0; 6, 2, 1]",
            "[1, 0, 0; 6, 1, 0; 5, 6, 1]",
            "[1, 0, 0; 5, 1, 0; 0, 2, 1]",
            "[1, 0, 0; 6, 1, 0; 5, 2, 1]",
            "[1, 0, 0; 6, 1, 0; 1, 2, 1]",
            "[1, 0, 0; 5, 1, 0; 5, 2, 1]",
            "[1, 0, 0; 6, 1, 0; 4, 5, 1]",
            "[1, 0, 0; 3, 1, 0; 2, 6, 1]",
            "[1, 0, 0; 5, 1, 0; 6, 1, 1]",
        ],
    );
    // 2-by-4, random diagonal, modulo 2
    random_unsigned_lower_triangular_matrices_mod_helper(
        2,
        4,
        false,
        2,
        &[
            "[1, 0, 0, 0; 1, 1, 0, 0]",
            "[1, 0, 0, 0; 0, 1, 0, 0]",
            "[1, 0, 0, 0; 0, 1, 0, 0]",
            "[1, 0, 0, 0; 0, 1, 0, 0]",
            "[1, 0, 0, 0; 1, 1, 0, 0]",
            "[1, 0, 0, 0; 1, 1, 0, 0]",
            "[1, 0, 0, 0; 1, 1, 0, 0]",
            "[1, 0, 0, 0; 0, 1, 0, 0]",
            "[1, 0, 0, 0; 1, 1, 0, 0]",
            "[1, 0, 0, 0; 1, 1, 0, 0]",
            "[1, 0, 0, 0; 1, 1, 0, 0]",
            "[1, 0, 0, 0; 1, 1, 0, 0]",
            "[1, 0, 0, 0; 0, 1, 0, 0]",
            "[1, 0, 0, 0; 1, 1, 0, 0]",
            "[1, 0, 0, 0; 1, 1, 0, 0]",
            "[1, 0, 0, 0; 1, 1, 0, 0]",
            "[1, 0, 0, 0; 1, 1, 0, 0]",
            "[1, 0, 0, 0; 0, 1, 0, 0]",
            "[1, 0, 0, 0; 1, 1, 0, 0]",
            "[1, 0, 0, 0; 0, 1, 0, 0]",
        ],
    );
    // 0-by-3, modulo 1
    random_unsigned_lower_triangular_matrices_mod_helper(
        0,
        3,
        false,
        1,
        &[
            "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}",
            "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}",
            "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}",
        ],
    );
}

#[test]
#[should_panic]
fn random_unsigned_lower_triangular_matrices_mod_fail_1() {
    let _ = random_unsigned_lower_triangular_matrices_mod(EXAMPLE_SEED, 2, 3, false, 0u8);
}

#[test]
#[should_panic]
fn random_unsigned_lower_triangular_matrices_mod_fail_2() {
    let _ = random_unsigned_lower_triangular_matrices_mod(EXAMPLE_SEED, 1, 1, true, 1u8);
}
