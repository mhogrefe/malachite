// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Abs, AbsSquared, AbsSquaredAssign, Square};
use malachite_q::Rational;
use malachite_q::test_util::generators::rational_gen;
use std::str::FromStr;

#[test]
fn test_abs_squared() {
    let test = |s, out| {
        let x = Rational::from_str(s).unwrap();

        let squared = x.clone().abs_squared();
        assert!(squared.is_valid());
        assert_eq!(squared.to_string(), out);

        let squared = (&x).abs_squared();
        assert!(squared.is_valid());
        assert_eq!(squared.to_string(), out);

        let mut squared = x;
        squared.abs_squared_assign();
        assert!(squared.is_valid());
        assert_eq!(squared.to_string(), out);
    };
    test("0", "0");
    test("1", "1");
    test("123", "15129");
    test("-123", "15129");
    test("22/7", "484/49");
    test("-22/7", "484/49");
}

#[test]
fn abs_squared_properties() {
    rational_gen().test_properties(|x| {
        let abs_squared = x.clone().abs_squared();
        assert!(abs_squared.is_valid());
        assert_eq!((&x).abs_squared(), abs_squared);
        let mut x_alt = x.clone();
        x_alt.abs_squared_assign();
        assert_eq!(x_alt, abs_squared);
        assert_eq!((&x).square(), abs_squared);
        assert_eq!((-&x).abs_squared(), abs_squared);
        assert_eq!((&x).abs().abs_squared(), abs_squared);
        assert!(abs_squared >= 0u32);
    });
}
