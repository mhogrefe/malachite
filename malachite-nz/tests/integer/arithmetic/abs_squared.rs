// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Abs, AbsSquared, Square};
use malachite_nz::integer::Integer;
use malachite_nz::test_util::generators::integer_gen;
use std::str::FromStr;

#[test]
fn test_abs_squared() {
    let test = |s, out| {
        let x = Integer::from_str(s).unwrap();

        let squared = x.clone().abs_squared();
        assert!(squared.is_valid());
        assert_eq!(squared.to_string(), out);

        let squared = (&x).abs_squared();
        assert!(squared.is_valid());
        assert_eq!(squared.to_string(), out);
    };
    test("0", "0");
    test("1", "1");
    test("123", "15129");
    test("-123", "15129");
    test("-1000000000000", "1000000000000000000000000");
}

#[test]
fn abs_squared_properties() {
    integer_gen().test_properties(|x| {
        let abs_squared = x.clone().abs_squared();
        assert!(abs_squared.is_valid());
        assert_eq!((&x).abs_squared(), abs_squared);
        assert_eq!((&x).square(), abs_squared);
        assert_eq!((-&x).abs_squared(), abs_squared);
        assert_eq!((&x).abs().abs_squared(), abs_squared);
        assert!(abs_squared >= 0u32);
    });
}
