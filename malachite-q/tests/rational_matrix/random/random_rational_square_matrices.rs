// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_q::rational_matrix::random::random_rational_square_matrices;

fn random_rational_square_matrices_helper(
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_size_numerator: u64,
    mean_size_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        (random_rational_square_matrices(
            EXAMPLE_SEED,
            mean_bits_numerator,
            mean_bits_denominator,
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
fn test_random_rational_square_matrices() {
    // mean bits = 2/1, mean size = 1/1
    random_rational_square_matrices_helper(
        2,
        1,
        1,
        1,
        &[
            "[]_{0,0}",
            "[]_{0,0}",
            "[]_{0,0}",
            "[0]",
            "[]_{0,0}",
            "[-49/4, 1; 0, 0]",
            "[]_{0,0}",
            "[]_{0,0}",
            "[-4/3]",
            "[5, 1, 5/4, 0, -1, -1; -1, 7/3, 0, -1, -4, 15; -19/7, -1/4, 1, -1, 0, 0; 0, -3, 5/7, \
            -8, -3, -31; 0, 0, 25, 0, -3, 2; -1, -1/3, -1/3, 0, 5, 0]",
            "[2/3]",
            "[-7, 1/2, 7, 8, -1; 0, -4, -3/23, 0, 1; -1, 0, -2, -7, 1; 0, 45/2, 6, 0, -3; -56, 31, \
            1, 0, -16]",
            "[0]",
            "[0]",
            "[-7/2, 0; -2, -14/5]",
            "[12, -3; 0, 29]",
            "[-1/37, 1/2, 1, 1; -3, 0, -26/3, 66/5; -95, 0, 0, 1; 0, -1/15, 0, -55]",
            "[-5/2, 4; -6, 1]",
            "[0, 1/3, 0, -15; 0, -1, 0, 3; 1/2, 0, -13, 96; 17, 0, -7, 1/3]",
            "[]_{0,0}",
        ],
    );
    // mean bits = 3/1, mean size = 1/1
    random_rational_square_matrices_helper(
        3,
        1,
        1,
        1,
        &[
            "[]_{0,0}",
            "[]_{0,0}",
            "[]_{0,0}",
            "[0]",
            "[]_{0,0}",
            "[-113/4, 1/2; 0, 0]",
            "[]_{0,0}",
            "[]_{0,0}",
            "[-4]",
            "[9/28, 1, 2863/12, 1/3, -1/5, -40/3; -1, 5/7, 1/3, -1, -11/2, 38; -46/3, -3, 3/115, \
            -1, 0, -1/6; 0, -5/2, 5, -12, -13/2, -248; 1/5, -1, 225/2, 0, -13/3, 1/2; -3, -1/3, \
            -1/53, 1, 2, 0]",
            "[8]",
            "[-29, 1/2, 7/4, 26, -3/2; -1, -5, -2/3, 0, 1/12; -1/3, 0, -1, -20, 1/3; 0, 427, 6, 0, \
            -6; -167/10, 77, 1/23, 0, -28]",
            "[0]",
            "[0]",
            "[-8, 0; -27/2, -10]",
            "[113/6, -1/19; 0, 68/303]",
            "[-15, 1, 14/13, 15/4; -3/2, 0, -152, 71; -481/3, 0, -1, 1; -1, -1/3, 0, -440/3]",
            "[-52, 16/5; -5, 67]",
            "[-2, 3/5, 1/118, -23; 0, -2, 0, 7/5; 1/2, -1/14, -271/4, 330; 100/11, 0, -13, 1]",
            "[]_{0,0}",
        ],
    );
    // mean bits = 4/1, mean size = 3/2
    random_rational_square_matrices_helper(
        4,
        1,
        3,
        2,
        &[
            "[]_{0,0}",
            "[]_{0,0}",
            "[-9/91, 0, 82, -8, 1/3, -5; 47/28, 44, 1979435/4, 4, -3/5, -3; -2, 3/31, 1/3, -1, -5, \
            83; -1, 0, 1/243, -8, 19/33, -1/2; -1/7, 0, 23, -4, 0, -7; 2/5, -1, 3/2, -207, -43/3, \
            5]",
            "[]_{0,0}",
            "[-11/5, 0; -61504/49, 4]",
            "[1/2, 0; 5/3, -40/3]",
            "[1/4]",
            "[]_{0,0}",
            "[0, 1/2, -270253/4; -50, -5, -899157898/23; -41, 12/7, 0]",
            "[]_{0,0}",
            "[1/9, 0, -1, 0, 16/3; 0, 171/2, 0, -4, -1/488; 1, 41/11, 5, 0, 1940/9; 40/3, -1/4, 0, \
            -4537, -5/9; 1/14, -1/67, 47544287/6, 1/2863, -1/4]",
            "[0, 0, 1/124, -257/2, -2/267, 0; 11861/45, -1, 1, -31, 18/7, 0; -1/3, 92, -2/3, 0, \
            4/5, -1; 3, -2, 11/5, 1/2038, -1, -1/4; -29, 1411/67, 19/5, 1/14, 0, -13/6; 0, 1/246, \
            -37, 0, 13/3, -17/28]",
            "[]_{0,0}",
            "[-303/860]",
            "[-1/15]",
            "[-6/5]",
            "[]_{0,0}",
            "[1/33, -1/2, 0; -7/2, 157/3, -1/7; -1/2, -1/17, -1/15]",
            "[0, -2359/19, 1/3, -1/6; -1, 0, 0, 203; 0, 0, -1/2, 1/47146; 1/33, -53/6, 0, 1]",
            "[-5/107, 1/20, -9; -3, 15/2, -6; -1/3, -2831, -6]",
        ],
    );
}

#[test]
#[should_panic]
fn random_rational_square_matrices_fail_1() {
    // mean_bits_denominator is zero
    let _ = random_rational_square_matrices(EXAMPLE_SEED, 1, 0, 2, 1);
}

#[test]
#[should_panic]
fn random_rational_square_matrices_fail_2() {
    // mean_bits_numerator is zero
    let _ = random_rational_square_matrices(EXAMPLE_SEED, 0, 1, 2, 1);
}

#[test]
#[should_panic]
fn random_rational_square_matrices_fail_3() {
    // mean_size_denominator is zero
    let _ = random_rational_square_matrices(EXAMPLE_SEED, 4, 1, 2, 0);
}

#[test]
#[should_panic]
fn random_rational_square_matrices_fail_4() {
    // mean_size_numerator is zero
    let _ = random_rational_square_matrices(EXAMPLE_SEED, 4, 1, 0, 1);
}
