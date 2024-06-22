// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::Zero;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::integer::random::get_uniform_random_integer_in_range;
use malachite_nz::integer::Integer;
use std::str::FromStr;

fn get_uniform_random_integer_in_range_helper(a: &str, b: &str, out: &str) {
    assert_eq!(
        get_uniform_random_integer_in_range(
            &mut random_primitive_ints(EXAMPLE_SEED),
            Integer::from_str(a).unwrap(),
            Integer::from_str(b).unwrap()
        )
        .to_string(),
        out
    );
}

#[test]
fn test_get_uniform_random_integer_in_range() {
    get_uniform_random_integer_in_range_helper("0", "1", "0");
    get_uniform_random_integer_in_range_helper("1950", "2020", "1957");
    get_uniform_random_integer_in_range_helper("-10", "10", "7");
    get_uniform_random_integer_in_range_helper("-10", "0", "-9");
}

#[test]
#[should_panic]
fn get_uniform_random_integer_in_range_fail() {
    get_uniform_random_integer_in_range(
        &mut random_primitive_ints(EXAMPLE_SEED),
        Integer::ZERO,
        Integer::ZERO,
    );
}
