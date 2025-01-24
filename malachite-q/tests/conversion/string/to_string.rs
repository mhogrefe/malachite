// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::strings::string_is_subset;
use malachite_base::strings::ToDebugString;
use malachite_nz::test_util::generators::integer_gen;
use malachite_q::test_util::generators::rational_gen;
use malachite_q::Rational;
use num::BigRational;
use std::str::FromStr;

#[test]
pub fn test_to_string() {
    fn test(u: &str) {
        let x = Rational::from_str(u).unwrap();
        assert_eq!(x.to_string(), u);
        assert_eq!(x.to_debug_string(), u);
    }
    test("0");
    test("2");
    test("123");
    test("1000");
    test("1000000");
    test("1000000000000000");
    test("-2");
    test("-123");
    test("-1000");
    test("-1000000");
    test("-1000000000000000");
    test("99/100");
    test("101/100");
    test("22/7");
    test("-99/100");
    test("-101/100");
    test("-22/7");
}

#[test]
fn to_string_properties() {
    rational_gen().test_properties(|x| {
        let s = x.to_string();
        assert_eq!(x.to_debug_string(), s);
        assert_eq!(BigRational::from(&x).to_string(), s);
        assert_eq!(rug::Rational::from(&x).to_string(), s);
        assert!(string_is_subset(&s, "-/0123456789"));
        if x != 0 {
            assert!(!s.starts_with('0'));
        }
    });

    integer_gen().test_properties(|x| {
        assert_eq!(Rational::from(&x).to_string(), x.to_string());
    });
}
