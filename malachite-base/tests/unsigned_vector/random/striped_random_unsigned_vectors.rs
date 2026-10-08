// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::unsigned_vector::random::striped_random_unsigned_vectors;

fn striped_random_unsigned_vectors_helper(
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        striped_random_unsigned_vectors::<u8>(
            EXAMPLE_SEED,
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_length_numerator,
            mean_length_denominator,
        )
        .take(20)
        .map(|v| v.to_string())
        .collect_vec(),
        expected_values
    );
}

#[test]
fn test_striped_random_unsigned_vectors() {
    // mean stripe length = 4, mean dimension = 1
    striped_random_unsigned_vectors_helper(
        4,
        1,
        1,
        1,
        &[
            "(255)",
            "(130, 152)",
            "(6, 62)",
            "(119, 1)",
            "(128)",
            "(203, 3)",
            "()",
            "()",
            "(30)",
            "()",
            "()",
            "()",
            "(7)",
            "(0, 24, 63, 192)",
            "()",
            "(30, 0)",
            "()",
            "(240, 195, 248, 56, 231)",
            "(7)",
            "(193)",
        ],
    );
    // mean stripe length = 4, mean dimension = 2
    striped_random_unsigned_vectors_helper(
        4,
        1,
        2,
        1,
        &[
            "(255, 130, 152, 6, 62, 119)",
            "(1)",
            "(128, 203, 3, 30, 7, 0, 24, 63)",
            "(192)",
            "(30, 0, 240, 195, 248, 56, 231, 7, 193, 194, 96, 126, 253, 119)",
            "()",
            "(7, 77, 231, 20, 120)",
            "(127, 177, 39, 179)",
            "(14)",
            "()",
            "(140, 25, 252, 224, 211, 199)",
            "(8, 15)",
            "()",
            "()",
            "(1)",
            "(11, 31, 126)",
            "()",
            "(248)",
            "()",
            "(231)",
        ],
    );
    // mean stripe length = 16, mean dimension = 4
    striped_random_unsigned_vectors_helper(
        16,
        1,
        4,
        1,
        &[
            "()",
            "(254, 255, 243, 15, 31, 0, 0, 255, 255, 0, 0, 0, 1, 0)",
            "(12, 254, 3, 0)",
            "(135, 128, 255, 0)",
            "(192)",
            "()",
            "(0, 252, 255, 0, 0)",
            "(251, 0)",
            "(0, 0, 255, 7)",
            "()",
            "(0, 1, 255, 0, 255, 127)",
            "()",
            "()",
            "(255, 0, 255, 252, 255, 255, 0, 17, 0, 0, 15)",
            "(127, 255, 252, 255, 56, 255, 255, 159)",
            "()",
            "(0, 15, 0)",
            "()",
            "(255, 63, 0, 0, 0)",
            "(255, 127, 62, 255, 255, 128, 255, 63, 239)",
        ],
    );
    // mean stripe length = 2, mean dimension = 4
    striped_random_unsigned_vectors_helper(
        2,
        1,
        4,
        1,
        &[
            "()",
            "(200, 207, 160, 21, 68, 47, 97, 221, 150, 26, 94, 92, 118, 29)",
            "(93, 230, 17, 87)",
            "(231, 201, 229, 18)",
            "(128)",
            "()",
            "(89, 192, 225, 11, 81)",
            "(232, 56)",
            "(40, 19, 155, 58)",
            "()",
            "(2, 15, 253, 100, 145, 54)",
            "()",
            "()",
            "(243, 33, 147, 172, 245, 208, 49, 39, 111, 109, 45)",
            "(21, 187, 135, 161, 26, 129, 234, 189)",
            "()",
            "(88, 113, 35)",
            "()",
            "(174, 106, 5, 38, 122)",
            "(171, 67, 92, 162, 230, 169, 189, 20, 174)",
        ],
    );
}

#[test]
#[should_panic]
fn striped_random_unsigned_vectors_fail_1() {
    // mean_stripe_denominator is zero
    let _ = striped_random_unsigned_vectors::<u8>(EXAMPLE_SEED, 4, 0, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_unsigned_vectors_fail_2() {
    // mean_stripe_numerator < mean_stripe_denominator
    let _ = striped_random_unsigned_vectors::<u8>(EXAMPLE_SEED, 1, 2, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_unsigned_vectors_fail_3() {
    // mean_length_denominator is zero
    let _ = striped_random_unsigned_vectors::<u8>(EXAMPLE_SEED, 4, 1, 1, 0);
}

#[test]
#[should_panic]
fn striped_random_unsigned_vectors_fail_4() {
    // mean_length_numerator is zero
    let _ = striped_random_unsigned_vectors::<u8>(EXAMPLE_SEED, 4, 1, 0, 1);
}
