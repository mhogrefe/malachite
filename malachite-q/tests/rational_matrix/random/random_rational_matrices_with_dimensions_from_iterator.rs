// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_q::Rational;
use malachite_q::rational::random::random_rational_range;
use malachite_q::rational_matrix::random::random_rational_matrices_with_dimensions_from_iterator;

fn random_rational_matrices_with_dimensions_from_iterator_helper(
    height: u64,
    width: u64,
    bound: i32,
    expected_values: &[&str],
) {
    assert_eq!(
        (random_rational_matrices_with_dimensions_from_iterator(
            height,
            width,
            random_rational_range(
                EXAMPLE_SEED,
                Rational::from(-bound),
                Rational::from(bound),
                3,
                1,
                3,
                1
            )
        ))
        .take(20)
        .map(|m| m.to_string())
        .collect_vec(),
        expected_values
    );
}

#[test]
fn test_random_rational_matrices_with_dimensions_from_iterator() {
    // 0-by-3, elements bounded by 8
    random_rational_matrices_with_dimensions_from_iterator_helper(
        0,
        3,
        8,
        &[
            "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}",
            "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}",
            "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}",
        ],
    );
    // 2-by-2, elements bounded by 1
    random_rational_matrices_with_dimensions_from_iterator_helper(
        2,
        2,
        1,
        &[
            "[1/2, 0; 0, -1/2]",
            "[4/11, -1; -1, -1]",
            "[-1/2, 1/8; -9/11, -5/9]",
            "[1/2, 1/8; -2/5, -1]",
            "[-1/5, 11/15; -1/3, -1]",
            "[1/3, 1/2; 3/5, -9/10]",
            "[-1/2, -2/7; 1/5, 1/2]",
            "[-1, 1/2; -1/10, -1/4]",
            "[-1/2, 3/8; -4/25, -1/8]",
            "[-1/3, -1; -1, 0]",
            "[0, 1/3; 1/6, -1]",
            "[0, -1; 0, -1]",
            "[1/2, 3/8; -5/7, -1/6]",
            "[0, 0; -2/3, 4/5]",
            "[-4/5, 1/11; 1/2, 1/3]",
            "[-1/10, -1; -1/2, -1/9]",
            "[-1/3, -1/3; -2/3, -1]",
            "[-1/3, 1/8; 1/4, 0]",
            "[-1/4, -1/6; 3/4, 1/4]",
            "[1/2, 7/12; 2/3, 1/2]",
        ],
    );
    // 2-by-3, elements bounded by 8
    random_rational_matrices_with_dimensions_from_iterator_helper(
        2,
        3,
        8,
        &[
            "[-1/2, 0, -8; 1/2, 4/11, -6]",
            "[0, 5, -7/2; 1/8, -1/11, -5/9]",
            "[1/2, -13/8, -2/5; -7, -7/5, -2/15]",
            "[23/3, -5, -7/3; 15/2, -1/5, -11/10]",
            "[-1/2, 40/7, 1/5; -1/2, -3, 1/2]",
            "[-1/10, -5/4, -1/2; 1/8, 44/25, 3/8]",
            "[11/3, -8, 1; 0, 0, -1/3]",
            "[1/6, 1, -8; -1, -1, 1]",
            "[3/2, -1/8, -6/7; -1/6, 1, 1]",
            "[2/3, -2/5, -1/5; -45/11, 3/2, -4/3]",
            "[-1/10, 7, 5/2; -1/9, 1/3, 5/3]",
            "[8/3, -3, -11/3; 1/8, 1/4, 1]",
            "[1/4, 1/6, 3/4; 1/4, 1/2, 7/12]",
            "[2/3, 1/2, 1/3; 17/3, -3, 1]",
            "[-1, 1, 1/3; 7/5, 7/3, -5/14]",
            "[-1/2, -1/4, 1/2; -1/3, -1/3, -2/3]",
            "[0, 1/2, -8; 1/2, -1/2, 0]",
            "[0, 3/10, -1/8; 5/3, 5/3, -50/9]",
            "[3/2, 1/4, 3/2; -31/6, 1/3, -6/17]",
            "[1/3, -1/3, -2; 5, 1/3, -1/2]",
        ],
    );
    // 3-by-1, elements bounded by 100
    random_rational_matrices_with_dimensions_from_iterator_helper(
        3,
        1,
        100,
        &[
            "[99/2; 0; -1]",
            "[-1/2; 4/11; -7]",
            "[0; 5; 7/2]",
            "[753/8; 256/11; -5/9]",
            "[-1/2; -1/8; -2/5]",
            "[-7; -7/5; 19/15]",
            "[31/3; -6; -7/3]",
            "[15/2; -1/5; -11/10]",
            "[1/2; 54/7; -229/5]",
            "[-1/2; 58; -3/2]",
            "[-1/10; 259/4; 1/2]",
            "[-1/8; 44/25; 3/8]",
            "[218/3; 0; 1]",
            "[0; 0; 1/3]",
            "[1/6; 1; -10]",
            "[-1; -1; -15]",
            "[1/2; 49/8; -5/7]",
            "[-1/6; 1; 1]",
            "[1/3; 2/5; -67/5]",
            "[-45/11; -27/2; 223/3]",
        ],
    );
}

#[test]
#[should_panic]
fn random_rational_matrices_with_dimensions_from_iterator_fail_1() {
    // height * width overflows
    let _ = random_rational_matrices_with_dimensions_from_iterator(
        u64::MAX,
        2,
        random_rational_range(
            EXAMPLE_SEED,
            Rational::from(-8),
            Rational::from(8),
            3,
            1,
            3,
            1,
        ),
    );
}
