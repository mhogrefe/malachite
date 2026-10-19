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
use malachite_nz::integer_matrix::random::random_integer_matrices_with_determinant;

fn random_integer_matrices_with_determinant_helper(
    size: u64,
    determinant: &str,
    expected_values: &[&str],
) {
    assert_eq!(
        random_integer_matrices_with_determinant(
            EXAMPLE_SEED,
            size,
            &Integer::from_str(determinant).unwrap()
        )
        .map(|m| m.to_string())
        .take(20)
        .collect_vec(),
        expected_values
    );
}

#[test]
fn test_random_integer_matrices_with_determinant() {
    // size 3, determinant -60
    random_integer_matrices_with_determinant_helper(
        3,
        "-60",
        &[
            "[6, 0, 0; 0, 0, -5; 0, -2, 0]",
            "[-5, 0, 0; 0, -3, 0; 0, 0, -4]",
            "[0, 2, 0; 0, 0, 5; -6, 0, 0]",
            "[3, 0, 0; 0, -1, 0; 0, 0, 20]",
            "[3, 0, 0; 0, 0, -5; 0, -4, 0]",
            "[0, 0, -2; -30, 0, 0; 0, -1, 0]",
            "[0, 3, 0; 0, 0, 2; -10, 0, 0]",
            "[0, 2, 0; 1, 0, 0; 0, 0, 30]",
            "[0, 0, -2; -5, 0, 0; 0, -6, 0]",
            "[10, 0, 0; 0, -3, 0; 0, 0, 2]",
            "[0, -1, 0; 30, 0, 0; 0, 0, -2]",
            "[0, 3, 0; 0, 0, 1; -20, 0, 0]",
            "[0, 15, 0; 2, 0, 0; 0, 0, 2]",
            "[0, 6, 0; 1, 0, 0; 0, 0, 10]",
            "[15, 0, 0; 0, 0, -2; 0, -2, 0]",
            "[0, 30, 0; 2, 0, 0; 0, 0, 1]",
            "[0, 0, -2; 0, 30, 0; -1, 0, 0]",
            "[-30, 0, 0; 0, 0, 2; 0, -1, 0]",
            "[1, 0, 0; 0, 0, -20; 0, -3, 0]",
            "[0, 2, 0; 0, 0, 15; -2, 0, 0]",
        ],
    );
    // size 2, determinant 1
    random_integer_matrices_with_determinant_helper(
        2,
        "1",
        &[
            "[-1, 0; 0, -1]",
            "[-1, 0; 0, -1]",
            "[0, 1; -1, 0]",
            "[0, 1; -1, 0]",
            "[0, -1; 1, 0]",
            "[0, 1; -1, 0]",
            "[0, 1; -1, 0]",
            "[-1, 0; 0, -1]",
            "[0, 1; -1, 0]",
            "[1, 0; 0, 1]",
            "[1, 0; 0, 1]",
            "[0, -1; 1, 0]",
            "[0, -1; 1, 0]",
            "[-1, 0; 0, -1]",
            "[1, 0; 0, 1]",
            "[0, 1; -1, 0]",
            "[1, 0; 0, 1]",
            "[1, 0; 0, 1]",
            "[0, 1; -1, 0]",
            "[0, -1; 1, 0]",
        ],
    );
    // size 4, determinant 0
    random_integer_matrices_with_determinant_helper(
        4,
        "0",
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
    // size 1, determinant 13
    random_integer_matrices_with_determinant_helper(
        1,
        "13",
        &[
            "[13]", "[13]", "[13]", "[13]", "[13]", "[13]", "[13]", "[13]", "[13]", "[13]", "[13]",
            "[13]", "[13]", "[13]", "[13]", "[13]", "[13]", "[13]", "[13]", "[13]",
        ],
    );
    // size 0, determinant 1
    random_integer_matrices_with_determinant_helper(
        0,
        "1",
        &[
            "[]_{0,0}", "[]_{0,0}", "[]_{0,0}", "[]_{0,0}", "[]_{0,0}", "[]_{0,0}", "[]_{0,0}",
            "[]_{0,0}", "[]_{0,0}", "[]_{0,0}", "[]_{0,0}", "[]_{0,0}", "[]_{0,0}", "[]_{0,0}",
            "[]_{0,0}", "[]_{0,0}", "[]_{0,0}", "[]_{0,0}", "[]_{0,0}", "[]_{0,0}",
        ],
    );
}

#[test]
#[should_panic]
fn random_integer_matrices_with_determinant_fail_1() {
    let _ =
        random_integer_matrices_with_determinant(EXAMPLE_SEED, 0, &Integer::from_str("2").unwrap());
}

#[test]
#[should_panic]
fn random_integer_matrices_with_determinant_fail_2() {
    let _ = random_integer_matrices_with_determinant(
        EXAMPLE_SEED,
        2,
        &Integer::from_str("18446744073709551616").unwrap(),
    );
}
