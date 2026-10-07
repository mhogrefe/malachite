// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::natural_vector::random::striped_random_natural_vectors;

fn striped_random_natural_vectors_helper(
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        striped_random_natural_vectors(
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
fn test_striped_random_natural_vectors() {
    // mean stripe = 2, mean bits = 1, mean dimension = 1
    striped_random_natural_vectors_helper(
        2,
        1,
        1,
        1,
        1,
        1,
        &[
            "(1)",
            "(0, 8)",
            "(0, 0)",
            "(0, 0)",
            "(7)",
            "(0, 0)",
            "()",
            "()",
            "(0)",
            "()",
            "()",
            "()",
            "(2)",
            "(2, 15, 3, 0)",
            "()",
            "(1, 1)",
            "()",
            "(1, 0, 6, 1, 0)",
            "(1)",
            "(10)",
        ],
    );
    // mean stripe = 8, mean bits = 4, mean dimension = 2
    striped_random_natural_vectors_helper(
        8,
        1,
        4,
        1,
        2,
        1,
        &[
            "(2, 0, 8200, 1, 4, 1)",
            "(0)",
            "(15, 15, 3, 1, 0, 1, 0, 0)",
            "(1)",
            "(7, 1, 8, 1, 1984, 7, 2, 51, 2, 127, 0, 511, 12, 7)",
            "()",
            "(1823, 3, 0, 0, 3)",
            "(1, 261, 760, 1)",
            "(0)",
            "()",
            "(1, 7, 56, 8319, 0, 4)",
            "(0, 0)",
            "()",
            "()",
            "(4)",
            "(8, 1, 24)",
            "()",
            "(1024)",
            "()",
            "(0)",
        ],
    );
    // mean stripe = 64, mean bits = 8, mean dimension = 4
    striped_random_natural_vectors_helper(
        64,
        1,
        8,
        1,
        4,
        1,
        &[
            "()",
            "(1, 64, 7, 2, 1, 3, 127, 7, 0, 65535, 1, 16, 0, 39)",
            "(0, 1024, 4096, 3)",
            "(128, 127, 1, 3)",
            "(1048576)",
            "()",
            "(0, 131071, 16777217, 0, 0)",
            "(4096, 1)",
            "(7, 3, 127, 8)",
            "()",
            "(262143, 2095104, 31, 2016, 255, 16)",
            "()",
            "()",
            "(515, 65536, 32, 8, 2097152, 2, 0, 256, 8, 0, 128)",
            "(1, 19, 262144, 14336, 4, 1023, 131071, 1016)",
            "()",
            "(511, 131071, 256)",
            "()",
            "(15, 8, 32, 31, 7)",
            "(8388607, 0, 2303, 7, 2, 3, 31, 0, 8)",
        ],
    );
}

#[test]
#[should_panic]
fn striped_random_natural_vectors_fail_1() {
    // mean_stripe_denominator is zero
    let _ = striped_random_natural_vectors(EXAMPLE_SEED, 1, 0, 4, 1, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_natural_vectors_fail_2() {
    // mean_stripe_numerator < mean_stripe_denominator
    let _ = striped_random_natural_vectors(EXAMPLE_SEED, 1, 2, 4, 1, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_natural_vectors_fail_3() {
    // mean_bits_denominator is zero
    let _ = striped_random_natural_vectors(EXAMPLE_SEED, 16, 1, 1, 0, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_natural_vectors_fail_4() {
    // mean_length_denominator is zero
    let _ = striped_random_natural_vectors(EXAMPLE_SEED, 16, 1, 4, 1, 2, 0);
}
