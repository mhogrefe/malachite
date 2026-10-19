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
use malachite_nz::natural_matrix::random::random_natural_matrices_with_rank_mod;

fn random_natural_matrices_with_rank_mod_helper(
    height: u64,
    width: u64,
    rank: u64,
    m: &str,
    expected_values: &[&str],
) {
    assert_eq!(
        random_natural_matrices_with_rank_mod(
            EXAMPLE_SEED,
            height,
            width,
            rank,
            Natural::from_str(m).unwrap()
        )
        .map(|m| m.to_string())
        .take(20)
        .collect_vec(),
        expected_values
    );
}

#[test]
fn test_random_natural_matrices_with_rank_mod() {
    // 3-by-4, rank 2, modulo 7
    random_natural_matrices_with_rank_mod_helper(
        3,
        4,
        2,
        "7",
        &[
            "[0, 0, 0, 0; 3, 0, 0, 0; 0, 0, 0, 1]",
            "[3, 0, 0, 0; 0, 0, 0, 0; 0, 0, 5, 0]",
            "[0, 0, 0, 0; 0, 4, 0, 0; 0, 0, 0, 1]",
            "[0, 0, 0, 0; 0, 0, 0, 2; 0, 3, 0, 0]",
            "[0, 4, 0, 0; 0, 0, 3, 0; 0, 0, 0, 0]",
            "[1, 0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 1]",
            "[0, 0, 0, 0; 0, 0, 6, 0; 0, 0, 0, 6]",
            "[0, 0, 0, 0; 0, 0, 5, 0; 0, 3, 0, 0]",
            "[0, 2, 0, 0; 0, 0, 0, 5; 0, 0, 0, 0]",
            "[0, 5, 0, 0; 4, 0, 0, 0; 0, 0, 0, 0]",
            "[0, 0, 0, 1; 0, 0, 1, 0; 0, 0, 0, 0]",
            "[0, 0, 0, 0; 0, 6, 0, 0; 0, 0, 6, 0]",
            "[0, 0, 0, 6; 0, 0, 0, 0; 0, 5, 0, 0]",
            "[0, 0, 0, 5; 0, 0, 0, 0; 0, 0, 3, 0]",
            "[0, 0, 0, 0; 1, 0, 0, 0; 0, 4, 0, 0]",
            "[0, 0, 4, 0; 0, 0, 0, 0; 3, 0, 0, 0]",
            "[0, 3, 0, 0; 0, 0, 0, 0; 0, 0, 6, 0]",
            "[0, 0, 0, 0; 2, 0, 0, 0; 0, 0, 3, 0]",
            "[1, 0, 0, 0; 0, 4, 0, 0; 0, 0, 0, 0]",
            "[0, 0, 0, 6; 5, 0, 0, 0; 0, 0, 0, 0]",
        ],
    );
    // 2-by-2, rank 2, modulo 2
    random_natural_matrices_with_rank_mod_helper(
        2,
        2,
        2,
        "2",
        &[
            "[1, 0; 0, 1]",
            "[0, 1; 1, 0]",
            "[1, 0; 0, 1]",
            "[1, 0; 0, 1]",
            "[1, 0; 0, 1]",
            "[0, 1; 1, 0]",
            "[1, 0; 0, 1]",
            "[1, 0; 0, 1]",
            "[1, 0; 0, 1]",
            "[1, 0; 0, 1]",
            "[1, 0; 0, 1]",
            "[0, 1; 1, 0]",
            "[1, 0; 0, 1]",
            "[0, 1; 1, 0]",
            "[1, 0; 0, 1]",
            "[1, 0; 0, 1]",
            "[1, 0; 0, 1]",
            "[1, 0; 0, 1]",
            "[1, 0; 0, 1]",
            "[0, 1; 1, 0]",
        ],
    );
    // 4-by-4, rank 0, modulo 1
    random_natural_matrices_with_rank_mod_helper(
        4,
        4,
        0,
        "1",
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
    // 3-by-3, rank 3, modulo 1000
    random_natural_matrices_with_rank_mod_helper(
        3,
        3,
        3,
        "1000",
        &[
            "[0, 0, 691; 0, 153, 0; 183, 0, 0]",
            "[0, 0, 957; 0, 163, 0; 479, 0, 0]",
            "[80, 0, 0; 0, 697, 0; 0, 0, 164]",
            "[0, 708, 0; 402, 0, 0; 0, 0, 163]",
            "[0, 595, 0; 697, 0, 0; 0, 0, 825]",
            "[0, 495, 0; 0, 0, 398; 663, 0, 0]",
            "[0, 0, 752; 734, 0, 0; 0, 207, 0]",
            "[0, 0, 443; 469, 0, 0; 0, 776, 0]",
            "[0, 992, 0; 0, 0, 922; 408, 0, 0]",
            "[500, 0, 0; 0, 0, 629; 0, 781, 0]",
            "[383, 0, 0; 0, 353, 0; 0, 0, 551]",
            "[6, 0, 0; 0, 0, 926; 0, 177, 0]",
            "[0, 0, 127; 0, 613, 0; 415, 0, 0]",
            "[0, 0, 576; 0, 582, 0; 807, 0, 0]",
            "[0, 0, 988; 0, 667, 0; 925, 0, 0]",
            "[112, 0, 0; 0, 0, 777; 0, 504, 0]",
            "[0, 195, 0; 808, 0, 0; 0, 0, 196]",
            "[0, 0, 56; 0, 374, 0; 531, 0, 0]",
            "[0, 139, 0; 0, 0, 460; 754, 0, 0]",
            "[205, 0, 0; 0, 0, 854; 0, 265, 0]",
        ],
    );
}

#[test]
#[should_panic]
fn random_natural_matrices_with_rank_mod_fail_1() {
    let _ = random_natural_matrices_with_rank_mod(
        EXAMPLE_SEED,
        2,
        3,
        3,
        Natural::from_str("7").unwrap(),
    );
}

#[test]
#[should_panic]
fn random_natural_matrices_with_rank_mod_fail_2() {
    let _ = random_natural_matrices_with_rank_mod(
        EXAMPLE_SEED,
        2,
        3,
        0,
        Natural::from_str("0").unwrap(),
    );
}

#[test]
#[should_panic]
fn random_natural_matrices_with_rank_mod_fail_3() {
    let _ = random_natural_matrices_with_rank_mod(
        EXAMPLE_SEED,
        2,
        3,
        1,
        Natural::from_str("1").unwrap(),
    );
}
