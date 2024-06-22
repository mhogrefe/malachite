// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::random::variable_range_generator;
use malachite_base::random::EXAMPLE_SEED;

#[test]
fn test_next_bool() {
    let mut range_generator = variable_range_generator(EXAMPLE_SEED);
    let mut xs = Vec::with_capacity(100);
    for _ in 0..100 {
        xs.push(range_generator.next_bool());
    }
    assert_eq!(
        xs,
        &[
            true, false, true, false, true, true, true, true, true, false, true, false, false,
            true, false, false, false, true, true, false, false, true, true, true, false, false,
            true, false, false, false, false, true, true, false, true, false, true, false, true,
            true, true, true, true, false, false, false, true, true, false, false, false, true,
            false, true, true, false, true, false, true, false, true, false, false, true, true,
            false, true, false, true, false, true, false, true, true, false, false, false, false,
            true, true, false, true, true, false, false, true, true, true, true, false, true, true,
            true, false, true, false, true, false, true, true
        ]
    );
}
