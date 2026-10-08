// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::random::random_unsigned_inclusive_range;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::unsigned_vector::random::random_unsigned_vectors_from_iterator;

fn random_unsigned_vectors_from_iterator_helper<T: PrimitiveUnsigned>(
    bound: T,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        random_unsigned_vectors_from_iterator(
            EXAMPLE_SEED,
            &|seed| random_unsigned_inclusive_range(seed, T::ZERO, bound),
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
fn test_random_unsigned_vectors_from_iterator() {
    // u8 elements in [0, 1], mean dimension = 1
    random_unsigned_vectors_from_iterator_helper::<u8>(
        1,
        1,
        1,
        &[
            "(1)",
            "(0, 1)",
            "(0, 1)",
            "(0, 1)",
            "(0)",
            "(1, 1)",
            "()",
            "()",
            "(0)",
            "()",
            "()",
            "()",
            "(1)",
            "(0, 0, 0, 0)",
            "()",
            "(0, 0)",
            "()",
            "(0, 1, 0, 0, 0)",
            "(1)",
            "(0)",
        ],
    );
    // u8 elements in [0, 15], mean dimension = 2
    random_unsigned_vectors_from_iterator_helper::<u8>(
        15,
        2,
        1,
        &[
            "(5, 5, 11, 0, 8, 8)",
            "(8)",
            "(12, 11, 14, 6, 8, 11, 12, 15)",
            "(13)",
            "(6, 2, 11, 14, 9, 13, 1, 11, 2, 10, 0, 2, 6, 10)",
            "()",
            "(10, 14, 14, 1, 10)",
            "(13, 10, 5, 10)",
            "(6)",
            "()",
            "(9, 0, 8, 13, 12, 12)",
            "(7, 9)",
            "()",
            "()",
            "(5)",
            "(13, 1, 6)",
            "()",
            "(13)",
            "()",
            "(15)",
        ],
    );
    // u64 elements in [0, 1000], mean dimension = 4
    random_unsigned_vectors_from_iterator_helper::<u64>(
        1000,
        4,
        1,
        &[
            "()",
            "(853, 514, 136, 943, 902, 621, 940, 473, 172, 522, 664, 746, 647, 429)",
            "(425, 9, 822, 380)",
            "(854, 353, 959, 436)",
            "(157)",
            "()",
            "(959, 683, 650, 935, 770)",
            "(392, 401)",
            "(795, 370, 4, 722)",
            "()",
            "(458, 321, 926, 733, 102, 144)",
            "()",
            "()",
            "(997, 820, 209, 329, 593, 440, 613, 178, 156, 498, 425)",
            "(423, 177, 648, 490, 207, 107, 363, 601)",
            "()",
            "(733, 374, 822)",
            "()",
            "(297, 777, 862, 838, 804)",
            "(927, 701, 783, 532, 235, 561, 530, 417, 580)",
        ],
    );
}

#[test]
#[should_panic]
fn random_unsigned_vectors_from_iterator_fail_1() {
    // mean_length_denominator is zero
    let _ = random_unsigned_vectors_from_iterator(
        EXAMPLE_SEED,
        &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, 1),
        1,
        0,
    );
}

#[test]
#[should_panic]
fn random_unsigned_vectors_from_iterator_fail_2() {
    // mean_length_numerator is zero
    let _ = random_unsigned_vectors_from_iterator(
        EXAMPLE_SEED,
        &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, 1),
        0,
        1,
    );
}
