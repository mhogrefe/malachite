// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::test_util::generators::string_gen;
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::test_util::generators::gaussian_rational_gen;

#[test]
fn test_from_str() {
    let test = |s, out| {
        assert_eq!(GaussianRational::from_str(s).unwrap().to_string(), out);
    };
    test("0", "0");
    test("-2/3", "-2/3");
    test("4/6", "2/3");
    test("+1", "1");
    test("007", "7");
    test("i", "i");
    test("-i", "-i");
    test("+i", "i");
    test("i/2", "i/2");
    test("-i/2", "-i/2");
    test("5i/6", "5i/6");
    test("-5i/6", "-5i/6");
    test("0i", "0");
    test("0i/5", "0");
    test("1i", "i");
    test("1i/2", "i/2");
    test("2i/4", "i/2");
    test("2/3+5i/6", "2/3+5i/6");
    test("2/3-5i/6", "2/3-5i/6");
    test("-2/3+i/2", "-2/3+i/2");
    test("1-i/2", "1-i/2");
    test("2/3+1i/2", "2/3+i/2");
    test("2/3+0i", "2/3");
    test("0+0i", "0");
    test("+2/3+5i/6", "2/3+5i/6");
    test("1/+2+3i/+4", "1/2+3i/4");

    let test_err = |s| {
        assert!(
            GaussianRational::from_str(s).is_err(),
            "should reject {s:?}"
        );
    };
    test_err("");
    test_err("+");
    test_err("-");
    test_err("i+1");
    test_err("2/3i");
    test_err("3/4i/5");
    test_err("i/");
    test_err("/2i");
    test_err("i/0");
    test_err("5i/0");
    test_err("1/0+i");
    test_err("2i/");
    test_err("ii");
    test_err("2i2");
    test_err("i2");
    test_err("1 + i");
    test_err("2+-5i/6");
    test_err("2/-3");
    test_err("5i/-6");
    test_err("1+3i/");
    test_err("2//3i");
    test_err("j");
    test_err("1+j");
}

#[test]
fn from_str_properties() {
    string_gen().test_properties(|s| {
        // must not panic, whether or not the string is valid
        let _ = GaussianRational::from_str(&s);
    });

    gaussian_rational_gen().test_properties(|x| {
        assert_eq!(GaussianRational::from_str(&x.to_string()).unwrap(), x);
    });
}
