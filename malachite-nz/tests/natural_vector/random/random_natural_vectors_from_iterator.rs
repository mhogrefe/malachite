// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::natural::Natural;
use malachite_nz::natural::random::uniform_random_natural_inclusive_range;
use malachite_nz::natural_vector::random::random_natural_vectors_from_iterator;

fn random_natural_vectors_from_iterator_helper(
    bound: u32,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        random_natural_vectors_from_iterator(
            EXAMPLE_SEED,
            &|seed| uniform_random_natural_inclusive_range(
                seed,
                Natural::ZERO,
                Natural::from(bound)
            ),
            mean_length_numerator,
            mean_length_denominator
        )
        .take(20)
        .map(|v| v.to_string())
        .collect_vec(),
        expected_values
    );
}

#[test]
fn test_random_natural_vectors_from_iterator() {
    // elements in [0, 1], mean dimension = 1
    random_natural_vectors_from_iterator_helper(
        1,
        1,
        1,
        &[
            "(1)",
            "(0, 0)",
            "(1, 0)",
            "(0, 1)",
            "(1)",
            "(0, 1)",
            "()",
            "()",
            "(1)",
            "()",
            "()",
            "()",
            "(0)",
            "(0, 1, 1, 0)",
            "()",
            "(0, 1)",
            "()",
            "(1, 1, 1, 1, 0)",
            "(1)",
            "(0)",
        ],
    );
    // elements in [0, 15], mean dimension = 2
    random_natural_vectors_from_iterator_helper(
        15,
        2,
        1,
        &[
            "(5, 6, 14, 5, 10, 2)",
            "(13)",
            "(5, 10, 1, 9, 0, 14, 9, 13)",
            "(4)",
            "(4, 7, 1, 11, 9, 3, 12, 9, 14, 10, 15, 14, 0, 3)",
            "()",
            "(15, 4, 7, 9, 5)",
            "(6, 14, 15, 14)",
            "(11)",
            "()",
            "(3, 3, 0, 15, 11, 12)",
            "(5, 7)",
            "()",
            "()",
            "(13)",
            "(9, 7, 8)",
            "()",
            "(12)",
            "()",
            "(1)",
        ],
    );
    // elements in [0, 1000], mean dimension = 4
    random_natural_vectors_from_iterator_helper(
        1000,
        4,
        1,
        &[
            "()",
            "(853, 806, 542, 469, 170, 370, 429, 325, 874, 689, 777, 304, 190, 217)",
            "(541, 660, 788, 23)",
            "(369, 251, 361, 419)",
            "(28)",
            "()",
            "(953, 302, 890, 478, 64)",
            "(227, 239)",
            "(228, 855, 473, 85)",
            "()",
            "(950, 94, 175, 814, 539, 995)",
            "()",
            "()",
            "(627, 272, 543, 715, 588, 645, 679, 829, 329, 695, 952)",
            "(732, 993, 945, 913, 570, 63, 42, 66)",
            "()",
            "(718, 17, 311)",
            "()",
            "(374, 627, 860, 87, 309)",
            "(824, 230, 720, 848, 520, 935, 175, 181, 928)",
        ],
    );
}

#[test]
#[should_panic]
fn random_natural_vectors_from_iterator_fail_1() {
    // mean_length_denominator is zero
    let _ = random_natural_vectors_from_iterator(
        EXAMPLE_SEED,
        &|seed| uniform_random_natural_inclusive_range(seed, Natural::ZERO, Natural::ONE),
        1,
        0,
    );
}

#[test]
#[should_panic]
fn random_natural_vectors_from_iterator_fail_2() {
    // mean_length_numerator is zero
    let _ = random_natural_vectors_from_iterator(
        EXAMPLE_SEED,
        &|seed| uniform_random_natural_inclusive_range(seed, Natural::ZERO, Natural::ONE),
        0,
        1,
    );
}
