// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use itertools::Itertools;
use malachite_base::test_util::generators::string_gen;
use malachite_base::vector::Vector;
use malachite_nz::integer::Integer;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::test_util::generators::{integer_vec_gen, integer_vector_gen};

#[test]
fn test_from_str() {
    let test = |s, out| {
        let v = IntegerVector::from_str(s).unwrap();
        assert_eq!(v.to_string(), out);
    };
    test("()", "()");
    test("(0)", "(0)");
    test("(5)", "(5)");
    test("(1, 2, 3)", "(1, 2, 3)");
    test("(0, 0)", "(0, 0)");
    test("(4294967295, 10)", "(4294967295, 10)");
    test(
        "(123456789012345678901234567890)",
        "(123456789012345678901234567890)",
    );
    // An element is read the way `Integer::from_str` reads one.
    test("(007)", "(7)");
    test("(+8, 9)", "(8, 9)");
    test("(-1)", "(-1)");
    test("(-007, 8)", "(-7, 8)");
    test("(-0, +0)", "(0, 0)");
    test(
        "(-123456789012345678901234567890, 5)",
        "(-123456789012345678901234567890, 5)",
    );
}

#[test]
fn test_from_str_fail() {
    let test = |s| {
        assert!(
            IntegerVector::from_str(s).is_err(),
            "{s:?} should not parse"
        );
    };
    test("");
    test("(");
    test(")");
    test(")(");
    test("1");
    test("1, 2");
    test("(1, 2");
    test("1, 2)");
    test("(1,2)");
    test("(1,  2)");
    test("( 1, 2)");
    test("(1, 2 )");
    test("( )");
    test("(, )");
    test("(1, )");
    test("(, 1)");
    test("(1, , 2)");
    test("(--1)");
    test("(-)");
    test("(+-1)");
    test("(+)");
    test("(a)");
    test("((1))");
    test("(1)(2)");
}

#[test]
fn from_str_properties() {
    integer_vector_gen().test_properties(|v| {
        assert_eq!(IntegerVector::from_str(&v.to_string()).unwrap(), v);
    });

    integer_vec_gen().test_properties(|xs| {
        // A vector's string is its elements' strings, joined and parenthesized.
        let s = format!("({})", xs.iter().join(", "));
        assert_eq!(IntegerVector::from_str(&s).unwrap().elements, xs);
    });

    string_gen().test_properties(|s| {
        if let Ok(v) = IntegerVector::from_str(&s) {
            // Whatever parses is a parenthesized list of strings that parse as `Integer`s.
            let inner = &s[1..s.len() - 1];
            if inner.is_empty() {
                assert_eq!(v.dimension(), 0);
            } else {
                let parts: Vec<&str> = inner.split(", ").collect();
                assert_eq!(v.dimension(), u64::try_from(parts.len()).unwrap());
                for (part, x) in parts.into_iter().zip(v.elements.iter()) {
                    assert_eq!(&Integer::from_str(part).unwrap(), x);
                }
            }
            assert_eq!(IntegerVector::from_str(&v.to_string()).unwrap(), v);
        }
    });
}
