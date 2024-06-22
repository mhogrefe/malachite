// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Pow;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::random::{random_primitive_ints, variable_range_generator};
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::integer::random::get_random_integer_from_range_to_infinity;
use malachite_nz::integer::Integer;
use std::str::FromStr;

fn get_random_integer_from_range_to_infinity_helper(
    a: &str,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    out: &str,
) {
    assert_eq!(
        get_random_integer_from_range_to_infinity(
            &mut random_primitive_ints(EXAMPLE_SEED.fork("ints")),
            &mut variable_range_generator(EXAMPLE_SEED.fork("vr")),
            Integer::from_str(a).unwrap(),
            mean_bits_numerator,
            mean_bits_denominator
        )
        .to_string(),
        out
    );
}

#[test]
fn test_get_random_integer_from_range_to_infinity() {
    get_random_integer_from_range_to_infinity_helper("0", 1, 1, "0");
    get_random_integer_from_range_to_infinity_helper("0", 10, 1, "7");
    get_random_integer_from_range_to_infinity_helper(
        "0",
        100,
        1,
        "5101205056696451696397798478058511",
    );
    get_random_integer_from_range_to_infinity_helper("1000", 11, 1, "1015");
    get_random_integer_from_range_to_infinity_helper(
        "1000",
        100,
        1,
        "1206982412795330974999926231143439",
    );
    get_random_integer_from_range_to_infinity_helper("-1000", 1, 1, "-3");
    get_random_integer_from_range_to_infinity_helper("-1000", 11, 1, "-3");
    get_random_integer_from_range_to_infinity_helper("-1000", 100, 1, "70671");
}

#[test]
#[should_panic]
fn get_random_integer_from_range_to_infinity_fail_1() {
    get_random_integer_from_range_to_infinity(
        &mut random_primitive_ints(EXAMPLE_SEED.fork("ints")),
        &mut variable_range_generator(EXAMPLE_SEED.fork("vr")),
        Integer::ZERO,
        1,
        0,
    );
}

#[test]
#[should_panic]
fn get_random_integer_from_range_to_infinity_fail_2() {
    get_random_integer_from_range_to_infinity(
        &mut random_primitive_ints(EXAMPLE_SEED.fork("ints")),
        &mut variable_range_generator(EXAMPLE_SEED.fork("vr")),
        Integer::ZERO,
        2,
        0,
    );
}

#[test]
#[should_panic]
fn get_random_integer_from_range_to_infinity_fail_3() {
    get_random_integer_from_range_to_infinity(
        &mut random_primitive_ints(EXAMPLE_SEED.fork("ints")),
        &mut variable_range_generator(EXAMPLE_SEED.fork("vr")),
        Integer::from(10u32).pow(100),
        10,
        1,
    );
}
