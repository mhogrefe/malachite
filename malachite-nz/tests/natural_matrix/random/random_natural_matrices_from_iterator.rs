// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::basic::traits::Zero;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::natural::Natural;
use malachite_nz::natural::random::uniform_random_natural_inclusive_range;
use malachite_nz::natural_matrix::random::random_natural_matrices_from_iterator;

fn random_natural_matrices_from_iterator_helper(
    bound: u32,
    mean_dimension_numerator: u64,
    mean_dimension_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        (random_natural_matrices_from_iterator(
            EXAMPLE_SEED,
            &|seed| uniform_random_natural_inclusive_range(
                seed,
                Natural::ZERO,
                Natural::from(bound)
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
fn test_random_natural_matrices_from_iterator() {
    // elements bounded by 1, mean dimension = 1/1
    random_natural_matrices_from_iterator_helper(
        1,
        1,
        1,
        &[
            "[]_{0,0}",
            "[]_{0,1}",
            "[]_{0,2}",
            "[]_{0,0}",
            "[0, 1, 0, 1, 1, 1]",
            "[1, 1, 1, 1, 1]",
            "[1]",
            "[1, 1; 0, 0]",
            "[0, 0; 0, 0; 1, 0; 1, 1]",
            "[]_{4,0}",
            "[]_{1,0}",
            "[]_{1,0}",
            "[]_{0,0}",
            "[0, 1]",
            "[]_{2,0}",
            "[]_{3,0}",
            "[1; 0; 1]",
            "[]_{2,0}",
            "[]_{0,1}",
            "[1; 1]",
        ],
    );
    // elements bounded by 8, mean dimension = 2/1
    random_natural_matrices_from_iterator_helper(
        8,
        2,
        1,
        &[
            "[]_{0,1}",
            "[]_{0,11}",
            "[]_{2,0}",
            "[4, 0; 5, 5; 1, 8; 4, 4]",
            "[]_{6,0}",
            "[]_{0,2}",
            "[]_{1,0}",
            "[]_{0,3}",
            "[]_{6,0}",
            "[7]",
            "[2, 7, 4, 7; 1, 7, 0, 8]",
            "[]_{0,1}",
            "[]_{0,3}",
            "[]_{5,0}",
            "[]_{4,0}",
            "[]_{0,6}",
            "[7, 0]",
            "[]_{0,1}",
            "[]_{0,0}",
            "[]_{0,0}",
        ],
    );
    // elements bounded by 100, mean dimension = 2/1
    random_natural_matrices_from_iterator_helper(
        100,
        2,
        1,
        &[
            "[]_{0,1}",
            "[]_{0,11}",
            "[]_{2,0}",
            "[36, 11; 96, 63; 91, 53; 29, 73]",
            "[]_{6,0}",
            "[]_{0,2}",
            "[]_{1,0}",
            "[]_{0,3}",
            "[]_{6,0}",
            "[21]",
            "[15, 65, 88, 60; 76, 20, 57, 55]",
            "[]_{0,1}",
            "[]_{0,3}",
            "[]_{5,0}",
            "[]_{4,0}",
            "[]_{0,6}",
            "[2, 55]",
            "[]_{0,1}",
            "[]_{0,0}",
            "[]_{0,0}",
        ],
    );
}

#[test]
#[should_panic]
fn random_natural_matrices_from_iterator_fail_1() {
    // mean_dimension_denominator is zero
    let _ = random_natural_matrices_from_iterator(
        EXAMPLE_SEED,
        &|seed| uniform_random_natural_inclusive_range(seed, Natural::ZERO, Natural::from(8u32)),
        2,
        0,
    );
}

#[test]
#[should_panic]
fn random_natural_matrices_from_iterator_fail_2() {
    // mean_dimension_numerator is zero
    let _ = random_natural_matrices_from_iterator(
        EXAMPLE_SEED,
        &|seed| uniform_random_natural_inclusive_range(seed, Natural::ZERO, Natural::from(8u32)),
        0,
        1,
    );
}
