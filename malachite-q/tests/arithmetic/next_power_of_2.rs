// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    CeilingLogBase2, IsPowerOf2, NextPowerOf2, NextPowerOf2Assign, PowerOf2,
};
use malachite_base::num::basic::traits::Zero;
use malachite_nz::test_util::generators::natural_gen_var_2;
use malachite_q::test_util::generators::rational_gen_var_2;
use malachite_q::Rational;
use std::str::FromStr;

#[test]
fn test_next_power_of_2() {
    let test = |u, out| {
        let mut n = Rational::from_str(u).unwrap();
        n.next_power_of_2_assign();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Rational::from_str(u).unwrap().next_power_of_2();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&Rational::from_str(u).unwrap()).next_power_of_2();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("1", "1");
    test("2", "2");
    test("3", "4");
    test("4", "4");
    test("5", "8");
    test("6", "8");
    test("7", "8");
    test("8", "8");
    test("9", "16");
    test("10", "16");
    test("123", "128");
    test("1000", "1024");
    test("1000000", "1048576");
    test("1000000000", "1073741824");
    test("1000000000000", "1099511627776");
    test("2/3", "1");
    test("22/7", "4");
    test("1/10", "1/8");
    test("1/100", "1/64");
    test("1/1000000", "1/524288");
}

#[test]
#[should_panic]
fn next_power_of_2_fail() {
    Rational::ZERO.next_power_of_2();
}

#[test]
#[should_panic]
fn next_power_of_2_ref_fail() {
    (&Rational::ZERO).next_power_of_2();
}

#[test]
#[should_panic]
fn next_power_of_2_assign_fail() {
    let mut x = Rational::ZERO;
    x.next_power_of_2_assign();
}

#[test]
fn next_power_of_2_properties() {
    rational_gen_var_2().test_properties(|x| {
        let mut mut_x = x.clone();
        mut_x.next_power_of_2_assign();
        assert!(mut_x.is_valid());
        let result = mut_x;

        let result_alt = (&x).next_power_of_2();
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = x.clone().next_power_of_2();
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        assert!(result.is_power_of_2());
        assert!(result >= x);
        assert!(&result >> 1 < x);
        assert_eq!(Rational::power_of_2(x.ceiling_log_base_2()), result);
    });

    natural_gen_var_2().test_properties(|x| {
        assert_eq!((&x).next_power_of_2(), Rational::from(x).next_power_of_2());
    });
}
