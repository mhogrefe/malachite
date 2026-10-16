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
use malachite_q::rational_matrix::random::random_rational_square_matrices_from_iterator;

fn random_rational_square_matrices_from_iterator_helper(
    bound: i32,
    mean_size_numerator: u64,
    mean_size_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        (random_rational_square_matrices_from_iterator(
            EXAMPLE_SEED,
            &|seed| random_rational_range(
                seed,
                Rational::from(-bound),
                Rational::from(bound),
                3,
                1,
                3,
                1
            ),
            mean_size_numerator,
            mean_size_denominator
        ))
        .take(20)
        .map(|m| m.to_string())
        .collect_vec(),
        expected_values
    );
}

#[test]
fn test_random_rational_square_matrices_from_iterator() {
    // elements bounded by 1, mean size = 1/1
    random_rational_square_matrices_from_iterator_helper(
        1,
        1,
        1,
        &[
            "[]_{0,0}",
            "[]_{0,0}",
            "[]_{0,0}",
            "[1/6]",
            "[]_{0,0}",
            "[1/11, 1/2; 1/2, 3/5]",
            "[]_{0,0}",
            "[]_{0,0}",
            "[1/2]",
            "[-2/5, -1, 0, -1/5, 2/3, 4/5; -1/6, 1/2, 1/6, -1/3, -1/9, -3/4; -1/4, -5/6, -19/26, \
            2/3, 0, 1/3; -1/4, 1/15, -1/4, -2/3, 1/2, -1/2; 0, -1/4, -1/9, -1/4, -1/3, 2/5; 0, \
            -1/16, -1/8, 1/2, -1, -1/10]",
            "[-1/8]",
            "[-1/3, -1/2, -1/3, 0, 1/18; -1/10, 1/6, 1/3, -1/5, -2/3; 2/5, -1/3, -1/2, -5/8, -1/2; \
            0, -1, -1, -1, -3/4; -2/19, 1/6, 2/3, 1/16, 0]",
            "[1/2]",
            "[-1/4]",
            "[1/11, 0; 1/2, 2/3]",
            "[-1/7, -1/7; 2/7, -7/9]",
            "[1/12, -1, 1/3, -1/2; -3/4, -1, -2/5, -1; -1/3, 2/3, -1/2, -1/4; 1/14, 1/7, 1/3, \
            1/2]",
            "[0, 1/4; 0, -1/2]",
            "[-1/2, 1/3, 1/2, 0; -1/4, 1/2, -1/5, 1/2; -1/2, -1, 6/7, 0; 0, 0, -1/3, -1/2]",
            "[]_{0,0}",
        ],
    );
    // elements bounded by 8, mean size = 1/1
    random_rational_square_matrices_from_iterator_helper(
        8,
        1,
        1,
        &[
            "[]_{0,0}",
            "[]_{0,0}",
            "[]_{0,0}",
            "[1/6]",
            "[]_{0,0}",
            "[1/11, -3/2; 1/2, 2/5]",
            "[]_{0,0}",
            "[]_{0,0}",
            "[-7/2]",
            "[-4/5, -1, -3, 2/5, 2/3, 22/5; 1/6, -3/2, -1/6, 1/3, 43/9, -7/4; 7/4, 7/6, -31/26, \
            5/3, -5, 20/3; -1/4, 32/15, -5/4, 13/3, -1/2, -3/2; 6, -3/4, -7/9, -1/4, -2/3, 2/5; \
            -8, -1/16, -1/8, 3/2, -3, -1/10]",
            "[9/8]",
            "[-1/3, -5/2, 1/3, 0, -5/18; -3/10, -1/6, 4/3, -1/5, 14/3; 3/5, -2/3, 1/2, -7/8, 11/2; \
            2, -2, 3, -7, 1/4; -2/19, -17/6, -1/3, -95/16, -1]",
            "[-3/2]",
            "[3/4]",
            "[-6/11, 3; 7/2, -19/3]",
            "[-1/7, -1/7; 1/7, -2/9]",
            "[1/12, 3, -1/3, 3/2; -3/4, 3, -1/5, 1; 14/3, -22/3, -3/2, -29/4; 13/14, 11/7, 1/3, \
            1/2]",
            "[4, -1/4; -1, -3/2]",
            "[-1/2, 20/3, -5/2, -2; -3/4, -1/2, -2/5, 9/2; -1/2, -2, -3/7, -1; 5, 1, -1/3, 7/2]",
            "[]_{0,0}",
        ],
    );
    // elements bounded by 100, mean size = 3/2
    random_rational_square_matrices_from_iterator_helper(
        100,
        3,
        2,
        &[
            "[]_{0,0}",
            "[]_{0,0}",
            "[1/6, 1/11, -19/2, 1/2, -3/5, 27/2; -26/5, -1, -3, 4/5, 1/3, 2/5; -1/6, -39/2, -1/6, \
            14/3, -50/9, -7/4; 7/4, 5/6, -31/26, -4/3, -5, 37/3; -1/4, 32/15, -5/4, 1/3, 3/2, \
            -3/2; 7, -29/4, -7/9, -291/4, -49/3, 26/5]",
            "[]_{0,0}",
            "[-12, -1/16; -1/8, 3/2]",
            "[12, -1/10; -1/8, 10/3]",
            "[5/2]",
            "[]_{0,0}",
            "[2/3, -3, -5/18; -3/10, -41/6, -80/3; -2/5, 1/3, 1/5]",
            "[]_{0,0}",
            "[-10/3, -11/2, 5/8, -1/2, 0; 0, -3, 3, -1/4, -2/19; -17/6, 26/3, -95/16, 3, -3/2; \
            -97/4, -1080/11, -7, -181/2, -1/3; -1/7, -1/7, 11/7, -2/9, 1/12]",
            "[1, -23/3, -7/2, 39/4, 0, 3/5; 3, -1/3, 1/3, 197/2, -29/4, 13/14; 3/7, 7/3, 3/2, 3, \
            -3/4, 2; -5/2, 1/2, -4/3, -3/2, 1, 3/4; 1/2, -242/5, 5/2, -1/2, 29, -1/7; -22, -2, -2, \
            -13/3, -9/2, 1/7]",
            "[]_{0,0}",
            "[-2/5]",
            "[-1]",
            "[49/2]",
            "[]_{0,0}",
            "[0, -2/5, 23/10; -1/12, 2/3, -1; 1/2, -1/4, 1/6]",
            "[-17/6, -67, 27/10, 87/2; -55/6, -305/4, 101/14, 2/5; -4/7, 11/7, 1/2, -1/4; 25/9, \
            -1/4, 1/7, -119/2]",
            "[-1/3, 6/7, -25/3; -1/12, 3/4, 1/2; -1/3, 3, -1/2]",
        ],
    );
}

#[test]
#[should_panic]
fn random_rational_square_matrices_from_iterator_fail_1() {
    // mean_size_denominator is zero
    let _ = random_rational_square_matrices_from_iterator(
        EXAMPLE_SEED,
        &|seed| random_rational_range(seed, Rational::from(-8), Rational::from(8), 3, 1, 3, 1),
        2,
        0,
    );
}

#[test]
#[should_panic]
fn random_rational_square_matrices_from_iterator_fail_2() {
    // mean_size_numerator is zero
    let _ = random_rational_square_matrices_from_iterator(
        EXAMPLE_SEED,
        &|seed| random_rational_range(seed, Rational::from(-8), Rational::from(8), 3, 1, 3, 1),
        0,
        1,
    );
}
