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
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::natural_matrix::random::{
    random_natural_lower_triangular_matrices_mod,
    random_natural_matrices_from_elementary_operations_mod, random_natural_matrices_with_rank_mod,
};

fn random_natural_matrices_from_elementary_operations_mod_helper<
    I: Iterator<Item = NaturalMatrix>,
>(
    xs: I,
    count: u64,
    m: &str,
    expected_values: &[&str],
) {
    assert_eq!(
        random_natural_matrices_from_elementary_operations_mod(
            EXAMPLE_SEED,
            xs,
            count,
            Natural::from_str(m).unwrap()
        )
        .map(|m| m.to_string())
        .take(20)
        .collect_vec(),
        expected_values
    );
}

#[test]
fn test_random_natural_matrices_from_elementary_operations_mod() {
    // 3-by-3 rank 2 modulo 7, 10 operations
    random_natural_matrices_from_elementary_operations_mod_helper(
        random_natural_matrices_with_rank_mod(
            EXAMPLE_SEED.fork("rank"),
            3,
            3,
            2,
            Natural::from_str("7").unwrap(),
        ),
        10,
        "7",
        &[
            "[0, 0, 0; 4, 0, 3; 5, 5, 4]",
            "[0, 0, 0; 5, 5, 4; 0, 0, 3]",
            "[0, 4, 6; 0, 0, 6; 0, 6, 6]",
            "[1, 4, 4; 1, 4, 4; 6, 3, 2]",
            "[0, 0, 2; 0, 0, 0; 1, 0, 0]",
            "[0, 2, 2; 0, 0, 0; 0, 0, 2]",
            "[0, 0, 0; 1, 5, 2; 4, 2, 5]",
            "[0, 0, 5; 0, 0, 5; 0, 6, 6]",
            "[6, 1, 0; 2, 0, 0; 1, 6, 0]",
            "[2, 2, 2; 0, 3, 0; 5, 5, 5]",
            "[4, 0, 4; 6, 3, 6; 3, 3, 3]",
            "[1, 6, 0; 5, 0, 0; 0, 0, 0]",
            "[3, 0, 2; 3, 2, 2; 4, 6, 5]",
            "[3, 0, 6; 3, 0, 5; 0, 0, 0]",
            "[6, 1, 3; 3, 1, 3; 0, 0, 0]",
            "[0, 0, 0; 0, 0, 2; 0, 1, 5]",
            "[6, 0, 1; 2, 0, 2; 1, 0, 1]",
            "[6, 1, 0; 0, 0, 0; 6, 1, 1]",
            "[2, 6, 3; 0, 0, 0; 0, 0, 5]",
            "[0, 2, 0; 2, 3, 0; 5, 2, 0]",
        ],
    );
    // 2-by-2 unit lower triangular modulo 5, 5 operations
    random_natural_matrices_from_elementary_operations_mod_helper(
        random_natural_lower_triangular_matrices_mod(
            EXAMPLE_SEED.fork("lower"),
            2,
            2,
            true,
            Natural::from_str("5").unwrap(),
        ),
        5,
        "5",
        &[
            "[3, 4; 4, 4]",
            "[1, 3; 0, 1]",
            "[2, 1; 4, 0]",
            "[2, 4; 4, 1]",
            "[1, 0; 0, 1]",
            "[1, 1; 2, 3]",
            "[0, 4; 1, 4]",
            "[0, 1; 4, 4]",
            "[1, 0; 0, 1]",
            "[1, 1; 1, 2]",
            "[1, 0; 3, 1]",
            "[0, 1; 4, 4]",
            "[1, 0; 2, 1]",
            "[3, 4; 3, 1]",
            "[1, 0; 4, 1]",
            "[4, 3; 1, 1]",
            "[1, 4; 0, 1]",
            "[1, 1; 0, 1]",
            "[1, 0; 3, 1]",
            "[4, 4; 2, 1]",
        ],
    );
    // 2-by-3 rank 1 modulo 2, no operations
    random_natural_matrices_from_elementary_operations_mod_helper(
        random_natural_matrices_with_rank_mod(
            EXAMPLE_SEED.fork("rank"),
            2,
            3,
            1,
            Natural::from_str("2").unwrap(),
        ),
        0,
        "2",
        &[
            "[0, 0, 0; 0, 1, 0]",
            "[0, 1, 0; 0, 0, 0]",
            "[0, 0, 1; 0, 0, 0]",
            "[0, 0, 0; 0, 0, 1]",
            "[0, 0, 0; 0, 0, 1]",
            "[0, 0, 1; 0, 0, 0]",
            "[1, 0, 0; 0, 0, 0]",
            "[0, 0, 0; 0, 1, 0]",
            "[0, 0, 0; 0, 0, 1]",
            "[0, 0, 1; 0, 0, 0]",
            "[1, 0, 0; 0, 0, 0]",
            "[1, 0, 0; 0, 0, 0]",
            "[0, 0, 0; 0, 1, 0]",
            "[0, 1, 0; 0, 0, 0]",
            "[1, 0, 0; 0, 0, 0]",
            "[1, 0, 0; 0, 0, 0]",
            "[0, 1, 0; 0, 0, 0]",
            "[0, 0, 1; 0, 0, 0]",
            "[0, 1, 0; 0, 0, 0]",
            "[0, 0, 0; 0, 0, 1]",
        ],
    );
}

#[test]
#[should_panic]
fn random_natural_matrices_from_elementary_operations_mod_fail_1() {
    let _ = random_natural_matrices_from_elementary_operations_mod(
        EXAMPLE_SEED,
        random_natural_matrices_with_rank_mod(
            EXAMPLE_SEED,
            2,
            2,
            1,
            Natural::from_str("7").unwrap(),
        ),
        1,
        Natural::from_str("0").unwrap(),
    );
}

#[test]
#[should_panic]
fn random_natural_matrices_from_elementary_operations_mod_fail_2() {
    // The elements are reduced modulo 7 but not modulo 3.
    let _ = random_natural_matrices_from_elementary_operations_mod(
        EXAMPLE_SEED,
        random_natural_matrices_with_rank_mod(
            EXAMPLE_SEED,
            2,
            2,
            2,
            Natural::from_str("7").unwrap(),
        )
        .skip_while(|a| a.elements.iter().all(|x| *x < 3u32)),
        1,
        Natural::from_str("3").unwrap(),
    )
    .next();
}
