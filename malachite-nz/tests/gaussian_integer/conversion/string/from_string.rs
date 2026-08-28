// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::test_util::generators::string_gen;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::test_util::generators::gaussian_integer_gen;

#[test]
fn test_from_str() {
    let test = |s, out| {
        assert_eq!(GaussianInteger::from_str(s).unwrap().to_string(), out);
    };
    test("0", "0");
    test("-2", "-2");
    test("+1", "1");
    test("007", "7");
    test("-0", "0");
    test("i", "i");
    test("-i", "-i");
    test("+i", "i");
    test("0i", "0");
    test("-0i", "0");
    test("1i", "i");
    test("-1i", "-i");
    test("+1i", "i");
    test("007i", "7i");
    test("2+3i", "2+3i");
    test("2-3i", "2-3i");
    test("-2+3i", "-2+3i");
    test("-2-3i", "-2-3i");
    test("2+i", "2+i");
    test("2-i", "2-i");
    test("1+0i", "1");
    test("2-0i", "2");
    test("0+0i", "0");
    test("0+1i", "i");
    test("2+1i", "2+i");
    test("+2+3i", "2+3i");

    let test_err = |s| {
        assert!(GaussianInteger::from_str(s).is_err(), "should reject {s:?}");
    };
    test_err("");
    test_err("+");
    test_err("-");
    test_err("1-");
    test_err("i+1");
    test_err("1 + i");
    test_err(" 1");
    test_err("1+i2");
    test_err("2i+1");
    test_err("1+2");
    test_err("2+3");
    test_err("3i+2i");
    test_err("1++i");
    test_err("++i");
    test_err("+-i");
    test_err("-+i");
    test_err("++1");
    test_err("2+-3i");
    test_err("2-+3i");
    test_err("ii");
    test_err("2ii");
    test_err("i2");
    test_err("12i3i");
    test_err("1+3-2i");
    test_err("j");
    test_err("1+j");
    test_err("0x1i");
}

#[test]
fn from_str_properties() {
    string_gen().test_properties(|s| {
        // must not panic, whether or not the string is valid
        let _ = GaussianInteger::from_str(&s);
    });

    gaussian_integer_gen().test_properties(|x| {
        assert_eq!(GaussianInteger::from_str(&x.to_string()).unwrap(), x);
    });
}
