// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_q::rational_matrix::random::striped_random_rational_square_matrices;

fn striped_random_rational_square_matrices_helper(
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_size_numerator: u64,
    mean_size_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        (striped_random_rational_square_matrices(
            EXAMPLE_SEED,
            mean_stripe_numerator,
            mean_stripe_denominator,
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
fn test_striped_random_rational_square_matrices() {
    // mean stripe = 2/1, mean bits = 2/1, mean size = 1/1
    striped_random_rational_square_matrices_helper(
        2,
        1,
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
            "[-62/5, 1; 0, 0]",
            "[]_{0,0}",
            "[]_{0,0}",
            "[-7/3]",
            "[4, 1, 1, 0, -1, -9/14; -1, 2, 0, -1, -6, 8; -13/3, -1/6, 1, -1, 0, 0; 0, -3, 5/6, \
            -13, -2, -26; 0, 0, 91/6, 0, -2, 3; -1, -1/3, -1/3, 0, 7, 0]",
            "[1]",
            "[-4, 1/2, 6, 8, -1; 0, -6, -2/17, 0, 1; -1, 0, -3, -5, 1; 0, 17, 7, 0, -2; -89/2, 19, \
            1, 0, -29]",
            "[0]",
            "[0]",
            "[-5/2, 0; -2, -8/7]",
            "[8, -3; 0, 20]",
            "[-7/226, 1/2, 1, 1; -2, 0, -9, 146/11; -82, 0, 0, 1; 0, -1/11, 0, -34]",
            "[-7/3, 4; -6, 1]",
            "[0, 1/2, 0, -10; 0, -1, 0, 2; 1/3, 0, -6, 84; 24, 0, -4, 1/2]",
            "[]_{0,0}",
        ],
    );
    // mean stripe = 16/1, mean bits = 4/1, mean size = 1/1
    striped_random_rational_square_matrices_helper(
        16,
        1,
        4,
        1,
        1,
        1,
        &[
            "[]_{0,0}",
            "[]_{0,0}",
            "[]_{0,0}",
            "[-1/8]",
            "[]_{0,0}",
            "[0, 575/14; -15, 1/4]",
            "[]_{0,0}",
            "[]_{0,0}",
            "[-4]",
            "[63/31, 32, 1677671/3, 31/7, -1/2, -4; -2, 1/8, 1/3, -1, -31/7, 64; -3/2, 0, 1/128, \
            -15, 3/4, -1/2; -7/16, 0, 32, -7, 0, -7; 3/4, -3/2, 3/2, -511/3, -20, 6; -2, 0, \
            -262144/129, 7, 1/2, 0]",
            "[4/3]",
            "[-16, 1/7, 0, 1/2, -1048575/8; -39, -7, -34636833, -32, 63/16; 0, 3/16, 0, -1, 0; \
            31/2, 0, 85, 0, -4; -1/256, 1, 193/32, 4, 0]",
            "[1055/14]",
            "[21]",
            "[-1/4, 0; -4096, -4/15]",
            "[1/15, -1/64; 16776184, 1/2552]",
            "[-1/4, 0, 0, 1/127; -511/3, -2/511, 0, 16384/127; -1, 1, -24, 31/7; 0, -1/3, 129/2, \
            -2/3]",
            "[0, 1; -1, 2]",
            "[-3, 15/4, 1/1039, -1; -8/63, -22, 1536/127, 31/7; 1/15, 0, -31/15, 0; 1/128, -60, 0, \
            15/2]",
            "[]_{0,0}",
        ],
    );
    // mean stripe = 4/1, mean bits = 8/1, mean size = 3/2
    striped_random_rational_square_matrices_helper(
        4,
        1,
        8,
        1,
        3,
        2,
        &[
            "[]_{0,0}",
            "[]_{0,0}",
            "[-1/7, -1272959/39, 65343/8, -15531526, 76, -39881/135; 88/9, 0, 71/11, 15/31, \
            -67630979/8643, 0; -1920, 2132737536/7, 1/15619, -1/256, -63/31, 13/3; -236, -2273/2, \
            17537397896193/4, 0, 3/4, -134; -130174/3, -31/125, 1/5, -7/2046, -63/3811, \
            -1/4337881; 1/90, -3/496, 15/4, -8/7, -4/11, 2]",
            "[]_{0,0}",
            "[0, 0; -1342/127, 1/17]",
            "[8/31, -128/2559; 4/7, -147456]",
            "[59/4096]",
            "[]_{0,0}",
            "[127/2019, 499/30, -515/12; -2559, -182390848/17, -9735/231443; -11807, 1389/4, \
            -16475]",
            "[]_{0,0}",
            "[493/2, -4/7, -68/79, 1569017/6, 1/198367003648; 8/3, 0, 33553615/3, -13123, \
            -98729989/16; 319/511, 0, 48, -32/5, 3/2; 0, -2/7, 1/258, -39/28430, -56/1535; 961, \
            -7936, 192735/46, 1/912, -7/3456]",
            "[1/4, 79999/63, 54484974/8183, -1, -4/135, -239/47; 712/8727217, -28, 57, -1, \
            1/57375, -1/30; -2/16331, 95/1398123, 0, 0, 11, -2121731/2; 7/3, -15487/56, 575, \
            3807196/3, -5, -28/95; -59/2986337247, 33/7, 1151/49, 254/15, -2/19, -7/3; 24, 1/30, \
            -548836, -3/2056, 3945465/2096, 0]",
            "[]_{0,0}",
            "[0]",
            "[-1/4]",
            "[-3/152]",
            "[]_{0,0}",
            "[146162986993/3, -15/8, -95/2; -30765/8, 1/32, -7/73715; -279/19, -31/925795, \
            -1/569]",
            "[955368192/31, -503/256, 6540/31, -1/21; 0, -1088/377, -21/844, 30/4121; \
            -63641/12543, 0, -10, 2774433/13; 248, -7/61895, 192/8457733085, 4/3]",
            "[-3463/492, 2/7, -1/49526214652; -123874, 168/5, -8273886/2095; -2/7, -127, -1]",
        ],
    );
}

#[test]
#[should_panic]
fn striped_random_rational_square_matrices_fail_1() {
    // mean_stripe_denominator is zero
    let _ = striped_random_rational_square_matrices(EXAMPLE_SEED, 1, 0, 4, 1, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_rational_square_matrices_fail_2() {
    // mean_stripe_numerator < mean_stripe_denominator
    let _ = striped_random_rational_square_matrices(EXAMPLE_SEED, 1, 2, 4, 1, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_rational_square_matrices_fail_3() {
    // mean_bits_numerator is zero
    let _ = striped_random_rational_square_matrices(EXAMPLE_SEED, 4, 1, 0, 1, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_rational_square_matrices_fail_4() {
    // mean_size_denominator is zero
    let _ = striped_random_rational_square_matrices(EXAMPLE_SEED, 4, 1, 4, 1, 2, 0);
}
