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

fn next_bit_chunk_helper<T: PrimitiveUnsigned>(chunk_size: u64, expected_values: &[T]) {
    let mut range_generator = variable_range_generator(EXAMPLE_SEED);
    let mut xs = Vec::with_capacity(20);
    for _ in 0..20 {
        xs.push(range_generator.next_bit_chunk::<T>(chunk_size));
    }
    assert_eq!(xs, expected_values);
}

#[test]
fn test_next_bit_chunk() {
    next_bit_chunk_helper::<u16>(
        1,
        &[1, 0, 0, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 0],
    );
    next_bit_chunk_helper::<u32>(
        2,
        &[1, 0, 3, 1, 3, 3, 2, 3, 1, 1, 0, 1, 0, 3, 2, 1, 0, 1, 2, 3],
    );
    next_bit_chunk_helper::<u64>(
        3,
        &[1, 6, 5, 7, 6, 3, 1, 2, 4, 5, 1, 2, 6, 5, 4, 6, 0, 5, 6, 0],
    );
    next_bit_chunk_helper::<u128>(
        7,
        &[113, 94, 23, 98, 70, 92, 52, 84, 33, 47, 1, 113, 54, 10, 47, 17, 89, 92, 119, 66],
    );
    next_bit_chunk_helper::<u8>(
        8,
        &[
            113, 239, 69, 108, 228, 210, 168, 161, 87, 32, 110, 83, 188, 34, 89, 238, 93, 200, 149,
            115,
        ],
    );
}

fn next_bit_chunk_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!({
        let mut range_generator = variable_range_generator(EXAMPLE_SEED);
        range_generator.next_bit_chunk::<T>(0)
    });
    assert_panic!({
        let mut range_generator = variable_range_generator(EXAMPLE_SEED);
        range_generator.next_bit_chunk::<T>(T::WIDTH + 1)
    });
}

#[test]
fn next_bit_chunk_fail() {
    apply_fn_to_unsigneds!(next_bit_chunk_fail_helper);
}
