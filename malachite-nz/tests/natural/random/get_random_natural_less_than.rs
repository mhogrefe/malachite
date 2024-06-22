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
use malachite_nz::natural::random::get_random_natural_less_than;
use malachite_nz::natural::Natural;
use std::str::FromStr;

fn get_random_natural_less_than_helper(limit: &str, out: &str) {
    assert_eq!(
        get_random_natural_less_than(
            &mut random_primitive_ints(EXAMPLE_SEED),
            &Natural::from_str(limit).unwrap()
        )
        .to_string(),
        out
    );
}

#[test]
fn test_get_random_natural_less_than() {
    get_random_natural_less_than_helper("1", "0");
    get_random_natural_less_than_helper("10", "1");
    get_random_natural_less_than_helper("100", "87");
    get_random_natural_less_than_helper("1000", "881");
    get_random_natural_less_than_helper(
        "10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000",
        "65136961652069212043112615237392696010837604314202102958615643812057836484425634981051963\
        29975541617",
    );
}

#[test]
#[should_panic]
fn get_random_natural_less_than_fail() {
    get_random_natural_less_than(&mut random_primitive_ints(EXAMPLE_SEED), &Natural::ZERO);
}
