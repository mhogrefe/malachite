// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::strings::ToDebugString;
use malachite_base::test_util::generators::{string_gen, unsigned_matrix_gen};
use malachite_base::unsigned_matrix::UnsignedMatrix;

#[test]
fn test_from_str() {
    let test = |s, height, width, out| {
        let m = UnsignedMatrix::<u64>::from_str(s).unwrap();
        assert!(m.is_valid());
        assert_eq!((m.height(), m.width()), (height, width));
        assert_eq!(m.to_string(), out);
    };
    test("[]_{0,0}", 0, 0, "[]_{0,0}");
    test("[]_{0,4}", 0, 4, "[]_{0,4}");
    test("[]_{4,0}", 4, 0, "[]_{4,0}");
    // - leading zeros in a dimension
    test("[]_{007,0}", 7, 0, "[]_{7,0}");
    test("[]_{00,00}", 0, 0, "[]_{0,0}");
    test("[0]", 1, 1, "[0]");
    test("[5]", 1, 1, "[5]");
    test("[1, 2, 3]", 1, 3, "[1, 2, 3]");
    test("[1; 2; 3]", 3, 1, "[1; 2; 3]");
    test("[1, 2; 3, 4]", 2, 2, "[1, 2; 3, 4]");
    test("[1, 20, 3; 40, 5, 60]", 2, 3, "[1, 20, 3; 40, 5, 60]");
    test(
        "[18446744073709551615; 1]",
        2,
        1,
        "[18446744073709551615; 1]",
    );
    // - the largest height
    let s = format!("[]_{{{},0}}", usize::MAX);
    let m = UnsignedMatrix::<u64>::from_str(&s).unwrap();
    assert_eq!((m.height(), m.width()), (u64::exact_from(usize::MAX), 0));
    assert_eq!(m.to_string(), s);
    // - an element is read the way `u64::from_str` reads one
    test("[007, +8; 0, 009]", 2, 2, "[7, 8; 0, 9]");
}

#[test]
fn test_from_str_fail() {
    let test = |s: &str| {
        assert!(
            UnsignedMatrix::<u64>::from_str(s).is_err(),
            "{s:?} should not parse"
        );
    };
    test("");
    test("[");
    test("]");
    test("][");
    test("1");
    test("[]");
    test("[[1]]");
    test("[1][2]");
    test("(1, 2)");
    // - neither dimension is zero
    test("[]_{1,1}");
    test("[]_{2,3}");
    // - malformed dimensions
    test("[]_{0}");
    test("[]_{0,0,0}");
    test("[]_{,0}");
    test("[]_{0,}");
    test("[]_{+0,4}");
    test("[]_{-0,4}");
    test("[]_{0, 4}");
    test("[]_{ 0,4}");
    test("[]_(0,4)");
    test("[]{0,4}");
    test("[]_{0,4");
    test("[]_0,4}");
    // - a dimension too large for a `usize`
    test(&format!("[]_{{{}0,0}}", usize::MAX));
    test("[]_{a,0}");
    // - rows of different lengths
    test("[1, 2; 3]");
    test("[1; 2, 3]");
    // - spacing
    test("[1,2]");
    test("[1,  2]");
    test("[1, 2;3, 4]");
    test("[1, 2 ;3, 4]");
    test("[1, 2;  3, 4]");
    test("[ 1]");
    test("[1 ]");
    // - empty elements and rows
    test("[ ]");
    test("[, ]");
    test("[1, ]");
    test("[; ]");
    test("[1; ]");
    test("[; 1]");
    test("[1, 2; ; 3, 4]");
    // - an element too large for a `u64`
    test("[18446744073709551616]");
    test("[1; 18446744073709551616]");
    // - bad elements
    // - a negative element, even -0
    test("[-1]");
    test("[-0]");
    test("[1, -2]");
    test("[--1]");
    test("[-]");
    test("[+]");
    test("[a]");
}

#[test]
fn from_str_properties() {
    unsigned_matrix_gen().test_properties(|m| {
        let s = m.to_string();
        let m_alt = UnsignedMatrix::<u64>::from_str(&s).unwrap();
        assert!(m_alt.is_valid());
        assert_eq!(m_alt, m);
        assert_eq!(
            UnsignedMatrix::<u64>::from_str(&m.to_debug_string()).unwrap(),
            m
        );
    });

    unsigned_matrix_gen().test_properties(|m| {
        // The empty form with both dimensions nonzero is rejected, and with either zero accepted.
        let (h, w) = (m.height(), m.width());
        let parsed = UnsignedMatrix::<u64>::from_str(&format!("[]_{{{h},{w}}}"));
        if h == 0 || w == 0 {
            assert_eq!(parsed.unwrap(), m);
        } else {
            assert!(parsed.is_err());
        }
    });

    string_gen().test_properties(|s| {
        if let Ok(m) = UnsignedMatrix::<u64>::from_str(&s) {
            assert!(m.is_valid());
            assert_eq!(UnsignedMatrix::<u64>::from_str(&m.to_string()).unwrap(), m);
            if m.elements.is_empty() {
                assert!(s.starts_with("[]_{"));
            } else {
                // Whatever parses with elements is a bracketed list of rows of `u64` strings.
                let inner = &s[1..s.len() - 1];
                let xs: Vec<u64> = inner
                    .split("; ")
                    .flat_map(|row| row.split(", "))
                    .map(|x| u64::from_str(x).unwrap())
                    .collect();
                assert_eq!(xs, m.elements);
            }
        }
    });
}
