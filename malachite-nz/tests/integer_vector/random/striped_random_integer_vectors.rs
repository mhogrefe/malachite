// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::integer_vector::random::striped_random_integer_vectors;

fn striped_random_integer_vectors_helper(
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        striped_random_integer_vectors(
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
fn test_striped_random_integer_vectors() {
    // mean stripe = 2, mean bits = 1, mean dimension = 1
    striped_random_integer_vectors_helper(
        2,
        1,
        1,
        1,
        1,
        1,
        &[
            "(0)",
            "(1, 4)",
            "(-3, 0)",
            "(4, -6)",
            "(-7511)",
            "(-1, 0)",
            "()",
            "()",
            "(0)",
            "()",
            "()",
            "()",
            "(1)",
            "(-1, 1, 1, -1)",
            "()",
            "(-4, -1)",
            "()",
            "(0, 0, 0, 1, 0)",
            "(0)",
            "(8)",
        ],
    );
    // mean stripe = 8, mean bits = 4, mean dimension = 2
    striped_random_integer_vectors_helper(
        8,
        1,
        4,
        1,
        2,
        1,
        &[
            "(-272, -1, 1, 16, 512, 515)",
            "(-1)",
            "(79, -15999, 511, 32, -65, -4, 0, -142)",
            "(-1)",
            "(0, 4, -1, 10, 15, -61966, 0, -1, -1023, 16, 0, 0, -7, 3)",
            "()",
            "(-255, 131, 256, -511, 1)",
            "(-63, -30720, 1, 1)",
            "(-31)",
            "()",
            "(-1, 64, -1, -2016, -32, 0)",
            "(0, -4096)",
            "()",
            "()",
            "(4)",
            "(2046, 511, 243)",
            "()",
            "(2)",
            "()",
            "(-10)",
        ],
    );
    // mean stripe = 64, mean bits = 8, mean dimension = 4
    striped_random_integer_vectors_helper(
        64,
        1,
        8,
        1,
        4,
        1,
        &[
            "()",
            "(-4096, -67108864, 32771, 8192, 8589934592, 2047, -15, 47, -2097151, 511, -32, 1, 16, \
            -4)",
            "(-127, 0, -1, -127)",
            "(256, -1023, 8128, 8191)",
            "(63)",
            "()",
            "(64, -127, -129, -63, -15)",
            "(-32, -15)",
            "(-4, 7, -1, -1)",
            "()",
            "(-128, -7, 8, 1, -1, 8192)",
            "()",
            "()",
            "(16, -256, 17406, -1, 63, 0, 16383, 0, -512, -65536, 4)",
            "(49151, 4095, 256, -34359738367, -2047, -1, 0, 3)",
            "()",
            "(-16, 4, 0)",
            "()",
            "(4159, 1, 11, 1023, 1023)",
            "(0, 256, 4095, 17179873279, -8, 3, 524288, -17179869168, -2)",
        ],
    );
}

#[test]
#[should_panic]
fn striped_random_integer_vectors_fail_1() {
    // mean_stripe_denominator is zero
    let _ = striped_random_integer_vectors(EXAMPLE_SEED, 1, 0, 4, 1, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_integer_vectors_fail_2() {
    // mean_stripe_numerator < mean_stripe_denominator
    let _ = striped_random_integer_vectors(EXAMPLE_SEED, 1, 2, 4, 1, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_integer_vectors_fail_3() {
    // mean_bits_denominator is zero
    let _ = striped_random_integer_vectors(EXAMPLE_SEED, 16, 1, 1, 0, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_integer_vectors_fail_4() {
    // mean_length_denominator is zero
    let _ = striped_random_integer_vectors(EXAMPLE_SEED, 16, 1, 4, 1, 2, 0);
}
