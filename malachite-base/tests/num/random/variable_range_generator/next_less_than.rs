// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::random::VariableRangeGenerator;
use malachite_base::random::EXAMPLE_SEED;
use std::panic::catch_unwind;

fn next_less_than_helper<T: PrimitiveUnsigned>(limit: T, expected_values: &[T]) {
    let mut range_generator = VariableRangeGenerator::new(EXAMPLE_SEED);
    let mut xs = Vec::with_capacity(20);
    for _ in 0..20 {
        xs.push(range_generator.next_less_than(limit));
    }
    assert_eq!(xs, expected_values);
}

#[test]
fn test_next_less_than() {
    next_less_than_helper::<u8>(1, &[0; 20]);
    next_less_than_helper::<u16>(
        2,
        &[1, 0, 0, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 0],
    );
    next_less_than_helper::<u32>(
        3,
        &[1, 0, 1, 2, 1, 1, 0, 1, 0, 2, 1, 0, 1, 2, 2, 0, 1, 0, 2, 2],
    );
    next_less_than_helper::<u64>(
        4,
        &[1, 0, 3, 1, 3, 3, 2, 3, 1, 1, 0, 1, 0, 3, 2, 1, 0, 1, 2, 3],
    );
    next_less_than_helper::<u128>(
        10,
        &[1, 7, 5, 4, 6, 4, 2, 8, 1, 7, 5, 0, 2, 6, 3, 5, 2, 2, 9, 5],
    );
}

fn next_less_than_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!({
        let mut range_generator = VariableRangeGenerator::new(EXAMPLE_SEED);
        range_generator.next_less_than(T::ZERO);
    });
}

#[test]
fn next_less_than_fail() {
    apply_fn_to_unsigneds!(next_less_than_fail_helper);
}
