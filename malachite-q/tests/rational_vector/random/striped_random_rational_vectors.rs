// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_q::rational_vector::random::striped_random_rational_vectors;

fn striped_random_rational_vectors_helper(
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        striped_random_rational_vectors(
            EXAMPLE_SEED,
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_bits_numerator,
            mean_bits_denominator,
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
fn test_striped_random_rational_vectors() {
    // mean stripe = 2, mean bits = 2, mean dimension = 1
    striped_random_rational_vectors_helper(
        2,
        1,
        2,
        1,
        1,
        1,
        &[
            "(0)",
            "(0, -6)",
            "(3, 0)",
            "(0, 0)",
            "(-7/16)",
            "(0, -1/5)",
            "()",
            "()",
            "(1)",
            "()",
            "()",
            "()",
            "(0)",
            "(0, 1, 0, 0)",
            "()",
            "(19/2, 0)",
            "()",
            "(1, 0, -13, -1, -1/2)",
            "(1/3)",
            "(5/3)",
        ],
    );
    // mean stripe = 8, mean bits = 4, mean dimension = 2
    striped_random_rational_vectors_helper(
        8,
        1,
        4,
        1,
        2,
        1,
        &[
            "(0, 1, -3/71, 0, -7/128, 5/96)",
            "(-1/65)",
            "(-1/4, 1231/255, -1/7, 1, 0, 0, 25/7, 1/8)",
            "(1)",
            "(1047015/16, -192, 0, -15/8, -783784, -11, -8, 0, 0, 1365/2, -13/6, 56, 63/2, \
            -32/32259)",
            "()",
            "(0, -1, -31/16, 15/4, -1/3)",
            "(0, 0, -603, 0)",
            "(1/17)",
            "()",
            "(126/31, -8, -63/2, -2/3, 19/48, 3/127)",
            "(-1/4, -60)",
            "()",
            "()",
            "(5/87)",
            "(5/3, 2/15, 1/387)",
            "()",
            "(-1/7)",
            "()",
            "(-2/3)",
        ],
    );
    // mean stripe = 64, mean bits = 8, mean dimension = 4
    striped_random_rational_vectors_helper(
        64,
        1,
        8,
        1,
        4,
        1,
        &[
            "()",
            "(0, 1/31, 0, 63/1024, -31/512, 241/1920, -4095, -2048, 511/3, -2048/21, 9/191, \
            -32768/33587199, 1, 1/2048)",
            "(15/32752, 1/2147483648, 1/4, -1/4)",
            "(3, -4194304, -17, -31/64)",
            "(-1/128)",
            "()",
            "(7/1023, 87376/341, 1023/8, 0, 65011712)",
            "(16384, -4095)",
            "(1, -262144, 0, 1/16)",
            "()",
            "(-1048576/3, -1152921435887403007/8, -8192, -2/3, 3/4, 1/64)",
            "()",
            "()",
            "(4/511, -540671/8, -7/8, -2/3, 31, 273/4369, -4, -127/2048, 1048575/7, 1/131071, 510)",
            "(512, 0, -2048/3, 2/3, 0, -53/5, 16/31, 2/3)",
            "()",
            "(2, 2048/15, -1310719/16)",
            "()",
            "(7/15, 2047/1024, -33554432/32767, 1/32768, -1)",
            "(-48/31, -4096/65535, 43648/85, 16384/15, 273/17, 16/255, -127/16, 128, -4/2047)",
        ],
    );
}

#[test]
#[should_panic]
fn striped_random_rational_vectors_fail_1() {
    // mean_stripe_denominator is zero
    let _ = striped_random_rational_vectors(EXAMPLE_SEED, 1, 0, 4, 1, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_rational_vectors_fail_2() {
    // mean_stripe_numerator < mean_stripe_denominator
    let _ = striped_random_rational_vectors(EXAMPLE_SEED, 1, 2, 4, 1, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_rational_vectors_fail_3() {
    // mean_bits_denominator is zero
    let _ = striped_random_rational_vectors(EXAMPLE_SEED, 16, 1, 1, 0, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_rational_vectors_fail_4() {
    // mean_bits_numerator <= mean_bits_denominator
    let _ = striped_random_rational_vectors(EXAMPLE_SEED, 16, 1, 1, 1, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_rational_vectors_fail_5() {
    // mean_length_denominator is zero
    let _ = striped_random_rational_vectors(EXAMPLE_SEED, 16, 1, 4, 1, 2, 0);
}
