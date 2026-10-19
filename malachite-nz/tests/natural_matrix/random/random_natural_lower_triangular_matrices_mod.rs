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
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::random::random_natural_lower_triangular_matrices_mod;

fn random_natural_lower_triangular_matrices_mod_helper(
    height: u64,
    width: u64,
    unit: bool,
    m: &str,
    expected_values: &[&str],
) {
    assert_eq!(
        random_natural_lower_triangular_matrices_mod(
            EXAMPLE_SEED,
            height,
            width,
            unit,
            Natural::from_str(m).unwrap()
        )
        .map(|m| m.to_string())
        .take(20)
        .collect_vec(),
        expected_values
    );
}

#[test]
fn test_random_natural_lower_triangular_matrices_mod() {
    // 3-by-3, random diagonal, modulo 7
    random_natural_lower_triangular_matrices_mod_helper(
        3,
        3,
        false,
        "7",
        &[
            "[2, 0, 0; 5, 6, 0; 1, 2, 1]",
            "[3, 0, 0; 3, 5, 0; 3, 6, 6]",
            "[4, 0, 0; 1, 4, 0; 5, 1, 1]",
            "[2, 0, 0; 6, 6, 0; 5, 4, 2]",
            "[5, 0, 0; 0, 5, 0; 2, 5, 6]",
            "[4, 0, 0; 1, 2, 0; 3, 3, 3]",
            "[1, 0, 0; 5, 4, 0; 3, 4, 4]",
            "[2, 0, 0; 0, 2, 0; 2, 2, 2]",
            "[1, 0, 0; 4, 1, 0; 5, 6, 4]",
            "[6, 0, 0; 6, 5, 0; 6, 2, 4]",
            "[2, 0, 0; 3, 5, 0; 2, 6, 1]",
            "[3, 0, 0; 2, 4, 0; 1, 6, 3]",
            "[4, 0, 0; 6, 1, 0; 3, 4, 2]",
            "[1, 0, 0; 2, 5, 0; 4, 3, 1]",
            "[2, 0, 0; 0, 5, 0; 1, 0, 2]",
            "[3, 0, 0; 2, 3, 0; 2, 3, 5]",
            "[6, 0, 0; 6, 1, 0; 5, 2, 5]",
            "[1, 0, 0; 1, 1, 0; 5, 1, 3]",
            "[3, 0, 0; 4, 1, 0; 3, 6, 3]",
            "[5, 0, 0; 4, 5, 0; 3, 1, 4]",
        ],
    );
    // 3-by-3, unit diagonal, modulo 7
    random_natural_lower_triangular_matrices_mod_helper(
        3,
        3,
        true,
        "7",
        &[
            "[1, 0, 0; 1, 1, 0; 5, 5, 1]",
            "[1, 0, 0; 1, 1, 0; 2, 0, 1]",
            "[1, 0, 0; 2, 1, 0; 3, 4, 1]",
            "[1, 0, 0; 3, 1, 0; 6, 5, 1]",
            "[1, 0, 0; 6, 1, 0; 6, 3, 1]",
            "[1, 0, 0; 1, 1, 0; 3, 5, 1]",
            "[1, 0, 0; 1, 1, 0; 0, 1, 1]",
            "[1, 0, 0; 6, 1, 0; 6, 6, 1]",
            "[1, 0, 0; 5, 1, 0; 5, 4, 1]",
            "[1, 0, 0; 1, 1, 0; 4, 0, 1]",
            "[1, 0, 0; 4, 1, 0; 2, 5, 1]",
            "[1, 0, 0; 5, 1, 0; 3, 1, 1]",
            "[1, 0, 0; 1, 1, 0; 3, 3, 1]",
            "[1, 0, 0; 2, 1, 0; 0, 5, 1]",
            "[1, 0, 0; 3, 1, 0; 3, 4, 1]",
            "[1, 0, 0; 3, 1, 0; 1, 0, 1]",
            "[1, 0, 0; 1, 1, 0; 2, 2, 1]",
            "[1, 0, 0; 6, 1, 0; 1, 0, 1]",
            "[1, 0, 0; 4, 1, 0; 0, 5, 1]",
            "[1, 0, 0; 6, 1, 0; 3, 5, 1]",
        ],
    );
    // 2-by-4, random diagonal, modulo 2
    random_natural_lower_triangular_matrices_mod_helper(
        2,
        4,
        false,
        "2",
        &[
            "[1, 0, 0, 0; 1, 1, 0, 0]",
            "[1, 0, 0, 0; 1, 1, 0, 0]",
            "[1, 0, 0, 0; 1, 1, 0, 0]",
            "[1, 0, 0, 0; 1, 1, 0, 0]",
            "[1, 0, 0, 0; 1, 1, 0, 0]",
            "[1, 0, 0, 0; 1, 1, 0, 0]",
            "[1, 0, 0, 0; 0, 1, 0, 0]",
            "[1, 0, 0, 0; 0, 1, 0, 0]",
            "[1, 0, 0, 0; 0, 1, 0, 0]",
            "[1, 0, 0, 0; 1, 1, 0, 0]",
            "[1, 0, 0, 0; 0, 1, 0, 0]",
            "[1, 0, 0, 0; 1, 1, 0, 0]",
            "[1, 0, 0, 0; 0, 1, 0, 0]",
            "[1, 0, 0, 0; 1, 1, 0, 0]",
            "[1, 0, 0, 0; 0, 1, 0, 0]",
            "[1, 0, 0, 0; 0, 1, 0, 0]",
            "[1, 0, 0, 0; 1, 1, 0, 0]",
            "[1, 0, 0, 0; 1, 1, 0, 0]",
            "[1, 0, 0, 0; 1, 1, 0, 0]",
            "[1, 0, 0, 0; 1, 1, 0, 0]",
        ],
    );
    // 0-by-3, modulo 1
    random_natural_lower_triangular_matrices_mod_helper(
        0,
        3,
        false,
        "1",
        &[
            "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}",
            "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}",
            "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}",
        ],
    );
}

#[test]
#[should_panic]
fn random_natural_lower_triangular_matrices_mod_fail_1() {
    let _ = random_natural_lower_triangular_matrices_mod(
        EXAMPLE_SEED,
        2,
        3,
        false,
        Natural::from_str("0").unwrap(),
    );
}

#[test]
#[should_panic]
fn random_natural_lower_triangular_matrices_mod_fail_2() {
    let _ = random_natural_lower_triangular_matrices_mod(
        EXAMPLE_SEED,
        1,
        1,
        true,
        Natural::from_str("1").unwrap(),
    );
}
