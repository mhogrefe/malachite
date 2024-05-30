// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::random::variable_range_generator;
use malachite_base::random::EXAMPLE_SEED;
use std::panic::catch_unwind;

fn next_in_inclusive_range_helper<T: PrimitiveUnsigned>(a: T, b: T, expected_values: &[T]) {
    let mut range_generator = variable_range_generator(EXAMPLE_SEED);
    let mut xs = Vec::with_capacity(20);
    for _ in 0..20 {
        xs.push(range_generator.next_in_inclusive_range(a, b));
    }
    assert_eq!(xs, expected_values);
}

#[test]
fn test_next_in_inclusive_range() {
    next_in_inclusive_range_helper::<u8>(5, 5, &[5; 20]);
    next_in_inclusive_range_helper::<u16>(
        1,
        6,
        &[2, 6, 4, 2, 3, 5, 6, 2, 3, 6, 5, 1, 6, 1, 3, 6, 3, 1, 5, 1],
    );
    next_in_inclusive_range_helper::<u32>(
        10,
        19,
        &[11, 17, 15, 14, 16, 14, 12, 18, 11, 17, 15, 10, 12, 16, 13, 15, 12, 12, 19, 15],
    );
    next_in_inclusive_range_helper::<u8>(
        0,
        u8::MAX,
        &[
            113, 239, 69, 108, 228, 210, 168, 161, 87, 32, 110, 83, 188, 34, 89, 238, 93, 200, 149,
            115,
        ],
    );
}

fn next_in_inclusive_range_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!({
        let mut range_generator = variable_range_generator(EXAMPLE_SEED);
        range_generator.next_in_inclusive_range(T::TWO, T::ONE);
    });
}

#[test]
fn next_in_inclusive_range_fail() {
    apply_fn_to_unsigneds!(next_in_inclusive_range_fail_helper);
}
