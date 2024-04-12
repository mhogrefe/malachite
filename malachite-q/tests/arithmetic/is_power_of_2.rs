// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    CheckedLogBase2, IsPowerOf2, NextPowerOf2, Reciprocal,
};
use malachite_nz::test_util::generators::natural_gen;
use malachite_q::test_util::generators::{rational_gen, rational_gen_var_1, rational_gen_var_2};
use malachite_q::Rational;
use std::str::FromStr;

#[test]
fn test_is_power_of_2() {
    let test = |x, out| {
        assert_eq!(Rational::from_str(x).unwrap().is_power_of_2(), out);
    };
    test("0", false);
    test("1", true);
    test("2", true);
    test("3", false);
    test("4", true);
    test("5", false);
    test("6", false);
    test("7", false);
    test("8", true);
    test("1024", true);
    test("1025", false);
    test("1000000000000", false);
    test("1099511627776", true);
    test("1/2", true);
    test("1/3", false);
    test("1/4", true);
    test("1/5", false);
    test("1/6", false);
    test("1/7", false);
    test("1/8", true);
    test("1/1024", true);
    test("1/1025", false);
    test("1/1000000000000", false);
    test("1/1099511627776", true);

    test("22/7", false);
    test("-1", false);
}

#[test]
fn is_power_of_2_properties() {
    rational_gen().test_properties(|x| {
        if x.is_power_of_2() {
            assert!(x >= 0u32);
        }
    });

    rational_gen_var_2().test_properties(|x| {
        let is_power = x.is_power_of_2();
        assert_eq!((&x).next_power_of_2() == x, is_power);
        assert_eq!(x.checked_log_base_2().is_some(), is_power);
    });

    rational_gen_var_1().test_properties(|x| {
        assert_eq!((&x).reciprocal().is_power_of_2(), x.is_power_of_2());
    });

    natural_gen().test_properties(|x| {
        assert_eq!(x.is_power_of_2(), Rational::from(x).is_power_of_2());
    });
}
