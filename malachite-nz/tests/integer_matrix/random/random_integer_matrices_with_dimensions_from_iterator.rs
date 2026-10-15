// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::integer::Integer;
use malachite_nz::integer::random::uniform_random_integer_inclusive_range;
use malachite_nz::integer_matrix::random::random_integer_matrices_with_dimensions_from_iterator;

fn random_integer_matrices_with_dimensions_from_iterator_helper(
    height: usize,
    width: usize,
    bound: i32,
    expected_values: &[&str],
) {
    assert_eq!(
        (random_integer_matrices_with_dimensions_from_iterator(
            height,
            width,
            uniform_random_integer_inclusive_range(
                EXAMPLE_SEED,
                Integer::from(-bound),
                Integer::from(bound)
            )
        ))
        .take(20)
        .map(|m| m.to_string())
        .collect_vec(),
        expected_values
    );
}

#[test]
fn test_random_integer_matrices_with_dimensions_from_iterator() {
    // 0-by-3, elements bounded by 8
    random_integer_matrices_with_dimensions_from_iterator_helper(
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
    random_integer_matrices_with_dimensions_from_iterator_helper(
        2,
        2,
        1,
        &[
            "[0, 0; 0, 0]",
            "[1, -1; 1, -1]",
            "[1, 0; 1, 1]",
            "[0, 0; 0, -1]",
            "[0, 1; 1, 1]",
            "[0, 0; -1, 0]",
            "[-1, -1; -1, 1]",
            "[0, 0; 0, 0]",
            "[1, -1; 0, -1]",
            "[0, -1; 0, 1]",
            "[1, 1; 0, -1]",
            "[-1, -1; 0, 1]",
            "[0, 1; -1, 1]",
            "[1, 0; -1, 1]",
            "[1, -1; 1, 1]",
            "[1, 0; 1, 1]",
            "[1, 1; 1, -1]",
            "[-1, 0; -1, 1]",
            "[-1, -1; -1, 0]",
            "[-1, -1; 0, -1]",
        ],
    );
    // 2-by-3, elements bounded by 8
    random_integer_matrices_with_dimensions_from_iterator_helper(
        2,
        3,
        8,
        &[
            "[-1, 1, 0; 3, 6, 3]",
            "[-7, -1, -7; -2, -2, -2]",
            "[-1, -3, 4; -1, 7, -1]",
            "[-1, -3, 3; 7, -7, -5]",
            "[8, -5, -4; -7, 8, 2]",
            "[6, -7, 8; 7, 8, -3]",
            "[6, -5, -1; -2, 4, -1]",
            "[1, -5, 8; -6, -6, 3]",
            "[-6, 3, -2; 0, 0, 2]",
            "[-4, 0, -7; -8, -1, 3]",
            "[5, -2, 8; -3, 4, -8]",
            "[-7, -6, 2; 8, 3, -1]",
            "[-2, -4, -4; -5, 1, -1]",
            "[-5, 3, -2; -1, -5, 1]",
            "[4, -6, 1; 1, 5, 0]",
            "[7, 6, 7; -5, 8, -1]",
            "[-4, -5, -4; -4, 6, 4]",
            "[6, -5, -4; -3, 6, 8]",
            "[-2, -7, 2; 7, -8, 0]",
            "[0, -1, -6; 4, -6, -4]",
        ],
    );
    // 3-by-1, elements bounded by 100
    random_integer_matrices_with_dimensions_from_iterator_helper(
        3,
        1,
        100,
        &[
            "[13; -13; -7]",
            "[17; -93; 14]",
            "[4; -16; 55]",
            "[74; -7; -78]",
            "[29; 15; 35]",
            "[-71; 53; 84]",
            "[-62; -30; 34]",
            "[-45; 3; 57]",
            "[40; -93; -13]",
            "[11; -19; -77]",
            "[80; -12; -93]",
            "[-40; -93; 78]",
            "[1; -79; 71]",
            "[-51; -85; 61]",
            "[-9; 95; 86]",
            "[-64; 55; 29]",
            "[-84; 70; -22]",
            "[93; -52; -8]",
            "[83; -20; -95]",
            "[42; 31; -29]",
        ],
    );
}

#[test]
#[should_panic]
fn random_integer_matrices_with_dimensions_from_iterator_fail_1() {
    // height * width overflows
    let _ = random_integer_matrices_with_dimensions_from_iterator(
        usize::MAX,
        2,
        uniform_random_integer_inclusive_range(EXAMPLE_SEED, Integer::from(-8), Integer::from(8)),
    );
}
