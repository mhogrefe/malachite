// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::test_util::generators::{string_gen, unsigned_vec_gen, unsigned_vector_gen};
use malachite_base::unsigned_vector::UnsignedVector;

#[test]
fn test_from_str() {
    let test = |s, out| {
        assert_eq!(UnsignedVector::<u32>::from_str(s).unwrap().to_string(), out);
    };
    test("()", "()");
    test("(0)", "(0)");
    test("(1, 2, 3)", "(1, 2, 3)");
    test("(+5)", "(5)");
    test("(007, 8)", "(7, 8)");
    test("(4294967295, 0)", "(4294967295, 0)");
}

#[test]
fn test_from_str_fail() {
    let test = |s| {
        assert!(
            UnsignedVector::<u32>::from_str(s).is_err(),
            "{s:?} should not parse"
        );
    };
    test("");
    test("(");
    test(")");
    test("1, 2");
    test("(1,2)");
    test("(1,  2)");
    test("( 1, 2)");
    test("(1, 2 )");
    test("( )");
    test("(1, )");
    test("(, 1)");
    test("(1, , 2)");
    test("(-1)");
    test("(256, 1)x");
    test("(4294967296)");
    test("(1, -2)");
    test("(a)");
}

#[test]
fn from_str_properties() {
    unsigned_vector_gen().test_properties(|v| {
        assert_eq!(UnsignedVector::<u64>::from_str(&v.to_string()).unwrap(), v);
    });

    unsigned_vec_gen::<u8>().test_properties(|xs| {
        let v = UnsignedVector { elements: xs };
        assert_eq!(UnsignedVector::<u8>::from_str(&v.to_string()).unwrap(), v);
    });

    string_gen().test_properties(|s| {
        if let Ok(v) = UnsignedVector::<u32>::from_str(&s) {
            assert_eq!(v.to_string(), s);
        }
    });
}
