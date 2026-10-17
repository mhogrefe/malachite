// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_q::rational_matrix::random::random_rational_matrices_with_dimensions;

fn random_rational_matrices_with_dimensions_helper(
    height: u64,
    width: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        (random_rational_matrices_with_dimensions(
            EXAMPLE_SEED,
            height,
            width,
            mean_bits_numerator,
            mean_bits_denominator
        ))
        .take(20)
        .map(|m| m.to_string())
        .collect_vec(),
        expected_values
    );
}

#[test]
fn test_random_rational_matrices_with_dimensions() {
    // 0-by-3, mean bits = 4/1
    random_rational_matrices_with_dimensions_helper(
        0,
        3,
        4,
        1,
        &[
            "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}",
            "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}",
            "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}",
        ],
    );
    // 2-by-2, mean bits = 2/1
    random_rational_matrices_with_dimensions_helper(
        2,
        2,
        2,
        1,
        &[
            "[-1, -5/3; 0, 1]",
            "[0, 1/2; 356, 0]",
            "[0, 3/2; 3/5, -14/3]",
            "[0, -1/3; -19/3, -1/2]",
            "[0, -1; 0, -10]",
            "[-4, 0; 0, 0]",
            "[0, -2; -3/20, 1/5]",
            "[-1, 6; 0, 0]",
            "[2, 2/5; 0, -1613/5]",
            "[0, 8; 3, -1]",
            "[0, 1; -4/13, 0]",
            "[-14, 13; 0, 1]",
            "[0, -125; -1, 3/2]",
            "[-17/29, -1; 39, 15/2]",
            "[0, 0; 0, 1]",
            "[0, -14; -3/13, -1]",
            "[7/2, 3/17; 1, 0]",
            "[-1, 27/2; -3/2, 4/5]",
            "[1, 37/7; 3/2, -3]",
            "[-3/2, 0; 2/27, -1/178]",
        ],
    );
    // 2-by-3, mean bits = 3/1
    random_rational_matrices_with_dimensions_helper(
        2,
        3,
        3,
        1,
        &[
            "[-1/2, -13/3, 0; 1/457, 0, 7/4]",
            "[612/5, 0, 0; 3/22, 1, -318/5]",
            "[0, -3/5, -1043/11; -1/786, 0, -1/5]",
            "[0, -26/25, -12/31; 0, 0, 0]",
            "[-1/3, -7, -5; 14/5, -2/31, 54]",
            "[0, 0, 4/43; 1, -1/24, -6861]",
            "[-1, 94/3, 1; -1/3, 0, 13]",
            "[-10/3, 0, -29/115; 30, 0, 1/125]",
            "[-1, -160, -7; 55/54, -51, -1/46]",
            "[17, 13, 0; 0, 0, 7]",
            "[1/5, -23/7, -6; -7, 7/2, 36/65]",
            "[1, 0, -1/12; 369/68, -1/4, 5/3]",
            "[2/289, 581/11, 1; -3, -5, 1/2]",
            "[7/859, -1, 0; -3, -11/10, 1]",
            "[1/2, -1, -5/3; -8, 2, 0]",
            "[-6, 1, 1; 2/29, 0, -15/4]",
            "[-6, -191/11, -3/4; -1, 7, 44756]",
            "[9, -5088, 4/3; 0, -51415, -1]",
            "[-2, 59, -3; -3316/111, 1/7, 2]",
            "[0, -26/7, -1/4; -13/3, 2/9, -1]",
        ],
    );
    // 3-by-1, mean bits = 4/1
    random_rational_matrices_with_dimensions_helper(
        3,
        1,
        4,
        1,
        &[
            "[0; -1/3; 1]",
            "[4/323; 0; 7/12]",
            "[1/17; 7; 1/97]",
            "[15/19; 0; -79/21]",
            "[7/6; -9912911/5; 0]",
            "[-5/786; -986/97; -12/5]",
            "[0; 0; 0]",
            "[256/15; 0; -1]",
            "[-9/7; -1062790142; -2]",
            "[6/113; -184/31; 29]",
            "[56/20583; -717; 1625/619]",
            "[6; -5/66; 0]",
            "[0; 22/7; 1/4]",
            "[-1/3; 5/3; 7]",
            "[-3/5; -1/5; -16/115]",
            "[3; 3/46; 1/63]",
            "[-126; -1/61; 0]",
            "[0; -13; -15/22]",
            "[0; 37; -39/1328]",
            "[-1/2; 0; 11]",
        ],
    );
}

#[test]
#[should_panic]
fn random_rational_matrices_with_dimensions_fail_1() {
    // mean_bits_denominator is zero
    let _ = random_rational_matrices_with_dimensions(EXAMPLE_SEED, 2, 2, 1, 0);
}

#[test]
#[should_panic]
fn random_rational_matrices_with_dimensions_fail_2() {
    // mean_bits_numerator is zero
    let _ = random_rational_matrices_with_dimensions(EXAMPLE_SEED, 2, 2, 0, 1);
}

#[test]
#[should_panic]
fn random_rational_matrices_with_dimensions_fail_3() {
    // height * width overflows
    let _ = random_rational_matrices_with_dimensions(EXAMPLE_SEED, u64::MAX, 2, 4, 1);
}
