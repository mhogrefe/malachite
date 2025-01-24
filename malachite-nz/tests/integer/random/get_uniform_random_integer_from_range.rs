// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::strings::ToDebugString;
use malachite_nz::integer::random::get_uniform_random_integer_from_range;
use malachite_nz::integer::Integer;
use std::str::FromStr;

fn get_uniform_random_integer_from_range_helper(a: &str, b: &str, out: &str) {
    let mut xs = random_primitive_ints(EXAMPLE_SEED.fork("ints"));
    let xs = (0..10)
        .map(|_| {
            get_uniform_random_integer_from_range(
                &mut xs,
                Integer::from_str(a).unwrap(),
                Integer::from_str(b).unwrap(),
            )
        })
        .collect_vec();
    assert_eq!(xs.to_debug_string(), out);
}

#[test]
fn test_get_uniform_random_integer_from_range() {
    get_uniform_random_integer_from_range_helper("0", "1", "[0, 0, 0, 0, 0, 0, 0, 0, 0, 0]");
    get_uniform_random_integer_from_range_helper(
        "1950",
        "2020",
        "[1965, 1958, 1994, 1952, 1963, 1953, 1999, 1971, 1970, 2011]",
    );
    get_uniform_random_integer_from_range_helper("-10", "10", "[5, -2, 2, -8, 3, -8, -7, 4, 9, 7]");
    get_uniform_random_integer_from_range_helper(
        "-10",
        "0",
        "[-2, -8, -8, -7, -7, -9, -7, -5, -3, -6]",
    );
}

#[test]
#[should_panic]
fn get_uniform_random_integer_from_range_fail() {
    get_uniform_random_integer_from_range(
        &mut random_primitive_ints(EXAMPLE_SEED),
        Integer::ZERO,
        Integer::ZERO,
    );
}
