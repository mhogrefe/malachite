// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    CheckedLogBase2, NextPowerOf2, Pow, PowerOf2, Reciprocal,
};
use malachite_base::num::basic::traits::{One, Two};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::{signed_gen_var_5, unsigned_gen_var_5};
use malachite_nz::natural::Natural;
use malachite_q::Rational;

#[test]
fn test_power_of_2() {
    let test = |pow: u64, out| assert_eq!(Rational::power_of_2(pow).to_string(), out);
    test(0, "1");
    test(1, "2");
    test(2, "4");
    test(3, "8");
    test(32, "4294967296");
    test(100, "1267650600228229401496703205376");

    let test = |pow: i64, out| assert_eq!(Rational::power_of_2(pow).to_string(), out);
    test(0, "1");
    test(1, "2");
    test(2, "4");
    test(3, "8");
    test(32, "4294967296");
    test(100, "1267650600228229401496703205376");
    test(-1, "1/2");
    test(-2, "1/4");
    test(-3, "1/8");
    test(-32, "1/4294967296");
    test(-100, "1/1267650600228229401496703205376");
}

#[test]
fn power_of_2_properties() {
    unsigned_gen_var_5().test_properties(|pow| {
        let x = Rational::power_of_2(pow);
        assert!(x.is_valid());

        assert_eq!(x, Rational::ONE << pow);
        assert_eq!(x, Rational::TWO.pow(pow));
        assert_eq!(x.checked_log_base_2(), Some(i64::exact_from(pow)));
        assert_eq!((&x).next_power_of_2(), x);
        assert_eq!(Natural::power_of_2(pow), x);
    });

    signed_gen_var_5::<i64>().test_properties(|pow| {
        let x = Rational::power_of_2(pow);
        assert!(x.is_valid());

        assert_eq!(x, Rational::ONE << pow);
        assert_eq!(x, Rational::TWO.pow(pow));
        assert_eq!(x.checked_log_base_2(), Some(pow));
        assert_eq!((&x).next_power_of_2(), x);
        if pow >= 0 {
            assert_eq!(Natural::power_of_2(pow.unsigned_abs()), x);
        } else {
            assert_eq!(Natural::power_of_2(pow.unsigned_abs()), x.reciprocal());
        }
    });
}
