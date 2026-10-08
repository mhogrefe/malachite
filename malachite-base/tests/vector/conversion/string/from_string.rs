// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::test_util::generators::{string_gen, unsigned_vec_gen, unsigned_vector_gen};
use malachite_base::vector::Vector;

#[test]
fn test_from_str() {
    let test = |s, out| {
        assert_eq!(Vector::<i32>::from_str(s).unwrap().to_string(), out);
    };
    test("()", "()");
    test("(0)", "(0)");
    test("(1, -2, 3)", "(1, -2, 3)");
    test("(+5)", "(5)");
    // An element whose own string contains ", ": a vector of vectors.
    let vv = Vector::<Vector<u8>>::from_str("((1, 2), (), (3))").unwrap();
    assert_eq!(vv.dimension(), 3);
    assert_eq!(vv.to_string(), "((1, 2), (), (3))");
}

#[test]
fn test_from_str_fail() {
    let test = |s| {
        assert!(
            Vector::<u32>::from_str(s).is_err(),
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
    assert!(Vector::<Vector<u8>>::from_str("((1, 2), (3)").is_err());
}

#[test]
fn from_str_properties() {
    unsigned_vector_gen().test_properties(|v| {
        assert_eq!(Vector::<u64>::from_str(&v.to_string()).unwrap(), v);
    });

    unsigned_vec_gen::<u8>().test_properties(|xs| {
        let v = Vector { elements: xs };
        assert_eq!(Vector::<u8>::from_str(&v.to_string()).unwrap(), v);
    });

    string_gen().test_properties(|s| {
        if let Ok(v) = Vector::<u32>::from_str(&s) {
            assert_eq!(v.to_string(), s);
        }
    });
}
