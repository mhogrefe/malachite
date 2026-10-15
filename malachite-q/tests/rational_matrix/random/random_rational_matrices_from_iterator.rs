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
use malachite_q::rational_matrix::random::random_rational_matrices_from_iterator;

fn random_rational_matrices_from_iterator_helper(
    bound: i32,
    mean_dimension_numerator: u64,
    mean_dimension_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        (random_rational_matrices_from_iterator(
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
            mean_dimension_numerator,
            mean_dimension_denominator
        ))
        .take(20)
        .map(|m| m.to_string())
        .collect_vec(),
        expected_values
    );
}

#[test]
fn test_random_rational_matrices_from_iterator() {
    // elements bounded by 1, mean dimension = 1/1
    random_rational_matrices_from_iterator_helper(
        1,
        1,
        1,
        &[
            "[]_{0,0}",
            "[]_{0,1}",
            "[]_{0,2}",
            "[]_{0,0}",
            "[1/6, 1/11, 1/2, 1/2, 3/5, 1/2]",
            "[-2/5, -1, 0, -1/5, 2/3]",
            "[4/5]",
            "[-1/6, 1/2; 1/6, -1/3]",
            "[-1/9, -3/4; -1/4, -5/6; -19/26, 2/3; 0, 1/3]",
            "[]_{4,0}",
            "[]_{1,0}",
            "[]_{1,0}",
            "[]_{0,0}",
            "[-1/4, 1/15]",
            "[]_{2,0}",
            "[]_{3,0}",
            "[-1/4; -2/3; 1/2]",
            "[]_{2,0}",
            "[]_{0,1}",
            "[-1/2; 0]",
        ],
    );
    // elements bounded by 8, mean dimension = 2/1
    random_rational_matrices_from_iterator_helper(
        8,
        2,
        1,
        &[
            "[]_{0,1}",
            "[]_{0,11}",
            "[]_{2,0}",
            "[1/6, 1/11; -3/2, 1/2; 2/5, -7/2; -4/5, -1]",
            "[]_{6,0}",
            "[]_{0,2}",
            "[]_{1,0}",
            "[]_{0,3}",
            "[]_{6,0}",
            "[-3]",
            "[2/5, 2/3, 22/5, 1/6; -3/2, -1/6, 1/3, 43/9]",
            "[]_{0,1}",
            "[]_{0,3}",
            "[]_{5,0}",
            "[]_{4,0}",
            "[]_{0,6}",
            "[-7/4, 7/4]",
            "[]_{0,1}",
            "[]_{0,0}",
            "[]_{0,0}",
        ],
    );
    // elements bounded by 100, mean dimension = 2/1
    random_rational_matrices_from_iterator_helper(
        100,
        2,
        1,
        &[
            "[]_{0,1}",
            "[]_{0,11}",
            "[]_{2,0}",
            "[1/6, 1/11; -19/2, 1/2; -3/5, 27/2; -26/5, -1]",
            "[]_{6,0}",
            "[]_{0,2}",
            "[]_{1,0}",
            "[]_{0,3}",
            "[]_{6,0}",
            "[-3]",
            "[4/5, 1/3, 2/5, -1/6; -39/2, -1/6, 14/3, -50/9]",
            "[]_{0,1}",
            "[]_{0,3}",
            "[]_{5,0}",
            "[]_{4,0}",
            "[]_{0,6}",
            "[-7/4, 7/4]",
            "[]_{0,1}",
            "[]_{0,0}",
            "[]_{0,0}",
        ],
    );
}

#[test]
#[should_panic]
fn random_rational_matrices_from_iterator_fail_1() {
    // mean_dimension_denominator is zero
    let _ = random_rational_matrices_from_iterator(
        EXAMPLE_SEED,
        &|seed| random_rational_range(seed, Rational::from(-8), Rational::from(8), 3, 1, 3, 1),
        2,
        0,
    );
}

#[test]
#[should_panic]
fn random_rational_matrices_from_iterator_fail_2() {
    // mean_dimension_numerator is zero
    let _ = random_rational_matrices_from_iterator(
        EXAMPLE_SEED,
        &|seed| random_rational_range(seed, Rational::from(-8), Rational::from(8), 3, 1, 3, 1),
        0,
        1,
    );
}
