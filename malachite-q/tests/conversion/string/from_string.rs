// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::string_gen;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::generators::integer_gen;
use malachite_q::test_util::generators::string_gen_var_12;
use malachite_q::Rational;
use num::BigRational;
use std::str::FromStr;

#[test]
fn test_from_str() {
    let test_ok = |s, n| {
        assert_eq!(Rational::from_str(s).unwrap().to_string(), n);
        assert_eq!(BigRational::from_str(s).unwrap().to_string(), n);
        assert_eq!(rug::Rational::from_str(s).unwrap().to_string(), n);
    };
    test_ok("0", "0");
    test_ok("-0", "0");
    test_ok("123456", "123456");
    test_ok("1000000000000000000000000", "1000000000000000000000000");
    test_ok("-123456", "-123456");
    test_ok("-1000000000000000000000000", "-1000000000000000000000000");
    test_ok("01/02", "1/2");
    test_ok("3/21", "1/7");

    let test_err = |s, rug_err| {
        assert!(Rational::from_str(s).is_err());
        assert!(BigRational::from_str(s).is_err());
        let rn = rug::Rational::from_str(s);
        assert_eq!(rn.is_err() || rn.unwrap() < 0, rug_err);
    };
    test_err("12A", true);
    test_err(" 10", false);
    test_err("1.0", true);
    test_err("$%^", true);
    test_err("", true);
    test_err("-", true);
    test_err("1/0", true);
    test_err("/1", true);
    test_err("--0", true);
    test_err("-+0", true);
    test_err("+-0", true);
    test_err("++0", true);
    test_err("--1", true);
    test_err("-+1", true);
    test_err("+-1", true);
    test_err("++1", true);
}

#[allow(unused_must_use)]
#[test]
fn from_str_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 64);
    string_gen().test_properties_with_config(&config, |s| {
        Rational::from_str(&s);
    });

    string_gen_var_12().test_properties(|s| {
        let n = Rational::from_str(&s).unwrap();
        assert_eq!(BigRational::from_str(&s).unwrap(), BigRational::from(&n));
        assert_eq!(
            rug::Rational::from_str(&s).unwrap(),
            rug::Rational::from(&n)
        );
    });

    integer_gen().test_properties(|x| {
        let s = x.to_string();
        assert_eq!(
            Rational::from_str(&s).unwrap(),
            Integer::from_str(&s).unwrap()
        );
    });
}
