// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::vector::random::random_vectors;

fn random_vectors_helper(
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        random_vectors(
            EXAMPLE_SEED,
            &random_primitive_ints::<u8>,
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
fn test_random_vectors() {
    // mean dimension = 1
    random_vectors_helper(
        1,
        1,
        &[
            "(85)",
            "(11, 136)",
            "(200, 235)",
            "(134, 203)",
            "(223)",
            "(38, 235)",
            "()",
            "()",
            "(217)",
            "()",
            "()",
            "()",
            "(177)",
            "(162, 32, 166, 234)",
            "()",
            "(30, 218)",
            "()",
            "(90, 106, 9, 216, 204)",
            "(151)",
            "(213)",
        ],
    );
    // mean dimension = 2
    random_vectors_helper(
        2,
        1,
        &[
            "(85, 11, 136, 200, 235, 134)",
            "(203)",
            "(223, 38, 235, 217, 177, 162, 32, 166)",
            "(234)",
            "(30, 218, 90, 106, 9, 216, 204, 151, 213, 97, 253, 78, 91, 39)",
            "()",
            "(191, 175, 170, 232, 233)",
            "(2, 35, 22, 217)",
            "(198)",
            "()",
            "(114, 17, 32, 173, 114, 65)",
            "(121, 222)",
            "()",
            "()",
            "(173)",
            "(25, 144, 148)",
            "()",
            "(79)",
            "()",
            "(115)",
        ],
    );
    // mean dimension = 4
    random_vectors_helper(
        4,
        1,
        &[
            "()",
            "(85, 11, 136, 200, 235, 134, 203, 223, 38, 235, 217, 177, 162, 32)",
            "(166, 234, 30, 218)",
            "(90, 106, 9, 216)",
            "(204)",
            "()",
            "(151, 213, 97, 253, 78)",
            "(91, 39)",
            "(191, 175, 170, 232)",
            "()",
            "(233, 2, 35, 22, 217, 198)",
            "()",
            "()",
            "(114, 17, 32, 173, 114, 65, 121, 222, 173, 25, 144)",
            "(148, 79, 115, 52, 73, 69, 137, 91)",
            "()",
            "(153, 178, 112)",
            "()",
            "(34, 95, 106, 167, 197)",
            "(130, 168, 122, 207, 172, 177, 86, 150, 221)",
        ],
    );
}

#[test]
#[should_panic]
fn random_vectors_fail_1() {
    // mean_length_denominator is zero
    let _ = random_vectors(EXAMPLE_SEED, &random_primitive_ints::<u8>, 1, 0);
}

#[test]
#[should_panic]
fn random_vectors_fail_2() {
    // mean_length_numerator is zero
    let _ = random_vectors(EXAMPLE_SEED, &random_primitive_ints::<u8>, 0, 1);
}
